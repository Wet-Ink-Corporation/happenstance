---
id: "kb-decision-wi-51f0e9"
title: "Add test-util"
kind: decision
status: accepted
authority_tier: decision
adr_id: null
summary: "In the context of \"Add tokio's `test-util` feature to happenstance-neon's dev-dependency so `a_hung_read_times_out_and_settles` runs on paused time?\", facing the reviewer flagged a 1 ms real-time tokio timeout in a test (TST-10); paused time needs tokio's test-util feature, we decided for Add test-util and neglected Keep the real timer, on the premise that you want the TST-10 rule met literally, accepting that if wrong: one more dev feature for a 1 ms wait."
depends_on: []
related: []
source_paths:
  - .kb/_intake/decisions/wi-51f0e9-add-tokio-s-test-util-feature-to-happenstance.md
last_reviewed: "2026-10-08"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-51f0e9"
question: "Add tokio's `test-util` feature to happenstance-neon's dev-dependency so `a_hung_read_times_out_and_settles` runs on paused time?"
door: two-way
blast_radius: module
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-10-08T13:20:39Z"
tree_hash: "dd6bad51954e6e40662347a5f2a6f018f407350b"
recommended: "A"
flip_condition: "the test racing real work instead of pending()"
---

# Add test-util

## Context and problem statement

Add tokio's `test-util` feature to happenstance-neon's dev-dependency so `a_hung_read_times_out_and_settles` runs on paused time?

The reviewer flagged a 1 ms real-time tokio timeout in a test (TST-10); paused time needs tokio's test-util feature.

Raised by an agent (marker) as a question and captured by Weigh-In as `wi-51f0e9`. Anchor: `crates/happenstance-neon/tests/support/transport.rs:1`.

## Decision drivers

- manifest change

## Considered options

### A · Keep the real timer

No manifest change; the outcome is deterministic

- Holds if the inner future stays pending().
- If wrong: a later edit makes it race real time.

### B · Add test-util (chosen)

start_paused = true on the test

- Holds if you want the TST-10 rule met literally.
- If wrong: one more dev feature for a 1 ms wait.

## Evidence

- `crates/happenstance-neon/tests/support/transport.rs:1`: bounded(core::future::pending(), Duration::from_millis(1), Some(ticket))

## Decision outcome

Chosen option: **Add test-util**, overriding the recommendation (A).

Decider's note: add tokio test-util to happenstance-neon's dev-dependency; start_paused on the test

### Consequences

- Good, because it holds if you want the TST-10 rule met literally.
- Bad, because if wrong: one more dev feature for a 1 ms wait.

### Confirmation

Revisit when: the test racing real work instead of pending()

## Why this might be wrong

the rule exists so future edits don't silently become timing-bound

## Provenance

- Decided 2026-10-08T13:20:39Z by human:ryan (user-directed), via chat.
- Raised in session `6ec241b3-3771-51a8-b5a3-78697abb6042`.
- Verbatim: "kind: question"
