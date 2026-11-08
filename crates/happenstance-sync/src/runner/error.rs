//! Why a step failed: the phase, and the class the failure stated.
//!
//! Its own file for the reason `ingest/error.rs` is one: the runner's control
//! flow reads better without the error's shape interleaved with it.

use core::fmt;

use crate::classify::{Classify, FailureClass};
use crate::ingest::{IngestError, IngestStore};
use crate::peer::SyncPeer;
use crate::write_group::WriteGroupStore;

/// The error [`Edge::step`](super::Edge::step) returns for a peer `P` and a
/// store `S`.
pub type StepFailure<P, S> =
    StepError<<P as SyncPeer>::Error, <S as WriteGroupStore>::Error, <S as IngestStore>::Error>;

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

/// Why [`Edge::step`](super::Edge::step) failed: the phase, and the failure it
/// met.
///
/// `P` is the peer's error, `R` the local store's group-read error and `I` its
/// ingest error. They are separate because a store may implement
/// [`WriteGroupStore`] and [`IngestStore`] with different error types. No bounds sit
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
