//! Reading the log back in write groups: the body of
//! [`CloudflareIngestStore`](crate::CloudflareIngestStore)'s `WriteGroupStore`
//! impl, which lives in `ingest.rs` with the rest of the `happenstance-sync`
//! surface.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, SequencePosition};
use happenstance_sync::write_group::{GroupPageBuilder, decode_guard};
use happenstance_sync::{GroupPage, ReplicatedEvent, Waiting};

use crate::event_store::{
    CloudflareEventStore, CloudflareEventStoreError, READ_COLUMNS, STAMPED, decode_position,
    decode_values, position_as_i64,
};
use crate::namespace::Tables;
use crate::sql_storage::{SqlRow, SqlValue};

/// Every event above a group's first, in position order, with its group and
/// guard: [`READ_COLUMNS`], then the group, then the guard.
///
/// The two `?` are both the `after` position, or 0 for the start of the log.
/// A group's first is never above its own rows, so `position > ?` keeps every
/// row the next condition keeps and lets the planner seek on the primary key;
/// the `coalesce` then drops the tail of the group `after` names, and reads a
/// row written before the columns existed as a group of its own. [`STAMPED`]
/// keeps out the rows a failed append left behind, as every event read here
/// does. The aliases keep the row object's keys distinct, which is how
/// `SqlStorage` hands columns back.
pub(super) fn groups_sql(tables: &Tables) -> String {
    format!(
        "SELECT {READ_COLUMNS}, coalesce(write_group, position) AS group_first, \
         group_guard FROM {} \
         WHERE position > ? AND coalesce(write_group, position) > ? AND {STAMPED} \
         ORDER BY position",
        tables.event
    )
}

/// How many columns [`groups_sql`] selects: [`READ_COLUMNS`]' eight and two.
const GROUP_COLUMNS: usize = 10;

impl CloudflareEventStore {
    /// Up to `max_events` events of whole write groups above `after`.
    ///
    /// One statement, stepped synchronously only as far as the page takes, so
    /// nothing is awaited while its cursor is live and no other statement can
    /// invalidate it. A group's rows are contiguous because this object has no
    /// second writer, so a scan in position order meets each group whole.
    ///
    /// # Errors
    ///
    /// The statement threw, a stored row does not decode, a stored guard does
    /// not ([`StoredGuard`](CloudflareEventStoreError::StoredGuard)), or a
    /// stored `write_group` names a group the scan had already passed, which
    /// only a damaged object holds
    /// ([`GroupOrder`](CloudflareEventStoreError::GroupOrder)).
    pub(crate) fn read_write_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, CloudflareEventStoreError> {
        let after = after.map_or(0, position_as_i64);
        let mut cursor = self.sql.exec(
            &groups_sql(&self.tables),
            &[SqlValue::Integer(after), SqlValue::Integer(after)],
        )?;
        let mut page = GroupPageBuilder::new(max_events);
        while let Some(row) = cursor.next_row().transpose()? {
            let (first, guard, event) = decode_group_row(&row)?;
            if page.offer(first, guard, event).is_break() {
                return Ok(page.finish(Waiting::Unknown)?);
            }
        }
        Ok(page.finish(Waiting::Nothing)?)
    }
}

/// One row of [`groups_sql`]: the group it belongs to, the group's guard if
/// this is the group's first row, and the event.
fn decode_group_row(
    row: &SqlRow,
) -> Result<(SequencePosition, Option<AppendCondition>, ReplicatedEvent), CloudflareEventStoreError>
{
    let values = row.values();
    let (Some((event, [first, guard])), GROUP_COLUMNS) =
        (values.split_last_chunk::<2>(), values.len())
    else {
        return Err(CloudflareEventStoreError::RowShape {
            expected: GROUP_COLUMNS,
            actual: values.len(),
        });
    };
    let stored = decode_values(event)?;
    let first = decode_position(first, "group_first")?;
    // Only the group's first row holds its guard; a later row's column is
    // `NULL`, and is not read.
    let guard = match guard {
        _ if stored.position != first => None,
        SqlValue::Null => None,
        SqlValue::Blob(bytes) => Some(
            decode_guard(bytes)
                .map_err(|source| CloudflareEventStoreError::StoredGuard { first, source })?,
        ),
        _ => {
            return Err(CloudflareEventStoreError::ColumnType {
                column: "group_guard",
            });
        }
    };
    let event = ReplicatedEvent::new(stored.id, stored.recorded_at, stored.event);
    Ok((first, guard, event))
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests;
