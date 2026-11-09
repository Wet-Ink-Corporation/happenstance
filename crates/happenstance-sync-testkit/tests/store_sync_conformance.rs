//! `StoreSyncPeer`: a peer whose far side is a store the caller holds.
//!
//! The far side is `MemoryIngestFixture`'s store, which is both halves the
//! peer needs — an `EventStore` to read and an `IngestStore` to write — over
//! one log, as an adapter's store is. `MemoryEventStore` is not used: it cannot
//! ingest, and a peer whose pushes landed in a store its pulls never read would
//! fail SY-1 by construction.
//!
//! The sync suite is mounted at the bottom; the tests above it pin what the
//! suite does not: the token, the page shape, and the classes of refusal.

#![cfg(not(target_arch = "wasm32"))]

use core::cell::Cell;

use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, MemoryStoreError, Query, QueryItem, ReadOptions,
    RecordedAt, SequencePosition, SequencedEvent, StoreId, collect,
};
use happenstance_sync::wire::ResumeToken;
use happenstance_sync::{
    Classify, EventGroup, FailureClass, IngestError, IngestGroup, IngestStore, Ingested,
    PeerLimits, PushBatch, ReplicatedEvent, StorePeerError, StoreResume, StoreSyncPeer, SyncPeer,
    Waiting, Watermark,
};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, MemoryIngestHandle, RefusalsOf, SyncFixture,
    oversized_push_refusal,
};

const ORIGIN: StoreId = StoreId::from_bytes([0x71; 16]);

type MemoryStorePeer = StoreSyncPeer<MemoryIngestHandle, MemoryIngestHandle>;

/// One fresh store, and a store peer over it declaring `limits`; the handle is
/// kept so a test can read the log the peer wrote.
async fn peer_over(limits: PeerLimits) -> (MemoryStorePeer, MemoryIngestHandle) {
    let store = MemoryIngestFixture::new();
    let handle = store.connect().await;
    let peer = StoreSyncPeer::with_limits(store.connect().await, store.connect().await, limits);
    (peer, handle)
}

async fn log(store: &MemoryIngestHandle) -> Vec<SequencedEvent> {
    collect(EventStore::read(store, &Query::all(), ReadOptions::new()))
        .await
        .expect("the in-memory store's error is uninhabited")
}

/// A replicated event from `ORIGIN` at `position`, carrying `payload`.
fn replicated(position: u64, payload: &[u8]) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(
            ORIGIN,
            SequencePosition::new(position).expect("test positions are not zero"),
        ),
        RecordedAt::from_millis(1_800_000_000_000),
        Event::new("Replicated", payload.to_vec()).expect("a static event type is valid"),
    )
}

/// One group per event, `count` of them, from position 1.
fn singles(count: u64) -> PushBatch {
    PushBatch::new(
        (1..=count)
            .map(|position| EventGroup::new(None, vec![replicated(position, b"x")]))
            .collect(),
    )
}

/// A position-free guard: evidence that travels, never a refusal (SY-6).
fn guard() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Replicated"]).expect("a valid item"),
    ))
}

fn limits_of(events: usize) -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = events;
    limits
}

#[tokio::test]
async fn a_pushed_batch_comes_back_on_pull_byte_for_byte() {
    let (peer, _) = peer_over(PeerLimits::minimum()).await;
    let batch = PushBatch::new(vec![EventGroup::new(
        None,
        vec![replicated(3, &[0, 159, 146, 150]), replicated(7, b"seven")],
    )]);

    let ack = peer.push(&batch).await.expect("a push inside the limits");
    let pulled = peer.pull(None).await.expect("a pull");

    assert_eq!((ack.appended, ack.skipped), (2, 0));
    assert_eq!(ack.confirmed.get(ORIGIN), SequencePosition::new(7));
    let back: Vec<&ReplicatedEvent> = pulled.batch.events().collect();
    let sent: Vec<&ReplicatedEvent> = batch.events().collect();
    assert_eq!(back, sent);
    assert_eq!(pulled.waiting, Waiting::Nothing);
}

#[tokio::test]
async fn pushing_the_same_batch_twice_is_a_skip_and_the_log_is_unchanged() {
    let (peer, store) = peer_over(PeerLimits::minimum()).await;
    let batch = singles(3);

    peer.push(&batch).await.expect("the first push");
    let before = log(&store).await;
    let again = peer.push(&batch).await.expect("the replay");

    assert_eq!((again.appended, again.skipped), (0, 3));
    assert_eq!(log(&store).await, before);
}

/// Every answer names the far store's own identity (ADR-0104): an ack, a
/// page, an empty pull, and a pull from the last position a store can number,
/// which answers without reading the log. A second store's peer names the
/// second store, so the name is the store's and not a constant.
///
/// **Wrong implementations it rejects:** a peer that answers through
/// `Pulled::new` or `Ack::default` and names nobody, and one that stamps the
/// identity on one path and forgets another — the early return included.
#[tokio::test]
async fn every_answer_names_the_far_store_as_its_responder() {
    let (peer, store) = peer_over(PeerLimits::minimum()).await;
    let (other_peer, other_store) = peer_over(PeerLimits::minimum()).await;
    let named = IngestStore::store_id(&store)
        .await
        .expect("the in-memory store's error is uninhabited");
    let other = IngestStore::store_id(&other_store)
        .await
        .expect("the in-memory store's error is uninhabited");
    assert_ne!(named, other, "two fixtures are two stores");

    let ack = peer.push(&singles(2)).await.expect("a push");
    let page = peer.pull(None).await.expect("a pull");
    let empty = peer.pull(Some(&page.resume)).await.expect("an empty pull");
    let at_the_end = StoreResume::after(
        SequencePosition::new(u64::MAX).expect("the largest position is not zero"),
    );
    let past = peer
        .pull(Some(&at_the_end))
        .await
        .expect("a pull past the end");
    let elsewhere = other_peer
        .pull(None)
        .await
        .expect("a pull from the other store");

    assert_eq!(ack.responder, Some(named), "the ack");
    assert_eq!(page.responder, Some(named), "the page");
    assert_eq!(empty.responder, Some(named), "the empty pull");
    assert_eq!(past.responder, Some(named), "the pull past the end");
    assert_eq!(elsewhere.responder, Some(other), "the other store's pull");
}

/// The store [`StoreSyncPeer`] stands in front of when it is re-minted between
/// two answers: [`MemoryIngestHandle`], whose identity answers the store's own
/// id on the first read and [`REMINTED`] on every read after it.
///
/// A hand-written fake (TST-09) because nothing in the reference store changes
/// its identity; `Cell` because the port takes `&self`.
#[derive(Debug)]
struct RemintedAfterFirstRead {
    inner: MemoryIngestHandle,
    reads: Cell<usize>,
}

/// The identity a [`RemintedAfterFirstRead`] answers from its second read on.
const REMINTED: StoreId = StoreId::from_bytes([0x9e; 16]);

impl IngestStore for RemintedAfterFirstRead {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        let reads = self.reads.get();
        self.reads.set(reads + 1);
        if reads == 0 {
            IngestStore::store_id(&self.inner).await
        } else {
            Ok(REMINTED)
        }
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        IngestStore::ingest(&self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        IngestStore::watermark(&self.inner).await
    }
}

/// A store peer whose ingest half is re-minted after its first identity read,
/// and the store's id before the re-mint.
async fn remints_after_first_read() -> (
    StoreSyncPeer<MemoryIngestHandle, RemintedAfterFirstRead>,
    StoreId,
) {
    let store = MemoryIngestFixture::new();
    let before = IngestStore::store_id(&store.connect().await)
        .await
        .expect("the in-memory store's error is uninhabited");
    assert_ne!(before, REMINTED, "the re-mint must be observable");
    let ingest = RemintedAfterFirstRead {
        inner: store.connect().await,
        reads: Cell::new(0),
    };
    (StoreSyncPeer::new(store.connect().await, ingest), before)
}

/// The responder is read on every call (ADR-0104): a far store re-minted
/// between two pulls, or two pushes, is named by its new identity on the
/// second answer.
///
/// **Wrong implementation it rejects:** a peer that reads the identity once —
/// at construction, or into a `OnceCell` on the first call — and stamps that
/// copy on every answer after it. The re-mint the field exists to report is
/// exactly the change such a cache never sees.
#[tokio::test]
async fn a_far_store_re_minted_between_two_answers_is_named_anew_on_the_second() {
    let (pulling, before) = remints_after_first_read().await;
    let first = pulling.pull(None).await.expect("the first pull");
    let second = pulling.pull(None).await.expect("the second pull");

    let (pushing, pushed_before) = remints_after_first_read().await;
    let first_ack = pushing.push(&singles(1)).await.expect("the first push");
    let second_ack = pushing.push(&singles(1)).await.expect("the second push");

    assert_eq!(first.responder, Some(before), "the first pull");
    assert_eq!(second.responder, Some(REMINTED), "the second pull");
    assert_eq!(first_ack.responder, Some(pushed_before), "the first push");
    assert_eq!(second_ack.responder, Some(REMINTED), "the second push");
}

#[tokio::test]
async fn the_token_resumes_above_the_last_forwarded_event_and_an_empty_pull_keeps_it() {
    let (peer, _) = peer_over(PeerLimits::minimum()).await;
    peer.push(&singles(2)).await.expect("a push");

    let first = peer.pull(None).await.expect("a first pull");
    let second = peer.pull(Some(&first.resume)).await.expect("a second pull");

    assert_eq!(first.batch.events().count(), 2);
    assert_eq!(second.batch.events().count(), 0, "nothing is sent twice");
    assert_eq!(second.resume, first.resume, "an empty pull keeps the token");
    assert_eq!(second.waiting, Waiting::Nothing);
}

/// The store is the far side, so what it appended locally is forwarded too,
/// under the identity it minted.
#[tokio::test]
async fn a_local_append_on_the_far_store_is_forwarded_with_its_minted_identity() {
    let (peer, store) = peer_over(PeerLimits::minimum()).await;
    EventStore::append(
        &store,
        &[Event::new("Local", &b"here"[..]).expect("a valid event")],
        None,
    )
    .await
    .expect("a local append");

    let pulled = peer.pull(None).await.expect("a pull");

    let forwarded: Vec<EventId> = pulled.batch.events().map(|event| event.id).collect();
    let minted: Vec<EventId> = log(&store).await.iter().map(|event| event.id).collect();
    assert_eq!(forwarded, minted);
}

#[tokio::test]
async fn a_page_cut_by_count_does_not_say_nothing_and_the_last_page_does() {
    let (peer, store) = peer_over(limits_of(2)).await;
    StoreSyncPeer::new(store.clone(), store)
        .push(&singles(3))
        .await
        .expect("a push under the minimum limits");

    let first = peer.pull(None).await.expect("a first pull");
    let second = peer.pull(Some(&first.resume)).await.expect("a second pull");

    assert_eq!(first.batch.events().count(), 2);
    assert_eq!(
        first.waiting,
        Waiting::Unknown,
        "the page ran out, not the log"
    );
    assert_eq!(second.batch.events().count(), 1);
    assert_eq!(second.waiting, Waiting::Nothing);
}

/// An event the far store's own writer accepted, over the declared
/// `max_event_bytes`, fits no page: replication stalls at it rather than
/// skipping it or sending it over the limit.
#[tokio::test]
async fn an_oversized_local_event_at_the_head_stalls_the_pull_on_one_token() {
    let (peer, store) = peer_over(PeerLimits::minimum()).await;
    let declared = peer.limits().max_event_bytes;
    let append = |payload: Vec<u8>| {
        let store = &store;
        async move {
            EventStore::append(
                store,
                &[Event::new("Local", payload).expect("a valid event")],
                None,
            )
            .await
            .expect("the far store's own writer has no declared limit")
        }
    };
    append(b"before".to_vec()).await;
    let first = peer.pull(None).await.expect("a first pull");
    append(vec![0; declared + 1]).await;
    append(b"after".to_vec()).await;

    let stalled = peer.pull(Some(&first.resume)).await.expect("a pull");
    let again = peer.pull(Some(&stalled.resume)).await.expect("a pull");

    assert_eq!(first.batch.events().count(), 1);
    for pulled in [&stalled, &again] {
        assert_eq!(pulled.batch, PushBatch::empty());
        assert_eq!(pulled.waiting, Waiting::Unknown, "the log goes on");
        assert_eq!(pulled.resume, first.resume, "the token never moves");
    }
}

/// A pull hands back the groups the far store wrote, one for one, with an
/// ingested group's guard (SY-30, SY-39). Rejects the page-as-one-group peer
/// #420 replaced, which merged the four into one unguarded group.
#[tokio::test]
async fn a_page_carries_the_groups_the_store_wrote() {
    let (peer, _) = peer_over(PeerLimits::minimum()).await;
    let guarded = EventGroup::new(
        Some(guard()),
        vec![replicated(4, b"pair"), replicated(5, b"pair")],
    );
    let mut groups = singles(3).groups;
    groups.push(guarded);
    let batch = PushBatch::new(groups);
    peer.push(&batch).await.expect("a push");

    let pulled = peer.pull(None).await.expect("a pull");

    assert_eq!(pulled.batch, batch);
    assert_eq!(pulled.waiting, Waiting::Nothing);
}

/// A group that would take a page past its budget moves to the next page
/// whole. Rejects a pull that fills the page by event and cuts the group.
#[tokio::test]
async fn a_group_crossing_the_page_budget_moves_to_the_next_page_whole() {
    let (peer, _) = peer_over(limits_of(3)).await;
    let pair =
        |at: u64| EventGroup::new(None, vec![replicated(at, b"x"), replicated(at + 1, b"x")]);
    peer.push(&PushBatch::new(vec![pair(1)]))
        .await
        .expect("a push inside the limits");
    peer.push(&PushBatch::new(vec![pair(3)]))
        .await
        .expect("a push inside the limits");

    let first = peer.pull(None).await.expect("a first pull");
    let second = peer.pull(Some(&first.resume)).await.expect("a second pull");

    assert_eq!(first.batch, PushBatch::new(vec![pair(1)]));
    assert_eq!(
        first.waiting,
        Waiting::Unknown,
        "the page ran out, not the log"
    );
    assert_eq!(second.batch, PushBatch::new(vec![pair(3)]));
    assert_eq!(second.waiting, Waiting::Nothing);
}

/// A group the far store wrote that no page under the declared budget can
/// carry stalls the pull at it, token unmoved, rather than being cut or
/// skipped: the oversized event's rule, extended to a group (D-2).
#[tokio::test]
async fn an_oversized_group_at_the_head_stalls_the_pull_on_one_token() {
    let (peer, store) = peer_over(limits_of(2)).await;
    let events: Vec<Event> = (0..3)
        .map(|_| Event::new("Local", &b"x"[..]).expect("a valid event"))
        .collect();
    EventStore::append(&store, &events, None)
        .await
        .expect("the far store's own writer has no declared limit");

    let stalled = peer.pull(None).await.expect("a pull");
    let again = peer.pull(Some(&stalled.resume)).await.expect("a pull");

    for pulled in [&stalled, &again] {
        assert_eq!(pulled.batch, PushBatch::empty());
        assert_eq!(pulled.waiting, Waiting::Unknown, "the log goes on");
        assert_eq!(pulled.resume, StoreResume::start(), "the token never moves");
    }
}

#[tokio::test]
async fn a_push_over_the_declared_events_is_refused_as_split_and_writes_nothing() {
    let (peer, store) = peer_over(limits_of(2)).await;

    let refused = peer
        .push(&singles(3))
        .await
        .expect_err("three events over a limit of two");

    assert!(matches!(
        refused,
        StorePeerError::TooManyEvents {
            actual: 3,
            limit: 2,
            ..
        }
    ));
    assert_eq!(refused.class(), FailureClass::Split);
    assert_eq!(log(&store).await, Vec::new());
}

#[tokio::test]
async fn an_event_over_the_declared_size_is_refused_as_park() {
    let limits = PeerLimits::minimum();
    let (peer, store) = peer_over(limits.clone()).await;
    let oversized = vec![0_u8; limits.max_event_bytes + 1];

    let refused = peer
        .push(&PushBatch::new(vec![EventGroup::new(
            None,
            vec![replicated(1, &oversized)],
        )]))
        .await
        .expect_err("one byte over max_event_bytes");

    assert!(matches!(refused, StorePeerError::EventTooLarge { .. }));
    assert_eq!(refused.class(), FailureClass::Park);
    assert_eq!(log(&store).await, Vec::new());
}

#[test]
fn a_store_token_survives_its_text_form() {
    let after = StoreResume::after(SequencePosition::new(42).expect("not zero"));

    for token in [StoreResume::start(), after] {
        let text = ResumeToken::from(token);
        assert_eq!(StoreResume::try_from(&text), Ok(token));
    }
}

#[test]
fn text_a_store_peer_never_minted_is_refused_as_stop() {
    // Not one of these is what `From<StoreResume>` writes: the text form is
    // one-to-one, so `"+5"` and `"05"` are refused rather than read as 5.
    for foreign in ["", "group:42", "-1", "+5", "05", "18446744073709551616"] {
        let token = ResumeToken::new(foreign);
        let refused = StoreResume::try_from(&token).expect_err("not a store token");
        assert_eq!(refused.token(), &token);
        assert_eq!(refused.class(), FailureClass::Stop);
    }
}

/// One fresh in-memory store per fixture; each `connect` is a new peer over
/// new handles onto it, so a dropped peer's writes are what the next one reads.
#[derive(Debug, Default)]
struct StorePeerFixture {
    store: MemoryIngestFixture,
}

impl SyncFixture for StorePeerFixture {
    type Peer = MemoryStorePeer;

    async fn connect(&self) -> Self::Peer {
        StoreSyncPeer::new(self.store.connect().await, self.store.connect().await)
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

happenstance_sync_testkit::sync_peer_conformance!(super::StorePeerFixture::default());

// The model family plays a runner over one fixture per node, which is where a
// page-as-one-group peer would show a forwarding or convergence fault the
// single-peer rules cannot.
#[cfg(feature = "proptest")]
happenstance_sync_testkit::sync_model_conformance!(super::StorePeerFixture::default());
