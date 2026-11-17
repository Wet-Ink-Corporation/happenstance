//! A local store pruned outside the port, then synced again by a restored
//! edge (SY-40, ADR-0110, #483, #484): the retention-reader's R2-dst-1 and
//! R2-dst-2 (`experiments/retention-reader/tests/it/r2_ingest_destination.rs`)
//! on the oracle, asserting the fixed behaviour rather than the hazard.
//!
//! The pruning is `MemoryIngestStore::remove`, the oracle's raw `DELETE`. The
//! watermark is kept in a `MemoryProjectionStore` beside a projection that
//! counts how often it applied each event, so "applies nothing twice" is read
//! off the projection itself, not inferred from an ingest count.

use std::collections::BTreeMap;

use happenstance_core::{
    Authority, Checkpoint, EventId, EventStore, MemoryProjectionStore, ProjectionId,
    ProjectionStore, Query, ReadOptions, collect,
};
use happenstance_sync::runner::{Edge, EdgeState, Stepped, Watermark};
use happenstance_sync::{EventGroup, MemoryIngestStore, MemoryResume, MemorySyncPeer};

use super::{FAR, LOCAL, ORIGIN, append_local, foreign};

/// The most steps a test takes before it decides the edge will never sync
/// (H-13): far above what any arrangement here needs.
const MAX_STEPS: usize = 64;

/// Steps `edge` until it reports synced, summing what the steps did, and fails
/// the test if a step fails or the edge never syncs.
async fn sync(
    edge: &mut Edge<MemoryResume>,
    peer: &MemorySyncPeer,
    store: &MemoryIngestStore,
) -> Stepped {
    let mut total = Stepped::default();
    for _ in 0..MAX_STEPS {
        let stepped = edge.step(peer, store).await.expect("an in-memory step");
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
    /// the last event considered. Returns how many events it applied.
    async fn run(&mut self, store: &MemoryIngestStore, models: &MemoryProjectionStore) -> usize {
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
        let Ok(events) = collect(store.read(&Query::all(), options)).await;
        let Some(last) = events.last() else {
            return 0;
        };
        for event in &events {
            *self.applied.entry(event.id).or_default() += 1;
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

/// Keeps `edge`'s watermark in `models`, and builds the edge a restart would
/// build from it.
///
/// A `MemorySyncPeer` names no responder, so the edge never pins one and
/// [`Edge::watermark`] has no peer to put in it. The caller knows which peer
/// it synced with, and names it, as an application keeping one edge does.
async fn restart(edge: &Edge<MemoryResume>, models: &MemoryProjectionStore) -> Edge<MemoryResume> {
    let confirmed = edge.status().confirmed.expect("the push was acknowledged");
    Watermark::new(FAR, confirmed)
        .commit(models, models.open())
        .await
        .expect("an in-memory commit lands");
    let kept = Watermark::load(models, FAR)
        .await
        .expect("an in-memory read")
        .expect("the watermark was just committed");
    Edge::restored(kept)
}

/// R2-dst-1. Rejects a store that dedupes only against what it still holds:
/// the restored edge re-pulls the far side's whole log, the two pruned
/// foreign events land again at the tail, and the projection, whose
/// checkpoint they now sit above, applies them a second time (#483).
#[tokio::test]
async fn a_restored_edge_over_a_pruned_memory_store_applies_nothing_twice() {
    let shipped: Vec<_> = (1..=4).map(|at| foreign(ORIGIN, at)).collect();
    let hub = MemorySyncPeer::new(FAR);
    for event in &shipped {
        // The hub stores its own copy, as a real one holds its own row.
        hub.seed(EventGroup::new(None, vec![event.clone()]))
            .expect("an unpoisoned peer");
    }
    let spoke = MemoryIngestStore::with_store_id(LOCAL);
    let models = MemoryProjectionStore::new();
    let mut counter = Counter::new();
    let mut edge = Edge::new();
    sync(&mut edge, &hub, &spoke).await;
    assert_eq!(counter.run(&spoke, &models).await, shipped.len());
    let pruned: Vec<EventId> = [1, 3]
        .iter()
        .filter_map(|&at| shipped.get(at))
        .map(|event| event.id)
        .collect();
    let pruned_at: Vec<_> = spoke
        .snapshot()
        .iter()
        .filter(|row| pruned.contains(&row.id))
        .map(|row| row.position)
        .collect();
    let mut restored = restart(&edge, &models).await;
    let before = spoke.snapshot();

    assert_eq!(spoke.remove(&pruned_at), 2);
    let stepped = sync(&mut restored, &hub, &spoke).await;
    let second_run = counter.run(&spoke, &models).await;

    assert_eq!(
        stepped.ingested.appended, 0,
        "a pruned event was re-admitted"
    );
    assert!(
        stepped.ingested.skipped >= shipped.len(),
        "the re-pull of the whole log was skipped, pruned events included: {stepped:?}"
    );
    assert_eq!(second_run, 0, "the projection had nothing new to apply");
    assert_eq!(counter.applied_twice(), Vec::<EventId>::new());
    let kept: Vec<_> = before
        .into_iter()
        .filter(|row| !pruned.contains(&row.id))
        .collect();
    assert_eq!(
        spoke.snapshot(),
        kept,
        "the log is the pruned log, unchanged"
    );
    assert_eq!(restored.status().state, EdgeState::Running);
}

/// R2-dst-2. Rejects a store that refuses a pruned own event as an unheld own
/// identity: the restored edge pulls it back from the far side, which already
/// held it, and the refusal stops the edge for good, the foreign event in the
/// same page included (#484).
#[tokio::test]
async fn a_restored_edge_over_a_memory_store_missing_an_own_event_syncs() {
    let hub = MemorySyncPeer::new(FAR);
    let spoke = MemoryIngestStore::with_store_id(LOCAL);
    let own = append_local(&spoke, 3).await;
    let models = MemoryProjectionStore::new();
    let mut edge = Edge::new();
    sync(&mut edge, &hub, &spoke).await;
    let pruned_at = *own.first().expect("three were appended");
    let pruned = EventId::new(LOCAL, pruned_at);
    // A foreign event the far side learns of while the spoke is down.
    let arrived = foreign(ORIGIN, 1);
    // The hub stores its own copy; `arrived` is kept for its identity.
    hub.seed(EventGroup::new(None, vec![arrived.clone()]))
        .expect("an unpoisoned peer");
    let mut restored = restart(&edge, &models).await;

    assert_eq!(spoke.remove(&[pruned_at]), 1);
    let stepped = sync(&mut restored, &hub, &spoke).await;

    assert_eq!(restored.status().state, EdgeState::Running);
    assert_eq!(
        stepped.ingested.appended, 1,
        "the foreign event in the page landed"
    );
    let Ok(landed) = spoke.contains_event_id(arrived.id).await;
    let Ok(echoed) = spoke.contains_event_id(pruned).await;
    assert!(landed, "the foreign event beside the echo did not land");
    assert!(!echoed, "the pruned own event came back");
}
