//! The typed projection runner: decoded events into a read model.
//!
//! One trait an application implements over its own domain enum, and one
//! function that drives it. The rows and the checkpoint move together because
//! the port beneath offers no way to move them apart.

use core::pin::Pin;

use futures_core::Stream;
use happenstance_core::{
    Authority, Checkpoint, CommitError, EventStore, InvalidQuery, ProjectionId, ProjectionStore,
    ReadOptions, SequencePosition, SequencedEvent, Tags,
};

use crate::boundary::{AtLeastOneType, derive_query};
use crate::checkpoint::{self, InvalidCheckpointId};
use crate::chunk::Chunk;
use crate::codec::{Codec, CodecError, decode_event};
use crate::delivered::Delivered;
use crate::domain::DomainEvent;
use crate::policy::{ApplyFailure, Policy};

/// The adapter error of whatever store a projection writes into.
type StoreError<P> = <<P as Projection>::Store as ProjectionStore>::Error;
/// The write set a projection's `apply` writes into.
type StoreBatch<P> = <<P as Projection>::Store as ProjectionStore>::Batch;

/// A read model an application builds from its own decoded events.
///
/// Four things and no more: which read model this is, which events it wants,
/// which store holds it, and what one of those events does to it.
///
/// # The query is derived, and there is nowhere to put a hand-written one
///
/// The events a projection is handed come from a [`Query`] built out of
/// [`Self::Event`]'s own `EVENT_TYPES` and [`scope`](Self::scope) — the same
/// derivation a decision model gets, so there is no second filtering vocabulary
/// to learn. The alternative that lost was a `subscription()` method returning
/// a query of the projection's own: it makes the event set nameable in two
/// places, and the day they disagree the read model silently stops seeing a
/// variant its fold still handles.
///
/// # Why `Store` is an associated type
///
/// A projection writes into exactly one store, and that is a type-level fact
/// rather than a documented convention. There is no cross-store transaction, so
/// a projection spanning two of them could not honour the invariant
/// [`ProjectionStore`] exists to defend; making `Store` an associated type
/// makes such a projection **unrepresentable** instead of merely discouraged.
/// The cost is real and worth naming: two read models in two stores are two
/// implementations of this trait, and their checkpoints advance independently.
///
/// # Why `apply` is `async`
///
/// Because a batch can be a live transaction. A buffered batch, a map or a
/// statement list, is written by pushing: its `apply` never awaits, so its
/// future is ready at the first poll and the keyword costs a buffered
/// projection nothing at run time. A batch that *is* the transaction, such as
/// `happenstance-postgres`'s `LivePostgresBatch`, is written by awaiting a
/// statement, which a synchronous `apply` could not do. The alternative that
/// lost was a second, `async` trait beside a synchronous one, with a second
/// runner: it doubles what an application chooses between, where one
/// `async fn` doubles only the names this crate exports (ADR-0074).
///
/// # Its error is its own
///
/// [`apply`](Self::apply) returns [`Self::Error`], not the store's error, so
/// a projection that refuses an event says why in its own type and the runner
/// reports it as [`ProjectionError::Apply`] beside the refused event's
/// position. The store's error converts into it through `From`, so a write
/// inside `apply` still propagates with `?`. A projection whose only failure
/// is the store's names that error and is done.
///
/// # What it does with an event it cannot take
///
/// That is the projection's to decide, not the runner's: two read models of
/// different tolerance are the case one run-level answer gets wrong (PS-26).
/// [`on_error`](Self::on_error) is provided and defaults to halting, so a
/// projection that says nothing stops at the first failure; one that skips
/// writes its record into the batch that moves past the event (PS-27).
///
/// # Which flavour to implement
///
/// `#[trait_variant::make]` derives [`SendProjection`], whose `apply` future
/// is `Send`, and a blanket impl makes every `SendProjection` a `Projection`.
/// Implement exactly one, as an adapter does for the ports: implementing both
/// is `E0119`. A projection whose concrete type is spawned works through
/// either flavour: the compiler sees its `apply` future, and its `Send`-ness
/// leaks as any `async fn`'s does. `SendProjection` matters where code
/// **generic over the projection** spawns the run: there, only
/// `P: SendProjection` promises that `apply`'s future is `Send`. Implement
/// `SendProjection` for a projection handed to such code, a supervisor or a
/// library's spawner, and `Projection` for one holding an `Rc`, writing a
/// `!Send` batch, or built for `wasm32`. `#[async_trait]` is not used,
/// because the `+ Send` it injects makes the `wasm32` target impossible.
///
/// # Spawning a run from code generic over the projection
///
/// The bounds are the caller's to write, and [`run_projection`]'s page says
/// what each one pays for. `Store = St` is the one that surprises: a fresh
/// type parameter, not `P::Store`. The fence needs `memory` and `json`.
///
/// ```
/// use std::sync::Arc;
///
/// use happenstance::{Chunk, Json, Progressed, run_projection};
/// use happenstance::{SendEventStore, SendProjection};
/// use happenstance::SendProjectionStore;
/// # use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// # use happenstance::{Delivered, Event, EventType, ProjectionId, Tags};
/// # use happenstance::{MemoryEventStore, MemoryProjectionBatch};
/// # use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// # #[derive(serde::Serialize, serde::Deserialize)]
/// # enum Seat { Taken }
/// # const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// # impl DomainEvent for Seat {
/// #     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
/// #     fn event_type(&self) -> EventType { SEAT_TAKEN }
/// #     fn tags(&self) -> Tags { Tags::empty() }
/// #     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
/// #         c.encode(self)
/// #     }
/// #     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
/// #         -> Result<Self, CodecError> { c.decode(d) }
/// # }
/// # struct Sold { id: ProjectionId, scope: Tags, sold: u64 }
/// # impl SendProjection for Sold {
/// #     type Event = Seat;
/// #     type Store = MemoryProjectionStore;
/// #     type Error = MemoryProjectionStoreError;
/// #     fn id(&self) -> &ProjectionId { &self.id }
/// #     fn scope(&self) -> &Tags { &self.scope }
/// #     async fn apply(
/// #         &mut self,
/// #         _: Delivered<Seat>,
/// #         batch: &mut MemoryProjectionBatch,
/// #     ) -> Result<(), MemoryProjectionStoreError> {
/// #         self.sold += 1;
/// #         batch.write("sold", self.sold);
/// #         Ok(())
/// #     }
/// # }
///
/// fn spawn_run<S, P, St>(
///     events: Arc<S>,
///     models: Arc<St>,
///     mut projection: P,
///     chunk: Chunk,
/// ) -> tokio::task::JoinHandle<(P, Result<Progressed, String>)>
/// where
///     S: SendEventStore<Error: Send> + Sync + 'static,
///     P: SendProjection<Store = St, Event: Send, Error: Send + Sync>
///         + 'static,
///     St: SendProjectionStore<Batch: Send> + Sync + 'static,
/// {
///     tokio::spawn(async move {
///         // Reduced to its message: the handle asks no more bounds.
///         // To keep the chain, return it and add `St::Error: Send`.
///         let run = run_projection(
///             &*events, &*models, &mut projection, &Json, chunk,
///         )
///         .await
///         .map_err(|e| e.to_string());
///         (projection, run)
///     })
/// }
///
/// # #[tokio::main(flavor = "multi_thread", worker_threads = 2)]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let events = Arc::new(MemoryEventStore::new());
/// let taken = Event::new("SeatTaken", Json.encode(&Seat::Taken)?)?;
/// events.append(&[taken], None).await?;
///
/// let models = Arc::new(MemoryProjectionStore::new());
/// let sold = Sold {
///     id: ProjectionId::new("seats_sold")?,
///     scope: Tags::empty(),
///     sold: 0,
/// };
/// let chunk = Chunk::of(64.try_into()?);
/// let (sold, run) =
///     spawn_run(events, Arc::clone(&models), sold, chunk).await?;
///
/// assert_eq!(run?.applied, 1);
/// assert_eq!(sold.sold, 1, "the projection came back with its state");
/// assert_eq!(models.get("sold"), Some(1));
/// # Ok(()) }
/// ```
///
/// Written over `P::Store`, the same spawner does not compile. The
/// associated type's own bound is the bare [`ProjectionStore`], and that is
/// the bound the compiler reads, so none of the store's futures is `Send`
/// (`E0277`), whatever the `where` clause says about `P::Store`:
///
/// ```compile_fail,E0277
/// # use std::sync::Arc;
/// # use happenstance::{Chunk, Json, Progressed, run_projection};
/// # use happenstance::{SendEventStore, SendProjection};
/// # use happenstance::SendProjectionStore;
/// fn spawn_run<S, P>(
///     events: Arc<S>,
///     models: Arc<P::Store>,
///     mut projection: P,
///     chunk: Chunk,
/// ) -> tokio::task::JoinHandle<(P, Result<Progressed, String>)>
/// where
///     S: SendEventStore<Error: Send> + Sync + 'static,
///     P: SendProjection<Event: Send, Error: Send + Sync> + 'static,
///     P::Store: SendProjectionStore<Batch: Send> + Sync + 'static,
/// {
///     tokio::spawn(async move {
///         let run = run_projection(
///             &*events, &*models, &mut projection, &Json, chunk,
///         )
///         .await
///         .map_err(|e| e.to_string());
///         (projection, run)
///     })
/// }
/// ```
///
/// [`Query`]: happenstance_core::Query
#[trait_variant::make(SendProjection: Send)]
pub trait Projection {
    /// The application's own event enum. Not `Bytes`: the runner decodes.
    type Event: DomainEvent;

    /// The store this read model and its checkpoint live in.
    type Store: ProjectionStore;

    /// The projection's own refusal (PS-28).
    ///
    /// `From` the store's error, so `batch.execute(..).await?` inside `apply`
    /// needs no `map_err`. There is no default: a projection whose only
    /// failure is the store's writes `type Error = <its store's error>;`,
    /// which the reflexive `From<T> for T` satisfies.
    type Error: core::error::Error + From<<Self::Store as ProjectionStore>::Error> + 'static;

    /// The projection's name: which read model this is, within
    /// [`Self::Store`].
    ///
    /// The runner does not commit under it. It commits under
    /// [`checkpoint_id`](crate::checkpoint_id)`(self)`, the name and a digest
    /// of the derived query, so a projection whose query changes starts again
    /// (PS-25). Read the checkpoint through `checkpoint_id`, never through
    /// `id()`, which reads [`Checkpoint::NeverRun`]. A name longer than
    /// [`MAX_CHECKPOINT_NAME_LEN`](crate::MAX_CHECKPOINT_NAME_LEN), 238 bytes,
    /// leaves room for no digest and the run refuses it.
    ///
    /// Checkpoints are per store, so two stores projecting the same events
    /// sit at different positions and a caller reading both must tolerate
    /// the skew.
    fn id(&self) -> &ProjectionId;

    /// The tags this projection is scoped to, already validated.
    ///
    /// `&Tags` rather than a fallible constructor called per read, for the
    /// reason [`DecisionModel::scope`](crate::DecisionModel::scope) is: a
    /// fallible construction inside an infallible signature forces an `unwrap`
    /// somewhere, and the somewhere is always library code.
    fn scope(&self) -> &Tags;

    /// Applies one delivered event into the store's open write set.
    ///
    /// The event arrives as a [`Delivered`] of [`Self::Event`]:
    /// [`into_event`](Delivered::into_event) or [`event`](Delivered::event)
    /// gives the application's own enum to match on, and
    /// [`id`](Delivered::id) its origin identity. There is no arrival
    /// position to read, on purpose; [`Delivered`] says why. Nothing here is
    /// durable until the runner commits the chunk.
    ///
    /// `async` so that a live batch can be awaited through. A buffered
    /// `apply` never awaits. Dropping the run while `apply` is pending drops
    /// the open batch uncommitted, so a cancelled run commits nothing.
    ///
    /// # Errors
    ///
    /// Returns the projection's own [`Self::Error`]; a store error converts
    /// into it through `?`. The runner offers the failure to
    /// [`on_error`](Self::on_error) first; under the default it discards the
    /// whole chunk through [`rollback`](ProjectionStore::rollback) and reports
    /// the position of the event that failed, so the checkpoint stays where
    /// the last commit left it.
    //
    // The store's types are spelled as explicit projections rather than through
    // `StoreBatch<Self>` and `StoreError<Self>`: those aliases go through
    // `<P as Projection>`, which the derived `SendProjection` satisfies only
    // with `Self: Sized`, and the derivation fails with `E0277` (ADR-0074).
    // The parameters are plain names because the derived blanket impl forwards
    // by name and refuses a pattern.
    async fn apply(
        &mut self,
        event: Delivered<Self::Event>,
        batch: &mut <Self::Store as ProjectionStore>::Batch,
    ) -> Result<(), Self::Error>;

    /// Decides what happens to an event that could not be decoded or applied.
    ///
    /// Provided, and the default answers [`Policy::Halt`]: a projection that
    /// says nothing stops at the first such event, exactly as before.
    ///
    /// [`Policy::Skip`] leaves the event out. It still counts toward `chunk`,
    /// it is counted in [`Progressed::skipped`], and the checkpoint moves past
    /// it when the chunk commits. A record written into `batch` here commits
    /// with that move or not at all (PS-27). [`Policy::Halt`] rolls the chunk
    /// back, with anything written here, and the run reports
    /// [`ProjectionError::Decode`] or [`ProjectionError::Apply`].
    ///
    /// The batch is handed over as `apply` left it: a refused `apply` that had
    /// already written keeps those writes under `Skip`. The runner cannot undo
    /// part of a batch without a generic write (PS-9), so an `apply` that may
    /// refuse should refuse before it writes. [`ApplyFailure`] says which half
    /// failed and the event's identity, never its local position.
    ///
    /// The runner holds the borrowed failure across this await, so code
    /// generic over the projection that spawns the run adds `Error: Sync`.
    ///
    /// # Errors
    ///
    /// An `Err` stops the run with [`ProjectionError::Policy`] at the event's
    /// position, and the chunk is rolled back.
    //
    // `-> impl Future` with a block, not `async fn` with a body: trait_variant
    // 0.1.3 copies the block into `SendProjection`'s `-> impl Future + Send`.
    // The parameters are named and discarded with `let _`, never `_`-prefixed:
    // the derived blanket impl forwards them by name, and an `_failure` there
    // is a used underscore binding that pedantic clippy refuses (ADR-0074,
    // finding F2). Neither value is fallible, so nothing is being swallowed.
    fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, Self::Error>,
        batch: &mut <Self::Store as ProjectionStore>::Batch,
    ) -> impl Future<Output = Result<Policy, Self::Error>> {
        let _ = failure;
        let _ = batch;
        async { Ok(Policy::Halt) }
    }
}

/// How far a run got, and how much it applied.
///
/// A named struct rather than a `(Option<SequencePosition>, usize)` tuple,
/// because the arity is exactly what a later observability pass wants to grow
/// and a tuple freezes it. Readable, not fabricable: `#[non_exhaustive]` blocks
/// the struct literal outside this crate, so a third field is not a breaking
/// change.
///
/// It is returned on success **and** carried on failure, so *"stopped early"*
/// and *"caught up"* are distinguishable without a second look at the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[must_use = "a Progressed says how far the checkpoint moved and how much was applied"]
#[non_exhaustive]
pub struct Progressed {
    /// Where the checkpoint was last moved to, or `None` if it did not move.
    ///
    /// `None` after a completed run is an idle one: the query nominated
    /// nothing above the checkpoint, so there was nothing to commit.
    pub through: Option<SequencePosition>,
    /// How many events were applied **and committed** by this run.
    ///
    /// Events applied into a batch that was then discarded do not count: a
    /// number that included them would describe work no reader can observe.
    pub applied: usize,
    /// How many events `on_error` chose to skip, and were committed past.
    ///
    /// Not in `applied`: a skipped event changed no row but its record, and a
    /// count that folded the two would hide the skip PS-27 exists to show.
    pub skipped: usize,
}

/// Why a projection run stopped.
///
/// Three type parameters: `R` is the event store's error, `W` the projection
/// store's, and `A` the projection's own, [`Projection::Error`] (PS-28). An
/// application's refusal is reported as itself, rather than forged into the
/// adapter's `#[non_exhaustive]` error enum. `A` has no default `A = W`: a
/// default would let a signature naming two parameters keep compiling while
/// silently meaning the store's error (ADR-0074). Nor is it a
/// `Box<dyn Error>`, which would erase the type a caller matches on.
///
/// The decode failure carries the **concrete** [`CodecError`] rather than a
/// fourth parameter: an associated codec error would be one more parameter on
/// every signature downstream, for an error this crate already owns.
///
/// Two parameters do not compile, so no caller is left naming the old shape:
///
/// ```compile_fail,E0107
/// use happenstance::{MemoryProjectionStoreError, MemoryStoreError};
/// use happenstance::ProjectionError;
/// type Old =
///     ProjectionError<MemoryStoreError, MemoryProjectionStoreError>;
/// ```
///
/// ```
/// use happenstance::{MemoryProjectionStoreError, MemoryStoreError};
/// use happenstance::ProjectionError;
///
/// type Same = MemoryProjectionStoreError;
/// type Now = ProjectionError<MemoryStoreError, Same, Same>;
/// ```
///
/// Every foreign error is a typed [`source`](core::error::Error::source), never
/// a string, so a caller reports the chain rather than re-parsing a message.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ProjectionError<R, W, A>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// The projection constrains neither event types nor tags.
    ///
    /// Absorbed rather than unwrapped: the derivation is fallible and library
    /// code here does not `unwrap`, even where the arm is unreachable for a
    /// well-formed projection.
    #[error(transparent)]
    Boundary(#[from] InvalidQuery),

    /// The checkpoint id could not be derived from the name and the query.
    ///
    /// Nothing was read and nothing was applied. `#[source]` rather than
    /// `#[from]`, so a `?` elsewhere cannot land here by accident.
    #[error("deriving the checkpoint id failed")]
    CheckpointId(#[source] InvalidCheckpointId),

    /// The projection store could not report where this projection had got to.
    ///
    /// Nothing was read and nothing was applied.
    #[error("reading the checkpoint failed")]
    Checkpoint(#[source] W),

    /// The checkpoint sits at the last representable position.
    ///
    /// [`SequencePosition::next`] is `checked_add` rather than `saturating_add`
    /// precisely so this is representable: a consumer resuming from the last
    /// position is told it has run out of key space instead of re-reading it
    /// forever. Consuming that `None` with an `unwrap` reintroduces the bug the
    /// method exists to prevent, so it becomes this arm instead.
    #[error("the position key space is exhausted at {through}")]
    KeySpaceExhausted {
        /// The checkpoint that cannot be advanced past.
        through: SequencePosition,
    },

    /// The event store's stream failed part-way through the replay.
    ///
    /// Chunks already committed stay committed — that is what committing them
    /// was for — and `progress` names exactly how far it got.
    #[error("reading the event stream failed")]
    Read {
        /// What the run had committed before this.
        progress: Progressed,
        /// The event store's own refusal.
        #[source]
        source: R,
        /// A rollback that also failed while the chunk was being discarded.
        ///
        /// Carried beside the cause rather than replacing it: the cause is what
        /// a reader must act on, and a rollback failure reported in its place
        /// would hide it.
        rollback: Option<W>,
    },

    /// A nominated event could not be decoded into the projection's own type.
    ///
    /// And the projection's [`on_error`](Projection::on_error) answered
    /// [`Policy::Halt`]. The half-applied chunk is discarded, so the checkpoint
    /// still sits at the last good position and a restart applies nothing
    /// twice.
    #[error("decoding the event at position {position} failed")]
    Decode {
        /// What the run had committed before this.
        progress: Progressed,
        /// Where the offending event sits in the store's order.
        position: SequencePosition,
        /// The codec's own refusal.
        #[source]
        source: CodecError,
        /// A rollback that also failed while the chunk was being discarded.
        rollback: Option<W>,
    },

    /// [`Projection::apply`] refused the event.
    ///
    /// And the projection's [`on_error`](Projection::on_error) answered
    /// [`Policy::Halt`]. The chunk is discarded, as for a decode failure.
    #[error("applying the event at position {position} failed")]
    Apply {
        /// What the run had committed before this.
        progress: Progressed,
        /// Where the offending event sits in the store's order.
        position: SequencePosition,
        /// The projection's own refusal, [`Projection::Error`] (PS-28).
        #[source]
        source: A,
        /// A rollback that also failed while the chunk was being discarded.
        rollback: Option<W>,
    },

    /// The projection's failure policy itself failed.
    ///
    /// [`Projection::on_error`] returned an error, writing its record for
    /// instance (ADR-0074 decisions 6 and 8). The chunk is discarded. The
    /// failure it was handling is not carried: it is the event at `position`,
    /// and a restart offers it again.
    #[error("the failure policy for the event at position {position} failed")]
    Policy {
        /// What the run had committed before this.
        progress: Progressed,
        /// Where the event the policy was handling sits in the store's order.
        position: SequencePosition,
        /// The projection's own error, as its policy reported it.
        #[source]
        source: A,
        /// A rollback that also failed while the chunk was being discarded.
        rollback: Option<W>,
    },

    /// The projection store could not open a write set for the next chunk.
    ///
    /// A buffering adapter never produces this — opening a buffer cannot fail —
    /// but a batch that is a live transaction has a connection to acquire and a
    /// `BEGIN` to issue, and either can be refused (ADR-0062). Chunks already
    /// committed stay committed. The first event of the chunk that would have
    /// followed *was* read — a batch is only opened once one is in hand — and
    /// it is not applied; a restart re-reads it from the checkpoint.
    #[error("opening a write set for the next chunk failed")]
    Begin {
        /// What the run had committed before this.
        progress: Progressed,
        /// The projection store's own refusal.
        #[source]
        source: W,
    },

    /// The chunk's single commit failed.
    ///
    /// Nothing in that chunk is durable and the checkpoint did not move, so the
    /// events it covered are **not** counted in `progress`.
    #[error("committing the chunk failed")]
    Commit {
        /// What the run had committed before this.
        progress: Progressed,
        /// The port's own outcome, not just the adapter's.
        ///
        /// It distinguishes a foreign batch and a checkpoint regression from
        /// an adapter failure, which are three different things to do next.
        #[source]
        source: CommitError<W>,
    },
}

impl<R, W, A> ProjectionError<R, W, A>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// What the run committed before it stopped.
    ///
    /// The failure and the partial progress are one value, so a caller never
    /// has to ask the store how far it got in order to decide whether to retry.
    pub fn progress(&self) -> Progressed {
        match self {
            Self::Boundary(_)
            | Self::CheckpointId(_)
            | Self::Checkpoint(_)
            | Self::KeySpaceExhausted { .. } => Progressed::default(),
            Self::Read { progress, .. }
            | Self::Decode { progress, .. }
            | Self::Apply { progress, .. }
            | Self::Policy { progress, .. }
            | Self::Begin { progress, .. }
            | Self::Commit { progress, .. } => *progress,
        }
    }

    /// The position of the event that stopped the run, where there was one.
    ///
    /// `None` for a failure that is not about a particular event — the store
    /// refusing a checkpoint read, or a commit that failed after the chunk was
    /// assembled.
    #[must_use]
    pub fn position(&self) -> Option<SequencePosition> {
        match self {
            Self::Decode { position, .. }
            | Self::Apply { position, .. }
            | Self::Policy { position, .. } => Some(*position),
            Self::KeySpaceExhausted { through } => Some(*through),
            _ => None,
        }
    }
}

/// Streams the events a projection nominates and applies them in chunks.
///
/// One call drives one projection: it derives the query, asks the store where
/// this projection had got to, resumes **past** that position, and then pulls
/// the replay item by item. Every `chunk` events it opens a write set, applies
/// them into it, and hands the write set and the position of the last event
/// it took to the port's single [`commit`](ProjectionStore::commit) — which is
/// what makes the read model and the checkpoint move together or not at all.
///
/// # Which flavour this binds, and what spawning it costs
///
/// [`EventStore`], the flavour that does **not** require `Send` — so a store
/// held through an `Rc` on a single-threaded edge runtime runs a projection
/// here, and a `Send` store does too, through the blanket impl.
///
/// Spawning the returned future from code generic over the projection asks
/// for these bounds, and they are the caller's to write. Each was measured
/// load-bearing by removing it:
///
/// - `P: SendProjection<Store = St>`, with `St` a type parameter of your
///   own. Written as `P::Store`, the store's futures are not `Send`
///   (`E0277`), because the associated type's bound is the bare
///   [`ProjectionStore`].
/// - `St: SendProjectionStore<Batch: Send>`: the batch is held across every
///   `apply` await.
/// - `P::Event: Send`: the blanket `Projection` impl forwards to
///   `SendProjection::apply` from an `async fn` of its own, whose state
///   holds the delivered event across that await.
/// - `P::Error: Send`, and `Error: Send` on the `Send` flavour of the event
///   store: on a failure this runner holds the stop, which carries either,
///   across the port's `rollback` await, because one [`ProjectionError`]
///   needs the error *and* what the rollback said.
/// - `P::Error: Sync`: the runner holds the [`ApplyFailure`] it hands
///   [`on_error`](Projection::on_error), which borrows the error, across
///   that await, and `&A` is `Send` only where `A` is `Sync`.
/// - `Sync` on both stores, which are borrowed across awaits, and `'static`
///   on all three, for the spawn.
///
/// `P: Send` comes with `SendProjection`, its supertrait. No store error
/// crosses an await, so no store error needs `Sync`. `Error` has no `Send`
/// bound by design, on the port's error and the projection's, so the bound
/// is yours to add, or declare ADR-0009's marker trait and bound on that.
/// [`Projection`]'s *Spawning a run from code generic over the projection*
/// compiles the shape and the spelling that fails, and the test
/// `run_projection_spawns_from_code_generic_over_the_projection` in
/// `crates/happenstance/tests/flavours.rs` runs it.
///
/// # It streams, and that is not an implementation detail
///
/// The replay is never collected. [`EventStore::read`] returns its stream at
/// the top level, and is deliberately not `async`, so that an adapter can hand
/// back a million events without buffering them; a runner that called
/// [`collect`](happenstance_core::collect) here would defeat that and make a
/// rebuild larger than memory unwritable. The only buffer is the
/// at-most-`chunk` events between one `begin` and its `commit`, which is a
/// bounded window rather than the log.
///
/// # There is no per-run failure policy, and offering one would be wrong
///
/// The runner takes no policy of its own. The alternative that lost was an
/// `on_error: SkipPolicy` argument, as a builder API invites: it forces one
/// answer onto every projection an application runs, and two read models of
/// different tolerance are the case that makes one answer wrong. The policy
/// is the projection's: [`Projection::on_error`], which defaults to halting.
///
/// # One projection per call
///
/// There is no fan-out over N projections, because the write set is **owned**:
/// it cannot be shared between tasks, and a runner that moved it into a
/// `tokio::spawn` would need it to be `'static` and `Send`, neither of which an
/// adapter holding a live transaction can promise. N read models therefore cost
/// N independent reads.
///
/// # Rebuilding
///
/// The port can say *"considered through P"* and *"rebuilding, considered
/// through P"*, but this runner only ever claims
/// [`Authority::Live`]. To
/// rebuild without serving half-built rows, run the rebuild under a **second**
/// name, which derives a second checkpoint id, and swap the reader over when it
/// catches up: the port already permits it and nothing else has to change.
///
/// # Errors
///
/// See [`ProjectionError`]. In short: the derived query and checkpoint id,
/// the checkpoint read, the stream, the decode, `apply`, a failing
/// `on_error`, the commit and the rollback each have their own arm, every one
/// of them carries the partial [`Progressed`], and the three that name an
/// event carry its position.
///
/// # Example
///
/// A read model over one domain enum, run to the end of the log. The fence
/// needs `memory` and `json`, which are both defaults.
///
/// ```
/// use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// use happenstance::{Event, EventStore, EventType, Json, Tags};
/// use happenstance::{MemoryEventStore, MemoryProjectionBatch};
/// use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// use happenstance::{Delivered, Projection, ProjectionId, run_projection};
/// use happenstance::Chunk;
/// # use std::error::Error;
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// enum Seat { Taken }
///
/// const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// impl DomainEvent for Seat {
///     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
///     fn event_type(&self) -> EventType { SEAT_TAKEN }
///     fn tags(&self) -> Tags { Tags::empty() }
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
///         batch.write("sold", self.sold);
///         Ok(())
///     }
/// }
///
/// let events = MemoryEventStore::new();
/// let taken = Event::new("SeatTaken", Json.encode(&Seat::Taken)?)?;
/// events.append(&[taken.clone(), taken], None).await?;
///
/// let models = MemoryProjectionStore::new();
/// let mut sold = Sold {
///     id: ProjectionId::new("seats_sold")?,
///     scope: Tags::empty(),
///     sold: 0,
/// };
/// let chunk = Chunk::of(64.try_into()?);
/// let done =
///     run_projection(&events, &models, &mut sold, &Json, chunk)
///         .await?;
///
/// assert_eq!(done.applied, 2);
/// assert_eq!(models.get("sold"), Some(2));
/// # Ok::<(), Box<dyn Error>>(()) }
/// ```
/// # Choosing `chunk`
///
/// One number, trading two things against each other in both directions.
/// Neither direction is a correctness question: every value produces the
/// same read model, and none of them can produce a wrong one.
///
/// **Small.** One `begin`/`commit` pair per `chunk` events, so a chunk of
/// 1 is one write set, one commit and one durable checkpoint move per
/// event. That is the most frequent progress anything outside this call
/// can see, and the highest per-event cost — the port's pair is a
/// transaction on every adapter that has one.
///
/// **Large.** Fewer commits, and a write set holding every application
/// since the last one. `SqliteProjectionStore`'s batch is an owned
/// statement list, so a chunk of a million is a million statements held in
/// memory before anything is durable — and the bounded-window claim above
/// stays true throughout, because the window is bounded by the number you
/// passed. Large costs more on a restart too: a chunk that fails is
/// discarded whole, so everything since the last commit is re-read and
/// re-applied.
///
/// The number is a named type, [`Chunk`], as [`Retry`](crate::Retry) — the
/// same kind of caller-supplied bound one module over — is. Like `Retry`
/// it has no default, by ADR-0070 §3, and for a different reason: `Retry`'s
/// is a worst case a caller must see, while a chunk changes no outcome, so
/// a default here would hide only a cost. What would choose one is a
/// measurement — wall time, peak resident memory and commit count against
/// a real adapter across the range — which belongs in `experiments/`, out
/// of the gate, and which nobody has run. Until then the `64` above is the
/// doctest's number and not advice.
///
/// # What can be seen while it runs
///
/// Nothing this call offers. There is no callback, no channel and no
/// `tracing` instrumentation anywhere in this workspace, and
/// [`Progressed`] is returned once, at the end. Watched from the outside, a
/// rebuild over a large log is indistinguishable from a hang for its whole
/// duration — including the rebuild this page recommends below.
///
/// What *is* observable is durable, and it is one thing: the checkpoint
/// moves once per chunk, and a second handle on the same store reads it
/// through [`ProjectionStore::checkpoint`]. An operator who wants
/// "850,000 of 1,000,000 applied" polls that and compares it against the
/// log themselves.
///
/// An observed entry point beside this one would be additive, and free —
/// these items are behind `unstable-projection` and make no semver promise
/// — so it is absent rather than foreclosed. It is a question about the
/// port's surface and belongs with the projection-store freeze.
///
/// # Exactly one runner per projection, and the caller owns that
///
/// Nothing here enforces it. A [`ProjectionId`] names a checkpoint, not a
/// lease, and a second runner on the same `(store, checkpoint_id)` is
/// accepted by every store in this workspace — so **exactly one runner per
/// `(store, checkpoint_id)` is the caller's to guarantee**, by whatever
/// their deployment already uses to elect a singleton. The id is
/// [`checkpoint_id`](crate::checkpoint_id): the name and a digest of the
/// derived query, so two runners of one name and one query share it.
///
/// Only the *backwards* half is guarded. `CheckpointRegression` stops a
/// stale runner dragging the checkpoint down, which is the rolling-redeploy
/// case the specification names under PS-22 — but two runners both moving
/// **forwards** never trip it. They interleave, each applies events the
/// other has already applied, and the monotonic checkpoint they leave
/// behind is exactly what a reader would take as evidence that nothing went
/// wrong. A projection whose `apply` is idempotent survives that; one that
/// counts, sums or appends does not.
///
/// A lease, an ownership token or a fencing token would move the obligation
/// off the caller and into the port. That is a real design question, it is
/// owed a measurement rather than a preference, and it belongs with the
/// projection-store freeze — so what is published now is the obligation,
/// not a mechanism nobody has run.
///
// The three sections above sit *after* the fence rather than beside the subject
// each belongs to, because three sentences of `SPECIFICATION.md` cite
// `crates/happenstance/src/runner.rs:780` as the location of this item and that
// line number is only right while the fence stays where it is: prose added
// above it pushes the item's declaration down and leaves the three citations
// pointing at whatever line the item used to occupy.
//
// `spec-trace` anchors the three on the item's name within twelve lines of
// the cited one. At #107 the item moved twenty lines and the stale citations
// still passed, because this comment then named the item in backticks inside
// that window. It no longer names it, so only the declaration can satisfy the
// anchor; keep it that way. The test
// `crates/happenstance/tests/projection_runner_page.rs` asserts the two
// sections this lane added stay below the fence, closer to the cause.
//
// Moving a section up is still *allowed*; it costs the three citations moving
// with it.
pub async fn run_projection<S, P, C>(
    events: &S,
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: Chunk,
) -> Result<Progressed, ProjectionError<S::Error, StoreError<P>, P::Error>>
where
    S: EventStore,
    P: Projection,
    C: Codec,
{
    // Reading the `const` is what evaluates it: a domain type declaring no
    // event types is a compile error rather than a query that nominates
    // nothing.
    let () = AtLeastOneType::<P::Event>::CHECKED;

    let query = derive_query(P::Event::EVENT_TYPES, projection.scope())?;
    // Keyed on the name **and** the query (PS-25), so a projection whose query
    // changed reads `NeverRun` and replays, instead of resuming past events
    // its old query never nominated. Read and commit share this one value.
    let key = checkpoint::derive(projection.id(), &query).map_err(ProjectionError::CheckpointId)?;

    let checkpoint = models
        .checkpoint(&key)
        .await
        .map_err(ProjectionError::Checkpoint)?;

    // `from` is **inclusive**, so feeding a checkpoint straight in re-applies
    // the last event on every run. `next` is the only sanctioned way to move
    // past a position without doing arithmetic on an opaque ordering key, and
    // its `None` arm is a real state rather than something to `unwrap`.
    let mut options = ReadOptions::new();
    if let Some(through) = considered_through(checkpoint) {
        let Some(resume) = through.next() else {
            return Err(ProjectionError::KeySpaceExhausted { through });
        };
        options = options.from(resume);
    }

    // One read for the whole run. Re-reading per chunk would re-derive the
    // query and re-anchor `from`, turning one replay into N — which is the
    // cost of polling, not a way to do chunking.
    let stream = events.read(&query, options);
    let mut stream = core::pin::pin!(stream);

    let mut progress = Progressed::default();

    loop {
        // The first event of a chunk is pulled *before* a write set is opened.
        // An empty replay, and a replay whose last chunk was exactly `chunk`
        // long, therefore never open a batch they would only roll back. That
        // was free while `begin` allocated a `Vec`; for a batch that is a live
        // transaction it is a `BEGIN` round trip and a failure path (ADR-0062),
        // and a run that has nothing to apply should touch the store not at all.
        let first = match next_event(stream.as_mut()).await {
            None => return Ok(progress),
            // No batch is open, so there is nothing to roll back — which is
            // what the `None` says, rather than a rollback that was skipped.
            Some(Err(source)) => return Err(Stopped::Read(source).into_error(progress, None)),
            Some(Ok(event)) => event,
        };

        let mut batch = match models.begin().await {
            Ok(batch) => batch,
            Err(source) => return Err(ProjectionError::Begin { progress, source }),
        };

        let mut tally = Tally::default();
        match apply_one(projection, codec, &first, &mut batch).await {
            Ok(took) => tally.took(took),
            Err(stopped) => {
                let rollback = models.rollback(batch).await.err();
                return Err(stopped.into_error(progress, rollback));
            }
        }
        let mut last = first.position;
        let mut stopped = None;
        let mut exhausted = false;

        // The bounded buffer: at most `chunk` events between one `begin` and
        // its `commit`. The stream is pulled item by item and never collected,
        // so memory stays flat in the length of the replay. A skipped event
        // counts toward it, so a log of nothing but poison still commits.
        while tally.len() < chunk.get() {
            match next_event(stream.as_mut()).await {
                None => {
                    exhausted = true;
                    break;
                }
                Some(Err(source)) => {
                    stopped = Some(Stopped::Read(source));
                    break;
                }
                Some(Ok(event)) => match apply_one(projection, codec, &event, &mut batch).await {
                    Ok(took) => {
                        tally.took(took);
                        last = event.position;
                    }
                    Err(stop) => {
                        stopped = Some(stop);
                        break;
                    }
                },
            }
        }

        if let Some(stopped) = stopped {
            // The half-applied chunk is discarded whole, so a restart re-reads
            // from the last committed checkpoint: nothing applied twice, and
            // nothing skipped.
            let rollback = models.rollback(batch).await.err();
            return Err(stopped.into_error(progress, rollback));
        }

        // The rows and the checkpoint in the port's **single** `commit`. The
        // position is the one the last event the chunk took actually carries,
        // applied or skipped, never a computed or anticipated one — and there
        // is always one, because a chunk is only opened once its first event
        // is in hand. A skip record rides this commit with it (PS-27).
        models
            .commit(batch, &key, last, Authority::Live)
            .await
            .map_err(|source| ProjectionError::Commit { progress, source })?;
        progress.through = Some(last);
        progress.applied += tally.applied;
        progress.skipped += tally.skipped;

        if exhausted {
            return Ok(progress);
        }
    }
}

/// What became of one event the chunk took.
#[derive(Clone, Copy)]
enum Took {
    Applied,
    Skipped,
}

/// How many events the open chunk took, by what became of them.
///
/// Both counts are bounded by `chunk`, a `usize`, so `+= 1` cannot overflow
/// before the window closes.
#[derive(Default)]
struct Tally {
    applied: usize,
    skipped: usize,
}

impl Tally {
    fn took(&mut self, took: Took) {
        match took {
            Took::Applied => self.applied += 1,
            Took::Skipped => self.skipped += 1,
        }
    }

    /// The window: every event the chunk took, applied or skipped.
    const fn len(&self) -> usize {
        self.applied + self.skipped
    }
}

/// Decodes one read event and applies it into the open write set.
///
/// Split out so the chunk loop reads as a loop. A decode or apply failure is
/// offered to [`Projection::on_error`] with the same batch, before anything
/// is rolled back; its answer decides whether the chunk carries on.
async fn apply_one<P, C, R>(
    projection: &mut P,
    codec: &C,
    event: &SequencedEvent,
    batch: &mut StoreBatch<P>,
) -> Result<Took, Stopped<R, P::Error>>
where
    P: Projection,
    C: Codec,
{
    let position = event.position;
    // The crate's one decode path, through the codec tag the writer left, so an
    // event written under an older encoding still folds into today's model.
    let decoded = match decode_event::<P::Event, C>(codec, event) {
        Ok(decoded) => decoded,
        Err(source) => {
            let answer = projection
                .on_error(&ApplyFailure::decode(event.id, &source), batch)
                .await;
            return settle(answer, position, || Stopped::Decode { position, source });
        }
    };

    // The identity the store read, handed over unchanged: the origin's, never
    // one re-minted from the local position (SY-21). A plain `.await`, so the
    // future is held inline in the runner's own state machine and nothing is
    // boxed or spawned.
    match projection
        .apply(Delivered::new(event.id, decoded), batch)
        .await
    {
        Ok(()) => Ok(Took::Applied),
        Err(source) => {
            let answer = projection
                .on_error(&ApplyFailure::apply(event.id, &source), batch)
                .await;
            settle(answer, position, || Stopped::Apply { position, source })
        }
    }
}

/// Turns `on_error`'s answer into what the chunk does next.
///
/// `halt` is built only on [`Policy::Halt`], after the borrow `on_error` was
/// handed has ended, so the original failure moves into the stop unchanged.
fn settle<R, A>(
    answer: Result<Policy, A>,
    position: SequencePosition,
    halt: impl FnOnce() -> Stopped<R, A>,
) -> Result<Took, Stopped<R, A>> {
    match answer {
        Ok(Policy::Skip) => Ok(Took::Skipped),
        Ok(Policy::Halt) => Err(halt()),
        Err(source) => Err(Stopped::Policy { position, source }),
    }
}

/// Why a chunk stopped, before the partial progress is attached to it.
///
/// The loop knows what went wrong; only the caller's frame knows how far the
/// run had got and whether the rollback that followed also failed. Keeping them
/// apart is what stops a `progress` field being threaded through every
/// construction site.
///
/// `A` is the projection's error; the store's `W` appears only on `into_error`,
/// because no stop ever holds one and a type parameter would put it in the
/// runner's state machine for nothing.
enum Stopped<R, A> {
    Read(R),
    Decode {
        position: SequencePosition,
        source: CodecError,
    },
    Apply {
        position: SequencePosition,
        source: A,
    },
    Policy {
        position: SequencePosition,
        source: A,
    },
}

impl<R, A> Stopped<R, A>
where
    R: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// Attaches the run's partial progress and whatever the rollback said.
    fn into_error<W>(self, progress: Progressed, rollback: Option<W>) -> ProjectionError<R, W, A>
    where
        W: core::error::Error + 'static,
    {
        match self {
            Self::Read(source) => ProjectionError::Read {
                progress,
                source,
                rollback,
            },
            Self::Decode { position, source } => ProjectionError::Decode {
                progress,
                position,
                source,
                rollback,
            },
            Self::Apply { position, source } => ProjectionError::Apply {
                progress,
                position,
                source,
                rollback,
            },
            Self::Policy { position, source } => ProjectionError::Policy {
                progress,
                position,
                source,
                rollback,
            },
        }
    }
}

/// Pulls one item from a pinned stream.
///
/// Hand-written rather than pulling in `futures-util` for its `next`: one
/// `poll_fn` is the whole of it, and a crate's dependency graph is part of its
/// semver surface.
async fn next_event<S, T, E>(mut stream: Pin<&mut S>) -> Option<Result<T, E>>
where
    S: Stream<Item = Result<T, E>>,
{
    core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await
}

/// Where a checkpoint says a run should resume from, if anywhere.
///
/// [`Checkpoint::NeverRun`] is not position zero and not *"caught up"*: it
/// means the replay starts at the beginning of what the query selects. A
/// rebuild in flight resumes exactly like a live projection — this runner does
/// not serve rows, so it has no reason to treat the two differently.
const fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
        // `NeverRun`, and whatever `#[non_exhaustive]` adds later: a state this
        // build cannot name is not a position to resume from.
        _ => None,
    }
}
