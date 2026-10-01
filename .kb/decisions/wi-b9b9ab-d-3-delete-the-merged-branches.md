---
id: "kb-decision-wi-b9b9ab"
title: "Delete all seven"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"D-3 — Delete the merged branches.\", facing seven branches read as unmerged work in every stock-take, and six of them are not, we decided for Delete all seven and neglected Delete six, keep wip; Keep all, on the premise that nothing in d8fd819 is wanted, accepting that if wrong: the partial harness is gone once the reflog expires."
depends_on: []
related: []
source_paths:
  - runbook/roadmap.md
  - .kb/_intake/decisions/wi-b9b9ab-d-3-delete-the-merged-branches.md
last_reviewed: "2026-09-28"
reversibility: low
phase: 15
supersedes: null
superseded_by: null
weighin_item: "wi-b9b9ab"
question: "D-3 — Delete the merged branches."
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-28T19:42:30Z"
tree_hash: "cd4f937c02cc1d262dad3a512411a08b46ebe484"
recommended: "A"
flip_condition: "anything in d8fd819 not already on main"
---

# Delete all seven

## Context and problem statement

D-3 — Delete the merged branches.

Seven branches read as unmerged work in every stock-take, and six of them are not.

Raised by an agent (scan) as a question and captured by Weigh-In as `wi-b9b9ab`. Anchor: `runbook/roadmap.md:103`.

## Decision drivers

- Reversible
- Noise left

## Considered options

### A · Delete all seven (chosen)

Delete the six lane/* branches locally and on origin, and the local wip branch.

- Holds if nothing in d8fd819 is wanted.
- If wrong: the partial harness is gone once the reflog expires.

### B · Delete six, keep wip

Delete the six squash-merged lane/* branches; keep wip/hs-p0012-benchmark-harness.

- Holds if the wip commit might still be consulted.
- If wrong: one stale branch keeps showing as unmerged.

### C · Keep all

Leave every branch in place.

- Holds if branch names serve as a record.
- If wrong: every stock-take re-derives that they are merged.

## Evidence

- `gh pr list --state merged`: PRs #7, #9, #10, #11, #12, #13 merged from lane/projection-probe-seam, crate-descriptions, 0.3.1, 0.3.2, contended-store-instrument, busy-timeout-is-fifteen-seconds
- `git log main..wip/hs-p0012-benchmark-harness`: d8fd819 wip(hs-p0012): partial benchmark-harness from interrupted run

## Decision outcome

Chosen option: **Delete all seven**, the recommended option.

Decider's note: Delete all seven

### Consequences

- Good, because it holds if nothing in d8fd819 is wanted.
- Bad, because if wrong: the partial harness is gone once the reflog expires.

### Confirmation

Revisit when: anything in d8fd819 not already on main

## Why this might be wrong

If the interrupted run's notes held something the main-line harness dropped, they are lost.

## Provenance

- Decided 2026-09-28T19:42:30Z by human:ryan (user-approved), via chat.
- Raised in session `17346bba-2dfe-4f6c-b0aa-cedd9d318c9f`.
- Verbatim: "- **D-3 — Delete the merged branches.** Six `lane/*` branches were squash-merged"
