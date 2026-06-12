//! Omics layers and the newtype quantities measured within them.
//!
//! Domain quantities are newtypes — never bare `String`/`f64` — so a feature
//! identifier cannot be confused with an arbitrary string, nor an abundance with
//! an arbitrary float. Construction is checked, so a non-finite abundance or an
//! empty feature id is unrepresentable.

use core::fmt;

use crate::error::{CdmError, Result};

/// A molecular measurement layer in the sepsis multi-omics CDM.
///
/// This enum is intentionally **not** `#[non_exhaustive]`: adding a layer is a
/// breaking change to the CDM contract, hence a major version bump and a
/// coordinated node-upgrade event (see `ARCHITECTURE.md` §6). `cargo-semver-checks`
/// enforces this. Downstream code is expected to `match` exhaustively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmicsLayer {
    /// Bulk transcriptomics (e.g. RNA-seq gene expression).
    Transcriptomics,
    /// Proteomics (protein abundance).
    Proteomics,
    /// Single-cell transcriptomics.
    SingleCellTranscriptomics,
    /// Metabolomics (metabolite abundance).
    Metabolomics,
    /// Lipidomics (lipid abundance).
    Lipidomics,
    /// Epigenomics (e.g. DNA methylation).
    Epigenomics,
}

impl OmicsLayer {
    /// Every layer defined by this CDM version, in declaration order.
    pub const ALL: [OmicsLayer; 6] = [
        OmicsLayer::Transcriptomics,
        OmicsLayer::Proteomics,
        OmicsLayer::SingleCellTranscriptomics,
        OmicsLayer::Metabolomics,
        OmicsLayer::Lipidomics,
        OmicsLayer::Epigenomics,
    ];

    /// The stable, lowercase, machine-readable identifier for this layer.
    ///
    /// These identifiers are part of the serialization contract and must not change
    /// without a major version bump.
    pub const fn as_str(self) -> &'static str {
        match self {
            OmicsLayer::Transcriptomics => "transcriptomics",
            OmicsLayer::Proteomics => "proteomics",
            OmicsLayer::SingleCellTranscriptomics => "single_cell_transcriptomics",
            OmicsLayer::Metabolomics => "metabolomics",
            OmicsLayer::Lipidomics => "lipidomics",
            OmicsLayer::Epigenomics => "epigenomics",
        }
    }

    /// Parse a wire identifier (as produced by [`OmicsLayer::as_str`]) into a layer.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::UnknownVariant`] if `value` matches no known layer.
    pub fn from_wire(value: &str) -> Result<Self> {
        OmicsLayer::ALL
            .into_iter()
            .find(|layer| layer.as_str() == value)
            .ok_or_else(|| CdmError::UnknownVariant {
                kind: "OmicsLayer",
                value: value.to_owned(),
            })
    }
}

impl fmt::Display for OmicsLayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A stable identifier for a measured feature (gene, transcript, protein, …).
///
/// Newtype around a non-empty string, so a feature id cannot be confused with an
/// arbitrary string. Construct via [`FeatureId::new`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureId(String);

impl FeatureId {
    /// Construct a feature id, rejecting empty / whitespace-only input.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::EmptyIdentifier`] if the trimmed input is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CdmError::EmptyIdentifier { field: "FeatureId" });
        }
        Ok(FeatureId(value))
    }

    /// Borrow the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for FeatureId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
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
        for layer in OmicsLayer::ALL {
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
