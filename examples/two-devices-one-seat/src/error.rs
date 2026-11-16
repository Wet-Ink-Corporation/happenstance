//! What a run of the story can fail with.

use core::convert::Infallible;
use std::path::PathBuf;

use happenstance::{CodecError, CommandError, InvalidQuery, InvalidTag, ProjectionError};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use happenstance_sqlite::projection_store::SqliteProjectionStoreError;
use happenstance_sync::RemoteSyncPeer;
use happenstance_sync::runner::{Failure, StepFailure};

use crate::domain::{Device, Place, Refusal};
use crate::rule::RuleError;
use crate::uplink::{CentralError, Uplink};

/// What one step failure looks like for a device: its peer is the uplink to
/// central, and its own log is a SQLite file. The same type whichever rule
/// central mounts, since every rule's error is a [`RuleError`].
pub type DeviceStep = StepFailure<RemoteSyncPeer<Uplink>, SqliteEventStore>;

/// What a run of the story can fail with. Every variant names the place or
/// file it concerns.
///
/// The library errors it carries are boxed where they are large, so every
/// `Result` in this crate stays a pointer wide on its error side; the box
/// costs one allocation on a path that is already failing.
#[derive(Debug, thiserror::Error)]
pub enum StoryError {
    /// The show's name is not a valid tag value.
    #[error("the show's name")]
    Show(#[source] InvalidTag),
    /// A file's event store did not open.
    #[error("opening the event store in {file}")]
    Open {
        /// The file.
        file: PathBuf,
        /// The adapter's error.
        #[source]
        source: SqliteEventStoreError,
    },
    /// A file's projection store did not open.
    #[error("opening the projection store in {file}")]
    OpenModel {
        /// The file.
        file: PathBuf,
        /// The adapter's error.
        #[source]
        source: SqliteProjectionStoreError,
    },
    /// The application's own read-model table could not be made or read.
    #[error("the read model in {file}")]
    ReadModel {
        /// The file.
        file: PathBuf,
        /// The driver's error.
        #[source]
        source: rusqlite::Error,
    },
    /// A sale did not commit: refused, or the store failed.
    #[error("{place} could not sell")]
    Sell {
        /// Where the sale was attempted.
        place: Place,
        /// Why, the refusal included.
        #[source]
        source: Box<CommandError<SqliteEventStoreError, Refusal>>,
    },
    /// A sale's decision committed and appended nothing. `sell_seat` always
    /// decides one event or refuses, so this is a broken decision, reported
    /// rather than assumed away.
    #[error("{place} decided a sale and appended nothing")]
    NothingSold {
        /// Where.
        place: Place,
    },
    /// The show could not be scheduled.
    #[error("central could not schedule the show")]
    Schedule(#[source] Box<CommandError<CentralError, Infallible>>),
    /// Central's rule could not judge what it was asked about.
    #[error("central could not keep its rule")]
    Adjudicate(#[source] RuleError),
    /// A sync step failed.
    #[error("{device} could not sync")]
    Sync {
        /// The device.
        device: Device,
        /// The step's failure, with its phase and class.
        #[source]
        source: Box<DeviceStep>,
    },
    /// A sync took every step it was allowed and still was not synced.
    #[error("{device} was not synced after {steps} steps")]
    NotSynced {
        /// The device.
        device: Device,
        /// The steps it took.
        steps: usize,
    },
    /// The show's query could not be derived.
    #[error("the show's query")]
    Query(#[from] InvalidQuery),
    /// A log could not be read.
    #[error("reading the log at {place}")]
    Read {
        /// Where.
        place: Place,
        /// The adapter's error.
        #[source]
        source: SqliteEventStoreError,
    },
    /// An event in a log did not decode.
    #[error("decoding the log at {place}")]
    Decode {
        /// Where.
        place: Place,
        /// The codec's error.
        #[source]
        source: CodecError,
    },
    /// A read model could not be brought up to date.
    #[error("projecting at {place}")]
    Project {
        /// Where.
        place: Place,
        /// The runner's error.
        #[source]
        source: Box<
            ProjectionError<
                SqliteEventStoreError,
                SqliteProjectionStoreError,
                SqliteProjectionStoreError,
            >,
        >,
    },
    /// Something the story claims did not happen: the story stops rather than
    /// print a transcript that is not true.
    #[error("the story's claim failed: {claim}")]
    Contradicted {
        /// The claim, as the transcript would have made it.
        claim: &'static str,
    },
    /// The transcript could not be written.
    #[error("writing the story")]
    Write(#[from] std::io::Error),
}

impl StoryError {
    /// The decision's own refusal, if a sale was refused.
    #[must_use]
    pub fn refusal(&self) -> Option<&Refusal> {
        match self {
            Self::Sell { source, .. } => match source.as_ref() {
                CommandError::Refused(refusal) => Some(refusal),
                _ => None,
            },
            _ => None,
        }
    }

    /// The phase and class of a failed sync step, if this is one.
    #[must_use]
    pub fn failure(&self) -> Option<Failure> {
        match self {
            Self::Sync { source, .. } => Some(source.failure()),
            _ => None,
        }
    }
}
