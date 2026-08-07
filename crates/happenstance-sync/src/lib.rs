//! Replication between happenstance instances.
//!
//! # Status: a phase-2 sketch, not the protocol
//!
//! [`SyncPeer`] and [`IngestStore`] exist here to be *falsified by a type
//! checker*, which is the only thing that can falsify a port before an adapter
//! is written. Bodies outside [`memory`] are `todo!()` on purpose: the type
//! checker is the instrument, not the runtime.
//!
//! The question this sketch was built to answer is falsifiable and it is not
//! "write a good trait". It is: **can a peer be stated without naming a
//! transport?** The evidence is in this crate's `tests/`, where the two real
//! peers are stood in for and both implement the trait — see [`peer`] for what
//! that proved and [`ingest`] for what the coherence experiment proved
//! separately.
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
//! documentation — and it is why
//! [`Watermark`] is a version vector rather than a scalar.
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
//! **The envelope types carry no `Serialize`/`Deserialize` yet, deliberately.**
//! They did briefly, and the derives were withdrawn: a `#[derive]` on a public
//! struct with no version field *is* a wire format, WF-8 puts a version first,
//! and the format is phase 5's to settle against `happenstance-core`'s own
//! private-mirror pattern (`event.rs`'s `EventWire`, `append.rs`'s `Wire`) rather
//! than this crate's to assert in passing. Nothing here serialises anything, so
//! the derives bought the sketch nothing and committed it to a shape nobody had
//! authorised. `happenstance-core/serde` stays on because the argument above is
//! still the reason this crate exists.
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
//! * **Event identity across instances.** Sketched, as
//!   [`identity::EventId`] — the pair `(StoreId, SequencePosition)`.
//!   Where it should finally live is a contract-crate question and is **not**
//!   settled here; see [`identity`], which also records why these three types
//!   are reachable only through their module.
//! * **Idempotent ingest.** In the port's contract
//!   ([`IngestStore::ingest`]) and implemented in [`memory`]. Not yet checked by
//!   anything, because `happenstance-sync-testkit` does not exist.
//! * **Append conditions across a boundary.** Still *the* central design
//!   question. This sketch carries the origin's condition as
//!   [`EventGroup::guard`](peer::EventGroup::guard) and documents it as evidence
//!   rather than as an instruction, which is a position rather than an answer.
//! * **Ordering.** Untouched. The merge rule is not sketched and nothing here
//!   should be read as choosing one.
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

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. This crate is one of the phase-2
// skeleton exceptions, scoped here rather than left open in the workspace
// manifest so that it is visible in review. The phase that implements
// replication removes both the bodies and this line.
#![allow(clippy::todo)]

extern crate alloc;

pub mod identity;
pub mod ingest;
pub mod memory;
pub mod peer;

// `EventId`, `StoreId` and `RecordedAt` are deliberately absent from this list.
// They are phase 4's types (VT-4 – VT-10, ADR-0014), placeheld here only because
// the sketch cannot be written without an identity, and re-exporting them would
// put a second `EventId` on the same import path as the settled one. See
// [`identity`]'s module documentation.
pub use identity::{ReplicatedEvent, Watermark};
pub use ingest::{IngestStore, Ingested, SendIngestStore};
pub use memory::{MemoryPeerError, MemoryResume, MemorySyncPeer};
pub use peer::{
    Ack, EventGroup, PeerLimits, PullBatchLimit, Pulled, PushBatch, SendSyncPeer, SyncError,
    SyncPeer,
};
