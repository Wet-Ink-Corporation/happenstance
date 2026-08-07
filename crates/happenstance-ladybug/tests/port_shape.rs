//! What the projection store port permits, checked by the compiler rather than
//! asserted in prose.
//!
//! Nothing here runs anything: every body in the crate is `todo!()`, so calling
//! one would panic. What these tests do is *instantiate* generic code at both
//! of this crate's two batch shapes, which is where the trait obligations are
//! actually discharged. A signature that only holds for one shape fails here.

use happenstance_ladybug::{LadybugProjectionStore, LiveHandleProjectionStore};

/// Generic code binds the **weak** flavour, per CLAUDE.md's constraint 4 — it
/// is the weaker requirement, so it accepts both flavours. Note that the two
/// flavours are imported in separate modules; having both names in scope at
/// once makes every method call ambiguous (`error[E0034]`).
mod weak_flavour {
    use happenstance_core::{ProjectionId, ProjectionStore, SequencePosition};

    /// The minimal runner: open a batch, commit it with a checkpoint.
    ///
    /// This is the call site that proves `begin` and `commit` compose — that
    /// the batch `begin` hands out is the one `commit` accepts, for a store the
    /// caller knows nothing else about.
    pub(crate) async fn advance<S: ProjectionStore>(
        store: &S,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), S::Error> {
        let batch = store.begin().await?;
        store.commit(batch, id, position).await
    }
}

/// The `Send` flavour, and the bounds a real runner turns out to need.
mod send_flavour {
    use std::sync::Arc;

    use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};

    /// Hold a batch across an await inside a real `tokio::spawn`.
    ///
    /// The two extra bounds are the finding. `SendProjectionStore` marks the
    /// *futures* `Send` and says nothing about the associated types they carry,
    /// so a runner that wants to be spawned must add both by hand:
    ///
    /// * without `Error: Send`, `tokio::spawn` rejects the task because its
    ///   `Output` is `Result<(), S::Error>`;
    /// * without `for<'a> S::Batch<'a>: Send`, it rejects the task because the
    ///   batch is live across the await — which is exactly what a runner does
    ///   between applying an event and deciding to commit.
    ///
    /// The `for<'a>` is forced by the GAT and is the concrete cost PS-5 is
    /// arguing about: under an owned `type Batch;` this is `S::Batch: Send`,
    /// which a caller might write without help. The higher-ranked form is not
    /// something a caller discovers unaided, and rustc's own suggestion is
    /// `<S as SendProjectionStore>::Batch<'_>: Send`, which does not compile
    /// as printed — a where clause is not an elision context, so it is
    /// `error[E0637]: `'_` cannot be used here`. Checked, not assumed.
    pub(crate) fn spawn_a_batch_across_an_await<S>(store: Arc<S>, id: ProjectionId)
    where
        S: SendProjectionStore<Error: Send> + Send + Sync + 'static,
        for<'a> S::Batch<'a>: Send,
    {
        let handle = tokio::spawn(async move {
            let batch = store.begin().await?;
            tokio::task::yield_now().await;
            store.commit(batch, &id, SequencePosition::FIRST).await
        });
        drop(handle);
    }
}

// Instantiating a generic function is what discharges its obligations; calling
// it is not required and would panic. `const _` forces the instantiation at
// compile time and binds no name.
const _: () = {
    let _ = weak_flavour::advance::<LadybugProjectionStore>;
    let _ = weak_flavour::advance::<LiveHandleProjectionStore>;
    let _ = send_flavour::spawn_a_batch_across_an_await::<LadybugProjectionStore>;
    let _ = send_flavour::spawn_a_batch_across_an_await::<LiveHandleProjectionStore>;
};

#[test]
fn both_batch_shapes_satisfy_the_same_generic_code() {
    // The assertion is the `const _` block above, which the compiler has
    // already checked by the time this runs. This test exists so that a reader
    // scanning `cargo test` output sees the claim named.
}
