# Security Policy — septicomics

septicomics federates patient-level sepsis multi-omics data across sovereign nodes. Its
entire reason to exist rests on **one safety invariant**:

> **No subject-level datum leaves a node. Every value crossing the node boundary is an
> aggregate that has passed the disclosure-control guard.**

A defect that breaks this invariant is not a normal bug — it is the most severe class of
issue this project can have. This policy reflects that.

## Severity tiers

| Tier | Definition | Examples |
|---|---|---|
| **Critical — disclosure-control bypass** | Any path by which subject-level data, or an aggregate below the disclosure threshold, can leave a node. | A node-egress type that can encode a raw record; the guard failing to suppress a small-N aggregate; a plan that coerces a node into returning row-level data. |
| **Critical (standard)** | Remote code execution, authentication bypass, secret disclosure, arbitrary code execution at a node. | Plan injection that runs non-allow-listed code in the in-node runtime. |
| **High** | Trust-boundary or privilege violation that does not directly leak subject data. | Orchestrator gaining standing access it should not hold; missing per-plan consent-scope authorization. |
| **Medium** | DoS, integrity issues, audit-log gaps. | A node can be forced offline; aggregate provenance can be forged. |
| **Low** | Hardening gaps with limited impact. | Verbose error messages, missing rate limits. |

Any report describing a way to move a **subject-level value past the node boundary** is
treated as **Critical — disclosure-control bypass** by default, even if a working exploit is
not yet demonstrated.

## Reporting a vulnerability

**Do not open a public issue, PR, or discussion for a suspected vulnerability.**

Report privately via GitHub Security Advisories ("Report a vulnerability" on the repository
Security tab) once the repository is published, or to the security contact named in
`GOVERNANCE.md` (**[TBD: consortium security contact — to be set at node onboarding]**).

Please include:

- The component and version/commit.
- Which invariant or trust boundary is affected (cite `ARCHITECTURE.md` §2 or §4 if useful).
- Reproduction steps or a proof-of-concept, and the data/consent scope involved.
- Whether any real patient data was involved (if so, say so immediately and prominently).

## Coordinated disclosure timeline

| Stage | Target |
|---|---|
| Acknowledgement of report | within **3 business days** |
| Initial severity triage | within **7 days** |
| Fix or mitigation for Critical | as fast as feasible; **node operators notified before public disclosure** |
| Public disclosure | coordinated with the reporter, normally within **90 days**, sooner for actively exploited issues |

For **disclosure-control bypass**, node operators in the federation are notified through the
governance channel **before** public disclosure so they can pause affected analyses. The
federation's design means a fix may require a coordinated node upgrade (see the SemVer
discipline in `ARCHITECTURE.md` §6).

## Scope

In scope: all code in this repository — `cdm`, `fed-protocol`, `disclosure-guard`,
`node-agent`, `orchestrator`, the in-node Python runtime, and the web portal — plus the
CI/supply-chain configuration.

Out of scope: a node operator's own infrastructure, network, and data-handling outside the
node agent; third-party services; and the contents of any node's raw data store (which this
project never has access to by design).

## Safe harbor

Good-faith security research that respects this policy, avoids privacy violations and
service disruption, and uses only synthetic or properly authorized data will not be pursued
as a policy violation. **Never** test against real patient data you are not authorized to
access; use the synthetic CDM fixtures (Phase 9) instead.
