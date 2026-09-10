---
id: kb-decision-0025
title: The Ladybug projection adapter — a checkpoint node, raw Cypher, and a blocking driver
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0025
reversibility: medium
phase: 11
supersedes: null
superseded_by: null
summary: >-
  What a projection adapter must do to satisfy PS-1 on an engine with no
  transaction handle type, a blocking driver and Cypher as its only mutation
  surface. The checkpoint is a __hs_checkpoint node in the graph over UINT64,
  written as the last statement before COMMIT: in the graph because
  BEGIN TRANSACTION … COMMIT on one connection is the only atomicity the
  engine offers and PS-1 needs both writes in one, and UINT64 because it
  round-trips u64::MAX - 1 exactly, so SequencePosition's NonZeroU64 needs no
  narrowing — which deletes PositionOutOfRange as unreachable and narrows
  MalformedCheckpoint to a stored zero. The write vocabulary is raw
  parameterised Cypher, because PS-9 says a projection writes through the
  adapter's inherent API and a typed builder binds nobody outside the crate
  and needs a raw() hatch anyway. Blocking-only at 0.2.0, with
  runtime-agnosticism falsified for free by mounting the suite under both the
  blocking and the tokio emitters, and the module doc's stated reason — that
  spawn_blocking is available because the store is 'static — corrected:
  spawn_blocking needs FnOnce + Send + 'static and every port method takes
  &self, so availability is a decision about the store's fields, not its
  call sites. The store is an Arc<Database> and a second handle is a second
  Connection, forced by measurement rather than preference. commit's error
  path issues no ROLLBACK after a STATEMENT error, because the engine aborts
  the whole transaction itself and a rollback afterwards is refused, so
  issuing one masks the first error with a second — and the qualifier is
  load-bearing, because after a decode failure the statement succeeded, the
  engine aborted nothing, and a bare ? returns with the transaction still
  open; that path rolls back best-effort and discards the result. lbug ships
  behind an off-by-default feature, because the driver is a 1.44 GB prebuilt
  static archive plus an unavoidable OpenSSL toolchain and
  cargo test --workspace links. Settled narrowly about other clauses: PS-4's
  Cypher-level condition did not fire, because a transaction gives
  read-your-own-writes within itself — a fact about this adapter on this
  engine, not a discharge of a clause generalising over write-behind shapes;
  PS-4's Rust-level condition is foreclosed by the port for every batch
  shape, since Projection::apply is synchronous and a traversal is I/O,
  which is equally true of the SQLite and Postgres adapters. Ladybug is not
  one of PS-9's or PS-11's data points — that falsifier names a second
  generic consumer owned by a different phase, and an adapter is evidence
  about a clause's cost. Its worth is bounded up front: it is the fifth
  owned-buffered-batch implementer rather than PS-2's second shape, since
  ADR-0060 established that PS-2's live-transaction end is forbidden by the
  port rather than merely unbuilt, and what phase 11 fills is the
  write-vocabulary axis — Cypher rather than SQL, a graph rather than
  tables. The verdict was pre-registered before any body: four capability
  predictions (SECOND_HANDLE supported, RESET_REFUSAL declined,
  COMMIT_FAULT supported, READS_THROUGH_BATCH false) and four named
  it-did-not-hold conditions each citing its clause.
depends_on:
  - kb-decision-0017
  - kb-decision-0030
related:
  - kb-decision-0060
  - kb-decision-0036
  - kb-reference-ladybug-driver-probes-001
  - kb-open-question-reset-refusal-declension-001
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-probe-read-through-signature-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - references/adr/0025-the-ladybug-projection-adapter.md
  - experiments/ladybug-driver-probes/README.md
  - crates/happenstance-ladybug/src/lib.rs
  - crates/happenstance-ladybug/src/projection_store.rs
  - crates/happenstance-ladybug/tests/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
---

# The Ladybug projection adapter — a checkpoint node, raw Cypher, and a blocking driver

## Context

`happenstance-ladybug` had to answer one question phase 11 owned: what must a
projection adapter do to satisfy PS-1 — read-model write and checkpoint write
in one transaction — on an engine that has no transaction handle type to name
in a `Batch` associated type, ships only a blocking driver, and offers Cypher
as its sole mutation surface? This decision is that answer, taken as five
paragraph-sized commitments plus what it does and does not settle about
adjacent clauses.

## Decision

**Checkpoint.** A `__hs_checkpoint` node in the graph, storing the position as
`UINT64`, written as the last statement before `COMMIT`. It lives in the graph
rather than beside it because `BEGIN TRANSACTION … COMMIT` on one connection
is the only atomicity this engine offers at all, and PS-1 needs the read-model
write and the checkpoint write inside that one transaction. `UINT64` is the
type rather than a narrower integer because it round-trips `u64::MAX - 1`
exactly (`kb-reference-ladybug-driver-probes-001`), so `SequencePosition`'s
`NonZeroU64` needs no narrowing conversion on the way in or out — which
deletes `PositionOutOfRange` as unreachable for this adapter and narrows
`MalformedCheckpoint` down to the single case of a stored zero.

**Vocabulary.** Raw, parameterised Cypher, not a typed query builder. PS-9
requires that a projection write through the adapter's own inherent API, and a
typed builder would bind nobody outside this crate while still needing a
`raw()` escape hatch for anything it does not cover — so the builder would be
pure overhead against PS-9's actual requirement.

**Blocking-only at `0.2.0`.** Runtime-agnosticism is checked for free by
mounting the conformance suite under both a blocking and a tokio emitter. The
module doc's own stated justification for why `spawn_blocking` is available —
that the store is `'static` — is wrong and is corrected here:
`spawn_blocking` requires `FnOnce + Send + 'static`, and every port method
takes `&self`, so what actually makes the hop legal is a decision about what
fields the store holds, not a property of where the call happens.

**`Arc<Database>`, second handle is a second `Connection`.** Forced by
measurement, not preference: a second `Database::new` on one directory is
refused by the engine's own file lock, while a second `Connection` over one
shared `Arc<Database>` succeeds and observes the other connection's commits.

**`commit`'s error path issues no `ROLLBACK` after a STATEMENT error.** The
engine aborts the whole transaction itself on a statement error, and issuing a
rollback afterwards is refused by the engine — so issuing one anyway would
mask the real error behind a second, spurious one. The qualifier is
load-bearing: it is not true of every error path through `commit`. After a
*decode* failure — `read_checkpoint` returning `UnreadableRow`,
`MalformedCheckpoint`, or `MalformedAuthority` — the query already succeeded,
the engine aborted nothing, and a bare `?` would return with the transaction
still open. That path rolls back best-effort and discards the rollback's own
result, closing the transaction when it is genuinely open and dropping a
refusal harmlessly when the engine has already closed it.

**`lbug` ships behind an off-by-default feature.** The driver is a 1.44 GB
prebuilt static archive with an unavoidable OpenSSL toolchain dependency
(`kb-reference-ladybug-driver-probes-001`), and `cargo test --workspace`
links every default-enabled dependency.

## What this settles narrowly, and what it does not

**PS-4's Cypher-level condition did not fire.** A transaction gives
read-your-own-writes within itself, so a deferred write set answers it because
replay is one connection, one transaction, in order. That is a fact about this
adapter on this engine — it is not a discharge of a clause stated over
write-behind shapes in general.

**PS-4's Rust-level condition is foreclosed by the port itself**, for every
batch shape: `Projection::apply` is synchronous and a graph traversal is I/O.
That is equally true of the SQLite and Postgres adapters, and is a finding
about the port rather than something this adapter resolved.

**This adapter is not one of PS-9's or PS-11's data points.** That falsifier
names a second *generic* consumer — library code inside `happenstance` itself
writing into an unknown adapter's batch — owned by a different phase. An
adapter is evidence about a clause's cost to implement, not the data point
those two clauses are waiting on.

Its worth is bounded deliberately: `happenstance-ladybug` is the fifth
owned-buffered-batch implementer rather than PS-2's still-missing second
shape — `kb-decision-0060` already established that PS-2's live-transaction
end is forbidden by the port for the drivers it names, not merely unbuilt.
What phase 11 actually fills is a different axis: write vocabulary, Cypher
rather than SQL, a graph rather than tables.

## Consequences

The verdict was pre-registered before any implementation body was written, so
it could not be shaped to fit the outcome: four capability predictions
(`SECOND_HANDLE` supported, `RESET_REFUSAL` declined, `COMMIT_FAULT`
supported, `READS_THROUGH_BATCH` false) and four named "it did not hold"
conditions, each citing the clause it would have falsified. All four held.
