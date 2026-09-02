---
item: "HS-S0036"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — Migration 1, as ADR-0022 amends it

## Findings Ledger

**Nine of nine ACs satisfied.** `happenstance-sqlite` has stopped being a crate
that has never written a byte to a database. Migration 1 exists, the incarnation
is minted once and persisted, the three pragmas are in effect, and all of it is
read back **out of SQLite** by a new real `cargo test` target rather than
asserted from the code that wrote it.

**Mount point:** `crates/happenstance-sqlite/tests/migration.rs` — a new
integration target, auto-discovered by `cargo test -p happenstance-sqlite` and
therefore run by `cargo xtask affected --base main` and `cargo xtask ci --fast`
with no script edit. It is reachable from `tests/`, never from a `mod` only
`cargo check` sees, which is architecture brief AC-A01's rule in substance.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the amended schema | **Met** | `migration.rs::migration_creates_the_amended_schema`. `sqlite_master` read back as exactly `{table event, table event_tag, table store_meta, table tag_cardinality, index event_type_idx}`; `AUTOINCREMENT` asserted from the stored DDL; `event_tag` `WITHOUT ROWID` keyed `(tag, 1), (position, 2)` with `event_type` at `pk = 0` from `pragma_table_info`; **one** `UNIQUE` index over `(origin_store, origin_position)` in that order; `schema_version = 1`. The architectural claim is asserted by **query plan**, not by DDL text: `EXPLAIN QUERY PLAN` for a tag-plus-type probe names no access to `event` |
| **AC-002** — a second open changes nothing | **Met** | `migration.rs::second_open_of_a_migrated_file_changes_nothing`. Object set identical before and after, exactly one identity row, both handles reporting one `StoreId`. `migrate` runs on every connect (`event_store.rs:392-414`) and is `CREATE ... IF NOT EXISTS` throughout |
| **AC-003** — N concurrent opens agree | **Met, and it took a real fix** | `migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id`. Eight threads on a `std::sync::Barrier`; the observed `StoreId` **set has one element**, asserted as a set property rather than against a literal N. This is the criterion that failed for a real reason — see *The finding* below |
| **AC-004** — the identity survives a reopen | **Met** | `migration.rs::store_id_survives_a_close_and_reopen`. Byte-identical, `to_bytes` round-trips, and the storage form asserted from SQLite's own `typeof(v)`/`length(v)` as blob-or-text and sixteen bytes — never an integer, which a 128-bit value cannot survive being |
| **AC-005** — mint-once is *earned* | **Met** | `migration.rs::remint_replaces_the_persisted_identity`, plus `SqliteEventStore::remint_identity` (`event_store.rs:319-374`) whose rustdoc has a `# When a deployment invokes this` section in prose and an `# Errors` section naming conditions. VT-6's second MUST is discharged at `references/adapter-shapes.md:367-404`, **appended at the end of a 365-line file** so the three `SPECIFICATION.md` citations at `:97-102`, `:220` and `:297` still point at what they meant |
| **AC-006** — `recorded_at` is read back, never re-stamped | **Met** | `migration.rs::recorded_at_is_returned_as_stored_after_a_reopen`. A known stamp survives a reopen and a third open. `migrate` writes only to `store_meta`, so there is no path by which an open could touch it |
| **AC-007** — the pragmas are in effect | **Met** | `migration.rs::pragmas_are_in_effect_on_every_connection`. `journal_mode = wal`, `synchronous = 1` (separately asserted `!= 0`, because CF-14 names `OFF` by name), `busy_timeout = 5000` ms (asserted both non-zero and finite) — **per handle**, for two event-store handles and for a projection connection, every value read off the live connection through `ConnectionSettings::read_back` |
| **AC-008** — the published schema is the schema | **Met** | `migration.rs::module_doc_schema_matches_sqlite_master`. The `# Intended schema` block is pulled in by `include_str!` and compared object-by-object against `sqlite_master`, mechanically rather than by review; plus an assertion that the `contains_event_id` comment no longer claims the origin columns are absent |
| **AC-009** — the standing guards still hold | **Met** | `cargo test -p happenstance-sqlite --test shapes`: **10 passed**, file unchanged, after `SqliteEventStore` gained a `StoreId` field — sixteen plain bytes, so `Send`/`Sync` are untouched and no borrowing `rusqlite` handle reached a field. `cargo xtask affected --base main` green whole. The shared module is gated `any(...)`, so `--no-default-features` does not build it dead |

## The finding

**The busy timeout does not absorb the `SQLITE_BUSY` that concurrent opens of a
*fresh* file produce, and reordering it does not help.** With `busy_timeout =
5000` installed before anything else, seven of eight threads opening one new file
behind a barrier failed instantly with `DatabaseBusy`.

The cause is that `PRAGMA journal_mode = WAL` reads the schema first — taking a
read lock — and then needs an exclusive one. That is a **lock promotion**, and
SQLite documents that it returns `SQLITE_BUSY` for a promotion *without* invoking
the busy handler, deliberately, to break the deadlock two promoting readers would
otherwise reach. Waiting is the wrong response; releasing and retrying is the
documented one, and that is `connection::ensure_wal`
(`crates/happenstance-sqlite/src/connection.rs:112-156`) — bounded at 64 attempts
so it can never become the hang CF-33 forbids, yielding rather than sleeping so
no clock is read.

Left unfound, this would have surfaced two stories later as an intermittently red
conformance run against `SqliteFixture` — a fixture that opens two handles onto
one file — and it would have looked like flakiness rather than like a missing
line.

## Deferred, and to whom

Nothing in this story's scope is deferred. Two things leave with an owner:

- **Seven drifted `spec/SPECIFICATION.md` citations into
  `crates/happenstance-sqlite/src/event_store.rs`** — `:47-53` (cited from
  `SPECIFICATION.md:3329`, `:3844`, `:7532`, `:7599`), `:151-193` (`:2645`),
  `:195` (`:3961`) and `:202` (`:2555`). The module-doc rewrite moved them. None
  is anchored, so `spec-trace` reports them as *resolving*, which is exactly the
  silent defect. Owner: **`spec-and-code-reconciliation`**, under the standing
  criterion at `RUNBOOK.md:3810-3820` (project AC-016). Listed here so that story
  has a list rather than a search.
- **`SqliteProjectionStore::settings`** was designed, then deliberately not
  added: it would have re-pointed the one *anchored* citation into
  `projection_store.rs`. The projection store's change is one line with zero net
  drift, and AC-007's projection half is proved by the shared `open_configured`
  path and by an end-to-end observation on an untouched file.

## What is still `todo!()`, on purpose

`append`, `head`, `contains_event_id`, `ReadCursor::fetch_page` and the
projection store's four. `#![allow(clippy::todo)]` is still at
`crates/happenstance-sqlite/src/lib.rs:83`, and DR-01 puts its deletion in the
same change as the *last* `todo!()` — which is a later story's.
