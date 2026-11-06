//! One domain, one projection type over it, and two instruments.
//!
//! `Shard` is the only projection type: a fan-out runner drives N views of
//! one type, and each test builds its views from it with a different name,
//! scope, policy and fault. `Watched` and `Instrumented` wrap the memory
//! stores and record what the runner did to them, so a test asserts on the
//! runner's calls and not only on what they left behind.
//!
//! Positions always come from the store's own snapshot and never from a
//! literal: the specification permits gaps.

use core::cell::{Cell, RefCell};
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::BTreeMap;
use std::rc::Rc;

use happenstance::bytes::Bytes;
use happenstance::futures_core::Stream;
use happenstance::{
    AppendCondition, AppendError, ApplyFailure, Authority, Checkpoint, Chunk, Codec, CodecError,
    CommitError, Delivered, DomainEvent, Event, EventId, EventStore, EventType, Json,
    MemoryEventStore, MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError,
    MemoryStoreError, Policy, Projection, ProjectionId, ProjectionStore, Query, ReadOptions,
    ResetError, SequencePosition, SequencedEvent, Tags, checkpoint_id,
};

// ---------------------------------------------------------------------------
// The domain
// ---------------------------------------------------------------------------

pub(crate) const SHREDDED: EventType = EventType::from_static("PartShredded");

/// A part, shredded, in one of a few shards.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum Part {
    Shredded { vin: String, shard: u8 },
}

pub(crate) fn shard_tags(shard: u8) -> Tags {
    Tags::from_pairs([("shard", shard.to_string().as_str())]).expect("a valid shard tag")
}

impl DomainEvent for Part {
    const EVENT_TYPES: &'static [EventType] = &[SHREDDED];

    fn event_type(&self) -> EventType {
        SHREDDED
    }

    fn tags(&self) -> Tags {
        let Self::Shredded { shard, .. } = self;
        shard_tags(*shard)
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// A part in shard 0.
pub(crate) fn part(vin: &str) -> Event {
    part_in(vin, 0)
}

/// A part in the named shard.
pub(crate) fn part_in(vin: &str, shard: u8) -> Event {
    let value = Part::Shredded {
        vin: vin.to_owned(),
        shard,
    };
    let payload = Json.encode(&value).expect("the fixture encodes");
    Event::new(SHREDDED, payload)
        .expect("a valid event type")
        .with_tags(value.tags())
}

/// An event of the declared type, in shard 0, whose payload no codec reads.
pub(crate) fn poisoned() -> Event {
    poisoned_in(0)
}

/// An unreadable payload in the named shard.
pub(crate) fn poisoned_in(shard: u8) -> Event {
    Event::new(SHREDDED, &b"shredded, and so is the payload"[..])
        .expect("a valid event type")
        .with_tags(shard_tags(shard))
}

pub(crate) fn chunk(n: usize) -> Chunk {
    Chunk::of(NonZeroUsize::new(n).expect("a non-zero chunk"))
}

// ---------------------------------------------------------------------------
// The projection
// ---------------------------------------------------------------------------

/// What `Shard::on_error` answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Answers {
    /// Answers `Halt` and writes nothing, as the provided default does.
    Halt,
    /// Records the failure under `<name>:skipped:<id>` and skips it.
    Skip,
}

/// The projection's own refusal.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ShardError {
    #[error("refused the part")]
    Refused,
    #[error(transparent)]
    Store(#[from] WatchedError),
}

/// One view: rows keyed `<name>:<vin>`, valued by how many it had applied.
///
/// The value is a running count rather than `1`, so a view that applied an
/// event twice, or in another order, leaves a different row.
#[derive(Debug)]
pub(crate) struct Shard {
    pub(crate) id: ProjectionId,
    pub(crate) name: &'static str,
    pub(crate) scope: Tags,
    pub(crate) answers: Answers,
    /// The VIN whose `apply` writes its row, marks the batch, then panics.
    pub(crate) panic_on: Option<&'static str>,
    /// The VIN whose `apply` refuses before writing.
    pub(crate) refuse_on: Option<&'static str>,
    pub(crate) applied: u64,
}

impl Shard {
    /// A view over every shard that halts on any failure.
    pub(crate) fn new(name: &'static str) -> Self {
        Self {
            id: ProjectionId::from_static(name),
            name,
            scope: Tags::empty(),
            answers: Answers::Halt,
            panic_on: None,
            refuse_on: None,
            applied: 0,
        }
    }

    pub(crate) fn in_shard(mut self, shard: u8) -> Self {
        self.scope = shard_tags(shard);
        self
    }

    pub(crate) const fn skipping(mut self) -> Self {
        self.answers = Answers::Skip;
        self
    }

    pub(crate) const fn panicking_on(mut self, vin: &'static str) -> Self {
        self.panic_on = Some(vin);
        self
    }

    pub(crate) const fn refusing(mut self, vin: &'static str) -> Self {
        self.refuse_on = Some(vin);
        self
    }

    /// The same view, fresh: the same configuration and no state.
    pub(crate) fn fresh(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name,
            scope: self.scope.clone(),
            answers: self.answers,
            panic_on: self.panic_on,
            refuse_on: self.refuse_on,
            applied: 0,
        }
    }

    pub(crate) fn row(&self, vin: &str) -> String {
        format!("{}:{vin}", self.name)
    }

    pub(crate) fn skip_row(&self, id: EventId) -> String {
        format!("{}:skipped:{id}", self.name)
    }
}

impl Projection for Shard {
    type Event = Part;
    type Store = Watched;
    type Error = ShardError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Part>,
        batch: &mut WatchedBatch,
    ) -> Result<(), ShardError> {
        let Part::Shredded { vin, .. } = event.into_event();
        if self.refuse_on == Some(vin.as_str()) {
            return Err(ShardError::Refused);
        }
        self.applied += 1;
        batch.write(self.row(&vin), self.applied);
        if self.panic_on == Some(vin.as_str()) {
            batch.mark_panicking();
            panic!("`{}` panics inside apply on `{vin}`", self.name);
        }
        Ok(())
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, ShardError>,
        batch: &mut WatchedBatch,
    ) -> Result<Policy, ShardError> {
        match self.answers {
            Answers::Halt => Ok(Policy::Halt),
            Answers::Skip => {
                batch.write(self.skip_row(failure.id()), 1);
                Ok(Policy::Skip)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// `Watched`: a projection store that logs every write set's life
// ---------------------------------------------------------------------------

/// One thing the runner did to a write set, by its serial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Call {
    Begin(u32),
    Commit(u32),
    Rollback(u32),
    /// Dropped without a commit or a rollback.
    Dropped(u32),
    /// The projection is about to panic with this write set open.
    Panicking(u32),
}

impl Call {
    pub(crate) const fn serial(self) -> u32 {
        match self {
            Self::Begin(s)
            | Self::Commit(s)
            | Self::Rollback(s)
            | Self::Dropped(s)
            | Self::Panicking(s) => s,
        }
    }

    /// Whether this call ends the write set it names.
    pub(crate) const fn ends(self) -> bool {
        matches!(self, Self::Commit(_) | Self::Rollback(_) | Self::Dropped(_))
    }
}

/// The instrument's own failure, injected on request.
#[derive(Debug, thiserror::Error)]
pub(crate) enum WatchedError {
    #[error("an injected failure")]
    Injected,
    #[error(transparent)]
    Store(#[from] MemoryProjectionStoreError),
}

/// A `MemoryProjectionStore` that logs every `begin`, `commit`, `rollback`
/// and bare drop, and fails one `begin` or one view's commits on request.
#[derive(Debug, Default)]
pub(crate) struct Watched {
    inner: MemoryProjectionStore,
    log: Rc<RefCell<Vec<Call>>>,
    serials: Cell<u32>,
    /// The zero-based `begin` that fails, counted across every view.
    pub(crate) fail_begin: Option<usize>,
    begins: Cell<usize>,
    /// The checkpoint id whose commits fail.
    pub(crate) fail_commit: Option<ProjectionId>,
}

impl Watched {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// A store whose zero-based `nth` `begin`, across every view, fails.
    pub(crate) fn failing_begin(nth: usize) -> Self {
        Self {
            fail_begin: Some(nth),
            ..Self::default()
        }
    }

    /// A store whose commits under `id` fail.
    pub(crate) fn failing_commit(id: ProjectionId) -> Self {
        Self {
            fail_commit: Some(id),
            ..Self::default()
        }
    }

    pub(crate) fn calls(&self) -> Vec<Call> {
        self.log.borrow().clone()
    }

    pub(crate) fn snapshot(&self) -> BTreeMap<String, u64> {
        self.inner.snapshot()
    }

    pub(crate) fn get(&self, key: &str) -> Option<u64> {
        self.inner.get(key)
    }

    pub(crate) async fn checkpoint_of(&self, view: &Shard) -> Checkpoint {
        self.checkpoint(&checkpoint_id(view).expect("a derivable id"))
            .await
            .expect("the memory store reads its checkpoint")
    }

    fn record(&self, call: Call) {
        self.log.borrow_mut().push(call);
    }
}

/// A memory write set that says when it ends, and when it is dropped bare.
#[derive(Debug)]
pub(crate) struct WatchedBatch {
    serial: u32,
    inner: Option<MemoryProjectionBatch>,
    log: Rc<RefCell<Vec<Call>>>,
}

impl WatchedBatch {
    pub(crate) fn write(&mut self, key: impl Into<String>, value: u64) {
        self.inner
            .as_mut()
            .expect("a write set is written only while open")
            .write(key, value);
    }

    pub(crate) fn mark_panicking(&self) {
        self.log.borrow_mut().push(Call::Panicking(self.serial));
    }

    fn end(mut self, call: Call) -> MemoryProjectionBatch {
        self.log.borrow_mut().push(call);
        self.inner.take().expect("a write set ends once")
    }
}

impl Drop for WatchedBatch {
    fn drop(&mut self) {
        if self.inner.is_some() {
            self.log.borrow_mut().push(Call::Dropped(self.serial));
        }
    }
}

impl ProjectionStore for Watched {
    type Error = WatchedError;
    type Batch = WatchedBatch;

    async fn begin(&self) -> Result<WatchedBatch, WatchedError> {
        let nth = self.begins.get();
        self.begins.set(nth + 1);
        if self.fail_begin == Some(nth) {
            return Err(WatchedError::Injected);
        }
        let serial = self.serials.get();
        self.serials.set(serial + 1);
        let inner = self.inner.begin().await?;
        self.record(Call::Begin(serial));
        Ok(WatchedBatch {
            serial,
            inner: Some(inner),
            log: Rc::clone(&self.log),
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, WatchedError> {
        Ok(self.inner.checkpoint(id).await?)
    }

    async fn commit(
        &self,
        batch: WatchedBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<WatchedError>> {
        let serial = batch.serial;
        let inner = batch.end(Call::Commit(serial));
        if self.fail_commit.as_ref() == Some(id) {
            drop(inner);
            return Err(CommitError::Store(WatchedError::Injected));
        }
        self.inner
            .commit(inner, id, position, authority)
            .await
            .map_err(|error| match error {
                CommitError::ForeignBatch => CommitError::ForeignBatch,
                CommitError::CheckpointRegression { current, attempted } => {
                    CommitError::CheckpointRegression { current, attempted }
                }
                CommitError::Store(never) => match never {},
                other => panic!("an outcome this instrument does not know: {other:?}"),
            })
    }

    async fn reset(
        &self,
        batch: WatchedBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<WatchedError>> {
        let serial = batch.serial;
        let inner = batch.end(Call::Commit(serial));
        self.inner
            .reset(inner, id)
            .await
            .map_err(|error| match error {
                ResetError::ForeignBatch => ResetError::ForeignBatch,
                ResetError::Refused => ResetError::Refused,
                ResetError::Store(never) => match never {},
                other => panic!("an outcome this instrument does not know: {other:?}"),
            })
    }

    async fn rollback(&self, batch: WatchedBatch) -> Result<(), WatchedError> {
        let serial = batch.serial;
        let inner = batch.end(Call::Rollback(serial));
        Ok(self.inner.rollback(inner).await?)
    }
}

// ---------------------------------------------------------------------------
// `Instrumented`: an event store that counts reads, and can fail one
// ---------------------------------------------------------------------------

/// The instrument's own read failure.
#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub(crate) enum ReadFault {
    #[error("an injected read failure")]
    Injected,
}

/// A `MemoryEventStore` counting `read` calls and the items they yielded,
/// and failing every read after it has yielded `fail_after` items.
#[derive(Debug, Default)]
pub(crate) struct Instrumented {
    pub(crate) inner: MemoryEventStore,
    reads: Cell<usize>,
    delivered: Cell<usize>,
    pub(crate) fail_after: Option<usize>,
}

impl Instrumented {
    pub(crate) async fn seeded(log: &[Event]) -> Self {
        let inner = MemoryEventStore::new();
        EventStore::append(&inner, log, None)
            .await
            .expect("the fixture seeds");
        Self {
            inner,
            ..Self::default()
        }
    }

    pub(crate) fn reads(&self) -> usize {
        self.reads.get()
    }

    pub(crate) fn delivered(&self) -> usize {
        self.delivered.get()
    }

    pub(crate) fn snapshot(&self) -> Vec<SequencedEvent> {
        self.inner.snapshot()
    }
}

/// The read stream, counting as it is pulled.
struct Counted<'a, S> {
    inner: Pin<Box<S>>,
    delivered: &'a Cell<usize>,
    fail_after: Option<usize>,
    yielded: usize,
    failed: bool,
}

impl<S> Stream for Counted<'_, S>
where
    S: Stream<Item = Result<SequencedEvent, MemoryStoreError>>,
{
    type Item = Result<SequencedEvent, ReadFault>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.failed {
            return Poll::Ready(None);
        }
        if this.fail_after == Some(this.yielded) {
            this.failed = true;
            return Poll::Ready(Some(Err(ReadFault::Injected)));
        }
        match this.inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(event))) => {
                this.yielded += 1;
                this.delivered.set(this.delivered.get() + 1);
                Poll::Ready(Some(Ok(event)))
            }
            Poll::Ready(Some(Err(never))) => match never {},
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl EventStore for Instrumented {
    type Error = ReadFault;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, ReadFault>> {
        self.reads.set(self.reads.get() + 1);
        Counted {
            inner: Box::pin(EventStore::read(&self.inner, query, options)),
            delivered: &self.delivered,
            fail_after: self.fail_after,
            yielded: 0,
            failed: false,
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<ReadFault>> {
        EventStore::append(&self.inner, events, condition)
            .await
            .map_err(|error| error.map_store(|never| match never {}))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, ReadFault> {
        EventStore::head(&self.inner)
            .await
            .map_err(|never| match never {})
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, ReadFault> {
        EventStore::contains_event_id(&self.inner, id)
            .await
            .map_err(|never| match never {})
    }
}
