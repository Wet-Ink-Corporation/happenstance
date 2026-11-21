//! [`MemoryIngestStore`]'s removal outside the port, and the record it keeps
//! of what was removed (SY-40, ADR-0110).

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use happenstance_core::SequencePosition;

use super::MemoryIngestStore;

impl MemoryIngestStore {
    /// Removes the events this store holds at `positions`, outside every port,
    /// as an operator's `DELETE` on a real store would, records each removed
    /// identity, and returns how many it removed.
    ///
    /// The oracle's "outside the port", for the conformance suites' removal
    /// capability. A position it does not hold is ignored, and a position named
    /// twice is removed once. The position allocator stays where it was, so no
    /// position is assigned again (ES-38), and the incarnation is unchanged.
    ///
    /// The record is what ingest reads: an event whose identity it holds is
    /// skipped on re-delivery, counted in
    /// [`Ingested::skipped`](crate::Ingested::skipped), and never refused as
    /// an unheld own identity (SY-40). It is kept for the store's life.
    ///
    /// Removing part of a write group is outside what ES-38 describes. The
    /// rows left keep the group's name, and lose its guard if the removed row
    /// carried it: a group's guard travels on its first row.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_core::{Event, EventStore};
    /// use happenstance_sync::MemoryIngestStore;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
    /// let store = MemoryIngestStore::new();
    /// let first = store.append(&[Event::new("Noted", &b"a"[..])?], None).await?;
    ///
    /// assert_eq!(store.remove(&[first, first]), 1);
    /// let next = store.append(&[Event::new("Noted", &b"b"[..])?], None).await?;
    /// assert!(next > first, "a removed position is never assigned again");
    /// # Ok(())
    /// # }
    /// ```
    #[must_use = "the count says whether anything was held at the positions named"]
    pub fn remove(&self, positions: &[SequencePosition]) -> usize {
        let named: BTreeSet<SequencePosition> = positions.iter().copied().collect();
        let mut guard = self.write_guard();
        let state = &mut *guard;
        // One flag per row of `log`, computed once: every cut below reads it.
        let doomed: Vec<bool> = state
            .log
            .iter()
            .map(|row| named.contains(&row.position))
            .collect();
        let mut removed = 0;
        for (row, _) in state.log.iter().zip(&doomed).filter(|&(_, &gone)| gone) {
            state.held.remove(&row.id);
            state.removed.insert(row.id);
            removed += 1;
        }
        // `group_of` is index for index with `log`, so both are cut by the
        // same flags, in the same order.
        let mut flags = doomed.iter();
        state
            .group_of
            .retain(|_| !flags.next().copied().unwrap_or(false));
        let mut flags = doomed.iter();
        state
            .log
            .retain(|_| !flags.next().copied().unwrap_or(false));
        // A guard is read from its group's opening row only (`read_groups`),
        // so it is kept while that row is, and goes with it.
        let openers: BTreeSet<SequencePosition> = state
            .log
            .iter()
            .zip(&state.group_of)
            .filter(|&(row, &first)| row.position == first)
            .map(|(_, &first)| first)
            .collect();
        state.guards.retain(|first, _| openers.contains(first));
        removed
    }
}

#[cfg(test)]
mod tests {
    //! No literal local position: every local position compared is one the
    //! store assigned, read back from `append` or from the log. The literal
    //! positions are **origin** positions, which a peer supplies.

    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventStore, Query, QueryItem, ReadOptions, RecordedAt,
        SequencePosition, SequencedEvent, StoreId, collect,
    };

    use super::super::MemoryIngestStore;
    use crate::{IngestError, IngestGroup, IngestStore, ReplicatedEvent, WriteGroupStore};

    fn event(data: &str) -> Event {
        Event::new("Written", data.as_bytes().to_vec()).expect("a valid event type")
    }

    async fn log(store: &MemoryIngestStore) -> Vec<SequencedEvent> {
        let Ok(log) = collect(store.read(&Query::all(), ReadOptions::new())).await;
        log
    }

    async fn append(store: &MemoryIngestStore, data: &str) -> SequencePosition {
        store
            .append(&[event(data)], None)
            .await
            .expect("an unconditional append lands")
    }

    #[tokio::test]
    async fn remove_takes_the_named_events_out_and_never_reuses_their_positions() {
        let store = MemoryIngestStore::new();
        let first = append(&store, "a").await;
        let middle = append(&store, "b").await;
        let last = append(&store, "c").await;

        let removed = store.remove(&[middle]);
        let after = append(&store, "d").await;

        assert_eq!(removed, 1);
        let held: Vec<SequencePosition> =
            log(&store).await.iter().map(|row| row.position).collect();
        assert_eq!(held, [first, last, after]);
        assert!(after > last, "{after} was assigned at or below {last}");
        let Ok(gone) = store
            .contains_event_id(EventId::new(store.store_id(), middle))
            .await;
        assert!(!gone, "the removed event is still held");
    }

    #[tokio::test]
    async fn remove_ignores_a_position_it_does_not_hold_and_reports_the_count() {
        let store = MemoryIngestStore::new();
        let first = append(&store, "a").await;
        let second = append(&store, "b").await;
        let beyond = second.next().expect("a position past the head exists");
        let before = log(&store).await;

        let nothing = store.remove(&[beyond]);
        let twice = store.remove(&[first, first, beyond]);
        let again = store.remove(&[first]);

        assert_eq!((nothing, twice, again), (0, 1, 0));
        let after = log(&store).await;
        assert_eq!(
            after.as_slice(),
            before.get(1..).expect("two were appended")
        );
    }

    #[tokio::test]
    async fn a_removed_foreign_event_is_skipped_on_redelivery() {
        let store = MemoryIngestStore::new();
        let sent = [
            shipped(id(ORIGIN, 1), "x1"),
            shipped(id(ORIGIN, 2), "x2"),
            shipped(id(ORIGIN, 3), "x3"),
        ];
        let groups = one_group_each(&sent);
        store
            .ingest(&groups)
            .await
            .expect("the first delivery lands");
        let at = position_of(&store, sent[1].id).await;
        assert_eq!(store.remove(&[at]), 1);
        let before = log(&store).await;

        let again = store
            .ingest(&groups)
            .await
            .expect("a re-delivery is no error");

        assert_eq!(
            (again.appended, again.skipped, again.compensated),
            (0, 3, 0)
        );
        assert_eq!(log(&store).await, before);
        let Ok(held) = store.contains_event_id(sent[1].id).await;
        assert!(!held, "the removed event came back");
    }

    #[tokio::test]
    async fn a_removed_own_event_is_skipped_not_refused() {
        let store = MemoryIngestStore::new();
        let removed_at = append(&store, "l1").await;
        append(&store, "l2").await;
        let echo = [ReplicatedEvent::new(
            EventId::new(store.store_id(), removed_at),
            RecordedAt::from_millis(1),
            event("l1"),
        )];
        assert_eq!(store.remove(&[removed_at]), 1);
        let foreign = [shipped(id(ORIGIN, 9), "y")];

        let ingested = store
            .ingest(&[
                IngestGroup::new(&echo, &[]),
                IngestGroup::new(&foreign, &[]),
            ])
            .await;

        let ingested = ingested.expect("a removed own event is a skip, not a refusal");
        assert_eq!((ingested.appended, ingested.skipped), (1, 1));
        let Ok(echoed) = store.contains_event_id(echo[0].id).await;
        let Ok(landed) = store.contains_event_id(foreign[0].id).await;
        assert!(!echoed, "the removed own event came back");
        assert!(landed, "the foreign event beside it did not land");
    }

    /// I-3: the record is a set of identities, not "anything unheld is fine".
    #[tokio::test]
    async fn an_own_claim_never_held_is_refused_after_a_removal() {
        let store = MemoryIngestStore::new();
        let removed_at = append(&store, "l1").await;
        let head = append(&store, "l2").await;
        assert_eq!(store.remove(&[removed_at]), 1);
        let claim = EventId::new(
            store.store_id(),
            head.next()
                .and_then(SequencePosition::next)
                .and_then(SequencePosition::next)
                .expect("three positions past the head exist"),
        );
        let forged = [shipped(id(ORIGIN, 1), "z"), shipped(claim, "forged")];
        let before = log(&store).await;

        let refused = store.ingest(&[IngestGroup::new(&forged, &[])]).await;

        assert!(
            matches!(refused, Err(IngestError::UnheldOwnIdentity { id }) if id == claim),
            "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
        );
        assert_eq!(log(&store).await, before);
    }

    /// SY-11: a group of nothing but removed and held events writes no
    /// compensation, as a group of held events alone writes none.
    #[tokio::test]
    async fn a_group_of_removed_and_held_events_writes_no_compensation() {
        let store = MemoryIngestStore::new();
        let sent = [shipped(id(ORIGIN, 1), "x1"), shipped(id(ORIGIN, 2), "x2")];
        let answer = [event("compensated")];
        store
            .ingest(&[IngestGroup::new(&sent, &[])])
            .await
            .expect("the first delivery lands");
        let at = position_of(&store, sent[0].id).await;
        assert_eq!(store.remove(&[at]), 1);
        let before = log(&store).await;

        let again = store
            .ingest(&[IngestGroup::new(&sent, &answer)])
            .await
            .expect("a re-delivery is no error");

        assert_eq!(
            (again.appended, again.skipped, again.compensated),
            (0, 2, 0)
        );
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn a_group_with_one_new_event_lands_it_and_compensates_once() {
        let store = MemoryIngestStore::new();
        let first = [shipped(id(ORIGIN, 1), "x1")];
        store
            .ingest(&[IngestGroup::new(&first, &[])])
            .await
            .expect("the first delivery lands");
        let at = position_of(&store, first[0].id).await;
        assert_eq!(store.remove(&[at]), 1);
        let sent = [shipped(id(ORIGIN, 1), "x1"), shipped(id(ORIGIN, 2), "x2")];
        let answer = [event("compensated")];

        let ingested = store
            .ingest(&[IngestGroup::new(&sent, &answer)])
            .await
            .expect("a group with one new event lands it");

        assert_eq!(
            (ingested.appended, ingested.skipped, ingested.compensated),
            (1, 1, 1)
        );
        let held: Vec<Event> = log(&store).await.into_iter().map(|row| row.event).collect();
        assert_eq!(held, [event("x2"), event("compensated")]);
    }

    /// SY-39's bookkeeping after a removal: a group with no row left is not
    /// read back, its guard goes with it, and the groups around it keep their
    /// names and guards.
    #[tokio::test]
    async fn the_group_read_drops_a_removed_group() {
        let store = MemoryIngestStore::new();
        let guard = AppendCondition::new(Query::from_item(
            QueryItem::of_types(["Written"]).expect("a valid item"),
        ));
        let x = [shipped(id(ORIGIN, 1), "x1"), shipped(id(ORIGIN, 2), "x2")];
        let y = [shipped(id(ORIGIN, 3), "y1")];
        let z = [shipped(id(ORIGIN, 4), "z1")];
        store
            .ingest(&[
                IngestGroup::new(&x, &[]).with_guard(Some(&guard)),
                IngestGroup::new(&y, &[]).with_guard(Some(&guard)),
                IngestGroup::new(&z, &[]).with_guard(Some(&guard)),
            ])
            .await
            .expect("the oracle ingests");
        let x_at = [
            position_of(&store, x[0].id).await,
            position_of(&store, x[1].id).await,
        ];
        let y_first = position_of(&store, y[0].id).await;
        let z_first = position_of(&store, z[0].id).await;

        assert_eq!(store.remove(&x_at), 2);
        let page = store
            .read_groups(None, NonZeroUsize::new(16).expect("non-zero"))
            .await
            .expect("the oracle's scan is in group order");

        let named: Vec<(SequencePosition, Option<&AppendCondition>)> = page
            .groups
            .iter()
            .map(|group| (group.first(), group.guard()))
            .collect();
        assert_eq!(named, [(y_first, Some(&guard)), (z_first, Some(&guard))]);
        let kept: Vec<SequencePosition> = store.read_guard().guards.keys().copied().collect();
        assert_eq!(
            kept,
            [y_first, z_first],
            "the removed group's guard is gone"
        );
    }

    /// Removing the opening row of a guarded group drops the guard with it:
    /// the read takes a group's guard from its opening row only, so the rows
    /// left are read back under the group's name with no guard, and the store
    /// keeps no guard entry nothing can read.
    #[tokio::test]
    async fn removing_a_groups_first_row_drops_its_guard() {
        let store = MemoryIngestStore::new();
        let guard = AppendCondition::new(Query::from_item(
            QueryItem::of_types(["Written"]).expect("a valid item"),
        ));
        let x = [shipped(id(ORIGIN, 1), "x1"), shipped(id(ORIGIN, 2), "x2")];
        let y = [shipped(id(ORIGIN, 3), "y1")];
        store
            .ingest(&[
                IngestGroup::new(&x, &[]).with_guard(Some(&guard)),
                IngestGroup::new(&y, &[]).with_guard(Some(&guard)),
            ])
            .await
            .expect("the oracle ingests");
        let x_first = position_of(&store, x[0].id).await;
        let y_first = position_of(&store, y[0].id).await;

        assert_eq!(store.remove(&[x_first]), 1);
        let page = store
            .read_groups(None, NonZeroUsize::new(16).expect("non-zero"))
            .await
            .expect("the oracle's scan is in group order");

        let named: Vec<(SequencePosition, Option<&AppendCondition>, usize)> = page
            .groups
            .iter()
            .map(|group| (group.first(), group.guard(), group.events().len()))
            .collect();
        assert_eq!(named, [(x_first, None, 1), (y_first, Some(&guard), 1)]);
        let kept: Vec<SequencePosition> = store.read_guard().guards.keys().copied().collect();
        assert_eq!(kept, [y_first], "a guard nothing can read is still kept");
    }

    const ORIGIN: StoreId = StoreId::from_bytes([0x64; 16]);

    fn id(store: StoreId, position: u64) -> EventId {
        EventId::new(
            store,
            SequencePosition::new(position).expect("every position written here is non-zero"),
        )
    }

    fn shipped(id: EventId, data: &str) -> ReplicatedEvent {
        ReplicatedEvent::new(id, RecordedAt::from_millis(1_800_000_000_000), event(data))
    }

    fn one_group_each(events: &[ReplicatedEvent]) -> Vec<IngestGroup<'_>> {
        events
            .chunks(1)
            .map(|event| IngestGroup::new(event, &[]))
            .collect()
    }

    /// The local position of the one event carrying `id`.
    async fn position_of(store: &MemoryIngestStore, id: EventId) -> SequencePosition {
        log(store)
            .await
            .iter()
            .find(|row| row.id == id)
            .map(|row| row.position)
            .expect("the event is held")
    }
}
