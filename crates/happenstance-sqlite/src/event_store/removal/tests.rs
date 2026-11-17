//! The removal record read back out of SQLite: what the trigger writes, what
//! a rollback takes back, and the plan of the probe ingest runs under the
//! write lock.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rusqlite::Connection;

use crate::event_store::SqliteEventStore;

/// A store's file, deleted with its WAL sidecars on drop, so a second
/// connection can delete rows as an operator would.
struct TempFile(PathBuf);

impl TempFile {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-removal-{}-{}.db",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let file = Self(path);
        file.remove();
        file
    }

    /// Deletes the file and its sidecars where they exist. A failure panics,
    /// except while a test is already unwinding, where a second panic would
    /// abort and hide the first.
    fn remove(&self) {
        for suffix in ["", "-wal", "-shm"] {
            // An owned path per sidecar: `push` needs one to extend.
            let mut path = self.0.clone().into_os_string();
            path.push(suffix);
            let path = PathBuf::from(path);
            if path.exists()
                && let Err(error) = std::fs::remove_file(&path)
                && !std::thread::panicking()
            {
                panic!("could not delete {}: {error}", path.display());
            }
        }
    }

    /// A second connection onto the file, opened by the test, not the store.
    fn raw(&self) -> Connection {
        Connection::open(&self.0).expect("the temporary file opens")
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        self.remove();
    }
}

/// Every `(origin_store, origin_position)` the record holds, in key order.
fn record(connection: &Connection) -> Vec<(Vec<u8>, i64)> {
    let mut statement = connection
        .prepare(
            "SELECT origin_store, origin_position FROM event_removed \
             ORDER BY origin_store, origin_position",
        )
        .expect("the record table exists");
    let rows = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .expect("the record reads");
    rows.collect::<Result<_, _>>()
        .expect("every record row reads")
}

/// Every `(position, origin_store, origin_position)` in the log.
fn origins(connection: &Connection) -> Vec<(i64, Vec<u8>, i64)> {
    let mut statement = connection
        .prepare("SELECT position, origin_store, origin_position FROM event ORDER BY position")
        .expect("the log table exists");
    let rows = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .expect("the log reads");
    rows.collect::<Result<_, _>>().expect("every row reads")
}

/// Deletes the event at `position` and its tag rows, on `connection`.
fn delete(connection: &Connection, position: i64) {
    connection
        .execute("DELETE FROM event_tag WHERE position = ?1", [position])
        .expect("the tag rows go");
    let deleted = connection
        .execute("DELETE FROM event WHERE position = ?1", [position])
        .expect("the event row goes");
    assert_eq!(deleted, 1, "position {position} was held");
}

/// Two own events appended, one foreign event ingested; returns the store
/// and the log as the file holds it.
#[cfg(feature = "sync")]
async fn own_and_foreign(file: &TempFile) -> (SqliteEventStore, Vec<(i64, Vec<u8>, i64)>) {
    use happenstance_core::{Event, EventId, EventStore, RecordedAt, SequencePosition, StoreId};
    use happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};

    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let event = |kind: &str| Event::new(kind, &b"{}"[..]).expect("a valid event type");
    store
        .append(&[event("Own"), event("Own")], None)
        .await
        .expect("a local append lands");
    let shipped = [ReplicatedEvent::new(
        EventId::new(
            StoreId::from_bytes([0xF0; 16]),
            SequencePosition::new(7).expect("seven is not zero"),
        ),
        RecordedAt::from_millis(1),
        event("Foreign"),
    )];
    let ingested = IngestStore::ingest(&store, &[IngestGroup::new(&shipped, &[])])
        .await
        .expect("a foreign event lands");
    assert_eq!(ingested.appended, 1);
    let log = origins(&file.raw());
    assert_eq!(log.len(), 3, "two own events and one foreign: {log:?}");
    (store, log)
}

/// SY-40: a `DELETE` from a connection the store never saw records the
/// identity of every row it deleted, own and foreign alike, under the origin
/// pair the row carried, and nothing else.
///
/// **Named wrong schema, checked red:** no trigger (the shape `0.7.0`
/// shipped); the record stays empty and the first assertion fails.
#[cfg(feature = "sync")]
#[tokio::test]
async fn a_raw_delete_records_each_removed_identity() {
    let file = TempFile::new();
    let (store, log) = own_and_foreign(&file).await;
    let &[(first_own, ..), _, (foreign, ..)] = &log[..] else {
        panic!("three rows: {log:?}");
    };

    let operator = file.raw();
    delete(&operator, first_own);
    delete(&operator, foreign);

    let mut expected: Vec<(Vec<u8>, i64)> = log
        .into_iter()
        .filter(|&(position, ..)| position == first_own || position == foreign)
        .map(|(_, store, origin)| (store, origin))
        .collect();
    expected.sort();
    assert_eq!(
        record(&operator),
        expected,
        "the record is not exactly the deleted rows' identities"
    );
    assert_eq!(origins(&operator).len(), 1, "one row is left");
    drop(store);
}

/// SY-40: a `DELETE` that rolls back records nothing, because the trigger's
/// insert is in the deleting transaction. A record written outside it would
/// mark a held event removed, and ingest would then skip it for ever.
#[test]
fn a_rolled_back_delete_records_nothing() {
    let file = TempFile::new();
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let mut operator = file.raw();
    operator
        .execute_batch(
            "INSERT INTO event (event_type, data, tags, origin_store, origin_position, recorded_at) \
             VALUES ('Held', X'7B7D', X'', X'00112233445566778899AABBCCDDEEFF', 1, 0);",
        )
        .expect("a stamped row goes in");
    let position: i64 = operator
        .query_row("SELECT max(position) FROM event", [], |row| row.get(0))
        .expect("the row is there");

    let transaction = operator.transaction().expect("a transaction opens");
    delete(&transaction, position);
    assert_eq!(
        record(&transaction).len(),
        1,
        "inside the transaction the trigger has recorded the delete"
    );
    transaction.rollback().expect("the delete rolls back");

    assert_eq!(
        record(&operator),
        Vec::new(),
        "a rolled-back delete left a record"
    );
    assert_eq!(origins(&operator).len(), 1, "the row is still held");
    drop(store);
}

/// A row with no origin, as a pre-identity build could leave, is deleted
/// without a record: the trigger's `WHEN` passes it by rather than failing the
/// `DELETE` on the record's `NOT NULL`.
#[test]
fn deleting_an_unstamped_row_records_nothing_and_succeeds() {
    let file = TempFile::new();
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let operator = file.raw();
    operator
        .execute_batch(
            "INSERT INTO event (event_type, data, tags, recorded_at) \
             VALUES ('Unstamped', X'7B7D', X'', 0);",
        )
        .expect("an unstamped row goes in");
    let position: i64 = operator
        .query_row("SELECT max(position) FROM event", [], |row| row.get(0))
        .expect("the row is there");

    delete(&operator, position);

    assert_eq!(record(&operator), Vec::new());
    drop(store);
}

/// The probe runs under the write lock, once per foreign event of an ingest
/// into a store that has removed anything, so it must seek the record's
/// primary key and never scan it: the `WATERMARK_SQL` precedent.
///
/// **Named wrong statement, checked red:** the probe keyed on
/// `origin_position` alone, which the composite key cannot seek without its
/// leading column; SQLite plans `SCAN event_removed` and both assertions
/// would fail, the first one first.
#[cfg(feature = "sync")]
#[test]
fn the_removal_probe_seeks_the_primary_key() {
    let mut connection = Connection::open_in_memory().expect("an in-memory database");
    SqliteEventStore::migrate(&mut connection).expect("migrating must succeed");

    let mut statement = connection
        .prepare(&format!("EXPLAIN QUERY PLAN {}", super::PROBE_SQL))
        .expect("the statement prepares");
    let rows: Vec<String> = statement
        .query_map(rusqlite::params![&[0u8; 16][..], 1], |row| row.get(3))
        .expect("the plan reads")
        .collect::<Result<_, _>>()
        .expect("every plan row reads");

    assert!(
        rows.iter()
            .any(|detail| detail.contains("event_removed") && detail.contains("PRIMARY KEY")),
        "the probe does not seek the record's primary key: {rows:?}"
    );
    assert!(
        !rows
            .iter()
            .any(|detail| detail.starts_with("SCAN event_removed")),
        "the probe scans under the write lock: {rows:?}"
    );
}
