---
item: "HS-S0036"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — Migration 1, as ADR-0022 amends it

> **STATUS: nine of nine ACs satisfied.** `happenstance-sqlite` now runs SQL. The
> crate's first real body is `SqliteEventStore::migrate`, and the first thing a
> reviewer can look at is `crates/happenstance-sqlite/tests/migration.rs` — nine
> tests that open a real temporary file and read the schema, the identity and the
> pragmas back **out of SQLite** rather than asserting them from the code that
> wrote them.
>
> One finding came out of the run and it is not cosmetic: **the busy timeout does
> not absorb the `SQLITE_BUSY` that eight concurrent opens of a fresh file
> produce**, because converting to WAL is a lock *promotion* and SQLite
> deliberately declines to invoke a busy handler for one. Seven of eight opens
> failed instantly until `connection::ensure_wal` was written. That is exactly the
> class of defect this story's concurrent-open criterion exists to find, and it
> would have surfaced two stories later as a flaky fixture.

## TDD Evidence

Rust makes one part of Red/Green explicit that other languages hide: a test
naming a method that does not exist cannot compile, so the red step is *signatures
with `todo!()` bodies plus the whole test file*. The failure is then a genuine
missing-behaviour panic at the exact line the behaviour is owed, not a typo.

**RED.** `crates/happenstance-sqlite/tests/migration.rs` written in full;
`store_id()`, `settings()`, `remint_identity()`, `SCHEMA_VERSION` and a fallible
`new` added to `SqliteEventStore` as `todo!()` bodies beside the `migrate` that
was already one. `cargo test -p happenstance-sqlite --test migration`:

```text
running 9 tests
test result: FAILED. 1 passed; 8 failed; 0 ignored

---- migration_creates_the_amended_schema stdout ----
thread 'migration_creates_the_amended_schema' panicked at
crates\happenstance-sqlite\src\event_store.rs:126:9:
not yet implemented: SQLite event store: open and migrate
```

Every one of the eight named the same missing behaviour. The ninth,
`settings_report_what_the_connection_answered`, is a supporting check on the
settings type rather than an AC row, and passed from the start.

**GREEN.** `MIGRATION_1`, `migrate`, `read_identity`, `remint_identity`, the
`store_id` field and `crates/happenstance-sqlite/src/connection.rs`.
`cargo test -p happenstance-sqlite --test migration`: **9 passed, 0 failed**.

**The red run in the middle, which is the one worth reading.** Between those two
states the concurrent-open criterion failed *for a real reason* rather than for a
missing body:

```text
---- concurrent_opens_of_a_fresh_file_agree_on_one_store_id stdout ----
called `Result::unwrap()` on an `Err` value:
Sqlite(SqliteFailure(Error { code: DatabaseBusy, extended_code: 5 }, Some("database is locked")))
```

Seven of eight threads, with `busy_timeout = 5000` already installed. Moving the
timeout above the pragma batch did not fix it, which is what identified the cause:
`PRAGMA journal_mode = WAL` reads the schema first (taking a read lock) and then
needs an exclusive one, so it is a **promotion**, and SQLite documents that it
returns `SQLITE_BUSY` for a promotion *without* calling the busy handler — on
purpose, to break the deadlock two promoting readers would otherwise reach. The
documented response is to release and retry, which is `ensure_wal`
(`crates/happenstance-sqlite/src/connection.rs:112-156`): bounded at 64 attempts
so it can never become the hang CF-33 forbids, and yielding rather than sleeping
so no clock is read.

| AC | Test that encodes it | Red → Green |
| --- | --- | --- |
| AC-001 | `migration.rs::migration_creates_the_amended_schema` | `todo!("open and migrate")` → sqlite_master, `pragma_table_info` and `EXPLAIN QUERY PLAN` all read back |
| AC-002 | `migration.rs::second_open_of_a_migrated_file_changes_nothing` | same panic → object set identical, one identity row |
| AC-003 | `migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id` | same panic → then a **second** red, `DatabaseBusy` on 7 of 8 → `ensure_wal` → 8 of 8, one `StoreId` |
| AC-004 | `migration.rs::store_id_survives_a_close_and_reopen` | same panic → byte-identical across a reopen, stored as a 16-byte blob |
| AC-005 | `migration.rs::remint_replaces_the_persisted_identity` | `todo!("re-mint the persisted identity")` → a fresh value, one row |
| AC-006 | `migration.rs::recorded_at_is_returned_as_stored_after_a_reopen` | same panic → the stamp survives two further opens |
| AC-007 | `migration.rs::pragmas_are_in_effect_on_every_connection` | same panic → three pragmas read back per handle, both stores |
| AC-008 | `migration.rs::module_doc_schema_matches_sqlite_master` | same panic → the doc block compared object-by-object |
| AC-009 | `tests/shapes.rs`, unchanged, plus `cargo xtask affected --base main` | 10 passed, gate green |

## Commits

`feat(sqlite-durable-store): Migration 1, as ADR-0022 amends it` — see the
`Story: sqlite-durable-store/schema-migration-and-identity` trailer. The SHA is
recorded in this story's `report.md` *Findings Ledger* and in the slice digest.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/src/connection.rs` | **New.** The one connection configuration both stores use: `JOURNAL_MODE`, `SYNCHRONOUS`, `BUSY_TIMEOUT_MS` as documented public constants with the alternative that lost beside each; `open_configured` / `configure`; `ensure_wal`, the bounded WAL-conversion retry; and `ConnectionSettings::read_back`, which asks a live connection what it is running under. Gated `any(feature = "event-store", feature = "projection-store")` so `--no-default-features` does not build it dead |
| `crates/happenstance-sqlite/src/event_store.rs` | The `# Intended schema` module-doc block rewritten to what `migrate` creates, with the two ADR-0022 amendments stated as the reasons it changed; the status banner corrected; `MIGRATION_1`; a real `migrate` in one `BEGIN IMMEDIATE`; `read_identity`; a `store_id: StoreId` field with `store_id()`; `settings()`; `remint_identity()`; `new` made fallible; three new error variants (`UnsupportedSchemaVersion`, `MissingIdentity`, `MalformedIdentity`); the `contains_event_id` comment corrected |
| `crates/happenstance-sqlite/src/lib.rs` | One line: the new `connection` module, feature-gated |
| `crates/happenstance-sqlite/src/projection_store.rs` | **One line**, deliberately: `Connection::open` → `crate::connection::open_configured`. Its own `migrate` stays `todo!()` |
| `crates/happenstance-sqlite/tests/migration.rs` | **New target.** Nine tests, a self-deleting `TempDb`, and every assertion about the file made through a raw `rusqlite::Connection` the store never held |
| `references/adapter-shapes.md` | A new `## 7. VT-6 identity mechanisms, per adapter`, **appended at the end** |

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test migration` | **9 passed**, 0 failed |
| `cargo test -p happenstance-sqlite --test shapes` | **10 passed**, unchanged in intent |
| `cargo xtask affected --base main` | **PASSED** — fmt, clippy `-D warnings` over all targets and all features, the five file-reading lints, `spec-trace` (201 clauses, 112 rules, 389 citations, none unresolved), and the tests |
| `cargo fmt --all` | clean; run as the last pass |

`clippy::too_many_lines` fired once on the schema test and was answered by
splitting it into four named assertion helpers, not by an `#[allow]`.

## Notes

**One deliberate design decision the spec flagged and left open: `new` is now
fallible.** It promised the caller had applied the schema and returned `Self`.
Once the store carries an incarnation, an infallible `new` would have to invent
one — and a `StoreId`-less store mints every `EventId` under a zero origin, which
VT-6 describes as having no error path and no observable symptom. So `new` reads
the identity back and says so in its rustdoc.

**Entropy comes from `SELECT randomblob(16)`, not from a new crate.** NF-002
forbids a manifest change and names this as the option to weigh. It is SQLite's
own CSPRNG, it is evaluated inside the same `INSERT OR IGNORE` that decides
whether a mint happens at all, and no zero or constant is ever a fallback: a row
that reads back as anything other than sixteen bytes is `MalformedIdentity` and
the transaction rolls back.

**The schema-version marker lives in `store_meta`, not in `PRAGMA user_version`.**
Both were legitimate; `store_meta` was chosen so the marker is an object a
reviewer can see in the file beside the identity it travels with, and because
`user_version` is a 32-bit integer that could never have held the `StoreId`
anyway.

**`SqliteProjectionStore` did not gain a `settings()` method, and the omission is
deliberate.** The obvious way to prove AC-007's "for both stores" half was to add
one — but `spec/SPECIFICATION.md:375` cites
`crates/happenstance-sqlite/src/projection_store.rs:235-260` *anchored to
`rollback`*, and adding ~25 lines above it re-points that citation. `cargo xtask
spec-trace` caught it immediately, which is the check working. Rather than edit
`spec/SPECIFICATION.md` — explicitly out of this PR's boundary — the projection
store's change was reduced to exactly one line with zero net drift, and AC-007's
projection half is proved two other ways: reading the three pragmas back off a
connection from the same `open_configured` the store calls, and observing on an
untouched file that `SqliteProjectionStore::open` leaves it in WAL *before* its
own `todo!()` migration panics (WAL is persisted in the file, so the observation
survives the unwind).

**Carry-forward, recorded rather than fixed: seven `spec/SPECIFICATION.md`
citations into `crates/happenstance-sqlite/src/event_store.rs` have drifted.**
The module-doc rewrite moved the file's line numbers, so `:47-53` (cited four
times, at `SPECIFICATION.md:3329`, `:3844`, `:7532`, `:7599`), `:151-193`
(`:2645`), `:195` (`:3961`) and `:202` (`:2555`) no longer point at the text
their sentences describe. None is anchored, so `spec-trace` reports them as
resolving — which is precisely the silent-drift defect this story was warned
about for `references/adapter-shapes.md`. They belong to the standing
reconciliation criterion (`RUNBOOK.md:3810-3820`, project AC-016), whose owner is
`spec-and-code-reconciliation`. **Recorded here so that story has a list rather
than a search.**

**Nothing out of boundary.** No `spec/SPECIFICATION.md` edit, no `[PROVISIONAL]`
marker moved, no ADR authored, no `.kb/` file touched, no `Cargo.toml` change and
no new dependency, `publish = false` and `xtask/src/package.rs`'s `PUBLISHABLE`
unchanged, `#![allow(clippy::todo)]` still at `lib.rs:83`, and the only `todo!()`
removed is `migrate`'s — `append`, `head`, `contains_event_id`, `fetch_page` and
the projection store's four are all still marked.
