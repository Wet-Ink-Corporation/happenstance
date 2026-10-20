//! The public constructors (#108 part E, the owner's D5 **B**): an ingest store
//! is built from the object's `DurableObjectState`, the value a
//! `#[durable_object]` class's `State::_inner` hands back, and from nothing
//! else.

use happenstance_core::StoreId;
use happenstance_sync::{IngestGroup, IngestStore};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{log, minted, scalar};
use crate::event_store::CloudflareEventStore;
use crate::event_store::ingest::{CloudflareIngestStore, model};
use crate::host::DurableObjectHost;
use crate::namespace::TableNamespace;
use crate::sql_storage::SqlError;
use crate::{CloudflareEventStoreError, SqlStorage};

/// A peer's identity.
const PEER: StoreId = StoreId::from_bytes([0x65; 16]);

/// A fresh object with the model's `transactionSync` installed.
fn modelled() -> (DurableObjectHost, SqlStorage) {
    let host = DurableObjectHost::new();
    model::install(&host).expect("the model installs on the shim");
    let sql = host.storage();
    (host, sql)
}

/// A store built from the object's state ingests a foreign event into the
/// object's log, through the object's `transactionSync`, and reads it back
/// under its origin identity.
///
/// Rejects a constructor that binds a second, unrelated storage (the event
/// would be absent from the object's own log), and one that keeps a storage
/// other than the state's (the model would count no call).
#[wasm_bindgen_test]
async fn a_store_from_object_state_ingests() {
    let (host, sql) = modelled();
    CloudflareEventStore::new(sql.clone())
        .migrate()
        .expect("the schema applies");
    let store = CloudflareIngestStore::from_object_state(&host.object_state())
        .expect("the shim's storage carries the model's transactionSync");
    let sent = [minted(PEER, 9, 9, b"from-state", &[])];

    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");

    let held = log(&CloudflareEventStore::new(sql)).await;
    assert_eq!(held.len(), 1, "the object's own log holds the event");
    assert_eq!(held[0].id, sent[0].id, "under its origin identity");
    assert_eq!(
        model::calls(&host).expect("the model is installed"),
        1,
        "one call, one transactionSync on the state's storage"
    );
}

/// The namespaced constructor keeps the log in the namespace's tables, and
/// leaves the unprefixed ones untouched.
///
/// Rejects a namespaced constructor that drops its namespace.
#[wasm_bindgen_test]
async fn a_namespaced_store_from_object_state_ingests_into_its_namespace() {
    let (host, sql) = modelled();
    let namespace = TableNamespace::new("tenant_7").expect("a valid namespace");
    for store in [
        CloudflareEventStore::new(sql.clone()),
        CloudflareEventStore::namespaced(sql.clone(), &namespace),
    ] {
        store.migrate().expect("the schema applies");
    }
    let store =
        CloudflareIngestStore::namespaced_from_object_state(&host.object_state(), &namespace)
            .expect("the shim's storage carries the model's transactionSync");
    let sent = [minted(PEER, 4, 4, b"namespaced", &[])];

    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");

    assert_eq!(
        scalar(&sql, "SELECT count(*) FROM tenant_7_event"),
        Some(1),
        "the namespace's log holds it"
    );
    assert_eq!(
        scalar(&sql, "SELECT count(*) FROM event"),
        Some(0),
        "the unprefixed log does not"
    );
}

/// A storage with no `transactionSync` is refused at construction, so no
/// ingest store exists that would write without a transaction.
///
/// Rejects a constructor that defers the check to the first ingest, or falls
/// back to untransacted writes.
#[wasm_bindgen_test]
fn from_object_state_refuses_a_storage_without_transaction_sync() {
    // The bare shim: no model installed, and the shim carries none of its own.
    let host = DurableObjectHost::new();

    let refused = CloudflareIngestStore::from_object_state(&host.object_state());

    assert!(
        matches!(
            refused,
            Err(CloudflareEventStoreError::Sql(SqlError::Thrown(_)))
        ),
        "{refused:?}"
    );
}
