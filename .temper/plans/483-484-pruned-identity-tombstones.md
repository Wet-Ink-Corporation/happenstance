# #483 + #484: a store that removed an event does not take it back (removal records)

Planned 2026-11-16 against `main` at `5f71d44f`. Status: **D-1 to D-10 and T1 to T7
answered "A" by the owner on 2026-11-17 ("all as recommended"); PR 1 in progress on
`claude/project-thread-qkv7ws-483`, cut from `origin/phase-21-prep` at `1b7c85d`.**
ADR-0110's text still needs the owner's acceptance, so it is `proposed` in PR 1.

## Goal

A store that has had events removed outside the port (a raw `DELETE`, the operator's
prune) and then re-pulls or is re-pushed those events **skips** them, counting them as
skipped, instead of:

- **#483**: re-admitting a removed *foreign* event at the tail, so a projection applies it
  twice and `runner/watermark.rs:20-24`'s "a re-read is harmless" turns out false;
- **#484**: refusing a removed *own* event as `IngestError::UnheldOwnIdentity`
  (class `Stop`), which wedges the edge for good, on the spoke side
  (`Edge::restored` pull) and on the hub side (a fresh `Edge::new` push into a
  `StoreSyncPeer` over a pruned hub).

Both are fixed for `0.8.0`, the last breaking window before 1.0, together with an ADR that
amends the frozen SY-1 and SY-38 and mints SY-40.

## Constraints and non-goals

- **No change to `EventStore`.** ES-39's refusal is frozen (ADR-0108, ADR-0028 decision 9):
  no port value reports held or unheld history. The removal record is read **only** inside
  each adapter's `IngestStore::ingest`, and it reaches callers only as the existing
  `Ingested::skipped` count (D-4).
- **No change to the `IngestStore`, `WriteGroupStore` or `SyncPeer` traits.** No new
  required method (RS-40-1). No new `IngestError` arm.
- **No change to `IngestStore::watermark`.** It stays "the highest origin position
  *held*", because the `Ack` a `StoreSyncPeer` sends is built from it and
  `ack_never_confirms_a_non_durable_event` (#242, SY-31) requires an `Ack` to confirm only
  what the far side still holds. Removing an origin's tail lowers the watermark, so the next
  pull re-reads that tail; the record turns the re-read into skips. A re-read costs a round
  trip, not a duplicate.
- **`append` does not read the record.** Detecting a rewound allocator on append (a VT-6
  violation the record could see) is a separate idea. Non-goal, and listed under Risks as a
  follow-on.
- No new dependency. MSRV unchanged. `#[async_trait]` stays out (constraint 1). `serde`
  stays out of core's defaults (constraint 2).
- No `TRUNCATE` support (D-9). No compaction of the record (D-5).
- `experiments/retention-reader/` stays a dated record (D-7).

## Invariants (each a test below)

1. **I-1, removed foreign:** after a foreign event `X` is removed outside the port,
   ingesting `X` again lands nothing. `appended == 0`, `skipped` counts it, the log is
   unchanged, `contains_event_id(X)` stays `false` (ES-38 still holds), and a group made
   only of removed and held events writes no compensation (SY-11).
2. **I-2, removed own:** after one of this store's own events `L` is removed, ingesting `L`
   is a skip. It is not a refusal, and the rest of the call lands.
3. **I-3, SY-38 survives:** an own claim the store **never** held, such as a position past
   the allocator, is still refused whole as `UnheldOwnIdentity`, even in a store that has
   recorded removals. So the record is a set of identities, not "anything unheld is fine".
4. **I-4, durable and shared:** the record survives reopen and reaches every handle, both
   those connected before the removal and those connected after it.
5. **I-5, a restore rolls the record back with the allocator:** a store restored from a
   backup holds the record as it was at the backup. A claim on an own event minted *after*
   the backup is still refused (SY-38). Removal is not restore: a removal keeps the position
   allocator, and a restore rolls it back.
6. **I-6, no race on stores whose writers do not serialise:** on Postgres and Neon, a
   removal that commits during an ingest that re-delivers the same event does not re-admit
   it (D-8).
7. **I-7, held events dedupe as before:** an event the store holds is still skipped through
   the origin pair's `UNIQUE` (VT-8). The record is consulted only for identities the store
   no longer holds, under the write lock (SQLite, Durable Object) or under a table lock that
   excludes every removal (Postgres, Neon). It is never a race-prone read-then-append.

## Design

### The mechanism: a removal record per store

Each adapter keeps one extra table, the **removal record**:

| Adapter | Table | Written by | Read by |
|---|---|---|---|
| SQLite | `event_removed (origin_store BLOB NOT NULL, origin_position INTEGER NOT NULL, PRIMARY KEY (origin_store, origin_position)) WITHOUT ROWID` | `AFTER DELETE ON event` trigger, `WHEN OLD.origin_store IS NOT NULL`, `INSERT OR IGNORE` | `ingest_locked`, inside the `BEGIN IMMEDIATE` transaction |
| Durable Object | `{ns_}event_removed`, the same shape, rendered through `Tables` | `{ns_}event_record_removal`, the same trigger shape | the ingest `transactionSync` |
| Postgres | `event_removed (origin_store bytea, origin_position bigint, PRIMARY KEY (...))` | `BEFORE DELETE ... FOR EACH ROW WHEN (OLD.origin_store IS NOT NULL)` trigger calling a `plpgsql` function, `ON CONFLICT DO NOTHING`. The function names the table through `TG_TABLE_SCHEMA` (`format('%I.event_removed', TG_TABLE_SCHEMA)`), so it is correct whatever `search_path` the deleting session has | `ingest_in_transaction`, after `LOCK TABLE event_removed IN SHARE MODE` |
| Neon | `@removed@`, the same shape, schema-qualified | the same trigger, with the qualified table name rendered into the body | the ingest request: a `LOCK` statement first, then an anti-join in the ingest statement |
| `MemoryIngestStore` | `State::removed: BTreeSet<EventId>` | new inherent `MemoryIngestStore::remove` | `stage` |

The **key is the `EventId`**, the origin pair `(origin_store, origin_position)`. It covers
foreign and own events alike, because every committed row carries a non-null origin: SQLite
and the Durable Object stamp a local row with `(self, position)` in the same transaction,
and Postgres and Neon bind it at insert. The key is also stable across a re-mint. After
`remint_identity`, the old identity is foreign and its removal records still match it.

**Why a trigger (D-1).** Both bug reports remove rows with a raw `DELETE` outside the port,
and so do `experiments/retention-reader/src/prune.rs` and every `Fixture::remove_outside_port`
(#147). A trigger is part of the schema, so it fires for every client: the `sqlite3` CLI, a
second `rusqlite` connection, `psql`. An inherent `prune` method fires only for callers who
know to use it, so on its own it would leave both reproductions exactly as they are.

**Why a removal is not a restore (the semantic core of the ADR).** VT-6's hazard is a store
that re-issues an `EventId` it already gave out. That needs a rolled-back allocator.
`DELETE` never rolls the allocator back: SQLite's and the Durable Object's `AUTOINCREMENT`
keep `sqlite_sequence`, and Postgres's and Neon's `nextval` sequences keep their value
(ES-38 is what checks this). So an identity a `DELETE` removed can never be re-minted, and a
re-delivered copy of it is the very event the store once held. Skipping it is exact. A
restore rolls back the record along with the allocator and the log (a file copy,
`pg_restore`, a Neon branch, a Durable Object bookmark), so own events minted after the
backup are absent from the restored record, and SY-38 still refuses them. That is I-5.

### Where the record is consulted, per adapter

- **`MemoryIngestStore`** (`crates/happenstance-sync/src/memory_ingest.rs`). `stage`
  checks `self.held.contains(id) || self.removed.contains(id)` **before** the SY-38 arm, and
  counts a match as `skipped`.
  - Removal needs a position allocator that does not depend on density. `State` gains
    `assigned: u64`, the highest position ever assigned, and `next_position` derives from
    it rather than from `log.len()`. Any lookup that indexes `log` by `position - 1` becomes
    a binary search on `log` (it stays sorted by position). The `group_of` and `guards`
    entries of removed rows are dropped in the same critical section.
  - The module doc's "Positions are dense" paragraph (`memory_ingest.rs:68`) is rewritten:
    dense until something is removed.
- **SQLite** (`event_store.rs::ingest_locked`). At the top of the transaction, one
  `SELECT EXISTS (SELECT 1 FROM event_removed)` is folded into the `started_at` query, so
  it costs no extra statement.
  - When the answer is true, each group's foreign events are probed with a cached prepared
    point lookup on the primary key. A matching event is dropped from `foreign` before
    `write_batch`.
  - `skipped` is computed from `group.events.len() - written.inserted`, not
    `foreign.len()`.
  - `write_batch` is unchanged, so append's write path is untouched (VT-10's one-`INSERT`
    claim stands).
  - A filtered event burns no `AUTOINCREMENT` value, and a call that lands nothing still
    rolls back (#359).
- **Durable Object** (`event_store/ingest/transaction.rs`). The same fast-path `EXISTS`
  and the same per-event probe inside `transactionSync`, through `SqlStorage::exec`. The
  probe reads rows, and rows read are metered. The `EXISTS` fast path keeps a store that has
  never removed anything at one extra row read per call.
- **Postgres** (`event_store/ingest.rs::ingest_in_transaction`):
  1. The first round trip becomes a simple-protocol batch:
     `LOCK TABLE event_removed IN SHARE MODE;` followed by `IDENTITY_AND_SNAPSHOT_SQL`,
     extended with a third column `EXISTS (SELECT 1 FROM event_removed)`. Sent with
     `sqlx::raw_sql`, because neither statement binds anything. The implementer must check
     `raw_sql(...).fetch_all` row semantics against the `sqlx` version in `Cargo.lock` before
     relying on this. If it does not return the `SELECT`'s row, the fallback is two round
     trips, recorded in Progress.
  2. If any removals exist, one probe per call:
     `SELECT origin_store, origin_position FROM event_removed WHERE (origin_store, origin_position) IN (SELECT * FROM unnest($1::bytea[], $2::bigint[]))`.
     Removed ids are filtered out of each `OwnedGroup` before `allocate_positions`, so no
     sequence value is drawn for them.
  3. The rest is unchanged: `HIDDEN_CLAIMS_SQL` and the begin snapshot still decide
     SY-38. A filtered own claim never reaches the insert, so it is neither landed nor
     hidden.
  - **Why `SHARE`:** it is the weakest mode that conflicts with the `ROW EXCLUSIVE` the
    trigger's insert takes and does not conflict with itself. So concurrent ingests do not
    serialise against each other, appends never touch the table, and a removal and an
    ingest exclude each other. That gives I-6.
  - **Why a `BEFORE` row trigger:** the deleting transaction takes the table lock at its
    first row, before that row is marked deleted. An ingest holding `SHARE` that meets the
    row under `ON CONFLICT` sees a live row and skips it. It does not wait on the deleter,
    so there is no lock cycle. An `AFTER` trigger would mark the row deleted first and then
    block on the table lock, while the ingest waits on the deleter: a deadlock (`40P01`),
    which is classified transient but is avoidable.
- **Neon** (`event_store/ingest.rs`). The request stays **one round trip** (SY-14). It
  becomes three statements: `LOCK TABLE @removed@ IN SHARE MODE`, the ingest statement,
  then the refusal statement.
  - At `REPEATABLE READ` the transaction snapshot is taken by the first statement that is
    not a utility statement. `LOCK` is a utility statement, so the snapshot is taken by the
    ingest statement, after the lock is granted, and sees every committed removal.
  - `ingest_statement`'s foreign-row source gains `WHERE NOT EXISTS (SELECT 1 FROM @removed@ r WHERE r.origin_store = v.origin_store AND r.origin_position = v.origin_position)`.
    The skip count already falls out of `RETURNING`, because a row that did not land is a
    skip.
  - Verify in `live-neon` that the `/sql` batch endpoint accepts `LOCK TABLE` inside its
    transaction. If it does not, stop and bring it back as D-8 again.

### Ingested and the docs it carries (D-4)

`Ingested::skipped`'s doc becomes "foreign events skipped because this store holds their
identity, or once held it and recorded its removal (SY-40)". No new field, because a
separate count of removals would be a port value that reports unheld history, which
ES-39's refusal rules out. The runner's `read_through_echoes`
(`runner/drive.rs:173`) keys on `skipped > 0`, and a re-pulled removed range is an echo in
exactly that sense, so the runner needs no change.

Other docs that change:

- `IngestStore::store_id` and `IngestStore::ingest`: the paragraph naming "restored or
  rewound" gains "or removed, which is a skip".
- `IngestError::UnheldOwnIdentity`: "not held **and not recorded as removed**".
- `runner/watermark.rs:20-24`: "a re-read is harmless, because ingest skips what it holds
  **or has recorded as removed** (SY-11, SY-40)".

### Types and signatures (ownership decided)

```rust
// happenstance-sync, behind `memory`: the oracle's "outside the port".
impl MemoryIngestStore {
    /// Removes the events at `positions` (unheld positions ignored), records their
    /// identities as removed, and returns how many it removed. Never reuses a position.
    pub fn remove(&self, positions: &[SequencePosition]) -> usize;
}
```

- `&self`, because the store is shared behind `Arc` by every handle. Interior mutability is
  the existing `RwLock<State>`, and no new sharing is introduced (OWN-11).
- `&[SequencePosition]`, borrowed: read-only, and the caller keeps it (OWN-01).
- Returns `usize`: a count, owned, and infallible like the rest of the oracle. The existing
  lock helper's poisoning policy applies.
- Positions rather than ids, to match `Fixture::remove_outside_port`.

```rust
// happenstance-sync-testkit, src/ingest_fixture.rs: defaulted items, additive (RS-40-5).
pub trait IngestFixture {
    // ...existing items...
    /// Whether this fixture can remove events from its store outside the port, as an
    /// operator's `DELETE` would. Declined by default.
    const REMOVAL: Capability = Capability::declined(UNSTATED_INGEST_REMOVAL);

    /// Removes the events at `positions` outside every port. The contract is
    /// `happenstance_testkit::Fixture::remove_outside_port`'s, restated here: held
    /// positions only, whole write groups, the removal reaches open handles, and it never
    /// rewinds the allocator or mints a new incarnation.
    fn remove_outside_port(
        &self,
        positions: &[SequencePosition],
    ) -> impl Future<Output = happenstance_testkit::Removed> {
        let _ = (self, positions);
        core::future::ready(happenstance_testkit::Removed::Nothing)
    }
}

pub const UNSTATED_INGEST_REMOVAL: &str = "...";
```

- `Removed` is reused from `happenstance-testkit` rather than redefined, so there is one
  answer type for one question.
- `ingest_declensions_are_stated` does **not** check `REMOVAL`, for the reason
  `contract.rs:1671` gives for `Fixture::REMOVAL`: silence misrepresents nothing. A fixture
  that claims the capability and forgets the override is failed by the rule, because the
  rule asserts `Removed::Deleted` first.
- `MemoryIngestFixture` grants it and overrides through `MemoryIngestStore::remove`.

Adapter schema items (D-3, D-6):

- **SQLite:** `pub const SCHEMA_VERSION: u32 = 3`, a private `MIGRATION_3` (table and
  trigger) applied by `migrate` through `removal::migrate_to_3`, the module-doc schema block
  extended, and `tests/migration.rs` updated.
- **Postgres:** `migrations/0004_removal_record.sql`, `pub const MIGRATION_4`, `EVENT_LOG`
  concatenates it, `SCHEMA_VERSION = 3`, and `apply` covers it.
- **Neon:** the `plpgsql` body cannot pass `migration.rs`'s splitter (no dollar quotes, by
  its own rule). So `pub const MIGRATION_4: &str` holds the table DDL only, and
  `pub fn removal_trigger_statements(config: &NeonConfig) -> [SqlStatement; 2]` (the
  function, and `DROP TRIGGER IF EXISTS` plus `CREATE TRIGGER`) is a statement-per-constant
  builder, which is the "honest move" that module's doc names. `request` and the fixture
  apply both.
- **Cloudflare:** `migration(tables)` grows from `[String; 4]` to `[String; 6]` (table,
  trigger), with `Tables` gaining `event_removed` and `event_record_removal`. No marker,
  following #420's D-1 precedent: idempotent `IF NOT EXISTS`, and an older build ignores the
  table.

### Errors

None new. Driver failures of the probe, the lock and the migration map to each adapter's
existing `Store`/driver variants and their existing classes:

- a Postgres `40P01` or `55P03` is transient;
- a missing table (the migration was not applied) is a driver error, class `Stop`, as a
  missing `write_group` column is today.

There is no `IngestError` arm for removal: a removal is not a failure.

### Concurrency and resources

- SQLite and the Durable Object serialise their writers, so the probe runs under the write
  lock and cannot race.
- On Postgres and Neon, see I-6 and D-8. An ingest waits for an in-flight removal to commit
  and a removal waits for an in-flight ingest. Removals are rare operator actions, and the
  lock is per schema, on one small table.
  - `SHARE` also conflicts with `SHARE UPDATE EXCLUSIVE` (vacuum). Autovacuum yields to a
    blocked lock request, except in anti-wraparound mode. Noted, not mitigated.
- **H-13 (unbounded growth).** The record gains one row per event ever removed (about 30
  bytes plus its index entry), bounded by the number of events the store ever held. A
  removal still frees the payload, so the store shrinks net. Kept for ever by default (D-5).
  Each adapter documents that deleting a record row re-opens resurrection for that identity,
  and that the only safe time to do so is once no peer can serve the identity again.

### Placement (ARC-02)

| Crate | Change | Dependency direction |
|---|---|---|
| `happenstance-core` | **nothing** | unchanged: depends on nothing in the workspace |
| `happenstance-sync` | `memory_ingest` record + `remove`; docs on `Ingested`, `IngestStore`, `IngestError`, `runner/watermark.rs` | unchanged |
| `happenstance-sync-testkit` | `IngestFixture::REMOVAL` + `remove_outside_port`; rule `a_removed_event_is_not_readmitted` in the ingest family; wrong stores | unchanged (already depends on `happenstance-testkit` for `Capability`, and `Removed` lives there too) |
| `happenstance-sqlite` | migration 3, trigger, ingest filter, ingest fixture grants `REMOVAL`, end-to-end regression tests | unchanged |
| `happenstance-cloudflare` | table + trigger in `migrate`, ingest filter, fixture `REMOVAL` | unchanged |
| `harness/workerd` | `WorkerdIngestFixture` grants `REMOVAL` | unchanged |
| `happenstance-postgres` | migration 4, trigger, lock + probe, faithful restore fixture | unchanged |
| `happenstance-neon` | migration 4 + trigger statements, lock + anti-join | unchanged |
| `spec/`, `.kb/`, `docs/`, CHANGELOGs, READMEs | ADR-0110, SY-40, SY-1/SY-38 amendments, ES-39 pointer, topology doc | n/a |

No adapter depends on another. No new crate.

### Alternatives considered

- **An ever-held identity ledger** (insert every identity on append and ingest, and never
  delete it on removal). Race-free by a single unique index and retroactive for nothing.
  It lost because it adds a second index write to **every append** in four adapters (the
  hot path), needs a backfill over the whole log on migration, and makes the ledger as
  large as the log's identity index for ever.
- **The allocator high-water mark for own events** (D-2). An unheld own claim at or below
  the allocator was removed and is skipped; one above it means a restore and is refused. It
  needs no record and is retroactive. It lost as the primary cure because it does not cover
  foreign events (#483), so it would be a second mechanism beside the record, and it
  re-plumbs SY-38's detection in four adapters (SQLite's `started_at` head, Postgres's
  `HIDDEN_CLAIMS_SQL`, Neon's refusal statement, the Durable Object's).
- **A persisted pull token** (the other cure #483 names). It saves the re-read of a
  confirmed range, but `Edge::restored` exists precisely for the case where no token
  survived, and a token does nothing for #484's push side.
- **A provided `EventStore` method reporting completeness.** Refused by ES-39 (ADR-0108).
  The retention-reader found these harms fail C4.
- **Soft delete** (keep the row, hide it from reads). ES-38 and ES-40 forbid it:
  `RememberingConditionFixture` is the named wrong implementation.
- **A Postgres `RULE ... DO ALSO INSERT`** instead of a trigger, which has no `plpgsql` body
  and so would pass Neon's splitter. Rejected: rules rewrite queries rather than fire per
  row, and Postgres's own docs steer away from them.

## Test list (drive in this order; TST-01)

Each line names the red it must show first. "Wrong store X" means a store in
`crates/happenstance-sync-testkit/tests/wrong_ingest_stores.rs` that the rule must fail.

### PR 1: ADR, clause, rule, oracle (`happenstance-sync`, `happenstance-sync-testkit`, `spec/`, `.kb/`)

- [x] `memory_ingest::tests::remove_takes_the_named_events_out_and_never_reuses_their_positions`.
      Red: no `remove`. Then append 3, remove the middle one, append 1: the new position is
      above all three (ES-38 on the oracle).
- [x] `..::remove_ignores_a_position_it_does_not_hold_and_reports_the_count`.
- [x] `..::a_removed_foreign_event_is_skipped_on_redelivery`. Red: `appended == 1` (#483).
- [x] `..::a_removed_own_event_is_skipped_not_refused`. Red: `UnheldOwnIdentity` (#484).
- [x] `..::an_own_claim_never_held_is_refused_after_a_removal` (I-3). Green from the start
      if the removed set is checked by id. This guards the next refactor.
- [x] `..::a_group_of_removed_and_held_events_writes_no_compensation` (SY-11).
- [x] `..::a_group_with_one_new_event_lands_it_and_compensates_once`.
- [x] `..::the_group_read_drops_a_removed_group` (SY-39 bookkeeping after removal).
- [x] sync-testkit `ingest_fixture::tests`: a silent fixture declines `REMOVAL` with
      `UNSTATED_INGEST_REMOVAL`, and its `remove_outside_port` answers `Removed::Nothing`.
- [x] Rule `ingest_rules::a_removed_event_is_not_readmitted`, green on
      `MemoryIngestFixture`. Four arrangements on three fresh fixtures; (c) runs on (b)'s store after its removal:
      (a) removed foreign: ingest `X1..X3`, remove `X2` by position, re-ingest the same
      groups through the handle held before the removal **and** a freshly connected one;
      `appended == 0`, `skipped == 3`, the log is unchanged, and `X2` is unheld;
      (b) removed own: append `L1, L2`, remove `L1`, then ingest the echo `[L1]` plus a new
      foreign `Y`; `Ok`, `Y` lands, `L1` is skipped and unheld;
      (c) SY-38 still fires: after (b)'s removal, a claim three past the head is refused
      whole and writes nothing;
      (d) compensation: a group of only removed events carrying compensation writes none.
      Positions are compared against what the store assigned, never against literals
      (CLAUDE.md).
- [x] Wrong store `Resurrects` (ignores the record for foreign ids, today's adapters' behaviour)
      fails (a).
- [x] Wrong store `RefusesARemovedOwnIdentity` (today's behaviour on own ids) fails (b).
- [x] Wrong store `SkipsEveryUnheldOwnIdentity` (once the record is non-empty, treats any
      unheld own claim as removed) fails (c), and fails SY-40 alone: with no removal it
      refuses as SY-38 says, so the existing `an_unheld_own_identity_stops_the_ingest` passes it.
- [x] Wrong store `RecordsRemovalsPerHandle` (the record lives on the handle, not the store)
      fails (a)'s fresh-handle half.
- [x] Wrong store `CompensatesARemovedGroup` fails (d).
- [x] A fixture that declines `REMOVAL` gets the rule reported as declined with its reason,
      not vanished (existing `require!` behaviour; one test).
- [x] `happenstance-sync/tests/runner`: `a_restored_edge_over_a_pruned_memory_store_applies_nothing_twice`.
      The experiment's R2-dst-1 on the oracle: `Edge::restored` + `MemorySyncPeer` +
      `MemoryProjectionStore`, with a counter projection applying each id once.
- [x] `..::a_restored_edge_over_a_memory_store_missing_an_own_event_syncs`. R2-dst-2 on the
      oracle: not `Stopped`, and the foreign event in the same page lands.
- [x] `cargo xtask spec-trace` green with SY-40 and the amended SY-1 and SY-38.
      `cargo xtask lint-kb` green: ADR-0110 is new, and the accepted 0098, 0107 and 0108
      are untouched.

### PR 2: SQLite (`happenstance-sqlite`)

- [ ] `tests/migration.rs::a_fresh_file_is_at_schema_version_three`. Red: 2.
- [ ] `..::a_version_two_file_migrates_to_three_on_open_and_keeps_its_rows`.
- [ ] `..::module_doc_schema_matches_sqlite_master`, extended to the table and the trigger.
- [ ] `event_store` unit: `a_raw_delete_records_each_removed_identity`, covering own and
      foreign rows, deleted from a second `rusqlite` connection.
- [ ] `..::a_rolled_back_delete_records_nothing`.
- [ ] `ingest_spike` units mirroring PR 1's five oracle behaviours on SQLite. The reds are
      `appended == 1` and `UnheldOwnIdentity`.
- [ ] `..::a_skip_against_a_removal_burns_no_position_and_leaves_the_file_unchanged` (#359).
- [ ] `plan_tests::the_removal_probe_seeks_the_primary_key` (`EXPLAIN QUERY PLAN`, the
      `WATERMARK_SQL` precedent).
- [ ] `SqliteIngestFixture` grants `REMOVAL` (a raw `DELETE` on the file, maintaining
      `event_tag` and `tag_cardinality` as `tests/support/mod.rs:304` does). The rule
      passes under the mounted `ingest_store_conformance!`.
- [ ] `tests/pruned_replica.rs` (new; `#![cfg(feature = "sync")]`): the three experiment
      scenarios as regression tests, ported from
      `experiments/retention-reader/tests/it/r2_ingest_destination.rs`, now asserting the
      fixed behaviour, with the control kept:
      `a_restored_edge_does_not_resurrect_pruned_foreign_events`,
      `a_restored_edge_skips_a_pruned_own_event`,
      `a_fresh_edge_pushing_into_a_pruned_hub_is_a_no_op`,
      `control_a_fresh_edge_pushing_into_an_unpruned_hub_is_a_no_op`.
- [ ] The existing `an_unheld_own_identity_stops_the_ingest_and_writes_nothing` and
      `a_claim_lands_as_foreign_after_the_store_is_reminted` stay green, unchanged.

### PR 3: Durable Object (`happenstance-cloudflare`, `harness/workerd`)

- [ ] `event_store::tests::migrate_creates_the_removal_record_idempotently`, for the
      unprefixed log and a namespaced one.
- [ ] `..::a_raw_delete_records_the_identity` through `SqlStorage::exec`.
- [ ] Ingest units: the five behaviours, as in PR 2.
- [ ] `..::a_restored_bookmark_rolls_the_record_back` (I-5): remove, bookmark-restore to
      before the removal and the later appends, re-mint skipped. A claim on an own event
      minted after the bookmark is refused (SY-38), not skipped.
- [ ] `CloudflareIngestFixture` and `WorkerdIngestFixture` grant `REMOVAL`. The rule is
      green on the host shim in the gate and under `workerd` (local, cold, deployed) in the
      `workerd` job.

### PR 4: Postgres and Neon (`live-postgres`, `live-neon`)

- [ ] Postgres `migration::tests`: `MIGRATION_4` is idempotent, `SCHEMA_VERSION == 3`, and
      `EVENT_LOG` contains it.
- [ ] Postgres live: `a_raw_delete_records_the_identity_whatever_the_search_path` (the
      deleting session's `search_path` does not name the schema).
- [ ] Postgres ingest units (`ingest/tests/`): the five behaviours; reds as in PR 2.
- [ ] `..::a_removal_waits_for_an_ingest_in_flight`. Deterministic with no sleep (TST-10):
      transaction A takes the ingest's `SHARE` lock, and transaction B sets
      `lock_timeout = '50ms'` and `DELETE`s, failing with `55P03`.
- [ ] `..::an_ingest_waits_for_a_removal_in_flight`: the mirror case.
- [ ] `PostgresRestoreFixture::restore` rolls `event_removed` back to the backup (D-7). The
      restore family stays green. Without this change it goes red, because the post-backup
      own events read as removed after the re-mint and are skipped rather than landing as
      foreign events, which SY-38's remedy sentence requires.
- [ ] `PostgresIngestFixture` grants `REMOVAL`; the rule is green.
- [ ] Neon scripted-transport units: the ingest request is one round trip whose first
      statement is the `LOCK`, and whose ingest statement carries the anti-join. The
      migration request carries the trigger statements unsplit.
- [ ] Neon live: the five behaviours, and the rule green with `NeonIngestFixture` granting
      `REMOVAL`.
- [ ] Final step, in PR 4 or a one-line follow-up: SY-40 `[PROVISIONAL]` becomes `[FROZEN]`
      if D-10 says it freezes on four green adapters.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|---|
| D-1 | How a removal gets recorded when it happens outside the port | **A** a trigger on the events table in all four adapters; **B** an inherent `prune`/`forget` method per adapter that writes the record, with a raw `DELETE` documented as voiding the guarantee; **C** both | **A.** It is the only option that fixes the two reproductions as filed (raw `DELETE`), it fires for every client, and it adds no public method to four published crates. **B** alone leaves #483/#484 open for anyone using SQL. A `prune` method that also maintains `event_tag` and `tag_cardinality` is worth having, but as its own issue, on top of A | owner | A, 2026-11-17 |
| D-2 | The cure for own events (#484) | **A** the same record as foreign events; **B** compare an unheld own claim with the allocator's high-water mark (skip if at or below, refuse if above); **C** both (A going forward, plus B for removals made before the migration) | **A.** One mechanism, one rule, one sentence in SY-38. B is retroactive and needs no record, but it re-plumbs SY-38's detection in four adapters and still leaves #483 needing A. Removals made before the upgrade are documented, with the `INSERT` an operator runs to record them by hand | owner | A, 2026-11-17 |
| D-3 | Schema migrations | SQLite: **A** `SCHEMA_VERSION` 2→3, applied on open, and a build that predates it refuses a v3 file (the #420 precedent) vs B no bump. Postgres: **A** `0004_removal_record.sql`, `SCHEMA_VERSION` 2→3, applied by the operator before deploying, and ingest fails without it (the `write_group` precedent) vs B ingest tolerates a missing table and degrades. Neon: as Postgres, with the trigger as statement constants because the splitter forbids `plpgsql` bodies. Cloudflare: idempotent in `migrate()`, no marker (#420's D-1) | **A** for each. A degraded ingest (B) is a silent version of the bug this fixes. CHANGELOG and README upgrade steps state the order: apply the migration, then deploy | owner | A, 2026-11-17 |
| D-4 | How a removal-skip is reported | **A** count it in `Ingested::skipped` and amend the doc; **B** a new `Ingested::removed` field (additive, `#[non_exhaustive]`) | **A.** ES-39 (frozen) says no port value reports held or unheld history, and a `removed` count is such a value. The runner's echo logic already keys on `skipped` | owner | A, 2026-11-17 |
| D-5 | Growth of the record (H-13) | **A** keep for ever, document the cost and the consequence of deleting a row; **B** range-compress per origin; **C** expire below a peer's retention floor (SY-32) | **A** for 0.8.0. B and C are additive later, and C needs a real compacting peer, which SY-32's risk acceptance says does not exist yet | owner | A, 2026-11-17 |
| D-6 | New public items | `IngestFixture::REMOVAL`, `IngestFixture::remove_outside_port`, `UNSTATED_INGEST_REMOVAL` (sync-testkit: a new rule is a minor, and consumers pin exactly); `MemoryIngestStore::remove`; Postgres `MIGRATION_4`; Neon `MIGRATION_4` + `removal_trigger_statements`; SQLite and Postgres `SCHEMA_VERSION` values change | **Accept all.** Every one is additive except the two `SCHEMA_VERSION` values, which is why this rides in 0.8.0. No port trait changes | owner | A, 2026-11-17 |
| D-7 | Tests whose expectation changes | (i) `PostgresRestoreFixture::restore` must also roll `event_removed` back. That changes a fixture's behaviour, not an assertion, and makes the simulated restore faithful to a real `pg_restore`. (ii) `experiments/retention-reader`'s `a_restored_edge_resurrects_pruned_foreign_events`, `a_restored_edge_stops_on_a_pruned_own_event` and `a_fresh_edge_pushing_into_a_pruned_hub_stops` assert the bugs and go red on the fixed tree: **A** leave the experiment as a record dated to `1cf9d418` and append a README addendum naming the fixing commits and the regression tests that replace it; **B** flip its assertions | (i) **yes**. (ii) **A**: the experiment is a dated measurement outside the gate, and its criteria and verdict stay true for the tree they ran on | owner | A, 2026-11-17 |
| D-8 | Postgres and Neon: a removal committing during an ingest of the same event | **A** each ingest takes `LOCK TABLE event_removed IN SHARE MODE` first (race-free; an ingest and a removal wait for each other; free in round trips on Neon; on Postgres folded into the first round trip via `raw_sql`); **B** no lock, and document the residual window | **A.** Every other guarantee in this crate family closes its race (VT-8's Rejects line is a race). A removal is a rare operator action, so the wait costs nothing in practice | owner | A, 2026-11-17 |
| D-9 | `TRUNCATE event` (Postgres and Neon) fires no row trigger | **A** document that it voids the record; **B** a `BEFORE TRUNCATE` statement trigger that refuses it | **A.** Refusing an operator's `TRUNCATE` is a behaviour change outside this issue. `TRUNCATE ... RESTART IDENTITY` is already outside ES-38 | owner | A, 2026-11-17 |
| D-10 | The ADR and the frozen clauses | ADR-0110 amends SY-1 (a second exception), SY-38 (its wording), points SY-11 and ES-39 at the new clause, and mints SY-40 | The owner approved the fix with an ADR. ADR-0110 still needs the owner's **yes on its text** before the amendments take effect (the ADR-0098 precedent). Recommend SY-40 enters `[PROVISIONAL]` in PR 1 and freezes in PR 4 on four green adapters, so the freeze rests on the far end of the axis (Postgres, Neon) and not only on SQLite | owner | A, 2026-11-17 |

## ADR outline: ADR-0110 "A store that removed an event records it, and ingest skips it"

The atom goes in `.kb/decisions/0110-a-removed-event-is-not-readmitted.md`. Copy
0098's frontmatter shape: `status: proposed`; `depends_on` 0098, 0107, 0108, 0028;
`related` 0105 and 0073. Add it to the map under `.kb/maps/`. Write no long form unless the
owner wants one.

1. **Context.** The retention-reader (#156, PR #482), findings F1 and F2. The reproductions
   are named by test. Why ES-39's method cannot cure them (C4): the dedupe and the SY-38
   check live inside `IngestStore::ingest`.
2. **The clauses this amends, quoted as they stand.** SY-1's first MUST ("MUST append every
   event it receives that it does not already hold"), SY-38's title and its "restored or
   rewound" sentence, and SY-11 (pointer only). Say plainly that skipping a removed event
   breaks SY-1's words, as ADR-0098 said of its own exception.
3. **Options.** Trigger-written record (chosen), inherent prune method, ever-held ledger,
   allocator high-water for own events, persisted pull token, completeness report (refused
   by ES-39), soft delete (refused by ES-38/ES-40).
4. **Decision.**
   1. Every store that implements ingest keeps a durable record of the identities removed
      from it by any means that keeps its allocator.
   2. Ingest treats a recorded identity as held: skipped, counted in `skipped`, no
      compensation for a group of nothing else, never refused under SY-38.
   3. The record is consulted under the write lock, or under a lock that excludes every
      removal. Held events still dedupe on the origin pair's uniqueness (VT-8 is not
      reopened).
   4. A restore rolls the record back with the allocator.
   5. The record is never exposed through a port value (ES-39 stands).
5. **Why a removal is not a restore.** The allocator argument above, with ES-38 as the
   clause that guarantees `DELETE` keeps the allocator.
6. **What acceptance changes (edits to frozen text):**
   - SY-1 gains: "SY-40 is the second: an event whose identity the store has recorded as
     removed is skipped as held."
   - SY-38's title becomes: "...which the receiver neither held when the ingest began nor
     has recorded as removed (SY-40)...". Its body sentence "so the receiver was restored or
     rewound" gains: "A removal keeps the allocator and is recorded, so it is a skip, not
     this refusal."
   - SY-11 gains a pointer to SY-40.
   - ES-39's note marks #483 and #484 resolved by ADR-0110.
   - SY-40 is new (text below).
7. **SY-40 text (proposed).** "A store that implements ingest MUST durably record the
   `EventId` of every event removed from it by any means that leaves its position allocator
   where it was, and ingest MUST treat an event whose identity it has recorded as removed as
   one it holds (SY-11): skipped, never appended, never refused under SY-38. The record
   MUST reach every handle and survive reopening, and a restore that rolls the allocator
   back MUST roll the record back with it."
   - Rule: `a_removed_event_is_not_readmitted` (`happenstance-sync-testkit`).
   - Cases: E2E-46, E2E-42.
   - Rejects: an adapter that dedupes only against what it holds, the shape all four
     adapters shipped in 0.7.0, and the projection that then applies a removed event twice.
8. **Costs.** One table per store, growing by one row per removal (H-13). A
   `SHARE`/`ROW EXCLUSIVE` exclusion on Postgres and Neon. Schema bumps for SQLite and
   Postgres. Removals made before the migration are not recorded, and the manual remedy is
   published.
9. **Falsifier, or what reopens this.** A deployment in which a removed event must come back
   on re-delivery (re-hydrating a pruned replica from its hub). The remedy then is an
   explicit operation that clears the record for a range, not a change to ingest. Also: an
   adapter that cannot keep its allocator across a removal.
10. **Freeze condition.** SY-40 `[PROVISIONAL]` until the rule is green on SQLite, the
    Durable Object (host and `workerd`), Postgres and Neon. Then `[FROZEN]` by this record.

## PR split

1. **PR 1: decision, rule, oracle.** ADR-0110 (proposed), SY-40 `[PROVISIONAL]` and the
   SY-1/SY-38/SY-11/ES-39 edits (these land only after the owner's yes on the ADR),
   `MemoryIngestStore` record and `remove`, the `IngestFixture` capability, the rule and
   its five wrong stores, the runner tests on the oracle, and the docs in `happenstance-sync`.
   - Adapters' mounted suites report the rule as declined until their PR, so the gate stays
     green.
   - About 1,000 lines over 12 files.
2. **PR 2: SQLite.** Migration 3, trigger, ingest filter, fixture `REMOVAL`, the four
   end-to-end regression tests, the experiment README addendum, and a paragraph in
   `docs/choose-a-topology.md` on pruning a replica.
   - Closes the reported reproductions (both were SQLite).
   - About 650 lines over 7 files.
3. **PR 3: Durable Object.** `migrate` additions, ingest filter, the host and `workerd`
   fixtures, and the bookmark rollback test.
   - About 400 lines over 6 files.
4. **PR 4: Postgres and Neon.** Migration 4 in each crate, triggers, lock + probe and lock +
   anti-join, the faithful restore fixture, the lock tests, both fixtures, CHANGELOG upgrade
   steps, and the SY-40 freeze.
   - Run under `live-postgres` and `live-neon`.
   - About 800 lines over 12 files.

PR 2 and PR 3 can go in parallel after PR 1. PR 4 needs PR 1. Each says
`Closes`/`Refs #483 #484`, and only the last says `Closes`.

## Risks

- **Neon's `/sql` batch may refuse `LOCK TABLE`.** Caught by the first live-neon run in
  PR 4. If it does, stop and bring D-8 back to the owner with the measured answer.
- **`sqlx::raw_sql` may not return the `SELECT` row after the `LOCK`.** Caught by the
  Postgres ingest units. The fallback is two round trips, recorded in Progress.
- **The Durable Object platform's SQLite and triggers.** The `workerd` harness already
  creates triggers through the object's SQL (`harness/workerd/src/fixture.rs:189`), so it is
  supported. A trigger that writes to another table is checked by the PR 3 `workerd` run.
- **A test that simulates a restore with `DELETE`** reads, after this change, as a removal.
  Found so far: `PostgresRestoreFixture` (fixed by D-7(i)). `tests/remint.rs` in Postgres and
  Neon rewind with `DELETE` + `setval` but assert only on `append`, which does not read the
  record, so they are unaffected. A grep for `DELETE FROM event` across `crates/` and
  `harness/` is a PR 4 checklist item.
- **Write groups partly removed.** The fixture contract removes whole groups. A partial
  removal is outside ES-38's description and is documented, not tested.
- **Unstamped legacy rows** (SQLite `UnstampedEvent`): the trigger skips a `NULL` origin, so
  such a row records nothing when removed. No peer can hold an identity for it either.
- **Follow-on, not in scope:** append could refuse to mint an identity that is in the record
  (a rewound allocator, the VT-6 hazard made detectable). File it as an issue after PR 4.

## Size (lower bound)

About **2,850 lines over about 37 files**: roughly 1,700 of Rust (about 650 of it production,
the rest tests and wrong stores), 150 of SQL, and 550 of spec, ADR and docs. Most of it is
four adapters each repeating the same five-behaviour test set, plus the conformance rule and
its five wrong stores.

## Progress

- 2026-11-16: plan written. Waiting on D-1 to D-10.
- 2026-11-17: PR 1 resumed from WIP `70017a6` after a container restart. Reds recorded by
  temporarily reverting the implementation, each restored before the next step:
  - `memory_ingest::removal::tests`, with `stage` checking `held` only and `next_position`
    derived from `log.len()`: `a_removed_foreign_event_is_skipped_on_redelivery` failed
    `left: (1, 2, 0) right: (0, 3, 0)` (#483); `a_removed_own_event_is_skipped_not_refused`
    failed `UnheldOwnIdentity { id: ..:1 }` (#484);
    `a_group_of_removed_and_held_events_writes_no_compensation` `(1, 1, 1)` vs `(0, 2, 0)`;
    `a_group_with_one_new_event_lands_it_and_compensates_once` `(2, 0, 1)` vs `(1, 1, 1)`;
    `remove_takes_the_named_events_out_and_never_reuses_their_positions` at its `held ==
    [first, last, after]` assertion (position reused). Green on restore (25 passed).
  - The rule against each wrong store, `a_removed_event_is_not_readmitted`:
    `Resurrects` (a) counted `(1, 2, 0)`, should be `(0, 3, 0)`; `RefusesARemovedOwnIdentity`
    refused the echo of `..:1` with `UnheldOwnIdentity`; `SkipsEveryUnheldOwnIdentity` answered
    the never-minted claim `Ok(Ingested { appended: 1, skipped: 1, .. })`;
    `RecordsRemovalsPerHandle` counted `(1, 2, 0)` through the handle connected after the
    removal; `CompensatesARemovedGroup` counted `(0, 2, 1)`, should be `(0, 2, 0)`. Also red:
    `DropsTheClaim` (c), `RecompensatesOnRedelivery` (d) and `RefusesEveryOwnIdentity` (b),
    whose tests now list SY-40 beside their own rule.
  - `tests/runner/pruned.rs`, with the oracle's record check reverted:
    `a_restored_edge_over_a_pruned_memory_store_applies_nothing_twice` failed `a pruned event
    was re-admitted, left: 2 right: 0`; `a_restored_edge_over_a_memory_store_missing_an_own_event_syncs`
    failed `Ingest(UnheldOwnIdentity { .., position: 1 })`. Green on restore (78 passed).
  - Decided without the owner, in the WIP's favour: `SkipsEveryUnheldOwnIdentity` skips only
    once the record is non-empty (the `EXISTS` fast-path shape), so it fails SY-40 and not
    the existing SY-38 rule, which the test list expected; the unconditional version is
    `DropsTheClaim`, already registered. `RefusesEveryOwnIdentity` is left as it was (the WIP
    had narrowed it to held echoes) and its test lists SY-40 as a second failing rule.
  - The restored edge's watermark is built as `Watermark::new(FAR, confirmed)`, because a
    `MemorySyncPeer` names no responder and `Edge::watermark` is then `None`.
- 2026-11-17, review fixes. F1: `remove` now keeps a guard only while its group's opening row
  survives, since `read_groups` reads it from that row alone. Red first:
  `removing_a_groups_first_row_drops_its_guard` failed `a guard nothing can read is still kept,
  left: [1, 3] right: [3]` (positions as the store assigned them); green after. F5: `doomed`
  computed once. F7: the pruned re-pull asserts `skipped == 4` exactly. F3, F4: SY-40's Rule
  paragraph says (c) shares (b)'s store and that reopen and restore are each adapter's
  regression tests (I-4, I-5) until the ingest family has `REOPEN`.
  F6: the `SkipsEveryUnheldOwnIdentity` deviation from this list (it fails SY-40 alone, not
  also the SY-38 rule) goes to the owner for confirmation on the PR.

