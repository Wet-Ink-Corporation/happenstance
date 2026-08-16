//! The whole suite against an **unarmed** `SendFaultyStore`.
//!
//! CLAUDE.md's rule that matters admits no exception for a store that ships in
//! a testkit, and this is the strong claim rather than the ceremonial one: a
//! wrapper that mangles ordering, positions, identity or errors when nothing is
//! injected is exactly the failure a delegating `read`/`append` invites, and it
//! would be invisible to every test in `faulty_store_instruments.rs`, all of
//! which arm something first.
//!
//! Native only — the harness is `#[tokio::test]`.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};
use happenstance_testkit::{Capability, Fixture, SendFaultyStore};

/// One `MemoryEventStore` behind an unarmed wrapper, any number of handles.
///
/// It delegates its isolation to [`MemoryFixture`] rather than reimplementing
/// it, which is what keeps the only difference between this fixture and the
/// reference one the wrapper itself.
struct FaultyFixture(MemoryFixture);

impl FaultyFixture {
    fn new() -> Self {
        Self(MemoryFixture::new())
    }
}

impl Fixture for FaultyFixture {
    type Store = SendFaultyStore<MemoryHandle>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // Inherited honestly rather than re-answered: the wrapper adds no durable
    // medium, so the reason `MemoryFixture` declines is unchanged by wrapping
    // it. Restating it here is what the suite prints.
    const REOPEN: Capability = Capability::declined(
        "the wrapped store is a MemoryEventStore — a Vec behind an RwLock — so \
         there is no durable medium to reopen over, and FaultyStore adds none: \
         discarding process state is indistinguishable from discarding the \
         events",
    );

    async fn connect(&self) -> Self::Store {
        SendFaultyStore::new(self.0.connect().await)
    }
}

happenstance_testkit::event_store_conformance!(FaultyFixture::new());
