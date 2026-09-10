# ADR-0025 is written and wants an atom

**Date:** 2026-09-08
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Long form:** `references/adr/0025-the-ladybug-projection-adapter.md`
**Evidence:** `experiments/ladybug-driver-probes/`

## Why this is a brief and not the atom

`.kb/decisions/` atoms are authored by `/redkiln:kb-ingest` from staged
documents. Hand-writing one produces the directory layout of the process without
the process, which is why the first attempt at that in this repository was
reverted. The long-form record is written and committed; this is the intake
document the atom should be authored from.

**The number is 0025, taken from `RUNBOOK.md`'s reservation** — the ADR queue has
held 0025–0028 open for phases 11, 13, 13 and 14 since the plan was written, and
the two directories together hold 0001–0024 and 0029–0059. This session initially
read the listing as "0059 is the highest, so 0060 is next" and was wrong; a gap in
a sequence is invisible to a listing and obvious to a diff.

## The one question

What must a projection adapter do to satisfy PS-1 on an engine with no
transaction handle type, a blocking driver and Cypher as its only mutation
surface — and which half of PS-4's falsifier can actually fire against it?

## The decision, in one paragraph each

- **Checkpoint**: a `__hs_checkpoint` node in the graph, over `UINT64`, written as
  the last statement before `COMMIT`. In the graph because a `BEGIN
  TRANSACTION` … `COMMIT` on one connection is the only atomicity the engine
  offers and PS-1 needs both writes in one. `UINT64` because it round-trips
  `u64::MAX - 1` exactly, so `SequencePosition`'s `NonZeroU64` needs no narrowing
  — which deletes `PositionOutOfRange` as unreachable and narrows
  `MalformedCheckpoint` to a stored zero.
- **Vocabulary**: raw parameterised Cypher. PS-9 says a projection writes through
  the adapter's inherent API, so a typed builder binds nobody outside the crate
  and needs a `raw()` hatch anyway.
- **Blocking-only** at `0.2.0`, with runtime-agnosticism falsified for free by
  mounting the suite under both the blocking and the tokio emitters. The module
  doc's stated reason — that `spawn_blocking` is "available here" because the
  store is `'static` — is **wrong** and is corrected: `spawn_blocking` needs
  `FnOnce + Send + 'static` and every port method takes `&self`, so availability
  is a decision about the store's fields, not its call sites.
- **`Arc<Database>`, and a second handle is a second `Connection`** — forced by
  measurement: a second `Database::new` on one directory is refused by a file
  lock.
- **`commit`'s error path issues no `ROLLBACK` after a STATEMENT error** — the
  engine aborts the whole transaction itself, and a rollback afterwards is
  refused, so issuing one masks the first error with a second.

  **The qualifier is load-bearing and the original wording lacked it.** §7's
  probe measured statement errors, and the finding is true of them. It is not
  true of a failure in which the statement *succeeded* and the decode did not:
  `read_checkpoint` returns `UnreadableRow`, `MalformedCheckpoint` and
  `MalformedAuthority` with the query already answered, so the engine has aborted
  nothing and a bare `?` inside `commit_inner`'s transaction returned with that
  transaction still open. Fixed at the `0.2.0` pass by rolling back
  **best-effort** on that path and discarding the result — which closes the
  transaction when it is open and, when the engine has already aborted, drops the
  refusal so the first error still reaches the caller. The atom minted from this
  brief should carry the distinction rather than §7's unqualified form, because
  the unqualified form reads as licence to `?` out of an open transaction.
- **`lbug` ships behind an off-by-default feature** — the driver is a 1.44 GB
  prebuilt static archive plus an unavoidable OpenSSL toolchain, and `cargo test
  --workspace` links.

## What it settles about other clauses, stated narrowly

**PS-4's Cypher-level condition did not fire** — a transaction gives
read-your-own-writes within itself, so a deferred write set answers it because
replay is one connection, one transaction, in order. That is a fact about **this
adapter on this engine**, not a discharge of a clause generalising over
write-behind shapes.

**PS-4's Rust-level condition is foreclosed by the port** for every batch shape,
because `Projection::apply` is synchronous and a traversal is I/O. That is a
finding about the port and is equally true of the SQLite and Postgres adapters.

**Ladybug is not one of PS-9/PS-11's data points.** PS-9's own falsifier names
*a second generic consumer* — library code `happenstance` itself ships that must
write into an unknown adapter's batch — owned by a different phase. An adapter is
evidence about a clause's **cost**, not the data point the clause waits on.

The crate root **has since been corrected** and the atom must be written in the
past tense: `crates/happenstance-ladybug/src/lib.rs:151` now reads *"This crate is
not one of PS-9's or PS-11's data points, which this paragraph used to claim."*
Nothing is owed here — do not mint an atom asking for an edit that has landed.

## The pre-registered verdict

Committed before any body, so it cannot be written to fit the outcome: four
capability predictions (`SECOND_HANDLE` supported, `RESET_REFUSAL` declined,
`COMMIT_FAULT` supported, `READS_THROUGH_BATCH` false) and four named "it did not
hold" conditions each citing its clause.

Its worth is bounded up front. Ladybug is the **fifth** owned-buffered-batch
implementer rather than PS-2's second shape, and phase 10b established that PS-2's
other end is not merely unbuilt but forbidden by the port for both drivers the
clause names. What phase 11 fills is the **write-vocabulary** axis — Cypher rather
than SQL, a graph rather than tables.

That sentence **has since been corrected too**, and its anchor has moved:
`RUNBOOK.md:4917` now reads *"Fills the write-vocabulary axis"* and records that
it had said *"fills the batch-shape axis"* and that this is not what it filled.
The brief's original citation of `RUNBOOK.md:4802` is stale in both line and
tense. Again: past tense, and nothing owed.
