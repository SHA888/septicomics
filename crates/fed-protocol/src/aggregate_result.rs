use serde::{Deserialize, Deserializer, Serialize};

/// Errors that can occur when constructing or deserializing aggregate results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateError {
    /// A count value is invalid.
    InvalidCount {
        /// Description of the count error.
        field: String,
    },
    /// A probability/p-value is outside [0, 1].
    InvalidProbability {
        /// Description of the probability error.
        field: String,
    },
    /// A statistic value is invalid (e.g., NaN or infinite).
    InvalidStatistic {
        /// Description of the statistic error.
        field: String,
    },
    /// A time value is invalid (e.g., negative).
    InvalidTime {
        /// Description of the time error.
        field: String,
    },
}

impl std::fmt::Display for AggregateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateError::InvalidCount { field } => write!(f, "invalid count: {}", field),
            AggregateError::InvalidProbability { field } => {
                write!(f, "invalid probability (must be in [0, 1]): {}", field)
            }
            AggregateError::InvalidStatistic { field } => {
                write!(f, "invalid statistic (must be finite): {}", field)
            }
            AggregateError::InvalidTime { field } => {
                write!(f, "invalid time (must be non-negative): {}", field)
            }
        }
    }
}

impl std::error::Error for AggregateError {}

/// Validates that a count is positive.
fn validate_positive_count(count: u64, field: &str) -> Result<(), AggregateError> {
    if count == 0 {
        Err(AggregateError::InvalidCount {
            field: format!("{} must be > 0", field),
        })
    } else {
        Ok(())
    }
}

/// Validates that a bounded float is in the range [0, 1].
fn validate_bounded_float(value: f64, field: &str) -> Result<(), AggregateError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        Err(AggregateError::InvalidProbability {
            field: format!("{} must be in [0, 1]", field),
        })
    } else {
        Ok(())
    }
}

/// Validates that a statistic is finite.
fn validate_statistic(stat: f64, field: &str) -> Result<(), AggregateError> {
    if !stat.is_finite() {
        Err(AggregateError::InvalidStatistic {
            field: format!("{} must be finite", field),
        })
    } else {
        Ok(())
    }
}

/// Validates that a time value is non-negative.
fn validate_time(time: f64, field: &str) -> Result<(), AggregateError> {
    if !time.is_finite() || time < 0.0 {
        Err(AggregateError::InvalidTime {
            field: format!("{} must be non-negative and finite", field),
        })
    } else {
        Ok(())
    }
}

/// Differential expression aggregate result.
///
/// Contains only computed statistics, not raw expression values or subject-level data.
/// This makes it impossible to reconstruct subject-level measurements from the aggregate.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DifferentialExpressionResult {
    /// Number of features tested (must be > 0).
    pub features_tested: u64,
    /// Number of features meeting significance threshold (must be ≤ features_tested).
    pub features_significant: u64,
    /// Mean log2 fold-change across significant features.
    pub mean_log2_fold_change: f64,
    /// Adjusted p-value threshold used (in [0, 1]).
    pub fdr_threshold: f64,
}

impl DifferentialExpressionResult {
    /// Creates a new differential expression result from computed aggregates.
    ///
    /// # Errors
    ///
    /// Returns [`AggregateError`] if:
    /// - `features_tested` is zero
    /// - `features_significant` is greater than `features_tested`
    /// - `mean_log2_fold_change` is not finite
    /// - `fdr_threshold` is outside [0, 1]
    pub fn new(
        features_tested: u64,
        features_significant: u64,
        mean_log2_fold_change: f64,
        fdr_threshold: f64,
    ) -> Result<Self, AggregateError> {
        validate_positive_count(features_tested, "features_tested")?;
        if features_significant > features_tested {
            return Err(AggregateError::InvalidCount {
                field: "features_significant must be ≤ features_tested".to_string(),
            });
        }
        validate_statistic(mean_log2_fold_change, "mean_log2_fold_change")?;
        validate_bounded_float(fdr_threshold, "fdr_threshold")?;

        Ok(Self {
            features_tested,
            features_significant,
            mean_log2_fold_change,
            fdr_threshold,
        })
    }
}

impl<'de> Deserialize<'de> for DifferentialExpressionResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct Raw {
            features_tested: u64,
            features_significant: u64,
            mean_log2_fold_change: f64,
            fdr_threshold: f64,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.features_tested,
            raw.features_significant,
            raw.mean_log2_fold_change,
            raw.fdr_threshold,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Kaplan-Meier survival analysis aggregate result.
///
/// Contains only computed survival probabilities and test statistics,
/// not raw event times or subject-level data.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct KaplanMeierResult {
    /// Total number of subjects in analysis (must be > 0).
    pub total_subjects: u64,
    /// Total number of events observed (must be ≤ total_subjects).
    pub total_events: u64,
    /// Log-rank test statistic.
    pub log_rank_statistic: f64,
    /// P-value for log-rank test (in [0, 1]).
    pub log_rank_pvalue: f64,
    /// Median survival time (in study units), or None if not reached.
    pub median_survival: Option<f64>,
}

impl KaplanMeierResult {
    /// Creates a new Kaplan-Meier result from computed aggregates.
    ///
    /// # Errors
    ///
    /// Returns [`AggregateError`] if:
    /// - `total_subjects` is zero
    /// - `total_events` is greater than `total_subjects`
    /// - `log_rank_statistic` is not finite
    /// - `log_rank_pvalue` is outside [0, 1]
    /// - `median_survival` (if present) is not a non-negative finite value
    pub fn new(
        total_subjects: u64,
        total_events: u64,
        log_rank_statistic: f64,
        log_rank_pvalue: f64,
        median_survival: Option<f64>,
    ) -> Result<Self, AggregateError> {
        validate_positive_count(total_subjects, "total_subjects")?;
        if total_events > total_subjects {
            return Err(AggregateError::InvalidCount {
                field: "total_events must be ≤ total_subjects".to_string(),
            });
        }
        validate_statistic(log_rank_statistic, "log_rank_statistic")?;
        validate_bounded_float(log_rank_pvalue, "log_rank_pvalue")?;
        if let Some(median) = median_survival {
            validate_time(median, "median_survival")?;
        }

        Ok(Self {
            total_subjects,
            total_events,
            log_rank_statistic,
            log_rank_pvalue,
            median_survival,
        })
    }
}

impl<'de> Deserialize<'de> for KaplanMeierResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct Raw {
            total_subjects: u64,
            total_events: u64,
            log_rank_statistic: f64,
            log_rank_pvalue: f64,
            median_survival: Option<f64>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.total_subjects,
            raw.total_events,
            raw.log_rank_statistic,
            raw.log_rank_pvalue,
            raw.median_survival,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Endotype prevalence aggregate result.
///
/// Contains only counts and proportions, not subject-level phenotypes or raw data.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct EndotypePrevalenceResult {
    /// Total subjects in the analyzed cohort (must be > 0).
    pub cohort_size: u64,
    /// Subjects with the target endotype (must be ≤ cohort_size).
    pub endotype_count: u64,
}

impl EndotypePrevalenceResult {
    /// Creates a new endotype prevalence result from aggregate counts.
    ///
    /// # Errors
    ///
    /// Returns [`AggregateError`] if:
    /// - `cohort_size` is zero
    /// - `endotype_count` is greater than `cohort_size`
    pub fn new(cohort_size: u64, endotype_count: u64) -> Result<Self, AggregateError> {
        validate_positive_count(cohort_size, "cohort_size")?;
        if endotype_count > cohort_size {
            return Err(AggregateError::InvalidCount {
                field: "endotype_count must be ≤ cohort_size".to_string(),
            });
        }
        Ok(Self {
            cohort_size,
            endotype_count,
        })
    }

    /// Returns the proportion of subjects with the endotype.
    pub fn prevalence(&self) -> f64 {
        self.endotype_count as f64 / self.cohort_size as f64
    }
}

impl<'de> Deserialize<'de> for EndotypePrevalenceResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct Raw {
            cohort_size: u64,
            endotype_count: u64,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.cohort_size, raw.endotype_count).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differential_expression_valid() {
        let result = DifferentialExpressionResult::new(1000, 50, 1.5, 0.05);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.features_tested, 1000);
        assert_eq!(r.features_significant, 50);
        assert_eq!(r.mean_log2_fold_change, 1.5);
        assert_eq!(r.fdr_threshold, 0.05);
    }

    #[test]
    fn differential_expression_zero_features_tested() {
        let result = DifferentialExpressionResult::new(0, 0, 1.0, 0.05);
        assert!(result.is_err());
    }

    #[test]
    fn differential_expression_significant_exceeds_tested() {
        let result = DifferentialExpressionResult::new(100, 101, 1.0, 0.05);
        assert!(result.is_err());
    }

    #[test]
    fn differential_expression_nan_fold_change() {
        let result = DifferentialExpressionResult::new(100, 50, f64::NAN, 0.05);
        assert!(result.is_err());
    }

    #[test]
    fn differential_expression_invalid_fdr() {
        let result = DifferentialExpressionResult::new(100, 50, 1.0, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn kaplan_meier_valid() {
        let result = KaplanMeierResult::new(500, 150, 2.5, 0.01, Some(180.5));
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.total_subjects, 500);
        assert_eq!(r.total_events, 150);
        assert_eq!(r.log_rank_statistic, 2.5);
        assert_eq!(r.log_rank_pvalue, 0.01);
        assert_eq!(r.median_survival, Some(180.5));
    }

    #[test]
    fn kaplan_meier_zero_subjects() {
        let result = KaplanMeierResult::new(0, 0, 1.0, 0.05, None);
        assert!(result.is_err());
    }

    #[test]
    fn kaplan_meier_events_exceed_subjects() {
        let result = KaplanMeierResult::new(100, 101, 1.0, 0.05, None);
        assert!(result.is_err());
    }

    #[test]
    fn kaplan_meier_invalid_pvalue() {
        let result = KaplanMeierResult::new(100, 50, 1.0, -0.1, None);
        assert!(result.is_err());
    }

    #[test]
    fn kaplan_meier_nan_median_survival() {
        let result = KaplanMeierResult::new(100, 50, 1.0, 0.05, Some(f64::NAN));
        assert!(result.is_err());
    }

    #[test]
    fn kaplan_meier_negative_median_survival() {
        let result = KaplanMeierResult::new(100, 50, 1.0, 0.05, Some(-100.5));
        assert!(result.is_err());
    }

    #[test]
    fn endotype_prevalence_valid() {
        let result = EndotypePrevalenceResult::new(1000, 250);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.cohort_size, 1000);
        assert_eq!(r.endotype_count, 250);
        assert_eq!(r.prevalence(), 0.25);
    }

    #[test]
    fn endotype_prevalence_zero_cohort() {
        let result = EndotypePrevalenceResult::new(0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn endotype_prevalence_count_exceeds_cohort() {
        let result = EndotypePrevalenceResult::new(100, 101);
        assert!(result.is_err());
    }

    #[test]
    fn serde_roundtrip_differential_expression() {
        let result = DifferentialExpressionResult::new(1000, 50, 1.5, 0.05).unwrap();
        let json = serde_json::to_string(&result).expect("serialization failed");
        let deserialized: DifferentialExpressionResult =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(result, deserialized);
    }

    #[test]
    fn serde_roundtrip_kaplan_meier() {
        let result = KaplanMeierResult::new(500, 150, 2.5, 0.01, Some(180.5)).unwrap();
        let json = serde_json::to_string(&result).expect("serialization failed");
        let deserialized: KaplanMeierResult =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(result, deserialized);
    }

    #[test]
    fn serde_roundtrip_endotype_prevalence() {
        let result = EndotypePrevalenceResult::new(1000, 250).unwrap();
        let json = serde_json::to_string(&result).expect("serialization failed");
        let deserialized: EndotypePrevalenceResult =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(result, deserialized);
    }

    #[test]
    fn deserialization_enforces_invariants_differential_expression() {
        let invalid_json = r#"{"features_tested": 100, "features_significant": 101, "mean_log2_fold_change": 1.5, "fdr_threshold": 0.05}"#;
        let result: Result<DifferentialExpressionResult, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject features_significant > features_tested"
        );
    }

    #[test]
    fn deserialization_enforces_invariants_kaplan_meier() {
        let invalid_json = r#"{"total_subjects": 100, "total_events": 101, "log_rank_statistic": 1.0, "log_rank_pvalue": 0.05, "median_survival": null}"#;
        let result: Result<KaplanMeierResult, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject total_events > total_subjects"
        );
    }

    #[test]
    fn deserialization_enforces_invariants_kaplan_meier_negative_time() {
        let invalid_json = r#"{"total_subjects": 100, "total_events": 50, "log_rank_statistic": 1.0, "log_rank_pvalue": 0.05, "median_survival": -100.5}"#;
        let result: Result<KaplanMeierResult, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject negative median_survival"
        );
    }

    #[test]
    fn deserialization_enforces_invariants_endotype_prevalence() {
        let invalid_json = r#"{"cohort_size": 100, "endotype_count": 101}"#;
        let result: Result<EndotypePrevalenceResult, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject endotype_count > cohort_size"
        );
    }

    #[test]
    fn subject_level_data_unrepresentable() {
        // Verify that there is no way to construct an aggregate result with raw subject data.
        // All constructors require pre-computed statistics (counts, probabilities, statistics),
        // not raw measurements or subject identifiers.
        //
        // We cannot construct:
        //   - DifferentialExpressionResult::new_from_raw_values(...) ← does not exist
        //   - KaplanMeierResult::new_from_subjects(...) ← does not exist
        //   - EndotypePrevalenceResult::new_from_subject_list(...) ← does not exist
        //
        // The only way to create these types is to pass pre-computed aggregates,
        // and all deserialization also goes through validation via new().

        let _de = DifferentialExpressionResult::new(1000, 50, 1.5, 0.05).unwrap();
        let _km = KaplanMeierResult::new(500, 150, 2.5, 0.01, None).unwrap();
        let _ep = EndotypePrevalenceResult::new(1000, 250).unwrap();
    }
}
