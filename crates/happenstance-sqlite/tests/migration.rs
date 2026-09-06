//! Migration 1, read back out of SQLite rather than asserted from the code
//! that wrote it.
//!
//! This target asks three questions the conformance suite never asks: *what did
//! SQLite actually create*, *do two concurrent opens agree*, and *is a pragma in
//! effect* — the last one because SQLite silently accepts a pragma it does not
//! recognise, so a statement that executed is not a setting that took.
//!
//! Everything here runs real SQL against a real temporary file through a real
//! [`rusqlite::Connection`]. Nothing is doubled: there is no fake driver, no
//! in-memory stand-in for the file, and no second hand-rolled encoding.
//!
//! Each half is gated on the feature that provides it, so nothing here compiles
//! a test against a module that was configured out. The event-store half is the
//! file-level `#![cfg]` below; the projection half is one `#[test]`, gated at its
//! own boundary, because `projection-store` is off by default and widening the
//! file-level attribute would take the pragma readbacks out of the default build
//! with it.
//!
//! That second gate is newer than this sentence's first draft, which claimed the
//! property for the whole file while one assertion mid-body of
//! `pragmas_are_in_effect_on_every_connection` reached
//! `happenstance_sqlite::projection_store` unguarded — so `cargo test -p
//! happenstance-sqlite` did not compile at all. No gate step caught it: the
//! `cargo hack` feature powerset runs `--no-dev-deps`, which cannot be combined
//! with `--all-targets` and so never builds a test target, and every other step
//! passes `--all-features`.
//!
//! # Why the gate was not widened to catch the next one, and what was measured
//!
//! The obvious repair is a second `cargo hack` step carrying `--all-targets`
//! (the existing one cannot: `cargo hack` 0.6.45 answers *"--no-dev-deps may not
//! be used together with --all-targets"*, because `--no-dev-deps` rewrites each
//! manifest). It was measured before being declined. `cargo hack check
//! --workspace --feature-powerset --all-targets --keep-going`, 2026-09-03 at
//! rustc 1.97.1: **68 of 214 configurations fail.** Four are this bug, fixed
//! here. The other 64 are `happenstance`, every one an `error[E0432]` on
//! `happenstance::Json` and `happenstance::commit` from four test targets that
//! assume the `json` and `memory` features their crate's `default` supplies —
//! the same defect in a crate this change does not own.
//!
//! So the step stays out rather than arriving red, and this paragraph is what a
//! step would have been: a gate step that cannot go green is not a gate step,
//! and one softened with a skip list is worse than none. Adding it is right
//! once `happenstance`'s 64 are fixed, and it will owe `--locked` — it rewrites
//! no manifest, so the exemption the existing powerset step relies on does not
//! reach it.

#![cfg(feature = "event-store")]
#![allow(clippy::unwrap_used)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};

use happenstance_core::{AppendError, Event, SendEventStore, StoreId};
use happenstance_sqlite::connection::{
    BUSY_TIMEOUT_MS, ConnectionSettings, JOURNAL_MODE, SYNCHRONOUS, open_configured,
};
use happenstance_sqlite::event_store::{SCHEMA_VERSION, SqliteEventStore, SqliteEventStoreError};
use rusqlite::Connection;

/// A temporary database path that deletes itself, and its WAL sidecars, on drop.
///
/// A process-local ordinal plus the process id, in the shape
/// `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102` already
/// uses: no new dependency, and two tests in this file can never collide on a
/// path — which is what would otherwise make the concurrent-open criterion flaky
/// in exactly the way that gets a test deleted.
#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-{label}-{}-{ordinal}.db",
            std::process::id()
        ));
        // A leftover from a previous run would make "a path that has never been
        // migrated" a lie.
        let _ = std::fs::remove_file(&path);
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.0.clone().into_os_string();
            path.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

/// A raw connection onto the file, opened by the test rather than by the store.
///
/// Every assertion about *what is in the file* is made through one of these, so
/// that a wrong `open` cannot both write the file and describe it.
fn raw(path: &Path) -> Connection {
    Connection::open(path).unwrap()
}

/// The smallest valid event, for the identity assertions below.
fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).unwrap()
}

/// How many rows the `event` table holds, read through a raw connection.
fn rows(path: &Path) -> i64 {
    raw(path)
        .query_row("SELECT count(*) FROM event", [], |row| row.get(0))
        .unwrap()
}

/// The names SQLite reports for the objects this crate created, with its own
/// internal ones (`sqlite_sequence`, the `UNIQUE` auto-index) filtered out.
fn schema_objects(connection: &Connection) -> BTreeSet<(String, String)> {
    let mut statement = connection
        .prepare("SELECT type, name FROM sqlite_master ORDER BY type, name")
        .unwrap();
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap();

    rows.map(Result::unwrap)
        .filter(|(_, name)| !name.starts_with("sqlite_"))
        .collect()
}

/// The `CREATE …` text SQLite stored for one object.
fn create_sql(connection: &Connection, name: &str) -> String {
    connection
        .query_row(
            "SELECT sql FROM sqlite_master WHERE name = ?",
            [name],
            |row| row.get::<_, String>(0),
        )
        .unwrap()
}

/// `(name, primary-key ordinal)` for every column of `table`, in declared order.
///
/// The primary-key ordinal is the load-bearing half: it is what tells a covering
/// column apart from a key column, and writing
/// `PRIMARY KEY (tag, position, event_type)` compiles, passes every conformance
/// rule, and destroys the position-sorted range the amendment exists for.
fn columns(connection: &Connection, table: &str) -> Vec<(String, i64)> {
    let mut statement = connection
        .prepare("SELECT name, pk FROM pragma_table_info(?) ORDER BY cid")
        .unwrap();
    let rows = statement
        .query_map([table], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })
        .unwrap();
    rows.map(Result::unwrap).collect()
}

/// Every `EXPLAIN QUERY PLAN` detail line for `sql`.
fn query_plan(connection: &Connection, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Vec<String> {
    let mut statement = connection
        .prepare(&format!("EXPLAIN QUERY PLAN {sql}"))
        .unwrap();
    let rows = statement
        .query_map(params, |row| row.get::<_, String>(3))
        .unwrap();
    rows.map(Result::unwrap).collect()
}

/// AC-001 — migration 1 creates the schema ADR-0022 amended, and the amendment
/// is visible in the *plan* rather than only in the DDL text.
#[test]
fn migration_creates_the_amended_schema() {
    let db = TempDb::new("amended-schema");
    let store = SqliteEventStore::open(db.path()).unwrap();
    drop(store);

    let connection = raw(db.path());

    let objects = schema_objects(&connection);
    let expected: BTreeSet<(String, String)> = [
        ("index", "event_type_idx"),
        ("table", "event"),
        ("table", "event_tag"),
        ("table", "store_meta"),
        ("table", "tag_cardinality"),
    ]
    .into_iter()
    .map(|(kind, name)| (kind.to_owned(), name.to_owned()))
    .collect();
    assert_eq!(objects, expected, "sqlite_master after migration 1");

    assert_event_table(&connection);
    assert_origin_pair_is_one_unique(&connection);
    assert_event_tag_key_and_covering_column(&connection);

    // The schema-version marker, so migration 2 has something to test against.
    let version: i64 = connection
        .query_row(
            "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(version, i64::from(SCHEMA_VERSION));

    assert_a_tag_and_type_probe_never_touches_event(&connection);
}

/// `event` carries an `AUTOINCREMENT` position and every column the log needs.
fn assert_event_table(connection: &Connection) {
    let event_sql = create_sql(connection, "event");
    assert!(
        event_sql.contains("AUTOINCREMENT"),
        "position must be AUTOINCREMENT so a position is never reused after a \
         delete; got: {event_sql}"
    );
    let event_columns: Vec<String> = columns(connection, "event")
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    for column in [
        "position",
        "event_type",
        "data",
        "metadata",
        "tags",
        "origin_store",
        "origin_position",
        "recorded_at",
    ] {
        assert!(
            event_columns.iter().any(|name| name == column),
            "event is missing the {column} column; got {event_columns:?}"
        );
    }
}

/// The `EventId` origin pair is one `UNIQUE` constraint over both columns, in
/// order — not two separate ones, and not none.
fn assert_origin_pair_is_one_unique(connection: &Connection) {
    let unique_indexes: Vec<String> = {
        let mut statement = connection
            .prepare("SELECT name FROM pragma_index_list('event') WHERE \"unique\" = 1")
            .unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(Result::unwrap).collect()
    };
    assert_eq!(
        unique_indexes.len(),
        1,
        "exactly one UNIQUE constraint is expected on event, over the origin \
         pair together; got {unique_indexes:?}"
    );
    let origin_columns: Vec<String> = {
        let mut statement = connection
            .prepare("SELECT name FROM pragma_index_info(?) ORDER BY seqno")
            .unwrap();
        let rows = statement
            .query_map([&unique_indexes[0]], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(Result::unwrap).collect()
    };
    assert_eq!(
        origin_columns,
        vec!["origin_store".to_owned(), "origin_position".to_owned()],
        "the EventId origin pair must be UNIQUE together, not separately"
    );
}

/// `event_tag` is `WITHOUT ROWID`, keyed `(tag, position)` in that order, with
/// `event_type` carried as a **non-key** covering column.
fn assert_event_tag_key_and_covering_column(connection: &Connection) {
    let tag_sql = create_sql(connection, "event_tag");
    assert!(
        tag_sql.to_uppercase().contains("WITHOUT ROWID"),
        "event_tag must be WITHOUT ROWID; got: {tag_sql}"
    );
    let tag_columns = columns(connection, "event_tag");
    let key: Vec<(String, i64)> = tag_columns
        .iter()
        .filter(|(_, pk)| *pk > 0)
        .cloned()
        .collect();
    assert_eq!(
        key,
        vec![("tag".to_owned(), 1), ("position".to_owned(), 2)],
        "the key stays (tag, position), in that order"
    );
    assert_eq!(
        tag_columns
            .iter()
            .find(|(name, _)| name == "event_type")
            .map(|(_, pk)| *pk),
        Some(0),
        "event_type is a covering column and NOT part of the key: in the key it \
         breaks the position ordering the amendment exists to preserve"
    );
}

/// NF-001, asserted rather than assumed: a tag-plus-type probe is answerable
/// from `event_tag` alone.
///
/// The assertion is on the *absence* of the `event` table in the plan rather
/// than a pattern match on SQLite's phrasing, which changes between versions and
/// would make this brittle for no gain.
fn assert_a_tag_and_type_probe_never_touches_event(connection: &Connection) {
    let plan = query_plan(
        connection,
        "SELECT position FROM event_tag WHERE tag = ? AND event_type IN (?)",
        &[&"course:c1", &"student.enrolled"],
    );
    assert!(!plan.is_empty(), "EXPLAIN QUERY PLAN returned no rows");
    for detail in &plan {
        let touches_event = detail
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .any(|word| word == "event");
        assert!(
            !touches_event,
            "a tag-plus-type probe must not join back to `event` — that walk \
             happens under the BEGIN IMMEDIATE write lock and serialises every \
             writer. Plan: {plan:?}"
        );
    }
}

/// AC-002 — `migrate` runs on every connect, so a second open must change
/// nothing a caller could observe.
#[test]
fn second_open_of_a_migrated_file_changes_nothing() {
    let db = TempDb::new("second-open");

    let first = SqliteEventStore::open(db.path()).unwrap();
    let before = schema_objects(&raw(db.path()));
    let first_id = first.store_id();

    let second = SqliteEventStore::open(db.path()).unwrap();
    let after = schema_objects(&raw(db.path()));

    assert_eq!(
        before, after,
        "a second open added or dropped a schema object"
    );
    assert_eq!(
        first_id,
        second.store_id(),
        "a second open minted a second incarnation"
    );

    let identity_rows: i64 = raw(db.path())
        .query_row(
            "SELECT count(*) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(identity_rows, 1);
}

/// AC-003 — N concurrent opens of a *fresh* path agree on one incarnation.
///
/// The threads rendezvous on a barrier before calling `open`, because N threads
/// that never actually overlap prove nothing. The assertion is a *set* property
/// — one distinct `StoreId` — which holds at any N above one and does not pin a
/// literal that would collide with the live 8-versus-64 contender question.
#[test]
fn concurrent_opens_of_a_fresh_file_agree_on_one_store_id() {
    let db = TempDb::new("concurrent-open");
    let contenders = 8;
    let barrier = Arc::new(Barrier::new(contenders));

    let ids: BTreeSet<StoreId> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..contenders)
            .map(|_| {
                let barrier = Arc::clone(&barrier);
                let path = db.path().to_path_buf();
                scope.spawn(move || {
                    barrier.wait();
                    SqliteEventStore::open(&path).unwrap().store_id()
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    });

    assert_eq!(
        ids.len(),
        1,
        "the loser of the INSERT OR IGNORE race must adopt the winner's value \
         by reading it back, not keep the one it generated; observed {ids:?}"
    );

    let connection = raw(db.path());
    let identity_rows: i64 = connection
        .query_row(
            "SELECT count(*) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(identity_rows, 1);
    assert_eq!(schema_objects(&connection).len(), 5);
}

/// AC-004 — the incarnation is persisted, not re-minted, and is stored in a form
/// a 128-bit value survives.
#[test]
fn store_id_survives_a_close_and_reopen() {
    let db = TempDb::new("identity-reopen");

    let minted = {
        let store = SqliteEventStore::open(db.path()).unwrap();
        store.store_id()
    };

    let reopened = SqliteEventStore::open(db.path()).unwrap();
    assert_eq!(minted, reopened.store_id());
    assert_eq!(
        StoreId::from_bytes(reopened.store_id().to_bytes()),
        minted,
        "to_bytes must round-trip the persisted value"
    );

    // Bytes or text, never an integer and never a pair of integers: Workers SQL
    // widens an integer through a JS number and bounds it at 2^53.
    let (kind, length): (String, i64) = raw(db.path())
        .query_row(
            "SELECT typeof(v), length(v) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .unwrap();
    assert!(
        kind == "blob" || kind == "text",
        "a StoreId is persisted as bytes or text, never as an integer; got {kind}"
    );
    if kind == "blob" {
        assert_eq!(length, 16);
    }
}

/// AC-005 — mint-once is legitimised by an explicit re-mint, which is what VT-6
/// charges for the permission.
#[test]
fn remint_replaces_the_persisted_identity() {
    let db = TempDb::new("remint");

    let before = SqliteEventStore::open(db.path()).unwrap().store_id();
    let minted = SqliteEventStore::remint_identity(db.path()).unwrap();
    let after = SqliteEventStore::open(db.path()).unwrap().store_id();

    assert_ne!(before, minted, "a re-mint must produce a fresh value");
    assert_eq!(minted, after, "the re-opened handle reports the new value");

    let identity_rows: i64 = raw(db.path())
        .query_row(
            "SELECT count(*) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(identity_rows, 1, "a re-mint replaces, it does not append");
}

/// R-3 — an append through a handle the file has outgrown is refused, not
/// stamped under the incarnation that was retired.
///
/// VT-6 permits mint-once **only if** an adapter can detect that its state was
/// restored or cloned, **or** the deployment is documented to invoke the
/// re-mint. This adapter takes the second branch, so `remint_identity`'s
/// documented procedure is the half of the permission it rests on — and that
/// procedure says *"Run it with nothing else holding the database open"*, a
/// precondition nothing enforced. A live handle keeps the `store_id` it read at
/// construction, so after a re-mint it goes on minting `EventId`s under the
/// retired incarnation; the collision surfaces only when a replication peer sees
/// the same `(StoreId, SequencePosition)` twice, and by then its dedup has
/// dropped real facts.
///
/// The cross-process and restore-from-backup cases are genuinely undetectable
/// from inside SQLite, and VT-6's `Rejects:` paragraph accepts that risk in
/// terms. This is the strictly narrower **in-process** one: no external actor is
/// involved, and the split is visible to the program that caused it — under a
/// lock the writer is already holding.
#[tokio::test]
async fn an_append_through_a_handle_the_file_has_outgrown_is_refused() {
    let db = TempDb::new("outgrown-handle");
    let store = SqliteEventStore::open(db.path()).unwrap();
    let retired = store.store_id();

    store.append(&[event("Before")], None).await.unwrap();
    let before = rows(db.path());

    let minted = SqliteEventStore::remint_identity(db.path()).unwrap();
    assert_ne!(
        retired, minted,
        "the re-mint must move the persisted identity"
    );

    let refused = store
        .append(&[event("After")], None)
        .await
        .expect_err("the stale handle must not stamp an event under the retired incarnation");
    assert!(
        matches!(
            refused,
            AppendError::Store(SqliteEventStoreError::IdentityMoved { handle, persisted })
                if handle == retired && persisted == minted
        ),
        "the refusal must name both incarnations, so an operator can tell which \
         handle to drop; got {refused:?}"
    );
    assert_eq!(
        rows(db.path()),
        before,
        "a refused append leaves the file as it was"
    );

    // The file is not poisoned: a handle opened after the re-mint appends under
    // the new incarnation, which is the whole point of the procedure.
    let fresh = SqliteEventStore::open(db.path()).unwrap();
    assert_eq!(fresh.store_id(), minted);
    fresh.append(&[event("After")], None).await.unwrap();
    assert_eq!(rows(db.path()), before + 1);
}

/// AC-006 — `recorded_at` is returned as stored, and neither `open` nor
/// `migrate` writes to it.
///
/// The row is inserted by the test through real SQL on a real file, which is
/// legitimate precisely because it is not a double: appending is a slice-mate's,
/// and what is testable here is the half the reopen rules depend on.
#[test]
fn recorded_at_is_returned_as_stored_after_a_reopen() {
    let db = TempDb::new("recorded-at");
    let stamped = 1_234_567_890_123_i64;

    let store = SqliteEventStore::open(db.path()).unwrap();
    let store_id = store.store_id();
    {
        let connection = raw(db.path());
        connection
            .execute(
                "INSERT INTO event \
                 (event_type, data, metadata, tags, origin_store, origin_position, recorded_at) \
                 VALUES ('test.stamped', X'7B7D', NULL, X'1F', ?, 1, ?)",
                rusqlite::params![&store_id.to_bytes()[..], stamped],
            )
            .unwrap();
    }
    drop(store);

    let reopened = SqliteEventStore::open(db.path()).unwrap();
    let read_back: i64 = raw(db.path())
        .query_row(
            "SELECT recorded_at FROM event WHERE event_type = 'test.stamped'",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(
        read_back, stamped,
        "a store whose reopen re-stamps hands every auditor the time of the \
         last restart"
    );
    // A third open, to be sure the value is not drifting one open at a time.
    drop(reopened);
    let _ = SqliteEventStore::open(db.path()).unwrap();
    let read_again: i64 = raw(db.path())
        .query_row(
            "SELECT recorded_at FROM event WHERE event_type = 'test.stamped'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(read_again, stamped);
}

/// AC-007 — the three pragmas are read back off the live connection, per handle.
///
/// A pragma that was executed is not a pragma that is in effect: SQLite silently
/// accepts one it does not recognise.
///
/// The projection store's half of AC-007 is
/// `projection_store_open_configures_its_connection` — not a link, because that
/// item does not exist in this crate's default configuration — split out so that
/// this one keeps running when `projection-store` is off.
#[test]
fn pragmas_are_in_effect_on_every_connection() {
    let db = TempDb::new("pragmas");

    let first = SqliteEventStore::open(db.path()).unwrap();
    let second = SqliteEventStore::open(db.path()).unwrap();

    for (label, settings) in [
        ("first handle", first.settings().unwrap()),
        ("second handle", second.settings().unwrap()),
    ] {
        assert_eq!(
            settings.journal_mode(),
            JOURNAL_MODE,
            "{label}: WAL is what makes two connections onto one file workable"
        );
        assert_eq!(
            settings.synchronous(),
            SYNCHRONOUS,
            "{label}: synchronous is ADR-0022's stated value"
        );
        assert_ne!(
            settings.synchronous(),
            0,
            "{label}: PRAGMA synchronous = OFF is named by CF-14 as the wrong \
             implementation the reopen rule exists to reject"
        );
        assert!(
            settings.busy_timeout_ms() > 0,
            "{label}: a zero busy timeout turns ordinary contention into a \
             conformance failure that is not about this adapter's logic"
        );
        assert_eq!(
            settings.busy_timeout_ms(),
            i64::try_from(BUSY_TIMEOUT_MS).unwrap(),
            "{label}: the timeout is finite — an unbounded handler converts a \
             livelock into a hung CI job that names no rule"
        );
    }

    // Both stores' connections are configured through the same one function, so
    // any handle it hands back runs under the same three values read off the
    // same kind of live connection. `open_configured` is compiled in under
    // `event-store` as well as under `projection-store`, so this stays here,
    // where it runs under the crate's default features.
    let shared_connection = open_configured(db.path()).unwrap();
    let settings = ConnectionSettings::read_back(&shared_connection).unwrap();
    assert_eq!(settings.journal_mode(), JOURNAL_MODE);
    assert_eq!(settings.synchronous(), SYNCHRONOUS);
    assert_eq!(
        settings.busy_timeout_ms(),
        i64::try_from(BUSY_TIMEOUT_MS).unwrap(),
        "a projection connection with no busy timeout fails immediately against \
         the event store's BEGIN IMMEDIATE write lock"
    );
}

/// AC-007, the projection store's half — that `open_configured` is genuinely the
/// path `SqliteProjectionStore::open` takes, observed rather than asserted from
/// the source.
///
/// On a file nothing else has ever touched, the journal mode left behind is the
/// one that `open` set before its own migration ran. WAL is a persistent
/// property of the file, which is what makes the observation outlive the
/// connection.
///
/// This assertion used to be a `catch_unwind` around a `todo!()`, with a message
/// telling whoever landed the projection migration to assert on the return value
/// instead. `projection-store-passes-the-borrowed-suite` landed it, so this is
/// that assertion.
///
/// # Why the `cfg` is here and not on the file
///
/// This is the only assertion in this target that names
/// `happenstance_sqlite::projection_store`, and that module is behind
/// `projection-store` — which left `default` under PS-3's verdict and ADR-0036
/// and is not coming back. Widening the file-level `#![cfg(feature =
/// "event-store")]` to cover both would take the pragma readbacks out of the
/// default build with it, which is the coverage this file exists for; leaving
/// the reference buried mid-body of a `cfg`-less test made `cargo test -p
/// happenstance-sqlite` an `error[E0433]` rather than a smaller run. The gate
/// is therefore written at the test boundary, where a reader can see which
/// feature buys which test.
#[cfg(feature = "projection-store")]
#[test]
fn projection_store_open_configures_its_connection() {
    let untouched = TempDb::new("projection-open");
    happenstance_sqlite::projection_store::SqliteProjectionStore::open(untouched.path())
        .expect("the projection store's migration has landed, and must apply cleanly");
    let mode: String = raw(untouched.path())
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .unwrap();
    assert_eq!(
        mode.to_lowercase(),
        JOURNAL_MODE,
        "SqliteProjectionStore::open must configure its connection before it \
         migrates"
    );
}

/// AC-008 — the published `# Intended schema` block is the schema.
///
/// Compared mechanically rather than by review: a reviewer reading SQL prose
/// against SQL code is exactly the check that passes by fatigue.
#[test]
fn module_doc_schema_matches_sqlite_master() {
    const SOURCE: &str = include_str!("../src/event_store.rs");

    let db = TempDb::new("module-doc");
    let _store = SqliteEventStore::open(db.path()).unwrap();

    let documented = documented_objects(SOURCE);
    assert!(
        !documented.is_empty(),
        "the `# Intended schema` block was not found in src/event_store.rs"
    );
    assert_eq!(
        documented,
        schema_objects(&raw(db.path())),
        "the published schema block and the database `migrate` creates disagree"
    );

    assert!(
        !SOURCE.contains("are absent from the schema"),
        "the contains_event_id comment still says the origin columns are absent \
         from the schema; migration 1 creates them"
    );
}

/// The `(type, name)` of every object the module doc's SQL block declares.
fn documented_objects(source: &str) -> BTreeSet<(String, String)> {
    let block: String = source
        .lines()
        .skip_while(|line| !line.contains("# Intended schema"))
        .skip_while(|line| !line.contains("```sql"))
        .skip(1)
        .take_while(|line| !line.contains("```"))
        .map(|line| line.trim_start().trim_start_matches("//!").trim())
        .collect::<Vec<_>>()
        .join(" ");

    let mut out = BTreeSet::new();
    let upper = block.to_uppercase();
    for (keyword, kind) in [("CREATE TABLE ", "table"), ("CREATE INDEX ", "index")] {
        let mut cursor = 0;
        while let Some(offset) = upper[cursor..].find(keyword) {
            let start = cursor + offset + keyword.len();
            let name = block[start..]
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .find(|word| !word.is_empty() && !word.eq_ignore_ascii_case("IF"))
                .unwrap_or_default();
            if !name.is_empty() && !name.eq_ignore_ascii_case("NOT") {
                out.insert((kind.to_owned(), name.to_owned()));
            }
            cursor = start;
        }
    }
    out
}

/// The settings type is reachable and reports what it read, which is what makes
/// every assertion above about a *live* connection rather than a constant.
#[test]
fn settings_report_what_the_connection_answered() {
    let db = TempDb::new("settings-shape");
    let connection = open_configured(db.path()).unwrap();
    let settings = ConnectionSettings::read_back(&connection).unwrap();

    assert_eq!(settings.journal_mode(), JOURNAL_MODE);
    assert_eq!(settings.synchronous(), SYNCHRONOUS);
    assert!(settings.busy_timeout_ms() > 0);
}
