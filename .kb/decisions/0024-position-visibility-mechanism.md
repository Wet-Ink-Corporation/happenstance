---
id: kb-decision-0024
title: happenstance-postgres buys position visibility with xid8 and pg_snapshot_xmin
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0024
reversibility: medium
phase: 10
supersedes: null
superseded_by: null
summary: >-
  The Postgres adapter buys ES-10's visibility invariant with an xid8 column and a
  pg_snapshot_xmin frontier predicate, measured rather than preferred: no measurable steady-state
  cost, and staleness bounded by the remainder of the longest open write transaction on the
  cluster. CF-13 still passes and the pass is not evidence — the naive arm passes too — so the
  mechanism is evidenced by direct staleness observation. No spec clause is added and ES-10's
  frozen marker does not move; ADR-0013 stays accepted and byte-identical.
depends_on:
  - kb-decision-0013
related:
  - kb-reference-position-visibility-adapter-remeasurement-001
  - kb-open-question-postgres-arm-c-cost-001
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-open-question-off-poll-visibility-defect-001
source_paths:
  - .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md
  - .kb/_intake/2026-09-06-poll-count-calibrated-and-a-second-limitation.md
  - references/adr/0024-position-visibility-mechanism.md
last_reviewed: 2026-09-07
---

# happenstance-postgres buys position visibility with xid8 and pg_snapshot_xmin

## Decision

`happenstance-postgres` buys ES-10's position-visibility invariant with `xid8` +
`pg_snapshot_xmin`. Every row carries `pg_current_xact_id()`; every `read` and `head()` admit only
rows beneath `pg_snapshot_xmin(pg_current_snapshot())`. `position` keeps no column default — the
sequence is read explicitly at the single `INSERT` that allocates, so "not `bigserial`" stays a
fact `information_schema` can check rather than a claim in prose.

This answers `kb-decision-0013`'s open mechanism question and nothing else. ADR-0013 lifted ES-10
to `[FROZEN]` on the strength of *some* affordable mechanism existing and explicitly left which
mechanism an adapter buys to phase 10. This decision does not touch ADR-0013's body, adds no
clause to `spec/SPECIFICATION.md`, and does not move ES-10's `[FROZEN]` marker. The full
transcript, the rejected alternatives, and the before/after numbers live in
`references/adr/0024-position-visibility-mechanism.md`; this atom cites it rather than restating
it.

## The two measurements, and the honest weight of each

Taken against the **built** adapter — pool, a transaction tied to `append`'s async boundary, a
cursor, error mapping — with a naive arm (same store, frontier predicate removed) as the paired
baseline, inside a container over the unix socket under `fsync=on`. Numbers and method in
`kb-reference-position-visibility-adapter-remeasurement-001`.

**Steady state: no measurable cost, and the imprecision is itself the finding.** Medians of
0.985 / 1.131 / 1.026 at 1 / 8 / 32 clients, spread about ±20%, against a phase-2 effect estimate
of 0.99–1.03. The measurement cannot resolve the cost further than that — it establishes only that
no cost large enough to matter hides inside the noise, which is sufficient because the two rejected
arms below cost 16x and 30x.

**Staleness: the real bill, and this one is tight.** Under a 5-second held write transaction, the
unguarded baseline moves from a 0.521 ms control to 0.595 ms; arm C moves from 0.593 ms to
**4799.3 ms**. The unguarded row is the control phase 2 never had — it compared arm C against
itself, which shows the frontier moves but not that the frontier is what moved. Under an identical
hold, same server, same load, only the predicate differs.

## CF-13 passes, and the pass is not evidence

The *naive* arm — the shipped store with its frontier predicate removed, reachable only behind an
off-by-default `naive-arm` feature — passes `nothing_below_an_observed_position_appears_later`
too, because this adapter advances **off-poll**: its transaction opens and commits on a captured
runtime `Handle`'s schedule rather than a poll-driven state machine the suite's schedule can
interleave against. `crates/happenstance-postgres/tests/naive_arm_probe.rs` demonstrates the
inversion directly rather than relying on the conformance rule to catch it. So the mechanism here
is evidenced by the staleness table above, not by a green conformance run — recording "conformant,
therefore correct" would be exactly the inference the naive-arm probe exists to refute. Whether the
suite should be able to catch an off-poll adapter's visibility defect at all is not decided here;
see `kb-open-question-off-poll-visibility-defect-001`.

## What `sqlx` forced, and what it did not

The port's trait signature is untouched. Three real costs were forced: a captured runtime `Handle`
for every operation, since `Handle::enter` is `!Send` and would break `SendEventStore`; the cursor
carrying that handle so `PoolConnection::drop` can return its connection; and `SERIALIZABLE` plus a
bounded retry on the conditional append path, because `READ COMMITTED` lets two writers racing one
boundary both win, and taking a write lock is the move this crate exists not to make.

## The losing arms, and why B-tag stays rejected on principle rather than cost

A (a serialised sequence table, 0.062x) and B-const (a constant advisory lock, 0.033x) at 64
writers are correct but delete the reason to reach for Postgres. B-tag (0.935x, nearly free)
reproduces the baseline inversion on disjoint keys — it is cheap because it buys a **weaker**
invariant, per-boundary where ES-10 is global, so ranking the arms by throughput alone would have
chosen the wrong one. B-tag's branch was "reconsider if arm C proves structurally expensive"; it
did not, so B-tag is not reconsidered.

## Why no new spec clause

Three consumer-facing consequences — a frontier `head`, no read-your-own-writes, cluster-wide
staleness — were weighed for a clause of their own and refused. No conformance rule can fail an
adapter over them: they are permissions granted, not requirements binding, and a clause naming no
rule and no wrong implementation is decorative. The one testable sentence here —
a head is a bound, not an equality — is already `head_is_the_highest_visible_position` under
ES-30. A second copy of the visibility invariant is a drift this specification has already
suffered once: ES-10's frozen prose still quotes phase 2's superseded numbers, unedited because it
is frozen.

## Inherited, not owned

ES-10's global framing rests on the projection checkpoint being global, which is phase 6's
decision, not this one's. A boundary-scoped checkpoint could make B-tag's weaker invariant
sufficient and would reopen both ADR-0013 and this measurement. See
`kb-open-question-global-vs-boundary-visibility-001`.

## What this does not decide

Whether the conformance suite can detect an off-poll adapter's visibility defect at all, and with
what instrument — the port exposes no suspension point between allocation and commit, so the thing
that catches it here is adapter-specific test code, not a portable rule. The global-versus-per-boundary
question. `contains_event_id`'s frontier disagreement, which leaves ES-41 `[PROVISIONAL]`. And the
steady-state ratio itself, which is owed a measurement instrument that can resolve inside ±20%.
