//! The model family against its reference peer, under the default tokio harness.
//!
//! `MemorySyncFixture` is the peer every rule is proven against before it may
//! fail an adapter: a rule red here is a wrong rule. The family opens one
//! fixture per node of a generated topology, so each node is its own isolated
//! peer.

#![cfg(not(target_arch = "wasm32"))]

happenstance_sync_testkit::sync_model_conformance!(
    happenstance_sync_testkit::MemorySyncFixture::new()
);
