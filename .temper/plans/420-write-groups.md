# #420 — write groups: a push and a pull never split what one write wrote

Branch `claude/p13-420` off `origin/phase-13`. Architect's plan; no production code written.
Owner decision on record (Ryan, 2026-11-08, "option A"): each adapter records a write-group
id per row through an additive schema migration, exposed by an additive trait in
`happenstance-sync`; `happenstance_core::EventStore` and every published read type stay as
they are. The migration and the trait are treated as approved. Everything else that needs a
human is in §Decisions.

## Goal

A runner push and a `StoreSyncPeer` pull carry the local log as the **write groups it was
written in**, one `EventGroup` per group, each whole, with an ingested group's origin guard
back on it. A multi-event append is never torn across two pushes or two pulls; independent
appends are never merged into one atomic unit; a foreign group relayed peer-to-peer (SY-10)
keeps its boundary and its `guard` (SY-30, SY-6 evidence).

## Constraints and non-goals

- `happenstance-core` does not change at all: not `EventStore`, not `SequencedEvent`, not
  `MemoryEventStore` (VT-10's letter; ADR-0001/0003 untouched).
- No `#[async_trait]`. The new trait is defined once, bare, and `trait_variant::make`
  derives the `Send` flavour (CLAUDE.md constraint 1, `standards/rust/20-two-flavour-ports.md`).
- **No wire change.** `EventGroup`, `PushBatch`, `Pulled`, `wire::FORMAT_VERSION` are
  untouched; a group id is store-local and never travels. This keeps clear of
  `claude/p13-incarnation` (ADR-0104, wire v3), which edits `peer.rs`, `wire.rs`,
  `serve.rs` and `store_peer.rs`. This plan does **not** touch `peer.rs` or `wire.rs`; it
  touches `serve.rs` in one `where` clause and one `use`, and `store_peer.rs` in the
  `pull` body, two bounds, its module docs and the hidden half of one doctest.
- New items go **below** every line the specification cites (`lib.rs` up to line 134,
  `ingest.rs` up to line 230, `peer.rs` not at all). New modules are declared at the end of
  `lib.rs`, after `pub mod runner;`, as the crate's own comments require.
- Non-goals: SY-6's `after` refusal at ingest (its rule is still †, phase 13 builds it);
  backoff / Split / Park handling (#244 part 2); an adjudication hook (#136, SY-7);
  persisting the condition a **local** append was made under (see D-A2); counting
  `Waiting::Events` on a pull.
- Container limits: implementer builds one crate at a time,
  `CARGO_TARGET_DIR=/home/user/tgt/420 cargo test -j2 -p <crate> --all-features`, never
  workspace-wide until the final gate.

## Invariants (each is a test below)

- I-1 Every row belongs to exactly one write group. One `append` call is one group. One
  `IngestGroup` that lands at least one foreign event is one group, **its compensation
  included** (SY-2). A fully re-delivered `IngestGroup` writes nothing and so no group (SY-11).
- I-2 A group's id is **the lowest local position among the rows it actually wrote**
  (`WriteGroup::first`). Not the first position *allocated*: Postgres and Neon burn a
  position on a skipped row, and an id no row holds is not covered by ES-10 (see Design §3).
- I-3 `read_groups` never returns part of a group.
- I-4 Within one page, and across pages chained with `after = last.first()`, group ids are
  strictly increasing and every group appears exactly once.
- I-5 No group with `first <= after` can appear in a later read. (Inherited from ES-10: a
  group commits in one transaction, and its `first` row was observed.)
- I-6 After a push is acknowledged, **every local row at a position `<= confirmed` has been
  pushed**, where `confirmed` is the `first` of the last pushed group.
- I-7 Each pushed `EventGroup` is exactly one local write group (no merge, no split), carrying
  that group's guard.
- I-8 An ingested group's guard reads back equal (`AppendCondition: Eq`) after a round trip
  through the store.
- I-9 Rows written before the migration read as **one group per row** (legacy rule).

## Design

### 1. Placement (ARC-02, ARC-05)

- `happenstance-sync/src/write_group.rs` (new, declared last in `lib.rs`): the trait, its
  two value types, and a child module `write_group/guard.rs` with the stored-guard codec.
  This is the port crate's job exactly as `IngestStore` is: coherence lets each adapter
  implement a trait this crate defines, so the contract crate never hears about it
  (`ingest.rs` module docs make the same argument). Dependencies still point inward: adapters
  → `happenstance-sync` (optional, `sync` feature) → `happenstance-core`.
- Re-export at the root: `pub use write_group::{GroupPage, SendWriteGroupStore, WriteGroup,
  WriteGroupStore};`. The codec stays module-qualified (`happenstance_sync::write_group::
  encode_guard`), as `wire`'s items are, because only adapters call it.
- Adapters: the **schema change and the per-append stamp are unconditional** (a store run
  without `sync` for a month must still have groups when `sync` is switched on); the trait
  impl, guard persistence on ingest and the guard decode error variant are behind `sync`.

### 2. The trait (exact signatures)

```rust
// happenstance-sync/src/write_group.rs
use core::num::NonZeroUsize;
use alloc::vec::Vec;
use happenstance_core::{AppendCondition, SequencePosition};
use crate::identity::ReplicatedEvent;
use crate::peer::{EventGroup, Waiting};

#[trait_variant::make(SendWriteGroupStore: Send)]
pub trait WriteGroupStore {
    /// How this store fails, stating a class so a runner never infers one.
    type Error: core::error::Error + crate::Classify + 'static;

    /// The write groups whose `first` position is above `after` (every group
    /// when `None`), in ascending `first` order, each whole.
    ///
    /// Fills to `max_events` events and stops **before** the group that would
    /// take the page past it. The first group is returned whole however many
    /// events it carries, so a caller always learns what blocks it; deciding
    /// whether it can travel is the caller's (see `Edge::step`, `StoreSyncPeer`).
    /// `waiting` is `Waiting::Nothing` only when the store saw no group past the
    /// page; `Unknown` otherwise. A store never counts.
    ///
    /// # Errors
    /// The adapter's error; its `Classify` impl states the class.
    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error>;
}
```

Why this shape (for the owner, the Rust reasoning):

- **`async fn` returning an owned page, not a top-level `impl Stream`.** Constraint 3 makes
  `EventStore::read` return its stream un-nested because a nested stream silently loses
  `+ Send` on the `Send` flavour. That hazard does not exist here: the output is an owned
  `GroupPage`, and `trait_variant` marks the `Send` flavour's *future* `Send`, which is the
  whole requirement. A stream would also force every adapter to hold a cursor open across
  calls, which Neon cannot (no cursor), and a page is bounded anyway (below).
- **`NonZeroUsize` for `max_events`.** A zero-event page is meaningless and, with "first
  group always returned", ambiguous. A `usize` would need a documented runtime rule; the
  `NonZero` type makes the state unwritable (API-02). Callers clamp a peer's declared `0` with
  `NonZeroUsize::new(n).unwrap_or(NonZeroUsize::MIN)` — `unwrap_or` here is a total
  function, not a hidden failure (H-02/H-10 do not apply), and the lone group is then offered
  and refused by the peer with its own class (SY-36).
- **`after: Option<SequencePosition>`**, `None` = from the start: the same encoding
  `SyncStatus::confirmed` and `StoreResume::last_forwarded` already use, so both pass it
  straight through (OWN-01: `Copy`, by value).
- **An associated `Error`, not `EventStore::Error`.** Same as `IngestStore`: the bound
  `Error: Classify` lives on the trait, so the runner needs no extra `where` clause, and
  adapters set it to their one error type. Generic code names `<S as WriteGroupStore>::Error`.
- **Not a supertrait of `EventStore`.** Nothing that reads groups needs `EventStore`; a
  supertrait would only force the bound on test doubles (CRF-05).
- **Bounded memory (H-13).** A page holds at most `max_events` events plus one group, and a
  group is bounded by the store's own per-call ceiling (`MAX_EVENTS_PER_BATCH`: 256 SQLite /
  Postgres, 128 Neon; the ingest call ceiling for ingested groups).
- Flavour story: adapters that cross threads (`MemoryIngestStore`, SQLite, Postgres, Neon)
  implement `SendWriteGroupStore` and get the bare trait from `trait_variant`'s blanket impl;
  the Durable Object store implements the bare `WriteGroupStore`. Generic code (the runner,
  `StoreSyncPeer`, the testkit) binds the bare trait (RS-20-2, CLAUDE.md rule 4) and imports
  only one of the two names per module.

### 3. Value types (OWN, API-02/03)

```rust
/// One write as the store recorded it: the events one append, or one ingested
/// group with its compensation, wrote together, and the origin's guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteGroup {
    first: SequencePosition,
    guard: Option<AppendCondition>,
    events: Vec<ReplicatedEvent>,   // never empty: built from a head event
}

impl WriteGroup {
    /// A group whose lowest local position is `first`, starting with `head`.
    #[must_use]
    pub fn new(first: SequencePosition, guard: Option<AppendCondition>, head: ReplicatedEvent) -> Self;
    /// Adds the group's next event, in local position order.
    pub fn push(&mut self, event: ReplicatedEvent);
    #[must_use] pub const fn first(&self) -> SequencePosition;
    #[must_use] pub const fn guard(&self) -> Option<&AppendCondition>;
    #[must_use] pub fn events(&self) -> &[ReplicatedEvent];
    #[must_use] pub fn payload_len(&self) -> usize;   // sum of ReplicatedEvent::payload_len
}

/// Into the wire shape: the guard travels as evidence (SY-6), the events in order.
impl From<WriteGroup> for EventGroup { .. }      // moves, no clone (H-08)

/// What one `read_groups` returned.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct GroupPage {
    /// Whole groups, ascending by `first`.
    pub groups: Vec<WriteGroup>,
    /// Whether any group lay past the page when it was read.
    pub waiting: Waiting,
}
impl GroupPage { #[must_use] pub const fn new(groups: Vec<WriteGroup>, waiting: Waiting) -> Self; }
```

- Private fields + `new(first, guard, head)` + `push` make an empty group unrepresentable
  without an error type (API-02). The alternative, `new(first, guard, Vec) -> Result<_,
  EmptyWriteGroup>`, lost: every adapter would map an error that cannot happen.
- No `#[non_exhaustive]` on `WriteGroup`: its fields are private, which already reserves the
  right to add one (API-05).
- `GroupPage::waiting` reuses `peer::Waiting` rather than a near-duplicate enum: the meaning
  is identical ("what lies past this answer"), and `StoreSyncPeer::pull` hands it through.
- No serde on either: they never travel. The wire shape is `EventGroup`.

**Why `first` and not "last position" is the cursor.** SQLite, the Durable Object and the
memory stores serialise writers, so a group's rows are contiguous and either end would do.
Postgres and Neon do not: `nextval()` per row lets two concurrent appends interleave
(G = {5, 7}, H = {6, 8}). Resuming "above the last pushed position" after pushing G would
skip H's row 6; resuming "above the first position of the last pushed group" does not, and
I-6 still holds (a row at `p <= first(G)` belongs to a group whose `first <= p`, already
read). I-5 holds because a group is one transaction and ES-10 forbids any row below an
observed position (G's `first`) from appearing later. This is also why I-2 says *lowest
position actually written*: Postgres burns a position on a skipped row, and an id equal to a
burnt position is not an observed position, so ES-10 says nothing about it.

### 4. Guard persistence (`write_group/guard.rs`)

```rust
/// The first byte of every stored guard: this module's own format number,
/// independent of `wire::FORMAT_VERSION`, so a wire bump never strands a stored guard.
pub const STORED_GUARD_FORMAT: u8 = 1;

/// # Errors  `GuardCodecError::Encode` if postcard refuses the value.
pub fn encode_guard(guard: &AppendCondition) -> Result<Vec<u8>, GuardCodecError>;
/// # Errors  `Empty`, `UnknownFormat { found }`, or `Decode` for bytes this module did not write.
pub fn decode_guard(stored: &[u8]) -> Result<AppendCondition, GuardCodecError>;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GuardCodecError {
    #[error("a stored guard is empty")] Empty,
    #[error("a stored guard is in format {found}, and this build reads format 1")]
    #[non_exhaustive] UnknownFormat { found: u8 },
    #[error("a guard could not be encoded")] Encode(#[source] crate::wire::PostcardError),
    #[error("a stored guard could not be decoded")] Decode(#[source] crate::wire::PostcardError),
}
```

- Encoding: one format byte, then postcard of `AppendCondition` through `happenstance-core`'s
  `serde` feature, which `happenstance-sync` already enables; postcard is already a normal
  dependency. **No new dependency.** `crate::wire::PostcardError` (`wire/codec.rs:97`) has a
  field private to `codec`, so `guard.rs` cannot build one: widen it to
  `pub struct PostcardError(pub(crate) postcard::Error);` — crate-visible only, so postcard
  stays out of the public API, which is that type's whole point. `wire/codec.rs` is not a
  file the incarnation branch is listed as editing; `wire.rs` is, and is not touched.
- Re-decoding validates: `Query`/`Tags` re-validate on deserialisation (spec SY-29 prose), so a
  corrupt row cannot smuggle an invalid condition into memory.
- Each adapter adds one `#[cfg(feature = "sync")]` variant to its (already `#[non_exhaustive]`)
  error, e.g. `SqliteEventStoreError::StoredGuard { first: SequencePosition, #[source] source:
  GuardCodecError }`, classified **`Stop`** (corrupt stored data; no retry cures it). The
  encode failure on the ingest path maps to `IngestError::Store(that variant)` before anything
  is written.
- Stored where: a nullable `group_guard` blob column on `event`, set **only on the group's
  `first` row**. One table, no join, no second table to create in four dialects. A side table
  keyed by group id was the alternative; it buys nothing (the id is still known only after
  the insert) and costs a `LEFT JOIN` on every read.

### 5. `IngestGroup` gains the guard (additive; `ingest.rs` after line 230)

```rust
pub struct IngestGroup<'a> {
    pub events: &'a [ReplicatedEvent],
    pub compensation: &'a [Event],
    /// The origin's condition for `events`: evidence only, never evaluated (SY-1, SY-6).
    /// Stored with the group and handed back by `WriteGroupStore::read_groups`.
    pub guard: Option<&'a AppendCondition>,
}
impl<'a> IngestGroup<'a> {
    pub const fn new(events, compensation) -> Self { .., guard: None }   // unchanged signature
    #[must_use]
    pub const fn with_guard(mut self, guard: Option<&'a AppendCondition>) -> Self;
}
```

`#[non_exhaustive]` already reserved this (its own doc says the guard "has nowhere to go here
yet"); rewrite that sentence. Borrowed, like the rest of the struct, for the reason its doc
gives: the runner owns the pulled batch.

### 6. Minting the group id, per adapter

| Store | `append` | ingest, per `IngestGroup` | read |
|---|---|---|---|
| `MemoryIngestStore` | `first` = first staged position | `first` = first staged foreign row; compensation joins; guard cloned | iterate `log` |
| SQLite (`rusqlite`) | `stamp_write_group(tx, first, None)` after `write_batch` | after the group's foreign `write_batch` **and** its compensation `write_batch`: `stamp_write_group(tx, first, guard_bytes)` | position scan |
| Durable Object | as SQLite (copied, never depended on) | as SQLite | position scan, paged |
| Postgres (`sqlx`) | bind `write_group = min(positions)` in the one `INSERT` | `RETURNING position` added; stamp landed foreign rows; compensation binds `write_group = first` | one CTE statement |
| Neon (one-shot HTTP) | `write_group = min(a.position) OVER ()` inside the one `INSERT … SELECT` | provisional negative tag, rewritten in the same batch | one CTE statement |

Details the implementer needs, so no SQL is a design decision of theirs:

- **SQLite / DO stamp** (rowid seek, NF-001-safe, under `BEGIN IMMEDIATE`):
  `UPDATE event SET write_group = ?1, group_guard = CASE WHEN position = ?1 THEN ?2 END
   WHERE position >= ?1 AND write_group IS NULL`.
  `Written` gains an unconditional `first: Option<i64>` (the first row that went in, local or
  foreign). Legacy rows are all below `?1`; earlier groups of the same call are already
  stamped; the writer lock rules out anyone else. `append_locked` calls it with `?2 = NULL`;
  `ingest_locked` calls it once per group that landed something, after the compensation.
  A helper `fn stamp_write_group(connection: &Connection, first: i64, guard: Option<&[u8]>)
  -> Result<(), SqliteEventStoreError>` in `event_store.rs`.
- **Postgres append**: `insert_statement` gains a ninth bind, `write_group: Option<i64>`
  (`Some(min(positions))` for an append, `None` for foreign rows, `Some(first)` for
  compensation). 9 × 256 = 2,304 binds, inside the protocol's 65,535.
- **Postgres ingest**: `CONFLICT_CLAUSE`'s `RETURNING` adds `position`. If any landed:
  `first = min(landed positions)`; then
  `UPDATE event SET write_group = $1, group_guard = CASE WHEN position = $1 THEN $2 END
   WHERE position = ANY($3)` over the landed positions; then compensation inserted with
  `write_group = first` (its positions come later from the same sequence, so they are higher).
- **Neon append**: the `INSERT … SELECT … JOIN (drawn_positions)` adds the column
  `write_group` with value `min(a.position) OVER ()`; an append has no conflict clause, so
  every drawn position lands.
- **Neon ingest** (one statement for all groups, no interactive transaction): rows bind
  `write_group = -(group_index + 1)` (both foreign and compensation rows; the group index is
  already in `encode_ingest`'s tagging). One more statement in the **same** batch, after the
  insert and after the unheld-claim refusal statement:
  `UPDATE @event@ e SET write_group = g.first, group_guard = CASE WHEN e.position = g.first
     THEN decode(($1::text[])[-g.tag], 'base64') END
   FROM (SELECT write_group AS tag, min(position) AS first FROM @event@
         WHERE xact_id = pg_current_xact_id() AND write_group < 0 GROUP BY write_group) g
   WHERE e.xact_id = pg_current_xact_id() AND e.write_group = g.tag`.
  `$1` is the per-group guard array (base64, `NULL` for none), indexed by group. A negative
  tag never survives the transaction; `event_xact_idx` serves the `xact_id` predicate. No
  `CHECK (write_group > 0)` is possible (a `CHECK` is per statement), and none is added.
- **Read, contiguous stores** (memory, SQLite, DO): scan
  `WHERE position > ?after AND coalesce(write_group, position) > ?after ORDER BY position`
  lazily (rusqlite `Rows` / `SqlStorage` cursor paging); accumulate rows into the current
  group by `coalesce(write_group, position)`; `group_guard` is decoded from the row where
  `position = group id`. Stop as soon as a started group cannot fit the remaining budget (or
  the first group is complete and the next starts past the budget): `waiting = Unknown`. Scan
  exhausted: `waiting = Nothing`. No new index: the `INTEGER PRIMARY KEY` range does it.
  SQLite runs it in `spawn_blocking` the way `watermark` does.
- **Read, Postgres and Neon** (groups can interleave, so the order is not `position`): one
  statement, one snapshot, under the ES-10 frontier predicate both adapters already use:
  ```sql
  WITH ids AS (
    SELECT DISTINCT g FROM (
      SELECT coalesce(write_group, position) AS g FROM event
      WHERE position > $1 AND coalesce(write_group, position) > $1 AND <frontier>
      ORDER BY position LIMIT $2 + 1) s)
  SELECT <row columns>, coalesce(write_group, position) AS g, group_guard FROM event
  WHERE (write_group IN (SELECT g FROM ids)
         OR (write_group IS NULL AND position IN (SELECT g FROM ids)))
    AND <frontier>
  ORDER BY g, position
  ```
  Scanning by position meets each group's lowest row first, so `ids` come out in group order.
  Trim in Rust: first group whole, then whole groups while the total stays within
  `max_events`. `waiting = Nothing` only if the inner scan returned at most `$2` rows and no
  group was trimmed. A group is one transaction, so if its id is visible all its rows are.
  New partial index (migration 3): `event_write_group_idx ON event (write_group) WHERE
  write_group IS NOT NULL`; legacy rows are found through the primary key.

### 7. Legacy rows (I-9; documented in each adapter's schema docs and the CHANGELOG)

A row written before the migration has `write_group IS NULL` and reads as **its own group,
id = its position, guard `None`**. One group per original append is unknowable: nothing
recorded which rows one call wrote. Consequences, stated in the docs: a pre-migration
multi-event append still travels split (exactly as today, no worse); a pre-migration ingested
group travels without its guard. No backfill: a full-table `UPDATE` on Postgres is a long lock
for no information gain, and `coalesce` answers the same question at read time.

### 8. Migrations (keeping each adapter's immutability convention)

- **SQLite** (marker in `store_meta`, `SCHEMA_VERSION` pub const): `MIGRATION_1` stays
  verbatim. New `const MIGRATION_2: &str = "ALTER TABLE event ADD COLUMN write_group INTEGER;
  ALTER TABLE event ADD COLUMN group_guard BLOB;"`. `migrate` (every connect, `BEGIN
  IMMEDIATE`): run `MIGRATION_1`, insert marker 1 if absent, then **if the marker reads 1**:
  run `MIGRATION_2` and set the marker to 2, same transaction, then the existing
  `found > SCHEMA_VERSION` refusal. `SCHEMA_VERSION` becomes `2`. `ALTER … ADD COLUMN` has no
  `IF NOT EXISTS`, which is why it is gated on the marker rather than run every connect. A
  fresh file and an old file take the same path. Update the module-doc schema block (and
  `tests/migration.rs::module_doc_schema_matches_sqlite_master`'s expectation if it compares
  column lists) to show the two columns. See D-1 for raising the marker eagerly.
- **Postgres** (published; `MIGRATION_1` text is frozen since first publish, per
  `migration.rs`'s own doc): new file `migrations/0003_write_group.sql` (the directory's next
  number; `0002` is the projection lineage):
  `ALTER TABLE event ADD COLUMN IF NOT EXISTS write_group bigint;`
  `ALTER TABLE event ADD COLUMN IF NOT EXISTS group_guard bytea;`
  `CREATE INDEX IF NOT EXISTS event_write_group_idx ON event (write_group) WHERE write_group IS NOT NULL;`
  `pub const MIGRATION_3: &str = include_str!(...)`; `SCHEMA_VERSION` → `2` (it names the
  event-log lineage, which this extends); `apply` runs `MIGRATION_1` then `MIGRATION_3` in the
  same `raw_sql` batch (string-joined), so one implicit transaction. Doc: on a large existing
  table a DBA may pre-create the index `CONCURRENTLY` under the same name, and `IF NOT EXISTS`
  then skips it (D-3).
- **Neon**: new `migrations/0003_write_group.sql` with `@event@` and a new
  `@write_group_idx@` placeholder (`render` gains one `.replace`); the splitter rules hold (no
  dollar quotes, no `;` in literals). `pub const MIGRATION_3`. Whatever applies `MIGRATION_1`
  in the crate (fixture, `migration::request` callers, README procedure) applies
  `MIGRATION_3` after it.
- **Durable Object** (no marker, `migrate` idempotent, called on every open, namespaced per
  `Tables`): after the existing statements, probe
  `SELECT 1 FROM pragma_table_info(?) WHERE name = 'write_group'` with the rendered event
  table name; if absent, `ALTER TABLE {event} ADD COLUMN write_group INTEGER` and
  `ALTER TABLE {event} ADD COLUMN group_guard BLOB`. Update the module-doc schema block.
  (D-1 for whether to introduce a marker instead.)

### 9. Runner (`runner.rs`, `runner/error.rs`)

- `Edge::step` / `exchange` / `pull_and_ingest` / `push_local` bound becomes
  `S: IngestStore + WriteGroupStore` (drop `EventStore`, nothing reads through it any more;
  the `<S as EventStore>::Error: Classify` clause disappears because the trait carries the
  bound). `StepFailure<P, S>` becomes
  `StepError<<P as SyncPeer>::Error, <S as WriteGroupStore>::Error, <S as IngestStore>::Error>`.
  The runner is **unreleased** (CHANGELOG `[Unreleased]`), so this is not a published break.
- Pull side: `IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref())`. One-line
  change; this is what makes a relayed group keep its guard on the relay's store.
- Push side, replacing `Tail`:
  ```text
  limits = peer.limits()
  max    = NonZeroUsize::new(limits.max_batch_events).unwrap_or(NonZeroUsize::MIN)
  page   = WriteGroupStore::read_groups(store, status.confirmed, max)   -> StepError::Read
  run    = Run::gather(page, &limits)
  ```
  `struct Run { groups: Vec<WriteGroup>, events: usize, left: usize, last_first:
  Option<SequencePosition>, waiting: Waiting }`. `gather` walks the page's groups with a
  `PullBatchLimit::from(&limits)` budget (reuse: it already holds exactly the three
  comparisons, events-left, bytes-left, every event `<= max_event_bytes`): take a group while
  `budget.admits(&group)`; take the **first** group even when it does not fit, and stop
  after it (offered alone, so the peer's refusal states the class, SY-36, as today for an
  oversized event). Stop at the first group that does not fit; never skip one (I-6 is a
  prefix). `PullBatchLimit::admits` takes `&EventGroup`, so convert each `WriteGroup` into an
  `EventGroup` before the check (a move, no clone) and keep its `first` beside it in `Run`.
  Do not add a `WriteGroup`-taking method to `PullBatchLimit`: that edits `peer.rs`.
- Pending: `left` = events of page groups not taken. `Pending::Exactly(left)` when
  `page.waiting == Waiting::Nothing`, `Pending::Exactly(left + n)` for `Events(n)`,
  `Pending::AtLeast(left)` for `Unknown`; before the push the same with `events + left`.
- Push `PushBatch::new(run.groups)`; on ack `status.confirmed = run.last_first`.
- `SyncStatus::confirmed` doc becomes: "the `first` position of the last local write group
  the far side acknowledged; every local event at or below it, and every event of that group,
  has been acknowledged". For single-event appends this is the position `append` returned, so
  every existing runner test and the `Edge` doctest keep their expectations.
- Delete the module doc's "A known gap" section and the matching `step` paragraph; replace
  with a "Groups" section stating I-6/I-7 and the oversized-group rule.

### 10. `StoreSyncPeer` and `serve_sync`

- Bounds: `S: EventStore` → `S: WriteGroupStore` on the inherent `admit` impl, the
  `SyncPeer` impl, and `serve_sync` (`S::Error: Classify` lines go; the trait carries it).
  `StorePeerError<R, W>`'s `R` is now the group store's error; `Read`'s message becomes "the
  far store's group read failed". Unreleased, so not a break.
- `pull`:
  ```text
  after = from.last_forwarded()
  max   = NonZeroUsize::new(limits.max_batch_events).unwrap_or(NonZeroUsize::MIN)
  page  = self.events.read_groups(after, max)                -> StorePeerError::Read
  for each group: g = EventGroup::from(group) (keep first);
      if !budget.admits(&g) { stalled = true; break }  budget.charge(&g); resume = StoreResume::after(first); push g
  waiting = Nothing iff !stalled && page.waiting == Nothing, else Unknown
  ```
  The `next_position()` early return goes (no `ReadOptions::from` overflow any more; a token
  at `SequencePosition::MAX` simply reads an empty page). If `StoreResume::next_position`
  becomes unused, delete it.
- A head group the declared budget cannot admit: the pull returns an empty page,
  `Waiting::Unknown` and the token it was given, and replication **stalls at that group**. This
  is the port's existing documented rule (`Waiting::Nothing`'s doc, `MemorySyncPeer`'s
  `a_stall_behind_an_oversized_group_counts_it`, the store peer's oversized-event test), now
  extended from "an event" to "a group". The `# Limits` doc gains: declare
  `max_batch_events >= ` the store's per-call ceiling and `max_batch_bytes >=` the largest
  write the store's writers accept, or a legitimate group stalls. See D-2 for the loud
  alternative.
- `push`: `IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref())`; the doc's
  "not stored: no store keeps one" becomes "stored with the group and forwarded on pull".
- `StoreResume`'s docs (`store_peer/resume.rs`): "the last position a pull forwarded" →
  "the `first` position of the last group a pull forwarded". The text form is unchanged.
- Module docs: rewrite "# Groups: one page is one group" as "# Groups: one write is one group"
  (boundaries and guards preserved; legacy rows are singletons; a page never splits a group).
- Doctest: the hidden `Shared` wrapper gains a `SendWriteGroupStore` impl (5 lines); the
  visible assertion `pulled.batch == batch` keeps holding (one group in, one group out).

### 11. In-memory stores

- `MemoryIngestStore` implements `SendWriteGroupStore` in a child module
  `memory_ingest/groups.rs` (declared at the end of `memory_ingest.rs`; the file is already
  818 lines). `State` gains `group_of: Vec<SequencePosition>` (parallel to `log`, written in
  the same `commit`) and `guards: BTreeMap<SequencePosition, AppendCondition>`. `stage`
  returns, per landed group, its `first` and guard; `commit` takes them. Ownership: the guard
  is cloned once out of the borrowed `IngestGroup` because the store keeps it (H-08: "the
  callee stores the value").
- `happenstance-core`'s `MemoryEventStore` does **not** implement the trait: it records no
  boundaries, nothing outside core can make it, and `ingest.rs` already settles that the sync
  oracle is `MemoryIngestStore`. Not relevant to this issue.
- `happenstance-sync-testkit`'s `MemoryIngestHandle` delegates `SendWriteGroupStore`.

### 12. Errors (ERR)

- No new error type in the port beyond `GuardCodecError`. Adapters reuse their one error and
  add a `#[cfg(feature = "sync")]` `StoredGuard { first, source: GuardCodecError }` variant,
  `Stop`. Read failures in the runner surface as `StepError::Read` (phase `Read`), unchanged.
- Oversized group on **push**: unchanged. The runner offers it alone; `StoreSyncPeer::admit`
  answers `TooManyEvents` / `BatchTooLarge` (`Split`), `MemorySyncPeer` answers
  `BatchTooLarge` (`Split`). A one-group `Split` cannot be split, and what part 2 does with
  it is #244's (the same deferral `IngestError::ExceedsStoreLimit`'s doc already makes). See
  D-4.

### 13. Conformance and the specification

**A. Peer family (`sync_suite.rs`), one new rule** — `push_envelope_preserves_group_boundaries`
(SY-30; today a † in §7.1). Push one batch of three groups: `[a1, a2]` under a position-free
guard `G1` (`AppendCondition::new(query)`, **no `after`**, so SY-6's future refusal never bites),
`[b1]` with `None`, `[c1, c2]` under `G2`. Pull everything (`pull_everything`). Assert the
pulled sequence of groups equals the pushed one: same boundaries, same events in order, same
guards. Rejects, written into `tests/wrong_peers.rs`: `PageMergingPeer` (today's
`StoreSyncPeer` shape: one group per page) and `GuardDroppingPeer`. The existing
`OneGroupPerEventPeer`-style flattener (if `wrong_peers.rs` has one for SY-2) must fail it too.
Mounted automatically on memory, kv-capped, store and remote fixtures.

**B. New family: write-group store conformance.** `src/write_group_suite.rs` (new; declared
after `ingest_suite` in `lib.rs`), `pub mod write_group_rules`, `for_each_write_group_rule!`,
`emit_write_group_tokio!`, `emit_write_group_blocking!`, `__write_group_rule_names!`, and
`write_group_store_conformance!` mirroring `ingest_store_conformance!` (same three arms). The
fixture is the existing `IngestFixture`; each rule is
`pub async fn name<F>(open: impl AsyncFn() -> F) -> Result<RuleOutcome, Violation> where F:
IngestFixture, F::Store: WriteGroupStore`. No new fixture trait, no bound added to
`IngestFixture::Store` (that would break the three adapters PR 1 does not touch). Rules, each
with the wrong store it rejects:

| Rule | Arrangement and assertion | Rejects (`tests/wrong_write_group_stores.rs`) |
|---|---|---|
| `an_append_reads_back_as_one_group` | append 3 events, then 1; one page; two groups `[3, 1]`, `first` equal to the positions the store assigned (read via `EventStore::read`, never literals), guards `None` | `EveryRowItsOwnGroup`, `MergesAdjacentAppends` |
| `an_ingested_group_keeps_its_boundary_and_guard` | one `ingest` of two groups, `[x1, x2]` with `G` and `[y1]` with `None`; two groups back, origin ids preserved, guards `[Some(G), None]` | `OneGroupPerIngestCall`, `DropsGuard` |
| `compensation_joins_its_group` | ingest `[losing]` + compensation `[reversal]`; one group holding both, in that order | `CompensationApart` |
| `a_read_never_cuts_a_group` | appends of 2, 3, 2; `read_groups(None, 4)` → only `[2]`, `waiting != Nothing`; `read_groups(after = that first, 1)` → the 3-group whole | `CutsAtMaxEvents`, `SkipsAnOversizedGroup` |
| `pages_chain_on_first_and_end_with_nothing` | appends of 2, 1, 3, 1 events; page with `max_events = 2` and `after = last.first()` until empty; every group exactly once, in order; `Waiting::Nothing` on the last page only | `ResumesByRowPosition` (filters rows `> after`, so a group's tail comes back as a partial group), `SaysNothingWhenCut` |

The wrong stores are one `WrongGroupStore { inner: MemoryIngestStore, mistake: Mistake }` that
delegates `EventStore` / `IngestStore` and corrupts `read_groups`' output (merge, split, drop
guard, separate compensation by `id.store() == own`, truncate, filter by row position, report
`Nothing`). Each test asserts the mistake fails **exactly** its intended rule(s) and passes the
rest, as `wrong_ingest_stores.rs` does. Mounts: `tests/memory_write_group_conformance.rs`
(`MemoryIngestFixture`), SQLite beside its ingest mount in `src/ingest_spike.rs` (PR 1),
Postgres and Neon `family.rs` (PR 2), Cloudflare `family.rs` and the workerd harness (PR 3,
which adds `emit_write_group_wasm!`).

**C. Specification** (`spec/SPECIFICATION.md`; `cargo xtask spec-trace` after every edit).
- SY-30 (PROVISIONAL, editable without an ADR): remove the † from
  `push_envelope_preserves_group_boundaries`; add a sentence that the store side now
  preserves the decomposition and cite SY-39. Its "Rejects" paragraph gains the page-merging
  relay.
- New **SY-39 [PROVISIONAL]**: *"A store that forwards its log MUST forward it in the write
  groups it wrote: one group per append and per landed ingest group with its compensation,
  each whole, ordered by its lowest local position, an ingested group carrying the guard it
  arrived with. Rows written before a store recorded groups are one group each."* Rules: the
  five above. Falsified if a store is found that cannot recover, or cannot record, which rows
  one write produced. *Rejects:* the page-as-one-group peer and the per-batch runner push
  (#420's two defects). §7.1's generated summary picks it up. See D-5.
- Do not move any cited line in `peer.rs`, `lib.rs` (<= 134) or `ingest.rs` (<= 230).

**D. Decision record**: a new atom in `.kb/decisions/` recording option A (owner, 2026-11-08):
per-row write-group id, additive migration on four adapters, `WriteGroupStore` in
`happenstance-sync`, `first` as the cursor and why (interleaving writers), legacy singleton
rule, no wire change. Number: the next free one at merge time after ADR-0104 (D-6). Add it to
`.kb/maps/` decisions map.

**E. Docs (PROC-11)**: `CHANGELOG.md` `[Unreleased]` — Added (the trait, `IngestGroup::guard`,
the testkit family and rule, per-adapter migrations), Changed (runner and store peer push and
pull whole write groups; remove the "Known gap" sentence from the runner entry), and a
**Upgrading** note per adapter (D-1, D-3). Crate READMEs of `happenstance-sync` (status) and
each adapter (schema/migration section). `happenstance-sync`'s `lib.rs` `# Status` if it
mentions #420.

### Alternatives considered (one line each)

- Infer groups from `recorded_at` equality: two appends in one millisecond merge; rejected.
- Infer groups from Postgres `xact_id`: one ingest call is one transaction holding many
  groups; and SQLite has no equivalent; rejected.
- Put the group id on `SequencedEvent`: changes a published core type (forbidden).
- Cursor on last position: wrong for interleaving writers (§3).
- A separate `write_group` table: same information, an extra join on every read.
- Backfill legacy rows: long write lock on Postgres, no information gained.

## Test list (drive in this order; PR boundaries marked)

### PR 1 — port, memory, SQLite, testkit, runner, store peer

`happenstance-sync` unit tests:
- [x] 1. `write_group::guard`: a guard round-trips (`encode` then `decode` equals the input),
      for a single-guard and a multi-guard (`and_guard`) condition.
- [x] 2. `decode_guard(&[])` is `Empty`; a leading byte `2` is `UnknownFormat { found: 2 }`;
      a truncated postcard body is `Decode`.
- [x] 3. `WriteGroup::new` + `push` keeps order; `From<WriteGroup> for EventGroup` moves
      guard and events.
- [x] 4. `IngestGroup::new(..)` has `guard: None`; `with_guard(Some(&g))` sets it.
`MemoryIngestStore` (`memory_ingest/groups.rs`), red first against the not-yet-written impl:
- [x] 5. an append of 3 reads back as one group with `first` = its first position.
- [x] 6. two appends read back as two groups.
- [x] 7. an ingest of two groups reads back as two groups with their guards.
- [x] 8. compensation joins its ingested group.
- [x] 9. a fully re-delivered group writes no group; a partly re-delivered one writes one
      group of its new events.
- [x] 10. `max_events` stops before a group that would overflow; the first group comes whole
      even when larger; `waiting` is `Nothing` only at the end.
testkit (write the rule, watch it go red on memory before step 5–10 land, green after):
- [x] 11. the five write-group rules green on `MemoryIngestFixture`.
- [x] 12. `wrong_write_group_stores.rs`: each `Mistake` fails exactly its rule(s).
- [x] 13. `push_envelope_preserves_group_boundaries` green on memory, kv-capped fixtures.
- [x] 14. `wrong_peers.rs`: `PageMergingPeer` and `GuardDroppingPeer` fail it.
Runner (`tests/runner/main.rs`; `probe.rs`'s `ProbeStore` gains a `WriteGroupStore` impl whose
injected read failure moves from `read` to `read_groups`):
- [x] 15. `a_multi_event_append_is_never_split_across_pushes`: appends of 2, 3, 2 one-byte
      events, `batches_of(4)`; step until `is_synced`; the far `MemorySyncPeer`'s held groups
      (pulled back) equal the local `read_groups` sequence exactly (I-7). Red on today's
      runner (it pushes `[2+2-of-3]`).
- [x] 16. `the_tear_check_rejects_a_runner_that_cuts_mid_append`: the comparison helper used
      by 15 (`fn torn(local: &[WriteGroup], held: &[EventGroup]) -> Option<String>`) reports a
      tear for the batches today's runner would build (a hand-written `per_event_push`
      reproducing the old `Tail::gather` cut at `max_batch_events`). This is the acceptance
      criterion's "wrong runner that cuts mid-append fails", and the corollary "a rule no
      implementation can fail is decorative".
- [x] 17. `independent_appends_are_not_merged`: three single-event appends, generous limits;
      three groups at the far side, not one.
- [x] 18. `a_relayed_foreign_group_keeps_its_boundary_and_guard`: far peer seeded with
      `EventGroup(Some(G), [a, b])` and `EventGroup(None, [c])`; edge 1 pulls into store R;
      edge 2 pushes R to a second `MemorySyncPeer`; it holds exactly those two groups, guards
      intact (SY-10 relay).
- [x] 19. `a_group_over_the_batch_limit_is_offered_alone_and_refused`: one append of 3 events,
      `batches_of(2)`; `StepError::Push(BatchTooLarge|…)`, `confirmed` unchanged, `pending`
      `Exactly(3)`; the next step offers it again (not skipped, not split).
- [x] 20. `a_sweep_never_tears_a_group`: deterministic table over append-size sequences
      (`[1..=4]` combinations of length 5) × `max_batch_events` in `4..=6`; property I-7 holds
      for each. (TST-06 without adding `proptest` to this crate.)
- [x] 21. Existing runner tests stay green unchanged (all single-event appends).
`tests/flavours.rs`:
- [x] 22. `LocalStore` (`!Send`) implements bare `WriteGroupStore` (local events as singletons
      from its `MemoryEventStore`); the existing bare-flavour `Edge::step` test compiles and
      passes; `edge_spawns_from_generic`'s bound gains `SendWriteGroupStore` and the
      `tokio::spawn` test passes (the Send flavour is real in generic code).
`StoreSyncPeer` (`happenstance-sync-testkit/tests/store_sync_conformance.rs`):
- [x] 23. Replace `a_page_travels_as_one_group_without_a_guard` with
      `a_page_carries_the_groups_the_store_wrote` (push three singles plus one guarded
      2-group; pull returns four groups, guard intact). **Changes an existing test's
      expectation — covered by the owner's option-A decision; recorded as D-7.**
- [x] 24. `a_group_crossing_the_page_budget_moves_to_the_next_page_whole`: limits of 3; push
      groups `[2]`, `[2]`; first pull carries `[2]` only, `Unknown`; second carries `[2]`,
      `Nothing`.
- [x] 25. `an_oversized_group_at_the_head_stalls_the_pull_on_one_token`: far-store append of 3
      events under limits of 2; pull returns empty, `Unknown`, same token, twice.
- [x] 26. the remaining store-peer tests (`byte_for_byte`, token resume, local append
      forwarded, page cut by count, oversized event stall, push refusals) stay green.
- [x] 27. `store_sync_conformance` and `remote_sync_conformance` pass the new peer rule.
SQLite (`happenstance-sqlite`):
- [x] 28. `tests/migration.rs`: a fresh file ends at marker 2 with both columns.
- [x] 29. `a_version_1_file_is_raised_to_2_and_its_rows_read_as_singletons`: build a v1 file
      with raw `rusqlite` (verbatim migration 1 text, marker 1, a 3-row append written by raw
      SQL), open with this build: marker 2, columns present, `read_groups` yields 3 singleton
      groups, and a new append afterwards reads as one group.
- [x] 30. second open of a migrated file changes nothing (existing test, extended to the
      marker staying 2).
- [x] 31. the stamp is a rowid seek: a `query_plan` assertion (as the file already does for
      other statements) that the stamp `UPDATE` uses the primary key, not a scan (NF-001).
- [x] 32. a corrupt `group_guard` (format byte 9) surfaces as `StoredGuard`, class `Stop`.
- [x] 33. `write_group_store_conformance!` mounted beside the ingest mount, green.

### PR 2 — Postgres and Neon

- [x] 34. Postgres `migration.rs` unit: `MIGRATION_3` adds both columns and the partial index;
      the comment-stripper guard tests cover it; `apply` sends 1 then 3.
- [x] 35. Postgres `insert_statement` text test: nine columns, `write_group` bound.
- [x] 36. Postgres live: an append's rows share `write_group = min(position)`; an ingest group
      whose first foreign event is a re-delivery takes `first` from the lowest **landed** row
      (I-2); compensation joins.
- [x] 37. Postgres live: two concurrent 3-event appends (interleaved positions forced by a
      shared sequence and a barrier, as the concurrency family does) read back as two whole
      groups in `first` order, and paging with `max_events = 3` returns each exactly once
      (the interleaving case §3 exists for).
- [x] 38. Postgres live: a schema with migration 1 only and pre-existing rows, then
      `apply`: rows read as singletons; new appends group.
- [x] 39. Neon: `render(MIGRATION_3)` splits into three statements with no placeholder left;
      the ingest request carries the stamp statement after the refusal statement (request
      shape unit test, CF-33 permits adapter-local shape checks).
- [x] 40. Neon live (`live-neon` CI job): the negative tags never survive a commit
      (`SELECT count(*) WHERE write_group < 0` = 0 after an ingest of three groups); guards
      round-trip.
- [x] 41. Both: `write_group_store_conformance!` mounted in `family.rs`, green; both
      `StoredGuard` variants classify `Stop`.

### PR 3 — Durable Object and the workerd harness

- [ ] 42. `migrate` on an object with the pre-#420 tables adds both columns once; a second
      `migrate` is a no-op (extend `migrate_is_idempotent`); namespaced logs migrate per
      namespace.
- [ ] 43. Pre-existing rows read as singletons; appends and ingested groups stamp; the stamp is
      a rowid seek.
- [ ] 44. `write_group_store_conformance!` with `emit_write_group_wasm!` in the crate's
      `family.rs`, green under `wasm-bindgen-test`.
- [ ] 45. `harness/workerd`: dispatch by name over `for_each_write_group_rule!` (mirroring
      `src/ingest/dispatch.rs`), a vitest entry per rule, run locally and against the
      deployed object in the `workerd` job.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D-0 | Shape: per-row write-group id, additive migrations, additive trait in `happenstance-sync` | A / others | A | Ryan, 2026-11-08 | **Approved** |
| D-1 | Compatibility of a migrated store with an **older build** | (a) SQLite raises its marker to 2 on first open by this build, so older builds refuse the file (`UnsupportedSchemaVersion`); DO and Postgres/Neon have no marker and keep accepting old builds, which write `NULL` groups (singletons). (b) SQLite adds the columns without raising the marker, so older builds keep running everywhere. (c) (a) plus a new `schema_version` row in the DO's `store_meta` | **(a)**. An older build running a *runner or pull* against a store a newer build writes groups into can tear a new group exactly as today; refusing the file is the cheap guarantee where a marker exists. Old **writers** are harmless everywhere (singletons). Document "deploy this build to every writer and peer of a store before it serves sync" | Ryan, 2026-11-08 | **Decided (a)**: marker raised; migrate-then-deploy documented |
| D-2 | A head group larger than a `StoreSyncPeer`'s declared page budget | (a) documented stall: empty page, `Waiting::Unknown`, same token (the port's existing rule for an oversized event, extended to a group). (b) a new `StorePeerError::GroupTooLarge` (`Park`) returned by `pull` — loud, but makes a lone oversized event loud too, changing a documented behaviour and an existing test | **(a)** now, with a follow-up issue to make both loud together | Ryan, 2026-11-08 | **Decided (a)**: the documented stall |
| D-3 | Postgres/Neon rollout: a **new** build against a schema without migration 3 fails its first append with an undefined-column driver error | (a) document "apply migration 3 (re-run `migration::apply`) before deploying"; CHANGELOG **Upgrading** entry; index may be pre-built `CONCURRENTLY`. (b) a store_meta schema row checked on first use with a typed error. (c) runtime column probe with a legacy fallback | **(a)**. (c) silently disables grouping; (b) adds a connect-time round trip to a crate with no connect step. Also confirm the non-concurrent index build inside `apply` is acceptable for existing deployments | Ryan, 2026-11-08 | **Decided (a)**: migrate-then-deploy documented |
| D-4 | Class of a **single** group over a peer's batch limits on push | (a) unchanged: peers say `Split`; what part 2 does with an unsplittable `Split` is #244's. (b) add `GroupTooLarge` (`Park`) to `StorePeerError::admit` and `MemorySyncPeer` — changes `oversized_push_refusal`'s declared class for its one-group batch and two runner test expectations | **(a)**; note the interaction on #244 | Ryan, 2026-11-08 | **Decided (a)**: `Split` unchanged |
| D-5 | Spec: a new provisional clause SY-39 for the store-side obligation, vs folding it into SY-30 | new SY-39 / amend SY-30 only | **New SY-39** (SY-30 is about the envelope; the store obligation needs its own rule list and falsifier) | Ryan, 2026-11-08 | **Decided**: new SY-39 |
| D-6 | Decision record | write an atom in PR 1 (number after ADR-0104, assigned at merge) / none | **Write it** — four published adapters change schema; the repo records such changes as ADRs | Ryan, 2026-11-08 | **Decided**: ADR-0105, written as proposed |
| D-7 | Changing `a_page_travels_as_one_group_without_a_guard`'s expectation in `store_sync_conformance.rs` and removing the runner's "Known gap" text | — | Implied by D-0; listed so the change is explicitly signed off (H-12) | Ryan, 2026-11-08 | **Decided**: expectation changed, "Known gap" removed |
| D-8 | `remote_sync_conformance`'s token expectation moves from "3" to "1" | — | Implied by the cursor being the `first` of the last group forwarded (§3), not the last position | Ryan, 2026-11-08 (design-implied expectation changes) | **Approved** |

Ryan, 2026-11-08, on the reviewer's F1: `SqliteEventStore::new` refusing a schema-1 file with a
typed `SchemaNeedsMigration { found, required }` (Stop), rather than failing its first append with
an untyped driver error, is a design-implied additive change and pre-authorised.

Architect decisions recorded for review (not H-16, but worth a glance):
- **D-A1** Compensation joins its ingested group's write group, and the group keeps the origin's
  guard. SY-2 requires them atomic, and only one `EventGroup` carries atomicity on the wire.
  The guard describes the foreign events only; revisit with #136 (adjudicator) if an
  adjudicator needs to tell them apart.
- **D-A2** A local append records `guard = None`. A local condition is almost always
  position-relative (`after`), which SY-6 makes unreplicable; storing it would make every
  relayed local group refusable once SY-6's refusal lands.
- **D-A3** One `EventGroup` per write group on the wire; the runner never merges groups even
  where merging would be safe, because merging is what loses the guard and the boundary on a
  relay.
- **D-A4** `read_write_groups` (SQLite) runs synchronously under the connection mutex, as
  `ingest` and `watermark` do, rather than on `spawn_blocking`. It is one bounded indexed read
  (`LIMIT` probe rows), the same shape and cost as those two calls, and a store that mixes the
  two disciplines on one mutex is harder to reason about than one that uses either throughout.

## Risks

- **Interleaving writers (Postgres, Neon).** The cursor argument in §3 is the crux. Test 37
  forces interleaving; without it the suite would only ever see contiguous groups and a
  last-position cursor would pass. Keep it.
- **Burnt positions.** An implementation that takes `first` from the first *allocated*
  position passes every rule on SQLite and memory and is wrong on Postgres/Neon (I-2). Test 36
  covers the re-delivered-first-row case.
- **Neon negative tags.** If the stamp statement is ever split into another request, negatives
  commit. Test 40 asserts none survive.
- **Line-number citations.** `spec-trace` fails on a moved cited line. Additions go below
  cited lines; run `cargo xtask spec-trace` after each spec or `happenstance-sync` source edit.
- **Merge with `claude/p13-incarnation`.** Overlap is `store_peer.rs` (pull body, bounds, docs,
  doctest helper) and `serve.rs` (one `where` clause). Rebase whichever lands second; neither
  changes the other's lines in `peer.rs`/`wire.rs`.
- **Rolling deploys (D-1).** An older build's runner reading a store a newer build writes
  groups into can tear a new group. Mitigated by SQLite's marker; documented for the others.
- **Stall on legitimate groups (D-2).** A peer declaring `PeerLimits::minimum` (64 KiB batch)
  stalls on a two-event 40 KiB append that used to travel torn. Correct, and loud only in
  `Waiting`/`pending`; the docs must say to size limits to the store's writers.
- **testkit is pinned exactly by consumers.** A new peer rule can fail an outside peer that
  merges groups; that is the rule's purpose, and the CHANGELOG says so.

## Size (lower bound)

- PR 1: ~26 files, ~2,800 lines (sync src ~700, sync tests ~450, testkit ~1,050 incl. the
  wrong stores, SQLite ~450, spec/ADR/CHANGELOG/READMEs ~250).
- PR 2: ~14 files, ~1,100 lines.
- PR 3: ~10 files, ~650 lines.
- Total: ~50 files, ~4,500 lines.

## Progress

- 2026-11-08: plan written (architect). Nothing implemented. Next: PR 1, test 1.
- 2026-11-08: PR 1 implemented (tests 1–33). Red runs recorded before each green:
  - test 4: E0599 no method `with_guard`, E0609 no field `guard` (`ingest.rs`).
  - tests 5–10: E0599 `read_groups` unsatisfied bounds (`memory_ingest/groups.rs`).
  - tests 15, 17–20 against the old runner: torn `[4, 3]` for writes `[2, 3, 2]`;
    `independent_appends` 1 vs 3; relay merged into one unguarded group; sweep
    `[4, 1]` for `[1, 1, 1, 1, 1]`; oversized group not offered alone. Test 16 passes by
    design (it checks the comparison helper).
  - test 22: E0277 `LocalStore: WriteGroupStore` not satisfied.
  - tests 23–25 against the old `StoreSyncPeer`: three FAILED.
  - SY-30 wrong peers: four FAILED, "not in the enumeration".
  - tests 28–30 (SQLite migration, append stamp): three FAILED against marker 1.
  - tests 31 and the group-read plan: the named wrong statements plan `SCAN event`,
    checked with the bundled planner's text.
  - test 12: `SaysNothingWhenCut` first passed the chain rule, because
    `GroupPageBuilder::finish` already says `Unknown` for a cut page; the mutant now
    overrules the builder, and fails it.
  Deviations: `remote_sync_conformance`'s token expectation moved from "3" to "1"
  (the token now names a group's first, D-8); the SY-2 flattener and paging mutants in
  `wrong_peers.rs` now fail SY-30 as well as SY-2. Next: PR 2 (Postgres, Neon).
- 2026-11-08: review follow-up (F1–F7), on top of the merge of `origin/phase-13` (#428's
  responder stamping kept beside the group reads in `store_peer.rs`). Red runs:
  - F1 `new_refuses_a_version_1_file_until_it_is_migrated` (tests/migration.rs): E0599 no
    variant `SchemaNeedsMigration`; green after `read_identity` refuses `version <
    SCHEMA_VERSION`. Classified `Stop`; README and CHANGELOG Upgrading note corrected.
  - F6 `a_group_offered_after_a_later_one_closed_is_refused` (write_group/page.rs): E0599/E0308
    against the infallible `finish`; green with a poisoned builder and `finish -> Result<_,
    GroupOrderError>`. `MemoryIngestStore`'s `WriteGroupStore::Error` is `GroupOrderError`
    (it was the uninhabited `MemoryStoreError`, which cannot carry the refusal);
    `SqliteEventStoreError::GroupOrder` (behind `sync`, `Stop`) carries it for SQLite, pinned
    by `a_row_naming_a_group_already_passed_stops_the_group_read`.
  - F2 `a_group_after_a_redelivery_is_named_by_its_first_landed_row` (SY-39, sixth rule): the
    new rule turned the three existing whole-set pins red (`EveryRowItsOwnGroup`,
    `MergesAdjacentGroups`, `DropsGuard` now also fail it), which is the pin doing its job;
    their sets grow by the rule. New wrong store `NamesByFirstAllocated` fails exactly it.
  - F3: spec `event_store.rs` continuations at the VT-21/22/24 discharge and VT-30's marker
    repointed to `MAX_*` (342/349/358) and `check_ceilings` (767/773/760); every other
    citation into the files this branch changed re-derived by a line mapping from the merge
    base, spec, standards, runbook and `.kb/open-questions` (not `_governance`, a record).
  - F4 CHANGELOG SY-30 entry describes the three pushed groups. F5/F7 recorded above.
- 2026-11-08: PR 2 (items 34-41), Postgres and Neon, on `claude/p13-420b`. Red runs:
  - 34: `migration_3_adds_the_write_group_columns_and_their_index` and
    `apply_sends_migration_1_then_migration_3`: E0425, no `MIGRATION_3` / `EVENT_LOG`.
  - 35: `the_insert_binds_a_write_group_on_every_row`: E0432 no `ROW_PARAMETERS`, E0061
    `insert_statement` takes three arguments.
  - 39: Neon `the_write_group_migration_renders_into_three_statements`: E0432 no
    `MIGRATION_3`.
  - 37 as a mutant: `GROUPS_SQL` ordered by `position` alone fails
    `interleaved_writers_read_back_as_whole_groups_in_first_order` with
    `GroupOrder { first: 1, after: 2 }`; the conformance family alone did not catch it.
  Live: Postgres 17.10 in Docker. The lib `--ignored` run passed 39/39 (six write-group
  rules, four `groups.rs` tests); `postgres_conformance` passed 110/110; the other live
  binaries passed. The naive-arm cfg targets were not run (they need `RUSTFLAGS`) and are
  left to CI. Neon: the unit and shape tests pass. Its rendered SQL (migrations 1+3, an
  append, two ingests including a whole re-delivery, the group read) was executed by hand
  against the local Postgres: 0 negative tags left, the guard on each first row only, the
  read in group order. The live leg (`groups.rs`, the mounted family) is `live-neon`'s; the
  existing lib `--ignored` invocation already runs it, so CI is unchanged.
  Expectations moved, each implied by the design:
  - Postgres `SCHEMA_VERSION` 1 -> 2 (migration 3).
  - Both conformance files' settled column set and index list gain migration 3's two columns
    and its index (line-neutral; ADR-0066 cites the Postgres lines).
  - Neon ingest request: 2 -> 3 statements in two tests (the stamp joins the batch).
  - Neon ingest SQL: `b.recorded_at FROM` becomes `b.recorded_at, -(b.grp + 1) FROM`.
  Deviations: 37 forces the interleaving deterministically (two open transactions drawing
  `nextval` alternately) rather than racing with a barrier. Neon refuses an unencodable guard
  before sending, through `NeonError::UnstorableGuard { group }`, a variant §12 did not list.
  Citations into Neon `event_store.rs` and the Postgres `insert_statement` were repointed in
  the spec, standards, `.kb/open-questions` and the experiment README. Accepted `.kb/decisions`
  bodies were not touched; see the report.
- 2026-11-08: review of `13896304` (F1-F5). Red runs:
  - F1 `the_group_read_seeks_on_the_write_group_index` (Postgres live, 20,000 rows in groups
    of four, `ANALYZE`d): FAILED on the plan's `Filter: ((ANY (write_group = (hashed SubPlan
    4).col1)) OR ...)` over the table. Green once the outer read became two keyed joins under
    `UNION ALL` (`ids JOIN event ON write_group = g`, and `ON position = g WHERE write_group IS
    NULL`). Neon's shape test went red on the frontier count (2, now 3) and green with the same
    rewrite; the new SQL was run by hand on the local Postgres against legacy singletons and two
    interleaved groups, at four `after` values.
  - F4: Postgres streams the read (`fetch`, dropped when the builder breaks). Both adapters
    name groups from the first `max` scanned rows only: those rows' groups hold at least `max`
    events, so a group named first by the probe row never fits, and `Nothing` needs a scan
    short of the probe, which then has no probe row to leave out. Pages are unchanged.
  - F3 `interleaved_groups_read_back_whole_and_page_on_first` (Neon live): left to `live-neon`.
  - F2: the additions above cited lines moved below them (`APPEND_GROUP`, `ROW_PARAMETERS`,
    Neon `MIGRATION_3`), `insert_rows`' signature back to four arguments (the group rides in
    `origin`), and docs above cited lines rewritten line for line. Every `path:N` citation into
    the files this branch changed resolves at the line it did before `13896304`; the spec,
    standards and open-question repoints of `13896304` are reverted.
  - F5: the gutter in `standards/rust/90` is consistent again with the restored line; migration
    3's comment and the settled-column docs say what they now cover.
