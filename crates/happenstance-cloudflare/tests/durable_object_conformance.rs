//! Every event-store conformance rule, executed against a real Durable Object.
//!
//! This is CF-23's **third harness** — `wasm-bindgen-test` — applied for the
//! first time to a real adapter rather than to the testkit's own fixture. The
//! two harnesses that came before it run against stores that happen to be
//! single-threaded; this one runs against the workspace's only `!Send` store, on
//! the target the whole two-flavour port design (ADR-0001) was paid for.
//!
//! # Three lines, and the length is the point
//!
//! What follows is `crates/happenstance-testkit/tests/memory_conformance_wasm.rs`
//! with one expression changed. It defines no `macro_rules!`, names no rule,
//! carries no `#[cfg]` over any individual rule, and writes no emitter of its
//! own — because the one wrong implementation nothing in this tree can grep for
//! is a *bespoke emitter*: a macro that wraps most rules in
//! `#[wasm_bindgen_test]` and quietly drops the three this runtime cannot pass
//! would satisfy `registry::no_orphan_rules` (which reads `suite.rs`'s source
//! and never looks at a harness) and `mutation_coverage::capability_skips_are_reported`
//! (which runs inside the *testkit's* binary and never sees an adapter's
//! harness). The gate would print green over a suite three rules short and the
//! count appears nowhere.
//!
//! The detector for that lives outside this process, in `xtask/src/proof.rs`:
//! this target's row there asserts every rule the enumeration declares out of
//! the target's own `cargo test -- --list` **before** anything runs. libtest
//! exposes nothing programmatically and a `#[test]` cannot enumerate the binary
//! it lives in, which is why CF-18's emission half was handed to `xtask` in
//! writing (`crates/happenstance-testkit/tests/mutation_coverage.rs`).
//!
//! # What is not invoked here, and why that is a sentence rather than a silence
//!
//! The **concurrency** family. `event_store_concurrency_conformance!` binds
//! `F::Store: EventStore + Send` and its module is
//! `#[cfg(not(target_arch = "wasm32"))]`: a `!Send` adapter cannot invoke it and
//! is not expected to. The **model** family is off by default behind `proptest`,
//! whose module carries the same target condition because a Cargo feature is not
//! target-scoped — so it is not in the dependency graph on `wasm32` at all. Both
//! are stated at length in `crates/happenstance-cloudflare/src/lib.rs` and in
//! `tests/support/mod.rs`, where a reader of the crate lands.
//!
//! # This target is empty on the host, and that is correct
//!
//! `#![cfg(target_arch = "wasm32")]` means an ordinary
//! `cargo test -p happenstance-cloudflare` reports it as an empty target. That
//! is not the vacuity the `xtask` row guards against — that guard runs against
//! the `wasm32` build — and it is not something to "fix". The fixture's own
//! contract has a host-native half; the *rules* need a JavaScript heap.

#![cfg(target_arch = "wasm32")]

mod support;

use support::CloudflareFixture;

happenstance_testkit::event_store_conformance!(
    mod_name = dcb_conformance_wasm,
    emit = happenstance_testkit::__emit_wasm,
    fixture = CloudflareFixture::new()
);
