//! The single error type returned by CDM boundary constructors and parsers.
//!
//! Boundary constructors return [`CdmError`] instead of ever constructing a
//! half-valid value (parse-don't-validate). The type is `#[non_exhaustive]` so new
//! diagnostic variants can be added in a minor release without breaking callers.

use core::fmt;

/// Error produced when raw input cannot be parsed into a valid CDM value.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CdmError {
    /// A required identifier was empty or contained only whitespace.
    EmptyIdentifier {
        /// The domain field that was empty (for diagnostics).
        field: &'static str,
    },
    /// A quantity required to be finite was `NaN` or infinite.
    NonFiniteQuantity {
        /// The domain field whose value was non-finite.
        field: &'static str,
    },
    /// A quantity fell outside its permitted range.
    OutOfRange {
        /// The domain field whose value was out of range.
        field: &'static str,
    },
    /// A string did not match any known variant of a CDM enumeration.
    UnknownVariant {
        /// The enumeration being parsed (e.g. `"OmicsLayer"`).
        kind: &'static str,
        /// The unrecognized input value.
        value: String,
    },
    /// A composite value violated an internal consistency rule.
    Inconsistent {
        /// What was inconsistent, for diagnostics (e.g. `"trajectory mixed anchors"`).
        context: &'static str,
    },
}

impl fmt::Display for CdmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CdmError::EmptyIdentifier { field } => {
                write!(f, "identifier field `{field}` was empty")
            }
            CdmError::NonFiniteQuantity { field } => {
                write!(f, "quantity field `{field}` was not finite")
            }
            CdmError::OutOfRange { field } => {
                write!(f, "quantity field `{field}` was out of range")
            }
            CdmError::UnknownVariant { kind, value } => {
                write!(f, "`{value}` is not a valid {kind}")
            }
            CdmError::Inconsistent { context } => {
                write!(f, "inconsistent value: {context}")
            }
        }
    }
}

impl std::error::Error for CdmError {}

/// Convenience alias for the result of a CDM boundary parse/construction.
pub type Result<T> = core::result::Result<T, CdmError>;
