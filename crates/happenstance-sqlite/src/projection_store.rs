//! SQLite-backed [`ProjectionStore`](happenstance::ProjectionStore).
//!
//! # Status: not implemented
//!
//! SQLite is the adapter that makes the checkpoint invariant easy to honour:
//! the read-model writes and the checkpoint update share one transaction, so
//! there is no window in which they can disagree. That is why it should be the
//! first projection adapter built — it validates the port before a harder
//! backend has to live with it.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE projection_checkpoint (
//!     projection_id TEXT    PRIMARY KEY,
//!     position      INTEGER NOT NULL
//! ) WITHOUT ROWID;
//! ```
//!
//! Read-model tables themselves are the application's business; this adapter
//! owns only the checkpoint and the transaction that carries it.

/// A SQLite-backed projection store.
///
/// # Status: not implemented
#[derive(Debug)]
#[non_exhaustive]
pub struct SqliteProjectionStore {}

/// How [`SqliteProjectionStore`] fails.
///
/// # Status: not implemented
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqliteProjectionStoreError {
    /// Placeholder variant; replaced by real failure modes on implementation.
    #[error("the SQLite projection store is not implemented yet")]
    Unimplemented,
}
