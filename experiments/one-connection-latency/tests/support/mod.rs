//! One fixture instance is one temporary database file, at one page size.
//!
//! `crates/happenstance-testkit/src/contract.rs` is the contract this satisfies:
//! **one fixture instance is one isolated backing store; each `connect()` is one
//! handle onto that store; two instances share nothing.** The page size is a
//! const parameter rather than a field because it is a parameter of the *type*
//! being measured — `Replica<64>` and `Replica<512>` are two adapters as far as
//! the suite is concerned, and CONTROL 2 requires the suite to have been green
//! against each of them before either is timed.

// Compiled once per test target that includes it, and no one target reaches
// every item.
#![allow(dead_code)]

use happenstance_core::SendEventStore;
use happenstance_testkit::{Capability, Fixture};
use one_connection_latency::replica::Replica;
use one_connection_latency::workload::Scratch;

/// One isolated replica store, on one temporary file, at page size `PAGE`.
#[derive(Debug)]
pub struct ReplicaFixture<const PAGE: usize> {
    scratch: Scratch,
}

impl<const PAGE: usize> ReplicaFixture<PAGE> {
    /// Creates a fixture over a fresh temporary database file.
    #[must_use]
    pub fn new() -> Self {
        Self {
            scratch: Scratch::new(&format!("conformance-p{PAGE}")),
        }
    }

    /// Opens a handle, panicking if the environment cannot supply one.
    ///
    /// # Panics
    ///
    /// Panics rather than returning `Result`, which is what the fixture contract
    /// asks for: a fixture that cannot connect is a broken *test environment*,
    /// not a non-conformant adapter.
    #[must_use]
    pub fn open(&self) -> Replica<PAGE> {
        Replica::open(self.scratch.path()).unwrap_or_else(|err| {
            panic!("opening {} failed: {err}", self.scratch.path().display())
        })
    }
}

impl<const PAGE: usize> Default for ReplicaFixture<PAGE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const PAGE: usize> Fixture for ReplicaFixture<PAGE> {
    type Store = Replica<PAGE>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, for the same reason `experiments/append-condition`'s fixture
    /// supports it: the events are on disk, and [`reopen`](Fixture::reopen)
    /// checkpoints the write-ahead log into the main database before the next
    /// `connect`, so what a later handle reads is what was durably committed
    /// rather than what a shared cache still remembers.
    const REOPEN: Capability = Capability::SUPPORTED;

    fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
        // Ready rather than `async move`, and honestly so: opening a SQLite file
        // is synchronous I/O on the calling thread.
        core::future::ready(self.open())
    }

    fn reopen(&self) -> impl core::future::Future<Output = ()> {
        let path = self.scratch.path().to_path_buf();
        async move {
            let connection = rusqlite::Connection::open(&path)
                .unwrap_or_else(|err| panic!("reopening {} failed: {err}", path.display()));
            connection
                .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
                .unwrap_or_else(|err| panic!("checkpointing {} failed: {err}", path.display()));
        }
    }
}

/// Asserts at compile time that a replica handle really is the `Send` flavour.
const fn _handles_are_send<const PAGE: usize>()
where
    Replica<PAGE>: SendEventStore + Send + Sync,
{
}
