//! LadybugDB projection store adapter for happenstance.
//!
//! # Status: skeleton
//!
//! Real types, `todo!()` bodies. It exists so that the projection store port
//! has a *non-SQL, owned-handle* implementer to be falsified by before it is
//! frozen — the far end of the axis every other adapter in this workspace sits
//! on. Phase 11 fills the bodies in and links the real driver.
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
//! [`AppendCondition`](happenstance_core::AppendCondition) demands and what a
//! graph engine optimised for analytical traversal is not built to give.
//! Forcing the event store port onto it would produce something that satisfies
//! the trait and not the specification.
//!
//! # The driver is deliberately absent
//!
//! This adapter is a separate crate rather than a feature flag somewhere else
//! because nobody who only wants SQLite should pay for the driver, and the
//! dependency is still not here because a skeleton whose job is to type-check
//! needs the *shapes*, not the C++ — [`stand_in`] carries those with its sources
//! cited.
//!
//! **What the cost actually is was measured at phase 11 and is not what this
//! paragraph used to say.** It said `lbug` builds LadybugDB's C++ from source
//! through `cxx` and `cmake`. Its `build.rs` tries a **prebuilt download first**
//! and only falls back to the source build; on the machine that measured it the
//! prebuilt path succeeded and cmake was never invoked. What arrives instead is a
//! single **1.44 GB static archive**, plus an OpenSSL toolchain that no feature
//! turns off. `experiments/ladybug-driver-probes/` carries the transcript.
//!
//! That is a different cost with a different remedy, and ADR-0025 takes it:
//! `cargo check` and `cargo clippy` do not link, `cargo test --workspace` does,
//! so the driver ships behind an off-by-default feature and the conformance run
//! is a probed step rather than a workspace one.
//!
//! # What this skeleton establishes
//!
//! 1. **LadybugDB has no transaction handle type.** Transactions are Cypher
//!    statements on a connection, so PS-4's "a `Batch` MUST NOT be required to
//!    be a live transaction" is not a concession here — a deferred write set is
//!    the shape the driver actually offers.
//! 2. **A live borrowed handle was nevertheless expressible on the `Send`
//!    flavour**, because `lbug`'s `Connection` is `Send + Sync`. That
//!    contradicted §4.2's stated reason for dropping the GAT, which generalises
//!    from `rusqlite` alone — so ADR-0017 rested the clause on the two
//!    transcripts below instead. The counter-example is preserved outside the
//!    gate at `experiments/live-handle-projection-batch/`, because the port's
//!    owned `type Batch;` no longer admits it and deleting the only compiled
//!    evidence against a decision is not how this workspace takes one.
//! 3. **The GAT port could not be implemented by a store that carries a
//!    lifetime.** Its `where Self: 'a` made that a region error, and
//!    rustc 1.97.1 **ICEs** while reporting it. Transcripts in the same
//!    experiment directory, and minimised in
//!    `experiments/rustc-ice-gat-foreign-trait/`.
//! 4. `Database` and `Connection` are both `Send + Sync`, so this adapter
//!    implements the
//!    [`SendProjectionStore`](happenstance_core::SendProjectionStore) flavour.
//!
//! # Open decisions
//!
//! * Whether the checkpoint lives in the graph as a node or beside it. As a
//!   node property is what keeps it inside the same `BEGIN TRANSACTION`, which
//!   is the only way to satisfy the port's transactional invariant — so this is
//!   nearly settled. **The narrowing those two error variants were written for
//!   does not exist**: a `UINT64` column round-trips `u64::MAX - 1` exactly, so
//!   `SequencePosition`'s `NonZeroU64` fits whole — unlike the two adapters over
//!   a signed `bigint`. ADR-0025 therefore deletes
//!   [`LadybugProjectionStoreError::PositionOutOfRange`], which no code path
//!   could construct, and narrows
//!   [`LadybugProjectionStoreError::MalformedCheckpoint`] to a stored zero.
//! * Whether `lbug`'s synchronous API is wrapped in `spawn_blocking` or the
//!   adapter is offered as blocking-only. **Settled blocking-only by ADR-0025**,
//!   with runtime-agnosticism falsified by mounting the suite under both the
//!   blocking and the tokio emitters.
//! * How a projection expresses graph mutations: raw Cypher, or a typed
//!   builder. [`GraphStatement`] is the raw-Cypher answer, and it is deliberate
//!   that it is *the port's* answer too.
//!
//!   **This crate is not one of PS-9's or PS-11's data points**, which this
//!   paragraph used to claim. Those clauses' falsifier names *a second generic
//!   consumer* — library code `happenstance` itself ships that must write into an
//!   unknown adapter's batch — and assigns it to another phase. An adapter is
//!   evidence about what the clauses **cost**, which is worth reporting and is a
//!   different thing from the evidence they are waiting on.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. Scoped here rather than left open in
// the workspace manifest so that it is visible in review and disappears with
// the last `todo!()`. Phase 11 removes both the bodies and this line.
#![allow(clippy::todo)]

pub mod projection_store;
pub mod stand_in;

pub use projection_store::{
    GraphStatement, GraphWriteSet, LadybugProjectionStore, LadybugProjectionStoreError,
};
