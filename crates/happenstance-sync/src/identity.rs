//! What replication moves, and how far it has got.
//!
//! # The identity is the contract's
//!
//! [`SequencePosition`] is meaningful only inside one store, so a peer cannot
//! use it to recognise an event it has already ingested. Something stable and
//! globally unique is needed, and the specification's answer (VT-5) is the pair
//! `(StoreId, SequencePosition)` — the store that minted the event, plus where
//! it landed in *that* store's log.
//!
//! That pair is [`happenstance_core::EventId`], beside
//! [`StoreId`] and [`RecordedAt`], and this module uses them as they are. It
//! used to carry placeholders of all three, sketched here at phase 2 so that the
//! port could compile before the contract had an identity, and marked for
//! deletion once it did. They outlived phase 4 by thirteen phases, and the cost
//! of that was not cosmetic: the placeholder `RecordedAt` was a `u64`, and the
//! contract's is an `i64`, so a pre-1970 recording could not travel through
//! [`ReplicatedEvent`] unchanged — which is VT-9's restated falsifier, fired by
//! construction. Phase 17 deleted them, because an ingest path that writes a
//! foreign identity into a real store has to speak the store's own types.
//!
//! What remains here is genuinely replication's own: [`ReplicatedEvent`], the
//! unit a peer moves, and [`Watermark`], the version vector it resumes against.
//! The contract defines nothing of either kind, and both are re-exported from
//! the crate root.

use alloc::vec::Vec;

use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};

/// An event on the wire: an [`Event`] plus the two facts its origin store
/// assigned to it.
///
/// This is the unit both [`SyncPeer`](crate::SyncPeer) and
/// [`IngestStore`](crate::IngestStore) move. It carries no
/// [`SequencePosition`] of its own beyond the one inside [`EventId`], because
/// the receiver assigns that and the sender's copy would be actively misleading.
/// It is [`SequencedEvent`](happenstance_core::SequencedEvent) field for field,
/// less the local `position` — and that omission is SY-19, not an economy.
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
