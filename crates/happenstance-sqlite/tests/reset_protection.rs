//! Protection from reset: PS-18's mechanism, read back out of a real file.
//!
//! `tests/projection.rs` runs the borrowed suite, and its rule
//! `refused_reset_changes_nothing` asks one question of a protection: is a reset
//! through a handle opened *before* the protection refused, with nothing changed?
//! That is the half every adapter owes. This target asks the rest, which only an
//! adapter whose mark is **persisted in the file** can answer, and each test names
//! the plausible wrong implementation it rejects:
//!
//! * the mark outlives every handle that saw it set (a mark held per handle, or
//!   per process, passes the suite and fails here);
//! * it is scoped to one `ProjectionId`, byte for byte (a table-wide check and a
//!   case-folded key both fail);
//! * it guards `reset` only, so the protected projection's own runner keeps
//!   committing;
//! * it can be lifted, and setting or lifting it twice is not an error;
//! * a refused reset leaves its handle usable, holding no transaction or lock;
//! * and the schema marker rises to 2 only when a file first carries a mark, so
//!   a file nothing has protected stays readable by a build that predates it.
//!
//! Gated like `tests/projection.rs`, and for its reason: the read-model rows these
//! tests reset are the probe's, and the probe is `conformance`'s.

#![cfg(all(feature = "projection-store", feature = "conformance"))]

use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};
use happenstance_sqlite::projection_store::{
    SCHEMA_VERSION, SqliteProjectionStore, SqliteProjectionStoreError,
};
use happenstance_sqlite::rusqlite;

/// The probe key these tests write, distinct from every key the suite uses.
const KEY: &str = "fgas-cylinder-17";

/// The value written under [`KEY`]. Not `0` or `1`, so a default cannot pass for it.
const VALUE: u64 = 7_309;

/// One temporary SQLite file that nothing else is using, removed on drop.
///
/// Every store a test opens onto it is a real `rusqlite::Connection` through
/// [`SqliteProjectionStore::open`]: a protection that only one connection could
/// see is exactly what these tests exist to reject.
#[derive(Debug)]
struct Database {
    path: PathBuf,
}

impl Database {
    /// A fresh file, named by process id and a process-local ordinal.
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-reset-protection-{}-{ordinal}.db",
            std::process::id()
        ));
        let database = Self { path };
        database.remove();
        database
    }

    /// One more real connection onto this file, migrated by this build.
    fn open(&self) -> SqliteProjectionStore {
        SqliteProjectionStore::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter")
    }

    /// A connection onto this file that no store owns, for reading what the
    /// adapter wrote rather than what it reports.
    fn raw(&self) -> rusqlite::Connection {
        rusqlite::Connection::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter")
    }

    /// Removes the file and both write-ahead-log sidecars; absence is not a failure.
    fn remove(&self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut sidecar = self.path.clone().into_os_string();
            sidecar.push(suffix);
            if let Err(error) = std::fs::remove_file(&sidecar)
                && error.kind() != ErrorKind::NotFound
            {
                eprintln!(
                    "could not remove the test database {}: {error}",
                    PathBuf::from(sidecar).display()
                );
            }
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.remove();
    }
}

/// Commits the probe row at `position` for `id`.
async fn commit_probe(
    store: &SqliteProjectionStore,
    id: &ProjectionId,
    position: SequencePosition,
) {
    let mut batch = store.begin().await.expect("begin never fails here");
    store
        .probe_write(&mut batch, KEY, VALUE)
        .await
        .expect("queueing a statement cannot fail");
    store
        .commit(batch, id, position, Authority::Live)
        .await
        .expect("the commit should succeed");
}

/// Resets `id` with a batch that deletes every probe row.
async fn reset_probe(
    store: &SqliteProjectionStore,
    id: &ProjectionId,
) -> Result<(), ResetError<SqliteProjectionStoreError>> {
    let mut batch = store.begin().await.expect("begin never fails here");
    store
        .probe_delete_all(&mut batch)
        .await
        .expect("queueing a statement cannot fail");
    store.reset(batch, id).await
}

/// The protection is set through one handle and refuses a reset through another
/// that was opened before it, and the refused reset changes nothing.
///
/// The wrong implementation is today's store, which has no refusal path: its
/// `reset` answers `Ok` and the ledger is gone.
#[tokio::test]
async fn a_protected_projection_refuses_reset_and_changes_nothing() {
    let database = Database::new();
    let writer = database.open();
    let id = ProjectionId::from_static("fgas_ledger");
    commit_probe(&writer, &id, SequencePosition::FIRST).await;

    database
        .open()
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection succeeds");
    let outcome = reset_probe(&writer, &id).await;

    assert!(
        matches!(outcome, Err(ResetError::Refused)),
        "a protected projection must refuse its reset, and `reset` answered {outcome:?}"
    );
    let observer = database.open();
    assert_eq!(
        observer.probe_read(KEY).await.expect("the read succeeds"),
        Some(VALUE),
        "a refused reset must leave the read model as it was"
    );
    assert_eq!(
        observer.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "a refused reset must leave the checkpoint as it was"
    );
}

/// The mark is in the file, so it holds after every handle that saw it set has
/// closed, for a store opened afterwards.
///
/// The wrong implementation is a mark held by the handle, say a set behind the
/// store's `Arc`: clones share it, so a rule that protects and resets through
/// one store passes, and the next `open` has never heard of it. That is the
/// protection PS-18's `Rejects:` says anyone holding the store bypasses.
#[tokio::test]
async fn protection_survives_every_handle_closing() {
    let database = Database::new();
    let id = ProjectionId::from_static("fgas_ledger");
    {
        let store = database.open();
        commit_probe(&store, &id, SequencePosition::FIRST).await;
        store
            .protect_from_reset(&id)
            .await
            .expect("protecting a projection succeeds");
    }

    let reopened = database.open();

    assert!(
        reopened
            .is_protected_from_reset(&id)
            .await
            .expect("the read succeeds"),
        "a protection must outlive every handle that saw it set"
    );
    let outcome = reset_probe(&reopened, &id).await;
    assert!(
        matches!(outcome, Err(ResetError::Refused)),
        "a store opened after every other handle closed must still refuse, and \
         `reset` answered {outcome:?}"
    );
    assert_eq!(
        reopened.probe_read(KEY).await.expect("the read succeeds"),
        Some(VALUE)
    );
}

/// Protection is scoped to one `ProjectionId`, byte for byte (PS-17, PS-39).
///
/// Two wrong implementations: a check for *any* mark in the table, which
/// refuses every reset in a file once one projection is protected, so
/// Kestrel's `van_stock` stops resetting the day `fgas_ledger` is protected;
/// and a case-folded key, which protects `ledger` when `Ledger` was named.
#[tokio::test]
async fn protection_is_scoped_to_one_projection() {
    let database = Database::new();
    let store = database.open();
    let ledger = ProjectionId::from_static("fgas_ledger");
    let stock = ProjectionId::from_static("van_stock");
    let upper = ProjectionId::from_static("Ledger");
    let lower = ProjectionId::from_static("ledger");
    for id in [&ledger, &stock, &lower] {
        commit_probe(&store, id, SequencePosition::FIRST).await;
    }
    for id in [&ledger, &upper] {
        store
            .protect_from_reset(id)
            .await
            .expect("protecting a projection succeeds");
    }

    for id in [&stock, &lower] {
        // An empty batch: the probe rows are shared, and this test is about
        // which checkpoint moves.
        let batch = store.begin().await.expect("begin never fails here");
        let outcome = store.reset(batch, id).await;
        assert!(
            outcome.is_ok(),
            "`{}` is not protected and its reset must proceed, and answered {outcome:?}",
            id.as_str()
        );
        assert_eq!(
            store.checkpoint(id).await.expect("the read succeeds"),
            Checkpoint::NeverRun,
            "`{}` was reset",
            id.as_str()
        );
    }
    assert_eq!(
        store.checkpoint(&ledger).await.expect("the read succeeds"),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "resetting another projection must not touch the protected one"
    );
    assert!(
        !store
            .is_protected_from_reset(&lower)
            .await
            .expect("the read succeeds"),
        "protecting `Ledger` must not protect `ledger`"
    );
}

/// Protection guards `reset` and nothing else: a protected projection still
/// commits.
///
/// The wrong implementation checks the mark in `commit` too, which stops the
/// protected ledger's own runner the moment it is protected, and so makes
/// protecting the projection that matters most the way to break it.
#[tokio::test]
async fn protection_does_not_block_commit() {
    let database = Database::new();
    let store = database.open();
    let id = ProjectionId::from_static("fgas_ledger");
    let second = SequencePosition::new(2).expect("2 is non-zero");
    commit_probe(&store, &id, SequencePosition::FIRST).await;
    store
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection succeeds");

    let batch = store.begin().await.expect("begin never fails here");
    let outcome = store.commit(batch, &id, second, Authority::Live).await;

    assert!(
        outcome.is_ok(),
        "a protected projection must still commit, and `commit` answered {outcome:?}"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::Live { through: second }
    );
}

/// `allow_reset` lifts the mark, and the next reset proceeds in full.
///
/// The wrong implementation is an `allow_reset` that does nothing, which makes
/// every protection permanent unless an operator reaches past the port into an
/// adapter-owned table with raw SQL.
#[tokio::test]
async fn allow_reset_lifts_the_protection() {
    let database = Database::new();
    let store = database.open();
    let id = ProjectionId::from_static("fgas_ledger");
    commit_probe(&store, &id, SequencePosition::FIRST).await;
    store
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection succeeds");

    store
        .allow_reset(&id)
        .await
        .expect("lifting a protection succeeds");
    let outcome = reset_probe(&store, &id).await;

    assert!(
        outcome.is_ok(),
        "a lifted protection must let the reset proceed, and `reset` answered {outcome:?}"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("the read succeeds"),
        Checkpoint::NeverRun
    );
    assert_eq!(
        store.probe_read(KEY).await.expect("the read succeeds"),
        None
    );
}

/// Protecting twice and lifting what is not there are both `Ok`, and the read
/// tracks each step.
///
/// The wrong implementation is a plain `INSERT`, which answers a `UNIQUE`
/// constraint failure the second time a deploy script protects the same
/// projection.
#[tokio::test]
async fn protecting_and_allowing_are_idempotent() {
    let database = Database::new();
    let store = database.open();
    let id = ProjectionId::from_static("fgas_ledger");
    let mut observed = Vec::new();

    store
        .allow_reset(&id)
        .await
        .expect("lifting an absent protection is not an error");
    observed.push(store.is_protected_from_reset(&id).await.expect("read"));
    for _ in 0..2 {
        store
            .protect_from_reset(&id)
            .await
            .expect("protecting a protected projection is not an error");
        observed.push(store.is_protected_from_reset(&id).await.expect("read"));
    }
    for _ in 0..2 {
        store
            .allow_reset(&id)
            .await
            .expect("lifting a lifted protection is not an error");
        observed.push(store.is_protected_from_reset(&id).await.expect("read"));
    }

    assert_eq!(observed, [false, true, true, false, false]);
}

/// A refused reset leaves its handle, and the file, usable: it holds no
/// transaction and no write lock.
///
/// The wrong implementation is a hand-rolled `BEGIN IMMEDIATE` with an early
/// return that never issues `ROLLBACK`. The connection is left inside a
/// transaction, so its next write cannot begin one, and every other connection
/// waits out the busy timeout for a lock nobody will release.
#[tokio::test]
async fn a_refused_reset_leaves_the_handle_usable() {
    let database = Database::new();
    let store = database.open();
    let id = ProjectionId::from_static("fgas_ledger");
    let second = SequencePosition::new(2).expect("2 is non-zero");
    commit_probe(&store, &id, SequencePosition::FIRST).await;
    store
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection succeeds");
    let refused = reset_probe(&store, &id).await;
    assert!(matches!(refused, Err(ResetError::Refused)), "{refused:?}");

    let same_handle = store
        .commit(
            store.begin().await.expect("begin never fails here"),
            &id,
            second,
            Authority::Live,
        )
        .await;
    let other = database.open();
    let other_handle = other
        .commit(
            other.begin().await.expect("begin never fails here"),
            &ProjectionId::from_static("van_stock"),
            SequencePosition::FIRST,
            Authority::Live,
        )
        .await;

    assert!(
        same_handle.is_ok(),
        "the refusing handle must commit afterwards, and answered {same_handle:?}"
    );
    assert!(
        other_handle.is_ok(),
        "another handle must commit afterwards, and answered {other_handle:?}"
    );
}

/// A fresh file is written at schema 1 and carries the protection table; the
/// first protection raises the marker to 2, and lifting it does not lower it.
///
/// Two wrong implementations, one each way. An eager bump marks every file 2
/// on its first open by this build, so an older build refuses files that hold
/// nothing it could mishandle. No bump at all lets an older build open a file
/// with a protected projection and reset it blind.
#[tokio::test]
async fn the_schema_marker_rises_only_when_a_projection_is_protected() {
    let database = Database::new();
    let store = database.open();
    let id = ProjectionId::from_static("fgas_ledger");

    let fresh = schema_marker(&database);
    let table = has_protection_table(&database);
    store
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection succeeds");
    let protected = schema_marker(&database);
    store
        .allow_reset(&id)
        .await
        .expect("lifting a protection succeeds");
    let lifted = schema_marker(&database);

    assert!(table, "every open must create `projection_protection`");
    assert_eq!(
        (fresh, protected, lifted),
        (1, SCHEMA_VERSION, SCHEMA_VERSION),
        "the marker must stay at 1 until a protection, then rise and stay"
    );
    drop(store);
    assert!(
        SqliteProjectionStore::open(&database.path).is_ok(),
        "this build must open a file it raised"
    );
}

/// A file written before protection existed opens, gains the table, and can be
/// protected.
///
/// The file is built by hand in migration 1's shape at marker 1, with no
/// `projection_protection`, which is what every build before this one left
/// behind. The wrong implementation creates the table only for a fresh file, so
/// the first reset on an upgraded file fails with `no such table`.
#[tokio::test]
async fn a_file_from_before_protection_opens_and_can_be_protected() {
    let database = Database::new();
    database
        .raw()
        .execute_batch(
            "CREATE TABLE projection_checkpoint (
                 projection_id TEXT    NOT NULL PRIMARY KEY,
                 position      INTEGER NOT NULL,
                 authority     TEXT    NOT NULL
             ) WITHOUT ROWID;
             CREATE TABLE projection_meta (
                 k TEXT    NOT NULL PRIMARY KEY,
                 v INTEGER NOT NULL
             ) WITHOUT ROWID;
             INSERT INTO projection_meta (k, v) VALUES ('schema_version', 1);",
        )
        .expect("the test environment writes the old file, not the adapter");
    let id = ProjectionId::from_static("fgas_ledger");

    let store = database.open();
    commit_probe(&store, &id, SequencePosition::FIRST).await;
    store
        .protect_from_reset(&id)
        .await
        .expect("protecting a projection on an upgraded file succeeds");
    let outcome = reset_probe(&store, &id).await;

    assert!(
        matches!(outcome, Err(ResetError::Refused)),
        "an upgraded file must refuse a protected reset, and `reset` answered {outcome:?}"
    );
    assert_eq!(
        store.probe_read(KEY).await.expect("the read succeeds"),
        Some(VALUE)
    );
}

/// The projection schema marker as the file stores it.
fn schema_marker(database: &Database) -> u32 {
    database
        .raw()
        .query_row(
            "SELECT v FROM projection_meta WHERE k = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .expect("every migrated file carries a marker")
}

/// Whether the file has a `projection_protection` table.
fn has_protection_table(database: &Database) -> bool {
    database
        .raw()
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master \
             WHERE type = 'table' AND name = 'projection_protection')",
            [],
            |row| row.get(0),
        )
        .expect("the schema is readable")
}
