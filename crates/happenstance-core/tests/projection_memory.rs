//! `MemoryProjectionStore`'s behaviour, observed through the **public** surface.
//!
//! An integration test file can name only public items, so this file compiling
//! at all is the proof that the store is mounted rather than merely written.
//! That is deliberate: a type that exists in `lib.rs` but not in `Cargo.toml`'s
//! `[features]`, or the reverse, is the library equivalent of a
//! constructed-but-unmounted component, and no unit test inside the crate would
//! notice.
//!
//! # Never a literal position
//!
//! Every assertion compares against a position the caller handed in. The
//! specification permits gaps and a conformant adapter may leave them, so an
//! assertion on `[1, 2, 3]` tests this store rather than the contract.
//!
//! # One flavour name in scope
//!
//! `ProjectionStore` only — the weaker requirement, which is what generic code
//! binds. With `SendProjectionStore` also in scope every method call here is
//! `error[E0034]`, because the store implements both and neither is more
//! specific.

#![cfg(feature = "memory")]

use happenstance_core::{
    Authority, Checkpoint, CommitError, MemoryProjectionStore, ProjectionId, ProjectionStore,
    ResetError, SequencePosition,
};

/// The second position, derived rather than written as a literal.
fn next(position: SequencePosition) -> SequencePosition {
    SequencePosition::new(position.get() + 1).expect("a successor of a NonZeroU64 is non-zero")
}

// =====================================================================
// AC-001 — the mount
// =====================================================================

#[tokio::test]
async fn store_is_reachable_from_the_public_surface() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");

    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "a projection this store has never seen has never run"
    );
}

// =====================================================================
// AC-002 — both flavours from one impl, and `begin` costs nothing
// =====================================================================

#[test]
fn begin_is_synchronous_and_infallible() {
    let store = MemoryProjectionStore::new();

    // No `.await`, no `?`. Restoring either for symmetry with `EventStore`
    // makes this line stop compiling.
    let batch = store.begin();

    drop(batch);
}

#[test]
fn send_impl_satisfies_the_bare_bound() {
    fn takes_any_projection_store<P: ProjectionStore>(_store: &P) {}

    let store = MemoryProjectionStore::new();
    takes_any_projection_store(&store);
}

#[test]
fn the_store_and_its_batch_are_send_and_sync() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<MemoryProjectionStore>();
    assert_sync::<MemoryProjectionStore>();
    assert_send::<<MemoryProjectionStore as ProjectionStore>::Batch>();
}

// =====================================================================
// AC-003 — atomicity, the invariant the store exists to demonstrate
// =====================================================================

#[tokio::test]
async fn open_batch_is_invisible_until_commit() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    let mut batch = store.begin();
    batch.write("depot-7", 12);

    assert_eq!(
        store.get("depot-7"),
        None,
        "nothing an open batch holds is visible through the read model"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "nor through the checkpoint"
    );

    store
        .commit(batch, &id, position, Authority::Live)
        .await
        .expect("commit succeeds");
}

#[tokio::test]
async fn commit_installs_rows_and_checkpoint_together() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    let mut batch = store.begin();
    batch.write("depot-7", 12);
    store
        .commit(batch, &id, position, Authority::Live)
        .await
        .expect("commit succeeds");

    assert_eq!(store.get("depot-7"), Some(12));
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position },
        "the row and the checkpoint became visible together"
    );
}

// =====================================================================
// AC-004 — authority, position-considered, regression, per-id scope
// =====================================================================

#[tokio::test]
async fn commit_records_the_authority_it_was_given() {
    let store = MemoryProjectionStore::new();
    let live = ProjectionId::new("live_one");
    let rebuilding = ProjectionId::new("rebuilding_one");
    let position = SequencePosition::FIRST;

    store
        .commit(store.begin(), &live, position, Authority::Live)
        .await
        .expect("commit succeeds");
    store
        .commit(store.begin(), &rebuilding, position, Authority::Rebuilding)
        .await
        .expect("commit succeeds");

    assert_eq!(
        store.checkpoint(&live).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position }
    );
    assert_eq!(
        store
            .checkpoint(&rebuilding)
            .await
            .expect("checkpoint reads"),
        Checkpoint::Rebuilding { through: position },
        "a rebuild in flight must not report itself as authoritative"
    );
}

#[tokio::test]
async fn empty_batch_still_advances_the_checkpoint() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    // Nothing written: the position is the one *considered*, not the one
    // applied, so a run that produced no read-model change still moves on.
    store
        .commit(store.begin(), &id, position, Authority::Live)
        .await
        .expect("commit succeeds");

    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position }
    );
}

#[tokio::test]
async fn regressing_position_is_rejected_and_changes_nothing() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let first = SequencePosition::FIRST;
    let second = next(first);

    let mut batch = store.begin();
    batch.write("depot-7", 12);
    store
        .commit(batch, &id, second, Authority::Live)
        .await
        .expect("commit succeeds");

    let mut regressing = store.begin();
    regressing.write("depot-7", 99);
    let error = store
        .commit(regressing, &id, first, Authority::Live)
        .await
        .expect_err("a position below the checkpoint is refused");

    assert_eq!(
        error,
        CommitError::CheckpointRegression {
            current: second,
            attempted: first,
        },
        "both values, so a caller can log the gap rather than re-derive it"
    );
    assert_eq!(
        store.get("depot-7"),
        Some(12),
        "the rejected commit applied nothing"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: second },
        "and moved nothing"
    );
}

#[tokio::test]
async fn distinct_projections_advance_independently() {
    let store = MemoryProjectionStore::new();
    let one = ProjectionId::new("one");
    let two = ProjectionId::new("two");
    let position = SequencePosition::FIRST;

    store
        .commit(store.begin(), &one, position, Authority::Live)
        .await
        .expect("commit succeeds");

    assert_eq!(
        store.checkpoint(&one).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position }
    );
    assert_eq!(
        store.checkpoint(&two).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "committing one id leaves every sibling untouched"
    );
}

// =====================================================================
// AC-005 — the foreign batch the owned type did not make unrepresentable
// =====================================================================

#[tokio::test]
async fn commit_rejects_a_foreign_batch() {
    let a = MemoryProjectionStore::new();
    let b = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    // `b.commit(a.begin(), ..)` still type-checks: a lifetime names a region,
    // not an instance, so this is a run-time rejection by design.
    let mut foreign = a.begin();
    foreign.write("depot-7", 12);

    let error = b
        .commit(foreign, &id, position, Authority::Live)
        .await
        .expect_err("a batch begun elsewhere is refused");

    assert_eq!(error, CommitError::ForeignBatch);
    assert_eq!(b.get("depot-7"), None);
    assert_eq!(
        b.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "the receiving store is unchanged"
    );
}

#[tokio::test]
async fn reset_rejects_a_foreign_batch() {
    let a = MemoryProjectionStore::new();
    let b = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    let mut seeded = b.begin();
    seeded.write("depot-7", 12);
    b.commit(seeded, &id, position, Authority::Live)
        .await
        .expect("commit succeeds");

    let error = b
        .reset(a.begin(), &id)
        .await
        .expect_err("a batch begun elsewhere is refused");

    assert_eq!(error, ResetError::ForeignBatch);
    assert_eq!(b.get("depot-7"), Some(12));
    assert_eq!(
        b.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position },
        "a refused reset leaves both halves unchanged"
    );
}

// =====================================================================
// AC-006 — rollback, and the half a store actually fails
// =====================================================================

#[tokio::test]
async fn dropped_batch_leaves_store_usable() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    {
        let mut abandoned = store.begin();
        abandoned.write("depot-7", 99);
        // No commit, no rollback: just a drop, which is what an apply loop that
        // panics or returns early actually does.
    }

    assert_eq!(store.get("depot-7"), None);

    // The half a reviewer's probe once found a store failing: it answered
    // `Busy` forever afterwards.
    let mut second = store.begin();
    second.write("depot-8", 5);
    store
        .commit(second, &id, position, Authority::Live)
        .await
        .expect("the store still serves a subsequent batch");

    assert_eq!(store.get("depot-8"), Some(5));
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position }
    );
}

#[tokio::test]
async fn rollback_leaves_both_unchanged() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");

    let mut batch = store.begin();
    batch.write("depot-7", 99);
    store.rollback(batch).await.expect("rollback succeeds");

    assert_eq!(store.get("depot-7"), None);
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun
    );
}

// =====================================================================
// AC-007 — reset as one scoped unit of work
// =====================================================================

#[tokio::test]
async fn reset_clears_rows_and_checkpoint_together() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let position = SequencePosition::FIRST;

    let mut seeded = store.begin();
    seeded.write("depot-7", 12);
    store
        .commit(seeded, &id, position, Authority::Live)
        .await
        .expect("commit succeeds");

    // The caller's own batch carries the deletes: the port has no idea what the
    // read model is.
    let mut clearing = store.begin();
    clearing.delete_all();
    store.reset(clearing, &id).await.expect("reset succeeds");

    assert_eq!(store.get("depot-7"), None);
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun
    );
}

#[tokio::test]
async fn reset_returns_the_projection_to_never_run() {
    let store = MemoryProjectionStore::new();
    let id = ProjectionId::new("van_stock");
    let first = SequencePosition::FIRST;

    store
        .commit(store.begin(), &id, first, Authority::Live)
        .await
        .expect("commit succeeds");

    let mut clearing = store.begin();
    clearing.delete_all();
    store.reset(clearing, &id).await.expect("reset succeeds");

    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "`NeverRun`, not a sentinel position — `commit(empty, id, FIRST)` is not this"
    );

    // And the distinction has teeth: a runner resuming from `NeverRun` replays
    // the first position, where one resuming from `Live { through: FIRST }`
    // would skip it.
    assert_ne!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live { through: first }
    );
}

#[tokio::test]
async fn reset_is_scoped_to_one_projection() {
    let store = MemoryProjectionStore::new();
    let one = ProjectionId::new("one");
    let two = ProjectionId::new("two");
    let position = SequencePosition::FIRST;

    store
        .commit(store.begin(), &one, position, Authority::Live)
        .await
        .expect("commit succeeds");
    store
        .commit(store.begin(), &two, position, Authority::Live)
        .await
        .expect("commit succeeds");

    store
        .reset(store.begin(), &one)
        .await
        .expect("reset succeeds");

    assert_eq!(
        store.checkpoint(&one).await.expect("checkpoint reads"),
        Checkpoint::NeverRun
    );
    assert_eq!(
        store.checkpoint(&two).await.expect("checkpoint reads"),
        Checkpoint::Live { through: position },
        "a sibling's checkpoint is untouched"
    );
}

// =====================================================================
// AC-008 — the probe seam, driven generically
// =====================================================================

#[cfg(feature = "conformance")]
mod probe {
    use super::{MemoryProjectionStore, ProjectionId, SequencePosition, next};
    use happenstance_core::{Authority, CommitError, ProjectionProbe, ProjectionStore};

    /// Bound on the traits only — no inherent method of the concrete store
    /// appears on the assertion path.
    async fn round_trip<S: ProjectionStore + ProjectionProbe>(
        store: &S,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<Option<u64>, CommitError<S::Error>> {
        let mut batch = store.begin();
        store.probe_write(&mut batch, "depot-7", 12);
        store.commit(batch, id, position, Authority::Live).await?;
        store
            .probe_read("depot-7")
            .await
            .map_err(CommitError::Store)
    }

    #[tokio::test]
    async fn probe_round_trip_through_the_traits_only() {
        let store = MemoryProjectionStore::new();
        let id = ProjectionId::new("van_stock");

        let value = round_trip(&store, &id, SequencePosition::FIRST)
            .await
            .expect("the round trip succeeds");

        assert_eq!(value, Some(12));
    }

    #[tokio::test]
    async fn probe_read_through_sees_a_pending_write() {
        let store = MemoryProjectionStore::new();

        // A `const` assertion, so declaring `false` here is a build failure
        // rather than a runtime one. Flipping it would make PS-12's rule
        // skippable by everything in the workspace, which is a finding for
        // `read-through-and-rebuild-rules` and never a quiet edit.
        const {
            assert!(
                <MemoryProjectionStore as ProjectionProbe>::READS_THROUGH_BATCH,
                "this store applies on write, so read-through is real rather than unimplemented"
            );
        }

        let mut batch = store.begin();
        store.probe_write(&mut batch, "depot-7", 12);

        assert_eq!(
            store.probe_read_through(&batch, "depot-7"),
            Some(12),
            "the pending write is visible through the open batch"
        );
        assert_eq!(
            store.probe_read("depot-7").await.expect("probe reads"),
            None,
            "and still invisible through the committed read model"
        );
    }

    #[tokio::test]
    async fn probe_read_through_layers_pending_over_committed() {
        let store = MemoryProjectionStore::new();
        let id = ProjectionId::new("van_stock");
        let first = SequencePosition::FIRST;

        let mut seeded = store.begin();
        store.probe_write(&mut seeded, "depot-7", 12);
        store.probe_write(&mut seeded, "depot-8", 1);
        store
            .commit(seeded, &id, first, Authority::Live)
            .await
            .expect("commit succeeds");

        let mut batch = store.begin();
        store.probe_write(&mut batch, "depot-7", 99);

        assert_eq!(
            store.probe_read_through(&batch, "depot-7"),
            Some(99),
            "a pending write shadows the committed row"
        );
        assert_eq!(
            store.probe_read_through(&batch, "depot-8"),
            Some(1),
            "and committed rows the batch does not touch are still visible"
        );
    }

    #[tokio::test]
    async fn probe_delete_all_supports_reset() {
        let store = MemoryProjectionStore::new();
        let id = ProjectionId::new("van_stock");
        let first = SequencePosition::FIRST;
        let second = next(first);

        let mut seeded = store.begin();
        store.probe_write(&mut seeded, "depot-7", 12);
        store.probe_write(&mut seeded, "depot-8", 5);
        store
            .commit(seeded, &id, second, Authority::Live)
            .await
            .expect("commit succeeds");

        // The suite can clear the read model without knowing its shape.
        let mut clearing = store.begin();
        store.probe_delete_all(&mut clearing);
        store.reset(clearing, &id).await.expect("reset succeeds");

        assert_eq!(
            store.probe_read("depot-7").await.expect("probe reads"),
            None
        );
        assert_eq!(
            store.probe_read("depot-8").await.expect("probe reads"),
            None
        );
    }
}
