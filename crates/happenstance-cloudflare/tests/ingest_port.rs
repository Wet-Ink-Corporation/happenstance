//! The ingest port, from outside the crate (VT-10, #108 part E).
//!
//! The port's behaviour is tested beside the impl, in
//! `src/event_store/ingest/tests/`, on `wasm32` where a store can run. This
//! target is a separate crate, so it compiles only when the store, its
//! constructors and the impl ship behind the `sync` feature: it detects any of
//! them left behind `cfg(test)`, which it meets as `E0277` or `E0425`. Plain
//! `#[test]`s, because the claim is about types and needs no JavaScript heap;
//! `required-features = ["sync"]` in `Cargo.toml` keeps the target out of a
//! build without the feature.

use happenstance_cloudflare::happenstance_core::EventStore;
// Through the adapter's own re-export, so a consumer needs no second
// dependency line to name the port the store implements.
use happenstance_cloudflare::happenstance_sync::IngestStore;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;
use happenstance_cloudflare::{CloudflareEventStoreError, CloudflareIngestStore, TableNamespace};

/// Generic code binds the bare flavours, the only ones a Durable Object store
/// has.
fn port<S: EventStore + IngestStore>() {}

/// Pins a constructor's signature: the object's raw state, borrowed.
fn built_from_state<F>(_: F)
where
    F: Fn(&DurableObjectState) -> Result<CloudflareIngestStore, CloudflareEventStoreError>,
{
}

/// Pins the namespaced constructor's signature.
fn built_from_state_in<F>(_: F)
where
    F: Fn(
        &DurableObjectState,
        &TableNamespace,
    ) -> Result<CloudflareIngestStore, CloudflareEventStoreError>,
{
}

#[test]
fn the_published_ingest_store_is_an_ingest_store() {
    port::<CloudflareIngestStore>();
}

#[test]
fn the_ingest_store_is_built_from_the_objects_state() {
    built_from_state(CloudflareIngestStore::from_object_state);
    built_from_state_in(CloudflareIngestStore::namespaced_from_object_state);
}
