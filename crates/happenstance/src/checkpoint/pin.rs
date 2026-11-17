//! The source pin: which event store a checkpoint's positions came from (PS-40).
//!
//! A position means something only inside the store that assigned it (ES-29,
//! SY-19), so a checkpoint resumed against another store skips that store's
//! events at or below it and applies the rest as though they followed. The
//! port stores a position per [`ProjectionId`] and nothing else, and it is
//! frozen (ADR-0063), so the store's identity rides in a second id beside the
//! main key, `name#digest`, as a position: a 63-bit mark of the [`StoreId`].

use core::fmt;

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, SequencePosition, StoreId,
};

use super::fnv1a64;
use crate::runner::{Progressed, ProjectionError, considered_through};

/// A 63-bit fingerprint of a [`StoreId`], as the runner records it.
///
/// It is 64-bit FNV-1a over the identity's sixteen bytes, shifted right by
/// one, so it is at most `i64::MAX` and every SQL adapter in this workspace
/// stores it exactly: SQLite and Neon saturate a position above `i64::MAX`
/// and Postgres refuses one. A hash of zero becomes one, because a position
/// is never zero. Two different stores share a mark with odds of about one
/// in 2^63.
///
/// The field is private and there is no constructor from a raw `u64`: a mark
/// is made from a `StoreId`, or read out of a
/// [`ProjectionError::SourceChanged`]. To ask whether a candidate store is the
/// one a refusal recorded, compare `SourceMark::of(candidate)` with it.
///
/// ```
/// use happenstance::{SourceMark, StoreId};
///
/// let id = StoreId::from_bytes([0xab; 16]);
/// let mark = SourceMark::of(id);
/// assert_eq!(mark.to_string(), "518cb382aeca0efa");
/// assert!(mark.get() <= i64::MAX.unsigned_abs());
/// assert_eq!(mark, SourceMark::of(id), "the same store, the same mark");
/// ```
///
/// # What is promised
///
/// The function is a promise, as [`checkpoint_id`](crate::checkpoint_id)'s
/// digest is: changing it changes every recorded mark, and every view would
/// refuse its own store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceMark(SequencePosition);

impl SourceMark {
    /// The mark of `id`. Total: every identity has one.
    #[must_use]
    pub fn of(id: StoreId) -> Self {
        let bits = fnv1a64(&id.to_bytes()) >> 1;
        Self(SequencePosition::new(bits).unwrap_or(SequencePosition::FIRST))
    }

    /// The mark as a number, in `1..=i64::MAX`.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl fmt::Display for SourceMark {
    /// Sixteen lowercase hex digits, zero-padded, as a checkpoint id's digest.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.get())
    }
}

/// The two ids one projection's run reads and commits under.
///
/// Derived together, by [`derive_pair`](super::derive_pair), so the main key
/// and the pin cannot follow different queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Keys {
    /// The checkpoint the read model's rows move with (PS-25).
    pub(crate) main: ProjectionId,
    /// The source pin, whose position is a [`SourceMark`] (PS-40).
    pub(crate) pin: ProjectionId,
}

/// Why binding a run to its source store stopped it.
///
/// Free of the event store's and the projection's error types, so both
/// runners share it and map it into their own [`ProjectionError`].
pub(crate) enum Bound<E> {
    /// Reading the pin or the main checkpoint failed.
    Read(E),
    /// The pin records another store.
    Changed {
        pin: ProjectionId,
        recorded: SourceMark,
        found: StoreId,
    },
    /// Opening the empty write set the pin commits with failed.
    Begin(E),
    /// Committing the pin failed.
    Record(CommitError<E>),
}

impl<W> Bound<W>
where
    W: core::error::Error + 'static,
{
    /// The runner's arm for this stop. Nothing was read from the event store
    /// and nothing applied, so the progress is the default.
    pub(crate) fn into_error<R, A>(self) -> ProjectionError<R, W, A>
    where
        R: core::error::Error + 'static,
        A: core::error::Error + 'static,
    {
        match self {
            Self::Read(source) => ProjectionError::Checkpoint(source),
            Self::Changed {
                pin,
                recorded,
                found,
            } => ProjectionError::SourceChanged {
                pin,
                recorded,
                found,
            },
            Self::Begin(source) => ProjectionError::Begin {
                progress: Progressed::default(),
                source,
            },
            Self::Record(source) => ProjectionError::RecordSource(source),
        }
    }
}

/// Binds a run to the store it reads, and returns the checkpoint to resume
/// from.
///
/// The pin is read first. If it holds `found`'s mark, the run resumes. If it
/// holds another, the run stops there, having written nothing and read no
/// further. If it holds nothing, the store is pinned with an empty commit
/// (PS-21) **before** anything is read from the event store: a fresh view,
/// and a checkpoint an earlier release wrote without a pin, which this
/// adopts (ADR-0109). Only then is the main checkpoint read.
///
/// Pinning first is the order that fails safe: a crash between the pin and
/// the first chunk leaves a pin over `NeverRun`, which resumes against the
/// same store and refuses another. The reverse would leave a checkpoint
/// without a pin, which the next run would adopt for whichever store it was
/// handed.
pub(crate) async fn bind_source<St: ProjectionStore>(
    models: &St,
    keys: &Keys,
    found: StoreId,
) -> Result<Checkpoint, Bound<St::Error>> {
    let mark = SourceMark::of(found);
    let pinned = models.checkpoint(&keys.pin).await.map_err(Bound::Read)?;
    match considered_through(pinned) {
        Some(recorded) if recorded == mark.0 => {}
        Some(recorded) => {
            return Err(Bound::Changed {
                // Owned, because the error outlives the borrow of `keys`.
                pin: keys.pin.clone(),
                recorded: SourceMark(recorded),
                found,
            });
        }
        None => {
            let empty = models.begin().await.map_err(Bound::Begin)?;
            models
                .commit(empty, &keys.pin, mark.0, Authority::Live)
                .await
                .map_err(Bound::Record)?;
        }
    }
    models.checkpoint(&keys.main).await.map_err(Bound::Read)
}
