//! SY-28's log and its comparison, kept out of `sync_suite.rs` so the rule
//! there reads as what it checks.

use alloc::collections::BTreeMap;
use alloc::collections::btree_map::Entry;
use alloc::format;
use alloc::vec;

use happenstance_core::{
    Event, EventId, EventType, RecordedAt, SequencePosition, StoreId, Tag, Tags,
};
use happenstance_sync::{EventGroup, PushBatch, ReplicatedEvent};

use crate::violation::{Violation, ensure};

/// The two origins whose events the log carries.
const NORTH_DEPOT: StoreId = StoreId::from_bytes([0xA1; 16]);
const SOUTH_DEPOT: StoreId = StoreId::from_bytes([0xA2; 16]);

const PART_SHIPPED: EventType = EventType::from_static("PartShipped");
const PART_PRICED: EventType = EventType::from_static("PartPriced");
const PART_INSPECTED: EventType = EventType::from_static("PartInspected");

/// A log spread over both axes a filter on `Query`'s surface cuts along,
/// types and tags: two origins, three event types, two regions, a
/// confidential pricing slice that rides in a two-event group, an untagged
/// event, and metadata.
///
/// Each origin's positions leave gaps, as a conformant origin may, and no
/// check reads them as anything but part of an identity.
///
/// # Errors
///
/// A [`Violation`] of `clause` if one of the rule's own events is invalid.
pub(super) fn sliced_log(clause: &'static str) -> Result<PushBatch, Violation> {
    let event = |origin: StoreId, position: u64, event_type: EventType, tags: &[&'static str]| {
        let position = SequencePosition::new(position)
            .ok_or_else(|| Violation::new(clause, "the rule's origin position is zero"))?;
        let tags: Tags = tags.iter().copied().map(Tag::from_static).collect();
        // Opaque bytes, distinct per event, so "byte for byte" means something
        // and nothing here is decoded (SY-35).
        let data = format!("{origin}/{position}").into_bytes();
        let event = Event::new(event_type, data)
            .map_err(|error| {
                Violation::new(clause, format!("the rule's own event is invalid: {error}"))
            })?
            .with_tags(tags);
        Ok::<_, Violation>(ReplicatedEvent::new(
            EventId::new(origin, position),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        ))
    };
    let shipped_north = event(NORTH_DEPOT, 2, PART_SHIPPED, &["part:p1", "region:north"])?;
    let priced = event(NORTH_DEPOT, 5, PART_PRICED, &["part:p1", "slice:pricing"])?;
    let mut repriced = event(NORTH_DEPOT, 9, PART_PRICED, &["part:p1", "slice:pricing"])?;
    repriced.event = repriced.event.with_metadata(b"price-list:2026-q4".to_vec());
    let shipped_south = event(SOUTH_DEPOT, 4, PART_SHIPPED, &["part:p2", "region:south"])?;
    let inspected = event(SOUTH_DEPOT, 11, PART_INSPECTED, &[])?;
    Ok(PushBatch::new(vec![
        EventGroup::new(None, vec![shipped_north]),
        EventGroup::new(None, vec![priced, repriced]),
        EventGroup::new(None, vec![shipped_south, inspected]),
    ]))
}

/// Holds what a peer handed back to what was pushed to it: every pushed
/// identity is there, each event equal to its push byte for byte, and no
/// identity comes back twice with different bytes.
///
/// # Errors
///
/// A [`Violation`] of `clause` for the first identity that breaks one of the
/// three.
pub(super) fn holds_the_whole_log(
    pushed: &PushBatch,
    handed_back: impl IntoIterator<Item = ReplicatedEvent>,
    clause: &'static str,
) -> Result<(), Violation> {
    let mut held: BTreeMap<EventId, ReplicatedEvent> = BTreeMap::new();
    for event in handed_back {
        match held.entry(event.id) {
            Entry::Vacant(slot) => {
                slot.insert(event);
            }
            Entry::Occupied(first) => ensure(*first.get() == event, clause, || {
                format!(
                    "the peer handed back {} twice, as {:?} and as {event:?}; one identity \
                     names one event",
                    event.id,
                    first.get()
                )
            })?,
        }
    }
    for expected in pushed.events() {
        let Some(got) = held.get(&expected.id) else {
            return Err(Violation::new(
                clause,
                format!(
                    "{} ({}, tags {:?}) was pushed and never handed back; a peer offers its \
                     counterpart every event it holds (SY-27), and a slice that must not \
                     travel is a store of its own, not a filter on the peer",
                    expected.id,
                    expected.event.event_type(),
                    expected.event.tags()
                ),
            ));
        };
        ensure(got == expected, clause, || {
            format!(
                "{} came back as {got:?} and was pushed as {expected:?}; a round trip \
                 preserves type, tags, data, metadata and recorded time byte for byte",
                expected.id
            )
        })?;
    }
    Ok(())
}
