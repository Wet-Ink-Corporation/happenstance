---
id: "kb-decision-wi-7e9a97"
title: "Land before 0.4.0"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Should ADR-0084's adapter-side Postgres parameter-count check land before 0.4.0 is released, or ship after it?\", facing ADR-0084 is accepted; its Postgres parameter-count check is the trace table's one Pending row, we decided for Land before 0.4.0 and neglected Release first, on the premise that the check is small (one adapter, one test), accepting that if wrong: release slips by a PR cycle."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-7e9a97-should-adr-0084-s-adapter-side-postgres.md
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-7e9a97"
question: "Should ADR-0084's adapter-side Postgres parameter-count check land before 0.4.0 is released, or ship after it?"
door: two-way
blast_radius: system
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-08T12:57:33Z"
tree_hash: "62904db838806d6ee58397ad1f39d294acf908da"
recommended: "A"
flip_condition: "wanting 0.4.0 out before more code lands"
---

# Land before 0.4.0

## Context and problem statement

Should ADR-0084's adapter-side Postgres parameter-count check land before 0.4.0 is released, or ship after it?

ADR-0084 is accepted; its Postgres parameter-count check is the trace table's one Pending row.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-7e9a97`. Anchor: `CHANGELOG.md:565`.

## Decision drivers

- release

## Considered options

### A · Land before 0.4.0 (chosen)

Implement the check, then release

- Holds if the check is small (one adapter, one test).
- If wrong: release slips by a PR cycle.

### B · Release first

Ship 0.4.0, check in 0.4.x

- Holds if the check is non-breaking.
- If wrong: if it changes an error variant it becomes a 0.5.0 break.

## Evidence

- `CHANGELOG.md:565`: 0.4.0 trace table: ADR-0084 Postgres check Pending

## Decision outcome

Chosen option: **Land before 0.4.0**, the recommended option.

Decider's note: ADR-0084 Postgres parameter-count check lands before 0.4.0

### Consequences

- Good, because it holds if the check is small (one adapter, one test).
- Bad, because if wrong: release slips by a PR cycle.

### Confirmation

Revisit when: wanting 0.4.0 out before more code lands

## Why this might be wrong

if the check is purely additive, B costs nothing

## Provenance

- Decided 2026-10-08T12:57:33Z by human:ryan (user-approved), via chat.
- Raised in session `6ec241b3-3771-51a8-b5a3-78697abb6042`.
- Verbatim: "kind: question"
