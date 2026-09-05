//! The SQLite arm: one temporary file per fixture, one real connection per
//! handle.
//!
//! # This is a copy, and the copy is checked
//!
//! `happenstance-sqlite`'s own fixture lives at
//! `crates/happenstance-sqlite/tests/support/mod.rs:69` and is `pub(crate)` in
//! a test target, so it cannot be imported. This is the same fixture, rewritten,
//! and `tests/conformance_first.rs` runs the whole `event_store_conformance!`
//! suite against it before anything timed through it is kept. That is not
//! ceremony: **a wrong arm is always the fastest**, and a fixture that had
//! quietly stopped opening a second connection would post better contention
//! numbers than the real adapter can.
//!
//! # Three things the conformance rules cannot see, which the body must get
//! right
//!
//! Each of the three would pass a wrong fixture, which is why the body is the
//! evidence and the green suite is only the precondition.
//!
//! 1. **`connect()` opens a second `rusqlite::Connection`**, through
//!    [`SqliteEventStore::open`]. Returning an `Arc` clone of one in-process
//!    store passes `two_handles_observe_each_others_appends` perfectly and
//!    measures a mutex where the adapter has a file.
//! 2. **`new()` mints one fresh temporary file per instance.** Pointing every
//!    instance at one path is what
//!    `two_fixture_instances_observe_none_of_each_others_appends` exists to
//!    catch, and it would make every "cold store" figure a figure about a store
//!    the previous arm had already warmed.
//! 3. **The three ceilings are mirrored from the adapter's own constants**
//!    rather than restated as literals, so a number can never be declared here
//!    at one value and enforced there at another. This matters more in a
//!    benchmark than in a test: `happenstance_testkit::bench`'s
//!    `append_throughput` reads `MAX_EVENTS_PER_BATCH` and reports a batch above
//!    it as a *refusal* rather than as work, so a fixture that left it `None`
//!    would report the error path as throughput.
//!
//! # The one constructor that is wrong here
//!
//! [`SqliteEventStore::open_in_memory`] is what an implementer reaches for
//! first. A private in-memory database is per-*connection*, so the second
//! `connect()` would open a second, empty database — and the arm would silently
//! stop being about a file.
//!
//! # The runtime trap
//!
//! `SqliteEventStore` captures `Handle::try_current().ok()` at **construction**
//! (`crates/happenstance-sqlite/src/event_store.rs:326-331`). [`connect`] is
//! `async`, so it is always polled inside a runtime and captures a live handle;
//! anything in this crate that builds a store *outside* a future must hold
//! [`crate::runtime::enter`] across the call or its first `read` fails with
//! `NoRuntime`.
//!
//! [`SqliteEventStore::open`]: happenstance_sqlite::event_store::SqliteEventStore::open
//! [`SqliteEventStore::open_in_memory`]: happenstance_sqlite::event_store::SqliteEventStore::open_in_memory
//! [`connect`]: Fixture::connect

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::{Capability, Fixture};

/// The sidecars WAL leaves beside a database file.
///
/// Named once because three places need the same list: the constructor clearing
/// a leftover, the destructor removing its own, and
/// [`SqliteFixture::file_bytes`], which sums them to report what an arm cost on
/// disk.
const SIDECARS: [&str; 3] = ["", "-wal", "-shm"];

/// A fresh temporary database path nothing else in this process is using.
///
/// The process id plus a process-local ordinal, which is the shape
/// `crates/happenstance-testkit/tests/fixture_instruments.rs` already uses: no
/// new dependency, and no two instances in a run can collide.
fn fresh_path(prefix: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "happenstance-bench-{prefix}-{}-{ordinal}.db",
        std::process::id()
    ));
    // A leftover from a previous run would make "a fresh, isolated backing
    // store" a lie, which is the one promise the fixture contract makes — and
    // in a benchmark it would also mean the first arm of a run measured a store
    // the last run had already filled.
    remove_database(&path);
    path
}

/// Removes a database file and both its WAL sidecars, ignoring absence.
fn remove_database(path: &Path) {
    for suffix in SIDECARS {
        let mut sidecar = path.to_path_buf().into_os_string();
        sidecar.push(suffix);
        let _ = std::fs::remove_file(PathBuf::from(sidecar));
    }
}

/// One fixture instance is **one temporary SQLite file**; each `connect` is one
/// more real connection onto it; two instances share nothing.
#[derive(Debug)]
pub struct SqliteFixture {
    path: PathBuf,
    /// The handles this fixture has handed out, kept so that
    /// [`reopen`](Fixture::reopen) has something to close.
    ///
    /// A handle is an `Arc` onto one `rusqlite::Connection`, so holding a clone
    /// keeps that connection open after a caller has dropped its own — which is
    /// what makes `reopen` a reopen rather than a no-op.
    handles: Mutex<Vec<SqliteEventStore>>,
}

impl SqliteFixture {
    /// A fixture over a fresh temporary file.
    pub fn new() -> Self {
        Self {
            path: fresh_path("store"),
            handles: Mutex::new(Vec::new()),
        }
    }

    /// The file this fixture's store lives in.
    ///
    /// Public so the floor arm in [`crate::fixtures::raw`] can open the *same*
    /// file with a plain connection, which is how a raw-SQL control ends up
    /// measured against the adapter's own schema rather than against a
    /// hand-restated copy of it.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Bytes on disk: the database file plus both WAL sidecars.
    ///
    /// Reported beside the throughput figures because write amplification is a
    /// cost a consumer pays and no timing shows. Absent files count as zero, so
    /// a checkpointed store reports its main file alone.
    pub fn file_bytes(&self) -> u64 {
        SIDECARS
            .iter()
            .map(|suffix| {
                let mut sidecar = self.path.clone().into_os_string();
                sidecar.push(suffix);
                std::fs::metadata(PathBuf::from(sidecar)).map_or(0, |meta| meta.len())
            })
            .sum()
    }

    /// Folds the write-ahead log into the main database file.
    ///
    /// Between arms rather than during one: a WAL that has grown across a
    /// seeding phase makes the *next* arm's reads slower for a reason that has
    /// nothing to do with the next arm.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be opened or the checkpoint refused — a broken
    /// measurement environment, not a finding.
    pub fn checkpoint(&self) {
        let connection = rusqlite::Connection::open(&self.path)
            .expect("a broken measurement environment, not a finding");
        connection
            .pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .expect("the write-ahead log could not be checkpointed");
    }
}

impl Default for SqliteFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SqliteFixture {
    fn drop(&mut self) {
        // Close every connection before the file goes, so the sidecars are not
        // left behind on a platform that refuses to unlink an open file. A
        // benchmark run creates far more fixtures than a test run does, and a
        // leaked half-gigabyte database per arm is how a suite fills a disk.
        if let Ok(mut handles) = self.handles.lock() {
            handles.clear();
        }
        remove_database(&self.path);
    }
}

impl Fixture for SqliteFixture {
    type Store = SqliteEventStore;

    /// A second `connect` is a second `rusqlite::Connection` onto the same
    /// file, not a refcount clone.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// A SQLite file outlives every connection onto it.
    const REOPEN: Capability = Capability::SUPPORTED;

    /// Declined by scope. This suite measures throughput and cost, and a
    /// mid-batch fault path is a correctness far end that
    /// `crates/happenstance-sqlite/tests/append.rs` already exercises with a
    /// trigger installed through a second connection. Arming it here would add
    /// no figure and would put a claim about ES-35's residual falsifier into a
    /// benchmark run's output.
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "by scope: this is a measurement harness, and the fault far end is a \
         correctness question that crates/happenstance-sqlite/tests/append.rs \
         already answers with a real AFTER INSERT trigger",
    );

    /// Mirrored from the adapter's own constant rather than restated.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(SqliteEventStore::MAX_EVENT_DATA_LEN);

    /// Mirrored; see [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(SqliteEventStore::MAX_TAGS_PER_EVENT);

    /// Mirrored; see [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(SqliteEventStore::MAX_EVENTS_PER_BATCH);

    /// One more real connection onto this fixture's file.
    ///
    /// Genuinely `async` work rather than a refcount bump: it opens a file,
    /// configures the connection — WAL conversion, `synchronous = NORMAL`, a
    /// 5,000 ms busy timeout — and runs migration 1. That cost is real and it
    /// is why every criterion routine here connects in `iter_batched`'s setup
    /// closure rather than inside the measured region.
    async fn connect(&self) -> Self::Store {
        let store = SqliteEventStore::open(&self.path)
            .expect("a broken measurement environment, not a finding");
        self.handles
            .lock()
            .expect("the fixture's handle list was poisoned")
            .push(store.clone());
        store
    }

    /// Closes every connection this fixture opened, checkpoints the log, and
    /// leaves the file.
    ///
    /// The file is never deleted or recreated. A `reopen` that did that would
    /// pass `acknowledged_writes_survive_a_reopen` only by making it vacuous.
    async fn reopen(&self) {
        self.handles
            .lock()
            .expect("the fixture's handle list was poisoned")
            .clear();
        self.checkpoint();
    }
}
