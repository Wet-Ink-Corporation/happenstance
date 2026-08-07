//! Neon serverless Postgres adapters for happenstance, over the one-shot `/sql`
//! HTTP endpoint.
//!
//! # Status: not implemented
//!
//! Every body is `todo!()` and the crate is `publish = false`. Its associated
//! types are real, because those are the only part of a skeleton a type checker
//! can disagree with.
//!
//! # What this adapter is an instrument for
//!
//! It is deliberately the **least capable** store in the workspace, and it is in
//! the tree to sit at the far end of the transport axis from
//! `MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object — all of
//! which serialise their writers behind a lock and can hold a transaction open
//! across a decision. Neon's `/sql` endpoint can do none of that:
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
//! statements, **type-checks against [`EventStore`](happenstance_core::EventStore)
//! perfectly**. [`ProbeThenWriteStore`] is that implementation, written out in
//! full so the compiling call site can be pointed at. It is also silently wrong
//! here: two statements are two round trips, each its own implicit transaction,
//! with a network-latency-wide window between them and no snapshot spanning it.
//! A conflicting append committed inside that window is invisible to the probe
//! and unopposed by the insert.
//!
//! A shape table that recorded only `error[E….]` would therefore rank this crate
//! the most compatible adapter in the workspace. It is the least. The limits
//! that matter here are not type errors.
//!
//! # Can the condition and the write collapse into one statement?
//!
//! Yes — and, contrary to the standing assumption in the decision ledger, the
//! collapse **keeps** `ConditionViolated::conflicting_position`. The naive
//! `INSERT … SELECT … WHERE NOT EXISTS` cannot: it returns zero rows on
//! conflict and zero rows carry no position. But a CTE can compute both on one
//! snapshot and project them side by side:
//!
//! ```sql
//! WITH probe AS (
//!     SELECT min(position) AS conflict
//!       FROM event
//!      WHERE position > $1              -- AppendCondition::after
//!        AND (<the condition's Query>)
//! ), ins AS (
//!     INSERT INTO event (event_type, data, metadata, tags)
//!     SELECT * FROM unnest($2::text[], $3::bytea[], $4::bytea[], $5::text[][])
//!      WHERE NOT EXISTS (SELECT 1 FROM probe WHERE conflict IS NOT NULL)
//!     RETURNING position
//! )
//! SELECT (SELECT max(position) FROM ins)   AS appended,
//!        (SELECT conflict      FROM probe) AS conflict;
//! ```
//!
//! One statement, one round trip, one snapshot, and a row that says both which
//! position was assigned and — when nothing was — which position conflicted.
//! So Neon, the case the ledger names as forcing `conflicting_position` down to
//! a hint, does not in fact force it. It costs one extra aggregate index scan on
//! every append including the uncontended ones, and it needs
//! [`IsolationLevel::Serializable`] to be sound, which is why
//! [`NeonConfig`]'s default is `Serializable` rather than Postgres'
//! `ReadCommitted`. Both are prices, and neither is a `None`.
//!
//! # Which flavour, and which claim
//!
//! [`NeonEventStore`] implements the **bare**
//! [`EventStore`](happenstance_core::EventStore), and
//! [`NeonProjectionStore`] the bare
//! [`ProjectionStore`](happenstance_core::ProjectionStore). The crate compiles
//! for the host target and for `wasm32-unknown-unknown`, *implementing the bare
//! flavour on each*. That is not the same claim as satisfying both flavours: the
//! contract crate's implication table runs one way only, so nothing here
//! satisfies `SendEventStore`, on either target, even where the transport
//! happens to be `Send`.
//!
//! # It owns no HTTP client
//!
//! [`SqlTransport`] is a one-method trait and [`NullTransport`] is the in-tree
//! implementation. See the [`transport`] module for why: a real client needs a
//! TLS stack on the host and `wasm-bindgen`'s `fetch` on `wasm32`, they are two
//! different clients, and neither is what this crate is here to prove. What that
//! costs is recorded honestly — the crate cannot demonstrate that a licence-clean
//! client exists for both targets, only that the shape above it does not need to
//! know which one it has.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. Scoped here rather than left open in
// the workspace manifest so it is visible in review and disappears with the last
// `todo!()`. Phase 10 removes both the bodies and this line.
#![allow(clippy::todo)]

pub mod config;
pub mod error;
pub mod transport;
pub mod wire;

#[cfg(feature = "event-store")]
pub mod event_store;

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
