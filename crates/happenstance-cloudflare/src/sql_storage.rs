//! The Workers `SqlStorage` API, **bound** — with the four shape
//! characteristics the stand-in was built to hold, now checked against the real
//! thing rather than asserted about a model.
//!
//! Four properties are load-bearing, and every one of them is a property the
//! ports had never been checked against. Each one below now names what the
//! binding does, not what a model did:
//!
//! 1. **`exec` is synchronous.** Cloudflare's own documentation says "SQL
//!    queries using `ctx.storage.sql.exec()` complete synchronously" — the
//!    storage is co-located with the object. `worker::SqlStorage::exec` is a
//!    plain `fn` returning `Result<SqlCursor>`: no future, no connection to
//!    acquire. This is the one storage in the workspace for which
//!    [`EventStore::read`](happenstance_core::EventStore::read) being *not*
//!    `async` is free rather than awkward.
//! 2. **The cursor is not a snapshot.** Cloudflare: "Although a cursor object
//!    can technically be held across an `await`, it does not provide a stable
//!    snapshot of query results." A lazy stream is exactly a cursor held across
//!    awaits, so this is a capability limit rather than a type error. The limit
//!    is *preserved* here and answered in [`crate::event_store`] by ADR-0011's
//!    ceiling-and-page mechanism; this module's contribution is
//!    [`SqlError::CursorInvalidated`], which is reported rather than silently
//!    returning torn results.
//! 3. **Everything is `!Send` and `!Sync`.** Not by accident and not by
//!    inheritance: `worker` declares `unsafe impl Send for SqlStorage {}` and
//!    the same for its cursor (`worker-0.8.5/src/sql.rs`), so holding either of
//!    them bare would hand this crate `Send`-ness through an escape hatch its
//!    own `unsafe_code = "forbid"` denies it. Both are held through an [`Rc`],
//!    which is `!Send` for every payload.
//! 4. **The object is single-threaded and re-entrant.** A Durable Object runs
//!    one thread with an event loop, so a second `append` future can be created
//!    and polled while the first is suspended. That is what makes
//!    [`SqlError::AlreadyBorrowed`] a real variant rather than a defensive one,
//!    and it is why the generation counter below lives in a `RefCell` that is
//!    *tried* rather than borrowed.

use std::cell::RefCell;
use std::rc::Rc;

use worker::js_sys::{ArrayBuffer, Number, Object, Uint8Array};
use worker::wasm_bindgen::{JsCast, JsValue};

use crate::js::{JsHandle, JsThrow};

/// A value that can be bound into a statement or read out of a row.
///
/// Workers SQL accepts and returns exactly these: `null`, numbers, strings and
/// `ArrayBuffer`. There is no boolean and no date, which is why the schema in
/// [`crate::event_store`] stores positions as integers and payloads as blobs.
///
/// `worker::SqlStorageValue` carries a sixth, `Boolean`. It is not mirrored
/// here: SQLite has no boolean type and never returns one, so a variant for it
/// would be a shape the storage cannot produce. A `true`/`false` arriving from
/// the JS side is folded into [`SqlValue::Integer`], which is what SQLite would
/// have stored.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SqlValue {
    /// SQL `NULL`.
    Null,
    /// An integer. Workers SQL widens these through a JS number, so values
    /// above 2^53 are not representable and positions must stay below it. A
    /// stored integer that did exceed it arrives back as [`SqlValue::Real`],
    /// which is the symptom [`crate::event_store`] reports rather than narrows.
    Integer(i64),
    /// A floating-point number.
    Real(f64),
    /// Text.
    Text(String),
    /// An `ArrayBuffer`.
    Blob(Vec<u8>),
}

impl SqlValue {
    /// The binding `worker` accepts for this value.
    fn to_binding(&self) -> worker::SqlStorageValue {
        match self {
            Self::Null => worker::SqlStorageValue::Null,
            Self::Integer(value) => worker::SqlStorageValue::Integer(*value),
            Self::Real(value) => worker::SqlStorageValue::Float(*value),
            Self::Text(value) => worker::SqlStorageValue::String(value.clone()),
            Self::Blob(value) => worker::SqlStorageValue::Blob(value.clone()),
        }
    }
}

/// One row, as the cursor yields it: positional values plus the live object.
///
/// Carries the JS-side row it was read from, so a row keeps a handle onto the
/// value a caller could still interrogate — and so a row is `!Send` for the
/// same reason everything else here is.
#[derive(Debug, Clone)]
pub struct SqlRow {
    /// The JS object the values were read out of.
    handle: JsHandle,
    /// The decoded values, positionally.
    values: Vec<SqlValue>,
}

impl SqlRow {
    /// Builds a row from a JS row handle and its decoded values.
    #[must_use]
    pub fn new(handle: JsHandle, values: Vec<SqlValue>) -> Self {
        Self { handle, values }
    }

    /// The values, in column order.
    #[must_use]
    pub fn values(&self) -> &[SqlValue] {
        &self.values
    }

    /// The JS row this was decoded from.
    #[must_use]
    pub fn handle(&self) -> &JsHandle {
        &self.handle
    }
}

/// How the Workers SQL API fails.
///
/// # Why `Thrown` carries a live error rather than a `String`
///
/// This is the ES-6 fork, made concrete. Replacing [`JsThrow`] with
/// `StringifiedThrow` — the crate-private alternative `js` keeps — here would
/// make this enum,
/// [`CloudflareEventStoreError`](crate::event_store::CloudflareEventStoreError)
/// and every future over them `Send + Sync`, and would delete the only type in
/// this workspace that can fail such a bound. The crate documentation records
/// what the workspace loses by doing so, which — on the evidence gathered here
/// — is less than the specification assumed.
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

    /// A cursor was polled after another statement ran against the object.
    ///
    /// Cloudflare documents that a cursor held across an `await` does not
    /// provide a stable snapshot. An adapter that returns a lazy cursor-backed
    /// stream has to detect that rather than silently return torn results —
    /// which is why [`SqlStorage::exec`] bumps a generation counter and every
    /// [`SqlCursor`] remembers the one it opened at.
    #[error(
        "the SQL cursor was polled after another statement and no longer holds a stable snapshot"
    )]
    CursorInvalidated,

    /// The object's SQL storage limit was reached.
    #[error("the Durable Object's SQL storage limit was exceeded")]
    StorageLimitExceeded,
}

impl SqlError {
    /// Classifies a `worker::Error` thrown out of the SQL binding.
    ///
    /// Capacity exhaustion is separated from every other throw here, and only
    /// here, because the two are different facts for a caller: a full object is
    /// "retry somewhere else", a syntax error or a constraint violation is not.
    /// SQLite's own `SQLITE_FULL` text and the runtime's own phrasing are both
    /// matched, because the adapter sees whichever of the two the object
    /// surfaces.
    fn from_worker(error: worker::Error) -> Self {
        let rendered = error.to_string();
        if STORAGE_LIMIT_MARKERS
            .iter()
            .any(|marker| rendered.contains(marker))
        {
            return Self::StorageLimitExceeded;
        }
        Self::Thrown(JsThrow::from_error(error))
    }

    /// An adapter-side failure with no JS value behind it.
    ///
    /// `worker::Error::RustError` rather than an invented `JsValue`: there was no
    /// throw, and manufacturing one would put a fabricated value where
    /// [`JsThrow::thrown`](crate::js::JsThrow::thrown) promises a real one.
    pub(crate) fn internal(message: &str) -> Self {
        Self::Thrown(JsThrow::from_error(worker::Error::RustError(
            message.to_owned(),
        )))
    }
}

/// The texts a full Durable Object reports itself through.
///
/// Matched rather than switched on, because Workers exposes no numeric code —
/// the same finding that forces the constraint-violation classification in
/// [`crate::js`] to read text. `measured-store-limits` is where these are
/// re-checked against what the runtime actually says at the ceiling; until then
/// an unmatched exhaustion arrives as [`SqlError::Thrown`], which is a worse
/// message and not a wrong one.
const STORAGE_LIMIT_MARKERS: &[&str] =
    &["database or disk is full", "SQLITE_FULL", "storage limit"];

/// What a statement did to the object, tracked so a cursor can tell.
///
/// One counter, bumped by every `exec`. It is the whole of this module's
/// interior mutability, and it lives in a `RefCell` that is always *tried*: a
/// Durable Object is re-entrant, so a `borrow_mut` that panicked would take the
/// object down where an error would have been recoverable.
#[derive(Debug, Default)]
struct StorageState {
    /// Bumped by every statement executed against this storage.
    generation: u64,
}

/// A Durable Object's SQL storage.
///
/// Binds `worker::SqlStorage`, which is what `state.storage().sql()` returns
/// inside a Durable Object class. `Clone` aliases the same storage rather than
/// copying it — which is also the far end of the specification's "handle
/// multiplicity" axis.
#[derive(Debug, Clone)]
pub struct SqlStorage {
    /// The bound storage. `Rc` because `worker` declares `unsafe impl Send`
    /// on this type and this crate may not inherit that.
    sql: Rc<worker::SqlStorage>,
    /// The JS-side object, kept so callers can still interrogate it.
    handle: JsHandle,
    /// Shared across every clone and every cursor, which is what makes
    /// invalidation observable.
    state: Rc<RefCell<StorageState>>,
}

impl SqlStorage {
    /// Binds a Durable Object's SQL storage.
    ///
    /// Takes the handle by injection: in production it comes off
    /// `state.storage().sql()` inside a `#[durable_object]` class, and nothing
    /// in this crate may conjure one, because a storage minted from nothing is
    /// a second construction path no fixture covers.
    #[must_use]
    pub fn new(sql: worker::SqlStorage) -> Self {
        let handle = JsHandle::new(AsRef::<JsValue>::as_ref(&sql).clone());
        Self {
            sql: Rc::new(sql),
            handle,
            state: Rc::new(RefCell::new(StorageState::default())),
        }
    }

    /// Binds the SQL storage of the Durable Object `state` belongs to.
    ///
    /// The convenience a host reaches for: a `#[durable_object]` class receives
    /// a [`worker::State`], and this is the one line between it and an event
    /// store.
    #[must_use]
    pub fn from_state(state: &worker::State) -> Self {
        Self::new(state.storage().sql())
    }

    /// Runs `statement` with `bindings` and returns a cursor over its results.
    ///
    /// **Synchronous**, like the API it binds. Statements that write return an
    /// empty cursor rather than nothing, so `INSERT ... RETURNING` is the
    /// natural way to learn the position an append landed at.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::AlreadyBorrowed`] if the object re-entered while a
    /// statement was being issued; [`SqlError::StorageLimitExceeded`] if the
    /// object reports itself full; and [`SqlError::Thrown`] for every other
    /// throw — including a constraint violation, which is how an append
    /// condition implemented as a unique index reports a conflict.
    pub fn exec(&self, statement: &str, bindings: &[SqlValue]) -> Result<SqlCursor, SqlError> {
        let mut state = self
            .state
            .try_borrow_mut()
            .map_err(|_| SqlError::AlreadyBorrowed)?;

        let bound: Vec<worker::SqlStorageValue> =
            bindings.iter().map(SqlValue::to_binding).collect();
        let cursor = self
            .sql
            .exec(statement, bound)
            .map_err(SqlError::from_worker)?;

        state.generation = state.generation.saturating_add(1);
        Ok(SqlCursor::new(
            cursor,
            Rc::clone(&self.state),
            state.generation,
        ))
    }

    /// The storage's current size in bytes, as `sql.databaseSize` reports it.
    #[must_use]
    pub fn database_size(&self) -> u64 {
        u64::try_from(self.sql.database_size()).unwrap_or(u64::MAX)
    }

    /// The JS-side object.
    #[must_use]
    pub fn handle(&self) -> &JsHandle {
        &self.handle
    }
}

/// A cursor over one statement's results.
///
/// Binds `SqlStorageCursor`. It is an *iterator over live storage*, not a
/// snapshot: it holds no borrow of the object's state between advances, and it
/// re-checks on every advance that no other statement has run since it opened.
/// That check is what stops a live stream from silently returning torn results,
/// and it is the reason [`crate::event_store`] can page rather than hold one
/// cursor across an `await`.
#[derive(Debug, Clone)]
pub struct SqlCursor {
    /// The bound cursor. `Rc` for the same reason [`SqlStorage`] uses one.
    cursor: Rc<worker::SqlCursor>,
    /// The result set's column names, read once at open.
    columns: Rc<[String]>,
    /// The storage being iterated. Held as an `Rc`, not a `Ref`.
    state: Rc<RefCell<StorageState>>,
    /// The generation this cursor was opened at.
    opened_at: u64,
    /// Set once the cursor has yielded its last row or failed.
    exhausted: bool,
}

impl SqlCursor {
    /// Wraps a bound cursor over the storage that produced it.
    fn new(cursor: worker::SqlCursor, state: Rc<RefCell<StorageState>>, opened_at: u64) -> Self {
        let columns: Rc<[String]> = cursor.column_names().into();
        Self {
            cursor: Rc::new(cursor),
            columns,
            state,
            opened_at,
            exhausted: false,
        }
    }

    /// Advances the cursor, returning the next row or `None` at the end.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::AlreadyBorrowed`] if the object re-entered, or
    /// [`SqlError::CursorInvalidated`] if another statement ran against the
    /// storage since this cursor opened.
    pub fn next_row(&mut self) -> Option<Result<SqlRow, SqlError>> {
        if self.exhausted {
            return None;
        }

        if let Err(err) = self.still_valid() {
            self.exhausted = true;
            return Some(Err(err));
        }

        // `worker::SqlCursor` is `Clone` and a clone is a refcount bump on the
        // same JS cursor, so advancing a clone advances the cursor. That is the
        // only way to reach `Iterator::next`, which wants `&mut`, from behind
        // the `Rc` this type needs in order to stay `!Send`.
        let mut cursor = (*self.cursor).clone();
        match Iterator::next(&mut cursor) {
            None => {
                self.exhausted = true;
                None
            }
            Some(Err(err)) => {
                self.exhausted = true;
                Some(Err(SqlError::from_worker(err)))
            }
            Some(Ok(row)) => Some(decode_row(&row)),
        }
    }

    /// Whether the storage has stayed still under this cursor.
    ///
    /// # Errors
    ///
    /// Returns [`SqlError::AlreadyBorrowed`] if the object re-entered, or
    /// [`SqlError::CursorInvalidated`] if a statement ran since this cursor
    /// opened.
    pub fn still_valid(&self) -> Result<(), SqlError> {
        let state = self
            .state
            .try_borrow()
            .map_err(|_| SqlError::AlreadyBorrowed)?;
        if state.generation == self.opened_at {
            Ok(())
        } else {
            Err(SqlError::CursorInvalidated)
        }
    }

    /// The column names of the result set, in the order rows carry them.
    #[must_use]
    pub fn column_names(&self) -> &[String] {
        &self.columns
    }

    /// How many rows SQLite has read to serve this cursor so far, as
    /// `cursor.rowsRead` reports it. Billing depends on this, so an adapter
    /// that ignores it is an adapter with a surprise invoice.
    #[must_use]
    pub fn rows_read(&self) -> u64 {
        u64::try_from(self.cursor.rows_read()).unwrap_or(u64::MAX)
    }
}

/// Decodes one JS row object into positional values.
fn decode_row(row: &JsValue) -> Result<SqlRow, SqlError> {
    let object: &Object = row.unchecked_ref();
    let values = Object::values(object);
    let mut decoded = Vec::with_capacity(usize::try_from(values.length()).unwrap_or(0));
    for value in values.iter() {
        decoded.push(decode_value(&value)?);
    }
    Ok(SqlRow::new(JsHandle::new(row.clone()), decoded))
}

/// Decodes one JS value into the marshalling contract's five variants.
///
/// The order of the tests is the contract: `null`/`undefined` first because
/// every other test would accept them, strings before numbers because a string
/// is not a number, and the two binary shapes last because `dyn_into` clones.
/// An integer outside the JS safe range arrives as [`SqlValue::Real`] rather
/// than being narrowed — that is the 2^53 ceiling made visible instead of
/// silent, and [`crate::event_store`] is where it becomes a reported error.
fn decode_value(value: &JsValue) -> Result<SqlValue, SqlError> {
    if value.is_null() || value.is_undefined() {
        return Ok(SqlValue::Null);
    }
    if let Some(text) = value.as_string() {
        return Ok(SqlValue::Text(text));
    }
    if let Some(flag) = value.as_bool() {
        return Ok(SqlValue::Integer(i64::from(flag)));
    }
    if let Some(number) = value.as_f64() {
        if Number::is_safe_integer(value) {
            return Ok(SqlValue::Integer(safe_integer(number)));
        }
        return Ok(SqlValue::Real(number));
    }
    if let Ok(bytes) = value.clone().dyn_into::<Uint8Array>() {
        return Ok(SqlValue::Blob(bytes.to_vec()));
    }
    if let Ok(buffer) = value.clone().dyn_into::<ArrayBuffer>() {
        return Ok(SqlValue::Blob(Uint8Array::new(&buffer).to_vec()));
    }
    Err(SqlError::internal(
        "Workers SQL returned a value outside the marshalling contract",
    ))
}

/// Narrows an `f64` that `Number.isSafeInteger` has already vouched for.
///
/// The cast is exact by construction — the guard is what makes it so — and the
/// scoped allow is here rather than at the call site so that the guard and the
/// cast cannot drift apart.
#[allow(clippy::cast_possible_truncation)]
fn safe_integer(number: f64) -> i64 {
    number as i64
}

/// Binds the Durable Object storage a Workers runtime hands over as a raw JS
/// value.
///
/// `worker::State` is what a `#[durable_object]` class receives, and
/// `From<DurableObjectState>` is the public conversion into it. Exposed
/// `pub(crate)` rather than `pub` because the *supported* entry point is
/// [`SqlStorage::from_state`]: a caller who has a `State` should hand over the
/// `State`, not a `JsValue` that might be anything.
pub(crate) fn storage_from_durable_object_state(state: JsValue) -> SqlStorage {
    let state: worker::State = state
        .unchecked_into::<worker::worker_sys::DurableObjectState>()
        .into();
    SqlStorage::from_state(&state)
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{SqlError, SqlValue};
    use crate::host::durable_object;

    /// Property 4, as an assertion. A Durable Object is single-threaded but
    /// re-entrant: the state a statement bumps is reached through a `RefCell`
    /// that is *tried*, so a second statement issued while the first still
    /// holds it reports rather than panicking. The alternative — a plain
    /// `borrow_mut` — takes the whole object down.
    #[wasm_bindgen_test]
    fn reentrant_borrow_is_reported_not_panicked() {
        let sql = durable_object();
        let held = sql
            .state
            .try_borrow_mut()
            .expect("nothing else holds the state");

        let refused = sql.exec("SELECT 1 AS one", &[]);

        assert!(
            matches!(refused, Err(SqlError::AlreadyBorrowed)),
            "a re-entrant statement must be reported, not panicked: {refused:?}"
        );
        drop(held);
        assert!(
            sql.exec("SELECT 1 AS one", &[]).is_ok(),
            "and the storage must still work once the re-entry unwinds"
        );
    }

    /// Property 1, as an assertion: `exec` returns a cursor with nothing
    /// awaited. The signature is the proof — this test only shows the value
    /// arrives.
    #[wasm_bindgen_test]
    fn exec_is_synchronous_and_yields_rows() {
        let sql = durable_object();
        sql.exec("CREATE TABLE probe (a INTEGER, b TEXT, c BLOB)", &[])
            .expect("ddl");
        sql.exec(
            "INSERT INTO probe VALUES (?, ?, ?)",
            &[
                SqlValue::Integer(7),
                SqlValue::Text("seven".to_owned()),
                SqlValue::Blob(vec![1, 2, 3]),
            ],
        )
        .expect("insert");

        let mut cursor = sql.exec("SELECT a, b, c FROM probe", &[]).expect("select");
        assert_eq!(cursor.column_names(), ["a", "b", "c"]);
        let row = cursor
            .next_row()
            .expect("one row")
            .expect("the row decodes");
        assert_eq!(
            row.values(),
            [
                SqlValue::Integer(7),
                SqlValue::Text("seven".to_owned()),
                SqlValue::Blob(vec![1, 2, 3]),
            ]
        );
        assert!(cursor.next_row().is_none(), "and then the cursor is done");
    }

    /// Property 2, as an assertion. The cursor is not a snapshot, and this
    /// adapter refuses to pretend otherwise: a cursor polled after another
    /// statement ran reports [`SqlError::CursorInvalidated`] instead of
    /// returning rows from a result set that has moved.
    #[wasm_bindgen_test]
    fn a_cursor_polled_after_another_statement_is_invalidated() {
        let sql = durable_object();
        sql.exec("CREATE TABLE probe (a INTEGER)", &[])
            .expect("ddl");
        sql.exec("INSERT INTO probe VALUES (1), (2)", &[])
            .expect("insert");

        let mut cursor = sql.exec("SELECT a FROM probe", &[]).expect("select");
        assert!(cursor.next_row().is_some(), "the first row arrives");

        sql.exec("INSERT INTO probe VALUES (3)", &[])
            .expect("a second statement");

        let torn = cursor
            .next_row()
            .expect("the cursor answers rather than ending");
        assert!(
            matches!(torn, Err(SqlError::CursorInvalidated)),
            "a moved result set must be reported: {torn:?}"
        );
    }

    /// An integer SQLite stored above the JS safe range comes back as
    /// [`SqlValue::Real`] rather than as a narrowed [`SqlValue::Integer`].
    /// That is the 2^53 ceiling arriving as a visible symptom; turning it into
    /// a reported error is [`crate::event_store`]'s job.
    #[wasm_bindgen_test]
    fn an_integer_above_the_safe_range_is_not_narrowed() {
        let sql = durable_object();
        sql.exec("CREATE TABLE probe (a INTEGER)", &[])
            .expect("ddl");
        sql.exec("INSERT INTO probe VALUES (9007199254740993)", &[])
            .expect("insert");

        let mut cursor = sql.exec("SELECT a FROM probe", &[]).expect("select");
        let row = cursor.next_row().expect("one row").expect("it decodes");
        assert!(
            matches!(row.values(), [SqlValue::Real(_)]),
            "2^53 + 1 must not come back as an Integer: {:?}",
            row.values()
        );
    }

    /// The classifier keeps capacity exhaustion out of the generic throw arm.
    #[wasm_bindgen_test]
    fn a_full_object_is_not_a_generic_throw() {
        let full = SqlError::from_worker(worker::Error::RustError(
            "database or disk is full".to_owned(),
        ));
        assert!(matches!(full, SqlError::StorageLimitExceeded));

        let other = SqlError::from_worker(worker::Error::RustError("no such table: t".to_owned()));
        assert!(matches!(other, SqlError::Thrown(_)));
    }
}
