---
id: "kb-decision-wi-73ab4f"
title: "Accept ADR-0105"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Accept ADR-0105 (write groups; stall, Split, SY-39) so SY-39 can be frozen or re-dispositioned before phase 13 closes?\", facing ADR-0105 defines write groups, now merged on every adapter; SY-39 needs it accepted, we decided for Accept ADR-0105 and neglected Keep proposed, on the premise that stall/Split/SY-39 semantics are what you want published, accepting that if wrong: superseding ADR."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-09"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-73ab4f"
question: "Accept ADR-0105 (write groups; stall, Split, SY-39) so SY-39 can be frozen or re-dispositioned before phase 13 closes?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-09T03:17:54Z"
tree_hash: "3dcfdba77d04cd2cf64dabbe4c3957bb65ec9b4e"
recommended: "A"
flip_condition: "a write-group semantic you want changed"
---

# Accept ADR-0105

## Context and problem statement

Accept ADR-0105 (write groups; stall, Split, SY-39) so SY-39 can be frozen or re-dispositioned before phase 13 closes?

ADR-0105 defines write groups, now merged on every adapter; SY-39 needs it accepted.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-73ab4f`. Anchor: `.kb/decisions/0105-write-groups.md:1`.

## Decision drivers

- effect

## Considered options

### A · Accept ADR-0105 (chosen)

Then SY-39 freeze goes in the freeze PR

- Holds if stall/Split/SY-39 semantics are what you want published.
- If wrong: superseding ADR.

### B · Keep proposed

SY-39 re-dispositioned past 0.6.0

- Holds if you want changes first.
- If wrong: phase 13 stays open.

## Evidence

- `PRs #434 #436 #437`: write groups green on memory, SQLite, Postgres, Neon, DO

## Decision outcome

Chosen option: **Accept ADR-0105**, the recommended option.

Decider's note: Ryan tapped A on the card, 2026-11-09 03:17Z

### Consequences

- Good, because it holds if stall/Split/SY-39 semantics are what you want published.
- Bad, because if wrong: superseding ADR.

### Confirmation

Revisit when: a write-group semantic you want changed

## Why this might be wrong

write groups are new and only one release old in practice

## Provenance

- Decided 2026-11-09T03:17:54Z by human:claude (user-approved), via chat.
- Raised in session `6ac6220e-cd11-54db-8225-134bd9de2532`.
- Verbatim: "Accept ADR-0105 (write groups; stall, Split, SY-39) so SY-39 can be frozen or re-dispositioned before phase 13 closes?"
