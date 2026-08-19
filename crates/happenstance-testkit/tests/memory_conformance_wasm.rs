//! The same rules as the host harnesses, under `wasm-bindgen-test`.
//!
//! Deliberately no count in that sentence. It read "the same 30 rules" while the
//! enumeration held eighty-nine — a number in prose beside the list it describes
//! is a second copy of that list, and it drifts the first time the list moves.
//! The current answer is printed by the gate's own wasm32 steps.
//!
//! This is the third harness (CF-23), and the one that matters most to the
//! design. `wasm32-unknown-unknown` is single-threaded: its futures are
//! `!Send`, so a suite that had quietly settled on `#[tokio::test]` would be
//! untestable there — which is the same failure mode
//! [ADR-0001](../../../.kb/decisions/0001-async-port-flavours.md) exists to prevent
//! at the port level, reappearing one layer up in the test harness.
//!
//! It is also CF-20's guard. [`Fixture`](happenstance_testkit::Fixture) carries
//! no `Send` bound and is not `trait_variant`-derived, and this is the target on
//! which a bound that crept back in would stop compiling rather than merely
//! being redundant.
//!
//! Compiled for `wasm32-unknown-unknown` and **run** with
//! `wasm-bindgen-test-runner` under node. Both halves are `cargo xtask ci`'s:
//! the `wasm32 check of the conformance harnesses` step type-checks it, and the
//! `wasm32 run of the conformance rules` step executes it. Until HS-S0048 only
//! the first was true here and the second lived in a GitHub Actions job, which
//! is a different claim — `#[tokio::test]` type-checks for this target and then
//! cannot run on it, and that is the whole of what CF-23 is about.
//!
//! The gate holds this file to the one rule enumeration in both directions:
//! every rule `for_each_event_store_rule!` declares must appear in this target's
//! own `--list`, and this file may name no rule of its own. A wasm32-only subset
//! is the failure both halves exist to refuse. See `xtask/src/proof.rs`'s
//! `WASM_TARGETS`.

#![cfg(target_arch = "wasm32")]

use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    fixture = MemoryFixture::new()
);
