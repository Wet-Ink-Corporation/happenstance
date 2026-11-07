//! The fan-out runner: N views of one projection type over one read.
//!
//! Its own module rather than a section of `runner.rs`, so that file does not
//! grow above the lines the specification cites. It borrows the runner's
//! private vocabulary, the decode-and-apply step and the chunk tally, so a
//! view here takes an event exactly as `run_projection` would take it.

use core::convert::Infallible;

use happenstance_core::{
    Authority, EventStore, ProjectionId, ProjectionStore, Query, QueryItem, ReadOptions,
    SequencePosition, SequencedEvent,
};

use crate::boundary::{AtLeastOneType, derive_query};
use crate::checkpoint;
use crate::chunk::Chunk;
use crate::codec::Codec;
use crate::domain::DomainEvent;
use crate::runner::{
    Progressed, Projection, ProjectionError, StoreBatch, StoreError, Tally, apply_one,
    considered_through, next_event,
};

/// What one fan-out run did to each view, and to the shared read.
///
/// One entry per view, in the order the slice held them, so index `i` is
/// view `i`. Readable, not fabricable: `#[non_exhaustive]` blocks the struct
/// literal outside this crate, as it does for [`Progressed`].
///
/// `read` is `Some` only when the shared read failed. Then no entry is `Ok`:
/// a view still running at that moment is [`ViewError::Interrupted`], and a
/// view that had already stopped keeps the failure it stopped on.
#[derive(Debug)]
#[must_use = "a FanOut is where each view's failure is reported, and nowhere else"]
#[non_exhaustive]
// `R` is unbounded (API-21). `W` and `A` carry the bounds only because the
// field names `ViewError<W, A>`, whose `ProjectionError` field requires them:
// a type is well-formed only where its fields' types are.
pub struct FanOut<R, W, A>
where
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// One outcome per view, in the order the slice held them.
    pub views: Vec<Result<Progressed, ViewError<W, A>>>,
    /// The shared read's failure, if one ended the run.
    pub read: Option<R>,
}

/// Why one view of a fan-out run stopped.
///
/// `W` is the projection store's error and `A` the projection's own, as on
/// [`ProjectionError`]. The event store's error is not a parameter: the read
/// belongs to every view, so its failure is reported once, in
/// [`FanOut::read`], and a view stopped by it is
/// [`Interrupted`](Self::Interrupted).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
// Bounded on the definition because `Stopped`'s field is a `ProjectionError`,
// which is (API-21's forced case), and `thiserror`'s `source` needs them.
pub enum ViewError<W, A>
where
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// The view stopped on a failure of its own.
    ///
    /// Reported exactly as [`run_projection`](crate::run_projection) would
    /// report it. The read parameter is [`Infallible`], so a view stopped by
    /// the shared read cannot be spelled here.
    #[error(transparent)]
    Stopped(ProjectionError<Infallible, W, A>),

    /// The shared read failed while this view was still running.
    ///
    /// Chunks the view had committed stay committed. Its open chunk, if it
    /// had one, was rolled back.
    #[error("the shared read failed before this view caught up")]
    Interrupted {
        /// What this view had committed before the read failed.
        progress: Progressed,
        /// A rollback of this view's open chunk that also failed.
        rollback: Option<W>,
    },
}

impl<W, A> ViewError<W, A>
where
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// What this view committed before it stopped.
    pub fn progress(&self) -> Progressed {
        match self {
            Self::Stopped(error) => error.progress(),
            Self::Interrupted { progress, .. } => *progress,
        }
    }

    /// The position of the event that stopped this view, where there was one.
    ///
    /// `None` for an interrupted view: the shared read failed, not an event.
    #[must_use]
    pub fn position(&self) -> Option<SequencePosition> {
        match self {
            Self::Stopped(error) => error.position(),
            Self::Interrupted { .. } => None,
        }
    }
}

/// Streams one read of the log into N views, each as `run_projection` would.
///
/// A view is a value of one projection type `P`, with its own name, scope and
/// state. The run derives each view's query and checkpoint id, reads where
/// each had got to, and issues **one** read: the union of the views' queries,
/// from the lowest position any of them has not considered. Every event is
/// offered to each view in slice order, and a view takes only what its own
/// query nominates above its own checkpoint.
///
/// So each view ends with the rows, the checkpoint and the [`Progressed`]
/// that [`run_projection`](crate::run_projection) alone would have given it:
/// its own write set, opened at the first event it takes, committed every
/// `chunk` events and at the end of the log, under its own checkpoint id.
///
/// # One read, N views
///
/// N calls to `run_projection` cost N reads of the log; this costs one. An
/// event two views share is still decoded twice, once per view, so each
/// view's `on_error` is handed its own decode failure. Views of two event
/// types are two projection types and two calls, or one enum projection
/// that delegates to both.
///
/// # Each view fails alone
///
/// A view whose decode, `apply`, `on_error`, `begin` or `commit` fails stops
/// with its open chunk rolled back, and the others carry on (PS-26, PS-29).
/// Its failure is in the returned [`FanOut`], at its index, as
/// [`ViewError::Stopped`] holding what `run_projection` would have returned.
/// That value is `#[must_use]` and is the only place a failure is reported:
/// there is no handle to drain and no method to poll.
///
/// A failure of the shared read stops every view still running, rolling back
/// its open chunk: [`FanOut::read`] holds the failure, and each such view is
/// [`ViewError::Interrupted`]. The read is dropped as soon as no view runs.
///
/// # A panic is not caught
///
/// A panic in a view's decode, `apply` or `on_error` unwinds out of this
/// call, as it does out of `run_projection`. Every open write set is dropped
/// on the way out, and the port requires a dropped batch to roll back, so
/// nothing applied since any view's last commit survives and no checkpoint
/// moves (PS-30).
///
/// The cost is that one view panicking on an event stops all N, and stops
/// them again on every restart until it is fixed. Where that matters, run
/// each view in a task of its own through `run_projection` and let the
/// executor contain the panic; one read per view is that containment's
/// price. Under `panic = "abort"`, and on `wasm32-unknown-unknown`, a panic
/// ends the process whichever runner is used.
///
/// # What it holds open
///
/// At most one write set per view, each holding at most `chunk` events, and
/// one stream, pulled item by item and never collected. Over a batch that is
/// a live transaction, such as `happenstance-postgres`'s
/// `LivePostgresBatch`, N views hold up to N connections at once and await
/// their statements one view at a time. A pool smaller than N leaves a later
/// view's `begin` waiting for a connection only this run could release; at
/// the pool's acquire timeout that view stops with
/// [`ProjectionError::Begin`].
///
/// # Which flavour this binds, and what spawning it costs
///
/// [`EventStore`], the flavour that does **not** require `Send`, so a store
/// held through an `Rc` on a single-threaded edge runtime fans out here, and
/// a `Send` store does too, through the blanket impl.
///
/// Spawning the returned future from code generic over the projection asks
/// for `run_projection`'s bounds, each for the same reason. MEASURED
///
/// # Exactly one runner per view
///
/// One runner per `(store, checkpoint_id)` is the caller's to guarantee, as
/// `run_projection`'s page explains, and that includes this call: two views
/// in one slice with one name and one scope share a checkpoint id, and
/// nothing here refuses them.
///
/// # Example
///
/// Two views of one type over two depots, from one read. The fence needs
/// `memory` and `json`, which are both defaults.
///
/// ```
/// use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// use happenstance::{Checkpoint, Event, EventStore, EventType, Json};
/// use happenstance::{MemoryEventStore, MemoryProjectionBatch, Tag, Tags};
/// use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// use happenstance::{Delivered, Projection, ProjectionId};
/// use happenstance::{Chunk, ProjectionStore, checkpoint_id, run_fan_out};
/// # use std::error::Error;
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
///
/// #[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
/// enum Seat { North, South }
///
/// const SOLD: EventType = EventType::from_static("SeatSold");
/// const NORTH: Tag = Tag::from_static("depot:north");
/// const SOUTH: Tag = Tag::from_static("depot:south");
///
/// impl DomainEvent for Seat {
///     const EVENT_TYPES: &'static [EventType] = &[SOLD];
///     fn event_type(&self) -> EventType { SOLD }
///     fn tags(&self) -> Tags {
///         let depot = match self { Self::North => NORTH, _ => SOUTH };
///         [depot].into_iter().collect()
///     }
///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
///         c.encode(self)
///     }
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
/// }
///
/// struct Sold { id: ProjectionId, scope: Tags, sold: u64 }
///
/// impl Projection for Sold {
///     type Event = Seat;
///     type Store = MemoryProjectionStore;
///     type Error = MemoryProjectionStoreError;
///     fn id(&self) -> &ProjectionId { &self.id }
///     fn scope(&self) -> &Tags { &self.scope }
///     async fn apply(
///         &mut self,
///         _: Delivered<Seat>,
///         batch: &mut MemoryProjectionBatch,
///     ) -> Result<(), MemoryProjectionStoreError> {
///         self.sold += 1;
///         batch.write(self.id.as_str(), self.sold);
///         Ok(())
///     }
/// }
///
/// let sold = |seat: Seat| -> Result<Event, Box<dyn Error>> {
///     let event = Event::new(SOLD, Json.encode(&seat)?)?;
///     Ok(event.with_tags(seat.tags()))
/// };
/// let events = MemoryEventStore::new();
/// let log = [sold(Seat::North)?, sold(Seat::South)?, sold(Seat::North)?];
/// events.append(&log, None).await?;
///
/// let view = |name: &'static str, depot: Tag| Sold {
///     id: ProjectionId::from_static(name),
///     scope: [depot].into_iter().collect(),
///     sold: 0,
/// };
/// let mut views = [view("north", NORTH), view("south", SOUTH)];
/// let models = MemoryProjectionStore::new();
/// let chunk = Chunk::of(64.try_into()?);
///
/// let ran = run_fan_out(&events, &models, &mut views, &Json, chunk).await;
/// assert!(ran.read.is_none(), "the one read did not fail");
/// for outcome in ran.views {
///     outcome?;
/// }
///
/// assert_eq!(models.get("north"), Some(2));
/// assert_eq!(models.get("south"), Some(1));
/// let [north, south] = &views;
/// for view in [north, south] {
///     let at = models.checkpoint(&checkpoint_id(view)?).await?;
///     assert!(matches!(at, Checkpoint::Live { .. }));
/// }
/// # Ok::<(), Box<dyn Error>>(()) }
/// ```
pub async fn run_fan_out<S, P, C>(
    events: &S,
    models: &P::Store,
    views: &mut [P],
    codec: &C,
    chunk: Chunk,
) -> FanOut<S::Error, StoreError<P>, P::Error>
where
    S: EventStore,
    P: Projection,
    C: Codec,
{
    // As in `run_projection`: reading the `const` is what evaluates it.
    let () = AtLeastOneType::<P::Event>::CHECKED;

    let mut slots = Vec::with_capacity(views.len());
    for view in views.iter() {
        slots.push(match prepare(view, models).await {
            Ok(lane) => Slot::Running(lane),
            Err(stop) => Slot::Done(ViewError::Stopped(stop)),
        });
    }

    let read = drive(events, models, views, codec, chunk, &mut slots).await;

    // A view still running holds an open chunk or none. After a read that
    // reached the end it commits; after a read that failed it rolls back.
    let mut outcomes = Vec::with_capacity(slots.len());
    for slot in slots {
        outcomes.push(match &read {
            None => caught_up(models, slot).await,
            Some(_) => interrupted(models, slot).await,
        });
    }
    FanOut {
        views: outcomes,
        read,
    }
}

/// A view's own failure, before it is wrapped as a [`ViewError`].
type Stop<P> = ProjectionError<Infallible, StoreError<P>, <P as Projection>::Error>;

/// One view's place in the run: still taking events, or stopped.
// Bounded because `Done` names `P::Error` and `P::Store`'s error (API-21's
// stated exception).
enum Slot<P: Projection> {
    Running(Lane<P>),
    Done(ViewError<StoreError<P>, P::Error>),
}

/// What a running view needs that the projection does not hold.
///
/// `resume` is the first position this view has not considered, or `None`
/// for a view that never ran. The shared read may start below it, for
/// another view, so every event is checked against it.
// `P: Projection` on the definition because a field names its associated
// types, through `Open<P>` (API-21's stated exception).
struct Lane<P: Projection> {
    query: Query,
    key: ProjectionId,
    resume: Option<SequencePosition>,
    open: Option<Open<P>>,
    progress: Progressed,
}

/// A write set holding at least one event this view took.
///
/// Built only after a take succeeded, so `last` always names a real event.
// `P: Projection` on the definition because `batch` names `P::Store::Batch`
// (API-21's stated exception).
struct Open<P: Projection> {
    batch: StoreBatch<P>,
    tally: Tally,
    last: SequencePosition,
}

impl<P: Projection> Lane<P> {
    /// Whether this event is the view's: above its checkpoint, in its query.
    fn nominates(&self, event: &SequencedEvent) -> bool {
        self.resume.is_none_or(|resume| event.position >= resume)
            && self
                .query
                .matches(event.event.event_type(), event.event.tags())
    }

    /// Commits an open chunk under this view's key, and credits its progress.
    async fn commit(&mut self, models: &P::Store, open: Open<P>) -> Result<(), Stop<P>> {
        let progress = self.progress;
        models
            .commit(open.batch, &self.key, open.last, Authority::Live)
            .await
            .map_err(|source| ProjectionError::Commit { progress, source })?;
        open.tally.credit(&mut self.progress, open.last);
        Ok(())
    }
}

/// Derives one view's query and key, and reads where it had got to.
///
/// The same three steps, and the same arms on failure, as `run_projection`.
async fn prepare<P: Projection>(view: &P, models: &P::Store) -> Result<Lane<P>, Stop<P>> {
    let query = derive_query(P::Event::EVENT_TYPES, view.scope())?;
    let key = checkpoint::derive(view.id(), &query).map_err(ProjectionError::CheckpointId)?;
    let checkpoint = models
        .checkpoint(&key)
        .await
        .map_err(ProjectionError::Checkpoint)?;
    let resume = match considered_through(checkpoint) {
        None => None,
        Some(through) => match through.next() {
            Some(resume) => Some(resume),
            None => return Err(ProjectionError::KeySpaceExhausted { through }),
        },
    };
    Ok(Lane {
        query,
        key,
        resume,
        open: None,
        progress: Progressed::default(),
    })
}

/// The one read every running view is served from, or `None` if none runs.
///
/// The union of the running views' queries, from the lowest position any of
/// them has not considered. Identical items are kept once, so N views of
/// one scope ask a store for one item rather than N.
fn plan_read<P: Projection>(slots: &[Slot<P>]) -> Option<(Query, ReadOptions)> {
    let mut items: Vec<QueryItem> = Vec::new();
    let mut everything = false;
    let mut from: Option<SequencePosition> = None;
    let mut from_start = false;
    for slot in slots {
        let Slot::Running(lane) = slot else { continue };
        match lane.query.items() {
            None => everything = true,
            Some(own) => {
                for item in own {
                    if !items.contains(item) {
                        // Owned, because `Query::from_items` takes items by
                        // value and each lane keeps its own query to filter by.
                        items.push(item.clone());
                    }
                }
            }
        }
        match lane.resume {
            None => from_start = true,
            Some(resume) => from = Some(from.map_or(resume, |at| at.min(resume))),
        }
    }
    // `from_items` refuses only an empty list, which is no view running:
    // there is nothing to read, and that is what `None` says.
    let query = if everything {
        Query::all()
    } else {
        Query::from_items(items).ok()?
    };
    let options = match from {
        Some(from) if !from_start => ReadOptions::new().from(from),
        _ => ReadOptions::new(),
    };
    Some((query, options))
}

/// Pulls the shared read and offers each event to every running view.
///
/// Returns the read's failure, if it had one. It stops pulling as soon as
/// no view is running, so a run whose every view halted reads no further.
async fn drive<S, P, C>(
    events: &S,
    models: &P::Store,
    views: &mut [P],
    codec: &C,
    chunk: Chunk,
    slots: &mut [Slot<P>],
) -> Option<S::Error>
where
    S: EventStore,
    P: Projection,
    C: Codec,
{
    let (query, options) = plan_read(slots)?;
    let stream = events.read(&query, options);
    let mut stream = core::pin::pin!(stream);

    loop {
        let event = match next_event(stream.as_mut()).await {
            None => return None,
            Some(Err(source)) => return Some(source),
            Some(Ok(event)) => event,
        };
        // In slice order, one view after another: a buffered `apply` is ready
        // at its first poll, and an order that never varies is one a reader
        // can reason about.
        for (slot, view) in slots.iter_mut().zip(views.iter_mut()) {
            let Slot::Running(lane) = slot else { continue };
            if let Err(stop) = offer(lane, view, models, codec, &event, chunk).await {
                *slot = Slot::Done(ViewError::Stopped(stop));
            }
        }
        if !slots.iter().any(|slot| matches!(slot, Slot::Running(_))) {
            return None;
        }
    }
}

/// Offers one event to one running view.
///
/// A view's write set is opened at the first event it takes, never before,
/// and committed when it holds `chunk` events, so each view keeps the window
/// `run_projection` would have given it.
async fn offer<P, C>(
    lane: &mut Lane<P>,
    view: &mut P,
    models: &P::Store,
    codec: &C,
    event: &SequencedEvent,
    chunk: Chunk,
) -> Result<(), Stop<P>>
where
    P: Projection,
    C: Codec,
{
    if !lane.nominates(event) {
        return Ok(());
    }
    let progress = lane.progress;
    let (mut batch, mut tally) = match lane.open.take() {
        Some(open) => (open.batch, open.tally),
        None => match models.begin().await {
            Ok(batch) => (batch, Tally::default()),
            Err(source) => return Err(ProjectionError::Begin { progress, source }),
        },
    };
    // The panic seam, and nothing here catches an unwind. One raised by the
    // decode, `apply` or `on_error` leaves this call with `batch` and every
    // other view's open write set dropped, which the port rolls back.
    match apply_one::<P, C, Infallible>(view, codec, event, &mut batch).await {
        Ok(took) => tally.took(took),
        Err(stopped) => {
            let rollback = models.rollback(batch).await.err();
            return Err(stopped.into_error(progress, rollback));
        }
    }
    let open = Open {
        batch,
        tally,
        last: event.position,
    };
    if open.tally.len() < chunk.get() {
        lane.open = Some(open);
        return Ok(());
    }
    lane.commit(models, open).await
}

/// The outcome of a view the read reached the end for: its last chunk commits.
async fn caught_up<P: Projection>(
    models: &P::Store,
    slot: Slot<P>,
) -> Result<Progressed, ViewError<StoreError<P>, P::Error>> {
    let mut lane = match slot {
        Slot::Done(error) => return Err(error),
        Slot::Running(lane) => lane,
    };
    if let Some(open) = lane.open.take() {
        lane.commit(models, open)
            .await
            .map_err(ViewError::Stopped)?;
    }
    Ok(lane.progress)
}

/// The outcome of a view the read failed under: its open chunk rolls back.
async fn interrupted<P: Projection>(
    models: &P::Store,
    slot: Slot<P>,
) -> Result<Progressed, ViewError<StoreError<P>, P::Error>> {
    let lane = match slot {
        Slot::Done(error) => return Err(error),
        Slot::Running(lane) => lane,
    };
    let rollback = match lane.open {
        Some(open) => models.rollback(open.batch).await.err(),
        None => None,
    };
    Err(ViewError::Interrupted {
        progress: lane.progress,
        rollback,
    })
}
