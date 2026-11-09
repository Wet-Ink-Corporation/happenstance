---
id: "kb-decision-wi-56271e"
title: "Freeze all four"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Freeze the four held clauses in ADR-0107 section 5: SY-36, SY-37, SY-38 and CF-42?\", facing you asked to approve SY-36, SY-37, SY-38 and CF-42 individually, we decided for Freeze all four and neglected Freeze all but SY-36, on the premise that SY-36's unit-test coverage of non-Split classes is enough, accepting that if wrong: superseding ADR."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-09"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-56271e"
question: "Freeze the four held clauses in ADR-0107 section 5: SY-36, SY-37, SY-38 and CF-42?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-09T03:17:43Z"
tree_hash: "3dcfdba77d04cd2cf64dabbe4c3957bb65ec9b4e"
recommended: "A"
flip_condition: "wanting rule coverage for all refusal classes"
---

# Freeze all four

## Context and problem statement

Freeze the four held clauses in ADR-0107 section 5: SY-36, SY-37, SY-38 and CF-42?

You asked to approve SY-36, SY-37, SY-38 and CF-42 individually.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-56271e`. Anchor: `spec/SPECIFICATION.md:1`.

## Decision drivers

- evidence

## Considered options

### A · Freeze all four (chosen)

Each green on every real adapter

- Holds if SY-36's unit-test coverage of non-Split classes is enough.
- If wrong: superseding ADR.

### B · Freeze all but SY-36

SY-36 to freeze-by-14 until a rule drives every class

- Holds if you want every refusal class rule-driven.
- If wrong: SY-36 stays provisional at 0.6.0.

## Evidence

- `PR #447 302cc37e`: one hunk per marker; revert any one to decline

## Decision outcome

Chosen option: **Freeze all four**, the recommended option.

Decider's note: Ryan tapped A on the card, 2026-11-09 03:17Z

### Consequences

- Good, because it holds if SY-36's unit-test coverage of non-Split classes is enough.
- Bad, because if wrong: superseding ADR.

### Confirmation

Revisit when: wanting rule coverage for all refusal classes

## Why this might be wrong

SY-36's rule exercises only Split

## Provenance

- Decided 2026-11-09T03:17:43Z by human:claude (user-approved), via chat.
- Raised in session `6ac6220e-cd11-54db-8225-134bd9de2532`.
- Verbatim: "Freeze the four held clauses in ADR-0107 section 5: SY-36, SY-37, SY-38 and CF-42?"
