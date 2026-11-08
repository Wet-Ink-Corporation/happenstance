//! What an edge reports about itself without a round trip.

use happenstance_core::{SequencePosition, StoreId};

use super::error::Failure;
use super::incarnation::StopCause;
use crate::identity::Watermark;
use crate::ingest::Ingested;
use crate::peer::Waiting;

/// What [`Edge::step`](super::Edge::step) moved.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Stepped {
    /// What the local store did with the pulled batch.
    pub ingested: Ingested,
    /// How many local events the far side acknowledged. A `usize`, as
    /// [`Pending`]'s counts and [`Ingested`]'s are.
    pub pushed: usize,
}

/// How many local events sit above the confirmed watermark.
///
/// A step reads at most one batch past the watermark, plus one event, rather
/// than the whole unconfirmed tail: reading the tail on every step costs
/// O(N²/B) over a backlog of N in batches of B, and on a one-shot-HTTP store
/// it puts the whole tail in one response. So a long backlog is reported as a
/// lower bound. Spelled as an enum, not a count beside a flag, so a reader
/// cannot take a bound for an exact figure.
///
/// `#[non_exhaustive]`, so an exact count a store can supply cheaply (a
/// `count(*)`) can be told apart later without breaking a match.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Pending {
    /// Not counted yet: no step has read the local log.
    #[default]
    Unknown,
    /// Exactly this many: the last read reached the end of the local log.
    Exactly(usize),
    /// At least this many: the last read stopped at its limit with more to
    /// come.
    AtLeast(usize),
}

/// One edge's status, as of its last step.
///
/// Held by the edge and updated by every step, so reading it costs no round
/// trip. `#[non_exhaustive]`: a retention-gap flag for SY-32 is still to come,
/// and a field added then must not break a reader.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SyncStatus {
    /// Running, parked, or stopped, and why.
    pub state: EdgeState,
    /// The far side's store incarnation as this edge knows it: the first
    /// responder named by an answer that took effect, or the one
    /// [`Edge::with_responder`](super::Edge::with_responder) named. `None`
    /// until then, and for good if the far side never names one (ADR-0104).
    ///
    /// An answer naming another store stops the edge
    /// ([`StopCause::Incarnation`]); this field keeps the pinned store, and the
    /// cause names both.
    pub responder: Option<StoreId>,
    /// The [`first`](crate::WriteGroup::first) position of the last local
    /// write group the far side acknowledged. Every local event at or below
    /// it, and every event of that group, has been acknowledged.
    ///
    /// For a single-event append it is the position `append` returned. A
    /// local [`SequencePosition`], meaningful only in the local store: it is
    /// the watermark SY-31 says is kept through the projection port, and it is
    /// never sent to the far side.
    pub confirmed: Option<SequencePosition>,
    /// How many local events, in write groups above
    /// [`confirmed`](Self::confirmed), are unacknowledged, as of the last
    /// step's read.
    ///
    /// Every local event counts, including ones this edge ingested from the far
    /// side, which a push hands back and the far side skips.
    pub pending: Pending,
    /// What the far side said it still holds past the last pull's token.
    /// Updated only once that pull's batch has landed locally.
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
    pub(super) const fn resuming(confirmed: Option<SequencePosition>) -> Self {
        Self {
            state: EdgeState::Running,
            responder: None,
            confirmed,
            pending: Pending::Unknown,
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
            && self.pending == Pending::Exactly(0)
            && self.waiting == Waiting::Nothing
    }
}

/// Whether an edge still exchanges.
///
/// `#[non_exhaustive]`, so a state added later does not break a `match`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EdgeState {
    /// The next step exchanges.
    Running,
    /// Something at the head of the edge can never land, and the edge does no
    /// more I/O: a failure of class [`Park`](crate::FailureClass::Park), or a
    /// [`Split`](crate::FailureClass::Split) the runner could not halve — one
    /// write group, which is never cut (SY-30), or a refusal outside the push,
    /// where the runner holds no batch to halve.
    ///
    /// The whole edge parks, not one event: the watermark and the pull token
    /// are both prefixes, so nothing behind the stuck group can move past it.
    /// An operator raises the limit, or accepts the loss, and builds a new
    /// edge.
    Parked(Failure),
    /// The edge halted for the cause named, and does no more I/O. An operator
    /// fixes the cause and builds a new edge. The local log is never touched
    /// on the way (SY-34).
    Stopped(StopCause),
}
