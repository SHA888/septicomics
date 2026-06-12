# Architecture — `septicomics`

This document describes the structure, trust boundaries, and the one safety
invariant the whole design rests on. It assumes the framing in `README.md`.

## 1. Shape: ports & adapters around two planes

The system is hexagonal. The domain core is the **Common Data Model (CDM)** and
the **federated query protocol**; everything else — storage engines at nodes, the
transport between orchestrator and nodes, the web front end, identity providers —
is an adapter behind a port. This keeps the sovereign substrate swappable: a node
backed by Postgres, a columnar store, or a center's existing warehouse all
present the same CDM port.

```
                 ┌───────────────────────────────────────┐
                 │            Open Derived Plane           │   (international, open)
   public users  │  catalog · signature upload · enrich    │
  ───────────────▶  · visualize · compare · promoted aggs  │
                 └───────────────────────▲─────────────────┘
                                          │ promotion (reviewed, one-way)
                 ┌───────────────────────┴─────────────────┐
                 │            Federation Bridge             │
   researcher    │  orchestrator: plan → fan-out → assemble │
  ───────────────▶  disclosure-control guard (aggregate-only)│
                 └───┬───────────────┬───────────────┬──────┘
                     │ typed plan    │ typed plan    │  (no raw data returns)
            ┌────────▼──────┐ ┌──────▼────────┐ ┌────▼──────────┐
            │   Node A      │ │   Node B      │ │   Node C      │   (sovereign, closed)
            │ raw multi-omics│ │ raw multi-omics│ │ raw multi-omics│
            │ + clinical     │ │ + clinical     │ │ + clinical     │
            │ CDM adapter    │ │ CDM adapter    │ │ CDM adapter    │
            └────────────────┘ └────────────────┘ └────────────────┘
```

## 2. Trust boundaries (least privilege)

Each node is its own trust boundary and its own jurisdiction. The orchestrator is
**untrusted by nodes**: it may submit plans and receive cleared aggregates, and it
holds no standing access to raw data. A node decides, per project and per consent
scope, which plans it will run. The Open Derived Plane is fully public and trusts
nothing upstream except the promotion pipeline, which is human-reviewed.

Consequence: a compromised orchestrator can, at worst, attempt queries that nodes
are free to refuse and that the disclosure guard would suppress. It cannot exfil
raw records, because raw records have no return path.

## 3. The federated query, step by step

1. A researcher composes an **analysis plan** — a typed value describing cohort
   selection, the omics layer(s), and a statistical estimator drawn from an
   allow-listed catalog (e.g. differential expression, survival, endotype
   prevalence). Free-form code is *not* a plan.
2. The orchestrator validates the plan against the CDM and fans it out to the
   nodes the project is authorized for.
3. Each node parses the plan, runs the estimator locally over its raw data via the
   in-node compute runtime, and produces an aggregate.
4. The **disclosure-control guard** runs at the node boundary: aggregates derived
   from fewer than _k_ subjects are suppressed; cell counts below threshold are
   suppressed; optional calibrated noise for sensitive estimators.
5. The orchestrator assembles cleared per-node aggregates into a combined result
   (meta-analysis / pooled estimate), never touching subject-level data.
6. Optionally, a result is submitted to the **promotion pipeline**: reviewed for
   disclosure and provenance, then published into the Open Derived Plane with a
   versioned, attributed catalog entry.

## 4. The load-bearing invariant

> **No subject-level datum leaves a node. Every value crossing the node boundary
> is an aggregate that has passed the disclosure-control guard.**

This is not a convention — it is encoded so the build fails when violated
(see `TODO.md`, CI phase): the node-egress type makes a raw record
*unrepresentable* on the wire, and property tests assert the guard suppresses
below-threshold outputs across generated inputs. Make-illegal-states-unrepresentable
is doing the heavy lifting here: there is no constructor that produces an
exportable subject-level value.

## 5. Components and crate/package layout

Rust workspace (library-first, crate-per-concern). Directory names are the short
concern names below; published crate names carry the `septicomics-` prefix
(`cdm` → `septicomics-cdm`, etc.):

- `cdm` — the Common Data Model types and the JDN-free, boundary-parsing layer
  (parse-don't-validate). The contract crate; SemVer-critical.
- `fed-protocol` — analysis-plan and aggregate-result types; the wire contract.
- `disclosure-guard` — the suppression/threshold logic; pure, heavily property-tested.
- `node-agent` (binary) — receives plans, drives in-node compute, applies the guard.
- `orchestrator` (binary) — plan validation, fan-out, aggregate assembly.

Python (uv) package:

- in-node compute runtime implementing the allow-listed estimators; Pydantic/msgspec
  at the plan/result boundary; reproducibility via `uv.lock` and pinned seeds.

TypeScript (pnpm) workspace, mirroring domain boundaries:

- catalog browser, signature-upload + enrichment, visualization; Zod (or Valibot)
  at every I/O edge; discriminated unions + branded types for catalog records.

## 6. Versioning discipline (SemVer, must-have)

The CDM and `fed-protocol` are the contracts that bind independently-operated
nodes; they version strictly and independently. A breaking change to either is a
major bump and a coordinated node-upgrade event, not a silent deploy. Nodes
advertise which CDM/protocol majors they speak; the orchestrator refuses
incompatible fan-out rather than guessing.

## 7. Substrate-swap check (does the logic survive?)

Replace "sepsis" with any other sovereignty-constrained, heterogeneous,
multi-omics disease and the architecture is unchanged — only the CDM contents
(endotypes, ICU phenotypes, temporal structure) are sepsis-specific. That is the
honest test result: the federation + disclosure logic is disease-agnostic
plumbing; the sepsis value lives entirely in the CDM. The residue that does *not*
transfer is the domain schema, and that is where the real scientific work sits.

## 8. What this architecture deliberately cannot do

No raw-matrix export, no arbitrary user code at nodes, no patient-facing path.
These are not missing features; removing them is what makes the open plane open
and the sovereign plane lawful.
