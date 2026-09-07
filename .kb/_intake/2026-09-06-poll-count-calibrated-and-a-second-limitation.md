# Intake — the poll-count question is answered, and a second one is opened

Staged for `/redkiln:kb-ingest`. This **resolves**
`kb-open-question-poll-count-rule-strength-001` and **opens** a successor. Do not
hand-author atoms from it; the ADR-0024 author is the intended reader.

## The question this closes

`.kb/open-questions/poll-count-bounds-the-visibility-rule.md`:
*"`nothing_below_an_observed_position_appears_later` … its discriminating power
varies with something nobody has measured: the adapter's poll shape … Refuted by
a poll-padding decorator over `PreCommitPositionStore` that the rule fails to
reject, calibrated against a three-poll adapter. Owned by phase 10."*

Phase 10 built the decorator, calibrated it, ran it, and it fired.

## The calibration, which is measured rather than chosen

ADR-0013 refused to let an author pick `n`: *"an author choosing n is the
reference-store failure mode with one more step — so the calibration waits for an
adapter with real I/O."* Sub-question 3 asked whether `n` should come from
`happenstance-postgres`'s real `append` or from a synthetic worst case.

It comes from the adapter. `crates/happenstance-postgres/tests/poll_shape.rs`
polls the shipped `append` against a live PostgreSQL 17.10:

| regime | polls to completion |
| --- | --- |
| one millisecond between polls | **3** |
| tight loop, nothing between polls | ~25,000 (26,740 and 25,096 on two runs) |

**Three** is the number, and it is the number `spec/SPECIFICATION.md` already
names: *"against a store whose `append` needs three polls the interleaving window
never opens where the rule looks and the rule cannot fail."* ADR-0013 predicted
the shape and phase 10 supplied the adapter that has it.

## The instrument, and what it showed

`PollPaddedPositionStore` is `PreCommitPositionStore` — positions allocated
outside the transaction, which is what Postgres does by default — with one extra
`Pending` in `append`. Same defect; two polls becomes three. It is registered in
`for_each_mutant!` and carries a `REGISTRY` row declaring the two rules its
defect implies.

Under the old `A, B, B, A` schedule it **passed**
`nothing_below_an_observed_position_appears_later`. The specification's own
disposition applies: *"if it fires, the rule changes and this clause does not."*

## What changed, and what did not

The rule's **schedule**: the slow writer is polled once to take its number, then
the fast writer is driven **to completion** rather than polled a fixed number of
times. It asks for what the rule needs — the fast writer has committed — rather
than a proxy that implies it only where a store suspends exactly once.

Unchanged: the rule's name, its assertions, its `observe` helper, ES-10 (still
`[FROZEN]`, and `spec/SPECIFICATION.md` is not edited). Every in-tree adapter
still passes; both `PreCommitPositionStore` and `PollPaddedPositionStore` now
fail it.

Answering sub-question 4 directly: the fix changed **only** the schedule, as
ADR-0013 predicted. Nothing about discovering a real three-poll adapter argues
the poll-count limitation should have gated the freeze — the clause was right and
the instrument was missing.

Sub-question 2 (`POLL_BUDGET` as a fixture capability) is **moot**, and that is
worth stating rather than leaving open: the new schedule needs no poll budget,
because it stopped counting polls. ADR-0013's CF-33 tension does not have to be
resolved.

## The successor question, which is not the same one

An adapter whose `append` hands its work to a runtime advances **off-poll**. Its
transaction opens and commits on the runtime's schedule, so no poll-based
schedule decides when. That is what the two numbers in the table above mean: for
a poll-driven store they are equal, because the poll count is a property of the
state machine; here they differ by four orders of magnitude, because the polls
are only asking whether the work has finished.

`happenstance-postgres` is that shape necessarily — `sqlx` needs a runtime in
thread-local scope, and the suite's own concurrency contenders run on raw threads
that have none, so the store hops onto a captured `Handle`.

**Consequence:** a deliberately naive arm of the adapter — the shipped store with
the frontier predicate removed, reachable only behind the off-by-default
`naive-arm` feature — passes CF-13 even after the schedule change.
`crates/happenstance-postgres/tests/naive_arm_probe.rs` shows the defect is real
by building the inversion by hand:

```
naive   | after fast commit: [2]      <- reader observes 2
naive   | after slow commit: [1, 2]   <- 1 appears BENEATH it
shipped | after fast commit: []
shipped | after slow commit: [1, 2]   <- never inverts
```

The decorator cannot reach this: padding a poll-driven state machine produces a
slower poll-driven state machine, not an off-poll one.

**What is not decided.** Whether the conformance suite should be able to detect
an off-poll adapter's visibility defect at all, and if so with what instrument —
the port exposes no suspension point between allocation and commit for a rule to
wedge, and the thing that does catch it (holding a transaction open) is
adapter-specific and lives in the adapter's own tests. Owner: ADR-0024.

## Links the atoms should carry

- `[[0013-position-assignment-and-visibility]]` — the prediction, now discharged
- `[[poll-count-bounds-the-visibility-rule]]` — resolved by this
- `[[postgres-arm-c-structural-cost]]` — the sibling question ADR-0024 also owns
- `spec/SPECIFICATION.md` ES-10 — untouched, and the disposition that authorised
  the rule change
- `crates/happenstance-testkit/src/suite.rs` — the schedule and its rustdoc
- `crates/happenstance-postgres/tests/poll_shape.rs` — the calibration
