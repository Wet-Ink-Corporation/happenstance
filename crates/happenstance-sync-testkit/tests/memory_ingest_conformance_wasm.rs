//! The ingest rules under `wasm-bindgen-test`, against their reference store,
//! on the target the `!Send` ingest store lives on.
//!
//! Executed by `cargo xtask ci`'s `wasm32 run of the conformance rules` step,
//! through this target's row in `xtask/src/proof.rs`'s `WASM_TARGETS`. That row
//! holds it to `for_each_ingest_store_rule!` exhaustively, which is what holds
//! `emit_ingest_wasm!` to the whole family on an executed target.

#![cfg(target_arch = "wasm32")]

happenstance_sync_testkit::ingest_store_conformance!(
    mod_name = ingest_conformance_wasm,
    emit = happenstance_sync_testkit::emit_ingest_wasm,
    fixture = happenstance_sync_testkit::MemoryIngestFixture::new()
);
