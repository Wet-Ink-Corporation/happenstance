//! ES-6's rule, `store_error_crosses_a_join_handle` (ADR-0114 option A).
//!
//! ES-6 puts the strength "a store's error crosses threads" in a marker
//! declared downstream, not on the port: ADR-0009's
//! `ThreadSafeEventStore: SendEventStore<Error: Send + Sync>` with a blanket
//! impl. The port itself carries no bound on `Error`, and
//! `happenstance-cloudflare`'s `SendStoreWithLocalError` is what pins that: it
//! implements `SendEventStore` with a `!Send` error and compiles.
//!
//! So the rule cannot be bound by `SendEventStore`. It is bound by the marker,
//! declared here exactly as a downstream crate would declare it, and it asserts
//! on the future's **output** (ADR-0008): `tokio::spawn` requires
//! `F::Output: Send`, so a spawned task that returns `append`'s `Result` meets
//! that requirement by construction, or does not compile.
//!
//! The negative half is a `compile_fail,E0277` doctest in
//! `crates/happenstance-cloudflare/src/send_shape.rs`: the same marker refuses
//! `SendStoreWithLocalError`. A marker written without the `Error` bound —
//! ADR-0008's decorative shape — makes that doctest compile, and it fails.

#![cfg(feature = "memory")]

use std::sync::Arc;

use happenstance_core::{AppendError, MemoryEventStore, SendEventStore, SequencePosition};

/// ADR-0009's marker, as a downstream crate declares it.
///
/// A supertrait with an associated-type bound (`Error: Send + Sync`) and a
/// blanket impl, so every `SendEventStore` whose error is thread-safe is a
/// `ThreadSafeEventStore` without opting in, and no other is. The bound is the
/// whole point: without it the marker is `SendEventStore` under another name.
trait ThreadSafeEventStore: SendEventStore<Error: Send + Sync> {}

impl<S> ThreadSafeEventStore for S where S: SendEventStore<Error: Send + Sync> {}

/// Spawns `append` on another task and returns its `Result` across the
/// `JoinHandle`.
///
/// Generic over the marker, so the obligation is discharged at the definition
/// rather than by auto-trait leakage from a concrete store: `S::Error: Send` is
/// known here only because the marker says so. Remove `Error: Send + Sync` from
/// the marker and this function stops compiling with `E0277`, because the task's
/// output, `Result<_, AppendError<S::Error>>`, is no longer known to be `Send`.
fn append_on_another_task<S>(
    store: Arc<S>,
) -> tokio::task::JoinHandle<Result<SequencePosition, AppendError<S::Error>>>
where
    S: ThreadSafeEventStore + Sync + 'static,
{
    tokio::spawn(async move { SendEventStore::append(&*store, &[], None).await })
}

#[tokio::test]
async fn store_error_crosses_a_join_handle() {
    let store = Arc::new(MemoryEventStore::new());

    let outcome = append_on_another_task(store)
        .await
        .expect("the spawned task neither panicked nor was cancelled");

    // An empty batch is refused (ES-20), so what crossed the handle is an
    // error, not a position: the rule is about the error's journey.
    assert!(
        matches!(outcome, Err(AppendError::NoEvents)),
        "an empty append is refused, and the refusal came back across the \
         `JoinHandle` intact: got {outcome:?}"
    );
}
