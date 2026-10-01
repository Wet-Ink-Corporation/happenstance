---
id: "kb-decision-wi-2ab1f3"
title: "Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed: B: independence is a promise about conflict, not liveness; Busy allowed; rule renamed k_disjoint_boundaries_never_conflict"
kind: decision
status: accepted
authority_tier: decision
summary: "In the context of \"Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed?\", facing the question an agent raised, we decided for B: independence is a promise about conflict, not liveness; Busy allowed; rule renamed k_disjoint_boundaries_never_conflict."
depends_on: []
related: ["kb-decision-0077"]
source_paths: ["crates/happenstance-testkit/src/concurrency.rs", ".kb/decisions/0077-appenderror-busy.md"]
last_reviewed: "2026-09-30"
reversibility: low
phase: 17
supersedes: []
superseded_by: null
weighin_item: "wi-2ab1f3"
question: "Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed?"
door: one-way
blast_radius: external
decider: "human:ryan"
provenance: user-directed
decided_at: "2026-09-30T20:36:46Z"
tree_hash: "b94e808e228aea7de2693778b023bf8fb50c0893"
---

# Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed: B: independence is a promise about conflict, not liveness; Busy allowed; rule renamed k_disjoint_boundaries_never_conflict

## Context and problem statement

Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed?

Raised by an agent (sweep) as a call and captured by Weigh-In as `wi-2ab1f3`. Anchor: `crates/happenstance-testkit/src/concurrency.rs`.

## Decision outcome

Chosen option: **B: independence is a promise about conflict, not liveness; Busy allowed; rule renamed k_disjoint_boundaries_never_conflict**.

Decider's note: Owner chose B after the options walkthrough, and asked for the rename while the testkit's 0.4.0 major makes it free.

## Provenance

- Decided 2026-09-30T20:36:46Z by human:ryan (user-directed), via chat.
- Raised in session `c8638052-7723-4bfd-8e86-255fbf4bff8b`.
- Verbatim: "Does DCB independence extend to Busy: may a command on a disjoint boundary be refused as Busy, or must it always succeed?"
