use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeSet;

/// Errors that can occur when constructing node capabilities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityError {
    /// CDM major version is zero.
    ZeroCdmMajor,
    /// Protocol major version is zero.
    ZeroProtocolMajor,
    /// A consent scope is empty or whitespace-only.
    EmptyConsentScope {
        /// Index of the invalid scope.
        index: usize,
    },
    /// Consent scopes list is empty.
    EmptyConsentScopes,
}

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityError::ZeroCdmMajor => write!(f, "CDM major version must be > 0"),
            CapabilityError::ZeroProtocolMajor => write!(f, "protocol major version must be > 0"),
            CapabilityError::EmptyConsentScope { index } => {
                write!(f, "consent scope at index {} is empty or whitespace", index)
            }
            CapabilityError::EmptyConsentScopes => {
                write!(f, "at least one consent scope is required")
            }
        }
    }
}

impl std::error::Error for CapabilityError {}

/// Represents a consent or authorization scope that a node supports.
///
/// Consent scopes are identifiers (e.g., "research", "clinical", "quality-improvement")
/// that the node advertises it can handle. The orchestrator uses these to determine
/// whether a node can execute a particular analysis plan.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsentScope(String);

impl ConsentScope {
    /// Creates a new consent scope.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityError::EmptyConsentScope`] if the scope is empty or whitespace-only.
    pub fn new(scope: impl Into<String>) -> Result<Self, CapabilityError> {
        let scope = scope.into();
        if scope.trim().is_empty() {
            return Err(CapabilityError::EmptyConsentScope { index: 0 });
        }
        Ok(ConsentScope(scope))
    }

    /// Returns the scope as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ConsentScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ConsentScope> for String {
    fn from(scope: ConsentScope) -> Self {
        scope.0
    }
}

/// Capabilities that a node advertises to the federation.
///
/// A node declares which versions of the CDM and federation protocol it speaks,
/// along with which consent scopes it can handle. The orchestrator uses this
/// to determine whether to send a plan to a node:
/// - CDM major version must match the plan's CDM version
/// - Protocol major version must match the plan's protocol version
/// - Node must support all consent scopes required by the plan
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct NodeCapability {
    /// CDM major version this node speaks (must be > 0).
    pub cdm_major: u64,
    /// Federation protocol major version this node speaks (must be > 0).
    pub protocol_major: u64,
    /// Consent scopes this node can handle (must be non-empty).
    pub consent_scopes: BTreeSet<ConsentScope>,
}

impl NodeCapability {
    /// Creates a new node capability advertisement.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityError`] if:
    /// - `cdm_major` is zero
    /// - `protocol_major` is zero
    /// - `consent_scopes` is empty
    /// - any consent scope is empty or whitespace-only
    pub fn new(
        cdm_major: u64,
        protocol_major: u64,
        consent_scopes: Vec<String>,
    ) -> Result<Self, CapabilityError> {
        if cdm_major == 0 {
            return Err(CapabilityError::ZeroCdmMajor);
        }
        if protocol_major == 0 {
            return Err(CapabilityError::ZeroProtocolMajor);
        }
        if consent_scopes.is_empty() {
            return Err(CapabilityError::EmptyConsentScopes);
        }

        let mut scopes = BTreeSet::new();
        for (idx, scope) in consent_scopes.iter().enumerate() {
            if scope.trim().is_empty() {
                return Err(CapabilityError::EmptyConsentScope { index: idx });
            }
            scopes.insert(ConsentScope::new(scope.clone())?);
        }

        Ok(Self {
            cdm_major,
            protocol_major,
            consent_scopes: scopes,
        })
    }

    /// Returns true if this node supports a specific consent scope.
    pub fn supports_scope(&self, scope: &ConsentScope) -> bool {
        self.consent_scopes.contains(scope)
    }

    /// Returns true if this node supports all required consent scopes.
    pub fn supports_all_scopes(&self, required: &[ConsentScope]) -> bool {
        required.iter().all(|scope| self.supports_scope(scope))
    }
}

impl<'de> serde::Deserialize<'de> for NodeCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct Raw {
            cdm_major: u64,
            protocol_major: u64,
            consent_scopes: Vec<String>,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.cdm_major, raw.protocol_major, raw.consent_scopes)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consent_scope_valid() {
        let scope = ConsentScope::new("research");
        assert!(scope.is_ok());
        assert_eq!(scope.unwrap().as_str(), "research");
    }

    #[test]
    fn consent_scope_empty() {
        let scope = ConsentScope::new("");
        assert!(scope.is_err());
    }

    #[test]
    fn consent_scope_whitespace() {
        let scope = ConsentScope::new("   ");
        assert!(scope.is_err());
    }

    #[test]
    fn node_capability_valid() {
        let result = NodeCapability::new(1, 1, vec!["research".to_string()]);
        assert!(result.is_ok());
        let cap = result.unwrap();
        assert_eq!(cap.cdm_major, 1);
        assert_eq!(cap.protocol_major, 1);
        assert_eq!(cap.consent_scopes.len(), 1);
    }

    #[test]
    fn node_capability_multiple_scopes() {
        let result = NodeCapability::new(
            1,
            1,
            vec![
                "research".to_string(),
                "clinical".to_string(),
                "quality-improvement".to_string(),
            ],
        );
        assert!(result.is_ok());
        let cap = result.unwrap();
        assert_eq!(cap.consent_scopes.len(), 3);
    }

    #[test]
    fn node_capability_zero_cdm_major() {
        let result = NodeCapability::new(0, 1, vec!["research".to_string()]);
        assert!(matches!(result, Err(CapabilityError::ZeroCdmMajor)));
    }

    #[test]
    fn node_capability_zero_protocol_major() {
        let result = NodeCapability::new(1, 0, vec!["research".to_string()]);
        assert!(matches!(result, Err(CapabilityError::ZeroProtocolMajor)));
    }

    #[test]
    fn node_capability_empty_scopes() {
        let result = NodeCapability::new(1, 1, vec![]);
        assert!(matches!(result, Err(CapabilityError::EmptyConsentScopes)));
    }

    #[test]
    fn node_capability_empty_scope_in_list() {
        let result = NodeCapability::new(1, 1, vec!["research".to_string(), "".to_string()]);
        assert!(matches!(
            result,
            Err(CapabilityError::EmptyConsentScope { index: 1 })
        ));
    }

    #[test]
    fn node_capability_supports_scope() {
        let cap = NodeCapability::new(1, 1, vec!["research".to_string(), "clinical".to_string()])
            .unwrap();
        let research = ConsentScope::new("research").unwrap();
        let admin = ConsentScope::new("admin").unwrap();
        assert!(cap.supports_scope(&research));
        assert!(!cap.supports_scope(&admin));
    }

    #[test]
    fn node_capability_supports_all_scopes() {
        let cap = NodeCapability::new(1, 1, vec!["research".to_string(), "clinical".to_string()])
            .unwrap();
        let required_match = vec![
            ConsentScope::new("research").unwrap(),
            ConsentScope::new("clinical").unwrap(),
        ];
        let required_subset = vec![ConsentScope::new("research").unwrap()];
        let required_mismatch = vec![ConsentScope::new("admin").unwrap()];
        assert!(cap.supports_all_scopes(&required_match));
        assert!(cap.supports_all_scopes(&required_subset));
        assert!(!cap.supports_all_scopes(&required_mismatch));
    }

    #[test]
    fn serde_roundtrip() {
        let cap = NodeCapability::new(
            2,
            1,
            vec![
                "research".to_string(),
                "clinical".to_string(),
                "quality-improvement".to_string(),
            ],
        )
        .unwrap();
        let json = serde_json::to_string(&cap).expect("serialization failed");
        let deserialized: NodeCapability =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(cap, deserialized);
    }

    #[test]
    fn deserialization_enforces_invariants_zero_cdm() {
        let invalid_json =
            r#"{"cdm_major": 0, "protocol_major": 1, "consent_scopes": ["research"]}"#;
        let result: Result<NodeCapability, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject zero CDM major"
        );
    }

    #[test]
    fn deserialization_enforces_invariants_empty_scopes() {
        let invalid_json = r#"{"cdm_major": 1, "protocol_major": 1, "consent_scopes": []}"#;
        let result: Result<NodeCapability, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject empty consent scopes"
        );
    }

    #[test]
    fn deserialization_enforces_invariants_invalid_scope() {
        let invalid_json = r#"{"cdm_major": 1, "protocol_major": 1, "consent_scopes": ["  "]}"#;
        let result: Result<NodeCapability, _> = serde_json::from_str(invalid_json);
        assert!(
            result.is_err(),
            "deserialization should reject whitespace-only consent scope"
        );
    }

    #[test]
    fn consent_scopes_are_sorted() {
        let cap = NodeCapability::new(
            1,
            1,
            vec!["zebra".to_string(), "alpha".to_string(), "beta".to_string()],
        )
        .unwrap();
        let scopes: Vec<&str> = cap.consent_scopes.iter().map(|s| s.as_str()).collect();
        assert_eq!(scopes, vec!["alpha", "beta", "zebra"]);
    }
}
