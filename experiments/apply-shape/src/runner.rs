//! A minimal runner over the proposed trait.
//!
//! The shipped `run_projection`'s loop (`crates/happenstance/src/runner.rs`),
//! cut to what the question needs and changed in exactly three places:
//!
//! 1. `apply` is awaited;
//! 2. the event is handed over as a [`Delivered`], built from the
//!    `SequencedEvent`'s `id` — which the shipped runner discards;
//! 3. a decode or apply failure is first offered to
//!    [`Projection::on_error`], and a [`Policy::Skip`] carries on in the same
//!    batch instead of discarding it.
//!
//! What it deliberately does not reproduce: codec **tag** resolution. The
//! shipped runner decodes through the crate-private `decode_event`, which reads
//! the codec tag framed into the metadata; this crate cannot reach it and calls
//! [`DomainEvent::decode`] with the codec in hand. The events these tests
//! append carry no framing, so the two agree here, and tag resolution is not
//! what is being asked about.

use core::num::NonZeroUsize;
use core::pin::Pin;

use futures_core::Stream;
use happenstance::{
    Authority, Checkpoint, Codec, CodecError, CommitError, DomainEvent, EventStore, InvalidQuery,
    ProjectionStore, Query, QueryItem, ReadOptions, SequencePosition, SequencedEvent,
};

use crate::shape::{ApplyFailure, Delivered, Policy, Projection};

/// The store error of a projection's store.
pub type StoreError<P> = <<P as Projection>::Store as ProjectionStore>::Error;
/// The write set a projection's `apply` writes into.
pub type StoreBatch<P> = <<P as Projection>::Store as ProjectionStore>::Batch;

/// How far a run got.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct Ran {
    /// Where the checkpoint was last moved to.
    pub through: Option<SequencePosition>,
    /// Events applied and committed.
    pub applied: usize,
    /// Events `on_error` chose to skip, and committed past.
    pub skipped: usize,
}

/// Why a run stopped — `ProjectionError<R, W, A>`, the three-parameter shape
/// the brief proposes for PS-28.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RunError<R, W, A>
where
    R: core::error::Error + 'static,
    W: core::error::Error + 'static,
    A: core::error::Error + 'static,
{
    /// The projection constrains nothing.
    #[error(transparent)]
    Boundary(#[from] InvalidQuery),
    /// Reading the checkpoint failed.
    #[error("reading the checkpoint failed")]
    Checkpoint(#[source] W),
    /// The checkpoint is the last representable position.
    #[error("the position key space is exhausted at {through}")]
    KeySpaceExhausted {
        /// The checkpoint.
        through: SequencePosition,
    },
    /// The event stream failed.
    #[error("reading the event stream failed")]
    Read {
        /// What had been committed.
        progress: Ran,
        /// The event store's error.
        #[source]
        source: R,
        /// A rollback that also failed.
        rollback: Option<W>,
    },
    /// Decoding failed and `on_error` said halt.
    #[error("decoding the event at position {position} failed")]
    Decode {
        /// What had been committed.
        progress: Ran,
        /// Where it sits in this store — the runner knows; `apply` does not.
        position: SequencePosition,
        /// The codec's refusal.
        #[source]
        source: CodecError,
        /// A rollback that also failed.
        rollback: Option<W>,
    },
    /// `apply` refused and `on_error` said halt.
    #[error("applying the event at position {position} failed")]
    Apply {
        /// What had been committed.
        progress: Ran,
        /// Where it sits in this store.
        position: SequencePosition,
        /// The **projection's own** error — not the store's (PS-28).
        #[source]
        source: A,
        /// A rollback that also failed.
        rollback: Option<W>,
    },
    /// `on_error` itself failed — for instance writing the skip record.
    #[error("the failure policy for the event at position {position} failed")]
    Policy {
        /// What had been committed.
        progress: Ran,
        /// Where it sits in this store.
        position: SequencePosition,
        /// The projection's error from `on_error`.
        #[source]
        source: A,
        /// A rollback that also failed.
        rollback: Option<W>,
    },
    /// Opening a batch failed.
    #[error("opening a write set failed")]
    Begin {
        /// What had been committed.
        progress: Ran,
        /// The store's error.
        #[source]
        source: W,
    },
    /// The chunk's commit failed.
    #[error("committing the chunk failed")]
    Commit {
        /// What had been committed.
        progress: Ran,
        /// The port's outcome.
        #[source]
        source: CommitError<W>,
    },
}

/// The error `run` returns for a given event store and projection.
pub type RunErrorFor<S, P> =
    RunError<<S as EventStore>::Error, StoreError<P>, <P as Projection>::Error>;

/// Streams what the projection nominates and applies it, in chunks.
///
/// Bound on the bare flavours — [`EventStore`] and [`Projection`] — so a `!Send`
/// store and a `!Send` projection run here, and a `Send` one does too through
/// the blanket impls (RS-20-2).
///
/// # Errors
///
/// Every arm of [`RunError`]; a chunk that stops is rolled back whole.
pub async fn run<S, P, C>(
    events: &S,
    models: &P::Store,
    projection: &mut P,
    codec: &C,
    chunk: NonZeroUsize,
) -> Result<Ran, RunErrorFor<S, P>>
where
    S: EventStore,
    P: Projection,
    C: Codec,
{
    let item = QueryItem::new(
        P::Event::EVENT_TYPES.iter().cloned(),
        projection.scope().clone(),
    )?;
    let query = Query::from_item(item);

    let checkpoint = models
        .checkpoint(projection.id())
        .await
        .map_err(RunError::Checkpoint)?;

    let mut options = ReadOptions::new();
    if let Some(through) = considered_through(checkpoint) {
        let Some(resume) = through.next() else {
            return Err(RunError::KeySpaceExhausted { through });
        };
        options = options.from(resume);
    }

    let stream = events.read(&query, options);
    let mut stream = core::pin::pin!(stream);
    let mut progress = Ran::default();

    loop {
        let first = match next_event(stream.as_mut()).await {
            None => return Ok(progress),
            Some(Err(source)) => {
                return Err(Stopped::Read(source).into_error(progress, None));
            }
            Some(Ok(event)) => event,
        };

        let mut batch = match models.begin().await {
            Ok(batch) => batch,
            Err(source) => return Err(RunError::Begin { progress, source }),
        };

        let mut applied = 0usize;
        let mut skipped = 0usize;
        let mut last = first.position;
        let mut exhausted = false;

        let mut outcome = apply_one(projection, codec, &first, &mut batch).await;
        loop {
            match outcome {
                Ok(Applied::Applied) => applied += 1,
                Ok(Applied::Skipped) => skipped += 1,
                Err(stopped) => {
                    let rollback = models.rollback(batch).await.err();
                    return Err(stopped.into_error(progress, rollback));
                }
            }
            if applied + skipped >= chunk.get() {
                break;
            }
            match next_event(stream.as_mut()).await {
                None => {
                    exhausted = true;
                    break;
                }
                Some(Err(source)) => {
                    let rollback = models.rollback(batch).await.err();
                    return Err(Stopped::Read(source).into_error(progress, rollback));
                }
                Some(Ok(event)) => {
                    last = event.position;
                    outcome = apply_one(projection, codec, &event, &mut batch).await;
                }
            }
        }

        models
            .commit(batch, projection.id(), last, Authority::Live)
            .await
            .map_err(|source| RunError::Commit { progress, source })?;
        progress.through = Some(last);
        progress.applied += applied;
        progress.skipped += skipped;

        if exhausted {
            return Ok(progress);
        }
    }
}

/// A payload did not decode: offer it to `on_error`, owning the error.
async fn decode_failed<P, R>(
    projection: &mut P,
    id: happenstance::EventId,
    position: SequencePosition,
    source: CodecError,
    batch: &mut StoreBatch<P>,
) -> Result<Applied, Stopped<R, P::Error>>
where
    P: Projection,
{
    let failure = ApplyFailure::Decode {
        id,
        source: &source,
    };
    match projection.on_error(&failure, batch).await {
        Ok(Policy::Skip) => Ok(Applied::Skipped),
        Ok(_halt) => Err(Stopped::Decode { position, source }),
        Err(source) => Err(Stopped::Policy { position, source }),
    }
}

/// What happened to one event.
enum Applied {
    Applied,
    Skipped,
}

/// Decodes, delivers, and — on failure — asks the projection what to do.
///
/// The position is the runner's to report and never the projection's to see:
/// it goes into [`Stopped`], and only the [`EventId`](happenstance::EventId)
/// crosses into `apply` and `on_error`.
async fn apply_one<P, C, R>(
    projection: &mut P,
    codec: &C,
    event: &SequencedEvent,
    batch: &mut StoreBatch<P>,
) -> Result<Applied, Stopped<R, P::Error>>
where
    P: Projection,
    C: Codec,
{
    let position = event.position;
    let id = event.id;

    // A decode failure is offered to `on_error` from inside the `Err` arm, so
    // the scrutinee — a `Result<P::Event, CodecError>` — is held across that
    // await, and a generic spawner proves `P::Event: Send`. One restructuring
    // (a two-step `match`) did not shed it on 1.97.1 (README §4.1); every real
    // `SendProjection` needs a `Send` event anyway, so the bound is recorded
    // rather than fought.
    let decoded = match P::Event::decode(codec, event.event.event_type(), event.event.data()) {
        Ok(decoded) => decoded,
        Err(source) => return decode_failed(projection, id, position, source, batch).await,
    };

    match projection.apply(Delivered::new(id, decoded), batch).await {
        Ok(()) => Ok(Applied::Applied),
        Err(source) => {
            let failure = ApplyFailure::Apply {
                id,
                source: &source,
            };
            match projection.on_error(&failure, batch).await {
                Ok(Policy::Skip) => Ok(Applied::Skipped),
                Ok(_halt) => Err(Stopped::Apply { position, source }),
                Err(policy) => Err(Stopped::Policy {
                    position,
                    source: policy,
                }),
            }
        }
    }
}

/// Why a chunk stopped, before progress and the rollback are attached.
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
    fn into_error<W>(self, progress: Ran, rollback: Option<W>) -> RunError<R, W, A>
    where
        W: core::error::Error + 'static,
    {
        match self {
            Self::Read(source) => RunError::Read {
                progress,
                source,
                rollback,
            },
            Self::Decode { position, source } => RunError::Decode {
                progress,
                position,
                source,
                rollback,
            },
            Self::Apply { position, source } => RunError::Apply {
                progress,
                position,
                source,
                rollback,
            },
            Self::Policy { position, source } => RunError::Policy {
                progress,
                position,
                source,
                rollback,
            },
        }
    }
}

/// One item from a pinned stream, without `futures-util`.
async fn next_event<S, T, E>(mut stream: Pin<&mut S>) -> Option<Result<T, E>>
where
    S: Stream<Item = Result<T, E>>,
{
    core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await
}

/// Where a checkpoint says to resume from.
const fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
        _ => None,
    }
}
