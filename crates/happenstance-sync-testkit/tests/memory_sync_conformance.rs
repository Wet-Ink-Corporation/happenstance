//! The sync suite against its reference peer, under the default tokio harness.
//!
//! `MemorySyncFixture` is the peer every rule is proven against before it may
//! fail an adapter: a rule red here is a wrong rule.

#![cfg(not(target_arch = "wasm32"))]

happenstance_sync_testkit::sync_peer_conformance!(
    happenstance_sync_testkit::MemorySyncFixture::new()
);
