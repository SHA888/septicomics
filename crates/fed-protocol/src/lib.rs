//! Federation protocol types: analysis-plan and aggregate-result wire types that enable
//! federated analysis while maintaining the invariant that no subject-level datum leaves a node.

/// Analysis plan types and invariants.
pub mod analysis_plan;

pub use analysis_plan::AnalysisPlan;
