# Consent-Scope Vocabulary — v0 (draft)

A **controlled vocabulary** by which a node declares what it authorizes. A node's consent
scope is a machine-readable declaration the node agent enforces locally and advertises to the
orchestrator (`ARCHITECTURE.md` §3, §6). The orchestrator only fans out plans that fall
within a node's declared scope; a node may always refuse even an in-scope plan.

This is **v0**, deliberately small (YAGNI). The vocabulary is versioned; adding terms is a
governance decision (`GOVERNANCE.md`). Terms are stable identifiers — renaming a term is a
breaking change.

## 1. Analysis classes (`analysis`)

What estimator families the node permits. Mirrors the allow-listed estimators
(`TODO.md` Phase 5); a node need not permit all of them.

| Term | Meaning |
|---|---|
| `differential-expression` | Group-contrast expression analysis over an omics layer. |
| `survival` | Time-to-event / survival association with an outcome. |
| `endotype-prevalence` | Prevalence of an inflammatory endotype in a cohort. |

*(Free-form / arbitrary code is never a member of this vocabulary — it is unrepresentable in
the protocol by design, `ARCHITECTURE.md` §3.)*

## 2. Omics layers (`layers`)

Which layers may be touched. Subset of the CDM's omics-layer types.

| Term | Meaning |
|---|---|
| `transcriptomics` | Bulk transcriptomic data. |
| `proteomics` | Proteomic data. |
| `single-cell` | Single-cell omics. |
| `[others as the CDM defines them]` | Declared against the CDM version the node speaks. |

## 3. Population scope (`population`)

The cohort boundary the node authorizes analyses over.

| Term | Meaning |
|---|---|
| `all-consented` | Any subject whose consent permits research reuse. |
| `cohort:<id>` | A specific named cohort the node curates. |
| `[exclusions]` | Node-specified carve-outs (e.g. withdrawn-consent subjects), enforced locally. |

## 4. Pooling permission (`pooling`)

Whether the node's cleared aggregates may be combined with other nodes' — the switch behind
the federation's differentiating capability.

| Term | Meaning |
|---|---|
| `pooling:allowed` | Cleared aggregates may enter cross-cohort pooled / meta-analytic results. |
| `pooling:denied` | Aggregates are returned but must not be pooled with other nodes. |

## 5. Promotion permission (`promotion`)

Whether cleared aggregates from this node may be submitted to the open catalog.

| Term | Meaning |
|---|---|
| `promotion:eligible` | Cleared aggregates may go through the promotion pipeline to the open plane. |
| `promotion:withheld` | Aggregates stay within federated results only; never published. |

## Example declaration

```json
{
  "consent_scope_version": "0",
  "cdm_major": 1,
  "protocol_major": 1,
  "analysis": ["differential-expression", "endotype-prevalence"],
  "layers": ["transcriptomics", "proteomics"],
  "population": "all-consented",
  "pooling": "allowed",
  "promotion": "eligible"
}
```

A node advertising the above will run differential-expression and endotype-prevalence over
its transcriptomic and proteomic data for consented subjects, permits its cleared aggregates
to be pooled across cohorts, and permits promotion to the open catalog — and will be refused
fan-out for anything outside this set.
