//! One fixture instance is one temporary database file.
//!
//! `crates/happenstance-testkit/src/contract.rs` is the contract this satisfies
//! and the reason the shape is what it is: **one fixture instance is one
//! isolated backing store; each `connect()` is one handle onto that store; two
//! instances share nothing.** Pointing every instance at one path is an
//! adapter's mistake and `two_fixture_instances_observe_none_of_each_others_appends`
//! is the rule that catches it, so the path is per-instance rather than
//! per-process.
//!
//! # No `tempfile`, and that is not an accident
//!
//! NF-002 holds this crate to adding no dependency any workspace manifest does
//! not already carry, and a process-local ordinal plus `Drop` cleanup has
//! precedent at `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102`.
//! The file, and both of WAL's companions, are removed when the fixture is
//! dropped.

// Shared by four test targets, and no one target uses every item: `path` is
// only reached by the durability control, `BUSY_TIMEOUT_MS` only by that and
// this module. A shared `tests/` module is compiled once per target, so the
// alternative to this allow is four copies of the fixture.
#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::SendEventStore;
use happenstance_testkit::{Capability, Fixture};

use append_condition_probes::{AppendStrategy, CandidateStore, TagStorage};

/// The busy timeout every arm runs under, in milliseconds.
///
/// **Finite and generous.** With sixty-four connections on one file, an
/// *unbounded* busy handler turns a livelock into a hung run naming no rule —
/// and there is no watchdog anywhere in the suite to notice (CF-33,
/// `crates/happenstance-testkit/src/concurrency.rs:24-43`). Five seconds is
/// generous enough that no honest contention run trips it and short enough that
/// a genuine deadlock ends the run rather than the day.
pub const BUSY_TIMEOUT_MS: u64 = 5_000;

/// One isolated candidate store, on one temporary file.
#[derive(Debug)]
pub struct CandidateFixture<A: AppendStrategy, T: TagStorage> {
    path: PathBuf,
    arms: core::marker::PhantomData<fn() -> (A, T)>,
}

impl<A: AppendStrategy, T: TagStorage> CandidateFixture<A, T> {
    /// Creates a fixture over a fresh temporary database file.
    #[must_use]
    pub fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);

        let name = format!(
            "happenstance-append-condition-{}-{}-{}-{ordinal}.sqlite3",
            std::process::id(),
            A::NAME,
            T::NAME,
        );

        Self {
            path: std::env::temp_dir().join(name),
            arms: core::marker::PhantomData,
        }
    }

    /// Where this fixture's database lives.
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Opens a handle, panicking if the environment cannot supply one.
    ///
    /// # Panics
    ///
    /// Panics rather than returning `Result`, which is what the fixture
    /// contract asks for: a fixture that cannot connect is a broken *test
    /// environment*, not a non-conformant adapter, and a `Result` here would put
    /// "the disk is full" into the same channel as "the store is wrong".
    #[must_use]
    pub fn open(&self) -> CandidateStore<A, T> {
        CandidateStore::open(&self.path, BUSY_TIMEOUT_MS)
            .unwrap_or_else(|err| panic!("opening {} failed: {err}", self.path.display()))
    }
}

impl<A: AppendStrategy, T: TagStorage> Default for CandidateFixture<A, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: AppendStrategy, T: TagStorage> Drop for CandidateFixture<A, T> {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.path.clone().into_os_string();
            path.push(suffix);
            // A failure here is a leaked temporary file rather than a wrong
            // measurement, so it is ignored rather than promoted to a panic in
            // a `Drop`.
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

impl<A: AppendStrategy, T: TagStorage> Fixture for CandidateFixture<A, T> {
    type Store = CandidateStore<A, T>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, and unlike the reference fixture's it is not vacuous.
    ///
    /// `MemoryFixture` declines this because discarding process state is
    /// indistinguishable from discarding the events. A file-backed store has
    /// the distinction the memory one lacks: the events are on disk, and
    /// [`reopen`](Fixture::reopen) checkpoints the write-ahead log into the main
    /// database before the next `connect`, so what a later handle reads is what
    /// was durably committed rather than what a shared cache still remembers.
    const REOPEN: Capability = Capability::SUPPORTED;

    fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
        // Ready rather than `async move`, and honestly so: opening a SQLite file
        // is synchronous I/O on the calling thread. `rusqlite` has no async
        // door, which is the whole reason the real adapter's read path defers a
        // `spawn_blocking` into its first poll.
        core::future::ready(self.open())
    }

    fn reopen(&self) -> impl core::future::Future<Output = ()> {
        let path = self.path.clone();
        async move {
            // A fresh connection, a full checkpoint, and then it is dropped. The
            // checkpoint is what makes this a real reopen rather than a no-op:
            // it forces the write-ahead log into the main database, so a
            // subsequent `connect` reads committed bytes rather than a
            // still-open WAL a shared cache might have been serving.
            let connection = rusqlite::Connection::open(&path)
                .unwrap_or_else(|err| panic!("reopening {} failed: {err}", path.display()));
            connection
                .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
                .unwrap_or_else(|err| panic!("checkpointing {} failed: {err}", path.display()));
        }
    }
}

/// Asserts at compile time that a candidate handle really is the `Send`
/// flavour.
///
/// The `!Send` flavour would be the honest one for a Durable Object and is not
/// honest here: a `rusqlite::Connection` behind a `Mutex` crosses threads
/// perfectly well, and a store that quietly weakened that would stop exercising
/// the flavour every native adapter claims.
const fn _handles_are_send<A: AppendStrategy, T: TagStorage>()
where
    CandidateStore<A, T>: SendEventStore + Send + Sync,
{
}
