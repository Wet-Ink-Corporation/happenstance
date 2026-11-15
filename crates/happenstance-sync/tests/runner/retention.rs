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

use happenstance_core::EventId;
use happenstance_sync::runner::{
    Backoff, Edge, EdgeState, Failure, Phase, RetentionGap, StepError, StopCause,
};
use happenstance_sync::{
    Classify, EventGroup, FailureClass, MemoryIngestStore, MemorySyncPeer, PeerLimits,
};

use super::probe::{ProbePeer, ProbeStore, Recorded};
use super::{FAR, LOCAL, ORIGIN, foreign, position};

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
