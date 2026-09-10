---
id: kb-decision-0060
title: The projection port keeps its gate, and the reason ADR-0036 gave has expired
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0060
reversibility: medium
phase: 11
supersedes: null
superseded_by: null
summary: >-
  ProjectionStore keeps its unstable-projection gate at 0.2.0 and the reason is
  new. ADR-0036 gated on scarcity — exactly one adapter had run the projection
  suite, at one end of PS-2's batch-shape axis — and that reason has expired:
  happenstance-neon is a cannot-hold-across-await adapter and it passes, the
  adapter population went from one to four across a file, a pooled server, a
  one-shot HTTP proxy and an embedded graph database, and phase 11's
  pre-registered condition (the batch needs a field the port cannot express)
  did not fire. The other end is unobservable rather than unbuilt. Both
  drivers PS-2 names are refuted by independent mechanisms: sqlx's
  Transaction::begin is async and fallible with private fields against a
  begin that is total, synchronous and infallible, so no total synchronous
  expression of Transaction<'static, Postgres> exists, and
  Pool::try_acquire yields a PoolConnection with BEGIN still a round trip;
  rusqlite's Transaction<'_> is !Send and costs the SendProjectionStore impl.
  probe_write and probe_delete_all close a second door independently, both
  synchronous and infallible, so even handed a live transaction there is no
  seam to issue a statement into. Deeper than the drivers: a store whose
  batch genuinely is a live transaction must declare
  READS_THROUGH_BATCH = false — a false statement about itself, since
  probe_read_through is synchronous, infallible and takes &Self::Batch — so
  it reports the same capability profile as a buffering one and the suite
  cannot tell the ends apart. The gate therefore stays because freezing
  begin, probe_write and probe_read_through would make a semver promise out
  of precisely the signatures that forbid the second shape: freezing them
  would make a promise out of the defect. That reason is indifferent to a
  fifth adapter arriving, which the old one was not. ADR-0036's asymmetry
  argument still decides the tie and PS-3 keeps [PROVISIONAL]. A signature
  change is proposed and deliberately not made — probe_read_through as
  &mut Self::Batch, async, returning Result — because it is breaking, it is
  PS-2's owner's call rather than an adapter lane's, and it is not sufficient
  alone. No compiler said any of this for a whole phase because todo!() has
  type ! and ! coerces to everything: happenstance-postgres declared
  type Batch = sqlx::Transaction<'static, Postgres> with five todo!() bodies
  and it type-checked, the type real and uninhabitable, which a skeleton
  cannot tell apart. Reopened by a live-transaction adapter that passes and
  can truthfully declare READS_THROUGH_BATCH = true, or by a fifth adapter
  finding a field the port cannot express; not reopened by another buffered-
  end adapter passing.
depends_on:
  - kb-decision-0036
related:
  - kb-decision-0017
  - kb-open-question-probe-read-through-signature-001
  - kb-open-question-provisional-falsifiers-001
  - kb-governance-referent-not-reasoning-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md
  - references/adr/0060-ps-2s-axis-re-evaluated.md
  - crates/happenstance-core/tests/probe_live_transaction_shape.rs
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-neon/tests/neon_projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
---

# The projection port keeps its gate, and the reason ADR-0036 gave has expired

## Context

PS-2 requires the projection port to stay unfrozen until two adapters at
opposite ends of a batch-shape axis pass the conformance suite. ADR-0036
applied that bar at phase 6 and found the second end unmet — no
cannot-hold-across-await adapter had passed at all. Phase 10b and phase 11
changed the facts twice, in opposite directions, and this decision is the
re-evaluation ADR-0036's own text asked for rather than a reversal of it.

**One end is now occupied.** `happenstance-neon` is a cannot-hold-across-await
adapter and it passes the projection suite. The population of adapters that
have run it went from one to four — a file (`happenstance-sqlite`), a pooled
server (`happenstance-postgres`), a one-shot HTTP proxy (`happenstance-neon`),
and an embedded graph database (`happenstance-ladybug`) — and phase 11's
pre-registered falsifier condition, that a batch would need a field the port
cannot express, did not fire.

**The other end turned out to be unobservable, not merely unbuilt.** Both
drivers PS-2 names as candidates for a live-transaction batch are refuted, and
by independent mechanisms — which is why finding one did not predict the
other. `ProjectionStore::begin` is total, synchronous and infallible; `sqlx`'s
only transaction constructor is `async` and fallible with private fields, so
no total synchronous expression of `Transaction<'static, Postgres>` exists,
and `Pool::try_acquire` still leaves `BEGIN` as a round trip. `rusqlite`'s
`Transaction<'_>` is `!Send`, which costs the `SendProjectionStore` impl
outright. `probe_write` and `probe_delete_all` close a second, independent
door: both are synchronous and infallible, so even a store handed a live
transaction has no seam to issue a statement into.

Deeper than either driver: a store whose batch genuinely *is* a live
transaction must declare `READS_THROUGH_BATCH = false`, because
`probe_read_through` is synchronous, infallible, and takes `&Self::Batch` — a
shape no borrowed-mutably, I/O-bound driver call can satisfy. That capability
constant is then a false statement about such a store, and the suite reports
the same profile for a live-transaction adapter as for a buffering one. The
axis's far end cannot be told apart from the near end by anything the port
currently asks.

## Decision

The port keeps its `unstable-projection` gate at `0.2.0`, and the reason
changes. Freezing `begin`, `probe_write`, and `probe_read_through` as they
stand would turn a semver promise into a promise about precisely the
signatures this finding shows forbid the live-transaction shape — freezing
the defect rather than the guarantee. That reason does not expire the moment
a fifth adapter arrives, unlike ADR-0036's scarcity argument. ADR-0036's own
asymmetry argument — a frozen port is a promise, a published crate can be
yanked but not withdrawn, and the cost of staying gated is a consumer typing a
feature name — still decides the tie, and PS-3 keeps `[PROVISIONAL]`: a SHOULD
still being satisfied is not a moved marker.

A signature change is named and deliberately not made:
`probe_read_through(&mut Self::Batch) -> impl Future<Output = Result<...>>`
would admit the axis end the current signature forecloses. It is left undone
because it is a breaking change to a trait testkit consumers implement,
because it is PS-2's owner's decision rather than an adapter lane's to make in
passing, and because it would not be sufficient alone — `probe_write` staying
synchronous and infallible still forces buffering, so the honest scope of a
fix is the whole probe seam, not one method.

**ADR-0036 is not superseded.** Its decision — gate the port — stands
unchanged; only its stated reason is replaced, recorded separately rather
than merged into the old text, because "only one adapter has run the suite"
was true when written and rewriting it into something it never said would
erase that it was once the whole argument.

## Why the compiler stayed silent

`happenstance-postgres` declared `type Batch = sqlx::Transaction<'static,
Postgres>` with five `todo!()` bodies for a whole phase, and it type-checked —
cited elsewhere in this repository as a model skeleton precisely because it
carried a real driver type. `todo!()` has type `!`, and `!` coerces to
everything, so a real-but-uninhabitable type and a real-and-inhabitable one
are indistinguishable to a skeleton. Only writing the bodies found the defect.

## Consequences

Thirteen `[PROVISIONAL]` clauses gated on PS-2 alone are unmoved by this
decision — it changes *how* the gate could be opened, not whether it is open
today. Reopened by a live-transaction adapter that both passes the suite and
can truthfully declare `READS_THROUGH_BATCH = true`, or by a fifth adapter
finding a field the port genuinely cannot express. Not reopened by another
adapter at the buffered end passing, however many arrive.
