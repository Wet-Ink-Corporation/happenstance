//! The runner above the port: one edge, driven one step at a time (#244).
//!
//! # What is here, and what is not yet
//!
//! An [`Edge`] is one replication relationship as the local side sees it: a
//! local store that both reads ([`EventStore`]) and ingests ([`IngestStore`]),
//! exchanging with one [`SyncPeer`]. [`Edge::step`] does one exchange — pull
//! from the far side and ingest locally, then read the local log above the
//! confirmed watermark and push it — and updates the edge's [`SyncStatus`],
//! which [`Edge::status`] hands back without a round trip.
//!
//! **Part 1 of #244.** The edge reacts to a failure's class minimally: it
//! records the class, and a [`Stop`](FailureClass::Stop) stops the edge. What
//! the other classes call for — jittered, capped backoff for
//! [`Transient`](FailureClass::Transient), halving for
//! [`Split`](FailureClass::Split), holding an event back for
//! [`Park`](FailureClass::Park) — a bounded outbound buffer, and tasks the
//! runner owns are part 2's. So is the incarnation check, which needs the far
//! side's own `StoreId` and the port does not carry it.
//!
//! # Caller-driven, and nothing spawned
//!
//! `step` borrows the peer and the store for the length of one exchange and
//! keeps neither. The edge owns only what is *about the link*: the far side's
//! resume token and the status. That is what lets the same edge be driven by a
//! test, by a Durable Object's alarm, or by a task part 2 spawns and owns, and
//! it is why nothing here needs `Send`.
//!
//! The bounds are the bare flavours ([`SyncPeer`], [`EventStore`],
//! [`IngestStore`]), the weaker requirement both flavours meet, so the Durable
//! Object's `!Send` store and peer drive an edge unchanged (SY-17;
//! `tests/flavours.rs` instantiates it with both).
//!
//! # Where the watermark lives is not decided here
//!
//! SY-31 says the per-peer confirmation watermark is written through
//! `ProjectionStore` under a reserved id, and that seam is not built. The edge
//! holds the watermark and the token in memory, exposes both
//! ([`SyncStatus::confirmed`], [`Edge::resume_token`]), and takes them back
//! through [`Edge::resuming`], so a caller can persist them wherever it must.

use alloc::vec;
use alloc::vec::Vec;

use core::fmt;

use happenstance_core::futures_core::Stream;
use happenstance_core::{EventStore, Query, ReadOptions, SequencePosition, SequencedEvent};

use crate::classify::{Classify, FailureClass};
use crate::identity::{ReplicatedEvent, Watermark};
use crate::ingest::{IngestError, IngestGroup, IngestStore, Ingested};
use crate::peer::{EventGroup, PeerLimits, PushBatch, SyncPeer, Waiting};

/// One replication edge: the local side's view of one peer relationship.
///
/// Generic over the far side's resume token `R` rather than over the peer, so
/// the struct carries no bound and the peer is borrowed per
/// [`step`](Self::step) rather than owned.
///
/// # Examples
///
/// A local store pushes what it holds to a hub and pulls what the hub holds,
/// then reads its status without another round trip:
///
/// ```
/// # #[cfg(not(feature = "memory"))]
/// # fn main() {}
/// # #[cfg(feature = "memory")]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use happenstance_core::{Event, EventStore, StoreId};
/// use happenstance_sync::runner::{Edge, EdgeState};
/// use happenstance_sync::{MemoryIngestStore, MemorySyncPeer};
///
/// let hub = MemorySyncPeer::new(StoreId::from_bytes([2; 16]));
/// let tablet = MemoryIngestStore::new();
/// let taken = tablet
///     .append(&[Event::new("ReadingTaken", &b"{\"celsius\":4}"[..])?], None)
///     .await?;
///
/// let mut edge = Edge::new();
/// let stepped = edge.step(&hub, &tablet).await?;
///
/// assert_eq!(stepped.pushed, 1);
/// let status = edge.status();
/// assert_eq!(status.state, EdgeState::Running);
/// assert_eq!(status.confirmed, Some(taken));
/// assert_eq!(status.pending, Some(0));
/// assert!(status.is_synced());
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Edge<R> {
    /// Where the next pull resumes; `None` before the first.
    resume: Option<R>,
    status: SyncStatus,
}

impl<R> Default for Edge<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Edge<R> {
    /// An edge that has never exchanged: it pulls from the start of the far
    /// side's log and pushes the whole local log.
    #[must_use]
    pub const fn new() -> Self {
        Self::resuming(None, None)
    }

    /// An edge resuming from a watermark and a token a caller kept.
    ///
    /// `confirmed` is the local position the far side last acknowledged
    /// everything up to, as [`SyncStatus::confirmed`] reported it; `resume` is
    /// the token [`resume_token`](Self::resume_token) reported. Both are about
    /// this link only, and where they are stored is SY-31's.
    #[must_use]
    pub const fn resuming(confirmed: Option<SequencePosition>, resume: Option<R>) -> Self {
        Self {
            resume,
            status: SyncStatus::resuming(confirmed),
        }
    }

    /// The edge's status as of its last step. No I/O.
    #[must_use]
    pub const fn status(&self) -> &SyncStatus {
        &self.status
    }

    /// The token the next pull resumes from, for a caller that persists it.
    #[must_use]
    pub const fn resume_token(&self) -> Option<&R> {
        self.resume.as_ref()
    }

    /// One exchange: pull and ingest, then read and push.
    ///
    /// The pull token advances only once the pulled batch has been ingested,
    /// and the watermark only once the far side has acknowledged a push, so a
    /// failure anywhere re-offers the same events on the next step and loses
    /// none. The only local write is the ingest: a refused push leaves the
    /// local log as it was (SY-34).
    ///
    /// Local events are pushed from the position after
    /// [`SyncStatus::confirmed`], as many as the peer's
    /// [`limits`](SyncPeer::limits) admit, as one atomic group. If not even the
    /// first fits, it is offered alone, and the peer's refusal states its class:
    /// the runner reads a class and never infers one (SY-36).
    ///
    /// # Errors
    ///
    /// A [`StepError`] naming the phase that failed and carrying the failure.
    /// Its class is recorded in [`SyncStatus::last_failure`]; a
    /// [`Stop`](FailureClass::Stop) stops the edge, and every later step on it
    /// returns [`StepError::Stopped`] without any I/O.
    pub async fn step<P, S>(&mut self, peer: &P, store: &S) -> Result<Stepped, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: EventStore + IngestStore,
        <S as EventStore>::Error: Classify,
    {
        if let EdgeState::Stopped(cause) = self.status.state {
            return Err(StepError::Stopped(cause));
        }
        let outcome = self.exchange(peer, store).await;
        if let Err(error) = &outcome {
            self.status.record(error.failure());
        }
        outcome
    }

    async fn exchange<P, S>(&mut self, peer: &P, store: &S) -> Result<Stepped, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: EventStore + IngestStore,
        <S as EventStore>::Error: Classify,
    {
        let ingested = self.pull_and_ingest(peer, store).await?;
        let pushed = self.push_local(peer, store).await?;
        Ok(Stepped { ingested, pushed })
    }

    async fn pull_and_ingest<P, S>(
        &mut self,
        peer: &P,
        store: &S,
    ) -> Result<Ingested, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: EventStore + IngestStore,
    {
        let pulled = peer
            .pull(self.resume.as_ref())
            .await
            .map_err(StepError::Pull)?;
        // One ingest group per pulled group: the far side's atomic units land
        // as atomic units (SY-30). No compensation: adjudication is SY-7's.
        let groups: Vec<IngestGroup<'_>> = pulled
            .batch
            .groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]))
            .collect();
        let ingested = if groups.is_empty() {
            Ingested::default()
        } else {
            IngestStore::ingest(store, &groups)
                .await
                .map_err(StepError::Ingest)?
        };
        self.resume = Some(pulled.resume);
        self.status.waiting = pulled.waiting;
        Ok(ingested)
    }

    async fn push_local<P, S>(&mut self, peer: &P, store: &S) -> Result<usize, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: EventStore + IngestStore,
    {
        let options = match self.status.confirmed.map(SequencePosition::next) {
            None => ReadOptions::new(),
            Some(Some(next)) => ReadOptions::new().from(next),
            // The far side confirmed the last position there is.
            Some(None) => {
                self.status.pending = Some(0);
                return Ok(0);
            }
        };
        let tail = Tail::gather(
            EventStore::read(store, &Query::all(), options),
            &peer.limits(),
        )
        .await
        .map_err(StepError::Read)?;
        self.status.pending = Some(tail.in_batch.saturating_add(tail.beyond));
        let Some(last) = tail.last else {
            return Ok(0);
        };

        let pushed = tail.events.len();
        let batch = PushBatch::new(vec![EventGroup::new(None, tail.events)]);
        let ack = peer.push(&batch).await.map_err(StepError::Push)?;
        self.status.confirmed = Some(last);
        self.status.pending = Some(tail.beyond);
        self.status.far_side = ack.confirmed;
        Ok(pushed)
    }
}

/// The error [`Edge::step`] returns for a peer `P` and a store `S`.
pub type StepFailure<P, S> =
    StepError<<P as SyncPeer>::Error, <S as EventStore>::Error, <S as IngestStore>::Error>;

/// The local events one step offers, and how many it left behind.
struct Tail {
    /// The contiguous run from the read's start that the peer's limits admit.
    events: Vec<ReplicatedEvent>,
    /// The local position of the last event in `events`.
    last: Option<SequencePosition>,
    /// How many events are in `events`, counted as `u64`.
    in_batch: u64,
    /// How many events the read yielded past `events`.
    beyond: u64,
}

impl Tail {
    /// Reads the whole stream once: the admitted prefix is kept, the rest is
    /// counted and dropped, so the memory a step holds is one batch.
    ///
    /// The run stops at the first event that does not fit rather than skipping
    /// it, because the watermark is a prefix: an event left behind with a later
    /// one pushed could never be confirmed. If the first event does not fit at
    /// all it is taken alone, so the peer refuses it and says why.
    async fn gather<T, E>(stream: T, limits: &PeerLimits) -> Result<Self, E>
    where
        T: Stream<Item = Result<SequencedEvent, E>>,
    {
        let mut stream = core::pin::pin!(stream);
        let mut tail = Self {
            events: Vec::new(),
            last: None,
            in_batch: 0,
            beyond: 0,
        };
        let mut bytes = 0_usize;
        let mut open = true;
        while let Some(row) = core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
            let row = row?;
            let event = ReplicatedEvent::new(row.id, row.recorded_at, row.event);
            let size = event.payload_len();
            let fits = tail.events.len() < limits.max_batch_events
                && size <= limits.max_event_bytes
                && bytes.saturating_add(size) <= limits.max_batch_bytes;
            if open && (fits || tail.events.is_empty()) {
                bytes = bytes.saturating_add(size);
                tail.last = Some(row.position);
                tail.events.push(event);
                tail.in_batch = tail.in_batch.saturating_add(1);
                open = fits;
            } else {
                open = false;
                tail.beyond = tail.beyond.saturating_add(1);
            }
        }
        Ok(tail)
    }
}

/// What [`Edge::step`] moved.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Stepped {
    /// What the local store did with the pulled batch.
    pub ingested: Ingested,
    /// How many local events the far side acknowledged.
    pub pushed: usize,
}

/// One edge's status, as of its last step.
///
/// Held by the edge and updated by every step, so reading it costs no round
/// trip. `#[non_exhaustive]`: part 2 of #244 adds to it (a retention-gap flag
/// for SY-32, the far side's incarnation), and a field added then must not
/// break a reader.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncStatus {
    /// Running, or stopped and why.
    pub state: EdgeState,
    /// The local position the far side has acknowledged everything up to.
    ///
    /// A local [`SequencePosition`], meaningful only in the local store: it is
    /// the watermark SY-31 says is kept through the projection port, and it is
    /// never sent to the far side.
    pub confirmed: Option<SequencePosition>,
    /// How many local events sit above [`confirmed`](Self::confirmed), counted
    /// at the last step's read; `None` before the first.
    ///
    /// Every local event counts, including ones this edge ingested from the far
    /// side, which a push hands back and the far side skips.
    pub pending: Option<u64>,
    /// What the far side said it still holds past the last pull's token.
    pub waiting: Waiting,
    /// What the far side last acknowledged holding, per origin store: its
    /// [`Ack::confirmed`](crate::Ack::confirmed), a high-water mark per origin
    /// rather than a contiguous prefix.
    pub far_side: Watermark,
    /// The phase and class of the most recent failed step, kept after later
    /// steps succeed; `None` if no step has failed.
    pub last_failure: Option<Failure>,
}

impl SyncStatus {
    const fn resuming(confirmed: Option<SequencePosition>) -> Self {
        Self {
            state: EdgeState::Running,
            confirmed,
            pending: None,
            waiting: Waiting::Unknown,
            far_side: Watermark::new(),
            last_failure: None,
        }
    }

    /// Running, with no local event unconfirmed and the far side saying nothing
    /// is waiting, as of the last step.
    #[must_use]
    pub fn is_synced(&self) -> bool {
        self.state == EdgeState::Running
            && self.pending == Some(0)
            && self.waiting == Waiting::Nothing
    }

    fn record(&mut self, failure: Failure) {
        self.last_failure = Some(failure);
        if failure.class == FailureClass::Stop {
            self.state = EdgeState::Stopped(failure);
        }
    }
}

/// Whether an edge still exchanges.
///
/// `#[non_exhaustive]`: part 2's backoff may add a state between the two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EdgeState {
    /// The next step exchanges.
    Running,
    /// A failure of class [`Stop`](FailureClass::Stop) halted the edge. It does
    /// no more I/O; an operator fixes the cause and builds a new edge.
    Stopped(Failure),
}

/// Which part of a step failed, and the class the failure stated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Failure {
    /// Where the step failed.
    pub phase: Phase,
    /// What the failure said to do next (SY-36).
    pub class: FailureClass,
}

impl Failure {
    /// A failure in `phase` of class `class`.
    #[must_use]
    pub const fn new(phase: Phase, class: FailureClass) -> Self {
        Self { phase, class }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = match self.class {
            FailureClass::Transient => "transient",
            FailureClass::Split => "split",
            FailureClass::Park => "park",
            FailureClass::Stop => "stop",
        };
        write!(f, "a {class}-class failure in the {} phase", self.phase)
    }
}

/// The four parts of a step, in the order a step runs them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Phase {
    /// Pulling from the far side.
    Pull,
    /// Ingesting the pulled batch into the local store.
    Ingest,
    /// Reading the local log above the watermark.
    Read,
    /// Pushing to the far side.
    Push,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Pull => "pull",
            Self::Ingest => "ingest",
            Self::Read => "read",
            Self::Push => "push",
        })
    }
}

/// Why [`Edge::step`] failed: the phase, and the failure it met.
///
/// `P` is the peer's error, `R` the local store's read error and `I` its
/// ingest error. They are separate because a store may implement
/// [`EventStore`] and [`IngestStore`] with different error types. No bounds sit
/// on the enum, so it is `Send` exactly when the three are, and a `!Send`
/// adapter's error stays legal (ADR-0009).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StepError<P, R, I> {
    /// The edge was already stopped, by the failure named; nothing was done.
    #[error("the edge is stopped by {0}")]
    Stopped(Failure),
    /// The pull failed.
    #[error("the pull from the far side failed")]
    Pull(#[source] P),
    /// The pulled batch did not land locally; nothing in it was written.
    #[error("the pulled batch did not land in the local store")]
    Ingest(#[source] IngestError<I>),
    /// Reading the local log failed.
    #[error("reading the local log failed")]
    Read(#[source] R),
    /// The far side refused or failed the push; the local log is unchanged.
    #[error("the push to the far side failed")]
    Push(#[source] P),
}

impl<P: Classify, R: Classify, I: Classify> StepError<P, R, I> {
    /// The phase that failed and the class its failure stated.
    #[must_use]
    pub fn failure(&self) -> Failure {
        match self {
            Self::Stopped(cause) => *cause,
            Self::Pull(error) => Failure::new(Phase::Pull, error.class()),
            Self::Ingest(error) => Failure::new(Phase::Ingest, error.class()),
            Self::Read(error) => Failure::new(Phase::Read, error.class()),
            Self::Push(error) => Failure::new(Phase::Push, error.class()),
        }
    }
}

/// The class of the failure inside, so a step's error is handled like a peer's
/// (ADR-0026 §2.4). A stopped edge answers [`Stop`](FailureClass::Stop).
impl<P: Classify, R: Classify, I: Classify> Classify for StepError<P, R, I> {
    fn class(&self) -> FailureClass {
        self.failure().class
    }
}
