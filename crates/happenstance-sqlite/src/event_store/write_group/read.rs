//! Reading the log back in write groups: the body of this store's
//! `WriteGroupStore` impl, which lives in `src/ingest_spike.rs` with the rest
//! of the `happenstance-sync` surface.

use core::num::NonZeroUsize;

use happenstance_core::SequencePosition;
use happenstance_sync::write_group::{GroupPageBuilder, decode_guard};
use happenstance_sync::{GroupPage, ReplicatedEvent, Waiting};

// `as_i64` saturates, which is right for a query bound: no row sits above
// `i64::MAX`, so a bound there reads nothing, as it should.
use crate::event_store::{SqliteEventStore, SqliteEventStoreError, as_i64, position_from_row};

/// Every row above a group's first, in position order, with its group and
/// guard.
///
/// `?1` is the `after` position, or 0 for the start of the log. A group's
/// first is never above its own rows, so `position > ?1` keeps every row the
/// next condition keeps and lets the planner seek on the primary key; the
/// `coalesce` then drops the tail of the group `after` names, and reads a row
/// written before migration 2 as a group of its own.
pub(super) const GROUPS_SQL: &str = "SELECT position, event_type, data, metadata, tags, \
     origin_store, origin_position, recorded_at, \
     coalesce(write_group, position), group_guard \
     FROM event WHERE position > ?1 AND coalesce(write_group, position) > ?1 \
     ORDER BY position";

impl SqliteEventStore {
    /// Up to `max_events` events of whole write groups above `after`.
    ///
    /// Synchronous under the connection's lock, as the watermark is: the
    /// statement is one primary-key range, stepped only as far as the page
    /// takes, so the lock is held for one page and no longer. A group's rows
    /// are contiguous because this store serialises its writers, so a scan in
    /// position order meets each group whole.
    ///
    /// # Errors
    ///
    /// The connection was poisoned, the driver failed, a stored row does not
    /// decode, a stored guard does not
    /// ([`StoredGuard`](SqliteEventStoreError::StoredGuard)), or a stored
    /// `write_group` names a group the scan had already passed, which only a
    /// damaged file holds ([`GroupOrder`](SqliteEventStoreError::GroupOrder)).
    pub(crate) fn read_write_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, SqliteEventStoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        let mut statement = connection.prepare(GROUPS_SQL)?;
        let mut rows = statement.query([after.map_or(0, as_i64)])?;
        let mut page = GroupPageBuilder::new(max_events);
        while let Some(row) = rows.next()? {
            let stored = crate::row::to_event(row)?;
            let first = position_from_row(row.get(8)?)?;
            let guard = match row.get::<_, Option<Vec<u8>>>(9)? {
                Some(bytes) if stored.position == first => Some(
                    decode_guard(&bytes)
                        .map_err(|source| SqliteEventStoreError::StoredGuard { first, source })?,
                ),
                _ => None,
            };
            let event = ReplicatedEvent::new(stored.id, stored.recorded_at, stored.event);
            if page.offer(first, guard, event).is_break() {
                return Ok(page.finish(Waiting::Unknown)?);
            }
        }
        Ok(page.finish(Waiting::Nothing)?)
    }
}

#[cfg(test)]
mod tests {
    //! What the write-group family cannot see from outside: a row from before
    //! migration 2, a guard column the codec refuses, and the plan. No literal
    //! local position: every one compared is read back from the store.

    use core::num::NonZeroUsize;

    use happenstance_core::{
        AppendCondition, Event, EventId, Query, QueryItem, RecordedAt, SendEventStore,
        SequencePosition, StoreId,
    };
    use happenstance_sync::write_group::{GroupOrderError, GuardCodecError};
    use happenstance_sync::{
        IngestGroup, ReplicatedEvent, SendIngestStore, SendWriteGroupStore, WriteGroup,
    };

    use super::GROUPS_SQL;
    use crate::event_store::{SqliteEventStore, SqliteEventStoreError};

    const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

    fn event(data: &str) -> Event {
        Event::new("Grouped", data.as_bytes().to_vec()).expect("a valid event type")
    }

    /// Runs `sql` on the store's own connection, as a file from an older
    /// build or a damaged one would present it.
    fn raw(store: &SqliteEventStore, sql: &str) {
        store
            .connection
            .lock()
            .expect("the connection is not poisoned")
            .execute_batch(sql)
            .expect("the arrangement's SQL runs");
    }

    /// Each group's first, and its size.
    fn shape(groups: &[WriteGroup]) -> Vec<(SequencePosition, usize)> {
        groups
            .iter()
            .map(|group| (group.first(), group.events().len()))
            .collect()
    }

    /// #420's legacy rule: a row written before migration 2, `NULL` in both
    /// columns, reads as a group of its own named by its position; the append
    /// after it reads as one group.
    #[tokio::test]
    async fn a_row_from_before_migration_2_reads_as_its_own_group() {
        let store = SqliteEventStore::open_in_memory().expect("an in-memory store");
        let legacy = store
            .append(&[event("legacy-1"), event("legacy-2")], None)
            .await
            .expect("an append");
        // Undo the stamp, which is what a version-1 build never wrote.
        raw(&store, "UPDATE event SET write_group = NULL");
        let grouped = store
            .append(&[event("new-1"), event("new-2")], None)
            .await
            .expect("an append");

        let page = SendWriteGroupStore::read_groups(&store, None, WHOLE_LOG)
            .await
            .expect("a group read");

        let firsts: Vec<SequencePosition> = page.groups.iter().map(WriteGroup::first).collect();
        let [first_legacy, second_legacy, new] = firsts[..] else {
            panic!("expected three groups, got {:?}", shape(&page.groups));
        };
        assert_eq!(
            second_legacy, legacy,
            "a legacy row is named by its own position"
        );
        assert!(first_legacy < second_legacy);
        assert!(
            new > legacy && new < grouped,
            "the append's group is named by its first row"
        );
        assert_eq!(
            shape(&page.groups),
            [(first_legacy, 1), (second_legacy, 1), (new, 2)]
        );
        let guards: Vec<Option<&AppendCondition>> =
            page.groups.iter().map(WriteGroup::guard).collect();
        assert_eq!(guards, [None, None, None]);
    }

    /// A guard column holding a format no build of the codec wrote stops the
    /// read with the group it belongs to, rather than forwarding the group
    /// without its guard.
    #[tokio::test]
    async fn a_guard_the_codec_cannot_read_stops_the_group_read() {
        let store = SqliteEventStore::open_in_memory().expect("an in-memory store");
        let origin = StoreId::from_bytes([0x5A; 16]);
        let shipped = [ReplicatedEvent::new(
            EventId::new(origin, SequencePosition::FIRST),
            RecordedAt::from_millis(1_800_000_000_000),
            event("guarded"),
        )];
        let guard = AppendCondition::new(Query::from_item(
            QueryItem::of_types(["Grouped"]).expect("a valid query"),
        ));
        SendIngestStore::ingest(
            &store,
            &[IngestGroup::new(&shipped, &[]).with_guard(Some(&guard))],
        )
        .await
        .expect("an ingest");
        let first = SendWriteGroupStore::read_groups(&store, None, WHOLE_LOG)
            .await
            .expect("the guard as stored reads back")
            .groups
            .first()
            .map(WriteGroup::first)
            .expect("one group");
        raw(
            &store,
            "UPDATE event SET group_guard = X'09' WHERE group_guard IS NOT NULL",
        );

        let refused = SendWriteGroupStore::read_groups(&store, None, WHOLE_LOG).await;

        assert!(
            matches!(
                refused,
                Err(SqliteEventStoreError::StoredGuard {
                    first: named,
                    source: GuardCodecError::UnknownFormat { found: 9, .. },
                }) if named == first
            ),
            "got {refused:?}"
        );
    }

    /// A `write_group` that names a group the scan already passed is refused
    /// with the two groups, rather than paged as a group torn in two under one
    /// name. No build writes one; a damaged or hand-edited file can.
    #[tokio::test]
    async fn a_row_naming_a_group_already_passed_stops_the_group_read() {
        let store = SqliteEventStore::open_in_memory().expect("an in-memory store");
        let a = store.append(&[event("a")], None).await.expect("an append");
        let b = store.append(&[event("b")], None).await.expect("an append");
        let c = store.append(&[event("c")], None).await.expect("an append");
        raw(
            &store,
            &format!(
                "UPDATE event SET write_group = {} WHERE position = {}",
                a.get(),
                c.get()
            ),
        );

        let refused = SendWriteGroupStore::read_groups(&store, None, WHOLE_LOG).await;

        assert!(
            matches!(
                refused,
                Err(SqliteEventStoreError::GroupOrder(GroupOrderError {
                    first,
                    after,
                    ..
                })) if first == a && after == b
            ),
            "got {refused:?}"
        );
    }

    /// The group read runs under the connection's lock: it must seek on the
    /// position and read in its order, never scan or sort the log.
    ///
    /// **Named wrong statement, checked red:** the same `SELECT` filtered on
    /// `coalesce(write_group, position) > ?1` alone, which no index serves;
    /// SQLite plans `SCAN event` and the first assertion fails.
    #[test]
    fn the_group_read_seeks_on_the_position_without_a_sort() {
        let mut connection = rusqlite::Connection::open_in_memory().expect("a database");
        SqliteEventStore::migrate(&mut connection).expect("migrating must succeed");
        let mut statement = connection
            .prepare(&format!("EXPLAIN QUERY PLAN {GROUPS_SQL}"))
            .expect("the statement prepares");
        let rows: Vec<String> = statement
            .query_map([0], |row| row.get::<_, String>(3))
            .expect("the plan reads")
            .collect::<Result<_, _>>()
            .expect("every plan row reads");

        assert!(
            rows.iter()
                .any(|detail| detail.contains("USING INTEGER PRIMARY KEY")),
            "the group read does not seek on the position: {rows:?}"
        );
        assert!(
            !rows.iter().any(|detail| detail.contains("TEMP B-TREE")),
            "the group read sorts the log: {rows:?}"
        );
    }
}
