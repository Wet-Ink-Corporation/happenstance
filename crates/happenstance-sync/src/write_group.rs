//! The write-group port: a store's log read back in the writes that made it.
//!
//! # Why a store has to say this
//!
//! [`EventStore::read`](happenstance_core::EventStore::read) yields one event
//! at a time, and nothing on a
//! [`SequencedEvent`](happenstance_core::SequencedEvent) says which `append`,
//! or which ingested group, wrote it. A runner pushing the local log, or a
//! [`StoreSyncPeer`](crate::StoreSyncPeer) answering a pull, therefore could
//! not keep a multi-event write whole: it cut one at a page boundary, or merged
//! independent writes into one atomic unit, and a relayed foreign group lost
//! the guard it arrived with (SY-30, #420).
//!
//! [`WriteGroupStore`] is the store saying it. Every row belongs to exactly one
//! write group: one `append` call is one group, and one
//! [`IngestGroup`](crate::IngestGroup) that lands at least one foreign event is
//! one group, its compensation included (SY-2). A group is named by its
//! [`first`](WriteGroup::first) position, the lowest local position among the
//! rows it wrote, and reads back with the guard it was ingested under.
//!
//! # Why this trait lives here and not on `EventStore`
//!
//! The argument [`ingest`](crate::ingest) makes for [`IngestStore`](crate::IngestStore):
//! coherence lets each adapter implement a trait this crate defines, so
//! `happenstance-core` never hears about write groups and no published read
//! type changes. A group id is store-local and never travels; the wire shape
//! stays [`EventGroup`].
//!
//! # Why `first` is the cursor
//!
//! A store that serialises its writers (SQLite, the Durable Object, the
//! in-memory stores) writes a group's rows contiguously, and either end of the
//! group would do. Postgres and Neon do not serialise: two concurrent appends
//! can interleave their positions, G = {5, 7} and H = {6, 8}. Resuming above
//! the last position pushed after G would skip H's row 6; resuming above G's
//! first position does not. Every row at a position at or below a group's
//! `first` belongs to a group whose own `first` is no higher, so it was read
//! already, and a group commits in one transaction, so no group with `first`
//! at or below an observed position can appear later (ES-10).
//!
//! # Rows written before a store recorded groups
//!
//! Read as **one group each**, named by their own position, with no guard.
//! Which rows one earlier call wrote is not recorded anywhere, so it cannot be
//! recovered; a pre-upgrade multi-event append still travels split, exactly as
//! it did before.

use alloc::vec::Vec;
use core::num::NonZeroUsize;

use happenstance_core::{AppendCondition, SequencePosition};

use crate::identity::ReplicatedEvent;
use crate::peer::{EventGroup, Waiting};

// Two adapter-side helpers, re-exported here and nowhere else: one import path
// each, and module-qualified at the call site, as `wire`'s items are.
mod guard;
mod page;

pub use guard::{GuardCodecError, STORED_GUARD_FORMAT, decode_guard, encode_guard};
pub use page::{GroupOrderError, GroupPageBuilder};

/// A store that can read its log back in the write groups it wrote.
///
/// Every row belongs to exactly one group: one `append` is one group, and one
/// [`IngestGroup`](crate::IngestGroup) that lands at least one foreign event is
/// one group, its compensation included. A group is named by the lowest local
/// position among the rows it wrote, which is its [`first`](WriteGroup::first).
///
/// Only a runner and a store peer should hold one of these, for the reason
/// [`IngestStore`](crate::IngestStore) gives: it is replication's view of the
/// log, not an application's.
///
/// # Flavours
///
/// This is the `!Send` flavour, and generic code should bind it (RS-20-2).
/// Adapters that can cross threads implement [`SendWriteGroupStore`] and get
/// this for free; the Durable Object implements this one.
#[trait_variant::make(SendWriteGroupStore: Send)]
pub trait WriteGroupStore {
    /// How this store fails, stating a class so a runner never infers one.
    type Error: core::error::Error + crate::Classify + 'static;

    /// The write groups whose [`first`](WriteGroup::first) position is above
    /// `after` (every group when `None`), in ascending `first` order, each
    /// whole.
    ///
    /// Fills to `max_events` events and stops **before** the group that would
    /// take the page past it. The first group is returned whole however many
    /// events it carries, so a caller always learns what blocks it; whether it
    /// can travel is the caller's to decide. Chaining pages with
    /// `after = Some(last.first())` returns every group exactly once.
    ///
    /// [`GroupPage::waiting`] is [`Waiting::Nothing`] only when the store saw
    /// no group past the page, and [`Waiting::Unknown`] otherwise. A store
    /// never counts: a count is a scan of the rest of the log.
    ///
    /// # Errors
    ///
    /// The adapter's error; its [`Classify`](crate::Classify) impl states the
    /// class. A stored guard the store cannot decode is one of them.
    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error>;
}

/// One write as the store recorded it: the events one `append`, or one
/// ingested group with its compensation, wrote together, and the origin's
/// guard.
///
/// Never empty: it is built from its first event and grown with
/// [`push`](Self::push), so a group with no events cannot be written down.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteGroup {
    first: SequencePosition,
    guard: Option<AppendCondition>,
    events: Vec<ReplicatedEvent>,
}

impl WriteGroup {
    /// A group whose lowest local position is `first`, starting with `head`,
    /// the event written there.
    ///
    /// `guard` is the condition an ingested group arrived under, and `None`
    /// for a local append: a local condition is position-relative, which SY-6
    /// makes unreplicable.
    #[must_use]
    pub fn new(
        first: SequencePosition,
        guard: Option<AppendCondition>,
        head: ReplicatedEvent,
    ) -> Self {
        Self {
            first,
            guard,
            events: alloc::vec![head],
        }
    }

    /// Adds the group's next event, in local position order.
    pub fn push(&mut self, event: ReplicatedEvent) {
        self.events.push(event);
    }

    /// The lowest local position among the rows this group wrote: its name,
    /// and the cursor a caller resumes above.
    #[must_use]
    pub const fn first(&self) -> SequencePosition {
        self.first
    }

    /// The condition the group was ingested under, if it arrived with one.
    #[must_use]
    pub const fn guard(&self) -> Option<&AppendCondition> {
        self.guard.as_ref()
    }

    /// The group's events, in local position order. Never empty.
    #[must_use]
    pub fn events(&self) -> &[ReplicatedEvent] {
        &self.events
    }

    /// Total payload bytes, as [`PushBatch::payload_len`](crate::PushBatch::payload_len)
    /// counts them.
    #[must_use]
    pub fn payload_len(&self) -> usize {
        self.events.iter().map(ReplicatedEvent::payload_len).sum()
    }
}

/// The wire shape of a write group: the guard travels as evidence (SY-6), the
/// events in order. Moves both; nothing is copied.
impl From<WriteGroup> for EventGroup {
    fn from(group: WriteGroup) -> Self {
        Self::new(group.guard, group.events)
    }
}

/// What one [`read_groups`](WriteGroupStore::read_groups) returned.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct GroupPage {
    /// Whole groups, ascending by [`first`](WriteGroup::first).
    pub groups: Vec<WriteGroup>,
    /// Whether any group lay past the page when it was read.
    pub waiting: Waiting,
}

impl GroupPage {
    /// A page of `groups`, saying what lay past them.
    #[must_use]
    pub const fn new(groups: Vec<WriteGroup>, waiting: Waiting) -> Self {
        Self { groups, waiting }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use happenstance_core::{Event, EventId, Query, QueryItem, RecordedAt, StoreId};

    use super::{EventGroup, SequencePosition, WriteGroup};
    use crate::identity::ReplicatedEvent;

    fn event(position: u64, data: &'static [u8]) -> ReplicatedEvent {
        let position = SequencePosition::new(position).expect("test positions are non-zero");
        ReplicatedEvent::new(
            EventId::new(StoreId::from_bytes([3; 16]), position),
            RecordedAt::from_millis(1_700_000_000_000),
            Event::new("Written", data).expect("a valid event type"),
        )
    }

    fn guard() -> happenstance_core::AppendCondition {
        happenstance_core::AppendCondition::new(Query::from_item(
            QueryItem::of_types(["Written"]).expect("a valid item"),
        ))
    }

    #[test]
    fn a_group_keeps_its_events_in_the_order_they_were_pushed() {
        let first = SequencePosition::new(7).expect("non-zero");
        let mut group = WriteGroup::new(first, None, event(7, b"a"));
        group.push(event(8, b"bb"));
        group.push(event(9, b"ccc"));

        assert_eq!(group.first(), first);
        assert_eq!(
            group.events(),
            &[event(7, b"a"), event(8, b"bb"), event(9, b"ccc")]
        );
        assert_eq!(group.payload_len(), 6);
    }

    #[test]
    fn into_an_event_group_moves_the_guard_and_the_events() {
        let mut group = WriteGroup::new(SequencePosition::FIRST, Some(guard()), event(1, b"a"));
        group.push(event(2, b"b"));

        let wire = EventGroup::from(group);

        assert_eq!(
            wire,
            EventGroup::new(Some(guard()), vec![event(1, b"a"), event(2, b"b")])
        );
    }
}
