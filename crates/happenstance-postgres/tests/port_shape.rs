//! What the projection store port permits, checked by the compiler rather than
//! asserted in prose.
//!
//! Nothing here runs anything and nothing here needs a server. What these tests
//! do is *instantiate* generic code over the port, which is where the trait
//! obligations are actually discharged. A signature that only holds for one
//! batch shape fails here.
//!
//! `tests/projection.rs` and `tests/live_projection.rs` run the conformance
//! suite and can only tell you that each store behaves; this file is what says
//! the *port* admits a store shaped like them from generic code that has never
//! heard of either.
//!
//! # Where this file came from
//!
//! It is `crates/happenstance-ladybug/tests/port_shape.rs`, moved here when
//! ADR-0078 retired that adapter and took its tests out of the gate. That file
//! was the workspace's only compiled instance of PS-5's caller-side benefit and
//! of the remedy PS-36 documents — a caller writing `S::Batch: Send` in their
//! own `where` clause — so the retirement would otherwise have left both as
//! prose. It is instantiated here at **both** ends of PS-2's batch-shape axis,
//! which the Ladybug copy never was: `PostgresProjectionStore`'s owned, buffered
//! write set and `LivePostgresProjectionStore`'s live `sqlx` transaction.

// Both stores are behind `projection-store`; without it there is nothing for
// generic code to be instantiated at, and this target configures out.
#![cfg(all(feature = "projection-store", not(target_arch = "wasm32")))]

use happenstance_postgres::live_projection_store::LivePostgresProjectionStore;
use happenstance_postgres::projection_store::PostgresProjectionStore;

/// Generic code binds the **weak** flavour, per CLAUDE.md's constraint 4 — it
/// is the weaker requirement, so it accepts both flavours. Note that the two
/// flavours are imported in separate modules; having both names in scope at
/// once makes every method call ambiguous (`error[E0034]`).
mod weak_flavour {
    use happenstance_core::{
        Authority, CommitError, ProjectionId, ProjectionStore, SequencePosition,
    };

    /// The minimal runner: open a batch, commit it with a checkpoint.
    ///
    /// This is the call site that proves `begin` and `commit` compose — that
    /// the batch `begin` hands out is the one `commit` accepts, for a store the
    /// caller knows nothing else about.
    ///
    /// `begin` is `async` and fallible since ADR-0062, and its failure has no
    /// arm of its own on `CommitError`: a batch that could not be opened is a
    /// store failure, which is what `CommitError::Store` names.
    pub(crate) async fn advance<S: ProjectionStore>(
        store: &S,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), CommitError<S::Error>> {
        let batch = store.begin().await.map_err(CommitError::Store)?;
        store.commit(batch, id, position, Authority::Live).await
    }
}

/// The `Send` flavour, and the bounds a real runner turns out to need.
mod send_flavour {
    use std::sync::Arc;

    use happenstance_core::{
        Authority, CommitError, ProjectionId, SendProjectionStore, SequencePosition,
    };
    use tokio::task::JoinHandle;

    /// Hold a batch across an await inside a real `tokio::spawn`.
    ///
    /// The two extra bounds are the finding. `SendProjectionStore` marks the
    /// *futures* `Send` and says nothing about the associated types they carry,
    /// so a runner that wants to be spawned must add both by hand:
    ///
    /// * without `Error: Send`, `tokio::spawn` rejects the task because its
    ///   `Output` is `Result<(), CommitError<S::Error>>`;
    /// * without `S::Batch: Send`, it rejects the task because the batch is
    ///   live across the await — which is exactly what a runner does between
    ///   applying an event and deciding to commit.
    ///
    /// # The measurement PS-5 was arguing about
    ///
    /// The second bound used to read `for<'a> S::Batch<'a>: Send`, forced by
    /// the GAT. That form is not something a caller discovers unaided: rustc's
    /// own suggestion was `<S as SendProjectionStore>::Batch<'_>: Send`, which
    /// **does not compile as printed** — a `where` clause is not an elision
    /// context, so it is ``error[E0637]: `'_` cannot be used here``.
    ///
    /// It now reads `S::Batch: Send`, which a caller might write without help.
    /// That is the whole of PS-5's caller-side benefit, and this line is where
    /// it is measured rather than claimed.
    ///
    /// # Why this is still able to fail
    ///
    /// Delete `S::Batch: Send` and the `const _` block below stops compiling:
    /// `tokio::spawn` requires the generated future to be `Send`, the future
    /// holds `S::Batch` across `yield_now().await`, and nothing else in the
    /// bound set proves it. Delete `Error: Send` and the task's `Output` fails
    /// the same way. Neither is decoration and neither is implied by
    /// `SendProjectionStore`.
    ///
    /// The handle is returned rather than dropped, so the spawned task has an
    /// owner; nothing calls this function, so no task is ever spawned.
    pub(crate) fn spawn_a_batch_across_an_await<S>(
        store: Arc<S>,
        id: ProjectionId,
    ) -> JoinHandle<Result<(), CommitError<S::Error>>>
    where
        S: SendProjectionStore<Error: Send> + Send + Sync + 'static,
        S::Batch: Send,
    {
        tokio::spawn(async move {
            let batch = store.begin().await.map_err(CommitError::Store)?;
            tokio::task::yield_now().await;
            store
                .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
                .await
        })
    }
}

// Instantiating a generic function is what discharges its obligations; calling
// it is not required and would need a server. `const _` forces the
// instantiation at compile time and binds no name.
const _: () = {
    let _ = weak_flavour::advance::<PostgresProjectionStore>;
    let _ = send_flavour::spawn_a_batch_across_an_await::<PostgresProjectionStore>;
    let _ = weak_flavour::advance::<LivePostgresProjectionStore>;
    let _ = send_flavour::spawn_a_batch_across_an_await::<LivePostgresProjectionStore>;
};

#[test]
fn both_batch_shapes_satisfy_generic_code_on_both_flavours() {
    // The assertion is the `const _` block above, which the compiler has
    // already checked by the time this runs. This test exists so that a reader
    // scanning `cargo test` output sees the claim named.
}
