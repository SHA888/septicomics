//! Clinical outcomes, carrying explicit units in the type.
//!
//! Outcome quantities never travel as bare numbers: a duration is [`Days`], an
//! organ-failure severity is a bounded [`SofaScore`] in points, and a mortality
//! result pins its assessment horizon. This makes unit/horizon mismatches (e.g.
//! comparing 28-day to 90-day mortality) representable only deliberately.

use core::fmt;

use crate::error::{CdmError, Result};
use crate::macros::wire_enum;

/// A duration in **days**. Newtype over a finite, non-negative `f64`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Days(f64);

impl Days {
    /// Construct a day-count, rejecting non-finite or negative values.
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::NonFiniteQuantity`] if `value` is `NaN`/infinite, or
    /// [`CdmError::OutOfRange`] if it is negative.
    pub fn new(value: f64) -> Result<Self> {
        if !value.is_finite() {
            return Err(CdmError::NonFiniteQuantity { field: "Days" });
        }
        if value < 0.0 {
            return Err(CdmError::OutOfRange { field: "Days" });
        }
        Ok(Days(value))
    }

    /// The underlying value, in days.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Days {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} d", self.0)
    }
}

wire_enum! {
    /// Vital status at an assessment horizon.
    pub enum VitalStatus {
        /// The subject was alive at the assessment horizon.
        Alive => "alive",
        /// The subject had died by the assessment horizon.
        Dead => "dead",
    }
}

/// A mortality outcome: [`VitalStatus`] at a specified [`Days`] horizon
/// (e.g. 28-day mortality is `status == Dead` at `horizon == 28 d`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MortalityOutcome {
    horizon: Days,
    status: VitalStatus,
}

impl MortalityOutcome {
    /// Construct a mortality outcome from a horizon and a status.
    #[must_use]
    pub fn new(horizon: Days, status: VitalStatus) -> Self {
        MortalityOutcome { horizon, status }
    }

    /// The assessment horizon.
    #[must_use]
    pub fn horizon(self) -> Days {
        self.horizon
    }

    /// The vital status at the horizon.
    #[must_use]
    pub fn status(self) -> VitalStatus {
        self.status
    }

    /// Whether the outcome is death at the horizon.
    #[must_use]
    pub fn is_death(self) -> bool {
        matches!(self.status, VitalStatus::Dead)
    }
}

/// Total SOFA (Sequential Organ Failure Assessment) score: organ-failure severity.
///
/// Unit: **points**, bounded to `0..=24`. Newtype over `u8`; out-of-range scores are
/// rejected at construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SofaScore(u8);

impl SofaScore {
    /// The maximum possible total SOFA score, in points.
    pub const MAX_POINTS: u8 = 24;

    /// Construct a SOFA score, rejecting values above [`SofaScore::MAX_POINTS`].
    ///
    /// # Errors
    ///
    /// Returns [`CdmError::OutOfRange`] if `points` exceeds 24.
    pub fn new(points: u8) -> Result<Self> {
        if points > Self::MAX_POINTS {
            return Err(CdmError::OutOfRange { field: "SofaScore" });
        }
        Ok(SofaScore(points))
    }

    /// The score, in points.
    #[must_use]
    pub fn points(self) -> u8 {
        self.0
    }
}

impl fmt::Display for SofaScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} pts", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn days_rejects_negative_and_non_finite() {
        assert!(Days::new(-1.0).is_err());
        assert!(Days::new(f64::NAN).is_err());
        assert_eq!(Days::new(0.0).unwrap().get(), 0.0);
        assert_eq!(Days::new(28.0).unwrap().get(), 28.0);
    }

    #[test]
    fn vital_status_roundtrips() {
        for &s in VitalStatus::ALL {
            assert_eq!(VitalStatus::from_wire(s.as_str()), Ok(s));
        }
        assert!(VitalStatus::from_wire("unknown").is_err());
    }

    #[test]
    fn mortality_outcome_reports_death() {
        let horizon = Days::new(28.0).unwrap();
        assert!(MortalityOutcome::new(horizon, VitalStatus::Dead).is_death());
        assert!(!MortalityOutcome::new(horizon, VitalStatus::Alive).is_death());
    }

    #[test]
    fn sofa_score_bounds() {
        assert!(SofaScore::new(25).is_err());
        assert_eq!(SofaScore::new(24).unwrap().points(), 24);
        assert!(SofaScore::new(0).unwrap() < SofaScore::new(10).unwrap());
    }
}
