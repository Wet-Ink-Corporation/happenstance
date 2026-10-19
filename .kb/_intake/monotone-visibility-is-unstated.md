# Proposal for an ADR pass: make "visibility never moves backward" normative

Raised by #79 (IMPL-7), 2026-10-19. Raw material for an ADR pass, not an atom.

## What is true now

`ops_agree_with_the_model` runs a **trailing model** against a fixture that
declines `READ_YOUR_OWN_WRITES` (`crates/happenstance-testkit/src/model/trailing.rs`).
Every check it makes composes existing clauses (ES-10, ES-11, ES-19, ES-25,
VT-11) except one: **a later full read never shows fewer events than an earlier
one on the same handle.** The trailing model asserts it (check T1(b), message
"the frontier went backward") and the lower end of its read bracket (T2) rests on
it. `RetreatingFrontierStore` in `crates/happenstance-testkit/tests/mutation_coverage/trailing.rs`
is the wrong store that breaks it, and the model rejects it.

No clause states it. ES-10 says position order is visibility order, which makes
the visible set a prefix at any instant. It does not say the prefix only grows.
Both frontier adapters meet it by construction, because `pg_snapshot_xmin` of a
fresh snapshot on one primary does not decrease, and the strict model implies
it.

This was recorded as decision D1 in `.temper/plans/model-family-behind-a-frontier.md`,
taken as option **A** (a stated testkit assumption) by the delegating agent as a
two-way default. It is not a human decision.

## What is proposed

Option **C** from that plan: make it normative. Either

1. amend ES-10, which is `[FROZEN]`, so this is one-way and needs its own ADR; or
2. mint a new provisional clause beside it, which raises the clause census by
   one and also needs an ADR.

## What forces it

- A store behind a read replica, or a Neon pooler that hops between computes,
  could break it. Today that store fails a conformance rule on an assumption the
  specification does not make, which is a rule asserting more than the contract.
- CF-5's argument: a rule that rejects a store the specification permits is the
  rule's defect. Either the specification forbids a retreating frontier, or the
  trailing model must stop assuming it never retreats.

## What reopens it

A live red on the `live-postgres` or `live-neon` CI job naming "the frontier went
backward". That would mean the premise is false for a real deployment, and the
ADR pass has to choose between the clause and weakening T1(b) and T2.
