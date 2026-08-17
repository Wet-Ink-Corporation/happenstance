//! The bar, run whole against a real file on disk.
//!
//! An adapter that compiles is not an adapter. This target is where
//! `happenstance-sqlite` stops being an instrument: one `SqliteFixture`, one
//! macro invocation, and every rule in `for_each_event_store_rule!` emitted as
//! its own `#[tokio::test]` driven against SQLite.
//!
//! # What the fixture is made of, which is where three criteria are won or lost
//!
//! The rules cannot see any of this. Each of the three would pass a wrong
//! fixture, which is why the fixture's *body* is the evidence and the green run
//! is only the precondition:
//!
//! * **`connect()` opens a second `rusqlite::Connection`**, through
//!   [`SqliteEventStore::open`]. Returning an `Arc` clone of one in-process
//!   store passes `two_handles_observe_each_others_appends` perfectly and fills
//!   none of the handle-multiplicity axis the runbook records as empty — *"every
//!   fixture still hands out refcount clones of one in-process object, so no
//!   connection has been opened twice."*
//! * **`new()` mints one fresh temporary file per instance.** Pointing every
//!   instance at one path is the adapter mistake `two_fixture_instances_observe_none_of_each_others_appends`
//!   exists to catch, and no testkit meta-test over the testkit's own fixture
//!   could ever see it.
//! * **The three ceilings are stated, and mirrored from the adapter's own
//!   constants** rather than restated as literals. Leaving them `None` produces
//!   a *green* suite in which `append_reports_exceeded_store_limits` reports a
//!   skip — a fact-shaped skip indistinguishable from a pass to anyone reading
//!   only the exit code.
//!
//! [`SqliteEventStore::open`]: happenstance_sqlite::event_store::SqliteEventStore::open
//!
//! # The one thing that is not the right constructor
//!
//! [`SqliteEventStore::open_in_memory`] is what an implementer reaches for
//! first, and it is wrong here: a private in-memory database is
//! per-*connection*, so the second `connect()` would open a second, empty
//! database and `two_handles_observe_each_others_appends` — a MUST that panics
//! rather than skips — would fail.
//!
//! [`SqliteEventStore::open_in_memory`]: happenstance_sqlite::event_store::SqliteEventStore::open_in_memory
//!
//! The tokio harness, so: native only.

#![cfg(all(feature = "event-store", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::{Capability, Fixture};

/// One fixture instance is **one temporary SQLite file**; each `connect` is one
/// more real connection onto it; two instances share nothing.
#[derive(Debug)]
pub struct SqliteFixture {
    path: PathBuf,
    /// The handles this fixture has handed out, kept so that
    /// [`reopen`](Fixture::reopen) has something to close.
    ///
    /// A handle is an `Arc` onto one `rusqlite::Connection`, so holding a clone
    /// keeps that connection open after the rule has dropped its own — which is
    /// precisely what makes `reopen` a *reopen* rather than a no-op.
    handles: Mutex<Vec<SqliteEventStore>>,
}

impl SqliteFixture {
    /// A fixture over a fresh temporary file that nothing else is using.
    ///
    /// A process-local ordinal plus the process id, in the shape
    /// `crates/happenstance-testkit/tests/fixture_instruments.rs` already uses:
    /// no new dependency, and no two instances in a run can collide on a path.
    #[must_use]
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-conformance-{}-{ordinal}.db",
            std::process::id()
        ));
        // A leftover from a previous run would make "a fresh, isolated backing
        // store" a lie, which is the one promise the fixture contract makes.
        for suffix in ["", "-wal", "-shm"] {
            let mut sidecar = path.clone().into_os_string();
            sidecar.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(sidecar));
        }
        Self {
            path,
            handles: Mutex::new(Vec::new()),
        }
    }
}

impl Default for SqliteFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SqliteFixture {
    fn drop(&mut self) {
        // Close every connection before the file goes, so the WAL sidecars are
        // not left behind on a platform that refuses to unlink an open file.
        if let Ok(mut handles) = self.handles.lock() {
            handles.clear();
        }
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.path.clone().into_os_string();
            path.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

impl Fixture for SqliteFixture {
    type Store = SqliteEventStore;

    /// A MUST, and this adapter meets it for real: a second `connect` is a
    /// second `rusqlite::Connection` onto the same file, not a refcount clone.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// The durability far end. A SQLite file outlives every connection onto it,
    /// so discarding process state and reading the store again is exactly what
    /// this fixture can do.
    const REOPEN: Capability = Capability::SUPPORTED;

    /// Declined, in this adapter's own words rather than the trait's generic
    /// default — because inheriting the default would still be green and would
    /// put a sentence about *some* store into *this* adapter's CI log.
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "this adapter writes a batch inside one BEGIN IMMEDIATE transaction and \
         has no supported way to make SQLite fail between two of its rows: a \
         trigger or a CHECK armed for one write would be schema this store does \
         not have, and killing the connection mid-statement is not something \
         rusqlite offers a caller. It supplies the reopen far end instead, and \
         ES-35 keeps a live falsifier this project cannot retire — a store that \
         loses a write to a fault rather than to an instruction",
    );

    /// Mirrored from the adapter's own constant rather than restated, so a
    /// number can never be declared here at one value and enforced there at
    /// another.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(SqliteEventStore::MAX_EVENT_DATA_LEN);

    /// Mirrored; see [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(SqliteEventStore::MAX_TAGS_PER_EVENT);

    /// Mirrored; see [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(SqliteEventStore::MAX_EVENTS_PER_BATCH);

    /// One more real connection onto this fixture's file.
    ///
    /// Genuinely `async` work rather than a refcount bump: it opens a file,
    /// configures the connection and runs migration 1. `SqliteEventStore::open`
    /// is the constructor, never `open_in_memory` — see the module docs for what
    /// that mistake costs.
    ///
    /// It panics rather than returning a `Result`, per the trait: a fixture that
    /// cannot connect is a broken **test environment**, not a non-conformant
    /// adapter, and putting "the database is down" into the same channel as "the
    /// adapter is wrong" leaves the suite's own messages guessing.
    async fn connect(&self) -> Self::Store {
        let store = SqliteEventStore::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter");
        self.handles
            .lock()
            .expect("the fixture's handle list was poisoned")
            .push(store.clone());
        store
    }

    /// Closes every connection this fixture opened, and leaves the file.
    ///
    /// Two things, and both are needed for the reopen to mean anything:
    ///
    /// 1. **Every handle is dropped**, which closes the underlying
    ///    `rusqlite::Connection`s. The rules drop their own handle before
    ///    calling this, so afterwards nothing in the process holds the database
    ///    open — every scrap of connection-level state is gone.
    /// 2. **The write-ahead log is checkpointed and truncated.** A commit under
    ///    WAL lands in the `-wal` sidecar, so a fixture that merely dropped its
    ///    handles would still be reading a log another connection had written.
    ///    Folding it into the main database file first is what makes the
    ///    subsequent `connect` observe *only what was durably committed*.
    ///
    /// The file itself is never deleted or recreated. A `reopen` that did that
    /// would pass `acknowledged_writes_survive_a_reopen` only by making it
    /// vacuous.
    async fn reopen(&self) {
        self.handles
            .lock()
            .expect("the fixture's handle list was poisoned")
            .clear();

        let connection = rusqlite::Connection::open(&self.path)
            .expect("a broken test environment, not a non-conformant adapter");
        connection
            .pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .expect("the write-ahead log could not be checkpointed");
        drop(connection);
    }
}

happenstance_testkit::event_store_conformance!(SqliteFixture::new());
