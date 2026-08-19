//! The flavour guard: a wrapped read stream is `Send` at the *definition*.
//!
//! Modelled on `happenstance-core`'s `spawns_from_generic`, and here for the
//! same reason: `SendFaultyStore::read` returning the stream at the top level is
//! what lets `trait_variant` mark the **stream** `Send` rather than merely the
//! future that produces it. Refactor it to `async fn read(..) -> impl Stream`
//! and this file stops compiling, because the stream held across `collect`'s
//! await is then `!Send`.
//!
//! Native only. `tokio::spawn` needs a runtime with threads, and the axis this
//! guards — `Send` on a stream — is one the `wasm32` target cannot express at
//! all; `cargo xtask wasm`'s harness step is what compiles the rest of the crate
//! there.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use happenstance_core::{Event, MemoryEventStore, Query, ReadOptions, SendEventStore, collect};
use happenstance_testkit::SendFaultyStore;

/// Holds a wrapped read across an await inside a real spawned task.
///
/// The bound is written at the definition, so inside the function the compiler
/// knows nothing about `S` beyond what `SendEventStore` promises and the
/// obligation is discharged before monomorphisation. Asserting `Send` on a
/// concrete stream would instead be satisfied by auto-trait leakage from the
/// hidden type, whatever the trait said.
fn spawns_from_generic<S: SendEventStore + Send + Sync + 'static>(
    store: Arc<S>,
) -> tokio::task::JoinHandle<usize> {
    tokio::spawn(async move {
        // Bound rather than inlined: edition 2024 RPITIT captures every in-scope
        // lifetime, so the stream borrows the `&Query` even though its hidden
        // type owns everything. Inlining is E0716.
        let query = Query::all();
        let stream = SendEventStore::read(&*store, &query, ReadOptions::new());

        // Collapsed to a `usize` before the next await: `S::Error` carries no
        // `Send` bound (ADR-0009), so a `Result<_, S::Error>` held across a
        // suspension point would make this future `!Send` for a reason that has
        // nothing to do with the stream.
        let seen = collect(stream).await.map_or(0, |events| events.len());

        let appended =
            SendEventStore::append(&*store, &[Event::new("Spawned", &b"{}"[..]).unwrap()], None)
                .await
                .is_ok();

        seen + usize::from(appended)
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn spawns_from_generic_over_send_faulty_store() {
    let store = Arc::new(SendFaultyStore::new(MemoryEventStore::new()));

    SendEventStore::append(&*store, &[Event::new("Seated", &b"{}"[..]).unwrap()], None)
        .await
        .unwrap();

    let seen = spawns_from_generic(Arc::clone(&store)).await.unwrap();
    assert_eq!(seen, 2, "one event read, one appended");
}
