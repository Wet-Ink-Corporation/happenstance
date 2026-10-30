//! The ingest port, from outside the crate (VT-10, #108 part E).
//!
//! The port's behaviour is tested beside the impl, in
//! `src/event_store/ingest/tests.rs`, where a unit test cannot tell a
//! `#[cfg(test)]` impl from a published one. This target is a separate crate,
//! so it compiles only when the impl ships behind the `sync` feature: it
//! detects an impl left behind `cfg(test)`, which it meets as `E0277`. It needs
//! no endpoint, because the claim is about types. `required-features = ["sync"]`
//! in `Cargo.toml` keeps it out of a build without the feature.

use happenstance_neon::{NeonEventStore, NullTransport};
// Through the adapter's own re-export, so a consumer needs no second
// dependency line to name the port the store implements.
use happenstance_neon::happenstance_sync::IngestStore;

/// Generic code binds the bare flavour, the one this store implements.
fn port<S: IngestStore>() {}

#[test]
fn the_published_store_is_an_ingest_store() {
    port::<NeonEventStore<NullTransport>>();
}
