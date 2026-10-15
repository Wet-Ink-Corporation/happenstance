//! SY-18's rule, failing the peers whose declared limits lie — and only that
//! rule.
//!
//! Each mutant is the KV-capped reference peer with its declaration rewritten
//! and its behaviour untouched: it stores, pulls and refuses exactly as
//! `KvCappedPeer` does. So a red result is the declaration's, and the honest
//! wrapper is the control that shows it.
//!
//! In a file of its own rather than beside `wrong_peers.rs`'s mutants because
//! these wrap a different inner peer, and so that the two can grow apart.
//! `verdicts` and `fails_exactly` are that file's, copied.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_core::MIN_SUPPORTED_EVENT_DATA_LEN;
use happenstance_sync::{Ack, MemoryResume, PeerLimits, Pulled, PushBatch, SendSyncPeer};
use happenstance_sync_testkit::{
    KV_VALUE_CAP, KvCappedFixture, KvCappedPeer, KvPeerError, RefusalsOf, SyncFixture, rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

/// What a [`LyingKvPeer`] declares in place of the truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lie {
    /// None: the control.
    None,
    /// The `PeerLimits`-ignoring peer the issue names: a KV store that declares
    /// what the SQL stores do, 1 MiB per event, as if it had no value cap. A
    /// runner that reads it pushes the 340 KB seat map, and ingest parks it.
    NoValueCap,
    /// The most plausible wrong number: the value cap itself, as if a value held
    /// the payload and nothing else. An event at that size with any envelope is
    /// refused at ingest.
    RawValueCap,
    /// A declaration one byte below VT-21's floor.
    BelowTheFloor,
}

/// A [`KvCappedPeer`] that declares `lie` and does everything else unchanged.
#[derive(Debug)]
struct LyingKvPeer {
    inner: KvCappedPeer,
    lie: Lie,
}

impl SendSyncPeer for LyingKvPeer {
    type Error = KvPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.inner.pull(from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.inner.push(batch).await
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = self.inner.limits();
        match self.lie {
            Lie::None => {}
            Lie::NoValueCap => limits.max_event_bytes = 1024 * 1024,
            Lie::RawValueCap => limits.max_event_bytes = KV_VALUE_CAP,
            Lie::BelowTheFloor => limits.max_event_bytes = MIN_SUPPORTED_EVENT_DATA_LEN - 1,
        }
        limits
    }
}

#[derive(Debug)]
struct LyingKvFixture {
    inner: KvCappedFixture,
    lie: Lie,
}

impl LyingKvFixture {
    fn new(lie: Lie) -> Self {
        Self {
            inner: KvCappedFixture::new(),
            lie,
        }
    }
}

impl SyncFixture for LyingKvFixture {
    type Peer = LyingKvPeer;

    async fn connect(&self) -> Self::Peer {
        LyingKvPeer {
            inner: self.inner.connect().await,
            lie: self.lie,
        }
    }

    /// The reference fixture's declaration, provoked on the honest peer
    /// beneath the lie: the lie is about limits, not about classes.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        self.inner.refusals(&peer.inner).await
    }
}

/// Every rule in the enumeration against one fixture, by name, each verdict
/// reduced to the clause it reports.
fn verdicts<F, O>(open: O) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)>
where
    F: SyncFixture,
    O: AsyncFn() -> F + Copy,
{
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

/// `rule` reports `clause`, and every other rule passes.
fn fails_exactly(
    verdicts: &[(&'static str, Result<RuleOutcome, &'static str>)],
    rule: &str,
    clause: &'static str,
) {
    assert!(
        verdicts.iter().any(|(name, _)| *name == rule),
        "`{rule}` is not in the enumeration: {verdicts:?}"
    );
    for (name, verdict) in verdicts {
        if *name == rule {
            assert_eq!(*verdict, Err(clause), "`{name}` must reject its mutant");
        } else {
            assert_eq!(
                *verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` failed `{rule}`'s mutant, so that mutant proves nothing about \
                 `{rule}` alone"
            );
        }
    }
}

#[test]
fn the_honest_kv_peer_passes_every_rule() {
    let all = verdicts(async || LyingKvFixture::new(Lie::None));
    assert!(all.len() >= 4, "the enumeration ran no rules: {all:?}");
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_no_value_cap() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Lie::NoValueCap)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_the_raw_value_cap() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Lie::RawValueCap)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_below_the_floor() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Lie::BelowTheFloor)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}
