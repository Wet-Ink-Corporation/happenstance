//! A SQLite store pruned outside the port and then synced again (#483, #484,
//! SY-40, ADR-0110): `experiments/retention-reader`'s three R2-dst scenarios
//! (`tests/it/r2_ingest_destination.rs`), ported as regression tests and now
//! asserting the fixed behaviour rather than the hazard, with the control
//! kept beside them.
//!
//! The prune is a raw `DELETE` on a connection the store never saw, which is
//! how both bug reports removed rows. Every write group here holds one event,
//! as in the experiment. A projection that counts how often it applied each
//! event stands beside the spoke, so "applies nothing twice" is read off the
//! projection, not inferred from an ingest count. No literal local position is
//! asserted: positions are the ones the store assigned, read back from it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, Event, EventId, EventStore, MemoryProjectionStore, ProjectionId,
    ProjectionStore, Query, ReadOptions, RecordedAt, SequencePosition, StoreId, collect,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::happenstance_sync::runner::{Edge, EdgeState, Stepped, Watermark};
use happenstance_sqlite::happenstance_sync::{
    EventGroup, IngestStore, MemoryIngestStore, MemorySyncPeer, ReplicatedEvent, StoreSyncPeer,
    SyncPeer, WriteGroupStore,
};

/// The far side a spoke syncs with.
const HUB: StoreId = StoreId::from_bytes([2; 16]);
/// A third store, the origin of every foreign event the hub serves.
const ORIGIN: StoreId = StoreId::from_bytes([3; 16]);
/// The spoke, when the spoke is the in-memory oracle and the hub is SQLite.
const SPOKE: StoreId = StoreId::from_bytes([4; 16]);

/// The most steps any scenario takes before it decides the edge will never
/// sync (H-13): far above what any arrangement here needs.
const MAX_STEPS: usize = 16;

/// A temporary database path, deleted with its WAL sidecars on drop.
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-pruned-{label}-{}-{}.db",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let db = Self(path);
        db.remove();
        db
    }

    fn path(&self) -> &Path {
        &self.0
    }

    /// Deletes the file and its sidecars where they exist. A failure panics,
    /// except while a test is already unwinding.
    fn remove(&self) {
        for suffix in ["", "-wal", "-shm"] {
            // An owned path per sidecar: `push` needs one to extend.
            let mut path = self.0.clone().into_os_string();
            path.push(suffix);
            let path = PathBuf::from(path);
            if path.exists()
                && let Err(error) = std::fs::remove_file(&path)
                && !std::thread::panicking()
            {
                panic!("could not delete {}: {error}", path.display());
            }
        }
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        self.remove();
    }
}

/// Deletes the events at `positions` outside the port, as an operator's
/// retention script does: tag rows and the tag statistic first, then the
/// event rows, in one transaction on a fresh connection.
fn prune_outside_port(path: &Path, positions: &[SequencePosition]) {
    let mut operator = rusqlite::Connection::open(path).expect("the file opens");
    operator
        .pragma_update(None, "foreign_keys", "ON")
        .expect("the operator enforces foreign keys, as the store does");
    let transaction = operator
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .expect("the prune takes the write lock");
    for position in positions {
        let position = i64::try_from(position.get()).expect("a SQLite position fits i64");
        transaction
            .execute(
                "UPDATE tag_cardinality SET events = events - 1 \
                 WHERE tag IN (SELECT tag FROM event_tag WHERE position = ?1)",
                [position],
            )
            .expect("the tag statistic is decremented");
        transaction
            .execute("DELETE FROM event_tag WHERE position = ?1", [position])
            .expect("the tag rows go");
        let deleted = transaction
            .execute("DELETE FROM event WHERE position = ?1", [position])
            .expect("the event row goes");
        assert_eq!(deleted, 1, "position {position} was held");
    }
    transaction.commit().expect("the prune commits");
}

/// A plain event of `kind`.
fn event(kind: &str) -> Event {
    Event::new(kind, &b"{}"[..]).expect("a valid event type")
}

/// `count` events as `ORIGIN` minted them, at its first positions.
fn minted(count: u64) -> Vec<ReplicatedEvent> {
    (1..=count)
        .map(|at| {
            let position = SequencePosition::new(at).expect("from one, never zero");
            ReplicatedEvent::new(
                EventId::new(ORIGIN, position),
                RecordedAt::from_millis(1_700_000_000_000),
                event("Shipped"),
            )
        })
        .collect()
}

/// A hub holding `events`, one group each, its own copy of every one.
fn seeded(events: &[ReplicatedEvent]) -> MemorySyncPeer {
    let hub = MemorySyncPeer::new(HUB);
    for shipped in events {
        // The hub stores its own copy, as a real one holds its own row.
        hub.seed(EventGroup::new(None, vec![shipped.clone()]))
            .expect("an unpoisoned peer");
    }
    hub
}

/// Steps `edge` until it reports synced, summing what the steps did, and
/// fails the test if a step fails or the edge never syncs.
async fn sync<P, S>(edge: &mut Edge<P::Resume>, peer: &P, store: &S) -> Stepped
where
    P: SyncPeer,
    S: WriteGroupStore + IngestStore,
    P::Error: core::fmt::Debug,
    <S as WriteGroupStore>::Error: core::fmt::Debug,
    <S as IngestStore>::Error: core::fmt::Debug,
{
    let mut total = Stepped::default();
    for _ in 0..MAX_STEPS {
        let stepped = edge.step(peer, store).await.expect("a step succeeds");
        total.ingested.appended += stepped.ingested.appended;
        total.ingested.skipped += stepped.ingested.skipped;
        total.pushed += stepped.pushed;
        if edge.status().is_synced() {
            return total;
        }
    }
    panic!(
        "the edge did not sync in {MAX_STEPS} steps: {:?}",
        edge.status()
    );
}

/// The edge a restart builds from `edge`'s confirmed position, as an
/// application that kept the watermark beside its views would.
fn restored<R>(edge: &Edge<R>) -> Edge<R> {
    let confirmed = edge.status().confirmed.expect("the push was acknowledged");
    Edge::restored(Watermark::new(HUB, confirmed))
}

/// A projection that counts how many times it applied each event, resuming
/// strictly after its checkpoint as a runner does.
struct Counter {
    id: ProjectionId,
    applied: BTreeMap<EventId, u64>,
}

impl Counter {
    fn new() -> Self {
        Self {
            id: ProjectionId::new("applied").expect("a plain id is valid"),
            applied: BTreeMap::new(),
        }
    }

    /// Applies everything above the checkpoint and commits the checkpoint at
    /// the last event applied. Returns how many events it applied.
    async fn run(&mut self, store: &SqliteEventStore, models: &MemoryProjectionStore) -> usize {
        let options = match models
            .checkpoint(&self.id)
            .await
            .expect("an in-memory read")
        {
            Checkpoint::NeverRun => ReadOptions::new(),
            Checkpoint::Live { through } => match through.next() {
                Some(next) => ReadOptions::new().from(next),
                None => return 0,
            },
            other => panic!("this projection never rebuilds: {other:?}"),
        };
        let events = collect(store.read(&Query::all(), options))
            .await
            .expect("the log reads");
        let Some(last) = events.last() else {
            return 0;
        };
        for applied in &events {
            *self.applied.entry(applied.id).or_default() += 1;
        }
        models
            .commit(models.open(), &self.id, last.position, Authority::Live)
            .await
            .expect("an in-memory commit lands");
        events.len()
    }

    /// Every event this projection applied more than once.
    fn applied_twice(&self) -> Vec<EventId> {
        self.applied
            .iter()
            .filter(|&(_, &times)| times > 1)
            .map(|(&id, _)| id)
            .collect()
    }
}

/// The local positions of the events in `store` carrying `ids`.
async fn positions_of(store: &SqliteEventStore, ids: &[EventId]) -> Vec<SequencePosition> {
    collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads")
        .iter()
        .filter(|held| ids.contains(&held.id))
        .map(|held| held.position)
        .collect()
}

/// R2-dst-1, fixed (#483). A spoke prunes two foreign events it has
/// projected, the origin's highest among them, and its edge restarts by the
/// documented path, `Edge::restored`, which pulls the hub's log from the
/// start. Both pruned events are skipped as recorded removals: nothing lands,
/// the projection applies nothing twice, and the edge keeps running.
///
/// The spoke's watermark for the origin stays where the prune left it, below
/// what it was, because it is "the highest origin position held" (SY-31). The
/// next restored pull re-reads that tail as skips: a round trip, not a
/// duplicate.
#[tokio::test]
async fn a_restored_edge_does_not_resurrect_pruned_foreign_events() {
    let db = TempDb::new("resurrection");
    let shipped = minted(4);
    let hub = seeded(&shipped);
    let models = MemoryProjectionStore::new();
    let mut counter = Counter::new();
    let spoke = SqliteEventStore::open(db.path()).expect("a fresh file opens");
    let mut edge = Edge::new();
    sync(&mut edge, &hub, &spoke).await;
    assert_eq!(counter.run(&spoke, &models).await, shipped.len());
    let mark_before = spoke.watermark().await.expect("a read").get(ORIGIN);
    let pruned: Vec<EventId> = [1, 3]
        .iter()
        .filter_map(|&at| shipped.get(at))
        .map(|event| event.id)
        .collect();
    let pruned_at = positions_of(&spoke, &pruned).await;
    assert_eq!(pruned_at.len(), 2);
    drop(spoke);

    prune_outside_port(db.path(), &pruned_at);
    let spoke = SqliteEventStore::open(db.path()).expect("the pruned file opens");
    let pruned_log = collect(spoke.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads");
    let mut again = restored(&edge);
    let stepped = sync(&mut again, &hub, &spoke).await;
    let second_run = counter.run(&spoke, &models).await;

    assert_eq!(
        stepped.ingested.appended, 0,
        "a pruned event was re-admitted"
    );
    assert_eq!(
        stepped.ingested.skipped,
        shipped.len(),
        "the re-pull of the whole log was skipped, pruned events included: {stepped:?}"
    );
    assert_eq!(second_run, 0, "the projection had nothing new to apply");
    assert_eq!(counter.applied_twice(), Vec::<EventId>::new());
    for &id in &pruned {
        assert!(!spoke.contains_event_id(id).await.expect("a read"));
    }
    assert_eq!(
        collect(spoke.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the log reads"),
        pruned_log,
        "the log is the pruned log, unchanged"
    );
    assert!(
        spoke.watermark().await.expect("a read").get(ORIGIN) < mark_before,
        "the watermark counts only what is held"
    );
    assert_eq!(again.status().state, EdgeState::Running);
}

/// R2-dst-2, fixed (#484). A spoke prunes one of its own events after the
/// hub confirmed it. The restored edge pulls it back from the hub with a
/// foreign event the hub learned of meanwhile. The pruned own event is a
/// skip, not SY-38's refusal: the foreign event lands, the pruned one does
/// not come back, and the edge keeps running rather than wedging.
#[tokio::test]
async fn a_restored_edge_skips_a_pruned_own_event() {
    let db = TempDb::new("false-rewind");
    let hub = MemorySyncPeer::new(HUB);
    let spoke = SqliteEventStore::open(db.path()).expect("a fresh file opens");
    let mut own = Vec::new();
    for kind in ["One", "Two", "Three"] {
        own.push(
            spoke
                .append(&[event(kind)], None)
                .await
                .expect("an append lands"),
        );
    }
    let mut edge = Edge::new();
    sync(&mut edge, &hub, &spoke).await;
    let pruned_at = *own.first().expect("three were appended");
    let pruned = EventId::new(spoke.store_id(), pruned_at);
    // A foreign event the hub learns of while the spoke is down.
    let arrived = minted(1);
    let arrived_id = arrived.first().expect("one was minted").id;
    hub.seed(EventGroup::new(None, arrived))
        .expect("an unpoisoned peer");
    drop(spoke);

    prune_outside_port(db.path(), &[pruned_at]);
    let spoke = SqliteEventStore::open(db.path()).expect("the pruned file opens");
    let mut again = restored(&edge);
    let stepped = sync(&mut again, &hub, &spoke).await;

    assert_eq!(again.status().state, EdgeState::Running);
    assert_eq!(
        stepped.ingested.appended, 1,
        "the foreign event in the page landed"
    );
    assert!(
        spoke.contains_event_id(arrived_id).await.expect("a read"),
        "the foreign event beside the echo did not land"
    );
    assert!(
        !spoke.contains_event_id(pruned).await.expect("a read"),
        "the pruned own event came back"
    );
}

/// The hub of R2-dst-3 and its control: a SQLite store holding three of its
/// own events, synced once to an in-memory spoke through a `StoreSyncPeer`.
/// Returns the spoke, the hub's positions and its identity.
async fn hub_synced_to_a_spoke(db: &TempDb) -> (MemoryIngestStore, Vec<SequencePosition>, StoreId) {
    let hub = SqliteEventStore::open(db.path()).expect("a fresh file opens");
    let mut own = Vec::new();
    for kind in ["One", "Two", "Three"] {
        own.push(
            hub.append(&[event(kind)], None)
                .await
                .expect("an append lands"),
        );
    }
    let hub_id = hub.store_id();
    let spoke = MemoryIngestStore::with_store_id(SPOKE);
    let peer = StoreSyncPeer::new(
        SqliteEventStore::open(db.path()).expect("a second handle opens"),
        hub,
    );
    sync(&mut Edge::new(), &peer, &spoke).await;
    (spoke, own, hub_id)
}

/// R2-dst-3, fixed (#484, the hub side). A hub prunes one of its own events
/// that a spoke already holds, and a spoke edge that lost its watermark
/// (`Edge::new`) pushes its whole log back, the pruned event included. The
/// hub skips the pruned event as a recorded removal rather than refusing the
/// push as an unheld own identity, so the edge syncs and the hub's log is
/// unchanged.
#[tokio::test]
async fn a_fresh_edge_pushing_into_a_pruned_hub_is_a_no_op() {
    let db = TempDb::new("pruned-hub");
    let (spoke, own, hub_id) = hub_synced_to_a_spoke(&db).await;
    let pruned_at = *own.first().expect("three were appended");

    prune_outside_port(db.path(), &[pruned_at]);
    let peer = StoreSyncPeer::new(
        SqliteEventStore::open(db.path()).expect("the pruned file opens"),
        SqliteEventStore::open(db.path()).expect("a second handle opens"),
    );
    let reader = SqliteEventStore::open(db.path()).expect("a third handle opens");
    let head = reader.head().await.expect("a read");
    let mut fresh = Edge::new();
    let stepped = sync(&mut fresh, &peer, &spoke).await;

    assert!(stepped.pushed > 0, "the whole log was offered back");
    assert_eq!(fresh.status().state, EdgeState::Running);
    assert_eq!(
        reader.head().await.expect("a read"),
        head,
        "the hub took nothing in"
    );
    assert!(
        !reader
            .contains_event_id(EventId::new(hub_id, pruned_at))
            .await
            .expect("a read"),
        "the pruned own event came back to the hub"
    );
}

/// Control for R2-dst-3, kept from the experiment. The same staging with
/// nothing pruned: a fresh edge pushes the whole spoke log back, the hub skips
/// every event it holds, and the edge keeps running. What the fixed R2-dst-3
/// adds over this is only the prune.
#[tokio::test]
async fn control_a_fresh_edge_pushing_into_an_unpruned_hub_is_a_no_op() {
    let db = TempDb::new("unpruned-hub");
    let (spoke, _, _) = hub_synced_to_a_spoke(&db).await;
    let peer = StoreSyncPeer::new(
        SqliteEventStore::open(db.path()).expect("the file opens"),
        SqliteEventStore::open(db.path()).expect("a second handle opens"),
    );
    let reader = SqliteEventStore::open(db.path()).expect("a third handle opens");
    let head = reader.head().await.expect("a read");
    let mut fresh = Edge::new();
    let stepped = sync(&mut fresh, &peer, &spoke).await;

    assert_eq!(stepped.ingested.appended, 0);
    assert!(stepped.pushed > 0, "the whole log was offered back");
    assert_eq!(fresh.status().state, EdgeState::Running);
    assert_eq!(
        reader.head().await.expect("a read"),
        head,
        "the hub skipped every event"
    );
}
