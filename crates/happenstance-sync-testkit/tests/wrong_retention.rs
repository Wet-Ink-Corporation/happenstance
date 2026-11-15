//! SY-32's rule against the peers that compact and say nothing, or say the
//! wrong thing (#158).
//!
//! The case is the vessel away 120 days against a hub that keeps 90:
//! `retention_gap_is_reported_not_silent` stages a stale token and a recent
//! one, has the fixture compact, and holds the peer to a report, a refusal of
//! class `Reseed`, and a resumable recent token. [`CompactingPeer`] owns a
//! small log and compacts it as a retention window would; it carries one
//! [`Defect`], and is otherwise correct. Each defect fails the rule with SY-32
//! and a message naming it, and [`Defect::Absent`] passes, so the rule is not
//! strict by failing every peer.

#![cfg(not(target_arch = "wasm32"))]

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;

use happenstance_core::{EventId, RecordedAt};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, PeerLimits, Pulled, PushBatch, SyncPeer,
};
use happenstance_sync_testkit::{RefusalsOf, SyncFixture, Violation, rules};
use happenstance_testkit::{Capability, RuleOutcome, block_on};

/// The one thing a [`CompactingPeer`] gets wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Defect {
    /// None: the control.
    Absent,
    /// Drains the prefix without keeping an offset, so a stale index lands
    /// on a later group and is answered with a strict subset.
    ShiftingCompaction,
    /// Refuses correctly and declares no floor.
    MuteFloor,
    /// Declares the floor, and answers a stale token from it: the peer that
    /// converges silently on a subset.
    ClampToFloor,
    /// Refuses a stale token with class `Transient`, which a runner retries
    /// for ever.
    TransientGap,
    /// Refuses every token once it has compacted, resumable or not.
    OverRefusal,
    /// Serves a pull from the start from its floor: an edge that lost its
    /// token converges on the retained subset in silence.
    StartIsFine,
}

/// A refusal, stating the class the peer chose for it.
#[derive(Debug)]
struct Refused(FailureClass);

impl core::fmt::Display for Refused {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "the peer refused the pull as {:?}", self.0)
    }
}

impl core::error::Error for Refused {}

impl Classify for Refused {
    fn class(&self) -> FailureClass {
        self.0
    }
}

/// The log behind every handle of one fixture.
#[derive(Debug, Default)]
struct Log {
    groups: Vec<EventGroup>,
    held: BTreeSet<EventId>,
    /// Groups compaction dropped from the front.
    dropped: usize,
    /// The newest event compaction dropped.
    floor: Option<EventId>,
}

/// A peer whose resume token is an absolute group index.
#[derive(Debug)]
struct CompactingPeer {
    log: Rc<RefCell<Log>>,
    defect: Defect,
}

impl CompactingPeer {
    /// The absolute index a pull from `from` starts at, or the refusal.
    fn start(&self, log: &Log, from: Option<usize>) -> Result<usize, Refused> {
        let compacted = log.dropped > 0;
        match (from, self.defect) {
            // No offset: the token is read as an index into what is left.
            (from, Defect::ShiftingCompaction) => Ok(from.unwrap_or(0) + log.dropped),
            (None, Defect::StartIsFine) => Ok(log.dropped),
            (None, _) if compacted => Err(Refused(FailureClass::Reseed)),
            (None, _) => Ok(0),
            (Some(token), Defect::ClampToFloor) => Ok(token.max(log.dropped)),
            (Some(token), defect) => {
                let refused = token < log.dropped || (defect == Defect::OverRefusal && compacted);
                match (refused, defect) {
                    (false, _) => Ok(token),
                    (true, Defect::TransientGap) => Err(Refused(FailureClass::Transient)),
                    (true, _) => Err(Refused(FailureClass::Reseed)),
                }
            }
        }
    }
}

impl SyncPeer for CompactingPeer {
    type Error = Refused;
    type Resume = usize;

    /// Everything past the start, in one batch: the rule's log is small.
    async fn pull(&self, from: Option<&usize>) -> Result<Pulled<usize>, Refused> {
        let log = self.log.borrow();
        let start = self.start(&log, from.copied())?;
        let groups: Vec<EventGroup> = log
            .groups
            .iter()
            .skip(start - log.dropped)
            .cloned()
            .collect();
        let next = match self.defect {
            Defect::ShiftingCompaction => start - log.dropped + groups.len(),
            _ => start + groups.len(),
        };
        Ok(Pulled::new(PushBatch::new(groups), next))
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Refused> {
        let mut log = self.log.borrow_mut();
        let mut ack = Ack::default();
        for group in &batch.groups {
            let fresh: Vec<_> = group
                .events
                .iter()
                .filter(|event| log.held.insert(event.id))
                .cloned()
                .collect();
            ack.appended += u64::try_from(fresh.len()).expect("a small batch");
            ack.skipped += u64::try_from(group.events.len() - fresh.len()).expect("small");
            if !fresh.is_empty() {
                log.groups.push(EventGroup::new(group.guard.clone(), fresh));
            }
        }
        Ok(ack)
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = PeerLimits::minimum();
        if self.defect != Defect::MuteFloor {
            limits.retention_floor = self.log.borrow().floor;
        }
        limits
    }
}

#[derive(Debug)]
struct CompactingFixture {
    log: Rc<RefCell<Log>>,
    defect: Defect,
}

impl SyncFixture for CompactingFixture {
    type Peer = CompactingPeer;

    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> CompactingPeer {
        CompactingPeer {
            log: Rc::clone(&self.log),
            defect: self.defect,
        }
    }

    async fn refusals(&self, _peer: &CompactingPeer) -> RefusalsOf<CompactingPeer> {
        Ok(Vec::new())
    }

    /// A prefix, by recorded time, as the reference peer compacts.
    async fn compact_recorded_before(
        &self,
        peer: &CompactingPeer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        let mut log = peer.log.borrow_mut();
        let expired = log
            .groups
            .iter()
            .take_while(|group| {
                group
                    .events
                    .iter()
                    .all(|event| event.recorded_at.as_millis() < cutoff.as_millis())
            })
            .count();
        let dropped: Vec<EventGroup> = log.groups.drain(..expired).collect();
        if let Some(newest) = dropped.iter().rev().find_map(|group| group.events.last()) {
            log.floor = Some(newest.id);
        }
        for event in dropped.iter().flat_map(|group| &group.events) {
            log.held.remove(&event.id);
        }
        log.dropped += expired;
        Ok(())
    }
}

fn verdict(defect: Defect) -> Result<RuleOutcome, Violation> {
    block_on(rules::retention_gap_is_reported_not_silent(async || {
        CompactingFixture {
            log: Rc::new(RefCell::new(Log::default())),
            defect,
        }
    }))
}

/// The defect fails the rule under SY-32, with a message containing `names`.
fn fails(defect: Defect, names: &str) {
    let violation = verdict(defect).expect_err("the rule must fail this peer");
    assert_eq!(violation.clause(), "SY-32", "{violation}");
    assert!(
        violation.to_string().contains(names),
        "{defect:?}: expected the message to name {names:?}: {violation}"
    );
}

/// The same peer behind a fixture that says nothing about compaction.
#[derive(Debug)]
struct Silent(CompactingFixture);

impl SyncFixture for Silent {
    type Peer = CompactingPeer;

    async fn connect(&self) -> CompactingPeer {
        self.0.connect().await
    }

    async fn refusals(&self, peer: &CompactingPeer) -> RefusalsOf<CompactingPeer> {
        self.0.refusals(peer).await
    }
}

/// Rejects a rule that compacts without checking the capability, or that
/// passes a fixture it could not run against.
#[test]
fn the_rule_skips_a_fixture_that_declines_compaction_with_its_reason() {
    let skipped = block_on(rules::retention_gap_is_reported_not_silent(async || {
        Silent(CompactingFixture {
            log: Rc::new(RefCell::new(Log::default())),
            defect: Defect::ShiftingCompaction,
        })
    }));

    assert_eq!(
        skipped,
        Ok(RuleOutcome::Skipped {
            capability: "COMPACTION",
            reason: happenstance_sync_testkit::UNSTATED_COMPACTION,
        })
    );
}

#[test]
fn the_rule_passes_a_peer_that_compacts_and_reports_it() {
    assert_eq!(verdict(Defect::Absent), Ok(RuleOutcome::Ran));
}

#[test]
fn the_rule_fails_a_peer_whose_stale_index_shifts_onto_later_groups() {
    fails(Defect::ShiftingCompaction, "in silence");
}

#[test]
fn the_rule_fails_a_peer_that_refuses_and_reports_no_floor() {
    fails(Defect::MuteFloor, "no retention floor");
}

#[test]
fn the_rule_fails_a_peer_that_answers_a_stale_token_from_its_floor() {
    fails(Defect::ClampToFloor, "in silence");
}

#[test]
fn the_rule_fails_a_peer_that_calls_a_gap_transient() {
    fails(Defect::TransientGap, "Transient");
}

#[test]
fn the_rule_fails_a_peer_that_refuses_a_token_above_its_floor() {
    fails(Defect::OverRefusal, "above the floor");
}

#[test]
fn the_rule_fails_a_peer_that_serves_the_start_from_its_floor() {
    fails(
        Defect::StartIsFine,
        "a pull from the start, below its floor",
    );
}
