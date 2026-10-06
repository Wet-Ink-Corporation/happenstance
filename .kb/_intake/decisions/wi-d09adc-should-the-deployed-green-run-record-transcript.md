---
id: "kb-decision-wi-d09adc"
title: "Add to #35 before merge"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should the deployed green-run record (transcript under experiments/durable-object-limits/results/, run URL in handover and phase-17 log, exit-criterion ticks) land in PR #35 before merge, or in a later commit?\", facing the L6b plan says to record the green run, and the deployed leg just produced it, we decided for Add to #35 before merge and neglected Separate PR after merge, on the premise that #35 should close L6b completely, accepting that if wrong: One more CI run (~20 min)."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-06"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-d09adc"
question: "Should the deployed green-run record (transcript under experiments/durable-object-limits/results/, run URL in handover and phase-17 log, exit-criterion ticks) land in PR #35 before merge, or in a later commit?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-06T13:23:37Z"
tree_hash: "cab814da9263e77984587bd7e7c699b6acf84eed"
recommended: "A"
flip_condition: "If you'd rather merge the exact reviewed commit"
---

# Add to #35 before merge

## Context and problem statement

Should the deployed green-run record (transcript under experiments/durable-object-limits/results/, run URL in handover and phase-17 log, exit-criterion ticks) land in PR #35 before merge, or in a later commit?

The L6b plan says to record the green run, and the deployed leg just produced it.

Raised by an agent (sweep) as a deferral and captured by Weigh-In as `wi-d09adc`. Anchor: `runbook/phases/17-breaking-window.md:275`.

## Decision drivers

- CI cost

## Considered options

### A · Add to #35 before merge (chosen)

Docs-only commit: deployed transcript, run URL, exit ticks.

- Holds if #35 should close L6b completely.
- If wrong: One more CI run (~20 min).

### B · Separate PR after merge

Merge #35 now; record in a follow-up.

- Holds if You want #35 merged as reviewed.
- If wrong: Handover briefly says the run is owed.

## Evidence

- `runbook/handover.md:68`: record the green run;
- `PR #35 run 37419423991`: executed 96 of 96; 0 failed; max-length tags in one query item: accepted 32514

## Decision outcome

Chosen option: **Add to #35 before merge**, the recommended option.

Decider's note: Add the green-run record to #35 before merge.

### Consequences

- Good, because it holds if #35 should close L6b completely.
- Bad, because if wrong: One more CI run (~20 min).

### Confirmation

Revisit when: If you'd rather merge the exact reviewed commit

## Why this might be wrong

The reviewed tree changes, so the review gate re-runs too

## Outcome

Overridden by events, not by a new choice. #35 was merged (`6a3adf6a`, 2026-10-06 13:15 UTC) before the record could be added to it, so the record landed in **#36** instead: `experiments/durable-object-limits/results/run-workerd-deployed-2026-10-06.txt`, the README's deployed cells, the phase-17 exit tick and session-log entry, and the handover. The intent — record the green run as part of closing L6b — held; only the PR changed.

## Provenance

- Decided 2026-10-06T13:23:37Z by human:ryan (user-approved), via chat.
- Raised in session `f7fec0fb-352c-4bd4-92ac-d997606ea87e`.
- Verbatim: "Should the deployed green-run record (transcript under experiments/durable-object-limits/results/, run URL in handover and phase-17 log, exit-criterion ticks) land in PR #35 before merge, or in a later commit?"
