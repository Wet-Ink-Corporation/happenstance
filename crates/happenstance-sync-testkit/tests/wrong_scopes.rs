//! SY-27 and SY-28's rule against a peer that replicates a slice of its log.
//!
//! `wrong_peers.rs`'s shape, in a file of its own because the mistake carries
//! a value: the [`Query`] whose matches the peer withholds, which `Mistake`,
//! being `Copy` and matched exhaustively there, has no room for. Each test runs
//! the **whole** enumeration, so the mutant must fail
//! `round_trip_preserves_the_whole_log` and pass every other rule; the wrapper
//! with no scope is the control.
//!
//! The mutant is the scoped replication ADR-0027 kept off the 1.0 port: a hub
//! that serves its counterpart only what a filter admits, which is also what a
//! filtered spoke looks like from the side that pulls from it. It stores every
//! push whole and acknowledges it truthfully, so the only place the slice shows
//! is the log a pull hands back, and that is where the rule looks.

#![cfg(not(target_arch = "wasm32"))]

use std::sync::Arc;

use happenstance_core::{EventType, Query, QueryItem, StoreId, Tag, Tags};
use happenstance_sync::{
    Ack, EventGroup, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch,
    ReplicatedEvent, SendSyncPeer,
};
use happenstance_sync_testkit::{RefusalsOf, SyncFixture, oversized_push_refusal, rules};
use happenstance_testkit::{RuleOutcome, block_on};

/// The reference peer, withholding from every pull the events `withheld`
/// matches. `None` withholds nothing: the control.
#[derive(Debug)]
struct ScopedPeer {
    inner: Arc<MemorySyncPeer>,
    withheld: Option<Query>,
}

impl ScopedPeer {
    /// Whether this peer keeps `event` out of what it hands back.
    fn withholds(&self, event: &ReplicatedEvent) -> bool {
        self.withheld
            .as_ref()
            .is_some_and(|scope| scope.matches(event.event.event_type(), event.event.tags()))
    }
}

impl SendSyncPeer for ScopedPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    /// The inner peer's pull with the slice taken out. A page the scope
    /// empties is skipped rather than handed back empty, as a scope evaluated
    /// page by page would do, so an empty batch still means the log ran out
    /// and the mutant withholds the slice and nothing else.
    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        let mut pulled = self.inner.pull(from).await?;
        loop {
            if pulled.batch.groups.is_empty() {
                return Ok(pulled);
            }
            let groups: Vec<EventGroup> = core::mem::take(&mut pulled.batch.groups)
                .into_iter()
                .filter_map(|group| {
                    let events: Vec<_> = group
                        .events
                        .into_iter()
                        .filter(|event| !self.withholds(event))
                        .collect();
                    (!events.is_empty()).then(|| EventGroup::new(group.guard, events))
                })
                .collect();
            if !groups.is_empty() {
                pulled.batch.groups = groups;
                return Ok(pulled);
            }
            pulled = self.inner.pull(Some(&pulled.resume)).await?;
        }
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.inner.push(batch).await
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

#[derive(Debug)]
struct ScopedFixture {
    inner: Arc<MemorySyncPeer>,
    withheld: Option<Query>,
}

impl ScopedFixture {
    fn new(withheld: Option<Query>) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x79; 16]))),
            withheld,
        }
    }
}

impl SyncFixture for ScopedFixture {
    type Peer = ScopedPeer;

    async fn connect(&self) -> Self::Peer {
        ScopedPeer {
            inner: Arc::clone(&self.inner),
            withheld: self.withheld.clone(),
        }
    }

    /// The reference fixture's declaration: the mistake is in what this peer
    /// hands back, and its push and its errors are the reference peer's own.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

/// Every rule in the enumeration against a fresh [`ScopedFixture`] withholding
/// `withheld`, by name, each verdict reduced to the clause it reports.
fn verdicts(withheld: Option<&Query>) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)> {
    let open = async || ScopedFixture::new(withheld.cloned());
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$(
                (
                    stringify!($name),
                    block_on(rules::$name(open)).map_err(|violation| violation.clause()),
                )
            ),*]
        };
    }
    happenstance_sync_testkit::for_each_sync_peer_rule!(run_each)
}

/// The whole-log rule reports SY-28 against a peer withholding `withheld`,
/// and every other rule runs.
fn only_the_whole_log_rule_fails(withheld: &Query) {
    let all = verdicts(Some(withheld));
    assert!(
        all.iter()
            .any(|(name, _)| *name == "round_trip_preserves_the_whole_log"),
        "the whole-log rule is not in the enumeration: {all:?}"
    );
    for (name, verdict) in all {
        let expected = if name == "round_trip_preserves_the_whole_log" {
            Err("SY-28")
        } else {
            Ok(RuleOutcome::Ran)
        };
        assert_eq!(
            verdict, expected,
            "`{name}` against a peer withholding {withheld:?}"
        );
    }
    let reason = block_on(rules::round_trip_preserves_the_whole_log(async || {
        ScopedFixture::new(Some(withheld.clone()))
    }))
    .map_err(|violation| violation.message().to_owned());
    assert!(
        matches!(&reason, Err(message) if message.contains("was pushed and never handed back")),
        "the whole-log rule must fail for withholding, not another reason: {reason:?}"
    );
}

/// The slices the rule's log carries, spelled here as the rule spells them.
/// If the rule's log moves and these do not, the scopes below match nothing,
/// the mutant passes, and the tests that use them go red rather than vacuous.
fn tags(values: &[&'static str]) -> Tags {
    values.iter().copied().map(Tag::from_static).collect()
}

fn item(types: &[&'static str], tagged: &[&'static str]) -> QueryItem {
    QueryItem::new(
        types.iter().copied().map(EventType::from_static),
        tags(tagged),
    )
    .expect("a query of at least one type or tag is valid")
}

#[test]
fn the_unscoped_wrapper_passes_every_rule() {
    let all = verdicts(None);
    assert!(
        all.iter()
            .any(|(name, _)| *name == "round_trip_preserves_the_whole_log"),
        "the whole-log rule is not in the enumeration: {all:?}"
    );
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

/// Kestrel Cold Chain's confidential slice: parts pricing, by tag alone.
#[test]
fn round_trip_preserves_the_whole_log_fails_a_peer_that_withholds_a_tagged_slice() {
    only_the_whole_log_rule_fails(&Query::from_item(item(&[], &["slice:pricing"])));
}

/// A slice by event type alone, which takes untagged and tagged events alike.
#[test]
fn round_trip_preserves_the_whole_log_fails_a_peer_that_withholds_an_event_type() {
    only_the_whole_log_rule_fails(&Query::from_item(item(&["PartShipped"], &[])));
}

/// A slice by type and tag together: one region's shipments, the other
/// region's left in, so the peer hands back most of the type and withholds one.
#[test]
fn round_trip_preserves_the_whole_log_fails_a_peer_that_withholds_one_region() {
    only_the_whole_log_rule_fails(&Query::from_item(item(&["PartShipped"], &["region:north"])));
}
