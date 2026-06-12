# CDM Versioning Policy

The Common Data Model (`septicomics-cdm`) is the contract that binds
independently-operated federation nodes. Unlike an internal library, a breaking
change here is not a silent deploy — it is a **coordinated node-upgrade event**.
This document is the policy; the machine-readable version lives in
`crates/cdm/src/version.rs` as `CDM_SCHEMA_VERSION`.

## Semantic versioning, applied to a schema

The CDM version is `major.minor.patch`:

- **major** — a **breaking** schema change: removing/renaming a field, changing a
  type's meaning, removing an enum variant, or changing a stable wire identifier
  (e.g. an `OmicsLayer::as_str` value). Adding a variant to a closed enum is also
  breaking, because downstream `match`es are exhaustive by design.
- **minor** — a **backward-compatible** addition that existing nodes can ignore.
- **patch** — a backward-compatible fix (docs, internal logic) with no schema change.

## The compatibility rule (what nodes negotiate on)

> Two CDM schema versions are **wire-compatible iff their `major` components match.**

This is implemented by `SchemaVersion::is_compatible_with` and enforced at
fan-out: a node advertises the CDM major it speaks, and the orchestrator **refuses**
to dispatch a plan to a node whose major differs, rather than guessing
(`ARCHITECTURE.md` §6). Minor/patch differences within the same major are always
compatible.

## Pre-1.0 caveat

While `major == 0`, the schema is explicitly **unstable**: per SemVer, any `0.x`
release may break. During this phase the federation is expected to run a single,
pinned `0.x` and upgrade in lockstep; the major-match rule still holds (all `0.x`
share major `0`), but do **not** rely on cross-`0.x` stability. The first
`1.0.0` of `septicomics-cdm` is the point at which the compatibility guarantee
becomes load-bearing (see `TODO.md` Phase 9).

## Enforcement (the gate, not a wish)

- `CDM_SCHEMA_VERSION` is asserted equal to the crate's `CARGO_PKG_VERSION` by a
  unit test, so the embedded schema version cannot drift from the published crate
  SemVer.
- `cargo-semver-checks` runs in CI on `septicomics-cdm` (Phase 1 CI gate): a change
  that is semver-breaking fails the build unless the major is bumped.

## Changing the CDM

1. Make the change; let `cargo-semver-checks` classify it (breaking vs additive).
2. Bump `version` in `crates/cdm/Cargo.toml` accordingly (major for breaking).
3. The version test keeps `CDM_SCHEMA_VERSION` in sync — update it if needed.
4. A **major** bump is a governance event: it is a coordinated node-upgrade
   (`GOVERNANCE.md`: breaking CDM change is a high-bar Steering Committee decision).
