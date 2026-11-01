use std::collections::BTreeSet;

use happenstance_cloudflare::happenstance_sync::{FailureClass, ReplicatedEvent};
use happenstance_core::{EventId, SequencePosition, Tag};

use super::{
    Attempt, CEILING_ORIGIN, MAX_EVENTS_PER_CALL, MAX_TAGS_EVENTS_AT_CEILING, Shape, ShapeError,
    foreign_batch,
};
use crate::fixture::WorkerdFixture;

fn at(position: u64) -> EventId {
    EventId::new(
        CEILING_ORIGIN,
        SequencePosition::new(position).expect("a test position is not zero"),
    )
}

#[test]
fn each_shape_builds_n_distinct_foreign_events_at_its_maxima() {
    for shape in [Shape::Minimal, Shape::MaxTags, Shape::MaxData] {
        let batch = foreign_batch(shape, 3).expect("three events of every shape build");

        let ids: Vec<EventId> = batch.iter().map(|event| event.id).collect();
        assert_eq!(ids, vec![at(1), at(2), at(3)], "{shape:?}");
        let distinct: BTreeSet<_> = ids.iter().map(ToString::to_string).collect();
        assert_eq!(distinct.len(), 3, "{shape:?}");
        for event in &batch {
            let (data, tags) = match shape {
                Shape::Minimal => (8, 1),
                Shape::MaxTags => (8, 1_024),
                Shape::MaxData => (1024 * 1024, 1),
            };
            assert_eq!(event.event.data().len(), data, "{shape:?}");
            assert_eq!(event.event.tags().len(), tags, "{shape:?}");
            assert_eq!(event.event.metadata(), None, "{shape:?}");
        }
    }
}

#[test]
fn an_unknown_shape_is_refused_by_name() {
    assert_eq!("max-tags".parse::<Shape>(), Ok(Shape::MaxTags));
    assert_eq!("minimal".parse::<Shape>(), Ok(Shape::Minimal));
    assert_eq!("max-data".parse::<Shape>(), Ok(Shape::MaxData));

    let refused = "max-metadata".parse::<Shape>();

    assert_eq!(refused, Err(ShapeError::Unknown("max-metadata".to_owned())));
    assert_eq!(
        refused.map_err(|error| error.to_string()),
        Err("no ingest ceiling shape max-metadata; known: minimal, max-tags, max-data".to_owned())
    );
}

#[test]
fn zero_events_is_refused() {
    assert_eq!(
        foreign_batch(Shape::Minimal, 0).map(|batch| batch.len()),
        Err(ShapeError::Position)
    );
}

#[test]
fn a_count_past_the_cap_is_refused_before_anything_is_allocated() {
    assert_eq!(
        foreign_batch(Shape::MaxData, MAX_EVENTS_PER_CALL + 1).map(|batch| batch.len()),
        Err(ShapeError::TooMany {
            events: MAX_EVENTS_PER_CALL + 1
        })
    );
    // One past the declared batch ceiling, so the top of the search reaches
    // the adapter's own refusal.
    assert_eq!(
        Some(MAX_EVENTS_PER_CALL),
        <WorkerdFixture<'static> as happenstance_testkit::Fixture>::MAX_EVENTS_PER_BATCH
            .map(|ceiling| ceiling + 1)
    );
}

#[test]
fn a_refusal_line_names_its_class_and_quotes_the_error() {
    let refused = Attempt::Refused {
        class: FailureClass::Split,
        error: "ingest exceeds the store's events per batch limit: 1025".to_owned(),
    };
    let accepted = Attempt::Accepted {
        events: 37,
        appended: 37,
    };

    assert_eq!(
        refused.line(),
        "refused class=Split\ningest exceeds the store's events per batch limit: 1025"
    );
    assert_eq!(accepted.line(), "accepted events=37 appended=37");
}

/// The addresses of one event's tag names, in order.
fn name_addresses(event: &ReplicatedEvent) -> Vec<*const u8> {
    event
        .event
        .tags()
        .iter()
        .map(|tag| tag.as_str().as_ptr())
        .collect()
}

#[test]
fn every_max_tags_event_borrows_one_static_set_of_tag_names() {
    let first_call = foreign_batch(Shape::MaxTags, 2).expect("two max-tags events build");
    let second_call = foreign_batch(Shape::MaxTags, 1).expect("one max-tags event builds");
    let [one, two] = first_call.as_slice() else {
        panic!("two events were asked for, got {}", first_call.len());
    };
    let [three] = second_call.as_slice() else {
        panic!("one event was asked for, got {}", second_call.len());
    };

    // An owned tag is a fresh `String` per event, at a fresh address; a
    // borrowed one points at the same name for every event and every call.
    assert_eq!(name_addresses(one).len(), 1_024);
    assert_eq!(
        name_addresses(one),
        name_addresses(two),
        "two events, one call"
    );
    assert_eq!(name_addresses(one), name_addresses(three), "two calls");
}

#[test]
fn the_tag_names_are_unchanged() {
    for (shape, count) in [
        (Shape::Minimal, 1),
        (Shape::MaxTags, 1_024),
        (Shape::MaxData, 1),
    ] {
        let batch = foreign_batch(shape, 1).expect("one event builds");
        let names: Vec<&str> = batch
            .iter()
            .flat_map(|event| event.event.tags().iter().map(Tag::as_str))
            .collect();
        // `Tags` holds its canonical, sorted form.
        let expected: BTreeSet<String> = (0..count).map(|n| format!("ceiling:{n}")).collect();
        let expected: Vec<&str> = expected.iter().map(String::as_str).collect();
        assert_eq!(names, expected, "{shape:?}");
    }
}

/// The `max-tags` shape's wall is now the adapter's own refusal: 512 events of
/// 1,024 tags is exactly the declared tags-per-batch ceiling, and 513 is over
/// it (VT-36, ADR-0100). Arithmetic only, from the fixture's declarations: 513
/// events of 1,024 tags is half a million owned tags, which this host has no
/// business building.
#[test]
fn the_max_tags_shape_crosses_the_tags_per_batch_ceiling_past_512_events() {
    let fixture = |name: &str, declared: Option<usize>| {
        declared.unwrap_or_else(|| panic!("{name} is declared by this fixture"))
    };
    let per_batch = fixture(
        "MAX_TAGS_PER_BATCH",
        <WorkerdFixture<'static> as happenstance_testkit::Fixture>::MAX_TAGS_PER_BATCH,
    );
    let per_event = fixture(
        "MAX_TAGS_PER_EVENT",
        <WorkerdFixture<'static> as happenstance_testkit::Fixture>::MAX_TAGS_PER_EVENT,
    );

    assert_eq!(MAX_TAGS_EVENTS_AT_CEILING, 512);
    assert_eq!(MAX_TAGS_EVENTS_AT_CEILING * per_event, per_batch);
    assert!((MAX_TAGS_EVENTS_AT_CEILING + 1) * per_event > per_batch);
    assert!(
        MAX_TAGS_EVENTS_AT_CEILING < MAX_EVENTS_PER_CALL,
        "the search reaches past the tags-per-batch wall"
    );
}
