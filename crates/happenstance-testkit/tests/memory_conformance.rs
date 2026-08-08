//! Validates the conformance suite against the reference implementation.
//!
//! This runs in both directions at once. It checks that
//! [`MemoryEventStore`](happenstance_core::MemoryEventStore) is DCB-compliant, and —
//! more importantly — it checks that the suite itself is sane. A rule that no
//! correct store can pass is worse than no rule at all, because an adapter
//! author will spend a day believing their code is broken.
//!
//! [`MemoryFixture`] is what the suite is actually handed: one fixture instance
//! is one `MemoryEventStore` behind an `Arc`, and each `connect` is a refcount
//! clone. It supports a second handle and declines `REOPEN`, so this harness is
//! also where the skip machinery is exercised end to end — one rule reports a
//! skip on every run rather than vanishing from the binary.
//!
//! The `fixture_isolation` module at the bottom is CF-15's meta-test: the proof
//! that the isolation *rule* can fail, which is a different obligation from the
//! rule itself and is discharged in a different place. See its documentation.
//!
//! The tokio harness, so: native only. The wasm32 build of this same suite is
//! `memory_conformance_wasm.rs`, and the runtime-free one is
//! `memory_conformance_blocking.rs`.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_testkit::fixtures::MemoryFixture;

happenstance_testkit::event_store_conformance!(MemoryFixture::new());

/// CF-15's other half: the proof that the isolation rule can fail at all.
///
/// The split between this and
/// [`two_fixture_instances_observe_none_of_each_others_appends`][rule] is
/// deliberate and is the point of the clause. The mistake CF-15 names — a
/// file-backed fixture that points every instance at one temporary path — is an
/// **adapter's**, so only a rule the adapter runs can catch it; a meta-test over
/// the testkit's own fixture never sees an adapter's fixture. What a meta-test
/// *can* do is what CLAUDE.md's corollary demands of every rule: name a wrong
/// implementation and show the rule rejects it. A rule no implementation can
/// fail is decorative.
///
/// [`MemoryFixture::sharing`] is that wrong implementation, reproduced in
/// memory: hand every instance the same `Arc` and the two "isolated" stores are
/// one.
///
/// [rule]: happenstance_testkit::rules::two_fixture_instances_observe_none_of_each_others_appends
mod fixture_isolation {
    #![allow(clippy::unwrap_used)]

    use std::sync::Arc;

    use happenstance_core::MemoryEventStore;
    use happenstance_testkit::fixtures::MemoryFixture;

    #[tokio::test]
    async fn the_reference_fixture_is_isolated() {
        // The control. Without it the `#[should_panic]` below would also pass if
        // the rule panicked for some reason having nothing to do with isolation.
        happenstance_testkit::rules::two_fixture_instances_observe_none_of_each_others_appends(
            || async { MemoryFixture::new() },
        )
        .await
        .report("two_fixture_instances_observe_none_of_each_others_appends");
    }

    #[tokio::test]
    #[should_panic(expected = "two fixture instances must share no backing store")]
    async fn a_shared_backing_fixture_fails_the_isolation_rule() {
        let shared = Arc::new(MemoryEventStore::new());

        // A capturing `async ||` closure, which is also the shape a caller-side
        // registry would use — `AsyncFn` is satisfied by it exactly as it is by
        // the `async fn` item the emitters pass.
        let open = async || MemoryFixture::sharing(Arc::clone(&shared));

        happenstance_testkit::rules::two_fixture_instances_observe_none_of_each_others_appends(
            open,
        )
        .await
        .report("two_fixture_instances_observe_none_of_each_others_appends");
    }
}
