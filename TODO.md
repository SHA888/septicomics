# TODO — `septicomics`

Atomic task/subtask breakdown, ordered by SDLC phase. Checkbox = a unit small
enough to land in one PR. The **meta-rule** governs everything below: a principle
that is not enforced by a CI gate is decoration, so each phase ends with the gate
that makes its principle fail the build when violated.

Legend: `[ ]` todo · `[~]` blocked · ⛔ hard blocker.

---

## Phase 0 — Governance & licensing (blocks all code) — ✅ COMPLETE

- [x] Decide LICENSE split — **decided**: two-tier (Apache-2.0 for node-facing software
      incl. node agent + CDM/protocol/guard; AGPL-3.0-or-later for orchestrator + web
      portal; CC-BY-4.0 catalog). See `LICENSING.md` and root `LICENSE-*` files.
- [x] Decide contributor model (DCO vs CLA) — **DCO**; recorded in `CONTRIBUTING.md`.
- [x] Name the data-governance custodian and the node-onboarding authority — **model
      decided** (multi-stakeholder steering committee) in `GOVERNANCE.md`; named
      appointments `[TBD: consortium ratification]` (filled at node onboarding).
- [x] Draft the node data-sharing agreement template and consent-scope vocabulary —
      `docs/governance/NODE_DATA_SHARING_AGREEMENT.md`,
      `docs/governance/CONSENT_SCOPE_VOCABULARY.md`.
- [x] Write `SECURITY.md` and the disclosure/coordinated-vuln policy — done, with a
      dedicated *disclosure-control bypass* critical tier.

## Phase 1 — Common Data Model (`cdm` crate)

- [ ] Define omics-layer types (transcriptomics, proteomics, single-cell, …) as
      newtypes; no bare `String`/`f64` for domain quantities.
- [ ] Define sample / subject / timepoint structure (temporal trajectories are
      first-class, not an afterthought).
- [ ] Define inflammatory-endotype and clinical-phenotype enums (exhaustive `match`).
- [ ] Define outcome types (mortality, organ-failure, …) with explicit units.
- [ ] Implement boundary parsers (parse-don't-validate): raw input → CDM value or
      typed error; no half-validated states constructible.
- [ ] Write the CDM versioning policy and embed a machine-readable schema version.
- [ ] CI gate: `cargo fmt --check`, `clippy -D warnings`, `cargo-semver-checks`,
      doctests; `cargo install cargo-skill` wired into the dev bootstrap.

## Phase 2 — Federation protocol (`fed-protocol` crate)

- [ ] Define the analysis-plan type: cohort selector + omics layer(s) + an
      allow-listed estimator variant (discriminated; free code is unrepresentable).
- [ ] Define the aggregate-result type; make a subject-level value **impossible to
      construct on the wire** (the core invariant, in the type system).
- [ ] Define node-capability advertisement (CDM major, protocol major, consent scopes).
- [ ] Version-negotiation logic: orchestrator refuses incompatible majors.
- [ ] CI gate: property test that no `fed-protocol` egress type can encode a record;
      `cargo-semver-checks` on the protocol crate.

## Phase 3 — Disclosure-control guard (`disclosure-guard` crate)

- [ ] Implement small-N suppression (configurable _k_) and small-cell suppression.
- [ ] Implement optional calibrated noise for sensitive estimators (feature-gated).
- [ ] Make the guard a pure function over (aggregate, policy) → cleared | suppressed.
- [ ] CI gate: property tests asserting suppression holds across generated inputs
      and policies; mutation-test the threshold logic.

## Phase 4 — Node agent (`node-agent` binary)

- [ ] Implement the CDM storage port + one reference adapter (Postgres).
- [ ] Implement plan intake → in-node compute dispatch → guard → aggregate egress.
- [ ] Enforce least privilege: agent holds no orchestrator credentials; per-plan
      authorization checked against local consent scopes.
- [ ] Structured audit log of every plan received, decision, and aggregate emitted.
- [ ] CI gate: integration test proving raw rows never appear in egress payloads
      (golden test + fuzz on the egress encoder).

## Phase 5 — In-node compute runtime (Python, uv)

- [ ] Implement allow-listed estimators v1: differential expression, survival,
      endotype prevalence.
- [ ] Pydantic/msgspec parsing at the plan/result boundary (parse-don't-validate).
- [ ] Reproducibility: `uv.lock` committed, seeds pinned, estimator versions recorded
      in result provenance.
- [ ] CI gate: `mypy`/`pyright` strict, deterministic-output test under fixed seed.

## Phase 6 — Orchestrator (`orchestrator` binary)

- [ ] Plan validation against CDM + protocol before any fan-out.
- [ ] Fan-out to authorized nodes; collect cleared aggregates; assemble pooled /
      meta-analytic result without subject-level data.
- [ ] Partial-failure handling (a node offline → report coverage, never block).
- [ ] CI gate: end-to-end test across ≥2 mock nodes; semver-check.

## Phase 7 — Open Derived Plane web portal (TypeScript, pnpm)

- [ ] Catalog browser over promoted aggregates / curated signatures (read path).
- [ ] Signature upload + enrichment/comparison against the catalog (the SeptiSearch
      borrow).
- [ ] Visualization of endotype/signature/summary views.
- [ ] Zod/Valibot validation at every I/O edge; branded types for catalog IDs;
      audit every `any`/`as` as a leak site.
- [ ] CI gate: typecheck, schema-contract test against `cdm` types, lint.

## Phase 8 — Promotion pipeline (Bridge → Open plane)

- [ ] One-way promotion: cleared aggregate → disclosure re-review → provenance +
      attribution → versioned catalog entry.
- [ ] Reject promotion lacking provenance or failing re-review (Chesterton's Fence:
      the review step exists for a reason; do not auto-promote).
- [ ] CI gate: test that no catalog entry exists without provenance + license tag.

## Phase 9 — Pilot federation & DevSecOps hardening

- [ ] Stand up a 2-node mock federation with synthetic CDM data end-to-end.
- [ ] Threat-model the orchestrator-untrusted assumption; pen-test egress paths.
- [ ] 12-Factor review of both services (config, logs, disposability).
- [ ] Supply-chain: `cargo audit`, `pip-audit`/`uv` audit, pnpm audit in CI;
      SBOM generation; pinned, reproducible builds.
- [ ] Publish `cdm` + client SDK as the first SemVer-stable release; everything else
      stays pre-1.0 until the protocol settles.

---

## Cross-cutting CI gates (the meta-rule, in one place)

| Principle | Mechanism that fails the build |
|---|---|
| Illegal states unrepresentable | type-level egress invariant + property tests |
| Parse-don't-validate | boundary parsers; no ad-hoc validation paths |
| SemVer on contracts | `cargo-semver-checks` on `cdm` and `fed-protocol` |
| Disclosure control | property + mutation tests on `disclosure-guard` |
| No raw egress | golden + fuzz tests on node egress encoder |
| Reproducibility | locked deps, pinned seeds, deterministic-output test |
| Typed web edges | strict typecheck + Zod/Valibot contract tests |
| Supply chain | audit + SBOM gates in CI |

A convention without a red CI here is a wish, not a rule.
