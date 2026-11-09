//! Write groups (#420): which rows one `append`, or one ingested group with its
//! compensation, wrote together, and the log read back in them (SY-39).
//!
//! # What is recorded
//!
//! Migration 3's two columns. `write_group` holds the lowest position the
//! write put in, the same value on every row of it (I-2); `group_guard` holds
//! the guard an ingested group arrived under, in `happenstance-sync`'s stored
//! form, on that first row only. An append binds its group in the one
//! `INSERT` that writes it (`insert_statement`'s ninth column). An ingested
//! group's foreign rows are written with `NULL` and stamped by [`STAMP_SQL`]
//! once the conflict clause has said which landed, because the group is named
//! by the first row it **wrote**, and a re-delivered row burns its position
//! without writing one. Its compensation binds the stamped value directly: it
//! draws its positions after the foreign rows, from the same sequence, so they
//! are higher.
//!
//! A row written before migration 3 has `NULL` in both and reads as a group of
//! its own, named by its position, with no guard.
//!
//! # Why the read orders by group and not by position
//!
//! This store does not serialise its writers. Two concurrent appends draw from
//! one sequence and can interleave, G = {5, 7} and H = {6, 8}, so a scan by
//! position meets G's rows on both sides of H's first and would tear it.
//! [`GROUPS_SQL`] finds the groups by a scan by position, which meets each
//! group's lowest row first and so finds them in group order, and then reads
//! their rows ordered by group. One statement, so one snapshot, under the same
//! frontier every read here carries: a group is one transaction, so if its
//! name is visible all its rows are.
//!
//! Behind `sync`, through its declaration in `event_store.rs`, as the ingest
//! module is.

use core::future::poll_fn;
use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, SequencePosition};
use happenstance_sync::write_group::{GroupPageBuilder, decode_guard, encode_guard};
use happenstance_sync::{GroupPage, ReplicatedEvent, SendWriteGroupStore, Waiting};
use sqlx::postgres::{PgArguments, PgRow};
use sqlx::query::Query;
use sqlx::{PgPool, Postgres, Row as _, Transaction};

use super::{PostgresEventStore, Visibility, as_i64, position_from_row};
use crate::error::PostgresEventStoreError;

/// Stamps the rows an ingested group landed with the group, and its first row
/// with its guard.
///
/// `$1` is the lowest landed position, `$2` the stored guard or `NULL`, `$3`
/// every landed position. `= ANY($3)` is a primary-key lookup per position, so
/// the stamp touches exactly the rows the conflict clause reported and no
/// others: unlike SQLite's, this store has no writer lock that would make
/// "every row from `$1` up" this transaction's.
pub(super) const STAMP_SQL: &str = "UPDATE event \
     SET write_group = $1, group_guard = CASE WHEN position = $1 THEN $2::bytea END \
     WHERE position = ANY($3)";

/// The group read, before the frontier predicate is spliced in at `{frontier}`.
///
/// `$1` is `after`, or 0 for the start of the log; `$2` is the page's event
/// budget and `$3` one more.
///
/// * `scan` takes the first `$3` rows above `after` in position order, each
///   with its group. A group's lowest row is its name and sits below the rest
///   of it, so every group named here has its name in the scan, and the
///   groups it names are the lowest above `after`: a prefix of the groups in
///   order. `position > $1` lets the planner seek on the primary key; the
///   `coalesce` then drops the tail of the group `after` names.
/// * `ids` keeps the groups named by the first `$2` of those rows only. The
///   groups those rows belong to hold at least `$2` events between them, so a
///   group named first by the probe row could never fit the page, and leaving
///   it out changes no page; it bounds what the read returns to the rows of
///   `$2` groups.
/// * The two arms read every row of those groups by key, each a join the
///   planner runs as an index lookup per group: by `write_group` through
///   `event_write_group_idx`, and, for a row from before migration 3, whose
///   `write_group` is `NULL`, by its own position through the primary key. One
///   `IN (…) OR …` over both would be a filter over every row of the table.
///   No row is in both arms: a row with a group is not `NULL`, and a legacy
///   row's position names no group but its own.
/// * `scanned` is how many rows `scan` found. Fewer than `$3` means it reached
///   the end of the log, which is the only case the page may say
///   [`Waiting::Nothing`]; `ids`' cap cannot change that, because a scan that
///   short has no probe row to leave out.
///
/// `scan` is referenced twice, so Postgres materialises it once and both uses
/// agree.
const GROUPS_SQL: &str = "WITH scan AS (\
     SELECT position AS p, coalesce(write_group, position) AS g FROM event \
     WHERE position > $1 AND coalesce(write_group, position) > $1 AND {frontier} \
     ORDER BY position LIMIT $3), \
     ids AS (SELECT DISTINCT g FROM (SELECT g FROM scan ORDER BY p LIMIT $2) AS head), \
     counted AS (SELECT count(*) AS n FROM scan) \
     SELECT e.position, e.event_type, e.data, e.metadata, e.tags, e.origin_store, \
     e.origin_position, e.recorded_at, ids.g AS write_group_first, e.group_guard, \
     (SELECT n FROM counted) AS scanned \
     FROM ids JOIN event AS e ON e.write_group = ids.g \
     WHERE {frontier} \
     UNION ALL \
     SELECT e.position, e.event_type, e.data, e.metadata, e.tags, e.origin_store, \
     e.origin_position, e.recorded_at, ids.g AS write_group_first, e.group_guard, \
     (SELECT n FROM counted) AS scanned \
     FROM ids JOIN event AS e ON e.position = ids.g \
     WHERE e.write_group IS NULL AND {frontier} \
     ORDER BY write_group_first, position";

/// [`GROUPS_SQL`] with its binds: `after` (0 for the start of the log), the
/// page's budget, and the probe.
pub(super) fn groups_query(
    sql: &str,
    after: i64,
    max_events: NonZeroUsize,
) -> Query<'_, Postgres, PgArguments> {
    sqlx::query(sql)
        .bind(after)
        .bind(budget(max_events))
        .bind(probe(max_events))
}

/// The budget as the `bigint` a `LIMIT` takes. A budget past `i64::MAX`
/// saturates, which no log can tell apart: it holds fewer positions than that.
fn budget(max_events: NonZeroUsize) -> i64 {
    i64::try_from(max_events.get()).unwrap_or(i64::MAX)
}

/// One more than the budget, so a scan that fills it is told apart from one
/// that reached the end of the log.
fn probe(max_events: NonZeroUsize) -> i64 {
    budget(max_events).saturating_add(1)
}

/// [`GROUPS_SQL`] under `visibility`'s predicate.
pub(super) fn groups_sql(visibility: Visibility) -> String {
    GROUPS_SQL.replace("{frontier}", visibility.predicate())
}

/// Records that the rows at `landed`, which this transaction just wrote, are
/// one write group named `first`, and stores `guard` on the row at `first`.
///
/// # Errors
///
/// [`StoredGuard`](PostgresEventStoreError::StoredGuard) if the guard does not
/// encode, before anything is sent, and the driver's error if the stamp fails.
/// Either way the caller's transaction is dropped uncommitted.
pub(super) async fn stamp(
    transaction: &mut Transaction<'static, Postgres>,
    first: i64,
    landed: &[i64],
    guard: Option<&AppendCondition>,
) -> Result<(), PostgresEventStoreError> {
    let stored = match guard.map(encode_guard).transpose() {
        Ok(stored) => stored,
        Err(source) => {
            return Err(PostgresEventStoreError::StoredGuard {
                first: position_from_row(first)?,
                source,
            });
        }
    };
    sqlx::query(STAMP_SQL)
        .bind(first)
        .bind(stored)
        .bind(landed)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

impl PostgresEventStore {
    /// Up to `max_events` events of whole write groups above `after`, in one
    /// statement on the store's runtime.
    ///
    /// The rows are streamed into the page builder and the stream dropped as
    /// soon as it breaks, so only the page's own rows are held, never the
    /// whole answer (H-13).
    async fn read_write_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, PostgresEventStoreError> {
        // A refcounted handle, moved into the `'static` work as every method
        // here moves one.
        let pool = self.pool.clone();
        let sql = groups_sql(self.visibility);
        let after = after.map_or(0, as_i64);
        self.on_runtime(async move { page_of_groups(&pool, &sql, after, max_events).await })
            .await
    }
}

/// Streams [`GROUPS_SQL`]'s rows into a page of at most `max_events` events.
///
/// # Errors
///
/// The driver's error, what [`decode_group_row`] refuses, and
/// [`GroupOrder`](PostgresEventStoreError::GroupOrder) for rows out of group
/// order.
async fn page_of_groups(
    pool: &PgPool,
    sql: &str,
    after: i64,
    max_events: NonZeroUsize,
) -> Result<GroupPage, PostgresEventStoreError> {
    let mut rows = groups_query(sql, after, max_events).fetch(pool);
    let mut page = GroupPageBuilder::new(max_events);
    let mut scanned = 0;
    while let Some(row) = poll_fn(|cx| rows.as_mut().poll_next(cx)).await {
        let row = row?;
        // Every row carries the same count; the first is as good as any.
        scanned = row.try_get::<i64, _>("scanned")?;
        let (first, guard, event) = decode_group_row(&row)?;
        if page.offer(first, guard, event).is_break() {
            return Ok(page.finish(Waiting::Unknown)?);
        }
    }
    let past = if scanned < probe(max_events) {
        Waiting::Nothing
    } else {
        Waiting::Unknown
    };
    Ok(page.finish(past)?)
}

/// One row of [`GROUPS_SQL`]: its group, the group's guard if this is the
/// group's first row, and the event.
///
/// # Errors
///
/// What [`decode_row`](crate::read_stream::decode_row) refuses, a group name
/// that is not a position, and [`StoredGuard`](PostgresEventStoreError::StoredGuard)
/// for a guard the codec cannot read.
fn decode_group_row(
    row: &PgRow,
) -> Result<(SequencePosition, Option<AppendCondition>, ReplicatedEvent), PostgresEventStoreError> {
    let stored = crate::read_stream::decode_row(row)?;
    let first = position_from_row(row.try_get::<i64, _>("write_group_first")?)?;
    let guard = match row.try_get::<Option<Vec<u8>>, _>("group_guard")? {
        Some(bytes) if stored.position == first => Some(
            decode_guard(&bytes)
                .map_err(|source| PostgresEventStoreError::StoredGuard { first, source })?,
        ),
        _ => None,
    };
    let event = ReplicatedEvent::new(stored.id, stored.recorded_at, stored.event);
    Ok((first, guard, event))
}

/// The log in the writes that made it (SY-39): what a runner pushes from and a
/// `StoreSyncPeer` serves a pull from. The `Send` flavour, as this store's
/// `SendIngestStore` is; `trait_variant` derives the bare one.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
impl SendWriteGroupStore for PostgresEventStore {
    type Error = PostgresEventStoreError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        self.read_write_groups(after, max_events).await
    }
}

#[cfg(test)]
mod tests {
    use super::{GROUPS_SQL, STAMP_SQL, groups_sql};
    use crate::event_store::Visibility;

    /// The read carries the frontier in all three of its reads of the table:
    /// the scan that names the groups and both arms that read their rows.
    /// Without it in the scan, a group above the frontier is named and its rows
    /// filtered out, and the page says it read a group it did not; without it
    /// in an arm, a row above the frontier is read beside groups that are not.
    #[test]
    fn the_group_read_carries_the_frontier_in_every_read_of_the_table() {
        let sql = groups_sql(Visibility::Frontier);
        assert_eq!(
            sql.matches(Visibility::Frontier.predicate()).count(),
            3,
            "{sql}"
        );
        assert!(!sql.contains("{frontier}"), "a placeholder survived: {sql}");
    }

    /// The rows are ordered by group, then position; a scan by position is the
    /// read that tears interleaved groups, and the page builder refuses it.
    /// The rows are read by key in two arms, never by a filter over the table.
    #[test]
    fn the_group_read_joins_by_key_and_orders_by_group_first() {
        assert!(
            GROUPS_SQL.ends_with("ORDER BY write_group_first, position"),
            "{GROUPS_SQL}"
        );
        assert!(
            GROUPS_SQL.contains("JOIN event AS e ON e.write_group = ids.g")
                && GROUPS_SQL.contains("JOIN event AS e ON e.position = ids.g")
                && GROUPS_SQL.contains(" UNION ALL "),
            "{GROUPS_SQL}"
        );
        assert!(!GROUPS_SQL.contains(" OR "), "{GROUPS_SQL}");
    }

    /// The stamp names the landed rows by key, never a range: another
    /// transaction may hold any position between two of this group's.
    #[test]
    fn the_stamp_touches_only_the_landed_rows() {
        assert!(
            STAMP_SQL.ends_with("WHERE position = ANY($3)"),
            "{STAMP_SQL}"
        );
        assert!(!STAMP_SQL.contains(">="), "{STAMP_SQL}");
    }
}
