# Dream Machine build — 2026-10-03 — EvoSeal

Status: PUBLICATION-READY DRAFT; not a published Gist.
Research Gist URL: BLOCKED_UNPUBLISHED

## Problem

Harness evolution can optimize against a narrow evaluator and then promote its own winner. EvoSeal is an advisory firewall: candidates must pass independent RRSI, OOD, safety, cost, provenance, persistence, witness, and replay checks before a human may approve promotion.

## Implemented system

A six-crate Rust workspace provides domain, application, Google RRSI adapter, RuVector/RVF adapter, CLI, and evaluation boundaries. The CLI accepts a real candidate packet, invokes pinned RRSI through a bounded process protocol, applies independent Rust gates, persists evidence through RuVector, emits and verifies an RVF witness chain, and returns inspectable JSON with `authority: none`.

Architecture graphic:
https://github.com/nicholas-ruest/evoseal/blob/66ac3809fceb997210bc042b3cf79000cbbcb849/docs/assets/evoseal-overview.svg

## Exact ingredients

- `ruvector-core = 2.3.1`, persistent file-backed VectorDB.
- `rvf-crypto = 0.2.0`, SHAKE256 witness chain.
- Google RRSI commit `be50316e1db05914068a973f322770ef08ed7ba1`, Apache-2.0.
- MetaHarness inspected source `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`; npm Darwin 0.10.3 and Flywheel 0.1.12. npm did not publish `gitHead`, so source/artifact equivalence is not claimed.

## Reproducible commands and results

All commands ran against the source later bound to validated evidence commit `66ac3809fceb997210bc042b3cf79000cbbcb849`.

- `cargo fmt --all -- --check` — exit 0.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` — exit 0.
- `cargo test --locked --workspace --all-targets --all-features` — exit 0.
- `RRSI_SOURCE=... cargo test --locked -p evoseal-adapter-rrsi --test upstream -- --ignored` — 1/1 passed.
- `cargo deny check` — advisories, bans, licenses, and sources passed.
- CLI vertical slice — exit 0; PROMOTE recommendation with `authority: none`; RuVector readback; RVF two-entry chain verified.
- Benchmark — 10,000 iterations; exit 0; score-only 1/4, RRSI-only 2/4, EvoSeal 4/4; debug-build fixture elapsed 9,547,085 ns.
- Darwin 0.10.3 — two generations, four children each, seed 20261003; baseline retained.
- Flywheel 0.1.12 — two generations; promotions 0; replay verdict pass; all receipt/parent/gate/sealed-field checks true.

## Publication and maturity

- Repository: https://github.com/nicholas-ruest/evoseal
- Review branch: https://github.com/nicholas-ruest/evoseal/tree/factory-2026-10-03-evoseal
- Validated commit: https://github.com/nicholas-ruest/evoseal/commit/66ac3809fceb997210bc042b3cf79000cbbcb849
- Main was not modified.
- No crate was deployed or published.
- Gist publication/readback remains blocked; this file is the preserved retry artifact.

## Limitations

The benchmark is a deterministic four-case mechanism fixture, not a production superiority claim. Live Vertex roles, Cloud SQL/IAM integration, realistic trace scale, cancellation under production load, and long-horizon distribution shift remain open.
