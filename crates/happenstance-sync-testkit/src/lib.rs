//! The conformance suite for `happenstance-sync` peers.
//!
//! # Status: four rules, and the shape every later one plugs into
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
//! | [`every_refusal_states_its_class`](rules::every_refusal_states_its_class) | SY-36 | a peer whose errors all state one failure class, whatever the failure |
//! | [`nothing_waiting_means_the_next_pull_is_empty`](rules::nothing_waiting_means_the_next_pull_is_empty) | SY-37 | a peer that says nothing is waiting when its page ran out, not its log |
//!
//! Each rule names a wrong implementation, and that implementation lives in
//! this crate's `tests/wrong_peers.rs`, where the rule is shown to fail it. A
//! rule no peer can fail is decorative.
//!
//! **No rule decodes a payload** (SY-35), and **no rule holds two peers**
//! (SY-8). [`rules`] says how the rules keep both.
//!
//! # Harnesses
//!
//! The emitter is a parameter, as in the event-store family, so the suite never
//! decides which runtime a peer is tested on:
//! [`emit_sync_tokio!`](crate::emit_sync_tokio) (the default),
//! [`emit_sync_blocking!`](crate::emit_sync_blocking) — no runtime and no
//! `Send` bound, which is what a `!Send` peer runs under — and
//! [`emit_sync_wasm!`](crate::emit_sync_wasm).
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
// `sync_suite.rs` and not `suite.rs`: the specification cites the event-store
// family's file by bare name, and `cargo xtask spec-trace` refuses a bare name
// two files answer to.
mod sync_suite;
mod violation;

pub use fixture::{
    MAX_PROVOKED_BATCH_BYTES, MemorySyncFixture, MemorySyncHandle, Refusal, RefusalsOf,
    SyncFixture, batch_over_the_limit, oversized_push_refusal,
};
pub use sync_suite::rules;
pub use violation::Violation;

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
    pub use crate::sync_suite::rules;
    pub use happenstance_testkit::block_on;
}
