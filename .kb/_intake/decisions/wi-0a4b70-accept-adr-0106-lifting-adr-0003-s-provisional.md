---
id: "kb-decision-wi-0a4b70"
title: "Accept ADR-0106"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Accept ADR-0106, lifting ADR-0003's provisional marker on the three byte-identical SQLite round-trip tests (workerd, live-neon, live-postgres)?\", facing ADR-0003's provisional marker named byte-identical round trips as its lift; three now exist, we decided for Accept ADR-0106 and neglected Keep ADR-0003 provisional, on the premise that the three round trips cover the payload paths, accepting that if wrong: superseding ADR."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-09"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-0a4b70"
question: "Accept ADR-0106, lifting ADR-0003's provisional marker on the three byte-identical SQLite round-trip tests (workerd, live-neon, live-postgres)?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-09T03:18:09Z"
tree_hash: "3dcfdba77d04cd2cf64dabbe4c3957bb65ec9b4e"
recommended: "A"
flip_condition: "an uncovered payload path"
---

# Accept ADR-0106

## Context and problem statement

Accept ADR-0106, lifting ADR-0003's provisional marker on the three byte-identical SQLite round-trip tests (workerd, live-neon, live-postgres)?

ADR-0003's provisional marker named byte-identical round trips as its lift; three now exist.

Raised by an agent (sweep) as a question and captured by Weigh-In as `wi-0a4b70`. Anchor: `.kb/decisions/0106-adr-0003-is-no-longer-provisional.md:1`.

## Decision drivers

- coverage

## Considered options

### A · Accept ADR-0106 (chosen)

Flip to accepted, merge #443

- Holds if the three round trips cover the payload paths.
- If wrong: superseding ADR.

### B · Keep ADR-0003 provisional

Close #443 unmerged for now

- Holds if you want another path covered.
- If wrong: one more provisional at 0.6.0.

## Evidence

- `PR #443`: three named tests, workerd/live-neon/live-postgres all green

## Decision outcome

Chosen option: **Accept ADR-0106**, the recommended option.

Decider's note: Ryan tapped A on the card, 2026-11-09 03:17Z

### Consequences

- Good, because it holds if the three round trips cover the payload paths.
- Bad, because if wrong: superseding ADR.

### Confirmation

Revisit when: an uncovered payload path

## Why this might be wrong

sqlite-to-sqlite direct sync is not a named test

## Provenance

- Decided 2026-11-09T03:18:09Z by human:claude (user-approved), via chat.
- Raised in session `6ac6220e-cd11-54db-8225-134bd9de2532`.
- Verbatim: "Accept ADR-0106, lifting ADR-0003's provisional marker on the three byte-identical SQLite round-trip tests (workerd, live-neon, live-postgres)?"
