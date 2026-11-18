//! The removal record (#483, #484, SY-40, ADR-0110): the identity of every
//! event deleted from a log's `event` table, written by a trigger, so that
//! ingest can skip a removed event offered again rather than take it back or
//! refuse it.
//!
//! `happenstance-sqlite`'s migration 3, copied, never depended on
//! (`CLAUDE.md`, dependency rule), and rendered through [`Tables`] so a
//! [`TableNamespace`](crate::TableNamespace)'s log gets its own record under
//! its own names:
//!
//! ```sql
//! CREATE TABLE event_removed (
//!     origin_store    BLOB    NOT NULL,
//!     origin_position INTEGER NOT NULL,
//!     PRIMARY KEY (origin_store, origin_position)
//! ) WITHOUT ROWID;
//!
//! CREATE TRIGGER event_record_removal AFTER DELETE ON event
//! WHEN OLD.origin_store IS NOT NULL
//! BEGIN
//!     INSERT INTO event_removed (origin_store, origin_position)
//!     VALUES (OLD.origin_store, OLD.origin_position)
//!     ON CONFLICT (origin_store, origin_position) DO NOTHING;
//! END;
//! ```
//!
//! The trigger is part of the object's schema, so it fires for **every**
//! statement that deletes a row: a retention job's `sql.exec`, a harness, an
//! operator's script. An inherent `prune` method would fire only for callers
//! who knew to use it, and both bug reports deleted rows with a raw `DELETE`.
//!
//! # What it never records
//!
//! The adapter deletes rows of its own: `append`'s compensating discard
//! (ES-18) and ingest's discard of a skipped event's staged row. Both delete
//! only rows with no origin, which are not events (`STAMPED`), and the
//! trigger's `WHEN` passes those by, so the record holds identities that were
//! events and nothing else. The conflict clause names its target rather than
//! `OR IGNORE`, for the stamp's reason in SQLite's `write_batch`: a `NOT NULL`
//! violation is not "already recorded" and must not be silenced as if it
//! were.
//!
//! # Why a removal is a skip and not a restore
//!
//! `DELETE` never rewinds `AUTOINCREMENT`: `sqlite_sequence` keeps its value,
//! so a deleted identity is never minted again (ES-38). A copy of it offered
//! later is therefore the very event the object once held, and skipping it is
//! exact. A point-in-time restore rolls back every table in the object, the
//! record with the allocator and the log, so an own event minted after the
//! bookmark is absent from the restored record and SY-38 still refuses it,
//! until [`remint_identity`](super::CloudflareEventStore::remint_identity)
//! makes it foreign.
//!
//! # No marker
//!
//! [`migrate`](super::CloudflareEventStore::migrate) creates both, `IF NOT
//! EXISTS`, on every call, as it adds the write-group columns (#420's D-1):
//! this store never had a schema marker, and an older build ignores the table.
//! Such a build ingests without consulting the record, so deploy this one to
//! every writer of an object before it serves replication again.
//!
//! # What the record does not cover
//!
//! * A row deleted **before** this build migrated the object: nothing was
//!   listening. An operator who knows what they removed can record it by hand,
//!   one row per identity: `INSERT INTO event_removed (origin_store,
//!   origin_position) VALUES (X'<the origin's 16 bytes>', <its origin
//!   position>)`, with the namespace's prefix on the table for a namespaced
//!   log.
//! * `ctx.storage.deleteAll()`: it takes the record with everything else and
//!   the next `migrate` mints a new store (VT-6), which is not a removal.
//! * Deleting a row of the record itself re-opens that identity to
//!   re-admission. The only safe time to do so is once no peer can offer the
//!   identity again.
//!
//! The record grows by one row per removed event, about thirty bytes, and is
//! kept for ever (D-5); a removal still frees the payload and its tag rows, so
//! the object shrinks net.

use super::CloudflareEventStoreError;
use crate::namespace::Tables;
use crate::sql_storage::SqlStorage;
#[cfg(feature = "sync")]
use crate::sql_storage::{SqlError, SqlValue};
#[cfg(feature = "sync")]
use happenstance_core::StoreId;

/// The record's two statements, the table and then the trigger, rendered for
/// `tables`. Mirrored in the module documentation.
///
/// Separate from the event store's own `migration` so that one stays the
/// schema a build before the record applied, which the write-group tests
/// stand an old object up from.
pub(super) fn migration(tables: &Tables) -> [String; 2] {
    let Tables {
        event,
        event_removed,
        event_record_removal,
        ..
    } = tables;
    [
        format!(
            "CREATE TABLE IF NOT EXISTS {event_removed} (\
                origin_store    BLOB    NOT NULL, \
                origin_position INTEGER NOT NULL, \
                PRIMARY KEY (origin_store, origin_position)) WITHOUT ROWID"
        ),
        format!(
            "CREATE TRIGGER IF NOT EXISTS {event_record_removal} AFTER DELETE ON {event} \
             WHEN OLD.origin_store IS NOT NULL \
             BEGIN \
             INSERT INTO {event_removed} (origin_store, origin_position) \
             VALUES (OLD.origin_store, OLD.origin_position) \
             ON CONFLICT (origin_store, origin_position) DO NOTHING; \
             END"
        ),
    ]
}

/// Creates the record and its trigger for `tables`' log, each only if it is
/// not there.
///
/// After the event table, which the trigger is on, and safe on every open.
///
/// # Errors
///
/// [`CloudflareEventStoreError::Sql`] if either statement throws.
pub(super) fn create(sql: &SqlStorage, tables: &Tables) -> Result<(), CloudflareEventStoreError> {
    for statement in migration(tables) {
        sql.exec(&statement, &[])?;
    }
    Ok(())
}

/// Whether the record holds any row at all: the fast path, one statement per
/// ingest call.
///
/// An object that has never had an event removed answers it from one empty
/// B-tree and runs no probe for the rest of the call, so its ingest reads one
/// row more than it did before the record and no more.
#[cfg(feature = "sync")]
pub(super) fn any_removed_sql(tables: &Tables) -> String {
    format!(
        "SELECT EXISTS (SELECT 1 FROM {}) AS any_removed",
        tables.event_removed
    )
}

/// Whether one identity is recorded as removed: a point lookup on the
/// record's primary key, which `the_removal_probe_seeks_the_primary_key`
/// asserts from the plan.
#[cfg(feature = "sync")]
pub(super) fn probe_sql(tables: &Tables) -> String {
    format!(
        "SELECT EXISTS (SELECT 1 FROM {} WHERE origin_store = ? AND origin_position = ?) \
         AS removed",
        tables.event_removed
    )
}

/// The record as one ingest call consults it: the probe rendered once for
/// the whole call, or nothing at all when the record is empty.
///
/// The `Option` is the fast path made a type, as in `happenstance-sqlite`'s
/// `Probe`. With no removal recorded there is no statement to run, so no event
/// of the call can be probed by mistake. Built inside the call's
/// `transactionSync`, and it borrows the store's storage, so it cannot outlive
/// the call whose writes it decides; a Durable Object has no other writer, so
/// nothing can record a removal between the probe and the write.
#[cfg(feature = "sync")]
pub(super) struct Probe<'s>(Option<Armed<'s>>);

/// The probe of a call that found the record non-empty.
#[cfg(feature = "sync")]
struct Armed<'s> {
    /// The object's storage, borrowed from the store for the call.
    sql: &'s SqlStorage,
    /// [`probe_sql`], rendered once for the call's tables.
    statement: String,
}

#[cfg(feature = "sync")]
impl<'s> Probe<'s> {
    /// Asks whether the record holds anything, and arms the probe only if it
    /// does.
    ///
    /// # Errors
    ///
    /// [`CloudflareEventStoreError::Sql`] if the statement throws, which a
    /// missing record does (`migrate` has not run on this build), and
    /// [`CloudflareEventStoreError::ColumnType`] or
    /// [`CloudflareEventStoreError::RowShape`] if its answer is not one
    /// boolean.
    pub(super) fn open(
        sql: &'s SqlStorage,
        tables: &Tables,
    ) -> Result<Self, CloudflareEventStoreError> {
        let armed = answer(
            sql,
            &any_removed_sql(tables),
            &[],
            "event_removed.any_removed",
        )?;
        Ok(Self(armed.then(|| Armed {
            sql,
            statement: probe_sql(tables),
        })))
    }

    /// Whether the identity `(store, position)` is recorded as removed.
    ///
    /// `position` is the origin position as the integer column holds it,
    /// already checked to bind exactly (`StoredOrigin`).
    ///
    /// # Errors
    ///
    /// [`open`](Self::open)'s, for the probe's own statement.
    pub(super) fn recorded(
        &self,
        store: StoreId,
        position: i64,
    ) -> Result<bool, CloudflareEventStoreError> {
        let Some(armed) = &self.0 else {
            return Ok(false);
        };
        answer(
            armed.sql,
            &armed.statement,
            &[
                // `worker`'s binding owns what it binds, as the stamp's do.
                SqlValue::Blob(store.to_bytes().to_vec()),
                SqlValue::Integer(position),
            ],
            "event_removed.removed",
        )
    }
}

/// The one boolean an `EXISTS` statement answers.
#[cfg(feature = "sync")]
fn answer(
    sql: &SqlStorage,
    statement: &str,
    bindings: &[SqlValue],
    column: &'static str,
) -> Result<bool, CloudflareEventStoreError> {
    let mut cursor = sql.exec(statement, bindings)?;
    let row = cursor
        .next_row()
        .transpose()?
        .ok_or_else(|| SqlError::internal("an EXISTS statement answered no row"))?;
    match row.values() {
        [SqlValue::Integer(0)] => Ok(false),
        [SqlValue::Integer(1)] => Ok(true),
        [_] => Err(CloudflareEventStoreError::ColumnType { column }),
        values => Err(CloudflareEventStoreError::RowShape {
            expected: 1,
            actual: values.len(),
        }),
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests;
