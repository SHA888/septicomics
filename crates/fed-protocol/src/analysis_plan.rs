use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use septicomics_cdm::omics::OmicsLayer;

/// Errors that can occur when constructing or validating an analysis plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisError {
    /// Omics layers list is empty.
    EmptyOmicsLayers,
    /// Analysis plan ID is empty or whitespace-only.
    EmptyPlanId,
    /// A filter key or value is empty or whitespace-only.
    EmptyFilterValue {
        /// The filter key with invalid value.
        key: String,
    },
    /// A group identifier (reference or comparison) is empty or whitespace-only.
    EmptyGroupIdentifier {
        /// The context (e.g., "DifferentialExpression").
        context: String,
    },
    /// An outcome or phenotype variable name is empty or whitespace-only.
    EmptyVariableName {
        /// The context (e.g., "KaplanMeier.event_outcome").
        context: String,
    },
}

impl std::fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::EmptyOmicsLayers => write!(f, "omics layers list must not be empty"),
            AnalysisError::EmptyPlanId => {
                write!(f, "analysis plan ID must not be empty or whitespace")
            }
            AnalysisError::EmptyFilterValue { key } => {
                write!(
                    f,
                    "filter value for key '{}' must not be empty or whitespace",
                    key
                )
            }
            AnalysisError::EmptyGroupIdentifier { context } => {
                write!(
                    f,
                    "group identifier in {} must not be empty or whitespace",
                    context
                )
            }
            AnalysisError::EmptyVariableName { context } => {
                write!(
                    f,
                    "variable name in {} must not be empty or whitespace",
                    context
                )
            }
        }
    }
}

impl std::error::Error for AnalysisError {}

/// Checks that a string is non-empty and not whitespace-only.
fn validate_nonempty(s: &str, context: &str) -> Result<(), AnalysisError> {
    if s.trim().is_empty() {
        match context {
            "plan_id" => Err(AnalysisError::EmptyPlanId),
            "group" => Err(AnalysisError::EmptyGroupIdentifier {
                context: "DifferentialExpression".to_string(),
            }),
            "outcome" => Err(AnalysisError::EmptyVariableName {
                context: "KaplanMeier.event_outcome".to_string(),
            }),
            "time" => Err(AnalysisError::EmptyVariableName {
                context: "KaplanMeier.time_variable".to_string(),
            }),
            "endotype" => Err(AnalysisError::EmptyVariableName {
                context: "EndotypePrevalence.endotype".to_string(),
            }),
            _ => Err(AnalysisError::EmptyPlanId),
        }
    } else {
        Ok(())
    }
}

/// Specifies parameters for selecting a cohort of subjects from node-local data.
/// Parameters are key-value pairs that nodes interpret against local phenotypes/outcomes.
///
/// This is parameterizable (nodes can define what phenotype keys and operators they support)
/// but does not allow arbitrary code execution. Both keys and values are validated
/// for non-emptiness to prevent malformed selections.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CohortSelector {
    /// Named filters; e.g., {"endotype": "sepsis_canonical", "age_min": "18"}
    /// Both keys and values must be non-empty and non-whitespace.
    filters: HashMap<String, String>,
}

impl CohortSelector {
    /// Creates a new cohort selector with the given filters.
    ///
    /// # Errors
    ///
    /// Returns [`AnalysisError::EmptyFilterValue`] if any key or value is empty or whitespace-only.
    pub fn new(filters: HashMap<String, String>) -> Result<Self, AnalysisError> {
        for (key, value) in &filters {
            if key.trim().is_empty() {
                return Err(AnalysisError::EmptyFilterValue {
                    key: "(empty key)".to_string(),
                });
            }
            if value.trim().is_empty() {
                return Err(AnalysisError::EmptyFilterValue { key: key.clone() });
            }
        }
        Ok(Self { filters })
    }

    /// Creates an empty selector that selects all subjects.
    pub fn all() -> Self {
        Self {
            filters: HashMap::new(),
        }
    }

    /// Returns true if this selector has no filters (selects all subjects).
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// Returns a reference to the filters map.
    pub fn filters(&self) -> &HashMap<String, String> {
        &self.filters
    }
}

/// Allow-listed estimator variants that nodes can apply to cohorts.
/// By making this an exhaustive enum, we prevent nodes from executing arbitrary code.
///
/// Each variant corresponds to a pre-approved estimator implementation:
/// nodes only execute the code path matching the selected variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "estimator", content = "params", rename_all = "snake_case")]
pub enum EstimatorVariant {
    /// Differential expression: compare two groups on an omics layer.
    DifferentialExpression {
        /// Reference (baseline) group identifier (non-empty).
        reference_group: String,
        /// Comparison (treatment) group identifier (non-empty).
        comparison_group: String,
        /// False discovery rate threshold for significance.
        fdr_threshold: f32,
    },
    /// Survival analysis: fit Kaplan-Meier curves and log-rank test.
    KaplanMeier {
        /// Outcome variable name (non-empty, e.g., "mortality").
        event_outcome: String,
        /// Time-to-event variable name (non-empty, e.g., "days_icu").
        time_variable: String,
    },
    /// Prevalence of endotypes within a cohort.
    EndotypePrevalence {
        /// Endotype identifier (non-empty).
        endotype: String,
    },
}

impl EstimatorVariant {
    /// Returns the name of this estimator variant.
    pub fn name(&self) -> &'static str {
        match self {
            EstimatorVariant::DifferentialExpression { .. } => "differential_expression",
            EstimatorVariant::KaplanMeier { .. } => "kaplan_meier",
            EstimatorVariant::EndotypePrevalence { .. } => "endotype_prevalence",
        }
    }

    /// Validates that all string fields are non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`AnalysisError`] if any identifier field is empty or whitespace-only.
    pub fn validate(&self) -> Result<(), AnalysisError> {
        match self {
            EstimatorVariant::DifferentialExpression {
                reference_group,
                comparison_group,
                ..
            } => {
                validate_nonempty(reference_group, "group")?;
                validate_nonempty(comparison_group, "group")?;
                Ok(())
            }
            EstimatorVariant::KaplanMeier {
                event_outcome,
                time_variable,
            } => {
                validate_nonempty(event_outcome, "outcome")?;
                validate_nonempty(time_variable, "time")?;
                Ok(())
            }
            EstimatorVariant::EndotypePrevalence { endotype } => {
                validate_nonempty(endotype, "endotype")?;
                Ok(())
            }
        }
    }
}

/// Analysis plan: the complete specification for federated analysis across nodes.
///
/// Combines:
/// - **cohort**: patient selection (parameterizable but not arbitrary)
/// - **omics layers**: which data modalities to analyze (exhaustive enum from CDM)
/// - **estimator**: which pre-approved analysis to run (not free-form code)
///
/// This structure makes it impossible to represent arbitrary code on the wire,
/// enforcing the invariant: free-form code is unrepresentable.
///
/// All string fields are validated at construction to reject empty or whitespace-only values,
/// enforcing the parse-don't-validate discipline.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnalysisPlan {
    /// Stable identifier for this analysis across federation (non-empty).
    id: String,
    /// Patient selection criteria (parameterizable but not arbitrary code).
    cohort: CohortSelector,
    /// Omics modalities to include in analysis (non-empty, from CDM).
    omics_layers: Vec<OmicsLayer>,
    /// Pre-approved estimator to apply (exhaustive enum: not free-form code).
    estimator: EstimatorVariant,
}

impl AnalysisPlan {
    /// Creates a new analysis plan.
    ///
    /// # Errors
    ///
    /// Returns [`AnalysisError`] if:
    /// - `id` is empty or whitespace-only
    /// - `omics_layers` is empty
    /// - `estimator` contains empty identifier fields
    pub fn new(
        id: String,
        cohort: CohortSelector,
        omics_layers: Vec<OmicsLayer>,
        estimator: EstimatorVariant,
    ) -> Result<Self, AnalysisError> {
        validate_nonempty(&id, "plan_id")?;
        if omics_layers.is_empty() {
            return Err(AnalysisError::EmptyOmicsLayers);
        }
        estimator.validate()?;

        Ok(Self {
            id,
            cohort,
            omics_layers,
            estimator,
        })
    }

    /// Returns the stable identifier for this analysis.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the patient selection criteria.
    pub fn cohort(&self) -> &CohortSelector {
        &self.cohort
    }

    /// Returns the omics modalities to analyze.
    pub fn omics_layers(&self) -> &[OmicsLayer] {
        &self.omics_layers
    }

    /// Returns the estimator to apply.
    pub fn estimator(&self) -> &EstimatorVariant {
        &self.estimator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cohort_selector_empty() {
        let selector = CohortSelector::all();
        assert!(selector.is_empty());
        assert!(selector.filters().is_empty());
    }

    #[test]
    fn cohort_selector_with_filters() {
        let mut filters = HashMap::new();
        filters.insert("endotype".to_string(), "sepsis_canonical".to_string());
        filters.insert("age_min".to_string(), "18".to_string());

        let selector = CohortSelector::new(filters.clone()).expect("valid filters");
        assert!(!selector.is_empty());
        assert_eq!(selector.filters(), &filters);
    }

    #[test]
    fn cohort_selector_rejects_empty_value() {
        let mut filters = HashMap::new();
        filters.insert("endotype".to_string(), "   ".to_string());

        let result = CohortSelector::new(filters);
        assert!(matches!(
            result,
            Err(AnalysisError::EmptyFilterValue { .. })
        ));
    }

    #[test]
    fn cohort_selector_rejects_empty_key() {
        let mut filters = HashMap::new();
        filters.insert("  ".to_string(), "value".to_string());

        let result = CohortSelector::new(filters);
        assert!(matches!(
            result,
            Err(AnalysisError::EmptyFilterValue { .. })
        ));
    }

    #[test]
    fn estimator_variant_de_validates() {
        let de = EstimatorVariant::DifferentialExpression {
            reference_group: "control".to_string(),
            comparison_group: "treatment".to_string(),
            fdr_threshold: 0.05,
        };
        assert!(de.validate().is_ok());

        let invalid_de = EstimatorVariant::DifferentialExpression {
            reference_group: "".to_string(),
            comparison_group: "treatment".to_string(),
            fdr_threshold: 0.05,
        };
        assert!(invalid_de.validate().is_err());
    }

    #[test]
    fn estimator_variant_km_validates() {
        let km = EstimatorVariant::KaplanMeier {
            event_outcome: "mortality".to_string(),
            time_variable: "days_icu".to_string(),
        };
        assert!(km.validate().is_ok());

        let invalid_km = EstimatorVariant::KaplanMeier {
            event_outcome: "   ".to_string(),
            time_variable: "days_icu".to_string(),
        };
        assert!(invalid_km.validate().is_err());
    }

    #[test]
    fn estimator_variant_ep_validates() {
        let ep = EstimatorVariant::EndotypePrevalence {
            endotype: "sepsis_canonical".to_string(),
        };
        assert!(ep.validate().is_ok());

        let invalid_ep = EstimatorVariant::EndotypePrevalence {
            endotype: "".to_string(),
        };
        assert!(invalid_ep.validate().is_err());
    }

    #[test]
    fn estimator_variant_names() {
        let de = EstimatorVariant::DifferentialExpression {
            reference_group: "control".to_string(),
            comparison_group: "treatment".to_string(),
            fdr_threshold: 0.05,
        };
        assert_eq!(de.name(), "differential_expression");

        let km = EstimatorVariant::KaplanMeier {
            event_outcome: "mortality".to_string(),
            time_variable: "days_icu".to_string(),
        };
        assert_eq!(km.name(), "kaplan_meier");

        let ep = EstimatorVariant::EndotypePrevalence {
            endotype: "sepsis_canonical".to_string(),
        };
        assert_eq!(ep.name(), "endotype_prevalence");
    }

    #[test]
    fn analysis_plan_new_success() {
        let plan = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics],
            EstimatorVariant::DifferentialExpression {
                reference_group: "control".to_string(),
                comparison_group: "sepsis".to_string(),
                fdr_threshold: 0.05,
            },
        )
        .expect("valid plan");

        assert_eq!(plan.id(), "analysis_001");
        assert!(plan.cohort().is_empty());
        assert_eq!(plan.omics_layers().len(), 1);
        assert_eq!(plan.omics_layers()[0], OmicsLayer::Transcriptomics);
    }

    #[test]
    fn analysis_plan_empty_layers_error() {
        let result = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![],
            EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        );
        assert!(matches!(result, Err(AnalysisError::EmptyOmicsLayers)));
    }

    #[test]
    fn analysis_plan_empty_id_error() {
        let result = AnalysisPlan::new(
            "   ".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics],
            EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        );
        assert!(matches!(result, Err(AnalysisError::EmptyPlanId)));
    }

    #[test]
    fn analysis_plan_invalid_estimator_error() {
        let result = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics],
            EstimatorVariant::DifferentialExpression {
                reference_group: "".to_string(),
                comparison_group: "treatment".to_string(),
                fdr_threshold: 0.05,
            },
        );
        assert!(matches!(
            result,
            Err(AnalysisError::EmptyGroupIdentifier { .. })
        ));
    }

    #[test]
    fn analysis_plan_serde_roundtrip() {
        let plan = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics, OmicsLayer::Proteomics],
            EstimatorVariant::DifferentialExpression {
                reference_group: "control".to_string(),
                comparison_group: "sepsis".to_string(),
                fdr_threshold: 0.05,
            },
        )
        .expect("valid plan");

        let json = serde_json::to_string(&plan).expect("serialization failed");
        let deserialized: AnalysisPlan =
            serde_json::from_str(&json).expect("deserialization failed");

        assert_eq!(plan.id(), deserialized.id());
        assert_eq!(plan.cohort(), deserialized.cohort());
        assert_eq!(plan.omics_layers(), deserialized.omics_layers());
    }

    #[test]
    fn free_form_code_unrepresentable() {
        // Verify that there is no way to represent arbitrary code in an AnalysisPlan.
        // The only way to specify computation is through the EstimatorVariant enum,
        // which has a closed set of variants.
        //
        // We cannot construct:
        //   - EstimatorVariant::ArbitraryCode { code: "..." } ← does not exist
        //   - EstimatorVariant::ExecuteFunction { name: "..." } ← does not exist
        //   - Any variant that takes a string and evaluates it as code ← by design
        //
        // This is enforced at compile time: only the listed variants are constructible.
        let _plan = AnalysisPlan::new(
            "test".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics],
            EstimatorVariant::KaplanMeier {
                event_outcome: "mortality".to_string(),
                time_variable: "days_icu".to_string(),
            },
        )
        .expect("valid plan");
    }

    #[test]
    fn omics_layers_from_cdm_work() {
        // Verify that we can use all CDM omics layers in a plan.
        for &layer in OmicsLayer::ALL {
            let plan = AnalysisPlan::new(
                "test".to_string(),
                CohortSelector::all(),
                vec![layer],
                EstimatorVariant::EndotypePrevalence {
                    endotype: "test".to_string(),
                },
            );
            assert!(plan.is_ok(), "layer {:?} should be valid", layer);
        }
    }
}
