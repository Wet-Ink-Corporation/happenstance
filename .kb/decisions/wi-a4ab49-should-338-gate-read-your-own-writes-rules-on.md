---
id: "kb-decision-wi-a4ab49"
title: "Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier: settle-frontier-hook"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier?\", facing the question an agent raised, we decided for settle-frontier-hook."
depends_on: []
related: []
source_paths: []
last_reviewed: "2026-11-01"
reversibility: high
phase: null
supersedes: []
superseded_by: null
weighin_item: "wi-a4ab49"
question: "Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier?"
door: two-way
decider: "human:claude"
provenance: user-directed
decided_at: "2026-11-01T00:59:05Z"
tree_hash: "97bfc1967c0b1d544eae97aedc2fc8be45f18033"
---

# Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier: settle-frontier-hook

## Context and problem statement

Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier?

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-a4ab49`. Anchor: `crates/happenstance-testkit/src/lib.rs:1`.

## Decision outcome

Chosen option: **settle-frontier-hook**.

Decider's note: Settled by Ryan's A on wi-87e964: the fixture brings the frontier forward.

## Provenance

- Decided 2026-11-01T00:59:05Z by human:claude (user-directed), via chat.
- Raised in session `372d1deb-288a-5665-a0d6-d5c62b1d61c1`.
- Verbatim: "Should #338 gate read-your-own-writes rules on the capability, or make them read to a settled frontier?"
