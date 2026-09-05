//! Whether the adapter shape PS-2 still needs can implement `probe_read_through`.
//!
//! PS-2 is `[FROZEN]` and names the two ends of the batch-shape axis the port
//! must be proved against. One end is unbuilt, and ADR-0036 says so: *"one
//! adapter holding a live transaction (rusqlite or `sqlx`)"*. Every store that
//! has ever declared `READS_THROUGH_BATCH = true` in this workspace answers from
//! an in-process map or a buffer, and the one adapter that has run the suite
//! declares `false`.
//!
//! This file asks whether that is scarcity or structure, and answers it by
//! construction. `LiveTransactionStore`'s batch **is** a transaction: statements
//! are issued as they are made, against a connection the batch owns, and reading
//! one back is real I/O — `&mut` because a driver borrows the connection to run a
//! statement (`sqlx`'s `Executor for &mut Transaction`, `rusqlite`'s `&mut
//! Transaction`), and `.await` because it yields.
//!
//! Nothing here proposes a signature. `ProjectionProbe` is behind `conformance`
//! and the port beneath it is `[PROVISIONAL]`; whether the probe's shape moves
//! belongs to whoever owns PS-2's clause. What this file supplies is the
//! measurement that decision would otherwise be taken without: the three bodies
//! the current signature admits for this store, each run rather than argued.
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
use std::panic::AssertUnwindSafe;
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
    /// Reads one row through the open transaction, committed state included.
    ///
    /// **This is the method `ProjectionProbe::probe_read_through` exists to
    /// expose, and its signature is the whole finding.** `&mut self` because the
    /// driver borrows the connection to issue a statement; `async` because the
    /// statement is I/O; `Result` because a statement on a live transaction can
    /// fail, and after it does the transaction is poisoned rather than merely
    /// unhelpful.
    async fn select(&mut self, key: &str) -> Result<Option<u64>, LiveError> {
        // A real yield, not a formality: it is what makes this future
        // unbridgeable from a synchronous caller already inside a runtime.
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

    fn begin(&self) -> Self::Batch {
        LiveTransaction {
            server: Arc::clone(&self.server),
            uncommitted: BTreeMap::new(),
            clear_all: false,
        }
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let checkpoints = self.server.checkpoints.lock().map_err(|_| LiveError)?;
        Ok(checkpoints.get(id).cloned().unwrap_or(Checkpoint::NeverRun))
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

/// The probe, written the only way the current signature permits.
///
/// `READS_THROUGH_BATCH` is `false`, and that is **a false statement about this
/// store** — the transaction above reads its own uncommitted writes by
/// construction, which `the_transaction_reads_its_own_uncommitted_writes` runs.
/// It is declared `false` because the alternative bodies are worse, and the
/// three tests below are what "worse" means, measured.
impl ProjectionProbe for LiveTransactionStore {
    const READS_THROUGH_BATCH: bool = false;

    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.uncommitted.insert(key.to_owned(), value);
    }

    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.uncommitted.clear();
        batch.clear_all = true;
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        tokio::task::yield_now().await;
        let rows = self.server.rows.lock().map_err(|_| LiveError)?;
        Ok(rows.get(key).copied())
    }

    fn probe_read_through(&self, _batch: &Self::Batch, _key: &str) -> Option<u64> {
        unimplemented!("a live transaction cannot be read synchronously or infallibly")
    }
}

// =====================================================================
// What the store can do, and what the probe can report about it
// =====================================================================

/// The property PS-2's unbuilt end is defined by. It holds.
#[tokio::test(flavor = "current_thread")]
async fn the_transaction_reads_its_own_uncommitted_writes() {
    let store = LiveTransactionStore::default();
    let mut batch = store.begin();
    store.probe_write(&mut batch, "depot-7", 12);

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

/// Body 1 — decline the capability. What every adapter does today, and the
/// declaration is false.
#[tokio::test(flavor = "current_thread")]
async fn declining_the_capability_is_the_only_body_that_compiles_and_it_lies() {
    assert!(
        !<LiveTransactionStore as ProjectionProbe>::READS_THROUGH_BATCH,
        "the shipped signature admits no other honest declaration for this store"
    );

    // The declaration is false, and the falseness is measurable rather than
    // rhetorical: the same batch, read through its own async path, answers.
    let store = LiveTransactionStore::default();
    let mut batch = store.begin();
    store.probe_write(&mut batch, "van-3", 5);
    assert_eq!(batch.select("van-3").await, Ok(Some(5)));

    // PS-12's rule is emitted as a reported skip carrying this store's stated
    // reason, so the suite goes green having exercised nothing. That is PS-2
    // part 2 being recorded as met by an adapter whose defining property was
    // never run.
}

/// Body 2 — answer from committed state. Compiles, and returns the wrong answer.
///
/// Not hypothetical: it is the shortest body that type-checks, and PS-12 is the
/// only thing that forbids it. Written out here so the wrong implementation is
/// named rather than left to be inferred.
#[tokio::test(flavor = "current_thread")]
async fn answering_from_committed_state_compiles_and_is_wrong() {
    fn probe_read_through_from_committed(
        store: &LiveTransactionStore,
        _batch: &LiveTransaction,
        key: &str,
    ) -> Option<u64> {
        let rows = store.server.rows.lock().ok()?;
        rows.get(key).copied()
    }

    let store = LiveTransactionStore::default();
    let mut batch = store.begin();
    store.probe_write(&mut batch, "hub-1", 9);

    assert_eq!(
        probe_read_through_from_committed(&store, &batch, "hub-1"),
        None,
        "the committed-state body answers `None` for a row the transaction can \
         see. A suite believing this reads a pending write as absent"
    );
    assert_eq!(batch.select("hub-1").await, Ok(Some(9)));
}

/// Body 3 — block on the future. Panics, inside the runtime the suite runs in.
///
/// The suite drives an async port, so the probe is always called from inside a
/// runtime. `Handle::block_on` refuses there by design; on a multi-thread
/// runtime `block_in_place` would be needed as well, which a current-thread
/// runtime does not have and `wasm32` has neither of.
#[tokio::test(flavor = "current_thread")]
async fn blocking_on_the_future_panics_inside_the_runtime_the_suite_uses() {
    let store = LiveTransactionStore::default();
    let mut batch = store.begin();
    store.probe_write(&mut batch, "yard-2", 4);

    let handle = tokio::runtime::Handle::current();
    let blocked = std::panic::catch_unwind(AssertUnwindSafe(|| {
        handle.block_on(async { batch.select("yard-2").await })
    }));

    let panic = blocked.expect_err("`block_on` inside a runtime is not permitted");
    let message = panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&str>().copied())
        .unwrap_or("<non-string panic>");
    assert!(
        message.contains("blocking") || message.contains("runtime"),
        "the panic is the runtime refusing a nested block_on, and its wording is \
         tokio's rather than ours; got: {message}"
    );
}

/// The probe, called. There is one body and it is a panic.
#[test]
#[should_panic(expected = "a live transaction cannot be read synchronously")]
fn calling_the_probe_is_the_third_outcome_and_it_is_a_panic() {
    let store = LiveTransactionStore::default();
    let batch = store.begin();
    let _ = store.probe_read_through(&batch, "depot-7");
}

// =====================================================================
// The pin: the constraint is recorded where the freeze decision reads
// =====================================================================

/// The port's own declaration.
///
/// Read as text rather than named, because the property being pinned is that
/// **this exact receiver** is what the section describes.
const DECLARATION: &str =
    "fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;";

/// The heading the port carries while that declaration stands.
const SECTION: &str = "# This signature cannot be met by a batch that is a live transaction";

/// The section and the signature stand or fall together.
///
/// Two directions, and both matter. A signature that moves — to `&mut`, to a
/// hand-desugared `impl Future`, to a `Result` — leaves a page telling adapter
/// authors about an obstacle that is gone; and a section deleted while the
/// signature stands puts the workspace back where the audit found it, with the
/// far end of PS-2's axis unbuilt and nothing saying why.
///
/// Not a substitute for reading this file: the three bodies above are the
/// evidence, and this is only what keeps them attached to the thing they are
/// evidence about.
///
/// Known cost, stated rather than discovered: [`DECLARATION`] is matched as a
/// string, so renaming the `batch` parameter fails this test for a cosmetic
/// reason. That is the price of pinning the receiver rather than the method
/// name, and the receiver is the whole subject. The message says which of the
/// two came apart, so the fix is a one-line edit here.
#[test]
fn the_recorded_constraint_is_pinned_to_the_signature_it_describes() {
    const PORT: &str = include_str!("../src/projection.rs");
    assert_eq!(
        PORT.contains(DECLARATION),
        PORT.contains(SECTION),
        "`probe_read_through`'s declaration and the section recording what it \
         costs a live-transaction adapter have come apart. If the signature \
         moved, this file moves with it; if the section went, PS-2's owner is \
         back to reading the absence of an adapter as scarcity"
    );
}

/// The section says "compiler-checked". Deleting the fences makes it prose.
///
/// Added because reverting the section leg by leg found this one pinned by
/// nothing: the heading and the paragraphs survived the removal of both fences
/// and the whole gate stayed green, leaving a page that *claims* a compiler
/// checked something no compiler is looking at.
///
/// The error codes are named, not just the fences. `compile_fail` on its own
/// passes when the snippet fails to compile for any reason at all — a typo, a
/// missing import — which is the wrong implementation this rule forbids, and it
/// is the exact failure mode `standards/rust/60-what-a-test-must-prove.md`
/// describes for a test that cannot say what it rejected.
#[test]
fn the_two_halves_of_the_obstacle_stay_compiler_checked() {
    const PORT: &str = include_str!("../src/projection.rs");
    if !PORT.contains(SECTION) {
        return; // The test above owns that failure; this one would only echo it.
    }
    for fence in ["```compile_fail,E0596", "```compile_fail,E0728"] {
        assert!(
            PORT.contains(fence),
            "the section claims both halves are compiler-checked, and `{fence}` \
             is gone. E0596 is the mutable borrow a driver needs to issue a \
             statement; E0728 is the await the statement is. Prose asserting \
             either is what this file exists to replace"
        );
    }
}
