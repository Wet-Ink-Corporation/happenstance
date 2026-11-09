//! Every restore rule against the wrong peers it names, each pinned to the
//! exact set of rules that report it.
//!
//! A rule that no peer can fail is decorative. Each wrong peer here is one
//! small, hand-written peer over its own log with one plausible mistake made
//! in how it restores, and each test runs the **whole** enumeration against it
//! and asserts the exact set of rules that report it. The faithful peer is the
//! control: the same plumbing with no mistake passes every rule, so a red
//! result below is the mistake's and not the plumbing's.
//!
//! A hand-written peer rather than the in-memory oracle with a switch
//! (TST-09), because the oracle cannot make the central mistake at all: it
//! refuses a claim on its own identity it does not hold (SY-38), so a restore
//! that keeps the identity cannot be built from its public API.

#![cfg(not(target_arch = "wasm32"))]

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, PeerLimits, Pulled, PushBatch, ReplicatedEvent,
    SyncPeer, Waiting,
};
use happenstance_sync_testkit::{RestoreFixture, Violation, restore_rules};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a [`DevicePeer`] makes when it is restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control. The log goes back to the backup and the store
    /// re-mints its identity, as ADR-0086's procedure says.
    Faithful,
    /// VT-6's `Rejects:` line: the identity is keyed on the device, so it
    /// survives the restore, and positions numbered past the backup are
    /// numbered again. The negative control: the faithful restore without
    /// its re-mint.
    KeysIdentityOnTheDevice,
    /// The re-mint is taken and the storage is not put back: a restore that
    /// restores nothing, which a rule judging only reissue would pass.
    RestoresNothing,
    /// The store keeps positions and stamps its **current** incarnation on
    /// every event it reads back, so the re-mint rewrites the identity of
    /// every event the restore brought back (VT-5), and a peer that pulled
    /// them takes them again as new facts.
    StampsTheIncarnationOnRead,
}

/// The time every event of this peer is recorded at. Not a subject here.
const RECORDED: RecordedAt = RecordedAt::from_millis(1_800_000_000_000);

/// One row of a [`DevicePeer`]'s log: what its writer was handed, and the
/// identity it minted then.
#[derive(Debug, Clone)]
struct Row {
    id: EventId,
    event: Event,
}

/// A store that issues identities of its own, and backs up and restores.
///
/// Positions are dense from 1 and taken from the log's length, so a restore
/// that shortens the log hands the next write a position already given away,
/// as rewinding a sequence does. `Cell` and `RefCell` because the peer port
/// takes `&self` and the fixture runs on one thread under `block_on`; no
/// borrow outlives the statement that takes it.
#[derive(Debug)]
struct Device {
    mistake: Mistake,
    incarnation: Cell<u8>,
    log: RefCell<Vec<Row>>,
    backup: RefCell<Option<Vec<Row>>>,
}

impl Device {
    fn identity(&self) -> StoreId {
        StoreId::from_bytes([self.incarnation.get(); 16])
    }

    /// The log as a pull reads it, with the identity each row is read back
    /// under.
    fn read_back(&self) -> Vec<ReplicatedEvent> {
        self.log
            .borrow()
            .iter()
            .map(|row| {
                let id = if self.mistake == Mistake::StampsTheIncarnationOnRead {
                    EventId::new(self.identity(), row.id.position())
                } else {
                    row.id
                };
                ReplicatedEvent::new(id, RECORDED, row.event.clone())
            })
            .collect()
    }

    fn append(&self, events: &[Event]) {
        let mut log = self.log.borrow_mut();
        for event in events {
            let position = u64::try_from(log.len() + 1)
                .ok()
                .and_then(SequencePosition::new)
                .expect("a test log is short and its next position is not zero");
            log.push(Row {
                id: EventId::new(self.identity(), position),
                event: event.clone(),
            });
        }
    }

    fn back_up(&self) {
        let rows = self.log.borrow().clone();
        self.backup.replace(Some(rows));
    }

    fn restore(&self) -> Result<(), Violation> {
        let rows = self
            .backup
            .take()
            .ok_or_else(|| Violation::new("VT-6", "restored before any backup"))?;
        if self.mistake != Mistake::RestoresNothing {
            self.log.replace(rows);
        }
        if self.mistake != Mistake::KeysIdentityOnTheDevice {
            self.incarnation.set(self.incarnation.get() + 1);
        }
        Ok(())
    }
}

/// One handle onto a [`Device`].
#[derive(Debug, Clone)]
struct DevicePeer(Rc<Device>);

/// The one refusal a [`DevicePeer`] makes: it takes no push, because no
/// restore rule pushes.
#[derive(Debug)]
struct TakesNoPush;

impl core::fmt::Display for TakesNoPush {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("this test peer takes no push")
    }
}

impl core::error::Error for TakesNoPush {}

impl Classify for TakesNoPush {
    fn class(&self) -> FailureClass {
        FailureClass::Stop
    }
}

impl SyncPeer for DevicePeer {
    type Error = TakesNoPush;
    type Resume = usize;

    /// The whole log past `from` rows, one group per event, in one page.
    async fn pull(&self, from: Option<&usize>) -> Result<Pulled<usize>, Self::Error> {
        let events = self.0.read_back();
        let skip = from.copied().unwrap_or(0);
        let resume = events.len();
        let groups = events
            .into_iter()
            .skip(skip)
            .map(|event| EventGroup::new(None, vec![event]))
            .collect();
        Ok(Pulled::new(PushBatch::new(groups), resume).with_waiting(Waiting::Nothing))
    }

    async fn push(&self, _batch: &PushBatch) -> Result<Ack, Self::Error> {
        Err(TakesNoPush)
    }

    fn limits(&self) -> PeerLimits {
        PeerLimits::minimum()
    }
}

/// A fixture over one fresh [`Device`] that makes `mistake`.
#[derive(Debug)]
struct DeviceFixture(Rc<Device>);

impl DeviceFixture {
    fn new(mistake: Mistake) -> Self {
        Self(Rc::new(Device {
            mistake,
            incarnation: Cell::new(1),
            log: RefCell::new(Vec::new()),
            backup: RefCell::new(None),
        }))
    }
}

impl RestoreFixture for DeviceFixture {
    type Peer = DevicePeer;

    async fn connect(&self) -> Self::Peer {
        DevicePeer(Rc::clone(&self.0))
    }

    async fn append_local(&self, events: &[Event]) -> Result<(), Violation> {
        self.0.append(events);
        Ok(())
    }

    async fn back_up(&self) -> Result<(), Violation> {
        self.0.back_up();
        Ok(())
    }

    async fn restore(&self) -> Result<(), Violation> {
        self.0.restore()
    }
}

/// Every rule in the enumeration against a fresh fixture making `mistake`,
/// by name, each verdict kept whole so a test can read why it failed.
///
/// Driven by `for_each_restore_rule!`, so a rule added to the family is run
/// against every mutant here without an edit.
fn verdicts(mistake: Mistake) -> Vec<(&'static str, Result<RuleOutcome, Violation>)> {
    let open = async || DeviceFixture::new(mistake);
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$(
                (stringify!($name), block_on(restore_rules::$name(open))),
            )*]
        };
    }
    happenstance_sync_testkit::for_each_restore_rule!(run_each)
}

/// Exactly the rules in `failing` report VT-6, each with a message carrying
/// its `because`, and every other rule runs and passes.
fn fails_exactly(mistake: Mistake, failing: &[(&str, &str)]) {
    let all = verdicts(mistake);
    for (rule, _) in failing {
        assert!(
            all.iter().any(|(name, _)| name == rule),
            "`{rule}` is not in the enumeration"
        );
    }
    for (name, verdict) in all {
        match failing.iter().find(|(rule, _)| *rule == name) {
            Some((_, because)) => {
                let violation = verdict.expect_err("the rule must reject its mutant");
                assert_eq!(violation.clause(), "VT-6", "`{name}`: {violation}");
                assert!(
                    violation.message().contains(because),
                    "`{name}` failed {mistake:?} for another reason: {violation}"
                );
            }
            None => assert_eq!(
                verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` against {mistake:?}"
            ),
        }
    }
}

#[test]
fn the_faithful_peer_passes_every_rule() {
    fails_exactly(Mistake::Faithful, &[]);
}

#[test]
fn restored_peer_does_not_reissue_identities_fails_a_peer_that_keys_its_identity_on_the_device() {
    fails_exactly(
        Mistake::KeysIdentityOnTheDevice,
        &[(
            "restored_peer_does_not_reissue_identities",
            "reissued an identity it had already given away",
        )],
    );
}

#[test]
fn restored_peer_does_not_reissue_identities_fails_a_restore_that_restores_nothing() {
    fails_exactly(
        Mistake::RestoresNothing,
        &[(
            "restored_peer_does_not_reissue_identities",
            "did not put the storage back",
        )],
    );
}

#[test]
fn restored_peer_does_not_reissue_identities_fails_a_peer_that_stamps_its_incarnation_on_read() {
    fails_exactly(
        Mistake::StampsTheIncarnationOnRead,
        &[(
            "restored_peer_does_not_reissue_identities",
            "did not keep the identities it restored",
        )],
    );
}
