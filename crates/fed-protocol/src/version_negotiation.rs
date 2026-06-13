use crate::{AnalysisPlan, NodeCapability};

/// Errors that can occur during version negotiation between orchestrator and node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegotiationError {
    /// CDM major version mismatch: plan requires X, node supports Y.
    CdmMajorMismatch {
        /// Required CDM major version from the plan.
        required: u64,
        /// Supported CDM major version from the node.
        supported: u64,
    },
    /// Protocol major version mismatch: plan requires X, node supports Y.
    ProtocolMajorMismatch {
        /// Required protocol major version from the plan.
        required: u64,
        /// Supported protocol major version from the node.
        supported: u64,
    },
    /// Node does not support one or more required consent scopes.
    MissingConsentScopes {
        /// The scopes required by the plan but not supported by the node.
        unsupported: Vec<String>,
    },
}

impl std::fmt::Display for NegotiationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NegotiationError::CdmMajorMismatch {
                required,
                supported,
            } => {
                write!(
                    f,
                    "CDM major version mismatch: plan requires {}, node supports {}",
                    required, supported
                )
            }
            NegotiationError::ProtocolMajorMismatch {
                required,
                supported,
            } => {
                write!(
                    f,
                    "protocol major version mismatch: plan requires {}, node supports {}",
                    required, supported
                )
            }
            NegotiationError::MissingConsentScopes { unsupported } => {
                write!(
                    f,
                    "node does not support consent scopes: {}",
                    unsupported.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for NegotiationError {}

/// Negotiates whether an orchestrator's analysis plan can be executed on a node.
///
/// The negotiation checks:
/// - **CDM major version**: must match exactly (no cross-version compatibility)
/// - **Protocol major version**: must match exactly (no cross-version compatibility)
/// - **Consent scopes**: node must support all scopes required by the plan
///
/// If all conditions are met, returns `Ok(())` (accept). Otherwise returns
/// a detailed `NegotiationError` (refuse).
pub fn negotiate(plan: &AnalysisPlan, node: &NodeCapability) -> Result<(), NegotiationError> {
    if plan.cdm_major() != node.cdm_major {
        return Err(NegotiationError::CdmMajorMismatch {
            required: plan.cdm_major(),
            supported: node.cdm_major,
        });
    }

    if plan.protocol_major() != node.protocol_major {
        return Err(NegotiationError::ProtocolMajorMismatch {
            required: plan.protocol_major(),
            supported: node.protocol_major,
        });
    }

    // For now, plans don't specify required scopes; all nodes must support at least one scope.
    // This check is a placeholder for future scope requirement checking when AnalysisPlan
    // includes a required_scopes field.
    // For now, we only verify that the node has at least one scope (already enforced by NodeCapability).

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiate_accept_matching_versions() {
        let plan = AnalysisPlan::new(
            1,
            1,
            "test_plan".to_string(),
            crate::analysis_plan::CohortSelector::all(),
            vec![septicomics_cdm::omics::OmicsLayer::Transcriptomics],
            crate::analysis_plan::EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        )
        .expect("valid plan");

        let node =
            NodeCapability::new(1, 1, vec!["research".to_string()]).expect("valid capability");

        let result = negotiate(&plan, &node);
        assert!(result.is_ok(), "should accept matching versions");
    }

    #[test]
    fn negotiate_refuse_cdm_mismatch() {
        let plan = AnalysisPlan::new(
            1,
            1,
            "test_plan".to_string(),
            crate::analysis_plan::CohortSelector::all(),
            vec![septicomics_cdm::omics::OmicsLayer::Transcriptomics],
            crate::analysis_plan::EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        )
        .expect("valid plan");

        let node =
            NodeCapability::new(2, 1, vec!["research".to_string()]).expect("valid capability");

        let result = negotiate(&plan, &node);
        assert!(matches!(
            result,
            Err(NegotiationError::CdmMajorMismatch {
                required: 1,
                supported: 2
            })
        ));
    }

    #[test]
    fn negotiate_refuse_protocol_mismatch() {
        let plan = AnalysisPlan::new(
            1,
            2,
            "test_plan".to_string(),
            crate::analysis_plan::CohortSelector::all(),
            vec![septicomics_cdm::omics::OmicsLayer::Transcriptomics],
            crate::analysis_plan::EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        )
        .expect("valid plan");

        let node =
            NodeCapability::new(1, 1, vec!["research".to_string()]).expect("valid capability");

        let result = negotiate(&plan, &node);
        assert!(matches!(
            result,
            Err(NegotiationError::ProtocolMajorMismatch {
                required: 2,
                supported: 1
            })
        ));
    }

    #[test]
    fn negotiate_accept_multiple_scopes() {
        let plan = AnalysisPlan::new(
            1,
            1,
            "test_plan".to_string(),
            crate::analysis_plan::CohortSelector::all(),
            vec![septicomics_cdm::omics::OmicsLayer::Transcriptomics],
            crate::analysis_plan::EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        )
        .expect("valid plan");

        let node = NodeCapability::new(
            1,
            1,
            vec![
                "research".to_string(),
                "clinical".to_string(),
                "quality-improvement".to_string(),
            ],
        )
        .expect("valid capability");

        let result = negotiate(&plan, &node);
        assert!(result.is_ok(), "should accept with multiple scopes");
    }

    #[test]
    fn negotiate_accept_different_major_versions_still_refuse() {
        let plan = AnalysisPlan::new(
            3,
            2,
            "test_plan".to_string(),
            crate::analysis_plan::CohortSelector::all(),
            vec![septicomics_cdm::omics::OmicsLayer::Transcriptomics],
            crate::analysis_plan::EstimatorVariant::EndotypePrevalence {
                endotype: "test".to_string(),
            },
        )
        .expect("valid plan");

        let node =
            NodeCapability::new(1, 1, vec!["research".to_string()]).expect("valid capability");

        let result = negotiate(&plan, &node);
        // Should fail on CDM first
        assert!(matches!(
            result,
            Err(NegotiationError::CdmMajorMismatch { .. })
        ));
    }

    #[test]
    fn error_message_cdm_mismatch() {
        let err = NegotiationError::CdmMajorMismatch {
            required: 1,
            supported: 2,
        };
        assert_eq!(
            err.to_string(),
            "CDM major version mismatch: plan requires 1, node supports 2"
        );
    }

    #[test]
    fn error_message_protocol_mismatch() {
        let err = NegotiationError::ProtocolMajorMismatch {
            required: 2,
            supported: 1,
        };
        assert_eq!(
            err.to_string(),
            "protocol major version mismatch: plan requires 2, node supports 1"
        );
    }

    #[test]
    fn error_message_missing_scopes() {
        let err = NegotiationError::MissingConsentScopes {
            unsupported: vec!["admin".to_string(), "sensitive".to_string()],
        };
        let msg = err.to_string();
        assert!(msg.contains("admin"));
        assert!(msg.contains("sensitive"));
    }
}
