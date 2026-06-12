//! CDM schema versioning.
//!
//! The CDM is a contract between independently-operated nodes, so its version is
//! machine-readable and its compatibility rule is explicit: **two schema versions
//! are wire-compatible iff their major components match** (`ARCHITECTURE.md` §6).
//! Nodes advertise the CDM major they speak and the orchestrator refuses
//! incompatible fan-out rather than guessing. The full policy — including the
//! pre-1.0 caveat — lives in `docs/cdm-versioning.md`.

use core::fmt;

/// A semantic version of the CDM schema (`major.minor.patch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaVersion {
    /// Breaking schema changes. A bump here is a coordinated node-upgrade event.
    pub major: u32,
    /// Backward-compatible additions.
    pub minor: u32,
    /// Backward-compatible fixes.
    pub patch: u32,
}

impl SchemaVersion {
    /// Construct a schema version.
    #[must_use]
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        SchemaVersion {
            major,
            minor,
            patch,
        }
    }

    /// Whether `self` is wire-compatible with `other` — i.e. their majors match.
    ///
    /// ```
    /// use septicomics_cdm::version::SchemaVersion;
    ///
    /// let a = SchemaVersion::new(1, 2, 0);
    /// let b = SchemaVersion::new(1, 5, 3);
    /// let c = SchemaVersion::new(2, 0, 0);
    /// assert!(a.is_compatible_with(b));
    /// assert!(!a.is_compatible_with(c));
    /// ```
    #[must_use]
    pub const fn is_compatible_with(self, other: SchemaVersion) -> bool {
        self.major == other.major
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// The CDM schema version this build implements.
///
/// Kept in lock-step with the crate version (asserted by a test), so the published
/// SemVer of `septicomics-cdm` is the schema version nodes negotiate on.
pub const CDM_SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(0, 1, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatibility_is_by_major() {
        assert!(CDM_SCHEMA_VERSION.is_compatible_with(SchemaVersion::new(0, 9, 9)));
        assert!(!CDM_SCHEMA_VERSION.is_compatible_with(SchemaVersion::new(1, 0, 0)));
    }

    #[test]
    fn schema_version_tracks_crate_version() {
        // Drift guard: the embedded schema version must equal the crate's SemVer.
        assert_eq!(CDM_SCHEMA_VERSION.to_string(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn display_is_dotted() {
        assert_eq!(SchemaVersion::new(1, 2, 3).to_string(), "1.2.3");
    }
}
