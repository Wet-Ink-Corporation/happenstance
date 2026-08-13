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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::migration_creates_the_amended_schema"

- id: AC-002
  criterion: "GIVEN the adapter author whose fixture will call `connect()` more than once and whose `open` therefore runs `migrate` on every connect, WHEN `SqliteEventStore::open` is called a second time on an already-migrated path, THEN it returns `Ok`, the set of objects in `sqlite_master` is unchanged from the first open, exactly one identity row exists, and the second handle's `store_id()` equals the first handle's."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::second_open_of_a_migrated_file_changes_nothing"

- id: AC-003
  criterion: "GIVEN an adapter author whose CI opens two handles onto one file and cannot control which wins, WHEN N threads (N >= 2, and at least the handle count the slice's fixture will use) call `SqliteEventStore::open` on one fresh path simultaneously, THEN every call returns `Ok`, every handle reports the same `store_id()`, and the file afterwards holds exactly one identity row and one copy of each schema object — the loser of the insert race having adopted the winner's value rather than kept its own."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id"

- id: AC-004
  criterion: "GIVEN the application author whose fear is a second, independent reader building a wrong answer from a log whose identities moved under it, WHEN a store is opened on a fresh path, its `store_id()` recorded, the handle dropped, and the same path re-opened into a new `SqliteEventStore`, THEN `store_id()` is byte-identical to the recorded value, `StoreId::to_bytes` round-trips it, and the persisted form is bytes or text — never an integer and never a pair of integers."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::store_id_survives_a_close_and_reopen"

- id: AC-005
  criterion: "GIVEN an operator who restored this file from Friday's backup — a state a SQLite adapter cannot detect, which is why VT-6 grants mint-once only against a documented procedure — WHEN they invoke the adapter's public re-mint operation and re-open, THEN the persisted identity has been replaced by a fresh 128-bit value different from the old one, the re-opened handle reports the new one, the operation's rustdoc states in prose when a deployment invokes it (after a restore or a clone) with an `# Errors` section naming conditions rather than types, and `references/adapter-shapes.md` records mint-once-with-explicit-re-mint as this adapter's chosen VT-6 mechanism."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::remint_replaces_the_persisted_identity (plus `cargo xtask spec-trace`)"

- id: AC-006
  criterion: "GIVEN `recorded_time_survives_a_reopen`, which has had nothing in the workspace able to fail it since phase 4, WHEN a row carrying a known `recorded_at` value is present and the store is closed and re-opened, THEN the column reads back exactly the stored value, and neither `open` nor `migrate` writes to `recorded_at` on any existing row — a re-stamp on open is the defect this criterion exists to reject."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::recorded_at_is_returned_as_stored_after_a_reopen"

- id: AC-007
  criterion: "GIVEN an adapter author who has been told the journal mode and busy timeout are set, and knows SQLite accepts an unknown pragma silently, WHEN either `SqliteEventStore::open` or `SqliteProjectionStore::open` returns, THEN reading the pragmas back on that live connection returns `wal` for `journal_mode`, ADR-0022's stated value for `synchronous` and never `OFF`/`0`, and a finite, non-zero millisecond value for `busy_timeout` — per handle, for both stores."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::pragmas_are_in_effect_on_every_connection"

- id: AC-008
  criterion: "GIVEN a reader of this crate's published documentation, for whom the `# Intended schema` block is the schema, WHEN they compare that block against the database `migrate` creates, THEN every table and index the block names exists in `sqlite_master` with the same key order and the same covering columns, and the `contains_event_id` comment no longer states that the origin columns are absent."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/migration.rs::module_doc_schema_matches_sqlite_master"

- id: AC-009
  criterion: "GIVEN the workspace's standing guards — the shape target that catches a field added carelessly, and a feature powerset that builds each feature alone — WHEN `SqliteEventStore` has gained its persisted-identity field and both stores share one connection-configuration path, THEN `tests/shapes.rs` passes unchanged in intent, `--no-default-features` and each single-feature build compile clean under `-D warnings`, `publish = false` and `xtask/src/package.rs`'s `PUBLISHABLE` are unchanged, and no `todo!()` outside `migrate` has been removed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/migration.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs (all six assertions) via `cargo test -p happenstance-sqlite --test shapes`; `cargo xtask affected --base main`"
```
