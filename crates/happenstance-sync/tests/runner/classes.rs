//! What a step does with each failure class (SY-36, #244 part 2), and the
//! bounded outbound buffer.
//!
//! `Transient` changes nothing in a step, and is the run loop's to wait out:
//! `drive` tests it. Here: a `Split` the runner can halve, one it cannot, a
//! `Park` in either direction, and the buffer that holds whatever the peer
//! declares.

use core::num::NonZeroUsize;

use happenstance_core::StoreLimit;
use happenstance_sync::runner::{Edge, EdgeState, Failure, Pending, Phase, StepError};
use happenstance_sync::{
    EventGroup, FailureClass, MemoryIngestStore, MemoryResume, MemorySyncPeer, PeerLimits,
};

use super::probe::{ProbePeer, ProbeStore};
use super::{FAR, LOCAL, ORIGIN, append_local, event, foreign};

/// Limits that bound nothing: what a careless or hostile peer may declare.
fn unbounded() -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = usize::MAX;
    limits.max_batch_bytes = usize::MAX;
    limits.max_batch_events = usize::MAX;
    limits
}

/// A probe store holding `count` single-event local appends.
async fn local_singles(count: usize) -> ProbeStore {
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    append_local(&store.inner, count).await;
    store
}

/// Rejects a runner that sizes its read by the peer's declaration alone: a
/// peer declaring `usize::MAX` events per batch would make it read, and hold,
/// the whole unconfirmed tail in one step (H-13).
#[tokio::test]
async fn a_peer_declaring_no_limit_is_read_one_buffer_at_a_time() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.limits.replace(Some(unbounded()));
    let store = local_singles(10).await;
    let four = NonZeroUsize::MIN.saturating_add(3);
    let mut edge = Edge::new().with_buffer(four);

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(*store.read_limits.borrow(), vec![5], "one buffer plus one");
    assert_eq!(stepped.pushed, 4);
    assert_eq!(
        edge.status().pending,
        Pending::AtLeast(1),
        "the one event read past the push, and possibly more"
    );
}

/// Rejects a default edge with no buffer of its own.
#[tokio::test]
async fn a_default_edge_reads_at_most_its_default_buffer() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.limits.replace(Some(unbounded()));
    let store = local_singles(1).await;
    let mut edge = Edge::new();

    edge.step(&peer, &store).await.expect("an in-memory step");

    let bound = Edge::<MemoryResume>::DEFAULT_BUFFER.get() + 1;
    assert_eq!(*store.read_limits.borrow(), vec![bound]);
}

/// A `Split` halves what the edge offers until one group is left, and a split
/// of that one group parks the edge. Rejects a runner that re-offers the same
/// batch for ever (four groups every time), one that halves to nothing, and
/// one that stops or carries on after a split it cannot satisfy.
#[tokio::test]
async fn a_split_halves_the_offer_until_one_group_then_parks() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.fail_push.set(Some(FailureClass::Split));
    let store = local_singles(4).await;
    let log = store.inner.snapshot();
    let mut edge = Edge::new();

    let mut outcomes = Vec::new();
    for _ in 0..4 {
        outcomes.push(edge.step(&peer, &store).await);
    }

    let cause = Failure::new(Phase::Push, FailureClass::Split);
    assert_eq!(*peer.offered.borrow(), vec![4, 2, 1], "halved, then parked");
    assert!(
        matches!(outcomes.last(), Some(Err(StepError::Parked(failure))) if *failure == cause),
        "{outcomes:?}"
    );
    assert_eq!(edge.status().state, EdgeState::Parked(cause));
    assert_eq!(edge.status().confirmed, None);
    assert_eq!(store.inner.snapshot(), log, "the local log is unchanged");
}

/// One write group is never cut to satisfy a split (SY-30): a refused group of
/// three events parks the edge at once. Rejects a runner that halves the
/// events of a single group, which would push a torn write.
#[tokio::test]
async fn a_split_of_one_multi_event_group_parks_without_cutting_it() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.fail_push.set(Some(FailureClass::Split));
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    happenstance_core::EventStore::append(
        &store.inner,
        &[event(b"a"), event(b"b"), event(b"c")],
        None,
    )
    .await
    .expect("an in-memory append");
    let mut edge = Edge::new();

    edge.step(&peer, &store)
        .await
        .expect_err("the peer refuses every push");
    let parked = edge.step(&peer, &store).await;

    assert_eq!(*peer.offered.borrow(), vec![1], "offered whole, once");
    assert!(matches!(parked, Err(StepError::Parked(_))), "{parked:?}");
}

/// The peer's declared limits overstate what it enforces: its first refusal
/// halves the offer, and the halved offer then lands, in two pushes, and stays
/// halved. Rejects a runner that parks on a split it could halve, and one that
/// forgets the halving after a success and offers four again.
#[tokio::test]
async fn a_halved_offer_lands_and_stays_halved() {
    let mut enforced = PeerLimits::minimum();
    enforced.max_batch_bytes = 2;
    let peer = ProbePeer::new(MemorySyncPeer::with_limits(FAR, enforced));
    peer.limits.replace(Some(PeerLimits::minimum()));
    let store = local_singles(4).await;
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;
    edge.step(&peer, &store).await.expect("two groups fit");
    edge.step(&peer, &store).await.expect("and two more");

    assert!(matches!(refused, Err(StepError::Push(_))), "{refused:?}");
    assert_eq!(*peer.offered.borrow(), vec![4, 2, 2]);
    assert_eq!(edge.status().pending, Pending::Exactly(0));
    assert_eq!(edge.status().state, EdgeState::Running);
}

/// A `Park` parks the edge, and a parked edge does no I/O. Rejects a runner
/// that treats a park as transient and re-offers the event for ever, and one
/// that stops: the cause is the content, not the configuration.
#[tokio::test]
async fn a_park_parks_the_edge_and_the_next_step_does_no_io() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.fail_push.set(Some(FailureClass::Park));
    let store = local_singles(1).await;
    let mut edge = Edge::new();

    edge.step(&peer, &store)
        .await
        .expect_err("the peer refuses the push");
    let (calls, store_calls) = (peer.calls.get(), store.calls.get());
    let parked = edge.step(&peer, &store).await;

    let cause = Failure::new(Phase::Push, FailureClass::Park);
    assert_eq!(edge.status().state, EdgeState::Parked(cause));
    assert!(
        matches!(parked, Err(StepError::Parked(failure)) if failure == cause),
        "{parked:?}"
    );
    assert_eq!(peer.calls.get(), calls, "no peer call");
    assert_eq!(store.calls.get(), store_calls, "no store call");
}

/// Inbound, too: a pulled event the local store can never hold parks the edge
/// (VT-25's per-event limits are `Park`), and a batch over the local store's
/// batch limit parks it as well, because a step holds no batch of its own to
/// halve on the ingest side. Rejects a runner that halves only pushes and
/// retries an unhalvable ingest for ever.
#[tokio::test]
async fn an_ingest_the_local_store_cannot_hold_parks_the_edge() {
    for (limit, class) in [
        (StoreLimit::EventDataLen, FailureClass::Park),
        (StoreLimit::EventsPerBatch, FailureClass::Split),
    ] {
        let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
        peer.inner
            .seed(EventGroup::new(None, vec![foreign(ORIGIN, 1)]))
            .expect("an unpoisoned peer");
        let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
        store.refuse_ingest.set(Some(limit));
        let mut edge = Edge::new();

        let refused = edge.step(&peer, &store).await;

        assert!(matches!(refused, Err(StepError::Ingest(_))), "{refused:?}");
        assert_eq!(
            edge.status().state,
            EdgeState::Parked(Failure::new(Phase::Ingest, class)),
            "{limit:?}"
        );
        assert_eq!(edge.resume_token(), None, "the token did not move");
    }
}
