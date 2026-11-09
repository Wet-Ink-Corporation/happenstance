//! CF-40's budget unit (#143), and the wrong implementation it rejects.
//!
//! The question phase 13 settled: whether a store's byte ceilings are one per
//! field, or one shared budget for `data` plus `metadata`, the sum
//! `happenstance-sync`'s `ReplicatedEvent::payload_len` computes. The answer is
//! the field: each stated ceiling bounds its own field and holds with the other
//! field at its own ceiling, so one event at both is accepted. `payload_len` is
//! the replication port's unit, and SY-18's rule holds a peer's single budget to
//! both of its store's field ceilings.
//!
//! The store here states both ceilings and enforces the sum. It refuses every
//! over-ceiling value under the right variant and accepts each field at its
//! ceiling alone, so it passed every branch the rule had before #143. Only an
//! event carrying both ceilings at once tells it apart.

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    AppendCondition, AppendError, Event, SequencePosition, SequencedEvent, StoreLimit,
};

use crate::correct::{self, LogError};
use crate::mutants::Defect;

/// The payload ceiling the store states: VT-21's floor, the least a store may
/// state, and far above the metadata ceiling, so an event with a few bytes of
/// `data` and metadata at its own ceiling fits the shared sum and only the
/// at-once event does not.
const DATA_CEILING: usize = 65_536;

/// The metadata ceiling the store states.
const METADATA_CEILING: usize = 4096;

/// The metadata length of `event`, or 0 when it carries none.
fn metadata_len(event: &Event) -> usize {
    event.metadata().map_or(0, Bytes::len)
}

/// Both ceilings stated, and `data + metadata` held to the payload's.
///
/// The adapter that read ADR-0043's "roughly two-to-one in the shared sum's
/// favour" as the answer: one row-sized budget, checked as
/// `data.len() + metadata.len() > MAX_EVENT_DATA_LEN`, and a metadata ceiling
/// stated beside it because the variant exists. A caller who sizes an event to
/// the two ceilings the fixture states is refused, and trimming the field the
/// refusal names does not make it fit.
pub(crate) struct SharedPayloadBudgetStore;

impl Defect for SharedPayloadBudgetStore {
    const NAME: &'static str = "SharedPayloadBudgetStore";

    const MAX_EVENT_DATA_LEN: Option<usize> = Some(DATA_CEILING);
    const MAX_METADATA_LEN: Option<usize> = Some(METADATA_CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        for event in events {
            let metadata = metadata_len(event);
            if metadata > METADATA_CEILING {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::MetadataLen,
                    len: metadata,
                });
            }
            // THE DEFECT: the payload ceiling is a budget the two fields share.
            let payload = event.data().len().saturating_add(metadata);
            if payload > DATA_CEILING {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    len: payload,
                });
            }
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
