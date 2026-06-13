//! Federation protocol types: analysis-plan and aggregate-result wire types that enable
//! federated analysis while maintaining the invariant that no subject-level datum leaves a node.

/// Analysis plan types and invariants.
pub mod analysis_plan;

/// Aggregate result types: computed statistics only, never raw subject data.
pub mod aggregate_result;

pub use aggregate_result::{
    AggregateError, DifferentialExpressionResult, EndotypePrevalenceResult, KaplanMeierResult,
};
pub use analysis_plan::{AnalysisError, AnalysisPlan, CohortSelector, EstimatorVariant};
