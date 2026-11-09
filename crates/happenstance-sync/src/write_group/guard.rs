//! How a store keeps an ingested group's guard: one format byte, then postcard.
//!
//! The guard is evidence (SY-6) that a relay must forward with the group, so a
//! store that ingests one has to hold it until a pull reads the group back.
//! These two functions are the one spelling of the bytes, so four adapters
//! cannot write four formats. Only adapters call them, which is why they stay
//! module-qualified rather than re-exported at the crate root.
//!
//! The format byte is this module's own, independent of
//! [`wire::FORMAT_VERSION`](crate::wire::FORMAT_VERSION): a wire bump never
//! strands a guard already stored. Decoding re-validates, because `Query` and
//! `Tags` re-validate on deserialisation, so a corrupt row cannot bring an
//! invalid condition into memory.

use alloc::vec::Vec;

use happenstance_core::AppendCondition;

use crate::wire::PostcardError;

/// The first byte of every stored guard.
pub const STORED_GUARD_FORMAT: u8 = 1;

/// The bytes a store keeps for `guard`.
///
/// # Errors
///
/// [`GuardCodecError::Encode`] if postcard refuses the value.
pub fn encode_guard(guard: &AppendCondition) -> Result<Vec<u8>, GuardCodecError> {
    let mut stored = Vec::from([STORED_GUARD_FORMAT]);
    let body = postcard::to_stdvec(guard)
        .map_err(|cause| GuardCodecError::Encode(PostcardError(cause)))?;
    stored.extend(body);
    Ok(stored)
}

/// The guard `stored` holds, as [`encode_guard`] wrote it.
///
/// # Errors
///
/// [`GuardCodecError::Empty`] for no bytes at all,
/// [`GuardCodecError::UnknownFormat`] for a format byte this build does not
/// read, [`GuardCodecError::Decode`] for a body that is not a valid
/// condition, and [`GuardCodecError::TrailingBytes`] for a valid one with
/// bytes after it.
pub fn decode_guard(stored: &[u8]) -> Result<AppendCondition, GuardCodecError> {
    let Some((&format, body)) = stored.split_first() else {
        return Err(GuardCodecError::Empty);
    };
    if format != STORED_GUARD_FORMAT {
        return Err(GuardCodecError::UnknownFormat { found: format });
    }
    let (guard, rest) = postcard::take_from_bytes(body)
        .map_err(|cause| GuardCodecError::Decode(PostcardError(cause)))?;
    if !rest.is_empty() {
        return Err(GuardCodecError::TrailingBytes { extra: rest.len() });
    }
    Ok(guard)
}

/// Why a guard could not be stored or read back.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum GuardCodecError {
    /// The stored value has no bytes, not even the format byte.
    #[error("a stored guard is empty")]
    Empty,
    /// The stored value is in a format this build does not read.
    #[error(
        "a stored guard is in format {found}, and this build reads format {STORED_GUARD_FORMAT}"
    )]
    #[non_exhaustive]
    UnknownFormat {
        /// The format byte found.
        found: u8,
    },
    /// The guard could not be encoded.
    #[error("a guard could not be encoded")]
    Encode(#[source] PostcardError),
    /// The stored body is not a condition this build reads.
    #[error("a stored guard could not be decoded")]
    Decode(#[source] PostcardError),
    /// The stored body holds a condition and then bytes no condition owns.
    #[error("a stored guard carries {extra} bytes past its condition")]
    #[non_exhaustive]
    TrailingBytes {
        /// How many bytes followed the condition.
        extra: usize,
    },
}

#[cfg(test)]
mod tests {
    use happenstance_core::{AppendCondition, Query, QueryItem, SequencePosition, Tags};

    use super::{GuardCodecError, decode_guard, encode_guard};

    fn query(kind: &str, course: &str) -> Query {
        Query::from_item(
            QueryItem::new(
                [kind],
                Tags::from_pairs([("course", course)]).expect("valid tags"),
            )
            .expect("a valid item"),
        )
    }

    #[test]
    fn a_single_guard_round_trips() {
        let guard = AppendCondition::new(query("CourseDefined", "c1"));

        let stored = encode_guard(&guard).expect("a condition encodes");

        assert_eq!(decode_guard(&stored), Ok(guard));
    }

    #[test]
    fn a_multi_guard_condition_with_boundaries_round_trips() {
        let guard = AppendCondition::new(query("CourseDefined", "c1"))
            .after_every_guard_opt(SequencePosition::new(9))
            .and_guard(query("StudentSubscribed", "c1"), SequencePosition::new(12));

        let stored = encode_guard(&guard).expect("a condition encodes");

        assert_eq!(decode_guard(&stored), Ok(guard));
    }

    #[test]
    fn no_bytes_is_empty() {
        assert_eq!(decode_guard(&[]), Err(GuardCodecError::Empty));
    }

    #[test]
    fn an_unknown_format_byte_is_named() {
        let mut stored =
            encode_guard(&AppendCondition::new(query("CourseDefined", "c1"))).expect("encodes");
        if let Some(format) = stored.first_mut() {
            *format = 2;
        }

        assert_eq!(
            decode_guard(&stored),
            Err(GuardCodecError::UnknownFormat { found: 2 })
        );
    }

    #[test]
    fn bytes_past_the_condition_are_refused() {
        let mut stored =
            encode_guard(&AppendCondition::new(query("CourseDefined", "c1"))).expect("encodes");
        stored.push(0);

        assert_eq!(
            decode_guard(&stored),
            Err(GuardCodecError::TrailingBytes { extra: 1 })
        );
    }

    #[test]
    fn a_truncated_body_does_not_decode() {
        let stored =
            encode_guard(&AppendCondition::new(query("CourseDefined", "c1"))).expect("encodes");
        let truncated = stored
            .get(..stored.len() - 3)
            .expect("longer than three bytes");

        assert!(matches!(
            decode_guard(truncated),
            Err(GuardCodecError::Decode(_))
        ));
    }
}
