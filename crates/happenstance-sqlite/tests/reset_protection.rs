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
use happenstance_sqlite::projection_store::{SqliteProjectionStore, SqliteProjectionStoreError};

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
