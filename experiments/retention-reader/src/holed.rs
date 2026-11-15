//! CF-27's instrument, with the two sync traits it cannot supply itself.
//!
//! [`WithholdingStore`] lives in the testkit, and [`IngestStore`] and
//! [`WriteGroupStore`] live in `happenstance-sync`. The testkit cannot depend
//! on sync, and a third crate cannot implement a foreign trait for a foreign
//! type (the orphan rule, E0117), so [`Holed`] wraps the instrument in a type
//! local to this crate.
//!
//! The bare flavours are implemented, because the instrument implements only
//! the bare `EventStore`, and the edge and the store peer bind the bare
//! flavours. Every scenario drives with `block_on`, so nothing needs `Send`.

use core::num::NonZeroUsize;

use happenstance_core::{EventStore, SequencePosition, StoreId};
use happenstance_sync::write_group::GroupOrderError;
use happenstance_sync::{
    GroupPage, IngestError, IngestGroup, IngestStore, Ingested, Waiting, Watermark, WriteGroup,
    WriteGroupStore,
};
use happenstance_testkit::WithholdingStore;

use crate::shared::Shared;

/// A store that physically holds everything and, at the port, answers as if
/// it had forgotten what its instrument withholds.
///
/// Cloning gives a second handle onto the same log and the same withheld set.
#[derive(Debug, Clone)]
pub struct Holed {
    /// The port view: what a pruned store would answer.
    view: WithholdingStore<Shared>,
    /// The same log, unfiltered: what the store physically holds.
    inner: Shared,
}

impl Holed {
    /// Wraps `inner`, withholding nothing yet.
    #[must_use]
    pub fn new(inner: Shared) -> Self {
        Self {
            // A second handle onto the same log, not a copy (`Shared` is an `Arc`).
            view: WithholdingStore::new(inner.clone()),
            inner,
        }
    }

    /// The port view. Reads and withholding go through here.
    #[must_use]
    pub const fn view(&self) -> &WithholdingStore<Shared> {
        &self.view
    }

    /// The physical log, for scenario setup only.
    #[must_use]
    pub const fn inner(&self) -> &Shared {
        &self.inner
    }
}

/// Write groups as the port view holds them: withheld events are dropped from
/// their groups, and a group left empty is dropped whole.
///
/// A partially withheld group keeps its original `first`, where a physically
/// pruned store would name it by its lowest remaining row. Only the far side's
/// resume token ever sees `first`, and pagination stays monotone, so the
/// difference is recorded rather than modelled.
impl WriteGroupStore for Holed {
    type Error = GroupOrderError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        let mut cursor = after;
        loop {
            let page = WriteGroupStore::read_groups(&self.inner, cursor, max_events).await?;
            let last_first = page.groups.last().map(WriteGroup::first);
            let mut kept = Vec::with_capacity(page.groups.len());
            for group in &page.groups {
                if let Some(held) = self.retained(group).await {
                    kept.push(held);
                }
            }
            // A page made only of withheld groups must not end a pull early:
            // read on past it. `first` strictly increases along a chain of
            // pages, so this ends.
            match last_first {
                Some(last) if kept.is_empty() && page.waiting != Waiting::Nothing => {
                    cursor = Some(last);
                }
                _ => return Ok(GroupPage::new(kept, page.waiting)),
            }
        }
    }
}

impl Holed {
    /// `group` without its withheld events, or `None` if all are withheld.
    async fn retained(&self, group: &WriteGroup) -> Option<WriteGroup> {
        let mut kept: Option<WriteGroup> = None;
        for event in group.events() {
            // Membership through the instrument is withheld-aware, and every
            // event in an inner group is physically held, so `false` means
            // withheld. `MemoryStoreError` is uninhabited.
            let held = EventStore::contains_event_id(&self.view, event.id)
                .await
                .unwrap_or_else(|never| match never {});
            if !held {
                continue;
            }
            // The page owns its events: each retained one is copied once, and
            // its payload is a cheap `Bytes` handle.
            match kept.as_mut() {
                Some(group) => group.push(event.clone()),
                None => {
                    kept = Some(WriteGroup::new(
                        group.first(),
                        group.guard().cloned(),
                        event.clone(),
                    ));
                }
            }
        }
        kept
    }
}

/// A pure delegation to the physical log. This is the instrument's stated
/// fidelity gap ("What it cannot model": ingest deduplication), and the
/// instrument controls in `tests/it/r2_ingest_destination.rs` show it is
/// material.
impl IngestStore for Holed {
    type Error = <Shared as IngestStore>::Error;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        IngestStore::store_id(&self.inner).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        IngestStore::ingest(&self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        IngestStore::watermark(&self.inner).await
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;

    use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencePosition, collect};
    use happenstance_sync::{Waiting, WriteGroupStore};
    use happenstance_testkit::block_on;

    use super::Holed;
    use crate::shared::Shared;

    type Outcome = Result<(), Box<dyn core::error::Error>>;

    fn event(data: &'static str) -> Result<Event, Box<dyn core::error::Error>> {
        Ok(Event::new("Written", data.as_bytes())?)
    }

    /// Every page from the start, chained on `first`, as a store peer reads.
    async fn chain(
        store: &Holed,
        max: NonZeroUsize,
    ) -> Result<Vec<(SequencePosition, Vec<Vec<u8>>)>, Box<dyn core::error::Error>> {
        let mut seen = Vec::new();
        let mut after = None;
        // Bounded: twelve pages is more than any fixture here writes.
        for _ in 0..12 {
            let page = store.read_groups(after, max).await?;
            for group in &page.groups {
                let data = group.events().iter().map(|e| e.event.data().to_vec());
                seen.push((group.first(), data.collect()));
            }
            match page.groups.last() {
                Some(last) => after = Some(last.first()),
                None => return Ok(seen),
            }
            if page.waiting == Waiting::Nothing {
                return Ok(seen);
            }
        }
        Err("the page chain did not end".into())
    }

    #[test]
    fn holed_read_groups_hides_withheld_events_and_pages_past_all_withheld_pages() -> Outcome {
        let holed = Holed::new(Shared::new());
        block_on(async {
            let mut firsts = Vec::new();
            for data in ["1", "2", "3", "4", "5", "6"] {
                firsts.push(holed.inner().append(&[event(data)?], None).await?);
            }
            holed
                .view()
                .withhold(firsts.get(1..4).ok_or("six")?)
                .await?;

            let seen = chain(&holed, NonZeroUsize::MIN).await?;

            let kept: Vec<_> = [0, 4, 5].iter().filter_map(|&i| firsts.get(i)).collect();
            assert_eq!(
                seen.iter().map(|(first, _)| first).collect::<Vec<_>>(),
                kept
            );
            assert_eq!(
                seen.iter()
                    .map(|(_, data)| data.clone())
                    .collect::<Vec<_>>(),
                [
                    vec![b"1".to_vec()],
                    vec![b"5".to_vec()],
                    vec![b"6".to_vec()]
                ]
            );
            Ok(())
        })
    }

    #[test]
    fn holed_read_groups_cuts_a_partially_withheld_group_keeping_first() -> Outcome {
        let holed = Holed::new(Shared::new());
        block_on(async {
            // `append` answers the position of the last event it wrote.
            let second = holed
                .inner()
                .append(&[event("a")?, event("b")?], None)
                .await?;
            let all = collect(holed.inner().read(&Query::all(), ReadOptions::new())).await?;
            let first = all.first().ok_or("two were appended")?.position;
            holed.view().withhold(&[first]).await?;

            let seen = chain(&holed, NonZeroUsize::MIN).await?;

            assert_eq!(seen, [(first, vec![b"b".to_vec()])]);
            // The port view agrees: only the second event is held.
            assert_eq!(EventStore::head(holed.view()).await?, Some(second));
            Ok(())
        })
    }
}
