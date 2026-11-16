//! Shared setup for the scenarios.

use core::num::NonZeroUsize;

use happenstance::{
    Chunk, EventStore, Json, MemoryProjectionStore, Progressed, SequencePosition, Source, StoreId,
    run_projection,
};
use happenstance_sync::runner::{Edge, Stepped};
use happenstance_sync::{IngestStore, SyncPeer, WriteGroupStore};
use retention_reader::domain::{Claim, ClaimEvent, Counter};

pub(crate) type Error = Box<dyn core::error::Error>;
pub(crate) type Outcome = Result<(), Error>;

/// The most steps any scenario takes to reach a synced edge (H-13).
const MAX_STEPS: usize = 8;

/// A validated claim.
pub(crate) fn claim(id: &str) -> Result<Claim, Error> {
    Ok(Claim::new(id)?)
}

/// Twelve events over claims `a`, `b` and `c`, interleaved: three
/// registrations, then three notes on each, round-robin.
pub(crate) fn interleaved() -> Result<Vec<ClaimEvent>, Error> {
    let claims = [claim("a")?, claim("b")?, claim("c")?];
    // Each event stores its own claim: a short string, copied per event.
    let mut events: Vec<ClaimEvent> = claims
        .iter()
        .map(|c| ClaimEvent::registered(c.clone()))
        .collect();
    for note in 1..=3 {
        events.extend(claims.iter().map(|c| ClaimEvent::noted(c.clone(), note)));
    }
    Ok(events)
}

/// Appends each event alone, so every write group holds one event, and
/// returns the positions the store assigned, in order.
pub(crate) async fn append_each<S: EventStore>(
    store: &S,
    events: &[ClaimEvent],
) -> Result<Vec<SequencePosition>, Error> {
    let mut landed = Vec::with_capacity(events.len());
    for event in events {
        landed.push(store.append(&[event.to_event()?], None).await?);
    }
    Ok(landed)
}

/// The chunk every projection run uses.
pub(crate) const fn chunk() -> Chunk {
    Chunk::of(NonZeroUsize::MIN.saturating_add(3))
}

/// The identity [`project`] pins every run to.
///
/// Every scenario but R1-4 reads one event store per projection store, or
/// reopens the same file, so one identity stands for it. R1-4 is the scenario
/// that hands a second store to the same projection store, and it names each
/// store's own identity through [`project_from`].
const SOLE_SOURCE: StoreId = StoreId::from_bytes([0x5e; 16]);

/// Runs `counter` over `events` into `models` to the end of the log.
pub(crate) async fn project<S: EventStore>(
    events: &S,
    models: &MemoryProjectionStore,
    counter: &mut Counter,
) -> Result<Progressed, Error> {
    project_from(Source::new(events, SOLE_SOURCE), models, counter).await
}

/// Runs `counter` over `source` into `models`, pinned to the source's identity
/// (PS-40, #485).
pub(crate) async fn project_from<S: EventStore>(
    source: Source<'_, S>,
    models: &MemoryProjectionStore,
    counter: &mut Counter,
) -> Result<Progressed, Error> {
    Ok(run_projection(source, models, counter, &Json, chunk()).await?)
}

/// Steps `edge` until it reports synced, at most [`MAX_STEPS`] times, and
/// sums what the steps did.
pub(crate) async fn sync<P, S>(
    edge: &mut Edge<P::Resume>,
    peer: &P,
    store: &S,
) -> Result<Stepped, Error>
where
    P: SyncPeer,
    S: WriteGroupStore + IngestStore,
    P::Error: 'static,
    <S as WriteGroupStore>::Error: 'static,
    <S as IngestStore>::Error: 'static,
{
    let mut total = Stepped::default();
    for _ in 0..MAX_STEPS {
        let stepped = edge.step(peer, store).await?;
        total.ingested.appended += stepped.ingested.appended;
        total.ingested.skipped += stepped.ingested.skipped;
        total.pushed += stepped.pushed;
        if edge.status().is_synced() {
            return Ok(total);
        }
    }
    Err(format!("not synced after {MAX_STEPS} steps: {:?}", edge.status()).into())
}
