# Licensing — septicomics

**Status: DECIDED (2026-06-12).** This resolves the Phase 0 ⛔ hard blocker. Source
code may now proceed against the per-component licenses below.

This repository is **multi-licensed by component**. Each component declares its license
via an SPDX identifier in its manifest (`Cargo.toml` `license = "..."`,
`pyproject.toml`, `package.json`), and the canonical license texts live at the repo root
(`LICENSE-AGPL-3.0`, `LICENSE-APACHE-2.0`, `LICENSE-CC-BY-4.0`).

## The decision

| Component | Directory / package | License | SPDX |
|---|---|---|---|
| CDM types + boundary parsers | `cdm` → `septicomics-cdm` | Apache-2.0 | `Apache-2.0` |
| Federation wire protocol | `fed-protocol` → `septicomics-fedproto` | Apache-2.0 | `Apache-2.0` |
| Disclosure-control guard | `disclosure-guard` → `septicomics-guard` | Apache-2.0 | `Apache-2.0` |
| **Node agent** | `node-agent` → `septicomics-node` | **Apache-2.0** | `Apache-2.0` |
| In-node compute runtime | Python (uv) `septicomics` | Apache-2.0 | `Apache-2.0` |
| Orchestrator | `orchestrator` → `septicomics-orchestrator` | **AGPL-3.0-or-later** | `AGPL-3.0-or-later` |
| Web portal (Open Derived Plane) | `@septicomics/web` | **AGPL-3.0-or-later** | `AGPL-3.0-or-later` |
| Curated derived catalog content | signatures, summary stats, gene-sets | CC-BY-4.0 | `CC-BY-4.0` |

Two tiers, by *who operates the software and what risk it carries*:

- **Permissive tier (Apache-2.0)** — everything an institution installs and runs **inside
  its own sovereign boundary**: the node agent, the in-node Python compute, and the three
  contract crates the node must link (`cdm`, `fed-protocol`, `disclosure-guard`).
- **Reciprocal tier (AGPL-3.0-or-later)** — the network services operated **centrally or
  publicly**: the orchestrator (the federation hub) and the web portal.
- **Content (CC-BY-4.0)** — curated catalog data. Data is not code; attribution-only.

## Why this differs from the README's first draft (and is better)

The `README.md` draft put **all service code, including the node agent, under AGPL** and
then raised the open question: *"does AGPL on the orchestrator deter the very institutions
you want as nodes?"*

This decision resolves that question by **moving the node agent (and in-node compute) to
Apache-2.0**, and keeping AGPL only on the orchestrator and web portal. The reasoning:

1. **What institutions vet is the node agent, not the orchestrator.** A prospective node's
   legal/IT team must approve the software it *installs inside its walls* — that is the node
   agent and the in-node compute runtime. Many hospitals, universities, and ministries
   maintain blanket bans or heavy review burdens on AGPL software. Putting the node agent
   under AGPL hands every prospective node a reason to say no *before evaluating the
   science*. Since "node onboarding" is one of the project's two live blockers
   (per `README.md`), this directly threatens the only path to the differentiating
   cross-cohort capability (latent until ≥2 nodes federate).

2. **AGPL's reciprocity concern does not apply to the node agent.** AGPL exists to stop a
   third party from running a *modified, closed, hosted fork* of a network service. The node
   agent is operated privately by each node against its own data — it is not a competing
   public service. There is no closed-SaaS-fork risk to defend against there. The risk *is*
   real for the **orchestrator** (the hub someone could re-host as a closed service) and the
   **web portal** (a public service) — so those stay AGPL.

3. **Institutions connecting a node are not bound by the orchestrator's license.** They
   interact with it over the wire; they neither distribute nor operate it. So AGPL on the
   orchestrator does not legally bind a node — and the node-agent license being permissive
   removes the perception barrier as well.

Net effect: **maximize node adoption** (permissive at the trust boundary institutions
actually evaluate) **while preserving federation reciprocity** exactly where SaaS-fork risk
lives (the hub and the public portal).

## License-compatibility check (the dependency direction is clean)

Apache-2.0 code may be incorporated into an AGPL-3.0 work; the reverse is not permitted.
Our dependency edges respect this one-way direction:

```
orchestrator (AGPL) ──depends on──▶ cdm (Apache), fed-protocol (Apache)     ✅ allowed
web portal   (AGPL) ──depends on──▶ cdm (Apache) types/contracts            ✅ allowed
node-agent (Apache) ──depends on──▶ cdm, fed-protocol, disclosure-guard      ✅ all Apache — no AGPL pulled in
python compute (Apache) ── standalone in-node runtime                        ✅ no AGPL deps
```

The node agent must **never** take an AGPL dependency, or it would become effectively AGPL
and defeat the adoption rationale above. In particular `disclosure-guard` is Apache-2.0
**by design**: wide scrutiny and reuse of disclosure-control logic is a public good, and it
must be linkable into the permissive node agent. CI should assert the node-agent dependency
closure contains no AGPL/GPL crate (see Phase 9 supply-chain gate).

## Contributor terms

All contributions are under the license of the component they touch, certified via the
**Developer Certificate of Origin** (see `CONTRIBUTING.md`). No CLA — no rights assignment
to any central entity. Rationale recorded in `CONTRIBUTING.md` and `GOVERNANCE.md`.

## Resolved open questions (from README §Licensing)

1. *Does AGPL deter prospective nodes?* — **Resolved.** Node-facing software is Apache-2.0;
   AGPL is confined to the centrally/publicly operated hub and portal.
2. *DCO vs CLA?* — **Resolved: DCO.** See `CONTRIBUTING.md`.
3. *Who is the data-governance custodian?* — **Model decided; appointments pending consortium
   ratification.** See `GOVERNANCE.md`.
