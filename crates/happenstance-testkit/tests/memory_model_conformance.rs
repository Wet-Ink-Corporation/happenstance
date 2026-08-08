//! The model-based suite, against the reference implementation.
//!
//! `memory_conformance.rs`'s sibling, and it runs in the same two directions:
//! it checks that `MemoryEventStore` agrees with the model, and — because a
//! model-based test that no store can pass is a bug in the model — that the
//! model is a faithful reading of the contract rather than a second opinion
//! about it.
//!
//! The gate that runs this is `cargo test --workspace --all-features`, which is
//! where the testkit's `proptest` feature is on. Both conditions in the `cfg`
//! below are load-bearing: a feature is not target-scoped, so `--all-features`
//! sets `proptest` on `wasm32` as well, where the crate is not in the graph and
//! `event_store_model_conformance!` therefore does not exist. An adapter with a
//! wasm32 harness gates its own invocation exactly like this.
//!
//! Which mutants this rule rejects is measured, not assumed:
//! `mutation_coverage::the_model_rule_rejects_exactly_what_it_claims` is the
//! measurement, and it is in the proof artefact rather than here because that is
//! where the wrong stores live.
//!
//! Both shipped emitters are exercised below. CF-23 requires three harnesses for
//! the event-store family and this family can only have two — `wasm-bindgen-test`
//! is unreachable for a rule set that does not exist on `wasm32` — so the
//! demonstration is that the emitter is still a *parameter*, which is the part
//! of CF-23 that is about design rather than about targets.

#![cfg(all(not(target_arch = "wasm32"), feature = "proptest"))]

use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_model_conformance!(MemoryFixture::new());

happenstance_testkit::event_store_model_conformance!(
    mod_name = dcb_model_conformance_blocking,
    emit = happenstance_testkit::__emit_model_blocking,
    fixture = MemoryFixture::new()
);
