//! ADR-0069's total constructor: `QueryItem::of` cannot be called with zero
//! types, so it returns a `QueryItem` rather than a `Result`, and it
//! canonicalises exactly as `QueryItem::new` does.
//!
//! The public API only, so the oracle cannot borrow the implementation's
//! helper (RS-60-4). The comparison against `new` is what pins the two doors
//! together; the `BTreeSet` oracle is what stops that comparison passing when
//! both doors are wrong the same way.

use std::collections::BTreeSet;

use happenstance_core::{EventType, QueryItem, Tags};
use proptest::prelude::*;

/// The spelling ADR-0069 named, settled by compiling it: a type the caller
/// already holds, no further types (`[]` infers its element from the
/// parameter), and a `Tags`. No `?` and no `expect` on the call — it is total.
#[test]
fn of_with_no_further_types_is_total_and_agrees_with_new() {
    const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
    let tags = Tags::from_pairs([("course", "c1")]).expect("a literal, valid tag");

    let item: QueryItem = QueryItem::of(COURSE_DEFINED, [], tags.clone());

    assert_eq!(item.types(), [COURSE_DEFINED]);
    assert_eq!(
        item,
        QueryItem::new([COURSE_DEFINED], tags).expect("one type constrains the item")
    );
}

/// An empty `Tags` is not a reason to refuse: the first type already
/// constrains the item, which `new` turns away only when the types are empty
/// too.
#[test]
fn of_with_empty_tags_is_the_type_only_item() {
    let item = QueryItem::of(
        EventType::from_static("B"),
        [EventType::from_static("A"), EventType::from_static("B")],
        Tags::empty(),
    );

    assert_eq!(
        item,
        QueryItem::of_types(["A", "B"]).expect("two literal, valid types")
    );
}

/// A deliberately small alphabet, so duplicates and reorderings are the common
/// case rather than the rare one: canonicalisation is invisible on distinct,
/// sorted input.
fn any_type() -> impl Strategy<Value = EventType> {
    prop::sample::select(vec!["A", "B", "C", "D"]).prop_map(EventType::from_static)
}

fn any_tags() -> impl Strategy<Value = Tags> {
    prop::collection::vec(
        (
            prop::sample::select(vec!["course", "student"]),
            prop::sample::select(vec!["1", "2"]),
        ),
        0..4,
    )
    .prop_map(|pairs| Tags::from_pairs(pairs).expect("an alphabet of valid tags"))
}

proptest! {
    #[test]
    fn both_doors_canonicalise_identically(
        first in any_type(),
        rest in prop::collection::vec(any_type(), 0..6),
        tags in any_tags(),
    ) {
        let all: Vec<EventType> = core::iter::once(first.clone()).chain(rest.iter().cloned()).collect();

        let through_of = QueryItem::of(first, rest, tags.clone());
        let through_new = QueryItem::new(all.iter().cloned(), tags.clone())
            .expect("at least one type constrains the item");

        // The oracle: a sorted set, which deduplicates by construction rather
        // than by `sort` then `dedup`.
        let expected: Vec<EventType> = all.into_iter().collect::<BTreeSet<_>>().into_iter().collect();

        prop_assert_eq!(through_of.types(), expected.as_slice());
        prop_assert_eq!(through_of.tags(), &tags);
        prop_assert_eq!(through_of, through_new);
    }
}
