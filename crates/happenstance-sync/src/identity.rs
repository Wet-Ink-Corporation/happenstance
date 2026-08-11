//! Cross-instance event identity.
//!
//! # Why these types are here and not in `happenstance-core`
//!
//! [`SequencePosition`] is meaningful only inside one store, so a peer cannot
//! use it to recognise an event it has already ingested. Something stable and
//! globally unique is needed, and the specification's answer (VT-5) is the pair
//! `(StoreId, SequencePosition)` — the store that minted the event, plus where
//! it landed in *that* store's log.
//!
//! The specification puts that pair in section 2, which is the contract crate.
//! It is sketched **here** instead, on purpose and temporarily: this crate is a
//! phase-2 instrument, `happenstance-core` is frozen for the duration, and the
//! whole point of the exercise is to find out whether an identity minted outside
//! the contract crate can reach a store *through* [`IngestStore`](crate::IngestStore)
//! without the contract crate changing. Coherence is the mechanism, and
//! [`crate::ingest`]'s documentation records what compiling it proved and what
//! it did not.
//!
//! When identity moves to `happenstance-core`, nothing about that result
//! changes: the orphan rule cares about where the *trait* is defined, not about
//! where its argument types come from.
//!
//! # These three types are placeholders, and phase 4 deletes them
//!
//! [`StoreId`], [`EventId`] and [`RecordedAt`] are **not** this crate's to
//! define. VT-4 – VT-10 settle them, ADR-0014 writes them, and they land on
//! `SequencedEvent` in `happenstance-core` at
//! [phase 4](../../../../RUNBOOK.md#phase-4--freeze-the-contract-signatures-value-types-and-identity).
//! VT-5 is `[FROZEN]`. What is sketched here is the *shape the sketch needed in
//! order to compile*, and it is a coincidence rather than a design if it matches
//! what phase 4 arrives at.
//!
//! So they are deliberately **not re-exported from the crate root**, and the
//! omission is load-bearing rather than an oversight. A peer adapter written
//! against this crate also imports from `happenstance`, and once phase 4 lands
//! there would be two `EventId`s in scope — one settled, one a placeholder —
//! with a plain `use` picking whichever came first. Spelling them
//! `identity::EventId` makes the collision unwriteable by accident and makes
//! every site phase 4 must revisit greppable by one name.
//!
//! [`ReplicatedEvent`] and [`Watermark`] *are* re-exported, because they are
//! genuinely replication's own and phase 4 defines nothing of the kind.

use alloc::vec::Vec;
use core::fmt;

use happenstance_core::{Event, SequencePosition};

/// Names one incarnation of a store.
///
/// Not a device and not a peer name (VT-6). A tablet that is wiped and re-seeded
/// is a new incarnation and gets a new `StoreId`, because the old one's
/// positions are about to be re-issued to different events; a peer *name* —
/// "London hub", "tablet 88" — has to survive that, so it belongs to the
/// runner's configuration rather than here.
///
/// The value is 128 opaque bits rather than a string. A string invites operators
/// to encode meaning into it, and every such meaning is one that must then
/// survive a restore-from-backup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreId([u8; 16]);

impl StoreId {
    /// Wraps 128 bits that some other layer generated.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// The raw 128 bits.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl fmt::Display for StoreId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// Globally unique identity for one event: the store that minted it, and where
/// it landed in that store's log.
///
/// A newtype struct rather than `type EventId = (StoreId, SequencePosition)`,
/// and the difference is not cosmetic. A type alias is transparent: every
/// `(StoreId, SequencePosition)` in the program would silently *be* an
/// `EventId`, `.0` and `.1` would be public API, and the compiler would accept a
/// pair assembled from the wrong store's identifier without a word. The newtype
/// makes construction deliberate and makes an `EventId` and a bare position
/// non-interchangeable at every call site (VT-6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId {
    store: StoreId,
    position: SequencePosition,
}

impl EventId {
    /// Mints an identity for an event that `store` accepted at `position`.
    #[must_use]
    pub const fn new(store: StoreId, position: SequencePosition) -> Self {
        Self { store, position }
    }

    /// The store incarnation that minted this identity.
    #[must_use]
    pub const fn store(&self) -> StoreId {
        self.store
    }

    /// Where the event landed in its *origin* store's log.
    ///
    /// Not where it landed here. A replicated event's local position is arrival
    /// order and is unrelated to this one (SY-19).
    #[must_use]
    pub const fn position(&self) -> SequencePosition {
        self.position
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.store, self.position)
    }
}

/// When a store accepted an event, in milliseconds since the Unix epoch.
///
/// A recording, not an ordering key: two stores' clocks disagree, and using this
/// to merge two logs would make the merge depend on NTP (VT-9). It is here
/// because an operator debugging a replication lag needs it, and for no other
/// reason.
///
/// `u64` milliseconds rather than a `chrono`/`time` type, because this crate has
/// no business making that choice for the twelve crates downstream of it, and
/// because the wire format has to be stable across a version bump of whichever
/// one it would have picked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedAt(u64);

impl RecordedAt {
    /// Wraps milliseconds since the Unix epoch.
    #[must_use]
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    /// Milliseconds since the Unix epoch.
    #[must_use]
    pub const fn as_millis(self) -> u64 {
        self.0
    }
}

/// An event on the wire: an [`Event`] plus the two facts its origin store
/// assigned to it.
///
/// This is the unit both [`SyncPeer`](crate::SyncPeer) and
/// [`IngestStore`](crate::IngestStore) move. It carries no
/// [`SequencePosition`] of its own beyond the one inside [`EventId`], because
/// the receiver assigns that and the sender's copy would be actively misleading.
///
/// The payload inside `event` is never decoded here. That is the whole payoff of
/// keeping `happenstance-core` free of `serde` in its default features: a peer
/// forwards bytes it does not understand and cannot corrupt by re-encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ReplicatedEvent {
    /// Who minted this event, and where it landed in their log.
    pub id: EventId,
    /// When the origin store accepted it.
    pub recorded_at: RecordedAt,
    /// The event itself, payload opaque.
    pub event: Event,
}

impl ReplicatedEvent {
    /// Assembles a wire event from an origin store's three facts.
    #[must_use]
    pub const fn new(id: EventId, recorded_at: RecordedAt, event: Event) -> Self {
        Self {
            id,
            recorded_at,
            event,
        }
    }

    /// Bytes this event occupies in a peer's size budget.
    ///
    /// Only the payload and metadata are counted; the type and tags are already
    /// bounded at 255 bytes each by the contract, and no engine struggles with
    /// those. See [`PeerLimits`](crate::PeerLimits).
    #[must_use]
    pub fn payload_len(&self) -> usize {
        self.event.data().len()
            + self
                .event
                .metadata()
                .map_or(0, happenstance_core::bytes::Bytes::len)
    }
}

/// A per-store high-water mark: the last position this side has seen from each
/// origin store it knows about.
///
/// A version vector rather than a scalar, and that is a constraint the *port*
/// imposes rather than a convenience. A hub sees many logs and a spoke sees one;
/// a scalar watermark can express the spoke and cannot express the hub, so a
/// scalar would make hub-and-spoke a special case of peer-to-peer when the
/// specification requires both to be first-class (SY-10).
///
/// Kept sorted by [`StoreId`] so that two watermarks compare and serialise
/// canonically.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Watermark {
    marks: Vec<(StoreId, SequencePosition)>,
}

impl Watermark {
    /// An empty watermark: nothing seen from anybody.
    #[must_use]
    pub const fn new() -> Self {
        Self { marks: Vec::new() }
    }

    /// The last position seen from `store`, if any.
    #[must_use]
    pub fn get(&self, store: StoreId) -> Option<SequencePosition> {
        self.marks
            .binary_search_by_key(&store, |(id, _)| *id)
            .ok()
            .map(|index| self.marks[index].1)
    }

    /// Raises the mark for `store` to `position`, if that is an advance.
    ///
    /// Monotonic on purpose: a watermark that can go backwards turns a
    /// re-delivered batch into a replay of everything after it.
    pub fn advance(&mut self, store: StoreId, position: SequencePosition) {
        match self.marks.binary_search_by_key(&store, |(id, _)| *id) {
            Ok(index) => {
                let existing = &mut self.marks[index].1;
                if position > *existing {
                    *existing = position;
                }
            }
            Err(index) => self.marks.insert(index, (store, position)),
        }
    }

    /// Every mark, ascending by [`StoreId`].
    pub fn iter(&self) -> impl Iterator<Item = (StoreId, SequencePosition)> + '_ {
        self.marks.iter().copied()
    }

    /// How many stores this watermark knows about.
    #[must_use]
    pub fn len(&self) -> usize {
        self.marks.len()
    }

    /// Whether nothing has been seen from anybody.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }
}
