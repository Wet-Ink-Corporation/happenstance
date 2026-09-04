//! One codec for a stored row, used in both directions.
//!
//! The encode half is what `append` writes and the decode half is what the read
//! path reads, and they are **one module rather than two spellings** on purpose:
//! a decode that disagrees with its encode by one byte produces a store that
//! passes `append_preserves_event_payload` and fails
//! `append_preserves_event_type_and_tags_byte_for_byte`, and the diagnosis costs
//! a day.
//!
//! # The tag encoding, and why the delimiter is safe
//!
//! [`Tags`] is canonically sorted, so a single delimited column is a *canonical*
//! encoding rather than an arbitrary one. The delimiter is `0x1F`, the ASCII
//! unit separator, and it is safe rather than merely convenient: the shared tag
//! validator rejects every character in Unicode general category `Cc`, and
//! `0x1F` is one of them — so no tag can contain the byte that separates tags. A
//! delimiter a value can contain is how a canonical encoding becomes a matching
//! bug.
//!
//! The form is `UNIT tag UNIT tag UNIT`, with a leading *and* trailing
//! delimiter. Both are load-bearing for anything matching on the column with
//! `instr`: without them, `course:c1` matches inside `course:c10`.

use happenstance_core::{Event, EventId, RecordedAt, SequencedEvent, StoreId, Tag, Tags};

use crate::event_store::{SqliteEventStoreError, position_from_row};

/// The byte that separates one encoded tag from the next.
const UNIT: u8 = 0x1f;

/// The columns a stored row is rebuilt from, in the order [`to_event`] reads
/// them.
///
/// Spelled once so the `SELECT` list and the decode cannot drift apart — which
/// is the same failure the one-codec rule above exists to prevent, one level
/// down.
pub(crate) const COLUMNS: &str = "position, event_type, data, metadata, tags, \
                                  origin_store, origin_position, recorded_at";

/// The canonical column value for a tag set.
///
/// An empty set encodes to the single delimiter rather than to nothing, so that
/// "no tags" and "a tag that is the empty string" could never collide — the
/// second is unconstructible, and the encoding keeps it that way by shape.
pub(crate) fn encode_tags(tags: &Tags) -> Vec<u8> {
    let mut out = Vec::with_capacity(tags.len() * 16 + 1);
    out.push(UNIT);
    for tag in tags {
        out.extend_from_slice(tag.as_str().as_bytes());
        out.push(UNIT);
    }
    out
}

/// Reads the canonical tag column back.
///
/// # Errors
///
/// Returns [`SqliteEventStoreError::StoredTag`] if a stored value no longer
/// validates as a [`Tag`], and [`SqliteEventStoreError::CorruptTags`] if the
/// column is not UTF-8. Both are reported rather than panicked, so a schema
/// mistake surfaces as a failing rule naming this store rather than as a crash
/// naming the suite.
pub(crate) fn decode_tags(raw: &[u8]) -> Result<Tags, SqliteEventStoreError> {
    let text = core::str::from_utf8(raw).map_err(|_| SqliteEventStoreError::CorruptTags)?;
    text.split(UNIT as char)
        .filter(|part| !part.is_empty())
        .map(|part| Tag::new(part).map_err(SqliteEventStoreError::StoredTag))
        .collect::<Result<Vec<Tag>, _>>()
        .map(|tags| tags.into_iter().collect())
}

/// Rebuilds a stored row as the contract's own type.
///
/// Three things here are decisions rather than plumbing:
///
/// * **`metadata` keeps `NULL` and an empty blob apart.** They are two values
///   the contract keeps apart, and coercing one into the other loses a state a
///   caller can observe.
/// * **The [`EventId`] is reconstructed from the *stored* origin pair**, never
///   from this store's own incarnation plus the row's own position. Those agree
///   for a locally appended event and disagree for every ingested one, so the
///   shortcut is correct exactly until replication exists.
/// * **`recorded_at` is returned as stored.** A read that stamps `now()` is the
///   defect `recorded_time_survives_a_reopen` exists to reject.
///
/// # Errors
///
/// Returns the stored-value errors above, [`SqliteEventStoreError::Sqlite`] if a
/// column cannot be read, [`SqliteEventStoreError::InvalidPosition`] if a stored
/// position is not representable, and
/// [`SqliteEventStoreError::UnstampedEvent`] if a row carries no identity.
pub(crate) fn to_event(row: &rusqlite::Row<'_>) -> Result<SequencedEvent, SqliteEventStoreError> {
    let position: i64 = row.get(0)?;
    let event_type: String = row.get(1)?;
    let data: Vec<u8> = row.get(2)?;
    let metadata: Option<Vec<u8>> = row.get(3)?;
    let tags: Vec<u8> = row.get(4)?;
    let origin_store: Option<Vec<u8>> = row.get(5)?;
    let origin_position: Option<i64> = row.get(6)?;
    let recorded_at: i64 = row.get(7)?;

    // Both positions go through the one decoder, and neither absolutises. See
    // `position_from_row` for why the absolutising spelling is a guard that
    // fires for one input in 2^64: this decode is where it would forge VT-11's
    // position uniqueness, and the origin decode below is where it would forge
    // VT-8's `EventId` uniqueness.
    let position = position_from_row(position)?;

    let (origin_store, origin_position) = origin_store
        .zip(origin_position)
        .ok_or(SqliteEventStoreError::UnstampedEvent { position })?;
    let origin_bytes: [u8; 16] = origin_store.as_slice().try_into().map_err(|_| {
        SqliteEventStoreError::MalformedIdentity {
            len: origin_store.len(),
        }
    })?;
    let origin = position_from_row(origin_position)?;

    let mut event = Event::new(event_type, data)
        .map_err(SqliteEventStoreError::StoredEventType)?
        .with_tags(decode_tags(&tags)?);
    if let Some(metadata) = metadata {
        event = event.with_metadata(metadata);
    }

    Ok(SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes(origin_bytes), origin),
        RecordedAt::from_millis(recorded_at),
        event,
    ))
}
