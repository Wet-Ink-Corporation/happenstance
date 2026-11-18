//! The removal record's schema and trigger, read raw: what SY-40's rule cannot
//! see from outside the port. No literal local position is compared; each is
//! read back from the store.

use happenstance_core::{Event, EventStore};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::event_store::CloudflareEventStore;
use crate::host::durable_object;
use crate::namespace::{TableNamespace, Tables};
use crate::sql_storage::{SqlStorage, SqlValue};

fn event(event_type: &str) -> Event {
    Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
}

/// Every schema object named `name`, as `(type, tbl_name)`.
fn objects_named(sql: &SqlStorage, name: &str) -> Vec<(String, String)> {
    let mut cursor = sql
        .exec(
            "SELECT type, tbl_name FROM sqlite_schema WHERE name = ?",
            &[SqlValue::Text(name.to_owned())],
        )
        .expect("the schema reads");
    let mut out = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the schema row decodes").values() {
            [SqlValue::Text(kind), SqlValue::Text(table)] => {
                out.push((kind.clone(), table.clone()));
            }
            other => panic!("an unexpected schema row: {other:?}"),
        }
    }
    out
}

/// The columns `table` has, in order, from `PRAGMA table_info`.
fn columns(sql: &SqlStorage, table: &str) -> Vec<String> {
    let mut cursor = sql
        .exec(&format!("PRAGMA table_info({table})"), &[])
        .expect("the pragma runs");
    let mut names = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the pragma row decodes").values() {
            [_, SqlValue::Text(name), ..] => names.push(name.clone()),
            other => panic!("an unexpected pragma row: {other:?}"),
        }
    }
    names
}

/// Every identity `table` records, in key order.
pub(crate) fn recorded(sql: &SqlStorage, table: &str) -> Vec<(Vec<u8>, i64)> {
    let mut cursor = sql
        .exec(
            &format!(
                "SELECT origin_store, origin_position FROM {table} \
                 ORDER BY origin_store, origin_position"
            ),
            &[],
        )
        .expect("the record reads");
    let mut out = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the record row decodes").values() {
            [SqlValue::Blob(store), SqlValue::Integer(position)] => {
                out.push((store.clone(), *position));
            }
            other => panic!("an unexpected record row: {other:?}"),
        }
    }
    out
}

/// `migrate` creates the record and its trigger, on the log's own event
/// table, for the unprefixed log and for a namespaced one, and a second and
/// third `migrate` add nothing.
///
/// Rejects a migration that names the unprefixed tables in a namespaced log,
/// whose trigger would never fire on the namespace's rows, and one that is not
/// idempotent, which throws on the second open.
#[wasm_bindgen_test]
async fn migrate_creates_the_removal_record_idempotently() {
    let sql = durable_object();
    let tenant = TableNamespace::new("tenant").expect("a valid namespace");
    for (store, tables) in [
        (CloudflareEventStore::new(sql.clone()), Tables::UNPREFIXED),
        (
            CloudflareEventStore::namespaced(sql.clone(), &tenant),
            Tables::namespaced(&tenant),
        ),
    ] {
        for _ in 0..3 {
            store.migrate().expect("migrate is idempotent");
        }

        assert_eq!(
            objects_named(&sql, &tables.event_removed),
            [("table".to_owned(), tables.event_removed.to_string())],
            "the record, once"
        );
        assert_eq!(
            columns(&sql, &tables.event_removed),
            ["origin_store", "origin_position"]
        );
        assert_eq!(
            objects_named(&sql, &tables.event_record_removal),
            [("trigger".to_owned(), tables.event.to_string())],
            "the trigger, once, on the log's own event table"
        );
    }
}

/// A raw `DELETE` through `SqlStorage::exec` records the identity of each
/// event it removes, own and foreign alike, keyed by the origin pair. A
/// second delete of a re-recorded identity is absorbed by the conflict clause.
///
/// Rejects a migration without the trigger, and a trigger keyed on the local
/// position, which a foreign identity does not carry.
#[wasm_bindgen_test]
async fn a_raw_delete_records_the_identity() {
    let sql = durable_object();
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("the schema applies");
    let own = store.store_id().expect("the identity reads");
    let local = store
        .append(&[event("Local")], None)
        .await
        .expect("an append lands");
    let foreign = [0xF0_u8; 16];
    sql.exec(
        "INSERT INTO event (event_type, data, tags, recorded_at, origin_store, origin_position) \
         VALUES ('Shipped', x'00', x'', 0, ?, 7)",
        &[SqlValue::Blob(foreign.to_vec())],
    )
    .expect("a foreign row");

    sql.exec("DELETE FROM event_tag", &[])
        .expect("the tag rows go");
    sql.exec("DELETE FROM event", &[]).expect("the rows go");
    // The same identity recorded a second way, then deleted again: absorbed.
    sql.exec(
        "INSERT INTO event (event_type, data, tags, recorded_at, origin_store, origin_position) \
         VALUES ('Shipped', x'00', x'', 0, ?, 7)",
        &[SqlValue::Blob(foreign.to_vec())],
    )
    .expect("the foreign row again, outside the port");
    sql.exec("DELETE FROM event", &[])
        .expect("a second delete of a recorded identity");

    let mut expected = vec![
        (
            own.to_bytes().to_vec(),
            i64::try_from(local.get()).expect("a position fits i64"),
        ),
        (foreign.to_vec(), 7),
    ];
    expected.sort();
    assert_eq!(recorded(&sql, "event_removed"), expected);
}

/// The rows the adapter deletes itself are never events, and the trigger's
/// `WHEN` records none of them: `append`'s compensating discard (ES-18), whose
/// rows carry no origin, is the case driven here.
///
/// Rejects a trigger with no `WHEN`, which would fail every such delete on
/// `NOT NULL` and with it the discard ES-18 relies on.
#[wasm_bindgen_test]
async fn a_row_that_is_not_an_event_records_nothing() {
    let sql = durable_object();
    CloudflareEventStore::new(sql.clone())
        .migrate()
        .expect("the schema applies");
    sql.exec(
        "INSERT INTO event (event_type, data, tags, recorded_at) VALUES ('Staged', x'00', x'', 0)",
        &[],
    )
    .expect("an unstamped row, as a batch writes before its stamp");

    sql.exec("DELETE FROM event WHERE origin_position IS NULL", &[])
        .expect("the discard runs");

    assert!(recorded(&sql, "event_removed").is_empty());
}
