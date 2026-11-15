//! SY-31's rule across a reopen (#448): the peer that keeps what it
//! acknowledged in memory every handle shares, which no handle swap can see
//! and only a reopen can.
//!
//! `wrong_peers.rs`'s `AcksOnAdmission` buffers per handle, so dropping the
//! acking handle loses the buffer and the rule's fresh handle sees the loss.
//! The peer here buffers per *fixture*: every handle flushes the one buffer
//! before it answers, so a fresh handle reads everything back, and the rule
//! passes it unless the fixture discards that buffer between the two handles.
//! That discard is `SyncFixture::reopen`, the process crash a declared
//! [`REOPEN`](SyncFixture::REOPEN) stands for.
//!
//! The tests run the whole enumeration, as `wrong_peers.rs` does, so a red
//! verdict here is the reopen's and every other rule is held to `Ran`.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use happenstance_core::{EventId, RecordedAt, StoreId};
use happenstance_sync::{
    Ack, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch, SendSyncPeer,
};
use happenstance_sync_testkit::{
    MemorySyncFixture, MemorySyncHandle, RefusalsOf, SyncFixture, Violation,
    oversized_push_refusal, rules,
};
use happenstance_testkit::{Capability, Reopened, RuleOutcome, block_on};

/// Every rule's verdict against the fixture `open` builds, by name.
///
/// `wrong_peers.rs`'s, restated: each file under `tests/` is a crate of its
/// own. Driven by `for_each_sync_peer_rule!`, so a rule added to the suite is
/// held here too without an edit.
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

/// SY-31's rule reports SY-31, and every other rule runs.
fn fails_exactly_sy_31(verdicts: &[(&'static str, Result<RuleOutcome, &'static str>)]) {
    const RULE: &str = "ack_never_confirms_a_non_durable_event";
    assert!(
        verdicts.iter().any(|(name, _)| *name == RULE),
        "`{RULE}` is not in the enumeration: {verdicts:?}"
    );
    for (name, verdict) in verdicts {
        let expected = if *name == RULE {
            Err("SY-31")
        } else {
            Ok(RuleOutcome::Ran)
        };
        assert_eq!(*verdict, expected, "`{name}`");
    }
}

/// Every rule runs and passes.
fn passes_every_rule(verdicts: &[(&'static str, Result<RuleOutcome, &'static str>)]) {
    assert!(
        verdicts.len() >= 3,
        "the enumeration ran no rules: {verdicts:?}"
    );
    for (name, verdict) in verdicts {
        assert_eq!(*verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

/// What every handle onto one [`Buffered`] fixture shares: the reference peer,
/// and the batches acknowledged and not yet committed to it.
#[derive(Debug)]
struct Buffered {
    inner: Arc<MemorySyncPeer>,
    /// Shared by every handle, so a dropped handle loses nothing; only a
    /// reopen does. A `std::sync::Mutex` because the peer's futures must be
    /// `Send`; no guard is held across an `.await`.
    admitted: Arc<Mutex<Vec<PushBatch>>>,
}

impl Buffered {
    fn new() -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x79; 16]))),
            admitted: Arc::default(),
        }
    }

    fn connect(&self) -> BuffersAcrossHandles {
        BuffersAcrossHandles {
            inner: Arc::clone(&self.inner),
            admitted: Arc::clone(&self.admitted),
        }
    }

    /// The process crash: what was acknowledged and not committed is gone.
    fn discard(&self) -> Reopened {
        self.admitted
            .lock()
            .expect("no test thread panics holding the buffer")
            .clear();
        Reopened::Discarded
    }
}

/// SY-31's second wrong implementation: a write-behind cache in memory every
/// handle shares. It acknowledges a push on admission, with the ack the
/// reference peer would send, and commits the batch on any handle's next push
/// or pull. A fresh handle commits it before it reads, so only a reopen
/// between the two can show the ack confirmed what the peer never held.
#[derive(Debug)]
struct BuffersAcrossHandles {
    inner: Arc<MemorySyncPeer>,
    admitted: Arc<Mutex<Vec<PushBatch>>>,
}

impl BuffersAcrossHandles {
    /// Commits everything any handle has acknowledged.
    async fn flush(&self) -> Result<(), MemoryPeerError> {
        let pending = core::mem::take(
            &mut *self
                .admitted
                .lock()
                .map_err(|_| MemoryPeerError::Poisoned)?,
        );
        for batch in &pending {
            self.inner.push(batch).await?;
        }
        Ok(())
    }

    /// Everything the reference peer holds, by identity.
    async fn held(&self) -> Result<BTreeSet<EventId>, MemoryPeerError> {
        let mut held = BTreeSet::new();
        let mut resume = None;
        loop {
            let pulled = self.inner.pull(resume.as_ref()).await?;
            if pulled.batch.groups.is_empty() {
                return Ok(held);
            }
            held.extend(pulled.batch.events().map(|event| event.id));
            resume = Some(pulled.resume);
        }
    }

    /// Commits the buffer, then compacts the reference peer behind it: a
    /// compaction is over everything the peer acknowledged, and a batch still
    /// in the buffer is part of that. Either failure is reported as SY-32's.
    async fn compact(&self, cutoff: RecordedAt) -> Result<(), Violation> {
        self.flush().await.map_err(|error| sy_32(&error))?;
        self.inner
            .compact_recorded_before(cutoff)
            .map(drop)
            .map_err(|error| sy_32(&error))
    }
}

/// A reference peer's failure while compacting, reported as SY-32's.
fn sy_32(error: &MemoryPeerError) -> Violation {
    Violation::new("SY-32", error.to_string())
}

impl SendSyncPeer for BuffersAcrossHandles {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.flush().await?;
        self.inner.pull(from).await
    }

    /// Refuses what the reference peer refuses, predicts the ack it would
    /// send, and buffers the batch.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        let limit = self.inner.limits().max_batch_bytes;
        let offered = batch.payload_len();
        if offered > limit {
            return Err(MemoryPeerError::BatchTooLarge {
                actual: offered,
                limit,
            });
        }
        self.flush().await?;
        let mut held = self.held().await?;
        // The reference peer's confirmation before this batch, advanced by
        // each event it would append: the ack a faithful peer sends.
        let mut ack = self.inner.push(&PushBatch::empty()).await?;
        for event in batch.events() {
            if held.insert(event.id) {
                ack.appended += 1;
                ack.confirmed.advance(event.id.store(), event.id.position());
            } else {
                ack.skipped += 1;
            }
        }
        self.admitted
            .lock()
            .map_err(|_| MemoryPeerError::Poisoned)?
            .push(batch.clone());
        Ok(ack)
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

/// The buffering peer, under a fixture that declares a reopen and makes it
/// discard the buffer: what a process crash does to it.
#[derive(Debug)]
struct ReopensTheBuffer(Buffered);

impl SyncFixture for ReopensTheBuffer {
    type Peer = BuffersAcrossHandles;

    /// Supported: the shared buffer is the process state a reopen discards,
    /// and the reference peer behind it stands in for the durable medium.
    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Peer {
        self.0.connect()
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }

    async fn reopen(&self) -> Reopened {
        self.0.discard()
    }

    /// The reference peer's compaction, so SY-32's rule runs here too.
    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn compact_recorded_before(
        &self,
        peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        peer.compact(cutoff).await
    }
}

/// The same peer, under a fixture that declines the reopen.
#[derive(Debug)]
struct DeclinesTheReopen(Buffered);

impl SyncFixture for DeclinesTheReopen {
    type Peer = BuffersAcrossHandles;

    const REOPEN: Capability = Capability::declined(
        "a test fixture over a write-behind buffer, declining the reopen to record what a \
         declension costs",
    );

    async fn connect(&self) -> Self::Peer {
        self.0.connect()
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }

    /// The reference peer's compaction, so SY-32's rule runs here too.
    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn compact_recorded_before(
        &self,
        peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        peer.compact(cutoff).await
    }
}

/// The reference peer under a fixture that declares a reopen and implements
/// it honestly: its handles hold no state of their own, so discarding them is
/// all a reopen has to do, and the reference peer stands in for the medium.
#[derive(Debug, Default)]
struct ReopensHonestly(MemorySyncFixture);

impl SyncFixture for ReopensHonestly {
    type Peer = MemorySyncHandle;

    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Peer {
        self.0.connect().await
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }

    async fn reopen(&self) -> Reopened {
        Reopened::Discarded
    }

    /// The reference fixture's compaction, so SY-32's rule runs here too.
    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn compact_recorded_before(
        &self,
        peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        self.0.compact_recorded_before(peer, cutoff).await
    }
}

/// The reference peer under a fixture that declares a reopen and forgets to
/// override `reopen`.
#[derive(Debug, Default)]
struct ClaimsTheReopen(MemorySyncFixture);

impl SyncFixture for ClaimsTheReopen {
    type Peer = MemorySyncHandle;

    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Peer {
        self.0.connect().await
    }

    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }

    /// The reference fixture's compaction, so SY-32's rule runs here too.
    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn compact_recorded_before(
        &self,
        peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        self.0.compact_recorded_before(peer, cutoff).await
    }
}

/// The mutant only a reopen can see: every handle flushes the shared buffer
/// before it reads, so a fresh handle alone reads everything back.
#[test]
fn ack_never_confirms_a_non_durable_event_fails_a_peer_that_buffers_across_handles() {
    fails_exactly_sy_31(&verdicts(async || ReopensTheBuffer(Buffered::new())));
}

/// What a declension costs, recorded as a test: the same peer, under a fixture
/// that declines the reopen, passes every rule. SY-31's rule cannot see a
/// buffer every handle shares without one.
#[test]
fn a_declined_reopen_lets_a_peer_that_buffers_across_handles_pass_every_rule() {
    passes_every_rule(&verdicts(async || DeclinesTheReopen(Buffered::new())));
}

/// The control: a reopen that keeps what was committed passes every rule.
#[test]
fn an_honest_reopen_passes_every_rule() {
    passes_every_rule(&verdicts(async || ReopensHonestly::default()));
}

/// A fixture that claims the capability and keeps the provided `reopen`
/// fails SY-31's rule, and only that rule.
#[test]
fn a_declared_reopen_without_an_override_fails_sy_31() {
    fails_exactly_sy_31(&verdicts(async || ClaimsTheReopen::default()));
}

/// And the violation names the method the fixture did not override.
#[test]
fn a_declared_reopen_without_an_override_names_the_method() {
    let violation = block_on(rules::ack_never_confirms_a_non_durable_event(async || {
        ClaimsTheReopen::default()
    }))
    .expect_err("a reopen that discarded nothing is a violation");

    assert!(
        violation.message().contains("SyncFixture::reopen"),
        "{violation}"
    );
}
