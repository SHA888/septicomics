//! Inflammatory endotypes and clinical phenotypes.
//!
//! These are exhaustive, closed enumerations (not `#[non_exhaustive]`): the set of
//! recognized endotypes/phenotypes is part of the CDM contract, so adding one is a
//! major version bump and a coordinated node-upgrade event. Absence of a
//! classification is modeled by an `Option` at the use site, never by an "unknown"
//! variant — that keeps every `match` meaningful and exhaustive.
//!
//! The vocabularies follow established sepsis literature:
//! - [`InflammatoryEndotype`] — the hyper-/hypo-inflammatory axis.
//! - [`SepsisResponseSignature`] — transcriptomic SRS endotypes (Davenport et al.).
//! - [`ClinicalPhenotype`] — the α/β/γ/δ clinical phenotypes (Seymour et al.).

use core::fmt;

use crate::error::{CdmError, Result};

/// The inflammatory endotype axis: a subject's molecular inflammatory state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InflammatoryEndotype {
    /// Heightened inflammation; typically higher shock burden and mortality.
    Hyperinflammatory,
    /// Lower inflammation.
    Hypoinflammatory,
}

impl InflammatoryEndotype {
    /// Every endotype defined by this CDM version, in declaration order.
    pub const ALL: [InflammatoryEndotype; 2] = [
        InflammatoryEndotype::Hyperinflammatory,
        InflammatoryEndotype::Hypoinflammatory,
    ];

    /// The stable, machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            InflammatoryEndotype::Hyperinflammatory => "hyperinflammatory",
            InflammatoryEndotype::Hypoinflammatory => "hypoinflammatory",
        }
    }

    /// Parse a wire identifier (as produced by [`InflammatoryEndotype::as_str`]).
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::UnknownVariant`] if `value` matches no known endotype.
    pub fn from_wire(value: &str) -> Result<Self> {
        InflammatoryEndotype::ALL
            .into_iter()
            .find(|e| e.as_str() == value)
            .ok_or_else(|| CdmError::UnknownVariant {
                kind: "InflammatoryEndotype",
                value: value.to_owned(),
            })
    }
}

impl fmt::Display for InflammatoryEndotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Transcriptomic Sepsis Response Signature endotypes (Davenport et al.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SepsisResponseSignature {
    /// SRS1: relatively immunosuppressed profile; associated with higher mortality.
    Srs1,
    /// SRS2: relatively immunocompetent profile.
    Srs2,
}

impl SepsisResponseSignature {
    /// Every SRS endotype defined by this CDM version, in declaration order.
    pub const ALL: [SepsisResponseSignature; 2] =
        [SepsisResponseSignature::Srs1, SepsisResponseSignature::Srs2];

    /// The stable, machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            SepsisResponseSignature::Srs1 => "srs1",
            SepsisResponseSignature::Srs2 => "srs2",
        }
    }

    /// Parse a wire identifier (as produced by [`SepsisResponseSignature::as_str`]).
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::UnknownVariant`] if `value` matches no known SRS endotype.
    pub fn from_wire(value: &str) -> Result<Self> {
        SepsisResponseSignature::ALL
            .into_iter()
            .find(|e| e.as_str() == value)
            .ok_or_else(|| CdmError::UnknownVariant {
                kind: "SepsisResponseSignature",
                value: value.to_owned(),
            })
    }
}

impl fmt::Display for SepsisResponseSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Clinical sepsis phenotypes α/β/γ/δ (Seymour et al.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClinicalPhenotype {
    /// α (alpha): fewest abnormalities, lowest vasopressor use.
    Alpha,
    /// β (beta): older, more chronic illness and renal dysfunction.
    Beta,
    /// γ (gamma): more inflammation and pulmonary dysfunction.
    Gamma,
    /// δ (delta): more hepatic dysfunction and shock; highest mortality.
    Delta,
}

impl ClinicalPhenotype {
    /// Every clinical phenotype defined by this CDM version, in declaration order.
    pub const ALL: [ClinicalPhenotype; 4] = [
        ClinicalPhenotype::Alpha,
        ClinicalPhenotype::Beta,
        ClinicalPhenotype::Gamma,
        ClinicalPhenotype::Delta,
    ];

    /// The stable, machine-readable identifier.
    pub const fn as_str(self) -> &'static str {
        match self {
            ClinicalPhenotype::Alpha => "alpha",
            ClinicalPhenotype::Beta => "beta",
            ClinicalPhenotype::Gamma => "gamma",
            ClinicalPhenotype::Delta => "delta",
        }
    }

    /// Parse a wire identifier (as produced by [`ClinicalPhenotype::as_str`]).
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::UnknownVariant`] if `value` matches no known phenotype.
    pub fn from_wire(value: &str) -> Result<Self> {
        ClinicalPhenotype::ALL
            .into_iter()
            .find(|p| p.as_str() == value)
            .ok_or_else(|| CdmError::UnknownVariant {
                kind: "ClinicalPhenotype",
                value: value.to_owned(),
            })
    }
}

impl fmt::Display for ClinicalPhenotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inflammatory_endotype_roundtrips() {
        for e in InflammatoryEndotype::ALL {
            assert_eq!(InflammatoryEndotype::from_wire(e.as_str()), Ok(e));
        }
        assert!(InflammatoryEndotype::from_wire("mixed").is_err());
    }

    #[test]
    fn srs_roundtrips() {
        for e in SepsisResponseSignature::ALL {
            assert_eq!(SepsisResponseSignature::from_wire(e.as_str()), Ok(e));
        }
        assert!(SepsisResponseSignature::from_wire("srs3").is_err());
    }

    #[test]
    fn clinical_phenotype_roundtrips() {
        for p in ClinicalPhenotype::ALL {
            assert_eq!(ClinicalPhenotype::from_wire(p.as_str()), Ok(p));
        }
        assert!(ClinicalPhenotype::from_wire("epsilon").is_err());
    }
}
