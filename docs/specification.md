# Specification

## Original context

Agent-harness evolution systems can search prompts, tool policies, memory strategies, and orchestration code. Recent work shows both real gains and a recurring failure: a candidate can improve the evolve set while transferring poorly across tasks, models, or safety conditions. Existing optimizers are allowed to propose and score candidates; none should silently acquire promotion authority.

## Thesis

A candidate may be recommended for promotion only when all of the following are bound to exact source commits: upstream RRSI admissibility, in-domain floor, out-of-distribution regression ceiling, cost ceiling, zero new safety failures, a verified RVF witness chain, and a RuVector write/read-back receipt.

## Acceptance criteria

1. Accept a JSON request whose candidate and incumbent have exact 40-character commit SHAs.
2. Invoke Google RRSI `select_round` and `edit_budget` from the pinned checkout, with a hard timeout.
3. Reject invalid, missing, non-finite, or non-positive metrics at the boundary.
4. HOLD a candidate on any frozen RRSI, OOD, cost, or safety failure.
5. Create and verify a two-entry RVF witness chain over canonical request and decision bytes.
6. Insert the decision feature vector into persistent RuVector storage and verify it through read-back.
7. Return inspectable JSON with `authority: "none"`.
8. Compare combined behavior with score-only and RRSI-only baselines on the same frozen cases.

## Non-goals

- model training or live RRSI proposer execution;
- automatic merge, deployment, release, or policy learning;
- replacing MetaHarness, Darwin, Flywheel, RuVector, RVF, or RRSI;
- claiming benchmark transfer beyond the frozen fixture.

## Failure semantics

Invalid input fails before invoking upstreams. RRSI timeout or malformed output returns an explicit command failure. Witness verification or RuVector read-back failure blocks the report. No fallback converts missing evidence into a pass.

