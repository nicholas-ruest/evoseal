# ADR-0002: RRSI through a bounded process adapter

## Status

Accepted.

## Context

Google RRSI is Python and exposes the selection and scheduling mechanisms required by EvoSeal. Reimplementing Algorithm 2 in Rust would create semantic drift; embedding Python in-process would widen the Rust service's dependency and failure surface.

## Alternatives

1. Reimplement RRSI selection in Rust.
2. Embed Python with PyO3.
3. Invoke a pinned checkout through a narrow JSON subprocess protocol.

## Decision

Choose option 3. The adapter verifies the checkout contains `rrsi/selection.py`, sets `PYTHONPATH` to the pinned root, invokes the project bridge, writes one JSON request, applies a hard timeout, and accepts one typed JSON response.

## Tradeoffs

Process startup adds latency and Python must be installed. In return, the exact upstream code executes and failures are isolated and observable.

## Consequences

Live proposer/critic roles remain out of scope without authorized Vertex credentials. The integration still executes the real `RRSIConfig`, `edit_budget`, and `select_round` APIs.

## Validation

Run the ignored upstream integration test with `RRSI_SOURCE` bound to commit `be50316e1db05914068a973f322770ef08ed7ba1`, plus the complete CLI example.

