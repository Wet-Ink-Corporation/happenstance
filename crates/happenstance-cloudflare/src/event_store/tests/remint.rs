//! `CloudflareEventStore::remint_identity`, held against a simulated
//! point-in-time restore (#262).
//!
//! A restore rolls back the whole object: its events, the `store_meta` row that
//! carries its [`StoreId`](happenstance_core::StoreId), and the
//! `sqlite_sequence` counter `AUTOINCREMENT` hands positions out from. Without
//! a re-mint the restored object hands out `(StoreId, SequencePosition)` pairs
//! it already gave to events that are gone from it but not from its peers,
//! which is what VT-6 forbids.
//!
//! The oracles here read raw SQL, or the [`EventId`]s `read` yields, and never
//! the adapter's own identity read-back, so no check shares a subroutine with
//! the code under test. The restart that applies a restore is modelled by
//! binding **fresh** handles off [`DurableObjectHost::storage`] afterwards.

use happenstance_core::{Event, EventId, EventStore, Query, ReadOptions, collect};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::host::{DurableObjectHost, arm_throw, bookmark, durable_object, restore_bookmark};
use crate::sql_storage::{SqlStorage, SqlValue};
use crate::{CloudflareEventStore, CloudflareEventStoreError, TableNamespace};

fn event(event_type: &str) -> Event {
    Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
}

/// A fresh object and one migrated handle onto it.
fn open() -> (DurableObjectHost, CloudflareEventStore) {
    let host = DurableObjectHost::new();
    let store = CloudflareEventStore::new(host.storage());
    store.migrate().expect("the schema applies");
    (host, store)
}

/// Every `v` stored under `store_id` in `table`, read through raw SQL.
///
/// A `Vec` of rows rather than one value, so that a second row is visible
/// rather than hidden behind the first.
fn persisted_identity(sql: &SqlStorage, table: &str) -> Vec<Vec<u8>> {
    let mut cursor = sql
        .exec(&format!("SELECT v FROM {table} WHERE k = 'store_id'"), &[])
        .expect("the select runs");
    let mut out = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the row decodes").values() {
            [SqlValue::Blob(bytes)] => out.push(bytes.clone()),
            other => panic!("the identity is not a blob: {other:?}"),
        }
    }
    out
}

/// The high-water mark `AUTOINCREMENT` holds for `event`, read through raw SQL.
fn position_counter(sql: &SqlStorage) -> i64 {
    let mut cursor = sql
        .exec("SELECT seq FROM sqlite_sequence WHERE name = 'event'", &[])
        .expect("the select runs");
    let row = cursor
        .next_row()
        .expect("the counter has a row")
        .expect("the row decodes");
    match row.values() {
        [SqlValue::Integer(seq)] => *seq,
        other => panic!("the counter is not an integer: {other:?}"),
    }
}

/// The kind and name of every schema object, in name order.
fn schema_objects(sql: &SqlStorage) -> Vec<(String, String)> {
    let mut cursor = sql
        .exec("SELECT type, name FROM sqlite_schema ORDER BY name", &[])
        .expect("the select runs");
    let mut out = Vec::new();
    while let Some(row) = cursor.next_row() {
        match row.expect("the row decodes").values() {
            [SqlValue::Text(kind), SqlValue::Text(name)] => out.push((kind.clone(), name.clone())),
            other => panic!("unexpected schema row: {other:?}"),
        }
    }
    out
}

/// Every event the store yields, as its `EventId` and its type.
async fn ids(store: &CloudflareEventStore) -> Vec<(EventId, String)> {
    collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads")
        .into_iter()
        .map(|item| (item.id, item.event.event_type().as_str().to_owned()))
        .collect()
}

/// The `EventId` the store yields for the one event of type `event_type`.
async fn id_of(store: &CloudflareEventStore, event_type: &str) -> EventId {
    let matching: Vec<EventId> = ids(store)
        .await
        .into_iter()
        .filter(|(_, ty)| ty == event_type)
        .map(|(id, _)| id)
        .collect();
    match matching.as_slice() {
        [id] => *id,
        other => panic!("expected one `{event_type}`, found {other:?}"),
    }
}

async fn append(store: &CloudflareEventStore, event_type: &str) {
    store
        .append(&[event(event_type)], None)
        .await
        .expect("the append lands");
}

/// The instrument. Every later test leans on the simulated restore rolling back
/// the rows, the identity row **and** the position counter together; one that
/// copied `event` but not `store_meta` or `sqlite_sequence` would make each of
/// them vacuous.
#[wasm_bindgen_test]
async fn a_bookmark_restore_rolls_back_rows_identity_and_the_position_counter() {
    let (host, store) = open();
    let sql = host.storage();
    sql.exec(
        "CREATE TRIGGER audit_marker AFTER INSERT ON event BEGIN SELECT 1; END",
        &[],
    )
    .expect("the trigger applies");
    append(&store, "A1").await;
    append(&store, "A2").await;
    let identity = persisted_identity(&sql, "store_meta");
    let counter = position_counter(&sql);
    let schema = schema_objects(&sql);
    let mark = bookmark(&sql);

    append(&store, "B1").await;
    sql.exec(
        "UPDATE store_meta SET v = randomblob(16) WHERE k = 'store_id'",
        &[],
    )
    .expect("the identity is overwritten");
    assert_ne!(persisted_identity(&sql, "store_meta"), identity);
    assert_ne!(position_counter(&sql), counter);

    restore_bookmark(&sql, &mark);

    let restored = host.storage();
    let types: Vec<String> = ids(&CloudflareEventStore::new(restored.clone()))
        .await
        .into_iter()
        .map(|(_, ty)| ty)
        .collect();
    assert_eq!(types, ["A1", "A2"], "the rows roll back");
    assert_eq!(
        persisted_identity(&restored, "store_meta"),
        identity,
        "the identity row rolls back"
    );
    assert_eq!(
        position_counter(&restored),
        counter,
        "the position counter rolls back"
    );
    assert_eq!(
        schema_objects(&restored),
        schema,
        "the schema, indexes and triggers included, rolls back"
    );
}

/// The negative control. Without a re-mint, a restored object hands an event
/// the very `EventId` it already gave a different one, and a peer that saw the
/// first drops the second as a duplicate. It passes against the adapter as it
/// stands, which is what a control is for: it proves the restore reproduces the
/// hazard the re-mint exists to remove.
#[wasm_bindgen_test]
async fn without_a_remint_a_restored_object_reissues_an_event_id() {
    let (host, store) = open();
    let sql = host.storage();
    append(&store, "Kept").await;
    let mark = bookmark(&sql);
    append(&store, "Lost").await;
    let lost = id_of(&store, "Lost").await;

    restore_bookmark(&sql, &mark);
    let restarted = CloudflareEventStore::new(host.storage());
    restarted.migrate().expect("a migrate on reopen is a no-op");
    append(&restarted, "Different").await;

    assert_eq!(
        id_of(&restarted, "Different").await,
        lost,
        "an un-reminted restore re-issues the lost event's identity to a different event"
    );
}

/// The re-mint persists a new incarnation, replacing the row rather than adding
/// one, as the 16-byte blob every other identity in this store is.
#[wasm_bindgen_test]
async fn remint_identity_replaces_the_persisted_incarnation() {
    let (host, store) = open();
    let sql = host.storage();
    let before = persisted_identity(&sql, "store_meta");

    let minted = store.remint_identity().expect("the re-mint lands");

    assert_ne!(before, [minted.to_bytes().to_vec()], "a new incarnation");
    assert_eq!(
        persisted_identity(&sql, "store_meta"),
        [minted.to_bytes().to_vec()],
        "exactly one identity row, a 16-byte blob equal to the one returned"
    );
    let fresh = CloudflareEventStore::new(host.storage());
    append(&fresh, "AfterTheRemint").await;
    assert_eq!(id_of(&fresh, "AfterTheRemint").await.store(), minted);
}

/// The operation is total: on an object that was never migrated it applies the
/// schema first, rather than updating a table that is not there.
#[wasm_bindgen_test]
async fn remint_identity_migrates_an_object_that_never_was() {
    let sql = durable_object();
    let store = CloudflareEventStore::new(sql.clone());

    let minted = store.remint_identity().expect("the re-mint migrates first");

    assert_eq!(
        persisted_identity(&sql, "store_meta"),
        [minted.to_bytes().to_vec()]
    );
    append(&store, "First").await;
    assert_eq!(id_of(&store, "First").await.store(), minted);
}

/// A namespaced log carries its own incarnation, so re-minting one log leaves
/// every other log in the object as it was.
#[wasm_bindgen_test]
async fn remint_identity_touches_only_its_own_namespace() {
    let sql = durable_object();
    let plain = CloudflareEventStore::new(sql.clone());
    let tenant = CloudflareEventStore::namespaced(
        sql.clone(),
        &TableNamespace::new("tenant_7").expect("a valid namespace"),
    );
    plain.migrate().expect("the plain log migrates");
    tenant.migrate().expect("the tenant log migrates");
    let plain_before = persisted_identity(&sql, "store_meta");
    let tenant_before = persisted_identity(&sql, "tenant_7_store_meta");

    tenant.remint_identity().expect("the tenant re-mint lands");
    let tenant_after = persisted_identity(&sql, "tenant_7_store_meta");
    assert_ne!(tenant_after, tenant_before, "the tenant log re-minted");
    assert_eq!(
        persisted_identity(&sql, "store_meta"),
        plain_before,
        "the plain log did not"
    );

    plain.remint_identity().expect("the plain re-mint lands");
    assert_ne!(
        persisted_identity(&sql, "store_meta"),
        plain_before,
        "the plain log re-minted"
    );
    assert_eq!(
        persisted_identity(&sql, "tenant_7_store_meta"),
        tenant_after,
        "the tenant log did not"
    );
}

/// A re-mint whose `UPDATE` throws says so, and the object keeps the
/// incarnation it had rather than being told about one that never landed.
#[wasm_bindgen_test]
async fn a_remint_whose_update_throws_reports_it_and_keeps_the_old_incarnation() {
    let (host, store) = open();
    let sql = host.storage();
    let before = persisted_identity(&sql, "store_meta");
    arm_throw(&sql, "SET v = randomblob", "simulated storage fault");

    let refused = store.remint_identity();

    assert!(
        matches!(refused, Err(CloudflareEventStoreError::Sql(_))),
        "the thrown UPDATE is reported: {refused:?}"
    );
    assert_eq!(persisted_identity(&sql, "store_meta"), before);
    append(&store, "AfterTheFault").await;
    assert_eq!(
        [id_of(&store, "AfterTheFault")
            .await
            .store()
            .to_bytes()
            .to_vec()],
        before.as_slice(),
        "the next append stamps the incarnation that is still persisted"
    );
}

/// The events written after the bookmark, which the restore rolls back.
const LOST: [&str; 2] = ["B1", "B2"];

/// The events written after the restore: more than were lost, so positions both
/// inside and beyond the rolled-back range are handed out.
const REISSUED: [&str; 3] = ["C1", "C2", "C3"];

/// A restored object with two events before the bookmark and two after it,
/// rolled back, and the `(EventId, type)` of all four as the object held them
/// before the restore.
async fn restored_after_losing_two() -> (DurableObjectHost, Vec<(EventId, String)>) {
    let (host, store) = open();
    let sql = host.storage();
    append(&store, "A1").await;
    append(&store, "A2").await;
    let mark = bookmark(&sql);
    for lost in LOST {
        append(&store, lost).await;
    }
    let before = ids(&store).await;
    assert_eq!(before.len(), 4, "the arrangement holds all four events");
    restore_bookmark(&sql, &mark);
    (host, before)
}

/// The acceptance test for #262 item 3. After a restore and a re-mint, no
/// `EventId` the object yields names a different event than the one it named
/// before the restore — including at positions the restore handed out again.
#[wasm_bindgen_test]
async fn a_remint_after_a_restore_reissues_no_event_id_for_a_different_event() {
    let (host, before) = restored_after_losing_two().await;
    let store = CloudflareEventStore::new(host.storage());
    store.migrate().expect("a migrate on reopen is a no-op");
    store.remint_identity().expect("the re-mint lands");

    for reissued in REISSUED {
        append(&store, reissued).await;
    }

    let after = ids(&store).await;
    for (id, ty) in &after {
        if let Some((_, was)) = before.iter().find(|(seen, _)| seen == id) {
            assert_eq!(
                was, ty,
                "{id:?} named {was} before the restore and {ty} after"
            );
        }
    }
    let new_ids: Vec<EventId> = after
        .iter()
        .filter(|(_, ty)| REISSUED.contains(&ty.as_str()))
        .map(|(id, _)| *id)
        .collect();
    assert_eq!(new_ids.len(), REISSUED.len());
    assert!(
        new_ids
            .iter()
            .all(|id| before.iter().all(|(seen, _)| seen != id)),
        "no event appended after the re-mint wears an identity issued before the restore"
    );
    let kept = |list: &[(EventId, String)]| -> Vec<(EventId, String)> {
        list.iter()
            .filter(|(_, ty)| ty.starts_with('A'))
            .cloned()
            .collect()
    };
    assert_eq!(
        kept(&after),
        kept(&before),
        "events from before the bookmark keep the identity they were written with"
    );
    assert!(
        new_ids.iter().any(|id| before
            .iter()
            .any(|(seen, ty)| LOST.contains(&ty.as_str()) && seen.position() == id.position())),
        "the restore handed out a lost position again, so identity is what told them apart"
    );
}

/// Every handle onto the log stamps the new incarnation after a re-mint: one
/// cloned from the handle that re-minted, and one built separately, both
/// opened before the call.
#[wasm_bindgen_test]
async fn a_handle_cloned_before_a_remint_stamps_the_new_incarnation() {
    let (host, before) = restored_after_losing_two().await;
    let primary = CloudflareEventStore::new(host.storage());
    primary.migrate().expect("a migrate on reopen is a no-op");
    let cloned = primary.clone();
    let separate = CloudflareEventStore::new(host.storage());
    separate.migrate().expect("a migrate on reopen is a no-op");

    let minted = primary.remint_identity().expect("the re-mint lands");
    append(&cloned, "ThroughTheClone").await;
    append(&separate, "ThroughTheSeparateHandle").await;

    for event_type in ["ThroughTheClone", "ThroughTheSeparateHandle"] {
        let id = id_of(&primary, event_type).await;
        assert_eq!(
            id.store(),
            minted,
            "{event_type} stamps the new incarnation"
        );
        assert!(
            before.iter().all(|(seen, _)| *seen != id),
            "{event_type} re-uses an identity issued before the restore"
        );
    }
}
