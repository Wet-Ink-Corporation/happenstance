# #485: pin a projection's checkpoint to the store it was built from

## Goal

A typed-layer runner (`run_projection`, `run_fan_out`) records which event store
a projection's checkpoint was built against. When the same projection and
projection store are later resumed against a **different** event store, the run
**stops with `ProjectionError::SourceChanged`**. Today it silently skips that
store's events at or below the old checkpoint and applies again the ones above it
(F3 / R1-4, `experiments/retention-reader/tests/it/r1_runner.rs:160`, PR #482).
Nothing is read and nothing is applied before the refusal. Ships in `0.8.0`, the
last breaking window before 1.0.

## Correction to the brief (load-bearing)

**`EventStore` has no `store_id`.** The async, fallible `store_id` from #342 is
`happenstance_sync::IngestStore::store_id` (`crates/happenstance-sync/src/ingest.rs:163`).
It sits behind each adapter's `sync` feature. `happenstance` depends only on
`happenstance-core` (`crates/happenstance/Cargo.toml`), and the runner binds
`S: EventStore` (`runner.rs:853-862`), so **the runner cannot ask the store for
its identity today.** The experiment says this too (README.md:167: "`run_projection`
binds `EventStore`, which has no `store_id`"). The design question is therefore
also *where the identity comes from* (Q1), not only where it is recorded (Q2).

Inherent accessors that exist today:
- `MemoryEventStore::store_id(&self) -> StoreId`: `const`, at `happenstance-core/src/memory.rs:106`.
- `SqliteEventStore::store_id(&self) -> StoreId`: `const`, at `happenstance-sqlite/src/event_store.rs:631`.
- Postgres and Neon: none public. Each reads a `store_meta` row only inside its
  `IngestStore` impl (`postgres/src/event_store.rs:1252` `IDENTITY_SQL`; `neon/src/event_store/ingest.rs:264`).
- Cloudflare: a private `fn store_id` (`cloudflare/src/event_store.rs:690`).

## Constraints and non-goals

- `ProjectionStore` is frozen (ADR-0063). No port method changes and no new port
  method. The cure uses only `checkpoint`, `begin`, `commit` and `reset` as they are.
- PS-25 is frozen (ADR-0102). Its `name@digest` id, encoding and FNV-1a goldens stay
  byte-identical. **The main checkpoint key does not change**, so no 0.7.x
  checkpoint is orphaned.
- No change to `happenstance-core`'s `EventStore` (Q1 option B is the alternative
  that would need one).
- No new dependency. `happenstance` keeps `happenstance-core` as its only workspace dependency (ARC-02).
- Runtime-agnostic, and no `Send` bound added (constraint 1, ADR-0001). Bind `EventStore` (constraint 4).
- Non-goals: a typed `reset` or rebuild helper; a lease or single-runner guarantee
  (still the caller's, per `runner.rs:813-816`); D3's multi-item scope (separate issue; see Interactions).

## Invariants (each is a test below)

1. A checkpoint committed by a runner reading store A is never resumed by a runner
   reading store B ≠ A. The run returns `SourceChanged`, applies nothing,
   commits nothing, and leaves both the main checkpoint and the pin unchanged.
2. The same store, resumed: behaviour is byte-identical to 0.7, with the same
   `Progressed` and the same main key.
3. The main key is still `checkpoint_id(&p)`, and PS-25's goldens are unchanged.
4. Before any chunk under the main key is committed, the pin is durable. No
   committed row exists whose source is unrecorded, except a 0.7.x leftover (Q3).
5. The mark fits `1..=i64::MAX`, so SQLite and Neon, which **saturate** a position
   above `i64::MAX` (`sqlite/src/projection_store.rs:1100`, `neon/src/projection_store.rs:681`),
   and Postgres, which **refuses** one (`postgres/src/projection_store.rs:431`),
   all round-trip it exactly.
6. One view's `SourceChanged` in a fan-out does not stall the others (PS-29's shape).

## Design

### Recording (Q2, recommended A): a sibling "source pin" checkpoint

The port stores a `Checkpoint { position, authority }` per `ProjectionId` and
nothing else. PS-25's own record already rejected a "digest-in-checkpoint" shape
because it changes the frozen `commit`. So the identity has to ride in a
`(ProjectionId, position)` pair. The design:

- **Pin id**: `{name}#{digest:016x}`. It uses the same name, the same query
  digest, and `#` in place of `@`. It is the same length as the main key, so it
  fits `MAX_PROJECTION_ID_LEN` under the existing 238-byte name cap (ADR-0082 §D5),
  and no new length refusal is needed. Every derived main key ends `@<16 hex>` and
  every pin ends `#<16 hex>`, so the two namespaces are disjoint. `#` is in no
  reserved prefix. The id passes through `ProjectionId::new` (the checking door),
  as `derive` does, so a `sync/` name still refuses. A query change gives a new
  main key **and** a new pin (both follow the digest), so PS-25 and the pin never
  disagree.
- **Pin value**: the `SourceMark` of the store's `StoreId`, committed as the pin's
  position with an **empty batch** (PS-21 permits a position no event occupies)
  and `Authority::Live` (the authority on a pin is not read).
  `SourceMark::of(id) = SequencePosition::new(fnv1a64(&id.to_bytes()) >> 1).unwrap_or(SequencePosition::FIRST)`.
  It is 63 bits, so it is at most `i64::MAX` (invariant 5), and non-zero. It reuses
  `checkpoint.rs`'s existing `fnv1a64`. No `unwrap`, and no `as`. The odds of two
  different stores colliding are about 2^-63, the same argument `checkpoint.rs`
  makes for the query digest.
- **Why a fingerprint and not the 128 bits**: a position holds 63 usable bits on
  every SQL adapter. Three pin rows per view, read and written non-atomically,
  would buy an exact `recorded` id in an error message, and nothing else. The
  operator can still identify the recorded store: `SourceMark::of(candidate) == recorded`.

**Resume decision.** It is shared by both runners, in a new `pub(crate)` function
`checkpoint::bind_source`. Read the pin first, then the main key:

| main | pin | action |
|---|---|---|
| any | `Live`/`Rebuilding { through: m }`, `m == mark(found)` | resume as today |
| any | `m != mark(found)` | `Err(SourceChanged)`. Nothing read, nothing written |
| `NeverRun` | `NeverRun` | **fresh**: commit the pin (empty batch), then run from the start |
| `Live`/`Rebuilding` | `NeverRun` | **0.7.x leftover**: Q3 (recommended: adopt, i.e. commit the pin, then resume) |

The pin is written **eagerly, before the stream is opened**, whenever it reads
`NeverRun`. The rejected alternative was lazy (just before the first chunk's
`begin`). Lazy saves one `begin`+`commit` on a fresh idle run, once per view
lifetime. It costs a second code path in `fan_out::drive`, and it lets a fresh view
pointed at the wrong store bind to whichever store first has an event. Eager is one
path, in the derivation step both runners already share, and it satisfies
invariant 4 trivially. The trade is stated so the reviewer can contest it.

**Ordering.** pin commit → stream → chunk commits. A crash after the pin and
before the first chunk leaves `(NeverRun, pin)`. Retrying against the same store
resumes. Against another store it refuses, which is fail-safe. The reverse order
would leave `(Live, NeverRun)`, which Q3's adoption would then bind to whatever
store the retry used. That is the bug.

**Remedy for a refusal** (documented on the variant and in `docs/`):
`models.reset(models.begin().await?, &source_checkpoint_id(&p)?)`, and also reset
the main key with the read model's deletes if the view must be rebuilt for the new
store. Under Q3-A, a pin reset to `NeverRun` over a `Live` main key is
**re-adoption on the next run**. So the same reset, alone, is the remedy after a
restore that brought the event store and the projection store back together and
then re-minted the identity (VT-6). No forged position and no new helper are needed.

### Source identity (Q1, recommended A): the caller supplies it, in a `Source`

New module `crates/happenstance/src/source.rs`. It is re-exported from `lib.rs`.

```rust
/// The event store a run reads, and the identity its checkpoint is pinned to.
#[derive(Debug)]
pub struct Source<'a, S> {          // no bounds on the struct (API-21)
    events: &'a S,
    id: StoreId,
}
impl<'a, S> Source<'a, S> {
    pub const fn new(events: &'a S, id: StoreId) -> Self;
    pub const fn events(&self) -> &'a S;
    pub const fn id(&self) -> StoreId;
}
// Manual Clone + Copy without `S: Clone` (a derive would add the bound; RS-12).
impl<S> Clone for Source<'_, S> { fn clone(&self) -> Self { *self } }
impl<S> Copy for Source<'_, S> {}

#[cfg(feature = "memory")]
impl<'a> From<&'a MemoryEventStore> for Source<'a, MemoryEventStore> {
    fn from(events: &'a MemoryEventStore) -> Self { Self::new(events, events.store_id()) }
}
```

Ownership: `&'a S` is borrowed because the run reads it and does not keep it
(OWN-01). `StoreId` is `Copy` and taken by value. `Source` is `Copy`, so it is
passed by value. It pairs the store with its identity at one call site. That
pairing does not make a wrong id unrepresentable: a caller can still write
`Source::new(&b, a_id)`. The `From<&MemoryEventStore>` impl is the one door where
the id cannot be wrong, and the docs say to read the id from the store being passed
(`events.store_id()` on SQLite and memory, the Q4 accessors elsewhere), never to
keep it in configuration. **Q1 option B** is the shape that makes the pairing
correct by construction. Its cost is a core port change.

### Signatures (breaking, typed layer only)

```rust
pub async fn run_projection<S, P, C>(
    source: Source<'_, S>,            // was: events: &S
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: Chunk,
) -> Result<Progressed, ProjectionError<S::Error, StoreError<P>, P::Error>>
where S: EventStore, P: Projection, C: Codec;

pub async fn run_fan_out<S, P, C>(
    source: Source<'_, S>,            // was: events: &S
    models: &P::Store,
    views: &mut [P],
    codec: &C,
    chunk: Chunk,
) -> FanOut<S::Error, StoreError<P>, P::Error>
where S: EventStore, P: Projection, C: Codec;
```

It stays at five parameters (API-10). The item names are unchanged, so
`spec-trace`'s anchors on `run_projection` (the fence comment at `runner.rs:830-852`)
still resolve. Keep the declaration where it is. The `Send`-flavour spawn bounds
are unchanged, because `Source` holds `&S` exactly as `events: &S` did. Re-run
`tests/flavours.rs` to prove it.

New public helpers in `checkpoint.rs`, beside `checkpoint_id`:

```rust
/// The id the runner pins this projection's source store under.
pub fn source_checkpoint_id<P: Projection>(projection: &P) -> Result<ProjectionId, InvalidCheckpointId>;

/// A 63-bit fingerprint of a `StoreId`, as the runner records it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceMark(SequencePosition);           // private field (API-02)
impl SourceMark {
    pub fn of(id: StoreId) -> Self;                 // total, never panics
    pub const fn get(self) -> u64;
}
impl fmt::Display for SourceMark { /* 16 lowercase hex, as the digest */ }
```

`SourceMark` is not constructible from a raw `u64` (minimal surface, API-14). The
one way to get one is from a `StoreId`, or from an error. Internally,
`pub(crate) fn derive_pair(name, &Query) -> Result<(ProjectionId, ProjectionId), InvalidCheckpointId>`
replaces both runners' `checkpoint::derive` calls, so the main key and the pin are
derived in one place and cannot diverge.

### Errors (ERR-01..04)

On `ProjectionError<R, W, A>` (already `#[non_exhaustive]`):

```rust
/// The checkpoint was built from a different event store than this run reads.
///
/// Nothing was read, applied or committed...  (remedy text, see Design)
#[error("checkpoint {pin} records source {recorded}, but this run reads store {found}")]
SourceChanged {
    /// The pin id, the one to reset to re-point the view.
    pin: ProjectionId,
    /// The mark the pin holds.
    recorded: SourceMark,
    /// The store this run was handed.
    found: StoreId,
},

/// Recording the source store's mark failed.
///
/// Nothing was read or applied. The pin is unchanged.
#[error("recording the checkpoint's source store failed")]
RecordSource(#[source] CommitError<W>),
```

- The pin **read** reuses `Checkpoint(W)`, because it is the same operation on the
  same port. The pin's `begin` failure reuses `Begin { progress: Progressed::default(), source }`,
  whose doc gains one sentence.
- `progress()` returns `Progressed::default()` for both new arms. `position()`
  returns `None`.
- `#[source]`, not `#[from]` (ERR-04). `ProjectionId` is owned in the error,
  because the error outlives the borrow. That clone has an ownership reason (H-08).
- `ViewError::Stopped(ProjectionError<Infallible, W, A>)` carries both new arms
  per view. No change to `ViewError` or `FanOut`.

### Placement (ARC-02, ARC-05)

All logic stays in `crates/happenstance`:
- `checkpoint.rs` gets the pin id, `SourceMark` and `bind_source`. The file is at
  329 lines. If it passes ~450, move the pin into `checkpoint/source.rs`
  (a `checkpoint/` directory already exists for its tests).
- `source.rs` gets `Source`.
- `runner.rs` gets the signature, the new variants, and one `bind_source(...).await?`
  call replacing lines 874-879.
- `fan_out.rs` gets the signature, and `derive`/`resume` (lines 393-420) calling
  `derive_pair` + `bind_source`.

Dependencies still point inward. Q4's accessors are additive inherent methods on
three adapters, behind no feature.

### `bind_source` (one function both runners call)

```rust
pub(crate) async fn bind_source<St: ProjectionStore>(
    models: &St, key: &ProjectionId, pin: &ProjectionId, found: StoreId,
) -> Result<Checkpoint, Bound<St::Error>>
```

It returns the main `Checkpoint` to resume from. `Bound<E>` is a private enum
(`Read(E) | Changed{..} | Begin(E) | Record(CommitError<E>)`). Each runner maps it
into its own `ProjectionError` arms. That keeps `bind_source` free of `R`/`A`. It
holds no borrow across an await beyond `&St` and the two `&ProjectionId`s, so
fan-out's "no `&P` across an await" property (`fan_out.rs:303-304`) holds. Pin
values are read through `considered_through` (`runner.rs:1174`), and an unknown
`#[non_exhaustive]` state is `None`, i.e. treated as absent.

### Concurrency and resources

There are no new tasks, locks or buffers. The cost is one extra `checkpoint` read
per run per view. There is one extra `begin`+`commit` the first time a view runs
under 0.8 (fresh or adopted). PS-38 (read from the primary) already covers the pin
read. Two concurrent runners on one view are still the caller's to prevent. If they
race the first pin, the loser's commit gets `CheckpointRegression` or succeeds with
an equal mark, so `RecordSource` surfaces it.

### Alternatives considered (the record behind Q1 and Q2)

- **Fold `StoreId` into the main key (Q2-B).** A store switch reads `NeverRun` and
  replays from the start **into rows that were never reset**. That is double
  application, silently, so the bug moves rather than closes. It changes every
  PS-25 golden (a frozen 1.0 promise) and orphans every deployed 0.7.x checkpoint.
  On upgrade, every view replays into its existing rows. It is rejected by the new
  rule (W3 below).
- **A projection-implemented hook** (the projection writes and reads its own source
  row through its batch). The runner cannot read rows generically, so every
  `Projection` impl would owe two new methods. That is a burden on every user for a
  runner concern.
- **Bind `happenstance_sync::IngestStore` in the runner.** That adds a new
  dependency edge from the typed layer into replication, and it requires every
  source to be a store that can "forge history" (`ingest.rs` doc: only a peer runner
  should hold one).
- **Keep `run_projection` and add `run_projection_pinned`.** The default entry
  would keep the bug, and the owner approved the break.

## Spec and ADR: yes, one new ADR and one new clause

- PS-25's text is **not** edited, and its id and goldens are untouched. Q2-A needs
  no PS-25 amendment. Q2-B would need an ADR superseding ADR-0102's PS-25 promise,
  which is one more reason to reject it.
- **Mint PS-40** in §4.7, after PS-25, by a new ADR (next number: **ADR-0109**;
  the last is `0108-phase-14s-clauses-are-frozen.md`). The precedents for minting a
  clause are ADR-0030 (PS-38) and ADR-0082 (PS-39). ADR-0102 shows the freeze shape.
  Draft:
  > **PS-40 — A runner MUST NOT resume a checkpoint against an event store other
  > than the one its positions came from. It MUST record the source store's
  > identity with the checkpoint, before the first commit under it, and MUST
  > refuse a run whose source differs, applying nothing.**
  > The record is a second derived id, `name#digest`, whose position is a 63-bit
  > mark of the `StoreId`. The port is unchanged (ADR-0063) and PS-25's id is
  > unchanged. **Rule:** `a_checkpoint_is_not_resumed_against_another_store`.
  > **Cases:** E2E-59. **Rejects:** W1–W5 below. It names ES-29 / SY-19 (positions
  > are store-local) as the reason.
- The marker is Q5.
- The §7.2 trace row is
  `| PS-40 | PROVISIONAL | `a_checkpoint_is_not_resumed_against_another_store` † | E2E-59 |`.
  The dagger is the typed-layer-rule convention PS-25 uses. Regenerate §7.1's count.
- Add `spec/E2E-CASES.md` **E2E-59 — A projection resumed against another store**
  (after E2E-58).
- Write the `.kb/decisions/0109-…md` atom by hand from `.kb/_intake/`, add it to
  `.kb/maps/`, and add a `runbook/ledgers.md` row. `cargo xtask spec-trace` and
  `lint-kb` must stay green.
- No conformance (`happenstance-testkit`) rule. The obligation is the typed
  runner's, and no `ProjectionStore` adapter can fail it. CF-36 forbids naming an
  adapter rule for it, as PS-25's rule is a typed-layer test. The one adapter-facing
  fact, that the mark round-trips, rests on invariant 5. It is proved by a property
  test plus test 14's real SQLite run.

## The wrong implementations the new rule rejects

"A rule that no adapter can fail is decorative." Each wrong implementation below is
written as a **private mutant** in the test file, as a `fn` that drives the
pre-change resume logic, or is reached through the public API as noted. Each is
asserted to fail the rule.
- **W1, today's runner**: no pin. R1-4 exactly: B's own events skipped, one applied twice.
- **W2, pin written, never compared**: it checks presence only, so it resumes on B.
- **W3, StoreId folded into the main key (Q2-B)**: it returns `Ok` with every B
  event applied into A's rows. The rule asserts `Err(SourceChanged)` **and** that the
  rows are untouched.
- **W4, unbounded mark**: `fnv1a64(..)` without `>> 1`. Rejected by the property
  test (`mark.get() <= i64::MAX as u64`, using `i64::MAX.unsigned_abs()`, with no
  `as`), and by test 14 on SQLite, where two marks above `i64::MAX` saturate to one.
- **W5, pin written after the first chunk commit**: rejected by test 9, where the
  first chunk's commit fails. The pin must already be durable.

## Test list (drive in this order; TST-01)

Unit (`crates/happenstance/src/checkpoint/tests.rs`):
1. [x] `source_pin_id_is_name_hash_digest`: golden. `"van_stock#dc525f3f5d262b19"`
   for the projection whose main key is `van_stock@dc525f3f5d262b19`
   (`tests/projection_runner.rs:1115`). It shares the digest and differs only at the separator.
2. [x] `source_pin_and_main_key_are_never_equal`: a property over names and queries;
   no pin equals any main key.
3. [x] `source_pin_refuses_what_the_main_key_refuses`: a 239-byte name gives
   `NameTooLong`, and `sync/x` gives `Refused`.
4. [x] `source_mark_fits_a_signed_sixty_four_bit_column`: a property over random
   `[u8;16]`: `1 <= get() <= i64::MAX`. (W4)
5. [x] `source_mark_goldens`: three pinned `StoreId → mark` values. They are a
   promise like PS-25's, and changing the function strands every pin.

Behaviour (new `crates/happenstance/tests/source_pin.rs`, memory store unless noted):
6. [x] `a_first_run_records_its_source`: after a run,
   `models.checkpoint(&source_checkpoint_id(&p))` is `Live { through: mark(a) }`.
7. [x] `the_same_source_resumes_unchanged`: run, append, run. The `Progressed`
   matches the 0.7 behaviour, and so do the main key and checkpoint (invariant 2).
8. [x] **PS-40 rule** `a_checkpoint_is_not_resumed_against_another_store`
   (in `tests/projection_runner.rs`, next to `changed_query_starts_a_new_checkpoint`).
   R1-4's setup: A with 8 events, run; B with 4 own events plus 5 of A's. The run on
   B gives `Err(SourceChanged { pin, recorded: mark(a), found: b })`. It asserts
   `applied == 0`, the main checkpoint is unchanged, the pin is unchanged, and the
   rows are unchanged. Mutants W1–W3 fail it.
9. [x] `the_pin_is_durable_before_the_first_chunk`: a test-local `ProjectionStore`
   wrapper refuses `commit` under the main key. The run gives `Commit`, and the pin
   reads `Live { mark(a) }`. (W5)
10. [x] `an_idle_fresh_run_still_pins` (records the eager choice): an empty A gives
    `Ok`, `applied == 0`, and the pin is recorded.
11. [x] `a_0_7_checkpoint_is_adopted_on_first_run` (Q3-A): commit the main key only
    (the 0.7 shape). The run on A resumes past it, applies nothing twice, and the pin
    becomes `mark(a)`. *If Q3-B is chosen, it becomes
    `a_0_7_checkpoint_is_refused_until_adopted` → `SourceUnrecorded`.*
12. [x] `resetting_the_pin_re_adopts`: refused on B, reset the pin, and the run on B
    adopts. This is the documented remedy, executed.
13. [~] `a_failing_pin_read_is_a_checkpoint_error` (blocked, below) and
    `a_refused_pin_commit_is_record_source`: wrapper store, with
    `progress() == default`.

Fan-out (`crates/happenstance/tests/fan_out/rules.rs`):
14. [x] `a_source_switch_stops_only_that_view`: two views, one built on A and one
    fresh. Run on B: view 1 gives `Stopped(SourceChanged)`, and view 2 runs and pins B
    (invariant 6).
15. [x] `fan_out_pins_every_view_it_runs`.

Real database (`examples/transfers-on-sqlite/tests/runs.rs`, the only place both
installed crates compile together):
16. [x] `a_view_refuses_another_sqlite_file`: two `SqliteEventStore` files and one
    `SqliteProjectionStore`. Run on file 1, drop every handle, reopen, and run on
    file 2 gives `SourceChanged`. This proves the mark survives a real `INTEGER`
    column (W4 on a real adapter).

Compile and flavour:
17. [x] `tests/flavours.rs`: the spawned generic `run_projection`/`run_fan_out`
    still compile with the same bounds, through `Source`.
18. [x] Doctests: `checkpoint_id`'s adoption example (`checkpoint.rs:165-234`)
    gains the pin step, or a note that adoption also needs the pin absent. The new
    items get compiled examples (RS-70). `source_checkpoint_id` gets the remedy
    snippet.

Then the mechanical migration of every call site, done in the same commit as the
signature change so the tree stays green (counts measured with `grep`):
`tests/projection_runner.rs` (25), `fan_out/runs.rs` (14), `failure_policy.rs` (9),
`flavours.rs` (7), `projection_clauses.rs` (3), `convergence.rs` (2),
`projection_runner_page.rs`, `fan_out/rules.rs`, `src/{runner,fan_out,policy,checkpoint,lib}.rs`
doctests, `crates/happenstance/README.md` (doctested via `lib.rs:10`),
`happenstance-postgres/tests/live_projection.rs` (8),
`happenstance-sync-testkit/src/convergence_suite.rs:176` (published; consumers pin
it exactly, so a minor bump), `benchmarks/benches/projection_runner.rs` (6),
`examples/{transfers-on-sqlite,rebuilding-read-models,tickets-over-http,two-devices-one-seat}`.
With memory stores, `(&events).into()` is the one-token change. Update
`tests/contract_surface.rs`/`doc_surface.rs` if they enumerate exports. Not one
existing expectation changes. If one must change (for example a test counting
checkpoint rows), stop: that is H-12, and you need to ask.

## Docs (PROC-11)

- `docs/choose-a-topology.md:36-40`, "Positions belong to one store": add that the
  typed runner now enforces this, refusing a view resumed against another store
  (PS-40).
- `docs/rebuild-a-read-model.md`: the reset section (`:94-125`, the code at
  `:141-217`) gets a paragraph and step. When re-pointing a view at another store,
  reset `source_checkpoint_id` too. A reset of the main key alone, against the same
  store, needs nothing more. Its `run_projection` calls move to `Source`.
- **New `docs/upgrading-to-0-8.md`**, following `upgrading-to-0-6.md`'s shape:
  the signature change, the `(&events).into()` / `Source::new(&events, id)` recipe
  per adapter, and the first-run adoption (Q3). The warning in that page: run the
  first 0.8 run against the store each view was built from. Link it from
  `docs/README.md` and the site's routing if `cargo xtask site` requires it.
- `docs/choose-a-store.md`, `docs/upgrading-to-0-5.md`: their `run_projection`
  examples. Leave the 0-5 page historical if it is a frozen upgrade note. Check
  whether `xtask` compiles it (it is not in `narrative.rs`'s list).
- `crates/happenstance/README.md`, the crate root `lib.rs` prose (`:109`), and the
  `run_projection` doc's "one runner per `(store, checkpoint_id)`" paragraph
  (`runner.rs:813-826`): now `(store, checkpoint_id, source)`.

## CHANGELOG (`[Unreleased]`, to become `[0.8.0]`)

- **Breaking (`happenstance`)**: `run_projection` and `run_fan_out` take a
  `Source<'_, S>` in place of `&S`. The checkpoint is pinned to its source store,
  and a run against another store stops with `ProjectionError::SourceChanged`
  (PS-40, ADR-0109, #485). New: `Source`, `SourceMark`, `source_checkpoint_id`,
  `ProjectionError::{SourceChanged, RecordSource}`. A 0.7 checkpoint is adopted on
  its first 0.8 run. Read `docs/upgrading-to-0-8.md`.
- `happenstance-sync-testkit`: follows the signature, so it is a minor bump.
- If Q4 is accepted: **Added**, `PostgresEventStore::store_id`,
  `NeonEventStore::store_id`, `CloudflareEventStore::store_id` (additive).

## Interactions

- **D3 (additive multi-item scope, separate issue).** The pin is derived from the
  same `derive_query` → digest path as the main key, through the one `derive_pair`
  function, so a multi-item query pins under `name#<its digest>` with no extra work.
  D3 must keep `derive_pair` as the single derivation. If D3 adds a second query
  path, it must call `derive_pair`, not `derive`. The two land independently in
  either order. Whichever lands second rebases a few lines in `fan_out::derive`.
- **Identity re-mint (VT-6).** After `remint_identity`, every typed view over that
  store refuses. That is correct when the event store was restored alone, because
  its positions rewound. When the projection store was restored with it, the remedy
  is the pin reset (test 12). The re-mint docs on the three adapters gain one
  sentence pointing at it.
- **Sync.** A device's views over its own store are unaffected until a re-mint. The
  convergence suite builds fresh models per log (`convergence_suite.rs:174`), so it
  is unaffected beyond the signature.
- **Experiment R1-4** is Q6.

## Decisions (H-16)

Labelled `485-Q*` so they cannot be confused with the D1–D5 of the other sitting.

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| 485-Q1 | Where the runner gets the source store's identity | **A** the caller passes it in a `Source { &S, StoreId }` (typed layer only; a wrong id is possible but the doors are documented; `From<&MemoryEventStore>` is exact). **B** add `store_id` to core `EventStore` (the runner asks the store it reads, correct by construction; but it changes the contract crate and every adapter, needs an ES ADR, and runs against ES-39's verdict that the retention harms do not belong on `EventStore`; a provided method defaulting to "unknown" would make the cure silently opt-out). **C** bind `happenstance_sync::IngestStore` (new dependency edge, and it hands the runner the ability to forge history) | **A**: it is the least breaking cure that closes the bug, and the experiment located the seam in the typed layer, not `EventStore` | owner | |
| 485-Q2 | How the identity is recorded, given the frozen port | **A** a sibling pin checkpoint `name#digest`, position = a 63-bit mark, refuse on mismatch. **B** fold `StoreId` into the main key (a store switch becomes a silent replay into un-reset rows; breaks PS-25's frozen goldens; every 0.7 view replays on upgrade). **C** a projection-implemented source row (two new methods on every `Projection`) | **A** | owner | |
| 485-Q3 | A 0.7.x checkpoint (main present, pin absent) on first 0.8 run | **A** adopt: pin the store this run reads, then resume (trust on first use; wrong only if the store is switched in the same deploy as the upgrade; also makes "reset the pin" the one remedy for every refusal). **B** refuse with a new `SourceUnrecorded` until the operator adopts each view (loud, but every deployment's every view stops on upgrade, and adoption needs a forged position or a new helper). **C** rebuild (impossible generically, because the runner cannot clear rows) | **A**, with the upgrade page's warning | owner | |
| 485-Q4 | Public identity accessors on adapters that lack one outside `sync` | **A** add inherent `store_id()` to `PostgresEventStore` (async, `PostgresEventStoreError`), `NeonEventStore` (async), `CloudflareEventStore` (sync; make the private fn public), with `IngestStore::store_id` delegating, as additive minors in the same release (separate PRs). **B** document "enable `sync` and call `IngestStore::store_id`" | **A**: without it, a Postgres user must pull in replication to run a view | owner | |
| 485-Q5 | PS-40's marker at minting | **A** `[PROVISIONAL]` now, frozen in the 1.0 freeze pass with the other freeze-by-21 rows. **B** `[FROZEN]` now by ADR-0109 (the mark function and `#` separator become a promise immediately) | **A**: it is minted and frozen in one breath only if the owner wants it settled before 1.0's pass. The goldens (test 5) bind the encoding either way | owner | |
| 485-Q6 | The experiment's R1-4 test, which no longer compiles against HEAD (path dependencies, `support.rs:57-63`) | **A** update `support.rs::project` to pass a `Source` and invert R1-4 to assert the refusal, keeping its doc's "recorded hazard" history plus a "closed by #485" line; note it in README F3. This rewrites a test's expectation, so it is an H-12 decision. **B** leave it, since experiments are not in the gate (but they then rot uncompiled) | **A** | owner | |

Decided in this plan without a human (contestable at review): eager pin write;
`#` separator; FNV-1a `>> 1` as the mark; `SourceChanged` carries the pin id;
the pin's read and begin errors reuse the `Checkpoint`/`Begin` arms; no typed
reset helper.

## Risks

- **An existing test asserts "an idle run touches nothing", or counts checkpoint
  rows.** The eager pin breaks it. The grep found none (`a_buffered_run_is_ready_at_its_first_poll_without_a_runtime`
  only needs memory to stay `Ready`, and an empty commit is). If one appears, it is
  H-12: ask, and do not edit it.
- **A caller passes the wrong `StoreId`** (Q1-A). Docs and the memory `From` reduce
  the risk, but a type cannot remove it. Q1-B is the cure if this proves common.
  Record it as PS-40's reopening condition.
- **The mark collides** (2^-63 per pair): accepted, as for PS-25's digest. It is
  stated in the clause.
- **`spec-trace` anchors** on `run_projection`/`run_fan_out` shift if the
  declarations move. Keep them in place, and run `cargo xtask spec-trace` after the
  signature change.
- **`cargo semver-checks` against `0.7.0`** will report the break. That is expected
  for `0.8.0` and must be the only break it reports for `happenstance`.
- **The `docs/` site build** may need the new upgrading page registered. Run
  `cargo xtask site --no-api` locally if zola is present.

## Size (lower bound)

About 30 files and about 1,500 lines changed:
- `happenstance/src`: `checkpoint.rs` +170, `source.rs` +100, `runner.rs` +70,
  `fan_out.rs` +40, `lib.rs` +6, `checkpoint/tests.rs` +90.
- New `tests/source_pin.rs`: ~350.
- Rule and fan-out tests: +120.
- Mechanical call-site migration: ~80 sites, ±200.
- Sync-testkit, examples and bench: ±50.
- Q4 adapters: +120.
- Docs: new upgrading page ~90, edits ~60.
- Spec PS-40, E2E-59 and the trace row: ~70.
- ADR-0109 atom ~110, plus the map, ledger and CHANGELOG ~40.

## Progress

- [x] Explored. The brief's `EventStore::store_id` premise is corrected above.
- [x] Owner answers 485-Q1..Q6: all **A** (relayed by the delegating session; the
  owner was asked to confirm). Q4 lands as its own commit on this branch.
- [x] ADR-0109 (`status: proposed`) + PS-40 `[PROVISIONAL]`, E2E-59, §7.1/§7.2.
- [x] Tests 1–5. **Red:** compile error E0432, unresolved imports
  `super::SourceMark` and `super::derive_pair` in `checkpoint/tests.rs`. Green
  after `checkpoint/pin.rs` and `derive_pair`.
- [x] Signature change + call-site migration (one commit, tree green).
- [x] Tests 6–16. Where they live moved: the behaviour tests are
  `tests/fan_out/pins.rs` (a `tests/` binary that already had the
  `RecordingProjectionStore` harness), and test 16 is in
  `examples/rebuilding-read-models/tests/rebuild_through_reset.rs`, because
  `transfers-on-sqlite` is binary-only. **Red, by mutant** (each reverted):
  - W2 (pin read, never compared): the PS-40 rule fails legs A, B, C, E (the run
    on B returned `Ok`, applied 8, main checkpoint moved to 9);
    `resetting_the_pin_re_adopts` and `a_source_switch_stops_only_that_view` fail.
  - W1/W5 (no pin written): nine `pins.rs` tests, the two `runs.rs` tests and the
    rule test fail.
  - W4 (mark without `>> 1`): every `rebuild_through_reset` test fails, including
    `a_view_refuses_another_sqlite_file`, because SQLite saturates the mark and a
    store then refuses itself.
  - W3 is a runner written in the rule's own test file
    (`FoldedIntoTheKey`), asserted to fail legs A, B, E.
- [~] Test 13's pin-read-failure half (review F3) is **blocked on an H-16
  decision.** A local fake store wrapping the recording store was written and
  passed, with two `bind_source` mutants as red evidence (the pin read's error
  folded into `NeverRun` failed the pin-read and fan-out cases; the main read's
  failed the main-read case). But `crates/happenstance/tests/doc_surface.rs`'s
  `no_local_projection_store` forbids any `impl ProjectionStore for` in this
  crate (HS-P0010's AC-012), and relaxing it is H-12. The remaining routes are
  an additive knob on the published testkit's `RecordingProjectionStore` (a
  public API change) or the fake in a crate outside `happenstance`. The draft
  is kept outside the tree for whichever is chosen.
- [x] **H-12, flagged for the owner:** two existing expectations in
  `tests/fan_out/runs.rs` count calls on the recording store and now include the
  eager pin's `begin`/`commit` (`a_commit_for_one_view_moves_no_other_views_checkpoint`,
  `a_read_failure_interrupts_every_running_view`). The Postgres live test
  `a_commit_fault_takes_the_projections_rows_with_it` runs an idle run before
  arming its fault, because the trigger fires on every checkpoint insert.
- [x] **H-12, flagged for the owner, after review (F1):**
  `a_failing_begin_or_commit_stops_only_its_view` used `failing_begin(1)`, which
  the eager pins turned into view b's *pin* begin, duplicating
  `a_refused_pin_begin_is_begin_with_no_progress` and leaving no chunk-begin
  failure covered anywhere. Retargeted to `failing_begin(7)` at `chunk(1)`: b's
  second chunk begin, after one committed chunk, asserting `Begin` carries
  `(Some(first), 1, 0)` progress, b's checkpoint at the first event, and c's
  second row. Red: with the old index 1 it fails on `(None, 0, 0)`. The single
  runner had no chunk-begin test either:
  `pins.rs::a_refused_chunk_begin_is_begin_with_the_committed_progress` adds it
  (`failing_begin(2)`).
- [x] Review F2, F4, F5: the bench and experiment are rustfmt'd and compile
  (`cargo check --benches`; the experiment's tests run green, 23 passed);
  `a_view_refuses_another_sqlite_file`'s doc and PS-40's Rule say W4 is pinned
  deterministically by `source_mark_fits_a_signed_sixty_four_bit_column` and the
  SQLite test shows only the round trip; §7.6's "three of the four" sentence
  names the five single-claim cases.
- [x] Docs (upgrading-to-0-8, rebuild, choose-a-store, choose-a-topology),
  CHANGELOG + `0.8.0` trace, ledger rows, decision map, Q6 experiment edit.
- [x] Gate: `cargo xtask ci --fast` steps all green (the run was split by an
  environment OOM and a linker bus error on a full disk, then each remaining step
  run on its own); temper gate GREEN at `bb637e679c1cf072`. Commit A.
- [x] Q4 accessors, commit B. Inherent `store_id` on `PostgresEventStore` and
  `CloudflareEventStore` in new child modules (`event_store/identity.rs`) so the
  cited lines of each `event_store.rs` do not move; Neon's lives in
  `event_store/remint.rs`, beside `remint_identity`, with `identity_statement`
  and `identity_from` moved out of the `sync`-gated `ingest.rs`. Each
  `IngestStore::store_id` delegates. Cloudflare's private reader is renamed
  `incarnation` in place, so its existing wasm tests now call the public
  method. **Not test-first:** the reads moved rather than changed, so the
  tests were written after: Neon's two unit tests (no feature), and one live
  Postgres test, `#[ignore]`d with the file's standing reason.
- Also changed, beyond the plan: `xtask/src/spec_trace.rs`'s
  `the_real_documents_orphan_no_case` counts 59 cases, not 58, because E2E-59
  was added; it is a census of the document, as §7.6's sentence is.
