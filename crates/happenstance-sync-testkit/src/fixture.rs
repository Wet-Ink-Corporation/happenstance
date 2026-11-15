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

use happenstance_testkit::{Capability, Reopened};

use crate::violation::{Violation, ensure};

/// The clause a fixture's own failure to build a provocation is reported
/// under: the rule that asked for it.
const SY_36: &str = "SY-36";

/// The clause a fixture's failure to compact is reported under.
const SY_32: &str = "SY-32";

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

    /// Whether this fixture can make its peer forget a prefix of its log, as
    /// a peer with a retention window does. SY-32's rule,
    /// [`retention_gap_is_reported_not_silent`](crate::rules::retention_gap_is_reported_not_silent),
    /// needs it, and reports [`Skipped`](happenstance_testkit::RuleOutcome::Skipped)
    /// with the stated reason when it is declined.
    ///
    /// Nothing in the port compacts, so the operation is the adapter's.
    const COMPACTION: Capability = Capability::declined(UNSTATED_COMPACTION);

    /// Makes `peer` forget the longest prefix of its log, in its forwarding
    /// order, in which every event was recorded before `cutoff`, as the
    /// fixture's adapter compacts on a retention window (SY-32).
    ///
    /// `cutoff` is a window predicate on each event's
    /// [`RecordedAt`], never an order: `RecordedAt` orders nothing. A fixture
    /// that declares [`COMPACTION`](Self::COMPACTION) supported overrides this.
    ///
    /// # Errors
    ///
    /// The provided body always returns a [`Violation`] of SY-32 naming the
    /// two ways to reach it: a fixture that declares the capability and does
    /// not compact, or a rule that compacts without checking it.
    fn compact_recorded_before(
        &self,
        _peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> impl Future<Output = Result<(), Violation>> {
        core::future::ready(Err(Violation::new(
            SY_32,
            format!(
                "`SyncFixture::compact_recorded_before({})` was called and not overridden: \
                 either this fixture declares COMPACTION supported and does not compact, or \
                 a rule compacted without checking the capability",
                cutoff.as_millis()
            ),
        )))
    }
    /// Whether this fixture can close and reopen its backing peer, discarding
    /// every handle's process-level state.
    ///
    /// `happenstance_testkit::ProjectionFixture::REOPEN` for the peer port, and
    /// CF-17 governs it in the same words: a fixture declaring it supported
    /// MUST make [`reopen`](Self::reopen) discard that state **over a medium
    /// that outlives the process's hold on it**, and MUST state the mechanism —
    /// which pool is closed, which client is rebuilt. A fixture over a peer
    /// with no medium outside the process MUST decline it, with that as its
    /// stated reason.
    ///
    /// It is read by exactly one rule,
    /// [`ack_never_confirms_a_non_durable_event`](crate::rules::ack_never_confirms_a_non_durable_event),
    /// which reads SY-31's *durably seen* across a reopen. Without one the rule
    /// can only drop the acking handle and connect another, so a peer that
    /// keeps what it acknowledged in memory every handle shares passes it.
    ///
    /// # Defaulted, and checked
    ///
    /// Defaulted to a declension, in [`UNSTATED_SYNC_REOPEN`]'s words, so the
    /// item is additive to every fixture written before it: a default may
    /// decline on an author's behalf, never declare. A fixture that inherits
    /// it is told so in its own CI by [`sync_declensions_are_stated`], which
    /// [`sync_peer_conformance!`](crate::sync_peer_conformance) emits.
    ///
    /// # What declaring it does not prove
    ///
    /// An override that answers [`Reopened::Discarded`] and discards nothing is
    /// indistinguishable, through the port, from an honest one. So the rule
    /// rejects a *peer* that loses a confirmed event across a reopen, and
    /// cannot reject a *fixture* that only claims to reopen; the declaration
    /// MUST above carries that hazard, as it does for the projection port.
    const REOPEN: Capability = Capability::declined(UNSTATED_SYNC_REOPEN);

    /// Closes and reopens the backing peer, discarding every outstanding
    /// handle's process-level state, and says whether it did.
    ///
    /// After an override returns [`Reopened::Discarded`], a fresh
    /// [`connect`](Self::connect) must observe exactly what the peer durably
    /// holds. Handles obtained before the call may stop working; a rule drops
    /// every one before it calls this.
    ///
    /// # The provided body says it reopened nothing
    ///
    /// It returns [`Reopened::Nothing`], and the rule reports a violation on
    /// that answer before it reads anything back. A fixture that declares
    /// [`REOPEN`](Self::REOPEN) supported and forgets this override therefore
    /// fails on every run, with a message naming this method: the projection
    /// port's shape, not the event-store port's panicking one, so the failure
    /// lands in the rule and no panic is in this crate's library code.
    fn reopen(&self) -> impl Future<Output = Reopened> {
        core::future::ready(Reopened::Nothing)
    }
}

/// What [`SyncFixture::REOPEN`] says when a fixture says nothing.
///
/// `pub` so an adapter author can read what their silence says. A fixture
/// cannot agree with it by naming it: [`sync_declensions_are_stated`] compares
/// the text, so the constant named is the constant inherited.
pub const UNSTATED_SYNC_REOPEN: &str = "this fixture has not said whether its peer can be reopened; a peer with \
     no medium outside the process has nothing a reopen could read back, and this \
     is that answer given by default rather than by its author";

/// CF-18's machine-checked half for the sync-peer family: no capability `F`
/// declines carries the reason it would inherit by saying nothing.
///
/// [`ingest_declensions_are_stated`](crate::ingest_declensions_are_stated)'s
/// twin. Emitted once per mounted suite by
/// [`sync_peer_conformance!`](crate::sync_peer_conformance), not by the
/// emitter, so a third-party emitter cannot drop it. It cannot see a reason
/// that is merely bad.
///
/// The opener is taken, and never called, so `F` is inferred from the
/// macro's hoisted fixture function.
///
/// # Errors
///
/// A [`Violation`] of CF-18 naming each capability `F` declines with the
/// inherited reason.
pub fn sync_declensions_are_stated<F: SyncFixture>(
    _opener: impl AsyncFn() -> F,
) -> Result<(), Violation> {
    let declared = [("REOPEN", F::REOPEN, UNSTATED_SYNC_REOPEN)];
    let unstated: Vec<&str> = declared
        .iter()
        .filter(|(_, capability, default)| capability.reason() == Some(*default))
        .map(|(name, _, _)| *name)
        .collect();
    ensure(unstated.is_empty(), "CF-18", || {
        format!(
            "this fixture declines {} without saying why: the reason printed on every run is \
             this crate's words, not the adapter's account of its peer. Write the constant on \
             the `impl`, with the reason this peer cannot co-operate, in your own words",
            unstated.join(", ")
        )
    })
}

/// What [`SyncFixture::COMPACTION`] says when a fixture says nothing.
///
/// `pub` so an adapter author can read what their silence says.
pub const UNSTATED_COMPACTION: &str = "this fixture cannot make its peer forget a prefix of its log; nothing \
     in the port compacts, so compaction has to come from the adapter and this one has none to \
     offer";

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

    /// Declined, by incapacity: the reference peer's log is process memory,
    /// so a reopen would read back nothing or everything, never what was
    /// durable. `tests/wrong_reopens.rs` proves the rule's reopen half instead.
    const REOPEN: Capability = Capability::declined(
        "the reference peer keeps its log in process memory and has no medium outside the \
         process a reopen could read back; SY-31's reopen half is proven against the wrong \
         peers in tests/wrong_reopens.rs and run against the Postgres and Neon peers",
    );

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

    const COMPACTION: Capability = Capability::SUPPORTED;

    /// [`MemorySyncPeer::compact_recorded_before`] on the peer behind `peer`.
    async fn compact_recorded_before(
        &self,
        peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        compact_memory_peer(&peer.peer, cutoff)
    }
}

/// [`MemorySyncPeer::compact_recorded_before`], its one failure reported as a
/// violation of SY-32: the body of every fixture here over the reference peer.
pub(crate) fn compact_memory_peer(
    peer: &MemorySyncPeer,
    cutoff: RecordedAt,
) -> Result<(), Violation> {
    peer.compact_recorded_before(cutoff)
        .map(drop)
        .map_err(|error| {
            Violation::new(
                SY_32,
                format!("the reference peer could not compact: {error}"),
            )
        })
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
    use happenstance_core::RecordedAt;
    use happenstance_sync::PeerLimits;
    use happenstance_testkit::{Capability, Reopened, block_on};

    use super::{
        MAX_PROVOKED_BATCH_BYTES, MemorySyncFixture, MemorySyncHandle, RefusalsOf, SyncFixture,
        UNSTATED_COMPACTION, batch_over_the_limit, oversized_push_refusal,
        sync_declensions_are_stated,
    };

    /// A fixture over the reference peer whose `REOPEN` is `$reopen`, or the
    /// trait's default when none is given.
    macro_rules! over_the_reference {
        ($name:ident $(, $reopen:expr)?) => {
            struct $name(MemorySyncFixture);

            impl SyncFixture for $name {
                type Peer = MemorySyncHandle;
                $(const REOPEN: Capability = $reopen;)?

                async fn connect(&self) -> Self::Peer {
                    self.0.connect().await
                }

                async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
                    oversized_push_refusal(peer).await
                }
            }
        };
    }

    over_the_reference!(Silent);
    over_the_reference!(Supports, Capability::SUPPORTED);
    over_the_reference!(
        States,
        Capability::declined("a test fixture over the reference peer, which keeps no medium")
    );

    /// A fixture that says nothing declines compaction in this crate's words,
    /// and its provided compaction is a violation of SY-32, never a silent
    /// no-op a rule could mistake for a compaction that dropped nothing.
    #[test]
    fn a_silent_fixture_declines_compaction_and_cannot_compact() {
        let fixture = Silent(MemorySyncFixture::new());
        let peer = block_on(fixture.connect());

        let compacted =
            block_on(fixture.compact_recorded_before(&peer, RecordedAt::from_millis(0)));

        assert_eq!(Silent::COMPACTION.reason(), Some(UNSTATED_COMPACTION));
        let violation = compacted.expect_err("the provided body never compacts");
        assert_eq!(violation.clause(), "SY-32");
    }

    #[test]
    fn a_fixture_that_inherits_the_default_reopen_declension_is_rejected() {
        let checked = sync_declensions_are_stated(async || Silent(MemorySyncFixture::new()));

        assert_eq!(
            checked.map_err(|violation| violation.clause()),
            Err("CF-18")
        );
    }

    #[test]
    fn a_supported_or_stated_reopen_passes() {
        assert_eq!(
            sync_declensions_are_stated(async || Supports(MemorySyncFixture::new())),
            Ok(())
        );
        assert_eq!(
            sync_declensions_are_stated(async || States(MemorySyncFixture::new())),
            Ok(())
        );
    }

    /// The provided body answers that it reopened nothing, which is what the
    /// rule reports a declared-and-forgotten override by.
    #[test]
    fn the_default_reopen_reopens_nothing() {
        let reopened = block_on(Silent(MemorySyncFixture::new()).reopen());

        assert_eq!(reopened, Reopened::Nothing);
    }

    /// The reference fixture states its declension, so its own mount passes.
    #[test]
    fn the_reference_fixture_states_its_reopen_declension() {
        assert_eq!(
            sync_declensions_are_stated(async || MemorySyncFixture::new()),
            Ok(())
        );
    }

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
