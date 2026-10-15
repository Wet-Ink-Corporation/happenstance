---
id: kb-open-question-sy-8-rule-line-multi-peer-001
title: SY-8's Rule line says every sync rule holds one peer, and the model family holds several
kind: open_question
status: accepted
authority_tier: note
summary: >-
  SY-8 is FROZEN and its Rule line reads "the whole of happenstance-sync-testkit — every rule in it
  takes exactly one peer handle. A rule that needed two would be testing the runner." The model
  family (#246) holds one peer handle per node of a generated topology, three to six of them, and
  plays the runner itself; SY-24's own FROZEN Rule line already names a three-peer A—B—C test, so
  the tension predates the family and the family only makes it concrete. Decided for #246 (D1):
  leave SY-8's text alone, narrow the crate documentation's "no rule holds two peers" to the
  worked-example rules, and record the tension here. Not decided: whether SY-8's Rule line is
  amended by ADR to say what it actually protects — the port describes one relationship, and a
  rule exercises each peer only through its own port — or whether multi-peer rules move to a crate
  or family that SY-8's line does not cover. Forced by the next FROZEN-clause edit to SY-8, by the
  runner (#244) arriving with rules of its own, or by a reviewer reading SY-8 against the
  testkit and finding them in contradiction.
depends_on: []
related:
  - kb-decision-0026
  - kb-open-question-model-family-rule-no-clause-001
source_paths:
  - spec/SPECIFICATION.md
  - crates/happenstance-sync-testkit/src/sync_model.rs
  - crates/happenstance-sync-testkit/src/lib.rs
  - crates/happenstance-sync-testkit/src/fixture.rs
last_reviewed: 2026-10-15
---

# SY-8's Rule line says every sync rule holds one peer, and the model family holds several

## What is true today

SY-8 (`spec/SPECIFICATION.md`, §5) is `[FROZEN]`. Its normative sentence is about the **port**:
`SyncPeer` describes exactly one peer relationship, and fan-out, ordering between peers and
reconciliation live in a runner above it. Its `Rule:` line goes further and makes a claim about
the **testkit**: "the whole of `happenstance-sync-testkit` — every rule in it takes exactly one
peer handle. A rule that needed two would be testing the runner."

That line was true of the worked-example family in `sync_suite.rs`, where each rule opens one
fixture, connects once, and plays the far side itself. It is not true of the model family that
#246 added in `sync_model.rs`, behind the crate's `proptest` feature. That family opens **one
fixture per node** of a generated topology of three to six peers, holds a handle on each, and
drives push and pull between them as a minimal runner of its own, so that it can check the
clauses that are only statements about a set of peers: SY-24 and SY-4 (every peer ends up holding
the union), SY-11 (each holds it once) and SY-25 (each forwards in arrival order).

The tension did not start with #246. SY-24's own `[FROZEN]` `Rule:` line names
`transitive_convergence_over_a_partial_mesh` as "three peers A—B—C, A and C never communicate,
each peer syncs only with its neighbour", which is a three-handle test by construction. Two frozen
clauses therefore already disagreed about what a sync rule may hold; the model family is the
first code that had to pick one.

What #246 did (decision D1, recorded in its plan): left SY-8's text alone, narrowed the crate
root's sentence to "no rule in `rules` holds two peers", added the same scope to
`SyncFixture`'s "One peer handle per rule" section, and stated in `sync_model`'s module
documentation that each peer is still driven only through its own port, one relationship at a
time — which is what SY-8's normative sentence asks of the port.

## What is not decided

Whether SY-8's `Rule:` line should be amended — by a new ADR, since the clause is frozen — to
state what it protects (no rule needs a port method that takes a peer set; every peer is
exercised through its own port) rather than a count of handles; or whether rules that hold
several peers belong somewhere SY-8's line does not reach, such as a runner-level suite once
#244's runner exists.

## What forces it

- The next edit to SY-8 for any other reason.
- #244's runner arriving with conformance rules of its own, which would hold several peers for
  the reason SY-8's line forbids — they would be testing the runner, deliberately.
- A reviewer, or `spec-trace` if it ever learns to read `Rule:` lines as claims, finding the
  frozen line and the testkit in contradiction.

## Ordered sub-questions

1. Is SY-8's `Rule:` line part of the frozen proposition, or the clause's evidence? If evidence,
   a correction is a repointing, not an amendment.
2. If amended: what is the testable property — "no rule calls a method taking a peer set", or
   "every rule's assertions are about one peer's port at a time"?
3. Should the model family, or a successor, move to a runner-level crate once the runner exists?
