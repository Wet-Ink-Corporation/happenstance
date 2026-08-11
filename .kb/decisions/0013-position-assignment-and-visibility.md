---
id: kb-decision-0013
title: Positions are assigned once and become visible in order
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0013
reversibility: low
phase: 4
supersedes: null
superseded_by: null
summary: >-
  The visibility invariant lifts from provisional to frozen: once any reader has observed an
  event at position P, no later read against that store may yield an event at a position at or
  below P that was not already visible, and an adapter must not make an event visible below one
  it has already exposed. An adapter that allocates positions before it commits does not
  satisfy this — a position taken from a sequence at transaction start is released at commit,
  so a later transaction can become visible earlier and an append condition's after boundary
  stops enforcing with no error anywhere. The lift is grounded in a phase-2 measurement against
  real PostgreSQL: xid8 plus pg_snapshot_xmin is the only one of four arms that both passes an
  inversion detector and leaves writers unserialised, at 0.987/0.993/1.015/1.026 throughput
  ratio at 1/8/32/64 clients. Three caveats travel with the freeze rather than being left
  implicit. First, freezing the clause moves the position-allocation axis into this decision's
  own risk acceptance, because the marker itself was that axis's disclosure. Second, the
  invariant stays global rather than per-boundary on purpose: a per-boundary invariant would
  make the append condition sound and the projection checkpoint unsound, since head() is
  deliberately not query-scoped, and that trade is rejected at a measured nine percent cost.
  Third, head() reports the visibility frontier, not the maximum assigned position, because
  under the accepted mechanism a reader's visibility is a predicate over a snapshot rather than
  an identity, and an adapter choosing this mechanism does not satisfy read-your-own-writes —
  staleness is bounded by the longest open write transaction anywhere on the server, measured
  from 0.688 ms to 4010.719 ms behind an unrelated write. SequencePosition::next uses
  NonZeroU64::checked_add instead of saturating_add, so overflow correctly signals None instead
  of silently returning the wrong next position; the Option costs nothing extra because the
  type's forbidden-zero niche already pays for it. checkpoint.next() is the sound resume idiom
  over a store with gaps, because ReadOptions::from is a threshold predicate rather than an
  equality seek, so a resume position that lands on a gap simply finds the next occupied one
  rather than stalling.
depends_on:
  - kb-decision-0010
related:
  - kb-decision-0011
  - kb-reference-position-visibility-experiment-001
  - kb-concept-torn-read-append-boundary-001
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/src/store.rs
  - experiments/position-visibility/
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# Positions are assigned once and become visible in order

## Decision

The visibility invariant lifts from `[PROVISIONAL]` to `[FROZEN]`: once any reader has observed
an event at position *P*, no subsequent read against that store may yield an event at a
position at or below *P* that was not already visible, and an adapter must never make an event
visible at a position below one it has already exposed. The lift condition the marker itself
named — one affordable mechanism — is met: a phase-2 experiment ran four candidate SQL
strategies against a real PostgreSQL with `fsync=on`, and `xid8` plus `pg_snapshot_xmin` is the
only one that both passes an inversion detector on concurrent writer pairs and leaves writers
unserialised, at a throughput ratio close to baseline across 1, 8, 32 and 64 clients. Two
serialising strategies were rejected on the same measurement — they collapse to a fraction of
baseline throughput at high concurrency, which would delete the reason a Postgres adapter
belongs in this workspace at all: sitting at the far end of the position-allocation axis every
other adapter shares.

Three caveats are written into the freeze rather than left for a later reader to infer. The
freeze *removes* a disclosure rather than only adding a guarantee — the provisional marker was
itself the acceptance of risk on this axis, so lifting the clause without stating the
acceptance somewhere would lose it silently, which is exactly the failure the acceptance
mechanism exists to prevent. The invariant is kept global rather than per-boundary, a real cost
measured near nine percent at high concurrency, because a per-boundary invariant is sound for
the append condition and unsound for the projection checkpoint, whose global resume position
covers boundaries it never reads. And `head()` on the accepted mechanism is a visibility
frontier rather than the maximum assigned position — the two disagree under load because
visibility there is a snapshot predicate, not an identity — with a real, measured cost:
`append` returning `Ok` does not promise the next `head()` call sees it, and staleness tracks
the slowest open write transaction anywhere on the server, including in an unrelated database.

`SequencePosition::next` is repaired from `saturating_add` to `NonZeroU64::checked_add`,
returning `None` on overflow as its own doc comment already promised but its old implementation
could not deliver — `saturating_add` clamps and wraps the clamped value in `Some`, handing a
caller a position that is not actually next. The fix costs nothing at runtime because
`NonZeroU64`'s forbidden-zero bit pattern already gives `Option<SequencePosition>` the same
size as `SequencePosition` itself. `checkpoint.next()` is documented as the correct, gap-safe
resume idiom on every adapter, not only densely-allocating ones, because reads treat a resume
position as a threshold rather than an exact seek.

## What the lift does not decide

Which mechanism a real `happenstance-postgres` adapter uses is left open for the adapter's own
phase; this decision only establishes that one affordable mechanism exists. The single
conformance rule behind the lift has an unbounded strength — it drains whatever poll
interleaving a hand-written schedule leaves unfinished, and a store whose `append` genuinely
suspends across more than the rule's two polls could pass without ever exercising the window.
That limitation is recorded rather than fixed, with its calibrating instrument named for a
later phase with a real adapter to run it against.

## Alternatives rejected

Leaving the clause provisional despite its own stated condition being met was rejected as
teaching every future reader that maturity markers are decoration. Weakening the invariant to
per-boundary was rejected on the projection-checkpoint argument above. Declaring `head()` to be
the maximum position and the accepted mechanism non-conformant was rejected as incoherent with
the invariant itself — that maximum names a position no read will yield.
