//! `Edge::run`: the caller-owned loop that steps until synced or halted,
//! waiting out transient failures through the caller's sleep (#244 part 2).
//!
//! The sleep is `probe::Recorded`, which returns at once and keeps every
//! duration, so the waits are asserted rather than taken (TST-10). Its hook
//! changes the far side "while the run waits".

use core::num::NonZeroU32;
use core::time::Duration;

use happenstance_core::StoreId;
use happenstance_sync::runner::{
    Backoff, Edge, EdgeState, IncarnationChanged, Phase, StepError, StopCause,
};
use happenstance_sync::{FailureClass, MemoryIngestStore, MemorySyncPeer, PeerLimits};

use super::probe::{ProbePeer, Recorded};
use super::{FAR, LOCAL, append_local, batches_of};

const BEFORE: StoreId = StoreId::from_bytes([0xB0; 16]);
const AFTER: StoreId = StoreId::from_bytes([0xAF; 16]);

fn backoff(attempts: u32) -> Backoff {
    Backoff::new(Duration::from_millis(100), Duration::from_secs(30), 0x5EED)
        .expect("the base is below the cap")
        .with_attempts(NonZeroU32::new(attempts).expect("at least one attempt"))
}

/// The delays a fresh copy of `backoff(attempts)` draws, `count` of them.
fn drawn(attempts: u32, count: usize) -> Vec<Duration> {
    let mut copy = backoff(attempts);
    core::iter::from_fn(|| copy.next_delay())
        .take(count)
        .collect()
}

/// Rejects a run that returns after one step with a backlog left, and one that
/// waits when nothing failed.
#[tokio::test]
async fn a_run_steps_until_synced_without_waiting() {
    let peer = MemorySyncPeer::with_limits(FAR, batches_of(2));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 5).await;
    let sleep = Recorded::new();
    let mut edge = Edge::new();

    edge.run(&peer, &store, &mut backoff(3), &sleep)
        .await
        .expect("an in-memory run");

    assert!(edge.status().is_synced(), "{:?}", edge.status());
    assert!(sleep.slept.borrow().is_empty());
}

/// A transient pull failure is waited out, delay by delay, through the
/// caller's sleep, and the run then syncs. Rejects a run that retries without
/// waiting (a hot loop against a down peer), one that waits a fixed interval,
/// and one that gives up on the first transient failure.
#[tokio::test]
async fn a_transient_failure_is_retried_after_each_backoff_delay() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.fail_pull.set(Some(FailureClass::Transient));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 1).await;
    let sleep = Recorded::then(|count| {
        if count == 3 {
            peer.fail_pull.set(None);
        }
    });
    let mut edge = Edge::new();

    edge.run(&peer, &store, &mut backoff(5), &sleep)
        .await
        .expect("the peer comes back");

    assert_eq!(
        *sleep.slept.borrow(),
        drawn(5, 3),
        "the backoff's own delays"
    );
    assert!(edge.status().is_synced());
    assert_eq!(
        edge.status().last_failure.map(|failure| failure.class),
        Some(FailureClass::Transient)
    );
}

/// Once the backoff is spent the run returns the last failure, with the edge
/// still running. Rejects a run with no attempt cap, which never returns while
/// the peer is down, and one that stops the edge on a count of transient
/// failures, which the agreed design leaves to the incarnation check.
#[tokio::test]
async fn a_spent_backoff_returns_the_failure_and_leaves_the_edge_running() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.fail_pull.set(Some(FailureClass::Transient));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    let sleep = Recorded::new();
    let mut edge = Edge::new();

    let spent = edge.run(&peer, &store, &mut backoff(3), &sleep).await;

    assert!(matches!(spent, Err(StepError::Pull(_))), "{spent:?}");
    assert_eq!(
        sleep.slept.borrow().len(),
        3,
        "three waits, then it returned"
    );
    assert_eq!(peer.calls.get(), 4, "the first try and three retries");
    assert_eq!(edge.status().state, EdgeState::Running);
}

/// The Durable Object reset, as agreed on 2026-11-08: the far side fails
/// transiently while it is down, and comes back under a new `StoreId`. The run
/// waits out the outage, then stops at the first answer from the new store,
/// with the local log as it was. Rejects a run that loops for ever on the
/// reset, one that resumes against the re-minted store, and one that wipes or
/// rewinds the local log to match it.
#[tokio::test]
async fn a_reset_far_side_back_under_a_new_store_stops_the_run_and_keeps_the_log() {
    let peer = ProbePeer::answering_as(MemorySyncPeer::new(FAR), BEFORE);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 2).await;
    let mut edge = Edge::new();
    edge.run(&peer, &store, &mut backoff(5), &Recorded::new())
        .await
        .expect("a first run syncs");
    append_local(&store, 1).await;
    let log = store.snapshot();
    let confirmed = edge.status().confirmed;
    peer.fail_pull.set(Some(FailureClass::Transient));
    let sleep = Recorded::then(|count| {
        if count == 2 {
            peer.fail_pull.set(None);
            peer.answer_as(Some(AFTER));
        }
    });

    let stopped = edge.run(&peer, &store, &mut backoff(5), &sleep).await;

    let change = IncarnationChanged::new(Phase::Pull, BEFORE, AFTER);
    assert!(
        matches!(stopped, Err(StepError::Incarnation(seen)) if seen == change),
        "{stopped:?}"
    );
    assert_eq!(sleep.slept.borrow().len(), 2, "the outage was waited out");
    assert_eq!(
        edge.status().state,
        EdgeState::Stopped(StopCause::Incarnation(change))
    );
    assert_eq!(store.snapshot(), log, "the local log is unchanged");
    assert_eq!(edge.status().confirmed, confirmed);
}

/// A split the edge can halve is retried at once, without a wait, and the run
/// still syncs: the peer declared more than it enforces. Rejects a run that
/// backs off on a split, and one that returns on it.
#[tokio::test]
async fn a_halvable_split_is_retried_at_once_and_the_run_syncs() {
    let mut enforced = PeerLimits::minimum();
    enforced.max_batch_bytes = 2;
    let peer = ProbePeer::new(MemorySyncPeer::with_limits(FAR, enforced));
    peer.limits.replace(Some(PeerLimits::minimum()));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&store, 4).await;
    let sleep = Recorded::new();
    let mut edge = Edge::new();

    edge.run(&peer, &store, &mut backoff(3), &sleep)
        .await
        .expect("the halved offer lands");

    assert_eq!(*peer.offered.borrow(), vec![4, 2, 2]);
    assert!(sleep.slept.borrow().is_empty(), "a split is not waited out");
    assert!(edge.status().is_synced());
}
