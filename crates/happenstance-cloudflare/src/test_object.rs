//! A Durable Object's SQL storage, stood up for this crate's own tests.
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
//! `worker`, and executed by SQLite. Nothing in `src/` is compiled differently
//! because these tests exist — the module is `#[cfg(all(test, target_arch =
//! "wasm32"))]` and the types it produces are the ones production uses.
//!
//! # What it is *not*, and who owns the difference
//!
//! It is not a Durable Object runtime. There is no eviction, no hibernation, no
//! event loop re-entering the object mid-`await`, and no real storage ceiling —
//! so it cannot answer "what is `MAX_EVENT_DATA_LEN` here" or "does an
//! acknowledged write survive a reopen". Those are
//! `durable-object-host-and-fixture` and `measured-store-limits`, which mount
//! the conformance suite on a real `workerd`. This module exists so that the
//! write and read paths are written against a real SQL engine in the slice that
//! writes them, instead of being written blind and first executed two
//! milestones later.
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

  const sql = {
    get databaseSize() { return scalar('PRAGMA page_count') * scalar('PRAGMA page_size'); },
    // Every statement the adapter issued, in order. Read by
    // `statements()` below, which is how a test counts *how many times* the
    // adapter asked a question rather than only what it got back — the
    // difference between one ceiling per read and one per query item.
    get issuedStatements() { return issued.slice(); },
    exec(query, ...bindings) {
      issued.push(query);
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

/// A fresh, isolated Durable Object's SQL storage.
///
/// One call is one object: a new in-memory database nothing else can see, which
/// is the isolation the conformance suite's own fixture contract asks for.
/// Cloning the returned [`SqlStorage`] aliases it, which is the second half of
/// the same contract — one instance, many handles.
///
/// # Panics
///
/// If the shim fails to evaluate, which means the test host is not Node or is
/// too old for `node:sqlite`. There is nothing to recover to, and a test that
/// silently ran against no storage would be worse than a failure.
pub(crate) fn durable_object() -> SqlStorage {
    let state: JsValue = js_sys::eval(DURABLE_OBJECT_STATE)
        .expect("the Durable Object shim evaluates on a Node host with `node:sqlite`");
    storage_from_durable_object_state(state)
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
pub(crate) fn statements(sql: &SqlStorage) -> Vec<String> {
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
