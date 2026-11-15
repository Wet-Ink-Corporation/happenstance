//! SY-32 meets the runner: a far side that has forgotten history the edge's
//! token has not reached refuses the pull with class `Reseed`, and the edge
//! stops with [`StopCause::RetentionGap`] rather than retrying or converging
//! on a strict subset in silence (#158).
//!
//! The probe tests inject the class. The reference tests make
//! [`MemorySyncPeer`] compact on a 90-day window and play the vessel that was
//! away for 120 days, and the one away for 60, on a test clock (TST-10).

use core::num::NonZeroU32;
use core::time::Duration;

use happenstance_core::{EventId, RecordedAt, StoreId};
use happenstance_sync::runner::{
    Backoff, Edge, EdgeState, Failure, Phase, RetentionGap, StepError, StopCause,
};
use happenstance_sync::{
    Classify, EventGroup, FailureClass, MemoryIngestStore, MemorySyncPeer, PeerLimits, PushBatch,
    ReplicatedEvent, SyncPeer,
};

use super::probe::{ProbePeer, ProbeStore, Recorded};
use super::{FAR, LOCAL, ORIGIN, event, foreign, ids, position};

/// The shore station whose log the hub relays.
const SHORE: StoreId = StoreId::from_bytes([0x5A; 16]);
const VESSEL_AWAY_120: StoreId = StoreId::from_bytes([0x78; 16]);
const VESSEL_AWAY_60: StoreId = StoreId::from_bytes([0x3C; 16]);
const T0: i64 = 1_800_000_000_000;
const DAY_MS: i64 = 86_400_000;
/// The hub keeps 90 days.
const WINDOW_DAYS: i64 = 90;

fn backoff(attempts: u32) -> Backoff {
    Backoff::new(Duration::from_millis(100), Duration::from_secs(30), 0x5EED)
        .expect("the base is below the cap")
        .with_attempts(NonZeroU32::new(attempts).expect("at least one attempt"))
}

/// A probe whose every pull is refused as `Reseed`, declaring `floor`.
fn refusing(floor: Option<EventId>) -> ProbePeer {
    let probe = ProbePeer::new(MemorySyncPeer::new(FAR));
    probe
        .inner
        .seed(EventGroup::new(None, vec![foreign(ORIGIN, 7)]))
        .expect("an unpoisoned peer");
    let mut limits = PeerLimits::minimum();
    limits.retention_floor = floor;
    *probe.limits.borrow_mut() = Some(limits);
    probe.fail_pull.set(Some(FailureClass::Reseed));
    probe
}

fn floor_id() -> EventId {
    EventId::new(ORIGIN, position(3))
}

/// Rejects a runner that treats `Reseed` as transient (it would stay
/// running), one that reports it as a plain `Stop` with no floor, and one
/// that lands anything from the refused pull.
#[tokio::test]
async fn a_reseed_refusal_stops_the_edge_with_the_peers_floor() {
    let peer = refusing(Some(floor_id()));
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;

    let gap = RetentionGap::new(Phase::Pull, Some(floor_id()));
    assert!(
        matches!(&refused, Err(error) if error.class() == FailureClass::Reseed),
        "the step states the peer's class: {refused:?}"
    );
    assert_eq!(
        edge.status().state,
        EdgeState::Stopped(StopCause::RetentionGap(gap))
    );
    assert_eq!(edge.status().retention_gap(), Some(gap));
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Pull, FailureClass::Reseed))
    );
    assert_eq!(edge.resume_token(), None, "the token did not advance");
    assert!(store.inner.snapshot().is_empty(), "nothing landed");
}

/// Rejects a runner that steps a gapped edge again: the next step does no I/O.
#[tokio::test]
async fn a_gapped_edge_does_no_more_io() {
    let peer = refusing(Some(floor_id()));
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    let mut edge = Edge::new();
    let first = edge.step(&peer, &store).await;
    assert!(first.is_err(), "the first step is refused");
    let calls = peer.calls.get();

    let again = edge.step(&peer, &store).await;

    let gap = RetentionGap::new(Phase::Pull, Some(floor_id()));
    assert!(
        matches!(again, Err(StepError::Stopped(StopCause::RetentionGap(seen))) if seen == gap),
        "a stopped edge refuses: {again:?}"
    );
    assert_eq!(peer.calls.get(), calls, "no peer call");
}

/// Rejects a run loop that waits out a gap as if it were transient: it
/// returns the gap after one step, having slept not at all.
#[tokio::test]
async fn a_run_returns_a_gap_without_waiting() {
    let peer = refusing(Some(floor_id()));
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    let sleep = Recorded::new();
    let mut edge = Edge::new();

    let ran = edge.run(&peer, &store, &mut backoff(10), &sleep).await;

    assert!(
        matches!(&ran, Err(error) if error.class() == FailureClass::Reseed),
        "the run hands the gap back: {ran:?}"
    );
    assert!(
        sleep.slept.borrow().is_empty(),
        "{:?}",
        sleep.slept.borrow()
    );
    assert_eq!(peer.calls.get(), 1, "one pull, no retry");
}

/// A peer that refuses without declaring a floor still reports a gap: the
/// class is the signal, the floor only the report.
#[tokio::test]
async fn a_gap_with_no_declared_floor_is_still_a_gap() {
    let peer = refusing(None);
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;

    assert!(refused.is_err(), "the step is refused");
    assert_eq!(
        edge.status().retention_gap(),
        Some(RetentionGap::new(Phase::Pull, None))
    );
}

/// The cause reads as a `Reseed` failure in its phase, and its text names
/// the floor and the remedy.
#[test]
fn a_gap_is_a_reseed_failure_and_names_its_floor() {
    let gap = RetentionGap::new(Phase::Pull, Some(floor_id()));
    let cause = StopCause::RetentionGap(gap);

    assert_eq!(
        cause.failure(),
        Failure::new(Phase::Pull, FailureClass::Reseed)
    );
    let text = cause.to_string();
    assert!(text.contains(&floor_id().to_string()), "{text}");
    assert!(text.contains("re-seed"), "{text}");
    let unnamed = RetentionGap::new(Phase::Pull, None).to_string();
    assert!(unnamed.contains("named no floor"), "{unnamed}");
}

fn day(day: i64) -> RecordedAt {
    RecordedAt::from_millis(T0 + day * DAY_MS)
}

/// The shore's event at `at`, recorded on `on`.
fn shore(at: u64, on: i64) -> ReplicatedEvent {
    ReplicatedEvent::new(EventId::new(SHORE, position(at)), day(on), event(b"shore"))
}

/// The shore pushes one group per day through the hub, positions `first..`.
async fn shore_writes(hub: &MemorySyncPeer, first: u64, days: &[i64]) {
    for (at, &on) in (first..).zip(days) {
        let group = EventGroup::new(None, vec![shore(at, on)]);
        hub.push(&PushBatch::new(vec![group]))
            .await
            .expect("an in-memory push");
    }
}

/// Runs `edge` between `vessel` and `hub` to the end.
async fn sync(
    edge: &mut Edge<happenstance_sync::MemoryResume>,
    hub: &MemorySyncPeer,
    vessel: &MemoryIngestStore,
) -> Result<(), String> {
    edge.run(hub, vessel, &mut backoff(3), &Recorded::new())
        .await
        .map_err(|error| error.to_string())
}

/// The hub on day 130, compacted on its 90-day window, and the two vessels:
/// one last synced on day 10 (away 120 days), one on day 70 (away 60).
struct Voyage {
    hub: MemorySyncPeer,
    away_120: (MemoryIngestStore, Edge<happenstance_sync::MemoryResume>),
    away_60: (MemoryIngestStore, Edge<happenstance_sync::MemoryResume>),
}

async fn voyage() -> Voyage {
    let hub = MemorySyncPeer::new(FAR);
    let away_120 = MemoryIngestStore::with_store_id(VESSEL_AWAY_120);
    let away_60 = MemoryIngestStore::with_store_id(VESSEL_AWAY_60);
    let mut edge_120 = Edge::new();
    let mut edge_60 = Edge::new();

    shore_writes(&hub, 1, &[0, 10]).await;
    sync(&mut edge_120, &hub, &away_120).await.expect("day 10");
    sync(&mut edge_60, &hub, &away_60).await.expect("day 10");
    shore_writes(&hub, 3, &[20, 50, 70]).await;
    sync(&mut edge_60, &hub, &away_60).await.expect("day 70");
    shore_writes(&hub, 6, &[100, 125]).await;
    let floor = hub
        .compact_recorded_before(day(130 - WINDOW_DAYS))
        .expect("an unpoisoned hub");
    assert_eq!(floor, Some(shore_id(3)), "days 0, 10 and 20 are forgotten");

    Voyage {
        hub,
        away_120: (away_120, edge_120),
        away_60: (away_60, edge_60),
    }
}

fn shore_id(at: u64) -> EventId {
    EventId::new(SHORE, position(at))
}

/// E2E-44: either it converges, or it is told it cannot. The vessel away
/// 120 days against a 90-day window is stopped with the gap and its log is
/// untouched; the vessel away 60 days converges on the tail. Rejects a
/// runner that ingests the retained subset from a stale token, and one that
/// retries a gap.
#[tokio::test]
async fn a_vessel_away_past_the_window_is_told_and_one_inside_it_converges() {
    let Voyage {
        hub,
        away_120: (vessel_120, mut edge_120),
        away_60: (vessel_60, mut edge_60),
    } = voyage().await;
    let before = vessel_120.snapshot();

    let refused = sync(&mut edge_120, &hub, &vessel_120).await;
    let converged = sync(&mut edge_60, &hub, &vessel_60).await;

    refused.expect_err("the vessel away 120 days is told it cannot converge");
    assert_eq!(
        edge_120.status().retention_gap(),
        Some(RetentionGap::new(Phase::Pull, Some(shore_id(3))))
    );
    assert_eq!(vessel_120.snapshot(), before, "nothing landed");

    converged.expect("the vessel inside the window converges");
    assert!(edge_60.status().is_synced(), "{:?}", edge_60.status());
    assert_eq!(
        ids(&vessel_60.snapshot()),
        (1..=7).map(shore_id).collect::<Vec<_>>(),
        "everything up to day 70, then the tail it missed"
    );
}
