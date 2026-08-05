//! Replication between happenstance instances.
//!
//! # Status: not implemented
//!
//! # The target topology
//!
//! A local-first application holds its own event store on the device and syncs
//! with a shared instance — the motivating deployment being SQLite inside a
//! Cloudflare Durable Object, reached through a Rust Worker. Both peers are
//! happenstance instances; neither is privileged in the protocol, though a
//! deployment may well designate one as authoritative.
//!
//! # Why this is a thin crate and not a hard one
//!
//! Because [`Event`](happenstance::Event) payloads are opaque bytes, a peer
//! forwards events **without deserialising them**. It never needs the sender's
//! domain types, cannot fail to parse a payload it does not understand, and
//! cannot corrupt one by re-encoding it. That is the whole payoff of keeping
//! `happenstance` free of `serde` in its default feature set, and it is why
//! this crate depends on `happenstance/serde` explicitly: the envelope is
//! serialised, the payload is passed through.
//!
//! # The hard part, stated honestly
//!
//! [`SequencePosition`](happenstance::SequencePosition) is meaningful only
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
//!   [`AppendCondition`](happenstance::AppendCondition) checked against the
//!   local log says nothing about the remote one. Whether ingest re-checks
//!   conditions, or whether replication is defined as unconditional
//!   append-of-facts-already-decided, is *the* central design question of this
//!   crate.
//! * **Ordering.** The specification requires a total order per store. Merging
//!   two independently-ordered logs means choosing a merge rule and accepting
//!   that a replicated event's local position differs from its origin position.
//! * **`wasm32` compatibility.** The Cloudflare side is single-threaded, so the
//!   ingest path must be written against
//!   [`EventStore`](happenstance::EventStore) — the flavour with no `Send`
//!   bound — not [`SendEventStore`](happenstance::SendEventStore).
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
