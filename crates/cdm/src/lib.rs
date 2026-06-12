//! # septicomics Common Data Model (CDM)
//!
//! The CDM is the load-bearing contract of septicomics: the shared sepsis
//! multi-omics schema that every federation node must speak. The web app and the
//! orchestrator are replaceable; the CDM is not (see `ARCHITECTURE.md`).
//!
//! ## Design disciplines
//!
//! - **Parse, don't validate.** Raw input is parsed at the boundary into a CDM
//!   value or a typed error; no half-validated states are constructible.
//! - **Make illegal states unrepresentable.** Domain quantities are newtypes, not
//!   bare `String`/`f64`; categorical domains are exhaustive enums.
//! - **SemVer is a contract.** This crate versions strictly and independently; a
//!   breaking schema change is a major bump and a coordinated node-upgrade event.
//!
//! This crate is licensed **Apache-2.0** so nodes and third parties can depend on
//! the contract without copyleft friction (see `LICENSING.md`).
//!
//! ## Example
//!
//! ```
//! use septicomics_cdm::omics::{Abundance, FeatureId, OmicsLayer};
//!
//! let layer = OmicsLayer::from_wire("proteomics")?;
//! assert_eq!(layer, OmicsLayer::Proteomics);
//!
//! let feature = FeatureId::new("P12345")?;
//! let value = Abundance::new(8.21)?;
//! assert_eq!(feature.as_str(), "P12345");
//! assert_eq!(value.get(), 8.21);
//! # Ok::<(), septicomics_cdm::error::CdmError>(())
//! ```

pub mod error;
pub mod omics;
pub mod outcome;
pub mod parse;
pub mod phenotype;
pub mod sample;
