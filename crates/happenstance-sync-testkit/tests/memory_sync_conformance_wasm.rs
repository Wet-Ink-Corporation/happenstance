//! The same rules under `wasm-bindgen-test`, on the target the `!Send` peer
//! lives on.
//!
//! Compiled by `cargo xtask ci`'s `wasm32 build of the sync crate` step, which
//! checks this crate with `--tests`. Nothing in the gate *runs* it yet: the
//! executed wasm32 targets are `happenstance-testkit`'s, enumerated in
//! `xtask/src/proof.rs`.

#![cfg(target_arch = "wasm32")]

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = sync_conformance_wasm,
    emit = happenstance_sync_testkit::emit_sync_wasm,
    fixture = happenstance_sync_testkit::MemorySyncFixture::new()
);
