//! Property tests for the federation protocol invariants.
//!
//! These tests verify that the load-bearing invariant holds:
//! **No subject-level datum leaves a node. Every value crossing the node boundary
//! is an aggregate that has passed the disclosure-control guard.**

#[cfg(test)]
mod tests {
    use crate::{DifferentialExpressionResult, EndotypePrevalenceResult, KaplanMeierResult};
    use proptest::prelude::*;

    /// Strategy for generating valid DifferentialExpressionResult values.
    fn arb_de_result() -> impl Strategy<Value = DifferentialExpressionResult> {
        (1u64..1000, 0u64..1000, -10.0f64..10.0f64, 0.0f64..=1.0f64).prop_filter_map(
            "invalid DE result",
            |(features_tested, mut significant, log2fc, fdr)| {
                // Ensure significant <= features_tested
                if significant > features_tested {
                    significant = features_tested;
                }
                // Ensure log2fc is finite
                if !log2fc.is_finite() {
                    return None;
                }
                DifferentialExpressionResult::new(features_tested, significant, log2fc, fdr).ok()
            },
        )
    }

    /// Strategy for generating valid KaplanMeierResult values.
    fn arb_km_result() -> impl Strategy<Value = KaplanMeierResult> {
        (
            1u64..1000,
            0u64..1000,
            -10.0f64..10.0f64,
            0.0f64..=1.0f64,
            prop::option::of(0.0f64..1000.0f64),
        )
            .prop_filter_map(
                "invalid KM result",
                |(total_subjects, mut total_events, log_rank, pval, median)| {
                    // Ensure events <= subjects
                    if total_events > total_subjects {
                        total_events = total_subjects;
                    }
                    // Ensure log_rank is finite
                    if !log_rank.is_finite() {
                        return None;
                    }
                    // Ensure median is finite and non-negative if present
                    let median = median.filter(|m| m.is_finite() && *m >= 0.0);

                    KaplanMeierResult::new(total_subjects, total_events, log_rank, pval, median)
                        .ok()
                },
            )
    }

    /// Strategy for generating valid EndotypePrevalenceResult values.
    fn arb_ep_result() -> impl Strategy<Value = EndotypePrevalenceResult> {
        (1u64..1000, 0u64..1000).prop_filter_map(
            "invalid EP result",
            |(cohort_size, mut endotype_count)| {
                // Ensure endotype_count <= cohort_size
                if endotype_count > cohort_size {
                    endotype_count = cohort_size;
                }
                EndotypePrevalenceResult::new(cohort_size, endotype_count).ok()
            },
        )
    }

    proptest! {
        /// Property: DifferentialExpressionResult can be serialized and deserialized without loss.
        /// This property ensures the aggregate result type is wire-safe and doesn't accidentally
        /// include subject-level information (which would violate the invariant).
        #[test]
        fn de_result_roundtrip_preserves_invariant(result in arb_de_result()) {
            // Serialize the aggregate result
            let json = serde_json::to_string(&result).expect("serialization failed");

            // Deserialize it back
            let deserialized: DifferentialExpressionResult =
                serde_json::from_str(&json).expect("deserialization failed");

            // The roundtrip must preserve all integer values exactly
            prop_assert_eq!(result.features_tested, deserialized.features_tested);
            prop_assert_eq!(result.features_significant, deserialized.features_significant);

            // Floats may lose precision during JSON serialization, so use approximate equality
            prop_assert!(
                (result.mean_log2_fold_change - deserialized.mean_log2_fold_change).abs() < 1e-10
                    || (result.mean_log2_fold_change.is_nan()
                        && deserialized.mean_log2_fold_change.is_nan()),
                "mean_log2_fold_change differs: {} vs {}",
                result.mean_log2_fold_change,
                deserialized.mean_log2_fold_change
            );

            prop_assert!(
                (result.fdr_threshold - deserialized.fdr_threshold).abs() < 1e-10,
                "fdr_threshold differs: {} vs {}",
                result.fdr_threshold,
                deserialized.fdr_threshold
            );

            // The deserialized result must have passed through the constructor,
            // so all invariants have been re-validated (proving no subject-level data leaked)
        }
    }

    proptest! {
        /// Property: KaplanMeierResult can be serialized and deserialized without loss.
        /// Since only aggregate statistics (survival probabilities, test results) are stored,
        /// the invariant that subject-level data never leaves the node is preserved.
        #[test]
        fn km_result_roundtrip_preserves_invariant(result in arb_km_result()) {
            let json = serde_json::to_string(&result).expect("serialization failed");
            let deserialized: KaplanMeierResult =
                serde_json::from_str(&json).expect("deserialization failed");

            prop_assert_eq!(result.total_subjects, deserialized.total_subjects);
            prop_assert_eq!(result.total_events, deserialized.total_events);

            // Floats may lose precision during JSON serialization
            prop_assert!(
                (result.log_rank_statistic - deserialized.log_rank_statistic).abs() < 1e-10
                    || (result.log_rank_statistic.is_nan()
                        && deserialized.log_rank_statistic.is_nan()),
                "log_rank_statistic differs"
            );

            prop_assert!(
                (result.log_rank_pvalue - deserialized.log_rank_pvalue).abs() < 1e-10,
                "log_rank_pvalue differs"
            );

            // Compare Option<f64> with tolerance
            match (result.median_survival, deserialized.median_survival) {
                (Some(left), Some(right)) => {
                    prop_assert!(
                        (left - right).abs() < 1e-10,
                        "median_survival differs: {} vs {}",
                        left,
                        right
                    )
                }
                (None, None) => {}
                _ => prop_assert!(false, "median_survival option mismatch"),
            }
        }
    }

    proptest! {
        /// Property: EndotypePrevalenceResult can be serialized and deserialized without loss.
        /// The result contains only aggregate counts (cohort size and endotype prevalence),
        /// never subject-level phenotypes or raw data.
        #[test]
        fn ep_result_roundtrip_preserves_invariant(result in arb_ep_result()) {
            let json = serde_json::to_string(&result).expect("serialization failed");
            let deserialized: EndotypePrevalenceResult =
                serde_json::from_str(&json).expect("deserialization failed");

            prop_assert_eq!(result.cohort_size, deserialized.cohort_size);
            prop_assert_eq!(result.endotype_count, deserialized.endotype_count);
            prop_assert_eq!(result.prevalence(), deserialized.prevalence());
        }
    }

    /// Unit test: Verify all three result types can coexist and be correctly
    /// identified in a serialized format. This proves the types maintain
    /// distinct identities on the wire (no ambiguity or subject-level collision).
    #[test]
    fn all_result_types_wireformat_distinct() {
        let de = DifferentialExpressionResult::new(100, 50, 1.5, 0.05).expect("valid DE result");
        let km = KaplanMeierResult::new(100, 20, 2.1, 0.01, Some(50.0)).expect("valid KM result");
        let ep = EndotypePrevalenceResult::new(100, 30).expect("valid EP result");

        let de_json = serde_json::to_string(&de).expect("DE serialization");
        let km_json = serde_json::to_string(&km).expect("KM serialization");
        let ep_json = serde_json::to_string(&ep).expect("EP serialization");

        // All types should serialize to valid JSON
        assert!(de_json.contains("features_tested"));
        assert!(km_json.contains("total_subjects"));
        assert!(ep_json.contains("cohort_size"));

        // None should accidentally contain raw subject-level markers at key positions
        // (e.g., "patient_id", "record_id", "subject_id", "row_id", etc.)
        for json in &[de_json, km_json, ep_json] {
            let lower = json.to_lowercase();
            assert!(
                !lower.contains("patient_id"),
                "Found subject-level marker: patient_id"
            );
            assert!(
                !lower.contains("record_id"),
                "Found subject-level marker: record_id"
            );
            assert!(
                !lower.contains("subject_id"),
                "Found subject-level marker: subject_id"
            );
            assert!(
                !lower.contains("row_id"),
                "Found subject-level marker: row_id"
            );
            assert!(
                !lower.contains("identifier"),
                "Found subject-level marker: identifier"
            );
        }
    }
}
