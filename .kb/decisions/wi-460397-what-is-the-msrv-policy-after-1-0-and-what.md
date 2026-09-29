---
id: "kb-decision-wi-460397"
title: "Hold 1.97.1; bounded rises"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"What is the MSRV policy after 1.0, and what happens to the unexecuted 2026-09-06 ratification?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Hold 1.97.1; bounded rises and neglected Lower to the measured minimum, on the premise that consumers use resolver 3's MSRV-aware fallback, accepting that if wrong: a consumer on resolver 2 is protected only by the bound."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-460397-what-is-the-msrv-policy-after-1-0-and-what.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-460397"
question: "What is the MSRV policy after 1.0, and what happens to the unexecuted 2026-09-06 ratification?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Hold 1.97.1; bounded rises

## Context and problem statement

What is the MSRV policy after 1.0, and what happens to the unexecuted 2026-09-06 ratification?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-460397`. Anchor: `.kb/open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md:1`.

## Considered options

### A · Hold 1.97.1; bounded rises (chosen)

Withdraw the ratification; after 1.0 a rise ships only in a minor, to a stable at least six months old, with a CHANGELOG entry

- Holds if consumers use resolver 3's MSRV-aware fallback.
- If wrong: a consumer on resolver 2 is protected only by the bound.

### B · Lower to the measured minimum

Re-measure (1.95) and lower before 1.0

- Holds if consumers need an older compiler.
- If wrong: another CI pin to maintain.

## Evidence

- `.kb/open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md:1`: the item this decision settles

## Decision outcome

Chosen option: **Hold 1.97.1; bounded rises**, the recommended option.

Decider's note: the owner chose A (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if consumers use resolver 3's MSRV-aware fallback.
- Bad, because if wrong: a consumer on resolver 2 is protected only by the bound.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-approved), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "What is the MSRV policy after 1.0, and what happens to the unexecuted 2026-09-06 ratification?"
