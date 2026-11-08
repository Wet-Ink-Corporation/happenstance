//! The conformance suite for `happenstance-sync` peers, and for the stores
//! they ingest into.
//!
//! # Status: two peer families and an ingest family, and the shape every later rule plugs into
//!
//! Published since #108 part E, with `happenstance-sync`, and **unfinished**:
//! phase 13 grows it rule by rule. It is the bar a peer or ingest adapter must
//! clear before it is considered to exist, exactly as `happenstance-testkit` is
//! for a store, and an adapter outside this workspace mounts it the same way.
//! **Pin it exactly**: a new rule is a minor of this crate's number.
//!
//! Publishing settled the two questions this paragraph used to defer. Its
//! `version` is the workspace's, in lockstep until 1.0 — ADR-0066 §4 makes a
//! new rule a minor of *its* number, and lockstep keeps that true while the
//! workspace is `0.y`. And `happenstance-testkit`'s `RuleOutcome` is part of
//! this crate's public API, through every rule's signature.
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
//! | [`event_id_order_is_identical_on_every_peer`](rules::event_id_order_is_identical_on_every_peer) | SY-23 | a peer that dedupes by the highest origin position it has seen, and so drops an origin's lower event that arrives late |
//! | [`peer_declares_its_own_limits`](rules::peer_declares_its_own_limits) | SY-18 | a peer whose declared limits admit an event its ingest then refuses, or that declares below VT-21's floor |
//! | [`peer_takes_a_batch_at_its_declared_limits`](rules::peer_takes_a_batch_at_its_declared_limits) | SY-18 | a peer that refuses a push its own batch limits admit: more events, or more bytes, than it really takes |
//! | [`push_envelope_preserves_group_boundaries`](rules::push_envelope_preserves_group_boundaries) | SY-30 | a peer that merges the groups of a page into one, or strips the guard a group was pushed with |
//! | [`every_refusal_states_its_class`](rules::every_refusal_states_its_class) | SY-36 | a peer whose errors all state one failure class, whatever the failure |
//! | [`nothing_waiting_means_the_next_pull_is_empty`](rules::nothing_waiting_means_the_next_pull_is_empty) | SY-37 | a peer that says nothing is waiting when its page ran out, not its log |
//! | [`ack_never_confirms_a_non_durable_event`](rules::ack_never_confirms_a_non_durable_event) | SY-31 | a peer that confirms an event before it is durable |
//! | [`round_trip_preserves_the_whole_log`](rules::round_trip_preserves_the_whole_log) | SY-28, SY-27 | a peer that withholds from its counterpart the events a type or tag filter matches, among those the rule's log carries |
//!
//! Each rule names a wrong implementation, and that implementation lives in
//! this crate's `tests/wrong_peers.rs` — SY-18's in `tests/wrong_limits.rs`,
//! over [`KvCappedFixture`], its batch rule's in `tests/wrong_peers.rs`,
//! SY-23's in `tests/wrong_orders.rs`, and SY-28's in `tests/wrong_scopes.rs`
//! — where the rule is shown to fail it. A rule no peer can fail is
//! decorative.
//!
//! **No peer or ingest rule decodes a payload** (SY-35), and **no rule in
//! [`rules`] holds two peers** (SY-8). [`rules`] says how the rules keep both. The model family
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
//! [`MemoryIngestFixture`], over `happenstance-sync`'s `MemoryIngestStore`;
//! `happenstance-sqlite`'s spike and the Durable Object mount the family too.
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | [`ingest_preserves_origin_identity`](ingest_rules::ingest_preserves_origin_identity) | VT-5 | a store that re-mints an ingested event |
//! | [`ingest_preserves_the_recorded_time`](ingest_rules::ingest_preserves_the_recorded_time) | VT-9 | a store that restamps an ingested event with its arrival time, keeps whole seconds, holds a time before 1970 as the epoch, holds times non-decreasing with position, binds one time per group, or clamps a future time to its own clock |
//! | [`ingested_events_land_above_the_local_head`](ingest_rules::ingested_events_land_above_the_local_head) | SY-5 | an order-preserving merge, which hides an event below a checkpoint already read past |
//! | [`dedupe_reaches_identity_without_decoding`](ingest_rules::dedupe_reaches_identity_without_decoding) | SY-12, SY-11 | content-keyed dedupe, and a compensation written again on redelivery |
//! | [`an_unheld_own_identity_stops_the_ingest`](ingest_rules::an_unheld_own_identity_stops_the_ingest) | SY-38 | a dropped claim, a check of the batch's head only, a group-by-group commit, a claim hidden by the call's own compensation, and a refused held echo |
//! | [`ingest_group_is_atomic_under_a_mid_group_fault`](ingest_rules::ingest_group_is_atomic_under_a_mid_group_fault) | SY-30 | a group written in chunks without a transaction, a compensation committed apart, a fault answered `Ok`, and an armed fault that never fires |
//! | [`ingest_validates_every_event_not_the_head`](ingest_rules::ingest_validates_every_event_not_the_head) | SY-18 | per-event ceilings checked on each group's head only, a compensation never checked, and a ceiling off by one |
//! | [`ingest_holds_the_call_to_its_tags_per_batch_ceiling`](ingest_rules::ingest_holds_the_call_to_its_tags_per_batch_ceiling) | VT-36, SY-18 | a tags-per-batch ceiling held to each group apart, a compensation left out of the total, a ceiling off by one, and a total refused as per-event, `Park` rather than `Split` |
//! | [`only_the_adjudicator_compensates`](ingest_rules::only_the_adjudicator_compensates) | SY-7 | a compensation minted under an incarnation the store never read, so its log names an author nobody assigned |
//!
//! Each wrong store lives in `tests/wrong_ingest_stores.rs`, where a test runs
//! the whole family against it and asserts it fails exactly its rule.
//!
//! SY-7's rule opens **two** stores, so a fixture must give each store it
//! opens a fresh incarnation, and the rule fails one that does not. It plays
//! the runner above them and holds the adjudicator assignment itself, because
//! no runner in `happenstance-sync` authors a compensation yet; what it holds
//! the store to is the identity a compensation is filed under. Every peer
//! adjudicating, the configuration SY-7 forbids, is run by the clause's
//! falsification test in this crate's own unit tests, which records the two
//! compensations it leaves for one conflict. SY-33's
//! rule, which must exempt SY-38's refusal (ADR-0098), is not written yet; the
//! exemption is carried forward to it.
//!
//! ## Capabilities
//!
//! Three rules need the fixture's co-operation, and say so rather than pass
//! when they do not get it. SY-30's needs
//! [`MID_GROUP_FAULT`](IngestFixture::MID_GROUP_FAULT) and
//! [`arm_mid_group_fault`](IngestFixture::arm_mid_group_fault), because only
//! the adapter can fail its own write between two rows. SY-18's reads
//! [`MAX_EVENT_DATA_LEN`](IngestFixture::MAX_EVENT_DATA_LEN),
//! [`MAX_TAGS_PER_EVENT`](IngestFixture::MAX_TAGS_PER_EVENT) and
//! [`MAX_METADATA_LEN`](IngestFixture::MAX_METADATA_LEN), each mirrored from
//! the adapter's own constant. VT-36's reads
//! [`MAX_TAGS_PER_BATCH`](IngestFixture::MAX_TAGS_PER_BATCH), mirrored the
//! same way; only a store whose call has a tags wall of its own states one,
//! today `happenstance-cloudflare`'s. [`MemoryIngestFixture`] declines the fault in its own words and declares
//! no ceiling, so it reports all three rules skipped; the faithful store in
//! `tests/wrong_ingest_stores.rs` runs them, and `happenstance-sqlite`'s
//! ingest spike runs the first two. The mount also emits
//! `every_declined_capability_is_stated_by_this_fixture`, which fails a
//! fixture whose declension is this crate's default
//! ([`ingest_declensions_are_stated`]).
//!
//! # The write-group family
//!
//! A store that a runner or a [`StoreSyncPeer`](happenstance_sync::StoreSyncPeer)
//! reads from must hand its log back in the writes that made it (SY-39), so a
//! relay forwards each write as one group with its guard (SY-30). The family
//! mounts on the same [`IngestFixture`], whose store must also implement
//! [`WriteGroupStore`](happenstance_sync::WriteGroupStore):
//!
//! ```
//! # macro_rules! ignore { ($($t:tt)*) => {} }
//! # ignore! {
//! happenstance_sync_testkit::write_group_store_conformance!(
//!     happenstance_sync_testkit::MemoryIngestFixture::new()
//! );
//! # }
//! ```
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | [`an_append_reads_back_as_one_group`](write_group_rules::an_append_reads_back_as_one_group) | SY-39 | a store that records no groups, and one that merges adjacent appends |
//! | [`an_ingested_group_keeps_its_boundary_and_guard`](write_group_rules::an_ingested_group_keeps_its_boundary_and_guard) | SY-39, SY-30 | a store that drops an ingested group's guard, or merges what one call ingested |
//! | [`compensation_joins_its_group`](write_group_rules::compensation_joins_its_group) | SY-39, SY-2 | a store that records a compensation as a group of its own |
//! | [`a_group_after_a_redelivery_is_named_by_its_first_landed_row`](write_group_rules::a_group_after_a_redelivery_is_named_by_its_first_landed_row) | SY-39, SY-11 | a store that names a group by the first position it allocated, not the first row it wrote |
//! | [`a_read_never_cuts_a_group`](write_group_rules::a_read_never_cuts_a_group) | SY-39 | a page filled by event, a first group skipped for being too large, and a resume above a row position |
//! | [`pages_chain_on_first_and_end_with_nothing`](write_group_rules::pages_chain_on_first_and_end_with_nothing) | SY-39, SY-37 | a page its budget cut that says nothing is waiting |
//!
//! Each wrong store lives in `tests/wrong_write_group_stores.rs`, where a test
//! runs the whole family against it and asserts the exact set it fails.
//!
//! # The convergence family
//!
//! Not an adapter's bar but a projection's: a fold that declares itself
//! convergent, by implementing
//! [`ConvergentProjection`](happenstance::ConvergentProjection), is run under
//! two interleavings of one set of events from two origins, and its read
//! models must be byte-identical. The rule is called directly over a
//! [`ConvergenceFixture`]; there is no mounting macro, because there is no
//! adapter to mount it beside and no capability to decline.
//!
//! | Rule | Clause | What it rejects |
//! |---|---|---|
//! | [`convergent_projection_is_interleaving_independent`](convergence_rules::convergent_projection_is_interleaving_independent) | SY-20 | `unit-ledger`, and any declared fold whose answer depends on arrival order |
//! | [`non_convergent_projections_are_excluded_not_failed`](convergence_rules::non_convergent_projections_are_excluded_not_failed) | SY-22 | a check with no exclusion, which fails FIFO cost layers, and a check that does not really reorder |
//!
//! A payload is decoded here, and only by the projection under test: a fold
//! reads its events, which is what makes it a fold. The check itself reads no
//! event at all: it compares the bytes of two read models. What the runner
//! reads to select and route events is their type and the projection's scope
//! tags, which is what SY-35 requires. The wrong projections live in
//! `tests/wrong_projections.rs`.
//! [`StockOnHandFixture`] and [`CostLayersFixture`] are the reference cases:
//! one domain, a sum that converges and a FIFO queue that cannot.
//!
//! # Harnesses
//!
//! The emitter is a parameter, as in the event-store family, so the suite never
//! decides which runtime a peer is tested on:
//! [`emit_sync_tokio!`](crate::emit_sync_tokio) (the default),
//! [`emit_sync_blocking!`](crate::emit_sync_blocking) — no runtime and no
//! `Send` bound, which is what a `!Send` peer runs under — and
//! [`emit_sync_wasm!`](crate::emit_sync_wasm). The ingest family has
//! [`emit_ingest_tokio!`](crate::emit_ingest_tokio) (the default),
//! [`emit_ingest_blocking!`](crate::emit_ingest_blocking) and
//! [`emit_ingest_wasm!`](crate::emit_ingest_wasm), the Durable Object's. The
//! write-group family has
//! [`emit_write_group_tokio!`](crate::emit_write_group_tokio) (the default),
//! [`emit_write_group_blocking!`](crate::emit_write_group_blocking) and
//! [`emit_write_group_wasm!`](crate::emit_write_group_wasm).
//!
//! # Failure is a value
//!
//! A rule returns `Result<RuleOutcome, Violation>` and an emitted test returns
//! `Result<(), Violation>`, so a breach reaches the test runner as an error
//! naming its clause. [`Violation`] says why this family does not panic as the
//! other two do.
//!
//! # Features
//!
//! * **`proptest`**, off by default: the model family above. Absent on wasm32
//!   whatever the feature says, because `proptest` does not build there.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(doctest, doc = include_str!("../README.md"))]

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
    MAX_PROVOKED_BATCH_BYTES, MAX_PROVOKED_BATCH_EVENTS, MemorySyncFixture, MemorySyncHandle,
    Refusal, RefusalsOf, SyncFixture, batch_over_the_limit, oversized_push_refusal,
};
pub use kv_capped::{KV_VALUE_CAP, KvCappedFixture, KvCappedPeer, KvPeerError};
pub use sync_suite::{
    BYTE_LOG_REFUSED_REASON, COUNT_LOG_REFUSED_REASON, LOG_REFUSED, NO_CONFIRMATION,
    NO_CONFIRMATION_REASON, NO_SECOND_PAGE, NO_SECOND_PAGE_REASON, rules,
};
pub use violation::Violation;

// The ingest family (#108), declared after the peer families so their lines
// do not move.
mod ingest_fixture;
// `ingest_suite.rs` for `sync_suite.rs`'s reason: a bare name two files answer
// to is one `cargo xtask spec-trace` refuses.
mod ingest_suite;
pub use ingest_fixture::{
    IngestFixture, MemoryIngestFixture, MemoryIngestHandle, UNSTATED_MID_GROUP_FAULT,
    ingest_declensions_are_stated,
};
pub use ingest_suite::{
    MAX_PROVOKED_TAGS, MAX_PROVOKED_TAGS_PER_BATCH, NO_HIDDEN_CLAIM, NO_HIDDEN_CLAIM_REASON,
    NO_PER_EVENT_LIMITS, NO_PER_EVENT_LIMITS_REASON, NO_TAGS_PER_BATCH_LIMIT,
    NO_TAGS_PER_BATCH_LIMIT_REASON, ingest_rules,
};

// The write-group family (#420), declared after the ingest family for the
// same reason, and mounted on its fixture.
mod write_group_suite;
pub use write_group_suite::write_group_rules;

// The convergence family (#141), declared after the others so their lines do
// not move. `convergence_suite.rs` and not `convergence.rs`: `happenstance`
// has a `src/convergence.rs` and a `tests/convergence.rs`, and
// `cargo xtask spec-trace` refuses a bare name two files answer to.
mod convergence_fixture;
mod convergence_suite;
pub use convergence_fixture::{
    ConvergenceFixture, CostLayers, CostLayersFixture, EventOf, Stock, StockOnHand,
    StockOnHandFixture, StoreOf,
};
pub use convergence_suite::convergence_rules;

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

            // CF-18's machine-checked half, emitted here rather than through
            // `$emit` so a caller's emitter cannot drop it:
            // `happenstance_testkit::event_store_conformance!`'s copy carries
            // the argument, and the reason the `cfg_attr` pair stays inert on
            // a native build.
            #[cfg_attr(not(target_arch = "wasm32"), test)]
            #[cfg_attr(target_arch = "wasm32", ::wasm_bindgen_test::wasm_bindgen_test)]
            fn every_declined_capability_is_stated_by_this_fixture()
            -> ::core::result::Result<(), $crate::Violation> {
                $crate::ingest_declensions_are_stated(__ingest_fixture)
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

/// Generates the write-group conformance suite for a store adapter.
///
/// Takes an expression that produces an [`IngestFixture`] whose store also
/// implements [`WriteGroupStore`](happenstance_sync::WriteGroupStore). The
/// rule list comes from
/// [`for_each_write_group_rule!`](crate::for_each_write_group_rule).
///
/// # Examples
///
/// ```
/// # macro_rules! ignore { ($($t:tt)*) => {} }
/// # ignore! {
/// happenstance_sync_testkit::write_group_store_conformance!(
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
/// happenstance_sync_testkit::write_group_store_conformance!(
///     mod_name = blocking_write_group_conformance,
///     emit = happenstance_sync_testkit::emit_write_group_blocking,
///     fixture = super::MyFixture::new()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! write_group_store_conformance {
    (mod_name = $mod_name:ident, emit = $emit:path, fixture = $fixture:expr) => {
        mod $mod_name {
            // Hoisted as in the ingest family; the opaque type names the one
            // extra bound every rule here needs.
            async fn __write_group_fixture()
            -> impl $crate::__private::IngestFixture<Store: $crate::__private::WriteGroupStore>
            {
                $fixture
            }

            $crate::for_each_write_group_rule!($emit);
        }
    };
    (mod_name = $mod_name:ident, fixture = $fixture:expr) => {
        $crate::write_group_store_conformance!(
            mod_name = $mod_name,
            emit = $crate::emit_write_group_tokio,
            fixture = $fixture
        );
    };
    ($fixture:expr) => {
        $crate::write_group_store_conformance!(
            mod_name = write_group_conformance,
            emit = $crate::emit_write_group_tokio,
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
    pub use crate::write_group_suite::write_group_rules;
    pub use happenstance_sync::WriteGroupStore;
    pub use happenstance_testkit::block_on;
}
