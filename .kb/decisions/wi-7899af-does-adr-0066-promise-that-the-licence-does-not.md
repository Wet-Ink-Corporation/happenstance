---
id: "kb-decision-wi-7899af"
title: "Promise for the nine crates"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Does ADR-0066 promise that the licence does not change during 1.x?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Promise for the nine crates and neglected No statement, on the premise that adopters value certainty over a relicensing option, accepting that if wrong: the core cannot be relicensed within 1.x."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-7899af-does-adr-0066-promise-that-the-licence-does-not.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-7899af"
question: "Does ADR-0066 promise that the licence does not change during 1.x?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Promise for the nine crates

## Context and problem statement

Does ADR-0066 promise that the licence does not change during 1.x?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-7899af`. Anchor: `references/seeds/licensing-and-the-commercial-seam.md:29`.

## Considered options

### A · No statement

Leave the licence out of the 1.0 promise

- Holds if a relicensing path for the core is worth keeping.
- If wrong: adopters have less certainty.

### B · Promise for the nine crates (chosen)

The nine 1.0 crates stay MIT OR Apache-2.0 for all of 1.x; new crates may differ

- Holds if adopters value certainty over a relicensing option.
- If wrong: the core cannot be relicensed within 1.x.

## Evidence

- `references/seeds/licensing-and-the-commercial-seam.md:29`: the item this decision settles

## Decision outcome

Chosen option: **Promise for the nine crates**, overriding the recommendation (A).

Decider's note: the owner chose B (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if adopters value certainty over a relicensing option.
- Bad, because if wrong: the core cannot be relicensed within 1.x.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-directed), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "Does ADR-0066 promise that the licence does not change during 1.x?"
