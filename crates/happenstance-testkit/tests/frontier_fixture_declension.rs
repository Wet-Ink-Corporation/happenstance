//! What CF-18's emitted check says about `HELD_ABOVE_FRONTIER`, and when.
//!
//! `Fixture::HELD_ABOVE_FRONTIER` defaults to a declension in the testkit's
//! own words. For a store with no visibility frontier those words are a fact,
//! and the check lets the fixture leave them. For a store whose fixture
//! declines `READ_YOUR_OWN_WRITES` the words are false, because that
//! declension says reads trail commits, so the check rejects the silence
//! (ADR-0095).
//!
//! The three tests below are the three corners: a frontier fixture that says
//! nothing is rejected, the same fixture with a stated reason is accepted, and
//! a fixture with no frontier may say nothing. The first is what keeps the
//! check from being decorative; the other two are what keep it from being a
//! bar nobody can clear.
//!
//! Native only: the first test catches the check's panic, and `catch_unwind`
//! is not how a wasm32 target reports one.

#![cfg(not(target_arch = "wasm32"))]

use std::panic;

use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};
use happenstance_testkit::{Capability, Fixture};

/// The reasons every fixture below states, so that the only inherited
/// declension any of them can carry is the one under test.
const MID_BATCH_FAULT: Capability = Capability::declined(
    "this instrument's store appends under one lock, so no fault can fire between two rows",
);
const READ_FAULT: Capability = Capability::declined(
    "this instrument's store reads a Vec it already holds, so there is no fetch to fail",
);
const TRAILING_READS: Capability = Capability::declined(
    "this instrument says its reads trail its commits, which is the claim under test",
);

/// Declines `READ_YOUR_OWN_WRITES` and says nothing about
/// `HELD_ABOVE_FRONTIER`: the wrong implementation the coupled check rejects.
#[derive(Debug, Default)]
struct SilentFrontier(MemoryFixture);

impl Fixture for SilentFrontier {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined("a Vec in this process outlives nothing");
    const MID_BATCH_FAULT: Capability = MID_BATCH_FAULT;
    const READ_FAULT: Capability = READ_FAULT;
    const READ_YOUR_OWN_WRITES: Capability = TRAILING_READS;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// Declines `READ_YOUR_OWN_WRITES` and states its own `HELD_ABOVE_FRONTIER`.
#[derive(Debug, Default)]
struct StatedFrontier(MemoryFixture);

impl Fixture for StatedFrontier {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined("a Vec in this process outlives nothing");
    const MID_BATCH_FAULT: Capability = MID_BATCH_FAULT;
    const READ_FAULT: Capability = READ_FAULT;
    const READ_YOUR_OWN_WRITES: Capability = TRAILING_READS;
    const HELD_ABOVE_FRONTIER: Capability = Capability::declined(
        "this instrument's frontier is server-wide and cannot be held for one test \
         without stalling the others",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// Supports `READ_YOUR_OWN_WRITES` and says nothing about
/// `HELD_ABOVE_FRONTIER`, which is the honest silence of a store with no
/// frontier.
#[derive(Debug, Default)]
struct NoFrontier(MemoryFixture);

impl Fixture for NoFrontier {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined("a Vec in this process outlives nothing");
    const MID_BATCH_FAULT: Capability = MID_BATCH_FAULT;
    const READ_FAULT: Capability = READ_FAULT;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// A fixture that admits a frontier and inherits `HELD_ABOVE_FRONTIER`'s
/// default is rejected, by name.
#[test]
fn a_frontier_fixture_that_inherits_the_held_declension_is_rejected() {
    let previous = panic::take_hook();
    // The check below is *expected* to panic; without this the expected
    // backtrace is sprayed across a passing run.
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(|| {
        happenstance_testkit::__private::assert_declensions_are_stated(async || {
            SilentFrontier::default()
        });
    });
    panic::set_hook(previous);

    let payload = outcome.expect_err(
        "a fixture declining READ_YOUR_OWN_WRITES and inheriting \
         HELD_ABOVE_FRONTIER was accepted. Its CI log would print the testkit's \
         sentence that the store has no frontier, about a store that has just \
         said its reads trail its commits",
    );
    let message = payload
        .downcast_ref::<String>()
        .map_or("", String::as_str)
        .to_owned();
    assert!(
        message.contains("declines HELD_ABOVE_FRONTIER without saying why"),
        "the refusal must name the constant the author can change, and only \
         that one, since this fixture stated the other two: {message}"
    );
}

/// The same fixture, with its own reason, is accepted.
#[test]
fn a_frontier_fixture_that_states_the_held_declension_is_accepted() {
    happenstance_testkit::__private::assert_declensions_are_stated(async || {
        StatedFrontier::default()
    });
}

/// A fixture with no frontier may leave the default, because for it the
/// default is true.
#[test]
fn a_fixture_without_a_frontier_may_inherit_the_held_declension() {
    assert_eq!(
        NoFrontier::HELD_ABOVE_FRONTIER.reason(),
        Some(happenstance_testkit::UNSTATED_HELD_ABOVE_FRONTIER),
        "the instrument must actually inherit the default, or this test checks \
         nothing"
    );
    happenstance_testkit::__private::assert_declensions_are_stated(async || NoFrontier::default());
}
