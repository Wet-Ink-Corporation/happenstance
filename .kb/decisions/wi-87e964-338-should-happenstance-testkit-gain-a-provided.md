---
id: "kb-decision-wi-87e964"
title: "#338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server: settle-frontier-hook"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server?\", facing the question an agent raised, we decided for settle-frontier-hook."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-01"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-87e964"
question: "#338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server?"
door: one-way
decider: "human:claude"
provenance: user-directed
decided_at: "2026-11-01T00:59:05Z"
tree_hash: "97bfc1967c0b1d544eae97aedc2fc8be45f18033"
---

# #338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server: settle-frontier-hook

## Context and problem statement

#338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server?

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-87e964`. Anchor: `crates/happenstance-testkit/src/fixture.rs:1`.

## Decision outcome

Chosen option: **settle-frontier-hook**.

Decider's note: Ryan 2026-11-01 00:58Z in thread: '#338 : A'. Fixture::settle_frontier/Settled, provisional CF-42, ADR-0099; Ryan merges.

## Provenance

- Decided 2026-11-01T00:59:05Z by human:claude (user-directed), via chat.
- Raised in session `372d1deb-288a-5665-a0d6-d5c62b1d61c1`.
- Verbatim: "#338: should happenstance-testkit gain a provided Fixture::settle_frontier() -> Settled hook (additive, with a provisional CF-42 and ADR-0099), so that 83 read-back rules settle on Postgres and Neon instead of relying on a quiet server?"
