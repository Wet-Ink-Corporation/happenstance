//! What `Fixture::REMOVAL` says when a fixture says nothing, and what CF-18's
//! emitted check does about it.
//!
//! `REMOVAL` defaults to a declension in the testkit's own words, on
//! `MID_BATCH_FAULT`'s precedent, and `remove_outside_port`'s provided body
//! answers `Removed::Nothing`. The default's words ("has not said … given by
//! default rather than by its author") are true of every silent fixture, which
//! is `HELD_ABOVE_FRONTIER`'s argument for a store without a frontier, so the
//! emitted check accepts the silence. A fixture that declares the capability
//! and forgets the override is caught by the rule instead, on every run.
//!
//! Native only, for `frontier_fixture_declension.rs`'s reason: the check under
//! test fails by panicking.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_core::{EventStore, Query, ReadOptions, SequencePosition, collect};
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle, event};
use happenstance_testkit::{Capability, Fixture, Removed, UNSTATED_REMOVAL, block_on};

/// Says nothing about `REMOVAL`, and states every other defaulted declension,
/// so that the only inherited one it carries is the one under test.
#[derive(Debug, Default)]
struct SilentRemoval(MemoryFixture);

impl Fixture for SilentRemoval {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined("a Vec in this process outlives nothing");
    const MID_BATCH_FAULT: Capability = Capability::declined(
        "this instrument's store appends under one lock, so no fault can fire between two rows",
    );
    const READ_FAULT: Capability = Capability::declined(
        "this instrument's store reads a Vec it already holds, so there is no fetch to fail",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// A fixture silent on `REMOVAL` declines it, in the testkit's words.
#[test]
fn a_fixture_silent_on_removal_declines_it_in_the_testkits_words() {
    assert!(
        !SilentRemoval::REMOVAL.is_supported(),
        "a fixture that says nothing about REMOVAL must decline it: a default of \
         supported would send every silent fixture into a rule its store never \
         co-operated with"
    );
    assert_eq!(
        SilentRemoval::REMOVAL.reason(),
        Some(UNSTATED_REMOVAL),
        "the declension a silent fixture inherits is `UNSTATED_REMOVAL`, so a \
         reader of the skip line can tell the testkit's words from the adapter's"
    );
}

/// The provided body removes nothing, and says so.
#[test]
fn the_provided_removal_removes_nothing_and_says_so() {
    let fixture = SilentRemoval::default();
    let store = block_on(fixture.connect());
    let position: SequencePosition =
        block_on(store.append(&[event("Kept")], None)).expect("an unconditional append lands");

    let answer = block_on(fixture.remove_outside_port(&[position]));

    assert_eq!(
        answer,
        Removed::Nothing,
        "the provided body must answer `Nothing`: that answer is how the rule \
         fails a fixture that declares REMOVAL and forgets the override"
    );
    let held: Vec<SequencePosition> =
        block_on(collect(store.read(&Query::all(), ReadOptions::new())))
            .expect("the read succeeds")
            .iter()
            .map(|held| held.position)
            .collect();
    assert_eq!(
        held,
        [position],
        "and it must leave the store as it found it"
    );
}

/// CF-18's emitted check accepts a fixture silent on `REMOVAL`, because the
/// inherited words are true of it.
#[test]
fn the_declension_check_accepts_a_fixture_silent_on_removal() {
    happenstance_testkit::__private::assert_declensions_are_stated(async || {
        SilentRemoval::default()
    });
}
