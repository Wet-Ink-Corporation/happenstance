//! The sync suite against the KV-capped reference peer: Turnstile's peer D, a
//! 128 KiB cap on each value, under the default tokio harness.
//!
//! The peer SY-18's marker names, and the third point in VT-21's comparison
//! across the peer set: a rule red here is a wrong rule or a wrong declaration.

#![cfg(not(target_arch = "wasm32"))]

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = kv_capped_sync_conformance,
    fixture = happenstance_sync_testkit::KvCappedFixture::new()
);
