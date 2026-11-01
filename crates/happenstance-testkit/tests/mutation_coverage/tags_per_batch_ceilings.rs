//! CF-40's tags-per-batch branch (VT-36, ADR-0100), and the seven wrong
//! implementations it rejects, beside the one store that gets it right.
//!
//! A module of its own for `metadata_ceilings`' reason: `mutants.rs` is already
//! the longest file in this binary. The eight are one check, a total tag count
//! across one append, got right once and wrong seven ways, one per assertion the
//! branch makes.
//!
//! * [`TagsPerBatchCeilingStore`] is the control: it refuses a total over the
//!   ceiling before writing, which shows the branch can be passed at all.
//! * Three answer the over batch wrongly: through `AppendError::Store`, under
//!   the per-event limit's name, or by comparing each event against the batch
//!   ceiling, so that the over batch lands.
//! * One refuses the ceiling itself, which only the at-ceiling anchor sees, and
//!   one refuses after writing, which only the leftovers check sees.
//! * Two enforce every ceiling they state correctly and state an incoherent
//!   set: a batch ceiling no batch can reach under the other two, and a batch
//!   ceiling below the per-event one.
//!
//! The first six state **only** `MAX_TAGS_PER_BATCH`, so a gate that forgot
//! the fifth constant would skip all six rather than run them, and
//! `mutants_fail_exactly_their_declared_rules` would report a declared failure
//! that never ran.

use happenstance_core::{
    AppendCondition, AppendError, Event, MIN_SUPPORTED_EVENTS_PER_BATCH,
    MIN_SUPPORTED_TAGS_PER_BATCH, MIN_SUPPORTED_TAGS_PER_EVENT, SequencePosition, SequencedEvent,
    StoreLimit,
};

use crate::correct::{self, LogError};
use crate::mutants::Defect;

/// The tags-per-batch ceiling the first six stores state.
///
/// The floor itself, and the tightest legal value (VT-36). It is clear of every
/// other rule's writes, the largest of which carry 128 tags in one append, so
/// each store is wrong only at the boundary this branch writes and fails that
/// rule alone.
pub(crate) const CEILING: usize = MIN_SUPPORTED_TAGS_PER_BATCH;

/// The total number of tags across `events`.
fn total_tags(events: &[Event]) -> usize {
    events.iter().map(|event| event.tags().len()).sum()
}

/// `ExceedsStoreLimit` naming `limit`, for `len`.
const fn refusal(limit: StoreLimit, len: usize) -> AppendError<LogError> {
    AppendError::ExceedsStoreLimit { limit, len }
}

/// The total is checked before anything is written, and refused by name.
///
/// The conformant shape: a store whose transaction is bounded by its row count
/// (a Durable Object resetting on CPU or storage time) counts the tag rows an
/// append will write and refuses beyond its declared number, as
/// `StoreLimit::TagsPerBatch`, before the first statement.
pub(crate) struct TagsPerBatchCeilingStore;

impl Defect for TagsPerBatchCeilingStore {
    const NAME: &'static str = "TagsPerBatchCeilingStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let total = total_tags(events);
        if total > CEILING {
            return Err(refusal(StoreLimit::TagsPerBatch, total));
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

/// The total is refused through the adapter's own error.
///
/// What an adapter had to do before `StoreLimit::TagsPerBatch` existed: the
/// wall is a transaction's row count, the refusal has no contract name, and
/// the only channel is `AppendError::Store`. A sync runner reading it cannot
/// tell "split this batch" from "retry", which is VT-25's complaint.
pub(crate) struct TagsPerBatchThroughStoreStore;

impl Defect for TagsPerBatchThroughStoreStore {
    const NAME: &'static str = "TagsPerBatchThroughStoreStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: the right check, through the channel VT-25 closed.
        if total_tags(events) > CEILING {
            return Err(AppendError::Store(LogError::TooManyTags));
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

/// The total is refused as `TagsPerEvent`.
///
/// The per-event check pasted to make the batch check, with its limit name
/// left alone. The refusal is distinguishable, and it names the wrong scope: a
/// runner reading `TagsPerEvent` parks an event that would land in a smaller
/// batch, where `TagsPerBatch` would have had it split.
pub(crate) struct MisnamedTagsPerBatchStore;

impl Defect for MisnamedTagsPerBatchStore {
    const NAME: &'static str = "MisnamedTagsPerBatchStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let total = total_tags(events);
        if total > CEILING {
            // THE DEFECT: the per-event limit's name on the batch's refusal.
            return Err(refusal(StoreLimit::TagsPerEvent, total));
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

/// Each event's tag count is compared with the batch ceiling, and none summed.
///
/// The per-event loop reused with the batch's number: `events.iter().any(|e|
/// e.tags().len() > CEILING)`. A batch whose tags are spread over several
/// events is never refused, and lands whole however many rows it writes.
pub(crate) struct PerEventTagsPerBatchStore;

impl Defect for PerEventTagsPerBatchStore {
    const NAME: &'static str = "PerEventTagsPerBatchStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: one event at a time, so no sum is ever over.
        if let Some(event) = events.iter().find(|event| event.tags().len() > CEILING) {
            return Err(refusal(StoreLimit::TagsPerBatch, event.tags().len()));
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

/// The total is checked `>=` where `>` was meant.
///
/// The ceiling is documented as the most an append carries and checked as the
/// first total refused. Every total above it is refused correctly, so only an
/// append of exactly the ceiling, the branch's at-ceiling anchor, tells the
/// two apart.
pub(crate) struct OffByOneTagsPerBatchStore;

impl Defect for OffByOneTagsPerBatchStore {
    const NAME: &'static str = "OffByOneTagsPerBatchStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let total = total_tags(events);
        // THE DEFECT: `>=`, so the stated ceiling itself is refused.
        if total >= CEILING {
            return Err(refusal(StoreLimit::TagsPerBatch, total));
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

/// The total is counted from the rows written, after they are written.
///
/// The count moved below the tag `INSERT`, where `changes()` hands it over for
/// free, with no transaction around the two. The answer is the right variant,
/// so the caller is told the batch was refused, and the batch is in the log.
/// Only the branch's leftovers check, which reads the log back, sees it.
pub(crate) struct RefuseAfterInsertTagsPerBatchStore;

impl Defect for RefuseAfterInsertTagsPerBatchStore {
    const NAME: &'static str = "RefuseAfterInsertTagsPerBatchStore";

    const MAX_TAGS_PER_BATCH: Option<usize> = Some(CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let position = correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )?;
        // THE DEFECT: the refusal is right, and it arrives after the write.
        let total = total_tags(events);
        if total > CEILING {
            return Err(refusal(StoreLimit::TagsPerBatch, total));
        }
        Ok(position)
    }
}

/// Refuses whichever of the three stated ceilings an append crosses first.
fn enforce_all<D: Defect>(
    stored: &mut Vec<SequencedEvent>,
    events: &[Event],
    condition: Option<&AppendCondition>,
    per_event: usize,
    per_batch: usize,
) -> Result<SequencePosition, AppendError<LogError>> {
    if let Some(ceiling) = D::MAX_EVENTS_PER_BATCH
        && events.len() > ceiling
    {
        return Err(refusal(StoreLimit::EventsPerBatch, events.len()));
    }
    if let Some(event) = events.iter().find(|event| event.tags().len() > per_event) {
        return Err(refusal(StoreLimit::TagsPerEvent, event.tags().len()));
    }
    let total = total_tags(events);
    if total > per_batch {
        return Err(refusal(StoreLimit::TagsPerBatch, total));
    }
    correct::commit_with(
        stored,
        events,
        condition,
        D::ALLOCATE,
        D::violation,
        D::sequence,
    )
}

/// A batch ceiling no batch can reach without crossing another ceiling first.
///
/// The ceilings are VT-24's and VT-22's floors and VT-36's, all three enforced
/// correctly: 128 events of at most 64 tags is 8,192 tags, so a batch of 8,193
/// is refused for its length or for one of its events before its total is ever
/// counted. The store copied the floors into its documentation as three
/// ceilings without multiplying them. The declaration says the store refuses a
/// total no caller can send, which is no refusal at all.
pub(crate) struct UnreachableTagsPerBatchStore;

impl Defect for UnreachableTagsPerBatchStore {
    const NAME: &'static str = "UnreachableTagsPerBatchStore";

    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(MIN_SUPPORTED_EVENTS_PER_BATCH);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(MIN_SUPPORTED_TAGS_PER_EVENT);
    // THE DEFECT: 128 x 64 is this number, so one tag more cannot be sent.
    const MAX_TAGS_PER_BATCH: Option<usize> = Some(MIN_SUPPORTED_TAGS_PER_BATCH);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        enforce_all::<Self>(
            stored,
            events,
            condition,
            MIN_SUPPORTED_TAGS_PER_EVENT,
            MIN_SUPPORTED_TAGS_PER_BATCH,
        )
    }
}

/// The per-event ceiling of [`TagsPerBatchBelowTagsPerEventStore`].
///
/// Above VT-22's floor and above every other rule's widest event, so that the
/// store passes the rest of the suite, and above [`NARROW_BATCH`], which is
/// the one thing wrong with it.
const WIDE_EVENT: usize = 16_384;

/// The batch ceiling of [`TagsPerBatchBelowTagsPerEventStore`], below its per-event one.
///
/// At VT-36's floor, so the store is wrong only in coherence: a batch ceiling
/// below the floor would be a second defect the rule does not name.
const NARROW_BATCH: usize = MIN_SUPPORTED_TAGS_PER_BATCH;

/// A batch ceiling below the per-event ceiling.
///
/// Both enforced correctly, and incoherent together: an event of 16,384 tags is
/// within its own ceiling and refused as a batch of one, `Split`. A sync
/// runner halves the batch until one event is left, and then has nothing left
/// to halve and a refusal that says it could. The two numbers came from two
/// places, the per-event one from the schema and the batch one from a
/// measurement, and nobody put them side by side.
pub(crate) struct TagsPerBatchBelowTagsPerEventStore;

impl Defect for TagsPerBatchBelowTagsPerEventStore {
    const NAME: &'static str = "TagsPerBatchBelowTagsPerEventStore";

    const MAX_TAGS_PER_EVENT: Option<usize> = Some(WIDE_EVENT);
    // THE DEFECT: below the per-event ceiling, so `Split` cannot terminate.
    const MAX_TAGS_PER_BATCH: Option<usize> = Some(NARROW_BATCH);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        enforce_all::<Self>(stored, events, condition, WIDE_EVENT, NARROW_BATCH)
    }
}
