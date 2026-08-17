---
item: "HS-S0036"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Migration 1: the amended schema, persisted StoreId, and idempotent concurrent open

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author holding ADR-0022 and needing a schema whose append-condition probe does not walk a join under the write lock, WHEN `SqliteEventStore::open` is called on a path that has never been migrated, THEN the objects SQLite reports through `sqlite_master` are: `event` with `position INTEGER PRIMARY KEY AUTOINCREMENT` plus columns for type, payload, metadata, canonical tags, the origin pair and `recorded_at`; `event_tag` `WITHOUT ROWID` keyed `(tag, position)` in that order with `event_type` carried as a non-key covering column; `tag_cardinality`; one `UNIQUE` constraint over `(origin_store, origin_position)` together; and a schema-version marker — AND `EXPLAIN QUERY PLAN` for a tag-plus-type probe names no access to `event`."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:173-199 (MIGRATION_1) and :392-414 (migrate). Test `crates/happenstance-sqlite/tests/migration.rs::migration_creates_the_amended_schema` PASSED: sqlite_master read back out of a real file as exactly {table event, table event_tag, table store_meta, table tag_cardinality, index event_type_idx}; AUTOINCREMENT asserted from the stored DDL; the event_tag key asserted as (tag, 1), (position, 2) with event_type at pk = 0 through pragma_table_info (tests/migration.rs:242-267); exactly one UNIQUE index over (origin_store, origin_position) together (tests/migration.rs:208-240); schema_version = 1; and EXPLAIN QUERY PLAN for a tag-plus-type probe naming no access to `event` (tests/migration.rs:276-297)"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::migration_creates_the_amended_schema"

- id: AC-002
  criterion: "GIVEN the adapter author whose fixture will call `connect()` more than once and whose `open` therefore runs `migrate` on every connect, WHEN `SqliteEventStore::open` is called a second time on an already-migrated path, THEN it returns `Ok`, the set of objects in `sqlite_master` is unchanged from the first open, exactly one identity row exists, and the second handle's `store_id()` equals the first handle's."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:392-414 — one BEGIN IMMEDIATE, `CREATE ... IF NOT EXISTS` throughout, `INSERT OR IGNORE` then read back. Test `crates/happenstance-sqlite/tests/migration.rs::second_open_of_a_migrated_file_changes_nothing` PASSED: the sqlite_master object set is identical before and after the second open, exactly one store_id row exists, and both handles report the same StoreId"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::second_open_of_a_migrated_file_changes_nothing"

- id: AC-003
  criterion: "GIVEN an adapter author whose CI opens two handles onto one file and cannot control which wins, WHEN N threads (N >= 2, and at least the handle count the slice's fixture will use) call `SqliteEventStore::open` on one fresh path simultaneously, THEN every call returns `Ok`, every handle reports the same `store_id()`, and the file afterwards holds exactly one identity row and one copy of each schema object — the loser of the insert race having adopted the winner's value rather than kept its own."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:405-412 — `INSERT OR IGNORE` then `read_identity` INSIDE the same transaction, which is the half that makes the loser adopt the winner's value; plus crates/happenstance-sqlite/src/connection.rs:131-156 (`ensure_wal`), a real finding from this run: with the busy timeout installed first, seven of eight concurrent opens of a FRESH file still failed instantly, because converting to WAL is a lock promotion and SQLite deliberately does not invoke a busy handler for one. Test `crates/happenstance-sqlite/tests/migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id` PASSED: eight threads rendezvous on a std::sync::Barrier, every open returns Ok, the observed StoreId set has exactly one element, and the file afterwards holds one identity row and five schema objects"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id"

- id: AC-004
  criterion: "GIVEN the application author whose fear is a second, independent reader building a wrong answer from a log whose identities moved under it, WHEN a store is opened on a fresh path, its `store_id()` recorded, the handle dropped, and the same path re-opened into a new `SqliteEventStore`, THEN `store_id()` is byte-identical to the recorded value, `StoreId::to_bytes` round-trips it, and the persisted form is bytes or text — never an integer and never a pair of integers."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:301-303 (`store_id()`, the accessor in MemoryEventStore's shape) and :416-445 (`read_identity`, which refuses to invent one). Test `crates/happenstance-sqlite/tests/migration.rs::store_id_survives_a_close_and_reopen` PASSED: byte-identical across a close and reopen, `StoreId::to_bytes` round-trips it, and the storage form is asserted from SQLite's own `typeof(v)` and `length(v)` as blob-or-text and sixteen bytes — never an integer and never a pair of integers"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::store_id_survives_a_close_and_reopen"

- id: AC-005
  criterion: "GIVEN an operator who restored this file from Friday's backup — a state a SQLite adapter cannot detect, which is why VT-6 grants mint-once only against a documented procedure — WHEN they invoke the adapter's public re-mint operation and re-open, THEN the persisted identity has been replaced by a fresh 128-bit value different from the old one, the re-opened handle reports the new one, the operation's rustdoc states in prose when a deployment invokes it (after a restore or a clone) with an `# Errors` section naming conditions rather than types, and `references/adapter-shapes.md` records mint-once-with-explicit-re-mint as this adapter's chosen VT-6 mechanism."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:319-374 — `remint_identity`, whose rustdoc carries a `# When a deployment invokes this` section stating in prose that it is run after restoring this file from a backup or after copying it, and an `# Errors` section naming conditions rather than types. Test `crates/happenstance-sqlite/tests/migration.rs::remint_replaces_the_persisted_identity` PASSED: a fresh 128-bit value different from the old one, reported by the re-opened handle, with exactly one identity row afterwards. The VT-6 mechanism record is references/adapter-shapes.md:367-404 — a new `## 7. VT-6 identity mechanisms, per adapter` section APPENDED at the end of the file (it was 365 lines), so the three SPECIFICATION.md citations into :97-102, :220 and :297 keep pointing at what they meant; `cargo xtask spec-trace` inside the gate still resolves all 389 citations and the count has not fallen"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::remint_replaces_the_persisted_identity (plus `cargo xtask spec-trace`)"

- id: AC-006
  criterion: "GIVEN `recorded_time_survives_a_reopen`, which has had nothing in the workspace able to fail it since phase 4, WHEN a row carrying a known `recorded_at` value is present and the store is closed and re-opened, THEN the column reads back exactly the stored value, and neither `open` nor `migrate` writes to `recorded_at` on any existing row — a re-stamp on open is the defect this criterion exists to reject."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:392-414 — `migrate` writes only to `store_meta`, never to `event`, so there is no path by which an open could re-stamp a row. Test `crates/happenstance-sqlite/tests/migration.rs::recorded_at_is_returned_as_stored_after_a_reopen` PASSED: a row inserted through raw SQL on the real file with recorded_at = 1_234_567_890_123 reads back exactly that value after a close-and-reopen, and again after a third open — so the value is not drifting one open at a time either"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::recorded_at_is_returned_as_stored_after_a_reopen"

- id: AC-007
  criterion: "GIVEN an adapter author who has been told the journal mode and busy timeout are set, and knows SQLite accepts an unknown pragma silently, WHEN either `SqliteEventStore::open` or `SqliteProjectionStore::open` returns, THEN reading the pragmas back on that live connection returns `wal` for `journal_mode`, ADR-0022's stated value for `synchronous` and never `OFF`/`0`, and a finite, non-zero millisecond value for `busy_timeout` — per handle, for both stores."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/connection.rs:89-100 (`configure`, the one path both stores take), :173-217 (`ConnectionSettings`, read off the live connection rather than restated from the constants), and crates/happenstance-sqlite/src/projection_store.rs:85 (the projection store's one-line adoption; its own `migrate` stays `todo!()`). Test `crates/happenstance-sqlite/tests/migration.rs::pragmas_are_in_effect_on_every_connection` PASSED: journal_mode = wal, synchronous = 1 (NORMAL, separately asserted != 0 because CF-14 names OFF by name), and busy_timeout = 5000 ms (asserted both > 0 and finite at the stated value) — per handle, for two event-store handles and for a projection connection; plus tests/migration.rs:544-566, which observes on an untouched file that SqliteProjectionStore::open configures BEFORE it migrates, since WAL is a persistent property of the file and survives the panic its still-`todo!()` migration raises"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::pragmas_are_in_effect_on_every_connection"

- id: AC-008
  criterion: "GIVEN a reader of this crate's published documentation, for whom the `# Intended schema` block is the schema, WHEN they compare that block against the database `migrate` creates, THEN every table and index the block names exists in `sqlite_master` with the same key order and the same covering columns, and the `contains_event_id` comment no longer states that the origin columns are absent."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:34-108 — the `# Intended schema` block rewritten to what `migrate` actually creates, with the two amendments (the covering `event_type`, `tag_cardinality`) stated as the reasons the block changed; and :506-512, where the `contains_event_id` comment no longer says the origin columns are absent. Test `crates/happenstance-sqlite/tests/migration.rs::module_doc_schema_matches_sqlite_master` PASSED: the block is pulled in by `include_str!` on the source file, parsed for its CREATE TABLE / CREATE INDEX names, and compared object-by-object against sqlite_master — plus a direct assertion that the phrase 'are absent from the schema' no longer occurs anywhere in the file. The `docs` step of `cargo xtask ci --fast` builds the rustdoc"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::module_doc_schema_matches_sqlite_master"

- id: AC-009
  criterion: "GIVEN the workspace's standing guards — the shape target that catches a field added carelessly, and a feature powerset that builds each feature alone — WHEN `SqliteEventStore` has gained its persisted-identity field and both stores share one connection-configuration path, THEN `tests/shapes.rs` passes unchanged in intent, `--no-default-features` and each single-feature build compile clean under `-D warnings`, `publish = false` and `xtask/src/package.rs`'s `PUBLISHABLE` are unchanged, and no `todo!()` outside `migrate` has been removed."
  satisfied: true
  evidence: "`cargo test -p happenstance-sqlite --test shapes` PASSED with the file unchanged — all ten assertions still hold after SqliteEventStore gained its `store_id: StoreId` field (crates/happenstance-sqlite/src/event_store.rs:145-153), which is sixteen plain bytes and therefore costs Send/Sync nothing; no `Statement`, `Rows` or `Transaction` reached a field. The shared connection module is gated `#[cfg(any(feature = \"event-store\", feature = \"projection-store\"))]` at crates/happenstance-sqlite/src/lib.rs:85-86, so `--no-default-features` does not build it dead. `cargo xtask affected --base main` PASSED whole (fmt, clippy `-D warnings` over all targets and all features, the five file-reading lints, spec-trace, and the tests). `publish = false` (crates/happenstance-sqlite/Cargo.toml:12) and `PUBLISHABLE` in xtask/src/package.rs are untouched, and the only `todo!()` removed is `migrate`'s: `rg -n 'todo!' crates/happenstance-sqlite/src` still reports `append`, `head`, `contains_event_id`, `fetch_page` and the projection store's `migrate`/`checkpoint`/`commit`, and `#![allow(clippy::todo)]` is still at lib.rs:83"
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs (all six assertions) via `cargo test -p happenstance-sqlite --test shapes`; `cargo xtask affected --base main`"
```
