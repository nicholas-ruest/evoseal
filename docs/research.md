# Research and candidate comparison — 2026-10-03

## Method

Two streams were refreshed: frontier work published in the preceding 60 days and enterprise OSS materially changed in the preceding 30 days. Dates refer to primary publication or inspected commit dates. Repositories were inspected at exact revisions; licenses were read from repository files.

## Frontier stream

| Date | Organization | Primary source | Mechanism and relevance |
|---|---|---|---|
| 2026-09-21 | Google Research | RRSI paper and `google-research/rrsi` | regularized proposal and Algorithm 2 selection to limit harness overfit |
| 2026-09-26 | Nanjing University et al. | Beyond the Model | harness effects vary by model and task; some components hurt repository generation |
| 2026-09-29 | UIUC / Salesforce Research et al. | Learning Meta-Skills | reusable meta-skills improve harness construction but remain evaluator-produced candidates |
| 2026-09-01 | multi-institution HarnessDev authors | HarnessDev | evolved harness gains are unstable and transfer partially to held-out tasks |
| 2026-09-24 | Microsoft | run-assert-eval | frozen before/after runtime-policy evaluation is useful prior art for consequential gating |

Primary URLs are recorded in the dated research Gist and factory receipt.

## Enterprise OSS radar

| Inspected date | Project | Revision | License | Code inspected |
|---|---|---|---|---|
| 2026-09-23 | Google RRSI | `be50316e1db05914068a973f322770ef08ed7ba1` | Apache-2.0 | `selection.py`, `schedule.py`, config, domain adapters |
| 2026-10-03 | Microsoft Agent Governance Toolkit | `c3e8229dfb19c697468cfb790495d7174ef8bc45` | MIT | policy engine, audit path, safety policy and tests |
| 2026-09-23 | NVIDIA NeMo Agent Toolkit | `c7e1162a1c7ff18bbd797e090a56cad97c281c92` | Apache-2.0 | evaluator/plugin API and evaluation workflow docs |
| 2026-10-03 | AWS CLI Agent Orchestrator | `0caec2c9fe13b44f43d1bb097e41a4264aca5d33` | Apache-2.0 | routing/supervision/plugin contracts and security fix revision |
| 2026-10-02 | Anthropic Claude Plugins Official | `d182ca456ca09d31d139f7d3818d1d333b103cce` | Apache-2.0 | marketplace schema, plugin scanner and license validation workflows |

## Ranked composition matrix

Scoring is 1–5 for novelty, enterprise pain, real integration fit, testability, and differentiation (25 maximum).

| Candidate | Novelty | Pain | Integration | Testability | Differentiation | Total | Decision |
|---|---:|---:|---:|---:|---:|---:|---|
| EvoSeal | 5 | 5 | 5 | 5 | 4 | 24 | selected |
| policy-trace firewall over Microsoft AGT | 3 | 5 | 4 | 4 | 3 | 19 | overlaps PlannerGate/PluginParity territory |
| durable agent saga over `pg_durable` | 4 | 4 | 3 | 2 | 4 | 17 | Cloud SQL extension compatibility unresolved |
| RRSI harness evolution runner | 2 | 4 | 4 | 2 | 2 | 14 | duplicates upstream; live roles need Vertex |

## Selected hypothesis

RRSI addresses selection overfit, but enterprise promotion also needs source identity, independent OOD/safety/cost gates, durable history, and tamper-evident receipts. RuVector and RVF are complementary rather than decorative: one supports prior-decision retrieval, the other binds evidence order. MetaHarness Darwin and Flywheel supply bounded candidate evaluation and replay without receiving promotion authority.

## Rejected alternatives

The generic governance firewall was rejected because Microsoft AGT already implements broad policy enforcement and previous Dream Machine work covers registration/authority boundaries. A `pg_durable` saga was not selected because production Cloud SQL compatibility is unverified. A thin RRSI wrapper was rejected as non-substantive and duplicative.

