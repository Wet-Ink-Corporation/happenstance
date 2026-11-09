//! The runner pushes a SQLite log through a real peer in the writes that made
//! it (SY-30, #143): each local append and each ingested group arrives whole,
//! with its guard, however the peer's batch limit cuts the pushes.
//!
//! `happenstance-sync`'s own `tests/runner/groups.rs` holds the runner to this
//! against `MemorySyncPeer`. Here the near end is a real store that records
//! write groups (SQLite) and the far end is one of the two SQL-backed peers, so
//! the groups cross a real wire and a real far-side ingest and come back on a
//! real pull. The Durable Object's case is `harness/workerd`'s.

use core::num::NonZeroUsize;

use happenstance_core::{
    AppendCondition, Event, EventId, EventStore, Query, QueryItem, RecordedAt, SequencePosition,
    StoreId,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sync::runner::Edge;
use happenstance_sync::{
    EventGroup, IngestGroup, IngestStore, PeerLimits, ReplicatedEvent, SyncPeer, WriteGroup,
    WriteGroupStore,
};

/// The most steps an edge takes before the test decides it will never sync
/// (H-13): far above the handful of pushes these groups need.
const MAX_STEPS: usize = 32;

/// The batch limit the far side declares: below the log's eight events, and
/// below two of its groups together, so a push that cut by event count would
/// cut a group.
const BATCH_EVENTS: usize = 4;

/// The foreign group's origin.
const ORIGIN: StoreId = StoreId::from_bytes([0x30; 16]);

/// `PeerLimits::minimum`, with a batch of at most [`BATCH_EVENTS`] events.
pub(super) fn small_batches() -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = BATCH_EVENTS;
    limits
}

fn written(n: usize) -> Event {
    Event::new("Written", format!("{{\"n\":{n}}}").into_bytes()).expect("a valid type")
}

fn guard() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Contested"]).expect("a valid item"),
    ))
}

fn foreign(at: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(
            ORIGIN,
            SequencePosition::new(at).expect("a test position is not zero"),
        ),
        RecordedAt::from_millis(1_800_000_000_000),
        Event::new("Relayed", at.to_be_bytes().to_vec()).expect("a valid type"),
    )
}

/// A SQLite log of four writes: appends of 2 and 3 events, a foreign group of
/// two ingested under a guard, and an append of 1.
async fn grouped_log() -> SqliteEventStore {
    let near = SqliteEventStore::open_in_memory().expect("an in-memory SQLite store opens");
    let append = async |events: &[Event]| {
        near.append(events, None)
            .await
            .expect("the near store's own append lands");
    };
    append(&[written(1), written(2)]).await;
    append(&[written(3), written(4), written(5)]).await;
    let relayed = [foreign(7), foreign(9)];
    let guard = guard();
    near.ingest(&[IngestGroup::new(&relayed, &[]).with_guard(Some(&guard))])
        .await
        .expect("a well-formed ingest into the near store lands");
    append(&[written(6)]).await;
    near
}

/// Every write group `near` holds, as a push carries them.
async fn local_groups(near: &SqliteEventStore) -> Vec<EventGroup> {
    let page = near
        .read_groups(None, NonZeroUsize::new(64).expect("non-zero"))
        .await
        .expect("the near store's groups read");
    page.groups.into_iter().map(EventGroup::from).collect()
}

/// `None` when `held` is exactly `local`, group for group, each whole with its
/// guard; otherwise what differs, by group size and guard.
fn torn(local: &[EventGroup], held: &[EventGroup]) -> Option<String> {
    if local == held {
        return None;
    }
    let shape = |groups: &[EventGroup]| -> Vec<(usize, bool)> {
        groups
            .iter()
            .map(|group| (group.events.len(), group.guard.is_some()))
            .collect()
    };
    Some(format!(
        "the far side holds groups of (events, guarded) {:?} where the near store wrote {:?}",
        shape(held),
        shape(local)
    ))
}

/// What the pre-#420 runner pushed: the unconfirmed log cut at the batch
/// limit, each cut one unguarded group.
fn per_event_push(local: &[EventGroup], max_batch_events: usize) -> Vec<EventGroup> {
    let events: Vec<ReplicatedEvent> = local
        .iter()
        .flat_map(|group| group.events.iter().cloned())
        .collect();
    events
        .chunks(max_batch_events)
        .map(|chunk| EventGroup::new(None, chunk.to_vec()))
        .collect()
}

/// Drives an edge from [`grouped_log`] to `peer` until it is synced, then
/// pulls the far side whole: it must hold the near store's write groups, each
/// whole with its guard.
///
/// `peer` must declare [`small_batches`], so the edge needs several pushes.
pub(super) async fn runner_keeps_write_groups<P>(peer: &P)
where
    P: SyncPeer,
    P::Error: core::fmt::Debug,
{
    assert_eq!(
        peer.limits().max_batch_events,
        BATCH_EVENTS,
        "the test's own arrangement"
    );
    let near = grouped_log().await;
    let mut edge = Edge::new();
    let mut pushes = 0_usize;
    let mut synced = false;
    for _ in 0..MAX_STEPS {
        let stepped = edge.step(peer, &near).await.expect("a step");
        pushes += usize::from(stepped.pushed > 0);
        if edge.status().is_synced() {
            synced = true;
            break;
        }
    }
    assert!(synced, "the edge never synced: {:?}", edge.status());

    let local = local_groups(&near).await;
    let held = super::pull_all(peer).await.groups;

    assert!(
        pushes >= 2,
        "the batch limit split the log over pushes: {pushes}"
    );
    assert_eq!(torn(&local, &held), None);
    assert_eq!(
        edge.status().confirmed,
        local_groups_first(&near).await.last().copied(),
        "the watermark is the first position of the last group pushed"
    );
}

/// The first position of each of `near`'s write groups.
async fn local_groups_first(near: &SqliteEventStore) -> Vec<SequencePosition> {
    near.read_groups(None, NonZeroUsize::new(64).expect("non-zero"))
        .await
        .expect("the near store's groups read")
        .groups
        .iter()
        .map(WriteGroup::first)
        .collect()
}

/// The check above is not decorative: the batches the pre-#420 runner built
/// from this very log fail it, and the log's own groups pass it. Not live, so
/// it runs in the gate.
#[tokio::test]
async fn the_group_check_rejects_a_push_cut_by_event_count() {
    let near = grouped_log().await;
    let local = local_groups(&near).await;

    let cut = per_event_push(&local, BATCH_EVENTS);

    assert_eq!(
        local
            .iter()
            .map(|group| (group.events.len(), group.guard.is_some()))
            .collect::<Vec<_>>(),
        [(2, false), (3, false), (2, true), (1, false)],
        "the near store recorded four writes, the ingested one with its guard"
    );
    assert!(torn(&local, &cut).is_some(), "{cut:?}");
    assert_eq!(torn(&local, &local), None);
}
