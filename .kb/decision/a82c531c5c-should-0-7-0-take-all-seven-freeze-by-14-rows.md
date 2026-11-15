---
id: "kb-decision-a82c531c5c"
title: "All seven"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should 0.7.0 take all seven freeze-by-14 rows (building #441/#450 for SY-7/SY-10, trying VT-36's deployed split) or renew SY-7, SY-10 and VT-36 past 1.0 and keep 0.7.0 to retention only?\", facing phase 14 is the last phase a freeze-by may name; seven rows point at it, we decided for All seven and neglected Retention only, on the premise that #450 is additive, accepting that if wrong: 0.7.0 slips a few days."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-d3b3c1de39"
question: "Should 0.7.0 take all seven freeze-by-14 rows (building #441/#450 for SY-7/SY-10, trying VT-36's deployed split) or renew SY-7, SY-10 and VT-36 past 1.0 and keep 0.7.0 to retention only?"
door: one-way
blast_radius: system
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:34:44Z"
tree_hash: "e3e026bfb7960b8cb224028372d3bb417c203567"
recommended: "A"
flip_condition: "#450 needs a breaking runner change"
---

# All seven

## Context and problem statement

Should 0.7.0 take all seven freeze-by-14 rows (building #441/#450 for SY-7/SY-10, trying VT-36's deployed split) or renew SY-7, SY-10 and VT-36 past 1.0 and keep 0.7.0 to retention only?

Phase 14 is the last phase a freeze-by may name; seven rows point at it.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-d3b3c1de39`. Anchor: `runbook/ledgers.md:325`.

## Considered options

### A · All seven (chosen)

Build #441 #450; try VT-36 deployed split, renew it if long

- Holds if #450 is additive.
- If wrong: 0.7.0 slips a few days.

### B · Retention only

Renew SY-7, SY-10, VT-36 past 1.0 by record

- Holds if speed to 0.7.0 matters most.
- If wrong: three provisional sync clauses ride into 1.0.

## Evidence

- `runbook/ledgers.md:325`: SY-7 | freeze-by-14 | ... binds a runner adjudication hook that does not exist yet

## Decision outcome

Chosen option: **All seven**, the recommended option.

### Consequences

- Good, because it holds if #450 is additive.
- Bad, because if wrong: 0.7.0 slips a few days.

### Confirmation

Revisit when: #450 needs a breaking runner change

## Why this might be wrong

VT-36's deployed leg can be flaky

## Provenance

- Decided 2026-11-15T05:34:44Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "Should 0.7.0 take all seven freeze-by-14 rows (building #441/#450 for SY-7/SY-10, trying VT-36's deployed split) or renew SY-7, SY-10 and VT-36 past 1.0 and keep 0.7.0 to retention only?"
