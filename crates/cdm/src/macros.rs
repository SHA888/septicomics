//! Internal declarative macros that generate the repetitive contract boilerplate.
//!
//! [`wire_enum!`] and [`id_newtype!`] exist so the wire enumerations and the
//! non-empty-string id newtypes share a single definition each, instead of
//! hand-repeating `ALL`/`as_str`/`from_wire`/`Display` (or `new`/`as_str`/`Display`).
//!
//! Beyond removing duplication, `wire_enum!` makes a duplicated wire string a
//! **compile error**: `from_wire` is generated as a `match` on the wire literals, so
//! two variants mapping to the same string trip the `unreachable_patterns` lint
//! (denied in CI), structurally guaranteeing the `as_str` mapping is injective.

/// Generate a closed "wire" enumeration — the enum plus `ALL`, `as_str`,
/// `from_wire`, and `Display` — from a single variant → wire-string table.
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $( $(#[$vmeta:meta])* $Variant:ident => $wire:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $Name {
            $( $(#[$vmeta])* $Variant, )+
        }

        impl $Name {
            /// Every variant defined by this CDM version, in declaration order.
            pub const ALL: &'static [$Name] = &[ $( $Name::$Variant ),+ ];

            /// The stable, machine-readable wire identifier for this variant.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $( $Name::$Variant => $wire, )+
                }
            }

            /// Parse a wire identifier (as produced by `as_str`) into a variant.
            ///
            /// # Errors
            ///
            /// Returns [`crate::error::CdmError::UnknownVariant`] if no variant matches.
            pub fn from_wire(value: &str) -> $crate::error::Result<Self> {
                match value {
                    $( $wire => Ok($Name::$Variant), )+
                    _ => Err($crate::error::CdmError::UnknownVariant {
                        kind: stringify!($Name),
                        value: value.to_owned(),
                    }),
                }
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

/// Generate a non-empty-string identifier newtype with a checked constructor,
/// `as_str`, and `Display`.
macro_rules! id_newtype {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $Name(String);

        impl $Name {
            #[doc = concat!("Construct a [`", stringify!($Name), "`], rejecting empty / whitespace-only input.")]
            ///
            /// # Errors
            ///
            /// Returns [`crate::error::CdmError::EmptyIdentifier`] if the trimmed input is empty.
            pub fn new(value: impl Into<String>) -> $crate::error::Result<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err($crate::error::CdmError::EmptyIdentifier {
                        field: stringify!($Name),
                    });
                }
                Ok($Name(value))
            }

            /// Borrow the identifier as a string slice.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::core::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

pub(crate) use id_newtype;
pub(crate) use wire_enum;
