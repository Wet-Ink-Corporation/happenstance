---
id: "kb-decision-wi-b56b44"
title: "Doc-only fix in this change"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"May I fix the stale rustdoc on a_concurrent_reader_never_sees_a_partial_batch in src/concurrency.rs, which still describes the wait as \"an atomic counter and std::thread::yield_now\"? It is a doc-only edit to src/, which you said to leave untouched.\", facing published rustdoc on a conformance rule describes an instrument mechanism that this change removed, we decided for Doc-only fix in this change and neglected Leave it owed, on the premise that the constraint was about behaviour, accepting that if wrong: src/ moved against the letter of the instruction."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-b56b44"
question: "May I fix the stale rustdoc on a_concurrent_reader_never_sees_a_partial_batch in src/concurrency.rs, which still describes the wait as \"an atomic counter and std::thread::yield_now\"? It is a doc-only edit to src/, which you said to leave untouched."
door: two-way
blast_radius: file
decider: "human:ryan"
provenance: user-approved
decided_at: "2026-10-08T04:52:15Z"
tree_hash: "9b8bbd90cc88369685f754a86cb4960bcfe4a6dc"
recommended: "A"
flip_condition: "The instruction meant byte-identical src/"
---

# Doc-only fix in this change

## Context and problem statement

May I fix the stale rustdoc on a_concurrent_reader_never_sees_a_partial_batch in src/concurrency.rs, which still describes the wait as "an atomic counter and std::thread::yield_now"? It is a doc-only edit to src/, which you said to leave untouched.

Published rustdoc on a conformance rule describes an instrument mechanism that this change removed.

Raised by an agent (marker) as a deferral and captured by Weigh-In as `wi-b56b44`. Anchor: `crates/happenstance-testkit/src/concurrency.rs:993`.

## Decision drivers

- Rule behaviour changed

## Considered options

### A · Doc-only fix in this change (chosen)

Rewrite the 4 lines to describe the census wait; the rule body is untouched.

- Holds if the constraint was about behaviour.
- If wrong: src/ moved against the letter of the instruction.

### B · Leave it owed

Keep the runbook's 'Still owed' entry.

- Holds if src/ must not move at all.
- If wrong: the docs.rs page stays wrong until someone picks it up.

## Evidence

- `crates/happenstance-testkit/src/concurrency.rs:993`: using an atomic counter and `std::thread::yield_now`, so the rejection is a rendezvous

## Decision outcome

Chosen option: **Doc-only fix in this change**, the recommended option.

Decider's note: Doc-only fix

### Consequences

- Good, because it holds if the constraint was about behaviour.
- Bad, because if wrong: src/ moved against the letter of the instruction.

### Confirmation

Revisit when: The instruction meant byte-identical src/

## Why this might be wrong

semver tooling or a reviewer reads any src/ diff as a rule change

## Provenance

- Decided 2026-10-08T04:52:15Z by human:ryan (user-approved), via chat.
- Raised in session `127a0aae-c5e8-45a1-b019-a198ad8718d2`.
- Verbatim: "kind: deferral"
