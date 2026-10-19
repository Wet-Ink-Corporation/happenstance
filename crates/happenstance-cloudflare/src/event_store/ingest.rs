//! VT-10's fourth run: `happenstance-sync`'s ingest port, implemented for the
//! Durable Object over the row writer `append` already uses.

use happenstance_core::StoreId;
use happenstance_sync::{Classify, FailureClass, ReplicatedEvent};

use super::{CloudflareEventStoreError, MAX_SAFE_POSITION};
use crate::sql_storage::SqlError;

/// An ingested event's origin, in the shape this runtime binds exactly: parse,
/// don't validate (API-03).
///
/// `worker` binds an integer as `i as f64` (`worker-0.8.5/src/sql.rs:112-123`),
/// so an origin position or a recorded time past 2^53 − 1 would be stored as a
/// neighbour of itself. Building one of these is the check, so no statement is
/// ever issued for a value that cannot land exactly.
#[derive(Debug, Clone, Copy)]
struct StoredOrigin {
    /// The store that minted the event.
    store: StoreId,
    /// The position its origin gave it, as the integer column holds it.
    position: i64,
}

impl TryFrom<&ReplicatedEvent> for StoredOrigin {
    type Error = CloudflareEventStoreError;

    /// Borrows, because it reads two `Copy` fields and the recorded time.
    fn try_from(event: &ReplicatedEvent) -> Result<Self, Self::Error> {
        let origin = event.id.position();
        let position = i64::try_from(origin.get())
            .ok()
            .filter(|position| *position <= MAX_SAFE_POSITION)
            .ok_or(CloudflareEventStoreError::OriginPositionOutOfRange { position: origin })?;
        let millis = event.recorded_at.as_millis();
        if !(-MAX_SAFE_POSITION..=MAX_SAFE_POSITION).contains(&millis) {
            return Err(CloudflareEventStoreError::RecordedAtOutOfRange {
                recorded_at: event.recorded_at,
            });
        }
        Ok(Self {
            store: event.id.store(),
            position,
        })
    }
}

/// SY-36's class for each way this adapter fails.
///
/// Exhaustive over this crate's own enums, with no wildcard, so a new variant
/// does not compile until somebody decides its class. `SqlError` is
/// `#[non_exhaustive]`, but it is this crate's, so an in-crate match needs no
/// wildcard either. Coherence allows the impl: a foreign trait on a local type.
impl Classify for CloudflareEventStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Sql(sql) => match sql {
                // A re-entrant statement was in flight, or a cursor outlived the
                // statement after it: the next attempt finds the storage free
                // and takes a fresh cursor.
                SqlError::AlreadyBorrowed | SqlError::CursorInvalidated => FailureClass::Transient,
                // An object's storage cap does not empty itself, and a thrown
                // storage error fails the same way again. No message is parsed
                // to tell a transient throw apart (Decision D7, until a workerd
                // run shows which throws recur).
                SqlError::StorageLimitExceeded | SqlError::Thrown(_) => FailureClass::Stop,
            },
            // Past what a binding carries exactly, so it can never land here,
            // and the rest of the batch can: VT-25's "park it".
            Self::OriginPositionOutOfRange { .. } | Self::RecordedAtOutOfRange { .. } => {
                FailureClass::Park
            }
            // Append's: its rows are not events (ES-18), so the failure that
            // stopped it is the one that matters.
            Self::PartialBatch { cause, .. } => cause.class(),
            // A corrupt store or a missing schema, which fails the same way
            // again.
            Self::RowShape { .. }
            | Self::ColumnType { .. }
            | Self::StoredEventType(_)
            | Self::StoredTag(_)
            | Self::CorruptTags
            | Self::UnstampedEvent { .. }
            | Self::StoredPosition { .. } => FailureClass::Stop,
        }
    }
}

// The whole file is `cfg(test)` already, through its declaration in
// `event_store.rs`. Said again here so the module reads as test code where it
// stands.
#[cfg(test)]
mod tests;
