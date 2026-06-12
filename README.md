# septicomics

**Federated Sepsis Multi-Omics Knowledge Portal**

Name **locked**: `septicomics`. Verified free across crates.io, PyPI, npm, and
GitHub, with no existing brand or research-entity collision (contrast `septomics`,
which is the ZIK Septomics centre in Jena). Trade-offs accepted at lock-in: it is
commodity-forward rather than federation-forward, and carries a latent "septi·comics"
misread — both known and chosen.

A sepsis knowledge portal that borrows SeptiSearch's open, community-curated
derived-data layer and CMAISE's patient-level multi-omics depth, joined by a
**data-stays-home federation** so raw patient data never crosses a sovereignty
boundary.

---

## Problem (stated without tools)

Sepsis research has two useful but disconnected resource shapes. One is an open,
internationally browsable catalog of *derived* molecular knowledge (signatures,
gene-sets, summary statistics) that anyone can query and test their own
signatures against. The other is deep, patient-level multi-omics with clinical
and temporal phenotyping held in multicenter cohorts. The first is open
*because* it holds nothing identifiable; the second is closed *because* it holds
everything. No resource gives the openness of the first over the depth of the
second.

**What would make this design wrong:**

- If derived data alone answered the research questions, federation is wasted
  effort — you would just build a SeptiSearch and stop.
- If the work genuinely required moving raw patient records across borders, no
  architecture fixes that; it is a legal/diplomatic problem, not a software one.

This design is correct only in the middle case: questions that need
patient-level computation, answered by analyses that *travel to the data* and
return only aggregates.

## What it borrows

- **From SeptiSearch** — an open, international, community surface: browse and
  visualize curated derived objects, upload your own signature and enrich/compare
  it against the catalog, standardized cross-study metadata.
- **From CMAISE** — patient-level multi-omics (transcriptomics, proteomics,
  single-cell, …) with deep clinical and temporal phenotyping across multiple
  centers.
- **The bridge** (the new part) — a federated analysis layer in the spirit of
  DataSHIELD / GA4GH: queries execute inside each node against its raw data;
  only disclosure-controlled aggregates leave; cleared aggregates are *promoted*
  into the open derived catalog.

What the combination yields, and neither parent can: **patient-level analysis
pooled across multiple sovereign cohorts at once**, with no cohort exporting a
record — e.g. testing whether an endotype's prevalence and mortality association
replicate across cohorts in different jurisdictions. That capability is the
reason to build this rather than a second SeptiSearch.

## Two planes

**Open Derived Plane** — the public, international face. A read-mostly catalog of
signatures, gene-sets, endotype definitions, summary statistics, and trained
models, plus community tooling (signature upload, enrichment, comparison). Holds
no patient-level data and is therefore free to be globally open.

**Sovereign Raw Plane** — a federation of nodes. Each node holds raw
patient-level multi-omics + clinical data in its home jurisdiction and under its
own consent and data-sharing agreements. A node never exports raw records.

The **Federation Bridge** connects them: an orchestrator submits a typed
analysis plan to selected nodes, each node runs it locally, a disclosure-control
guard suppresses below-threshold aggregates, and the orchestrator assembles only
the cleared aggregates. A review/promotion step can publish an aggregate into the
Open Derived Plane.

## The load-bearing artifact: the Common Data Model (CDM)

The system's real contract is the shared sepsis schema every node must speak:
omics layers, sample/timepoint structure, inflammatory endotypes, clinical
phenotypes, and outcomes. The web app and the orchestrator are replaceable; the
CDM is not. It is versioned independently and strictly (a breaking schema change
is a major version bump). Everything downstream parses against it at the
boundary rather than validating ad hoc.

## Non-goals (YAGNI)

- Not a raw-data download portal. There is no "export the matrix" path, by design.
- Not a pathogen-genomics or surveillance system.
- Not a clinical decision tool or anything patient-facing.
- No bespoke federated-ML framework before standard federated *statistics* are
  shown to be insufficient.

## Stack (latest stable)

- **Rust** — federation orchestrator, node agent, and the CDM types. Library-first,
  binary-last; crate-per-concern in a workspace.
- **Python (uv)** — in-node federated statistical/omics compute, where the
  bio-analysis and federated estimators live. Pinned `uv.lock`, pinned seeds.
- **TypeScript (pnpm)** — the Open Derived Plane web portal.

Tooling note: every Rust crate ships with `cargo install cargo-skill` set up, and
SemVer is enforced in CI via `cargo-semver-checks`.

Naming convention under the locked name:

- Cargo workspace: `septicomics`. Member crates keep short concern-named
  directories but **publish prefixed** to avoid generic-name collisions on
  crates.io: `septicomics-cdm`, `septicomics-fedproto`, `septicomics-guard`,
  `septicomics-node`, `septicomics-orchestrator`.
- Python (uv) package: `septicomics` (in-node compute distributable).
- TypeScript (pnpm) workspace: scoped `@septicomics/*` (e.g. `@septicomics/web`).
- GitHub: org/repo handle `septicomics`.

## Licensing — DECIDE BEFORE ANY CODE

No source is written until this is settled. Proposed split, for discussion:

- **Service code** (orchestrator, node agent, web app): **AGPL-3.0** — a hosted
  federation is a network service; AGPL keeps modified deployments reciprocal and
  discourages closed forks of the federation itself.
- **CDM schema + client SDK**: **Apache-2.0** — maximize adoption; nodes and third
  parties must be able to depend on the contract without copyleft friction.
- **Curated derived catalog content** (signatures, summary stats): **CC-BY-4.0** —
  data is not code; attribution-only matches the open-knowledge intent.

Open questions to resolve first: (1) does AGPL on the orchestrator deter the very
institutions you want as nodes? (2) is a contributor agreement / DCO required?
(3) who is the data-governance custodian — this is a consortium decision, not a
repository setting.

## Status

Scaffold only. Architecture in `ARCHITECTURE.md`, work breakdown in `TODO.md`.
First code task (CDM crate) is blocked on the LICENSE decision above.

**Maturity caveat.** The pooled cross-cohort capability is latent until at least
two nodes federate. Before that, `septicomics` is functionally a SeptiSearch: the
open catalog works, but the cross-cohort pooling that makes it more than its
parents switches on only when the network forms. Sequence accordingly — the live
blockers are licensing and node onboarding, not additional features.
