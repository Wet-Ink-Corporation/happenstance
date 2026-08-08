//! The concurrency family, against the reference implementation.
//!
//! `memory_conformance.rs`'s sibling, and it runs in the same two directions: it
//! checks that `MemoryEventStore` survives eight contenders, and — because a
//! concurrency suite no store can pass is a bug in the suite — that the rules
//! are a faithful reading of the contract rather than a stress test with
//! assertions bolted on.
//!
//! `MemoryEventStore` is an `RwLock` around a `Vec`, so it serialises its
//! writers and passes these rules for a structural reason rather than a lucky
//! one. That is worth being explicit about: **this harness is the demonstration
//! that the rules can be passed, not the demonstration that they can fail.**
//! The second half lives in `tests/mutation_coverage.rs`, where five racing
//! stores that are each wrong in one way are driven through the same five rules
//! and their verdicts pinned — `the_concurrency_rules_reject_exactly_what_they_claim`.
//!
//! Native only, and the `cfg` is load-bearing rather than tidy:
//! `happenstance_testkit::concurrency` does not exist on
//! `wasm32-unknown-unknown`, which has no threads to race on. An adapter with a
//! wasm32 harness gates its own invocation exactly like this.
//!
//! Both shipped emitters are exercised. CF-23's content is that the wrapper is a
//! parameter, and this family can have only two — `wasm_bindgen_test` is
//! unreachable for a rule set that does not exist on that target.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_concurrency_conformance!(MemoryFixture::new());

happenstance_testkit::event_store_concurrency_conformance!(
    mod_name = dcb_concurrency_conformance_blocking,
    emit = happenstance_testkit::__emit_concurrency_blocking,
    fixture = MemoryFixture::new()
);
