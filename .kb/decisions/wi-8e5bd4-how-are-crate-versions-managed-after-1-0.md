---
id: "kb-decision-wi-8e5bd4"
title: "Lockstep 1.0.0, then independent"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"How are crate versions managed after 1.0?\", facing it is part of what 1.0 promises, and ADR-0066 records it, we decided for Lockstep 1.0.0, then independent and neglected Shared workspace version, on the premise that driver majors keep forcing adapter majors (ADR-0044), accepting that if wrong: more release bookkeeping."
depends_on: []
related:
  - kb-decision-0066
source_paths:
  - .kb/_intake/decisions/wi-8e5bd4-how-are-crate-versions-managed-after-1-0.md
last_reviewed: "2026-09-29"
reversibility: low
phase: 16
supersedes: null
superseded_by: null
weighin_item: "wi-8e5bd4"
question: "How are crate versions managed after 1.0?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-29T15:31:09Z"
tree_hash: "47e887d507d1f5b2fa472f496b10c03e8c87fd3c"
recommended: "A"
---

# Lockstep 1.0.0, then independent

## Context and problem statement

How are crate versions managed after 1.0?

It is part of what 1.0 promises, and ADR-0066 records it.

Raised by an agent (manual) as a question and captured by Weigh-In as `wi-8e5bd4`. Anchor: `.kb/open-questions/adapter-version-lockstep-and-cf-32.md:1`.

## Considered options

### A · Lockstep 1.0.0, then independent (chosen)

All ship 1.0.0 together, then version independently; adapters declare minimum core; a minimal-versions job keeps bounds honest

- Holds if driver majors keep forcing adapter majors (ADR-0044).
- If wrong: more release bookkeeping.

### B · Shared workspace version

One version for everything, as today

- Holds if drivers are stable.
- If wrong: a driver major drags core to 2.0 with an unchanged contract.

## Evidence

- `.kb/open-questions/adapter-version-lockstep-and-cf-32.md:1`: the item this decision settles

## Decision outcome

Chosen option: **Lockstep 1.0.0, then independent**, the recommended option.

Decider's note: the owner chose A after how dependency minimums are enforced was explained (phase-16 planning session, 2026-09-29)

### Consequences

- Good, because it holds if driver majors keep forcing adapter majors (ADR-0044).
- Bad, because if wrong: more release bookkeeping.

### Confirmation

Revisit if the premises above stop holding.

## Why this might be wrong

see ADR-0066

## Provenance

- Decided 2026-09-29T15:31:09Z by human:ryan (user-approved), via chat.
- Raised in session `1a19c172-8a8c-448d-ad9a-ce7ebf44140f`.
- Verbatim: "How are crate versions managed after 1.0?"
