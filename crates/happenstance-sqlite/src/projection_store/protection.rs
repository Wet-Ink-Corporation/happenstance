//! Protection from reset: PS-18's mechanism, held in the file.
//!
//! The port supplies the refusal ([`ResetError::Refused`]) and leaves *what* to
//! protect to the domain. This module is where the two meet in this adapter: the
//! domain names a projection through [`SqliteProjectionStore::protect_from_reset`],
//! the name is written into `projection_protection`, and
//! [`reset`](happenstance_core::ProjectionStore::reset) reads that table inside its
//! own `BEGIN IMMEDIATE`, before any of the caller's statements, so every holder of
//! the store meets the mark.
//!
//! The table holds a **mark**, not a policy. Which projection a regulator already
//! holds a hash chain for is still the domain's knowledge; the adapter only
//! remembers that it was told, in the one place every handle onto the file reads.
//!
//! [`ResetError::Refused`]: happenstance_core::ResetError::Refused

use happenstance_core::ProjectionId;
use rusqlite::{Connection, TransactionBehavior};

use super::{
    PROTECTION_SCHEMA_VERSION, SCHEMA_VERSION_KEY, SqliteProjectionStore,
    SqliteProjectionStoreError,
};

/// The table the marks live in. Mirrored in the [parent module's
/// documentation](super).
///
/// Created on **every** open, `IF NOT EXISTS`, inside `migrate`'s transaction:
/// an empty table is inert to a build that predates it, so creating it costs an
/// older build nothing. What an older build must not do is open a file that
/// carries a mark, and that is the schema marker's job, raised by
/// [`protect_locked`] rather than here.
///
/// The key is `ProjectionId::as_str()` under SQLite's default binary collation,
/// so the scope is the id byte for byte (PS-39): `Ledger` and `ledger` are two
/// projections, and protecting one says nothing about the other.
pub(super) const PROTECTION_TABLE: &str = "\
CREATE TABLE IF NOT EXISTS projection_protection (
    projection_id TEXT NOT NULL PRIMARY KEY
) WITHOUT ROWID;";

impl SqliteProjectionStore {
    /// Protects `id` from [`reset`](happenstance_core::ProjectionStore::reset):
    /// every later reset of it, through any handle onto this file, answers
    /// [`ResetError::Refused`](happenstance_core::ResetError::Refused) and applies
    /// none of its batch.
    ///
    /// The mark is written into the file, so it holds for a handle opened before
    /// this call, for a clone, for another process, and after every handle has
    /// closed. It guards `reset` only: `commit` on a protected projection still
    /// commits, because its own runner has to keep running. Protecting a
    /// projection that is already protected is not an error.
    ///
    /// The first protection a file carries raises its projection schema marker
    /// to 2 (see [`SCHEMA_VERSION`](super::SCHEMA_VERSION)), in the same
    /// transaction as the mark, so a build that predates protection refuses to open the file
    /// rather than reset a protected projection without knowing. The marker is
    /// read when a handle opens, so a handle an older build opened **before**
    /// that is not stopped: deploy this build everywhere before protecting.
    ///
    /// # Errors
    ///
    /// * [`SqliteProjectionStoreError::Sqlite`] if the write fails, including
    ///   `SQLITE_BUSY` once the busy timeout has passed.
    /// * [`SqliteProjectionStoreError::ConnectionPoisoned`],
    ///   [`Worker`](SqliteProjectionStoreError::Worker) or
    ///   [`NoRuntime`](SqliteProjectionStoreError::NoRuntime), as for every
    ///   method that hops onto a blocking thread.
    pub async fn protect_from_reset(
        &self,
        id: &ProjectionId,
    ) -> Result<(), SqliteProjectionStoreError> {
        // Owned because the blocking closure must be `'static`: the same clone,
        // for the same reason, as `reset`'s.
        let id = id.clone();
        self.in_blocking_task(move |connection| protect_locked(connection, &id))
            .await
    }

    /// Lifts `id`'s protection, so its next
    /// [`reset`](happenstance_core::ProjectionStore::reset) proceeds. Lifting a
    /// protection `id` does not have is not an error.
    ///
    /// The schema marker does not fall back: a file that has carried a mark stays
    /// at the version that knows what one is.
    ///
    /// # Errors
    ///
    /// As for [`protect_from_reset`](Self::protect_from_reset).
    pub async fn allow_reset(&self, id: &ProjectionId) -> Result<(), SqliteProjectionStoreError> {
        let id = id.clone();
        self.in_blocking_task(move |connection| {
            connection.execute(
                "DELETE FROM projection_protection WHERE projection_id = ?",
                [id.as_str()],
            )?;
            Ok(())
        })
        .await
    }

    /// Whether `id` is protected from reset.
    ///
    /// The other half of a refusal.
    /// [`ResetError::Refused`](happenstance_core::ResetError::Refused) carries no
    /// context on purpose: an operator learns *that* a reset was declined from
    /// the variant, and *which policy* declined it from the store that holds the
    /// policy. This is that read.
    ///
    /// # Errors
    ///
    /// As for [`protect_from_reset`](Self::protect_from_reset).
    pub async fn is_protected_from_reset(
        &self,
        id: &ProjectionId,
    ) -> Result<bool, SqliteProjectionStoreError> {
        let id = id.clone();
        self.in_blocking_task(move |connection| is_protected(connection, &id))
            .await
    }
}

/// Whether `id` carries a mark, read through `connection`.
///
/// `reset` calls it on its open transaction, under the write lock, which is what
/// makes a protection and a reset racing from two processes linearizable: both
/// take SQLite's single write lock, so one of them sees the other's outcome.
pub(super) fn is_protected(
    connection: &Connection,
    id: &ProjectionId,
) -> Result<bool, SqliteProjectionStoreError> {
    Ok(connection.query_row(
        "SELECT EXISTS(SELECT 1 FROM projection_protection WHERE projection_id = ?)",
        [id.as_str()],
        |row| row.get(0),
    )?)
}

/// One transaction: the mark, then the schema marker raised to the version that
/// knows what a mark is.
///
/// `v < ?` keeps the update from ever lowering a marker, so a later migration's
/// version survives a protection made after it.
fn protect_locked(
    connection: &mut Connection,
    id: &ProjectionId,
) -> Result<(), SqliteProjectionStoreError> {
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;
    transaction.execute(
        "INSERT OR IGNORE INTO projection_protection (projection_id) VALUES (?)",
        [id.as_str()],
    )?;
    transaction.execute(
        "UPDATE projection_meta SET v = ?1 WHERE k = ?2 AND v < ?1",
        rusqlite::params![PROTECTION_SCHEMA_VERSION, SCHEMA_VERSION_KEY],
    )?;
    transaction.commit()?;
    Ok(())
}
