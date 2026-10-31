//! A store on a connection the caller opened, configured and keeps (#359).
//!
//! [`SqliteEventStore::open`] and [`SqliteEventStore::remint_identity`] both go
//! through `open_configured`, which switches the file to WAL. A consumer whose
//! files live where WAL breaks keeps the file in `journal_mode = TRUNCATE` and
//! owns `PRAGMA user_version` itself, so it reaches the store through
//! [`SqliteEventStore::open_on`] and [`SqliteEventStore::remint_identity_on`].
//! Every test here opens the file the way such a consumer does and then asks
//! the file, through a second raw connection, whether anything it owned moved.
//!
//! The wrong implementation each test rejects is one that "helpfully" calls
//! `configure` or `ensure_wal` on the connection it was handed: the journal mode
//! would read `wal`, and `-wal` and `-shm` files would appear beside the file.

#![cfg(feature = "event-store")]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencedEvent, StoreId, collect};
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;

/// The `user_version` the consumer owns, set before the store is opened.
const CALLERS_USER_VERSION: i64 = 7;

/// A temporary database path that deletes itself and every sidecar on drop.
#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        // Relaxed: only the uniqueness of the value matters, and no other
        // memory is published through it.
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-caller-{label}-{}-{ordinal}.db",
            std::process::id()
        ));
        let db = Self(path);
        db.remove_all();
        db
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn sidecar(&self, suffix: &str) -> PathBuf {
        let mut path = self.0.clone().into_os_string();
        path.push(suffix);
        PathBuf::from(path)
    }

    /// Best effort by design: a file that was never created is not a failure.
    fn remove_all(&self) {
        for suffix in ["", "-wal", "-shm", "-journal"] {
            if let Err(error) = std::fs::remove_file(self.sidecar(suffix)) {
                assert_eq!(
                    error.kind(),
                    std::io::ErrorKind::NotFound,
                    "could not clear {suffix} beside {}",
                    self.0.display()
                );
            }
        }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm", "-journal"] {
            // Drop cannot report, and a leftover temp file harms nothing.
            drop(std::fs::remove_file(self.sidecar(suffix)));
        }
    }
}

/// `synchronous = EXTRA`, which `configure` never sets (it sets `NORMAL`).
const CALLERS_SYNCHRONOUS: i64 = 3;

/// A busy timeout `configure` never sets (it sets fifteen seconds).
const CALLERS_BUSY_TIMEOUT_MS: i64 = 1_234;

/// The connection a TRUNCATE-only consumer opens: the journal mode,
/// `synchronous` and the busy timeout set to values `configure` never
/// chooses, plus `user_version`, so a store that reconfigured any of them is
/// caught by [`assert_callers_settings_held`].
fn callers_connection(path: &Path) -> Connection {
    let connection = Connection::open(path).expect("the temp file opens");
    let mode: String = connection
        .query_row("PRAGMA journal_mode = TRUNCATE", [], |row| row.get(0))
        .expect("journal_mode answers");
    assert_eq!(mode, "truncate", "the caller's setup took");
    connection
        .pragma_update(None, "synchronous", CALLERS_SYNCHRONOUS)
        .expect("synchronous is settable");
    connection
        .pragma_update(None, "busy_timeout", CALLERS_BUSY_TIMEOUT_MS)
        .expect("busy_timeout is settable");
    connection
        .pragma_update(None, "user_version", CALLERS_USER_VERSION)
        .expect("user_version is writable");
    connection
}

/// Asserts that the caller still owns what it set: TRUNCATE on the store's own
/// connection, no WAL recorded in the file, `user_version` untouched, and no
/// WAL sidecar on disk.
///
/// The journal mode is asked twice because SQLite stores it in two places.
/// TRUNCATE is per connection, so only the store's connection can answer for
/// it; a fresh connection reads the default. WAL is the one mode written into
/// the file header, so a fresh connection is what sees a store that switched it.
fn assert_callers_settings_held(store: &SqliteEventStore, db: &TempDb) {
    let settings = store.settings().expect("the store reads its settings");
    assert_eq!(
        settings.journal_mode(),
        "truncate",
        "the store changed its connection's journal mode"
    );
    assert_eq!(
        settings.synchronous(),
        CALLERS_SYNCHRONOUS,
        "the store changed synchronous"
    );
    assert_eq!(
        settings.busy_timeout_ms(),
        CALLERS_BUSY_TIMEOUT_MS,
        "the store changed the busy timeout"
    );
    let inspect = Connection::open(db.path()).expect("a second connection opens");
    let mode: String = inspect
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("journal_mode answers");
    assert_ne!(mode, "wal", "the store switched the file to WAL");
    let version: i64 = inspect
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .expect("user_version answers");
    assert_eq!(
        version, CALLERS_USER_VERSION,
        "the store wrote user_version"
    );
    for suffix in ["-wal", "-shm"] {
        assert!(
            !db.sidecar(suffix).exists(),
            "a {suffix} file appeared beside a TRUNCATE store"
        );
    }
}

fn event(kind: &str) -> Event {
    Event::new(kind, &b"{}"[..]).expect("a valid event type")
}

async fn log(store: &SqliteEventStore) -> Vec<SequencedEvent> {
    collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads back")
}

/// A fresh file gets the schema and an identity, and the caller's settings
/// survive it and an append.
#[tokio::test(flavor = "current_thread")]
async fn open_on_creates_the_store_without_touching_the_callers_settings() {
    let db = TempDb::new("fresh");
    let store = SqliteEventStore::open_on(callers_connection(db.path()))
        .expect("open_on migrates a fresh file");

    store
        .append(&[event("Opened")], None)
        .await
        .expect("an append on the caller's connection commits");

    let [only] = log(&store).await.try_into().expect("one event in the log");
    assert_eq!(only.id.store(), store.store_id());
    assert_callers_settings_held(&store, &db);
}

/// `open_on` of a file that already holds a store reads its identity rather
/// than minting a second one.
#[tokio::test(flavor = "current_thread")]
async fn open_on_keeps_an_existing_stores_identity() {
    let db = TempDb::new("existing");
    let first = SqliteEventStore::open_on(callers_connection(db.path()))
        .expect("open_on migrates a fresh file")
        .store_id();
    let second = SqliteEventStore::open_on(callers_connection(db.path()))
        .expect("open_on accepts a migrated file")
        .store_id();
    let wrapped = SqliteEventStore::new(callers_connection(db.path()))
        .expect("new accepts a file open_on migrated");

    assert_eq!(first, second, "a second open_on minted a fresh identity");
    assert_eq!(first, wrapped.store_id());
    assert_callers_settings_held(&wrapped, &db);
}

/// A re-mint on the caller's connection replaces the identity, and the store
/// wrapped afterwards stamps under it, with the caller's settings intact.
#[tokio::test(flavor = "current_thread")]
async fn remint_identity_on_keeps_the_callers_settings() {
    let db = TempDb::new("remint");
    let before = SqliteEventStore::open_on(callers_connection(db.path()))
        .expect("open_on migrates a fresh file")
        .store_id();

    let mut connection = callers_connection(db.path());
    let minted =
        SqliteEventStore::remint_identity_on(&mut connection).expect("the re-mint commits");
    let store = SqliteEventStore::new(connection).expect("new wraps the re-minted file");

    assert_ne!(before, minted, "a re-mint must produce a fresh value");
    assert_eq!(store.store_id(), minted);
    store
        .append(&[event("AfterRemint")], None)
        .await
        .expect("an append after the re-mint commits");
    let [only] = log(&store).await.try_into().expect("one event in the log");
    assert_eq!(only.id.store(), minted);
    assert_callers_settings_held(&store, &db);
}

/// A re-mint on a connection to a file that holds no store yet creates it, as
/// the path form does.
#[test]
fn remint_identity_on_migrates_a_fresh_file() {
    let db = TempDb::new("remint-fresh");
    let mut connection = callers_connection(db.path());
    let minted =
        SqliteEventStore::remint_identity_on(&mut connection).expect("the re-mint commits");
    let store = SqliteEventStore::new(connection).expect("the file now holds a store");
    assert_eq!(store.store_id(), minted);
    assert_callers_settings_held(&store, &db);
}

#[cfg(feature = "sync")]
mod ingest {
    use super::{
        Connection, SequencedEvent, SqliteEventStore, StoreId, TempDb,
        assert_callers_settings_held, callers_connection, event, log,
    };
    use happenstance_core::{EventId, EventStore, RecordedAt, SequencePosition};
    use happenstance_sqlite::happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};

    /// The brief's journal: 300 events from one foreign origin.
    const JOURNAL_LEN: u64 = 300;

    fn journal(origin: StoreId) -> Vec<ReplicatedEvent> {
        (1..=JOURNAL_LEN)
            .map(|position| {
                let position = SequencePosition::new(position).expect("positions start at one");
                let millis = 1_700_000_000_000_i64
                    .checked_add_unsigned(position.get())
                    .expect("a small offset fits");
                ReplicatedEvent::new(
                    EventId::new(origin, position),
                    RecordedAt::from_millis(millis),
                    event("Journalled"),
                )
            })
            .collect()
    }

    /// Ingests `events` in chunks the store accepts, returning the summed
    /// `(appended, skipped)`.
    async fn ingest_all(store: &SqliteEventStore, events: &[ReplicatedEvent]) -> (usize, usize) {
        let mut totals = (0, 0);
        for chunk in events.chunks(SqliteEventStore::MAX_EVENTS_PER_BATCH) {
            let ingested = store
                .ingest(&[IngestGroup::new(chunk, &[])])
                .await
                .expect("a chunk within the ceiling ingests");
            totals.0 += ingested.appended;
            totals.1 += ingested.skipped;
        }
        totals
    }

    fn shipped(log: &[SequencedEvent]) -> Vec<ReplicatedEvent> {
        log.iter()
            .map(|held| ReplicatedEvent::new(held.id, held.recorded_at, held.event.clone()))
            .collect()
    }

    /// The same journal twice: the second run is all skips and leaves the
    /// file byte-identical, under TRUNCATE throughout.
    #[tokio::test(flavor = "current_thread")]
    async fn a_reimported_journal_is_all_skips_and_writes_nothing() {
        let db = TempDb::new("journal");
        let store = SqliteEventStore::open_on(callers_connection(db.path()))
            .expect("open_on migrates a fresh file");
        let journal = journal(StoreId::from_bytes([0x5C; 16]));

        let first = ingest_all(&store, &journal).await;
        assert_eq!(first, (300, 0));
        let bytes = std::fs::read(db.path()).expect("the file reads");

        let second = ingest_all(&store, &journal).await;
        assert_eq!(second, (0, 300));
        assert!(
            std::fs::read(db.path()).expect("the file reads") == bytes,
            "a re-import that appended nothing changed the file"
        );
        assert_callers_settings_held(&store, &db);
    }

    /// Copy a store, re-mint the copy on the caller's connection, append to
    /// both, then cross-import: every event lands once in each, under the
    /// identity it was first given, and a local append afterwards still
    /// takes the next local position.
    #[tokio::test(flavor = "current_thread")]
    async fn a_reminted_copy_and_its_original_cross_import_once_each() {
        let original_db = TempDb::new("original");
        let copy_db = TempDb::new("copy");
        let original = SqliteEventStore::open_on(callers_connection(original_db.path()))
            .expect("open_on migrates a fresh file");
        original
            .append(&[event("BeforeCopy"), event("BeforeCopy")], None)
            .await
            .expect("the shared history commits");
        std::fs::copy(original_db.path(), copy_db.path()).expect("the file copies");

        let mut connection: Connection = callers_connection(copy_db.path());
        SqliteEventStore::remint_identity_on(&mut connection).expect("the copy re-mints");
        let copy = SqliteEventStore::new(connection).expect("the copy wraps");
        assert_ne!(copy.store_id(), original.store_id());

        original
            .append(&[event("OnlyInOriginal")], None)
            .await
            .expect("the original appends");
        copy.append(&[event("OnlyInCopy")], None)
            .await
            .expect("the copy appends");

        let into_copy = ingest_all(&copy, &shipped(&log(&original).await)).await;
        let into_original = ingest_all(&original, &shipped(&log(&copy).await)).await;
        assert_eq!(into_copy, (1, 2), "the shared history is held, one is new");
        assert_eq!(
            into_original,
            (1, 3),
            "everything but the copy's own is held"
        );

        let original_log = log(&original).await;
        let copy_log = log(&copy).await;
        let ids = |log: &[SequencedEvent]| {
            let mut ids: Vec<EventId> = log.iter().map(|held| held.id).collect();
            ids.sort_unstable();
            ids
        };
        assert_eq!(
            original_log.len(),
            4,
            "an event landed twice in the original"
        );
        assert_eq!(ids(&original_log), ids(&copy_log));

        let tail = original_log.last().expect("the log is not empty").position;
        let next = original
            .append(&[event("AfterImport")], None)
            .await
            .expect("a local append after an ingest is not blocked");
        assert!(next > tail, "a local append did not take the next position");
        assert!(
            original
                .contains_event_id(EventId::new(original.store_id(), next))
                .await
                .expect("the store answers"),
            "the local append was not stamped under the store's own identity"
        );
        assert_callers_settings_held(&original, &original_db);
        assert_callers_settings_held(&copy, &copy_db);
    }
}
