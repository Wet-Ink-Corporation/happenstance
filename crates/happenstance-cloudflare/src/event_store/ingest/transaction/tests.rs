//! `Transactions` against the Node shim's model of `transactionSync`, and
//! against three JavaScript stand-ins that misbehave in the three ways the
//! primitive can.

use happenstance_sync::{Classify, FailureClass};
use wasm_bindgen_test::wasm_bindgen_test;
use worker::js_sys;
use worker::wasm_bindgen::JsValue;

use super::{Settled, Transactions, settle};
use crate::event_store::CloudflareEventStoreError;
use crate::event_store::ingest::model;
use crate::host::{DurableObjectHost, statements};
use crate::js::JsHandle;
use crate::sql_storage::{SqlError, SqlStorage, SqlValue};

/// A storage object whose only member is `transactionSync`, written in
/// JavaScript as `body`.
fn stand_in(body: &str) -> Transactions {
    let storage = js_sys::eval(&format!("({{ transactionSync(callback) {{ {body} }} }})"))
        .expect("the stand-in evaluates");
    Transactions::of(JsHandle::new(storage)).expect("the stand-in carries the function")
}

/// A fresh object with the model installed, and a table to write to.
fn modelled() -> (DurableObjectHost, SqlStorage, Transactions) {
    let host = DurableObjectHost::new();
    let sql = host.storage();
    model::install(&host).expect("the model installs on the shim");
    sql.exec(
        "CREATE TABLE probe (n INTEGER PRIMARY KEY AUTOINCREMENT, v TEXT NOT NULL)",
        &[],
    )
    .expect("the probe table applies");
    let storage = model::storage(&host).expect("the shim's state carries a storage");
    let transactions = Transactions::of(storage).expect("the model is installed");
    (host, sql, transactions)
}

/// The rows of `probe`, and the counter `AUTOINCREMENT` hands them out from.
fn probe_state(sql: &SqlStorage) -> (Vec<String>, Vec<SqlValue>) {
    let mut rows = Vec::new();
    let mut cursor = sql
        .exec("SELECT v FROM probe ORDER BY n", &[])
        .expect("the probe reads");
    while let Some(row) = cursor.next_row() {
        if let [SqlValue::Text(v)] = row.expect("a row").values() {
            rows.push(v.clone());
        }
    }
    let mut cursor = sql
        .exec("SELECT seq FROM sqlite_sequence WHERE name = 'probe'", &[])
        .expect("the counter reads");
    let counter = cursor
        .next_row()
        .map(|row| row.expect("a row").values().to_vec())
        .unwrap_or_default();
    (rows, counter)
}

/// Inserts `v` into the probe table.
fn insert(sql: &SqlStorage, v: &str) -> Result<(), SqlError> {
    sql.exec(
        "INSERT INTO probe (v) VALUES (?)",
        &[SqlValue::Text(v.to_owned())],
    )
    .map(drop)
}

/// A thrown value, as the primitive would hand one back.
fn thrown() -> JsHandle {
    JsHandle::new(JsValue::from_str("thrown"))
}

/// `settle`'s six pairings, each named.
///
/// Rejects a mapping that trusts the primitive's return alone (it would call
/// a never-run callback committed) or the slot alone (it would call a failed
/// commit committed).
#[wasm_bindgen_test]
fn every_transaction_outcome_is_settled() {
    assert!(matches!(
        settle::<u8, &str>(Ok(()), Some(Ok(7))),
        Settled::Committed(7)
    ));
    assert!(matches!(
        settle::<u8, &str>(Err(thrown()), Some(Err("refused"))),
        Settled::RolledBack("refused")
    ));
    for misbehaved in [
        settle::<u8, &str>(Err(thrown()), Some(Ok(7))),
        settle::<u8, &str>(Ok(()), Some(Err("refused"))),
        settle::<u8, &str>(Err(thrown()), None),
        settle::<u8, &str>(Ok(()), None),
    ] {
        let Settled::Primitive(error) = misbehaved else {
            panic!("a misbehaving primitive must be reported as one: {misbehaved:?}");
        };
        assert!(
            matches!(error, CloudflareEventStoreError::Sql(SqlError::Thrown(_))),
            "{error:?}"
        );
        assert_eq!(error.class(), FailureClass::Stop, "{error:?}");
    }
}

/// The three ways the primitive can misbehave, each a real JavaScript function
/// called through the real binding, and each with the body that exposes it:
/// a never-run callback with either body, a swallowed throw with a failing
/// body, and a failed commit with a succeeding one.
///
/// Rejects a `run` that believes the primitive's return over the slot.
#[wasm_bindgen_test]
fn a_misbehaving_primitive_is_reported_not_trusted() {
    let never = stand_in("return undefined;");
    let swallows = stand_in("try { callback(); } catch (thrown) {}");
    let fails_commit = stand_in("callback(); throw new Error('commit failed');");

    for (what, outcome) in [
        ("never calls back", never.run(|| Ok::<u8, u8>(1))),
        ("never calls back", never.run(|| Err::<u8, u8>(2))),
        ("swallows the throw", swallows.run(|| Err::<u8, u8>(2))),
        (
            "throws after a clean callback",
            fails_commit.run(|| Ok::<u8, u8>(1)),
        ),
    ] {
        let Settled::Primitive(error) = outcome else {
            panic!("a stand-in that {what} was trusted: {outcome:?}");
        };
        assert_eq!(error.class(), FailureClass::Stop, "{what}: {error:?}");
    }

    // The controls: where the stand-in has nothing to misbehave over, it is a
    // well-behaved primitive, so the four above are about the misbehaviour.
    assert!(matches!(
        swallows.run(|| Ok::<u8, u8>(1)),
        Settled::Committed(1)
    ));
    assert!(matches!(
        fails_commit.run(|| Err::<u8, u8>(2)),
        Settled::RolledBack(2)
    ));
}

/// The body runs exactly once, inside the call: a commit keeps what it wrote,
/// and a rollback takes every row and the counter with it.
///
/// Rejects a `run` that calls the body outside the primitive (the rollback
/// would keep the rows), and one that runs it twice.
#[wasm_bindgen_test]
fn a_body_commits_on_ok_and_rolls_back_on_err() {
    let (host, sql, transactions) = modelled();
    let issued_before = statements(&sql).len();
    let mut runs = 0;

    let committed = transactions.run(|| {
        runs += 1;
        insert(&sql, "kept")
    });
    assert!(matches!(committed, Settled::Committed(())), "{committed:?}");
    let after_commit = probe_state(&sql);
    assert_eq!(after_commit.0, ["kept"]);

    let rolled_back = transactions.run(|| {
        runs += 1;
        insert(&sql, "dropped-1")?;
        insert(&sql, "dropped-2")?;
        Err::<(), _>(SqlError::AlreadyBorrowed)
    });
    assert!(
        matches!(rolled_back, Settled::RolledBack(SqlError::AlreadyBorrowed)),
        "{rolled_back:?}"
    );
    assert_eq!(
        probe_state(&sql),
        after_commit,
        "the rows and the counter rolled back"
    );
    assert_eq!(runs, 2, "each body ran exactly once");
    assert_eq!(model::calls(&host).expect("the model is installed"), 2);
    let control: Vec<String> = statements(&sql)
        .split_off(issued_before)
        .into_iter()
        .filter(|statement| model::STATEMENTS.contains(&statement.as_str()))
        .collect();
    assert_eq!(
        control,
        [
            model::SAVEPOINT,
            model::RELEASE,
            model::SAVEPOINT,
            model::ROLLBACK,
            model::RELEASE
        ],
        "the model, and not the adapter, issued the transaction control"
    );
}

/// A storage without `transactionSync` is refused at construction, with no
/// statement issued: there is no fallback to an untransacted write.
#[wasm_bindgen_test]
fn a_storage_without_transaction_sync_is_refused_at_construction() {
    let host = DurableObjectHost::new();
    let sql = host.storage();
    let storage = model::storage(&host).expect("the shim's state carries a storage");
    let before = statements(&sql).len();

    let refused = Transactions::of(storage);

    assert!(
        matches!(refused, Err(CloudflareEventStoreError::Sql(_))),
        "{refused:?}"
    );
    assert_eq!(statements(&sql).len(), before, "no statement was issued");
}
