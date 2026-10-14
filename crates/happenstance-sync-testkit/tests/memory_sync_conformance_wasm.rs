//! The same rules under `wasm-bindgen-test`, on the target the `!Send` peer
//! lives on.
//!
//! Compiled by `cargo xtask ci`'s `wasm32 build of the sync crate` step and
//! executed by its `wasm32 run of the conformance rules` step, through this
//! target's row in `xtask/src/proof.rs`'s `WASM_TARGETS`. That row holds it to
//! `for_each_sync_peer_rule!` exhaustively, so a rule that reaches the host
//! harnesses and not this one fails the gate.

#![cfg(target_arch = "wasm32")]

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = sync_conformance_wasm,
    emit = happenstance_sync_testkit::emit_sync_wasm,
    fixture = happenstance_sync_testkit::MemorySyncFixture::new()
);
