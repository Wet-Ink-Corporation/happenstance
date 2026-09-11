//! LadybugDB projection store adapter for happenstance.
//!
//! # Status: an adapter, behind an off-by-default feature
//!
//! It stopped being a skeleton at phase 11: real bodies, the real driver, and
//! the projection conformance suite run against a real LadybugDB directory.
//! What it is *for* has not changed — the projection store port needed a
//! **non-SQL** implementer to be falsified by before it is frozen — and what it
//! turned out to be worth is stated narrowly in
//! [What this establishes](#what-this-establishes) rather than claimed widely.
//!
//! `publish = false` **stands**, and not for the usual skeleton reason. See
//! [Why this crate is not published](#why-this-crate-is-not-published).
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
//! # The driver is behind `feature = "driver"`, and what that costs
//!
//! This adapter is a separate crate rather than a feature flag somewhere else
//! because nobody who only wants SQLite should pay for the driver. **Without
//! `driver` this crate is its documentation**: every item below names an `lbug`
//! type, so all of them are `cfg`-ed out and what remains compiles instantly.
//!
//! **What the cost actually is was measured at phase 11 and is not what this
//! paragraph used to say.** It said `lbug` builds LadybugDB's C++ from source
//! through `cxx` and `cmake`. Its `build.rs` tries a **prebuilt download first**
//! and only falls back to the source build; on the machine that measured it the
//! prebuilt path succeeded and cmake was never invoked. What arrives instead is a
//! single **1.44 GB static archive**, plus an OpenSSL toolchain that no feature
//! turns off. `experiments/ladybug-driver-probes/` carries the transcript, and
//! the trap worth more than the fix: `lbug`'s `build.rs` emits no
//! `cargo:rerun-if-env-changed` for `OPENSSL_DIR`, so setting it after a failed
//! build changes nothing until `cargo clean -p lbug`.
//!
//! That is a different cost with a different remedy, and ADR-0025 §9 takes it:
//! `cargo check` and `cargo clippy` do not link, `cargo test --workspace
//! --all-features` does, so the driver ships behind an off-by-default feature,
//! the workspace steps exclude this crate, and the conformance run is a probed
//! step that prints `skipped` when the driver is not configured — the same shape
//! `cargo deny` already has.
//!
//! # Why this crate is not published
//!
//! `publish = false` is kept, and the reason is upstream rather than local: **the
//! driver cannot render on docs.rs.** `lbug`'s `build.rs` returns early under
//! `DOCS_RS` — *"we're just building docs and don't need the C++ library"* —
//! **before** it emits the two `cargo:rustc-env=LBUG_PRECOMPILED_*` lines that
//! its own `src/lib.rs` reads with `env!`, and an undefined `env!` is a compile
//! error rather than a missing page. So a published `happenstance-ladybug` would
//! carry a feature whose documentation can never build, on a host where a bad
//! version can be yanked but never removed.
//!
//! Measured rather than reasoned about. `DOCS_RS=1 cargo check -p
//! happenstance-ladybug --features driver`:
//!
//! ```text
//! error: environment variable `LBUG_PRECOMPILED_SOURCE` not defined at compile time
//!   --> lbug-0.20.3/src/lib.rs:95:39
//! error: environment variable `LBUG_PRECOMPILED_LIBRARY_DIR` not defined at compile time
//!   --> lbug-0.20.3/src/lib.rs:97:36
//! ```
//!
//! Removing `publish = false` would therefore oblige more than the usual four
//! things — `PUBLISHABLE` in `xtask/src/package.rs`, two licence files, a README
//! and docs.rs metadata, with `reconcile` failing on either half of the first
//! alone. It would oblige a way to render a page for a feature that cannot be
//! compiled on the host that renders it, and this crate does not have one.
//!
//! # What this establishes
//!
//! Bounded up front, because ADR-0025 §8 bounds it: Ladybug is the **fifth**
//! owned-buffered-batch implementer, not PS-2's second shape.
//! `MemoryProjectionStore`, the testkit's buffering variant,
//! `SqliteProjectionStore`, `examples/outside-projection-adapter` and now this
//! one all buffer. Phase 10b established that the axis's other end was, as the
//! port then stood, **forbidden by its signatures** for both drivers PS-2 names;
//! ADR-0062 moved those signatures and `happenstance-postgres` built that end,
//! which is what let ADR-0063 freeze the port. None of that was this crate's to
//! do. What phase 11 fills is the **write-vocabulary** axis — Cypher rather than
//! SQL, a graph rather than tables.
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
//!    **Re-checked against the real types** in `tests/send_and_sync.rs`, which is
//!    the stand-in module's four assertions re-pointed rather than deleted with
//!    it: they reach the same conclusion by a different mechanism, because
//!    `unsafe_code = "forbid"` made the stand-in get `Send + Sync` by
//!    construction where `lbug` gets it by `unsafe impl` over a C++ pointer.
//! 5. **The engine enforces the atomicity PS-1 asks for, rather than this
//!    adapter remembering to ask for it.** On a statement error inside a
//!    transaction LadybugDB aborts the whole transaction itself and then refuses
//!    a `ROLLBACK`, so `commit`'s error path must not issue one — issuing it
//!    produces a second error that masks the first (ADR-0025 §7).
//!
//! # Decisions ADR-0025 closed
//!
//! Kept as a list rather than deleted, because each entry names what the
//! alternative was, and a reader arriving from an older commit will otherwise
//! find only the answer.
//!
//! * **The checkpoint lives in the graph, as a node, over `UINT64`.** In the
//!   graph rather than beside it because a `BEGIN TRANSACTION` … `COMMIT` on one
//!   connection is the only atomicity LadybugDB offers, and a sidecar file would
//!   put the two halves in two failure domains. **The narrowing two error
//!   variants were written for does not exist**: a `UINT64` column round-trips
//!   `u64::MAX - 1` exactly, so `SequencePosition`'s `NonZeroU64` fits whole —
//!   unlike the two adapters over a signed `bigint`. So `PositionOutOfRange`,
//!   which no code path could construct, is **deleted**, and
//!   [`LadybugProjectionStoreError::MalformedCheckpoint`] narrows to a stored
//!   zero.
//! * **Blocking-only.** No `tokio`, no `spawn-blocking` feature at `0.2.0`, and
//!   runtime-agnosticism is *falsified* by mounting the suite under both the
//!   blocking and the tokio emitters rather than asserted in a sentence.
//! * **Raw parameterised Cypher, not a typed builder.** [`GraphWriteSet`] is the
//!   raw-Cypher answer, and it is deliberate that it is *the port's* answer too:
//!   PS-9 says a projection writes through the adapter's **inherent** API, so a
//!   builder would bind nobody outside this crate and would need a `raw()`
//!   escape hatch anyway — at which point it is raw Cypher plus a maintenance
//!   burden. The `&'static str` seam that decides *provenance* is on
//!   [`GraphWriteSet::push`], with `push_raw_cypher` beside it, so that reaching
//!   for the unconstrained form is a decision.
//!
//!   **This crate is not one of PS-9's or PS-11's data points**, which this
//!   paragraph used to claim. Those clauses' falsifier names *a second generic
//!   consumer* — library code `happenstance` itself ships that must write into an
//!   unknown adapter's batch — and assigns it to another phase. An adapter is
//!   evidence about what the clauses **cost**, which is worth reporting and is a
//!   different thing from the evidence they are waiting on.

#![doc(html_no_source)]

#[cfg(feature = "driver")]
pub mod projection_store;

#[cfg(feature = "driver")]
pub use projection_store::{
    GraphStatement, GraphWriteSet, LadybugProjectionStore, LadybugProjectionStoreError,
};

/// The LadybugDB driver, re-exported so that a caller can name the types this
/// adapter's API is written in without a second version requirement of its own.
///
/// `happenstance-sqlite` re-exports `rusqlite` for the same reason and it is the
/// same obligation: [`GraphWriteSet::push`] takes `lbug::Value` parameters, so a
/// consumer that cannot name `lbug::Value` cannot call it.
#[cfg(feature = "driver")]
pub use lbug;
