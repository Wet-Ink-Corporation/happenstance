//! Every model rule, against the wrong peers it names.
//!
//! `wrong_peers.rs`'s shape, one family over: each mutant is the reference peer
//! with one plausible mistake, and each test pins the verdict of **every** model
//! rule against it — the clause the mistake breaks where it breaks one, and
//! `Ok` everywhere else. The table below is the plan's, and a cell that
//! disagrees is a wrong oracle or a wrong mutant, never a cell to edit.
//!
//! | Mutant | Union | ExactlyOnce | ArrivalOrder |
//! |---|---|---|---|
//! | `Faithful` | Ok | Ok | Ok |
//! | `Quarantines` | SY-4 | Ok | Ok |
//! | `DedupesByPositionPrefix` | SY-1 | SY-11 | Ok |
//! | `DedupesPerBatch` | Ok | SY-11 | Ok |
//! | `ResortsByEventId` | Ok | Ok | SY-25 |
//! | `ResumeSkips` | SY-24 | Ok | Ok |

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use happenstance_core::{EventId, StoreId};
use happenstance_sync::{
    Ack, EventGroup, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch,
    ReplicatedEvent, SendSyncPeer, SyncPeer, Watermark,
};
use happenstance_sync_testkit::sync_model::rules;
use happenstance_sync_testkit::{RefusalsOf, SyncFixture, Violation, oversized_push_refusal};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a [`MutantPeer`] makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TopologyMutant {
    /// None: the control.
    Faithful,
    /// SY-4's wrong implementation: a group that travels under a guard is held
    /// back for review. It is acknowledged as appended, so the pusher moves
    /// on, and never enters the log, so it is never handed back.
    Quarantines,
    /// Dedupe by "at or below the highest position seen from that origin". An
    /// origin's lower position that arrives after its higher one — a delayed
    /// page — is called held and dropped.
    DedupesByPositionPrefix,
    /// SF-03: dedupe per batch. A batch holding anything new is applied whole,
    /// so a page half of which the peer already holds appends that half again.
    DedupesPerBatch,
    /// SY-25's wrong implementation: hand back each page re-sorted by the
    /// identity of each group's first event.
    ResortsByEventId,
    /// An off-by-one at the tail of the log: a pull that returns nothing hands
    /// back a token one group past the end, so the next group the peer takes
    /// in is skipped by every lane that had caught up with it. A walk from the
    /// start never resumes from a caught-up token, so the peer's own log reads
    /// whole; only forwarding loses the group.
    ResumeSkips,
}

/// The mutant's resume token: the inner peer's, and how many groups past it
/// the next pull drops unseen. Zero for every mutant but
/// [`TopologyMutant::ResumeSkips`], so the others resume exactly as the inner
/// peer does.
#[derive(Debug, Clone, Copy, Default)]
struct MutantResume {
    inner: MemoryResume,
    skip: usize,
}

/// The mutant's own record of what it has accepted. Shared by every handle on
/// one fixture, as the inner peer's log is.
#[derive(Debug, Default)]
struct Side {
    accepted: BTreeSet<EventId>,
    high: Watermark,
}

#[derive(Debug)]
struct MutantPeer {
    inner: Arc<MemorySyncPeer>,
    mistake: TopologyMutant,
    /// `Mutex` because the peer is `SendSyncPeer` and `push` takes `&self`;
    /// the guard is never held across an `.await`.
    side: Arc<Mutex<Side>>,
}

impl MutantPeer {
    /// Decides, records and writes one batch, synchronously: the lock is taken
    /// and released inside, and `MemorySyncPeer::seed` does not await.
    fn take(&self, batch: &PushBatch) -> Result<Ack, MemoryPeerError> {
        let mut side = self.side.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        let mut ack = Ack::default();
        if self.mistake == TopologyMutant::DedupesPerBatch {
            let anything_new = batch
                .events()
                .any(|event| !side.accepted.contains(&event.id));
            for group in &batch.groups {
                for event in &group.events {
                    side.accepted.insert(event.id);
                    if anything_new {
                        ack.appended += 1;
                    } else {
                        ack.skipped += 1;
                    }
                }
                if anything_new {
                    self.inner.seed(group.clone())?;
                }
            }
            return Ok(ack);
        }
        for group in &batch.groups {
            let mut fresh: Vec<ReplicatedEvent> = Vec::new();
            for event in &group.events {
                let new = if self.mistake == TopologyMutant::DedupesByPositionPrefix {
                    let (origin, position) = (event.id.store(), event.id.position());
                    let new = side.high.get(origin) < Some(position);
                    side.high.advance(origin, position);
                    new
                } else {
                    side.accepted.insert(event.id)
                };
                if new {
                    ack.appended += 1;
                    fresh.push(event.clone());
                } else {
                    ack.skipped += 1;
                }
            }
            let quarantined = self.mistake == TopologyMutant::Quarantines && group.guard.is_some();
            if !fresh.is_empty() && !quarantined {
                self.inner
                    .seed(EventGroup::new(group.guard.clone(), fresh))?;
            }
        }
        Ok(ack)
    }
}

impl SendSyncPeer for MutantPeer {
    type Error = MemoryPeerError;
    type Resume = MutantResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        // `MemoryResume::default()` is the start, as `None` is.
        let from = from.copied().unwrap_or_default();
        let pulled = SendSyncPeer::pull(&*self.inner, Some(&from.inner)).await?;
        let mut groups: Vec<EventGroup> =
            pulled.batch.groups.into_iter().skip(from.skip).collect();
        if self.mistake == TopologyMutant::ResortsByEventId {
            groups.sort_by_key(|group| group.events.first().map(|event| event.id));
        }
        let resume = MutantResume {
            inner: pulled.resume,
            skip: usize::from(self.mistake == TopologyMutant::ResumeSkips && groups.is_empty()),
        };
        Ok(Pulled::new(PushBatch::new(groups), resume).with_waiting(pulled.waiting))
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.take(batch)
    }

    fn limits(&self) -> PeerLimits {
        SendSyncPeer::limits(&*self.inner)
    }
}

/// One isolated mutant peer per fixture, so one per node.
#[derive(Debug)]
struct MutantFixture {
    inner: Arc<MemorySyncPeer>,
    mistake: TopologyMutant,
    side: Arc<Mutex<Side>>,
}

impl MutantFixture {
    fn new(mistake: TopologyMutant) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x79; 16]))),
            mistake,
            side: Arc::default(),
        }
    }
}

impl SyncFixture for MutantFixture {
    type Peer = MutantPeer;

    async fn connect(&self) -> Self::Peer {
        MutantPeer {
            inner: Arc::clone(&self.inner),
            mistake: self.mistake,
            side: Arc::clone(&self.side),
        }
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

type Verdict = Result<RuleOutcome, Violation>;

/// Every model rule against `mistake`, in enumeration order.
fn verdicts(mistake: TopologyMutant) -> Vec<(&'static str, Verdict)> {
    let open = async || MutantFixture::new(mistake);
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$((stringify!($name), block_on(rules::$name(open)))),*]
        };
    }
    happenstance_sync_testkit::for_each_sync_model_rule!(run_each)
}

/// The verdict of `rule`, reduced to its clause.
fn clause_of<'a>(all: &'a [(&'static str, Verdict)], rule: &str) -> Result<RuleOutcome, &'a str> {
    let (_, verdict) = all
        .iter()
        .find(|(name, _)| *name == rule)
        .unwrap_or_else(|| panic!("`{rule}` is not in the enumeration"));
    match verdict {
        Ok(outcome) => Ok(*outcome),
        Err(violation) => Err(violation.clause()),
    }
}

fn message_of<'a>(all: &'a [(&'static str, Verdict)], rule: &str) -> &'a str {
    all.iter()
        .find(|(name, _)| *name == rule)
        .and_then(|(_, verdict)| verdict.as_ref().err())
        .map_or("", Violation::message)
}

const UNION: &str = "transitive_convergence_over_a_partial_mesh";
const EXACTLY_ONCE: &str = "redelivery_of_an_accepted_group_is_a_no_op";
const ARRIVAL_ORDER: &str = "forwarding_order_is_arrival_order";

/// Pins one row of the table: `expected` is the clause each of the three rules
/// reports, or `None` for a pass.
fn pins(mistake: TopologyMutant, expected: [Option<&str>; 3]) -> Vec<(&'static str, Verdict)> {
    let all = verdicts(mistake);
    assert_eq!(all.len(), 3, "the enumeration changed: {all:?}");
    for (rule, want) in [UNION, EXACTLY_ONCE, ARRIVAL_ORDER]
        .into_iter()
        .zip(expected)
    {
        let got = clause_of(&all, rule);
        match want {
            None => assert_eq!(got, Ok(RuleOutcome::Ran), "{mistake:?}: `{rule}`"),
            Some(clause) => assert_eq!(got, Err(clause), "{mistake:?}: `{rule}`"),
        }
    }
    all
}

#[test]
fn the_faithful_wrapper_passes_every_model_rule() {
    pins(TopologyMutant::Faithful, [None, None, None]);
}

/// Caught only by a generated case — the regressions write nothing guarded —
/// so this is also the end-to-end check that a failure prints its seed.
#[test]
fn the_model_family_fails_a_quarantining_peer() {
    let all = pins(TopologyMutant::Quarantines, [Some("SY-4"), None, None]);
    let message = message_of(&all, UNION);
    assert!(message.contains("seed 0x"), "{message}");
}

#[test]
fn the_model_family_fails_a_peer_that_dedupes_by_position_prefix() {
    let all = pins(
        TopologyMutant::DedupesByPositionPrefix,
        [Some("SY-1"), Some("SY-11"), None],
    );
    let message = message_of(&all, UNION);
    assert!(
        message.contains("regression scenario `delayed_page_lands_after_a_later_one`"),
        "{message}"
    );
}

#[test]
fn the_model_family_fails_a_peer_that_dedupes_per_batch() {
    pins(TopologyMutant::DedupesPerBatch, [None, Some("SY-11"), None]);
}

#[test]
fn the_model_family_fails_a_peer_that_resorts_what_it_forwards() {
    pins(
        TopologyMutant::ResortsByEventId,
        [None, None, Some("SY-25")],
    );
}

/// The one mutant that reaches SY-24's `NotForwarded` attribution through the
/// simulator rather than a hand-built trace: the skipped group is never pushed
/// along that lane, so the node beyond it was never offered the event.
#[test]
fn the_model_family_fails_a_peer_whose_resume_skips_a_group() {
    let all = pins(TopologyMutant::ResumeSkips, [Some("SY-24"), None, None]);
    let message = message_of(&all, UNION);
    assert!(message.contains("nothing forwarded it there"), "{message}");
}

/// The mutant table's premise: each fixture is its own peer, so a rule that
/// opened one fixture and connected to it per node would be testing one log.
#[test]
fn each_fixture_is_its_own_peer() {
    let (one, two) = (
        MutantFixture::new(TopologyMutant::Faithful),
        MutantFixture::new(TopologyMutant::Faithful),
    );
    assert!(!Arc::ptr_eq(&one.inner, &two.inner));
    let peer = block_on(one.connect());
    let pulled = block_on(SyncPeer::pull(&peer, None)).expect("an empty peer pulls");
    assert!(pulled.batch.groups.is_empty());
}
