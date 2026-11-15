//! The adjudication hook on the shipped runner (#450): SY-7's assignment and
//! SY-10's per-direction rules, at both of an edge's ingest sites.
//!
//! [`SeatRule`] is the merge rule the tests mount: the first claim on a seat
//! to land wins, and every later claim that is not yet reversed earns a
//! reversal naming it. It decides from its own read handle onto the receiving
//! store and from `earlier`, and writes nothing (SY-2). Re-judging what a store
//! already holds answers nothing new, so the only way a runner makes it write
//! twice is to ask it twice about something it should not have asked about.
//!
//! The wrong runners these tests' rules reject are in `wrong_runners.rs`.

use core::cell::Cell;
use core::num::NonZeroUsize;
use core::slice;

use happenstance_core::{
    Event, EventId, EventStore, EventType, MemoryStoreError, Query, QueryItem, ReadOptions,
    RecordedAt, SequencePosition, SequencedEvent, StoreId, Tag, Tags, collect,
};
use happenstance_sync::runner::{Adjudicate, Edge, EdgeState, Phase, StepError, StopCause};
use happenstance_sync::wire::{Envelope, Route};
use happenstance_sync::{
    Classify, EventGroup, FailureClass, IngestGroup, IngestStore, MemoryIngestStore, PeerLimits,
    PushBatch, ReplicatedEvent, StorePeerError, StoreSyncPeer, SyncPeer, WriteGroupStore,
    serve_sync_adjudicated,
};

use super::probe::Shared;

pub(crate) const HUB: StoreId = StoreId::from_bytes([0x4C; 16]);
pub(crate) const SPOKE: StoreId = StoreId::from_bytes([0x5C; 16]);
/// Origins whose claims reach a store by ingest: spokes it never meets.
pub(crate) const ORIGIN_ONE: StoreId = StoreId::from_bytes([0x6C; 16]);
pub(crate) const ORIGIN_TWO: StoreId = StoreId::from_bytes([0x7C; 16]);

const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");

fn seat() -> Tags {
    [Tag::from_static("seat:s450")].into_iter().collect()
}

/// A claim on the seat, carrying `data` so each claim's bytes are its own.
pub(crate) fn claim(data: &[u8]) -> Event {
    Event::new(SEAT_CLAIMED, data.to_vec())
        .expect("a static event type is valid")
        .with_tags(seat())
}

/// The reversal of `claim`, naming it in its bytes.
fn reversal_of(claim: EventId) -> Event {
    Event::new(CLAIM_REVERSED, claim.to_string().into_bytes())
        .expect("a static event type is valid")
        .with_tags(seat())
}

/// `origin`'s claim as it travels: its first event.
pub(crate) fn foreign_claim(origin: StoreId, data: &[u8]) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        claim(data),
    )
}

/// `store` takes `claim` by ingest, unjudged, as one group.
pub(crate) async fn ingest_unjudged<S: IngestStore>(store: &S, claim: &ReplicatedEvent) {
    IngestStore::ingest(store, &[IngestGroup::new(slice::from_ref(claim), &[])])
        .await
        .map_err(|error| error.to_string())
        .expect("the claim lands");
}

/// The reversals in `log`, found by type.
pub(crate) fn reversals(log: &[SequencedEvent]) -> Vec<&SequencedEvent> {
    log.iter()
        .filter(|row| *row.event_type() == CLAIM_REVERSED)
        .collect()
}

/// The reversals in `log` that name `claim`, each under the identity it was
/// filed as.
pub(crate) fn reversals_of(log: &[SequencedEvent], claim: EventId) -> Vec<EventId> {
    let named = claim.to_string().into_bytes();
    reversals(log)
        .into_iter()
        .filter(|row| row.event.data() == named.as_slice())
        .map(|row| row.id)
        .collect()
}

/// How a [`SeatRule`] fails: its read of the store did.
#[derive(Debug, thiserror::Error)]
#[error("the seat rule could not read its store")]
pub(crate) struct SeatRuleError(#[source] MemoryStoreError);

impl Classify for SeatRuleError {
    fn class(&self) -> FailureClass {
        self.0.class()
    }
}

/// The seat family's merge rule: the first claim to land wins; every later
/// claim not yet reversed earns one reversal, naming it.
///
/// Landing order is the store's log, then `earlier`, then the group itself.
/// It counts how often it was asked, so a test can see a rule that was never
/// consulted.
#[derive(Debug)]
pub(crate) struct SeatRule<'a> {
    store: &'a MemoryIngestStore,
    pub(crate) asked: Cell<usize>,
}

impl<'a> SeatRule<'a> {
    pub(crate) const fn new(store: &'a MemoryIngestStore) -> Self {
        Self {
            store,
            asked: Cell::new(0),
        }
    }
}

impl Adjudicate for SeatRule<'_> {
    type Error = SeatRuleError;

    async fn compensation(
        &self,
        group: &EventGroup,
        earlier: &[EventGroup],
    ) -> Result<Vec<Event>, SeatRuleError> {
        self.asked.set(self.asked.get() + 1);
        let family = Query::from_item(
            QueryItem::new([SEAT_CLAIMED, CLAIM_REVERSED], seat()).expect("a valid query item"),
        );
        let held = collect(self.store.read(&family, ReadOptions::new()))
            .await
            .map_err(SeatRuleError)?;
        let landed: Vec<(EventId, &Event)> = held
            .iter()
            .map(|row| (row.id, &row.event))
            .chain(
                earlier
                    .iter()
                    .chain(slice::from_ref(group))
                    .flat_map(|group| group.events.iter())
                    .map(|event| (event.id, &event.event)),
            )
            .collect();
        let winner = landed
            .iter()
            .find(|(_, event)| *event.event_type() == SEAT_CLAIMED)
            .map(|(id, _)| *id);
        let reversed: Vec<&[u8]> = landed
            .iter()
            .filter(|(_, event)| *event.event_type() == CLAIM_REVERSED)
            .map(|(_, event)| event.data().as_ref())
            .collect();
        Ok(group
            .events
            .iter()
            .filter(|event| *event.event.event_type() == SEAT_CLAIMED)
            .filter(|event| Some(event.id) != winner)
            .filter(|event| !reversed.contains(&event.id.to_string().as_bytes()))
            .map(|event| reversal_of(event.id))
            .collect())
    }
}

/// The rule by reference, so a test that mounts it can still read its count.
impl Adjudicate for &SeatRule<'_> {
    type Error = SeatRuleError;

    async fn compensation(
        &self,
        group: &EventGroup,
        earlier: &[EventGroup],
    ) -> Result<Vec<Event>, SeatRuleError> {
        SeatRule::compensation(self, group, earlier).await
    }
}

/// A rule that always fails with the class it was built with.
#[derive(Debug)]
struct Failing(FailureClass);

#[derive(Debug, thiserror::Error)]
#[error("an injected {0:?} failure to judge")]
struct FailingError(FailureClass);

impl Classify for FailingError {
    fn class(&self) -> FailureClass {
        self.0
    }
}

impl Adjudicate for Failing {
    type Error = FailingError;

    async fn compensation(
        &self,
        _group: &EventGroup,
        _earlier: &[EventGroup],
    ) -> Result<Vec<Event>, FailingError> {
        Err(FailingError(self.0))
    }
}

/// A hub holding the first claim on the seat, from a spoke it never meets,
/// and a spoke holding a claim of its own, served as a store peer.
async fn hub_and_spoke() -> (MemoryIngestStore, Shared, EventId) {
    let hub = MemoryIngestStore::with_store_id(HUB);
    ingest_unjudged(&hub, &foreign_claim(ORIGIN_ONE, b"first")).await;
    let spoke = Shared::new(SPOKE);
    let at = EventStore::append(&*spoke.0, &[claim(b"the-spokes")], None)
        .await
        .expect("an in-memory append does not fail");
    (hub, spoke, EventId::new(SPOKE, at))
}

/// T-450-2. A hub that drives its own edge adjudicates what it pulls: the
/// spoke's claim lands with one reversal, minted under the hub's identity, in
/// the same write. The ADR-0107 gap, closed.
///
/// Rejects a runner that never asks its rule, and one that asks it and drops
/// the answer.
#[tokio::test]
async fn a_hub_driven_edge_adjudicates_what_it_pulls() {
    let (hub, spoke, spokes) = hub_and_spoke().await;
    let peer = StoreSyncPeer::new(spoke.clone(), spoke.clone());
    let mut edge = Edge::new().with_adjudicator(SeatRule::new(&hub));

    let stepped = edge.step(&peer, &hub).await.expect("an in-memory step");

    assert_eq!(stepped.ingested.appended, 1, "the spoke's claim");
    assert_eq!(stepped.ingested.compensated, 1, "and the hub's reversal");
    assert_eq!(
        reversals_of(&hub.snapshot(), spokes)
            .iter()
            .map(|id| id.store())
            .collect::<Vec<_>>(),
        vec![HUB],
        "one reversal of the spoke's claim, the hub's"
    );
    assert_eq!(
        reversals_of(&spoke.0.snapshot(), spokes).len(),
        1,
        "the same step's push carried the reversal to the spoke"
    );
}

/// T-450-3. SY-7's runner half on the shipped runner: the same stores, the
/// edge under [`Accept`](happenstance_sync::runner::Accept), land the
/// contested claim with no reversal anywhere.
///
/// Rejects a runner that adjudicates on an edge nobody made an adjudicator.
#[tokio::test]
async fn an_edge_under_accept_lands_a_contested_claim_unjudged() {
    let (hub, spoke, _) = hub_and_spoke().await;
    let peer = StoreSyncPeer::new(spoke.clone(), spoke.clone());
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &hub).await.expect("an in-memory step");

    assert_eq!(stepped.ingested.appended, 1, "the spoke's claim");
    assert_eq!(stepped.ingested.compensated, 0, "judged by nobody");
    assert!(reversals(&hub.snapshot()).is_empty(), "no reversal");
}

/// T-450-5. A rule that cannot decide fails the step in the adjudicate phase:
/// nothing in the batch lands, the pull token does not advance, and the edge
/// reacts to the rule's class as to any other failure (SY-36). A split
/// outside the push has no batch of the runner's to halve, so it parks.
///
/// Rejects a runner that lands the batch unjudged when its rule fails, and one
/// that advances the token past a batch it never ingested.
#[tokio::test]
async fn a_rule_that_cannot_decide_lands_nothing_and_is_reacted_to_by_its_class() {
    for (class, expected) in [
        (FailureClass::Transient, "running"),
        (FailureClass::Split, "parked"),
        (FailureClass::Park, "parked"),
        (FailureClass::Stop, "stopped"),
    ] {
        let (hub, spoke, _) = hub_and_spoke().await;
        let before = hub.snapshot();
        let peer = StoreSyncPeer::new(spoke.clone(), spoke.clone());
        let mut edge = Edge::new().with_adjudicator(Failing(class));

        let failed = edge.step(&peer, &hub).await;

        let Err(error) = failed else {
            panic!("a {class:?} rule failure fails the step: {failed:?}");
        };
        assert!(
            matches!(error, StepError::Adjudicate(FailingError(answered)) if answered == class),
            "{error:?}"
        );
        let failure = error.failure();
        assert_eq!(failure.phase, Phase::Adjudicate);
        assert_eq!(failure.class, class);
        assert_eq!(hub.snapshot(), before, "nothing landed under {class:?}");
        assert!(edge.resume_token().is_none(), "the token did not advance");
        let state = match edge.status().state {
            EdgeState::Running => "running",
            EdgeState::Parked(_) => "parked",
            EdgeState::Stopped(StopCause::Failure(_)) => "stopped",
            other => panic!("an unexpected state {other:?}"),
        };
        assert_eq!(state, expected, "under {class:?}");
    }
}

/// The far side's half: a store peer given a rule adjudicates what it is
/// pushed, in the same write as the pushed claim, under its own identity.
/// What a hub served behind `serve_sync_adjudicated` needs (SY-10).
///
/// Rejects a store peer that ingests a push unjudged whatever it was given.
#[tokio::test]
async fn a_store_peer_adjudicates_what_it_is_pushed() {
    let hub = Shared::new(HUB);
    ingest_unjudged(&hub, &foreign_claim(ORIGIN_ONE, b"first")).await;
    let rule = SeatRule::new(&hub.0);
    let peer = StoreSyncPeer::new(hub.clone(), hub.clone()).with_adjudicator(rule);
    let spoke = MemoryIngestStore::with_store_id(SPOKE);
    let at = EventStore::append(&spoke, &[claim(b"the-spokes")], None)
        .await
        .expect("an in-memory append does not fail");
    let spokes = EventId::new(SPOKE, at);

    Edge::new()
        .step(&peer, &spoke)
        .await
        .expect("an in-memory step");

    let page = WriteGroupStore::read_groups(&hub, None, NonZeroUsize::MIN.saturating_add(63))
        .await
        .expect("an in-memory read");
    let landed = page
        .groups
        .iter()
        .find(|group| group.events().iter().any(|event| event.id == spokes))
        .expect("the spoke's claim reached the hub");
    let answered: Vec<EventId> = landed
        .events()
        .iter()
        .filter(|event| *event.event.event_type() == CLAIM_REVERSED)
        .map(|event| event.id)
        .collect();
    assert_eq!(answered.len(), 1, "one reversal, in the claim's own write");
    assert!(
        answered.iter().all(|id| id.store() == HUB),
        "minted by the hub: {answered:?}"
    );
    assert!(
        reversals(&spoke.snapshot()).is_empty(),
        "the spoke's edge judged nothing it pushed"
    );
}

/// T-450-6, the refusal check #441 handed over. A push the store peer's
/// declared limits refuse is refused before its rule is asked, and leaves the
/// far log as it was (SY-33, SY-34).
///
/// Rejects a store peer that adjudicates before it admits.
#[tokio::test]
async fn a_refused_push_reaches_no_rule_and_writes_nothing() {
    let hub = Shared::new(HUB);
    let rule = SeatRule::new(&hub.0);
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 1;
    let peer = StoreSyncPeer::with_limits(hub.clone(), hub.clone(), limits).with_adjudicator(&rule);
    let batch = PushBatch::new(vec![EventGroup::new(
        None,
        vec![foreign_claim(ORIGIN_ONE, b"too-large-for-the-hub")],
    )]);

    let refused = peer.push(&batch).await;

    assert!(
        matches!(refused, Err(StorePeerError::EventTooLarge { .. })),
        "{refused:?}"
    );
    assert_eq!(rule.asked.get(), 0, "the rule was never asked");
    assert!(hub.0.snapshot().is_empty(), "the hub's log is unchanged");
}

/// A rule that cannot decide on a push writes nothing, and its class is the
/// store peer's error's class (SY-36).
///
/// Rejects a store peer that lands the push unjudged when its rule fails.
#[tokio::test]
async fn a_store_peer_whose_rule_cannot_decide_writes_nothing() {
    let hub = Shared::new(HUB);
    let peer =
        StoreSyncPeer::new(hub.clone(), hub.clone()).with_adjudicator(Failing(FailureClass::Park));
    let batch = PushBatch::new(vec![EventGroup::new(
        None,
        vec![foreign_claim(ORIGIN_ONE, b"pushed")],
    )]);

    let failed = peer.push(&batch).await;

    let Err(error) = failed else {
        panic!("the push fails: {failed:?}");
    };
    assert!(matches!(error, StorePeerError::Adjudicate(_)), "{error:?}");
    assert_eq!(error.class(), FailureClass::Park);
    assert!(hub.0.snapshot().is_empty(), "nothing landed");
}

/// The server half carries the rule: a push answered through
/// `serve_sync_adjudicated` lands with the hub's reversal, bytes in and bytes
/// out.
///
/// Rejects a sibling that serves through a store peer of its own and drops
/// the rule.
#[tokio::test]
async fn serve_sync_adjudicated_answers_a_push_through_the_rule() {
    let hub = Shared::new(HUB);
    ingest_unjudged(&hub, &foreign_claim(ORIGIN_ONE, b"first")).await;
    let peer = StoreSyncPeer::new(hub.clone(), hub.clone()).with_adjudicator(SeatRule::new(&hub.0));
    let pushed = foreign_claim(ORIGIN_TWO, b"second");
    let batch = PushBatch::new(vec![EventGroup::new(None, vec![pushed.clone()])]);
    let request = postcard::to_stdvec(&Envelope::new(&batch)).expect("a push encodes");

    serve_sync_adjudicated(&peer, Route::Push, &request)
        .await
        .expect("the hub answers");

    assert_eq!(
        reversals_of(&hub.0.snapshot(), pushed.id)
            .iter()
            .map(|id| id.store())
            .collect::<Vec<_>>(),
        vec![HUB]
    );
}
