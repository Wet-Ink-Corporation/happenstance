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
//! Compiled for `wasm32-unknown-unknown` and **run** with
//! `wasm-bindgen-test-runner` under node. Both halves are `cargo xtask ci`'s:
//! the mandatory *wasm32 check of the conformance harnesses* step type-checks
//! it, and the *wasm32 run of the conformance rules* step executes it. Which is
//! how the skip lines above become observable rather than merely emitted —
//! `MemoryProjectionFixture` declines two capabilities, and their stated reasons
//! reach the gate's own scroll through `--nocapture`.
//!
//! This file is a row in `xtask/src/proof.rs`'s `WASM_TARGETS`, held to
//! `for_each_projection_store_rule!` — its own enumeration, not the event
//! store's. That per-row family is what made the row expressible: the seam's
//! first cut hard-coded the event-store enumeration into both wasm32 entry
//! points, which left this harness compiled by the gate and executed by nothing
//! at all.

#![cfg(target_arch = "wasm32")]

use happenstance_testkit::fixtures::MemoryProjectionFixture;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance_wasm,
    emit = happenstance_testkit::__emit_projection_wasm,
    fixture = MemoryProjectionFixture::new()
);
