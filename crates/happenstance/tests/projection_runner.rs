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
    AppendCondition, AppendError, Checkpoint, Chunk, Codec, CodecError, Delivered, DomainEvent,
    Event, EventId, EventStore, EventType, InvalidCheckpointId, Json, MemoryEventStore,
    MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError, MemoryStoreError,
    Progressed, Projection, ProjectionError, ProjectionId, ProjectionStore, Query, QueryItem,
    ReadOptions, SequencePosition, SequencedEvent, Tags, checkpoint_id, run_projection,
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
            id: ProjectionId::from_static("van_stock"),
            scope: depot_tags(depot),
            seen: Vec::new(),
            totals: BTreeMap::new(),
        }
    }
}

impl Projection for VanStock {
    type Event = Stock;
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
        event: Delivered<Self::Event>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        // The match is on the application's **own enum**, not on an
        // `EventType` and not on `Bytes`. A signature taking `Bytes` would not
        // compile against this impl.
        let event = event.into_event();
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

fn chunk(n: usize) -> Chunk {
    Chunk::of(NonZeroUsize::new(n).expect("a non-zero chunk"))
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
    let key = checkpoint_id(&projection).expect("a derivable id");

    // Before the run: neither half exists.
    assert!(rows_and_checkpoint_agree(&models, &key).await);

    let progress = run_projection(&events, &models, &mut projection, &Json, chunk(1))
        .await
        .expect("the runner completes");

    // After it: both do, and the checkpoint names a position the store really
    // assigned — never a literal, because positions may gap.
    assert!(rows_and_checkpoint_agree(&models, &key).await);
    let last = events
        .snapshot()
        .last()
        .expect("the fixture appended")
        .position;
    assert_eq!(progress.through, Some(last));
    assert_eq!(
        models.checkpoint(&key).await.unwrap(),
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
        let mut batch = models.begin().await.unwrap();
        let decoded: Stock = Json
            .decode(event.event.data())
            .expect("the fixture decodes");
        projection
            .apply(Delivered::new(event.id, decoded), &mut batch)
            .await
            .expect("the oracle cannot fail");
        // The defect, in one line: the batch is dropped and a fresh, empty one
        // carries the checkpoint forward.
        let empty = models.begin().await.unwrap();
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
    let key = checkpoint_id(&projection).expect("a derivable id");
    assert_eq!(models.checkpoint(&key).await.unwrap(), Checkpoint::NeverRun);

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

/// ADR-0070 §4 — one `Chunk` value serves every view it is handed to.
///
/// The wrong implementation this rejects is a `Chunk` that is not `Copy` —
/// one that grew a boxed policy, say — which a fan-out runner would have to
/// clone or rebuild per view. Here one binding is passed to two projections
/// and then a third time, which does not compile (`E0382`) unless it copies.
#[tokio::test]
async fn one_chunk_serves_every_view() {
    let events = MemoryEventStore::new();
    seed(
        &events,
        &[delivered("d7", 3), delivered("d8", 5), dispatched("d7", 1)],
    )
    .await;
    let positions: Vec<_> = events.snapshot().iter().map(|e| e.position).collect();
    let [_, d8_last, d7_last] = positions[..] else {
        panic!("three events were seeded");
    };

    let models = MemoryProjectionStore::new();
    let per_commit = chunk(2);
    let mut d7 = VanStock {
        id: ProjectionId::from_static("van_stock_d7"),
        ..VanStock::for_depot("d7")
    };
    let mut d8 = VanStock {
        id: ProjectionId::from_static("van_stock_d8"),
        ..VanStock::for_depot("d8")
    };

    let seven = run_projection(&events, &models, &mut d7, &Json, per_commit)
        .await
        .expect("the d7 view completes");
    let eight = run_projection(&events, &models, &mut d8, &Json, per_commit)
        .await
        .expect("the d8 view completes");
    let again = run_projection(&events, &models, &mut d7, &Json, per_commit)
        .await
        .expect("the idle d7 run completes");

    assert_eq!((seven.applied, eight.applied, again.applied), (2, 1, 0));
    assert_eq!(models.get("d7"), Some(2));
    assert_eq!(models.get("d8"), Some(5));
    // Each view's checkpoint is its own: the last event its scope matched.
    for (view, through) in [(&d7, d7_last), (&d8, d8_last)] {
        assert_eq!(
            models
                .checkpoint(&checkpoint_id(view).expect("a derivable id"))
                .await
                .unwrap(),
            Checkpoint::Live { through }
        );
    }
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

/// EC-005 — the resume point has no successor.
///
/// [`SequencePosition::next`] is `checked_add` rather than `saturating_add`
/// precisely so this state is representable, and the runner turns its `None`
/// into an error arm rather than an `unwrap`. Nothing else in the suite reaches
/// that arm, which left the reasoning in `event.rs` with an instrument on the
/// contract side of the port and none on this one.
///
/// The checkpoint is seeded through the **real** port — a commit at the last
/// representable position — so no double stands in for a store here.
#[tokio::test]
async fn a_checkpoint_at_the_last_position_reports_exhausted_key_space() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3), delivered("d7", 4)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");
    let key = checkpoint_id(&projection).expect("a derivable id");

    // A projection that has run to the top of the key space: rows written, and
    // a checkpoint at the last position anything can occupy.
    let last = SequencePosition::new(u64::MAX).expect("the last position is not zero");
    let mut batch = models.begin().await.unwrap();
    batch.write("d7", 7);
    models
        .commit(batch, &key, last, happenstance::Authority::Live)
        .await
        .expect("the oracle commits");

    let error = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect_err("there is no position past the last one to resume from");

    match &error {
        ProjectionError::KeySpaceExhausted { through } => assert_eq!(*through, last),
        other => panic!("expected exhausted key space, got {other:?}"),
    }
    // The failure names the position it could not advance past, and reports no
    // progress: nothing was read, so nothing was applied.
    assert_eq!(error.position(), Some(last));
    assert_eq!(error.progress(), Progressed::default());
    assert!(
        projection.seen.is_empty(),
        "the runner read the log anyway, which is the `saturating_add` \
         behaviour: a resume from the last position that re-reads it for ever"
    );

    // And it stopped without touching either half of the store.
    assert_eq!(models.get("d7"), Some(7));
    assert_eq!(
        models.checkpoint(&key).await.unwrap(),
        Checkpoint::Live { through: last }
    );
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
    let key = checkpoint_id(&projection).expect("a derivable id");
    assert_eq!(models.checkpoint(&key).await.unwrap(), Checkpoint::NeverRun);

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
        models.checkpoint(&key).await.unwrap(),
        Checkpoint::Live { through: good }
    );
}

// ---------------------------------------------------------------------------
// PS-28 — a projection fails with its own error, at the event that failed
// ---------------------------------------------------------------------------

/// The projection's own refusal, which no store error could carry.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    #[error("refused a delivery of {units} units")]
    Refused { units: u64 },
    #[error(transparent)]
    Store(#[from] MemoryProjectionStoreError),
}

/// Units delivered to one depot, refusing any delivery of exactly five.
struct RefusesFive {
    id: ProjectionId,
    scope: Tags,
    total: u64,
}

impl Projection for RefusesFive {
    type Event = Stock;
    type Store = MemoryProjectionStore;
    type Error = Refusal;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Self::Event>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Refusal> {
        if let Stock::Delivered { units, .. } = event.into_event() {
            if units == 5 {
                return Err(Refusal::Refused { units });
            }
            self.total += units;
            batch.write("d7", self.total);
        }
        Ok(())
    }
}

/// PS-28's rule. *The last good position* is **the last committed one**: a
/// failure mid-chunk discards the chunk, so the checkpoint and `progress` sit
/// at the previous chunk boundary, while `position` names the event whose
/// `apply` refused. Wrong implementations each assertion rejects: reporting
/// the last *applied* position (p4) fails `position`; committing the partial
/// chunk fails the checkpoint and the row; counting applied-but-uncommitted
/// events fails `progress`; and a runner whose `Apply` arm carries the
/// store's error cannot hold a `Refusal` at all.
#[tokio::test]
async fn pump_reports_the_failing_position() {
    let events = MemoryEventStore::new();
    let deliveries: Vec<Stock> = (1..=5).map(|units| delivered("d7", units)).collect();
    seed(&events, &deliveries).await;
    let snapshot = events.snapshot();
    let [_, _, third, _, fifth] = snapshot.as_slice() else {
        panic!("five events were seeded, found {}", snapshot.len());
    };
    let (committed, refused) = (third.position, fifth.position);

    let models = MemoryProjectionStore::new();
    let mut projection = RefusesFive {
        id: ProjectionId::from_static("refuses_five"),
        scope: depot_tags("d7"),
        total: 0,
    };
    let error = run_projection(&events, &models, &mut projection, &Json, chunk(3))
        .await
        .expect_err("the refused delivery stops the runner");

    assert!(
        matches!(
            &error,
            ProjectionError::Apply {
                position,
                source: Refusal::Refused { units: 5 },
                rollback: None,
                ..
            } if *position == refused
        ),
        "expected the projection's own refusal at the fifth event, got {error:?}"
    );
    assert_eq!(error.position(), Some(refused));
    let progress = error.progress();
    assert_eq!(progress.through, Some(committed));
    assert_eq!(progress.applied, 3);

    let key = checkpoint_id(&projection).expect("a derivable id");
    assert_eq!(
        models
            .checkpoint(&key)
            .await
            .expect("the memory store reads its checkpoint"),
        Checkpoint::Live { through: committed }
    );
    assert_eq!(models.get("d7"), Some(1 + 2 + 3));

    // The `#[source]` chain reaches the projection's own error, not the store's.
    let source = core::error::Error::source(&error).expect("a typed source");
    assert!(matches!(
        source.downcast_ref::<Refusal>(),
        Some(Refusal::Refused { units: 5 })
    ));
}

// ---------------------------------------------------------------------------
// ADR-0074 — a buffered run costs nothing for `apply` being `async`
// ---------------------------------------------------------------------------

/// The **runner**, not `apply` alone, polled once under a no-op waker.
///
/// A projection whose `apply` never awaits is ready at its first poll by
/// async-fn semantics alone, so polling `apply` by itself could not fail.
/// Polling the whole run over the two memory stores can: an await that yields
/// anywhere on the runner's path returns `Pending` here, and
/// `a_run_whose_apply_yields_is_pending_at_its_first_poll` is the control that
/// shows it would. No tokio: a `Waker::noop` context is the whole executor.
#[test]
fn a_buffered_run_is_ready_at_its_first_poll_without_a_runtime() {
    let events =
        MemoryEventStore::with_events([landed(&delivered("d7", 3)), landed(&delivered("d7", 4))]);
    let models = MemoryProjectionStore::new();
    let mut stock = VanStock::for_depot("d7");

    let mut context = Context::from_waker(core::task::Waker::noop());
    let run = core::pin::pin!(run_projection(
        &events,
        &models,
        &mut stock,
        &Json,
        chunk(1)
    ));
    let outcome = run.poll(&mut context);

    let Poll::Ready(Ok(progress)) = outcome else {
        panic!("a buffered run must complete on its first poll: {outcome:?}");
    };
    assert_eq!(progress.applied, 2);
    assert_eq!(progress.through, events.last_position());
}

/// A future that is `Pending` once, having woken itself, and then `Ready`.
struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// `VanStock`, except that `apply` yields once before it writes: the shape of
/// a projection awaiting a statement on a live batch.
#[derive(Debug)]
struct Yielding(VanStock);

impl Projection for Yielding {
    type Event = Stock;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        self.0.id()
    }

    fn scope(&self) -> &Tags {
        self.0.scope()
    }

    async fn apply(
        &mut self,
        event: Delivered<Stock>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        YieldOnce(false).await;
        self.0.apply(event, batch).await
    }
}

/// The control that keeps the first-poll test above from being decorative.
///
/// The same run, the same stores, one difference: `apply` yields. The first
/// poll is `Pending`, so the instrument can tell a run that yields from one
/// that does not, and the buffered run being `Ready` is a measurement.
#[test]
fn a_run_whose_apply_yields_is_pending_at_its_first_poll() {
    let events =
        MemoryEventStore::with_events([landed(&delivered("d7", 3)), landed(&delivered("d7", 4))]);
    let models = MemoryProjectionStore::new();
    let mut stock = Yielding(VanStock::for_depot("d7"));

    let mut context = Context::from_waker(core::task::Waker::noop());
    let run = core::pin::pin!(run_projection(
        &events,
        &models,
        &mut stock,
        &Json,
        chunk(1)
    ));
    let outcome = run.poll(&mut context);

    assert!(
        outcome.is_pending(),
        "an `apply` that yields must make the run yield: {outcome:?}"
    );
}

// ---------------------------------------------------------------------------
// SY-21 — `apply` is handed the origin's identity, not the local position
// ---------------------------------------------------------------------------

/// Records the identity of every event it is handed, and nothing else.
#[derive(Debug)]
struct Identities {
    id: ProjectionId,
    scope: Tags,
    seen: Vec<EventId>,
}

impl Projection for Identities {
    type Event = Stock;
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
        event: Delivered<Stock>,
        _: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.seen.push(event.id());
        Ok(())
    }
}

/// A restored log whose local order is not its origin's.
///
/// Locally the three events sit at 1, 2 and 3; the store that originated them
/// gave them 30, 10 and 20. The wrong implementation this rejects is a runner
/// that re-mints the identity from what it read locally,
/// `EventId::new(local_store, event.position)`: every id it hands over would
/// name the wrong store and the wrong position.
#[tokio::test]
async fn apply_is_handed_the_origin_identity_not_the_local_position() {
    let origin = happenstance::StoreId::from_bytes([7; 16]);
    let local = happenstance::StoreId::from_bytes([1; 16]);
    let log: Vec<SequencedEvent> = [(1, 30), (2, 10), (3, 20)]
        .into_iter()
        .map(|(here, there)| {
            SequencedEvent::new(
                SequencePosition::new(here).expect("a non-zero local position"),
                EventId::new(
                    origin,
                    SequencePosition::new(there).expect("a non-zero origin position"),
                ),
                happenstance::RecordedAt::from_millis(0),
                landed(&delivered("d7", here)),
            )
        })
        .collect();
    let expected: Vec<EventId> = log.iter().map(|event| event.id).collect();
    let events = MemoryEventStore::restore(local, log);
    let models = MemoryProjectionStore::new();
    let mut identities = Identities {
        id: ProjectionId::from_static("identities"),
        scope: depot_tags("d7"),
        seen: Vec::new(),
    };

    let progressed = run_projection(&events, &models, &mut identities, &Json, chunk(8))
        .await
        .expect("the runner completes");
    assert_eq!(progressed.applied, 3, "every restored event was applied");

    assert_eq!(
        identities.seen, expected,
        "`apply` saw the origin's identities, in local order"
    );
    assert!(
        identities.seen.iter().all(|id| id.store() == origin),
        "no identity was re-minted under the local store"
    );
}

// ---------------------------------------------------------------------------
// PS-25 — the runner commits under a derived checkpoint id
// ---------------------------------------------------------------------------

/// The run reads and commits under `checkpoint_id`, and never under the name.
///
/// The wrong implementations this rejects are a runner that reads under the
/// derived id and commits under the bare one, which leaves the derived row
/// `NeverRun`, and the reverse, which leaves the bare row `Live`. The digest
/// was computed outside Rust, from the encoding table on `checkpoint_id`.
#[tokio::test]
async fn the_runner_commits_under_the_derived_id() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3), dispatched("d7", 1)]).await;
    let last = events.snapshot().last().expect("two events").position;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock::for_depot("d7");
    let key = checkpoint_id(&projection).expect("a derivable id");
    assert_eq!(key.as_str(), "van_stock@dc525f3f5d262b19");

    let _ = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect("the runner completes");

    assert_eq!(
        models.checkpoint(&key).await.unwrap(),
        Checkpoint::Live { through: last }
    );
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::NeverRun,
        "the runner wrote under the bare name"
    );

    // And it resumes under the id it committed under: a second run applies
    // nothing.
    let mut again = VanStock::for_depot("d7");
    let idle = run_projection(&events, &models, &mut again, &Json, chunk(8))
        .await
        .expect("the idle run completes");
    assert_eq!(idle.applied, 0);
}

/// `Stock` narrowed to its deliveries: the query a view had before it was
/// widened to dispatches too.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DeliveriesOnly(Stock);

impl DomainEvent for DeliveriesOnly {
    const EVENT_TYPES: &'static [EventType] = &[DELIVERED];

    fn event_type(&self) -> EventType {
        self.0.event_type()
    }

    fn tags(&self) -> Tags {
        self.0.tags()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        self.0.encode(codec)
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
        Stock::decode(codec, event_type, data).map(Self)
    }
}

/// One view named `view`, over whichever event type `E` it is given.
#[derive(Debug)]
struct View<E> {
    id: ProjectionId,
    scope: Tags,
    seen: Vec<E>,
}

impl<E> View<E> {
    fn scoped(scope: Tags) -> Self {
        Self {
            id: ProjectionId::from_static("view"),
            scope,
            seen: Vec::new(),
        }
    }
}

impl<E: DomainEvent> Projection for View<E> {
    type Event = E;
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
        event: Delivered<E>,
        _: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.seen.push(event.into_event());
        Ok(())
    }
}

/// PS-25's rule: a projection whose query changed starts a new checkpoint.
///
/// `v1` sees deliveries; `v2`, under the same name, is widened to dispatches
/// too, and must be handed the dispatch that sits **below** `v1`'s
/// checkpoint. Wrong implementations each assertion rejects: today's runner,
/// keyed on the name alone, resumes `v2` past the dispatch and applies
/// nothing; a digest over the event types alone gives `v3`, which changed
/// only its scope, `v1`'s checkpoint; a digest over the name alone fails
/// every assertion.
#[tokio::test]
async fn changed_query_starts_a_new_checkpoint() {
    let events = MemoryEventStore::new();
    seed(
        &events,
        &[delivered("a", 1), dispatched("a", 1), delivered("a", 2)],
    )
    .await;
    let snapshot = events.snapshot();
    let [_, _, last] = snapshot.as_slice() else {
        panic!("three events were seeded, found {}", snapshot.len());
    };

    let models = MemoryProjectionStore::new();
    let mut v1 = View::<DeliveriesOnly>::scoped(Tags::empty());
    let first = run_projection(&events, &models, &mut v1, &Json, chunk(8))
        .await
        .expect("the narrow view completes");
    assert_eq!(first.applied, 2);
    let v1_key = checkpoint_id(&v1).expect("a derivable id");
    assert_eq!(
        models.checkpoint(&v1_key).await.unwrap(),
        Checkpoint::Live {
            through: last.position
        }
    );

    let mut v2 = View::<Stock>::scoped(Tags::empty());
    let v2_key = checkpoint_id(&v2).expect("a derivable id");
    assert_eq!(v1.id(), v2.id(), "one name, two queries");
    assert_ne!(v2_key, v1_key);
    assert_eq!(
        models.checkpoint(&v2_key).await.unwrap(),
        Checkpoint::NeverRun
    );

    let widened = run_projection(&events, &models, &mut v2, &Json, chunk(8))
        .await
        .expect("the widened view completes");
    assert_eq!(
        widened.applied, 3,
        "the widened view replayed from the start"
    );
    assert_eq!(
        v2.seen,
        vec![delivered("a", 1), dispatched("a", 1), delivered("a", 2)]
    );

    // A scope change is a query change.
    let v3 = View::<DeliveriesOnly>::scoped(depot_tags("a"));
    let v3_key = checkpoint_id(&v3).expect("a derivable id");
    assert_ne!(v3_key, v1_key);
    assert_eq!(
        models.checkpoint(&v3_key).await.unwrap(),
        Checkpoint::NeverRun
    );
}

/// A name with no room for the digest stops the run before anything is read.
///
/// The wrong implementations this rejects are a runner that truncates the
/// name to fit, which applies the event and commits under an id two
/// projections could share, and one that reports the refusal through another
/// arm, which a caller matching on `CheckpointId` would never see.
#[tokio::test]
async fn a_name_too_long_for_a_checkpoint_id_stops_the_run() {
    let events = MemoryEventStore::new();
    seed(&events, &[delivered("d7", 3)]).await;

    let models = MemoryProjectionStore::new();
    let mut projection = VanStock {
        id: ProjectionId::new("n".repeat(239)).expect("a valid 239-byte id"),
        ..VanStock::for_depot("d7")
    };
    assert_eq!(
        checkpoint_id(&projection),
        Err(InvalidCheckpointId::NameTooLong { len: 239 })
    );

    let error = run_projection(&events, &models, &mut projection, &Json, chunk(8))
        .await
        .expect_err("a name with no room for the digest stops the run");

    assert!(
        matches!(
            &error,
            ProjectionError::CheckpointId(InvalidCheckpointId::NameTooLong { len: 239 })
        ),
        "expected the checkpoint-id refusal, got {error:?}"
    );
    assert_eq!(error.progress(), Progressed::default());
    assert_eq!(error.position(), None);
    assert!(projection.seen.is_empty(), "the run read the log anyway");
    assert!(models.is_empty(), "the run wrote rows anyway");
    assert_eq!(
        models.checkpoint(projection.id()).await.unwrap(),
        Checkpoint::NeverRun
    );
}
