//! An in-memory [`SyncPeer`](crate::SyncPeer), behind the `memory` feature.
//!
//! It is a *peer*, not a store: it holds whatever was pushed to it and hands it
//! back on pull. Two of them wired to each other is the smallest thing that
//! exercises a runner.
//!
//! # Why it exists: three jobs, one type
//!
//! The same three `happenstance-core` gives for `MemoryEventStore`, one port
//! over, and the same cold start the projection port had without one:
//!
//! 1. **The oracle.** `happenstance-sync-testkit`'s suite is pointed at this
//!    peer first (its `memory_sync_conformance` test): the reference peer every
//!    conformance rule must pass against before it may fail a real adapter. A
//!    rule red here is a wrong rule, not a wrong peer. So the bodies are real — a fixture
//!    whose methods are `todo!()` cannot be the thing a suite is pointed at —
//!    and that is the difference between this type and the two stand-ins in
//!    this crate's `tests/`.
//! 2. **The doctest target.** Every example in this crate that needs a peer
//!    uses this one, so the examples run rather than merely compile.
//! 3. **Something to test against.** An application author can write and test
//!    their sync wiring before a real peer exists, as [`MemorySyncPeer`]'s
//!    example shows.
//!
//! # Behind a feature, and not a default
//!
//! A deployment never replicates to a peer that lives in its own memory, so the
//! type is off unless asked for: `features = ["memory"]`, normally in
//! `[dev-dependencies]`.

use alloc::vec::Vec;

use std::sync::Mutex;

use happenstance_core::{EventId, StoreId};

use crate::identity::{ReplicatedEvent, Watermark};
use crate::peer::{Ack, EventGroup, PeerLimits, PullBatchLimit, Pulled, PushBatch, SendSyncPeer};

/// Where a [`MemorySyncPeer`] pull left off.
///
/// An owned index, not a borrow into the peer and not an iterator held inside
/// it. Dropping the peer and rebuilding it from the same backing log leaves this
/// token valid, which is the property the port requires and the property an
/// in-memory cursor would quietly fail to have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MemoryResume {
    next_group: usize,
}

impl MemoryResume {
    /// The token that starts from the beginning of the far side's log.
    #[must_use]
    pub const fn start() -> Self {
        Self { next_group: 0 }
    }

    /// How many groups this token has already consumed.
    #[must_use]
    pub const fn consumed(self) -> usize {
        self.next_group
    }
}

/// How [`MemorySyncPeer`] fails.
///
/// Short, because an in-memory peer has almost no transport to fail. The two
/// variants that are here are the two that are real: a poisoned lock, and a
/// push that exceeds the limits this peer declared.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MemoryPeerError {
    /// A previous holder of the lock panicked, so the log may be torn.
    ///
    /// `std::sync::PoisonError` borrows the guard it came from, so it cannot be
    /// carried in an owned error. Flattening it loses nothing here: there is no
    /// recovery path for a torn in-memory log.
    #[error("the peer's log lock was poisoned by a panic in another thread")]
    Poisoned,

    /// The pushed batch exceeds the limits [`PeerLimits`] declared.
    ///
    /// Present so that a runner which ignores `limits()` still gets a
    /// distinguishable failure rather than a truncated log. Retrying the same
    /// batch will fail identically; it has to be split.
    #[error("push of {actual} bytes exceeds this peer's declared limit of {limit}")]
    BatchTooLarge {
        /// What was offered.
        actual: usize,
        /// What this peer declared it could take.
        limit: usize,
    },
}

/// An in-memory peer.
///
/// `Send + Sync`, so it implements [`SendSyncPeer`] and gets the bare
/// [`SyncPeer`](crate::SyncPeer) for free. That is the right choice for a
/// fixture: a suite written against it can be run under `tokio::spawn`, and the
/// `!Send` half of the port is exercised by the Durable-Object-shaped stand-in
/// in `tests/` instead.
///
/// `Mutex` rather than `RefCell` for exactly that reason, and the cost is
/// stated rather than hidden: every method can now fail with
/// [`MemoryPeerError::Poisoned`], a failure mode a single-threaded peer does not
/// have.
///
/// # Examples
///
/// A device pushes what it holds to a hub, and a second device pulls it — the
/// hub-and-spoke shape, with the hub played by this type:
///
/// ```
/// use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
/// use happenstance_sync::{
///     EventGroup, MemorySyncPeer, PushBatch, ReplicatedEvent, SyncPeer,
/// };
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let tablet = StoreId::from_bytes([1; 16]);
/// let hub = MemorySyncPeer::new(StoreId::from_bytes([2; 16]));
///
/// // An event the tablet already made durable, carrying the identity it minted.
/// let recorded = ReplicatedEvent::new(
///     EventId::new(tablet, SequencePosition::FIRST),
///     RecordedAt::from_millis(1_700_000_000_000),
///     Event::new("ReadingTaken", &b"{\"celsius\":4}"[..])?,
/// );
/// let batch = PushBatch::new(vec![EventGroup::new(None, vec![recorded])]);
///
/// // The hub takes it once; offering it again is a skip, never a second copy.
/// let first = hub.push(&batch).await?;
/// assert_eq!((first.appended, first.skipped), (1, 0));
/// let again = hub.push(&batch).await?;
/// assert_eq!((again.appended, again.skipped), (0, 1));
/// assert_eq!(again.confirmed.get(tablet), Some(SequencePosition::FIRST));
///
/// // Another device pulls from the start and gets it, with an owned token
/// // that survives this handle being dropped.
/// let pulled = hub.pull(None).await?;
/// assert_eq!(pulled.batch.events().count(), 1);
/// let caught_up = hub.pull(Some(&pulled.resume)).await?;
/// assert_eq!(caught_up.batch.events().count(), 0, "empty means caught up");
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MemorySyncPeer {
    store: StoreId,
    limits: PeerLimits,
    state: Mutex<PeerState>,
}

#[derive(Debug, Default)]
struct PeerState {
    groups: Vec<EventGroup>,
    seen: Watermark,
}

impl MemorySyncPeer {
    /// A peer with the minimum conformant limits.
    #[must_use]
    pub fn new(store: StoreId) -> Self {
        Self::with_limits(store, PeerLimits::minimum())
    }

    /// A peer that declares `limits`.
    ///
    /// Useful for the case the port exists to make survivable: an origin that
    /// has already durably committed an event larger than the far side can
    /// represent.
    #[must_use]
    pub fn with_limits(store: StoreId, limits: PeerLimits) -> Self {
        Self {
            store,
            limits,
            state: Mutex::new(PeerState::default()),
        }
    }

    /// The store incarnation this peer speaks for.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store
    }

    /// Seeds the peer's log directly, without going through a push.
    ///
    /// For building a fixture's starting state. Skips the limit checks on
    /// purpose: a test that needs an oversized event on the far side has to be
    /// able to put one there.
    ///
    /// # Errors
    ///
    /// [`MemoryPeerError::Poisoned`] if the log lock was poisoned.
    pub fn seed(&self, group: EventGroup) -> Result<(), MemoryPeerError> {
        let mut state = self.state.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        for event in &group.events {
            state.seen.advance(event.id.store(), event.id.position());
        }
        state.groups.push(group);
        Ok(())
    }

    /// How many groups this peer holds.
    ///
    /// # Errors
    ///
    /// [`MemoryPeerError::Poisoned`] if the log lock was poisoned.
    pub fn group_count(&self) -> Result<usize, MemoryPeerError> {
        let state = self.state.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        Ok(state.groups.len())
    }

    fn holds(state: &PeerState, id: EventId) -> bool {
        state
            .seen
            .get(id.store())
            .is_some_and(|high| high >= id.position())
    }
}

impl SendSyncPeer for MemorySyncPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        let state = self.state.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        let mut next = from.copied().unwrap_or_default().next_group;
        let mut budget = PullBatchLimit::from(&self.limits);
        let mut groups = Vec::new();

        // One round trip's worth and no more: the loop stops at the first group
        // that does not fit rather than truncating one, because a half-delivered
        // group is the one thing `EventGroup` exists to prevent.
        while let Some(group) = state.groups.get(next) {
            if !budget.admits(group) {
                break;
            }
            budget.charge(group);
            groups.push(group.clone());
            next += 1;
        }

        Ok(Pulled::new(
            PushBatch::new(groups),
            MemoryResume { next_group: next },
        ))
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        let offered = batch.payload_len();
        if offered > self.limits.max_batch_bytes {
            return Err(MemoryPeerError::BatchTooLarge {
                actual: offered,
                limit: self.limits.max_batch_bytes,
            });
        }

        let mut state = self.state.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        let mut ack = Ack::default();

        for group in &batch.groups {
            let fresh: Vec<ReplicatedEvent> = group
                .events
                .iter()
                .filter(|event| !Self::holds(&state, event.id))
                .cloned()
                .collect();

            ack.skipped += group.events.len() - fresh.len();
            if fresh.is_empty() {
                continue;
            }

            ack.appended += fresh.len();
            for event in &fresh {
                state.seen.advance(event.id.store(), event.id.position());
            }
            state
                .groups
                .push(EventGroup::new(group.guard.clone(), fresh));
        }

        ack.confirmed = state.seen.clone();
        Ok(ack)
    }

    fn limits(&self) -> PeerLimits {
        self.limits.clone()
    }
}
