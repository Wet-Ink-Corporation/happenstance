//! SY-23's rule, failing the wrong peer it names, and only that rule.
//!
//! A file of its own rather than a row in `tests/wrong_peers.rs`, with a
//! wrapper of its own, because the mistake lives in how a peer decides what it
//! already holds, which none of that file's wrappers touch. Like that file,
//! each test runs the **whole** enumeration, so the mutant must fail SY-23's
//! rule and pass every other one, and the faithful wrapper is the control that
//! shows the plumbing passes everything.

#![cfg(not(target_arch = "wasm32"))]

use std::sync::{Arc, Mutex};

use happenstance_core::StoreId;
use happenstance_sync::{
    Ack, EventGroup, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch,
    SendSyncPeer, Watermark,
};
use happenstance_sync_testkit::{RefusalsOf, SyncFixture, oversized_push_refusal, rules};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake an [`OrderPeer`] makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// SY-23's wrong implementation: decide "already held" by the highest
    /// origin position seen from each origin, the watermark a peer reports in
    /// [`Ack::confirmed`], rather than by identity. An origin's lower event
    /// that arrives after a higher one, as a delayed page does, is skipped
    /// and never held.
    DedupesByHighWaterMark,
}

/// The reference peer behind a wrapper that may make [`Mistake`].
#[derive(Debug)]
struct OrderPeer {
    inner: Arc<MemorySyncPeer>,
    /// The highest origin position each origin has been seen at, which only
    /// the mistake consults. Shared across handles, as the peer's log is.
    seen: Arc<Mutex<Watermark>>,
    mistake: Mistake,
}

impl OrderPeer {
    /// `batch` without the events the mistake calls held, and how many those
    /// were. A new batch, because the reference peer stores what it is
    /// pushed, so each event kept is cloned into it.
    fn admitted(&self, batch: &PushBatch) -> Result<(PushBatch, u64), MemoryPeerError> {
        let mut seen = self.seen.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        let mut skipped = 0_u64;
        let mut groups = Vec::with_capacity(batch.groups.len());
        for group in &batch.groups {
            let mut events = Vec::with_capacity(group.events.len());
            for event in &group.events {
                let origin = event.id.store();
                if seen
                    .get(origin)
                    .is_some_and(|highest| event.id.position() <= highest)
                {
                    skipped = skipped.saturating_add(1);
                } else {
                    seen.advance(origin, event.id.position());
                    events.push(event.clone());
                }
            }
            if !events.is_empty() {
                groups.push(EventGroup::new(group.guard.clone(), events));
            }
        }
        Ok((PushBatch::new(groups), skipped))
    }
}

impl SendSyncPeer for OrderPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.inner.pull(from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        if self.mistake == Mistake::Faithful {
            return self.inner.push(batch).await;
        }
        // The size check is the reference peer's, on the batch as offered, so
        // the mistake refuses exactly what the reference refuses.
        let offered = batch.payload_len();
        let limit = self.inner.limits().max_batch_bytes;
        if offered > limit {
            return Err(MemoryPeerError::BatchTooLarge {
                actual: offered,
                limit,
            });
        }
        let (admitted, skipped) = self.admitted(batch)?;
        let mut ack = self.inner.push(&admitted).await?;
        ack.skipped = ack.skipped.saturating_add(skipped);
        Ok(ack)
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

/// One isolated [`OrderPeer`] log per instance.
#[derive(Debug)]
struct OrderFixture {
    inner: Arc<MemorySyncPeer>,
    seen: Arc<Mutex<Watermark>>,
    mistake: Mistake,
}

impl OrderFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x78; 16]))),
            seen: Arc::new(Mutex::new(Watermark::new())),
            mistake,
        }
    }
}

impl SyncFixture for OrderFixture {
    type Peer = OrderPeer;

    async fn connect(&self) -> Self::Peer {
        OrderPeer {
            inner: Arc::clone(&self.inner),
            seen: Arc::clone(&self.seen),
            mistake: self.mistake,
        }
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

/// Every rule in the enumeration against one fixture, by name, each verdict
/// reduced to the clause it reports.
fn verdicts(mistake: Mistake) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)> {
    let open = async || OrderFixture::new(mistake);
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

#[test]
fn the_faithful_order_wrapper_passes_every_rule() {
    let all = verdicts(Mistake::Faithful);
    assert!(all.len() >= 3, "the enumeration ran no rules: {all:?}");
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

#[test]
fn event_id_order_is_identical_on_every_peer_fails_a_peer_that_dedupes_by_high_water_mark() {
    let all = verdicts(Mistake::DedupesByHighWaterMark);
    let rule = "event_id_order_is_identical_on_every_peer";
    assert!(
        all.iter().any(|(name, _)| *name == rule),
        "`{rule}` is not in the enumeration: {all:?}"
    );
    for (name, verdict) in all {
        let expected = if name == rule {
            Err("SY-23")
        } else {
            Ok(RuleOutcome::Ran)
        };
        assert_eq!(verdict, expected, "`{name}`");
    }
}
