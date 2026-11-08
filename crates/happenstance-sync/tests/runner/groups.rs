//! The push keeps the local log's write groups (#420, SY-30): a multi-event
//! append never splits across two pushes, independent appends never merge into
//! one atomic unit, and a relayed foreign group keeps its boundary and guard.
//!
//! The far side is a `MemorySyncPeer`, which stores each pushed group as it
//! arrived, so pulling it back shows exactly the units the runner sent.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, Event, EventStore, Query, QueryItem};
use happenstance_sync::runner::{Edge, Pending, StepError};
use happenstance_sync::{
    EventGroup, MemoryIngestStore, MemoryPeerError, MemorySyncPeer, PeerLimits, SyncPeer,
    WriteGroup, WriteGroupStore,
};

use super::{FAR, LOCAL, ORIGIN, batches_of, event, foreign};

/// The most steps a test takes before it decides the edge will never sync
/// (H-13): far above what any arrangement here needs.
const MAX_STEPS: usize = 64;

/// Appends one group per entry of `sizes`, each of that many one-byte events.
async fn append_groups(store: &MemoryIngestStore, sizes: &[usize]) {
    for &size in sizes {
        let events: Vec<Event> = (0..size).map(|_| event(b"x")).collect();
        EventStore::append(store, &events, None)
            .await
            .expect("an in-memory append does not fail");
    }
}

/// Every write group the store holds, read in one page.
async fn local_groups(store: &MemoryIngestStore) -> Vec<WriteGroup> {
    store
        .read_groups(None, NonZeroUsize::MAX)
        .await
        .expect("the reference store's scan is in group order")
        .groups
}

/// Every group the far side holds, pulled until it hands back an empty batch.
async fn held_groups(peer: &MemorySyncPeer) -> Vec<EventGroup> {
    let mut held = Vec::new();
    let mut from = None;
    for _ in 0..MAX_STEPS {
        let pulled = peer.pull(from.as_ref()).await.expect("an in-memory pull");
        if pulled.batch.groups.is_empty() {
            return held;
        }
        held.extend(pulled.batch.groups);
        from = Some(pulled.resume);
    }
    panic!("the far side was still handing back groups after {MAX_STEPS} pulls");
}

/// Steps `edge` until it reports synced, and fails the test if it never does.
async fn step_until_synced(
    edge: &mut Edge<happenstance_sync::MemoryResume>,
    peer: &MemorySyncPeer,
    store: &MemoryIngestStore,
) {
    for _ in 0..MAX_STEPS {
        edge.step(peer, store).await.expect("an in-memory step");
        if edge.status().is_synced() {
            return;
        }
    }
    panic!(
        "the edge did not sync in {MAX_STEPS} steps: {:?}",
        edge.status()
    );
}

/// `None` when the far side holds exactly the local write groups, one for one
/// and each whole with its guard; otherwise what differs. I-7.
fn torn(local: &[WriteGroup], held: &[EventGroup]) -> Option<String> {
    let expected: Vec<EventGroup> = local.iter().cloned().map(EventGroup::from).collect();
    if expected == held {
        return None;
    }
    let shape = |groups: &[EventGroup]| -> Vec<usize> {
        groups.iter().map(|group| group.events.len()).collect()
    };
    Some(format!(
        "the far side holds groups of {:?} events where the local log wrote {:?}",
        shape(held),
        shape(&expected)
    ))
}

/// What the pre-#420 runner pushed: the unconfirmed tail cut at
/// `max_batch_events`, each cut sent as one unguarded group.
fn per_event_push(local: &[WriteGroup], max_batch_events: usize) -> Vec<EventGroup> {
    let events: Vec<_> = local
        .iter()
        .flat_map(|group| group.events().iter().cloned())
        .collect();
    events
        .chunks(max_batch_events)
        .map(|chunk| EventGroup::new(None, chunk.to_vec()))
        .collect()
}

fn guard() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Written"]).expect("a valid item"),
    ))
}

/// Rejects the per-batch push: appends of 2, 3 and 2 events under a batch of
/// four would go as `[2 + 2 of 3]` and `[1 of 3 + 2]`, and between the two the
/// far side holds two thirds of a decision the origin made whole.
#[tokio::test]
async fn a_multi_event_append_is_never_split_across_pushes() {
    let peer = MemorySyncPeer::with_limits(FAR, batches_of(4));
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_groups(&store, &[2, 3, 2]).await;
    let mut edge = Edge::new();

    step_until_synced(&mut edge, &peer, &store).await;

    let local = local_groups(&store).await;
    assert_eq!(torn(&local, &held_groups(&peer).await), None);
    assert_eq!(
        edge.status().confirmed,
        local.last().map(WriteGroup::first),
        "the watermark is the first position of the last group pushed"
    );
}

/// The check above is not decorative: the batches the pre-#420 runner built
/// fail it. A rule no implementation can fail proves nothing.
#[tokio::test]
async fn the_tear_check_rejects_a_runner_that_cuts_mid_append() {
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_groups(&store, &[2, 3, 2]).await;
    let local = local_groups(&store).await;

    let held = per_event_push(&local, 4);

    assert!(torn(&local, &held).is_some(), "{held:?}");
    assert_eq!(
        torn(
            &local,
            &local
                .iter()
                .cloned()
                .map(EventGroup::from)
                .collect::<Vec<_>>()
        ),
        None
    );
}

/// Rejects a push that merges whatever fits into one atomic unit: three
/// independent appends are three groups on the far side, not one.
#[tokio::test]
async fn independent_appends_are_not_merged() {
    let peer = MemorySyncPeer::new(FAR);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_groups(&store, &[1, 1, 1]).await;
    let mut edge = Edge::new();

    let stepped = edge.step(&peer, &store).await.expect("an in-memory step");

    assert_eq!(stepped.pushed, 3);
    let held = held_groups(&peer).await;
    assert_eq!(held.len(), 3, "{held:?}");
    assert_eq!(torn(&local_groups(&store).await, &held), None);
}

/// SY-10's relay: a group pulled from one peer and pushed to another keeps its
/// boundary and its guard. Rejects a runner that drops the guard on ingest, and
/// one that merges or flattens what it relays.
#[tokio::test]
async fn a_relayed_foreign_group_keeps_its_boundary_and_guard() {
    let upstream = MemorySyncPeer::new(FAR);
    let guarded = EventGroup::new(Some(guard()), vec![foreign(ORIGIN, 1), foreign(ORIGIN, 2)]);
    let bare = EventGroup::new(None, vec![foreign(ORIGIN, 3)]);
    upstream.seed(guarded.clone()).expect("an unpoisoned peer");
    upstream.seed(bare.clone()).expect("an unpoisoned peer");
    let relay = MemoryIngestStore::with_store_id(LOCAL);
    let downstream = MemorySyncPeer::new(happenstance_core::StoreId::from_bytes([0xD0; 16]));

    step_until_synced(&mut Edge::new(), &upstream, &relay).await;
    step_until_synced(&mut Edge::new(), &downstream, &relay).await;

    assert_eq!(held_groups(&downstream).await, vec![guarded, bare]);
}

/// A group larger than the far side's batch is offered alone, so the peer's
/// refusal states the class; it is neither skipped nor split. The refusal is a
/// `Split` of one group, which the runner cannot halve, so the edge parks and
/// the next step offers nothing (#244 part 2). `MemorySyncPeer` polices
/// `max_batch_bytes`.
#[tokio::test]
async fn a_group_over_the_batch_limit_is_offered_alone_and_refused() {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_bytes = 2;
    let peer = MemorySyncPeer::with_limits(FAR, limits);
    let store = MemoryIngestStore::with_store_id(LOCAL);
    append_groups(&store, &[3]).await;
    let mut edge = Edge::new();

    let refused = edge.step(&peer, &store).await;
    let parked = edge.step(&peer, &store).await;

    assert!(
        matches!(
            refused,
            Err(StepError::Push(MemoryPeerError::BatchTooLarge {
                actual: 3,
                ..
            }))
        ),
        "the whole group went alone and was refused: {refused:?}"
    );
    assert!(
        matches!(parked, Err(StepError::Parked(_))),
        "the parked edge offered nothing more: {parked:?}"
    );
    assert_eq!(edge.status().confirmed, None);
    assert_eq!(edge.status().pending, Pending::Exactly(3));
    assert!(
        held_groups(&peer).await.is_empty(),
        "nothing was split off and landed"
    );
}

/// I-7 over every arrangement of five appends of one to four events, under
/// batches of four to six: a deterministic table rather than a generator, so
/// the property holds for each without a new dependency (TST-06).
#[tokio::test]
async fn a_sweep_never_tears_a_group() {
    for max_batch_events in 4..=6 {
        for code in 0..4_usize.pow(5) {
            let sizes: Vec<usize> = (0..5)
                .map(|digit| (code / 4_usize.pow(digit)) % 4 + 1)
                .collect();
            let peer = MemorySyncPeer::with_limits(FAR, batches_of(max_batch_events));
            let store = MemoryIngestStore::with_store_id(LOCAL);
            append_groups(&store, &sizes).await;

            step_until_synced(&mut Edge::new(), &peer, &store).await;

            assert_eq!(
                torn(&local_groups(&store).await, &held_groups(&peer).await),
                None,
                "appends of {sizes:?} under batches of {max_batch_events}"
            );
        }
    }
}
