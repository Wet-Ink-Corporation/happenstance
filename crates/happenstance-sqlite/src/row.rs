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

use happenstance_core::Tags;

/// The byte that separates one encoded tag from the next.
const UNIT: u8 = 0x1f;

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
