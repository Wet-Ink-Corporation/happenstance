//! Replication between happenstance instances.
//!
//! # Status: not implemented
//!
//! # This is a port, not a protocol
//!
//! `happenstance-core` defines two ports —
//! [`EventStore`](happenstance_core::EventStore) and
//! [`ProjectionStore`](happenstance_core::ProjectionStore) — and this crate
//! defines the third. It stands to its peer adapters as `happenstance-core`
//! stands to its store adapters: the trait and the runner live here, the
//! conformance suite lives in `happenstance-sync-testkit`, and a peer is a
//! sibling crate.
//!
//! The reason is the deployment. A local-first application syncing to a Durable
//! Object today should be able to add a Postgres as a second peer tomorrow, or
//! swap the first for the second, without touching a line of application code.
//! That is a port by definition, and the workspace's standing rule applies to it
//! exactly as it applies to the other two: **a port with one implementation is
//! shaped like that implementation.** So the trait is settled against two peers
//! that are as unlike each other as the deployment story allows — a Durable
//! Object reached over a socket, and a Postgres reached over one-shot HTTP with
//! no interactive transaction available at all.
//!
//! The port lives here rather than in the contract crate deliberately. Putting
//! it beside the other two would be more symmetric and would put replication
//! back on the publish path; keeping it here is what lets `happenstance-core`
//! reach 0.1 without waiting on this crate.
//!
//! ## One peer, and a runner above it
//!
//! The port describes a *single* peer. Fan-out across several, primary/secondary
//! ordering, and what to do when two peers disagree are policy, and policy on the
//! port would make every adapter author inherit the merge problem and would make
//! the conformance suite test a policy rather than a transport. They belong to a
//! runner, in the same division of labour that puts the projection runner above
//! [`ProjectionStore`](happenstance_core::ProjectionStore). "Add a second peer" is
//! then a runner configuration rather than a breaking change to the port.
//!
//! # The target topologies — plural
//!
//! **Peer-to-peer.** A local-first application holds its own event store on the
//! device and syncs with a shared instance — the motivating deployment being
//! SQLite inside a Cloudflare Durable Object, reached through a Rust Worker.
//! Both peers are happenstance instances; neither is privileged in the protocol,
//! though a deployment may well designate one as authoritative.
//!
//! **Hub and spoke.** Many devices sync to one authoritative store, and never to
//! each other. This is at least as common as the symmetric case and it is not a
//! special case of it: the hub sees every log, the spokes see one each, and the
//! merge rule a spoke needs is not the merge rule the hub needs. Both must be
//! expressible, which is a constraint on the port's shape and not merely on its
//! documentation.
//!
//! # Why this is a thin crate and not a hard one
//!
//! Because [`Event`](happenstance_core::Event) payloads are opaque bytes, a peer
//! forwards events **without deserialising them**. It never needs the sender's
//! domain types, cannot fail to parse a payload it does not understand, and
//! cannot corrupt one by re-encoding it. That is the whole payoff of keeping
//! `happenstance-core` free of `serde` in its default feature set, and it is why
//! this crate depends on `happenstance-core/serde` explicitly: the envelope is
//! serialised, the payload is passed through.
//!
//! # The hard part, stated honestly
//!
//! [`SequencePosition`](happenstance_core::SequencePosition) is meaningful only
//! within a single store. Two instances that each append independently will
//! assign the same positions to different events, so positions cannot be
//! replicated as-is and a naive "send everything after position N" protocol is
//! wrong.
//!
//! What follows from that, and must be designed rather than assumed:
//!
//! * **Event identity across instances.** Something stable and globally unique
//!   is needed — a UUIDv7 or a content hash in the event's metadata — so a peer
//!   can recognise an event it has already ingested.
//! * **Idempotent ingest.** Re-delivery must be harmless; a peer will see the
//!   same event more than once.
//! * **Append conditions across a boundary.** An
//!   [`AppendCondition`](happenstance_core::AppendCondition) checked against the
//!   local log says nothing about the remote one. Whether ingest re-checks
//!   conditions, or whether replication is defined as unconditional
//!   append-of-facts-already-decided, is *the* central design question of this
//!   crate.
//! * **Ordering.** The specification requires a total order per store. Merging
//!   two independently-ordered logs means choosing a merge rule and accepting
//!   that a replicated event's local position differs from its origin position.
//! * **`wasm32` compatibility.** The Cloudflare side is single-threaded, so the
//!   ingest path must be written against
//!   [`EventStore`](happenstance_core::EventStore) — the flavour with no `Send`
//!   bound — not [`SendEventStore`](happenstance_core::SendEventStore).
//! * **What a peer may be asked to do.** One of the two intended peers reaches
//!   its store over one-shot HTTP: no connection, no interactive transaction, no
//!   cursor, one round trip per operation. A port that assumes a peer can hold
//!   state open between calls excludes it. This is the constraint most likely to
//!   be discovered late, which is why a skeleton for it exists before the trait
//!   does.
//!
//! None of this is settled. It is written down here so the next pass starts
//! from the real questions rather than rediscovering them.

#![doc(html_no_source)]

/// How replication fails.
///
/// # Status: not implemented
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SyncError {
    /// Placeholder variant; replaced by real failure modes on implementation.
    #[error("event replication is not implemented yet")]
    Unimplemented,
}
