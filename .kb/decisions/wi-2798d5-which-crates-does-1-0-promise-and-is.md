---
id: "kb-decision-wi-2798d5"
title: "Nine; ladybug outside"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Which crates does 1.0 promise, and is happenstance-ladybug among them?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Nine; ladybug outside and neglected Ten, ladybug included, on the premise that lbug stays unbuildable on docs.rs, accepting that if wrong: ladybug users wait for a later 0.x."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-2798d5-which-crates-does-1-0-promise-and-is.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-2798d5"
question: "Which crates does 1.0 promise, and is happenstance-ladybug among them?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Nine; ladybug outside

## Context and problem statement

Which crates does 1.0 promise, and is happenstance-ladybug among them?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-2798d5`. Anchor: `runbook/phases/16-define-1-0.md:20`.

## Considered options

### A · Nine; ladybug outside (chosen)

The seven published crates plus happenstance-sync and happenstance-sync-testkit; ladybug keeps its own 0.x line

- Holds if lbug stays unbuildable on docs.rs.
- If wrong: ladybug users wait for a later 0.x.

### B · Ten, ladybug included

Also promise happenstance-ladybug

- Holds if upstream lbug fixes docs.rs.
- If wrong: phase 21 waits on a third party.

## Evidence

- `runbook/phases/16-define-1-0.md:20`: the item this decision settles

## Decision outcome

Chosen option: **Nine; ladybug outside**, the recommended option.

Decider's note: the owner answered A with the trade-off in view (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if lbug stays unbuildable on docs.rs.
- Bad, because if wrong: ladybug users wait for a later 0.x.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-approved), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "Which crates does 1.0 promise, and is happenstance-ladybug among them?"
