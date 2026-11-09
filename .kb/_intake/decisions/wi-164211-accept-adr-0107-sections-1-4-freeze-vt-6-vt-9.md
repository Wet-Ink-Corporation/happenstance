---
id: "kb-decision-wi-164211"
title: "Accept as proposed"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Accept ADR-0107 sections 1-4: freeze VT-6, VT-9, VT-21, VT-24, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-30, SY-31, CF-40, and renew SY-7, SY-10, VT-36 to freeze-by-14 (lints refuse freeze-by-21)?\", facing phase 13 exit needs its clauses frozen or re-dispositioned, and freezing is a one-way contract change, we decided for Accept as proposed and neglected Accept, but hold named clauses, on the premise that conditions A-D (SY-20 width, VT-6 two adapters, SY-18 fixture, SY-31 no DO store) are acceptable at 1.0, accepting that if wrong: a frozen clause needs a superseding ADR."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-09"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-164211"
question: "Accept ADR-0107 sections 1-4: freeze VT-6, VT-9, VT-21, VT-24, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-30, SY-31, CF-40, and renew SY-7, SY-10, VT-36 to freeze-by-14 (lints refuse freeze-by-21)?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-09T03:17:31Z"
tree_hash: "3dcfdba77d04cd2cf64dabbe4c3957bb65ec9b4e"
recommended: "A"
flip_condition: "a condition you do not accept"
---

# Accept as proposed

## Context and problem statement

Accept ADR-0107 sections 1-4: freeze VT-6, VT-9, VT-21, VT-24, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-30, SY-31, CF-40, and renew SY-7, SY-10, VT-36 to freeze-by-14 (lints refuse freeze-by-21)?

Phase 13 exit needs its clauses frozen or re-dispositioned, and freezing is a one-way contract change.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-164211`. Anchor: `.kb/decisions/0107-phase-13s-clauses-are-frozen.md:1`.

## Decision drivers

- evidence

## Considered options

### A · Accept as proposed (chosen)

Freeze the 14, renew SY-7/SY-10/VT-36 to freeze-by-14; follow-ups #451 #452 #453

- Holds if conditions A-D (SY-20 width, VT-6 two adapters, SY-18 fixture, SY-31 no DO store) are acceptable at 1.0.
- If wrong: a frozen clause needs a superseding ADR.

### B · Accept, but hold named clauses

Freeze the rest; held ones go to freeze-by-14

- Holds if one of the conditions is not acceptable.
- If wrong: more work before phase 13 can close.

## Evidence

- `PR #447`: CI all green incl. live-neon, live-postgres, workerd
- `run 42547020310`: SY-14: 1,840 events in 16 one-request pushes, 1.58 s

## Decision outcome

Chosen option: **Accept as proposed**, the recommended option.

Decider's note: Ryan tapped A on the card, 2026-11-09 03:17Z

### Consequences

- Good, because it holds if conditions A-D (SY-20 width, VT-6 two adapters, SY-18 fixture, SY-31 no DO store) are acceptable at 1.0.
- Bad, because if wrong: a frozen clause needs a superseding ADR.

### Confirmation

Revisit when: a condition you do not accept

## Why this might be wrong

VT-6 and SY-31 rest on fewer adapters than the rest

## Provenance

- Decided 2026-11-09T03:17:31Z by human:claude (user-approved), via chat.
- Raised in session `6ac6220e-cd11-54db-8225-134bd9de2532`.
- Verbatim: "Accept ADR-0107 sections 1-4: freeze VT-6, VT-9, VT-21, VT-24, SY-14, SY-18, SY-20, SY-22, SY-23, SY-27, SY-28, SY-30, SY-31, CF-40, and renew SY-7, SY-10, VT-36 to freeze-by-14 (lints refuse freeze-by-21)?"
