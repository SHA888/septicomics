use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Specifies parameters for selecting a cohort of subjects from node-local data.
/// Parameters are key-value pairs that nodes interpret against local phenotypes/outcomes.
///
/// This is parameterizable (nodes can define what phenotype keys and operators they support)
/// but does not allow arbitrary code execution.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CohortSelector {
    /// Named filters; e.g., {"endotype": "sepsis_canonical", "age_min": "18"}
    pub filters: HashMap<String, String>,
}

impl CohortSelector {
    /// Creates a new cohort selector with the given filters.
    pub fn new(filters: HashMap<String, String>) -> Self {
        Self { filters }
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
}

/// Omics layer types from the CDM that a node may analyze.
/// Exhaustive enum ensures only known layers can be selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmicsLayer {
    /// Transcriptomics: gene expression data.
    Transcriptomics,
    /// Proteomics: protein abundance data.
    Proteomics,
    /// Metabolomics: metabolite concentration data.
    Metabolomics,
    /// Single-cell: single-cell genomics data.
    SingleCell,
}

impl OmicsLayer {
    /// All known omics layers.
    pub const ALL: &'static [OmicsLayer] = &[
        OmicsLayer::Transcriptomics,
        OmicsLayer::Proteomics,
        OmicsLayer::Metabolomics,
        OmicsLayer::SingleCell,
    ];

    /// Human-readable name of the layer.
    pub fn name(self) -> &'static str {
        match self {
            OmicsLayer::Transcriptomics => "transcriptomics",
            OmicsLayer::Proteomics => "proteomics",
            OmicsLayer::Metabolomics => "metabolomics",
            OmicsLayer::SingleCell => "single-cell",
        }
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
        /// Reference (baseline) group identifier.
        reference_group: String,
        /// Comparison (treatment) group identifier.
        comparison_group: String,
        /// False discovery rate threshold for significance.
        fdr_threshold: f32,
    },
    /// Survival analysis: fit Kaplan-Meier curves and log-rank test.
    KaplanMeier {
        /// Outcome variable name (e.g., "mortality").
        event_outcome: String,
        /// Time-to-event variable name (e.g., "days_icu").
        time_variable: String,
    },
    /// Prevalence of endotypes within a cohort.
    EndotypePrevalence {
        /// Endotype identifier.
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
}

/// Analysis plan: the complete specification for federated analysis across nodes.
///
/// Combines:
/// - **cohort**: patient selection (parameterizable but not arbitrary)
/// - **omics layers**: which data modalities to analyze
/// - **estimator**: which pre-approved analysis to run (not free-form code)
///
/// This structure makes it impossible to represent arbitrary code on the wire,
/// enforcing the invariant: free-form code is unrepresentable.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnalysisPlan {
    /// Stable identifier for this analysis across federation.
    pub id: String,
    /// Patient selection criteria (parameterizable but not arbitrary code).
    pub cohort: CohortSelector,
    /// Omics modalities to include in analysis.
    pub omics_layers: Vec<OmicsLayer>,
    /// Pre-approved estimator to apply (exhaustive enum: not free-form code).
    pub estimator: EstimatorVariant,
}

impl AnalysisPlan {
    /// Creates a new analysis plan.
    ///
    /// # Panics
    /// Panics if `omics_layers` is empty.
    pub fn new(
        id: String,
        cohort: CohortSelector,
        omics_layers: Vec<OmicsLayer>,
        estimator: EstimatorVariant,
    ) -> Self {
        assert!(
            !omics_layers.is_empty(),
            "at least one omics layer required"
        );
        Self {
            id,
            cohort,
            omics_layers,
            estimator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cohort_selector_empty() {
        let selector = CohortSelector::all();
        assert!(selector.is_empty());
        assert!(selector.filters.is_empty());
    }

    #[test]
    fn cohort_selector_with_filters() {
        let mut filters = HashMap::new();
        filters.insert("endotype".to_string(), "sepsis_canonical".to_string());
        filters.insert("age_min".to_string(), "18".to_string());

        let selector = CohortSelector::new(filters.clone());
        assert!(!selector.is_empty());
        assert_eq!(selector.filters, filters);
    }

    #[test]
    fn omics_layer_names() {
        assert_eq!(OmicsLayer::Transcriptomics.name(), "transcriptomics");
        assert_eq!(OmicsLayer::Proteomics.name(), "proteomics");
        assert_eq!(OmicsLayer::Metabolomics.name(), "metabolomics");
        assert_eq!(OmicsLayer::SingleCell.name(), "single-cell");
    }

    #[test]
    fn omics_layer_all() {
        assert_eq!(OmicsLayer::ALL.len(), 4);
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
    fn analysis_plan_new() {
        let plan = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![OmicsLayer::Transcriptomics],
            EstimatorVariant::DifferentialExpression {
                reference_group: "control".to_string(),
                comparison_group: "sepsis".to_string(),
                fdr_threshold: 0.05,
            },
        );

        assert_eq!(plan.id, "analysis_001");
        assert!(plan.cohort.is_empty());
        assert_eq!(plan.omics_layers.len(), 1);
        assert_eq!(plan.omics_layers[0], OmicsLayer::Transcriptomics);
    }

    #[test]
    #[should_panic(expected = "at least one omics layer required")]
    fn analysis_plan_empty_layers_panics() {
        let _ = AnalysisPlan::new(
            "analysis_001".to_string(),
            CohortSelector::all(),
            vec![],
            EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        );
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
        );

        let json = serde_json::to_string(&plan).expect("serialization failed");
        let deserialized: AnalysisPlan =
            serde_json::from_str(&json).expect("deserialization failed");

        assert_eq!(plan.id, deserialized.id);
        assert_eq!(plan.cohort, deserialized.cohort);
        assert_eq!(plan.omics_layers, deserialized.omics_layers);
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
        );
    }
}
