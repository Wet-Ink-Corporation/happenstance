//! Write groups (#420): which rows one `append` or one ingested group wrote,
//! recorded beside the rows so a runner and a peer can read the log back in
//! the writes that made it (SY-39).
//!
//! Two columns, added by migration 2. `write_group` holds the position of the
//! first row the write put in, the same value on every row of it; `group_guard`
//! holds the guard an ingested group arrived under, on that first row only, in
//! `happenstance-sync`'s stored form. A row written before migration 2 has
//! `NULL` in both and reads as a group of its own: nothing recorded which rows
//! one call wrote, so one group per original append is unknowable.
//!
//! In a module of its own, beside the writer it is called from, so the
//! migration's gate is one function: the choice between raising the marker and
//! leaving it is made in [`migrate_to_2`] and nowhere else.

use rusqlite::Connection;

use super::SCHEMA_VERSION_KEY;
#[cfg(feature = "sync")]
use super::SqliteEventStoreError;

/// The marker a file carries once migration 1 is applied: the schema before
/// write groups.
pub(super) const BASELINE_SCHEMA_VERSION: u32 = 1;

/// The marker migration 2 raises a file to.
const WRITE_GROUP_SCHEMA_VERSION: u32 = 2;

/// Migration 2, verbatim. Mirrored in the [module documentation](super).
///
/// `ALTER TABLE … ADD COLUMN` has no `IF NOT EXISTS`, which is why it is gated
/// on the marker rather than run on every connect as migration 1 is.
const MIGRATION_2: &str = "\
ALTER TABLE event ADD COLUMN write_group INTEGER;
ALTER TABLE event ADD COLUMN group_guard BLOB;";

/// Stamps every row a write put in with the write's group, and the group's
/// first row with its guard.
///
/// `?1` is the first position the write was given and `?2` the stored guard,
/// or `NULL`. Bounded below by `?1` so the planner seeks on the `INTEGER
/// PRIMARY KEY` instead of scanning `event` under the write lock, as the
/// identity stamp in `write_batch` is and for the same reason; the write lock
/// is what guarantees every row from `?1` up is this write's, and the
/// `write_group IS NULL` marker leaves an earlier group of the same ingest,
/// already stamped, alone.
pub(super) const STAMP_SQL: &str = "UPDATE event \
     SET write_group = ?1, group_guard = CASE WHEN position = ?1 THEN ?2 END \
     WHERE position >= ?1 AND write_group IS NULL";

/// Applies migration 2 to a file migration 1 has just been checked on, inside
/// `migrate`'s transaction.
///
/// A fresh file and a version-1 file take the same path: `migrate` has written
/// the baseline marker if there was none, so the marker reads 1 and the
/// columns go in, and it is raised to 2 in the same transaction. A file at 2
/// or above is left alone; one above [`SCHEMA_VERSION`](super::SCHEMA_VERSION)
/// is refused afterwards, by `read_identity`.
///
/// # Raising the marker
///
/// Raising it is what stops a build that predates write groups from opening
/// the file, and so from running a runner or a store peer over it that would
/// tear a group exactly as it did before. The alternative leaves the marker at
/// 1, gates the `ALTER` on `pragma_table_info` instead, and lets an older
/// build keep writing rows that read as groups of one. Changing to it is this
/// function and the constant, nothing else.
///
/// # Errors
///
/// The driver failed.
pub(super) fn migrate_to_2(transaction: &Connection) -> rusqlite::Result<()> {
    let found: u32 = transaction.query_row(
        "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = ?",
        [SCHEMA_VERSION_KEY],
        |row| row.get(0),
    )?;
    if found != BASELINE_SCHEMA_VERSION {
        return Ok(());
    }
    transaction.execute_batch(MIGRATION_2)?;
    transaction.execute(
        "UPDATE store_meta SET v = ? WHERE k = ?",
        rusqlite::params![WRITE_GROUP_SCHEMA_VERSION, SCHEMA_VERSION_KEY],
    )?;
    Ok(())
}

/// Records that every row from `first` up, which this transaction wrote, is
/// one write group, and stores `guard` on its first row.
///
/// # Errors
///
/// The driver failed.
pub(super) fn stamp(
    transaction: &Connection,
    first: i64,
    guard: Option<&[u8]>,
) -> rusqlite::Result<()> {
    transaction.execute(STAMP_SQL, rusqlite::params![first, guard])?;
    Ok(())
}

/// [`stamp`] for an ingested group: its guard encoded in
/// `happenstance-sync`'s stored form first.
///
/// # Errors
///
/// [`StoredGuard`](SqliteEventStoreError::StoredGuard) if the guard does not
/// encode, and the driver's error if the stamp fails.
#[cfg(feature = "sync")]
pub(super) fn stamp_ingested(
    transaction: &Connection,
    first: i64,
    guard: Option<&happenstance_core::AppendCondition>,
) -> Result<(), SqliteEventStoreError> {
    let stored = guard
        .map(happenstance_sync::write_group::encode_guard)
        .transpose()
        .map_err(|source| match super::position_from_row(first) {
            Ok(first) => SqliteEventStoreError::StoredGuard { first, source },
            Err(invalid) => invalid,
        })?;
    stamp(transaction, first, stored.as_deref())?;
    Ok(())
}

#[cfg(feature = "sync")]
mod read;

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use super::STAMP_SQL;
    use crate::event_store::SqliteEventStore;

    /// `EXPLAIN QUERY PLAN`'s detail lines.
    fn plan(connection: &Connection, sql: &str) -> Vec<String> {
        let mut statement = connection
            .prepare(&format!("EXPLAIN QUERY PLAN {sql}"))
            .expect("the statement prepares");
        let rows = statement
            .query_map(rusqlite::params![1, None::<Vec<u8>>], |row| {
                row.get::<_, String>(3)
            })
            .expect("the plan reads");
        rows.collect::<Result<_, _>>()
            .expect("every plan row reads")
    }

    /// The stamp runs under the write lock on every write, so it must seek on
    /// the primary key and never scan the log.
    ///
    /// **Named wrong statement, checked red:** the same `UPDATE` with only
    /// `WHERE write_group IS NULL`, which no index serves; SQLite plans
    /// `SCAN event` and the second assertion fails.
    #[test]
    fn the_group_stamp_is_a_rowid_seek() {
        let mut connection = Connection::open_in_memory().expect("an in-memory database");
        SqliteEventStore::migrate(&mut connection).expect("migrating must succeed");

        let rows = plan(&connection, STAMP_SQL);

        assert!(
            rows.iter()
                .any(|detail| detail.contains("USING INTEGER PRIMARY KEY")),
            "the stamp does not seek on the position: {rows:?}"
        );
        assert!(
            !rows.iter().any(|detail| detail == "SCAN event"),
            "the stamp scans the log under the write lock: {rows:?}"
        );
    }
}
