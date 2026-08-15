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
//! `MemoryProjectionStore` is the oracle: it is *supposed* to pass, so this file
//! can only ever report that the machinery ran. The discrimination claim is
//! discharged elsewhere and by name: `CheckpointOnlyStore` is registered in
//! `tests/projection_mutation_coverage.rs` and fails
//! `commit_is_atomic_with_the_read_model` there, `PartialCommitStore` fails
//! `failed_commit_leaves_both_unchanged`, and every other rule in the family has
//! a registered store that fails exactly it
//! ([ADR-0010](../../../.kb/decisions/0010-the-suite-must-prove-itself.md)).
//! Read the two files together: a green run *here* plus a green run *there* is
//! the pair that means something.
//!
//! One rule in this run is answered by a `SKIP` rather than a pass.
//! `failed_commit_leaves_both_unchanged` is gated on `COMMIT_FAULT`, and the
//! reference fixture declines it with a reason the line carries — the reference
//! store applies both halves of a commit under one write lock and has no write
//! that can be made to fail. That is CF-18 working rather than a gap in this
//! file: the rule is emitted, answered and reported, and the adapter that can
//! arm a fault is the one that gets it checked.
//!
//! It is also only **one** of the two batch shapes this suite is now run
//! against. `projection_conformance_buffering.rs` drives the same rules,
//! unchanged, against a store whose batch is a replayable op journal and which
//! holds nothing between `begin` and `commit` — PS-4's shape. Both run inside
//! one `cargo xtask ci`, and it is the pair that lets a green projection run be
//! read as evidence about the *port* rather than about this store.
//!
//! The tokio harness, so: native only. The `wasm32` build of this same suite is
//! `projection_conformance_wasm.rs`, and the runtime-free one is
//! `projection_conformance_blocking.rs`. **No rule name appears in this file**,
//! which is CF-22's observable form: the set is written in
//! `for_each_projection_store_rule!` and nowhere else.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_testkit::fixtures::MemoryProjectionFixture;

happenstance_testkit::projection_store_conformance!(MemoryProjectionFixture::new());
