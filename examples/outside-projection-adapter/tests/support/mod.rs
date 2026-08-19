//! The fixtures, and why they are here rather than in `src/`.
//!
//! `happenstance-testkit` is a **dev-dependency**, so `ProjectionFixture` does
//! not exist for this crate's library build at all — a fixture written in
//! `src/lib.rs` would not compile. That is not an inconvenience; it is the shape
//! the extension surface is designed around, and it is why the *probe* impl has
//! to live in the library while the *fixture* impl lives out here.
//!
//! Both traits are local enough for the orphan rule in the place each is
//! written: `OutsideFixture` is defined in this crate, so implementing a foreign
//! trait for it is legal. `impl ProjectionProbe for OutsideProjectionStore` is
//! not, from here, because neither side is local — see the crate documentation
//! and the `error[E0117]` transcript in the story's gap record.
//!
//! One shared module rather than three copies, and `dead_code` allowed because
//! of it: each test binary compiles the whole module and uses one or both of the
//! fixtures in it.

#![allow(dead_code)]

use happenstance_testkit::{Capability, ProjectionFixture};
use outside_projection_adapter::{CheckpointOnlyStore, OutsideProjectionStore};

/// One isolated backing projection store; each `connect` is one handle onto it.
#[derive(Debug)]
pub(crate) struct OutsideFixture {
    store: OutsideProjectionStore,
}

impl OutsideFixture {
    /// One fresh, isolated backing store.
    pub(crate) fn new() -> Self {
        Self {
            store: OutsideProjectionStore::new(),
        }
    }
}

impl ProjectionFixture for OutsideFixture {
    type Store = OutsideProjectionStore;

    // A MUST rather than a trade, and this store can meet it honestly: the
    // backing state is behind an `Arc`, so a handle is a refcount bump and two
    // of them see each other's commits.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The one capability this fixture declines, in the store's own words. The
    // reason is read off the store rather than typed here, so a test asserting
    // on the skip cannot quietly assert on a different sentence.
    const RESET_REFUSAL: Capability =
        Capability::declined(OutsideProjectionStore::NO_RESET_PROTECTION);

    // Declared, because this store *can* be made to fail a commit — and having
    // declared it, it owes the rule an `Err` and an unchanged read model.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    async fn arm_commit_fault(&self) {
        self.store.arm_commit_fault();
    }

    async fn connect(&self) -> OutsideProjectionStore {
        self.store.handle()
    }
}

/// The same fixture contract, over the store that drops the read-model write.
///
/// It declares exactly what `OutsideFixture` declares except the fault switch,
/// so nothing about the *fixture* explains any rule it fails. Whatever goes red
/// is the store.
#[derive(Debug)]
pub(crate) struct CheckpointOnlyFixture {
    store: CheckpointOnlyStore,
}

impl CheckpointOnlyFixture {
    /// One fresh, isolated backing store.
    pub(crate) fn new() -> Self {
        Self {
            store: CheckpointOnlyStore::new(),
        }
    }
}

impl ProjectionFixture for CheckpointOnlyFixture {
    type Store = CheckpointOnlyStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const RESET_REFUSAL: Capability =
        Capability::declined(OutsideProjectionStore::NO_RESET_PROTECTION);

    const COMMIT_FAULT: Capability =
        Capability::declined("this store cannot be made to report a failed commit");

    async fn connect(&self) -> CheckpointOnlyStore {
        self.store.handle()
    }
}
