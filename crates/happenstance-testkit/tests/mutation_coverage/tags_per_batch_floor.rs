//! VT-36's floor (#368), and the three wrong implementations
//! `store_accepts_the_guaranteed_minimum_tags_per_batch` rejects.
//!
//! The floor is VT-22's 64 tags on each of VT-24's 128 events, 8,192 tags in
//! one append. Neither floor's own rule writes it, so a store could meet each
//! alone and fail the product. The three stores here are wrong only at that
//! product, and each fails the floor rule alone:
//!
//! * [`TagRowParameterCeilingStore`] binds every tag row of an append into one
//!   statement and meets its driver's parameter limit at the floor.
//! * [`TagRowClampingStore`] is that store's fix applied wrongly: it clamps the
//!   tag rows to what one statement carries and answers `Ok`.
//! * [`TagsPerBatchBelowTheFloorStore`] declares a ceiling one tag under the
//!   floor and enforces it correctly, which is honest about the wrong number.
//!
//! A module of its own, declared from `mutants.rs` for `tags_per_batch_ceilings`'
//! reasons: no line of that file moves, and no `#[path]` is needed.

use happenstance_core::{
    AppendCondition, AppendError, Event, MIN_SUPPORTED_TAGS_PER_BATCH, SequencePosition,
    SequencedEvent, StoreLimit, Tags,
};

use crate::correct::{self, Allocate, LogError};
use crate::mutants::Defect;

/// `SQLITE_MAX_VARIABLE_NUMBER`'s default since SQLite 3.32.
const DRIVER_PARAMETERS: usize = 32_766;

/// The parameters one tag row binds: tag, position, event type and store.
const PARAMETERS_PER_TAG_ROW: usize = 4;

/// The most tag rows one statement of [`TagRowParameterCeilingStore`] carries.
///
/// 8,191: one under VT-36's floor, so the floor is the first batch it meets.
const ROWS_PER_STATEMENT: usize = DRIVER_PARAMETERS / PARAMETERS_PER_TAG_ROW;

/// The total number of tags across `events`.
fn total_tags(events: &[Event]) -> usize {
    events.iter().map(|event| event.tags().len()).sum()
}

/// One multi-row `INSERT` carries every tag row of an append.
///
/// The bulk insert everybody writes first: the tag rows of the whole batch,
/// four bound parameters a row, in one statement. It is correct for every
/// batch anybody looked at, and at the floor the driver refuses the statement.
/// The refusal arrives at write time through `AppendError::Store`, after the
/// caller has decided, which is VT-24's complaint met one level down.
pub(crate) struct TagRowParameterCeilingStore;

impl Defect for TagRowParameterCeilingStore {
    const NAME: &'static str = "TagRowParameterCeilingStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: one statement for every tag row, so the driver's limit
        // is the store's, and nobody declared it.
        if total_tags(events).saturating_mul(PARAMETERS_PER_TAG_ROW) > DRIVER_PARAMETERS {
            return Err(AppendError::Store(LogError::TooManyParameters));
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

/// The tag rows are clamped to what one statement carries, and the rest are
/// dropped.
///
/// The botched fix for [`TagRowParameterCeilingStore`]: `&rows[..8_191]` where
/// `rows.chunks(8_191)` was meant. The batch's first 8,191 tags, in write
/// order, are stored; the rest are not, and `Ok` comes back carrying a real
/// position, so nothing downstream has anything to notice. Only reading the
/// batch back whole sees it.
pub(crate) struct TagRowClampingStore;

impl Defect for TagRowClampingStore {
    const NAME: &'static str = "TagRowClampingStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let mut left = ROWS_PER_STATEMENT;
        let clamped: Vec<Event> = events
            .iter()
            .map(|event| {
                // THE DEFECT: the first statement's rows are the whole write.
                let here = event.tags().len().min(left);
                left -= here;
                // The store owns what it writes and is lent the batch, so each
                // kept event is a copy, as in `correct::sequence`.
                let kept: Tags = event.tags().iter().take(here).cloned().collect();
                event.clone().with_tags(kept)
            })
            .collect();
        correct::sequence(&clamped, head, allocate)
    }
}

/// The ceiling this store declares: one tag under VT-36's floor.
const BELOW_THE_FLOOR: usize = MIN_SUPPORTED_TAGS_PER_BATCH - 1;

/// A measured wall declared as the tags-per-batch ceiling, below the floor.
///
/// The adapter measured where its transaction gives out, wrote the number
/// down as `MAX_TAGS_PER_BATCH`, and enforces it correctly: refused before any
/// write, as `StoreLimit::TagsPerBatch`, so CF-40's ceiling branch passes it.
/// Nobody compared the number with the floor. The declaration is honest about
/// a store that does not meet VT-36, and declaring the smaller number is not
/// the fix.
pub(crate) struct TagsPerBatchBelowTheFloorStore;

impl Defect for TagsPerBatchBelowTheFloorStore {
    const NAME: &'static str = "TagsPerBatchBelowTheFloorStore";

    // THE DEFECT: a ceiling under the floor every store MUST clear.
    const MAX_TAGS_PER_BATCH: Option<usize> = Some(BELOW_THE_FLOOR);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let total = total_tags(events);
        if total > BELOW_THE_FLOOR {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::TagsPerBatch,
                len: total,
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
