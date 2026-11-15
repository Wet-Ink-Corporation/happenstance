//! Three wrong instruments, each a plausible bug, each one overridden method.

use std::collections::BTreeSet;

use happenstance_core::{EventId, EventStore, MemoryStoreError, SequencePosition};

use super::accuracy::{Defect, Subject};

/// Reports the range from the lowest to the highest withheld position: the
/// floor, or range, thinking ES-39 rejects. A survivor between two holes falls
/// inside the range and is still readable.
pub(crate) struct RangeReport;

impl Defect for RangeReport {
    fn report(subject: &Subject<Self>) -> BTreeSet<SequencePosition> {
        let real = subject.real.withheld();
        match (real.first(), real.last()) {
            (Some(lowest), Some(highest)) => (lowest.get()..=highest.get())
                .filter_map(SequencePosition::new)
                .collect(),
            _ => BTreeSet::new(),
        }
    }
}

/// Reports only the most recent call's positions: "assign instead of extend".
pub(crate) struct LastCallReport;

impl Defect for LastCallReport {
    fn report(subject: &Subject<Self>) -> BTreeSet<SequencePosition> {
        subject
            .calls
            .borrow()
            .last()
            .map(|call| call.iter().copied().collect())
            .unwrap_or_default()
    }
}

/// Answers membership from the inner store: the decorator that forgot ES-41.
/// The suite cannot reach it: no rule asks `contains_event_id` of an event it
/// did not itself append, and only withheld history was appended before the rule.
pub(crate) struct LeakyMembership;

impl Defect for LeakyMembership {
    async fn contains(subject: &Subject<Self>, id: EventId) -> Result<bool, MemoryStoreError> {
        EventStore::contains_event_id(&subject.inner, id).await
    }
}
