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
//! `sync_peer_conformance!` emits none. A rule may still answer `skipped` with
//! the reason the suite writes for it (`NO_SECOND_PAGE`, `NO_CONFIRMATION`),
//! and the runner fails any skip: under `PeerLimits::minimum` this peer shows
//! a page boundary and confirms what it acknowledges, so every rule must
//! answer `ran`.

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
}
