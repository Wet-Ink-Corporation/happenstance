//! What the ingest port answers about the store: membership, the watermark,
//! the plan the watermark runs on, and the identity.

use happenstance_core::{EventId, EventStore, StoreId};
use happenstance_sync::{IngestGroup, IngestStore, Watermark};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{Opened, connect, local, log, minted, open};
use crate::event_store::CloudflareEventStore;
use crate::event_store::ingest::watermark_sql;
use crate::namespace::Tables;
use crate::sql_storage::SqlValue;

/// A peer's identity.
const PEER: StoreId = StoreId::from_bytes([0x63; 16]);
/// A second peer's.
const OTHER: StoreId = StoreId::from_bytes([0x64; 16]);

/// `contains_event_id` answers for a foreign identity, by identity: not for
/// another origin at the same position, and not for this store's identity at
/// the local position the foreign event landed on.
#[wasm_bindgen_test]
async fn contains_event_id_answers_for_a_foreign_id() {
    let Opened { store, .. } = open();
    let own = store.store_id().await.expect("the identity reads");
    let sent = [minted(PEER, 6, 6, b"member", &[])];
    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");
    let landed = log(&store).await[0].position;

    assert!(store.contains_event_id(sent[0].id).await.expect("reads"));
    assert!(
        !store
            .contains_event_id(EventId::new(OTHER, sent[0].id.position()))
            .await
            .expect("reads")
    );
    assert!(
        !store
            .contains_event_id(EventId::new(own, landed))
            .await
            .expect("reads"),
        "the local position a foreign event landed on is not an identity this store minted"
    );
}

/// The watermark is the highest origin position held per origin, this
/// store's own included, and equals a fold over the log. A fresh store's is
/// empty.
#[wasm_bindgen_test]
async fn watermark_is_max_origin_position_per_origin() {
    let Opened { store, .. } = open();
    assert!(store.watermark().await.expect("reads").is_empty());
    store
        .append(&[local("Local", b"l1", &[])], None)
        .await
        .expect("lands");
    let sent = [
        minted(PEER, 4, 4, b"p4", &[]),
        minted(PEER, 11, 11, b"p11", &[]),
        minted(OTHER, 2, 2, b"o2", &[]),
    ];
    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("lands");
    store
        .append(&[local("Local", b"l2", &[])], None)
        .await
        .expect("lands");

    let watermark = store.watermark().await.expect("reads");

    let mut folded = Watermark::new();
    for row in log(&store).await {
        folded.advance(row.id.store(), row.id.position());
    }
    assert_eq!(watermark, folded);
    assert_eq!(watermark.len(), 3, "two peers and this store");
    assert_eq!(watermark.get(PEER), Some(sent[1].id.position()));
}

/// The watermark reads the origin index, `UNIQUE (origin_store,
/// origin_position)`, covering, with no sort.
///
/// Rejects a grouping by an expression, which the planner cannot answer from
/// the index.
#[wasm_bindgen_test]
fn watermark_reads_the_origin_index_without_a_sort() {
    let Opened { sql, .. } = open();

    let mut cursor = sql
        .exec(
            &format!("EXPLAIN QUERY PLAN {}", watermark_sql(&Tables::UNPREFIXED)),
            &[],
        )
        .expect("the plan reads");
    let mut plan = Vec::new();
    while let Some(row) = cursor.next_row() {
        let row = row.expect("a plan row");
        if let Some(SqlValue::Text(detail)) = row.values().last() {
            plan.push(detail.clone());
        }
    }

    assert!(
        plan.iter()
            .any(|step| step.contains("USING COVERING INDEX sqlite_autoindex_event_1")),
        "{plan:?}"
    );
    assert!(
        plan.iter().all(|step| !step.contains("TEMP B-TREE")),
        "{plan:?}"
    );
}

/// `IngestStore::store_id` reads the meta row on every call, so a re-mint
/// through another handle reaches this one.
///
/// Rejects an identity cached at construction.
#[wasm_bindgen_test]
async fn store_id_answers_the_meta_row_on_every_call() {
    let Opened { host, sql, store } = open();
    let before = store.store_id().await.expect("the identity reads");
    let other = connect(&host, CloudflareEventStore::new(sql));

    let reminted = other.store.remint_identity().expect("the re-mint lands");

    assert_ne!(reminted, before);
    assert_eq!(
        store.store_id().await.expect("the identity reads"),
        reminted
    );
}
