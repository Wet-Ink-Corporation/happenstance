//! The typed projection runner, driven from **outside** the crate.
//!
//! Every test here runs under `--features unstable-projection,memory`: with the
//! feature off there is nothing to test, which is AC-008's point rather than a
//! gap. The whole target is gated at the top so a default-feature
//! `cargo test -p happenstance` compiles it away instead of failing to build.
//!
//! The projection store is **HS-P0010's** `MemoryProjectionStore`, reached
//! through the facade's glob re-export. Nothing in this file implements
//! `ProjectionStore`: writing a throwaway one here would freeze a fixture shape
//! this project does not own (EC-009, AC-012), and `doc_surface.rs` asserts the
//! absence.
#![cfg(all(feature = "unstable-projection", feature = "memory", feature = "json"))]
// A test target may unwrap; the house style permits the local override and the
// library code under test is held to `unwrap_used = "deny"` regardless.
#![allow(clippy::unwrap_used)]

use core::cell::RefCell;
use core::num::{NonZeroU64, NonZeroUsize};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::BTreeMap;

use futures_core::Stream;
use happenstance::bytes::Bytes;
use happenstance::{
    AppendCondition, AppendError, Checkpoint, Codec, CodecError, DomainEvent, Event, EventId,
    EventStore, EventType, Json, MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, MemoryStoreError, Progressed, Projection, ProjectionError,
    ProjectionId, ProjectionStore, Query, QueryItem, ReadOptions, SequencePosition, SequencedEvent,
    Tags, run_projection,
};
use happenstance_testkit::GappyMemoryStore;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// One two-variant domain, owned by the application
// ---------------------------------------------------------------------------

const DELIVERED: EventType = EventType::from_static("StockDelivered");
const DISPATCHED: EventType = EventType::from_static("StockDispatched");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Stock {
    Delivered { depot: String, units: u64 },
    Dispatched { depot: String, units: u64 },
}

impl Stock {
    fn depot(&self) -> &str {
        match self {
            Self::Delivered { depot, .. } | Self::Dispatched { depot, .. } => depot,
        }
    }
}

impl DomainEvent for Stock {
    const EVENT_TYPES: &'static [EventType] = &[DELIVERED, DISPATCHED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Delivered { .. } => DELIVERED,
            Self::Dispatched { .. } => DISPATCHED,
        }
    }

    fn tags(&self) -> Tags {
        depot_tags(self.depot())
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if !Self::EVENT_TYPES.contains(event_type) {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

fn depot_tags(depot: &str) -> Tags {
    Tags::from_pairs([("depot", depot)]).expect("a valid tag pair")
}

fn delivered(depot: &str, units: u64) -> Stock {
    Stock::Delivered {
        depot: depot.to_owned(),
        units,
    }
}

fn dispatched(depot: &str, units: u64) -> Stock {
    Stock::Dispatched {
        depot: depot.to_owned(),
        units,
    }
}

/// One domain event, encoded and tagged the way a command would have left it.
fn landed(event: &Stock) -> Event {
    let payload = event.encode(&Json).expect("the fixture encodes");
    Event::new(event.event_type(), payload)
        .expect("a valid event type")
        .with_tags(event.tags())
}

// ---------------------------------------------------------------------------
// The application's own projection
// ---------------------------------------------------------------------------

/// Units on hand per depot, written into the adapter's open batch.
///
/// `seen` is not part of the read model: it is how a test proves what `apply`
/// was handed, which is the whole of AC-001 and AC-002.
#[derive(Debug)]
struct VanStock {
    id: ProjectionId,
    scope: Tags,
    seen: Vec<Stock>,
    totals: BTreeMap<String, u64>,
}

impl VanStock {
    fn for_depot(depot: &str) -> Self {
        Self {
            id: ProjectionId::new("van_stock"),
            scope: depot_tags(depot),
            seen: Vec::new(),
            totals: BTreeMap::new(),
        }
    }
}

impl Projection for VanStock {
    type Event = Stock;
    type Store = MemoryProjectionStore;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(
        &mut self,
        event: Self::Event,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        // The match is on the application's **own enum**, not on an
        // `EventType` and not on `Bytes`. A signature taking `Bytes` would not
        // compile against this impl.
        let (depot, delta) = match &event {
            Stock::Delivered { depot, units } => (depot.clone(), i128::from(*units)),
            Stock::Dispatched { depot, units } => (depot.clone(), -i128::from(*units)),
        };
        let total = self.totals.entry(depot.clone()).or_insert(0);
        *total = u64::try_from(i128::from(*total) + delta).unwrap_or(0);
        batch.write(depot, *total);
        self.seen.push(event);
        Ok(())
    }
}

fn chunk(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).expect("a non-zero chunk")
}

/// The query the derivation must produce, spelled independently of the runner.
fn expected_query(scope: &Tags) -> Query {
    Query::from_item(
        QueryItem::new(Stock::EVENT_TYPES.iter().cloned(), scope.clone()).expect("a valid item"),
    )
}

async fn seed(store: &MemoryEventStore, events: &[Stock]) {
    for event in events {
        EventStore::append(store, &[landed(event)], None)
            .await
            .expect("the fixture seeds");
    }
}

// ---------------------------------------------------------------------------
// A store that records the query it was handed
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct QueryWatching {
    inner: MemoryEventStore,
    queries: RefCell<Vec<Query>>,
}

impl QueryWatching {
    fn new() -> Self {
        Self {
            inner: MemoryEventStore::new(),
            queries: RefCell::new(Vec::new()),
        }
    }
}

impl EventStore for QueryWatching {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.queries.borrow_mut().push(query.clone());
        EventStore::read(&self.inner, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        EventStore::append(&self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(&self.inner).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(&self.inner, id).await
    }
}

// ---------------------------------------------------------------------------
// A store whose stream records what the read model held at each pull
// ---------------------------------------------------------------------------

/// The AC-005 instrument. Each pull records how many rows the projection store
/// had already **committed** at that moment, so a runner that `collect`s the
/// stream records zero everywhere and a streaming one does not.
#[derive(Debug)]
struct PullWatching<'m> {
    events: Vec<SequencedEvent>,
    rows_at_pull: RefCell<Vec<usize>>,
    models: &'m MemoryProjectionStore,
}

struct Pulls<'a> {
    remaining: std::vec::IntoIter<SequencedEvent>,
    rows_at_pull: &'a RefCell<Vec<usize>>,
    models: &'a MemoryProjectionStore,
}

impl Stream for Pulls<'_> {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.remaining.next() {
            Some(event) => {
                this.rows_at_pull.borrow_mut().push(this.models.len());
                Poll::Ready(Some(Ok(event)))
            }
            None => Poll::Ready(None),
        }
    }
}

impl EventStore for PullWatching<'_> {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        let matched: Vec<SequencedEvent> = self
            .events
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()))
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .cloned()
            .collect();
        Pulls {
            remaining: matched.into_iter(),
            rows_at_pull: &self.rows_at_pull,
            models: self.models,
        }
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        unreachable!("the pull-order instrument is read-only")
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.events.last().map(|event| event.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.events.iter().any(|event| event.id == id))
    }
}

// ---------------------------------------------------------------------------
// AC-001 — the query is derived, and nothing outside the scope reaches `apply`
// ---------------------------------------------------------------------------

#[tokio::test]
async fn derived_query_matches_event_types_and_scope() {
    let events = QueryWatching::new();
    seed(
        &events.inner,
        &[delivered("d7", 12), delivered("d9", 5), dispatched("d7", 4)],
    )
    .await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");

    let progress = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect("the runner completes");

    // The nomination vocabulary is the contract's `Query`, derived from the
    // domain type's own `EVENT_TYPES` and the projection's scope. There is no
    // second filter type and no hand-maintained subscription.
    let recorded = events.queries.borrow();
    assert_eq!(
        recorded.len(),
        1,
        "the runner read once, not once per chunk"
    );
    assert_eq!(recorded[0], expected_query(&depot_tags("d7")));

    // An event outside the scope is never handed to `apply`.
    assert_eq!(
        projection.seen,
        vec![delivered("d7", 12), dispatched("d7", 4)]
    );
    assert_eq!(progress.applied, 2);
}

// ---------------------------------------------------------------------------
// AC-002 — `apply` is handed a decoded domain event, not `Bytes`
// ---------------------------------------------------------------------------

#[tokio::test]
async fn apply_receives_decoded_domain_events() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 12), dispatched("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");

    let _ = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect("the runner completes");

    assert_eq!(
        projection.seen,
        vec![delivered("d7", 12), dispatched("d7", 4)],
        "`apply` matched on its own enum variants, so the runner decoded"
    );
    assert_eq!(models.get("d7"), Some(8));
}

// ---------------------------------------------------------------------------
// AC-003 — the rows and the checkpoint move in one `commit`
// ---------------------------------------------------------------------------

/// The assertion the whole port exists to make true: after a run, the read
/// model and the checkpoint are **both** present or **both** absent.
async fn rows_and_checkpoint_agree(models: &MemoryProjectionStore, id: &ProjectionId) -> bool {
    let has_rows = !models.is_empty();
    let has_checkpoint = !matches!(
        models.checkpoint(id).await.expect("the oracle cannot fail"),
        Checkpoint::NeverRun
    );
    has_rows == has_checkpoint
}

#[tokio::test]
async fn read_model_and_checkpoint_commit_together() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 12), dispatched("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");

    // Before the run: neither half exists.
    assert!(rows_and_checkpoint_agree(&models, projection.id()).await);

    let progress = run_projection(&events, &models, &mut projection, &Json, chunk(1))
        .await
        .expect("the runner completes");

    // After it: both do, and the checkpoint names a position the store really
    // assigned — never a literal, because positions may gap.
    assert!(rows_and_checkpoint_agree(&models, projection.id()).await);
    let last = events
        .snapshot()
        .last()
        .expect("the fixture appended")
        .position;
    assert_eq!(progress.through, Some(last));
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::Live { through: last }
    );
}

/// The named wrong implementation: a runner that advances the checkpoint with
/// an **empty** batch, discarding the read-model writes `apply` made.
///
/// It lives here rather than in the library because its only job is to fail the
/// assertion above — a rule no implementation can fail is decorative.
async fn checkpoint_only_runner(
    events: &MemoryEventStore,
    models: &MemoryProjectionStore,
    projection: &mut VanStock,
) {
    let query = expected_query(projection.scope());
    let read = happenstance::collect(EventStore::read(events, &query, ReadOptions::new()))
        .await
        .expect("the fixture reads");

    for event in read {
        let mut batch = models.begin();
        let decoded: Stock = Json
            .decode(event.event.data())
            .expect("the fixture decodes");
        projection
            .apply(decoded, &mut batch)
            .expect("the oracle cannot fail");
        // The defect, in one line: the batch is dropped and a fresh, empty one
        // carries the checkpoint forward.
        let empty = models.begin();
        models
            .commit(
                empty,
                projection.id(),
                event.position,
                happenstance::Authority::Live,
            )
            .await
            .expect("the oracle commits");
    }
}

#[tokio::test]
async fn checkpoint_without_rows_is_rejected() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 12), dispatched("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");

    checkpoint_only_runner(&events, &models, &mut projection).await;

    assert!(
        !rows_and_checkpoint_agree(&models, projection.id()).await,
        "the atomicity assertion is decorative: a checkpoint-only runner passed it"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — resume, first run, and gaps
// ---------------------------------------------------------------------------

#[tokio::test]
async fn first_run_starts_from_the_beginning() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3), delivered("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");

    // `Checkpoint::NeverRun` means never run — not "caught up", and not
    // position zero.
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::NeverRun
    );

    let progress = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect("the runner completes");

    assert_eq!(progress.applied, 2);
    assert_eq!(projection.seen.len(), 2);
}

#[tokio::test]
async fn resume_advances_past_the_checkpoint() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3), delivered("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut first = VanStock::for_depot("d7");
    let _ = run_projection(&events, &models, &mut first, &Json, chunk(8))
        .await
        .expect("the first run completes");

    // A second run over an unchanged log applies nothing: `from` is inclusive,
    // so a runner feeding the checkpoint straight in would re-apply the last
    // event on every run.
    let mut idle = VanStock::for_depot("d7");
    let progress = run_projection(&events, &models, &mut idle, &Json, chunk(8))
        .await
        .expect("the idle run completes");
    assert_eq!(progress.applied, 0);
    assert!(idle.seen.is_empty());
    assert_eq!(progress.through, None);

    // One more event, and only that one is applied.
    seed(&events, &[dispatched("d7", 2)]).await;
    let mut third = VanStock::for_depot("d7");
    let progress = run_projection(&events, &models, &mut third, &Json, chunk(8))
        .await
        .expect("the resumed run completes");

    assert_eq!(third.seen, vec![dispatched("d7", 2)]);
    let last = events.snapshot().last().expect("three events").position;
    assert_eq!(progress.through, Some(last));
}

#[tokio::test]
async fn tolerates_gapped_positions() {
    // Stride seven: nothing about the runner may assume the next position is
    // the next integer.
    let stride = NonZeroU64::new(7).expect("seven is not zero");
    let events = GappyMemoryStore::with_stride(stride);
    for event in [delivered("d7", 3), delivered("d7", 4)] {
        EventStore::append(&events, &[landed(&event)], None)
            .await
            .expect("the gappy fixture seeds");
    }

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");
    let first = run_projection(&events, &models, &mut projection, &Json, chunk(1))
        .await
        .expect("the gapped run completes");
    assert_eq!(first.applied, 2);

    // Resume across a gap: the third event sits at a position two above the
    // checkpoint's successor, and it is still the only one applied.
    let third = EventStore::append(&events, &[landed(&dispatched("d7", 2))], None)
        .await
        .expect("the gappy fixture appends");
    let mut resumed = VanStock::for_depot("d7");
    let progress = run_projection(&events, &models, &mut resumed, &Json, chunk(1))
        .await
        .expect("the resumed gapped run completes");

    assert_eq!(resumed.seen, vec![dispatched("d7", 2)]);
    assert_eq!(progress.through, Some(third));
}

// ---------------------------------------------------------------------------
// AC-005 — the runner streams
// ---------------------------------------------------------------------------

#[tokio::test]
async fn commits_before_the_stream_ends() {
    let source = MemoryEventStore::new();
    seed(
        &source,
        &[
            delivered("d7", 1),
            delivered("d7", 1),
            delivered("d7", 1),
            delivered("d7", 1),
            delivered("d7", 1),
        ],
    )
    .await;

    let models = MemoryProjectionStore::new();
    let events = PullWatching {
        events: source.snapshot(),
        rows_at_pull: RefCell::new(Vec::new()),
        models: &models,
    };
    let mut projection = VanStock::for_depot("d7");

    let _ = run_projection(&events, &models, &mut projection, &Json, chunk(2))
        .await
        .expect("the runner completes");

    let rows = events.rows_at_pull.borrow();
    assert_eq!(rows.len(), 5, "every seeded event was pulled");
    assert_eq!(rows[0], 0, "nothing is committed before the first pull");
    assert!(
        rows.last().copied().unwrap_or(0) > 0,
        "the runner buffered the whole log: no commit was observed before the \
         final event was pulled, which is what `collect` looks like from here"
    );
}

// ---------------------------------------------------------------------------
// AC-006 — a decode failure names its position and discards the chunk
// ---------------------------------------------------------------------------

/// An event of a declared type whose payload no codec here can read.
fn undecodable() -> Event {
    Event::new(DELIVERED, &b"not json"[..])
        .expect("a valid event type")
        .with_tags(depot_tags("d7"))
}

#[tokio::test]
async fn decode_failure_names_its_position_and_rolls_back() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3), delivered("d7", 4)]).await;
    EventStore::append(&events, &[undecodable()], None)
        .await
        .expect("the fixture appends the poisoned event");
    let poisoned = events.snapshot().last().expect("three events").position;

    // One chunk over the whole log: the two good events are applied into the
    // batch, the third fails, and the batch is discarded whole.
    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");
    let error = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect_err("the poisoned event stops the runner");

    match &error {
        ProjectionError::Decode {
            position, source, ..
        } => {
            assert_eq!(*position, poisoned);
            assert!(matches!(source, CodecError::Decode(_)));
        }
        other => panic!("expected a decode failure, got {other:?}"),
    }
    assert_eq!(error.position(), Some(poisoned));
    assert_eq!(error.progress(), Progressed::default());

    // The `#[source]` chain is walkable to the codec's own refusal.
    let source = core::error::Error::source(&error).expect("a typed source");
    assert!(source.downcast_ref::<CodecError>().is_some());

    // Nothing was committed, so the checkpoint did not move.
    assert!(models.is_empty());
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::NeverRun
    );

    // With a chunk of two, the first chunk is durable and the checkpoint sits
    // at the last **good** position rather than at the poisoned one.
    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");
    let error = run_projection(&events, &models, &mut projection, &Json, chunk(2))
        .await
        .expect_err("the poisoned event stops the runner");

    let good = events.snapshot()[1].position;
    assert_eq!(error.position(), Some(poisoned));
    assert_eq!(error.progress().applied, 2);
    assert_eq!(error.progress().through, Some(good));
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::Live { through: good }
    );
}
