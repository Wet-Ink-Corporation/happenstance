//! What the write-group family cannot see from outside: a row from before the
//! columns existed, a guard column the codec refuses, groups out of order, and
//! the plan. No literal local position: every one compared is read back.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, Event, EventStore, SequencePosition, Tags};
use happenstance_sync::WriteGroup;
use happenstance_sync::write_group::{GroupOrderError, GuardCodecError};
use wasm_bindgen_test::wasm_bindgen_test;

use super::groups_sql;
use crate::event_store::write_group::{assert_seeks, plan};
use crate::event_store::{CloudflareEventStore, CloudflareEventStoreError, encode_tags, migration};
use crate::host::durable_object;
use crate::namespace::Tables;
use crate::sql_storage::{SqlStorage, SqlValue};

const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

fn event(data: &str) -> Event {
    Event::new("Grouped", data.as_bytes().to_vec()).expect("a valid event type")
}

/// A fresh object with the current schema.
fn open() -> (SqlStorage, CloudflareEventStore) {
    let sql = durable_object();
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("the schema applies");
    (sql, store)
}

/// Runs `statement` raw, as an older build or a damaged object would have.
fn raw(sql: &SqlStorage, statement: &str) {
    sql.exec(statement, &[])
        .expect("the arrangement's SQL runs");
}

/// Each group's first, and its size.
fn shape(groups: &[WriteGroup]) -> Vec<(SequencePosition, usize)> {
    groups
        .iter()
        .map(|group| (group.first(), group.events().len()))
        .collect()
}

/// #420's legacy rule (item 43): two events a build before write groups wrote,
/// in a table with no write-group column, then `migrate`, then an append of
/// two. The old rows read as a group each, named by their own position; the
/// append reads as one group named by its first.
///
/// Rejects a read that treats `NULL` groups as one group, and one that skips
/// them.
#[wasm_bindgen_test]
async fn a_row_from_before_the_columns_reads_as_its_own_group() {
    let sql = durable_object();
    for statement in migration(&Tables::UNPREFIXED) {
        raw(&sql, &statement);
    }
    for _ in 0..2 {
        sql.exec(
            "INSERT INTO event (event_type, data, tags, recorded_at) \
             VALUES ('Legacy', x'00', ?, 0)",
            &[SqlValue::Blob(encode_tags(&Tags::default()))],
        )
        .expect("an old build's row");
    }
    raw(
        &sql,
        "UPDATE event SET origin_store = x'000102030405060708090a0b0c0d0e0f', \
         origin_position = position",
    );
    let store = CloudflareEventStore::new(sql.clone());
    store.migrate().expect("migrate adds the columns");
    let last = store
        .append(&[event("new-1"), event("new-2")], None)
        .await
        .expect("an append");

    let page = store
        .read_write_groups(None, WHOLE_LOG)
        .expect("a group read");

    let firsts: Vec<SequencePosition> = page.groups.iter().map(WriteGroup::first).collect();
    let [old_1, old_2, new] = firsts[..] else {
        panic!("expected three groups, got {:?}", shape(&page.groups));
    };
    assert!(old_1 < old_2 && old_2 < new && new < last);
    assert_eq!(shape(&page.groups), [(old_1, 1), (old_2, 1), (new, 2)]);
    let ids: Vec<SequencePosition> = page
        .groups
        .iter()
        .take(2)
        .flat_map(|group| group.events().iter().map(|event| event.id.position()))
        .collect();
    assert_eq!(ids, [old_1, old_2], "a legacy row is named by its position");
    let guards: Vec<Option<&AppendCondition>> = page.groups.iter().map(WriteGroup::guard).collect();
    assert_eq!(guards, [None, None, None]);
}

/// A guard column holding a format no build of the codec wrote stops the read
/// with the group it belongs to, rather than forwarding the group without the
/// guard it arrived under.
#[wasm_bindgen_test]
async fn a_guard_the_codec_cannot_read_stops_the_group_read() {
    let (sql, store) = open();
    let first = store
        .append(&[event("guarded")], None)
        .await
        .expect("an append");
    raw(
        &sql,
        &format!(
            "UPDATE event SET group_guard = x'09' WHERE position = {}",
            first.get()
        ),
    );

    let refused = store.read_write_groups(None, WHOLE_LOG);

    assert!(
        matches!(
            refused,
            Err(CloudflareEventStoreError::StoredGuard {
                first: named,
                source: GuardCodecError::UnknownFormat { found: 9, .. },
            }) if named == first
        ),
        "got {refused:?}"
    );
}

/// A `write_group` naming a group the scan already passed is refused with the
/// two groups, rather than paged as a group torn in two under one name. No
/// build writes one; a damaged or hand-edited object can.
#[wasm_bindgen_test]
async fn a_row_naming_a_group_already_passed_stops_the_group_read() {
    let (sql, store) = open();
    let a = store.append(&[event("a")], None).await.expect("an append");
    let b = store.append(&[event("b")], None).await.expect("an append");
    let c = store.append(&[event("c")], None).await.expect("an append");
    raw(
        &sql,
        &format!(
            "UPDATE event SET write_group = {} WHERE position = {}",
            a.get(),
            c.get()
        ),
    );

    let refused = store.read_write_groups(None, WHOLE_LOG);

    assert!(
        matches!(
            refused,
            Err(CloudflareEventStoreError::GroupOrder(GroupOrderError { first, after, .. }))
                if first == a && after == b
        ),
        "got {refused:?}"
    );
}

/// The group read must seek on the position and read in its order, never scan
/// or sort the log.
///
/// **Named wrong statement:** the same `SELECT` filtered on
/// `coalesce(write_group, position) > ?` alone, which no index serves; SQLite
/// plans `SCAN event`.
#[wasm_bindgen_test]
async fn the_group_read_seeks_on_the_position_without_a_sort() {
    let (sql, _store) = open();

    let plan = plan(
        &sql,
        &groups_sql(&Tables::UNPREFIXED),
        &[SqlValue::Integer(0), SqlValue::Integer(0)],
    );

    assert_seeks(&plan);
}
