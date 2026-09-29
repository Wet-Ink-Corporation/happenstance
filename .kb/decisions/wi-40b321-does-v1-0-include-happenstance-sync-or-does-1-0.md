---
id: "kb-decision-wi-40b321"
title: "Sync inside 1.0"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Does v1.0 include happenstance-sync, or does 1.0 cover core, adapters and the typed layer with sync on its own 0.x line?\", facing it sets phase 21's dependencies and the length of the road to 1.0, we decided for Sync inside 1.0 and neglected Sync outside 1.0, on the premise that replication is the headline for first adopters, accepting that if wrong: 1.0 slips about 17 working days on the least-settled piece."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-40b321-does-v1-0-include-happenstance-sync-or-does-1-0.md
last_reviewed: "2026-09-28"
reversibility: low
phase: 15
supersedes: null
superseded_by: null
weighin_item: "wi-40b321"
question: "Does v1.0 include happenstance-sync, or does 1.0 cover core, adapters and the typed layer with sync on its own 0.x line?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-28T19:42:30Z"
tree_hash: "cd4f937c02cc1d262dad3a512411a08b46ebe484"
recommended: "A"
flip_condition: "replication being the headline for the first adopters"
---

# Sync inside 1.0

## Context and problem statement

Does v1.0 include happenstance-sync, or does 1.0 cover core, adapters and the typed layer with sync on its own 0.x line?

It sets phase 21's dependencies and the length of the road to 1.0.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-40b321`. Anchor: `RUNBOOK.md:5492`.

## Decision drivers

- Solo days to 1.0
- Risk of a post-1.0 break
- What adopters get at 1.0

## Considered options

### A · Sync outside 1.0

1.0 = core, typed layer, testkit, four adapters. Sync ships on its own 0.x line; its core-facing hooks land in phase 17.

- Holds if first adopters want a stable single-store library.
- If wrong: sync later needs a store-side seam, which is a major bump after 1.0.

### B · Sync inside 1.0 (chosen)

1.0 waits for phases 13 and 14; phase 21 depends on both.

- Holds if replication is the headline for first adopters.
- If wrong: 1.0 slips about 17 working days on the least-settled piece.

## Evidence

- `runbook/roadmap.md:77`: Solo, the 1.0 path without sync is about 17–26 working days… With sync inside 1.0 it is about 34–43
- `.bklg/from-contract-to-published-library/initiative.md:173`: Publishing happenstance-sync or a happenstance-sync-testkit to the registry [is out of scope for] this release train.
- `crates/happenstance-sync/src/ingest.rs:215`: four todo!() bodies, blocked on core having no write path that keeps a foreign EventId

## Decision outcome

Chosen option: **Sync inside 1.0**, overriding the recommendation (A).

Decider's note: Sync inside 1.0

### Consequences

- Good, because it holds if replication is the headline for first adopters.
- Bad, because if wrong: 1.0 slips about 17 working days on the least-settled piece.

### Confirmation

Revisit when: replication being the headline for the first adopters

## Why this might be wrong

If building sync reveals a seam EventStore must grow that phase 17's spike missed, that becomes a 2.0.

## Provenance

- Decided 2026-09-28T19:42:30Z by human:ryan (user-directed), via chat.
- Raised in session `17346bba-2dfe-4f6c-b0aa-cedd9d318c9f`.
- Verbatim: "kind: question"
