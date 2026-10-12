//! CF-40's metadata branch (ADR-0043), and the three wrong implementations it
//! rejects.
//!
//! A module of its own rather than three more entries in `mutants.rs`, which is
//! already the longest file in this binary. The three are one column with the
//! engine configured three ways, and each is a move ADR-0043 names: before
//! `StoreLimit::MetadataLen` existed, a store that could not hold an event's
//! metadata could refuse through `AppendError::Store`, which VT-25 forbids, or
//! name `EventDataLen`, which is about a different field. The third does not
//! refuse at all.
//!
//! All three state the same ceiling, so that every half of the rule's metadata
//! branch is checked at the same boundary, and all three state **only** this
//! ceiling. That second choice is what makes them reject a gate that forgot the
//! fourth constant too: with the other three at `None`, a rule gated on three
//! constants would skip each of these stores rather than run, and
//! `mutants_fail_exactly_their_declared_rules` would report a declared failure
//! that never ran.

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    AppendCondition, AppendError, Event, SequencePosition, SequencedEvent, StoreLimit,
};

use crate::correct::{self, Allocate, LogError};
use crate::mutants::Defect;

/// The metadata ceiling all three stores state.
///
/// Small on purpose, and not a measurement: no metadata the rest of the suite
/// writes comes near it (the largest is `causation`, nine bytes), so each store
/// is wrong only at the boundary the rule's metadata branch writes and fails
/// that rule alone. ADR-0043 sets no floor, so no number here is too small to
/// be legal.
pub(crate) const CEILING: usize = 4096;

/// The metadata length of `event`, or 0 when it carries none.
fn metadata_len(event: &Event) -> usize {
    event.metadata().map_or(0, Bytes::len)
}

/// The metadata column has a ceiling, refused through the adapter's own error.
///
/// What every adapter had to do before ADR-0043: the `metadata` column is a
/// `BLOB` with a size check, or a KV value with a cap, and the only channel the
/// refusal had was `AppendError::Store`. A sync runner reading it cannot tell
/// "park this event" from "retry", which is VT-25's whole complaint.
pub(crate) struct MetadataCeilingStore;

impl Defect for MetadataCeilingStore {
    const NAME: &'static str = "MetadataCeilingStore";

    const MAX_METADATA_LEN: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: the right refusal, through the channel VT-25 closed.
        if events.iter().any(|event| metadata_len(event) > CEILING) {
            return Err(AppendError::Store(LogError::MetadataTooLarge));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The metadata ceiling is refused as `EventDataLen`.
///
/// The move `happenstance-cloudflare` was one variant away from making: a
/// ceiling check written as `data.len() + metadata.len() > LIMIT`, or a second
/// check pasted from the first with the limit name left alone. The refusal is
/// distinguishable, so it passes for VT-25's purposes, and it names the wrong
/// field: a caller trimming `data` to fit is trimming the field that was not
/// too long. ADR-0043 calls it "a lie about which field failed".
pub(crate) struct MisnamedMetadataCeilingStore;

impl Defect for MisnamedMetadataCeilingStore {
    const NAME: &'static str = "MisnamedMetadataCeilingStore";

    const MAX_METADATA_LEN: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if let Some(event) = events.iter().find(|event| metadata_len(event) > CEILING) {
            // THE DEFECT: the only variant there was, before `MetadataLen`.
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                len: metadata_len(event),
            });
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The metadata column has the same ceiling, and the engine truncates.
///
/// [`MetadataCeilingStore`]'s column under `MySQL`'s non-strict `sql_mode`, the
/// pairing `TruncatingPayloadStore` makes one column over: over-length metadata
/// is stored clipped, a warning goes to a log nobody reads, and the caller is
/// told the whole write landed. Metadata is where a correlation or causation
/// header lives, so the clipped value is a trace that no longer joins.
pub(crate) struct TruncatingMetadataStore;

impl Defect for TruncatingMetadataStore {
    const NAME: &'static str = "TruncatingMetadataStore";

    const MAX_METADATA_LEN: Option<usize> = Some(CEILING);

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        // Cloned because `correct::sequence` stores owned events; an `Event`'s
        // payloads are `Bytes`, so each clone is a refcount bump, not a copy.
        let clipped: Vec<Event> = events
            .iter()
            .map(|event| match event.metadata() {
                // THE DEFECT: what fits is stored, and nothing says so.
                Some(metadata) if metadata.len() > CEILING => {
                    event.clone().with_metadata(metadata.slice(..CEILING))
                }
                _ => event.clone(),
            })
            .collect();
        correct::sequence(&clipped, head, allocate)
    }
}
