# Plans — septicomics

Atomic implementation phases. Tracked in sync with `TODO.md`.

**Meta-rule:** Every phase ends with a CI gate that makes its principle fail the build.

---

## Phase 0 — Governance & licensing ⛔ HARD BLOCKER

**Status:** cc:blocked

All subsequent phases blocked until Phase 0 completes.

- [ ] **Decide LICENSE split** (AGPL-3.0 service / Apache-2.0 CDM+SDK / CC-BY-4.0 catalog)
- [ ] Decide contributor model (DCO vs CLA); record in `CONTRIBUTING.md`
- [ ] Name data-governance custodian and node-onboarding authority
- [ ] Draft node data-sharing agreement template and consent-scope vocabulary
- [ ] Write `SECURITY.md` and disclosure/coordinated-vulnerability policy

---

## Phase 1 — Common Data Model (`cdm` crate)

**Status:** cc:todo (blocked by Phase 0)

The load-bearing contract. Every node speaks the same sepsis schema.

- [ ] Define omics-layer types (transcriptomics, proteomics, single-cell) as newtypes
- [ ] Define sample/subject/timepoint structure (temporal trajectories first-class)
- [ ] Define inflammatory-endotype and clinical-phenotype enums (exhaustive)
- [ ] Define outcome types (mortality, organ-failure, etc.) with explicit units
- [ ] Implement boundary parsers: raw input → CDM or typed error (parse-don't-validate)
- [ ] Write CDM versioning policy; embed machine-readable schema version
- [ ] **CI gate:** cargo fmt, clippy -D warnings, cargo-semver-checks, doctests, cargo-skill bootstrap

---

## Phase 2 — Federation protocol (`fed-protocol` crate)

**Status:** cc:todo (depends on Phase 1)

Analysis plans and aggregate results. Versions independently and strictly.

- [ ] Define analysis-plan type (cohort selector + omics layer + allow-listed estimator)
- [ ] Define aggregate-result type; make subject-level values unrepresentable on wire
- [ ] Define node-capability advertisement (CDM/protocol major, consent scopes)
- [ ] Implement version-negotiation logic (orchestrator refuses incompatible majors)
- [ ] **CI gate:** Property test (no raw records can be encoded); cargo-semver-checks

---

## Phase 3 — Disclosure-control guard (`disclosure-guard` crate)

**Status:** cc:todo (depends on Phase 2)

Pure suppression logic enforcing the core invariant.

- [ ] Implement small-N suppression (configurable k) and small-cell suppression
- [ ] Implement optional calibrated noise for sensitive estimators (feature-gated)
- [ ] Make guard pure: (aggregate, policy) → cleared | suppressed
- [ ] **CI gate:** Property tests (suppression holds across inputs/policies); mutation-test thresholds

---

## Phase 4 — Node agent (`node-agent` binary)

**Status:** cc:todo (depends on Phases 1-3)

Plan intake, compute dispatch, guard application, safe egress.

- [ ] Implement CDM storage port + one reference adapter (Postgres)
- [ ] Implement plan intake → compute dispatch → guard → aggregate egress
- [ ] Enforce least privilege (no orchestrator credentials; per-plan authorization)
- [ ] Implement structured audit log (plans received, decisions, aggregates emitted)
- [ ] **CI gate:** Integration test (raw rows never in egress); golden test + fuzz on egress encoder

---

## Phase 5 — In-node compute runtime (Python, uv)

**Status:** cc:todo (depends on Phases 1-3)

Allow-listed estimators. Reproducibility non-negotiable.

- [ ] Implement allow-listed estimators v1 (differential expression, survival, endotype prevalence)
- [ ] Implement Pydantic/msgspec parsing at plan/result boundary (parse-don't-validate)
- [ ] Ensure reproducibility (uv.lock committed, seeds pinned, estimator versions in provenance)
- [ ] **CI gate:** mypy/pyright strict, deterministic-output test under fixed seed

---

## Phase 6 — Orchestrator (`orchestrator` binary)

**Status:** cc:todo (depends on Phases 1-5)

Plan validation, fan-out to nodes, aggregate assembly.

- [ ] Implement plan validation against CDM + protocol before fan-out
- [ ] Implement fan-out to authorized nodes; collect cleared aggregates
- [ ] Assemble pooled / meta-analytic result (no subject-level data)
- [ ] Implement partial-failure handling (node offline → report coverage, never block)
- [ ] **CI gate:** End-to-end test across ≥2 mock nodes; semver-check

---

## Phase 7 — Open Derived Plane web portal (TypeScript, pnpm)

**Status:** cc:todo (depends on Phases 1-2)

Public, international catalog browser & signature tools.

- [ ] Implement catalog browser (promoted aggregates, curated signatures, read path)
- [ ] Implement signature upload + enrichment/comparison (the SeptiSearch borrow)
- [ ] Implement visualization (endotype, signature, summary views)
- [ ] Apply strict typing at all I/O edges (Zod/Valibot validation, branded types, audit any/as)
- [ ] **CI gate:** Strict typecheck, schema-contract test vs cdm types, lint

---

## Phase 8 — Promotion pipeline (Bridge → Open plane)

**Status:** cc:todo (depends on Phases 3, 6-7)

One-way, human-reviewed promotion of cleared aggregates to open catalog.

- [ ] Implement one-way promotion (cleared aggregate → disclosure re-review → provenance → catalog entry)
- [ ] Reject promotion lacking provenance or failing re-review
- [ ] **CI gate:** Test (no catalog entry exists without provenance + license tag)

---

## Phase 9 — Pilot federation & DevSecOps hardening

**Status:** cc:todo (depends on Phases 1-8)

Federation pooling, stable releases, supply-chain hardening.

- [ ] Stand up 2-node mock federation (synthetic CDM data, end-to-end)
- [ ] Threat-model orchestrator-untrusted assumption; pen-test egress paths
- [ ] 12-Factor review (config, logs, disposability)
- [ ] Supply-chain hardening: cargo audit, pip-audit, pnpm audit, SBOM, pinned reproducible builds
- [ ] Publish cdm + client SDK as SemVer stable (everything else pre-1.0)

---

## Cross-cutting CI gates (meta-rule)

A principle without a gate is decoration.

| Principle | Gate |
|---|---|
| Illegal states unrepresentable | Type-level egress invariant + property tests |
| Parse-don't-validate | Boundary parsers; no ad-hoc validation |
| SemVer on contracts | cargo-semver-checks on cdm + fed-protocol |
| Disclosure control | Property + mutation tests on disclosure-guard |
| No raw egress | Golden + fuzz tests on node egress encoder |
| Reproducibility | Locked deps, pinned seeds, deterministic-output test |
| Typed web edges | Strict typecheck + Zod/Valibot contract tests |
| Supply chain | Audit + SBOM gates in CI |

---

## Summary

- **cc:blocked** (1 phase) — Phase 0 (licensing decision required)
- **cc:todo** (8 phases) — Phases 1-9 (awaiting Phase 0)
- **cc:wip** (0 phases)
- **cc:done** (0 phases)

For detailed context and rationale, see `TODO.md`.
