# Design / implementation traceability

| Acceptance criterion | Context | Crate / artifact | ADR | Test or evidence |
|---|---|---|---|---|
| exact source input | Evidence Intake | `evoseal-domain` | 0001 | domain validation tests |
| real RRSI call | Regularized Selection | `evoseal-adapter-rrsi`, bridge | 0002 | ignored upstream integration test, run explicitly |
| OOD/cost/safety HOLD | Promotion Policy | `evoseal-domain` | 0001 | domain + frozen evaluation tests |
| append/read-back history | Decision Memory | `evoseal-adapter-ruvnet` | 0003 | `real_ruvector_write_is_read_back` |
| verified witness | Provenance | `evoseal-adapter-ruvnet` | 0003 | witness and tamper test |
| usable vertical slice | Orchestration | `evoseal-cli` | 0002 | executed example command |
| baseline and ablations | Evaluation | `evoseal-evaluation` | 0001 | frozen dataset + benchmark output |
| no self-promotion | all | report authority field | 0001 | output read-back |

