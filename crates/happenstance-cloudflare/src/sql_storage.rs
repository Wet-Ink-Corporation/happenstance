//! A stand-in for the Workers `SqlStorage` API, with its shape characteristics
//! intact.
//!
//! Four properties are load-bearing, and every one of them is a property the
//! ports have never been checked against:
//!
//! 1. **`exec` is synchronous.** Cloudflare's own documentation says "SQL
//!    queries using `ctx.storage.sql.exec()` complete synchronously" — the
//!    storage is co-located with the object. There is no future to await and no
//!    connection to acquire. This is the one storage in the workspace for which
//!    [`EventStore::read`](happenstance_core::EventStore::read) being *not*
//!    `async` is free rather than awkward.
//! 2. **The cursor is not a snapshot.** Cloudflare: "Although a cursor object
//!    can technically be held across an `await`, it does not provide a stable
//!    snapshot of query results." A lazy stream is exactly a cursor held across
//!    awaits, so this is a capability limit rather than a type error — see the
//!    crate documentation.
//! 3. **Everything is `!Send` and `!Sync`**, because everything is reached
//!    through a handle into a single-threaded JS heap.
//! 4. **The object is single-threaded and re-entrant.** A Durable Object runs
//!    one thread with an event loop, so a second `append` future can be created
//!    and polled while the first is suspended. That is what makes
//!    [`SqlError::AlreadyBorrowed`] a real variant rather than a defensive one.

use std::cell::RefCell;
use std::rc::Rc;

use crate::js::{JsHandle, JsThrow};

/// A value that can be bound into a statement or read out of a row.
///
/// Workers SQL accepts and returns exactly these: `null`, numbers, strings and
/// `ArrayBuffer`. There is no boolean and no date, which is why the schema in
/// [`crate::event_store`] stores positions as integers and payloads as blobs.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SqlValue {
    /// SQL `NULL`.
    Null,
    /// An integer. Workers SQL widens these through a JS number, so values
    /// above 2^53 are not representable and positions must stay below it.
    Integer(i64),
    /// A floating-point number.
    Real(f64),
    /// Text.
    Text(String),
    /// An `ArrayBuffer`.
    Blob(Vec<u8>),
}

/// One row, as `SqlStorageCursor::raw()` yields it: positional values.
///
/// Carries the JS-side array it was read from, so a row is `!Send` for the same
/// reason everything else here is.
#[derive(Debug, Clone)]
pub struct SqlRow {
    /// The JS array the values were read out of.
    handle: JsHandle,
    /// The decoded values, positionally.
    values: Vec<SqlValue>,
}

impl SqlRow {
    /// Builds a row from a JS array handle and its decoded values.
    #[must_use]
    pub fn new(handle: JsHandle, values: Vec<SqlValue>) -> Self {
        Self { handle, values }
    }

    /// The values, in column order.
    #[must_use]
    pub fn values(&self) -> &[SqlValue] {
        &self.values
    }

    /// The JS array this row was decoded from.
    #[must_use]
    pub fn handle(&self) -> &JsHandle {
        &self.handle
    }
}

/// How the Workers SQL API fails.
///
/// # Why `Thrown` carries a handle rather than a `String`
///
/// This is the ES-6 fork, made concrete. Replacing [`JsThrow`] with
/// [`StringifiedThrow`](crate::js::StringifiedThrow) here would make this enum,
/// [`CloudflareEventStoreError`](crate::event_store::CloudflareEventStoreError)
/// and every future over them `Send + Sync`. The crate documentation records
/// what the workspace loses by doing so, which — on the evidence gathered here —
/// is less than the specification assumed.
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum SqlError {
    /// `exec` threw. This is the common case and covers constraint violations,
    /// syntax errors and a full database alike; SQLite's own text arrives in the
    /// thrown `Error`'s `message`.
    #[error(transparent)]
    Thrown(#[from] JsThrow),

    /// The object's SQL state was already borrowed.
    ///
    /// A Durable Object is single-threaded but *re-entrant*: two `append`
    /// futures created from one handle and polled alternately both hold
    /// `&self`. Reporting this beats panicking, which is what `borrow_mut`
    /// would do.
    #[error("the Durable Object's SQL storage was already borrowed")]
    AlreadyBorrowed,

    /// A cursor was polled after the object yielded to the event loop.
    ///
    /// Cloudflare documents that a cursor held across an `await` does not
    /// provide a stable snapshot. An adapter that returns a lazy cursor-backed
    /// stream has to detect that rather than silently return torn results.
    #[error("the SQL cursor was polled after an await and no longer holds a stable snapshot")]
    CursorInvalidated,

    /// The object's SQL storage limit was reached.
    #[error("the Durable Object's SQL storage limit was exceeded")]
    StorageLimitExceeded,
}

/// The rows a [`SqlStorage`] holds.
///
/// Private, because the whole point of the stand-in is that the adapter reaches
/// storage only through [`SqlStorage::exec`].
#[derive(Debug, Default)]
struct StorageState {
    /// Rows in insertion order. Real storage is a B-tree; the difference does
    /// not show up in any signature.
    rows: Vec<SqlRow>,
}

/// A Durable Object's SQL storage.
///
/// Stands in for `worker::SqlStorage` / `ctx.storage.sql`. `Clone` is derived
/// because the JS object is a handle and cloning it aliases the same storage —
/// which is also the far end of the specification's "handle multiplicity" axis.
#[derive(Debug, Clone)]
pub struct SqlStorage {
    /// The JS-side `SqlStorage` object.
    handle: JsHandle,
    /// The rows behind it. `Rc` for the aliasing, `RefCell` for the interior
    /// mutability a `&self` API needs.
    state: Rc<RefCell<StorageState>>,
}

impl Default for SqlStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl SqlStorage {
    /// Opens an empty SQL storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            handle: JsHandle::new("[object SqlStorage]"),
            state: Rc::new(RefCell::new(StorageState::default())),
        }
    }

    /// Runs `statement` with `bindings` and returns a cursor over its results.
    ///
    /// **Synchronous**, like the API it models. Statements that write return an
    /// empty cursor rather than nothing, so `INSERT ... RETURNING` is the
    /// natural way to learn the position an append landed at.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::Thrown`] when the JS side throws — including on a
    /// constraint violation, which is how an append condition implemented as a
    /// unique index reports a conflict.
    pub fn exec(&self, _statement: &str, _bindings: &[SqlValue]) -> Result<SqlCursor, SqlError> {
        todo!("phase 9: SqlStorage::exec through the Workers binding")
    }

    /// The storage's current size in bytes, as `sql.databaseSize` reports it.
    #[must_use]
    pub fn database_size(&self) -> u64 {
        todo!("phase 9: sql.databaseSize")
    }

    /// The JS-side object.
    #[must_use]
    pub fn handle(&self) -> &JsHandle {
        &self.handle
    }
}

/// A cursor over one statement's results.
///
/// Stands in for `SqlStorageCursor`. It is an *iterator over live storage*, not
/// a snapshot, and it borrows nothing from the `RefCell` — it re-borrows on each
/// advance, which is what stops a live stream from deadlocking an append. That
/// is the same discipline `LocalMemoryEventStore` follows for a sharper reason
/// (`happenstance-testkit/tests/local_conformance.rs:76-82`).
#[derive(Debug)]
pub struct SqlCursor {
    /// The JS-side cursor object.
    handle: JsHandle,
    /// The storage being iterated. Held as an `Rc`, not a `Ref`.
    state: Rc<RefCell<StorageState>>,
    /// How far the cursor has advanced.
    offset: usize,
    /// Set once the cursor has yielded its last row.
    exhausted: bool,
}

impl SqlCursor {
    /// Builds a cursor over `storage`, positioned before the first row.
    #[must_use]
    pub fn new(handle: JsHandle, storage: &SqlStorage) -> Self {
        Self {
            handle,
            state: Rc::clone(&storage.state),
            offset: 0,
            exhausted: false,
        }
    }

    /// Advances the cursor, returning the next row or `None` at the end.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::AlreadyBorrowed`] if the object re-entered, or
    /// [`SqlError::CursorInvalidated`] if the storage moved under the cursor.
    pub fn next_row(&mut self) -> Option<Result<SqlRow, SqlError>> {
        if self.exhausted {
            return None;
        }
        todo!("phase 9: advance the SqlStorageCursor")
    }

    /// The column names of the result set.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::Thrown`] if reading `columnNames` throws.
    pub fn column_names(&self) -> Result<Vec<String>, SqlError> {
        todo!("phase 9: cursor.columnNames")
    }

    /// How many rows SQLite has read to serve this cursor so far, as
    /// `cursor.rowsRead` reports it. Billing depends on this, so an adapter
    /// that ignores it is an adapter with a surprise invoice.
    #[must_use]
    pub fn rows_read(&self) -> u64 {
        todo!("phase 9: cursor.rowsRead")
    }

    /// The JS-side cursor object.
    #[must_use]
    pub fn handle(&self) -> &JsHandle {
        &self.handle
    }

    /// How many rows the underlying storage currently holds.
    ///
    /// Only reachable from inside the crate; it exists so `state` is read
    /// somewhere and the field is not merely decoration.
    pub(crate) fn source_len(&self) -> Result<usize, SqlError> {
        self.state
            .try_borrow()
            .map(|state| state.rows.len())
            .map_err(|_| SqlError::AlreadyBorrowed)
    }

    /// The cursor's current offset into the result set.
    pub(crate) fn offset(&self) -> usize {
        self.offset
    }
}
