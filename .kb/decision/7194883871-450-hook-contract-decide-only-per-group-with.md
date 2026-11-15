---
id: "kb-decision-7194883871"
title: "Decide-only, per group + earlier"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"#450: hook contract decide-only, per group with the earlier groups of the same batch, returning owned compensation events (not batch-level, not a hook that writes)?\", facing the hook's contract is a published trait, we decided for Decide-only, per group + earlier and neglected Batch-level; Hook writes, on the premise that per-group judgment suffices, accepting that if wrong: batch-wide judgments need a new method."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-15"
reversibility: low
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-94cd0e581a"
question: "#450: hook contract decide-only, per group with the earlier groups of the same batch, returning owned compensation events (not batch-level, not a hook that writes)?"
door: one-way
blast_radius: external
decider: "human:claude"
provenance: user-approved
decided_at: "2026-11-15T05:49:09Z"
tree_hash: "bd0c59e4174da9e30c0f385aa4f0e01a4154b6bb"
recommended: "A"
flip_condition: "an adjudicator needs the whole batch"
---

# Decide-only, per group + earlier

## Context and problem statement

#450: hook contract decide-only, per group with the earlier groups of the same batch, returning owned compensation events (not batch-level, not a hook that writes)?

The hook's contract is a published trait.

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-94cd0e581a`. Anchor: `crates/happenstance-sync/src/runner.rs:180`.

## Considered options

### A · Decide-only, per group + earlier (chosen)

compensation(group, earlier) -> Vec<Event>; runner writes it atomically

- Holds if per-group judgment suffices.
- If wrong: batch-wide judgments need a new method.

### B · Batch-level

one call per batch, results paired by index

- Holds if adjudicators need the whole batch.
- If wrong: index pairing errors.

### C · Hook writes

hook appends itself

- Holds if never.
- If wrong: compensation committed apart from its group (SY-2).

## Evidence

- `spec/SPECIFICATION.md`: SY-2: a compensation is committed with its group

## Decision outcome

Chosen option: **Decide-only, per group + earlier**, the recommended option.

### Consequences

- Good, because it holds if per-group judgment suffices.
- Bad, because if wrong: batch-wide judgments need a new method.

### Confirmation

Revisit when: an adjudicator needs the whole batch

## Why this might be wrong

earlier slice may grow costly on big batches

## Provenance

- Decided 2026-11-15T05:49:09Z by human:claude (user-approved), via cli.
- Raised in session `456d08b0-9a08-5fef-92d6-6305f2ca971f`.
- Verbatim: "#450: hook contract decide-only, per group with the earlier groups of the same batch, returning owned compensation events (not batch-level, not a hook that writes)?"
