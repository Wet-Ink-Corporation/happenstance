//! The Durable Object host: one object's `state`, stood up so that a store can
//! be hung off it.
//!
//! # What this is, exactly
//!
//! A JavaScript object shaped like the Workers `DurableObjectState` the runtime
//! hands a `#[durable_object]` class, backed by **real SQLite** — Node's own
//! `node:sqlite`, which is the same engine a Durable Object runs. The adapter
//! reaches it through `worker::State::from(DurableObjectState)` →
//! `state.storage().sql()` → `worker::SqlStorage::exec`: the production path,
//! unmodified, with `worker`'s real `wasm-bindgen` externs in the middle.
//!
//! So what is doubled here is the *runtime*, never the adapter. Every statement
//! this crate's tests observe is rendered by the adapter, marshalled by
//! `worker`, and executed by SQLite; the types it produces are the ones
//! production uses.
//!
//! # A test-and-example surface, and not a second public API
//!
//! It is `pub`, and that is a visibility decision taken deliberately in the
//! crate root rather than an accident of refactoring. A conformance run needs
//! something to hang the store off, and the run lives in `tests/` — a **second
//! compilation unit**, from which a `#[cfg(test)]` module in `src/lib.rs`
//! cannot be named at all. That constraint is what moved this module out of
//! `#[cfg(test)]`. It is not a promise about the shape of a Durable Object
//! host, and the adapter itself stays a library type that any real
//! `#[durable_object]` class can hold — [`crate::CloudflareEventStore::new`] is
//! the one construction seam, and this module reaches it by exactly the call a
//! production class would make.
//!
//! It is *unconditional* rather than `#[cfg(target_arch = "wasm32")]` for the
//! reason the crate documentation gives about the host build generally: every
//! `worker` binding links off-target and resolves to a `wasm-bindgen` stub that
//! panics when called. So this module compiles everywhere and
//! [`DurableObjectHost::new`] can only *run* where a JavaScript heap exists,
//! which is the honest shape — a fixture that cannot connect is a broken test
//! environment, and it says so by panicking rather than by not existing.
//!
//! # What it is *not*, and who owns the difference
//!
//! It is not the whole Durable Object runtime. There is no eviction, no
//! hibernation, no event loop re-entering the object mid-`await`, and no
//! platform storage ceiling of its own. What it **does** model, because the
//! conformance suite needs both and both are properties of the *object* rather
//! than of the isolate, is a **reopen** — see [`DurableObjectHost::storage`] —
//! and a **fault armed for one statement** — see [`arm_throw_after`].
//!
//! # Why `js_sys::eval` rather than a `#[wasm_bindgen]` snippet
//!
//! This workspace sets `unsafe_code = "forbid"`, and a `#[wasm_bindgen]`
//! `extern` block expands to `unsafe` code in the crate that writes it. Every
//! JS binding this crate uses therefore has to come from `worker`, `js-sys` or
//! `web-sys`, which write their own. `js_sys::eval` is an ordinary safe
//! function, so the shim is data rather than a new binding — and
//! `process.getBuiltinModule` reaches `node:sqlite` from an indirect `eval`,
//! where `require` is not in scope.

use worker::js_sys;
use worker::wasm_bindgen::{JsCast, JsValue};

use crate::js::JsHandle;
use crate::sql_storage::{SqlStorage, storage_from_durable_object_state};

/// One Durable Object's `state`, as JavaScript.
///
/// The shape is dictated by `worker-sys`, not chosen: `storage` and `sql` are
/// declared there as **getters** and `exec` as a `variadic` method, so the
/// object has to present exactly that or the binding reads `undefined`.
///
/// Two details make it faithful rather than merely working. Integers are read
/// as `BigInt` and then widened through `Number`, which is what a Durable
/// Object does and is the entire origin of the 2^53 ceiling — a stored
/// `9007199254740993` comes back as `9007199254740992`, and this crate reports
/// that rather than narrowing it. And `next()` yields row *objects* while
/// `raw()` yields arrays, which is the split the Workers cursor makes.
const DURABLE_OBJECT_STATE: &str = r"
(() => {
  const { DatabaseSync } = process.getBuiltinModule('node:sqlite');
  const db = new DatabaseSync(':memory:');

  // A Durable Object hands integers back through a JS number. `node:sqlite`
  // hands them back as BigInt when asked to, which is the only way to read a
  // value SQLite stored above 2^53 at all; widening it here reproduces exactly
  // what Workers does to it on the way out.
  const widen = (value) => (typeof value === 'bigint' ? Number(value) : value);

  const scalar = (statement) => {
    const row = db.prepare(statement).get();
    return row === undefined ? 0 : Number(Object.values(row)[0]);
  };

  function makeCursor(names, rows, written) {
    let index = 0;
    const asObject = (row) => {
      const out = {};
      for (let i = 0; i < names.length; i += 1) { out[names[i]] = row[i]; }
      return out;
    };
    return {
      get columnNames() { return names.slice(); },
      get rowsRead() { return rows.length; },
      get rowsWritten() { return written; },
      next() {
        if (index >= rows.length) { return { done: true, value: undefined }; }
        const row = rows[index];
        index += 1;
        return { done: false, value: asObject(row) };
      },
      toArray() { return rows.slice(index).map(asObject); },
      one() {
        if (rows.length !== 1) { throw new Error('Expected exactly one row'); }
        return asObject(rows[0]);
      },
      raw() {
        return {
          next() {
            if (index >= rows.length) { return { done: true, value: undefined }; }
            const row = rows[index];
            index += 1;
            return { done: false, value: row };
          },
          [Symbol.iterator]() { return this; },
        };
      },
    };
  }

  const issued = [];
  let armed = null;

  const sql = {
    get databaseSize() { return scalar('PRAGMA page_count') * scalar('PRAGMA page_size'); },
    // Arms throws on the next `times` statements whose text contains `match`,
    // defaulting to exactly one. It is how a test constructs the thrown value a
    // Durable Object's SQLite would produce and then delivers it down the
    // *production* path — through `worker`'s real bindings and this crate's
    // real classifier — without needing a `workerd` runner or a race to lose.
    //
    // The count exists because one failure is not the only reachable shape: a
    // storage ceiling that fails an `INSERT` fails the compensating `DELETE`
    // beside it, and that pair is what `CloudflareEventStoreError::PartialBatch`
    // reports. A single-shot arming can never construct it.
    //
    // `skip` is the fourth parameter and the one CF-39 needs: a mid-batch fault
    // is a fault on the *k*-th write of a batch, not on the first, and a batch
    // whose very first row never lands is not a partial write anybody has to
    // undo. Skipping `skip` matching statements and then throwing is what puts
    // rows on the ground before the fault arrives.
    armThrow(match, message, times, skip) {
      armed = {
        match,
        message,
        left: times === undefined ? 1 : Number(times),
        skip: skip === undefined ? 0 : Number(skip),
      };
    },
    // Every statement the adapter issued, in order. Read by
    // `statements()` below, which is how a test counts *how many times* the
    // adapter asked a question rather than only what it got back — the
    // difference between one ceiling per read and one per query item.
    get issuedStatements() { return issued.slice(); },
    exec(query, ...bindings) {
      issued.push(query);
      if (armed !== null && query.includes(armed.match)) {
        if (armed.skip > 0) {
          armed.skip -= 1;
        } else {
          const message = armed.message;
          armed.left -= 1;
          if (armed.left <= 0) { armed = null; }
          throw new Error(message);
        }
      }
      let statement;
      try {
        statement = db.prepare(query);
      } catch (err) {
        // A Durable Object's `exec` accepts several statements at once; a
        // prepared statement does not. Falling back keeps the binding honest
        // for DDL that arrives as one blob.
        db.exec(query);
        return makeCursor([], [], 0);
      }
      statement.setReadBigInts(true);
      statement.setReturnArrays(true);
      const rows = statement.all(...bindings).map((row) => row.map(widen));
      const names = statement.columns().map((column) => column.name);
      const written = scalar('SELECT changes() AS changes');
      return makeCursor(names, rows, written);
    },
  };

  const storage = { get sql() { return sql; } };
  return { get storage() { return storage; } };
})()
";

/// One Durable Object, held so that its storage can be bound more than once.
///
/// The type exists for the one thing a bare [`SqlStorage`] cannot express: the
/// difference between *the object* and *a handle onto it*. Cloning a
/// `SqlStorage` aliases the same binding — same shared invalidation state, same
/// cursor generation — which is exactly right for a second handle and exactly
/// wrong for a **reopen**, where the whole point is that process-level state is
/// thrown away and the durable rows are not.
///
/// Holding the object's `state` value and re-deriving a binding from it on
/// demand is what makes both operations available from one place, and it is the
/// mechanism a fixture answers `REOPEN` `SUPPORTED` on: a Durable Object's
/// storage outlives its isolate, so discarding handle state and reading the
/// store again is something this runtime genuinely does, while restarting the
/// isolate from inside a test is not.
///
/// `Clone` aliases the object rather than standing up a second one, for the same
/// reason `SqlStorage`'s does. Two *fixture instances* must share nothing, so a
/// fixture builds a fresh host with [`DurableObjectHost::new`]; two handles onto
/// one object are what a clone and [`storage`](Self::storage) are for.
#[derive(Debug, Clone)]
pub struct DurableObjectHost {
    /// The object's `state`, kept live behind an `Rc`.
    ///
    /// [`JsHandle`] and not a bare `JsValue`: a real `JsValue` is `Send + Sync`
    /// on `wasm32` builds without `atomics`, so holding one directly is how this
    /// crate's central `!Send` property gets restored by accident and without a
    /// diagnostic.
    state: JsHandle,
}

impl DurableObjectHost {
    /// Stands up a fresh, isolated Durable Object.
    ///
    /// One call is one object: a new in-memory database nothing else can see,
    /// which is the isolation the conformance suite's own fixture contract asks
    /// for.
    ///
    /// # Panics
    ///
    /// If the shim fails to evaluate — because there is no JavaScript heap here
    /// at all (an ordinary host build, where every `worker` binding is a
    /// panicking stub), because the host is not Node, or because it is too old
    /// for `node:sqlite`. There is nothing to recover to, and a test that
    /// silently ran against no storage would be worse than a failure.
    #[must_use]
    pub fn new() -> Self {
        let state: JsValue = js_sys::eval(DURABLE_OBJECT_STATE)
            .expect("the Durable Object shim evaluates on a Node host with `node:sqlite`");
        Self {
            state: JsHandle::new(state),
        }
    }

    /// Binds this object's SQL storage, through the production path.
    ///
    /// `worker::State::from(DurableObjectState)` → `state.storage().sql()`, the
    /// same two calls a `#[durable_object]` class makes. Each call returns a
    /// **fresh** binding onto the **same** durable storage, which is what makes
    /// this the reopen seam: the returned handle carries none of the previous
    /// one's process-level state, and the rows are untouched.
    ///
    /// # Panics
    ///
    /// If the retained value is not a Durable Object `state`, which can only
    /// happen if this type was constructed some other way.
    #[must_use]
    pub fn storage(&self) -> SqlStorage {
        storage_from_durable_object_state(self.state.as_js().clone())
    }
}

impl Default for DurableObjectHost {
    fn default() -> Self {
        Self::new()
    }
}

/// A fresh, isolated Durable Object's SQL storage.
///
/// [`DurableObjectHost::new`] followed by [`DurableObjectHost::storage`], for
/// callers that want one handle and never a second binding. Cloning the returned
/// [`SqlStorage`] aliases it, which is the other half of the fixture contract —
/// one instance, many handles.
///
/// # Panics
///
/// [`DurableObjectHost::new`]'s, unchanged.
#[must_use]
pub fn durable_object() -> SqlStorage {
    DurableObjectHost::new().storage()
}

/// Arms one throw, on the next statement whose text contains `matching`.
///
/// This is how a test *constructs* a thrown value — `new Error(message)`, the
/// shape a Durable Object's SQLite produces — and then delivers it down the
/// production path: through `worker`'s real `wasm-bindgen` externs, into
/// [`SqlError::from_worker`](crate::sql_storage::SqlError), and out through this
/// crate's own classification. Nothing is mocked between the throw and the
/// caller, and no `workerd` runner is needed to observe it.
///
/// One arming is one throw: it disarms as it fires, so the statement that
/// follows behaves normally and a test can show the same store both failing and
/// succeeding.
///
/// # Panics
///
/// If the handle is not one of this module's shims, which means the caller built
/// the storage some other way.
pub fn arm_throw(sql: &SqlStorage, matching: &str, message: &str) {
    arm(sql, matching, message, 1, 0);
}

/// Arms `times` consecutive throws on statements whose text contains `matching`.
///
/// The generalisation of [`arm_throw`], and it exists for one reachable state
/// that a single throw cannot construct: a batch whose write fails **and** whose
/// compensating discard fails too, which is what
/// [`CloudflareEventStoreError::PartialBatch`](crate::CloudflareEventStoreError)
/// reports. Both statements name the same table, so one substring arms both.
///
/// # Panics
///
/// If the handle is not one of this module's shims, which means the caller built
/// the storage some other way.
pub fn arm_throws(sql: &SqlStorage, matching: &str, message: &str, times: u32) {
    arm(sql, matching, message, times, 0);
}

/// Arms one throw on the statement **after** `skip` matching ones have run.
///
/// The generalisation CF-39 needs, and it is not a nicety. A fixture's
/// `arm_mid_batch_fault` asks for the *k*-th write of a batch to fail, and a
/// fault that fires on the first write leaves no rows on the ground for the
/// store to have to undo — so the rule it feeds would be asking about an append
/// that never started rather than about atomicity. Arming after two of the
/// adapter's own `INSERT INTO event (` statements is what makes
/// `append_is_atomic_under_a_mid_batch_fault` a real question.
///
/// The thrown value is a real `new Error(message)` marshalled back through
/// `worker`'s own bindings, so the store classifies it exactly as it classifies
/// a Durable Object's SQLite refusing a statement: nothing is mocked between the
/// throw and the caller.
///
/// # Panics
///
/// If the handle is not one of this module's shims, which means the caller built
/// the storage some other way.
pub fn arm_throw_after(sql: &SqlStorage, matching: &str, message: &str, skip: u32) {
    arm(sql, matching, message, 1, skip);
}

/// The one call every arming above goes through.
///
/// `Function::apply` over a four-element array rather than `call3`, because the
/// shim's hook takes four arguments and `js_sys`'s positional helpers stop at
/// three.
fn arm(sql: &SqlStorage, matching: &str, message: &str, times: u32, skip: u32) {
    let hook = sql
        .handle()
        .property("armThrow")
        .expect("reading the arming hook does not throw")
        .expect("this module's shim always carries the hook");
    let hook: js_sys::Function = hook
        .as_js()
        .clone()
        .dyn_into()
        .expect("the arming hook is a function");
    let arguments = js_sys::Array::of4(
        &JsValue::from_str(matching),
        &JsValue::from_str(message),
        &JsValue::from_f64(f64::from(times)),
        &JsValue::from_f64(f64::from(skip)),
    );
    hook.apply(sql.handle().as_js(), &arguments)
        .expect("arming a throw does not itself throw");
}

/// Every statement this object has been asked to run, oldest first.
///
/// Read off the shim's own log through the public `SqlStorage::handle`, so a
/// test can assert on *how many times* the adapter asked a question — the
/// difference between one ceiling capture per `read` and one per `QueryItem` is
/// invisible in the rows that come back and obvious here.
///
/// # Panics
///
/// If the handle is not one of this module's shims, which means the caller built
/// the storage some other way.
pub fn statements(sql: &SqlStorage) -> Vec<String> {
    let log = sql
        .handle()
        .property("issuedStatements")
        .expect("reading the log does not throw")
        .expect("this module's shim always carries a log");
    log.as_js()
        .clone()
        .dyn_into::<js_sys::Array>()
        .expect("the log is an array")
        .iter()
        .filter_map(|value| value.as_string())
        .collect()
}
