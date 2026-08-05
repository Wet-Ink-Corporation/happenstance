//! The conformance rules themselves.
//!
//! Each rule is an independent async function taking a factory that produces a
//! fresh, empty store. [`event_store_conformance!`](crate::event_store_conformance)
//! wires them into `#[tokio::test]` functions; they are public so an adapter
//! can also call one directly when debugging a single failure.

/// The conformance rules. See the [crate documentation](crate) for the map of
/// what each group covers.
pub mod rules {
    // Every rule panics on failure — that is what a test does, and a `# Panics`
    // section on each of them would say "panics when the adapter is
    // non-conformant" thirty times over.
    #![allow(clippy::missing_panics_doc)]

    use eventum_core::{
        AppendError, EventStore, Query, ReadOptions, SequencePosition, SequencedEvent, collect,
    };

    use crate::fixtures::{
        condition, condition_after, event, event_with_payload, query_of, query_of_types,
        query_tagged, tagged_event, tags,
    };

    // ---------------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------------

    /// Appends and unwraps, failing the test with context on error.
    async fn append_ok<S: EventStore>(
        store: &S,
        events: &[eventum_core::Event],
    ) -> SequencePosition {
        match store.append(events, None).await {
            Ok(position) => position,
            Err(err) => panic!("unconditional append should succeed, got {err:?}"),
        }
    }

    /// Reads and unwraps, failing the test with context on error.
    async fn read_ok<S: EventStore>(
        store: &S,
        query: &Query,
        options: ReadOptions,
    ) -> Vec<SequencedEvent> {
        match collect(store.read(query, options)).await {
            Ok(events) => events,
            Err(err) => panic!("read should succeed, got {err:?}"),
        }
    }

    /// The event types of a read result, in the order returned.
    fn types_of(events: &[SequencedEvent]) -> Vec<&str> {
        events
            .iter()
            .map(|event| event.event_type().as_str())
            .collect()
    }

    /// The positions of a read result, in the order returned.
    fn positions_of(events: &[SequencedEvent]) -> Vec<u64> {
        events.iter().map(|event| event.position.get()).collect()
    }

    // ---------------------------------------------------------------------
    // Query semantics
    // ---------------------------------------------------------------------

    /// `Query::all()` matches every event in the store.
    pub async fn query_all_matches_every_event<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&found),
            ["A", "B", "C"],
            "Query::all() must match every event"
        );
    }

    /// Types within one query item combine with OR.
    pub async fn query_item_types_are_or<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let found = read_ok(&store, &query_of_types(&["A", "C"]), ReadOptions::new()).await;
        assert_eq!(
            types_of(&found),
            ["A", "C"],
            "an event matches a query item when its type is ONE OF the item's types"
        );
    }

    /// Tags within one query item combine with AND.
    pub async fn query_item_tags_are_and<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(
            &store,
            &[
                tagged_event("A", &[("course", "c1")]),
                tagged_event("A", &[("course", "c1"), ("student", "s1")]),
                tagged_event("A", &[("student", "s1")]),
            ],
        )
        .await;

        let found = read_ok(
            &store,
            &query_tagged(&[("course", "c1"), ("student", "s1")]),
            ReadOptions::new(),
        )
        .await;

        assert_eq!(
            positions_of(&found).len(),
            1,
            "an event matches only when it carries ALL of the item's tags"
        );
    }

    /// An event carrying extra tags still matches.
    pub async fn query_item_tags_match_supersets<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(
            &store,
            &[tagged_event(
                "A",
                &[("course", "c1"), ("student", "s1"), ("term", "t1")],
            )],
        )
        .await;

        let found = read_ok(
            &store,
            &query_tagged(&[("course", "c1")]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            found.len(),
            1,
            "the item's tags are a subset requirement, not an exact match"
        );
    }

    /// Overlapping-but-incomplete tags must not match.
    pub async fn query_item_rejects_partial_tag_overlap<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[tagged_event("A", &[("course", "c1")])]).await;

        let found = read_ok(
            &store,
            &query_tagged(&[("course", "c1"), ("student", "s1")]),
            ReadOptions::new(),
        )
        .await;

        assert!(
            found.is_empty(),
            "an event missing any of the item's tags must not match"
        );
    }

    /// Within one item, the type constraint and the tag constraint combine
    /// with AND.
    pub async fn query_item_combines_types_and_tags_with_and<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        append_ok(
            &store,
            &[
                tagged_event("A", &[("course", "c1")]),
                tagged_event("B", &[("course", "c1")]),
                tagged_event("A", &[("course", "c2")]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let found = read_ok(
            &store,
            &query_of(&["A"], &[("course", "c1")]),
            ReadOptions::new(),
        )
        .await;

        // Compared against positions the store actually assigned, never against
        // literal 1/2/3: the specification permits gaps, and an adapter that
        // leaves them is still conformant.
        assert_eq!(
            positions_of(&found),
            positions_of(&all[..1]),
            "an event must satisfy BOTH the type and the tag constraint of an item"
        );
    }

    /// Items within a query combine with OR.
    pub async fn query_items_are_or<S: EventStore, F: Fn() -> S>(factory: F) {
        use eventum_core::QueryItem;

        let store = factory();
        append_ok(
            &store,
            &[
                event("A"),
                tagged_event("Z", &[("student", "s1")]),
                event("Q"),
            ],
        )
        .await;

        let query = Query::from_items([
            QueryItem::of_types(["A"]).expect("valid item"),
            QueryItem::tagged(tags(&[("student", "s1")])).expect("valid item"),
        ])
        .expect("non-empty query");

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let found = read_ok(&store, &query, ReadOptions::new()).await;

        assert_eq!(
            positions_of(&found),
            positions_of(&all[..2]),
            "an event matches the query when it matches ANY of its items"
        );
    }

    /// A query that matches nothing yields an empty stream rather than an error.
    pub async fn query_matching_nothing_yields_empty<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A")]).await;

        let found = read_ok(
            &store,
            &query_of_types(&["Nonexistent"]),
            ReadOptions::new(),
        )
        .await;
        assert!(found.is_empty(), "a query with no matches must not error");
    }

    // ---------------------------------------------------------------------
    // Read options
    // ---------------------------------------------------------------------

    /// Reading is ascending by position unless asked otherwise.
    pub async fn read_defaults_to_ascending_order<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let positions = positions_of(&found);
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "default order must be ascending by sequence position, got {positions:?}"
        );
    }

    /// `ReadOptions::from` includes the event at that position.
    pub async fn read_from_is_inclusive<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let second = all[1].position;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().from(second)).await;
        assert_eq!(
            found.first().map(|event| event.position),
            Some(second),
            "`from` is inclusive: the event at that position must be returned"
        );
        assert_eq!(found.len(), 2, "and everything after it");
    }

    /// `ReadOptions::backwards` reverses the order.
    pub async fn read_backwards_reverses_order<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().backwards()).await;
        assert_eq!(
            types_of(&found),
            ["C", "B", "A"],
            "`backwards` must return events in descending position order"
        );
    }

    /// `ReadOptions::limit` truncates the result.
    pub async fn read_limit_truncates<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B"), event("C"), event("D")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().limit(2)).await;
        assert_eq!(
            types_of(&found),
            ["A", "B"],
            "`limit` must return the first N matches, not an arbitrary N"
        );
    }

    /// The specification's own worked example: N events at or before a
    /// position, newest first.
    pub async fn read_backwards_from_with_limit<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(
            &store,
            &[event("A"), event("B"), event("C"), event("D"), event("E")],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let fourth = all[3].position;

        let found = read_ok(
            &store,
            &Query::all(),
            ReadOptions::new().from(fourth).backwards().limit(2),
        )
        .await;

        assert_eq!(
            types_of(&found),
            ["D", "C"],
            "`from` + `backwards` + `limit` must read N events at or before the position"
        );
    }

    // ---------------------------------------------------------------------
    // Sequence positions
    // ---------------------------------------------------------------------

    /// Positions are unique across the store.
    pub async fn positions_are_unique<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B")]).await;
        append_ok(&store, &[event("C")]).await;

        let positions = positions_of(&read_ok(&store, &Query::all(), ReadOptions::new()).await);
        let mut sorted = positions.clone();
        sorted.sort_unstable();
        sorted.dedup();

        assert_eq!(
            sorted.len(),
            positions.len(),
            "sequence positions must be unique, got {positions:?}"
        );
    }

    /// Positions strictly increase. Gaps are permitted, so this checks ordering
    /// rather than density.
    pub async fn positions_are_strictly_monotonic<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B")]).await;
        append_ok(&store, &[event("C"), event("D")]).await;

        let positions = positions_of(&read_ok(&store, &Query::all(), ReadOptions::new()).await);
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "sequence positions must increase monotonically (gaps are fine), got {positions:?}"
        );
    }

    // ---------------------------------------------------------------------
    // Append
    // ---------------------------------------------------------------------

    /// `append` returns the position of the last event written.
    pub async fn append_returns_last_written_position<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        let returned = append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            Some(returned),
            all.last().map(|event| event.position),
            "`append` must return the position assigned to the LAST event in the batch"
        );
    }

    /// A batch lands entirely or not at all.
    pub async fn append_is_atomic<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("Existing")]).await;

        // This batch must be rejected: the condition already matches.
        let condition = condition(query_of_types(&["Existing"]));
        let result = store
            .append(&[event("X"), event("Y"), event("Z")], Some(&condition))
            .await;

        assert!(result.is_err(), "the append should have been rejected");

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&all),
            ["Existing"],
            "a rejected batch must leave NO partial writes behind"
        );
    }

    /// An empty batch is refused.
    pub async fn append_rejects_empty_batch<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        let result = store.append(&[], None).await;

        assert!(
            matches!(result, Err(AppendError::NoEvents)),
            "the specification defines a batch as non-empty; expected AppendError::NoEvents"
        );
    }

    /// Payload, tags and metadata survive the round trip unchanged.
    pub async fn append_preserves_event_payload<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        let original = event_with_payload("A", b"{\"answer\":42}")
            .with_tags(tags(&[("course", "c1")]))
            .with_metadata(&b"trace-id"[..]);

        append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1);
        assert_eq!(
            all[0].event, original,
            "an event must round-trip byte-for-byte, including tags and metadata"
        );
    }

    // ---------------------------------------------------------------------
    // Append conditions
    // ---------------------------------------------------------------------

    /// Without `after`, any matching event rejects the append.
    pub async fn condition_without_after_rejects_any_match<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        append_ok(&store, &[event("Blocker")]).await;

        let result = store
            .append(
                &[event("New")],
                Some(&condition(query_of_types(&["Blocker"]))),
            )
            .await;

        assert!(
            matches!(result, Err(AppendError::ConditionViolated(_))),
            "with no `after`, ANY matching event must reject the append"
        );
    }

    /// A condition that matches nothing lets the append through.
    pub async fn condition_without_after_allows_non_match<S: EventStore, F: Fn() -> S>(factory: F) {
        let store = factory();
        append_ok(&store, &[event("Unrelated")]).await;

        let result = store
            .append(
                &[event("New")],
                Some(&condition(query_of_types(&["Blocker"]))),
            )
            .await;

        assert!(
            result.is_ok(),
            "a condition matching no event must not reject the append, got {result:?}"
        );
    }

    /// `after` is exclusive: an event exactly at the boundary was already seen
    /// by the caller and must not reject the append.
    pub async fn condition_after_ignores_events_at_the_boundary<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        let boundary = append_ok(&store, &[event("Blocker")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            result.is_ok(),
            "`after` is exclusive: the event AT that position was already \
             accounted for and must not reject the append, got {result:?}"
        );
    }

    /// A matching event beyond the boundary rejects the append.
    pub async fn condition_after_rejects_events_beyond_the_boundary<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        let boundary = append_ok(&store, &[event("Seen")]).await;
        append_ok(&store, &[event("Blocker")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            matches!(result, Err(AppendError::ConditionViolated(_))),
            "a matching event AFTER the boundary is one the caller never saw \
             and must reject the append"
        );
    }

    /// Non-matching events beyond the boundary are irrelevant.
    pub async fn condition_after_ignores_non_matching_events<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        let boundary = append_ok(&store, &[event("Seen")]).await;
        append_ok(&store, &[event("Irrelevant")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            result.is_ok(),
            "only events matching the condition's query may reject an append, got {result:?}"
        );
    }

    /// A rejected append changes nothing observable.
    pub async fn condition_rejection_leaves_store_unchanged<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        append_ok(&store, &[event("A"), event("B")]).await;
        let before = read_ok(&store, &Query::all(), ReadOptions::new()).await;

        let _ = store
            .append(&[event("C")], Some(&condition(query_of_types(&["A"]))))
            .await;

        let after = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            before, after,
            "a rejected append must leave the store byte-identical"
        );
    }

    /// The rejection is reported as the specification's concurrency signal, not
    /// as an adapter-specific failure.
    pub async fn condition_rejection_is_reported_as_condition_violated<
        S: EventStore,
        F: Fn() -> S,
    >(
        factory: F,
    ) {
        let store = factory();
        append_ok(&store, &[event("Blocker")]).await;

        let result = store
            .append(
                &[event("New")],
                Some(&condition(query_of_types(&["Blocker"]))),
            )
            .await;

        match result {
            Err(err) => assert!(
                err.is_condition_violated(),
                "a condition violation must surface as \
                 AppendError::ConditionViolated, not AppendError::Store — \
                 callers distinguish `retry` from `something broke` on this \
                 alone. Got {err:?}"
            ),
            Ok(position) => panic!("the append should have been rejected, got {position:?}"),
        }
    }

    // ---------------------------------------------------------------------
    // Concurrency
    // ---------------------------------------------------------------------

    /// Two command handlers that decided from the same snapshot cannot both
    /// commit.
    ///
    /// This is the race DCB exists to prevent, expressed deterministically:
    /// both handlers read, both see position N, both append conditioned on
    /// `after: N` with the same query. Whichever lands first invalidates the
    /// other.
    ///
    /// It is written sequentially on purpose. A genuinely parallel version
    /// would need `Send + Sync + 'static` bounds that
    /// [`EventStore`] does not carry — the whole
    /// point of that flavour — and would risk a flaky conformance suite, which
    /// is worse than none. Adapters that are `Send + Sync` should add a
    /// multi-threaded stress test of their own; this rule fixes the semantics
    /// they would be stress-testing.
    pub async fn racing_conditional_appends_elect_one_winner<S: EventStore, F: Fn() -> S>(
        factory: F,
    ) {
        let store = factory();
        let boundary = append_ok(&store, &[event("CourseDefined")]).await;

        // Both handlers built the same decision model from the same snapshot.
        let query = query_of(&["StudentSubscribed"], &[("course", "c1")]);
        let handler_a = condition_after(query.clone(), boundary.get());
        let handler_b = condition_after(query, boundary.get());

        let subscribe = tagged_event("StudentSubscribed", &[("course", "c1")]);

        let first = store
            .append(core::slice::from_ref(&subscribe), Some(&handler_a))
            .await;
        assert!(
            first.is_ok(),
            "the first handler to commit must succeed, got {first:?}"
        );

        let second = store
            .append(core::slice::from_ref(&subscribe), Some(&handler_b))
            .await;
        assert!(
            matches!(second, Err(AppendError::ConditionViolated(_))),
            "the second handler decided from a snapshot that is now stale and \
             MUST be rejected — otherwise the consistency boundary is not \
             enforced at all. Got {second:?}"
        );

        let all = read_ok(
            &store,
            &query_of_types(&["StudentSubscribed"]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            all.len(),
            1,
            "exactly one of the two racing appends may land"
        );
    }
}
