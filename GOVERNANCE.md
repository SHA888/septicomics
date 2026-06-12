# Governance — septicomics

This document defines **how decisions are made** in septicomics and **who is accountable**
for the federation's two authorities: data governance and node onboarding. The model is
decided; the named appointments are marked **[TBD: consortium ratification]** because they
are a consortium decision, not a repository setting (per `README.md` and `TODO.md` Phase 0).

## Principle: governance mirrors the architecture

The architecture is decentralized — no node is privileged, and the orchestrator is
**untrusted by nodes** (`ARCHITECTURE.md` §2). Governance follows the same shape: authority
is **multi-stakeholder**, not vested in a single custodian who could become a single point
of control or failure. A single-custodian model is explicitly rejected because it would
contradict the sovereignty guarantee the whole system is built to provide.

## Bodies and roles

### Steering Committee (SC)

The federation's decision-making body. **Each participating node holds one seat.** The SC
ratifies the matters listed under "Decisions" below.

- **Quorum:** more than half of seated nodes.
- **Default rule:** rough consensus; where a vote is needed, a **two-thirds majority** of
  seated nodes carries, except for "high-bar" decisions (below) which require it explicitly.
- **Chair:** rotating among member nodes; the chair facilitates, holds no tie-breaking
  power over data-governance matters.
- Current seats: **[TBD: consortium ratification]**.

### Data-Governance Custodian (DGC)

Accountable for the **disclosure-control policy** (the `k` thresholds, suppression rules,
calibrated-noise policy in `disclosure-guard`), the CDM's handling of sensitive fields, and
the promotion-pipeline review standard. The DGC is a **role accountable to the SC**, not an
owner of data — it never holds raw data and cannot override a node's local decision to
refuse a plan.

- Held by: **[TBD: consortium ratification]** (recommended: a standing sub-committee, not a
  single individual, to avoid a single point of trust).
- Changes to disclosure thresholds are a **high-bar decision** (two-thirds SC majority).

### Node-Onboarding Authority (NOA)

Accountable for admitting new nodes: verifying a prospective node signs the data-sharing
agreement (`docs/governance/NODE_DATA_SHARING_AGREEMENT.md`), declares a valid consent scope
(`docs/governance/CONSENT_SCOPE_VOCABULARY.md`), and advertises a compatible CDM/protocol
major. The NOA cannot admit a node unilaterally over SC objection.

- Held by: **[TBD: consortium ratification]**.

### Security Contact

Receives and coordinates vulnerability reports per `SECURITY.md`, with authority to notify
node operators of disclosure-control bypasses before public disclosure.

- Contact: **[TBD: consortium ratification — set at first node onboarding]**.

### Maintainers

Accountable for code review and merge on a day-to-day basis. Maintainers enforce the CI
gates and the contribution rules (`CONTRIBUTING.md`); they do **not** decide governance
matters reserved to the SC.

## Decisions: who decides what

| Decision | Decided by | Bar |
|---|---|---|
| Day-to-day code merge | Maintainers | Standard review (`CONTRIBUTING.md`) |
| CDM **breaking** schema change (major bump) | SC | High-bar (two-thirds) — it is a coordinated node-upgrade event (`ARCHITECTURE.md` §6) |
| `fed-protocol` breaking change | SC | High-bar (two-thirds) |
| Disclosure-control threshold / policy change | DGC proposes → SC | High-bar (two-thirds) |
| Admitting a new node | NOA verifies → SC | Standard SC majority |
| Removing / suspending a node | SC | High-bar (two-thirds) |
| Promotion standard for the open catalog | DGC | Accountable to SC |
| License change to any component | SC | High-bar (two-thirds); see `LICENSING.md` |
| Adding an allow-listed estimator | Maintainers propose → DGC reviews disclosure risk → SC | Standard SC majority |

## Why a node can always say no

Nothing in this governance structure can compel a node to run a plan. A node's local consent
scope and data-sharing agreement are the final authority over its own data. The SC governs
the **shared contract and the federation's common policy**; it does not govern any node's
internal decisions. This is the governance expression of the architecture's least-privilege,
orchestrator-untrusted stance.

## Amending this document

Changes to `GOVERNANCE.md` itself are a high-bar SC decision. Until the consortium is
formed and the SC is seated, this document is a **proposed model**; the `[TBD]` appointments
are filled at the first node-onboarding event, which — together with licensing (now decided)
— is one of the project's two live blockers.
