# ADR-0062 — The probe seam moves, and the far end of PS-2's axis is built

- **Status:** proposed
- **Date:** 2026-09-10
- **Phase:** 12, after the `0.2.0` release
- **Acts on:** [ADR-0060](../../.kb/decisions/0060-ps-2s-axis-re-evaluated.md) §3, which named this change and declined to make it
- **Amends:** PS-2's `Rule` and PS-6's MUST in `spec/SPECIFICATION.md` §4; PS-6's falsifier is recorded as fired
- **Evidence:** `crates/happenstance-core/tests/probe_live_transaction_shape.rs`; `crates/happenstance-postgres/src/live_projection_store.rs`; `crates/happenstance-postgres/tests/live_projection.rs`, 20 of 20 against a live PostgreSQL

## The question

ADR-0060 found that PS-2's bar — the suite green against adapters at **both ends**
of the batch-shape axis — was not merely unmet but *unobservable*: a store whose
batch is a live transaction could implement the port only by declaring
`READS_THROUGH_BATCH = false`, a false statement about itself, so the suite
reported the same capability profile for both ends. It named the signature change
that would fix it and declined to make it, on three grounds: it is breaking to a
trait consumers implement; it is PS-2's owner's call rather than an adapter
lane's; and `probe_read_through` alone is not sufficient, because `probe_write`
staying synchronous still forces buffering.

**This is the owner's call. Does the seam move, and what does it cost?**

## What the compiler said, in the order it said it

The change was made first and the record written from what it produced, because
the alternative — a record asserting what a compiler would say — is the mistake
the skeleton made for a whole phase.

**Moving `probe_read_through` alone is not sufficient, as ADR-0060 said.** The
honest body for a live-transaction store is `batch.select(key).await`, and
`select` takes `&mut self` because the driver borrows the connection. With
`probe_write` still `fn(&self, &mut Batch, &str, u64)` — synchronous — the store
can put a write into the batch only by inserting into a map the batch owns, which
is the buffered shape. So the whole seam moved: `probe_write`, `probe_delete_all`
and `probe_read_through` all take `&mut Self::Batch` and return
`impl Future<Output = Result<…, Self::Error>>`, spelled by hand rather than as
`async fn` for the reason `probe_read` already gives — this trait is not under
`trait_variant`, `async fn` here fires `async_fn_in_trait` under `-D warnings`,
and the desugaring puts the *absence* of a `Send` bound where a reader can see
it.

**Moving the probe seam alone is not sufficient either, and this is the finding
ADR-0060 did not have.** `fn begin(&self) -> Self::Batch` is total, synchronous
and infallible, and `sqlx`'s only route to a `Transaction` is
`pool.begin().await?`. With the probe seam moved and `begin` unmoved, a
live-transaction store can *report* itself and still cannot *exist* for the
driver PS-2 names. The `sqlx` refutation from phase 10b was never a property of
the axis; it was a property of `begin`'s signature. So `begin` moved too:
`async fn begin(&self) -> Result<Self::Batch, Self::Error>`, under
`trait_variant` like the port's other four async members.

**PS-6's falsifier had already fired, and nobody had said so.** The clause read
*"`begin` MUST be neither `async` nor fallible"* and carried the falsifier *"an
adapter that must reserve something from the server before the first write, such
as a batch identifier or an advisory lock that cannot be taken at commit"*.
`BEGIN` on a pooled connection is exactly such a reservation. Phase 10b found it
and reported it as a result about PS-2; it was equally a result about PS-6, and
this record is where that is written down. PS-6's MUST is rewritten to the
discipline the old signature was protecting — *an adapter with nothing to
reserve MUST NOT spend a round trip in `begin`; a buffering adapter's `begin`
MUST resolve at its first poll and MUST NOT fail* — because the argument the old
text made, that `async fn begin() -> Result` *implies* a round trip, is false:
an `async fn` whose body never awaits is a future ready at its first poll and
costs what it looks like. What the signature can no longer enforce, two tests
now do: `begin_makes_no_round_trip` in `happenstance-neon` drives `begin` over a
transport that fails every request and receives `Ok`, and
`begin_resolves_at_its_first_poll_without_a_runtime` in the contract crate's
tests polls it once with a no-op waker and no executor.

**The rest of the ripple, counted.** Eight `ProjectionProbe` implementations
across five crates and one example moved with the seam; every buffering body
gained `Ok(())` and lost nothing else. The typed runner gained
`ProjectionError::Begin { progress, source }`. One hundred and seventeen
`begin()` call sites gained an `.await`. The constitution's
`21-send-is-not-inherited.md` needed both fences updated, and it is worth
recording why: its `compile_fail` fence would otherwise have gone on passing —
`E0308` on a future passed where a batch was expected, instead of the `Send`
diagnostic it exists to pin — which is precisely the vacuity RS-01-2 names, and
the fence was re-compiled standalone to confirm it still fails for *"future
cannot be sent between threads safely"*.

## The far end, built and run

`LivePostgresProjectionStore` binds `type Batch` to a struct owning a
`sqlx::Transaction<'static, Postgres>` and a stamp — the binding the phase-2
skeleton declared with five `todo!()` bodies and phase 10b could not discharge.
It declares `READS_THROUGH_BATCH = true`, and that is a true statement: a
`SELECT` through the open transaction sees the transaction's own uncommitted
writes, and a second backend session does not.

Against a live PostgreSQL in a container: **20 of 20** in
`crates/happenstance-postgres/tests/live_projection.rs` — all seventeen rules,
plus three adapter-private tests. One asserts on the `RuleOutcome` **value** that
`batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant`
returned `Ran`, where every adapter over a real database had returned a reported
skip. One is the control that the pending write lives on the server and only in
this session — visible through the batch, invisible to a second handle, gone
after `ROLLBACK` — so the read-through rule is not being passed by a shadow map.
One arms a `CHECK` constraint and shows a statement refused mid-batch surfacing
through `probe_write`'s `Result`, the transaction poisoned, `commit` refusing
with `CommitError::Store`, both halves unchanged, and the store usable afterwards.

The buffered `PostgresProjectionStore` stayed as it was and stayed green — 21
of 21 on the moved seam — and it stays the store an application uses.

## What the two ends disagreed about

Nothing the port had to change for, and three things worth writing down because
they are what the far end is *for*:

1. **The regression path.** A buffered store drops its `Vec` when the guarded
   upsert refuses; the live store's statements are already on the server, so it
   issues an explicit `ROLLBACK` and reports a failure of *that* rather than
   hiding it behind the regression. `rollback` staying a port method beside
   `Drop` is what this needed.
2. **PS-7.** Dropping a batch that holds a pooled connection must roll the
   transaction back and return the connection. `sqlx` does this on drop, and
   `dropped_batch_leaves_store_usable` is the rule that checked it rather than
   assumed it.
3. **Mid-batch failure.** A refused statement poisons a Postgres transaction —
   every later statement answers *"current transaction is aborted"* — and the
   seam's `Result` is where a store says so. The old infallible seam had nowhere
   to put this.

Each is the live end being harder than the buffered end. None is the port being
wrong. That is the measurement PS-2 asked for, and it came back clean.

## Decision

1. The probe seam moves, all three members. `begin` moves with it. Both are
   breaking to every implementer and exempt from semver because they sit behind
   `unstable-projection`, which is the exemption ADR-0036 kept for exactly this.
2. PS-6's MUST is rewritten to the discipline; its falsifier is recorded as
   fired; it keeps `[PROVISIONAL]` against an adapter that must reserve
   something *and* cannot afford the round trip, which nothing names.
3. PS-2's `Rule` is amended a second time to say the first end is occupied. Its
   MUST, its maturity and its `Cases` are untouched, and its MUST is now met.
4. `LivePostgresProjectionStore` ships in `happenstance-postgres` beside the
   buffered store, documented as an instrument for the port's freeze rather than
   a product, with the reason stated on it.

## What this deliberately does not decide

**Lifting `unstable-projection`.** PS-2's bar being met is the *precondition*
PS-3 names for the gate coming off; it is not the gate coming off. That is a
semver promise on a published crate — a published version can be yanked but not
withdrawn — and it gets its own record. What that record must weigh, found here
and not before: the typed layer's `Projection::apply` is synchronous, because
buffering a row does not await. An application can therefore push into a
buffered batch and **cannot issue a statement into a live one**, so the live
store is unusable by `run_projection` for a projection that writes rows. The
port is now proved at both ends; the typed layer is not, and whether `apply`
moves is the axis after this one. A freeze of the port that left `apply` where
it is would be a defensible promise — `Projection` is `happenstance`'s, not
`happenstance-core`'s — but it is a decision, and it is not this one.

## Alternatives rejected

- **Keep `begin` synchronous and build the far end on `rusqlite`.** Refuted by
  a property of the driver this ADR does not touch: `Transaction<'_>` is
  `!Send` because `Connection` is `Send` and not `Sync`, so the batch costs the
  `SendProjectionStore` impl. An owned-`Connection` batch with a hand-issued
  `BEGIN` is `Send` but forces `begin` to acquire a connection synchronously and
  infallibly — a pre-opened pool handed out on faith — which is a contortion
  built to satisfy the signature rather than the axis.
- **Move `probe_read_through` alone.** ADR-0060 said it was insufficient and
  writing the body confirmed it: `probe_write` synchronous forces the buffer.
- **Split `READS_THROUGH_BATCH` into "can" and "can be asked synchronously".**
  Removes the lie without opening the axis. With the seam moved there is no lie
  to remove.
- **Reword PS-2 to drop the far end.** Freezes the monoculture PS-2's own
  `Rejects` clause forbids.
- **Replace `PostgresProjectionStore`'s batch with the live one.** The buffered
  store is published, conformant, and the one an application can drive from a
  synchronous `apply`. Beside, not instead.

## Falsifier

This record is reopened by an adapter at either end that the moved seam still
cannot describe — a batch the probe's `&mut` borrow cannot issue a statement
into, or a `begin` that has no failure to report and is charged one anyway by a
consumer that cannot poll — or by a fifth buffering adapter finding that the
`async` `begin` cost it a round trip it could not avoid. Not reopened by the
typed layer's `apply` staying synchronous: that is the next record's subject,
named here so it is not read as this one's omission.
