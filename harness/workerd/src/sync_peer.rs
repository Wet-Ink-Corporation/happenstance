//! The Durable Object as a sync peer on `workerd` (#130): every rule of
//! `for_each_sync_peer_rule!`, run inside the object, through the routes
//! `/sync-peer-rules` and `/sync-peer-rule/<name>`.
//!
//! The peer under test is the one a client reaches over HTTP: a
//! `RemoteSyncPeer` whose every pull and push is a real `worker::Request`
//! answered by `happenstance_cloudflare::sync_http::serve_sync_request` over
//! `CloudflareIngestStore`, the handler the object mounts at `/sync/pull` and
//! `/sync/push` ([`fixture`] says what is and is not the network). Each
//! fixture instance wraps one of the ingest family's fixtures, so its log is
//! its own namespace, built by the published `namespaced_from_object_state`,
//! and each push is one real `ctx.storage.transactionSync`.
//!
//! The rules run through the ingest dispatcher's opener (its `run_family` with
//! `Family::SyncPeer`).
//!
//! # The guard against a bespoke emitter dropping rules
//!
//! [`crate::ingest::dispatch`]'s: [`rule_names`] expands the enumeration through
//! the testkit's `__sync_rule_names` callback, not through the emitter, so a
//! rule missing from the dispatch `match` answers
//! [`Dispatched::Unknown`], and the runner fails it by name.
//!
//! # No declension check
//!
//! `sync_peer_conformance!` emits one since #448, over `SyncFixture::REOPEN`,
//! and this runner does not mount the macro, so it runs none. The fixture
//! states its declension anyway, with its own reason rather than the default's
//! ([`fixture::WorkerdSyncFixture`]'s `REOPEN`): each connect already
//! re-derives its store from the object's state, and restarting the object is
//! beyond a runner inside one request. So SY-31's rule runs without a reopen,
//! as it did before the capability existed. A rule may still answer `skipped` with
//! the reason the suite writes for it (`NO_SECOND_PAGE`, `NO_CONFIRMATION`),
//! and the runner fails every skip but one: under `PeerLimits::minimum` this
//! peer shows a page boundary and confirms what it acknowledges, so every rule
//! must answer `ran`, except SY-32's.
//!
//! # The one tolerated skip (D10, #158)
//!
//! `retention_gap_is_reported_not_silent` skips here, with the fixture's own
//! `COMPACTION_DECLINED` reason and no other. The object serves a
//! `StoreSyncPeer`, and a store peer cannot read a retention floor through
//! the port (ADR-0028), so its fixture declines `COMPACTION`; giving the
//! Durable Object a compaction to offer is follow-up F1. The runner accepts
//! that skip for that rule with that exact text, and the host test below
//! holds the runner's copy of the text to the testkit's const.

use happenstance_cloudflare::worker::State;
use happenstance_cloudflare::worker::worker_sys::DurableObjectState;

use crate::dispatch::Dispatched;
use crate::ingest::dispatch::{Family, run_family};

pub mod fixture;

/// Every name the runner must run, from the sync family's own enumeration.
#[must_use]
pub fn rule_names() -> Vec<&'static str> {
    happenstance_sync_testkit::for_each_sync_peer_rule!(
        happenstance_sync_testkit::__sync_rule_names
    )
    .to_vec()
}

/// Runs the sync rule called `rule` inside the object whose state is `state`,
/// and whose raw state is `object_state` (the same JavaScript object), as
/// [`crate::ingest::dispatch::run`] runs an ingest rule.
pub async fn run(state: &State, object_state: &DurableObjectState, rule: &str) -> Dispatched {
    run_family(state, object_state, Family::SyncPeer, rule).await
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::rule_names;

    /// The names are the enumeration, in its order, each once: the runner
    /// names one object per rule from them.
    #[test]
    fn the_names_are_the_sync_peer_enumeration_each_once() {
        let names = rule_names();
        let enumerated = happenstance_sync_testkit::for_each_sync_peer_rule!(
            happenstance_sync_testkit::__sync_rule_names
        );
        let distinct: BTreeSet<_> = names.iter().collect();
        assert_eq!(distinct.len(), names.len(), "no name appears twice");
        assert_eq!(names, enumerated);
        assert!(names.contains(&"round_trip_preserves_the_whole_log"));
    }

    /// The one skip the runner tolerates is SY-32's, for the reason the
    /// object's fixture states when it declines compaction, and the runner
    /// names that reason word for word: a reworded reason, or a fixture that
    /// starts declining for another one, turns this red on the host before the
    /// `workerd` job sees it (D10). The reason is the fixture's own, never the
    /// suite's default, which `sync_declensions_are_stated` refuses (#480).
    #[test]
    fn the_one_tolerated_skip_is_compaction_in_the_fixtures_words() {
        use happenstance_sync_testkit::{SyncFixture as _, UNSTATED_COMPACTION};

        use super::fixture::{COMPACTION_DECLINED, WorkerdSyncFixture};

        assert_eq!(
            WorkerdSyncFixture::COMPACTION.reason(),
            Some(COMPACTION_DECLINED),
            "the object's fixture declines compaction in its own words"
        );
        assert_ne!(
            COMPACTION_DECLINED, UNSTATED_COMPACTION,
            "the stated reason is not the suite's default"
        );
        let runner = include_str!("../test/sync-peer.test.ts");
        assert!(
            runner.contains(COMPACTION_DECLINED),
            "test/sync-peer.test.ts tolerates the skip only with this exact reason"
        );
        assert!(
            !runner.contains(UNSTATED_COMPACTION),
            "test/sync-peer.test.ts no longer tolerates the suite's default reason"
        );
        assert!(
            rule_names().contains(&"retention_gap_is_reported_not_silent"),
            "the tolerated rule is in the enumeration"
        );
    }
}
