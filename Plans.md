# Plans — septicomics

Project implementation phases, tracked in sync with `TODO.md`.

**Meta-rule:** A principle without a CI gate is decoration. Every phase ends with the
gate that makes its principle fail the build when violated.

---

## Phase 0 — Governance & licensing ⛔ HARD BLOCKER

**Status:** cc:blocked — All subsequent phases blocked until Phase 0 completes.

- [ ] **BLOCKER: Decide LICENSE split**
  - AGPL-3.0 service code (orchestrator, node agent, web app)
  - Apache-2.0 CDM schema + client SDK
  - CC-BY-4.0 curated derived catalog content
  - See README.md for rationale and open questions.
  - **No source code can be written until this is decided.**
  
- [ ] Decide contributor model (DCO vs CLA); record in `CONTRIBUTING.md`
- [ ] Name data-governance custodian and node-onboarding authority
- [ ] Draft node data-sharing agreement template and consent-scope vocabulary
- [ ] Write `SECURITY.md` and disclosure/coordinated-vulnerability policy

---

## Phase 1 — Common Data Model (`cdm` crate)

**Status:** cc:todo — Blocked by Phase 0.

**Why:** The CDM is the load-bearing artifact. Every node must speak the same sepsis
schema (omics layers, sample/timepoint structure, endotypes, clinical phenotypes,
outcomes). Substrate-swap test: replace "sepsis" with any other sovereignty-constrained
multi-omics disease — only CDM contents change, the federation plumbing is unchanged.

- [ ] Define omics-layer types (transcriptomics, proteomics, single-cell, etc.)
  - Newtype-wrapped, not bare String/f64
  
- [ ] Define sample / subject / timepoint structure
  - Temporal trajectories as first-class, not an afterthought
  
- [ ] Define inflammatory-endotype and clinical-phenotype enums
  - Exhaustive match; no loose strings
  
- [ ] Define outcome types (mortality, organ-failure, etc.)
  - With explicit units
  
- [ ] Implement boundary parsers (parse-don't-validate)
  - Raw input → CDM value or typed error
  - No half-validated states constructible
  
- [ ] Write CDM versioning policy; embed machine-readable schema version

- [ ] **CI gate: Lint & semver**
  - cargo fmt --check
  - clippy -D warnings
  - cargo-semver-checks (SemVer-critical contract)
  - doctests pass
  - cargo install cargo-skill wired into dev bootstrap

---

## Phase 2 — Federation protocol (`fed-protocol` crate)

**Status:** cc:todo — Blocked by Phase 0, depends on Phase 1.

**Why:** Binds independently-operated nodes; versions independently and strictly.
Breaking change = major version bump = coordinated node-upgrade event, not silent deploy.

- [ ] Define analysis-plan type
  - Cohort selector + omics layer(s) + allow-listed estimator variant (discriminated)
  - Free-form code must be unrepresentable
  
- [ ] Define aggregate-result type; make subject-level value impossible to construct on wire
  - The core invariant enforced in the type system
  
- [ ] Define node-capability advertisement
  - CDM major, protocol major, consent scopes
  
- [ ] Implement version-negotiation logic
  - Orchestrator refuses incompatible majors, never guesses

- [ ] **CI gate: Property tests + semver**
  - Property test: no fed-protocol egress type can encode a raw record
  - cargo-semver-checks on protocol crate

---

## Phase 3 — Disclosure-control guard (`disclosure-guard` crate)

**Status:** cc:todo — Blocked by Phase 0, depends on Phase 2.

**Why:** Pure function enforcing the core invariant: no subject-level data leaves a node.

- [ ] Implement small-N suppression (configurable k)
- [ ] Implement small-cell suppression
- [ ] Implement optional calibrated noise for sensitive estimators (feature-gated)
- [ ] Make guard pure: (aggregate, policy) → cleared | suppressed

- [ ] **CI gate: Property + mutation tests**
  - Property tests: suppression holds across generated inputs and policies
  - Mutation-test the threshold logic

---

## Phase 4 — Node agent (`node-agent` binary)

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 1-3.

**Why:** Brings everything together at the node boundary: plan intake, compute dispatch,
guard application, safe egress.

- [ ] Implement CDM storage port + one reference adapter (Postgres)
- [ ] Implement plan intake → compute dispatch → guard → aggregate egress
- [ ] Enforce least privilege
  - Agent holds no orchestrator credentials
  - Per-plan authorization against local consent scopes
  
- [ ] Implement structured audit log
  - Every plan received, decision, aggregate emitted

- [ ] **CI gate: Integration test**
  - Prove raw rows never appear in egress payloads
  - Golden test + fuzz on egress encoder

---

## Phase 5 — In-node compute runtime (Python, uv)

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 1-3.

**Why:** Where the federated statistics and bio-analysis live. Allow-listed estimators
only; reproducibility is non-negotiable.

- [ ] Implement allow-listed estimators v1
  - Differential expression
  - Survival
  - Endotype prevalence
  
- [ ] Implement Pydantic/msgspec parsing at plan/result boundary
  - Parse-don't-validate at every edge
  
- [ ] Ensure reproducibility
  - uv.lock committed
  - Seeds pinned
  - Estimator versions recorded in result provenance

- [ ] **CI gate: Type + determinism**
  - mypy/pyright strict
  - Deterministic-output test under fixed seed

---

## Phase 6 — Orchestrator (`orchestrator` binary)

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 1-5.

**Why:** Orchestrates the federation: plan validation, fan-out to nodes, aggregate
assembly. Orchestrator is untrusted by nodes — can't exfil raw data because it has no
return path for raw records.

- [ ] Implement plan validation against CDM + protocol before fan-out
- [ ] Implement fan-out to authorized nodes
- [ ] Collect cleared aggregates; assemble pooled / meta-analytic result
  - Never touching subject-level data
  
- [ ] Implement partial-failure handling
  - Node offline → report coverage, never block

- [ ] **CI gate: End-to-end + semver**
  - End-to-end test across ≥2 mock nodes
  - Semver-check

---

## Phase 7 — Open Derived Plane web portal (TypeScript, pnpm)

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 1-2.

**Why:** The public, international face of the federation. Browse, upload, enrich,
compare. Holds no patient data, so it can be globally open.

- [ ] Implement catalog browser
  - Promoted aggregates / curated signatures (read path)
  
- [ ] Implement signature upload + enrichment/comparison
  - The SeptiSearch borrow
  
- [ ] Implement visualization
  - Endotype / signature / summary views

- [ ] Apply strict typing at all I/O edges
  - Zod or Valibot validation
  - Branded types for catalog IDs
  - Audit every any/as as a leak site
  
- [ ] **CI gate: Typecheck + contracts**
  - Strict typecheck
  - Schema-contract test against cdm types
  - Lint

---

## Phase 8 — Promotion pipeline (Bridge → Open plane)

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 3, 6-7.

**Why:** One-way, human-reviewed promotion of cleared aggregates from federation into
the open catalog. Chesterton's Fence: the review step exists for a reason; do not
auto-promote.

- [ ] Implement one-way promotion flow
  - Cleared aggregate → disclosure re-review → provenance + attribution → versioned catalog entry
  
- [ ] Reject promotion lacking provenance or failing re-review
- [ ] **CI gate: Provenance + metadata**
  - Test: no catalog entry exists without provenance + license tag

---

## Phase 9 — Pilot federation & DevSecOps hardening

**Status:** cc:todo — Blocked by Phase 0, depends on Phases 1-8.

**Why:** Federation pooling is latent until ≥2 nodes federate. Publish stable releases.

- [ ] Stand up 2-node mock federation
  - Synthetic CDM data, end-to-end flow
  
- [ ] Threat-model orchestrator-untrusted assumption
  - Pen-test egress paths
  
- [ ] 12-Factor review of both services
  - Config, logs, disposability
  
- [ ] Supply-chain hardening in CI
  - cargo audit
  - pip-audit / uv audit
  - pnpm audit
  - SBOM generation
  - Pinned, reproducible builds
  
- [ ] Publish first stable release
  - cdm + client SDK as SemVer stable
  - Everything else stays pre-1.0 until protocol settles

---

## Cross-cutting CI gates (the meta-rule)

These gates apply across all phases. A principle without a gate is decoration.

| Principle | Mechanism that fails the build |
|---|---|
| Illegal states unrepresentable | Type-level egress invariant + property tests |
| Parse-don't-validate | Boundary parsers; no ad-hoc validation paths |
| SemVer on contracts | cargo-semver-checks on cdm and fed-protocol |
| Disclosure control | Property + mutation tests on disclosure-guard |
| No raw egress | Golden + fuzz tests on node egress encoder |
| Reproducibility | Locked deps, pinned seeds, deterministic-output test |
| Typed web edges | Strict typecheck + Zod/Valibot contract tests |
| Supply chain | Audit + SBOM gates in CI |

---

## Status summary

- **cc:blocked** (1 phase) — Phase 0 (licensing decision required before any code)
- **cc:todo** (8 phases) — Phases 1-9 (all waiting for Phase 0 to unblock)
- **cc:wip** (0 phases)
- **cc:done** (0 phases)

For details, see `TODO.md`.
