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
///
/// Intentionally **not** `Ord`/`PartialOrd`: a lexicographic order would not match
/// the type's only meaningful relation — [`SchemaVersion::is_compatible_with`] — so
/// exposing `<`/`>=` would invite using ordering as a stand-in for compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// Whether `self` is wire-compatible with `other`.
    ///
    /// At `>= 1.0`, compatibility is **major-match** (additive minors are safe). While
    /// either side is **pre-1.0** (`major == 0`), every release may break per SemVer,
    /// so compatibility requires an **exact** version match. This mirrors
    /// `docs/cdm-versioning.md` and prevents declaring two divergent `0.x` schemas
    /// compatible.
    ///
    /// ```
    /// use septicomics_cdm::version::SchemaVersion;
    ///
    /// // >= 1.0: same major is compatible, different major is not.
    /// let a = SchemaVersion::new(1, 2, 0);
    /// assert!(a.is_compatible_with(SchemaVersion::new(1, 5, 3)));
    /// assert!(!a.is_compatible_with(SchemaVersion::new(2, 0, 0)));
    ///
    /// // pre-1.0: only an exact match is compatible.
    /// let z = SchemaVersion::new(0, 1, 0);
    /// assert!(z.is_compatible_with(SchemaVersion::new(0, 1, 0)));
    /// assert!(!z.is_compatible_with(SchemaVersion::new(0, 2, 0)));
    /// ```
    #[must_use]
    pub const fn is_compatible_with(self, other: SchemaVersion) -> bool {
        if self.major == 0 || other.major == 0 {
            // Pre-1.0: every release may break, so require an exact match.
            self.major == other.major && self.minor == other.minor && self.patch == other.patch
        } else {
            self.major == other.major
        }
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
    fn pre_1_0_requires_exact_match() {
        // CDM_SCHEMA_VERSION is 0.1.0; during 0.x only an exact match is compatible.
        assert!(CDM_SCHEMA_VERSION.is_compatible_with(SchemaVersion::new(0, 1, 0)));
        assert!(!CDM_SCHEMA_VERSION.is_compatible_with(SchemaVersion::new(0, 9, 9)));
        assert!(!CDM_SCHEMA_VERSION.is_compatible_with(SchemaVersion::new(1, 0, 0)));
    }

    #[test]
    fn post_1_0_is_major_match() {
        let v = SchemaVersion::new(1, 2, 0);
        assert!(v.is_compatible_with(SchemaVersion::new(1, 9, 5)));
        assert!(!v.is_compatible_with(SchemaVersion::new(2, 0, 0)));
        // A pre-1.0 peer is never compatible with a stable one.
        assert!(!v.is_compatible_with(SchemaVersion::new(0, 9, 0)));
    }

    #[test]
    fn schema_version_tracks_crate_version() {
        // Drift guard on the numeric components, robust to any pre-release/build
        // suffix on the crate version (which SchemaVersion does not model).
        let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();
        assert_eq!(CDM_SCHEMA_VERSION, SchemaVersion::new(major, minor, patch));
    }

    #[test]
    fn display_is_dotted() {
        assert_eq!(SchemaVersion::new(1, 2, 3).to_string(), "1.2.3");
    }
}
