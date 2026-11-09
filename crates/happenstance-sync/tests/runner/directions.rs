//! SY-10 at the runner's grain: one [`Edge`], a different merge rule in each
//! direction, over both topologies.
//!
//! The edge carries no merge rule of its own. What it pulls it ingests as
//! sent, with no compensation, which is a spoke's rule. What it pushes is
//! judged by the far side's push handler, which is the hub's to choose. So the
//! two directions of one edge already take different rules, and the edge
//! needs no role parameter to allow it (SY-9). [`Hub`] is that far side: a
//! [`StoreSyncPeer`] whose push adjudicates first, standing in for the hook
//! SY-7 plans, and which can decline a push (SY-33).
//!
//! The testkit's `directional_merge_rules_compose` holds a store to its half:
//! it writes what each direction hands it and nothing of its own. These tests
//! hold the shipped runner to the other half.

use core::cell::Cell;
use core::slice;

use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, MemoryStoreError, Query, QueryItem, ReadOptions,
    RecordedAt, SequencePosition, SequencedEvent, StoreId, Tag, Tags, collect,
};
use happenstance_sync::runner::{Edge, StepError};
use happenstance_sync::write_group::GroupOrderError;
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, IngestError, IngestGroup, IngestStore,
    MemoryIngestStore, PeerLimits, Pulled, PushBatch, ReplicatedEvent, StorePeerError, StoreResume,
    StoreSyncPeer, SyncPeer,
};

use super::probe::Shared;

const HUB: StoreId = StoreId::from_bytes([0x4B; 16]);
const SPOKE: StoreId = StoreId::from_bytes([0x5B; 16]);
const PEER: StoreId = StoreId::from_bytes([0x6B; 16]);
/// The spoke whose claim reached the hub first: an origin the hub has seen,
/// and the spoke never meets.
const OTHER_SPOKE: StoreId = StoreId::from_bytes([0x7B; 16]);

/// How the [`Hub`] fails.
#[derive(Debug, thiserror::Error)]
enum HubError {
    #[error(transparent)]
    Peer(#[from] StorePeerError<GroupOrderError, MemoryStoreError>),
    /// The device's token had expired: refreshed before the next step, so
    /// the same push can succeed later.
    #[error("the hub declined this device's push")]
    Declined,
    #[error("the hub could not read its log to adjudicate")]
    Read(#[source] MemoryStoreError),
    #[error("the hub could not write its adjudication")]
    Adjudicate(#[source] IngestError<MemoryStoreError>),
}

impl Classify for HubError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Peer(error) => error.class(),
            Self::Declined => FailureClass::Transient,
            Self::Read(error) => error.class(),
            Self::Adjudicate(error) => error.class(),
        }
    }
}

/// A hub: a [`StoreSyncPeer`] over its store, whose push handler adjudicates
/// each pushed group whose guard the hub's log breaks before the push lands,
/// and which declines pushes while `declining` is set.
#[derive(Debug)]
struct Hub {
    store: Shared,
    peer: StoreSyncPeer<Shared, Shared>,
    reversal: Event,
    declining: Cell<bool>,
}

impl Hub {
    fn new(id: StoreId) -> Self {
        let store = Shared::new(id);
        Self {
            peer: StoreSyncPeer::new(store.clone(), store.clone()),
            store,
            reversal: seat_event("ClaimReversed", b"reversed-by-the-hub"),
            declining: Cell::new(false),
        }
    }

    /// The hub's rule for one pushed group: where the group claims a seat
    /// the hub already holds another claim on, the group lands with the hub's
    /// reversal, in one write (SY-2).
    ///
    /// Judged by the fact family (SY-35), from what the hub holds, and not
    /// from a travelling guard: a local append records none, because its
    /// condition is position-relative (SY-6), so a spoke's own claim reaches
    /// the hub without one.
    async fn adjudicate(&self, group: &EventGroup) -> Result<(), HubError> {
        let pushed: Vec<EventId> = group.events.iter().map(|event| event.id).collect();
        let mut contested = false;
        for event in &group.events {
            if event.event.event_type().as_str() != "SeatClaimed" {
                continue;
            }
            let query = family(event.event.tags());
            let held = collect(self.store.0.read(&query, ReadOptions::new()))
                .await
                .map_err(HubError::Read)?;
            contested |= held.iter().any(|row| !pushed.contains(&row.id));
        }
        if !contested {
            return Ok(());
        }
        let group = IngestGroup::new(&group.events, slice::from_ref(&self.reversal))
            .with_guard(group.guard.as_ref());
        IngestStore::ingest(&self.store, &[group])
            .await
            .map_err(HubError::Adjudicate)?;
        Ok(())
    }
}

impl SyncPeer for Hub {
    type Error = HubError;
    type Resume = StoreResume;

    async fn pull(&self, from: Option<&StoreResume>) -> Result<Pulled<StoreResume>, HubError> {
        Ok(self.peer.pull(from).await?)
    }

    /// Adjudicates, then lets the store peer land the rest and acknowledge:
    /// a group the hub answered is a skip there.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, HubError> {
        if self.declining.get() {
            return Err(HubError::Declined);
        }
        for group in &batch.groups {
            self.adjudicate(group).await?;
        }
        Ok(self.peer.push(batch).await?)
    }

    fn limits(&self) -> PeerLimits {
        self.peer.limits()
    }
}

fn seat() -> Tags {
    [Tag::from_static("seat:s10")].into_iter().collect()
}

fn seat_event(event_type: &'static str, data: &'static [u8]) -> Event {
    Event::new(event_type, data)
        .expect("a static event type is valid")
        .with_tags(seat())
}

/// Every claim on the seat `tags` name: the fact family the hub adjudicates.
fn family(tags: &Tags) -> Query {
    let item = QueryItem::new(["SeatClaimed"], tags.clone()).expect("a valid query item");
    Query::from_item(item)
}

/// "No claim on this seat yet": the condition every claim is decided under.
fn guard() -> AppendCondition {
    AppendCondition::new(family(&seat()))
}

/// `store` appends its own claim under the guard, and the claim's identity.
async fn claim_locally(store: &MemoryIngestStore, data: &'static [u8]) -> EventId {
    let at = EventStore::append(store, &[seat_event("SeatClaimed", data)], Some(&guard()))
        .await
        .expect("nothing claims the seat here yet");
    EventId::new(
        IngestStore::store_id(store).await.expect("an in-memory id"),
        at,
    )
}

/// A log by type and bytes, sorted: the facts two converged logs share.
fn facts(log: &[SequencedEvent]) -> Vec<(String, Vec<u8>)> {
    let mut facts: Vec<_> = log
        .iter()
        .map(|row| {
            (
                row.event_type().as_str().to_owned(),
                row.event.data().to_vec(),
            )
        })
        .collect();
    facts.sort();
    facts
}

fn expected(rows: &[(&str, &[u8])]) -> Vec<(String, Vec<u8>)> {
    let mut facts: Vec<_> = rows
        .iter()
        .map(|(event_type, data)| ((*event_type).to_owned(), data.to_vec()))
        .collect();
    facts.sort();
    facts
}

/// SY-10's hub-and-spoke edge, driven by the spoke. The hub holds another
/// spoke's claim; the spoke holds its own. The first step pulls the other
/// claim, which the spoke's own breaks the guard of, and ingests it unjudged;
/// its push is declined, and its own claim stays (SY-34, E2E-45). The next
/// step's push is adjudicated at the hub, and the step after pulls the hub's
/// reversal. Both logs end with both claims and one reversal, the hub's.
///
/// Rejects a runner that judges what it pulls, which writes a spoke's
/// reversal of the claim that reached the hub first; and one that answers a
/// declined push by rolling the spoke back to what the hub holds.
#[tokio::test]
async fn hub_and_spoke_the_hub_adjudicates_the_push_and_the_spoke_takes_the_pull_unjudged() {
    let hub = Hub::new(HUB);
    let first = ReplicatedEvent::new(
        EventId::new(OTHER_SPOKE, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        seat_event("SeatClaimed", b"claimed-by-the-other-spoke"),
    );
    let guard = guard();
    IngestStore::ingest(
        &hub.store,
        &[IngestGroup::new(slice::from_ref(&first), &[]).with_guard(Some(&guard))],
    )
    .await
    .expect("the hub takes the first claim");
    let spoke = MemoryIngestStore::with_store_id(SPOKE);
    let own = claim_locally(&spoke, b"claimed-by-the-spoke").await;
    let mut edge = Edge::new();

    hub.declining.set(true);
    let declined = edge.step(&hub, &spoke).await;
    assert!(
        matches!(declined, Err(StepError::Push(HubError::Declined))),
        "the push is declined: {declined:?}"
    );
    let after_decline = spoke.snapshot();
    assert_eq!(
        facts(&after_decline),
        expected(&[
            ("SeatClaimed", b"claimed-by-the-spoke"),
            ("SeatClaimed", b"claimed-by-the-other-spoke"),
        ]),
        "the spoke took the conflicting claim unjudged and kept its own"
    );
    assert!(
        after_decline.iter().any(|row| row.id == own),
        "its own claim, as it wrote it"
    );

    hub.declining.set(false);
    let pushed = edge.step(&hub, &spoke).await.expect("the push is taken");
    assert_eq!(pushed.pushed, 2, "its own claim, and the claim it relays");
    let pulled = edge
        .step(&hub, &spoke)
        .await
        .expect("the reversal is pulled");
    assert_eq!(pulled.ingested.appended, 1, "the hub's reversal");
    assert_eq!(
        pulled.ingested.compensated, 0,
        "the spoke judges nothing it pulls"
    );

    let both = expected(&[
        ("SeatClaimed", b"claimed-by-the-other-spoke"),
        ("SeatClaimed", b"claimed-by-the-spoke"),
        ("ClaimReversed", b"reversed-by-the-hub"),
    ]);
    assert_eq!(facts(&spoke.snapshot()), both, "the spoke");
    assert_eq!(facts(&hub.store.0.snapshot()), both, "the hub");
    let reversal = spoke
        .snapshot()
        .into_iter()
        .find(|row| row.event_type().as_str() == "ClaimReversed")
        .expect("the reversal reached the spoke");
    assert_eq!(
        reversal.id.store(),
        HUB,
        "written by the hub, the one adjudicator"
    );
}

/// SY-10's peer-to-peer edge: both directions accept. Each peer holds its
/// own claim; one step pulls the far claim unjudged and pushes the near one,
/// which the far store peer lands unjudged. Both logs hold both claims and
/// no reversal: the conflict waits for the family's adjudicator (SY-7).
///
/// Rejects a runner that judges what it pulls, and so turns a symmetric
/// edge into one with an adjudicator nobody assigned.
#[tokio::test]
async fn peer_to_peer_both_directions_take_the_other_claim_unjudged() {
    let far = Shared::new(PEER);
    let far_claim = claim_locally(&far.0, b"claimed-by-the-far-peer").await;
    let peer = StoreSyncPeer::new(far.clone(), far.clone());
    let near = MemoryIngestStore::with_store_id(SPOKE);
    let near_claim = claim_locally(&near, b"claimed-by-the-near-peer").await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &near).await.expect("a step");
    assert_eq!(stepped.ingested.appended, 1, "the far claim");
    assert_eq!(stepped.ingested.compensated, 0, "judged by nobody");
    edge.step(&peer, &near).await.expect("a second step");
    assert!(edge.status().is_synced(), "{:?}", edge.status());

    let both = expected(&[
        ("SeatClaimed", b"claimed-by-the-far-peer"),
        ("SeatClaimed", b"claimed-by-the-near-peer"),
    ]);
    for (log, name) in [(near.snapshot(), "near"), (far.0.snapshot(), "far")] {
        assert_eq!(facts(&log), both, "the {name} peer");
        for claim in [near_claim, far_claim] {
            assert!(
                log.iter().any(|row| row.id == claim),
                "the {name} peer holds {claim}"
            );
        }
    }
}
