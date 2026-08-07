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
//! The `lbug` crate builds LadybugDB's C++ from source through `cxx` and
//! `cmake`. That is exactly why this adapter is a separate crate rather than a
//! feature flag somewhere else: nobody who only wants SQLite should pay a
//! multi-minute native build. It is also why the dependency is still not here —
//! a skeleton whose job is to type-check needs the *shapes*, not the C++, and
//! [`stand_in`] carries those with its sources cited. docs.rs itself fails to
//! build `lbug` 0.19.1, which is the same cost seen from outside.
//!
//! # What this skeleton establishes
//!
//! 1. **LadybugDB has no transaction handle type.** Transactions are Cypher
//!    statements on a connection, so PS-4's "a `Batch` MUST NOT be required to
//!    be a live transaction" is not a concession here — a deferred write set is
//!    the shape the driver actually offers.
//! 2. **A live borrowed handle is nevertheless expressible on the `Send`
//!    flavour**, because `lbug`'s `Connection` is `Send + Sync`. This
//!    contradicts §4.2's stated reason for dropping the GAT, which generalises
//!    from `rusqlite` alone. See [`live_handle`].
//! 3. **Today's port cannot be implemented by a store that carries a
//!    lifetime.** The GAT's `where Self: 'a` makes it a region error, and
//!    rustc 1.97.1 **ICEs** while reporting it. Transcript in [`live_handle`].
//! 4. `Database` and `Connection` are both `Send + Sync`, so this adapter
//!    implements the
//!    [`SendProjectionStore`](happenstance_core::SendProjectionStore) flavour.
//!
//! # Open decisions
//!
//! * Whether the checkpoint lives in the graph as a node or beside it. As a
//!   node property is what keeps it inside the same `BEGIN TRANSACTION`, which
//!   is the only way to satisfy the port's transactional invariant — so this is
//!   nearly settled, and what remains is the `INT64`/`NonZeroU64` narrowing
//!   that [`LadybugProjectionStoreError::MalformedCheckpoint`] and
//!   [`LadybugProjectionStoreError::PositionOutOfRange`] exist for.
//! * Whether `lbug`'s synchronous API is wrapped in `spawn_blocking` or the
//!   adapter is offered as blocking-only. See the [`projection_store`] module
//!   documentation for what the store's layout leaves open.
//! * How a projection expresses graph mutations: raw Cypher, or a typed
//!   builder. [`GraphStatement`] is the raw-Cypher answer, and it is deliberate
//!   that it is *the port's* answer too — PS-9 and PS-11 ask whether generic
//!   code needs a write vocabulary on `Batch`, and this crate is one of the two
//!   data points.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. Scoped here rather than left open in
// the workspace manifest so that it is visible in review and disappears with
// the last `todo!()`. Phase 11 removes both the bodies and this line.
#![allow(clippy::todo)]

pub mod live_handle;
pub mod projection_store;
pub mod stand_in;

pub use live_handle::{GraphWriteHandle, LiveHandleProjectionStore};
pub use projection_store::{
    GraphStatement, GraphWriteSet, LadybugProjectionStore, LadybugProjectionStoreError,
};
