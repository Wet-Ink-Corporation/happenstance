//! What the projection store port permits, checked by the compiler rather than
//! asserted in prose.
//!
//! Nothing here runs anything, and it did not when the bodies were `todo!()`
//! either. What these tests do is *instantiate* generic code over the port,
//! which is where the trait obligations are actually discharged. A signature
//! that only holds for one batch shape fails here.
//!
//! That is still the whole of this file's job now that the bodies are real, and
//! the division of labour is worth naming: `tests/projection.rs` runs the
//! conformance suite and can only tell you that this store behaves, while this
//! file is the only thing that says the *port* admits a store shaped like it
//! from generic code that has never heard of it.
//!
//! # What changed when the batch stopped carrying a lifetime
//!
//! This file used to instantiate its generic code at **two** shapes in this
//! crate: `LadybugProjectionStore`'s owned `GraphWriteSet` and
//! `LiveHandleProjectionStore`'s borrowed `GraphWriteHandle<'a>`. ADR-0017 made
//! `ProjectionStore::Batch` an owned associated type, which the borrowed shape
//! cannot bind, and moved that store to
//! `experiments/live-handle-projection-batch/` rather than deleting it.
//!
//! The remaining instantiation is not therefore decoration. The bound in
//! [`send_flavour`] below is the *measurement* PS-5 is arguing about, and it
//! still fails when the shape regresses — see that module's documentation for
//! which spellings are rejected and why.

// The whole file is behind `driver`, because `LadybugProjectionStore` is: every
// item in this crate names an `lbug` type, so without the feature there is no
// store for generic code to be instantiated at. Under a bare
// `cargo test -p happenstance-ladybug` this target configures out entirely.
#![cfg(feature = "driver")]

use happenstance_ladybug::LadybugProjectionStore;

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

    /// Hold a batch across an await inside a real `tokio::spawn`.
    ///
    /// The two extra bounds are the finding, and they are still the finding.
    /// `SendProjectionStore` marks the *futures* `Send` and says nothing about
    /// the associated types they carry, so a runner that wants to be spawned
    /// must add both by hand:
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
    /// context, so it is ``error[E0637]: `'_` cannot be used here``. Checked at
    /// the time, not assumed.
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
    pub(crate) fn spawn_a_batch_across_an_await<S>(store: Arc<S>, id: ProjectionId)
    where
        S: SendProjectionStore<Error: Send> + Send + Sync + 'static,
        S::Batch: Send,
    {
        let handle = tokio::spawn(async move {
            let batch = store.begin().await.map_err(CommitError::Store)?;
            tokio::task::yield_now().await;
            store
                .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
                .await
        });
        drop(handle);
    }
}

// Instantiating a generic function is what discharges its obligations; calling
// it is not required and would panic. `const _` forces the instantiation at
// compile time and binds no name.
const _: () = {
    let _ = weak_flavour::advance::<LadybugProjectionStore>;
    let _ = send_flavour::spawn_a_batch_across_an_await::<LadybugProjectionStore>;
};

#[test]
fn the_owned_batch_shape_satisfies_generic_code_on_both_flavours() {
    // The assertion is the `const _` block above, which the compiler has
    // already checked by the time this runs. This test exists so that a reader
    // scanning `cargo test` output sees the claim named.
}
