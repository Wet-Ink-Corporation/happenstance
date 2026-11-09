//! Replication between happenstance instances.
//!
//! # Status: published, and unfinished
//!
//! [`SyncPeer`] and [`IngestStore`] began here as a phase-2 sketch, to be
//! *falsified by a type checker* before an adapter was written. The stand-in
//! peers in `tests/` are still `todo!()` on purpose (see [`peer`]), because the
//! type checker is the instrument there, not the runtime.
//!
//! Published since #108 part E, so that a store adapter can implement
//! [`IngestStore`] behind its own `sync` feature. **Unfinished**: the [`runner`]
//! drives one edge (#244), by step or by a caller-owned backoff loop, and stops it on a
//! new far-side incarnation, but holds no peer set; its watermark goes through the projection
//! port in the caller's batch (SY-31, [`runner::Watermark`]); no [`Exchange`] runs over a real network for [`RemoteSyncPeer`]. A `[FROZEN]` `SY` clause binds this
//! crate's semver as an `ES` clause binds `happenstance-core`'s. SY-7, SY-10,
//! SY-29 and SY-36 – SY-39 are `[PROVISIONAL]`, binding until the falsifier each
//! names, and SY-32 is `[DEFERRED]` (ADR-0107).
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
//! device and syncs with a shared instance, such as SQLite inside a Durable
//! Object. Both are happenstance instances, neither is privileged in the
//! protocol, and each direction of their edge takes what it is sent, judged by
//! nobody unless one of them is the family's assigned adjudicator (SY-7).
//!
//! **Hub and spoke.** Many devices sync to one authoritative store, and never to
//! each other. This is at least as common as the symmetric case and it is not a
//! special case of it: the hub sees every log, the spokes see one each, and the
//! merge rule a spoke needs is not the merge rule the hub needs. The hub
//! adjudicates what spokes push (SY-7); a spoke takes what the hub sends, unjudged,
//! and keeps its own log when refused (SY-34). So a merge rule belongs to one
//! *direction* of an edge (SY-10), never to a type (SY-9) or to the port.
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
//! **Nothing here travels except inside the envelope.**
//! [`wire::Envelope`] carries a `Serialize` derive and a **hand-written**
//! `Deserialize` that reads the format version and refuses before the message is
//! touched — ADR-0016 §11, WF-8. The derive is not an option for the decoding
//! half: it decodes every field before anything can examine the version, which is
//! the partial decode WF-8's MUST NOT forbids. [`wire`] says so at the impl,
//! because that is where someone will try to simplify it.
//!
//! [`PushBatch`], [`EventGroup`], [`ReplicatedEvent`], [`Ack`] and [`Pulled`]
//! carry `serde` derives again, and [`Watermark`] a hand-written pair. Phase 5
//! withdrew them because a derive on a public message type *is* a wire format,
//! and **what travels was phase 13's**. #114 decided it: the message set is
//! [`wire`]'s four messages, each a type *in* an envelope and never a kind in an
//! enum around one, and ADR-0097 records why. [`SyncError`] is still not extended
//! (SY-30 is untouched): a version refusal is a decoding failure with its own
//! error, [`wire::WireError`], and folding it into the runner's enum would take a
//! phase-13 design decision inside an encoding change.
//! `happenstance-core/serde` stays on because the argument above is still the
//! reason this crate exists.
//!
//! # The hard part, stated honestly
//!
//! [`SequencePosition`](happenstance_core::SequencePosition) is meaningful only
//! within a single store. Two instances that each append independently will
//! assign the same positions to different events, so positions cannot be
//! replicated as-is and a naive "send everything after position N" protocol is
//! wrong.
//!
//! What follows from that, and how far this sketch got with each:
//!
//! * **Event identity across instances.** Settled: VT-5's pair
//!   `(StoreId, SequencePosition)` is [`happenstance_core::EventId`], and this
//!   crate speaks it. The phase-2 placeholders went at phase 17; [`identity`]
//!   records why they had to.
//! * **Idempotent ingest.** In the port's contract
//!   ([`IngestStore::ingest`]), and the write path is the adapter's — see
//!   [`ingest`]. `happenstance-sync-testkit`'s ingest family checks it against
//!   `MemoryIngestStore`, this crate's oracle, and against the SQLite spike.
//! * **Append conditions across a boundary.** Settled by SY-1 – SY-7: ingest
//!   never refuses, and a conflict is compensated by an ordinary append. The
//!   origin's condition travels as [`EventGroup::guard`](peer::EventGroup::guard),
//!   evidence rather than an instruction (SY-6). Nothing here enforces it yet.
//! * **Ordering.** Settled by SY-19: local position is arrival order, and two
//!   peers need not agree on a total order. The sketch implements neither.
//! * **`wasm32` compatibility.** Checked. The whole crate builds for
//!   `wasm32-unknown-unknown`, and the bare [`SyncPeer`] and [`IngestStore`]
//!   flavours are what the Cloudflare side implements.
//! * **What a peer may be asked to do.** This is the one the sketch bites
//!   hardest on. One of the two intended peers reaches its store over one-shot
//!   HTTP: no connection, no interactive transaction, no cursor, one round trip
//!   per operation. A port that assumes a peer can hold state open between calls
//!   excludes it — so [`SyncPeer::pull`] returns a bounded batch and an owned
//!   token rather than a stream. **The type checker did not force that choice**,
//!   and the transcript showing it did not is the most useful thing this sketch
//!   produced.
//!
//! # Features
//!
//! * **`memory`**, off by default: `MemorySyncPeer` and `MemoryIngestStore`, the
//!   in-memory reference peer and ingest oracle, for doctests and for testing
//!   sync wiring before a real peer exists. Turns on `happenstance-core/memory`.

#![doc(html_no_source)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
// No `#![allow(clippy::todo)]`: the last `todo!()` in `src/` was the
// `MemoryEventStore` ingest impl, which phase 17 deleted rather than finished
// (see [`ingest`]). The two stand-in peers in `tests/` carry their own.

extern crate alloc;

pub mod identity;
pub mod ingest;
#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub mod memory;
pub mod peer;
pub mod wire;

// `EventId`, `StoreId` and `RecordedAt` are `happenstance_core`'s and are not
// re-exported: one import path per type, and it is the contract crate's.
pub use identity::{ReplicatedEvent, Watermark};
pub use ingest::{IngestError, IngestGroup, IngestStore, Ingested, SendIngestStore};
#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use memory::{MemoryPeerError, MemoryResume, MemorySyncPeer};
pub use peer::{
    Ack, EventGroup, PeerLimits, PullBatchLimit, Pulled, PushBatch, SendSyncPeer, SyncError,
    SyncPeer, Waiting,
};

// `wire`'s items are deliberately left module-qualified. `wire::FORMAT_VERSION`
// says at the call site *which* version it is, and phase 13 will plausibly add a
// negotiated protocol version that is not this one; a bare `FORMAT_VERSION` at
// the crate root would make the two indistinguishable in an import list.

// SY-36's failure class. Declared after the rest rather than in alphabetical
// order, so the lines above keep the numbers the specification cites.
pub mod classify;
pub use classify::{Classify, FailureClass};

// The in-memory ingest oracle (#108). After `classify`, for the reason that one
// sits after the rest: the lines above keep the numbers the specification cites.
#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub mod memory_ingest;
#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use memory_ingest::MemoryIngestStore;

// The network peer's transport-free pieces (#117): a peer over a store, the
// server half that answers envelope bytes through it, the transport trait, and
// the client peer over that. After the rest, for the reason above. No feature:
// the Durable Object mounts `serve_sync` on wasm32 with none.
pub mod remote;
pub mod serve;
pub mod store_peer;
pub use remote::{Exchange, RemoteError, RemoteSyncPeer, SendExchange};
pub use serve::{ServeError, serve_sync};
pub use store_peer::{ForeignToken, StorePeerError, StoreResume, StoreSyncPeer};

// The runner above the port (#244). Last, for the reason `classify` is after
// the rest: the lines above keep the numbers the specification cites. Its items
// stay module-qualified, as `wire`'s do: `runner::Edge` and `runner::StepError`
// say at the call site which layer they belong to.
pub mod runner;

// The write-group port (#420): a store's log read back in the writes that made
// it, so a push and a pull never split one write or merge two. Last, for the
// reason `runner` is: the lines above keep the numbers the specification cites.
pub mod write_group;
pub use write_group::{GroupPage, SendWriteGroupStore, WriteGroup, WriteGroupStore};

// The `Send` flavour of the network peer's two halves (#421): a second type,
// because coherence refuses a second impl on the first (RS-20-4). Last, for the
// reason `write_group` is: the lines above keep the numbers the specification
// cites.
pub mod send_peer;
pub use send_peer::SendPeer;
