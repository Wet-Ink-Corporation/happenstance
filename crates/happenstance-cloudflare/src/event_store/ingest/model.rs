//! The Node shim's model of `ctx.storage.transactionSync` (Decision D2).
//!
//! The shim in `crate::host` carries no `transactionSync`, and the published
//! bytes of that module are not changed for a test-only port: this installs
//! one on the shim's `storage` object, in test code. It is built over the
//! object's own SQL, `SAVEPOINT` / `ROLLBACK TO` / `RELEASE`, which the shim's
//! `exec` accepts and `workerd`'s refuses. That is the point of a model: the
//! adapter never issues those statements itself (W16), and the real primitive
//! is measured under `workerd` by `harness/workerd`'s transaction probe
//! (Decision D3), which is what this model is held against.
//!
//! Probed for the plan on SQLite 3.50.4: a rolled-back savepoint restores
//! `sqlite_sequence`, so a refused call leaves the next positions dense.
//!
//! Every function returns a `Result` rather than panicking, so the module
//! holds the production rules although it is only ever compiled for tests.

use worker::js_sys::{self, Array, Function};
use worker::wasm_bindgen::{JsCast, JsValue};

use crate::host::DurableObjectHost;
use crate::js::{JsHandle, JsThrow};
use crate::sql_storage::SqlError;

/// Opens the model's transaction.
pub(super) const SAVEPOINT: &str = "SAVEPOINT happenstance_transaction_sync";
/// Undoes everything since [`SAVEPOINT`], the `AUTOINCREMENT` counter included.
pub(super) const ROLLBACK: &str = "ROLLBACK TO happenstance_transaction_sync";
/// Closes the model's transaction.
pub(super) const RELEASE: &str = "RELEASE happenstance_transaction_sync";

/// The three statements the model issues, which a test filters out of the
/// shim's log to see what the adapter issued.
pub(super) const STATEMENTS: [&str; 3] = [SAVEPOINT, ROLLBACK, RELEASE];

/// The installer: `transactionSync(callback)` runs `callback` inside a
/// savepoint, releases it when the callback returns, and rolls it back and
/// rethrows the same value when it throws, as `workerd` documents its own.
/// `transactionSyncCalls` counts the calls, as a string so the count crosses
/// into Rust without a float.
const INSTALL: &str = r"
(function (storage, savepoint, rollback, release) {
  const sql = storage.sql;
  let calls = 0;
  storage.transactionSync = function (callback) {
    calls += 1;
    sql.exec(savepoint);
    let result;
    try {
      result = callback();
    } catch (thrown) {
      sql.exec(rollback);
      sql.exec(release);
      throw thrown;
    }
    sql.exec(release);
    return result;
  };
  Object.defineProperty(storage, 'transactionSyncCalls', { get() { return String(calls); } });
})
";

/// A thrown JavaScript value, as this crate's storage error.
fn thrown(value: JsValue) -> SqlError {
    SqlError::Thrown(JsThrow::new(&JsHandle::new(value)))
}

/// The shim's `storage` object.
///
/// # Errors
///
/// When reading it throws, or `host`'s state carries none.
pub(super) fn storage(host: &DurableObjectHost) -> Result<JsHandle, SqlError> {
    host.state()
        .property("storage")?
        .ok_or_else(|| SqlError::internal("the shim's state carries no storage"))
}

/// Installs the model on `host`'s storage.
///
/// # Errors
///
/// When the installer does not evaluate or throws, which means `host` is not
/// one of `crate::host`'s shims.
pub(super) fn install(host: &DurableObjectHost) -> Result<(), SqlError> {
    let installer: Function = js_sys::eval(INSTALL)
        .map_err(thrown)?
        .dyn_into()
        .map_err(|_| SqlError::internal("the model's installer is not a function"))?;
    installer
        .apply(
            &JsValue::UNDEFINED,
            &Array::of4(
                storage(host)?.as_js(),
                &JsValue::from_str(SAVEPOINT),
                &JsValue::from_str(ROLLBACK),
                &JsValue::from_str(RELEASE),
            ),
        )
        .map(drop)
        .map_err(thrown)
}

/// How many times the model's `transactionSync` has been called on `host`.
///
/// # Errors
///
/// When the model is not installed on `host`.
pub(super) fn calls(host: &DurableObjectHost) -> Result<u32, SqlError> {
    storage(host)?
        .property("transactionSyncCalls")?
        .and_then(|count| count.as_js().as_string())
        .and_then(|count| count.parse().ok())
        .ok_or_else(|| SqlError::internal("the model is not installed on this object"))
}
