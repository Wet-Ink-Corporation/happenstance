//! Validates the projection conformance suite against the reference projection
//! store.
//!
//! This runs in both directions at once, exactly as `memory_conformance.rs`
//! does for the event-store family. It checks that
//! [`MemoryProjectionStore`](happenstance_core::MemoryProjectionStore) keeps
//! PS-1's coupling — the read-model write and the checkpoint write become
//! durable together or not at all — and, more importantly, that the projection
//! rules themselves are sane. A rule no correct store can pass is worse than no
//! rule at all.
//!
//! What a green run here does **not** prove is that the suite discriminates.
//! `MemoryProjectionStore` is the oracle: it is *supposed* to pass. The store
//! that must fail `commit_is_atomic_with_the_read_model` — `CheckpointOnlyStore`
//! — is `projection-mutant-registry`'s, and until it lands, this file proves the
//! machinery runs rather than that it can fail
//! ([ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md)).
//!
//! The tokio harness, so: native only. The `wasm32` build of this same suite is
//! `projection_conformance_wasm.rs`, and the runtime-free one is
//! `projection_conformance_blocking.rs`. **No rule name appears in this file**,
//! which is CF-22's observable form: the set is written in
//! `for_each_projection_store_rule!` and nowhere else.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_testkit::fixtures::MemoryProjectionFixture;

happenstance_testkit::projection_store_conformance!(MemoryProjectionFixture::new());
