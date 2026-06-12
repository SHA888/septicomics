# Node Data-Sharing Agreement — TEMPLATE

> **Status: template / not legal advice.** This is a structural skeleton for the agreement a
> prospective node signs before joining the federation. It must be reviewed and completed by
> the node's and the consortium's legal counsel before use. Bracketed `[...]` fields are
> filled per node. See `GOVERNANCE.md` (Node-Onboarding Authority) and `ARCHITECTURE.md`.

## Parties

- **The Node:** `[institution legal name, jurisdiction]` ("the Node").
- **The Federation:** the septicomics consortium, governed per `GOVERNANCE.md`.

## 1. Recitals and intent

The Node operates raw patient-level sepsis multi-omics and clinical data within its own
jurisdiction, consent framework, and data-protection law. The Node wishes to participate in
federated analysis **without exporting any subject-level data**. This agreement records the
terms under which the Node runs the septicomics node agent and responds to analysis plans.

## 2. The non-export guarantee (load-bearing)

2.1. The Node runs analyses **locally**; **no subject-level datum leaves the Node**. Every
value the Node emits is an aggregate that has passed the disclosure-control guard
(`disclosure-guard`), consistent with the invariant in `ARCHITECTURE.md` §4.

2.2. The Federation and the orchestrator hold **no standing access** to the Node's raw data
and cannot compel its release. The orchestrator is untrusted by the Node by design.

2.3. The Node may **refuse any plan** at its sole discretion, per project, per consent scope,
without justification.

## 3. Consent scope

3.1. The Node declares its **consent scope** using the controlled vocabulary in
`docs/governance/CONSENT_SCOPE_VOCABULARY.md`: the analysis classes, population scope,
omics layers, and pooling permissions it authorizes.

3.2. The Node warrants that its declared scope is consistent with the consents and
data-sharing approvals governing its underlying data.

3.3. The Node may **narrow or withdraw** scope at any time; changes take effect on the
Node's next capability advertisement.

## 4. Disclosure control

4.1. The Node enforces at minimum the Federation's disclosure-control policy (small-N
suppression at threshold _k_ = `[value]`, small-cell suppression, and any calibrated-noise
policy for sensitive estimators) as set by the Data-Governance Custodian.

4.2. The Node may apply **stricter** local thresholds than the Federation minimum.

## 5. Technical compatibility

5.1. The Node advertises the CDM major and protocol major it speaks (`ARCHITECTURE.md` §6).
The orchestrator refuses incompatible fan-out rather than guessing.

5.2. Breaking CDM/protocol changes are coordinated node-upgrade events; the Node is given
reasonable notice per `GOVERNANCE.md`.

## 6. Audit and provenance

6.1. The Node maintains a structured audit log of every plan received, the decision taken,
and every aggregate emitted.

6.2. Aggregates carry provenance (estimator, version, seed, CDM version) sufficient for the
promotion pipeline's review.

## 7. Software and licensing

7.1. The Node operates the node agent and in-node compute runtime under **Apache-2.0**
(`LICENSING.md`). No rights to the Node's data are granted by this license.

7.2. Nothing in this agreement assigns ownership of the Node's data to any party.

## 8. Liability, term, and withdrawal

8.1. **[Liability, indemnity, and warranty terms — to be drafted by counsel.]**

8.2. The Node may **withdraw** from the Federation on `[notice period]`. On withdrawal the
Node ceases responding to plans; aggregates already promoted to the open catalog remain
published under their `CC-BY-4.0` terms with attribution.

8.3. **[Governing law / dispute resolution — per the parties' jurisdictions.]**

## 9. Security

9.1. The Node follows `SECURITY.md` for coordinated vulnerability disclosure and accepts
out-of-band notification of any disclosure-control bypass.

---

**Signatures**

- For the Node: `[name, title, date]`
- For the Federation (Node-Onboarding Authority): `[name, title, date]`
