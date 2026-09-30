---
id: kb-decision-0074
title: Projection::apply is async, is handed a Delivered event, and fails with the projection's own error
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0074
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Settles the apply record ADR-0063 left owed and phase 17 was given. The typed layer's
  Projection::apply becomes async, on the one trait, declared with
  #[trait_variant::make(SendProjection: Send)], and is handed a Delivered<Self::Event> and the
  store's batch, through which it may await statements. Delivered is #[non_exhaustive], exposes
  id() -> EventId and event()/into_event(), has a public constructor so an application can
  unit-test its own apply, and has no accessor for the local position. The rejected alternative, a
  synchronous Projection beside an additive LiveProjection trait and a second runner, doubles the
  names an application meets, and PS-30's fan-out runner would double with it. The evidence is
  experiments/apply-shape: the trait compiled under trait_variant 0.1.3 with a provided on_error,
  and drove a memory buffer, a SQLite statement buffer and a live sqlx transaction against
  PostgreSQL 17.10, spawned from code generic over the projection, and type-checked for wasm32
  with a !Send store. Return-type notation is E0658 on 1.97.1, so the trait_variant pair stays.
  SY-21 is reworded to the arrival position, SequencedEvent::position, because EventId::position()
  legitimately returns the origin's. The failure-policy seam's shape is fixed: a provided on_error, defaulting to Halt,
  handed an ApplyFailure covering decode and apply failures and the batch, so a skip record is
  written by application code through the concrete store's inherent API. Over a live batch a
  server-refused statement aborts the transaction, so Skip after it needs a savepoint, and phase 18
  owes that. PS-9 and PS-11 are frozen on the spike's three executed legs. PS-28 is met at phase
  18 by type Error and ProjectionError<R, W, A>. PS-25 takes the derived-id remedy with a
  hand-written 64-bit FNV-1a digest and an exposed checkpoint_id. A generic spawner must name the
  store as its own type parameter, including for the shipped run_projection. ADR-0063's falsifier
  did not fire. Nothing published changes in 0.4.0.
depends_on:
  - kb-decision-0063
  - kb-decision-0066
  - kb-decision-0072
related:
  - kb-decision-0007
  - kb-decision-0008
  - kb-decision-0017
  - kb-decision-0019
  - kb-decision-0028
  - kb-decision-0062
  - kb-decision-0070
  - kb-decision-0075
  - kb-open-question-apply-synchronous-live-store-001
source_paths:
  - experiments/apply-shape/README.md
  - experiments/apply-shape/src/shape.rs
  - experiments/apply-shape/tests/postgres.rs
  - experiments/provided-method-spike/README.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance-postgres/src/live_projection_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# Projection::apply is async, is handed a Delivered event, and fails with the projection's own error

## The question

`happenstance::Projection::apply` is synchronous (`crates/happenstance/src/runner.rs:95-99`). A
projection can push a row into a buffered batch, but it cannot await a statement into a batch
that is a live transaction. `LivePostgresProjectionStore` is published, and its own module
documentation says `run_projection` cannot drive it for a projection that writes rows
(`crates/happenstance-postgres/src/live_projection_store.rs:38-47`). Phase 18's exit criterion
asks for exactly that write, against a real database. ADR-0063 froze the port and left `apply`
to a record of its own. The phase-16 note on
`kb-open-question-apply-synchronous-live-store-001` narrowed the question to sync versus async,
because the batch-handle parameter had already landed.

The record also has to carry the clauses whose surface is `apply`'s arguments: SY-21, PS-9 and
PS-11 with the failure-policy seam's shape (PS-27), PS-28's error type, and PS-25's remedy.

Nothing here is published. `Projection` and `run_projection` sit behind `happenstance`'s
`unstable-projection`, which makes no semver promise (ADR-0066). Phase 18 builds this record and
lifts the gate.

## Decision

1. **`apply` is `async`, on the one trait.** Phase 18 declares:

   ```rust
   #[trait_variant::make(SendProjection: Send)]
   pub trait Projection {
       type Event: DomainEvent;
       type Store: ProjectionStore;
       type Error: core::error::Error
           + From<<Self::Store as ProjectionStore>::Error>
           + 'static;
       fn id(&self) -> &ProjectionId;
       fn scope(&self) -> &Tags;
       async fn apply(
           &mut self,
           event: Delivered<Self::Event>,
           batch: &mut <Self::Store as ProjectionStore>::Batch,
       ) -> Result<(), Self::Error>;
       fn on_error(
           &mut self,
           failure: &ApplyFailure<'_, Self::Error>,
           batch: &mut <Self::Store as ProjectionStore>::Batch,
       ) -> impl Future<Output = Result<Policy, Self::Error>> { /* Halt */ }
   }
   ```

   The store's types are spelled as explicit projections, as the spike spelled them
   (`experiments/apply-shape/src/shape.rs:187`, `:202`, `:225`). The runner's aliases
   `StoreError<P>` and `StoreBatch<P>` go through `<P as Projection>`, so inside the trait
   `trait_variant`'s generated `SendProjection` would need `Self: Projection`, which holds only
   through the blanket impl and so needs `Self: Sized`. That is six `E0277`s on 1.97.1 with
   `trait_variant` 0.1.3, and the explicit spelling compiles clean. Signatures outside the trait
   keep the aliases.

   An application implements exactly one of `Projection` and `SendProjection`, as adapters do for
   the ports (RS-20-4). A buffered `apply` never awaits, so its future is ready at the first poll.
   The spike held that at the runner level, not only for `apply` alone
   (`experiments/apply-shape/tests/memory.rs:151`). That is PS-6's finding, carried up one layer.

2. **The `Send` mechanism is `trait_variant`, not return-type notation.**
   `P: Projection<apply(..): Send>` is `error[E0658]: return type notation is experimental` on
   1.97.1 (`experiments/apply-shape/results/rtn-probe.txt`). So the one-trait-plus-RTN option does
   not exist at this MSRV. `#[async_trait]` stays forbidden (CLAUDE.md constraint 1).

3. **`apply` is handed a `Delivered<E>`.** It is `#[non_exhaustive]`, with private fields, and
   exposes:
   - `id() -> EventId`;
   - `event() -> &E` and `into_event() -> E`;
   - optionally `recorded_at()`, phase 18's choice. If it is added, its rustdoc repeats VT-9: it is
     not an ordering key. **It is decided before the gate lifts.** Once a public two-argument
     `Delivered::new(id, event)` ships, a `recorded_at` added after 1.0 must be `Option`-typed or
     arrive with a builder or a second constructor, and `new` keeps its two arguments.

   It has **no accessor for the local `SequencedEvent::position`**. Asking for one is `E0599`
   (`experiments/apply-shape/results/refusals.txt`). `Delivered::new(id, event)` is **public**
   (README finding F7). An application unit-testing its own `apply` must be able to build the
   argument, and the constructor takes an `EventId`, so a local position still has nowhere to go.
   An envelope beats a second positional `EventId` parameter, because the next fact added later,
   metadata or `recorded_at`, would be one more break. `SequencedEvent::new`'s own rustdoc records
   that churn once already.

4. **SY-21 names the arrival position.** `delivered.id().position()` compiles and returns a
   `SequencePosition`: the **origin's**, which SY-23 relies on. The spike ran a restored log whose
   local positions are 1, 2, 3 and whose origin positions are 30, 10, 20, and `apply` saw the
   origin identities in local order (`experiments/apply-shape/tests/memory.rs:243`, finding F4). So
   the MUST NOT is true of the arrival position, `SequencedEvent::position`, and false of "a
   `SequencePosition`". The two coincide in value for an event this store authored, and that does
   not break convergence, because the value is reached through `EventId`, identical on every peer,
   never through the arrival field. SY-21's text is reworded in the specification in this change. It stays `[PROVISIONAL]` and `freeze-by-18`:
   phase 18 builds the signature and a typed-layer test, and freezes it there.

5. **The failure-policy seam's shape (PS-27).** `on_error` is a **provided** method. It defaults
   to `Policy::Halt`, so a projection that says nothing keeps today's behaviour. It is handed:
   - an `ApplyFailure`, `#[non_exhaustive]`, with a `Decode` and an `Apply` arm. Both are needed
     because the motivating skip, Kestrel Motor's crypto-shred, is a *decode* failure. Each arm
     carries the `EventId` and no local position;
   - the batch.

   A skip record is therefore written by **application code, through the concrete store's inherent
   API, into the batch that advances the checkpoint**. No library code writes into a batch. Two
   spellings are fixed by findings:
   - The default is spelled `-> impl Future<Output = ..> { .. async { .. } }`, never `async fn`
     with a body, because `trait_variant` 0.1.3 copies the block verbatim.
   - Its parameters are **named, not `_`-prefixed**, and discarded with `let _ = x;` (finding F2).
     The generated blanket impl forwards parameters by name, so an `_failure` is a used underscore
     binding, and pedantic clippy fails the gate on it.

   An override may be written as a plain `async fn` (finding F3). The default's future captures
   neither `&self` nor the batch, which is ADR-0028's rule for provided bodies. PS-27 stays
   `[DEFERRED]` and `freeze-by-18`. Phase 18 builds the seam and `skip_and_record_is_atomic` with a
   mutant. The executed test `no_skip_and_record_path_is_offered`
   (`crates/happenstance/tests/projection_clauses.rs`) goes red the day the runner grows
   `on_error`, and phase 18 moves PS-27's verdict in the same change.

6. **`Skip` over a live batch is only as good as the transaction (finding F5).** An application
   refusal raised before any statement leaves the transaction healthy, and the skip record
   commits with the checkpoint. A **server** refusal aborts the transaction. The skip write is then
   refused too, and the run stops with the adapter's error, with nothing written
   (`a_server_side_failure_leaves_nothing_for_on_error_to_write_into`, stated before it was run).
   `Policy::Skip` is therefore **not** universal, and the record says so rather than letting the
   variant's name promise it. A runner-issued `SAVEPOINT` around each `apply` was the one
   candidate second generic consumer the spike surfaced, and F5 left open who issues it. This record
   **declines it by design**: library code writing into an unknown adapter's batch is what PS-9
   forbids, and `Skip` is documented as not universal instead. That is a choice, not a
   measurement: the application-issued savepoint it leaves in place has not been run. **Phase 18
   owes three things:**
   - `on_error`'s rustdoc states the limit;
   - a live-batch test in which `apply` wraps its statement in a savepoint, a server refusal is
     rolled back to it, and the skip record commits with the checkpoint. That test is what
     confirms PS-9's freeze on this leg;
   - a session-log line recording whether `LivePostgresBatch` gains an inherent savepoint helper
     (additive on a published crate) or the pattern stays the application's.

7. **PS-9 and PS-11 are frozen.** The ledger asked for them to fall with the seam's shape, and the
   research brief set the bar for a freeze at compiled evidence rather than argument: the provided
   `on_error` compiled under `trait_variant`, and a skip row was written through a buffered batch
   and a live one with no bound on `Batch`. Each leg was run:
   - **The provided `on_error` compiled** under `trait_variant` 0.1.3 on the first attempt
     (`experiments/apply-shape/src/shape.rs:223-231`, `results/clippy.txt`). It was overridden by a
     `Send` implementor and by a `!Send` one (`src/edge.rs`).
   - **Buffered: `SqliteBatch`.** `on_error` pushed a skip row with `SqliteBatch::push`. The run
     applied 3 and skipped 2, and a fresh connection read back the rows, both skip records and the
     checkpoint at the head
     (`tests/sqlite.rs:83`, `a_buffered_sql_projection_applies_and_skips_through_the_spawned_runner`,
     `results/test-default.txt`). `MemoryProjectionBatch::write` did the same (`tests/memory.rs:212`).
   - **Live: `LivePostgresBatch`.** `on_error` awaited `LivePostgresBatch::execute` inside the live
     transaction against PostgreSQL 17.10. Both skip records committed, and the checkpoint read back
     through a fresh store is `Live` at the head (`tests/postgres.rs:113-130`, `:203-219`,
     `results/test-postgres.txt`).
   - **No bound on `Batch`.** The runner names the batch only as `StoreBatch<P>`
     (`experiments/apply-shape/src/runner.rs:34`), and no port signature moved (C10).
     The generic spawner's `St: SendProjectionStore<Batch: Send>` is `Send` for holding the batch
     across an await. That is PS-36's existing obligation, not a write vocabulary.

   The second generic consumer PS-9's marker feared, a generic dead-letter recorder, is the
   failure seam itself, and it writes nothing generically. The runner-issued savepoint of
   decision 6 was a second candidate, declined by design rather than measured away; the unrun
   application-savepoint test is what would confirm the freeze on that leg. **Restated falsifier:** a bound on
   `Batch` that carries a write vocabulary, in any crate this workspace publishes. PS-11 falls with
   PS-9, by its own marker. Phase 18 building PS-27's seam without a generic write is now
   confirmation of a frozen clause, not the discovery of its shape.

8. **PS-28 is met at phase 18, and this record commits the shape.** PS-28 is `[FROZEN]` and was
   unmet by the typed runner at 0.3.x. `apply` returns `StoreError<Self>`, and
   `ProjectionError::Apply` carries the store's error (`runner.rs:130`, *"two type parameters, not
   three"*), so an application refusal has to be forged into the adapter's `#[non_exhaustive]`
   error, which is PS-28's own *Rejects*. Phase 18 builds four things:
   - `type Error` on `Projection`, bounded `From<<Self::Store as ProjectionStore>::Error>` so `batch.execute(..).await?`
     needs no `map_err` (finding F8);
   - `apply` and `on_error` returning `Self::Error`;
   - `ProjectionError<R, W, A>` with `Apply { progress, position, source: A, rollback }`, the
     shipped arm's fields with `source` retyped, because PS-28's first MUST is to report the
     position it failed at, and **no default `A = W`**, which would re-bless the forged shape;
   - an arm for a failing `on_error`, the spike's `Policy { progress, position, source: A,
     rollback }` (`experiments/apply-shape/src/runner.rs:107-119`, `:317-320`), so the
     server-refusal case of decision 6 stops with the adapter's error at a named position rather
     than inside `Apply`. Its name is phase 18's.

   Three parameters on every downstream signature is the cost PS-28's *Rejects* already accepted.
   Whether an alias like the spike's `RunErrorFor<S, P>` ships is phase 18's choice.

9. **PS-25 takes the derived-id remedy.** The digest-in-checkpoint alternative changes the frozen
   `commit`, so it is refused here. The runner derives the `ProjectionId` it commits under from
   the projection's name and a digest of the `Query` it derives. The typed layer already holds
   that query (`derive_query(P::Event::EVENT_TYPES, projection.scope())`, `runner.rs:531`).
   - **The digest.** A hand-written 64-bit FNV-1a, rendered as 16 lowercase hex digits, over the
     query's items: each item encoded canonically, the encodings sorted and deduplicated, then
     hashed. The per-item encoding must be **injective**: length-prefixed fields with a marker
     between the type section and the tag section, or a `Cc` separator, which VT-14 keeps out of
     every `EventType` and `Tag`. Plain concatenation collides (types `["ab", "c"]` against
     `["a", "bc"]`, or a type `x` against a tag `x`), and without deduplication `[A, A]` and `[A]`
     match the same events and digest differently. Within an item the input is already
     canonical: `QueryItem::new` sorts and deduplicates types, and tags are canonical by VT-16.
     Golden values pin the encoding but do not show it is injective, so phase 18's rustdoc states
     the argument. That is the
     order SPECIFICATION.md's PS-25 prose requires, because VT-31 declines to canonicalise item
     order. It adds zero dependencies. `sha2` was the alternative, and it would add a crate for
     `cargo deny` to pass, to buy collision resistance a checkpoint key does not need against an
     adversary. **The function and its encoding become a 1.0 promise**, because changing either
     forces a rebuild in every deployment. Phase 18 pins them with golden values.
   - **The derived id is exposed**, as `happenstance::checkpoint_id(&projection)` or a name phase
     18 chooses. Otherwise code that reads `checkpoint(projection.id())` silently reads `NeverRun`.
     `tickets-over-http` and `rebuilding-read-models`' `print_checkpoints` both do that today. Its
     format must pass whatever `ProjectionId` validation phase 17's `projection-id-is-unvalidated`
     record adopts, and must not fall under SY-31's reserved `sync/` prefix.
   - **It is a behaviour change for runner users.** A checkpoint written under a bare id at 0.3.x
     reads `NeverRun` after the upgrade, which forces a rebuild. The **adoption procedure**, for a
     deployment that knows its query did not change: `begin()`, then
     `commit(empty_batch, derived_id, through, authority)` with the old checkpoint's `through` and
     authority. PS-21 permits a commit naming a position its batch did not write
     (`commit_accepts_a_position_the_batch_did_not_write`). Adoption is explicit, where a digest
     check would refuse silently, and PS-25's falsifier, a narrowed query forced into an
     unneeded rebuild, is served by that act through the existing port. When phase 18 ships the
     derived id, the release **needs a `CHANGELOG.md` entry** stating the rebuild and the adoption
     procedure.

   PS-25 stays `[PROVISIONAL]` and `freeze-by-18`.

10. **A generic spawner names the store as its own type parameter (finding F1).** It writes
    `P: SendProjection<Store = St>` with `St` a fresh type parameter, or every store future is
    `!Send`. The cause is the associated type's item bound, `type Store: ProjectionStore`. The
    solver prefers it to the blanket `impl<T: SendProjectionStore> ProjectionStore for T`, so
    `P::Store`'s methods resolve to opaque bare-flavour futures. `trait_variant` is not the cause.
    **The shipped `run_projection` has the same limit**: spawned generically with `P::Store` it
    fails with six `E0277`s, and with `Store = St` it runs
    (`experiments/apply-shape/tests/shipped_spawn.rs`,
    `results/spawn-shipped-store-as-projection.txt`). `type Store: SendProjectionStore` on the
    `Send` flavour alone cannot be written, because `trait_variant` copies bounds verbatim. **Phase
    18's documentation owes it:** the runner's *what spawning it costs* rustdoc states the bounds a
    generic spawner writes (`experiments/apply-shape/tests/common/mod.rs:22-25`), `Store = St`
    first, or the first generic spawner written from the docs will not compile.

11. **A panicking `apply`, if PS-30 applies.** If phase 18 builds the fan-out runner,
    `panicking_apply_rolls_back` over an async `apply` needs `catch_unwind` around **each poll**,
    not around the call. The runner avoids `futures-util`, so that is a hand-written `poll_fn`
    wrapper. The spike did not build it.

## ADR-0063's falsifier did not fire

ADR-0063 reopens if *"`Projection::apply` moving to a shape that requires the port to move with
it"*. It did not. The port already hands out `&mut Self::Batch`, and the live batch's statement
path is a published inherent `async fn` (`LivePostgresBatch::execute`,
`crates/happenstance-postgres/src/live_projection_store.rs:114`), used as it is. The spike
changed no file under `crates/` (criterion C10).

## Rejected

- **A synchronous `Projection` beside an additive `LiveProjection` trait and a second runner.** It
  meets phase 18's exit criterion too, and it lost on names. `trait_variant` doubles the names the
  *library* exports, and an application still implements one trait. Two traits double the names an
  *application* meets: two traits to choose between, two runners, two sets of bounds. PS-30's
  fan-out runner would double with them. The runner's *"Why `apply` is synchronous"* rustdoc
  (`runner.rs:50-59`) argued that an async `apply` doubles the surface every application
  implements. That is true of the exported surface and false of what an application writes, and
  phase 18 rewrites the paragraph rather than leaving it to contradict this record.
- **Keeping `apply` synchronous and doing the awaiting in `commit`.** A live batch *is* the
  transaction, so staging rows for `commit` to issue rebuilds the buffered store over a live one.
- **Return-type notation on one trait.** `E0658` on 1.97.1.
- **Digest-in-checkpoint for PS-25.** It changes the frozen `commit` and every adapter's checkpoint
  schema.
- **A default `A = W` on `ProjectionError`.** It re-blesses the forged error.

## What phase 18 builds

The trait above, `Delivered`, `ApplyFailure`, `Policy` and `SendProjection`. Every `impl
Projection` in the tree is rewritten, and the research brief counts twelve plus the `runner.rs`
doctest. Also: `on_error` and the savepoint work in decision 6, `ProjectionError<R, W, A>`, the
derived id with `checkpoint_id`, the adoption procedure and its `CHANGELOG.md` entry, F1's
documentation, and `catch_unwind` per poll if PS-30 is built.
`crates/happenstance-postgres/src/live_projection_store.rs:38-47`'s *cannot be driven* paragraph is
removed when it stops being true. `runbook/phases/18-typed-runner.md` carries the list.

## Left open, deliberately

- **Codec tag resolution in the runner.** The spike called `decode` with the codec in hand,
  because the shipped `decode_event` is crate-private. The events there carry no framing.
- **Whether the runner's gate comes off in 1.0.0 or the name stays declared and empty.** That is
  phase 18's session-log line, as its work list already says.
- **Where SY-20 and SY-22's convergence declaration attaches.** It attaches to this trait, and its
  spelling is phase 18's and 13's.

## Falsifier

This record is wrong if either of the following happens:
- Phase 18 cannot build the trait above against both batch shapes without moving a
  `ProjectionStore` signature. That also reopens ADR-0063.
- A `SendProjection` cannot be spawned from generic code by any caller-side bounds once `Store =
  St` is written, so that a second trait is needed after all.
