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
//! records the class, and a [`Stop`](crate::FailureClass::Stop) stops the edge. What
//! the other classes call for — jittered, capped backoff for
//! [`Transient`](crate::FailureClass::Transient), halving for
//! [`Split`](crate::FailureClass::Split), holding an event back for
//! [`Park`](crate::FailureClass::Park) — a bounded outbound buffer, and tasks the
//! runner owns are part 2's.
//!
//! # A known gap: the push does not keep the origin's append groups
//!
//! A push sends the batch it read as **one** [`EventGroup`], because
//! [`EventStore::read`] does not say which events one append wrote together.
//! That merges independent local appends into one atomic unit on the far side,
//! which is safe, and it can cut one multi-event append across two pushes at a
//! batch boundary, which is not: between the two, the far side holds half of a
//! decision the origin made whole, a state the origin never had (SY-30).
//! Tracked by #420.
//!
//! # Caller-driven, and nothing spawned
//!
//! `step` borrows the peer and the store for the length of one exchange and
//! keeps neither. The edge owns only what is *about the link*: the far side's
//! resume token and the status. That is what lets the same edge be driven by a
//! test, by a Durable Object's alarm, or by a task part 2 spawns and owns.
//!
//! The bounds are the bare flavours ([`SyncPeer`], [`EventStore`],
//! [`IngestStore`]), the weaker requirement both flavours meet, so the Durable
//! Object's `!Send` store and peer drive an edge unchanged (SY-17). For a peer
//! and store of the `Send` flavours the step's future is `Send` too, given a
//! `Send + Sync` resume token; `tests/flavours.rs` proves both, the second in
//! generic code under `tokio::spawn`.
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

use happenstance_core::futures_core::Stream;
use happenstance_core::{EventStore, Query, ReadOptions, SequencePosition, SequencedEvent};

use crate::classify::Classify;
use crate::identity::ReplicatedEvent;
use crate::ingest::{IngestGroup, IngestStore, Ingested};
use crate::peer::{EventGroup, PeerLimits, PushBatch, SyncPeer};

mod error;
mod status;

pub use error::{Failure, Phase, StepError, StepFailure};
pub use status::{EdgeState, Pending, Stepped, SyncStatus};

/// One replication edge: the local side's view of one peer relationship.
///
/// Generic over the far side's resume token `R` rather than over the peer, so
/// the struct carries no bound and the peer is borrowed per
/// [`step`](Self::step) rather than owned.
///
/// # One edge, one peer, for its whole life
///
/// The token is the peer's, and the watermark says what *that* peer
/// acknowledged. Stepped against a different peer, the edge would hand the
/// second peer a token it never minted and skip every local event the first
/// one confirmed. Nothing in the type stops that, because the port carries no
/// peer identity to check, so it is the caller's to keep: build one edge per
/// peer, and a new edge when a peer is replaced.
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
/// use happenstance_sync::runner::{Edge, EdgeState, Pending};
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
/// assert_eq!(status.pending, Pending::Exactly(0));
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
    /// The pull token, and [`SyncStatus::waiting`], advance only once the
    /// pulled batch has been ingested, and the watermark only once the far
    /// side has acknowledged a push, so a failure anywhere re-offers the same
    /// events on the next step and loses none. The only local write is the
    /// ingest: a refused push leaves the local log as it was (SY-34).
    ///
    /// Local events are read from the position after
    /// [`SyncStatus::confirmed`], at most one more than the peer's
    /// [`limits`](SyncPeer::limits) let a batch carry. The contiguous run the
    /// limits admit is pushed; the run stops at the first event that does not
    /// fit rather than skipping it, because the watermark is a prefix. If not
    /// even the first fits, it is offered alone, and the peer's refusal states
    /// its class: the runner reads a class and never infers one (SY-36).
    ///
    /// The run goes as **one** atomic group, which can tear a multi-event
    /// local append across two pushes; see the module's known gap (SY-30,
    /// #420).
    ///
    /// # Errors
    ///
    /// A [`StepError`] naming the phase that failed and carrying the failure.
    /// Its class is recorded in [`SyncStatus::last_failure`]; a
    /// [`Stop`](crate::FailureClass::Stop) stops the edge, and every later step on it
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
        let limits = peer.limits();
        // One past a full batch: enough to tell "this is all of it" from
        // "there is more", without reading the whole unconfirmed tail.
        let limit = limits.max_batch_events.saturating_add(1);
        let options = match self.status.confirmed.map(SequencePosition::next) {
            None => ReadOptions::new(),
            Some(Some(next)) => ReadOptions::new().from(next),
            // The far side confirmed the last position there is.
            Some(None) => {
                self.status.pending = Pending::Exactly(0);
                return Ok(0);
            }
        }
        .limit(limit);
        let tail = Tail::gather(EventStore::read(store, &Query::all(), options), &limits)
            .await
            .map_err(StepError::Read)?;
        self.status.pending = tail.pending(tail.events.len().saturating_add(tail.beyond), limit);
        let Some(last) = tail.last else {
            return Ok(0);
        };

        let pushed = tail.events.len();
        let left_after_push = tail.pending(tail.beyond, limit);
        let batch = PushBatch::new(vec![EventGroup::new(None, tail.events)]);
        let ack = peer.push(&batch).await.map_err(StepError::Push)?;
        self.status.confirmed = Some(last);
        self.status.pending = left_after_push;
        self.status.far_side = ack.confirmed;
        Ok(pushed)
    }
}

/// The local events one step offers, and how many it read past them.
struct Tail {
    /// The contiguous run from the read's start that the peer's limits admit.
    events: Vec<ReplicatedEvent>,
    /// The local position of the last event in `events`.
    last: Option<SequencePosition>,
    /// How many events the read yielded past `events`.
    beyond: usize,
}

impl Tail {
    /// Reads the (limited) stream once: the admitted prefix is kept, the rest
    /// is counted and dropped.
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
                open = fits;
            } else {
                open = false;
                tail.beyond = tail.beyond.saturating_add(1);
            }
        }
        Ok(tail)
    }

    /// Whether the read stopped at `limit` rather than at the end of the log.
    fn truncated(&self, limit: usize) -> bool {
        self.events.len().saturating_add(self.beyond) >= limit
    }

    /// `count` as exact, or as a lower bound when the read was cut at `limit`.
    fn pending(&self, count: usize, limit: usize) -> Pending {
        if self.truncated(limit) {
            Pending::AtLeast(count)
        } else {
            Pending::Exactly(count)
        }
    }
}
