//! The same projection rules, under `wasm-bindgen-test`.
//!
//! The third harness (CF-23), and the one that matters most to the design.
//! `wasm32-unknown-unknown` is single-threaded: its futures are `!Send`, so a
//! family that had quietly settled on `#[tokio::test]` would be untestable
//! there — which is the failure mode
//! [ADR-0001](../../../.kb/decisions/0001-async-port-flavours.md) exists to
//! prevent at the port level, reappearing one layer up in the harness.
//!
//! It is also where a *stated reason* would disappear if nobody looked.
//! `RuleOutcome::report` is a measured no-op on this target, so
//! `__emit_projection_wasm` calls `skip_line` and hands the string to
//! `console_log!` instead.
//!
//! Compiled for `wasm32-unknown-unknown`; run with `wasm-bindgen-test-runner`
//! under a headless browser or node. The mandatory *wasm32 check of the
//! conformance harnesses* step type-checks it, which is the part that can rot.

#![cfg(target_arch = "wasm32")]

use happenstance_testkit::fixtures::MemoryProjectionFixture;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance_wasm,
    emit = happenstance_testkit::__emit_projection_wasm,
    fixture = MemoryProjectionFixture::new()
);
