//! One fixture instance is one temporary database file.
//!
//! `crates/happenstance-testkit/src/contract.rs` is the contract this satisfies:
//! **one fixture instance is one isolated backing store; each `connect()` is one
//! handle onto that store; two instances share nothing.** Pointing every instance
//! at one path is an adapter's mistake and
//! `two_fixture_instances_observe_none_of_each_others_appends` is the rule that
//! catches it, so the path is per-instance rather than per-process.
//!
//! No `tempfile`: a process-local ordinal plus `Drop` cleanup has precedent at
//! `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102`, and this
//! crate adds no dependency the workspace does not already carry. The file and
//! both of WAL's companions are removed when the fixture is dropped.

// Shared by four test targets, and no one target uses every item — two of them
// only need the module to exist so that `mod support;` compiles. A shared
// `tests/` module is compiled once per target, so the alternative to this allow
// is four copies of the fixture.
#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::SendEventStore;
use happenstance_testkit::{Capability, Fixture};
use shipped_append_condition_sql::{ProbeStore, Shape};

/// One isolated probe store, on one temporary file, evaluating guards in one
/// [`Shape`].
///
/// The shape is a **const generic over a `Shape`-valued index** rather than a
/// constructor argument, because `event_store_conformance!` takes an expression
/// building a fixture and the four invocations have to name four distinct types
/// — a `Fixture` with a runtime field would give all four the same `Store` type
/// and the same fixture type, which is fine, and would give the *file name* no
/// way to say which shape it belongs to. `SHAPE` is that name.
#[derive(Debug)]
pub struct ProbeFixture<const SHAPE: usize> {
    path: PathBuf,
    keep: bool,
}

/// The [`Shape`] a fixture's `SHAPE` index names.
///
/// A `const fn` over `usize` rather than a `Shape` const parameter, because
/// `Shape` is not a permitted const-parameter type. The index is
/// [`Shape::ALL`]'s, so the two can never disagree about the order.
const fn shape_of(index: usize) -> Shape {
    Shape::ALL[index]
}

impl<const SHAPE: usize> ProbeFixture<SHAPE> {
    /// Creates a fixture over a fresh temporary database file.
    #[must_use]
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);

        let name = format!(
            "happenstance-shipped-guard-{}-{}-{ordinal}.sqlite3",
            std::process::id(),
            shape_of(SHAPE).name(),
        );

        Self {
            path: std::env::temp_dir().join(name),
            keep: false,
        }
    }

    /// Where this fixture's database lives.
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Which shape this fixture's handles evaluate guards with.
    #[must_use]
    pub const fn shape(&self) -> Shape {
        shape_of(SHAPE)
    }

    /// Opens a handle, panicking if the environment cannot supply one.
    ///
    /// # Panics
    ///
    /// Panics rather than returning `Result`, which is what the fixture contract
    /// asks for: a fixture that cannot connect is a broken *test environment*,
    /// not a non-conformant adapter, and a `Result` here would put "the disk is
    /// full" into the same channel as "the store is wrong".
    #[must_use]
    pub fn open(&self) -> ProbeStore {
        ProbeStore::open(&self.path, shape_of(SHAPE))
            .unwrap_or_else(|err| panic!("opening {} failed: {err}", self.path.display()))
    }

    /// Opens a handle onto the same file evaluating guards in a *different*
    /// shape.
    ///
    /// The four shapes share one schema, one write path and one identity story —
    /// they differ only in the guard SQL — so one seeded log can serve all four.
    /// That is not a convenience: seeding four logs would put "which million
    /// events" back on the list of things a difference between two figures could
    /// be.
    ///
    /// # Panics
    ///
    /// Panics if the file cannot be opened.
    #[must_use]
    pub fn open_as(&self, shape: Shape) -> ProbeStore {
        ProbeStore::open(&self.path, shape)
            .unwrap_or_else(|err| panic!("opening {} failed: {err}", self.path.display()))
    }

    /// Keeps the file after this fixture is dropped.
    pub fn keep_on_drop(&mut self) {
        self.keep = true;
    }

    /// Removes the database file and both of WAL's companions.
    pub fn remove(&self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.path.clone().into_os_string();
            path.push(suffix);
            // A failure here is a leaked temporary file rather than a wrong
            // measurement, so it is ignored rather than promoted to a panic in a
            // `Drop`.
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

impl<const SHAPE: usize> Default for ProbeFixture<SHAPE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SHAPE: usize> Drop for ProbeFixture<SHAPE> {
    fn drop(&mut self) {
        if !self.keep {
            self.remove();
        }
    }
}

impl<const SHAPE: usize> Fixture for ProbeFixture<SHAPE> {
    type Store = ProbeStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, and unlike the reference fixture's it is not vacuous: the
    /// events are on disk, and [`reopen`](Fixture::reopen) checkpoints the
    /// write-ahead log into the main database before the next `connect`, so what
    /// a later handle reads is what was durably committed rather than what a
    /// shared cache still remembers.
    const REOPEN: Capability = Capability::SUPPORTED;

    fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
        // Ready rather than `async move`, and honestly so: opening a SQLite file
        // is synchronous I/O on the calling thread. `rusqlite` has no async door,
        // which is the whole reason the real adapter's read path defers a
        // `spawn_blocking` into its first poll.
        core::future::ready(self.open())
    }

    fn reopen(&self) -> impl core::future::Future<Output = ()> {
        let path = self.path.clone();
        async move {
            // A fresh connection, a full checkpoint, then dropped. The checkpoint
            // is what makes this a real reopen rather than a no-op.
            let connection = rusqlite::Connection::open(&path)
                .unwrap_or_else(|err| panic!("reopening {} failed: {err}", path.display()));
            connection
                .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
                .unwrap_or_else(|err| panic!("checkpointing {} failed: {err}", path.display()));
        }
    }
}

/// Asserts at compile time that a probe handle really is the `Send` flavour.
///
/// The `!Send` flavour would be the honest one for a Durable Object and is not
/// honest here: a `rusqlite::Connection` behind a `Mutex` crosses threads
/// perfectly well, and a store that quietly weakened that would stop exercising
/// the flavour every native adapter claims.
const fn _handles_are_send()
where
    ProbeStore: SendEventStore + Send + Sync,
{
}
