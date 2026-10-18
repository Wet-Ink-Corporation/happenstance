//! The conformance suite for `happenstance-sync` peers, and for the stores
//! they ingest into.
//!
//! # Status: two peer families and an ingest family, and the shape every later rule plugs into
//!
//! `publish = false` because the suite is **unfinished**, not because it is
//! held back: phase 13 grows it rule by rule, and whether it publishes is
//! decided with `happenstance-sync`'s own release. Until then it is the bar a
//! peer adapter in this workspace must clear before it is considered to exist,
//! exactly as `happenstance-testkit` is for a store.
//!
//! Two things are decided before it publishes and deliberately not here: its
//! `version` key — the workspace's today, where CF-32 gives
//! `happenstance-testkit` its own, and ADR-0066 §4 makes a new rule in this
//! crate a minor of *its* number — and whether `happenstance-testkit`'s
//! `RuleOutcome` stays exposed through every rule's signature.
//!
//! # Mounting it
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_sync_testkit::sync_peer_conformance!(
//!     happenstance_sync_testkit::MemorySyncFixture::new()
//! );
//! # }
//! ```
//!
//! The expression builds a [`SyncFixture`]: one isolated backing peer per
//! instance. It is evaluated **inside the generated module**, so a fixture type
//! defined beside the invocation is named `super::MyFixture`; the macro does not
//! glob-import its parent, because an unused glob would be a warning in every
//! crate that named its fixture by full path.
//!
//! # What is checked
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | [`ingest_never_rejects`](rules::ingest_never_rejects) | SY-1 | a peer that evaluates the travelling guard and refuses or drops a conflicting fact |
//! | [`compensation_is_atomic_with_the_losing_event`](rules::compensation_is_atomic_with_the_losing_event) | SY-2 | a peer that splits a losing event from the compensation that rode with it |
//! | [`peer_declares_its_own_limits`](rules::peer_declares_its_own_limits) | SY-18 | a peer whose declared limits admit an event its ingest then refuses, or that declares below VT-21's floor |
//! | [`every_refusal_states_its_class`](rules::every_refusal_states_its_class) | SY-36 | a peer whose errors all state one failure class, whatever the failure |
//! | [`nothing_waiting_means_the_next_pull_is_empty`](rules::nothing_waiting_means_the_next_pull_is_empty) | SY-37 | a peer that says nothing is waiting when its page ran out, not its log |
//!
//! Each rule names a wrong implementation, and that implementation lives in
//! this crate's `tests/wrong_peers.rs` — SY-18's in `tests/wrong_limits.rs`,
//! over [`KvCappedFixture`] — where the rule is shown to fail it. A rule no
//! peer can fail is decorative.
//!
//! **No rule decodes a payload** (SY-35), and **no rule in [`rules`] holds two
//! peers** (SY-8). [`rules`] says how the rules keep both. The model family
//! below is the one exception to the second, and its module says why.
//!
//! # The model family
//!
//! Behind the off-by-default `proptest` feature, and absent on wasm32 because
//! `proptest` does not build there. `sync_model` generates peer topologies of
//! three to six peers — chains whose ends never meet included — and drives them
//! from one seed through writes, partitions, lost acks, duplicated pushes and
//! delayed pages, then checks every peer at quiescence against a model of its
//! own. It opens **one fixture per node**, so the fixture expression must build
//! a fresh, isolated peer each time it is evaluated.
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | `transitive_convergence_over_a_partial_mesh` | SY-24, SY-4 | a peer that quarantines what it accepted, or never forwards it |
//! | `redelivery_of_an_accepted_group_is_a_no_op` | SY-11 | a peer that dedupes per batch (SF-03), or miscounts its acknowledgement |
//! | `forwarding_order_is_arrival_order` | SY-25 | a peer that re-sorts what it forwards |
//!
//! The names are written out rather than linked, because the module they live
//! in does not exist without the feature. A failure names its seed in hex and
//! the smallest scenario it shrank to; `tests/wrong_topologies.rs` pins every
//! rule's verdict against four wrong peers.
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! #[cfg(not(target_arch = "wasm32"))]
//! happenstance_sync_testkit::sync_model_conformance!(
//!     happenstance_sync_testkit::MemorySyncFixture::new()
//! );
//! # }
//! ```
//!
//! # The ingest family
//!
//! The other side of a peer: the store an
//! [`IngestStore`](happenstance_sync::IngestStore) writes into. A store adapter
//! that implements the port mounts it beside the event-store suite:
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_sync_testkit::ingest_store_conformance!(
//!     happenstance_sync_testkit::MemoryIngestFixture::new()
//! );
//! # }
//! ```
//!
//! The expression builds an [`IngestFixture`], whose store is both an
//! `EventStore` and an `IngestStore`. The reference is
//! [`MemoryIngestFixture`], over `happenstance-sync`'s `MemoryIngestStore`, and
//! `happenstance-sqlite`'s ingest spike mounts the family too.
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | [`ingest_preserves_origin_identity`](ingest_rules::ingest_preserves_origin_identity) | VT-5, VT-9 | a store that re-mints an ingested event, or restamps its recorded time |
//! | [`ingested_events_land_above_the_local_head`](ingest_rules::ingested_events_land_above_the_local_head) | SY-5 | an order-preserving merge, which hides an event below a checkpoint already read past |
//! | [`dedupe_reaches_identity_without_decoding`](ingest_rules::dedupe_reaches_identity_without_decoding) | SY-12, SY-11 | content-keyed dedupe, and a compensation written again on redelivery |
//! | [`an_unheld_own_identity_stops_the_ingest`](ingest_rules::an_unheld_own_identity_stops_the_ingest) | SY-38 | a dropped claim, a check of the batch's head only, a group-by-group commit, a claim hidden by the call's own compensation, and a refused held echo |
//!
//! Each wrong store lives in `tests/wrong_ingest_stores.rs`, where a test runs
//! the whole family against it and asserts it fails exactly its rule. SY-33's
//! rule, which must exempt SY-38's refusal (ADR-0098), is not written yet; the
//! exemption is carried forward to it.
//!
//! # Harnesses
//!
//! The emitter is a parameter, as in the event-store family, so the suite never
//! decides which runtime a peer is tested on:
//! [`emit_sync_tokio!`](crate::emit_sync_tokio) (the default),
//! [`emit_sync_blocking!`](crate::emit_sync_blocking) — no runtime and no
//! `Send` bound, which is what a `!Send` peer runs under — and
//! [`emit_sync_wasm!`](crate::emit_sync_wasm). The ingest family has
//! [`emit_ingest_tokio!`](crate::emit_ingest_tokio) (the default) and
//! [`emit_ingest_blocking!`](crate::emit_ingest_blocking); its `wasm32`
//! emitter arrives with the first store that runs it there.
//!
//! # Failure is a value
//!
//! A rule returns `Result<RuleOutcome, Violation>` and an emitted test returns
//! `Result<(), Violation>`, so a breach reaches the test runner as an error
//! naming its clause. [`Violation`] says why this family does not panic as the
//! other two do.

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod fixture;
mod kv_capped;
// `sync_suite.rs` and not `suite.rs`: the specification cites the event-store
// family's file by bare name, and `cargo xtask spec-trace` refuses a bare name
// two files answer to.
mod sync_suite;
mod violation;
// Declared after the existing modules so their line numbers do not move. The
// gate is `happenstance-testkit`'s `model`'s: `proptest` does not build for
// wasm32, and a Cargo feature is not target-scoped.
#[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
#[cfg_attr(docsrs, doc(cfg(feature = "proptest")))]
pub mod sync_model;

pub use fixture::{
    MAX_PROVOKED_BATCH_BYTES, MemorySyncFixture, MemorySyncHandle, Refusal, RefusalsOf,
    SyncFixture, batch_over_the_limit, oversized_push_refusal,
};
pub use kv_capped::{KV_VALUE_CAP, KvCappedFixture, KvCappedPeer, KvPeerError};
pub use sync_suite::{NO_SECOND_PAGE, NO_SECOND_PAGE_REASON, rules};
pub use violation::Violation;

// The ingest family (#108), declared after the peer families so their lines
// do not move.
mod ingest_fixture;
// `ingest_suite.rs` for `sync_suite.rs`'s reason: a bare name two files answer
// to is one `cargo xtask spec-trace` refuses.
mod ingest_suite;
pub use ingest_fixture::{IngestFixture, MemoryIngestFixture, MemoryIngestHandle};
pub use ingest_suite::{NO_HIDDEN_CLAIM, NO_HIDDEN_CLAIM_REASON, ingest_rules};

/// Generates the ingest conformance suite for a store adapter.
///
/// Takes an expression that produces an [`IngestFixture`]. The rule list is
/// not written here; it comes from
/// [`for_each_ingest_store_rule!`](crate::for_each_ingest_store_rule), which
/// is the only place it is written at all.
///
/// # Examples
///
/// The default harness, `#[tokio::test]`:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::ingest_store_conformance!(
///     happenstance_sync_testkit::MemoryIngestFixture::new()
/// );
/// # }
/// ```
///
/// A different harness, and a module name of your own:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::ingest_store_conformance!(
///     mod_name = blocking_ingest_conformance,
///     emit = happenstance_sync_testkit::emit_ingest_blocking,
///     fixture = super::MyFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! ingest_store_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            // Hoisted behind a function, as in the other families, so an
            // emitter needs to know nothing about the fixture's type.
            // Re-evaluated per call, so every arrangement gets its own fixture.
            async fn __ingest_fixture() -> impl $crate::__private::IngestFixture {
                $fixture
            }

            $crate::for_each_ingest_store_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::ingest_store_conformance!(
            mod_name = $mod_name,
            emit = $crate::emit_ingest_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::ingest_store_conformance!(
            mod_name = ingest_conformance,
            emit = $crate::emit_ingest_tokio,
            fixture = $fixture
        );
    };
}

/// Generates the sync conformance suite for a peer adapter.
///
/// Takes an expression that produces a [`SyncFixture`]. The rule list is not
/// written here; it comes from
/// [`for_each_sync_peer_rule!`](crate::for_each_sync_peer_rule), which is the
/// only place it is written at all.
///
/// # Examples
///
/// The default harness, `#[tokio::test]`:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::sync_peer_conformance!(
///     happenstance_sync_testkit::MemorySyncFixture::new()
/// );
/// # }
/// ```
///
/// A different harness, and a module name of your own so two mounts can share
/// a file:
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::sync_peer_conformance!(
///     mod_name = blocking_sync_conformance,
///     emit = happenstance_sync_testkit::emit_sync_blocking,
///     fixture = super::MyFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! sync_peer_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            // Hoisted behind a function, as in the event-store family, so an
            // emitter needs to know nothing about the fixture's type; `async`
            // because a real fixture's construction is I/O. Re-evaluated per
            // test, so every rule gets its own fixture.
            async fn __conformance_fixture() -> impl $crate::__private::SyncFixture {
                $fixture
            }

            $crate::for_each_sync_peer_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::sync_peer_conformance!(
            mod_name = $mod_name,
            emit = $crate::emit_sync_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::sync_peer_conformance!(
            mod_name = sync_conformance,
            emit = $crate::emit_sync_tokio,
            fixture = $fixture
        );
    };
}

/// What the macro expansions name, so a caller need not have this crate's
/// dependencies in scope under any particular name. Not promised.
#[doc(hidden)]
pub mod __private {
    pub use crate::fixture::SyncFixture;
    pub use crate::ingest_fixture::IngestFixture;
    pub use crate::ingest_suite::ingest_rules;
    #[cfg(all(feature = "proptest", not(target_arch = "wasm32")))]
    pub use crate::sync_model::rules as model_rules;
    pub use crate::sync_suite::rules;
    pub use happenstance_testkit::block_on;
}
