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
//! The module surface is built up across Phase 1 (see `Plans.md`). This root file
//! currently establishes the crate, its lints, and the doctest gate.

// The empty-crate scaffold (task 1.1). Subsequent tasks add the omics, sample,
// phenotype, outcome, parsing, and versioning modules.
