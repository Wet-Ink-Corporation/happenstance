//! The projection suite over an **unarmed** `RecordingProjectionStore`.
//!
//! The same bar `FaultyStore` and `GappyMemoryStore` clear for the event
//! store: an instrument that wraps the reference store must not be lying
//! about rows, checkpoints or atomicity while it fails nothing, or a runner
//! test built on it proves nothing. Its clones are handles onto one store,
//! so the second-handle rules run.
//!
//! `emit_projection_blocking`, because the store is `!Send` by design and a
//! plain `#[test]` over `block_on` asks nothing of it.

#![cfg(all(feature = "memory", not(target_arch = "wasm32")))]

use happenstance_testkit::{Capability, ProjectionFixture, RecordingProjectionStore};

/// One recording store; each `connect` is a clone, a handle onto it.
#[derive(Debug, Default)]
struct RecordingFixture(RecordingProjectionStore);

impl ProjectionFixture for RecordingFixture {
    type Store = RecordingProjectionStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const RESET_REFUSAL: Capability = Capability::declined(
        "RecordingProjectionStore wraps MemoryProjectionStore, which holds no \
         protection policy, so `reset` returns `Refused` on no path",
    );
    const COMMIT_FAULT: Capability = Capability::declined(
        "RecordingProjectionStore's commit failure is fixed at construction for \
         one checkpoint id, not armed per call, and this run is the unarmed \
         store's: arming it is what its own instrument tests do",
    );
    const REOPEN: Capability = Capability::declined(
        "RecordingProjectionStore wraps MemoryProjectionStore, whose rows and \
         checkpoints live in process memory alone, so there is no medium a \
         reopen could read them back from",
    );

    fn connect(&self) -> impl Future<Output = RecordingProjectionStore> {
        // A clone is a second handle onto the same rows and log (its page).
        core::future::ready(self.0.clone())
    }
}

happenstance_testkit::projection_store_conformance!(
    mod_name = recording_store_conformance,
    emit = happenstance_testkit::emit_projection_blocking,
    fixture = RecordingFixture::default()
);
