//! Small builders shared by the conformance rules.
//!
//! Public because adapter authors writing their own extra tests should express
//! them in the same vocabulary the suite uses.

use happenstance_core::{
    AppendCondition, Event, EventType, Query, QueryItem, SequencePosition, Tags,
};

/// Builds an event of `event_type` with no tags.
///
/// # Panics
///
/// Panics if `event_type` is not a valid [`EventType`]. Test-only helper.
#[must_use]
pub fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).expect("valid event type")
}

/// Builds an event of `event_type` carrying `key:value` tags.
///
/// # Panics
///
/// Panics if `event_type` or any tag pair is invalid. Test-only helper.
#[must_use]
pub fn tagged_event(event_type: &str, tags: &[(&str, &str)]) -> Event {
    event(event_type).with_tags(self::tags(tags))
}

/// Builds an event with a distinct payload, for checking round-tripping.
///
/// # Panics
///
/// Panics if `event_type` is invalid. Test-only helper.
#[must_use]
pub fn event_with_payload(event_type: &str, payload: &'static [u8]) -> Event {
    Event::new(event_type, payload).expect("valid event type")
}

/// Builds a canonical tag set from `key:value` pairs.
///
/// # Panics
///
/// Panics if any pair is invalid. Test-only helper.
#[must_use]
pub fn tags(pairs: &[(&str, &str)]) -> Tags {
    Tags::from_pairs(pairs.iter().copied()).expect("valid tags")
}

/// Builds an [`EventType`].
///
/// # Panics
///
/// Panics if `value` is invalid. Test-only helper.
#[must_use]
pub fn event_type(value: &str) -> EventType {
    EventType::new(value).expect("valid event type")
}

/// Builds a single-item query constrained by type.
///
/// # Panics
///
/// Panics if `types` is empty or holds an invalid type. Test-only helper.
#[must_use]
pub fn query_of_types(types: &[&str]) -> Query {
    Query::from_item(QueryItem::of_types(types.iter().copied()).expect("valid types"))
        .expect("non-empty query")
}

/// Builds a single-item query constrained by tags.
///
/// # Panics
///
/// Panics if `pairs` is empty or invalid. Test-only helper.
#[must_use]
pub fn query_tagged(pairs: &[(&str, &str)]) -> Query {
    Query::from_item(QueryItem::tagged(tags(pairs)).expect("non-empty tags"))
        .expect("non-empty query")
}

/// Builds a single-item query constrained by both type and tags.
///
/// # Panics
///
/// Panics if either constraint is invalid. Test-only helper.
#[must_use]
pub fn query_of(types: &[&str], pairs: &[(&str, &str)]) -> Query {
    Query::from_item(QueryItem::new(types.iter().copied(), tags(pairs)).expect("valid query item"))
        .expect("non-empty query")
}

/// Builds an append condition over the whole log.
#[must_use]
pub fn condition(query: Query) -> AppendCondition {
    AppendCondition::new(query)
}

/// Builds an append condition restricted to events after `position`.
///
/// # Panics
///
/// Panics if `position` is zero. Test-only helper.
#[must_use]
pub fn condition_after(query: Query, position: u64) -> AppendCondition {
    AppendCondition::new(query).after(SequencePosition::new(position).expect("non-zero position"))
}
