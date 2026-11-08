//! Write groups (#420): which rows one `append`, or one ingested group with its
//! compensation, wrote together, and the log read back in them (SY-39).
//!
//! # What is recorded
//!
//! Migration 3's two columns. `write_group` holds the lowest position the
//! write put in, the same value on every row of it (I-2); `group_guard` holds
//! the guard an ingested group arrived under, in `happenstance-sync`'s stored
//! form, on that first row only. A row written before migration 3 has `NULL`
//! in both and reads as a group of its own, named by its position.
//!
//! An append computes its group in the one `INSERT` that writes it
//! (`APPEND_GROUP`). An ingest cannot: which of a group's rows land is decided
//! by the conflict clause inside the statement's `ins` CTE, and the rows a
//! data-modifying CTE writes are invisible to the rest of its statement. So
//! the ingest writes every row with [`PROVISIONAL_GROUP`], a negative tag
//! naming its group's index in the call, and [`stamp_statement`]'s `UPDATE`,
//! the third statement of the same request and so the same transaction,
//! replaces each tag with the lowest position its group landed and puts the
//! group's guard on that row. A position is never negative, so a tag cannot be
//! mistaken for a name, and none outlives the commit.
//!
//! # Why the read orders by group and not by position
//!
//! This store does not serialise its writers: two concurrent appends draw from
//! one sequence and can interleave, G = {5, 7} and H = {6, 8}, so a scan by
//! position would tear G. The read names the groups with a scan by position,
//! which meets each group's lowest row first and so finds them in order, and
//! then reads their rows ordered by group. One statement, so one snapshot,
//! under the frontier every read here carries; a group is one transaction, so
//! if its name is visible all its rows are. No cursor is needed: one round
//! trip answers one page.
//!
//! Behind `sync`, through its declaration in `event_store.rs`, as the ingest
//! module is.

use core::num::NonZeroUsize;

use base64::Engine as _;
use happenstance_core::{AppendCondition, SequencePosition};
use happenstance_sync::write_group::{GroupPageBuilder, decode_guard, encode_guard};
use happenstance_sync::{GroupPage, IngestGroup, ReplicatedEvent, Waiting, WriteGroupStore};

use super::{
    BASE64, FRONTIER, NeonEventStore, SELECTED_COLUMNS, as_i64, decode_base64, decode_body,
    decode_row, first_result_set, position_from_text,
};
use crate::error::NeonError;
use crate::transport::{SqlRequest, SqlStatement, SqlTransport};

/// An ingested row's provisional `write_group`: its group's index in the call,
/// `grp`, as `-(grp + 1)`, so the first group is `-1` and no tag is zero or a
/// position. Read off `b`, the rows subquery `insert_rows` joins.
pub(super) const PROVISIONAL_GROUP: &str = "-(b.grp + 1)";

impl<T> NeonEventStore<T> {
    /// The ingest request's third statement: each provisional tag this
    /// transaction wrote replaced by the lowest position its group landed, and
    /// that row given the group's guard.
    ///
    /// `$1` is a `jsonb` array of every group's stored guard, base64, or
    /// `null` for none, indexed by the group's index in the call, which is
    /// `-tag - 1`. `->>` of a JSON `null` is SQL `NULL`, and `decode` of
    /// `NULL` is `NULL`, so a group with no guard stores none.
    ///
    /// Both reads of the table are bounded by `xact_id =
    /// pg_current_xact_id()`, served by the frontier's own index: only this
    /// call's rows can carry a tag, and the filter says so to the planner. A
    /// group that landed nothing has no row, so nothing to stamp; its
    /// compensation was not written either.
    ///
    /// # Errors
    ///
    /// [`NeonError::UnstorableGuard`] for a guard the codec refuses, before
    /// anything is sent.
    pub(super) fn stamp_statement<E>(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<SqlStatement, NeonError<E>> {
        let guards = groups
            .iter()
            .enumerate()
            .map(|(group, ingested)| match ingested.guard {
                Some(guard) => encode_guard(guard)
                    .map(|stored| serde_json::Value::String(BASE64.encode(stored)))
                    .map_err(|source| NeonError::UnstorableGuard { group, source }),
                None => Ok(serde_json::Value::Null),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let event = self.config.qualified_event();
        let sql = format!(
            "UPDATE {event} AS e SET write_group = g.first, \
             group_guard = CASE WHEN e.position = g.first \
             THEN decode(($1::jsonb) ->> ((-g.tag - 1)::int), 'base64') END \
             FROM (SELECT write_group AS tag, min(position) AS first FROM {event} \
             WHERE xact_id = pg_current_xact_id() AND write_group < 0 \
             GROUP BY write_group) AS g \
             WHERE e.xact_id = pg_current_xact_id() AND e.write_group = g.tag"
        );
        Ok(SqlStatement::with_params(
            sql,
            vec![serde_json::Value::String(
                serde_json::Value::Array(guards).to_string(),
            )],
        ))
    }

    /// The group read: every row of the lowest groups above `after`, ordered
    /// by group, as one read-only statement.
    ///
    /// `scan` takes the first `max_events + 1` rows above `after` in position
    /// order, each with its group. A group's lowest row is its name and sits
    /// below the rest of it, so the groups `scan` names are the lowest above
    /// `after`, a prefix in order. The outer `SELECT` reads all their rows, by
    /// `write_group` through the partial index or, for a row from before
    /// migration 3, by position through the key. `scanned` is how many rows
    /// `scan` found: fewer than it asked for means it reached the end of the
    /// log, the only case a page may say [`Waiting::Nothing`].
    ///
    /// The bounds are interpolated `i64`s, as `read_request`'s are, so no
    /// caller string reaches the SQL. `ORDER BY` names qualified columns, for
    /// the reason `read_request` gives: `position` is also a text alias here.
    fn groups_request(&self, after: i64, probe: i64) -> SqlRequest {
        let event = self.config.qualified_event();
        let sql = format!(
            "WITH scan AS (SELECT coalesce(write_group, position) AS g FROM {event} \
             WHERE position > {after} AND coalesce(write_group, position) > {after} \
             AND {FRONTIER} ORDER BY position LIMIT {probe}), \
             ids AS (SELECT DISTINCT g FROM scan) \
             SELECT {SELECTED_COLUMNS}, \
             coalesce(e.write_group, e.position)::text AS write_group_first, \
             encode(e.group_guard, 'base64') AS group_guard, \
             (SELECT count(*) FROM scan)::text AS scanned \
             FROM {event} AS e \
             WHERE (e.write_group IN (SELECT g FROM ids) \
             OR (e.write_group IS NULL AND e.position IN (SELECT g FROM ids))) \
             AND {FRONTIER} \
             ORDER BY coalesce(e.write_group, e.position), e.position"
        );
        SqlRequest::single(SqlStatement::new(sql)).read_only()
    }
}

impl<T: SqlTransport> NeonEventStore<T> {
    /// Up to `max_events` events of whole write groups above `after`, in one
    /// round trip.
    ///
    /// The answer holds every row of up to `max_events + 1` groups, each
    /// bounded by this store's batch ceiling, and the response by
    /// `max_response_bytes`; the page builder keeps what the budget allows.
    async fn read_write_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, NeonError<T::Error>> {
        // One more than the budget, so a scan that fills it is told apart
        // from one that reached the end of the log.
        let probe = i64::try_from(max_events.get())
            .unwrap_or(i64::MAX)
            .saturating_add(1);
        let request = self.groups_request(after.map_or(0, as_i64), probe);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config.max_response_bytes)?;
        let rows = &first_result_set::<T::Error>(&body)?.rows;

        let scanned = match rows.first() {
            Some(row) => text::<T::Error>(row, "scanned")?
                .parse::<i64>()
                .map_err(|_| NeonError::MissingColumn { column: "scanned" })?,
            None => 0,
        };
        let mut page = GroupPageBuilder::new(max_events);
        for row in rows {
            let (first, guard, event) = decode_group_row::<T::Error>(row)?;
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

/// One column of one row, as text, or the shape complaint.
fn text<'r, E>(row: &'r serde_json::Value, column: &'static str) -> Result<&'r str, NeonError<E>> {
    row.get(column)
        .and_then(serde_json::Value::as_str)
        .ok_or(NeonError::MissingColumn { column })
}

/// One row of the group read: its group, the group's guard if this is the
/// group's first row, and the event.
///
/// # Errors
///
/// What `decode_row` refuses, a group name that is not a position, a guard
/// column that is not base64, and [`NeonError::StoredGuard`] for a guard the
/// codec cannot read.
fn decode_group_row<E>(
    row: &serde_json::Value,
) -> Result<(SequencePosition, Option<AppendCondition>, ReplicatedEvent), NeonError<E>> {
    let stored = decode_row::<E>(row)?;
    let first = position_from_text::<E>(text::<E>(row, "write_group_first")?)?;
    let guard = match row.get("group_guard").and_then(serde_json::Value::as_str) {
        Some(encoded) if stored.position == first => Some(
            decode_guard(&decode_base64::<E>(encoded)?)
                .map_err(|source| NeonError::StoredGuard { first, source })?,
        ),
        _ => None,
    };
    let event = ReplicatedEvent::new(stored.id, stored.recorded_at, stored.event);
    Ok((first, guard, event))
}

/// The log in the writes that made it (SY-39): what a runner pushes from and a
/// `StoreSyncPeer` serves a pull from. The bare flavour, the only one this
/// store has, as its `IngestStore` is; only that name is in scope here
/// (constraint 4).
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
impl<T: SqlTransport> WriteGroupStore for NeonEventStore<T> {
    type Error = NeonError<T::Error>;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        self.read_write_groups(after, max_events).await
    }
}

#[cfg(test)]
mod tests;
