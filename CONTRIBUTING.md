# Contributing to septicomics

Thank you for considering a contribution. This project is a sovereignty-respecting sepsis
multi-omics federation; please read `README.md` and `ARCHITECTURE.md` for the design
intent, and `LICENSING.md` for how components are licensed.

## Contributor model: DCO, not CLA

**Decided 2026-06-12.** Contributions are accepted under the
**[Developer Certificate of Origin (DCO) 1.1](https://developercertificate.org/)**.
We deliberately do **not** require a Contributor License Agreement.

Why DCO:

- **No central rights-holder is required.** A CLA assigns rights to a steward entity; this
  consortium has no single owner and does not intend to relicense the work. Requiring a CLA
  would force naming a rights-holding entity prematurely and centralize a federation that is
  architecturally decentralized.
- **Low friction for researchers.** Academic and institutional contributors can participate
  with a one-line sign-off rather than a legal-team review of a CLA.
- **It is the modern OSS default** (Linux kernel, GitLab, CNCF projects).

### How to sign off

Every commit must carry a `Signed-off-by` trailer matching the author, certifying the DCO:

```
git commit -s -m "your message"
```

This appends:

```
Signed-off-by: Your Name <your.email@example.com>
```

By signing off you certify the four DCO assertions (you wrote it or have the right to submit
it under the component's license). CI rejects commits without a valid sign-off.

> **Note on commit trailers:** Use `Signed-off-by` (DCO). Do **not** add `Co-Authored-By`
> trailers — commit attribution stays with the human author (see project convention in
> `CLAUDE.md`).

## License of your contribution

Your contribution is licensed under the license of the component you modify, as mapped in
`LICENSING.md`:

- Code in `cdm`, `fed-protocol`, `disclosure-guard`, `node-agent`, and the Python in-node
  runtime → **Apache-2.0**.
- Code in `orchestrator` and the web portal → **AGPL-3.0-or-later**.
- Curated catalog content → **CC-BY-4.0**.

**Critical compatibility rule:** the **node agent must never gain an AGPL/GPL dependency.**
A PR that introduces one into the node-agent dependency closure will fail CI and be rejected
regardless of other merits — it would silently relicense the software institutions install.

## Engineering disciplines (enforced by CI, not convention)

These are load-bearing; see `CLAUDE.md` and `TODO.md` for the gates that fail the build:

- **The safety invariant.** No subject-level datum leaves a node. Any change to node egress
  types, `disclosure-guard`, or the wire protocol is security-critical — see `SECURITY.md`.
- **Parse, don't validate.** Inputs are parsed into typed values at the boundary; no
  ad-hoc validation downstream.
- **SemVer on contracts.** `cdm` and `fed-protocol` version strictly; breaking changes are
  major bumps and coordinated node-upgrade events.
- **English-only for tracked files** (including `Plans.md` status markers, e.g. `cc:done`).

## Workflow

1. Open or claim an issue describing the change.
2. Branch from `main`; keep each PR small enough to review (one task from `Plans.md`).
3. Ensure the phase's CI gate passes locally before pushing.
4. Sign off all commits (`git commit -s`).
5. Submit a PR citing the `Plans.md` / `TODO.md` task it advances.

## Reporting security issues

Do **not** open a public issue for vulnerabilities — especially any disclosure-control
bypass. Follow `SECURITY.md`.
