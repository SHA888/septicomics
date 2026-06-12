//! Subjects, samples, and the temporal trajectories that connect them.
//!
//! Temporal structure is **first-class**: a [`Timepoint`] is always relative to an
//! explicit clinical [`TimeAnchor`], and a [`Trajectory`] is an anchor-consistent,
//! time-ordered sequence of one subject's samples. Sepsis phenotyping is inherently
//! temporal, so the schema models trajectories directly rather than treating time
//! as a loose attribute.

use core::fmt;

use crate::error::{CdmError, Result};
use crate::omics::OmicsLayer;

/// Identifier for a study subject (patient). Newtype over a non-empty string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubjectId(String);

impl SubjectId {
    /// Construct a subject id, rejecting empty / whitespace-only input.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::EmptyIdentifier`] if the trimmed input is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CdmError::EmptyIdentifier { field: "SubjectId" });
        }
        Ok(SubjectId(value))
    }

    /// Borrow the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SubjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Identifier for a biological sample. Newtype over a non-empty string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SampleId(String);

impl SampleId {
    /// Construct a sample id, rejecting empty / whitespace-only input.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::EmptyIdentifier`] if the trimmed input is empty.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CdmError::EmptyIdentifier { field: "SampleId" });
        }
        Ok(SampleId(value))
    }

    /// Borrow the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SampleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The clinical reference event a [`Timepoint`] is measured relative to.
///
/// Not `#[non_exhaustive]`: adding an anchor is a breaking CDM change (major bump).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeAnchor {
    /// ICU (or hospital) admission.
    IcuAdmission,
    /// Recognized onset of sepsis.
    SepsisOnset,
    /// Study enrollment.
    Enrollment,
}

impl TimeAnchor {
    /// Every anchor defined by this CDM version, in declaration order.
    pub const ALL: [TimeAnchor; 3] = [
        TimeAnchor::IcuAdmission,
        TimeAnchor::SepsisOnset,
        TimeAnchor::Enrollment,
    ];

    /// The stable, machine-readable identifier for this anchor.
    pub const fn as_str(self) -> &'static str {
        match self {
            TimeAnchor::IcuAdmission => "icu_admission",
            TimeAnchor::SepsisOnset => "sepsis_onset",
            TimeAnchor::Enrollment => "enrollment",
        }
    }

    /// Parse a wire identifier (as produced by [`TimeAnchor::as_str`]).
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::UnknownVariant`] if `value` matches no known anchor.
    pub fn from_wire(value: &str) -> Result<Self> {
        TimeAnchor::ALL
            .into_iter()
            .find(|anchor| anchor.as_str() == value)
            .ok_or_else(|| CdmError::UnknownVariant {
                kind: "TimeAnchor",
                value: value.to_owned(),
            })
    }
}

impl fmt::Display for TimeAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Hours elapsed from a [`TimeAnchor`]. May be negative (before the anchor).
///
/// Newtype over a finite `f64`; non-finite values are rejected at construction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoursFromAnchor(f64);

impl HoursFromAnchor {
    /// Construct an offset, rejecting non-finite values.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::NonFiniteQuantity`] if `value` is `NaN` or infinite.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(CdmError::NonFiniteQuantity {
                field: "HoursFromAnchor",
            });
        }
        Ok(HoursFromAnchor(value))
    }

    /// The underlying finite value, in hours.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

/// A point on a temporal trajectory: an [`HoursFromAnchor`] offset from a
/// [`TimeAnchor`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timepoint {
    anchor: TimeAnchor,
    offset: HoursFromAnchor,
}

impl Timepoint {
    /// Construct a timepoint from an anchor and a (validated) offset.
    #[must_use]
    pub fn new(anchor: TimeAnchor, offset: HoursFromAnchor) -> Self {
        Timepoint { anchor, offset }
    }

    /// The clinical reference event this timepoint is relative to.
    #[must_use]
    pub fn anchor(self) -> TimeAnchor {
        self.anchor
    }

    /// The offset from the anchor, in hours.
    #[must_use]
    pub fn offset(self) -> HoursFromAnchor {
        self.offset
    }
}

/// A measured biological sample: one subject, at one timepoint, on one omics layer.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    id: SampleId,
    subject: SubjectId,
    timepoint: Timepoint,
    layer: OmicsLayer,
}

impl Sample {
    /// Construct a sample from its already-validated components.
    #[must_use]
    pub fn new(id: SampleId, subject: SubjectId, timepoint: Timepoint, layer: OmicsLayer) -> Self {
        Sample {
            id,
            subject,
            timepoint,
            layer,
        }
    }

    /// The sample's identifier.
    #[must_use]
    pub fn id(&self) -> &SampleId {
        &self.id
    }

    /// The subject this sample belongs to.
    #[must_use]
    pub fn subject(&self) -> &SubjectId {
        &self.subject
    }

    /// The timepoint at which this sample was taken.
    #[must_use]
    pub fn timepoint(&self) -> Timepoint {
        self.timepoint
    }

    /// The omics layer this sample was measured on.
    #[must_use]
    pub fn layer(&self) -> OmicsLayer {
        self.layer
    }
}

/// A subject's temporal trajectory: an anchor-consistent, time-ordered set of
/// that subject's samples.
///
/// Invariants enforced at construction: non-empty, every sample belongs to the
/// same subject, every sample shares a single [`TimeAnchor`], and samples are
/// sorted by ascending offset.
#[derive(Debug, Clone, PartialEq)]
pub struct Trajectory {
    subject: SubjectId,
    anchor: TimeAnchor,
    samples: Vec<Sample>,
}

impl Trajectory {
    /// Build a trajectory for `subject` from its samples.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::Inconsistent`] if `samples` is empty, contains a sample
    /// belonging to another subject, or mixes [`TimeAnchor`]s.
    pub fn new(subject: SubjectId, mut samples: Vec<Sample>) -> Result<Self> {
        let Some(first) = samples.first() else {
            return Err(CdmError::Inconsistent {
                context: "empty trajectory",
            });
        };
        let anchor = first.timepoint().anchor();
        for sample in &samples {
            if sample.subject() != &subject {
                return Err(CdmError::Inconsistent {
                    context: "trajectory subject mismatch",
                });
            }
            if sample.timepoint().anchor() != anchor {
                return Err(CdmError::Inconsistent {
                    context: "trajectory mixed anchors",
                });
            }
        }
        // Sort by offset, then break ties on sample id so the ordering is fully
        // deterministic regardless of input order (two nodes parsing the same data
        // in a different row order must produce identical trajectories).
        samples.sort_by(|a, b| {
            a.timepoint()
                .offset()
                .get()
                .total_cmp(&b.timepoint().offset().get())
                .then_with(|| a.id().as_str().cmp(b.id().as_str()))
        });
        Ok(Trajectory {
            subject,
            anchor,
            samples,
        })
    }

    /// The subject this trajectory describes.
    #[must_use]
    pub fn subject(&self) -> &SubjectId {
        &self.subject
    }

    /// The single anchor shared by every sample on this trajectory.
    #[must_use]
    pub fn anchor(&self) -> TimeAnchor {
        self.anchor
    }

    /// The samples, sorted by ascending offset from the anchor.
    #[must_use]
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }

    /// The number of samples on the trajectory (always at least one).
    #[must_use]
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Always `false`; a trajectory is non-empty by construction. Present so the
    /// type satisfies the usual `len`/`is_empty` pairing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_at(subject: &str, id: &str, hours: f64) -> Sample {
        Sample::new(
            SampleId::new(id).unwrap(),
            SubjectId::new(subject).unwrap(),
            Timepoint::new(
                TimeAnchor::SepsisOnset,
                HoursFromAnchor::new(hours).unwrap(),
            ),
            OmicsLayer::Transcriptomics,
        )
    }

    #[test]
    fn ids_reject_empty() {
        assert!(SubjectId::new("  ").is_err());
        assert!(SampleId::new("").is_err());
        assert_eq!(SubjectId::new("S1").unwrap().as_str(), "S1");
    }

    #[test]
    fn hours_allow_negative_reject_non_finite() {
        assert_eq!(HoursFromAnchor::new(-6.0).unwrap().get(), -6.0);
        assert!(HoursFromAnchor::new(f64::NAN).is_err());
    }

    #[test]
    fn anchor_roundtrips_through_wire() {
        for anchor in TimeAnchor::ALL {
            assert_eq!(TimeAnchor::from_wire(anchor.as_str()), Ok(anchor));
        }
        assert!(TimeAnchor::from_wire("nonsense").is_err());
    }

    #[test]
    fn trajectory_sorts_by_offset() {
        let traj = Trajectory::new(
            SubjectId::new("S1").unwrap(),
            vec![
                sample_at("S1", "B", 24.0),
                sample_at("S1", "A", 0.0),
                sample_at("S1", "C", 48.0),
            ],
        )
        .unwrap();
        let order: Vec<&str> = traj.samples().iter().map(|s| s.id().as_str()).collect();
        assert_eq!(order, ["A", "B", "C"]);
        assert_eq!(traj.len(), 3);
        assert!(!traj.is_empty());
    }

    #[test]
    fn trajectory_breaks_offset_ties_deterministically() {
        // Same offset, supplied in two different input orders → identical result,
        // ordered by sample id.
        let forward = Trajectory::new(
            SubjectId::new("S1").unwrap(),
            vec![sample_at("S1", "B", 24.0), sample_at("S1", "A", 24.0)],
        )
        .unwrap();
        let reversed = Trajectory::new(
            SubjectId::new("S1").unwrap(),
            vec![sample_at("S1", "A", 24.0), sample_at("S1", "B", 24.0)],
        )
        .unwrap();
        let ids: Vec<&str> = forward.samples().iter().map(|s| s.id().as_str()).collect();
        assert_eq!(ids, ["A", "B"]);
        assert_eq!(forward, reversed);
    }

    #[test]
    fn trajectory_rejects_empty() {
        assert!(matches!(
            Trajectory::new(SubjectId::new("S1").unwrap(), vec![]),
            Err(CdmError::Inconsistent { .. })
        ));
    }

    #[test]
    fn trajectory_rejects_subject_mismatch() {
        assert!(matches!(
            Trajectory::new(
                SubjectId::new("S1").unwrap(),
                vec![sample_at("S2", "A", 0.0)],
            ),
            Err(CdmError::Inconsistent { .. })
        ));
    }

    #[test]
    fn trajectory_rejects_mixed_anchors() {
        let s1 = sample_at("S1", "A", 0.0);
        let s2 = Sample::new(
            SampleId::new("B").unwrap(),
            SubjectId::new("S1").unwrap(),
            Timepoint::new(
                TimeAnchor::IcuAdmission,
                HoursFromAnchor::new(12.0).unwrap(),
            ),
            OmicsLayer::Proteomics,
        );
        assert!(matches!(
            Trajectory::new(SubjectId::new("S1").unwrap(), vec![s1, s2]),
            Err(CdmError::Inconsistent { .. })
        ));
    }
}
