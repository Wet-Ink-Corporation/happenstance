//! SY-18's rule, failing the peers whose declared limits are not the limits
//! they impose — and only that rule.
//!
//! Each mutant is the KV-capped reference peer with one mistake: either its
//! declaration is rewritten and its behaviour untouched, or its behaviour is
//! changed under an honest declaration. Either way the peer and the
//! declaration it hands a runner disagree, and the honest wrapper is the
//! control that shows the plumbing is not the cause.
//!
//! In a file of its own rather than beside `wrong_peers.rs`'s mutants because
//! these wrap a different inner peer, and so that the two can grow apart.
//! `verdicts` and `fails_exactly` are that file's, copied.

#![cfg(not(target_arch = "wasm32"))]

use std::sync::atomic::{AtomicBool, Ordering};

use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, MIN_SUPPORTED_EVENT_DATA_LEN};
use happenstance_sync::{
    Ack, EventGroup, MemoryResume, PeerLimits, Pulled, PushBatch, ReplicatedEvent, SendSyncPeer,
};
use happenstance_sync_testkit::{
    KV_VALUE_CAP, KvCappedFixture, KvCappedPeer, KvPeerError, RefusalsOf, SyncFixture, rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

/// Workers KV's cap on the metadata stored beside a value: 1,024 bytes.
const KV_METADATA_CAP: usize = 1024;

/// Above this many payload bytes, the behavioural mutants misbehave. VT-21's
/// floor, so every rule but SY-18's, whose events are all far smaller, sees
/// the reference peer's behaviour.
const LARGE: usize = MIN_SUPPORTED_EVENT_DATA_LEN;

/// The one mistake a [`LyingKvPeer`] makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
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
    /// Stores metadata where Workers KV does, beside the value, and so refuses
    /// any over 1,024 bytes, without declaring it: `max_event_bytes` counts
    /// metadata, and this peer's declaration says it may be as large as data.
    UndeclaredMetadataCap,
    /// A declaration that changes from one read to the next: a peer that
    /// recomputes it from state, say. The runner checked one and pushed under
    /// another.
    FlipsBetweenReads,
    /// A declaration that admits no event at all: `max_batch_events` of zero,
    /// a configuration field left at its default. Nothing a runner reads from
    /// it says what the peer accepts.
    NoRoomInABatch,
    /// Accepts a large event and keeps only its first [`LARGE`] bytes of data,
    /// acknowledging it as appended. The declaration is honoured in letter and
    /// the log holds an event the origin never wrote.
    TruncatesLargeData,
    /// Stores a large event whole and acknowledges it as skipped, as a peer
    /// whose ack counts only the writes on its small-value path would. A
    /// runner reads that as "already held".
    AcksLargeEventsAsSkipped,
}

/// A [`KvCappedPeer`] that makes `mistake` and does everything else unchanged.
#[derive(Debug)]
struct LyingKvPeer {
    inner: KvCappedPeer,
    mistake: Mistake,
    /// Which way [`Mistake::FlipsBetweenReads`] answers next. Atomic rather than
    /// a `Cell` because the peer is `Send + Sync` for the tokio emitter.
    flipped: AtomicBool,
}

fn is_large(event: &ReplicatedEvent) -> bool {
    event.payload_len() > LARGE
}

/// `event` with its data cut to [`LARGE`] bytes, everything else kept.
fn truncated(event: &ReplicatedEvent) -> ReplicatedEvent {
    let data = event.event.data();
    let mut kept = Event::new(
        event.event.event_type().clone(),
        data.slice(..data.len().min(LARGE)),
    )
    .expect("the type came from a valid event")
    .with_tags(event.event.tags().clone());
    if let Some(metadata) = event.event.metadata() {
        kept = kept.with_metadata(metadata.clone());
    }
    ReplicatedEvent::new(event.id, event.recorded_at, kept)
}

impl SendSyncPeer for LyingKvPeer {
    type Error = KvPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.inner.pull(from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        match self.mistake {
            Mistake::UndeclaredMetadataCap => {
                let over = batch.events().find_map(|event| {
                    let len = event.event.metadata().map_or(0, Bytes::len);
                    (len > KV_METADATA_CAP).then_some((event.id, len))
                });
                if let Some((id, value_len)) = over {
                    return Err(KvPeerError::ValueTooLarge {
                        id,
                        value_len,
                        cap: KV_METADATA_CAP,
                    });
                }
                self.inner.push(batch).await
            }
            Mistake::TruncatesLargeData => {
                let groups = batch
                    .groups
                    .iter()
                    .map(|group| {
                        let events = group.events.iter().map(truncated).collect();
                        EventGroup::new(group.guard.clone(), events)
                    })
                    .collect();
                self.inner.push(&PushBatch::new(groups)).await
            }
            Mistake::AcksLargeEventsAsSkipped => {
                let mut ack = self.inner.push(batch).await?;
                if batch.events().any(is_large) {
                    ack.skipped += ack.appended;
                    ack.appended = 0;
                }
                Ok(ack)
            }
            Mistake::None
            | Mistake::NoValueCap
            | Mistake::RawValueCap
            | Mistake::BelowTheFloor
            | Mistake::FlipsBetweenReads
            | Mistake::NoRoomInABatch => self.inner.push(batch).await,
        }
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = self.inner.limits();
        match self.mistake {
            Mistake::NoValueCap => limits.max_event_bytes = 1024 * 1024,
            Mistake::RawValueCap => limits.max_event_bytes = KV_VALUE_CAP,
            Mistake::BelowTheFloor => limits.max_event_bytes = MIN_SUPPORTED_EVENT_DATA_LEN - 1,
            Mistake::NoRoomInABatch => limits.max_batch_events = 0,
            Mistake::FlipsBetweenReads => {
                // Relaxed: the flag orders nothing but itself, and the rule
                // reads it from one thread.
                if self.flipped.fetch_xor(true, Ordering::Relaxed) {
                    limits.max_event_bytes -= 1;
                }
            }
            Mistake::None
            | Mistake::UndeclaredMetadataCap
            | Mistake::TruncatesLargeData
            | Mistake::AcksLargeEventsAsSkipped => {}
        }
        limits
    }
}

#[derive(Debug)]
struct LyingKvFixture {
    inner: KvCappedFixture,
    mistake: Mistake,
}

impl LyingKvFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            inner: KvCappedFixture::new(),
            mistake,
        }
    }
}

impl SyncFixture for LyingKvFixture {
    type Peer = LyingKvPeer;

    async fn connect(&self) -> Self::Peer {
        LyingKvPeer {
            inner: self.inner.connect().await,
            mistake: self.mistake,
            flipped: AtomicBool::new(false),
        }
    }

    /// The reference fixture's declaration, provoked on the honest peer
    /// beneath the mistake: the mistake is about limits, not about classes.
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
    let all = verdicts(async || LyingKvFixture::new(Mistake::None));
    assert!(all.len() >= 4, "the enumeration ran no rules: {all:?}");
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_no_value_cap() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::NoValueCap)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_the_raw_value_cap() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::RawValueCap)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_declares_below_the_floor() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::BelowTheFloor)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_with_an_undeclared_metadata_cap() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::UndeclaredMetadataCap)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_declaration_that_changes_between_reads() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::FlipsBetweenReads)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_declaration_that_admits_no_event() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::NoRoomInABatch)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_truncates_large_data() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::TruncatesLargeData)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}

#[test]
fn peer_declares_its_own_limits_fails_a_kv_peer_that_acks_large_events_as_skipped() {
    fails_exactly(
        &verdicts(async || LyingKvFixture::new(Mistake::AcksLargeEventsAsSkipped)),
        "peer_declares_its_own_limits",
        "SY-18",
    );
}
