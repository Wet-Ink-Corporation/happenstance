---
id: "kb-decision-wi-016abe"
title: "Close HS-I0006 and re-plan"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Reconcile HS-I0006 by closing it as-is and replanning phases 13–14 as a new initiative, or by advancing all 190 stories through the CLI?\", facing the backlog reports 0 of 190 stories done, so redkiln status and next route to shipped work, we decided for Close HS-I0006 and re-plan and neglected Walk all 190 forward; Close reviewed projects only, on the premise that git history and the runbook are an adequate record of what shipped, accepting that if wrong: per-story gate records for the shipped work never exist."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-016abe-reconcile-hs-i0006-by-closing-it-as-is-and.md
last_reviewed: "2026-09-28"
reversibility: high
phase: 15
supersedes: null
superseded_by: null
weighin_item: "wi-016abe"
question: "Reconcile HS-I0006 by closing it as-is and replanning phases 13–14 as a new initiative, or by advancing all 190 stories through the CLI?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-28T19:42:30Z"
tree_hash: "cd4f937c02cc1d262dad3a512411a08b46ebe484"
recommended: "A"
flip_condition: "a per-story audit trail mattering more than a day of effort"
---

# Close HS-I0006 and re-plan

## Context and problem statement

Reconcile HS-I0006 by closing it as-is and replanning phases 13–14 as a new initiative, or by advancing all 190 stories through the CLI?

The backlog reports 0 of 190 stories done, so redkiln status and next route to shipped work.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-016abe`. Anchor: `.bklg/from-contract-to-published-library/initiative.md:170`.

## Decision drivers

- Effort
- Audit trail kept

## Considered options

### A · Close HS-I0006 and re-plan (chosen)

Close the initiative as it stands through the CLI; plan phases 13 onward as a new road-to-1.0 initiative.

- Holds if git history and the runbook are an adequate record of what shipped.
- If wrong: per-story gate records for the shipped work never exist.

### B · Walk all 190 forward

Advance every story and project through its gates, with verdicts, then continue HS-I0006.

- Holds if a per-story audit trail is worth the time.
- If wrong: days of gate-walking over finished work; the charter still names 1.0 a non-goal.

### C · Close reviewed projects only

Advance the six projects with approved reviews to closeout; close the rest of HS-I0006 and re-plan.

- Holds if the approved reviews are the records worth keeping.
- If wrong: two paths through the CLI for one clean-up.

## Evidence

- `.bklg/from-contract-to-published-library/initiative.md:170`: A 1.0 release, or any post-1.0 semver commitment [is out of scope]
- `runbook/roadmap.md:97`: 84 sit at report over work that shipped in v0.2.0, and roughly thirty at plan are finished on main

## Decision outcome

Chosen option: **Close HS-I0006 and re-plan**, the recommended option.

Decider's note: Close HS-I0006 and re-plan

### Consequences

- Good, because it holds if git history and the runbook are an adequate record of what shipped.
- Bad, because if wrong: per-story gate records for the shipped work never exist.

### Confirmation

Revisit when: a per-story audit trail mattering more than a day of effort

## Why this might be wrong

If redkiln telemetry or a later retrospective needs story-level closure, it will not exist.

## Provenance

- Decided 2026-09-28T19:42:30Z by human:ryan (user-approved), via chat.
- Raised in session `17346bba-2dfe-4f6c-b0aa-cedd9d318c9f`.
- Verbatim: "kind: question"
