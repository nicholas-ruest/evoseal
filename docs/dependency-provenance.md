# Dependency provenance

| Dependency | Version / revision | License | Feature choice | Direction |
|---|---|---|---|---|
| `ruvector-core` | `=2.3.1` | MIT | `storage`, `hnsw`; excludes API/ONNX embeddings | adapter only |
| `rvf-crypto` | `=0.2.0` | Apache-2.0/MIT ecosystem | default standard-library path | adapter only |
| Google RRSI | commit `be50316e1db05914068a973f322770ef08ed7ba1` | Apache-2.0 | selection/schedule APIs only | process adapter |
| MetaHarness Darwin | source `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`, npm `0.10.3` | MIT | CLI evaluation | external evaluator |
| MetaHarness Flywheel | source `9ce8b8dd89045c3b9a1f809ae58f3589029db4a4`, npm `0.1.12` | MIT | ESM replay APIs | external evaluator |

All Rust dependencies are locked. Ruvnet crates never cross the application port boundary. npm artifacts did not publish `gitHead`; source and registry provenance are deliberately separate claims.

