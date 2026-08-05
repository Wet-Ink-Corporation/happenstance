//! LadybugDB projection store adapter for eventum.
//!
//! # Status: not implemented
//!
//! # Why projections only
//!
//! [LadybugDB](https://ladybugdb.com/) is an embedded columnar property-graph
//! database with a Cypher interface — the successor to KuzuDB. It is a
//! genuinely interesting read-model target: the "which students share a course
//! with whom" question that costs a pile of joins in SQL is one hop in a graph.
//!
//! It is a poor event store, though, and this crate will not offer one. An
//! event log needs a monotonic append with a conditional write, which is what
//! [`AppendCondition`](eventum_core::AppendCondition) demands and what a graph
//! engine optimised for analytical traversal is not built to give. Forcing the
//! event store port onto it would produce something that satisfies the trait
//! and not the specification.
//!
//! # Build cost
//!
//! The `lbug` crate builds LadybugDB's C++ from source through `cxx` and
//! `cmake`. That is exactly why this adapter is a separate crate rather than a
//! feature flag somewhere else: nobody who only wants SQLite should pay a
//! multi-minute native build. The dependency is added when the adapter is
//! implemented.
//!
//! # Open decisions
//!
//! * Whether the checkpoint lives in the graph as a node or beside it. The
//!   checkpoint invariant in
//!   [`ProjectionStore`](eventum_core::ProjectionStore) requires the read-model
//!   write and the checkpoint write to commit together, so this hinges on what
//!   Ladybug's transaction API actually guarantees — the question the port
//!   needs answering before it can be frozen.
//! * How a projection expresses graph mutations: raw Cypher, or a typed builder.
//! * Whether `lbug`'s synchronous API is wrapped in `spawn_blocking` or the
//!   adapter is offered as blocking-only.

#![doc(html_no_source)]

/// How the Ladybug projection store fails.
///
/// # Status: not implemented
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LadybugProjectionStoreError {
    /// Placeholder variant; replaced by real failure modes on implementation.
    #[error("the LadybugDB projection store is not implemented yet")]
    Unimplemented,
}
