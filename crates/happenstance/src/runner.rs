//! The typed projection runner: decoded events into a read model.
//!
//! One trait an application implements over its own domain enum, and one
//! function that drives it. The rows and the checkpoint move together because
//! the port beneath offers no way to move them apart.

use core::num::NonZeroUsize;
use core::pin::Pin;

use futures_core::Stream;
use happenstance_core::{
    Authority, Checkpoint, CommitError, EventStore, InvalidQuery, ProjectionId, ProjectionStore,
    ReadOptions, SequencePosition, SequencedEvent, Tags,
};

use crate::boundary::{AtLeastOneType, derive_query};
use crate::codec::{Codec, CodecError, decode_event};
use crate::domain::DomainEvent;

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
/// # Why `apply` is synchronous
///
/// It writes into a batch that is already open — a buffer, a statement list, a
/// transaction the adapter is holding — and buffering a row does not await.
/// An `async fn apply` was the alternative: it would double the surface every
/// application implements (`trait_variant` derives the `Send` flavour, and
/// `#[async_trait]` is forbidden outright because its injected `+ Send` makes
/// the `wasm32` target impossible), in exchange for an await nothing here
/// needs. An implementation that genuinely must await should do the awaiting
/// in the adapter's `commit`, which is already `async`.
///
/// [`Query`]: happenstance_core::Query
pub trait Projection {
    /// The application's own event enum. Not `Bytes`: the runner decodes.
    type Event: DomainEvent;

    /// The store this read model and its checkpoint live in.
    type Store: ProjectionStore;

    /// Which read model this is, within [`Self::Store`].
    ///
    /// Checkpoints are per `(store, ProjectionId)`, so two stores projecting
    /// the same events sit at different positions and a caller reading both
    /// must tolerate the skew.
    fn id(&self) -> &ProjectionId;

    /// The tags this projection is scoped to, already validated.
    ///
    /// `&Tags` rather than a fallible constructor called per read, for the
    /// reason [`DecisionModel::scope`](crate::DecisionModel::scope) is: a
    /// fallible construction inside an infallible signature forces an `unwrap`
    /// somewhere, and the somewhere is always library code.
    fn scope(&self) -> &Tags;

    /// Applies one decoded event into the adapter's open write set.
    ///
    /// The event arrives as [`Self::Event`], so the fold matches on its own
    /// variants. Nothing here is durable until the runner commits the chunk.
    ///
    /// # Errors
    ///
    /// Returns the adapter's own error. The runner discards the whole chunk
    /// through [`rollback`](ProjectionStore::rollback) and reports the position
    /// it stopped at, so the checkpoint stays where the last good commit left
    /// it.
    fn apply(
        &mut self,
        event: Self::Event,
        batch: &mut StoreBatch<Self>,
    ) -> Result<(), StoreError<Self>>;
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
}

/// Why a projection run stopped.
///
/// Two type parameters, not three. `R` is the event store's error and `W` the
/// projection store's; the decode failure carries the **concrete**
/// [`CodecError`], which is the whole reason a decode failure has a home here
/// at all — an associated codec error would have been a third parameter on
/// every signature downstream, and a checkpoint pump typing its callback's
/// error as the projection store's cannot represent a decode failure without
/// the application forging one into the adapter's own error enum.
///
/// Every foreign error is a typed [`source`](core::error::Error::source), never
/// a string, so a caller reports the chain rather than re-parsing a message.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ProjectionError<R, W>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
{
    /// The projection constrains neither event types nor tags.
    ///
    /// Absorbed rather than unwrapped: the derivation is fallible and library
    /// code here does not `unwrap`, even where the arm is unreachable for a
    /// well-formed projection.
    #[error(transparent)]
    Boundary(#[from] InvalidQuery),

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
    /// The half-applied chunk is discarded, so the checkpoint still sits at the
    /// last good position and a restart applies nothing twice.
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
    #[error("applying the event at position {position} failed")]
    Apply {
        /// What the run had committed before this.
        progress: Progressed,
        /// Where the offending event sits in the store's order.
        position: SequencePosition,
        /// The projection store's own refusal, as `apply` reported it.
        #[source]
        source: W,
        /// A rollback that also failed while the chunk was being discarded.
        rollback: Option<W>,
    },

    /// The projection store could not open a write set for the next chunk.
    ///
    /// A buffering adapter never produces this — opening a buffer cannot fail —
    /// but a batch that is a live transaction has a connection to acquire and a
    /// `BEGIN` to issue, and either can be refused (ADR-0062). Chunks already
    /// committed stay committed, and nothing from the chunk that would have
    /// followed was read.
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

    /// Discarding a chunk that applied nothing failed.
    #[error("discarding the batch failed")]
    Rollback {
        /// What the run had committed before this.
        progress: Progressed,
        /// The projection store's own refusal.
        #[source]
        source: W,
    },
}

impl<R, W> ProjectionError<R, W>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
{
    /// What the run committed before it stopped.
    ///
    /// The failure and the partial progress are one value, so a caller never
    /// has to ask the store how far it got in order to decide whether to retry.
    pub fn progress(&self) -> Progressed {
        match self {
            Self::Boundary(_) | Self::Checkpoint(_) | Self::KeySpaceExhausted { .. } => {
                Progressed::default()
            }
            Self::Read { progress, .. }
            | Self::Decode { progress, .. }
            | Self::Apply { progress, .. }
            | Self::Begin { progress, .. }
            | Self::Commit { progress, .. }
            | Self::Rollback { progress, .. } => *progress,
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
            Self::Decode { position, .. } | Self::Apply { position, .. } => Some(*position),
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
/// them into it, and hands the write set and the position of the last applied
/// event to the port's single [`commit`](ProjectionStore::commit) — which is
/// what makes the read model and the checkpoint move together or not at all.
///
/// # Which flavour this binds, and what spawning it costs
///
/// [`EventStore`], the flavour that does **not** require `Send` — so a store
/// held through an `Rc` on a single-threaded edge runtime runs a projection
/// here, and a `Send` store does too, through the blanket impl.
///
/// Spawning the returned future onto a multi-threaded runtime asks for one
/// thing more, and it is the caller's to supply. On a failure this runner
/// holds the stop — which carries `S::Error` — across the port's `rollback`
/// await, because one [`ProjectionError`] needs the error *and* what the
/// rollback said. `Error` carries no `Send` bound by design, so add
/// `S::Error: Send + Sync` to your own signature, or declare the marker
/// trait ADR-0009 records and bound on that instead.
/// `crates/happenstance/tests/flavours.rs` carries the worked shape.
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
/// The runner halts on the first failure. The alternative that lost was an
/// `on_error: SkipPolicy` argument, which is what a builder API invites: it
/// forces one answer onto every projection an application runs, and two read
/// models of different tolerance are exactly the case that makes one answer
/// wrong. Failure policy belongs to the projection, not to the loop.
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
/// [`ProjectionId`] and swap the reader over when it catches up: the port
/// already permits it and nothing else has to change.
///
/// # Errors
///
/// See [`ProjectionError`]. In short: the derived query, the checkpoint read,
/// the event stream, the decode, `apply`, the commit and the rollback each have
/// their own arm, every one of them carries the partial [`Progressed`], and the
/// two that name an event carry its position.
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
/// use happenstance::{Projection, ProjectionId, run_projection};
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
///     fn id(&self) -> &ProjectionId { &self.id }
///     fn scope(&self) -> &Tags { &self.scope }
///     fn apply(&mut self, _: Seat, batch: &mut MemoryProjectionBatch)
///         -> Result<(), MemoryProjectionStoreError> {
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
///     id: ProjectionId::new("seats_sold"),
///     scope: Tags::empty(),
///     sold: 0,
/// };
/// let done =
///     run_projection(&events, &models, &mut sold, &Json, 64.try_into()?)
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
/// There is no default and no named type, and that is a gap rather than a
/// position. [`Retry`](crate::Retry) — the same kind of caller-supplied
/// bound one module over — carries both, and the argument it makes for
/// having *no* default is about a worst case a caller must see, which does
/// not transfer unexamined to a knob that changes no outcome. What settles
/// it is a measurement — wall time, peak resident memory and commit count
/// against a real adapter across the range — which belongs in
/// `experiments/`, out of the gate, and which nobody has run. Until then
/// the `64` above is the doctest's number and not advice.
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
/// lease, and a second runner on the same `(store, ProjectionId)` is
/// accepted by every store in this workspace — so **exactly one runner per
/// `(store, ProjectionId)` is the caller's to guarantee**, by whatever
/// their deployment already uses to elect a singleton.
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
// `crates/happenstance/src/runner.rs:401` as the location of this item and that
// line number is only right while the fence stays where it is: prose added
// above it pushes the `run_projection` call down and leaves the three citations
// pointing at whatever now occupies 401.
//
// **The gate does not catch that, and this comment used to say it did.**
// `spec-trace` counts a citation and only *anchors* it where it can derive a
// subject from the prose beside it — 80 of 401, and these three are not among
// them. Measured rather than assumed: moving `# Choosing `chunk`` above the
// fence puts the call 39 lines from the cited line, and `spec-trace` reports the
// same "80 anchored" and exits 0. What actually holds the line is
// `crates/happenstance/tests/projection_runner_page.rs`, which asserts the two
// sections this lane added stay below the fence and says why.
//
// Moving a section up is still *allowed*; it costs the three citations moving
// with it, and nothing but a reader will tell you.
pub async fn run_projection<S, P, C>(
    events: &S,
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: NonZeroUsize,
) -> Result<Progressed, ProjectionError<S::Error, StoreError<P>>>
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

    let checkpoint = models
        .checkpoint(projection.id())
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
        let mut batch = match models.begin().await {
            Ok(batch) => batch,
            Err(source) => return Err(ProjectionError::Begin { progress, source }),
        };
        let mut applied = 0usize;
        let mut last = None;
        let mut stopped = None;
        let mut exhausted = false;

        // The bounded buffer: at most `chunk` events between one `begin` and
        // its `commit`. The stream is pulled item by item and never collected,
        // so memory stays flat in the length of the replay.
        while applied < chunk.get() {
            match next_event(stream.as_mut()).await {
                None => {
                    exhausted = true;
                    break;
                }
                Some(Err(source)) => {
                    stopped = Some(Stopped::Read(source));
                    break;
                }
                Some(Ok(event)) => {
                    if let Err(stop) = apply_one(projection, codec, &event, &mut batch) {
                        stopped = Some(stop);
                        break;
                    }
                    applied += 1;
                    last = Some(event.position);
                }
            }
        }

        if let Some(stopped) = stopped {
            // The half-applied chunk is discarded whole, so a restart re-reads
            // from the last committed checkpoint: nothing applied twice, and
            // nothing skipped.
            let rollback = models.rollback(batch).await.err();
            return Err(stopped.into_error(progress, rollback));
        }

        match last {
            // The rows and the checkpoint in the port's **single** `commit`.
            // The position is the one the last applied event actually carries,
            // never a computed or anticipated one.
            Some(position) => {
                models
                    .commit(batch, projection.id(), position, Authority::Live)
                    .await
                    .map_err(|source| ProjectionError::Commit { progress, source })?;
                progress.through = Some(position);
                progress.applied += applied;
            }
            // Nothing was applied, so there is nothing to make durable and no
            // checkpoint to move. Committing an empty batch here would claim
            // the run had considered a position it never reached.
            None => {
                models
                    .rollback(batch)
                    .await
                    .map_err(|source| ProjectionError::Rollback { progress, source })?;
            }
        }

        if exhausted {
            return Ok(progress);
        }
    }
}

/// Decodes one read event and applies it into the open write set.
///
/// Split out so the chunk loop reads as a loop: the two failures it can produce
/// are the two that discard the chunk.
fn apply_one<P, C, R>(
    projection: &mut P,
    codec: &C,
    event: &SequencedEvent,
    batch: &mut StoreBatch<P>,
) -> Result<(), Stopped<R, StoreError<P>>>
where
    P: Projection,
    C: Codec,
{
    // The crate's one decode path, through the codec tag the writer left, so an
    // event written under an older encoding still folds into today's model.
    let decoded = decode_event::<P::Event, C>(codec, event).map_err(|source| Stopped::Decode {
        position: event.position,
        source,
    })?;

    projection
        .apply(decoded, batch)
        .map_err(|source| Stopped::Apply {
            position: event.position,
            source,
        })
}

/// Why a chunk stopped, before the partial progress is attached to it.
///
/// The loop knows what went wrong; only the caller's frame knows how far the
/// run had got and whether the rollback that followed also failed. Keeping them
/// apart is what stops a `progress` field being threaded through every
/// construction site.
enum Stopped<R, W> {
    Read(R),
    Decode {
        position: SequencePosition,
        source: CodecError,
    },
    Apply {
        position: SequencePosition,
        source: W,
    },
}

impl<R, W> Stopped<R, W>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
{
    /// Attaches the run's partial progress and whatever the rollback said.
    fn into_error(self, progress: Progressed, rollback: Option<W>) -> ProjectionError<R, W> {
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
