//! R2-dst, the ingest path writing *into* a pruned store: an edge over a
//! SQLite file whose rows are deleted outside the port, plus the instrument
//! controls that stage the same scenarios over CF-27's decorator.
//!
//! The projection over the SQLite spoke reads through `GroupRead`, because
//! `SqliteEventStore::read` needs a tokio runtime this crate does not have;
//! every write group here holds one event, which is where that read is
//! faithful (see `src/group_read.rs`).

use core::cell::Cell;

use happenstance::{
    EventId, EventStore, MemoryProjectionStore, Query, ReadOptions, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, collect,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sync::runner::{Edge, EdgeState, StepError, Watermark};
use happenstance_sync::store_peer::StorePeerError;
use happenstance_sync::{
    Ack, EventGroup, IngestError, IngestStore, MemorySyncPeer, PeerLimits, Pulled, PushBatch,
    ReplicatedEvent, StoreSyncPeer, SyncPeer,
};
use happenstance_testkit::block_on;
use retention_reader::domain::{ClaimEvent, Counter};
use retention_reader::group_read::GroupRead;
use retention_reader::holed::Holed;
use retention_reader::prune::{TempDb, prune_outside_port};
use retention_reader::shared::Shared;

use crate::support::{Error, Outcome, append_each, interleaved, project, sync};

const HUB: StoreId = StoreId::from_bytes([2; 16]);
/// A third store, the origin of every foreign event the hub serves.
const ORIGIN: StoreId = StoreId::from_bytes([3; 16]);

/// `events` as `ORIGIN` minted them, at its first positions.
fn minted(events: &[ClaimEvent]) -> Result<Vec<ReplicatedEvent>, Error> {
    let mut position = SequencePosition::FIRST;
    let mut minted = Vec::with_capacity(events.len());
    for (millis, event) in (1_700_000_000_000_i64..).zip(events) {
        let id = EventId::new(ORIGIN, position);
        minted.push(ReplicatedEvent::new(
            id,
            RecordedAt::from_millis(millis),
            event.to_event()?,
        ));
        position = position.next().ok_or("position space")?;
    }
    Ok(minted)
}

/// A hub holding `events`, one group each.
fn seeded(events: &[ReplicatedEvent]) -> Result<MemorySyncPeer, Error> {
    let hub = MemorySyncPeer::new(HUB);
    for event in events {
        // The hub stores its own copy: genuine duplication.
        hub.seed(EventGroup::new(None, vec![event.clone()]))?;
    }
    Ok(hub)
}

/// A peer that counts the calls it forwards, so a test can show a step did
/// no I/O.
#[derive(Debug)]
struct Counting<'p, P> {
    peer: &'p P,
    // Single-threaded under `block_on`, and only ever incremented: a `Cell`
    // is enough.
    calls: Cell<usize>,
}

impl<'p, P> Counting<'p, P> {
    const fn new(peer: &'p P) -> Self {
        Self {
            peer,
            calls: Cell::new(0),
        }
    }

    fn calls(&self) -> usize {
        self.calls.get()
    }

    fn count(&self) {
        self.calls.set(self.calls.get().saturating_add(1));
    }
}

impl<P: SyncPeer> SyncPeer for Counting<'_, P> {
    type Error = P::Error;
    type Resume = P::Resume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.count();
        self.peer.pull(from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.count();
        self.peer.push(batch).await
    }

    fn limits(&self) -> PeerLimits {
        self.peer.limits()
    }
}

/// Every event `store` holds, read through its write groups.
async fn held(store: &SqliteEventStore) -> Result<Vec<SequencedEvent>, Error> {
    Ok(
        collect(GroupRead::new(store, store.store_id()).read(&Query::all(), ReadOptions::new()))
            .await?,
    )
}

/// The local positions of the events carrying `ids`.
async fn local_positions(
    store: &SqliteEventStore,
    ids: &[EventId],
) -> Result<Vec<SequencePosition>, Error> {
    Ok(held(store)
        .await?
        .iter()
        .filter(|e| ids.contains(&e.id))
        .map(|e| e.position)
        .collect())
}

/// R2-dst-1. **A recorded hazard, not a desired behaviour.** A spoke prunes two
/// foreign events it has projected, including the origin's highest, outside
/// the port. Its edge restarts by the documented path, `Edge::restored`, which
/// pulls from the start of the hub's log. Ingest's deduplication is the
/// store's uniqueness on the origin pair (VT-8), which held only among rows
/// still present, so both events are admitted again at the tail, and the
/// projection, whose checkpoint they now sit above, applies them a second
/// time.
#[test]
fn a_restored_edge_resurrects_pruned_foreign_events() -> Outcome {
    let db = TempDb::new("resurrection")?;
    block_on(async {
        let shipped = minted(interleaved()?.get(..4).ok_or("twelve")?)?;
        let hub = seeded(&shipped)?;
        let models = MemoryProjectionStore::new();
        let mut counter = Counter::new("claims")?;

        let spoke = SqliteEventStore::open(db.path())?;
        let mut edge = Edge::new();
        sync(&mut edge, &hub, &spoke).await?;
        let first_run = project(
            &GroupRead::new(&spoke, spoke.store_id()),
            &models,
            &mut counter,
        )
        .await?;
        let confirmed = edge.status().confirmed.ok_or("the push was acknowledged")?;
        let head_before = spoke.head().await?;
        let mark_before = spoke.watermark().await?.get(ORIGIN);
        let pruned_ids: Vec<EventId> = [1, 3]
            .iter()
            .filter_map(|&i| shipped.get(i))
            .map(|e| e.id)
            .collect();
        let pruned_at = local_positions(&spoke, &pruned_ids).await?;
        drop(spoke);

        assert_eq!(prune_outside_port(db.path(), &pruned_at)?, 2);
        let spoke = SqliteEventStore::open(db.path())?;
        for &id in &pruned_ids {
            assert!(!spoke.contains_event_id(id).await?);
        }
        let mark_pruned = spoke.watermark().await?.get(ORIGIN);

        let mut restored = Edge::restored(Watermark::new(HUB, confirmed));
        let stepped = sync(&mut restored, &hub, &spoke).await?;
        let second_run = project(
            &GroupRead::new(&spoke, spoke.store_id()),
            &models,
            &mut counter,
        )
        .await?;

        assert_eq!(first_run.applied, 4);
        assert_eq!(mark_before, shipped.last().map(|e| e.id.position()));
        assert!(
            mark_pruned < mark_before,
            "the watermark regressed with the prune"
        );
        assert_eq!(
            stepped.ingested.appended, 2,
            "both pruned events were re-admitted"
        );
        assert_eq!(
            spoke.watermark().await?.get(ORIGIN),
            mark_before,
            "and it recovered"
        );
        let readmitted = local_positions(&spoke, &pruned_ids).await?;
        assert!(
            readmitted.iter().all(|&p| Some(p) > head_before),
            "at the tail"
        );
        assert_eq!(second_run.applied, 2);
        assert_eq!(
            counter.applied_twice().collect::<Vec<_>>(),
            pruned_ids,
            "applied twice"
        );
        drop(spoke);
        Ok::<(), Error>(())
    })?;
    db.finish()?;
    Ok(())
}

/// R2-dst-2. **A recorded hazard, not a desired behaviour.** A spoke prunes one
/// of its own events after the hub confirmed it, which is the rule a pruned
/// buffer would follow. The restored edge pulls it back from the hub, the
/// event lands because its row is gone, and ingest refuses the whole call as
/// SY-38's unheld own identity: "restored or rewound without re-minting". The
/// store was neither. The class is stop, nothing in the pull lands (a foreign
/// event in the same batch included, though the refused page carried it), and
/// every later step is refused with no call to the peer: the edge is wedged.
#[test]
fn a_restored_edge_stops_on_a_pruned_own_event() -> Outcome {
    let db = TempDb::new("false-rewind")?;
    block_on(async {
        let events = interleaved()?;
        let hub = MemorySyncPeer::new(HUB);
        let spoke = SqliteEventStore::open(db.path())?;
        let own = append_each(&spoke, events.get(..3).ok_or("twelve")?).await?;
        let mut edge = Edge::new();
        sync(&mut edge, &hub, &spoke).await?;
        let confirmed = edge.status().confirmed.ok_or("the push was acknowledged")?;
        let own_ids: Vec<EventId> = held(&spoke).await?.iter().map(|e| e.id).collect();
        let pruned = *own_ids.first().ok_or("three were appended")?;
        // A foreign event the hub learns of while the spoke is down.
        let foreign = minted(events.get(3..4).ok_or("twelve")?)?;
        let foreign_id = foreign.first().ok_or("one was minted")?.id;
        hub.seed(EventGroup::new(None, foreign))?;
        drop(spoke);

        assert_eq!(
            prune_outside_port(db.path(), own.get(..1).ok_or("three")?)?,
            1
        );
        let spoke = SqliteEventStore::open(db.path())?;
        let head_after_prune = spoke.head().await?;

        // The page the restored edge will pull: from the start of the hub.
        let page = hub.pull(None).await?;
        let offered: Vec<EventId> = page.batch.events().map(|e| e.id).collect();
        let counted = Counting::new(&hub);
        let mut restored = Edge::restored(Watermark::new(HUB, confirmed));
        let step = restored.step(&counted, &spoke).await;
        let calls_before = counted.calls();
        let again = restored.step(&counted, &spoke).await;

        assert!(
            matches!(step, Err(StepError::Ingest(IngestError::UnheldOwnIdentity { id, .. })) if id == pruned),
            "{step:?}"
        );
        assert!(
            offered.contains(&pruned) && offered.contains(&foreign_id),
            "the refused page carried both: {offered:?}"
        );
        assert!(matches!(restored.status().state, EdgeState::Stopped(_)));
        assert_eq!(spoke.head().await?, head_after_prune, "nothing landed");
        assert!(
            !spoke.contains_event_id(foreign_id).await?,
            "not even the foreign event"
        );
        assert!(
            matches!(again, Err(StepError::Stopped(_))),
            "wedged: {again:?}"
        );
        assert_eq!(calls_before, 1, "the refused step pulled once");
        assert_eq!(
            counted.calls(),
            calls_before,
            "the next step called no peer"
        );
        drop(spoke);
        Ok::<(), Error>(())
    })?;
    db.finish()?;
    Ok(())
}

/// R2-dst-3. **A recorded hazard, not a desired behaviour; not among the
/// predictions.** The same refusal on the hub side: a hub prunes one of its
/// own events that a spoke already holds, and a spoke edge that lost its
/// watermark (`Edge::new`) pushes its whole log, the pruned event included,
/// back to the hub. The hub's ingest refuses the push as an unheld own
/// identity, and the spoke's edge stops.
#[test]
fn a_fresh_edge_pushing_into_a_pruned_hub_stops() -> Outcome {
    let db = TempDb::new("pruned-hub")?;
    block_on(async {
        let events = interleaved()?;
        let spoke = Shared::new();
        let pruned = {
            let hub = SqliteEventStore::open(db.path())?;
            let own = append_each(&hub, events.get(..3).ok_or("twelve")?).await?;
            let peer = StoreSyncPeer::new(SqliteEventStore::open(db.path())?, hub);
            sync(&mut Edge::new(), &peer, &spoke).await?;
            own.first().copied().ok_or("three were appended")?
        };

        assert_eq!(prune_outside_port(db.path(), &[pruned])?, 1);
        let peer = StoreSyncPeer::new(
            SqliteEventStore::open(db.path())?,
            SqliteEventStore::open(db.path())?,
        );
        let mut fresh = Edge::new();
        let step = fresh.step(&peer, &spoke).await;

        let hub_id = SqliteEventStore::open(db.path())?.store_id();
        assert!(
            matches!(
                &step,
                Err(StepError::Push(StorePeerError::Ingest(IngestError::UnheldOwnIdentity { id, .. })))
                    if *id == EventId::new(hub_id, pruned)
            ),
            "{step:?}"
        );
        assert!(matches!(fresh.status().state, EdgeState::Stopped(_)));
        drop(peer);
        Ok::<(), Error>(())
    })?;
    db.finish()?;
    Ok(())
}

/// Control for R2-dst-3. The same staging with nothing pruned: a fresh edge
/// pushes the whole spoke log back, the hub skips every event it holds, and
/// the edge keeps running. The stop in R2-dst-3 is the prune's doing.
#[test]
fn control_a_fresh_edge_pushing_into_an_unpruned_hub_is_a_no_op() -> Outcome {
    let db = TempDb::new("unpruned-hub")?;
    block_on(async {
        let events = interleaved()?;
        let spoke = Shared::new();
        let hub = SqliteEventStore::open(db.path())?;
        append_each(&hub, events.get(..3).ok_or("twelve")?).await?;
        let head = hub.head().await?;
        let peer = StoreSyncPeer::new(SqliteEventStore::open(db.path())?, hub);
        sync(&mut Edge::new(), &peer, &spoke).await?;

        let mut fresh = Edge::new();
        let stepped = sync(&mut fresh, &peer, &spoke).await?;

        assert_eq!(stepped.ingested.appended, 0);
        assert!(stepped.pushed > 0, "the whole log was offered back");
        assert_eq!(fresh.status().state, EdgeState::Running);
        drop(peer);
        let hub = SqliteEventStore::open(db.path())?;
        assert_eq!(hub.head().await?, head, "the hub skipped every event");
        drop(hub);
        Ok::<(), Error>(())
    })?;
    db.finish()?;
    Ok(())
}

/// Control for R2-dst-1. The same resurrection staged over CF-27's decorator:
/// withheld rather than deleted, the events are still physically held, so the
/// re-pull skips them and nothing is visible. The instrument's stated fidelity
/// gap (ingest deduplication) hides the harm.
#[test]
fn instrument_control_resurrection_is_masked() -> Outcome {
    block_on(async {
        let shipped = minted(interleaved()?.get(..4).ok_or("twelve")?)?;
        let hub = seeded(&shipped)?;
        let spoke = Holed::new(Shared::new());
        let mut edge = Edge::new();
        sync(&mut edge, &hub, &spoke).await?;
        let confirmed = edge.status().confirmed.ok_or("the push was acknowledged")?;
        let all = collect(spoke.view().read(&Query::all(), ReadOptions::new())).await?;
        let gone: Vec<_> = all.iter().skip(1).step_by(2).map(|e| e.position).collect();
        spoke.view().withhold(&gone).await?;

        let mut restored = Edge::restored(Watermark::new(HUB, confirmed));
        let stepped = sync(&mut restored, &hub, &spoke).await?;

        assert_eq!(gone.len(), 2);
        assert_eq!(
            stepped.ingested.appended, 0,
            "skipped: still physically held"
        );
        assert_eq!(spoke.view().head().await?, all.get(2).map(|e| e.position));
        Ok(())
    })
}

/// Control for R2-dst-2. The same false rewind staged over the decorator: the
/// withheld own event is still physically held, so the re-pull skips it and
/// the edge keeps running.
#[test]
fn instrument_control_false_rewind_is_masked() -> Outcome {
    block_on(async {
        let hub = MemorySyncPeer::new(HUB);
        let spoke = Holed::new(Shared::new());
        let own = append_each(spoke.inner(), interleaved()?.get(..3).ok_or("twelve")?).await?;
        let mut edge = Edge::new();
        sync(&mut edge, &hub, &spoke).await?;
        let confirmed = edge.status().confirmed.ok_or("the push was acknowledged")?;
        spoke.view().withhold(own.get(..1).ok_or("three")?).await?;

        let mut restored = Edge::restored(Watermark::new(HUB, confirmed));
        let stepped = sync(&mut restored, &hub, &spoke).await?;

        assert_eq!(stepped.ingested.appended, 0);
        assert_eq!(restored.status().state, EdgeState::Running);
        Ok(())
    })
}
