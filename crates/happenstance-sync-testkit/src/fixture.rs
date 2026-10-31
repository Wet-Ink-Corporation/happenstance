//! What an adapter hands the suite, and the reference implementation of it.

use alloc::format;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::future::Future;

use happenstance_core::{Event, EventId, EventType, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{
    Ack, EventGroup, FailureClass, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits,
    Pulled, PushBatch, ReplicatedEvent, SendSyncPeer,
};

use crate::Violation;

/// The clause a fixture's own failure to build a provocation is reported
/// under: the rule that asked for it.
const SY_36: &str = "SY-36";

/// One isolated far side, and a way to reach it.
///
/// One fixture instance is one isolated backing peer, and each
/// [`connect`](Self::connect) is one handle onto it — the event-store family's
/// contract, one port over. A rule calls the opener it is given for a fresh
/// instance, so no rule sees another's events.
///
/// # No `Send`, on purpose
///
/// Not `trait_variant`-derived and bounded on the bare
/// [`SyncPeer`](happenstance_sync::SyncPeer), so a `!Send` peer — the Durable
/// Object's, which holds its transport behind an `Rc` — is a valid fixture.
/// That is SY-17's rule: the suite compiles against a `!Send` fixture peer,
/// which this crate's `tests/local_sync_conformance.rs` does.
///
/// # One peer handle per rule
///
/// A fixture yields peers, never a peer set, and every rule in
/// [`rules`](crate::rules) holds exactly one (SY-8). A rule that needed two
/// would be testing the runner. The model family (`sync_model`, behind the
/// `proptest` feature) is the exception, and says so: it plays the runner
/// itself over one fixture **per node**, so each call of the opener it is given
/// must build a fresh, isolated peer.
pub trait SyncFixture {
    /// The peer adapter under test.
    type Peer: happenstance_sync::SyncPeer;

    /// A handle onto this fixture's backing peer.
    fn connect(&self) -> impl Future<Output = Self::Peer>;

    /// Drives `peer` into every refusal it can produce, and declares the
    /// [`FailureClass`] each one is (SY-36).
    ///
    /// Only the adapter knows which of its failures can be provoked and what
    /// each calls for, so the fixture both provokes and declares, and the rule
    /// [`every_refusal_states_its_class`](crate::rules::every_refusal_states_its_class)
    /// compares what the error says against what the fixture said. A required
    /// method rather than a default returning nothing, because a fixture that
    /// declined it silently would pass that rule against nothing.
    ///
    /// # Errors
    ///
    /// A [`Violation`] if the fixture cannot build a provocation — its own
    /// failure, reported under SY-36 because that is the rule that asked.
    fn refusals(&self, peer: &Self::Peer) -> impl Future<Output = RefusalsOf<Self::Peer>>;
}

/// What [`SyncFixture::refusals`] resolves to for a peer `P`.
///
/// An alias only to keep the trait's signature readable: the error type is
/// reached through the peer's associated type, which spelled out in place is
/// three levels of angle brackets.
pub type RefusalsOf<P> = Result<Vec<Refusal<<P as happenstance_sync::SyncPeer>::Error>>, Violation>;

/// One refusal a fixture drove its peer into, and the class it declares for it.
///
/// Plain data with public fields: a fixture builds one per refusal and the rule
/// only reads it.
#[derive(Debug)]
pub struct Refusal<E> {
    /// What was done to the peer, for the violation message: `"a push one byte
    /// over max_batch_bytes"`.
    pub provoked_by: &'static str,
    /// The class the fixture says this refusal is.
    pub declared: FailureClass,
    /// What the peer returned. `Ok` is a provocation the peer accepted, which
    /// the rule reports rather than skips.
    pub outcome: Result<(), E>,
}

/// The largest `max_batch_bytes` [`batch_over_the_limit`] will build a batch
/// over: 64 MiB.
///
/// A ceiling because the batch is allocated whole, and the limit it is sized
/// from is the peer's own declaration — a value the suite does not control
/// (H-13). 64 MiB is three orders of magnitude above the minimum conformant
/// limit and well inside what a test process can hold.
pub const MAX_PROVOKED_BATCH_BYTES: usize = 64 * 1024 * 1024;

/// The most events one push a rule builds will carry: 4,096.
///
/// A ceiling for the reason [`MAX_PROVOKED_BATCH_BYTES`] is one: SY-18's batch
/// rule sizes its pushes from the peer's declared `max_batch_events`, a value
/// the suite does not control, and builds each push whole (H-13). 4,096 is
/// thirty-two times the minimum conformant limit, and above the 257 events the
/// largest push of SY-37's count-cut log carries, so a peer whose limits
/// refuse that push refuses the batch rule's corner too.
pub const MAX_PROVOKED_BATCH_EVENTS: usize = 4_096;

/// The oversized-push refusal, provoked on `peer` and declared
/// [`FailureClass::Split`].
///
/// The body every fixture whose peer honours its own [`PeerLimits`] would
/// otherwise write out: build [`batch_over_the_limit`] from what the peer
/// declares, push it, and record what came back. A fixture whose peer classifies
/// an oversized push as something other than `Split` declares its refusal by
/// hand instead.
///
/// # Errors
///
/// A [`Violation`] of SY-36 if [`batch_over_the_limit`] cannot build the batch.
pub async fn oversized_push_refusal<P: happenstance_sync::SyncPeer>(peer: &P) -> RefusalsOf<P> {
    let oversized = batch_over_the_limit(&peer.limits())?;
    Ok(vec![Refusal {
        provoked_by: "a push one byte over max_batch_bytes",
        declared: FailureClass::Split,
        outcome: peer.push(&oversized).await.map(drop),
    }])
}

/// A batch one byte over `limits.max_batch_bytes`, built from events no larger
/// than `limits.max_event_bytes`, so a peer that checks both refuses it for
/// the batch and not for any one event.
///
/// For a fixture whose peer refuses an oversized push, which is the one refusal
/// every peer that honours its own [`PeerLimits`] can be driven into. The
/// payloads are zero bytes and are never parsed (SY-35).
///
/// # Errors
///
/// A [`Violation`] of SY-36 if `limits.max_batch_bytes` is above
/// [`MAX_PROVOKED_BATCH_BYTES`]. The batch is built in memory, so a peer that
/// declares more than that would have the suite allocate whatever it claimed;
/// such a peer's fixture must provoke its own oversized refusal, by a route
/// that does not materialise the whole batch.
///
/// A [`Violation`] of SY-36 if the batch would need more events than one origin
/// can number. Unreachable under the ceiling.
pub fn batch_over_the_limit(limits: &PeerLimits) -> Result<PushBatch, Violation> {
    const ORIGIN: StoreId = StoreId::from_bytes([0x0B; 16]);
    const OVERSIZED: EventType = EventType::from_static("Oversized");

    if limits.max_batch_bytes > MAX_PROVOKED_BATCH_BYTES {
        return Err(Violation::new(
            SY_36,
            format!(
                "the peer declares max_batch_bytes of {}, above the {MAX_PROVOKED_BATCH_BYTES} \
                 bytes this helper will allocate; its fixture must provoke an oversized \
                 refusal of its own",
                limits.max_batch_bytes
            ),
        ));
    }

    let chunk = limits.max_event_bytes.max(1);
    let mut remaining = limits.max_batch_bytes.saturating_add(1);
    let mut position = SequencePosition::FIRST;
    let mut events = Vec::new();
    while remaining > 0 {
        let size = remaining.min(chunk);
        let event = Event::new(OVERSIZED, vec![0_u8; size]).map_err(|error| {
            Violation::new(
                SY_36,
                format!("the fixture's own event is invalid: {error}"),
            )
        })?;
        events.push(ReplicatedEvent::new(
            EventId::new(ORIGIN, position),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        ));
        remaining -= size;
        position = position
            .next()
            .ok_or_else(|| Violation::new(SY_36, "the oversized batch ran out of positions"))?;
    }
    Ok(PushBatch::new(vec![EventGroup::new(None, events)]))
}

/// The reference [`SyncFixture`], over [`MemorySyncPeer`].
///
/// Every rule is proven against this before it is allowed to fail an adapter:
/// a rule red here is a wrong rule, not a wrong peer. Each instance owns one
/// in-memory peer and each [`connect`](SyncFixture::connect) is a new handle
/// onto it.
#[derive(Debug, Clone)]
pub struct MemorySyncFixture {
    /// Shared by every handle, so a handle dropped and reconnected sees the same
    /// log (SY-16's precondition, for the rule that will check it).
    peer: Arc<MemorySyncPeer>,
}

impl MemorySyncFixture {
    /// A fixture over a fresh in-memory peer with the minimum conformant
    /// limits.
    #[must_use]
    pub fn new() -> Self {
        Self {
            peer: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0xA5; 16]))),
        }
    }
}

impl Default for MemorySyncFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncFixture for MemorySyncFixture {
    type Peer = MemorySyncHandle;

    async fn connect(&self) -> Self::Peer {
        MemorySyncHandle {
            peer: Arc::clone(&self.peer),
        }
    }

    /// The one refusal the reference peer can be driven into from outside.
    ///
    /// [`MemoryPeerError::Poisoned`] is its other, and provoking it takes a
    /// thread that panics while holding the peer's lock: not something a
    /// fixture should do to the peer a suite is about to use.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

/// One handle onto a [`MemorySyncFixture`]'s peer.
///
/// A handle rather than the peer itself because a fixture hands out many and
/// they must share one log; `Arc` because the reference peer is `Send + Sync`
/// and the handle keeps it so, which is what lets it run under the tokio
/// emitter.
#[derive(Debug, Clone)]
pub struct MemorySyncHandle {
    peer: Arc<MemorySyncPeer>,
}

impl SendSyncPeer for MemorySyncHandle {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        SendSyncPeer::pull(&*self.peer, from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        SendSyncPeer::push(&*self.peer, batch).await
    }

    fn limits(&self) -> PeerLimits {
        SendSyncPeer::limits(&*self.peer)
    }
}

#[cfg(test)]
mod tests {
    use happenstance_sync::PeerLimits;

    use super::{MAX_PROVOKED_BATCH_BYTES, batch_over_the_limit};

    /// A peer declaring more than the ceiling is told to provoke its own
    /// refusal, and nothing is allocated for it.
    #[test]
    fn a_limit_above_the_ceiling_is_refused_before_anything_is_allocated() {
        let mut limits = PeerLimits::minimum();
        limits.max_batch_bytes = MAX_PROVOKED_BATCH_BYTES + 1;
        let violation =
            batch_over_the_limit(&limits).expect_err("a limit over the ceiling builds no batch");
        assert_eq!(violation.clause(), "SY-36");
    }

    /// At the ceiling itself the batch is still built, one byte over the limit.
    #[test]
    fn a_limit_at_the_ceiling_is_built_one_byte_over() {
        let mut limits = PeerLimits::minimum();
        limits.max_batch_bytes = MAX_PROVOKED_BATCH_BYTES;
        let batch = batch_over_the_limit(&limits).expect("the ceiling itself is in range");
        assert_eq!(batch.payload_len(), MAX_PROVOKED_BATCH_BYTES + 1);
        assert!(!limits.admits(&batch));
    }
}
