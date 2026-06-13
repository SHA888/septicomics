# Plans — septicomics

Harness-format task tracker. One row per atomic task (PR-sized). Status markers:
`cc:done` · `cc:wip` · `cc:todo` · `cc:blocked`. Detailed rationale in `TODO.md`.

**Meta-rule:** every phase ends with a CI gate that makes its principle fail the build.

**Active phase:** Phase 2 — Federation protocol (`fed-protocol` crate). Phases 0–1 complete.

---

## Phase 0 — Governance & licensing ✅

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 0.1 | Decide LICENSE split | `LICENSING.md` + canonical `LICENSE-*` texts committed | — | cc:done [591db30] |
| 0.2 | Decide contributor model (DCO vs CLA) | `CONTRIBUTING.md` records DCO + sign-off | — | cc:done [591db30] |
| 0.3 | Define governance model (custodian, onboarding authority) | `GOVERNANCE.md` model decided; appointments TBD | — | cc:done [591db30] |
| 0.4 | Node data-sharing agreement + consent-scope vocabulary | templates in `docs/governance/` | — | cc:done [591db30] |
| 0.5 | Security & disclosure policy | `SECURITY.md` with disclosure-control-bypass tier | — | cc:done [591db30] |

---

## Phase 1 — Common Data Model (`cdm` crate) — ✅ COMPLETE

The load-bearing contract every node must speak. SemVer-critical. Apache-2.0.

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 1.1 | Scaffold Cargo workspace + `cdm` crate | `cargo build` + `cargo test` pass on empty crate; Apache-2.0 declared; workspace member published-name `septicomics-cdm` | 0.1 | cc:done [4c5a092] |
| 1.2 | Omics-layer newtypes | transcriptomics/proteomics/single-cell etc. as newtypes; no bare `String`/`f64` for domain quantities; unit tests | 1.1 | cc:done [1e16c53] |
| 1.3 | Sample / subject / timepoint structure | temporal trajectories first-class; types + tests | 1.1 | cc:done [96bc6a5] |
| 1.4 | Endotype & clinical-phenotype enums | exhaustive enums; `match` exhaustiveness enforced; tests | 1.1 | cc:done [6b99412] |
| 1.5 | Outcome types with explicit units | mortality/organ-failure etc. with unit-typed values; tests | 1.1 | cc:done [6c64e08] |
| 1.6 | Boundary parsers (parse-don't-validate) | raw input → CDM value or typed error; no half-validated states constructible; round-trip + failure tests | 1.2, 1.3, 1.4, 1.5 | cc:done [3196b23] |
| 1.7 | CDM versioning policy + machine-readable schema version | embedded schema version constant; `docs/cdm-versioning.md`; doctest | 1.1 | cc:done [23d09c5] |
| 1.8 | CI gate (Phase 1) | `cargo fmt --check`, `clippy -D warnings`, `cargo-semver-checks`, doctests wired in CI; cargo-skill in dev bootstrap | 1.2–1.7 | cc:done [ccd03d3] |

---

## Phase 2 — Federation protocol (`fed-protocol` crate)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 2.1 | Analysis-plan type (cohort + layer + allow-listed estimator) | free-form code unrepresentable; tests | 1.6 | cc:done [0efe589] |
| 2.2 | Aggregate-result type; subject-level value unrepresentable on wire | no constructor yields exportable record; property test | 2.1 | cc:done [cf14012] |
| 2.3 | Node-capability advertisement (CDM/protocol major, consent scopes) | type + serde round-trip test | 1.7 | cc:todo |
| 2.4 | Version-negotiation (orchestrator refuses incompatible majors) | unit tests for accept/refuse | 2.3 | cc:todo |
| 2.5 | CI gate (Phase 2): no-record-encoding property test + semver | property test + `cargo-semver-checks` in CI | 2.2, 2.4 | cc:todo |

---

## Phase 3 — Disclosure-control guard (`disclosure-guard` crate)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 3.1 | Small-N + small-cell suppression (configurable k) | pure fn; unit tests | 2.2 | cc:todo |
| 3.2 | Optional calibrated noise (feature-gated) | feature flag; tests | 3.1 | cc:todo |
| 3.3 | Guard as pure `(aggregate, policy) → cleared \| suppressed` | no I/O; deterministic; tests | 3.1 | cc:todo |
| 3.4 | CI gate (Phase 3): property + mutation tests | suppression holds across generated inputs; mutation score gate | 3.3 | cc:todo |

---

## Phase 4 — Node agent (`node-agent` binary, Apache-2.0)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 4.1 | CDM storage port + Postgres reference adapter | port trait + adapter; integration test | 1.6 | cc:todo |
| 4.2 | Plan intake → compute dispatch → guard → egress | end-to-end path within node; test | 3.3, 4.1 | cc:todo |
| 4.3 | Least privilege (no orchestrator creds; per-plan consent authz) | authz checks against local scopes; tests | 4.2 | cc:todo |
| 4.4 | Structured audit log (plan, decision, aggregate) | append-only log; test | 4.2 | cc:todo |
| 4.5 | CI gate (Phase 4): no-raw-egress golden + fuzz | golden test + fuzz on egress encoder; assert no AGPL dep in closure | 4.2 | cc:todo |

---

## Phase 5 — In-node compute runtime (Python, uv, Apache-2.0)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 5.1 | Allow-listed estimators v1 (diff-expr, survival, endotype-prevalence) | estimators implemented; tests | 2.1 | cc:todo |
| 5.2 | Pydantic/msgspec parsing at plan/result boundary | parse-don't-validate; tests | 5.1 | cc:todo |
| 5.3 | Reproducibility (uv.lock, pinned seeds, provenance) | locked deps; seeds recorded in provenance | 5.1 | cc:todo |
| 5.4 | CI gate (Phase 5): mypy/pyright strict + determinism | strict typecheck + fixed-seed deterministic-output test | 5.2, 5.3 | cc:todo |

---

## Phase 6 — Orchestrator (`orchestrator` binary, AGPL-3.0-or-later)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 6.1 | Plan validation against CDM + protocol pre-fan-out | rejects invalid plans; tests | 2.4 | cc:todo |
| 6.2 | Fan-out to authorized nodes; collect cleared aggregates | concurrent fan-out; test with mock nodes | 6.1 | cc:todo |
| 6.3 | Assemble pooled / meta-analytic result (no subject data) | pooling logic; tests | 6.2 | cc:todo |
| 6.4 | Partial-failure handling (node offline → report coverage) | never blocks; coverage reported; test | 6.2 | cc:todo |
| 6.5 | CI gate (Phase 6): e2e across ≥2 mock nodes + semver | e2e test + `cargo-semver-checks` | 6.3, 6.4 | cc:todo |

---

## Phase 7 — Open Derived Plane web portal (TypeScript, pnpm, AGPL-3.0-or-later)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 7.1 | Catalog browser (promoted aggregates, signatures, read path) | browse UI over catalog; test | 2.3 | cc:todo |
| 7.2 | Signature upload + enrichment/comparison | upload + enrich flow; test | 7.1 | cc:todo |
| 7.3 | Visualization (endotype/signature/summary views) | views render; test | 7.1 | cc:todo |
| 7.4 | Strict typing at I/O edges (Zod/Valibot, branded IDs) | every edge validated; `any`/`as` audited | 7.1 | cc:todo |
| 7.5 | CI gate (Phase 7): typecheck + schema-contract vs cdm + lint | strict typecheck, contract test against `cdm`, lint in CI | 7.2, 7.3, 7.4 | cc:todo |

---

## Phase 8 — Promotion pipeline (Bridge → Open plane)

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 8.1 | One-way promotion (cleared → re-review → provenance → catalog) | promotion flow; test | 3.3, 6.3, 7.1 | cc:todo |
| 8.2 | Reject promotion lacking provenance / failing re-review | rejection path; test | 8.1 | cc:todo |
| 8.3 | CI gate (Phase 8): no catalog entry without provenance + license | contract test | 8.2 | cc:todo |

---

## Phase 9 — Pilot federation & DevSecOps hardening

| #   | Task | DoD | Depends | Status |
|-----|------|-----|---------|--------|
| 9.1 | 2-node mock federation, synthetic CDM data, e2e | end-to-end run green | 6.5, 4.5 | cc:todo |
| 9.2 | Threat-model orchestrator-untrusted; pen-test egress | threat model doc; egress pen-test results | 9.1 | cc:todo |
| 9.3 | 12-Factor review (config, logs, disposability) | review notes + fixes | 9.1 | cc:todo |
| 9.4 | Supply-chain (cargo/pip/pnpm audit, SBOM, reproducible builds) | audit + SBOM gates in CI; AGPL-free node-agent closure asserted | 9.1 | cc:todo |
| 9.5 | First SemVer-stable release of `cdm` + client SDK | tagged release; everything else pre-1.0 | 9.4 | cc:todo |

---

## Cross-cutting CI gates (the meta-rule)

| Principle | Gate |
|---|---|
| Illegal states unrepresentable | Type-level egress invariant + property tests |
| Parse-don't-validate | Boundary parsers; no ad-hoc validation paths |
| SemVer on contracts | `cargo-semver-checks` on `cdm` + `fed-protocol` |
| Disclosure control | Property + mutation tests on `disclosure-guard` |
| No raw egress | Golden + fuzz tests on node egress encoder |
| Reproducibility | Locked deps, pinned seeds, deterministic-output test |
| Typed web edges | Strict typecheck + Zod/Valibot contract tests |
| Supply chain | Audit + SBOM gates; node-agent dependency closure AGPL-free |

---

## Summary

- **cc:done** — Phase 0 (5/5) and Phase 1 (8/8)
- **cc:wip** — none (Phase 2 next)
- **cc:todo** — Phases 2–9
- **cc:blocked** — none

Live non-code blocker: node onboarding (cross-cohort capability latent until ≥2 nodes federate).
