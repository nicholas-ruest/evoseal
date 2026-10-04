# ADR-0003: RuVector decision memory plus RVF witness evidence

## Status

Accepted.

## Context

A decision needs both useful historical comparison and tamper-evident provenance. A vector store alone does not prove integrity; a hash chain alone does not support similarity-based retrieval. Decorative dependencies would weaken the design.

## Alternatives

1. Store JSON files only.
2. Use RuVector alone.
3. Use RVF alone.
4. Compose RuVector for decision history and RVF for request-to-decision binding.

## Decision

Choose option 4. Store an inspectable eight-dimensional decision feature vector in `ruvector-core 2.3.1`; query nearest prior decisions before insertion; read back the inserted ID. Hash canonical request and decision bytes with `rvf-crypto 0.2.0`, create a two-entry witness chain, verify it, and expose the root.

## Tradeoffs

Two integrations add compilation weight and operational checks. They serve non-overlapping functions and fail closed independently.

## Consequences

The local slice uses RuVector's file-backed backend for executable evidence. Production remains stateless and routes the intelligence layer through RuVector backed by Cloud SQL PostgreSQL; no service-owned database is allowed.

## Validation

Real adapter tests must insert/read back in RuVector, verify an RVF chain, and reject a tampered chain. Cargo dependency and license checks bind exact versions.

