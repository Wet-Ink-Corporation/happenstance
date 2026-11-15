---
id: "kb-decision-f9d223bb79"
title: "phase-14 integration branch"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should phase 14 run through a phase-14 integration branch that the thread merges reviewed green PRs into (including additive one-way #147 and #450), with Ryan merging phase-14 into main and the release?\", facing where phase 14 PRs merge decides how often Ryan must act, we decided for phase-14 integration branch and neglected Per-PR into main, on the premise that the phase-13 model worked, accepting that if wrong: one big merge to review at the end."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-0b642c3b6f"
question: "Should phase 14 run through a phase-14 integration branch that the thread merges reviewed green PRs into (including additive one-way #147 and #450), with Ryan merging phase-14 into main and the release?"
door: two-way
blast_radius: system
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:26:27Z"
tree_hash: "e3e026bfb7960b8cb224028372d3bb417c203567"
recommended: "A"
flip_condition: "Ryan wants incremental main"
---

# phase-14 integration branch

## Context and problem statement

Should phase 14 run through a phase-14 integration branch that the thread merges reviewed green PRs into (including additive one-way #147 and #450), with Ryan merging phase-14 into main and the release?

Where phase 14 PRs merge decides how often Ryan must act.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-0b642c3b6f`. Anchor: `/mnt/project-files/notes/plan-0.7.0.md:51`.

## Considered options

### A · phase-14 integration branch (chosen)

Thread merges green reviewed PRs into phase-14; Ryan merges it to main once

- Holds if the phase-13 model worked.
- If wrong: one big merge to review at the end.

### B · Per-PR into main

Two-way PRs auto-merge into main; Ryan merges each one-way PR

- Holds if Ryan wants main to move incrementally.
- If wrong: minimal-versions leg red on main until release.

## Evidence

- `notes/plan-0.6.0.md:61`: One-way PRs merge into a phase-13 integration branch

## Decision outcome

Chosen option: **phase-14 integration branch**, the recommended option.

### Consequences

- Good, because it holds if the phase-13 model worked.
- Bad, because if wrong: one big merge to review at the end.

### Confirmation

Revisit when: Ryan wants incremental main

## Why this might be wrong

a long-lived branch can conflict with side work on main

## Provenance

- Decided 2026-11-15T05:26:27Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "Should phase 14 run through a phase-14 integration branch that the thread merges reviewed green PRs into (including additive one-way #147 and #450), with Ryan merging phase-14 into main and the release?"
