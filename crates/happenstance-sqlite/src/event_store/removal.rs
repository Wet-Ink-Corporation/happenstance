//! The removal record (#483, #484, SY-40, ADR-0110): the identity of every
//! event deleted from `event`, written by a trigger, so that ingest can skip a
//! removed event offered again rather than take it back or refuse it.
//!
//! One table and one trigger, added by migration 3. The trigger is part of
//! the schema, so it fires for **every** client of the file: this crate, a
//! second `rusqlite` connection, the `sqlite3` shell, an operator's retention
//! script. An inherent `prune` method would fire only for callers who knew to
//! use it, and both bug reports deleted rows with a raw `DELETE`.
//!
//! # Why a removal is a skip and not a restore
//!
//! `DELETE` never rewinds `AUTOINCREMENT`: `sqlite_sequence` keeps its value,
//! so a deleted identity is never minted again (ES-38). A copy of it offered
//! later is therefore the very event the store once held, and skipping it is
//! exact. A restore from a backup copy rolls back the file whole, the record
//! with the allocator and the log, so an own event minted after the backup is
//! absent from the restored record and SY-38 still refuses it.
//!
//! # What the record does not cover
//!
//! * A row deleted **before** migration 3 ran: nothing was listening. An
//!   operator who knows what they removed can record it by hand, one row per
//!   identity: `INSERT INTO event_removed (origin_store, origin_position)
//!   VALUES (X'<the origin's 16 bytes>', <its origin position>)`.
//! * A legacy row with no origin (`UnstampedEvent`): the trigger records
//!   nothing, and no peer can hold an identity for it either.
//! * Deleting a row of `event_removed` itself re-opens that identity to
//!   re-admission. The only safe time to do so is once no peer can offer the
//!   identity again.
//!
//! The record grows by one row per removed event, about thirty bytes, and is
//! kept for ever (D-5); a removal still frees the payload, so the file shrinks
//! net.
//!
//! In a module of its own, as [`write_group`](super::write_group) is, so the
//! migration's gate is one function: [`migrate_to_3`].

use rusqlite::Connection;

use super::SCHEMA_VERSION_KEY;

/// The marker migration 2 raised a file to, and the one migration 3 starts
/// from.
const WRITE_GROUP_SCHEMA_VERSION: u32 = 2;

/// The marker migration 3 raises a file to.
const REMOVAL_SCHEMA_VERSION: u32 = 3;

/// Migration 3, verbatim. Mirrored in the [module documentation](super), which
/// `tests/migration.rs` compares against `sqlite_master`.
///
/// `WITHOUT ROWID`, because the key is the whole row and the probe is a point
/// lookup on it. The trigger is `AFTER DELETE`, so it records only a row that
/// was actually deleted, in the deleting transaction: a rolled-back `DELETE`
/// rolls its record back with it. The conflict clause names its target rather
/// than `OR IGNORE`, for `write_batch`'s reason: a `NOT NULL` violation is not
/// "already recorded" and must not be silenced as if it were.
pub(super) const MIGRATION_3: &str = "\
CREATE TABLE IF NOT EXISTS event_removed (
    origin_store    BLOB    NOT NULL,
    origin_position INTEGER NOT NULL,
    PRIMARY KEY (origin_store, origin_position)
) WITHOUT ROWID;
CREATE TRIGGER IF NOT EXISTS event_record_removal AFTER DELETE ON event
WHEN OLD.origin_store IS NOT NULL
BEGIN
    INSERT INTO event_removed (origin_store, origin_position)
    VALUES (OLD.origin_store, OLD.origin_position)
    ON CONFLICT (origin_store, origin_position) DO NOTHING;
END;";

/// Applies migration 3 to a file migration 2 has just been checked on, inside
/// `migrate`'s transaction.
///
/// A fresh file reaches here at 2, raised by `migrate_to_2` a statement
/// earlier, and takes the same path as a version-2 file: the table and the
/// trigger go in and the marker is raised to 3 in the same transaction. A file
/// at 3 or above is left alone; one above
/// [`SCHEMA_VERSION`](super::SCHEMA_VERSION) is refused afterwards, by
/// `read_identity`.
///
/// # Raising the marker
///
/// Raising it is what stops a build that predates the record from opening the
/// file (D-3, the #420 precedent). Such a build would ingest without
/// consulting the record, and so re-admit a removed foreign event or refuse a
/// removed own one, which is the bug this migration fixes.
///
/// # Errors
///
/// The driver failed.
pub(super) fn migrate_to_3(transaction: &Connection) -> rusqlite::Result<()> {
    let found: u32 = transaction.query_row(
        "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = ?",
        [SCHEMA_VERSION_KEY],
        |row| row.get(0),
    )?;
    if found != WRITE_GROUP_SCHEMA_VERSION {
        return Ok(());
    }
    transaction.execute_batch(MIGRATION_3)?;
    transaction.execute(
        "UPDATE store_meta SET v = ? WHERE k = ?",
        rusqlite::params![REMOVAL_SCHEMA_VERSION, SCHEMA_VERSION_KEY],
    )?;
    Ok(())
}

/// The head before an ingest wrote anything, and whether the record holds any
/// row at all: the first statement of `ingest_locked`, one round trip for
/// both.
///
/// The `EXISTS` is the fast path. A store that has never had an event removed
/// answers it from one empty B-tree and runs no probe for the rest of the
/// call, so the ingest of a store nobody prunes costs what it did before
/// migration 3.
#[cfg(feature = "sync")]
pub(super) const HEAD_AND_ANY_REMOVED_SQL: &str = "SELECT coalesce(max(position), 0), \
     EXISTS (SELECT 1 FROM event_removed) FROM event";

/// Whether one identity is recorded as removed: a point lookup on the
/// record's primary key.
#[cfg(feature = "sync")]
pub(super) const PROBE_SQL: &str =
    "SELECT EXISTS (SELECT 1 FROM event_removed WHERE origin_store = ?1 AND origin_position = ?2)";

/// The record as one ingest consults it: [`PROBE_SQL`] prepared once for the
/// whole call, or nothing at all when the record is empty.
///
/// The `Option` is the fast path made a type. With no removal recorded there is
/// no statement to run, so no event of the call can be probed by mistake.
/// Prepared per call rather than cached per connection because this crate
/// builds `rusqlite` without its `cache` feature, and one `prepare` per
/// ingest is noise beside the transaction it runs in. The statement borrows
/// the transaction's connection, which is also what keeps it from outliving
/// the write lock it must run under.
#[cfg(feature = "sync")]
pub(super) struct Probe<'c>(Option<rusqlite::Statement<'c>>);

#[cfg(feature = "sync")]
impl<'c> Probe<'c> {
    /// The probe for an ingest whose first statement answered `any_removed`.
    ///
    /// # Errors
    ///
    /// The driver failed to prepare the statement.
    pub(super) fn new(transaction: &'c Connection, any_removed: bool) -> rusqlite::Result<Self> {
        Ok(Self(if any_removed {
            Some(transaction.prepare(PROBE_SQL)?)
        } else {
            None
        }))
    }

    /// Whether `id` is recorded as removed from this store.
    ///
    /// An origin position past `i64::MAX` cannot be in the record, since
    /// SQLite could never have held it, so it answers `false` and
    /// `write_batch` refuses it as `OriginPositionOutOfRange`, as it did
    /// before the record existed.
    fn recorded(&mut self, id: happenstance_core::EventId) -> rusqlite::Result<bool> {
        let Some(statement) = self.0.as_mut() else {
            return Ok(false);
        };
        let Ok(position) = i64::try_from(id.position().get()) else {
            return Ok(false);
        };
        statement.query_row(
            rusqlite::params![&id.store().to_bytes()[..], position],
            |row| row.get(0),
        )
    }

    /// The rows `write_batch` should write for a group's foreign events: every
    /// one whose identity is not recorded as removed, in the order it arrived.
    ///
    /// An event filtered here never reaches the insert, so it burns no
    /// `AUTOINCREMENT` value and cannot be refused under SY-38: ingest treats
    /// it as held (SY-40). One it does not filter goes through the insert's
    /// `ON CONFLICT` as before, so a held event still dedupes on the origin
    /// pair's uniqueness (VT-8).
    ///
    /// # Errors
    ///
    /// The driver failed.
    pub(super) fn unrecorded<'g>(
        &mut self,
        events: &'g [happenstance_sync::ReplicatedEvent],
    ) -> rusqlite::Result<Vec<super::NewRow<'g>>> {
        let mut rows = Vec::with_capacity(events.len());
        for replicated in events {
            if self.recorded(replicated.id)? {
                continue;
            }
            rows.push(super::NewRow {
                event: &replicated.event,
                origin: super::Origin::Foreign {
                    id: replicated.id,
                    recorded_at: replicated.recorded_at,
                },
            });
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests;
