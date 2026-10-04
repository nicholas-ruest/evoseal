# Dream Machine research — 2026-10-03 — EvoSeal

Status: PUBLICATION-READY DRAFT; not a published Gist.

## Selected hypothesis

Evolving agent harnesses need an independent promotion firewall. Google RRSI reduces evaluator overfit during candidate selection; EvoSeal adds source identity, out-of-distribution, safety, cost, durable-history, tamper-evident witness, replay, and human-approval gates. Evaluators may propose or score, but carry no promotion authority.

## Frontier research (60-day window)

- Google Research, RRSI, 2026-09-21: https://arxiv.org/abs/2609.24972
- HarnessDev collaboration, 2026-09-01: https://arxiv.org/abs/2609.01437
- UIUC / Salesforce Research, Learning Meta-Skills, 2026-09-29: https://arxiv.org/abs/2609.38143
- HarnessEvolve collaboration, 2026-09-01: https://arxiv.org/abs/2609.00829

## Enterprise OSS radar (30-day window)

| Project | Inspected revision | Date | License | Code inspected |
|---|---|---:|---|---|
| Google RRSI | `be50316e1db05914068a973f322770ef08ed7ba1` | 2026-09-23 | Apache-2.0 | selection, schedule, config, adapters |
| Microsoft Agent Governance Toolkit | `c3e8229dfb19c697468cfb790495d7174ef8bc45` | 2026-10-03 | MIT | policy, audit, safety, tests |
| NVIDIA NeMo Agent Toolkit | `c7e1162a1c7ff18bbd797e090a56cad97c281c92` | 2026-09-23 | Apache-2.0 | evaluator/plugin APIs |
| AWS CLI Agent Orchestrator | `0caec2c9fe13b44f43d1bb097e41a4264aca5d33` | 2026-10-03 | Apache-2.0 | routing, supervision, plugins |
| Anthropic Claude Plugins Official | `d182ca456ca09d31d139f7d3818d1d333b103cce` | 2026-10-02 | Apache-2.0 | marketplace schema and validation |

## Ranked candidate matrix

| Candidate | Score / 25 | Decision |
|---|---:|---|
| EvoSeal | 24 | selected |
| Policy-trace firewall | 19 | rejected: overlaps existing governance work |
| Durable agent saga | 17 | rejected: Cloud SQL compatibility unresolved |
| Thin RRSI runner | 14 | rejected: non-substantive duplication |

## Composition

- RuVector core 2.3.1: persistent decision history and prior retrieval.
- RVF crypto 0.2.0: ordered SHAKE256 witness chain and tamper detection.
- Google RRSI at `be50316...`: real bounded Python process adapter.
- MetaHarness Darwin 0.10.3 and Flywheel 0.1.12; inspected source `9ce8b8d...`: candidate search and replay, never promotion authority.

## Evaluation

Frozen four-case fixture:

| Strategy | Correct | Unsafe promotions |
|---|---:|---:|
| score-only | 1/4 | 3 |
| RRSI-only | 2/4 | 2 |
| EvoSeal | 4/4 | 0 |

This is mechanism evidence, not a production-performance claim. Darwin retained the baseline after two bounded generations. Flywheel completed two generations with replay verification; the custom gate rejected all self-promotion with `HUMAN_APPROVAL_REQUIRED`.

## Repository

- Repository: https://github.com/nicholas-ruest/evoseal
- Review branch: https://github.com/nicholas-ruest/evoseal/tree/factory-2026-10-03-evoseal
- Validated evidence commit: https://github.com/nicholas-ruest/evoseal/commit/66ac3809fceb997210bc042b3cf79000cbbcb849

## Limitations

Live Vertex-backed RRSI proposer/critic roles were not exercised. Production IAM, Cloud SQL-backed RuVector deployment, representative production traces, and long-horizon regression remain unvalidated. Promotion remains human-owned.
