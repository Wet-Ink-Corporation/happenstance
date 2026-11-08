//! [`MemoryIngestStore`]'s write groups: the oracle the write-group family is
//! proven against.

use core::num::NonZeroUsize;

use happenstance_core::{MemoryStoreError, SequencePosition};

use super::MemoryIngestStore;
use crate::identity::ReplicatedEvent;
use crate::peer::Waiting;
use crate::write_group::{GroupPage, GroupPageBuilder, SendWriteGroupStore};

/// Never fails. Writers are serialised under the lock, so a group's rows are
/// contiguous and a scan in position order meets each group whole.
impl SendWriteGroupStore for MemoryIngestStore {
    type Error = MemoryStoreError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        let state = self.read_guard();
        let mut page = GroupPageBuilder::new(max_events);
        let rows = state
            .log
            .iter()
            .zip(&state.group_of)
            .filter(|&(_, &first)| after.is_none_or(|after| first > after));
        // The page owns its events and guards, as `snapshot` does, because the
        // lock is released before the caller looks at them: each is copied
        // once, and an event's payload is a cheap `Bytes` handle.
        for (row, &first) in rows {
            let guard = if row.position == first {
                state.guards.get(&first).cloned()
            } else {
                None
            };
            let event = ReplicatedEvent::new(row.id, row.recorded_at, row.event.clone());
            if page.offer(first, guard, event).is_break() {
                return Ok(page.finish(Waiting::Unknown));
            }
        }
        Ok(page.finish(Waiting::Nothing))
    }
}

#[cfg(test)]
mod tests {
    //! No literal local position: every `first` compared is one the store
    //! assigned, read back from `append` or from the log.

    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventStore, Query, QueryItem, ReadOptions, RecordedAt,
        SequencePosition, StoreId, collect,
    };

    use super::super::MemoryIngestStore;
    use crate::{
        GroupPage, IngestGroup, IngestStore, ReplicatedEvent, Waiting, WriteGroup, WriteGroupStore,
    };

    const ORIGIN: StoreId = StoreId::from_bytes([0x63; 16]);

    fn event(data: &str) -> Event {
        Event::new("Written", data.as_bytes().to_vec()).expect("a valid event type")
    }

    fn foreign(position: u64, data: &str) -> ReplicatedEvent {
        ReplicatedEvent::new(
            EventId::new(
                ORIGIN,
                SequencePosition::new(position).expect("origin positions are non-zero"),
            ),
            RecordedAt::from_millis(1_700_000_000_000),
            event(data),
        )
    }

    fn guard() -> AppendCondition {
        AppendCondition::new(Query::from_item(
            QueryItem::of_types(["Written"]).expect("a valid item"),
        ))
    }

    fn max(events: usize) -> NonZeroUsize {
        NonZeroUsize::new(events).expect("a non-zero page size")
    }

    async fn page(
        store: &MemoryIngestStore,
        after: Option<SequencePosition>,
        events: usize,
    ) -> GroupPage {
        let Ok(page) = store.read_groups(after, max(events)).await;
        page
    }

    /// The payloads of each group, in order.
    fn shapes(page: &GroupPage) -> Vec<Vec<&[u8]>> {
        page.groups
            .iter()
            .map(|group| {
                group
                    .events()
                    .iter()
                    .map(|replicated| replicated.event.data().as_ref())
                    .collect()
            })
            .collect()
    }

    async fn append(store: &MemoryIngestStore, data: &[&str]) -> SequencePosition {
        let events: Vec<Event> = data.iter().map(|data| event(data)).collect();
        store
            .append(&events, None)
            .await
            .expect("an unconditional append lands")
    }

    async fn first_position(store: &MemoryIngestStore, data: &str) -> SequencePosition {
        let Ok(log) = collect(store.read(&Query::all(), ReadOptions::new())).await;
        log.iter()
            .find(|row| row.event.data().as_ref() == data.as_bytes())
            .map(|row| row.position)
            .expect("the event is in the log")
    }

    #[tokio::test]
    async fn an_append_of_three_reads_back_as_one_group_named_by_its_first_position() {
        let store = MemoryIngestStore::new();
        append(&store, &["a1", "a2", "a3"]).await;

        let read = page(&store, None, 16).await;

        assert_eq!(shapes(&read), [[&b"a1"[..], b"a2", b"a3"]]);
        let group: &WriteGroup = read.groups.first().expect("one group");
        assert_eq!(group.first(), first_position(&store, "a1").await);
        assert_eq!(group.guard(), None);
        assert_eq!(read.waiting, Waiting::Nothing);
    }

    #[tokio::test]
    async fn two_appends_read_back_as_two_groups() {
        let store = MemoryIngestStore::new();
        append(&store, &["a1", "a2"]).await;
        append(&store, &["b1"]).await;

        let read = page(&store, None, 16).await;

        assert_eq!(shapes(&read), [vec![&b"a1"[..], b"a2"], vec![b"b1"]]);
        let firsts: Vec<SequencePosition> = read.groups.iter().map(WriteGroup::first).collect();
        assert_eq!(
            firsts,
            [
                first_position(&store, "a1").await,
                first_position(&store, "b1").await
            ]
        );
    }

    #[tokio::test]
    async fn an_ingest_of_two_groups_reads_back_as_two_groups_with_their_guards() {
        let store = MemoryIngestStore::new();
        let guard = guard();
        let x = [foreign(4, "x1"), foreign(5, "x2")];
        let y = [foreign(6, "y1")];

        store
            .ingest(&[
                IngestGroup::new(&x, &[]).with_guard(Some(&guard)),
                IngestGroup::new(&y, &[]),
            ])
            .await
            .expect("the oracle ingests");
        let read = page(&store, None, 16).await;

        assert_eq!(shapes(&read), [vec![&b"x1"[..], b"x2"], vec![b"y1"]]);
        let guards: Vec<Option<&AppendCondition>> =
            read.groups.iter().map(WriteGroup::guard).collect();
        assert_eq!(guards, [Some(&guard), None]);
        let ids: Vec<EventId> = read
            .groups
            .iter()
            .flat_map(|group| group.events().iter().map(|event| event.id))
            .collect();
        assert_eq!(ids, [x[0].id, x[1].id, y[0].id], "origin identities kept");
    }

    #[tokio::test]
    async fn compensation_joins_its_ingested_group() {
        let store = MemoryIngestStore::new();
        let losing = [foreign(9, "losing")];
        let reversal = [event("reversal")];

        store
            .ingest(&[IngestGroup::new(&losing, &reversal)])
            .await
            .expect("the oracle ingests");
        let read = page(&store, None, 16).await;

        assert_eq!(shapes(&read), [[&b"losing"[..], b"reversal"]]);
    }

    #[tokio::test]
    async fn a_redelivered_group_writes_no_group_and_a_partial_one_writes_its_new_events() {
        let store = MemoryIngestStore::new();
        let first = [foreign(1, "p1"), foreign(2, "p2")];
        store
            .ingest(&[IngestGroup::new(&first, &[])])
            .await
            .expect("the oracle ingests");

        let again = [foreign(1, "p1"), foreign(2, "p2")];
        let partly = [foreign(2, "p2"), foreign(3, "p3")];
        store
            .ingest(&[
                IngestGroup::new(&again, &[]),
                IngestGroup::new(&partly, &[]),
            ])
            .await
            .expect("the oracle ingests");
        let read = page(&store, None, 16).await;

        assert_eq!(shapes(&read), [vec![&b"p1"[..], b"p2"], vec![b"p3"]]);
        let tail = read.groups.get(1).expect("two groups");
        assert_eq!(tail.first(), first_position(&store, "p3").await);
    }

    #[tokio::test]
    async fn a_page_stops_before_a_group_that_would_overflow_and_ends_with_nothing() {
        let store = MemoryIngestStore::new();
        append(&store, &["a1", "a2"]).await;
        append(&store, &["b1", "b2", "b3"]).await;
        append(&store, &["c1"]).await;

        let head = page(&store, None, 4).await;
        assert_eq!(shapes(&head), [[&b"a1"[..], b"a2"]]);
        assert_eq!(head.waiting, Waiting::Unknown);

        let after = head.groups.last().map(WriteGroup::first);
        let oversized = page(&store, after, 1).await;
        assert_eq!(
            shapes(&oversized),
            [[&b"b1"[..], b"b2", b"b3"]],
            "the first group comes whole however large"
        );
        assert_eq!(oversized.waiting, Waiting::Unknown);

        let after = oversized.groups.last().map(WriteGroup::first);
        let tail = page(&store, after, 4).await;
        assert_eq!(shapes(&tail), [[&b"c1"[..]]]);
        assert_eq!(tail.waiting, Waiting::Nothing);

        let after = tail.groups.last().map(WriteGroup::first);
        let empty = page(&store, after, 4).await;
        assert!(empty.groups.is_empty());
        assert_eq!(empty.waiting, Waiting::Nothing);
    }
}
