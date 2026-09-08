//! Neon serverless Postgres adapters for happenstance, over the one-shot `/sql`
//! HTTP endpoint.
//!
//! # Status
//!
//! Implemented, and run rather than asserted: the conformance suite passes
//! against a **live Neon endpoint** (PostgreSQL 18.6 behind the pooler), event
//! store and projection store both, including the concurrency family at
//! `CONTENDERS = 64`.
//!
//! It is still `publish = false`, and that is not an oversight. `0.2.0` ships
//! five crates — `happenstance-core`, `happenstance`, `happenstance-testkit`,
//! `happenstance-sqlite` and `happenstance-cloudflare` — and this is not one of
//! them. What packaging this crate owes beyond a green suite belongs to
//! `deskeleton-and-package-readiness`, and the flag stays until that story
//! removes it.
//!
//! # It owns no HTTP client, and that is still true
//!
//! [`SqlTransport`] is a one-method trait and [`NullTransport`] is the only
//! implementation in `src/`. See the [`transport`] module for why: a real client
//! needs a TLS stack on the host and `wasm-bindgen`'s `fetch` on `wasm32`, they
//! are two different clients, and neither is what this crate is here to prove.
//!
//! The conformance suite reaches the live endpoint through a `hyper` + `rustls`
//! transport that lives in `tests/`, as a **`[dev-dependencies]`** and never as a
//! feature. That distinction is load-bearing rather than tidy: `cargo xtask ci`
//! runs `cargo hack check -p happenstance-neon --target wasm32-unknown-unknown
//! --feature-powerset --no-dev-deps`, a feature is not target-scoped, and a
//! `live-transport` feature would therefore be switched on for `wasm32` and fail
//! to build. Dev-dependencies are invisible to that powerset and to the MSRV
//! job's `--no-dev-deps` check, so the client reaches no consumer's graph, no
//! feature combination and no licence surface.
//!
//! What that still costs is recorded honestly: the crate demonstrates a
//! licence-clean client for the **host**, and not for `wasm32`. The shape above
//! it does not need to know which one it has.
//!
//! # What this adapter is an instrument for
//!
//! It is deliberately the **least capable** store in the workspace, and it is in
//! the tree to sit at the far end of the transport axis from `MemoryEventStore`,
//! a `RefCell` store, rusqlite and a Durable Object — all of which serialise
//! their writers behind a lock and can hold a transaction open across a decision.
//! Neon's `/sql` endpoint can do none of that:
//!
//! | | |
//! |---|---|
//! | connection | none — every request is authenticated and routed on its own |
//! | transaction handle | none — no `BEGIN` that a later request can join |
//! | cursor | none — a result set is one buffered JSON document |
//! | round trips | exactly one per operation |
//! | response cap | 64 MiB, hard ([`MAX_RESPONSE_BYTES`]) |
//!
//! The one thing it *does* offer is a **non-interactive** transaction: an array
//! of statements executed server-side inside one `BEGIN`/`COMMIT`, at a chosen
//! isolation level, in a single round trip. Atomic, and unable to branch. Every
//! capability limit below follows from that distinction.
//!
//! # The trap this crate exists to spring
//!
//! An `append` that probes for a condition violation and then writes, in two
//! **round trips**, type-checks against
//! [`EventStore`](happenstance_core::EventStore) perfectly.
//! `ProbeThenWriteStore` is that implementation, written out in full so the
//! compiling call site can be pointed at. It is also silently wrong here: two
//! round trips are two implicit transactions, with a network-latency-wide window
//! between them and no snapshot spanning it. A conflicting append committed
//! inside that window is invisible to the probe and unopposed by the insert.
//!
//! A shape table that recorded only `error[E….]` would therefore rank this crate
//! the most compatible adapter in the workspace. It is the least. The limits that
//! matter here are not type errors.
//!
//! # Can the condition and the write collapse into one statement?
//!
//! **Yes, and it must not.** This is the one place where the crate's own recorded
//! answer was overturned by a measurement rather than by an argument, so the old
//! answer is stated before the new one.
//!
//! It used to say: a single CTE computes the probe and the insert on one snapshot
//! and projects both, so `ConditionViolated::conflicting_position` survives the
//! collapse — one statement, one round trip, one snapshot. Every word of that is
//! true of *SQL*. It is false of *this endpoint*, because
//! `Neon-Batch-Isolation-Level` is honoured only on a request carrying **two or
//! more** statements. Both halves are measured against the live endpoint:
//! `current_setting('transaction_isolation')` answers `serializable` inside a
//! two-statement batch carrying the header, and `read committed` inside a
//! one-statement request carrying the same header.
//!
//! So the single CTE runs at `READ COMMITTED`, where two racers both find no
//! conflict and both insert — the lost update `ProbeThenWriteStore` exists to
//! name, arriving through the door left open while the other one was being
//! closed. The append is therefore a **two-statement batch**: a probe that
//! reports the conflicting position, and a guarded `INSERT … WHERE NOT EXISTS`,
//! on one `SERIALIZABLE` snapshot in one round trip.
//!
//! Two statements in one request is safe; two round trips is not. That sentence
//! is the whole of this adapter's append design, and the `event_store` module
//! carries the measurement behind it.
//!
//! # Which flavour, and which claim
//!
//! `NeonEventStore` implements the **bare**
//! [`EventStore`](happenstance_core::EventStore), and `NeonProjectionStore` the
//! bare `ProjectionStore`. The crate compiles for the host target and for
//! `wasm32-unknown-unknown`, *implementing
//! the bare flavour on each*. That is not the same claim as satisfying both
//! flavours: the contract crate's implication table runs one way only, so nothing
//! here satisfies `SendEventStore`, on either target, even where the transport
//! happens to be `Send`.

#![doc(html_no_source)]

pub mod config;
pub mod error;
pub mod migration;
pub mod transport;
pub mod wire;

#[cfg(feature = "event-store")]
pub mod event_store;

#[cfg(feature = "event-store")]
mod query_sql;

#[cfg(feature = "projection-store")]
pub mod projection_store;

pub use config::NeonConfig;
pub use error::{NeonError, NeonSqlError};
pub use transport::{
    HttpResponse, IsolationLevel, MAX_RESPONSE_BYTES, NullTransport, NullTransportError,
    SqlRequest, SqlStatement, SqlTransport,
};

#[cfg(feature = "event-store")]
pub use event_store::{NeonEventStore, NeonReadStream, ProbeThenWriteStore};

#[cfg(feature = "projection-store")]
pub use projection_store::{NeonProjectionStore, NeonWriteBatch};
