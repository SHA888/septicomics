//! Boundary parsers: the single path from raw, untyped input into CDM values.
//!
//! This is where **parse-don't-validate** lives. Input arrives at the node boundary
//! as untyped strings (a CSV row, a decoded JSON object). A parser either turns a
//! whole raw record into a fully-valid CDM value or returns a typed [`CdmError`].
//! There is no partially-constructed CDM value: validation and construction are the
//! same step, so downstream code never has to re-check what a type already guarantees.

use crate::error::{CdmError, Result};
use crate::omics::OmicsLayer;
use crate::outcome::{Days, MortalityOutcome, VitalStatus};
use crate::sample::{HoursFromAnchor, Sample, SampleId, SubjectId, TimeAnchor, Timepoint};

/// Parse a decimal scalar, attributing failures to a named field.
fn parse_f64(raw: &str, field: &'static str) -> Result<f64> {
    let trimmed = raw.trim();
    trimmed.parse::<f64>().map_err(|_| CdmError::Malformed {
        // Report the value actually parsed (trimmed), not the surrounding whitespace.
        field,
        value: trimmed.to_owned(),
    })
}

/// Re-attribute a field-bearing [`CdmError`] to the raw column it came from, so a
/// failure reports the boundary column name (e.g. `"hours_from_anchor"`) rather than
/// the internal type name (e.g. `"HoursFromAnchor"`). Variants without a `field`
/// (e.g. `UnknownVariant`, which already carries `kind`) pass through unchanged.
fn at_field(err: CdmError, field: &'static str) -> CdmError {
    match err {
        CdmError::EmptyIdentifier { .. } => CdmError::EmptyIdentifier { field },
        CdmError::NonFiniteQuantity { .. } => CdmError::NonFiniteQuantity { field },
        CdmError::OutOfRange { .. } => CdmError::OutOfRange { field },
        other => other,
    }
}

/// An untyped sample row as it arrives at the node boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawSample {
    /// Raw sample identifier.
    pub sample_id: String,
    /// Raw subject identifier.
    pub subject_id: String,
    /// Raw time anchor (wire form, e.g. `"sepsis_onset"`).
    pub anchor: String,
    /// Raw offset from the anchor, in hours (decimal string).
    pub hours_from_anchor: String,
    /// Raw omics layer (wire form, e.g. `"transcriptomics"`).
    pub omics_layer: String,
}

impl RawSample {
    /// Parse this raw row into a [`Sample`], or fail with a typed [`CdmError`].
    ///
    /// All-or-nothing: if any field is invalid no `Sample` is produced.
    ///
    /// # Errors
    ///
    /// Returns the first [`CdmError`] encountered: empty ids, an unknown anchor or
    /// layer, or a malformed/non-finite hours value.
    pub fn parse(&self) -> Result<Sample> {
        let sample_id =
            SampleId::new(self.sample_id.as_str()).map_err(|e| at_field(e, "sample_id"))?;
        let subject_id =
            SubjectId::new(self.subject_id.as_str()).map_err(|e| at_field(e, "subject_id"))?;
        let anchor = TimeAnchor::from_wire(&self.anchor)?;
        let offset = HoursFromAnchor::new(parse_f64(&self.hours_from_anchor, "hours_from_anchor")?)
            .map_err(|e| at_field(e, "hours_from_anchor"))?;
        let layer = OmicsLayer::from_wire(&self.omics_layer)?;
        Ok(Sample::new(
            sample_id,
            subject_id,
            Timepoint::new(anchor, offset),
            layer,
        ))
    }
}

/// An untyped mortality row as it arrives at the node boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawMortality {
    /// Raw assessment horizon, in days (decimal string).
    pub horizon_days: String,
    /// Raw vital status (wire form, e.g. `"dead"`).
    pub vital_status: String,
}

impl RawMortality {
    /// Parse this raw row into a [`MortalityOutcome`], or fail with a typed error.
    ///
    /// # Errors
    ///
    /// Returns a [`CdmError`] for a malformed/negative horizon or an unknown status.
    pub fn parse(&self) -> Result<MortalityOutcome> {
        let horizon = Days::new(parse_f64(&self.horizon_days, "horizon_days")?)
            .map_err(|e| at_field(e, "horizon_days"))?;
        let status = VitalStatus::from_wire(&self.vital_status)?;
        Ok(MortalityOutcome::new(horizon, status))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_raw_sample() -> RawSample {
        RawSample {
            sample_id: "SMP-1".to_owned(),
            subject_id: "SUB-1".to_owned(),
            anchor: "sepsis_onset".to_owned(),
            hours_from_anchor: "12.5".to_owned(),
            omics_layer: "transcriptomics".to_owned(),
        }
    }

    #[test]
    fn parses_a_valid_sample() {
        let sample = good_raw_sample().parse().unwrap();
        assert_eq!(sample.id().as_str(), "SMP-1");
        assert_eq!(sample.subject().as_str(), "SUB-1");
        assert_eq!(sample.timepoint().anchor(), TimeAnchor::SepsisOnset);
        assert_eq!(sample.timepoint().offset().get(), 12.5);
        assert_eq!(sample.layer(), OmicsLayer::Transcriptomics);
    }

    #[test]
    fn sample_round_trips_through_raw() {
        let sample = good_raw_sample().parse().unwrap();
        let rebuilt = RawSample {
            sample_id: sample.id().as_str().to_owned(),
            subject_id: sample.subject().as_str().to_owned(),
            anchor: sample.timepoint().anchor().as_str().to_owned(),
            hours_from_anchor: sample.timepoint().offset().get().to_string(),
            omics_layer: sample.layer().as_str().to_owned(),
        };
        assert_eq!(rebuilt.parse().unwrap(), sample);
    }

    #[test]
    fn rejects_unknown_anchor() {
        let raw = RawSample {
            anchor: "moon_landing".to_owned(),
            ..good_raw_sample()
        };
        assert!(matches!(
            raw.parse(),
            Err(CdmError::UnknownVariant {
                kind: "TimeAnchor",
                ..
            })
        ));
    }

    #[test]
    fn rejects_unknown_layer() {
        let raw = RawSample {
            omics_layer: "genomics".to_owned(),
            ..good_raw_sample()
        };
        assert!(matches!(
            raw.parse(),
            Err(CdmError::UnknownVariant {
                kind: "OmicsLayer",
                ..
            })
        ));
    }

    #[test]
    fn rejects_empty_id() {
        let raw = RawSample {
            sample_id: "  ".to_owned(),
            ..good_raw_sample()
        };
        assert!(matches!(raw.parse(), Err(CdmError::EmptyIdentifier { .. })));
    }

    #[test]
    fn rejects_non_numeric_hours() {
        let raw = RawSample {
            hours_from_anchor: "soon".to_owned(),
            ..good_raw_sample()
        };
        assert!(matches!(
            raw.parse(),
            Err(CdmError::Malformed {
                field: "hours_from_anchor",
                ..
            })
        ));
    }

    #[test]
    fn rejects_non_finite_hours_with_column_field() {
        // "inf" parses as a float but is not a finite quantity; the error must name
        // the raw column ("hours_from_anchor"), not the internal type.
        let raw = RawSample {
            hours_from_anchor: "inf".to_owned(),
            ..good_raw_sample()
        };
        assert_eq!(
            raw.parse(),
            Err(CdmError::NonFiniteQuantity {
                field: "hours_from_anchor"
            })
        );
    }

    #[test]
    fn parses_and_rejects_mortality() {
        let good = RawMortality {
            horizon_days: "28".to_owned(),
            vital_status: "dead".to_owned(),
        };
        let outcome = good.parse().unwrap();
        assert!(outcome.is_death());
        assert_eq!(outcome.horizon().get(), 28.0);

        let negative = RawMortality {
            horizon_days: "-5".to_owned(),
            vital_status: "alive".to_owned(),
        };
        assert_eq!(
            negative.parse(),
            Err(CdmError::OutOfRange {
                field: "horizon_days"
            })
        );

        let bad_status = RawMortality {
            horizon_days: "28".to_owned(),
            vital_status: "zombie".to_owned(),
        };
        assert!(matches!(
            bad_status.parse(),
            Err(CdmError::UnknownVariant {
                kind: "VitalStatus",
                ..
            })
        ));
    }
}
