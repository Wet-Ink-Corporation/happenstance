//! Write groups (#420): which rows one `append` or one ingested group wrote,
//! recorded beside the rows so a runner and a peer can read the log back in
//! the writes that made it (SY-39).
//!
//! `happenstance-sqlite`'s design, copied, never depended on (`CLAUDE.md`,
//! dependency rule). Two columns: `write_group` holds the position of the first
//! row the write put in, the same value on every row of it, and `group_guard`
//! holds the guard an ingested group arrived under, on that first row only, in
//! `happenstance-sync`'s stored form. A row written before the columns existed
//! has `NULL` in both and reads as a group of its own: nothing recorded which
//! rows one call wrote, so one group per original append is unknowable.
//!
//! # Where the stamp is, and why not where SQLite's is
//!
//! SQLite stamps a group with one `UPDATE` after its rows are written, inside
//! the write's transaction. Here an `append` has no transaction: its
//! all-or-none rests on the one identity `UPDATE` that turns its rows into
//! events (`STAMPED`), and a second `UPDATE` after it could throw with the
//! batch already events and no group named. So the group rides in the
//! statements that already exist: the append's identity stamp sets
//! `write_group` to the batch's first position, and ingest's per-row foreign
//! stamp sets it, and the guard on the group's first landed row. A landed row
//! is written once, which a Durable Object, billing every row an `UPDATE`
//! writes, would otherwise charge twice.
//!
//! # The migration has no marker
//!
//! [`add_columns`] runs on every [`migrate`](super::CloudflareEventStore::migrate),
//! asks `PRAGMA table_info` which columns the log's event table has, and adds
//! each one it lacks. A marker in `store_meta` would let an older build refuse
//! the object, as SQLite's raised schema version does; this store never had
//! one, and an older build writing here writes `NULL` groups, which read as
//! groups of one rather than torn ones (the owner's D-1, #420). Each column is
//! probed on its own because a throw between the two `ALTER`s commits the
//! first: a Durable Object's turn commits when the handler returns normally.

use super::CloudflareEventStoreError;
use crate::namespace::Tables;
use crate::sql_storage::{SqlStorage, SqlValue};

/// The two columns, with the type each is added as, in the order they are
/// added. Mirrored in the [module documentation](super)'s schema block.
const COLUMNS: [(&str, &str); 2] = [("write_group", "INTEGER"), ("group_guard", "BLOB")];

/// Adds each write-group column `tables`' event table lacks.
///
/// `ALTER TABLE … ADD COLUMN` has no `IF NOT EXISTS`, which is why the table
/// is asked first. The probe's cursor is drained before any `ALTER` runs, so
/// no statement invalidates it.
///
/// # Errors
///
/// [`CloudflareEventStoreError::Sql`] if the probe or an `ALTER` throws, and
/// [`CloudflareEventStoreError::ColumnType`] if the probe's rows do not carry
/// a column name where `PRAGMA table_info` puts it.
pub(super) fn add_columns(
    sql: &SqlStorage,
    tables: &Tables,
) -> Result<(), CloudflareEventStoreError> {
    let mut present = [false; COLUMNS.len()];
    // The table name is rendered, not bound: a `PRAGMA` takes no parameters,
    // and a `TableNamespace` is validated to an identifier's alphabet.
    let mut cursor = sql.exec(&format!("PRAGMA table_info({})", tables.event), &[])?;
    while let Some(row) = cursor.next_row().transpose()? {
        let [_, SqlValue::Text(name), ..] = row.values() else {
            return Err(CloudflareEventStoreError::ColumnType {
                column: "table_info.name",
            });
        };
        for (seen, (column, _)) in present.iter_mut().zip(COLUMNS) {
            *seen |= name == column;
        }
    }
    for (seen, (column, kind)) in present.into_iter().zip(COLUMNS) {
        if !seen {
            sql.exec(
                &format!("ALTER TABLE {} ADD COLUMN {column} {kind}", tables.event),
                &[],
            )?;
        }
    }
    Ok(())
}

#[cfg(feature = "sync")]
mod read;

/// `EXPLAIN QUERY PLAN`'s detail lines for `statement` under `bindings`, on
/// the shim's SQLite.
#[cfg(all(test, target_arch = "wasm32"))]
pub(crate) fn plan(sql: &SqlStorage, statement: &str, bindings: &[SqlValue]) -> Vec<String> {
    let mut cursor = sql
        .exec(&format!("EXPLAIN QUERY PLAN {statement}"), bindings)
        .expect("the statement plans");
    let mut details = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the plan row decodes").values() {
            [.., SqlValue::Text(detail)] => details.push(detail.clone()),
            other => panic!("an unexpected plan row: {other:?}"),
        }
    }
    details
}

/// A plan that seeks on the position and neither scans nor sorts the log.
#[cfg(all(test, target_arch = "wasm32"))]
pub(crate) fn assert_seeks(plan: &[String]) {
    assert!(
        plan.iter()
            .any(|detail| detail.contains("USING INTEGER PRIMARY KEY")),
        "the statement does not seek on the position: {plan:?}"
    );
    assert!(
        !plan.iter().any(|detail| detail.starts_with("SCAN event")),
        "the statement scans the log: {plan:?}"
    );
    assert!(
        !plan.iter().any(|detail| detail.contains("TEMP B-TREE")),
        "the statement sorts the log: {plan:?}"
    );
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests;
