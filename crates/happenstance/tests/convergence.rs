//! The convergence declaration (SY-20, SY-21, SY-22), on the typed layer.
//!
//! A projection says its read model is the same in any ingest order by
//! implementing [`ConvergentProjection`]. Nothing here checks that claim over
//! every interleaving: that is phase 13's rule, in `happenstance-sync-testkit`.
//! What is pinned here is the declaration's shape, that both flavours can make
//! it and a generic spawner can still require it, and the one fact the runner
//! contributes: a declared projection is handed origin identities, so two
//! interleavings that agree per origin commit one read model.
//!
//! One flavour name is imported here, [`Projection`], and the other is reached
//! by full path (RS-20-3).
#![cfg(feature = "unstable-projection")]

use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

use happenstance::bytes::Bytes;
use happenstance::{
    Chunk, Codec, CodecError, ConvergentProjection, Delivered, DomainEvent, Event, EventId,
    EventType, Json, MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, Progressed, Projection, ProjectionId, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, Tags, run_projection,
};
use serde::{Deserialize, Serialize};

const COUNTED: EventType = EventType::from_static("Counted");

/// The one domain event: a number of units, and no tags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Counted {
    units: u64,
}

impl DomainEvent for Counted {
    const EVENT_TYPES: &'static [EventType] = &[COUNTED];

    fn event_type(&self) -> EventType {
        COUNTED
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if event_type != &COUNTED {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

/// The fold both projections share: keep the units of the greatest identity.
///
/// Ordering by `EventId` is the peer-independent order (SY-23), so the winner
/// is the same on every replica. Fed a local position instead, the winner is
/// whichever event arrived last, which is what makes this fold a falsifier.
#[derive(Debug, Default)]
struct Latest {
    latest: Option<(EventId, u64)>,
    count: u64,
}

impl Latest {
    fn fold(&mut self, delivered: Delivered<Counted>, batch: &mut MemoryProjectionBatch) {
        let id = delivered.id();
        let units = delivered.into_event().units;
        if self.latest.is_none_or(|(held, _)| id > held) {
            self.latest = Some((id, units));
        }
        self.count = self.count.saturating_add(1);
        if let Some((_, winner)) = self.latest {
            batch.write("latest", winner);
        }
        batch.write("count", self.count);
    }
}

/// The bare flavour. The `Rc` makes it `!Send`, so it cannot be a
/// `SendProjection` by accident.
#[derive(Debug)]
struct LatestByEventId {
    id: ProjectionId,
    scope: Tags,
    fold: Latest,
    _local: Rc<()>,
}

impl LatestByEventId {
    fn new() -> Self {
        Self {
            id: ProjectionId::from_static("latest_by_event_id"),
            scope: Tags::empty(),
            fold: Latest::default(),
            _local: Rc::new(()),
        }
    }
}

impl Projection for LatestByEventId {
    type Event = Counted;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Counted>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.fold.fold(event, batch);
        Ok(())
    }
}

impl ConvergentProjection for LatestByEventId {}

/// The same fold as a `SendProjection`, without the `Rc`.
#[derive(Debug)]
struct SendLatest {
    id: ProjectionId,
    scope: Tags,
    fold: Latest,
}

impl SendLatest {
    fn new() -> Self {
        Self {
            id: ProjectionId::from_static("send_latest"),
            scope: Tags::empty(),
            fold: Latest::default(),
        }
    }
}

impl happenstance::SendProjection for SendLatest {
    type Event = Counted;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Counted>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.fold.fold(event, batch);
        Ok(())
    }
}

impl ConvergentProjection for SendLatest {}

// ---------------------------------------------------------------------------
// The declaration is one name, and either flavour can make it
// ---------------------------------------------------------------------------

/// Accepts exactly the projections that declared convergence.
fn declared<P: Projection + ConvergentProjection>(_: &P) {}

/// Both flavours satisfy the bound a convergence check writes.
///
/// The wrong implementation this rejects is a marker over `SendProjection`
/// alone, which shuts out the `!Send` projection and every `wasm32` one.
#[test]
fn either_flavour_declares_convergence() {
    declared(&LatestByEventId::new());
    declared(&SendLatest::new());
}

// ---------------------------------------------------------------------------
// SY-21 on `Delivered`, and SY-20 previewed through the shipped runner
// ---------------------------------------------------------------------------

/// A test's own failure, boxed: the fixtures below fail in four error types.
type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

const ORIGIN_ONE: StoreId = StoreId::from_bytes([1; 16]);
const ORIGIN_TWO: StoreId = StoreId::from_bytes([2; 16]);
const LOCAL: StoreId = StoreId::from_bytes([9; 16]);

/// One event as its origin wrote it: who, at which position, how many units.
type Authored = (StoreId, u64, u64);

/// Interleaving A: the two origins alternate.
const ALTERNATING: [Authored; 4] = [
    (ORIGIN_ONE, 1, 10),
    (ORIGIN_TWO, 1, 30),
    (ORIGIN_ONE, 2, 20),
    (ORIGIN_TWO, 2, 40),
];

/// Interleaving B: all of origin two, then all of origin one. Per origin it
/// agrees with A; everywhere else it does not.
const ORIGIN_TWO_FIRST: [Authored; 4] = [
    (ORIGIN_TWO, 1, 30),
    (ORIGIN_TWO, 2, 40),
    (ORIGIN_ONE, 1, 10),
    (ORIGIN_ONE, 2, 20),
];

/// Which identity the restored log carries for each event.
#[derive(Debug, Clone, Copy)]
enum Identity {
    /// The origin's, as ingest preserves it.
    Origin,
    /// One minted from the local position: what a runner that leaked the
    /// arrival position would hand `apply`.
    Reminted,
}

/// The log a replica holds after ingesting `authored` in that order.
fn ingested(authored: &[Authored], identity: Identity) -> Fallible<MemoryEventStore> {
    let mut log = Vec::with_capacity(authored.len());
    for (local, &(origin, there, units)) in (1..).zip(authored) {
        let here = SequencePosition::new(local).ok_or("a local position is not zero")?;
        let id = match identity {
            Identity::Origin => EventId::new(
                origin,
                SequencePosition::new(there).ok_or("an origin position is not zero")?,
            ),
            Identity::Reminted => EventId::new(LOCAL, here),
        };
        let event = Event::new(COUNTED, Json.encode(&Counted { units })?)?;
        log.push(SequencedEvent::new(
            here,
            id,
            RecordedAt::from_millis(0),
            event,
        ));
    }
    Ok(MemoryEventStore::restore(LOCAL, log))
}

/// Runs a declared projection over `events` and returns what it committed.
///
/// Bound on the declaration, as phase 13's rule will be, so an undeclared
/// projection could not be handed to it.
async fn committed<P>(
    events: &MemoryEventStore,
    mut projection: P,
) -> Fallible<BTreeMap<String, u64>>
where
    P: Projection<Store = MemoryProjectionStore> + ConvergentProjection,
{
    let models = MemoryProjectionStore::new();
    let chunk = Chunk::of(2.try_into()?);
    let progressed = run_projection(events, &models, &mut projection, &Json, chunk)
        .await
        .map_err(|error| error.to_string())?;
    if progressed.applied != 4 {
        return Err(format!("applied {} of 4 events", progressed.applied).into());
    }
    Ok(models.snapshot())
}

/// Two interleavings that agree per origin commit one read model, and the same
/// fold over re-minted identities does not.
///
/// The wrong implementation this rejects is a runner that re-mints each
/// identity from the local position. The second half is what keeps the first
/// from being decorative: it shows the fold is order-sensitive in exactly the
/// way `EventId` repairs, so a fold that converged whatever it was handed
/// would fail here.
#[tokio::test]
async fn a_convergent_projection_commits_one_read_model_for_two_interleavings() -> Fallible<()> {
    let alternating = committed(
        &ingested(&ALTERNATING, Identity::Origin)?,
        LatestByEventId::new(),
    )
    .await?;
    let origin_two_first = committed(
        &ingested(&ORIGIN_TWO_FIRST, Identity::Origin)?,
        LatestByEventId::new(),
    )
    .await?;

    assert_eq!(
        alternating, origin_two_first,
        "two interleavings that agree per origin committed different read models"
    );
    let expected = BTreeMap::from([("count".to_owned(), 4), ("latest".to_owned(), 40)]);
    assert_eq!(
        alternating, expected,
        "the greatest identity is origin two's second"
    );

    let alternating = committed(
        &ingested(&ALTERNATING, Identity::Reminted)?,
        LatestByEventId::new(),
    )
    .await?;
    let origin_two_first = committed(
        &ingested(&ORIGIN_TWO_FIRST, Identity::Reminted)?,
        LatestByEventId::new(),
    )
    .await?;

    assert_ne!(
        alternating, origin_two_first,
        "the fold converged over local identities, so the first half proves nothing"
    );
    assert_eq!(
        alternating.get("latest"),
        Some(&40),
        "A arrives at O2/2 last"
    );
    assert_eq!(
        origin_two_first.get("latest"),
        Some(&20),
        "B arrives at O1/2 last"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// A generic spawner can require the declaration
// ---------------------------------------------------------------------------

/// `Projection`'s documented spawner, verbatim, plus `+ ConvergentProjection`.
fn spawn_run<S, P, St>(
    events: Arc<S>,
    models: Arc<St>,
    mut projection: P,
    chunk: Chunk,
) -> tokio::task::JoinHandle<(P, Result<Progressed, String>)>
where
    S: happenstance::SendEventStore<Error: Send> + Sync + 'static,
    P: happenstance::SendProjection<Store = St, Event: Send, Error: Send + Sync>
        + ConvergentProjection
        + 'static,
    St: happenstance::SendProjectionStore<Batch: Send> + Sync + 'static,
{
    tokio::spawn(async move {
        let run = run_projection(&*events, &*models, &mut projection, &Json, chunk)
            .await
            .map_err(|e| e.to_string());
        (projection, run)
    })
}

/// A declared `SendProjection` is spawned from code generic over it.
///
/// The wrong implementation this rejects is the trait spelled
/// `ConvergentProjection: Projection`: its where-clause shadows the blanket
/// impl `SendProjection` gets, and this body stops compiling (`E0271` on
/// `P::Store`, `E0277` on `apply`'s future).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_declared_send_projection_spawns_from_code_generic_over_it() -> Fallible<()> {
    let events = Arc::new(ingested(&ALTERNATING[..1], Identity::Origin)?);
    let models = Arc::new(MemoryProjectionStore::new());
    let chunk = Chunk::of(64.try_into()?);

    let (latest, run) = spawn_run(events, Arc::clone(&models), SendLatest::new(), chunk).await?;

    assert_eq!(run?.applied, 1);
    assert_eq!(
        latest.fold.count, 1,
        "the projection came back with its state"
    );
    assert_eq!(models.get("latest"), Some(10));
    Ok(())
}
