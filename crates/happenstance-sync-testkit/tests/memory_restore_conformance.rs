//! The restore family against its reference: a store peer over the in-memory
//! oracle, restored into a fresh incarnation (VT-6's re-mint).
//!
//! A rule red here is a wrong rule, not a wrong peer.

#![cfg(not(target_arch = "wasm32"))]

happenstance_sync_testkit::restore_conformance!(
    happenstance_sync_testkit::MemoryRestoreFixture::new()
);

// The blocking harness too, so the emitter a `!Send` peer runs under is
// exercised by a mount rather than only by its documentation.
happenstance_sync_testkit::restore_conformance!(
    mod_name = blocking_restore_conformance,
    emit = happenstance_sync_testkit::emit_restore_blocking,
    fixture = happenstance_sync_testkit::MemoryRestoreFixture::new()
);
