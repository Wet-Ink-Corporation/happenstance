//! The write seam, exercised through nothing but the port and the probe.
//!
//! Until this seam exists, generic code holding an adapter's projection batch
//! can do exactly two things with it — `commit` it or `rollback` it. There is no
//! way to put a row *into* it and no way to look at the read model afterwards,
//! which means the rule carrying this port's entire reason for existing (PS-1:
//! the read-model write and the checkpoint write become durable together or not
//! at all) cannot be written, and a suite built on the port alone degenerates
//! into a checkpoint test that `CheckpointOnlyStore` passes.
//!
//! # Why the instantiating store is defined here and is not the oracle
//!
//! `MemoryProjectionStore` does not exist at this story's boundary, and a test
//! that waited for it would invert the slice's merge order. But the deeper
//! reason is the one `frozen_signatures.rs:4-8` already gives for the event
//! store: *"A consumer written against `MemoryEventStore` proves nothing about
//! the port — it proves something about `MemoryEventStore`, and the two only
//! look alike while there is one implementation."*
//!
//! It also keeps the word `memory` out of this file entirely, which makes the
//! `conformance`-without-`memory` independence visible by inspection rather than
//! only by a feature-powerset step nobody runs locally.
//!
//! # One flavour name in scope, not both
//!
//! `ProjectionStore` only. With `SendProjectionStore` also in scope every
//! `store.begin()` and `store.commit(..)` here becomes
//! `error[E0034]: multiple applicable items in scope`, because the test store
//! implements both and neither is more specific (CLAUDE.md, binding
//! constraint 4).

#![cfg(feature = "conformance")]

use std::cell::RefCell;
use std::collections::BTreeMap;

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};

// =====================================================================
// The instrument: a projection store small enough to read in one sitting
// =====================================================================

/// The test store's error. Real rather than uninhabited, so `Self::Error`'s
/// bound is discharged by something other than `!`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("the round-trip test store failed")]
struct TestError;

/// Pending writes, plus the store instance that minted them.
#[derive(Debug, Default)]
struct TestBatch {
    writes: BTreeMap<String, u64>,
    clear_all: bool,
}

/// A read model behind a `RefCell`, and its checkpoints.
///
/// Applies on write: the batch is a materialised delta the store can read back
/// through, which is why `READS_THROUGH_BATCH` is `true` below.
#[derive(Debug, Default)]
struct TestStore {
    rows: RefCell<BTreeMap<String, u64>>,
    checkpoints: RefCell<BTreeMap<String, Checkpoint>>,
}

impl ProjectionStore for TestStore {
    type Error = TestError;

    type Batch = TestBatch;

    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        Ok(TestBatch::default())
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        Ok(self
            .checkpoints
            .borrow()
            .get(id.as_str())
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        let mut rows = self.rows.borrow_mut();
        let mut checkpoints = self.checkpoints.borrow_mut();

        if batch.clear_all {
            rows.clear();
        }
        rows.extend(batch.writes);
        checkpoints.insert(
            id.as_str().to_owned(),
            match authority {
                Authority::Live => Checkpoint::Live { through: position },
                _ => Checkpoint::Rebuilding { through: position },
            },
        );
        Ok(())
    }

    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        let mut rows = self.rows.borrow_mut();
        let mut checkpoints = self.checkpoints.borrow_mut();

        if batch.clear_all {
            rows.clear();
        }
        rows.extend(batch.writes);
        // Removing the key *is* returning it to `NeverRun`; a sentinel value
        // would reintroduce the ambiguity the enum exists to forbid.
        checkpoints.remove(id.as_str());
        Ok(())
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        drop(batch);
        Ok(())
    }
}

impl ProjectionProbe for TestStore {
    const READS_THROUGH_BATCH: bool = true;

    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        batch.writes.insert(key.to_owned(), value);
        Ok(())
    }

    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        batch.writes.clear();
        batch.clear_all = true;
        Ok(())
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        Ok(self.rows.borrow().get(key).copied())
    }

    async fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        Ok(batch
            .writes
            .get(key)
            .copied()
            .or_else(|| self.rows.borrow().get(key).copied()))
    }
}

// =====================================================================
// AC-001 — the shape the specification publishes, and no other
// =====================================================================

/// Coerce every member to an explicitly written signature.
///
/// A rename is `error[E0599]`, a re-type is `error[E0308]`, and a member that
/// grew or lost a parameter is `error[E0631]` — none of them a silent pass. The
/// bounds name `ProjectionProbe` only, never `TestStore`, so what is checked is
/// the trait rather than this file's instrument.
/// Every member is asynchronous since ADR-0062, so none can be coerced to a
/// plain `fn` pointer without erasing the future. These helpers pin them
/// instead: each return type names its `Result<…, P::Error>` explicitly, so a
/// re-typed member fails here, and each batch-touching helper takes
/// `&mut P::Batch`, so a member that slipped back to a shared borrow fails too.
///
/// No `+ Send` on any bound, deliberately — matching the declarations, which
/// have none and must not acquire one.
fn takes_the_write_future<'a, P: ProjectionProbe>(
    store: &'a P,
    batch: &'a mut P::Batch,
    key: &'a str,
    value: u64,
) -> impl Future<Output = Result<(), P::Error>> + 'a {
    store.probe_write(batch, key, value)
}

fn takes_the_delete_all_future<'a, P: ProjectionProbe>(
    store: &'a P,
    batch: &'a mut P::Batch,
) -> impl Future<Output = Result<(), P::Error>> + 'a {
    store.probe_delete_all(batch)
}

fn takes_the_read_future<P: ProjectionProbe>(
    store: &P,
    key: &str,
) -> impl Future<Output = Result<Option<u64>, P::Error>> {
    store.probe_read(key)
}

fn takes_the_read_through_future<'a, P: ProjectionProbe>(
    store: &'a P,
    batch: &'a mut P::Batch,
    key: &'a str,
) -> impl Future<Output = Result<Option<u64>, P::Error>> + 'a {
    store.probe_read_through(batch, key)
}

fn probe_members_have_the_published_signatures<P: ProjectionProbe>() {
    let reads_through_batch: bool = P::READS_THROUGH_BATCH;

    let write = takes_the_write_future::<P>;
    let delete_all = takes_the_delete_all_future::<P>;
    let read = takes_the_read_future::<P>;
    let read_through = takes_the_read_through_future::<P>;

    // The coercions above are the assertion; naming the values keeps clippy's
    // `no_effect_underscore_binding` off a test whose entire point is a
    // type-check.
    let _ = (reads_through_batch, write, delete_all, read_through, read);
}

#[test]
fn probe_shape_matches_the_specification() {
    // The assertion is the coercion above, which the compiler has already
    // discharged by the time this runs. Instantiating it is what forces it.
    let _ = probe_members_have_the_published_signatures::<TestStore>;
}

/// The probe is a **supertrait bound on the bare flavour**, so a bound of
/// `ProjectionProbe` alone already gives `ProjectionStore`'s six items.
///
/// There is no `SendProjectionProbe` and there must not be: `trait_variant`
/// emits a blanket impl, so `SendProjectionStore` implies `ProjectionStore` and
/// a second probe trait would collide for the reason ADR-0008 records
/// (`error[E0275]`). One probe serves both flavours.
#[tokio::test]
async fn the_probe_is_a_supertrait_of_the_bare_flavour() {
    async fn needs_only_the_probe<P: ProjectionProbe>(store: &P) -> Result<P::Batch, P::Error> {
        store.begin().await
    }

    let store = TestStore::default();
    let batch = needs_only_the_probe(&store)
        .await
        .expect("opening a batch should succeed");
    assert!(batch.writes.is_empty());
}

// =====================================================================
// AC-004 — the seam is generic, and every member is reachable through it
// =====================================================================

/// `begin` → `probe_write` → `commit` → `probe_read`, bound on the trait alone.
///
/// No inherent method of the concrete store appears anywhere on the assertion
/// path; `TestStore` is named only at the instantiation site. That is what makes
/// this a claim about the port rather than about this file.
async fn round_trip<P: ProjectionProbe>(
    store: &P,
    id: &ProjectionId,
    position: SequencePosition,
) -> Result<Option<u64>, P::Error> {
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut batch, "k", 7)
        .await
        .expect("the probe write should succeed");

    // Nothing an open batch holds is visible yet.
    assert_eq!(store.probe_read("k").await?, None);

    store
        .commit(batch, id, position, Authority::Live)
        .await
        .map_err(|error| match error {
            CommitError::Store(inner) => inner,
            other => panic!("the round trip must not fail at the port level: {other:?}"),
        })?;

    store.probe_read("k").await
}

#[tokio::test]
async fn writes_are_visible_through_the_trait_alone() {
    let store = TestStore::default();
    let id = ProjectionId::new("round_trip");

    let value = round_trip(&store, &id, SequencePosition::FIRST)
        .await
        .expect("the round trip must succeed");

    assert_eq!(
        value,
        Some(7),
        "probe_write then probe_read must round-trip"
    );
    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        },
        "the commit that carried the row also carried the checkpoint"
    );
}

/// All five members driven generically, so none is dead on arrival.
///
/// `probe_delete_all` exists so `reset` (PS-16) is checkable *without the suite
/// knowing what a read model is*, and `probe_read_through` is PS-12's gate.
/// Neither has a caller yet — the rules that need them arrive two slices later —
/// and a member nothing has ever compiled against is a member that is wrong when
/// it is first used.
async fn drive_every_member<P: ProjectionProbe>(
    store: &P,
    id: &ProjectionId,
    position: SequencePosition,
) -> Result<(), P::Error> {
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut batch, "pending", 42)
        .await
        .expect("the probe write should succeed");

    if P::READS_THROUGH_BATCH {
        assert_eq!(
            store
                .probe_read_through(&mut batch, "pending")
                .await
                .expect("reading through the batch should succeed"),
            Some(42),
            "a store declaring READS_THROUGH_BATCH must see its own pending write"
        );
    }

    store
        .commit(batch, id, position, Authority::Rebuilding)
        .await
        .map_err(|error| match error {
            CommitError::Store(inner) => inner,
            other => panic!("commit failed at the port level: {other:?}"),
        })?;
    assert_eq!(store.probe_read("pending").await?, Some(42));

    // The dual: the caller's own batch carries the deletes.
    let mut clearing = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_delete_all(&mut clearing)
        .await
        .expect("the probe delete should succeed");
    store
        .reset(clearing, id)
        .await
        .map_err(|error| match error {
            ResetError::Store(inner) => inner,
            other => panic!("reset failed at the port level: {other:?}"),
        })?;
    assert_eq!(store.probe_read("pending").await?, None);

    // `rollback` still discards, and the store is still usable afterwards.
    let mut abandoned = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut abandoned, "abandoned", 1)
        .await
        .expect("the probe write should succeed");
    store.rollback(abandoned).await?;
    assert_eq!(store.probe_read("abandoned").await?, None);

    Ok(())
}

#[tokio::test]
async fn all_five_members_are_reachable_generically() {
    let store = TestStore::default();
    let id = ProjectionId::new("every_member");

    drive_every_member(&store, &id, SequencePosition::FIRST)
        .await
        .expect("every member must be drivable through the trait");

    assert_eq!(
        store.checkpoint(&id).await.expect("checkpoint reads"),
        Checkpoint::NeverRun,
        "reset returns the projection to NeverRun, not to a sentinel position"
    );
}
