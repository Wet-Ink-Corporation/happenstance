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

// Two conditions, and neither is redundant (CF-21). `proptest` is an optional
// dependency of the *non-wasm32* target table, so on wasm32 the crate is not in
// the graph at all — but a **feature is not target-scoped**, and `--all-features`
// therefore sets `feature = "proptest"` on every target including that one.
// Without the target condition this file is compiled for wasm32 against a crate
// that does not exist there. These laws are target-independent in any case, so
// checking them once, natively, is enough.
#![cfg(all(not(target_arch = "wasm32"), feature = "proptest"))]
#![allow(clippy::unwrap_used)]

use happenstance_core::{AppendCondition, EventType, Query, QueryItem, SequencePosition, Tags};
// The generators are the testkit's, not this file's (CF-21). An adapter pushing
// query matching down into SQL is asserting these same laws about its `WHERE`
// clause, and generators private to an integration test are reachable by nobody
// — which made that claim false for as long as they lived here.
//
// It was false a second time and for a subtler reason, which is why this is a
// paragraph rather than a line. `any_query`/`any_query_item` were *copied* into
// this file rather than imported, so the oracle properties pinned a duplicate of
// the generator `model.rs` uses instead of the generator itself. Both copies
// compiled, both test sets passed, and widening `strategies::any_query_item` — a
// new item shape, a longer alphabet, a nested query form — would have started
// the model generating inputs these properties had never seen, silently. Only
// `any_position` is local now, and deliberately: its four-value range exists for
// the `position == boundary` off-by-one below and has no business in the shared
// alphabet.
// `any_query` transitively covers `any_query_item`, which is why the latter is
// not imported: pinning the query generator pins the item generator inside it.
use happenstance_testkit::fixtures::strategies::{any_event_type, any_query, any_tag, any_tags};
use proptest::prelude::*;

// -------------------------------------------------------------------------
// The naive oracles
// -------------------------------------------------------------------------
//
// Everything below this comment and above the `proptest!` block is written for
// a reader, not for a machine: nested linear scans, no early exit worth the
// name, no shared helper with the thing it is checking. That is the whole
// design. `Query::matches` reaches for `binary_search` over a sorted type list
// and `Tags::contains_all`'s single merge-scan over two sorted slices, both of
// which are the kind of code that is wrong only at a boundary; the oracle is a
// transcription of the doc comment on `QueryItem`.
//
// The two must not share a subroutine, or the property degenerates into
// `f(x) == f(x)`. So `naive_item_matches` spells tag containment as
// `wanted.iter().all(|t| held.iter().any(|h| h == t))` rather than calling
// `Tags::contains` — `contains` is itself a `binary_search`, and routing the
// oracle through it would leave the sorted-slice assumption unchecked on both
// sides of the equation.
//
// # Which level is independently written, and which is not
//
// Scoping this is worth a sentence, because `model.rs`'s non-circularity claim
// cites these two properties without qualification and a reader deciding how far
// to trust the model will read that sentence rather than this one.
//
// **Independent:** `naive_item_matches` — a linear `iter().any()` against the
// real `binary_search`, with tag containment spelled in the inverted direction
// against `Tags::contains_all`'s merge-scan. And `naive_is_violated_by` — a
// positive `position > boundary` against the real
// `match self.after { Some(after) if position <= after => false, … }`, routed
// through the naive matcher rather than through `Query::matches`.
//
// **Not independent:** the `Query`-level dispatch in `naive_query_matches` is the
// same expression as the implementation reached through a different accessor —
// real is `Self::Items(items) => items.iter().any(|item| item.matches(…))`,
// oracle is `Some(items) => items.iter().any(|item| naive_item_matches(item, …))`.
// So `query_matches_agrees_with_a_naive_definition` catches item-level defects
// and cannot catch a defect in the OR-over-items dispatch. That is a deliberate
// floor rather than an oversight: the OR has no boundary to get wrong, and the
// `All` arm is covered separately by `query_all_is_the_top_element`. It is
// recorded because "the model is grounded" is true of the item level and only
// partly true one level up.

/// The `QueryItem` doc comment, transcribed: an event matches when its type is
/// one of `types` (empty means any) **and** its tags contain all of `tags`
/// (empty means any).
fn naive_item_matches(item: &QueryItem, event_type: &EventType, tags: &Tags) -> bool {
    let type_ok = item.types().is_empty() || item.types().iter().any(|ty| ty == event_type);
    let tags_ok = item
        .tags()
        .iter()
        .all(|wanted| tags.iter().any(|held| held == wanted));
    type_ok && tags_ok
}

/// `Query::All` is true; `Query::Items` is an OR across the items.
///
/// Written against `Query::items()`, which returns `None` for `All`, so the
/// oracle never touches `Query::matches` or `Query::is_all`.
fn naive_query_matches(query: &Query, event_type: &EventType, tags: &Tags) -> bool {
    match query.items() {
        None => true,
        Some(items) => items
            .iter()
            .any(|item| naive_item_matches(item, event_type, tags)),
    }
}

/// The `AppendCondition` clause, transcribed: an event violates the condition
/// when it lies strictly after the boundary — if there is one — **and** the
/// query matches it.
///
/// `>` is load-bearing and is the entire reason this oracle exists.
/// `AppendCondition::after` is **exclusive** while `ReadOptions::from` is
/// **inclusive**, so the two neighbouring types in the same crate disagree by
/// one on purpose. An implementation that drifts to `>=` here still passes
/// every test that never generates a position equal to the boundary, which is
/// why `any_position` below is drawn from a deliberately tiny range.
fn naive_is_violated_by(
    condition: &AppendCondition,
    position: SequencePosition,
    event_type: &EventType,
    tags: &Tags,
) -> bool {
    let after_the_boundary = match condition.after {
        None => true,
        Some(boundary) => position > boundary,
    };
    after_the_boundary && naive_query_matches(&condition.fail_if_events_match, event_type, tags)
}

/// Positions from a **four-value** range, for the same reason `any_tag`'s
/// alphabet is five symbols: the interesting input is `position == boundary`,
/// and over a realistic range it never occurs.
fn any_position() -> impl Strategy<Value = SequencePosition> {
    (1u64..5).prop_map(|value| SequencePosition::new(value).unwrap())
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

    /// `Query::matches` agrees with a naive definition.
    ///
    /// The properties above constrain `matches` *algebraically* — it is
    /// order-insensitive, adding an item only widens it, `All` is the top
    /// element. Every one of those is satisfied by a function that returns
    /// `true` unconditionally. This is the one that pins the fast
    /// implementation to what the specification actually says, and it is what
    /// makes it honest to build the model-based suite on `Query::matches`:
    /// a model resting on this is not resting on itself.
    #[test]
    fn query_matches_agrees_with_a_naive_definition(
        query in any_query(),
        event_type in any_event_type(),
        event_tags in any_tags(),
    ) {
        prop_assert_eq!(
            query.matches(&event_type, &event_tags),
            naive_query_matches(&query, &event_type, &event_tags),
            "query = {:?}, type = {:?}, tags = {:?}",
            query, event_type, event_tags,
        );
    }

    /// `AppendCondition::is_violated_by` agrees with a naive definition.
    ///
    /// `is_violated_by` had no property at all before this: three hand-written
    /// unit tests in `happenstance-core` and nothing else. It is the function
    /// the whole consistency mechanism reduces to, and the one the model-based
    /// suite's expected-outcome calculation is built on.
    ///
    /// The generated boundary and the generated position are drawn from the
    /// same four values, so `position == boundary` — the exclusive/inclusive
    /// off-by-one — is hit in roughly a quarter of the cases where `after` is
    /// `Some`, rather than never.
    #[test]
    fn is_violated_by_agrees_with_a_naive_definition(
        query in any_query(),
        after in prop::option::of(any_position()),
        position in any_position(),
        event_type in any_event_type(),
        event_tags in any_tags(),
    ) {
        let condition = AppendCondition::new(query).after_opt(after);

        prop_assert_eq!(
            condition.is_violated_by(position, &event_type, &event_tags),
            naive_is_violated_by(&condition, position, &event_type, &event_tags),
            "condition = {:?}, position = {:?}, type = {:?}, tags = {:?}",
            condition, position, event_type, event_tags,
        );
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
