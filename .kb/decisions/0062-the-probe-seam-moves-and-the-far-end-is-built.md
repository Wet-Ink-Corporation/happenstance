---
id: kb-decision-0062
title: The probe seam moves, begin moves with it, and the far end is built
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0062
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Recorded from the 2026-09-10 brief for a decision taken on
  lane/projection-probe-seam; at this worktree's HEAD (86a410c) the long form
  the brief names is absent, ProjectionStore::begin is still synchronous at
  crates/happenstance-core/src/projection.rs:460 and PS-6's MUST is
  unrewritten at spec/SPECIFICATION.md:5183 — this atom states what the lane
  binds when it lands, not a fact already true of main. The decision: the
  probe seam moves, all of it. probe_write, probe_delete_all and
  probe_read_through take &mut Self::Batch and return
  impl Future<Output = Result<..., Self::Error>> with no Send bound, and
  begin moves with it — async fn begin(&self) -> Result<Self::Batch,
  Self::Error> — because with the probe moved and begin unmoved a
  live-transaction store could report itself and still could not exist for
  sqlx. PS-6's falsifier ("an adapter that must reserve something from the
  server before the first write") had fired — it is sqlx's BEGIN — and its
  MUST is rewritten to the discipline the signature protected: a buffering
  adapter's begin resolves at its first poll and never fails, held by two
  named tests. The far end is built: LivePostgresProjectionStore in
  happenstance-postgres beside the buffered store, Batch owning
  sqlx::Transaction<'static, Postgres>, READS_THROUGH_BATCH = true as a true
  statement, and the brief records 20 of 20 against live PostgreSQL. What the
  two ends disagreed about needed no port change, so PS-2's MUST is met as
  written. Deliberately not decided: lifting the gate, and whether
  Projection::apply moves.
depends_on:
  - kb-decision-0060
  - kb-decision-0036
  - kb-decision-0017
related:
  - kb-open-question-probe-read-through-signature-001
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-require-the-property-001
  - kb-playbook-repair-frozen-clause-001
  - kb-decision-0025
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-core/tests/probe_live_transaction_shape.rs
  - crates/happenstance-postgres/src/projection_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-11
---

# The probe seam moves, begin moves with it, and the far end is built

## Provenance note

This atom is minted from the 2026-09-10 intake brief for a decision taken on
`lane/projection-probe-seam`. At this worktree's `HEAD` (`86a410c`) the long
form the brief names — `references/adr/0062-the-probe-seam-moves-and-the-far-end-is-built.md`
— is not present in this checkout, `ProjectionStore::begin` is still
synchronous (`fn begin(&self) -> Self::Batch`,
`crates/happenstance-core/src/projection.rs:460`), and PS-6's MUST at
`spec/SPECIFICATION.md:5183` is unrewritten. This record states what the lane
binds when it lands, mirroring `kb-decision-0037`'s precedent for a decision
recorded ahead of its code landing on the branch that carries it.

## The one question

ADR-0060 found PS-2's far end unreachable through `begin`'s signature and
unobservable through the probe's own, named the signature change that would
fix it, and left the call to PS-2's owner. Does the seam move, and what does
it cost?

## The decision

**It moves, all of it.** `probe_write`, `probe_delete_all` and
`probe_read_through` take `&mut Self::Batch` and return
`impl Future<Output = Result<..., Self::Error>>`, with no `Send` bound.
**`begin` moves with it** — `async fn begin(&self) -> Result<Self::Batch,
Self::Error>` — because a probe seam moved without `begin` would let a
live-transaction store report itself and still be unable to exist for
`sqlx`: the phase-10b refutation was always a property of `begin`'s
signature, not of the port's axis.

PS-6's falsifier — *"an adapter that must reserve something from the server
before the first write"* — had already fired, unremarked: it is `sqlx`'s
`BEGIN`. Its MUST is rewritten to the discipline the old synchronous
signature used to protect by construction: a buffering adapter's `begin`
resolves at its first poll and never fails. Two tests hold that where the
signature no longer can — `begin_makes_no_round_trip` (Neon, over a
transport that fails every request) and
`begin_resolves_at_its_first_poll_without_a_runtime` (contract crate, polled
once with no executor).

The change is breaking to every implementer, and free under the
`unstable-projection` exemption ADR-0036 granted and ADR-0060 kept for
exactly this moment: the brief records eight probe implementations, one new
runner error arm (`ProjectionError::Begin`), 117 call sites, and one
constitution `compile_fail` fence that would otherwise have started passing
for the wrong reason.

## The far end, built

`LivePostgresProjectionStore` in `happenstance-postgres`, beside the
buffered store: `type Batch` owns `sqlx::Transaction<'static, Postgres>`, and
`READS_THROUGH_BATCH = true` is now a true statement about it. The brief
records 20 of 20 against a live PostgreSQL, including an adapter-private test
asserting `RuleOutcome::Ran` for `batch_reads_reflect_pending_writes` and
`rebuild_is_chunk_size_invariant` — the first time either rule has run
against anything but an in-process map. The buffered store stayed green
throughout.

What the two ends disagreed about needed no port change: an explicit
`ROLLBACK` on the regression path, PS-7 met by the driver's
rollback-on-drop, and a refused statement poisoning the transaction and
surfacing through the seam's new `Result`. Each is the far end being harder
to implement, not the port being wrong. **PS-2's MUST is met as written.**

## Rejected

`rusqlite` at the far end with `begin` kept synchronous (its
`Transaction<'_>` is `!Send`, forcing an owned-connection batch to acquire on
faith); `probe_read_through` moved alone (ADR-0060 said insufficient,
confirmed); splitting `READS_THROUGH_BATCH` (removes the false statement,
leaves the hole it was covering); rewording PS-2 to drop the far-end
requirement (the monoculture its own `Rejects` clause forbids); replacing the
buffered Postgres batch with the live one (the buffered store is the
product; the live one sits beside it as an instrument).

## Deliberately not decided

**Lifting the gate** — its own record, `kb-decision-0063`. What that record
weighs: the typed layer's `Projection::apply` is still synchronous, so an
application can push into a buffered batch and cannot issue a statement into
a live one. The live store is an instrument for the *port's* freeze here;
whether `apply` moves is the typed layer's axis, not this port's.

## Falsifier

Reopened by an adapter at either end the moved seam still cannot describe, or
by a buffering adapter for which the `async` `begin` genuinely costs a round
trip. Not reopened by `apply` staying synchronous.
