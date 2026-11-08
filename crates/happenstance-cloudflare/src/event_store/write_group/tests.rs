//! The write-group columns and the append stamp, read raw: what the
//! write-group family cannot see from outside. No literal local position is
//! compared; every one is read back from the store.

use happenstance_core::{Event, EventStore};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::event_store::{CloudflareEventStore, migration};
use crate::host::{durable_object, statements};
use crate::namespace::{TableNamespace, Tables};
use crate::sql_storage::{SqlStorage, SqlValue};

fn event(event_type: &str) -> Event {
    Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
}

/// The schema a build before #420 applied: `migration`'s statements, and no
/// write-group column.
fn pre_write_group_schema(sql: &SqlStorage, tables: &Tables) {
    for statement in migration(tables) {
        sql.exec(&statement, &[]).expect("the old schema applies");
    }
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

/// How many `ALTER TABLE … ADD COLUMN` statements `sql` has been asked to run.
fn alters(sql: &SqlStorage) -> usize {
    statements(sql)
        .iter()
        .filter(|statement| statement.contains("ADD COLUMN"))
        .count()
}

/// How many times `column` appears among `table`'s columns.
fn occurrences(sql: &SqlStorage, table: &str, column: &str) -> usize {
    columns(sql, table)
        .iter()
        .filter(|name| name.as_str() == column)
        .count()
}

/// #420 item 42: an object whose log a build before write groups created
/// gains both columns on the first `migrate`, and a second and third `migrate`
/// add nothing, because `ALTER TABLE … ADD COLUMN` has no `IF NOT EXISTS`.
///
/// Rejects a migration that skips an existing table (`CREATE … IF NOT EXISTS`
/// alone), and one that re-runs the `ALTER` on every open, which throws on
/// the second.
#[wasm_bindgen_test]
async fn migrate_adds_both_columns_to_a_log_from_before_write_groups_once() {
    let sql = durable_object();
    pre_write_group_schema(&sql, &Tables::UNPREFIXED);
    assert_eq!(occurrences(&sql, "event", "write_group"), 0);
    let store = CloudflareEventStore::new(sql.clone());

    store.migrate().expect("the first migrate adds the columns");
    store.migrate().expect("a second migrate is a no-op");
    store.migrate().expect("and a third");

    assert_eq!(occurrences(&sql, "event", "write_group"), 1);
    assert_eq!(occurrences(&sql, "event", "group_guard"), 1);
    assert_eq!(alters(&sql), 2, "{:#?}", statements(&sql));
}

/// A migration that added `write_group` and stopped before `group_guard`
/// (a throw between the two `ALTER`s commits the first: a Durable Object's
/// turn commits when the handler returns normally) is completed by the next
/// `migrate`, not left half-applied for ever.
///
/// Rejects a probe of `write_group` alone deciding both columns.
#[wasm_bindgen_test]
async fn a_half_applied_migration_is_completed_by_the_next() {
    let sql = durable_object();
    pre_write_group_schema(&sql, &Tables::UNPREFIXED);
    sql.exec("ALTER TABLE event ADD COLUMN write_group INTEGER", &[])
        .expect("the first half applies");
    let before = alters(&sql);

    CloudflareEventStore::new(sql.clone())
        .migrate()
        .expect("migrate completes the schema");

    assert_eq!(occurrences(&sql, "event", "write_group"), 1);
    assert_eq!(occurrences(&sql, "event", "group_guard"), 1);
    assert_eq!(alters(&sql) - before, 1, "only the missing column is added");
}

/// Each namespaced log is migrated by its own store, and only its own: the
/// probe and the `ALTER` name the namespace's table.
///
/// Rejects a probe of the unprefixed `event` table, which would add nothing
/// to a namespaced log or throw on a missing table.
#[wasm_bindgen_test]
async fn namespaced_logs_gain_the_columns_per_namespace() {
    let sql = durable_object();
    let alpha = TableNamespace::new("alpha").expect("a valid namespace");
    let beta = TableNamespace::new("beta").expect("a valid namespace");
    pre_write_group_schema(&sql, &Tables::namespaced(&alpha));
    pre_write_group_schema(&sql, &Tables::namespaced(&beta));

    CloudflareEventStore::namespaced(sql.clone(), &alpha)
        .migrate()
        .expect("alpha migrates");

    assert_eq!(occurrences(&sql, "alpha_event", "write_group"), 1);
    assert_eq!(occurrences(&sql, "beta_event", "write_group"), 0);

    CloudflareEventStore::namespaced(sql.clone(), &beta)
        .migrate()
        .expect("beta migrates");

    assert_eq!(occurrences(&sql, "beta_event", "group_guard"), 1);
}

/// Each row an append wrote, raw: `(position, write_group, group_guard)`.
fn stamped(sql: &SqlStorage) -> Vec<(i64, SqlValue, SqlValue)> {
    let mut cursor = sql
        .exec(
            "SELECT position, write_group, group_guard FROM event ORDER BY position",
            &[],
        )
        .expect("the oracle reads");
    let mut rows = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the oracle row decodes").values() {
            [SqlValue::Integer(position), group, guard] => {
                rows.push((*position, group.clone(), guard.clone()));
            }
            other => panic!("an unexpected row: {other:?}"),
        }
    }
    rows
}

/// #420 item 43: every row of one append names the append's first position as
/// its write group, and none carries a guard (a local condition is
/// position-relative, which SY-6 makes unreplicable). Two appends are two
/// groups.
///
/// Rejects a stamp that names each row by its own position, and one that
/// names a second append by the first's.
#[wasm_bindgen_test]
async fn an_append_names_every_row_by_its_first_position() {
    let sql = durable_object();
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("the schema applies");

    store
        .append(&[event("A1"), event("A2"), event("A3")], None)
        .await
        .expect("the first append lands");
    store
        .append(&[event("B1")], None)
        .await
        .expect("the second append lands");

    let rows = stamped(&sql);
    let [(a1, ..), _, _, (b1, ..)] = rows[..] else {
        panic!("four rows expected: {rows:?}");
    };
    let expected: Vec<(i64, SqlValue, SqlValue)> = rows
        .iter()
        .map(|(position, ..)| {
            let group = if *position < b1 { a1 } else { b1 };
            (*position, SqlValue::Integer(group), SqlValue::Null)
        })
        .collect();
    assert_eq!(rows, expected);
}

/// The append's stamp runs on every write, so it must seek on the position
/// and never scan the log. Asked of the statement the store actually issued.
///
/// **Named wrong statement:** the stamp bounded by `origin_position IS NULL`
/// alone, which no index serves; SQLite plans `SCAN event`.
#[wasm_bindgen_test]
async fn the_append_stamp_is_a_rowid_seek() {
    let sql = durable_object();
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("the schema applies");
    store
        .append(&[event("Planned")], None)
        .await
        .expect("the append lands");
    let stamp = statements(&sql)
        .into_iter()
        .find(|statement| statement.starts_with("UPDATE event SET origin_store"))
        .expect("the append issued its stamp");

    let plan = super::plan(
        &sql,
        &stamp,
        &[
            SqlValue::Blob(vec![0; 16]),
            SqlValue::Integer(1),
            SqlValue::Integer(1),
        ],
    );

    super::assert_seeks(&plan);
}
