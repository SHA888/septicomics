# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Conventions (hard rules)

- **No `Co-Authored-By:` trailers in commit messages.** This applies to all
  assistant-generated commits, including those produced by Claude Code or any other AI
  tool. Commit attribution stays with the human author. Boilerplate trailers add noise
  to the history without conveying meaningful authorship and have been retroactively
  stripped from past commits.
- **English-only for tracked files.**
  - All `Plans.md` content must be in English (headers, table columns, task
    descriptions, status markers).
  - No Japanese characters in `Plans.md` status markers (use `cc:done` instead of
    `cc:完了`, `cc:wip` instead of `cc:WIP`, etc).
  - All harness output and documentation must be in English.
  - This applies strictly to tracked files; commit to this constraint when editing `Plans.md`.
- **Pre-commit hooks are mandatory and cannot be bypassed.** All commits must pass:
  - `cargo fmt --check` (consistent formatting, Rust 2024 edition)
  - `cargo clippy -D warnings` (no warnings allowed)
  - `cargo test --all` (all tests pass)
  - `cargo-semver-checks` on `cdm` and `fed-protocol` (no breaking changes to contracts)
  - Invariant checks (no Co-Authored-By trailers, English-only Plans.md, no secrets)
  
  If a hook fails, **fix the issue—do not bypass.** See `docs/DEV_SETUP.md` for
  troubleshooting. All failures have straightforward solutions. The hooks mirror the
  CI pipeline exactly, so local validation prevents GitHub failures.

## Current state: scaffold only

There is **no source code yet** — only `README.md`, `ARCHITECTURE.md`, and `TODO.md`.
The first code task (the `cdm` crate, Phase 1) is **hard-blocked** on a governance
decision (see below). Do not scaffold crates, packages, or CI until that blocker clears.

When code does land, it spans three stacks, each with its own toolchain:

- **Rust** (`cargo`) — federation orchestrator, node agent, CDM + protocol types.
- **Python** (`uv`) — in-node federated statistical/omics compute.
- **TypeScript** (`pnpm`) — the Open Derived Plane web portal.

There is no monorepo build that drives all three; each is built and tested with its
native tool. Planned per-stack CI gates are enumerated in `TODO.md` (each SDLC phase
ends with the gate that makes its principle fail the build).

## The one invariant everything serves

> **No subject-level datum leaves a node. Every value crossing the node boundary is
> an aggregate that has passed the disclosure-control guard.**

This is not enforced by convention but by construction: the node-egress type must make
a raw subject-level record *unrepresentable on the wire* (no constructor produces an
exportable subject-level value), and property tests must assert the guard suppresses
below-threshold outputs. Any change touching `fed-protocol` egress types, the
`disclosure-guard`, or `node-agent` egress is changing the load-bearing safety
property — treat it as security-critical, not routine.

## Architecture in one breath

Two planes joined by a bridge:

- **Open Derived Plane** (TypeScript portal) — fully public catalog of *derived*
  objects (signatures, gene-sets, summary stats). Holds no patient data, so it can be
  globally open. Read-mostly; receives entries only via one-way, human-reviewed promotion.
- **Sovereign Raw Plane** — a federation of nodes, each holding raw patient-level
  multi-omics + clinical data in its own jurisdiction. A node **never** exports raw records.
- **Federation Bridge** — an orchestrator submits a *typed analysis plan* (cohort
  selector + omics layer + an **allow-listed estimator variant** — free-form code is
  unrepresentable) to authorized nodes; each node runs it locally, the disclosure
  guard suppresses below-threshold aggregates, and the orchestrator assembles only
  cleared aggregates into a pooled result.

The orchestrator is **untrusted by nodes**: it holds no standing access to raw data and
each node decides per-plan whether to run. A compromised orchestrator cannot exfiltrate
records because records have no return path.

Hexagonal/ports-and-adapters: the domain core is the CDM + the federated query protocol;
storage engines, transport, web front end, and identity are all adapters behind ports, so
a node's substrate (Postgres / columnar / existing warehouse) is swappable.

## The load-bearing artifact: the CDM

The real contract is the **Common Data Model** (`cdm` crate) — the shared sepsis schema
every node must speak (omics layers, sample/timepoint structure, endotypes, clinical
phenotypes, outcomes). The web app and orchestrator are replaceable; the CDM is not.
Substrate-swap test: replace "sepsis" with any other sovereignty-constrained multi-omics
disease and only the CDM contents change — the federation + disclosure plumbing is
disease-agnostic. The scientific value lives entirely in the CDM.

## Two non-negotiable engineering disciplines

1. **Parse, don't validate.** Raw input is parsed into a CDM/protocol value or a typed
   error at the boundary; there are no half-validated states and no ad-hoc validation
   paths downstream. Rust boundary parsers; Pydantic/msgspec at the Python plan/result
   boundary; Zod/Valibot at every TypeScript I/O edge (audit every `any`/`as` as a leak site).
2. **SemVer on the contracts.** `cdm` and `fed-protocol` bind independently-operated
   nodes. They version strictly and independently; a breaking change is a major bump and
   a coordinated node-upgrade event, **not** a silent deploy. Nodes advertise which
   CDM/protocol majors they speak; the orchestrator refuses incompatible fan-out rather
   than guessing. `cargo-semver-checks` gates these crates.

## Crate / package naming (collision-avoidance is deliberate)

Directories use short concern names; **published** Rust crates carry the `septicomics-`
prefix to avoid generic-name collisions on crates.io:

| Directory | Published crate | Role |
|---|---|---|
| `cdm` | `septicomics-cdm` | CDM types + boundary parsers; SemVer-critical contract |
| `fed-protocol` | `septicomics-fedproto` | analysis-plan + aggregate-result wire types |
| `disclosure-guard` | `septicomics-guard` | pure suppression/threshold logic; property-tested |
| `node-agent` | `septicomics-node` | binary: plan intake → compute → guard → egress |
| `orchestrator` | `septicomics-orchestrator` | binary: validate → fan-out → assemble |

- Python (uv) package: `septicomics`.
- TypeScript (pnpm) workspace: scoped `@septicomics/*` (e.g. `@septicomics/web`).
- The name `septicomics` is **locked** — verified free across crates.io/PyPI/npm/GitHub.

## Hard blocker — read before writing any code

`TODO.md` Phase 0 is a ⛔ gate: the **LICENSE split must be decided first**
(proposed: AGPL-3.0 service code / Apache-2.0 CDM+SDK / CC-BY-4.0 catalog content).
The README states plainly: *no source is written until this is settled.* The live
blockers for this project are **licensing and node onboarding**, not additional features.
If asked to start the `cdm` crate, confirm the LICENSE decision has been made first.

## Explicit non-goals (do not build these)

Not a raw-data download/export portal (there is no "export the matrix" path, by design);
not a pathogen-genomics or surveillance system; not a clinical decision or patient-facing
tool; no bespoke federated-ML framework before standard federated *statistics* are shown
insufficient. Removing these is what makes the open plane open and the sovereign plane lawful.

## Maturity caveat

The cross-cohort pooling that makes this more than a second SeptiSearch is **latent until
at least two nodes federate**. Before that, the open catalog works but the differentiating
capability is dormant. Sequence work accordingly.
