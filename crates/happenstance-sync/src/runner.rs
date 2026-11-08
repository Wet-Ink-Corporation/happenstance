//! The runner above the port: one edge, driven by its caller (#244).
//!
//! # What is here
//!
//! An [`Edge`] is one replication relationship as the local side sees it: a
//! local store that reads its write groups ([`WriteGroupStore`]) and ingests
//! ([`IngestStore`]), exchanging with one [`SyncPeer`]. [`Edge::step`] does one
//! exchange — pull from the far side and ingest locally, then read the local
//! log above the confirmed watermark and push it — and updates the edge's
//! [`SyncStatus`], which [`Edge::status`] hands back without a round trip.
//! [`Edge::run`] steps until the edge is synced or halts, waiting out each
//! transient failure through a [`Backoff`] and a [`Sleep`] the caller supplies.
//!
//! # What each failure class does
//!
//! A failed step records its phase and class in [`SyncStatus::last_failure`],
//! and the class decides what the edge does next (SY-36). The runner reads the
//! class; it never infers one.
//!
//! * [`Transient`](crate::FailureClass::Transient): nothing changes, and the
//!   same step can succeed later. [`Edge::run`] waits out a seeded, jittered,
//!   capped delay and retries, and returns the error once the backoff's
//!   attempts are spent, with the edge still running. It never escalates a
//!   transient failure to a stop by counting: a Durable Object that was reset
//!   comes back either as itself, and the edge carries on, or under a new
//!   identity, and the incarnation check stops it.
//! * [`Split`](crate::FailureClass::Split) on a push of several write groups:
//!   the edge halves the events it offers per push, for the rest of its life,
//!   and the next step offers fewer groups. A split refusing **one** group parks
//!   the edge, because a group is never cut (SY-30); so does a split outside
//!   the push, where the runner holds no batch it could halve.
//! * [`Park`](crate::FailureClass::Park): the edge parks ([`EdgeState::Parked`]).
//! * [`Stop`](crate::FailureClass::Stop): the edge stops ([`EdgeState::Stopped`]).
//!
//! A parked or stopped edge does no more I/O: every later step returns
//! [`StepError::Parked`] or [`StepError::Stopped`] at once.
//!
//! # The incarnation check
//!
//! An answer naming a store other than the one the edge synced with stops the
//! edge before the answer is acted on, and leaves the local log alone (SY-34,
//! ADR-0104). [`IncarnationChanged`] says which store it knew and which
//! answered; the module it lives in says why the check is shaped as it is.
//!
//! # Groups: a push carries the writes the local log was written in
//!
//! A push reads the local log through
//! [`read_groups`](WriteGroupStore::read_groups) and sends **one**
//! [`EventGroup`](crate::EventGroup) per local write group, each whole, with an
//! ingested group's guard back on it (SY-30, SY-39). So a multi-event append is
//! never cut across two pushes, independent appends are never merged into one
//! atomic unit, and
//! a foreign group this edge relays keeps its boundary and its guard. After an
//! acknowledged push, every local event at or below
//! [`SyncStatus::confirmed`] has been pushed.
//!
//! A group the peer's limits cannot carry is offered **alone**, never split,
//! and the peer's refusal states its class (SY-36).
//!
//! # A bounded outbound buffer
//!
//! A step holds at most one push's worth of local events in memory, and the
//! push is capped by the edge's own [buffer](Edge::with_buffer) as well as by
//! the peer's declared [`limits`](SyncPeer::limits): a peer that declares
//! `usize::MAX` events per batch does not make the edge read the whole
//! unconfirmed tail. The local log is the queue; nothing is copied out of it
//! ahead of the push that carries it.
//!
//! # Caller-driven, and nothing spawned
//!
//! `step` and `run` borrow the peer and the store for as long as they run and
//! keep neither. The edge owns only what is *about the link*: the far side's
//! resume token, its incarnation and the status. Nothing here spawns a task or
//! holds a timer, so the same edge is driven by a test, by a Durable Object's
//! alarm, or by a task the caller spawns on its own runtime; dropping
//! `run`'s future between steps loses nothing a later step does not redo.
//!
//! The bounds are the bare flavours ([`SyncPeer`], [`WriteGroupStore`],
//! [`IngestStore`], [`Sleep`]), the weaker requirement both flavours meet, so
//! the Durable Object's `!Send` store and peer drive an edge unchanged (SY-17).
//! For a peer and store of the `Send` flavours the step's future is `Send` too,
//! given a `Send + Sync` resume token; `tests/flavours.rs` proves both, the
//! second in generic code under `tokio::spawn`.
//!
//! # Where the watermark lives is not decided here
//!
//! SY-31 says the per-peer confirmation watermark is written through
//! `ProjectionStore` under a reserved id, and that seam is not built. The edge
//! holds the watermark, the token and the far side's incarnation in memory,
//! exposes them ([`SyncStatus::confirmed`], [`Edge::resume_token`],
//! [`SyncStatus::responder`]), and takes them back through [`Edge::resuming`]
//! and [`Edge::with_responder`], so a caller can persist them wherever it must.

use alloc::vec::Vec;
use core::num::NonZeroUsize;

use happenstance_core::{SequencePosition, StoreId};

use crate::classify::FailureClass;
use crate::ingest::{IngestGroup, IngestStore, Ingested};
use crate::peer::{PushBatch, SyncPeer};
use crate::write_group::WriteGroupStore;

mod backoff;
mod drive;
mod error;
mod incarnation;
mod run;
mod status;

use run::Run;

pub use backoff::{Backoff, BackoffError, SendSleep, Sleep};
pub use error::{Failure, Phase, StepError, StepFailure};
pub use incarnation::{IncarnationChanged, StopCause};
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
/// one confirmed. Where the far side names its store on its answers
/// (ADR-0104), the edge catches that: it stops at the first answer from
/// another store ([`StopCause::Incarnation`]). Where it does not, nothing in
/// the type stops it, so it is the caller's to keep: build one edge per peer,
/// and a new edge when a peer is replaced.
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
    /// The most local events one push carries, whatever the peer declares.
    buffer: NonZeroUsize,
    /// Lowered by each [`Split`](FailureClass::Split) the edge could halve,
    /// and never raised: the peer said its declared limit was too high.
    split_cap: Option<NonZeroUsize>,
    /// The size of the push in flight, so a split refusing it can be halved.
    offered: Option<Offered>,
}

/// How many write groups, and events, one push offered.
#[derive(Debug, Clone, Copy)]
struct Offered {
    groups: usize,
    events: usize,
}

impl<R> Default for Edge<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> Edge<R> {
    /// The most local events one push carries unless
    /// [`with_buffer`](Self::with_buffer) says otherwise: eight times
    /// [`PeerLimits::minimum`](crate::PeerLimits::minimum)'s batch.
    pub const DEFAULT_BUFFER: NonZeroUsize = NonZeroUsize::MIN.saturating_add(1023);

    /// An edge that has never exchanged: it pulls from the start of the far
    /// side's log and pushes the whole local log.
    #[must_use]
    pub const fn new() -> Self {
        Self::resuming(None, None)
    }

    /// An edge resuming from a watermark and a token a caller kept.
    ///
    /// `confirmed` is the first position of the last local write group the far
    /// side acknowledged, as [`SyncStatus::confirmed`] reported it; `resume` is
    /// the token [`resume_token`](Self::resume_token) reported. Both are about
    /// this link only, and where they are stored is SY-31's. Pair it with
    /// [`with_responder`](Self::with_responder), so the resumed edge still
    /// knows which store it synced with.
    #[must_use]
    pub const fn resuming(confirmed: Option<SequencePosition>, resume: Option<R>) -> Self {
        Self {
            resume,
            status: SyncStatus::resuming(confirmed),
            buffer: Self::DEFAULT_BUFFER,
            split_cap: None,
            offered: None,
        }
    }

    /// The same edge, knowing the far side as `responder`: the
    /// [`SyncStatus::responder`] a caller kept. Its first answer from any
    /// other store stops it.
    ///
    /// Without it, a resumed edge pins the first store that answers it, so a
    /// far side restored while the edge was down would go unnoticed.
    #[must_use]
    pub const fn with_responder(mut self, responder: StoreId) -> Self {
        self.status.responder = Some(responder);
        self
    }

    /// The same edge, carrying at most `buffer` local events per push, and
    /// holding at most one more than that in memory per step, whatever the
    /// peer's [`limits`](SyncPeer::limits) declare. A single write group larger
    /// than `buffer` is still read and offered whole, alone: the store's own
    /// write limits bound it.
    #[must_use]
    pub const fn with_buffer(mut self, buffer: NonZeroUsize) -> Self {
        self.buffer = buffer;
        self
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
    /// Local events are read in their write groups above
    /// [`SyncStatus::confirmed`], one event more than a batch may carry: the
    /// least of the peer's [`limits`](SyncPeer::limits), the edge's
    /// [buffer](Self::with_buffer) and any cap a split left. The groups that
    /// fit are pushed, one [`EventGroup`](crate::EventGroup) each; the run
    /// stops at the first group that does not fit rather than skipping it,
    /// because the watermark is a prefix. If not even the first fits, it is
    /// offered alone, and the peer's refusal states its class: the runner reads
    /// a class and never infers one (SY-36).
    ///
    /// An answer naming a store other than [`SyncStatus::responder`] is not
    /// acted on: a pulled batch is not ingested, and an ack does not move the
    /// watermark.
    ///
    /// # Errors
    ///
    /// A [`StepError`] naming the phase that failed and carrying the failure,
    /// or [`StepError::Incarnation`] for an answer from another store. Its
    /// class is recorded in [`SyncStatus::last_failure`] and acted on as the
    /// [module](self) describes. A parked or stopped edge returns
    /// [`StepError::Parked`] or [`StepError::Stopped`] without any I/O.
    pub async fn step<P, S>(&mut self, peer: &P, store: &S) -> Result<Stepped, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: WriteGroupStore + IngestStore,
    {
        match self.status.state {
            EdgeState::Running => {}
            EdgeState::Parked(failure) => return Err(StepError::Parked(failure)),
            EdgeState::Stopped(cause) => return Err(StepError::Stopped(cause)),
        }
        self.offered = None;
        let outcome = self.exchange(peer, store).await;
        if let Err(error) = &outcome {
            let changed = match error {
                StepError::Incarnation(change) => Some(*change),
                _ => None,
            };
            self.react(error.failure(), changed);
        }
        outcome
    }

    /// Records `failure` and moves the edge to the state its class calls for.
    fn react(&mut self, failure: Failure, changed: Option<IncarnationChanged>) {
        self.status.last_failure = Some(failure);
        self.status.state = if let Some(change) = changed {
            EdgeState::Stopped(StopCause::Incarnation(change))
        } else {
            match failure.class {
                FailureClass::Transient => EdgeState::Running,
                FailureClass::Split => match self.halved(failure.phase) {
                    Some(cap) => {
                        self.split_cap = Some(cap);
                        EdgeState::Running
                    }
                    None => EdgeState::Parked(failure),
                },
                FailureClass::Park => EdgeState::Parked(failure),
                FailureClass::Stop => EdgeState::Stopped(StopCause::Failure(failure)),
            }
        };
    }

    /// Half the events of the push a split refused, if it carried at least
    /// two groups. One group is never cut, and outside the push there is no
    /// batch of the runner's to halve. Each halving offers fewer groups next
    /// time, since a run of several groups under half its events cannot hold
    /// them all, so a run of splits ends in a park.
    fn halved(&self, phase: Phase) -> Option<NonZeroUsize> {
        let offered = self.offered.filter(|_| phase == Phase::Push)?;
        (offered.groups >= 2)
            .then(|| NonZeroUsize::new(offered.events >> 1).unwrap_or(NonZeroUsize::MIN))
    }

    /// Whether `answered` contradicts the store this edge knows, as a
    /// failure in `phase`.
    fn check(&self, phase: Phase, answered: Option<StoreId>) -> Result<(), IncarnationChanged> {
        match (self.status.responder, answered) {
            (Some(known), Some(answered)) if known != answered => {
                Err(IncarnationChanged::new(phase, known, answered))
            }
            _ => Ok(()),
        }
    }

    /// Pins the far side's incarnation from an answer that took effect, if
    /// none is pinned yet. An answer naming none pins nothing.
    const fn pin(&mut self, answered: Option<StoreId>) {
        if self.status.responder.is_none() {
            self.status.responder = answered;
        }
    }

    async fn exchange<P, S>(&mut self, peer: &P, store: &S) -> Result<Stepped, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: WriteGroupStore + IngestStore,
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
        S: WriteGroupStore + IngestStore,
    {
        let pulled = peer
            .pull(self.resume.as_ref())
            .await
            .map_err(StepError::Pull)?;
        // Before the ingest: a batch from another incarnation never lands.
        self.check(Phase::Pull, pulled.responder)
            .map_err(StepError::Incarnation)?;
        // One ingest group per pulled group: the far side's atomic units land
        // as atomic units (SY-30), each with its guard, which the store keeps
        // so a relay forwards it. No compensation: adjudication is SY-7's.
        let groups: Vec<IngestGroup<'_>> = pulled
            .batch
            .groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
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
        self.pin(pulled.responder);
        Ok(ingested)
    }

    async fn push_local<P, S>(&mut self, peer: &P, store: &S) -> Result<usize, StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: WriteGroupStore + IngestStore,
    {
        let mut limits = peer.limits();
        let ceiling = self
            .split_cap
            .map_or(self.buffer, |cap| cap.min(self.buffer));
        limits.max_batch_events = limits.max_batch_events.min(ceiling.get());
        // One past a full batch: enough to tell "this is all of it" from
        // "there is more", without reading the whole unconfirmed tail. The
        // ceiling keeps it finite whatever the peer declares.
        let read_limit = NonZeroUsize::MIN.saturating_add(limits.max_batch_events);
        let page = WriteGroupStore::read_groups(store, self.status.confirmed, read_limit)
            .await
            .map_err(StepError::Read)?;
        let run = Run::gather(page, &limits, read_limit);
        self.status.pending = run.pending(run.events.saturating_add(run.left));
        let Some(last_first) = run.last_first else {
            return Ok(0);
        };

        let pushed = run.events;
        let left_after_push = run.pending(run.left);
        self.offered = Some(Offered {
            groups: run.groups.len(),
            events: run.events,
        });
        let batch = PushBatch::new(run.groups);
        let ack = peer.push(&batch).await.map_err(StepError::Push)?;
        // Before the watermark: an ack from another incarnation confirms
        // nothing this edge can rely on.
        self.check(Phase::Push, ack.responder)
            .map_err(StepError::Incarnation)?;
        self.status.confirmed = Some(last_first);
        self.status.pending = left_after_push;
        self.status.far_side = ack.confirmed;
        self.pin(ack.responder);
        Ok(pushed)
    }
}
