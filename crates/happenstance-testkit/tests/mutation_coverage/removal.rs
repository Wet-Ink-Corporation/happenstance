//! `Fixture::REMOVAL`'s four wrong implementations, which
//! `removal_outside_the_port_removes_exactly_the_named_events` rejects (#147).
//!
//! A module of its own, for `metadata_ceilings`' reason: `mutants.rs` is
//! already the longest file in this binary. Every store here is `LogStore`
//! over one dense [`Log`], which is completely correct, so the only thing wrong
//! with each fixture is what its removal does. That is CF-39's shape one
//! capability over: the defect is in the fixture's claim, not in the store.
//!
//! Each one declares `REMOVAL` and declines `REOPEN`, and each answers the
//! model family exactly as the correct store does, because the model never
//! removes.

use core::cell::RefCell;
use core::marker::PhantomData;
use std::rc::Rc;

use happenstance_core::{Query, ReadOptions, SequencePosition};
use happenstance_testkit::{Capability, Fixture, Removed};

use crate::correct::{Log, LogStore, dense};
use crate::harness::Subject;

/// What `REOPEN` says for every fixture here.
const NO_DURABLE_MEDIUM: &str = "a Vec behind an Rc, with no durable medium to reopen over — this \
     instrument's axis is what a declared removal capability actually does";

/// A fixture that declares `REMOVAL` supported and **does not override**
/// `remove_outside_port`.
///
/// The author who declares first and wires later. The provided body answers
/// `Removed::Nothing`, and the rule's answer check fails on that before it
/// reads anything back, with a message naming the method.
#[derive(Debug)]
pub(crate) struct ForgottenRemovalFixture(Rc<RefCell<Log>>);

impl Fixture for ForgottenRemovalFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_DURABLE_MEDIUM);
    // THE DEFECT: the claim, with `remove_outside_port` left to the trait's
    // provided body, which removes nothing.
    const REMOVAL: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.0)
    }
}

impl Subject for ForgottenRemovalFixture {
    const NAME: &'static str = "ForgottenRemovalFixture";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Log::new(dense))))
    }
}

/// One way of getting a removal wrong, as a step a [`WrongRemovalFixture`]
/// takes in place of the correct one.
///
/// A trait of one step, on `mutants::Defect`'s pattern, so that the three
/// fixtures below share their connect, their capabilities and their opener,
/// and differ in exactly the function that is wrong.
pub(crate) trait WrongRemoval: 'static {
    /// The registry key.
    const NAME: &'static str;

    /// Does to `log` what this defect does when asked to remove `positions`,
    /// and answers as an override would.
    fn remove(log: &mut Log, positions: &[SequencePosition]) -> Removed;
}

/// A fixture that declares `REMOVAL` and removes the wrong way, as `R` says.
#[derive(Debug)]
pub(crate) struct WrongRemovalFixture<R> {
    log: Rc<RefCell<Log>>,
    removal: PhantomData<R>,
}

impl<R: WrongRemoval> Fixture for WrongRemovalFixture<R> {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(NO_DURABLE_MEDIUM);
    // THE DEFECT, first half: the claim.
    const REMOVAL: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.log)
    }

    // THE DEFECT, second half: the step `R` gets wrong, applied to the log
    // every handle shares, so it reaches the handle the rule already holds.
    async fn remove_outside_port(&self, positions: &[SequencePosition]) -> Removed {
        R::remove(&mut self.log.borrow_mut(), positions)
    }
}

impl<R: WrongRemoval> Subject for WrongRemovalFixture<R> {
    const NAME: &'static str = R::NAME;

    fn open() -> Self {
        Self {
            log: Rc::new(RefCell::new(Log::new(dense))),
            removal: PhantomData,
        }
    }
}

/// An override that answers `Removed::Deleted` and deletes nothing.
///
/// `NoopFaultFixture`'s author, one capability over: the override is present,
/// so the answer check passes, and empty, so every event is still there.
#[derive(Debug)]
pub(crate) struct NoopRemoval;

impl WrongRemoval for NoopRemoval {
    const NAME: &'static str = "NoopRemovalFixture";

    fn remove(_log: &mut Log, _positions: &[SequencePosition]) -> Removed {
        Removed::Deleted
    }
}

/// An override that deletes everything from the lowest named position up.
///
/// `DELETE … WHERE position >= ?`, which is the shape of
/// `CloudflareEventStore::discard_from`, the workspace's one existing raw
/// delete. Copying it for a removal of named positions takes every survivor
/// above the lowest one with it.
#[derive(Debug)]
pub(crate) struct OverRemoving;

impl WrongRemoval for OverRemoving {
    const NAME: &'static str = "OverRemovingFixture";

    fn remove(log: &mut Log, positions: &[SequencePosition]) -> Removed {
        let Some(from) = positions.iter().min().copied() else {
            return Removed::Deleted;
        };
        let doomed: Vec<SequencePosition> = log
            .select(&Query::all(), ReadOptions::new())
            .iter()
            .map(|event| event.position)
            .filter(|position| *position >= from)
            .collect();
        log.remove(&doomed);
        Removed::Deleted
    }
}

/// An override that rebuilds the log from its survivors through `append`.
///
/// The obvious removal on a store with no delete: read everything, drop the
/// named events, write the rest into a fresh log. Every survivor above the
/// first hole gets a new position and a new identity, which is ES-38's
/// *Rejects*: a store rebuilt from its survivors is a different store.
#[derive(Debug)]
pub(crate) struct Renumbering;

impl WrongRemoval for Renumbering {
    const NAME: &'static str = "RenumberingFixture";

    fn remove(log: &mut Log, positions: &[SequencePosition]) -> Removed {
        let mut rebuilt = Log::new(dense);
        for survivor in log
            .select(&Query::all(), ReadOptions::new())
            .into_iter()
            .filter(|event| !positions.contains(&event.position))
        {
            let replayed = rebuilt.append(&[survivor.event], None);
            assert!(
                replayed.is_ok(),
                "re-appending a survivor unconditionally lands: {replayed:?}"
            );
        }
        *log = rebuilt;
        Removed::Deleted
    }
}
