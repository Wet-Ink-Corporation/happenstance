---
id: "kb-decision-wi-1408e8"
title: "Conditions only, no floor"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"How long a soak between 1.0.0-rc.1 and 1.0.0?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Conditions only, no floor and neglected 14 days plus conditions, on the premise that nobody external is reading the rc, accepting that if wrong: no window for a late outside reader."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-1408e8-how-long-a-soak-between-1-0-0-rc-1-and-1-0-0.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-1408e8"
question: "How long a soak between 1.0.0-rc.1 and 1.0.0?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Conditions only, no floor

## Context and problem statement

How long a soak between 1.0.0-rc.1 and 1.0.0?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-1408e8`. Anchor: `runbook/phases/21-one-point-oh.md:48`.

## Considered options

### A · 14 days plus conditions

A time floor plus observable conditions

- Holds if outside adopters try the rc.
- If wrong: about two weeks with nobody external testing.

### B · Conditions only, no floor (chosen)

1.0.0 follows the rc as soon as the listed conditions hold

- Holds if nobody external is reading the rc.
- If wrong: no window for a late outside reader.

## Evidence

- `runbook/phases/21-one-point-oh.md:48`: the item this decision settles

## Decision outcome

Chosen option: **Conditions only, no floor**, overriding the recommendation (A).

Decider's note: the owner chose B over the recommended A (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if nobody external is reading the rc.
- Bad, because if wrong: no window for a late outside reader.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-directed), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "How long a soak between 1.0.0-rc.1 and 1.0.0?"
