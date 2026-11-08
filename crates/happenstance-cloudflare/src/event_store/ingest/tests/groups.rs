//! Write groups on the ingest path (#420), read raw and through the port: an
//! ingested group is named by its first **landed** row, which alone holds the
//! guard, and its compensation joins it.

use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, Query, QueryItem, SequencePosition};
use happenstance_sync::{IngestGroup, IngestStore, WriteGroupStore};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{Opened, local, log, minted, open};
use crate::event_store::write_group::{assert_seeks, plan};
use crate::host::statements;
use crate::sql_storage::{SqlStorage, SqlValue};

/// A peer's identity, for the foreign events below.
const PEER: happenstance_core::StoreId = happenstance_core::StoreId::from_bytes([0x71; 16]);

const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

/// A position-free guard, as the write-group family's.
fn guard() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["Shipped"]).expect("a valid query"),
    ))
}

/// Each row raw: `(position, write_group, group_guard)`, in position order.
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

/// #420 item 43, I-2: a group whose head was delivered before lands from its
/// second event, so it is named by the second event's position, the first row
/// it actually wrote; the guard is stored on that row and no other; the
/// compensation joins the group; and the group reads back whole with its
/// guard.
///
/// Rejects naming the group by the skipped head's staged position (no row
/// holds it), storing the guard on every row, and stamping the compensation
/// as a group of its own.
#[wasm_bindgen_test]
async fn an_ingested_group_is_named_by_its_first_landed_row_and_holds_its_guard_there() {
    let Opened { sql, store, .. } = open();
    let delivered = minted(PEER, 1, 1, b"head", &[]);
    store
        .ingest(&[IngestGroup::new(core::slice::from_ref(&delivered), &[])])
        .await
        .expect("the head lands");
    let shipped = [
        delivered,
        minted(PEER, 2, 2, b"tail-1", &[]),
        minted(PEER, 3, 3, b"tail-2", &[]),
    ];
    let answer = [local("Compensated", b"answer", &[])];
    let condition = guard();

    store
        .ingest(&[IngestGroup::new(&shipped, &answer).with_guard(Some(&condition))])
        .await
        .expect("the group lands from its second event");

    let positions: Vec<SequencePosition> = log(&store).await.iter().map(|e| e.position).collect();
    let [held, tail_1, tail_2, compensation] = positions[..] else {
        panic!("four events expected: {positions:?}");
    };
    let stored = happenstance_sync::write_group::encode_guard(&condition).expect("it encodes");
    let as_i64 = |position: SequencePosition| i64::try_from(position.get()).expect("small");
    assert_eq!(
        stamped(&sql),
        [
            (
                as_i64(held),
                SqlValue::Integer(as_i64(held)),
                SqlValue::Null
            ),
            (
                as_i64(tail_1),
                SqlValue::Integer(as_i64(tail_1)),
                SqlValue::Blob(stored)
            ),
            (
                as_i64(tail_2),
                SqlValue::Integer(as_i64(tail_1)),
                SqlValue::Null
            ),
            (
                as_i64(compensation),
                SqlValue::Integer(as_i64(tail_1)),
                SqlValue::Null
            ),
        ]
    );

    let page = store
        .read_groups(None, WHOLE_LOG)
        .await
        .expect("a group read");
    let read: Vec<(SequencePosition, usize, Option<&AppendCondition>)> = page
        .groups
        .iter()
        .map(|group| (group.first(), group.events().len(), group.guard()))
        .collect();
    assert_eq!(read, [(held, 1, None), (tail_1, 3, Some(&condition))]);
}

/// The foreign stamp runs once per ingested row inside the call's
/// transaction, so it must seek on the position. Asked of the statement the
/// store actually issued.
///
/// **Named wrong statement:** the stamp bounded by `origin_position IS NULL`
/// alone; SQLite plans `SCAN event`.
#[wasm_bindgen_test]
async fn the_foreign_stamp_is_a_rowid_seek() {
    let Opened { sql, store, .. } = open();
    let shipped = minted(PEER, 1, 1, b"planned", &[]);
    store
        .ingest(&[IngestGroup::new(core::slice::from_ref(&shipped), &[])])
        .await
        .expect("the ingest lands");
    let stamp = statements(&sql)
        .into_iter()
        .find(|statement| statement.starts_with("UPDATE OR IGNORE event SET origin_store"))
        .expect("the ingest issued its stamp");

    let plan = plan(
        &sql,
        &stamp,
        &[
            SqlValue::Blob(vec![0; 16]),
            SqlValue::Integer(9),
            SqlValue::Integer(1),
            SqlValue::Null,
            SqlValue::Integer(1),
        ],
    );

    assert_seeks(&plan);
}
