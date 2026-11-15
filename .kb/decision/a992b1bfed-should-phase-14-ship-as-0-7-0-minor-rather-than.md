---
id: "kb-decision-a992b1bfed"
title: "0.7.0"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should phase 14 ship as 0.7.0 (minor) rather than 0.6.1, given it is all additive?\", facing version number is a promise to pinned consumers, we decided for 0.7.0 and neglected 0.6.1, on the premise that new rules are a minor, accepting that if wrong: a needless minor."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-7d6bd7bab0"
question: "Should phase 14 ship as 0.7.0 (minor) rather than 0.6.1, given it is all additive?"
door: two-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:34:44Z"
tree_hash: "e3e026bfb7960b8cb224028372d3bb417c203567"
recommended: "A"
flip_condition: "Ryan treats rules as patch"
---

# 0.7.0

## Context and problem statement

Should phase 14 ship as 0.7.0 (minor) rather than 0.6.1, given it is all additive?

Version number is a promise to pinned consumers.

Raised by an agent (sweep) as a assumption and captured by Weigh-In as `wi-7d6bd7bab0`. Anchor: `/mnt/project-files/notes/plan-0.7.0.md:44`.

## Considered options

### A · 0.7.0 (chosen)

Minor, lockstep nine crates

- Holds if new rules are a minor.
- If wrong: a needless minor.

### B · 0.6.1

Patch

- Holds if additive is enough.
- If wrong: breaks the sync-testkit pinning promise.

## Evidence

- `CLAUDE.md`: a new rule is a minor: a consumer pins it exactly

## Decision outcome

Chosen option: **0.7.0**, the recommended option.

### Consequences

- Good, because it holds if new rules are a minor.
- Bad, because if wrong: a needless minor.

### Confirmation

Revisit when: Ryan treats rules as patch

## Why this might be wrong

none known

## Provenance

- Decided 2026-11-15T05:34:44Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "Should phase 14 ship as 0.7.0 (minor) rather than 0.6.1, given it is all additive?"
