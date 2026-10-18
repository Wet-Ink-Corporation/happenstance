//! The ingest oracle is an event store first: `MemoryIngestStore` passes the
//! whole event-store suite, so a rule of the ingest family that reads it back
//! reads what `MemoryEventStore` would have answered.
//!
//! Its `read` duplicates `MemoryEventStore`'s filter chain, and this mount is
//! what holds the copy to the contract.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_sync_testkit::{IngestFixture, MemoryIngestFixture, MemoryIngestHandle};
use happenstance_testkit::{Capability, Fixture};

/// The oracle's fixture, seen through the event-store family's trait.
#[derive(Debug, Clone, Default)]
struct OracleFixture(MemoryIngestFixture);

impl OracleFixture {
    fn new() -> Self {
        Self::default()
    }
}

impl Fixture for OracleFixture {
    type Store = MemoryIngestHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "MemoryIngestStore keeps its log and its held identities in process memory \
         behind one RwLock, so no durable medium outlives the process for a reopen \
         to find; building a new store is building an empty one",
    );

    const MID_BATCH_FAULT: Capability = Capability::declined(
        "MemoryIngestStore writes an append as two extends inside one write-lock \
         critical section, after every check has run, so no row of a batch is \
         written before another could fail",
    );

    const READ_FAULT: Capability = Capability::declined(
        "MemoryIngestStore answers a read from a vector it clones under the lock \
         before the stream is returned, so the stream has no fetch left that could \
         fail part way through",
    );

    const HELD_ABOVE_FRONTIER: Capability = Capability::declined(
        "MemoryIngestStore makes a row visible to every handle the moment its write \
         lock is released, so no committed row ever waits above a visibility \
         frontier",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        IngestFixture::connect(&self.0)
    }
}

happenstance_testkit::event_store_conformance!(OracleFixture::new());
