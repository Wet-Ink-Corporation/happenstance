//! A consuming `exec`: `SqlStorage::exec` less the `to_binding` clone.
//!
//! `happenstance_cloudflare::SqlStorage::exec` takes `&[SqlValue]` and clones
//! every `Text` and `Blob` into a `worker::SqlStorageValue`
//! (`sql_storage.rs:81-82`, collected at `:271-272`) before `worker` copies the
//! bytes once more into a JS `Uint8Array` (`worker-0.8.5/src/sql.rs:126-130`).
//! [`Raw::exec`] takes the `worker` values by value, so the only copy left is
//! `worker`'s, which no shape of `append` can remove.
//!
//! **Everything else is mirrored on purpose, allocation for allocation**: the
//! column names read into an `Rc<[String]>`, the cursor put behind an `Rc`, the
//! row decoded into a `Vec` and wrapped with an `Rc`'d handle. A mirror that
//! skipped that bookkeeping would be faster for a reason that has nothing to do
//! with the binding, and `b1_is_b0_minus_the_binding_clones` is the test that
//! says it does not.
//!
//! The `worker::SqlStorage` is reached the way the adapter reaches it —
//! `worker::State::from(DurableObjectState)`, then `.storage().sql()`
//! (`sql_storage.rs`, `storage_from_durable_object_state`) — over a state
//! object built around the published [`SqlStorage::handle`], because
//! `worker::SqlStorage`'s own constructor is `pub(crate)`.

use std::rc::Rc;

use happenstance_cloudflare::worker::js_sys::{Number, Object, Reflect};
use happenstance_cloudflare::worker::wasm_bindgen::{JsCast, JsValue};
use happenstance_cloudflare::worker::{self, SqlStorageValue, worker_sys};
use happenstance_cloudflare::{JsHandle, SqlRow, SqlStorage, SqlValue};

use super::ArmError;
use crate::exact::safe_integer;

/// The `worker` binding under a published [`SqlStorage`].
#[derive(Debug, Clone)]
pub(super) struct Raw {
    sql: worker::SqlStorage,
}

/// A cursor, with the bookkeeping `happenstance_cloudflare::SqlCursor` keeps.
#[derive(Debug)]
pub(super) struct Cursor {
    cursor: Rc<worker::SqlCursor>,
    /// Read for the allocation it costs the real cursor, never consulted.
    _columns: Rc<[String]>,
}

impl Raw {
    /// The `worker::SqlStorage` the published `sql` wraps.
    pub(super) fn under(sql: &SqlStorage) -> Result<Self, ArmError> {
        let storage = Object::new();
        Reflect::set(&storage, &JsValue::from_str("sql"), sql.handle().as_js())
            .map_err(ArmError::Js)?;
        let state = Object::new();
        Reflect::set(&state, &JsValue::from_str("storage"), &storage).map_err(ArmError::Js)?;
        let state: worker::State = state
            .unchecked_into::<worker_sys::DurableObjectState>()
            .into();
        Ok(Self {
            sql: state.storage().sql(),
        })
    }

    /// Runs `statement`, moving `bound` into the JS call.
    pub(super) fn exec(
        &self,
        statement: &str,
        bound: Vec<SqlStorageValue>,
    ) -> Result<Cursor, ArmError> {
        let cursor = self.sql.exec(statement, bound).map_err(ArmError::Worker)?;
        let columns: Rc<[String]> = cursor.column_names().into();
        Ok(Cursor {
            cursor: Rc::new(cursor),
            _columns: columns,
        })
    }
}

impl Cursor {
    /// The next row, decoded as `SqlCursor::next_row` decodes it.
    pub(super) fn next_row(&self) -> Option<Result<SqlRow, ArmError>> {
        // A clone of a `worker::SqlCursor` is a handle onto the same JS cursor,
        // which is how the adapter reaches `Iterator::next` from behind an `Rc`.
        let mut cursor = (*self.cursor).clone();
        match Iterator::next(&mut cursor) {
            None => None,
            Some(Err(err)) => Some(Err(ArmError::Worker(err))),
            Some(Ok(row)) => Some(decode_row(&row)),
        }
    }
}

fn decode_row(row: &JsValue) -> Result<SqlRow, ArmError> {
    let object: &Object = row.unchecked_ref();
    let values = Object::values(object);
    let mut decoded = Vec::with_capacity(usize::try_from(values.length()).unwrap_or(0));
    for value in values.iter() {
        decoded.push(decode_value(&value)?);
    }
    Ok(SqlRow::new(JsHandle::new(row.clone()), decoded))
}

/// `sql_storage.rs`'s `decode_value`, in its order, for the two shapes these
/// statements return: `NULL` and an integer.
fn decode_value(value: &JsValue) -> Result<SqlValue, ArmError> {
    if value.is_null() || value.is_undefined() {
        return Ok(SqlValue::Null);
    }
    match value.as_f64() {
        Some(number) if Number::is_safe_integer(value) => safe_integer(number)
            .map(SqlValue::Integer)
            .ok_or(ArmError::Column("an integer outside the safe range")),
        _ => Err(ArmError::Column("a value these statements never return")),
    }
}
