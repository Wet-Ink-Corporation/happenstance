//! T-450-1, the semver canary: code written against `0.6.0`'s runner and store
//! peer, before the adjudication hook, compiles and runs unchanged (D-450-6).
//!
//! Each item is named the way a consumer of `0.6.0` named it: `Edge<R>` with
//! one parameter, `StepError<P, R, I>` with three, `StepFailure<P, S>` with
//! two, `StoreSyncPeer<S, I>`, `StorePeerError<R, W>`, `ServeError<R, W>`, and
//! `serve_sync` by turbofish with two. The hook's parameters are defaulted, so
//! each of these still means the type it meant, with `Accept` in the new slot.
//! A defaulted parameter that stopped inferring, or a bound that moved onto
//! the struct, is a compile error here.

use happenstance_core::{EventStore, MemoryStoreError, StoreId};
use happenstance_sync::runner::{Edge, Failure, StepError, StepFailure, Stepped};
use happenstance_sync::wire::Route;
use happenstance_sync::write_group::GroupOrderError;
use happenstance_sync::{
    Classify, FailureClass, IngestStore, MemoryIngestStore, MemoryPeerError, MemoryResume,
    MemorySyncPeer, ServeError, StorePeerError, StoreSyncPeer, SyncPeer, WriteGroupStore,
    serve_sync,
};

use super::probe::Shared;

/// A step written generically over 0.6.0's bounds and alias.
async fn step_generic<P, S>(
    edge: &mut Edge<P::Resume>,
    peer: &P,
    store: &S,
) -> Result<Stepped, StepFailure<P, S>>
where
    P: SyncPeer,
    S: WriteGroupStore + IngestStore,
{
    edge.step(peer, store).await
}

/// 0.6.0's three-parameter error, classified in generic code.
fn class_of<P: Classify, R: Classify, I: Classify>(error: &StepError<P, R, I>) -> FailureClass {
    error.class()
}

/// The concrete error a memory edge returns, named with three parameters.
fn failure_of(error: &StepError<MemoryPeerError, GroupOrderError, MemoryStoreError>) -> Failure {
    error.failure()
}

/// A store peer and its errors, named with 0.6.0's parameters.
fn store_peer(store: &Shared) -> StoreSyncPeer<Shared, Shared> {
    StoreSyncPeer::new(store.clone(), store.clone())
}

fn store_peer_class(error: &StorePeerError<GroupOrderError, MemoryStoreError>) -> FailureClass {
    error.class()
}

fn serve_class(error: &ServeError<GroupOrderError, MemoryStoreError>) -> FailureClass {
    error.class()
}

#[tokio::test]
async fn code_written_against_0_6_0_compiles_and_runs_unchanged() {
    let peer = MemorySyncPeer::new(StoreId::from_bytes([0x45; 16]));
    let store = MemoryIngestStore::new();
    let mut edge: Edge<MemoryResume> = Edge::new();
    let mut defaulted: Edge<MemoryResume> = Edge::default();

    let stepped = step_generic(&mut edge, &peer, &store)
        .await
        .expect("an in-memory step");
    defaulted
        .step(&peer, &store)
        .await
        .map_err(|error| (class_of(&error), failure_of(&error)))
        .expect("an in-memory step");
    assert_eq!(stepped.pushed, 0);
    assert!(edge.status().is_synced());

    let far = Shared::new(StoreId::from_bytes([0x46; 16]));
    EventStore::append(
        &*far.0,
        &[happenstance_core::Event::new("Written", &b"{}"[..]).expect("a valid type")],
        None,
    )
    .await
    .expect("an in-memory append");
    let served = store_peer(&far);
    let request = postcard::to_stdvec(&happenstance_sync::wire::Envelope::new(
        happenstance_sync::wire::PullRequest::from_start(),
    ))
    .expect("a pull request encodes");
    let reply = serve_sync::<Shared, Shared>(&served, Route::Pull, &request)
        .await
        .map_err(|error| serve_class(&error))
        .expect("the store peer answers");
    assert!(!reply.is_empty());
    served
        .pull(None)
        .await
        .map_err(|error| store_peer_class(&error))
        .expect("the store peer pulls");
}
