//! The runner's single edge, driven one step at a time (#244, part 1).
//!
//! Every test drives an [`Edge`] over the in-memory reference types: a
//! [`MemoryIngestStore`] as the local store and a [`MemorySyncPeer`] as the far
//! side. Each names the wrong runner it rejects.

use happenstance_core::{
    Event, EventId, EventStore, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::runner::{Edge, EdgeState, Failure, Phase, StepError};
use happenstance_sync::{
    EventGroup, FailureClass, IngestError, MemoryIngestStore, MemorySyncPeer, PeerLimits,
    ReplicatedEvent, SyncPeer, Waiting,
};

const FAR: StoreId = StoreId::from_bytes([0xFA; 16]);
const ORIGIN: StoreId = StoreId::from_bytes([0x0A; 16]);
const LOCAL: StoreId = StoreId::from_bytes([0x10; 16]);

fn position(raw: u64) -> SequencePosition {
    SequencePosition::new(raw).expect("a test position is non-zero")
}

fn event(name: &str) -> Event {
    Event::new("Written", name.as_bytes().to_vec()).expect("a static event type is valid")
}

fn foreign(origin: StoreId, at: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, position(at)),
        RecordedAt::from_millis(1_800_000_000_000),
        event("foreign"),
    )
}

/// Appends `count` local events one append each, and returns the positions.
async fn append_local(store: &MemoryIngestStore, count: usize) -> Vec<SequencePosition> {
    let mut positions = Vec::with_capacity(count);
    for index in 0..count {
        let landed = EventStore::append(store, &[event(&index.to_string())], None)
            .await
            .expect("an in-memory append does not fail");
        positions.push(landed);
    }
    positions
}

/// Limits whose batch holds at most `events` events.
fn batches_of(events: usize) -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = events;
    limits
}

fn ids(log: &[SequencedEvent]) -> Vec<EventId> {
    log.iter().map(|row| row.id).collect()
}

/// Rejects a step that never ingests what it pulled.
#[tokio::test]
async fn a_step_pulls_the_far_sides_events_into_the_local_log() {
    let peer = MemorySyncPeer::new(FAR);
    peer.seed(EventGroup::new(
        None,
        vec![foreign(ORIGIN, 1), foreign(ORIGIN, 2)],
    ))
    .expect("an unpoisoned peer");
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.ingested.appended, 2);
    assert_eq!(
        ids(&store.snapshot()),
        vec![foreign(ORIGIN, 1).id, foreign(ORIGIN, 2).id],
        "the far side's events landed with their origin identity"
    );
    assert_eq!(edge.status().waiting, Waiting::Nothing);
}

/// Rejects a runner that confirms nothing, or that confirms what the far side
/// reports instead of the local position it pushed through.
#[tokio::test]
async fn a_step_pushes_local_events_and_confirms_the_last_one() {
    let peer = MemorySyncPeer::new(FAR);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_local(&store, 3).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 3);
    assert_eq!(edge.status().confirmed, positions.last().copied());
    assert_eq!(edge.status().pending, Some(0));
    assert_eq!(
        edge.status().far_side.get(LOCAL),
        positions.last().copied(),
        "the far side's last acknowledgement is kept per origin"
    );
    assert!(edge.status().is_synced());
    let pulled = peer.pull(None).await.expect("an in-memory pull");
    let held: Vec<EventId> = pulled.batch.events().map(|row| row.id).collect();
    assert_eq!(held, ids(&store.snapshot()));
}

/// Rejects a pending count that is the batch size, or the whole log.
#[tokio::test]
async fn pending_counts_the_local_events_the_peer_has_not_confirmed() {
    let peer = MemorySyncPeer::with_limits(FAR, batches_of(2));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_local(&store, 5).await;
    let mut edge = Edge::new();

    edge.step(&peer, &store).await.expect("a first step");
    let after_one = (edge.status().confirmed, edge.status().pending);
    edge.step(&peer, &store).await.expect("a second step");
    let after_two = (edge.status().confirmed, edge.status().pending);

    assert_eq!(after_one, (positions.get(1).copied(), Some(3)));
    assert_eq!(after_two, (positions.get(3).copied(), Some(1)));
    assert!(!edge.status().is_synced());
}

/// Rejects a runner that reads the local log from the start on every step.
#[tokio::test]
async fn a_second_step_pushes_nothing_already_confirmed() {
    let peer = MemorySyncPeer::new(FAR);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 3).await;
    let mut edge = Edge::new();
    edge.step(&peer, &store).await.expect("a first step");

    let second = edge.step(&peer, &store).await.expect("a second step");

    assert_eq!(second.pushed, 0);
    assert_eq!(
        second.ingested.skipped, 3,
        "the far side hands back what it holds"
    );
    assert_eq!(store.snapshot().len(), 3, "and nothing lands twice");
}

/// SY-34. Rejects the tidy wrong runner that rolls the spoke back on refusal,
/// and one that advances the watermark past a push that did not land.
#[tokio::test]
async fn a_refused_push_keeps_the_log_and_the_watermark_and_records_the_class() {
    let mut limits = PeerLimits::minimum();
    // Below every event's payload, so the first is offered alone and refused.
    limits.max_batch_bytes = 0;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 2).await;
    let before = store.snapshot();
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;

    assert!(
        matches!(refused, Err(StepError::Push(_))),
        "the peer refused the push: {refused:?}"
    );
    assert_eq!(store.snapshot(), before, "the local log is unchanged");
    assert_eq!(edge.status().confirmed, None);
    assert_eq!(edge.status().pending, Some(2));
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Push, FailureClass::Split))
    );
    assert_eq!(edge.status().state, EdgeState::Running);
}

/// SY-38 meets the runner: the far side offers an event under the local
/// store's own identity at a position the store does not hold. The ingest
/// refuses with `Stop`. Rejects a runner that keeps stepping after a stop, and
/// one that advances the pull token past a batch it did not ingest.
#[tokio::test]
async fn a_stop_class_failure_stops_the_edge_and_the_next_step_does_no_io() {
    let peer = MemorySyncPeer::new(FAR);
    peer.seed(EventGroup::new(None, vec![foreign(LOCAL, 9)]))
        .expect("an unpoisoned peer");
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 1).await;
    let before = store.snapshot();
    let mut edge = Edge::new();

    let stopped = edge.step(&peer, &store).await;
    let refused = edge.step(&peer, &store).await;

    let cause = Failure::new(Phase::Ingest, FailureClass::Stop);
    assert!(
        matches!(
            stopped,
            Err(StepError::Ingest(IngestError::UnheldOwnIdentity { .. }))
        ),
        "the ingest refused the claim: {stopped:?}"
    );
    assert!(
        matches!(refused, Err(StepError::Stopped(failure)) if failure == cause),
        "a stopped edge refuses: {refused:?}"
    );
    assert_eq!(edge.status().state, EdgeState::Stopped(cause));
    assert_eq!(edge.resume_token(), None, "the token did not advance");
    assert_eq!(edge.status().confirmed, None, "nothing was pushed");
    assert_eq!(store.snapshot(), before, "the local log is unchanged");
    assert_eq!(
        peer.group_count().expect("an unpoisoned peer"),
        1,
        "nothing was pushed after the stop"
    );
}

/// The SY-31 seam: an edge rebuilt from a stored watermark and token resumes
/// past them. Rejects a resumed edge that re-pushes from the start.
#[tokio::test]
async fn a_resumed_edge_does_not_push_what_was_confirmed() {
    let peer = MemorySyncPeer::new(FAR);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 3).await;
    let mut first = Edge::new();
    first.step(&peer, &store).await.expect("a first step");
    // The second step pulls the far side's echo of the first push, so the
    // token it leaves is past everything the far side holds.
    first.step(&peer, &store).await.expect("a second step");
    let token = first.resume_token().copied();
    let confirmed = first.status().confirmed;
    drop(first);
    let since = append_local(&store, 1).await;

    let mut resumed = Edge::resuming(confirmed, token);
    let stepped = resumed.step(&peer, &store).await.expect("a resumed step");

    assert_eq!(stepped.pushed, 1, "only the event appended since");
    assert_eq!(stepped.ingested.skipped, 0, "the token was honoured");
    assert_eq!(resumed.status().confirmed, since.first().copied());
}
