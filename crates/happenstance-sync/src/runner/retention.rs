//! The far side can no longer resume this edge: a retention gap (SY-32).
//!
//! # Why the runner does not compare token and floor
//!
//! [`PeerLimits::retention_floor`](crate::PeerLimits::retention_floor) is an
//! [`EventId`], and a resume token is opaque to the runner: a group index for
//! one peer, a local position for another, text for a third. An `EventId`'s
//! order is authorship order within one origin, not the far side's forwarding
//! order, so "token below floor" means nothing when the runner evaluates it.
//! Only the peer that minted the token and set the floor can compare them. So
//! the peer refuses the pull with class
//! [`Reseed`](crate::FailureClass::Reseed), the runner reads the class (SY-36),
//! and the floor is read from the peer's limits for the report.

use core::fmt;

use happenstance_core::EventId;

use super::error::{Failure, Phase};
use crate::classify::FailureClass;

/// The far side refused this edge's resume token because it has forgotten
/// history the token has not reached (SY-32).
///
/// `Copy`, as [`EventId`] and [`Phase`] are, so it travels by value inside
/// [`StopCause`](super::StopCause) and [`EdgeState`](super::EdgeState).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct RetentionGap {
    /// The phase whose failure stated [`Reseed`](FailureClass::Reseed):
    /// [`Phase::Pull`] for every peer this crate ships.
    pub phase: Phase,
    /// The far side's
    /// [`retention_floor`](crate::PeerLimits::retention_floor) when it
    /// refused: the newest event it has forgotten. `None` if it refused
    /// without declaring one, which is still a gap and is still reported.
    pub floor: Option<EventId>,
}

impl RetentionGap {
    /// A refusal in `phase` from a far side whose floor was `floor`.
    #[must_use]
    pub const fn new(phase: Phase, floor: Option<EventId>) -> Self {
        Self { phase, floor }
    }

    /// As a [`Failure`]: its phase, and class [`Reseed`](FailureClass::Reseed).
    #[must_use]
    pub const fn failure(&self) -> Failure {
        Failure::new(self.phase, FailureClass::Reseed)
    }
}

impl fmt::Display for RetentionGap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the far side cannot resume this edge from its token in the {} phase: ",
            self.phase
        )?;
        match self.floor {
            Some(floor) => write!(f, "it has forgotten history up to {floor}")?,
            None => f.write_str("it has forgotten history and named no floor")?,
        }
        f.write_str("; re-seed this side from a peer that holds it, or accept the gap")
    }
}
