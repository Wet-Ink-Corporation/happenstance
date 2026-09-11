# ADR-0062 is written and wants an atom

**Date:** 2026-09-10
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Long form:** `references/adr/0062-the-probe-seam-moves-and-the-far-end-is-built.md`
**Acts on:** ADR-0060 §3, which named this change and declined to make it
**Amends:** PS-2's `Rule` (MUST, maturity and `Cases` untouched — and the MUST is now met); PS-6's MUST, with its falsifier recorded as fired
**Depends on:** kb-decision-0060, kb-decision-0036, kb-decision-0017
**Resolves:** kb-open-question-probe-read-through-signature-001 — the seam moved, at seam grain, and `begin` with it

## The one question

ADR-0060 found PS-2's far end unreachable through `begin`'s signature and
unobservable through the probe's, named the signature change, and left it to
PS-2's owner. **Does the seam move, and what does it cost?**

## The decision

**It moves, all of it.** `probe_write`, `probe_delete_all` and
`probe_read_through` take `&mut Self::Batch` and return
`impl Future<Output = Result<…, Self::Error>>`, no `Send` bound.
**And `begin` moves with it** — `async fn begin(&self) -> Result<Self::Batch,
Self::Error>` — because with the probe seam moved and `begin` unmoved a
live-transaction store could report itself and still could not exist for `sqlx`.
The phase-10b `sqlx` refutation was a property of `begin`'s signature, not of
the axis.

**PS-6's falsifier had fired and nobody had said so.** *"An adapter that must
reserve something from the server before the first write"* is `sqlx`'s
`BEGIN`. Its MUST is rewritten to the discipline the old signature protected —
a buffering adapter's `begin` resolves at its first poll and never fails — and
two tests hold that where the signature no longer can: `begin_makes_no_round_trip`
(Neon, over a transport that fails every request) and
`begin_resolves_at_its_first_poll_without_a_runtime` (contract crate, polled
once with no executor).

Breaking to every implementer; free because `unstable-projection` carries the
exemption, which is what the gate was kept for. Eight probe impls, one runner
error arm (`ProjectionError::Begin`), 117 call sites, and one constitution
fence whose `compile_fail` would otherwise have started passing for the wrong
reason.

## The far end, built

`LivePostgresProjectionStore` in `happenstance-postgres`, beside the buffered
store: `type Batch` owns a `sqlx::Transaction<'static, Postgres>`, and
`READS_THROUGH_BATCH = true` is a true statement about it. **20 of 20** against
a live PostgreSQL, including an adapter-private test asserting on the
`RuleOutcome` value that `batch_reads_reflect_pending_writes` and
`rebuild_is_chunk_size_invariant` returned `Ran` — the first time either has run
against anything but an in-process map. The buffered store stayed green.

**What the two ends disagreed about: nothing the port had to move for.** The
live end needs an explicit `ROLLBACK` on the regression path, PS-7 is met by the
driver's rollback-on-drop, and a refused statement poisons the transaction and
surfaces through the seam's new `Result`. Each is the far end being harder, not
the port being wrong. **PS-2's MUST is met as written.**

## What it deliberately does not decide

**Lifting the gate.** A semver promise on a published crate; its own record.
What that record must weigh, found here: the typed layer's `Projection::apply`
is synchronous, so an application can push into a buffered batch and cannot
issue a statement into a live one. The live store is an instrument for the
*port's* freeze; whether `apply` moves is the typed layer's axis, not this
port's.

## Alternatives rejected

`rusqlite` at the far end with `begin` kept synchronous (the driver's
`Transaction<'_>` is `!Send`; an owned-connection batch forces `begin` to
acquire on faith); `probe_read_through` alone (ADR-0060 said insufficient,
confirmed); splitting `READS_THROUGH_BATCH` (removes the lie, leaves the hole);
rewording PS-2 to drop the far end (the monoculture its `Rejects` forbids);
replacing the buffered Postgres batch with the live one (the buffered one is the
product; beside, not instead).

## Falsifier

Reopened by an adapter at either end the moved seam still cannot describe, or by
a buffering adapter that the `async` `begin` genuinely cost a round trip. Not
reopened by `apply` staying synchronous — that is the next record's subject.
