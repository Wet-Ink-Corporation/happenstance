//! Property tests for the contract's invariants.
//!
//! The conformance suite checks worked examples: given these events and this
//! query, expect that result. These check the *laws* the examples are instances
//! of, over inputs nobody would think to write down.
//!
//! They live in the testkit rather than in `happenstance-core` because they are
//! the same claims an adapter must satisfy — an adapter that pushes query
//! matching down into SQL is asserting these laws about its `WHERE` clause, and
//! should be able to reuse the generators.

// `proptest` is a native-only dev-dependency; these laws are target-independent
// and checking them once, natively, is enough.
#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use happenstance_core::{EventType, Query, QueryItem, Tag, Tags};
use proptest::prelude::*;

/// Generates a tag from a small alphabet, so collisions and duplicates actually
/// occur rather than being vanishingly unlikely.
fn any_tag() -> impl Strategy<Value = Tag> {
    prop::sample::select(vec!["a", "b", "c", "d", "e"]).prop_map(|value| Tag::new(value).unwrap())
}

fn any_tags() -> impl Strategy<Value = Tags> {
    prop::collection::vec(any_tag(), 0..6).prop_map(|tags| tags.into_iter().collect())
}

fn any_event_type() -> impl Strategy<Value = EventType> {
    prop::sample::select(vec!["A", "B", "C"]).prop_map(|value| EventType::new(value).unwrap())
}

proptest! {
    /// Canonicalising an already-canonical `Tags` changes nothing.
    ///
    /// This is what lets adapters treat the sorted encoding as a stable index
    /// key: if it were not idempotent, a round trip through storage could
    /// produce a value that no longer compares equal to itself.
    #[test]
    fn tags_canonicalisation_is_idempotent(tags in any_tags()) {
        let again: Tags = tags.iter().cloned().collect();
        prop_assert_eq!(&tags, &again);
    }

    /// `Tags` equality is set equality, independent of insertion order.
    #[test]
    fn tags_equality_ignores_insertion_order(tags in prop::collection::vec(any_tag(), 0..6)) {
        let forwards: Tags = tags.iter().cloned().collect();
        let backwards: Tags = tags.iter().rev().cloned().collect();
        prop_assert_eq!(forwards, backwards);
    }

    /// Every tag set contains itself, and contains the empty set.
    #[test]
    fn contains_all_is_reflexive_and_has_an_identity(tags in any_tags()) {
        prop_assert!(tags.contains_all(&tags));
        prop_assert!(tags.contains_all(&Tags::empty()));
    }

    /// `contains_all` agrees with a naive per-element check.
    ///
    /// The real implementation is a single merge-scan over two sorted slices,
    /// which is easy to get subtly wrong at the boundaries. This pins it to the
    /// obviously-correct definition.
    #[test]
    fn contains_all_agrees_with_elementwise_containment(
        haystack in any_tags(),
        needles in any_tags(),
    ) {
        let naive = needles.iter().all(|tag| haystack.contains(tag));
        prop_assert_eq!(haystack.contains_all(&needles), naive);
    }

    /// Query matching does not depend on the order items were supplied in.
    #[test]
    fn query_matching_is_order_insensitive(
        event_type in any_event_type(),
        event_tags in any_tags(),
        types_a in prop::collection::vec(any_event_type(), 1..3),
        tags_b in prop::collection::vec(any_tag(), 1..3),
    ) {
        let item_a = QueryItem::of_types(types_a).unwrap();
        let item_b = QueryItem::tagged(tags_b.into_iter().collect()).unwrap();

        let forwards = Query::from_items([item_a.clone(), item_b.clone()]).unwrap();
        let backwards = Query::from_items([item_b, item_a]).unwrap();

        prop_assert_eq!(
            forwards.matches(&event_type, &event_tags),
            backwards.matches(&event_type, &event_tags),
        );
    }

    /// Adding an item to a query can only widen what it matches.
    ///
    /// Items are ORed, so this must hold; if it ever fails, the combinator is
    /// ANDing somewhere it should not.
    #[test]
    fn adding_a_query_item_is_monotonic(
        event_type in any_event_type(),
        event_tags in any_tags(),
        types in prop::collection::vec(any_event_type(), 1..3),
        extra_tags in prop::collection::vec(any_tag(), 1..3),
    ) {
        let narrow = QueryItem::of_types(types).unwrap();
        let extra = QueryItem::tagged(extra_tags.into_iter().collect()).unwrap();

        let one = Query::from_items([narrow.clone()]).unwrap();
        let two = Query::from_items([narrow, extra]).unwrap();

        if one.matches(&event_type, &event_tags) {
            prop_assert!(two.matches(&event_type, &event_tags));
        }
    }

    /// `Query::all` matches whatever any other query matches.
    #[test]
    fn query_all_is_the_top_element(
        event_type in any_event_type(),
        event_tags in any_tags(),
    ) {
        prop_assert!(Query::all().matches(&event_type, &event_tags));
    }

    /// An event matches a tag-only item exactly when its tags are a superset.
    #[test]
    fn tag_item_matches_iff_superset(
        event_type in any_event_type(),
        event_tags in any_tags(),
        wanted in prop::collection::vec(any_tag(), 1..4),
    ) {
        let wanted: Tags = wanted.into_iter().collect();
        let item = QueryItem::tagged(wanted.clone()).unwrap();

        prop_assert_eq!(
            item.matches(&event_type, &event_tags),
            event_tags.contains_all(&wanted),
        );
    }
}
