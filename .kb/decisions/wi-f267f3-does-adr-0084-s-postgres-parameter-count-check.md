---
id: "kb-decision-wi-f267f3"
title: "Both land before 0.4.0"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Does ADR-0084's Postgres parameter-count check (#89) land before 0.4.0, and does happenstance-neon stop exporting ProbeThenWriteStore (#78) in the same release?\", facing two breaking changes are pending and 0.4.0 is the last release that can carry a break before 1.0.0, we decided for Both land before 0.4.0 and neglected #84 deferred, #78 lands; Both deferred, on the premise that you want 1.0.0's surface clean and can spare two more PRs, accepting that if wrong: release slips by two PRs for changes nobody needed."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-f267f3-does-adr-0084-s-postgres-parameter-count-check.md
last_reviewed: "2026-10-10"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-f267f3"
question: "Does ADR-0084's Postgres parameter-count check (#89) land before 0.4.0, and does happenstance-neon stop exporting ProbeThenWriteStore (#78) in the same release?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-10T18:42:53Z"
tree_hash: "369163dec409bfc6fb5ef6afa83333663184bce7"
recommended: "A"
flip_condition: "#89 turning out to need a larger design than ADR-0084 already accepted"
---

# Both land before 0.4.0

## Context and problem statement

Does ADR-0084's Postgres parameter-count check (#89) land before 0.4.0, and does happenstance-neon stop exporting ProbeThenWriteStore (#78) in the same release?

Two breaking changes are pending and 0.4.0 is the last release that can carry a break before 1.0.0.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-f267f3`. Anchor: `runbook/handover.md:72`.

## Decision drivers

- release delay
- breaks left for 2.0

## Considered options

### A · Both land before 0.4.0 (chosen)

Build #89 (Postgres surplus-parameter refusal) and #78 (un-export ProbeThenWriteStore), then release

- Holds if you want 1.0.0's surface clean and can spare two more PRs.
- If wrong: release slips by two PRs for changes nobody needed.

### B · #84 deferred, #78 lands

Record ADR-0084's check as after-release; un-export ProbeThenWriteStore in 0.4.0

- Holds if the parameter check can arrive additively later.
- If wrong: the check becomes a 2.0 break if it must reject what 1.x accepts.

### C · Both deferred

Release 0.4.0 now; record both as left for after

- Holds if neither is worth delaying the release.
- If wrong: ProbeThenWriteStore is frozen into 1.x's public surface.

## Evidence

- `runbook/handover.md:70`: #84 — the decision above.
- `runbook/handover.md:71`: #78 — whether happenstance-neon stops exporting ProbeThenWriteStore. It is breaking

## Decision outcome

Chosen option: **Both land before 0.4.0**, the recommended option.

Decider's note: Both #89 and #78 land before 0.4.0.

### Consequences

- Good, because it holds if you want 1.0.0's surface clean and can spare two more PRs.
- Bad, because if wrong: release slips by two PRs for changes nobody needed.

### Confirmation

Revisit when: #89 turning out to need a larger design than ADR-0084 already accepted

## Why this might be wrong

if the parameter check is cheap to add as non-breaking later, A delays for nothing

## Provenance

- Decided 2026-10-10T18:42:53Z by human:ryan (user-approved), via chat.
- Raised in session `6c356295-9b73-5e6b-9e5a-c2ca6d15d8f1`.
- Verbatim: "kind: question"
