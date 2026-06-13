//! Federation protocol types: analysis-plan and aggregate-result wire types that enable
//! federated analysis while maintaining the invariant that no subject-level datum leaves a node.

/// Analysis plan types and invariants.
pub mod analysis_plan;

/// Aggregate result types: computed statistics only, never raw subject data.
pub mod aggregate_result;

/// Node capability advertisement: what versions and scopes each node supports.
pub mod node_capability;

/// Version negotiation: orchestrator compatibility checking against node capabilities.
pub mod version_negotiation;

pub use aggregate_result::{
    AggregateError, DifferentialExpressionResult, EndotypePrevalenceResult, KaplanMeierResult,
};
pub use analysis_plan::{AnalysisError, AnalysisPlan, CohortSelector, EstimatorVariant};
pub use node_capability::{CapabilityError, ConsentScope, NodeCapability};
pub use version_negotiation::{NegotiationError, negotiate};
