# ADR-0001: Human-owned promotion authority

## Status

Accepted for the bounded MVP.

## Context

RRSI, Darwin, Flywheel, MetaHarness, neural systems, and memory systems can propose or score harness candidates. Allowing any evaluator to expand its own safety envelope creates a self-promotion loop and makes a favorable score indistinguishable from authorization.

## Alternatives

1. Let the highest-scoring evaluator auto-promote.
2. Require a quorum of evaluators but still auto-promote.
3. Emit a source-bound advisory decision with no authority.

## Decision

Choose option 3. Every report includes `authority: "none"`. A `PROMOTE` status means the frozen evidence gates passed; it never merges, publishes, deploys, or changes policy.

## Tradeoffs

This adds human latency and prevents fully autonomous optimization. It also keeps evaluator compromise or reward hacking from directly becoming a production change.

## Consequences

The CLI has no GitHub write, deployment, or policy-mutation adapter. Publication remains a separate authorized workflow.

## Validation

Inspect CLI output and dependency graph; tests must never observe a side effect beyond the configured evaluation ledger.

