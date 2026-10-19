//! The ingest port, from outside the crate (VT-10, #108 part E).
//!
//! The spike's own tests are unit tests in `src/ingest_spike.rs`, and a unit
//! test cannot tell a `#[cfg(test)]` impl from a published one: both are in
//! scope under `cargo test`. This target is a separate crate, so it compiles
//! only when the impl ships behind the `sync` feature. That is the defect it
//! detects: an impl left behind `cfg(test)`, which this file meets as
//! `E0277`. `required-features = ["sync"]` in `Cargo.toml` keeps it out of a
//! build without the feature rather than leaving an empty binary that passes.

use happenstance_core::{
    Event, EventId, EventStore, Query, ReadOptions, RecordedAt, SequencePosition, StoreId, collect,
};
// Through the adapter's own re-export, so a consumer needs no second
// dependency line to name the port the store implements.
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};

/// Generic code binds the bare flavour and accepts the store.
fn port<S: IngestStore>(_: &S) {}

/// One foreign event goes in through the port and comes back out of `read`
/// under the identity and time its origin gave it.
#[tokio::test(flavor = "current_thread")]
async fn the_published_store_is_an_ingest_store() {
    let store = SqliteEventStore::open_in_memory().expect("an in-memory store opens");
    port(&store);

    let origin = StoreId::from_bytes([7; 16]);
    let shipped = ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::new(4).expect("four is not zero")),
        RecordedAt::from_millis(1_700_000_000_000),
        Event::new("Shipped", &b"opaque"[..]).expect("a valid event type"),
    );

    let ingested = store
        .ingest(&[IngestGroup::new(core::slice::from_ref(&shipped), &[])])
        .await
        .expect("a foreign event is ingested");
    assert_eq!(ingested.appended, 1);

    let log = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads back");
    let [landed] = log.as_slice() else {
        panic!("one event in the log, found {}", log.len());
    };
    assert_eq!(landed.id, shipped.id);
    assert_eq!(landed.recorded_at, shipped.recorded_at);
}
