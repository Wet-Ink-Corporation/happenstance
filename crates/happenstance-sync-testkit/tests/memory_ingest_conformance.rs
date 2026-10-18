//! The ingest family against its reference store, under the default tokio
//! harness.
//!
//! `MemoryIngestFixture` is the store every ingest rule is proven against
//! before it may fail an adapter: a rule red here is a wrong rule.

#![cfg(not(target_arch = "wasm32"))]

happenstance_sync_testkit::ingest_store_conformance!(
    happenstance_sync_testkit::MemoryIngestFixture::new()
);
