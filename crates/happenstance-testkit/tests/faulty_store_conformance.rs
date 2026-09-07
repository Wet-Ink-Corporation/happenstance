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
///
/// # Why it holds one wrapper rather than building one per `connect`
///
/// It used to wrap a freshly-connected [`MemoryHandle`] on every `connect`,
/// which gave every handle its **own** arming. That was invisible while nothing
/// armed anything through the fixture, and stopped being invisible the moment
/// `Fixture::arm_read_fault` existed: a rule connects first and arms afterwards,
/// so an arming that reaches only handles opened later reaches nothing the rule
/// is holding. `SendFaultyStore`'s arming sits behind an `Arc` and its `Clone`
/// shares it, so one prototype cloned per `connect` is one fixture, one arming,
/// any number of handles — which is what a real adapter's fault seam looks like
/// too.
struct FaultyFixture(SendFaultyStore<MemoryHandle>);

impl FaultyFixture {
    fn new() -> Self {
        // `MemoryFixture::connect` returns `core::future::ready`, so this
        // resolves without ever suspending. `block_on` here is not a runtime,
        // it is how a synchronous constructor spends an infallible future.
        let handle = happenstance_testkit::block_on(MemoryFixture::new().connect());
        Self(SendFaultyStore::new(handle))
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

    // Supported, and this fixture is the reason the read-fault rule is not
    // decorative: `SendFaultyStore` exists to produce "the input no in-process
    // store produces", and a read that fails is exactly that input. It is the
    // instrument CF-26 asks for — proof the rule can *pass* — with
    // `SwallowedReadFaultStore` in `tests/mutation_coverage/mutants.rs` proving
    // it can fail.
    const READ_FAULT: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        // A clone, not a fresh wrapper: see the type's documentation. The clone
        // shares the arming, which is what lets `arm_read_fault` below reach a
        // handle a rule is already holding.
        self.0.clone()
    }

    async fn arm_read_fault(&self) {
        // `fail_next_read` is a builder taking `self` by value, and the arming
        // it sets lives behind the `Arc` every clone shares — so setting it on a
        // throwaway clone sets it for every handle, which is the whole point.
        // The returned value is dropped deliberately; `#[must_use]` is there to
        // catch a caller who meant to keep a *newly built* store, not this.
        let _armed = self.0.clone().fail_next_read(1);
    }
}

happenstance_testkit::event_store_conformance!(FaultyFixture::new());
