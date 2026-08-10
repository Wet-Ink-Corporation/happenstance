//! VT-10's compile-level obligation: `EventStore::append` takes events and a
//! condition, and neither carries a slot for an identity the caller minted.
//!
//! It lives in the testkit rather than in `happenstance-core` because VT-10's
//! `Rule:` puts it here, and the vantage is the reason. The clause is about what
//! a **replicating peer** can reach for, and a peer is downstream of the
//! contract crate. `IngestStore`, in `happenstance-sync`, is the operation that
//! is *allowed* to accept a foreign identity; this file is the standing check
//! that the local write path never quietly grew one beside it.
//!
//! Two claims live here and they fail in different places. The signature claim
//! is discharged the moment this target builds. The value claim runs.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

use happenstance_core::{
    AppendError, Event, EventStore, MemoryEventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, collect,
};

/// Forwards a peer's events into a local store — the operation VT-10 keeps off
/// [`EventStore`].
///
/// Bound on the port and never on a concrete store, so the claim is about the
/// port. The load-bearing line is the rebuild: a [`SequencedEvent`] is what a
/// peer sends and it carries an `EventId`, while `append` takes `&[Event]`, so
/// forwarding one means dropping the peer's identity on the floor and letting
/// the receiving store mint its own. There is no argument to smuggle it in.
///
/// **Rejects:** VT-10's named shape — an `append` taking
/// `&[(Event, Option<EventId>)]`, or a `&[SequencedEvent]` batch. Under either,
/// the call below is `error[E0308]: mismatched types`, and the rewrite that
/// makes it compile again is precisely the one that carries the foreign
/// identity through the local write path. Every local command handler in every
/// application would then carry an identity slot it passes `None` for, and any
/// caller could forge an identity colliding with a peer's real event.
async fn forward_from_peer<S: EventStore>(
    local: &S,
    from_peer: &[SequencedEvent],
) -> Result<SequencePosition, AppendError<S::Error>> {
    let without_identity: Vec<Event> = from_peer
        .iter()
        .map(|sequenced| sequenced.event.clone())
        .collect();

    local.append(&without_identity, None).await
}

#[tokio::test]
async fn append_does_not_accept_a_foreign_identity() {
    let peer = MemoryEventStore::new();
    peer.append(&[Event::new("Issued", &b"1"[..]).unwrap()], None)
        .await
        .unwrap();
    let from_peer = collect(peer.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();

    let local = MemoryEventStore::new();
    forward_from_peer(&local, &from_peer).await.unwrap();

    let landed = collect(local.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    assert_eq!(landed.len(), 1);

    // The payload crossed and the identity did not, which is the observable
    // half of the signature claim: the receiving store minted its own, and the
    // peer's is not held here — so a sync runner asking "do I already have
    // this?" gets `false` and would replicate it twice. That is not a defect in
    // the store; it is the reason ingest is a separate port.
    assert_eq!(landed[0].event, from_peer[0].event);
    assert_ne!(landed[0].id, from_peer[0].id);
    assert!(!local.contains_event_id(from_peer[0].id).await.unwrap());

    // And `Event` itself has no identity slot to have carried one in. An event
    // that has been through a store is *equal* to one built from scratch out of
    // the same parts, so there is nothing identity-shaped inside `Event` for an
    // adapter to have filled in. Give `Event` an `id` — minted at construction,
    // so every caller gets one for free — and this is the assertion that fails,
    // because the rebuild above would then be forwarding it.
    assert_eq!(from_peer[0].event, Event::new("Issued", &b"1"[..]).unwrap());
}
