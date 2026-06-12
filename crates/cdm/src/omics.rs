//! Omics layers and the newtype quantities measured within them.
//!
//! Domain quantities are newtypes — never bare `String`/`f64` — so a feature
//! identifier cannot be confused with an arbitrary string, nor an abundance with
//! an arbitrary float. Construction is checked, so a non-finite abundance or an
//! empty feature id is unrepresentable.

use crate::error::{CdmError, Result};
use crate::macros::{id_newtype, wire_enum};

wire_enum! {
    /// A molecular measurement layer in the sepsis multi-omics CDM.
    ///
    /// This enum is intentionally **not** `#[non_exhaustive]`: adding a layer is a
    /// breaking change to the CDM contract, hence a major version bump and a
    /// coordinated node-upgrade event (see `ARCHITECTURE.md` §6). `cargo-semver-checks`
    /// enforces this. Downstream code is expected to `match` exhaustively, and the
    /// wire identifiers are part of the serialization contract.
    pub enum OmicsLayer {
        /// Bulk transcriptomics (e.g. RNA-seq gene expression).
        Transcriptomics => "transcriptomics",
        /// Proteomics (protein abundance).
        Proteomics => "proteomics",
        /// Single-cell transcriptomics.
        SingleCellTranscriptomics => "single_cell_transcriptomics",
        /// Metabolomics (metabolite abundance).
        Metabolomics => "metabolomics",
        /// Lipidomics (lipid abundance).
        Lipidomics => "lipidomics",
        /// Epigenomics (e.g. DNA methylation).
        Epigenomics => "epigenomics",
    }
}

id_newtype! {
    /// A stable identifier for a measured feature (gene, transcript, protein, …).
    ///
    /// Newtype around a non-empty string, so a feature id cannot be confused with an
    /// arbitrary string. Construct via [`FeatureId::new`].
    pub struct FeatureId;
}

/// A measured abundance / expression value within an omics layer.
///
/// Newtype around a finite `f64`; non-finite values (`NaN`, ±∞) are rejected at
/// construction so downstream statistics never encounter them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Abundance(f64);

impl Abundance {
    /// Construct an abundance, rejecting non-finite values.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::NonFiniteQuantity`] if `value` is `NaN` or infinite.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(CdmError::NonFiniteQuantity { field: "Abundance" });
        }
        Ok(Abundance(value))
    }

    /// The underlying finite value.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_roundtrips_through_wire() {
        for &layer in OmicsLayer::ALL {
            assert_eq!(OmicsLayer::from_wire(layer.as_str()), Ok(layer));
        }
    }

    #[test]
    fn unknown_layer_is_rejected() {
        assert!(matches!(
            OmicsLayer::from_wire("genomics"),
            Err(CdmError::UnknownVariant { .. })
        ));
    }

    #[test]
    fn feature_id_rejects_empty() {
        assert!(FeatureId::new("   ").is_err());
        assert_eq!(FeatureId::new("ENSG000001").unwrap().as_str(), "ENSG000001");
    }

    #[test]
    fn abundance_rejects_non_finite() {
        assert!(Abundance::new(f64::NAN).is_err());
        assert!(Abundance::new(f64::INFINITY).is_err());
        assert_eq!(Abundance::new(3.5).unwrap().get(), 3.5);
    }
}
