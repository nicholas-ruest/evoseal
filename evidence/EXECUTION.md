# Execution evidence

Executor: RuOS machine `8ee959f7363638`, Node 24.21.0, Rust 1.99.0. All commands used the pinned PATH documented by the factory recovery runbook.

| Stage | Exact command | Exit |
|---|---|---:|
| Format | `cargo fmt --all -- --check` | 0 |
| Clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 0 |
| Tests | `cargo test --locked --workspace --all-targets --all-features` | 0 |
| Google RRSI | `RRSI_SOURCE=/home/ruv/.local/share/dream-machine/sources/rrsi cargo test --locked -p evoseal-adapter-rrsi --test upstream -- --ignored` | 0 |
| Dependency/security | `cargo deny check` | 0 |
| CLI vertical slice | `RRSI_SOURCE=... cargo run --locked -p evoseal-cli -- --input examples/promotion-request.json --output evidence/cli-report.json --ledger evidence/runtime/ruvector.redb` | 0 |
| Benchmark | `cargo run --locked -p evoseal-evaluation --bin evoseal-eval -- --dataset evaluation/frozen-cases.json --iterations 10000` | 0 |
| Darwin | `node @metaharness/darwin/dist/cli.js evolve-numeric . --genome evaluation/darwin-genome.json --evaluator "target/debug/darwin_evaluator evaluation/frozen-cases.json" --generations 2 --children 4 --concurrency 2 --seed 20261003 --sigma 0.15` | 0 |
| Flywheel | `METAHARNESS_FLYWHEEL_ENTRY=.../@metaharness/flywheel/dist/index.js node evaluation/flywheel-evaluate.mjs .` | 0 |

The final combined quality gate returned exit 0. Preliminary failures are preserved in the run ledger: incorrect RuVector root imports, missing Clap `env`, an obsolete cargo-deny setting, wildcard local dependency declarations, an affected `anyhow` 1.0.100 lock, and strict-cast warnings were corrected before the final gate. No failing result is represented as passing.

Key raw files: `raw/final-clippy.log`, `raw/final-test.log`, `raw/final-rrsi.log`, `raw/final-deny.log`, `raw/darwin-evaluation.log`, `benchmark.json`, `cli-report.json`, `flywheel-evaluation.json`, and `flywheel-replay.json`.
