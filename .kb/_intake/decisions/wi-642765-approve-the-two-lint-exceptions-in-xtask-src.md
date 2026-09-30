---
id: "kb-decision-wi-642765"
title: "Accept as pre-existing"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Approve the two lint exceptions in xtask/src/lints.rs (lines 3166 and 3862, from commit 230065f on main), or move the temper gate's comparison point to 230065f?\", facing the temper gate scan is red only on two exceptions that came with 230065f, not with this branch, we decided for Accept as pre-existing and neglected Revert the citation fix; Fix them here, on the premise that they are 230065f's to answer for, accepting that if wrong: two unreviewed exceptions stay approved."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-09-30"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-642765"
question: "Approve the two lint exceptions in xtask/src/lints.rs (lines 3166 and 3862, from commit 230065f on main), or move the temper gate's comparison point to 230065f?"
door: two-way
blast_radius: file
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-09-30T02:54:01Z"
tree_hash: "196f0e0f336a42fd143c6a5c88bbbe30c0f559b4"
recommended: "A"
flip_condition: "a rule that a branch must not touch a file carrying an unapproved exception"
---

# Accept as pre-existing

## Context and problem statement

Approve the two lint exceptions in xtask/src/lints.rs (lines 3166 and 3862, from commit 230065f on main), or move the temper gate's comparison point to 230065f?

The temper gate scan is red only on two exceptions that came with 230065f, not with this branch.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-642765`. Anchor: `xtask/src/lints.rs:3862`.

## Decision drivers

- scope

## Considered options

### A · Accept as pre-existing (chosen)

Approve both as out of scope; re-base the temper gate to 230065f.

- Holds if they are 230065f's to answer for.
- If wrong: two unreviewed exceptions stay approved.

### B · Revert the citation fix

Leave lints.rs out of the diff; its comment cites lines 24 off.

- Holds if a stale comment is cheaper than an approval.
- If wrong: a wrong citation ships.

### C · Fix them here

Turn the allow into an expect with a reason; approve the too_many_lines expect.

- Holds if you want them cleaned now.
- If wrong: unrelated edits in a docs branch.

## Evidence

- `xtask/src/lints.rs:3862`: #![allow(clippy::unwrap_used)] in the test module
- `xtask/src/lints.rs:3166`: #[expect(clippy::too_many_lines, reason = ...)]
- `xtask/src/lints.rs:1651`: the one citation this branch repointed

## Decision outcome

Chosen option: **Accept as pre-existing**, the recommended option.

Decider's note: Accept as pre-existing; re-base the temper gate to 230065f.

### Consequences

- Good, because it holds if they are 230065f's to answer for.
- Bad, because if wrong: two unreviewed exceptions stay approved.

### Confirmation

Revisit when: a rule that a branch must not touch a file carrying an unapproved exception

## Why this might be wrong

if the exceptions were meant to be challenged at first touch

## Provenance

- Decided 2026-09-30T02:54:01Z by human:ryan (user-approved), via chat.
- Raised in session `f0834c2a-1fe5-44d1-9d05-77d50ee6b39b`.
- Verbatim: "kind: question"
