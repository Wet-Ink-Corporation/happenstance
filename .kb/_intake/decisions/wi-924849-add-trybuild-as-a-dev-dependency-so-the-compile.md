---
id: "kb-decision-wi-924849"
title: "Keep hand-verified"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Add `trybuild` as a dev-dependency so the `compile_fail` doctests on SqliteBatch::push and NeonWriteBatch::push pin their error codes (E0308/E0616), which stable rustdoc does not check?\", facing stable rustdoc does not check compile_fail error codes; trybuild would pin them, we decided for Keep hand-verified and neglected Add trybuild, on the premise that the doctests rarely change, accepting that if wrong: a doctest fails for the wrong reason and hides a regression."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-924849"
question: "Add `trybuild` as a dev-dependency so the `compile_fail` doctests on SqliteBatch::push and NeonWriteBatch::push pin their error codes (E0308/E0616), which stable rustdoc does not check?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-08T12:57:33Z"
tree_hash: "62904db838806d6ee58397ad1f39d294acf908da"
recommended: "A"
flip_condition: "a compile_fail doctest passing for an unrelated error"
---

# Keep hand-verified

## Context and problem statement

Add `trybuild` as a dev-dependency so the `compile_fail` doctests on SqliteBatch::push and NeonWriteBatch::push pin their error codes (E0308/E0616), which stable rustdoc does not check?

Stable rustdoc does not check compile_fail error codes; trybuild would pin them.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-924849`. Anchor: `crates/happenstance-neon/src/projection_store.rs:198`.

## Decision drivers

- cost

## Considered options

### A · Keep hand-verified (chosen)

No new dependency

- Holds if the doctests rarely change.
- If wrong: a doctest fails for the wrong reason and hides a regression.

### B · Add trybuild

dev-dependency pinning stderr

- Holds if you accept stderr snapshots churning with rustc.
- If wrong: snapshot churn on toolchain bumps.

## Evidence

- `crates/happenstance-neon/src/projection_store.rs:198`: NeonWriteBatch::push compile_fail doctest

## Decision outcome

Chosen option: **Keep hand-verified**, the recommended option.

Decider's note: keep compile_fail doctests hand-verified; no trybuild

### Consequences

- Good, because it holds if the doctests rarely change.
- Bad, because if wrong: a doctest fails for the wrong reason and hides a regression.

### Confirmation

Revisit when: a compile_fail doctest passing for an unrelated error

## Why this might be wrong

two doctests already rely on it

## Provenance

- Decided 2026-10-08T12:57:33Z by human:ryan (user-approved), via chat.
- Raised in session `6ec241b3-3771-51a8-b5a3-78697abb6042`.
- Verbatim: "kind: question"
