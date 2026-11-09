//! The far side answered as a different store: the incarnation check
//! (ADR-0104, SY-34).
//!
//! # What the check is
//!
//! Every [`Pulled`](crate::Pulled) and [`Ack`](crate::Ack) may name the store
//! incarnation that answered it. An edge pins the first one named, once that
//! answer has taken effect: a pull's batch ingested, a push acknowledged. A
//! later answer naming another stops the edge, before the answer is acted on:
//! a pulled batch from the new incarnation is never ingested, and an ack from it
//! never moves the watermark. A restored or re-minted far side (VT-6) holds a
//! history that has moved under the edge's token and watermark, so neither is
//! true of it, and the only safe move is to stop and say so.
//!
//! An answer that names no responder is no evidence either way. It neither pins
//! nor stops, because a peer with no store of its own has nothing truthful to
//! say (ADR-0104 §2).
//!
//! # What it never does
//!
//! It never resets, rewinds or wipes the local log. The equivalent check in the
//! library #244 audited wipes local data on a mismatch by default, and SY-34
//! forbids exactly that: the local log's events are facts. The remedy is an operator's: build a
//! new edge against the new incarnation, which re-pushes what it lacks and
//! pulls what it holds, and ingest's idempotence makes that converge.
//!
//! # Why a variant of its own, and not a class on a peer error
//!
//! The answer was well formed and the peer did not fail. The runner found the
//! answer came from somewhere else, so no adapter's [`Classify`](crate::Classify)
//! impl could have stated it. It reports class [`Stop`](FailureClass::Stop),
//! so a caller that only reads classes still halts.

use core::fmt;

use happenstance_core::StoreId;

use super::error::{Failure, Phase};
use crate::classify::FailureClass;

/// The far side answered as a store other than the one this edge has synced
/// with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct IncarnationChanged {
    /// The phase whose answer named the new store: [`Phase::Pull`] or
    /// [`Phase::Push`].
    pub phase: Phase,
    /// The store this edge pinned: the first responder it synced with, or the
    /// one [`Edge::with_responder`](super::Edge::with_responder) named.
    pub known: StoreId,
    /// The store the answer named instead.
    pub answered: StoreId,
}

impl IncarnationChanged {
    /// An answer in `phase` named `answered` where the edge knew `known`.
    #[must_use]
    pub const fn new(phase: Phase, known: StoreId, answered: StoreId) -> Self {
        Self {
            phase,
            known,
            answered,
        }
    }

    /// As a [`Failure`]: its phase, and class [`Stop`](FailureClass::Stop).
    #[must_use]
    pub const fn failure(&self) -> Failure {
        Failure::new(self.phase, FailureClass::Stop)
    }
}

impl fmt::Display for IncarnationChanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the far side answered the {} as store {}, not store {}: it was restored or re-minted",
            self.phase, self.answered, self.known
        )
    }
}

/// Why an edge stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum StopCause {
    /// A failure of class [`Stop`](FailureClass::Stop): authorisation or
    /// configuration.
    Failure(Failure),
    /// The far side answered as a different store incarnation.
    Incarnation(IncarnationChanged),
}

impl StopCause {
    /// As a [`Failure`]: the phase, and class [`Stop`](FailureClass::Stop).
    #[must_use]
    pub const fn failure(&self) -> Failure {
        match self {
            Self::Failure(failure) => *failure,
            Self::Incarnation(change) => change.failure(),
        }
    }
}

impl fmt::Display for StopCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failure(failure) => failure.fmt(f),
            Self::Incarnation(change) => change.fmt(f),
        }
    }
}
