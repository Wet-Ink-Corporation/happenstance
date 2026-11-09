//! The incarnation check (ADR-0104, SY-34): an answer from a store other than
//! the one the edge synced with stops the edge, names both stores, and leaves
//! the local log alone.

use happenstance_core::{EventStore, StoreId, StoreLimit};
use happenstance_sync::runner::{
    Edge, EdgeState, Failure, IncarnationChanged, Phase, StepError, StopCause,
};
use happenstance_sync::{
    EventGroup, FailureClass, MemoryIngestStore, MemorySyncPeer, StoreSyncPeer,
};

use super::probe::{ProbePeer, ProbeStore, Shared};
use super::{FAR, LOCAL, ORIGIN, append_local, event, foreign};

/// The far side's first incarnation, and the one it is restored under.
const BEFORE: StoreId = StoreId::from_bytes([0xB0; 16]);
const AFTER: StoreId = StoreId::from_bytes([0xAF; 16]);

/// #244's acceptance case, over the real stamping peer: a far side restored
/// from a backup under a new `StoreId` stops the edge, the local log is
/// unchanged, and the status names the cause.
///
/// Rejects a runner with no incarnation check, which ingests the restored
/// store's page and pushes into it as if nothing happened; the default #244
/// found elsewhere, which wipes the local log on a mismatch; and a check made
/// after the ingest,
/// which lands the new incarnation's batch before stopping.
#[tokio::test]
async fn a_far_side_restored_under_a_new_store_id_stops_the_edge_and_keeps_the_log() {
    let far = Shared::new(BEFORE);
    EventStore::append(&*far.0, &[event(b"far")], None)
        .await
        .expect("an in-memory append");
    let before = StoreSyncPeer::new(far.clone(), far.clone());
    let local = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&local, 2).await;
    let mut edge = Edge::new();
    edge.step(&before, &local).await.expect("a first step");
    assert_eq!(
        edge.status().responder,
        Some(BEFORE),
        "the first answer pinned"
    );

    // The far side is restored from a backup of its log, under a new identity,
    // and gains an event the edge has never seen.
    let restored = Shared::new(AFTER);
    Edge::new()
        .step(&before, &restored)
        .await
        .expect("the backup copies the far log");
    EventStore::append(&*restored.0, &[event(b"after the restore")], None)
        .await
        .expect("an in-memory append");
    let after = StoreSyncPeer::new(restored.clone(), restored.clone());
    append_local(&local, 1).await;
    let log = local.snapshot();
    let held = restored.0.snapshot();
    let token = edge.resume_token().copied();
    let confirmed = edge.status().confirmed;

    let stopped = edge.step(&after, &local).await;
    let again = edge.step(&after, &local).await;

    let change = IncarnationChanged::new(Phase::Pull, BEFORE, AFTER);
    assert!(
        matches!(stopped, Err(StepError::Incarnation(seen)) if seen == change),
        "the pull named the new store: {stopped:?}"
    );
    assert_eq!(
        edge.status().state,
        EdgeState::Stopped(StopCause::Incarnation(change)),
        "the status names the cause, with both stores"
    );
    assert_eq!(
        edge.status().last_failure,
        Some(Failure::new(Phase::Pull, FailureClass::Stop))
    );
    assert_eq!(local.snapshot(), log, "the local log is unchanged");
    assert_eq!(restored.0.snapshot(), held, "nothing was pushed into it");
    assert_eq!(
        edge.resume_token().copied(),
        token,
        "the token did not move"
    );
    assert_eq!(edge.status().confirmed, confirmed, "nor the watermark");
    assert_eq!(edge.status().responder, Some(BEFORE), "the pin is kept");
    assert!(
        matches!(again, Err(StepError::Stopped(StopCause::Incarnation(seen))) if seen == change),
        "a stopped edge refuses at once: {again:?}"
    );
    let stopped_text = stopped.map(|_| ()).map_err(|error| error.to_string());
    assert!(
        matches!(&stopped_text, Err(text) if text.contains(&AFTER.to_string()) && text.contains(&BEFORE.to_string())),
        "the message names both stores: {stopped_text:?}"
    );
}

/// A change first seen on an ack. The push landed on the new incarnation, but
/// the edge confirms nothing from it. Rejects a runner that checks pulls only,
/// and one that moves the watermark before checking the ack.
#[tokio::test]
async fn a_new_store_on_an_ack_stops_the_edge_and_confirms_nothing() {
    let peer = ProbePeer::new(MemorySyncPeer::new(FAR));
    peer.ack_responder.set(Some(BEFORE));
    let local = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&local, 1).await;
    let mut edge = Edge::new();
    edge.step(&peer, &local).await.expect("a first step");
    assert_eq!(edge.status().responder, Some(BEFORE), "pinned from the ack");
    let confirmed = edge.status().confirmed;
    let far_side = edge.status().far_side.clone();
    append_local(&local, 1).await;
    let log = local.snapshot();
    peer.ack_responder.set(Some(AFTER));

    let stopped = edge.step(&peer, &local).await;

    let change = IncarnationChanged::new(Phase::Push, BEFORE, AFTER);
    assert!(
        matches!(stopped, Err(StepError::Incarnation(seen)) if seen == change),
        "{stopped:?}"
    );
    assert_eq!(
        edge.status().state,
        EdgeState::Stopped(StopCause::Incarnation(change))
    );
    assert_eq!(edge.status().confirmed, confirmed, "the watermark held");
    assert_eq!(edge.status().far_side, far_side, "and the far side's");
    assert_eq!(local.snapshot(), log);
}

/// ADR-0104 §2: an answer that names no store is no evidence. Rejects a runner
/// that reads `None` as a change, which would stop every edge to a peer that
/// cannot say, and one that lets `None` clear the pin.
#[tokio::test]
async fn an_answer_naming_no_store_neither_stops_nor_unpins() {
    let peer = ProbePeer::answering_as(MemorySyncPeer::new(FAR), BEFORE);
    let local = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&local, 1).await;
    let mut edge = Edge::new();
    edge.step(&peer, &local).await.expect("a first step");
    peer.answer_as(None);
    append_local(&local, 1).await;

    edge.step(&peer, &local)
        .await
        .expect("a silent far side is not a change");
    peer.answer_as(Some(AFTER));
    let stopped = edge.step(&peer, &local).await;

    assert!(
        matches!(stopped, Err(StepError::Incarnation(seen)) if seen.known == BEFORE),
        "the pin survived the silent answers: {stopped:?}"
    );
}

/// The SY-31 seam: a resumed edge told its far side's store stops on the first
/// answer from another, before ingesting it. Rejects a resumed edge that pins
/// whoever answers first, which misses a restore made while it was down.
#[tokio::test]
async fn a_resumed_edge_told_its_store_stops_at_the_first_answer_from_another() {
    let peer = ProbePeer::answering_as(MemorySyncPeer::new(FAR), AFTER);
    peer.inner
        .seed(EventGroup::new(None, vec![foreign(ORIGIN, 1)]))
        .expect("an unpoisoned peer");
    let local = MemoryIngestStore::with_store_id(LOCAL);
    let mut edge = Edge::resuming(None, None).with_responder(BEFORE);

    let stopped = edge.step(&peer, &local).await;

    assert!(
        matches!(stopped, Err(StepError::Incarnation(seen)) if seen == IncarnationChanged::new(Phase::Pull, BEFORE, AFTER)),
        "{stopped:?}"
    );
    assert!(
        local.snapshot().is_empty(),
        "the new store's batch never landed"
    );
}

/// The store is pinned only once an answer naming it has taken effect. A pull
/// whose batch did not land pins nothing, because the edge has not synced with
/// that store; one whose batch landed pins it though the push after fails.
/// Rejects a runner that pins on receipt, which would hold an edge that never
/// synced to a store it only heard from.
#[tokio::test]
async fn a_store_is_pinned_only_once_its_answer_lands() {
    let refused = ProbePeer::answering_as(MemorySyncPeer::new(FAR), BEFORE);
    refused
        .inner
        .seed(EventGroup::new(None, vec![foreign(ORIGIN, 1)]))
        .expect("an unpoisoned peer");
    let store = ProbeStore::new(MemoryIngestStore::with_store_id(LOCAL));
    store.refuse_ingest.set(Some(StoreLimit::EventDataLen));
    let mut unlanded = Edge::new();

    let failed = unlanded.step(&refused, &store).await;

    assert!(matches!(failed, Err(StepError::Ingest(_))), "{failed:?}");
    assert_eq!(
        unlanded.status().responder,
        None,
        "an unlanded pull pins nothing"
    );

    let landed = ProbePeer::answering_as(MemorySyncPeer::new(FAR), BEFORE);
    landed.fail_push.set(Some(FailureClass::Transient));
    let local = MemoryIngestStore::with_store_id(LOCAL);
    append_local(&local, 1).await;
    let mut edge = Edge::new();

    let failed = edge.step(&landed, &local).await;

    assert!(matches!(failed, Err(StepError::Push(_))), "{failed:?}");
    assert_eq!(
        edge.status().responder,
        Some(BEFORE),
        "the landed pull pinned"
    );
}
