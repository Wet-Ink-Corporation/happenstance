//! The runner's single edge, driven one step at a time (#244, part 1).
//!
//! Every test drives an [`Edge`] over the in-memory reference types: a
//! [`MemoryIngestStore`] as the local store and a [`MemorySyncPeer`] as the far
//! side, or the fakes in `probe` over them where a test needs to count calls
//! or provoke a failure. Each names the wrong runner it rejects.

mod groups;
mod probe;

use happenstance_core::{
    Event, EventId, EventStore, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::runner::{Edge, EdgeState, Failure, Pending, Phase, StepError};
use happenstance_sync::{
    EventGroup, FailureClass, IngestError, MemoryIngestStore, MemorySyncPeer, PeerLimits,
    ReplicatedEvent, SyncPeer, Waiting,
};

use probe::{ProbePeer, ProbeStore};

const FAR: StoreId = StoreId::from_bytes([0xFA; 16]);
const ORIGIN: StoreId = StoreId::from_bytes([0x0A; 16]);
const LOCAL: StoreId = StoreId::from_bytes([0x10; 16]);

fn position(raw: u64) -> SequencePosition {
    SequencePosition::new(raw).expect("a test position is non-zero")
}

fn event(payload: &[u8]) -> Event {
    Event::new("Written", payload.to_vec()).expect("a static event type is valid")
}

fn foreign(origin: StoreId, at: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, position(at)),
        RecordedAt::from_millis(1_800_000_000_000),
        event(b"foreign"),
    )
}

/// Appends one local event per payload size, one append each, and returns the
/// positions.
async fn append_sized(store: &MemoryIngestStore, sizes: &[usize]) -> Vec<SequencePosition> {
    let mut positions = Vec::with_capacity(sizes.len());
    for &size in sizes {
        let landed = EventStore::append(store, &[event(&vec![b'x'; size])], None)
            .await
            .expect("an in-memory append does not fail");
        positions.push(landed);
    }
    positions
}

/// Appends `count` one-byte local events.
async fn append_local(store: &MemoryIngestStore, count: usize) -> Vec<SequencePosition> {
    append_sized(store, &vec![1; count]).await
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
    assert_eq!(edge.status().pending, Pending::Exactly(0));
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

/// The read is bounded at one batch plus one, so a backlog longer than that is
/// a lower bound and the last batch is exact. Rejects a pending count that is
/// the batch size or the whole log, and one that claims an exact count it did
/// not read.
#[tokio::test]
async fn pending_is_a_lower_bound_until_the_read_reaches_the_end() {
    let peer = MemorySyncPeer::with_limits(FAR, batches_of(2));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_local(&store, 5).await;
    let mut edge = Edge::new();

    edge.step(&peer, &store).await.expect("a first step");
    let after_one = (edge.status().confirmed, edge.status().pending);
    edge.step(&peer, &store).await.expect("a second step");
    let after_two = (edge.status().confirmed, edge.status().pending);
    edge.step(&peer, &store).await.expect("a third step");
    let after_three = (edge.status().confirmed, edge.status().pending);

    assert_eq!(after_one, (positions.get(1).copied(), Pending::AtLeast(1)));
    assert_eq!(after_two, (positions.get(3).copied(), Pending::AtLeast(1)));
    assert_eq!(
        after_three,
        (positions.get(4).copied(), Pending::Exactly(0))
    );
}

/// The per-event limit ends the run too, though the batch has bytes to spare.
/// Rejects a gather that checks only the batch budget: `MemorySyncPeer` polices
/// only `max_batch_bytes` on a push, so that runner would push all three and
/// hand the far side an event it declared it cannot hold.
#[tokio::test]
async fn the_push_stops_at_the_first_event_over_the_per_event_limit() {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 5;
    limits.max_batch_bytes = 100;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_sized(&store, &[1, 9, 1]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 1);
    assert_eq!(edge.status().confirmed, positions.first().copied());
    assert_eq!(edge.status().pending, Pending::Exactly(2));
}

/// A first event over the per-event limit is offered alone, so the peer's
/// refusal states the class (SY-36); the runner neither skips it nor infers a
/// class. The peer here refuses through its batch budget, the one limit
/// `MemorySyncPeer` polices on a push. Rejects a runner that pushes nothing
/// and reports success, which stalls the edge with no failure anyone sees.
/// It cannot tell whether the event after rides along, because the batch
/// budget would shut that one out anyway: the next test pins that.
#[tokio::test]
async fn a_first_event_over_the_per_event_limit_is_offered_alone_and_refused() {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 5;
    limits.max_batch_bytes = 8;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_sized(&store, &[9, 1]).await;
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;

    assert!(
        matches!(
            refused,
            Err(StepError::Push(
                happenstance_sync::MemoryPeerError::BatchTooLarge { actual: 9, .. }
            ))
        ),
        "the oversized event went alone and was refused: {refused:?}"
    );
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Push, FailureClass::Split))
    );
    assert_eq!(edge.status().confirmed, None);
    assert_eq!(edge.status().pending, Pending::Exactly(2));
}

/// A first event over the per-event limit goes alone even when the batch has
/// room for the next. Rejects a gather that, having taken the oversized
/// event, keeps the run open: it would push the 1-byte event behind it in the
/// same group, past the point the peer can be expected to refuse. The peer
/// accepts the lone event here, since `MemorySyncPeer` does not police
/// `max_event_bytes` on a push.
#[tokio::test]
async fn a_first_event_over_the_per_event_limit_closes_the_run() {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 5;
    limits.max_batch_bytes = 100;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_sized(&store, &[9, 1]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 1, "the oversized event went alone");
    assert_eq!(edge.status().confirmed, positions.first().copied());
    assert_eq!(edge.status().pending, Pending::Exactly(1));
}

/// An event of exactly `max_event_bytes` fits. Rejects a gather that reads
/// the limit as exclusive, which would cut the run after the first event.
#[tokio::test]
async fn an_event_of_exactly_the_per_event_limit_fits() {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 5;
    limits.max_batch_bytes = 100;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_sized(&store, &[5, 5]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 2);
    assert_eq!(edge.status().confirmed, positions.get(1).copied());
    assert_eq!(edge.status().pending, Pending::Exactly(0));
}

/// A run summing to exactly `max_batch_bytes` fits. Rejects a gather that
/// reads the batch budget as exclusive, which would leave the last event out.
#[tokio::test]
async fn a_run_of_exactly_the_batch_budget_fits() {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = 10;
    limits.max_batch_bytes = 10;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_sized(&store, &[4, 6]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 2);
    assert_eq!(edge.status().confirmed, positions.get(1).copied());
    assert_eq!(edge.status().pending, Pending::Exactly(0));
}

/// The byte budget ends the run at the first event that does not fit. Rejects
/// a gather that skips the 9-byte event and pushes the 1-byte one after it:
/// the watermark would then pass an event the far side never received, which
/// is silent loss.
#[tokio::test]
async fn the_push_stops_at_the_first_event_over_the_byte_budget() {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_bytes = 10;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let positions = append_sized(&store, &[4, 9, 1]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 1);
    assert_eq!(edge.status().confirmed, positions.first().copied());
    assert_eq!(edge.status().pending, Pending::Exactly(2));
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
    assert_eq!(edge.status().pending, Pending::Exactly(2));
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Push, FailureClass::Split))
    );
    assert_eq!(edge.status().state, EdgeState::Running);
}

/// SY-38 meets the runner: the far side offers an event under the local
/// store's own identity at a position the store does not hold. The ingest
/// refuses with `Stop`. Rejects a runner that keeps stepping after a stop, one
/// that advances the pull token past a batch it did not ingest, and one that
/// takes `waiting` from a pull whose batch did not land.
#[tokio::test]
async fn a_stop_class_failure_stops_the_edge_and_the_next_step_does_no_io() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.inner
        .seed(EventGroup::new(None, vec![foreign(LOCAL, 9)]))
        .expect("an unpoisoned peer");
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    append_local(&store.inner, 1).await;
    let before = store.inner.snapshot();
    let mut edge = Edge::new();

    let stopped = edge.step(&peer, &store).await;
    let calls_before = peer.calls.get();
    let store_calls_before = store.calls.get();
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
    assert_eq!(calls_before, 1, "the first step pulled once and stopped");
    assert_eq!(peer.calls.get(), 1, "the stopped step made no peer call");
    assert_eq!(
        store_calls_before, 1,
        "the first step ingested once and stopped"
    );
    assert_eq!(
        store.calls.get(),
        1,
        "the stopped step neither read nor ingested"
    );
    assert_eq!(edge.status().state, EdgeState::Stopped(cause));
    assert_eq!(edge.resume_token(), None, "the token did not advance");
    assert_eq!(
        edge.status().waiting,
        Waiting::Unknown,
        "waiting is the pull's only once its batch has landed"
    );
    assert_eq!(edge.status().confirmed, None, "nothing was pushed");
    assert_eq!(store.inner.snapshot(), before, "the local log is unchanged");
}

/// A pull that fails transiently is recorded and changes nothing else.
/// Rejects a runner that moves the token, the watermark or `waiting` on a
/// failed pull, or that stops on a class other than `Stop`.
#[tokio::test]
async fn a_failed_pull_records_its_class_and_moves_nothing() {
    let peer = ProbePeer::new(MemorySyncPeer::with_limits(FAR, batches_of(2)));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 3).await;
    let mut edge = Edge::new();
    edge.step(&peer, &store).await.expect("a first step");
    let token = edge.resume_token().copied();
    let before = edge.status().clone();
    peer.fail_pull.set(Some(FailureClass::Transient));

    let failed = edge.step(&peer, &store).await;

    assert!(
        matches!(failed, Err(StepError::Pull(_))),
        "the pull failed: {failed:?}"
    );
    let status = edge.status();
    assert_eq!(
        status.last_failure,
        Some(Failure::new(Phase::Pull, FailureClass::Transient))
    );
    assert_eq!(status.state, EdgeState::Running);
    assert_eq!(edge.resume_token().copied(), token);
    assert_eq!(status.confirmed, before.confirmed);
    assert_eq!(status.pending, before.pending);
    assert_eq!(status.waiting, before.waiting);
}

/// A local read that fails is recorded against the read phase, with the class
/// the store's error states. Rejects a runner that pushes, or moves the
/// watermark, after a read it could not finish.
#[tokio::test]
async fn a_failed_read_records_its_class_and_pushes_nothing() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    append_local(&store.inner, 2).await;
    let mut edge = Edge::new();
    edge.step(&peer, &store).await.expect("a first step");
    let confirmed = edge.status().confirmed;
    append_local(&store.inner, 1).await;
    store.fail_read.set(true);
    let calls_before = peer.calls.get();

    let failed = edge.step(&peer, &store).await;

    assert!(
        matches!(failed, Err(StepError::Read(_))),
        "the read failed: {failed:?}"
    );
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Read, FailureClass::Transient))
    );
    assert_eq!(edge.status().confirmed, confirmed);
    assert_eq!(peer.calls.get() - calls_before, 1, "a pull, and no push");
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
