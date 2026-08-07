//! The same 27 rules, under `wasm-bindgen-test`.
//!
//! This is the third harness (CF-23), and the one that matters most to the
//! design. `wasm32-unknown-unknown` is single-threaded: its futures are
//! `!Send`, so a suite that had quietly settled on `#[tokio::test]` would be
//! untestable there — which is the same failure mode
//! [ADR-0001](../../../docs/adr/0001-async-port-flavours.md) exists to prevent
//! at the port level, reappearing one layer up in the test harness.
//!
//! Compiled for `wasm32-unknown-unknown`; run with `wasm-bindgen-test-runner`
//! under a headless browser or node. `cargo xtask wasm` type-checks it, which
//! is the part that can rot.

#![cfg(target_arch = "wasm32")]

use happenstance_core::MemoryEventStore;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    factory = MemoryEventStore::new()
);
