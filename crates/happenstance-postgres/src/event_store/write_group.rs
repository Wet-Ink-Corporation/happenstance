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

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, SequencePosition};
use happenstance_sync::write_group::{GroupPageBuilder, decode_guard, encode_guard};
use happenstance_sync::{GroupPage, ReplicatedEvent, SendWriteGroupStore, Waiting};
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row as _, Transaction};

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
/// `$1` is `after`, or 0 for the start of the log; `$2` is one more than the
/// page's event budget.
///
/// * `scan` takes the first `$2` rows above `after` in position order, each
///   with its group. A group's lowest row is its name and sits below the rest
///   of it, so every group named here has its name in the scan, and the
///   groups it names are the lowest above `after`: a prefix of the groups in
///   order. `position > $1` lets the planner seek on the primary key; the
///   `coalesce` then drops the tail of the group `after` names.
/// * The outer `SELECT` reads every row of those groups, ordered by group and
///   then position: by `write_group` through `event_write_group_idx`, or, for
///   a row from before migration 3, by its own position through the key.
/// * `scanned` is how many rows `scan` found. Fewer than `$2` means it reached
///   the end of the log, which is the only case the page may say
///   [`Waiting::Nothing`].
///
/// `scan` is referenced twice, so Postgres materialises it once and both uses
/// agree.
const GROUPS_SQL: &str = "WITH scan AS (\
     SELECT coalesce(write_group, position) AS g FROM event \
     WHERE position > $1 AND coalesce(write_group, position) > $1 AND {frontier} \
     ORDER BY position LIMIT $2), \
     ids AS (SELECT DISTINCT g FROM scan) \
     SELECT position, event_type, data, metadata, tags, origin_store, origin_position, \
     recorded_at, coalesce(write_group, position) AS write_group_first, group_guard, \
     (SELECT count(*) FROM scan) AS scanned \
     FROM event \
     WHERE (write_group IN (SELECT g FROM ids) \
     OR (write_group IS NULL AND position IN (SELECT g FROM ids))) \
     AND {frontier} \
     ORDER BY coalesce(write_group, position), position";

/// [`GROUPS_SQL`] under `visibility`'s predicate.
fn groups_sql(visibility: Visibility) -> String {
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
    /// The statement returns every row of up to `max_events + 1` groups, each
    /// bounded by this store's batch ceiling, so the rows held are bounded
    /// (H-13); the page builder then keeps what the budget allows.
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
        // One more than the budget, so a scan that fills it is told apart
        // from one that reached the end of the log.
        let probe = i64::try_from(max_events.get())
            .unwrap_or(i64::MAX)
            .saturating_add(1);
        let rows: Vec<PgRow> = self
            .on_runtime(async move {
                sqlx::query(&sql)
                    .bind(after)
                    .bind(probe)
                    .fetch_all(&pool)
                    .await
                    .map_err(PostgresEventStoreError::from)
            })
            .await?;

        let scanned = match rows.first() {
            Some(row) => row.try_get::<i64, _>("scanned")?,
            None => 0,
        };
        let mut page = GroupPageBuilder::new(max_events);
        for row in &rows {
            let (first, guard, event) = decode_group_row(row)?;
            if page.offer(first, guard, event).is_break() {
                return Ok(page.finish(Waiting::Unknown)?);
            }
        }
        let past = if scanned < probe {
            Waiting::Nothing
        } else {
            Waiting::Unknown
        };
        Ok(page.finish(past)?)
    }
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

    /// The read carries the frontier in both halves: the scan that names the
    /// groups and the read of their rows. Without it in the scan, a group
    /// above the frontier is named and its rows filtered out, and the page
    /// says it read a group it did not; without it in the read, a legacy row
    /// above the frontier is read beside a group that is not.
    #[test]
    fn the_group_read_carries_the_frontier_twice() {
        let sql = groups_sql(Visibility::Frontier);
        assert_eq!(
            sql.matches(Visibility::Frontier.predicate()).count(),
            2,
            "{sql}"
        );
        assert!(!sql.contains("{frontier}"), "a placeholder survived: {sql}");
    }

    /// The rows are ordered by group, then position; a scan by position is the
    /// read that tears interleaved groups, and the page builder refuses it.
    #[test]
    fn the_group_read_orders_by_group_first() {
        assert!(
            GROUPS_SQL.ends_with("ORDER BY coalesce(write_group, position), position"),
            "{GROUPS_SQL}"
        );
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
