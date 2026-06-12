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

use crate::macros::wire_enum;

wire_enum! {
    /// The inflammatory endotype axis: a subject's molecular inflammatory state.
    pub enum InflammatoryEndotype {
        /// Heightened inflammation; typically higher shock burden and mortality.
        Hyperinflammatory => "hyperinflammatory",
        /// Lower inflammation.
        Hypoinflammatory => "hypoinflammatory",
    }
}

wire_enum! {
    /// Transcriptomic Sepsis Response Signature endotypes (Davenport et al.).
    pub enum SepsisResponseSignature {
        /// SRS1: relatively immunosuppressed profile; associated with higher mortality.
        Srs1 => "srs1",
        /// SRS2: relatively immunocompetent profile.
        Srs2 => "srs2",
    }
}

wire_enum! {
    /// Clinical sepsis phenotypes α/β/γ/δ (Seymour et al.).
    pub enum ClinicalPhenotype {
        /// α (alpha): fewest abnormalities, lowest vasopressor use.
        Alpha => "alpha",
        /// β (beta): older, more chronic illness and renal dysfunction.
        Beta => "beta",
        /// γ (gamma): more inflammation and pulmonary dysfunction.
        Gamma => "gamma",
        /// δ (delta): more hepatic dysfunction and shock; highest mortality.
        Delta => "delta",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inflammatory_endotype_roundtrips() {
        for &e in InflammatoryEndotype::ALL {
            assert_eq!(InflammatoryEndotype::from_wire(e.as_str()), Ok(e));
        }
        assert!(InflammatoryEndotype::from_wire("mixed").is_err());
    }

    #[test]
    fn srs_roundtrips() {
        for &e in SepsisResponseSignature::ALL {
            assert_eq!(SepsisResponseSignature::from_wire(e.as_str()), Ok(e));
        }
        assert!(SepsisResponseSignature::from_wire("srs3").is_err());
    }

    #[test]
    fn clinical_phenotype_roundtrips() {
        for &p in ClinicalPhenotype::ALL {
            assert_eq!(ClinicalPhenotype::from_wire(p.as_str()), Ok(p));
        }
        assert!(ClinicalPhenotype::from_wire("epsilon").is_err());
    }
}
