use serde::{Deserialize, Serialize};

/// Errors that can occur when constructing aggregate results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateError {
    /// A required aggregate field is missing or invalid.
    MissingAggregate {
        /// Description of what aggregate is missing.
        field: String,
    },
    /// A count or sample size is zero or negative when it should be positive.
    InvalidCount {
        /// The field that has an invalid count.
        field: String,
    },
    /// A probability value is outside [0, 1].
    InvalidProbability {
        /// The field with invalid probability.
        field: String,
    },
    /// A p-value is outside [0, 1].
    InvalidPvalue {
        /// The field with invalid p-value.
        field: String,
    },
    /// A statistic value is invalid (e.g., NaN or infinite).
    InvalidStatistic {
        /// The field with invalid statistic.
        field: String,
    },
}

impl std::fmt::Display for AggregateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateError::MissingAggregate { field } => {
                write!(f, "missing aggregate field: {}", field)
            }
            AggregateError::InvalidCount { field } => {
                write!(f, "count must be positive: {}", field)
            }
            AggregateError::InvalidProbability { field } => {
                write!(f, "probability must be in [0, 1]: {}", field)
            }
            AggregateError::InvalidPvalue { field } => {
                write!(f, "p-value must be in [0, 1]: {}", field)
            }
            AggregateError::InvalidStatistic { field } => {
                write!(f, "invalid statistic value: {}", field)
            }
        }
    }
}

impl std::error::Error for AggregateError {}

/// Validates that a count is positive.
fn validate_positive_count(count: u64, field: &str) -> Result<(), AggregateError> {
    if count == 0 {
        Err(AggregateError::InvalidCount {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validates that a probability is in [0, 1].
fn validate_probability(p: f64, field: &str) -> Result<(), AggregateError> {
    if !p.is_finite() || !(0.0..=1.0).contains(&p) {
        Err(AggregateError::InvalidProbability {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validates that a p-value is in [0, 1].
fn validate_pvalue(p: f64, field: &str) -> Result<(), AggregateError> {
    if !p.is_finite() || !(0.0..=1.0).contains(&p) {
        Err(AggregateError::InvalidPvalue {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validates that a statistic is finite.
fn validate_statistic(stat: f64, field: &str) -> Result<(), AggregateError> {
    if !stat.is_finite() {
        Err(AggregateError::InvalidStatistic {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Differential expression aggregate result.
///
/// Contains only computed statistics, not raw expression values or subject-level data.
/// This makes it impossible to reconstruct subject-level measurements from the aggregate.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DifferentialExpressionResult {
    /// Number of features tested (must be > 0).
    features_tested: u64,
    /// Number of features meeting significance threshold (must be ≤ features_tested).
    features_significant: u64,
    /// Mean log2 fold-change across significant features.
    mean_log2_fold_change: f64,
    /// Adjusted p-value threshold used (in [0, 1]).
    fdr_threshold: f64,
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
        validate_probability(fdr_threshold, "fdr_threshold")?;

        Ok(Self {
            features_tested,
            features_significant,
            mean_log2_fold_change,
            fdr_threshold,
        })
    }

    /// Returns the number of features tested.
    pub fn features_tested(&self) -> u64 {
        self.features_tested
    }

    /// Returns the number of significant features.
    pub fn features_significant(&self) -> u64 {
        self.features_significant
    }

    /// Returns the mean log2 fold-change.
    pub fn mean_log2_fold_change(&self) -> f64 {
        self.mean_log2_fold_change
    }

    /// Returns the FDR threshold used.
    pub fn fdr_threshold(&self) -> f64 {
        self.fdr_threshold
    }
}

/// Kaplan-Meier survival analysis aggregate result.
///
/// Contains only computed survival probabilities and test statistics,
/// not raw event times or subject-level data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KaplanMeierResult {
    /// Total number of subjects in analysis (must be > 0).
    total_subjects: u64,
    /// Total number of events observed (must be ≤ total_subjects).
    total_events: u64,
    /// Log-rank test statistic.
    log_rank_statistic: f64,
    /// P-value for log-rank test (in [0, 1]).
    log_rank_pvalue: f64,
    /// Median survival time (in study units), or None if not reached.
    median_survival: Option<f64>,
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
    /// - `median_survival` (if present) is not finite
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
        validate_pvalue(log_rank_pvalue, "log_rank_pvalue")?;
        if let Some(median) = median_survival {
            validate_statistic(median, "median_survival")?;
        }

        Ok(Self {
            total_subjects,
            total_events,
            log_rank_statistic,
            log_rank_pvalue,
            median_survival,
        })
    }

    /// Returns the total number of subjects.
    pub fn total_subjects(&self) -> u64 {
        self.total_subjects
    }

    /// Returns the total number of events.
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Returns the log-rank test statistic.
    pub fn log_rank_statistic(&self) -> f64 {
        self.log_rank_statistic
    }

    /// Returns the p-value for the log-rank test.
    pub fn log_rank_pvalue(&self) -> f64 {
        self.log_rank_pvalue
    }

    /// Returns the median survival time, if available.
    pub fn median_survival(&self) -> Option<f64> {
        self.median_survival
    }
}

/// Endotype prevalence aggregate result.
///
/// Contains only counts and proportions, not subject-level phenotypes or raw data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EndotypePrevalenceResult {
    /// Total subjects in the analyzed cohort (must be > 0).
    cohort_size: u64,
    /// Subjects with the target endotype (must be ≤ cohort_size).
    endotype_count: u64,
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

    /// Returns the cohort size.
    pub fn cohort_size(&self) -> u64 {
        self.cohort_size
    }

    /// Returns the count of subjects with the endotype.
    pub fn endotype_count(&self) -> u64 {
        self.endotype_count
    }

    /// Returns the proportion of subjects with the endotype.
    pub fn prevalence(&self) -> f64 {
        self.endotype_count as f64 / self.cohort_size as f64
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
        assert_eq!(r.features_tested(), 1000);
        assert_eq!(r.features_significant(), 50);
        assert_eq!(r.mean_log2_fold_change(), 1.5);
        assert_eq!(r.fdr_threshold(), 0.05);
    }

    #[test]
    fn differential_expression_zero_features_tested() {
        let result = DifferentialExpressionResult::new(0, 0, 1.0, 0.05);
        assert!(matches!(
            result,
            Err(AggregateError::InvalidCount {
                field
            }) if field == "features_tested"
        ));
    }

    #[test]
    fn differential_expression_significant_exceeds_tested() {
        let result = DifferentialExpressionResult::new(100, 101, 1.0, 0.05);
        assert!(matches!(result, Err(AggregateError::InvalidCount { .. })));
    }

    #[test]
    fn differential_expression_nan_fold_change() {
        let result = DifferentialExpressionResult::new(100, 50, f64::NAN, 0.05);
        assert!(matches!(
            result,
            Err(AggregateError::InvalidStatistic { .. })
        ));
    }

    #[test]
    fn differential_expression_invalid_fdr() {
        let result = DifferentialExpressionResult::new(100, 50, 1.0, 1.5);
        assert!(matches!(
            result,
            Err(AggregateError::InvalidProbability { .. })
        ));
    }

    #[test]
    fn kaplan_meier_valid() {
        let result = KaplanMeierResult::new(500, 150, 2.5, 0.01, Some(180.5));
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.total_subjects(), 500);
        assert_eq!(r.total_events(), 150);
        assert_eq!(r.log_rank_statistic(), 2.5);
        assert_eq!(r.log_rank_pvalue(), 0.01);
        assert_eq!(r.median_survival(), Some(180.5));
    }

    #[test]
    fn kaplan_meier_zero_subjects() {
        let result = KaplanMeierResult::new(0, 0, 1.0, 0.05, None);
        assert!(matches!(result, Err(AggregateError::InvalidCount { .. })));
    }

    #[test]
    fn kaplan_meier_events_exceed_subjects() {
        let result = KaplanMeierResult::new(100, 101, 1.0, 0.05, None);
        assert!(matches!(result, Err(AggregateError::InvalidCount { .. })));
    }

    #[test]
    fn kaplan_meier_invalid_pvalue() {
        let result = KaplanMeierResult::new(100, 50, 1.0, -0.1, None);
        assert!(matches!(result, Err(AggregateError::InvalidPvalue { .. })));
    }

    #[test]
    fn kaplan_meier_nan_median_survival() {
        let result = KaplanMeierResult::new(100, 50, 1.0, 0.05, Some(f64::NAN));
        assert!(matches!(
            result,
            Err(AggregateError::InvalidStatistic { .. })
        ));
    }

    #[test]
    fn endotype_prevalence_valid() {
        let result = EndotypePrevalenceResult::new(1000, 250);
        assert!(result.is_ok());
        let r = result.unwrap();
        assert_eq!(r.cohort_size(), 1000);
        assert_eq!(r.endotype_count(), 250);
        assert_eq!(r.prevalence(), 0.25);
    }

    #[test]
    fn endotype_prevalence_zero_cohort() {
        let result = EndotypePrevalenceResult::new(0, 0);
        assert!(matches!(result, Err(AggregateError::InvalidCount { .. })));
    }

    #[test]
    fn endotype_prevalence_count_exceeds_cohort() {
        let result = EndotypePrevalenceResult::new(100, 101);
        assert!(matches!(result, Err(AggregateError::InvalidCount { .. })));
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
        // The only way to create these types is to pass pre-computed aggregates.
        // This is enforced at compile time: only the constructor signatures shown
        // above are valid, and they take computed statistics, not raw data.

        let _de = DifferentialExpressionResult::new(1000, 50, 1.5, 0.05).unwrap();
        let _km = KaplanMeierResult::new(500, 150, 2.5, 0.01, None).unwrap();
        let _ep = EndotypePrevalenceResult::new(1000, 250).unwrap();
    }

    #[test]
    fn serde_bypass_kaplan_meier_events_exceed_subjects() {
        // VULNERABILITY: serde's default Deserialize bypasses validation.
        // This test documents that invalid JSON can be deserialized directly
        // without calling new(), bypassing invariant checks.
        //
        // The invariant "total_events ≤ total_subjects" is enforced in new(),
        // but derive(Deserialize) does not call new(). This allows construction
        // of invalid states from untrusted JSON.
        //
        // Fix: Implement custom Deserialize that calls new() instead of directly
        // constructing the struct.

        let invalid_json = r#"{
            "total_subjects": 100,
            "total_events": 200,
            "log_rank_statistic": 2.5,
            "log_rank_pvalue": 0.01,
            "median_survival": null
        }"#;

        // This should fail validation but currently succeeds
        let result: Result<KaplanMeierResult, _> = serde_json::from_str(invalid_json);
        match result {
            Ok(km) => {
                // VULNERABILITY CONFIRMED: We constructed an invalid state
                assert_eq!(km.total_subjects, 100);
                assert_eq!(km.total_events, 200);
                // This would never happen via new(), which enforces: total_events ≤ total_subjects
                panic!(
                    "VULNERABILITY: Deserialized invalid KaplanMeierResult without validation: \
                     total_events={} > total_subjects={}",
                    km.total_events, km.total_subjects
                );
            }
            Err(_) => {
                // If we get here, the vulnerability has been fixed with custom Deserialize
            }
        }
    }

    #[test]
    fn serde_bypass_endotype_prevalence_zero_cohort() {
        // VULNERABILITY: serde's default Deserialize bypasses validation.
        // The invariant "cohort_size > 0" is enforced in new(), but
        // derive(Deserialize) does not call new().

        let invalid_json = r#"{
            "cohort_size": 0,
            "endotype_count": 0
        }"#;

        let result: Result<EndotypePrevalenceResult, _> = serde_json::from_str(invalid_json);
        match result {
            Ok(ep) => {
                // VULNERABILITY CONFIRMED
                panic!(
                    "VULNERABILITY: Deserialized invalid EndotypePrevalenceResult: \
                     cohort_size={} (must be > 0)",
                    ep.cohort_size
                );
            }
            Err(_) => {
                // If we get here, the vulnerability has been fixed
            }
        }
    }

    #[test]
    fn serde_bypass_differential_expression_features_significant_exceeds_tested() {
        // VULNERABILITY: serde's default Deserialize bypasses validation.
        // The invariant "features_significant ≤ features_tested" is enforced in new(),
        // but derive(Deserialize) does not call new().

        let invalid_json = r#"{
            "features_tested": 100,
            "features_significant": 101,
            "mean_log2_fold_change": 1.5,
            "fdr_threshold": 0.05
        }"#;

        let result: Result<DifferentialExpressionResult, _> = serde_json::from_str(invalid_json);
        match result {
            Ok(de) => {
                // VULNERABILITY CONFIRMED
                panic!(
                    "VULNERABILITY: Deserialized invalid DifferentialExpressionResult: \
                     features_significant={} > features_tested={}",
                    de.features_significant, de.features_tested
                );
            }
            Err(_) => {
                // If we get here, the vulnerability has been fixed
            }
        }
    }

    #[test]
    fn serde_bypass_kaplan_meier_invalid_pvalue() {
        // VULNERABILITY: serde's default Deserialize bypasses validation.
        // The invariant "log_rank_pvalue in [0, 1]" is enforced in new(),
        // but derive(Deserialize) does not call new().

        let invalid_json = r#"{
            "total_subjects": 100,
            "total_events": 50,
            "log_rank_statistic": 2.5,
            "log_rank_pvalue": 1.5,
            "median_survival": null
        }"#;

        let result: Result<KaplanMeierResult, _> = serde_json::from_str(invalid_json);
        match result {
            Ok(km) => {
                // VULNERABILITY CONFIRMED
                panic!(
                    "VULNERABILITY: Deserialized invalid KaplanMeierResult: \
                     log_rank_pvalue={} (must be in [0, 1])",
                    km.log_rank_pvalue
                );
            }
            Err(_) => {
                // If we get here, the vulnerability has been fixed
            }
        }
    }
}
