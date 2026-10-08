//! The host-side control: the reference store's write path, borrowed and owned.
//!
//! ADR-0012 rules `MemoryEventStore` out as the subject — it is the least
//! representative storage shape in the workspace — and this module does not
//! make it the subject. It is here for two other reasons: it is the one store
//! whose borrowed write path is a *pure* clone (`memory.rs:400-413`,
//! `event.clone()` per event, nothing else), so it bounds what owning the batch
//! could ever save; and it is the host on which the caller-side axis — what a
//! retrying caller pays under each shape — can be measured without a JS heap in
//! the way.
//!
//! [`BorrowedLog`] is the extend at `memory.rs:400-413`, verbatim but for the
//! lock, and `tests/host.rs` holds it to the real store's allocation count
//! before anything it prints is trusted. [`OwnedLog`] is the same extend
//! moving each event instead of cloning it.

use happenstance_core::{
    AppendCondition, Event, EventId, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};

/// Why a replica refused a batch. The two refusals `memory.rs` makes before
/// writing, in its order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The batch was empty.
    NoEvents,
    /// The condition matched the stored event at this position.
    Conflict(SequencePosition),
}

/// The position of the event at `index` in the log, as `memory.rs` assigns it.
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index)
        .unwrap_or(u64::MAX - 1)
        .saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// A fixed stamp: the clock is not under measurement and reads no allocator.
const RECORDED_AT: RecordedAt = RecordedAt::from_millis(1_767_225_600_000);

/// The log both replicas keep.
#[derive(Debug)]
struct Log {
    store_id: StoreId,
    events: Vec<SequencedEvent>,
}

impl Log {
    fn new(store_id: StoreId) -> Self {
        Self {
            store_id,
            events: Vec::new(),
        }
    }

    /// The condition check at `memory.rs`, against the stored events.
    fn check(&self, condition: Option<&AppendCondition>) -> Result<(), Refused> {
        let Some(condition) = condition else {
            return Ok(());
        };
        match self.events.iter().find(|existing| {
            condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
        }) {
            Some(conflict) => Err(Refused::Conflict(conflict.position)),
            None => Ok(()),
        }
    }
}

/// One stored row, as `memory.rs` builds it.
fn sequenced(store_id: StoreId, index: usize, event: Event) -> SequencedEvent {
    let position = position_at(index);
    SequencedEvent::new(
        position,
        EventId::new(store_id, position),
        RECORDED_AT,
        event,
    )
}

/// `memory.rs:400-413` as shipped: the batch is borrowed, each event cloned.
#[derive(Debug)]
pub struct BorrowedLog(Log);

impl BorrowedLog {
    /// An empty log under `store_id`.
    #[must_use]
    pub fn new(store_id: StoreId) -> Self {
        Self(Log::new(store_id))
    }

    /// Appends a clone of every event in `events`.
    ///
    /// # Errors
    ///
    /// [`Refused::NoEvents`] for an empty batch; [`Refused::Conflict`] when
    /// `condition` matches a stored event. Either leaves the log unchanged.
    pub fn append(
        &mut self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, Refused> {
        if events.is_empty() {
            return Err(Refused::NoEvents);
        }
        self.0.check(condition)?;
        let first = self.0.events.len();
        let store_id = self.0.store_id;
        // The clone is the subject: it is `memory.rs:412`'s, copied verbatim,
        // because the log stores the event and the batch is only borrowed.
        self.0.events.extend(
            events
                .iter()
                .enumerate()
                .map(|(offset, event)| sequenced(store_id, first + offset, event.clone())),
        );
        Ok(position_at(first + events.len().saturating_sub(1)))
    }

    /// The stored events.
    #[must_use]
    pub fn events(&self) -> &[SequencedEvent] {
        &self.0.events
    }
}

/// The same extend, with the batch owned and each event moved.
#[derive(Debug)]
pub struct OwnedLog(Log);

impl OwnedLog {
    /// An empty log under `store_id`.
    #[must_use]
    pub fn new(store_id: StoreId) -> Self {
        Self(Log::new(store_id))
    }

    /// Appends every event in `events`, moving it.
    ///
    /// # Errors
    ///
    /// As [`BorrowedLog::append`]. A refused batch is dropped: the owned shape
    /// gives the caller nothing back, which is ADR-0012 item 5's problem.
    pub fn append(
        &mut self,
        events: Vec<Event>,
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, Refused> {
        let Some(last) = events.len().checked_sub(1) else {
            return Err(Refused::NoEvents);
        };
        self.0.check(condition)?;
        let first = self.0.events.len();
        let store_id = self.0.store_id;
        self.0.events.extend(
            events
                .into_iter()
                // Moved, never cloned: the arm under measurement.
                .enumerate()
                .map(|(offset, event)| sequenced(store_id, first + offset, event)),
        );
        Ok(position_at(first + last))
    }

    /// The stored events.
    #[must_use]
    pub fn events(&self) -> &[SequencedEvent] {
        &self.0.events
    }
}
