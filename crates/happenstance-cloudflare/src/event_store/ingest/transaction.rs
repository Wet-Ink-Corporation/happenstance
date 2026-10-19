//! The object's synchronous transaction, reached through `js_sys`.
//!
//! `worker` 0.8.5 binds only the asynchronous KV `transaction`
//! (`worker-0.8.5/src/durable.rs:546`), so `ctx.storage.transactionSync` is
//! read off the storage object as a property and called as a plain
//! `js_sys::Function`. Every API on the way is safe, so `unsafe_code =
//! "forbid"` holds, and there is no `#[wasm_bindgen] extern` block, which would
//! expand to `unsafe` in this crate (`host.rs`'s module documentation).

use worker::js_sys;
use worker::wasm_bindgen::closure::Closure;
use worker::wasm_bindgen::{JsCast, JsValue};

use crate::event_store::CloudflareEventStoreError;
use crate::js::{JsHandle, JsThrow};
use crate::sql_storage::SqlError;

/// The property a Durable Object's storage carries its synchronous transaction
/// under.
const TRANSACTION_SYNC: &str = "transactionSync";

/// What the callback throws to make the primitive roll back. Never read back:
/// [`settle`] decides from the slot the callback filled, not from the value.
const ROLLED_BACK: &str = "happenstance: the ingest call failed, so its transaction rolls back";

/// What the callback throws if the primitive calls it a second time, rather
/// than running the body twice.
const CALLED_TWICE: &str = "happenstance: transactionSync called its callback twice";

/// The object's synchronous transaction: `storage.transactionSync`, resolved
/// once.
///
/// Built only from a storage object that has the function, so "an ingest store
/// that cannot transact" is unrepresentable (API-02): there is no runtime
/// fallback to an untransacted write.
///
/// `Clone` is two `Rc` bumps, and aliases the same object.
#[derive(Debug, Clone)]
pub(super) struct Transactions {
    /// The storage object, which is `this` for every call.
    storage: JsHandle,
    /// `storage.transactionSync`, checked to be a function.
    transaction_sync: JsHandle,
}

impl Transactions {
    /// Resolves `storage.transactionSync`.
    ///
    /// # Errors
    ///
    /// `Sql(SqlError::Thrown(..))` when reading the property throws, or when it
    /// is absent or not a function.
    pub(super) fn of(storage: JsHandle) -> Result<Self, CloudflareEventStoreError> {
        let transaction_sync = storage
            .property(TRANSACTION_SYNC)
            .map_err(SqlError::from)?
            .filter(|function| function.as_js().is_function())
            .ok_or_else(|| {
                SqlError::internal("this Durable Object's storage offers no transactionSync")
            })?;
        Ok(Self {
            storage,
            transaction_sync,
        })
    }

    /// Runs `body` as one transaction: committed when it returns `Ok`, rolled
    /// back when it returns `Err`.
    ///
    /// The closure handed to JavaScript is a `ScopedClosure` that **borrows**
    /// `body` and the result slot for the length of the call, so nothing is
    /// cloned, nothing is `'static` and no `Rc<RefCell<_>>` carries the result
    /// back (OWN-11). `_aborting` rather than `borrow_mut`: this crate's
    /// production code does not panic (H-03) and `wasm32`'s panic strategy is
    /// abort, so catching an unwind would buy nothing, and the `&mut` captures
    /// are not `UnwindSafe`.
    pub(super) fn run<T, R>(&self, body: impl FnOnce() -> Result<T, R>) -> Settled<T, R> {
        let Some(function) = self.transaction_sync.as_js().dyn_ref::<js_sys::Function>() else {
            return Settled::Primitive(
                SqlError::internal("transactionSync stopped being a function").into(),
            );
        };
        // An `Option`, so an `FnOnce` can sit inside the `FnMut` the binding
        // requires: a second call finds it taken and throws instead.
        let mut body = Some(body);
        let mut slot = None;
        let called = {
            let mut callback = || -> Result<(), JsValue> {
                let body = body.take().ok_or_else(|| JsValue::from_str(CALLED_TWICE))?;
                let outcome = body();
                let commits = outcome.is_ok();
                slot = Some(outcome);
                if commits {
                    Ok(())
                } else {
                    Err(JsValue::from_str(ROLLED_BACK))
                }
            };
            let closure =
                Closure::<dyn FnMut() -> Result<(), JsValue>>::borrow_mut_aborting(&mut callback);
            function
                .call1(self.storage.as_js(), closure.as_ref())
                .map(drop)
                .map_err(JsHandle::new)
        };
        settle(called, slot)
    }
}

/// How one `transactionSync` call ended, every pairing named.
#[derive(Debug)]
pub(super) enum Settled<T, R> {
    /// The body returned `Ok` and the primitive returned normally.
    Committed(T),
    /// The body returned `Err` and the primitive rethrew.
    RolledBack(R),
    /// The primitive misbehaved: it never ran the callback, it threw after the
    /// callback returned `Ok` (a commit that failed), or it returned normally
    /// from a callback that threw.
    Primitive(CloudflareEventStoreError),
}

/// The mapping [`Transactions::run`] applies, pure so every pairing is testable
/// without a transaction.
///
/// It reads only the slot and whether the call threw, never the thrown value,
/// so a JavaScript value the platform rethrows as-is cannot be misread.
fn settle<T, R>(called: Result<(), JsHandle>, slot: Option<Result<T, R>>) -> Settled<T, R> {
    match (called, slot) {
        (Ok(()), Some(Ok(committed))) => Settled::Committed(committed),
        (Err(_), Some(Err(refused))) => Settled::RolledBack(refused),
        (Err(thrown), Some(Ok(_)) | None) => {
            Settled::Primitive(SqlError::Thrown(JsThrow::new(&thrown)).into())
        }
        (Ok(()), Some(Err(_))) => Settled::Primitive(
            SqlError::internal("transactionSync returned normally from a callback that threw")
                .into(),
        ),
        (Ok(()), None) => Settled::Primitive(
            SqlError::internal("transactionSync returned without calling its callback").into(),
        ),
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests;
