//! The adapter shape PS-2 needs, and the probe seam that now admits it.
//!
//! PS-2 is `[FROZEN]` and its Rule names the two ends of the batch-shape axis
//! the port must be proved against — *"one adapter holding a live transaction
//! (rusqlite or `sqlx`) and one that cannot hold anything across an await"*.
//! Until ADR-0062 this file was the compiled record of why the first end could
//! not be built honestly: `probe_read_through` was synchronous, infallible and
//! took `&Self::Batch`, so a store whose batch *is* a transaction could
//! implement the probe only by declaring `READS_THROUGH_BATCH = false` — a
//! false statement about itself — and the suite could not tell it apart from a
//! buffering store. ADR-0060 recorded that finding and declined to act on it in
//! an adapter lane; ADR-0062 acted on it.
//!
//! This file now asks the inverted question, and answers it by construction.
//! `LiveTransactionStore`'s batch **is** a transaction: statements are issued as
//! they are made, against a connection the batch owns, and reading one back is
//! real I/O — `&mut` because a driver borrows the connection to run a statement
//! (`sqlx`'s `Executor for &mut Transaction`, `rusqlite`'s `&mut Transaction`),
//! and `.await` because it yields. Under the moved seam it declares
//! `READS_THROUGH_BATCH = true` truthfully and the read-through rule can run
//! against it.
//!
//! What this file does **not** claim is that PS-2's bar is met. This store is an
//! in-process instrument with a simulated yield, not an adapter over storage the
//! workspace does not control. It establishes that the far end is now
//! *reachable and reportable* — the two properties ADR-0060 found missing — and
//! leaves standing at it to an adapter.
//!
//! # Why the store is defined here and is not `MemoryProjectionStore`
//!
//! The same reason `projection_probe_round_trip.rs` gives: a store that answers
//! from a `BTreeMap` behind a `RefCell` can implement anything, so it cannot
//! falsify anything. The property under test is *asynchrony plus a mutable
//! borrow*, which is exactly what the in-process instruments do not have.
//!
//! It also keeps `memory` out of the file, so `conformance`-without-`memory`
//! stays visible by inspection.

#![cfg(feature = "conformance")]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};

// =====================================================================
// The instrument: a store whose batch is a live transaction
// =====================================================================

/// The store's error. Real rather than uninhabited, so the fallible path below
/// is a path and not a formality.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("the live-transaction store failed")]
struct LiveError;

/// Committed state, shared by the store and every transaction it opens.
///
/// The `std` mutex is held across no await — it is locked and released inside
/// each statement, which is what a connection does. The asynchrony is in the
/// statement, not in the lock.
#[derive(Debug, Default)]
struct Server {
    rows: Mutex<BTreeMap<String, u64>>,
    checkpoints: Mutex<BTreeMap<ProjectionId, Checkpoint>>,
    /// Armed, the next statement is refused — a constraint violation, a lost
    /// connection. What a live transaction can do that a buffer cannot.
    refuse_statements: AtomicBool,
}

/// A live transaction, and the point of this file.
///
/// Uncommitted statements live here rather than being replayed at commit — the
/// difference between this and `SqliteBatch` — and reading one back goes through
/// the connection, so it is `async` and fallible and needs `&mut self`.
#[derive(Debug)]
struct LiveTransaction {
    server: Arc<Server>,
    uncommitted: BTreeMap<String, u64>,
    clear_all: bool,
}

impl LiveTransaction {
    /// Issues one statement through the open transaction.
    ///
    /// `&mut self` because the driver borrows the connection; `async` because
    /// the statement is I/O; `Result` because a statement on a live
    /// transaction can fail, and after it does the transaction is poisoned
    /// rather than merely unhelpful.
    async fn execute(&mut self, key: &str, value: u64) -> Result<(), LiveError> {
        // A real yield, not a formality: it is what the old synchronous probe
        // could not bridge from inside the suite's runtime.
        tokio::task::yield_now().await;
        if self.server.refuse_statements.load(Ordering::SeqCst) {
            return Err(LiveError);
        }
        self.uncommitted.insert(key.to_owned(), value);
        Ok(())
    }

    /// Reads one row through the open transaction, committed state included.
    ///
    /// **This is the method `ProjectionProbe::probe_read_through` exists to
    /// expose, and its signature is what ADR-0062 changed the probe to meet.**
    async fn select(&mut self, key: &str) -> Result<Option<u64>, LiveError> {
        tokio::task::yield_now().await;
        if let Some(pending) = self.uncommitted.get(key) {
            return Ok(Some(*pending));
        }
        if self.clear_all {
            return Ok(None);
        }
        let rows = self.server.rows.lock().map_err(|_| LiveError)?;
        Ok(rows.get(key).copied())
    }
}

/// A projection store that hands out live transactions.
#[derive(Debug, Default)]
struct LiveTransactionStore {
    server: Arc<Server>,
}

impl ProjectionStore for LiveTransactionStore {
    type Error = LiveError;

    type Batch = LiveTransaction;

    /// Acquires a connection and opens the transaction: a round trip, and a
    /// failure, which is why `begin` is a future of a `Result`.
    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        tokio::task::yield_now().await;
        Ok(LiveTransaction {
            server: Arc::clone(&self.server),
            uncommitted: BTreeMap::new(),
            clear_all: false,
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let checkpoints = self.server.checkpoints.lock().map_err(|_| LiveError)?;
        Ok(checkpoints.get(id).copied().unwrap_or(Checkpoint::NeverRun))
    }

    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        let mut rows = self
            .server
            .rows
            .lock()
            .map_err(|_| CommitError::Store(LiveError))?;
        let mut checkpoints = self
            .server
            .checkpoints
            .lock()
            .map_err(|_| CommitError::Store(LiveError))?;
        if batch.clear_all {
            rows.clear();
        }
        rows.extend(batch.uncommitted);
        checkpoints.insert(
            id.clone(),
            match authority {
                Authority::Live => Checkpoint::Live { through: position },
                // `Authority` is `#[non_exhaustive]`, so the arm is required
                // and is not dead code waiting to be deleted.
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
        let mut rows = self
            .server
            .rows
            .lock()
            .map_err(|_| ResetError::Store(LiveError))?;
        let mut checkpoints = self
            .server
            .checkpoints
            .lock()
            .map_err(|_| ResetError::Store(LiveError))?;
        if batch.clear_all {
            rows.clear();
        }
        rows.extend(batch.uncommitted);
        checkpoints.remove(id);
        Ok(())
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        drop(batch);
        Ok(())
    }
}

/// The probe, written the honest way — which the seam now permits.
///
/// `READS_THROUGH_BATCH` is `true` and that is a **true statement about this
/// store**: every batch-touching member issues a statement through the
/// transaction, awaiting the yield and surfacing the failure, exactly as a
/// driver-backed adapter would.
impl ProjectionProbe for LiveTransactionStore {
    const READS_THROUGH_BATCH: bool = true;

    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        batch.execute(key, value).await
    }

    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        tokio::task::yield_now().await;
        batch.uncommitted.clear();
        batch.clear_all = true;
        Ok(())
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        tokio::task::yield_now().await;
        let rows = self.server.rows.lock().map_err(|_| LiveError)?;
        Ok(rows.get(key).copied())
    }

    async fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        batch.select(key).await
    }
}

// =====================================================================
// What the store can do, and what the probe can now report about it
// =====================================================================

/// The property PS-2's far end is defined by. It holds.
#[tokio::test(flavor = "current_thread")]
async fn the_transaction_reads_its_own_uncommitted_writes() {
    let store = LiveTransactionStore::default();
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut batch, "depot-7", 12)
        .await
        .expect("the probe write should succeed");

    assert_eq!(
        batch.select("depot-7").await,
        Ok(Some(12)),
        "the batch is a live transaction, so its own statement sees its own \
         uncommitted write. This is what `READS_THROUGH_BATCH` names"
    );
    assert_eq!(
        store.probe_read("depot-7").await,
        Ok(None),
        "and nothing is durable until commit"
    );
}

/// The honest declaration compiles, and the probe answers **through the
/// port's own seam** rather than through an inherent method of this file.
///
/// Before ADR-0062 the only body that compiled here declared the capability
/// `false` and left `probe_read_through` as `unimplemented!()`; the suite then
/// took PS-12's rule as a reported skip and went green having exercised
/// nothing. This is that skip becoming a run.
#[tokio::test(flavor = "current_thread")]
async fn the_probe_reports_the_pending_write_through_the_batch() {
    const {
        assert!(
            <LiveTransactionStore as ProjectionProbe>::READS_THROUGH_BATCH,
            "a store whose batch is a live transaction declares the capability it has"
        );
    }

    let store = LiveTransactionStore::default();
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut batch, "van-3", 5)
        .await
        .expect("the probe write should succeed");

    assert_eq!(
        store.probe_read_through(&mut batch, "van-3").await,
        Ok(Some(5)),
        "the probe seam reaches the transaction's own uncommitted write"
    );
    assert_eq!(
        store.probe_read("van-3").await,
        Ok(None),
        "while the committed read model has not moved"
    );
}

/// A refused statement surfaces as the port's error rather than vanishing.
///
/// The old seam was infallible, so a store handed a live transaction had no
/// way to say a statement had failed and the transaction was poisoned. This is
/// the `Result` earning its place: a probe write that fails is an `Err` the
/// suite can see, not a silently empty batch.
#[tokio::test(flavor = "current_thread")]
async fn a_failed_statement_is_reported_through_the_seam() {
    let store = LiveTransactionStore::default();
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store.server.refuse_statements.store(true, Ordering::SeqCst);

    assert_eq!(
        store.probe_write(&mut batch, "yard-2", 4).await,
        Err(LiveError),
        "the refused statement is the port's error, surfaced where the suite          can decide not to commit the batch"
    );
    assert_eq!(
        store.probe_read_through(&mut batch, "yard-2").await,
        Ok(None),
        "and the transaction did not absorb the write it refused"
    );
}

/// The named wrong implementation PS-12 forbids, kept so it stays named.
///
/// Answering `probe_read_through` from committed state compiles under the new
/// seam exactly as it did under the old one — it is the shortest body that
/// type-checks — and returns `None` for a row the transaction can see. PS-12's
/// rule, `batch_reads_reflect_pending_writes`, is what rejects it; this test is
/// the wrong body written out so the rule has something to point at.
#[tokio::test(flavor = "current_thread")]
async fn answering_from_committed_state_still_compiles_and_is_still_wrong() {
    fn probe_read_through_from_committed(
        store: &LiveTransactionStore,
        _batch: &mut LiveTransaction,
        key: &str,
    ) -> Result<Option<u64>, LiveError> {
        let rows = store.server.rows.lock().map_err(|_| LiveError)?;
        Ok(rows.get(key).copied())
    }

    let store = LiveTransactionStore::default();
    let mut batch = store.begin().await.expect("opening a batch should succeed");
    store
        .probe_write(&mut batch, "hub-1", 9)
        .await
        .expect("the probe write should succeed");

    assert_eq!(
        probe_read_through_from_committed(&store, &mut batch, "hub-1"),
        Ok(None),
        "the committed-state body answers `None` for a row the transaction can \
         see. A suite believing this reads a pending write as absent"
    );
    assert_eq!(batch.select("hub-1").await, Ok(Some(9)));
}

// =====================================================================
// The pin: the seam's shape is recorded where the freeze decision reads
// =====================================================================

/// The port's own declaration of the member this file is about.
///
/// Read as text rather than named, because the property being pinned is that
/// **this exact receiver** — `&mut Self::Batch`, a future, a `Result` — is what
/// the section describes. A signature that slid back to `&Self::Batch` or to a
/// synchronous `Option<u64>` would make this store's honest body a compile
/// error again, and the section would be describing a seam that no longer
/// exists.
const DECLARATION: &str = "    fn probe_read_through(\n        &self,\n        batch: &mut Self::Batch,\n        key: &str,\n    ) -> impl Future<Output = Result<Option<u64>, Self::Error>>;";

/// The heading the port carries while that declaration stands.
const SECTION: &str = "# This signature is the one a live transaction can meet";

/// The section and the signature stand or fall together.
///
/// Two directions, and both matter. A signature that moves leaves a page
/// telling adapter authors the far end is reachable when it is not; a section
/// deleted while the signature stands loses the record of *why* the seam has
/// the shape it has, which is the record ADR-0060 was written to keep.
///
/// Known cost, stated rather than discovered: [`DECLARATION`] is matched as a
/// string, so reformatting the declaration fails this test for a cosmetic
/// reason. That is the price of pinning the receiver rather than the method
/// name, and the receiver is the whole subject.
///
/// The source is read with its line endings normalised. [`DECLARATION`] spans
/// four lines, and a checkout with `core.autocrlf` on hands `include_str!` a
/// `\r\n` file — which is how this test was green on every host but the
/// Windows runner, where it failed for a reason that had nothing to do with
/// what it asserts.
#[test]
fn the_recorded_seam_is_pinned_to_the_signature_it_describes() {
    let port = include_str!("../src/projection.rs").replace("\r\n", "\n");
    let port: &str = &port;
    assert_eq!(
        port.contains(DECLARATION),
        port.contains(SECTION),
        "`probe_read_through`'s declaration and the section recording why it has \
         that shape have come apart. If the signature moved, this file moves \
         with it; if the section went, the reason the seam is `&mut`, async and \
         fallible is no longer written where an adapter author reads"
    );
    assert!(
        port.contains(DECLARATION),
        "the declaration this file's honest body compiles against is gone"
    );
}
