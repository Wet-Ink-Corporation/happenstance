---
id: "kb-decision-wi-cbc941"
title: "Latest minor plus previous major for six months"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"What does the project promise for security fixes after 1.0?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Latest minor plus previous major for six months and neglected Latest minor only, on the premise that adapter majors follow driver majors, accepting that if wrong: double maintenance for six months."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-cbc941-what-does-the-project-promise-for-security.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-cbc941"
question: "What does the project promise for security fixes after 1.0?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Latest minor plus previous major for six months

## Context and problem statement

What does the project promise for security fixes after 1.0?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-cbc941`. Anchor: `SECURITY.md:77`.

## Considered options

### A · Latest minor plus previous major for six months (chosen)

Fixes on the latest 1.x minor; security fixes on the previous major for six months after the next ships

- Holds if adapter majors follow driver majors.
- If wrong: double maintenance for six months.

### B · Latest minor only

No backports

- Holds if adopters move with every major.
- If wrong: adopters must move immediately.

## Evidence

- `SECURITY.md:77`: the item this decision settles

## Decision outcome

Chosen option: **Latest minor plus previous major for six months**, the recommended option.

Decider's note: the owner chose A (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if adapter majors follow driver majors.
- Bad, because if wrong: double maintenance for six months.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-approved), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "What does the project promise for security fixes after 1.0?"
