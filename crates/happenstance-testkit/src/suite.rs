//! The conformance rules themselves.
//!
//! Each rule is an independent async function taking `impl AsyncFn() -> F`: not
//! a fixture, but **how to make one**. A rule that can call `open()` twice can
//! check that two fixture instances share nothing, which is an *adapter's*
//! mistake and therefore has to be a rule the adapter runs rather than a
//! meta-test over the testkit's own fixture. See [`Fixture`](crate::Fixture) for
//! the contract, and [`RuleOutcome`](crate::RuleOutcome) for why a rule that
//! cannot run still reports.
//!
//! [`event_store_conformance!`](crate::event_store_conformance) wires them into
//! per-rule test functions; they are public so an adapter can also call one
//! directly when debugging a single failure.

/// Returns [`RuleOutcome::Skipped`](crate::RuleOutcome::Skipped) unless the
/// fixture supports the named capability.
///
/// This is where the two halves of the skip contract are reconciled, and they
/// pull in opposite directions:
///
/// * *the rule is still **emitted** as a test* — so the emitters must have no
///   branch at all. A `macro_rules!` macro matches tokens and cannot read a
///   `const`'s value, so the only branch an emitter could take is `#[cfg]`,
///   which is exactly the arrangement the contract rejects: a rule absent from
///   the test binary is indistinguishable in CI output from one that passed.
/// * *the skip is **reported*** — so the branch lives here, in the rule body,
///   and the emitter simply calls `.report(…)` on whatever comes back.
///
/// The branch is on an associated `const`, so after monomorphisation it is a
/// constant and the dead half is discarded. The test still exists; it just does
/// nothing but say so.
///
/// Use it only when the rule's **entire** content needs the capability, and only
/// for a capability a fixture may honestly decline. A rule that merely
/// strengthens itself where a second handle exists has run, and must say `Ran`;
/// a capability that is a MUST uses `must!` instead.
macro_rules! require {
    ($fixture:ident : $capability:ident) => {
        if let Some(reason) = <$fixture as $crate::Fixture>::$capability.reason() {
            return $crate::RuleOutcome::Skipped {
                capability: ::core::stringify!($capability),
                reason,
            };
        }
    };
}

/// Panics unless the fixture supports the named capability.
///
/// `require!`'s sibling, and the difference between them is the whole reason
/// this one exists. A capability a fixture may honestly decline is a **trade**,
/// and the suite records it as a skip. `SECOND_HANDLE` is not one: CF-16 makes a
/// second handle onto one backing store an obligation on *every* fixture, and
/// the type cannot tell the two apart — both spell `Capability`.
///
/// # Why the check lives here and not only in a meta-test
///
/// The testkit's own `capability_skips_are_reported` reads the capabilities of
/// the fixtures registered in *its* `tests/`, and it never runs in an adapter's
/// CI — which is exactly where the fixture that declines a MUST lives. An
/// adapter author who meets a red `two_handles_observe_each_others_appends`,
/// writes `const SECOND_HANDLE: Capability = Capability::declined("…")` and
/// re-runs would otherwise get a green suite and one `SKIP` line. This macro is
/// what turns that into a red one, on the path an adapter actually executes.
///
/// The panic carries the fixture's own stated reason, so nothing is lost by
/// failing rather than skipping: the trade is still in the log, it is simply in
/// the log of a build that did not pass.
macro_rules! must {
    ($fixture:ident : $capability:ident) => {
        if let Some(reason) = <$fixture as $crate::Fixture>::$capability.reason() {
            panic!(
                "this fixture declines `{}`, which is a MUST and not a trade the \
                 suite can record as a skip. SPECIFICATION.md CF-16 requires \
                 every fixture to open a second, independent handle onto one \
                 backing store; declining it would buy a green suite for an \
                 adapter nothing had reached through two connections at all. \
                 Hold the backing store behind an `Arc` or an `Rc` and return a \
                 fresh handle from each `connect` — `fixtures::MemoryFixture` is \
                 the reference implementation. Reason given: {reason}",
                ::core::stringify!($capability),
            );
        }
    };
}

/// The conformance rules. See the [crate documentation](crate) for the map of
/// what each group covers.
pub mod rules {
    // Every rule panics on failure — that is what a test does, and a `# Panics`
    // section on each of them would say "panics when the adapter is
    // non-conformant" thirty times over.
    #![allow(clippy::missing_panics_doc)]

    use core::future::Future;
    use core::pin::{Pin, pin};
    use core::task::Poll;

    use happenstance_core::{
        AppendError, Event, EventStore, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN, Query, ReadOptions,
        SequencePosition, SequencedEvent, collect,
    };

    use crate::fixtures::{
        condition, condition_after, event, event_with_owned_payload, event_with_payload,
        item_of_types, item_tagged, query_of, query_of_items, query_of_types, query_tagged,
        tagged_event, tags,
    };
    use crate::{Fixture, RuleOutcome};

    // ---------------------------------------------------------------------
    // The four guaranteed minima (VT-21 – VT-24)
    //
    // Imported rather than written out. Phase 3 spelled the numbers here because
    // the constants did not exist and a rule that read a missing constant would
    // have asserted whatever it happened to say, including nothing; phase 4
    // created them, so the clause and the code now agree by construction and
    // raising a floor takes its rules with it.
    // ---------------------------------------------------------------------
    use happenstance_core::{
        MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_QUERY_ITEMS,
        MIN_SUPPORTED_TAGS_PER_EVENT,
    };

    // ---------------------------------------------------------------------
    // Helpers
    //
    // Bounded on `EventStore` rather than on `Fixture`: they operate on a
    // handle, and a handle is a store. They were unchanged by the move to the
    // fixture contract, which is the sign the seam is in the right place.
    // ---------------------------------------------------------------------

    /// Appends and unwraps, failing the test with context on error.
    async fn append_ok<S: EventStore>(store: &S, events: &[Event]) -> SequencePosition {
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

    /// A read result as `(position, event)` pairs, for comparing a store against
    /// itself.
    ///
    /// "Byte-identical" is ES-18's word and this is as close as the port gets to
    /// it: a position and the whole `Event` behind it, compared structurally.
    /// `SequencedEvent` is not itself compared because a future field — VT-9's
    /// store-assigned time is the one on the way — would make two reads of an
    /// unchanged store differ.
    fn snapshot_of(events: &[SequencedEvent]) -> Vec<(u64, &Event)> {
        events
            .iter()
            .map(|event| (event.position.get(), &event.event))
            .collect()
    }

    // ---------------------------------------------------------------------
    // The fixture contract
    // ---------------------------------------------------------------------

    /// Two fixture instances are two backing stores.
    ///
    /// The mistake this rejects is an adapter's, not the testkit's: a
    /// file-backed fixture that points every instance at one temporary path.
    /// Under the old `Fn() -> S` factory that produced cross-test contamination
    /// surfacing as some *unrelated* rule failing intermittently. Here it fails
    /// one named rule, every time.
    ///
    /// The read-back through the writing instance is not padding. Without it the
    /// rule is satisfied by an `append` that silently did nothing, which is a
    /// strictly easier way to pass it than isolation.
    pub async fn two_fixture_instances_observe_none_of_each_others_appends<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let first = open().await;
        let second = open().await;

        let writer = first.connect().await;
        let observer = second.connect().await;

        append_ok(&writer, &[event("A")]).await;

        let written = read_ok(&writer, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&written),
            ["A"],
            "the append must actually have landed in the first fixture's store — \
             otherwise the isolation assertion below holds for the wrong reason"
        );

        let elsewhere = read_ok(&observer, &Query::all(), ReadOptions::new()).await;
        assert!(
            elsewhere.is_empty(),
            "two fixture instances must share no backing store, but the second \
             observed {} event(s) appended through the first",
            elsewhere.len()
        );

        RuleOutcome::Ran
    }

    /// Two handles onto one backing store see each other's writes, on both the
    /// read side and the append-condition side.
    ///
    /// This is the rule `racing_conditional_appends_elect_one_winner` looks like
    /// and is not: that one is sequential *and* single-handle, so it pins the
    /// semantics of a race without ever crossing a connection.
    ///
    /// It rejects three adapters that pass every other rule in this suite: a
    /// cached `max(position)` fast path, a per-connection repeatable-read
    /// snapshot, and an advisory lock scoped to one pool member. All three are
    /// per-session correctness, and all three are strategies the decision ledger
    /// defers rather than rules out.
    ///
    /// The condition is **tagged**, not type-only, on purpose: the type-only
    /// probe returns an identical verdict here while being the join an adapter
    /// drops first, so it would test strictly less for no saving.
    ///
    /// The first of those three is written down, compiled and registered:
    /// `CachedHeadFixture` in `tests/mutation_coverage/mutants.rs` reads
    /// correctly through a second handle and evaluates its condition against a
    /// cache its own appends refresh, which is what makes the rule's second
    /// assertion something an implementation can fail rather than something no
    /// implementation would attempt. Two more mutants fail this rule for
    /// unrelated reasons — `AfterDefaultsToFirstStore` and
    /// `ViolationAsStoreErrorStore` — and the registry names all three.
    pub async fn two_handles_observe_each_others_appends<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`, not `require!`: CF-16 makes a second handle an obligation on
        // every fixture, so a decline here is a fixture that does not meet the
        // contract rather than a trade the suite may record and move past.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let reader = fixture.connect().await;

        let subscribed = tagged_event("StudentSubscribed", &[("course", "c1")]);
        let query = query_of(&["StudentSubscribed"], &[("course", "c1")]);

        let written = append_ok(&writer, core::slice::from_ref(&subscribed)).await;

        // The read side.
        let seen = read_ok(&reader, &query, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&seen),
            [written.get()],
            "an event appended through one handle must be readable through a \
             second handle onto the same store"
        );

        // The append side. A condition evaluated through the second handle must
        // observe the first handle's committed append — this is the half a
        // cached position or a session snapshot gets wrong while still reading
        // correctly.
        let result = reader
            .append(&[event("StudentSubscribed")], Some(&condition(query)))
            .await;
        assert!(
            matches!(result, Err(AppendError::ConditionViolated(_))),
            "an append condition evaluated through a second handle must see the \
             first handle's committed append, got {result:?}"
        );

        RuleOutcome::Ran
    }

    /// An append acknowledged with `Ok` is still there after a reopen.
    ///
    /// This rejects an adapter that acknowledges before its `COMMIT` returns: a
    /// pooled store doing its work in `spawn_blocking` and answering on the
    /// join, a Durable Object relying on output-gate semantics it does not
    /// actually have, any connection setup carrying `PRAGMA synchronous = OFF`.
    ///
    /// **This rule is formally CF-14, which is deferred to phase 8, and landing
    /// it now is a named exception.** Its *adapter* far end — a real store that
    /// can lose a write under a real fault — is phase 8's to build. It is here
    /// because without it the fixture contract ships with exactly one
    /// capability-gated rule and no second one to observe, leaving `REOPEN`'s
    /// provisional marker and the skip machinery both untested.
    ///
    /// Its **fixture** far end is not deferred and is in the tree:
    /// `DurableFixture` in `tests/fixture_instruments.rs` replays a durable log
    /// into a fresh store on `reopen`, so this body executes on every gate run
    /// rather than reporting a skip under all three shipped harnesses — and
    /// `LosingFixture` in `tests/mutation_coverage/mutants.rs`, which never
    /// records at all, is the registered mutant that shows it can fail.
    /// `MemoryFixture` and `LocalFixture` both decline `REOPEN` for honest
    /// reasons and still report the skip, which is what CF-18 is for.
    ///
    /// The handle is dropped before the reopen on purpose: a handle that carried
    /// the data across would make the rule pass for the wrong reason.
    pub async fn acknowledged_writes_survive_a_reopen<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: REOPEN);

        let fixture = open().await;

        let acknowledged = {
            let store = fixture.connect().await;
            append_ok(&store, &[event("A"), event("B")]).await
        };

        fixture.reopen().await;

        let store = fixture.connect().await;
        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&all),
            ["A", "B"],
            "every event of an acknowledged append must survive a reopen"
        );
        assert_eq!(
            all.last().map(|event| event.position),
            Some(acknowledged),
            "and the position `append` returned must still name the last of them"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Query semantics
    // ---------------------------------------------------------------------

    /// `Query::all()` matches every event in the store.
    ///
    /// The types are appended in **descending alphabetical order**, which costs
    /// four characters and is the difference between a rule and a decoration.
    /// With `"A", "B", "C"` the expected result is simultaneously insertion
    /// order and type order, so `SortByEventTypeStore` — `ORDER BY type,
    /// position`, which is what a covering index on `(type, position)` gives you
    /// — passes it. That mutant is in the registry, and this ordering is what it
    /// fails.
    pub async fn query_all_matches_every_event<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Cee"), event("Bee"), event("Ay")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&found),
            ["Cee", "Bee", "Ay"],
            "Query::all() must match every event, in the order the store assigned"
        );

        RuleOutcome::Ran
    }

    /// Types within one query item combine with OR.
    pub async fn query_item_types_are_or<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let found = read_ok(&store, &query_of_types(&["A", "C"]), ReadOptions::new()).await;
        assert_eq!(
            types_of(&found),
            ["A", "C"],
            "an event matches a query item when its type is ONE OF the item's types"
        );

        RuleOutcome::Ran
    }

    /// Tags within one query item combine with AND.
    pub async fn query_item_tags_are_and<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// An event carrying extra tags still matches.
    pub async fn query_item_tags_match_supersets<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// Overlapping-but-incomplete tags must not match.
    pub async fn query_item_rejects_partial_tag_overlap<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// Within one item, the type constraint and the tag constraint combine
    /// with AND.
    pub async fn query_item_combines_types_and_tags_with_and<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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
        // The anchor has to exist before it can be compared against. Without
        // this, a store that returns too few events fails the rule with `range
        // end index 1 out of range` — a panic that says nothing about the
        // property, and that `mutation_coverage`'s `FailureMode::Assertion`
        // rejects as the wrong reason for the right rule.
        //
        // A *count*, deliberately, not `types_of`. This rule is about the AND
        // inside an item and nothing else; asserting the order here would make
        // it fail every store whose defect is ordering, which is what keeps a
        // mutant a scalpel rather than a shotgun.
        assert_eq!(
            all.len(),
            3,
            "all three appended events must be readable back before one of \
             their positions can anchor the assertion below"
        );

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

        RuleOutcome::Ran
    }

    /// Items within a query combine with OR.
    ///
    /// The first event matches **both** items, and that is deliberate. An OR
    /// implemented as a `UNION ALL` of one statement per item returns it twice,
    /// and with three mutually exclusive events no result-set comparison here
    /// could ever see that. The rule that owns duplication is
    /// `duplicate_items_do_not_duplicate_events`; this arrangement is what lets
    /// it be written against the same fixture rather than needing its own.
    pub async fn query_items_are_or<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        use happenstance_core::QueryItem;

        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[
                tagged_event("A", &[("student", "s1")]),
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
        assert_eq!(
            all.len(),
            3,
            "all three appended events must be readable back before the first \
             two of their positions can anchor the assertion below"
        );

        let found = read_ok(&store, &query, ReadOptions::new()).await;

        assert_eq!(
            positions_of(&found),
            positions_of(&all[..2]),
            "an event matches the query when it matches ANY of its items"
        );

        RuleOutcome::Ran
    }

    /// An event carrying **no tags at all** is still yielded by `Query::all()`.
    ///
    /// ES-15, and the half `duplicate_items_do_not_duplicate_events` cannot
    /// carry: that rule needs a multi-tagged event, and a store whose tags live
    /// in a side table joined with `INNER JOIN` has no row to join an *untagged*
    /// event to, so it drops it from the query every projection runner starts
    /// from. That is silent data loss, and `InnerJoinTagStore` is it.
    ///
    /// `query_all_matches_every_event` used to be the only thing in the suite
    /// that would have noticed, by accident of appending three untagged events.
    /// This rule is that accident made deliberate, so that strengthening the
    /// other one cannot quietly narrow coverage.
    pub async fn untagged_events_match_query_all<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let plain = append_ok(&store, &[event("Plain")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            positions_of(&all),
            [plain.get()],
            "an event with no tags at all must still be matched by Query::all() \
             — a tag side table reached with INNER JOIN has no row to join it to"
        );

        RuleOutcome::Ran
    }

    /// An event matching several items, or carrying several tags, is yielded
    /// **once**.
    ///
    /// ES-15 and CF-9. The store holds one event carrying three tags, and the
    /// rule reads it twice: through `Query::all()`, where a row-per-tag side
    /// table joined without `DISTINCT` fans it out into three rows, and through
    /// a two-item query *both* of whose items match it, where an OR implemented
    /// as one statement per item unioned with `UNION ALL` returns it twice.
    ///
    /// The `Query::all()` half is the one nothing else in the suite could reach:
    /// every other rule that reads `Query::all()` does so over untagged events,
    /// where a fan-out join produces exactly one row per event and the defect is
    /// invisible. `TagJoinFanOutStore` is the compiled version.
    pub async fn duplicate_items_do_not_duplicate_events<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let multi = append_ok(
            &store,
            &[tagged_event(
                "Enrolled",
                &[("course", "c1"), ("student", "s1"), ("term", "t1")],
            )],
        )
        .await;
        let plain = append_ok(&store, &[event("Noted")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.iter().filter(|e| e.position == multi).count(),
            1,
            "an event carrying three tags must be yielded once, not once per \
             tag; Query::all() returned {:?}",
            positions_of(&all)
        );
        // A cardinality claim, deliberately not an order claim: this rule owns
        // duplication and `read_defaults_to_ascending_order` owns order. The
        // second event is what keeps the count above from being satisfied by a
        // store that returns nothing.
        assert_eq!(
            all.len(),
            2,
            "and the store holds exactly two events, so Query::all() must yield \
             two: {:?}",
            positions_of(&all)
        );
        assert!(
            all.iter().any(|e| e.position == plain),
            "including the untagged one"
        );

        // Both items match the multi-tagged event, which is what makes a
        // per-item `UNION ALL` visible at all.
        let overlapping = query_of_items([
            item_of_types(&["Enrolled"]),
            item_tagged(&[("course", "c1")]),
        ]);
        let found = read_ok(&store, &overlapping, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&found),
            [multi.get()],
            "an event matching two items of one query must be yielded once"
        );

        RuleOutcome::Ran
    }

    /// The order of a query's items does not change what it selects.
    ///
    /// ES-15 and VT-31. The two items are disjoint and each selects the event
    /// the other does not, so a store that evaluates one statement per item and
    /// concatenates the results client-side returns them in *item* order rather
    /// than in position order — and the two reads then disagree.
    ///
    /// The clause's own `Rejects:` names an adapter that sorts and deduplicates
    /// a query's *items* as an optimisation. That shape is real and it is not
    /// what this rule catches, which is worth saying so nobody writes the mutant
    /// for it: sorting the items is precisely what makes their order stop
    /// mattering, so a store that sorts them passes an order-invariance rule by
    /// construction. What can fail this rule is a store whose *output* order is
    /// the item order, and `ItemOrderedUnionStore` is that store.
    ///
    /// The length assertion is the non-vacuity anchor. Without it two empty
    /// reads agree, and a store that selected nothing would pass a rule about
    /// what selection depends on.
    pub async fn query_item_order_does_not_change_the_result_set<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // The event matching the *second* item is appended first, so item order
        // and position order disagree.
        let by_tag = append_ok(&store, &[tagged_event("Early", &[("k", "v")])]).await;
        let by_type = append_ok(&store, &[event("Later")]).await;

        let types_first = query_of_items([item_of_types(&["Later"]), item_tagged(&[("k", "v")])]);
        let tags_first = query_of_items([item_tagged(&[("k", "v")]), item_of_types(&["Later"])]);

        let first = read_ok(&store, &types_first, ReadOptions::new()).await;
        let second = read_ok(&store, &tags_first, ReadOptions::new()).await;

        assert_eq!(
            positions_of(&first),
            positions_of(&second),
            "the same items in a different order must select the same events in \
             the same order"
        );
        assert_eq!(
            positions_of(&first),
            [by_tag.get(), by_type.get()],
            "and both events must be selected, in position order — otherwise the \
             two reads agree by both being empty, or by both being in item order"
        );

        RuleOutcome::Ran
    }

    /// A query that matches nothing yields an empty stream rather than an error.
    pub async fn query_matching_nothing_yields_empty<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("A")]).await;

        let found = read_ok(
            &store,
            &query_of_types(&["Nonexistent"]),
            ReadOptions::new(),
        )
        .await;
        assert!(found.is_empty(), "a query with no matches must not error");

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Read options
    // ---------------------------------------------------------------------

    /// Reading is ascending by position unless asked otherwise.
    ///
    /// Descending types for `query_all_matches_every_event`'s reason: sorted by
    /// type, these three come back with descending positions. The length
    /// assertion is not padding either — `windows(2)` over an empty slice is
    /// vacuously true, so a store that returned nothing would otherwise pass a
    /// rule about ordering.
    pub async fn read_defaults_to_ascending_order<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Cee"), event("Bee"), event("Ay")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let positions = positions_of(&found);
        assert_eq!(
            positions.len(),
            3,
            "all three appended events must come back before their order means \
             anything, got {positions:?}"
        );
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "default order must be ascending by sequence position, got {positions:?}"
        );

        RuleOutcome::Ran
    }

    /// `ReadOptions::from` includes the event at that position.
    pub async fn read_from_is_inclusive<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            3,
            "all three appended events must be readable back before the second \
             of their positions can anchor the assertion below"
        );
        let second = all[1].position;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().from(second)).await;
        assert_eq!(
            found.first().map(|event| event.position),
            Some(second),
            "`from` is inclusive: the event at that position must be returned"
        );
        assert_eq!(found.len(), 2, "and everything after it");

        RuleOutcome::Ran
    }

    /// `ReadOptions::backwards` reverses the order.
    ///
    /// Descending types, so that the expected answer is ascending by type and a
    /// store that sorted by type could not produce it by accident.
    pub async fn read_backwards_reverses_order<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Cee"), event("Bee"), event("Ay")]).await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().backwards()).await;
        assert_eq!(
            types_of(&found),
            ["Ay", "Bee", "Cee"],
            "`backwards` must return events in descending position order"
        );

        RuleOutcome::Ran
    }

    /// `ReadOptions::limit` truncates the result.
    ///
    /// Descending types again, and for a second reason here: the truncation has
    /// to be the *first N by position*, so a store ordering by anything else
    /// returns a different two events rather than the same two in a different
    /// order.
    pub async fn read_limit_truncates<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[event("Dee"), event("Cee"), event("Bee"), event("Ay")],
        )
        .await;

        let found = read_ok(&store, &Query::all(), ReadOptions::new().limit(2)).await;
        assert_eq!(
            types_of(&found),
            ["Dee", "Cee"],
            "`limit` must return the first N matches, not an arbitrary N"
        );

        RuleOutcome::Ran
    }

    /// The specification's own worked example: N events at or before a
    /// position, newest first.
    pub async fn read_backwards_from_with_limit<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[event("A"), event("B"), event("C"), event("D"), event("E")],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            5,
            "all five appended events must be readable back before the fourth \
             of their positions can anchor the assertion below"
        );
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

        RuleOutcome::Ran
    }

    /// A store nobody has appended to reads as empty, and does not error.
    ///
    /// ES-9's degenerate end. Every other rule in the suite seeds the store
    /// before it reads, so the state **every adapter is in on its first run** is
    /// the one state nothing exercised. The read options are exercised too,
    /// because the arithmetic that breaks here is a paging window's: ES-11
    /// prescribes anchoring the window on the store's head at the first poll, and
    /// on an empty store there is no head. `NullHeadPagingStore` is that adapter,
    /// decoding `max(position)` — which is `NULL` over no rows — into a column
    /// type that cannot hold it.
    pub async fn reading_an_empty_store_yields_nothing<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert!(
            all.is_empty(),
            "a store that has never been appended to must yield nothing, got {:?}",
            positions_of(&all)
        );

        // The same question through the paging path, because that is where the
        // missing head is spent rather than merely observed.
        let paged = read_ok(
            &store,
            &query_of_types(&["Anything"]),
            ReadOptions::new().backwards().limit(1),
        )
        .await;
        assert!(
            paged.is_empty(),
            "and it must yield nothing under read options too, without erroring"
        );

        RuleOutcome::Ran
    }

    /// `limit` counts **matches**, not scanned rows.
    ///
    /// **ES-14 owns it** — it is the clause whose `Rule:` line names this rule
    /// and its mirror. **CF-12 is the obligation it discharges**: run *some*
    /// read option against a filtering query at all. Until this rule landed
    /// every read-option rule in the suite issued `Query::all()`, where the
    /// scanned set and the matched set are the same set and the ordering of the
    /// two operations cannot be observed. (Not VT-28, which is the `limit`
    /// *field* and whose own rules are `read_limit_zero_yields_nothing` and
    /// `read_limit_truncates`.)
    ///
    /// The layout is `Miss, Hit, Hit, Hit, Miss` so that a store pushing `LIMIT`
    /// down into the scan and filtering the rows that come back returns *fewer*
    /// than the limit — one match here — from either end. `LimitBeforeFilterStore`
    /// is that store, and a reviewer measured the shape passing the suite as it
    /// stood.
    ///
    /// Positions come from the appends themselves rather than from a `Query::all()`
    /// read, which is not fussiness: the anchor must not depend on the very read
    /// path the rule is testing, or a store that orders its output wrongly fails
    /// this rule for that reason instead.
    pub async fn read_limit_applies_after_filtering<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let (first, second, _third) = seed_hits_between_misses(&store).await;

        let found = read_ok(
            &store,
            &query_of_types(&["Hit"]),
            ReadOptions::new().limit(2),
        )
        .await;

        assert_eq!(
            positions_of(&found),
            [first.get(), second.get()],
            "`limit(2)` must return the first two events MATCHING the query, not \
             whatever matches among the first two rows scanned"
        );

        RuleOutcome::Ran
    }

    /// The same claim from the other end, because a store truncates the scan at
    /// whichever end it reads from.
    ///
    /// The mirror is not decoration: `LIMIT` pushed into a descending scan takes
    /// the *highest* rows, so a layout that puts a non-matching event only at the
    /// low end would let the backwards read pass while the forwards read failed.
    pub async fn read_backwards_limit_applies_after_filtering<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let (_first, second, third) = seed_hits_between_misses(&store).await;

        let found = read_ok(
            &store,
            &query_of_types(&["Hit"]),
            ReadOptions::new().backwards().limit(2),
        )
        .await;

        assert_eq!(
            positions_of(&found),
            [third.get(), second.get()],
            "under `backwards`, `limit(2)` must return the last two events \
             MATCHING the query, newest first"
        );

        RuleOutcome::Ran
    }

    /// Every read option composes with a **multi-item** query.
    ///
    /// CF-12. `from` commutes with filtering semantically and not in generated
    /// SQL: `WHERE a OR b AND position >= ?` binds the cursor to the last item
    /// alone, so item `a` is returned in full regardless of where the caller
    /// resumed — and a projection re-delivers already-checkpointed events
    /// forever, with no error anywhere.
    ///
    /// `backwards` and `limit` are exercised here too, and in the same rule
    /// rather than in three, because CF-12's measured finding is that **no read
    /// option was exercised against a filtering query at all**: the gap is one
    /// gap, and splitting it into three rules would suggest three.
    pub async fn read_from_composes_with_multi_item_query<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Matched by the type item, the tag item, the type item, nothing, and
        // the tag item — so neither item's matches form a contiguous run.
        let by_type_low = append_ok(&store, &[event("Alpha")]).await;
        let by_tag_low = append_ok(&store, &[tagged_event("Beta", &[("topic", "t1")])]).await;
        let anchor = append_ok(&store, &[event("Alpha")]).await;
        append_ok(&store, &[event("Gamma")]).await;
        let by_tag_high = append_ok(&store, &[tagged_event("Beta", &[("topic", "t1")])]).await;

        let query = query_of_items([item_of_types(&["Alpha"]), item_tagged(&[("topic", "t1")])]);

        let resumed = read_ok(&store, &query, ReadOptions::new().from(anchor)).await;
        assert_eq!(
            positions_of(&resumed),
            [anchor.get(), by_tag_high.get()],
            "`from` must bound EVERY item of the query. An unparenthesised \
             `WHERE a OR b AND position >= ?` bounds only the last one, and the \
             first item's matches come back from the start of the log every time"
        );

        let reversed = read_ok(&store, &query, ReadOptions::new().backwards()).await;
        assert_eq!(
            positions_of(&reversed),
            [
                by_tag_high.get(),
                anchor.get(),
                by_tag_low.get(),
                by_type_low.get()
            ],
            "`backwards` must order the whole match set, not each item's matches"
        );

        let newest = read_ok(&store, &query, ReadOptions::new().backwards().limit(2)).await;
        assert_eq!(
            positions_of(&newest),
            [by_tag_high.get(), anchor.get()],
            "and `limit` must truncate the merged, ordered result"
        );

        RuleOutcome::Ran
    }

    /// `Miss, Hit, Hit, Hit, Miss`, returning the three hits' positions.
    ///
    /// Shared by the two `limit`-after-filtering rules so that the layout — a
    /// non-matching event at *each* end — is stated once. One event per append,
    /// so every position returned is one the store assigned to a known event
    /// (CF-6) without reading anything back.
    async fn seed_hits_between_misses<S: EventStore>(
        store: &S,
    ) -> (SequencePosition, SequencePosition, SequencePosition) {
        append_ok(store, &[event("Miss")]).await;
        let first = append_ok(store, &[event("Hit")]).await;
        let second = append_ok(store, &[event("Hit")]).await;
        let third = append_ok(store, &[event("Hit")]).await;
        append_ok(store, &[event("Miss")]).await;
        (first, second, third)
    }

    // ---------------------------------------------------------------------
    // Sequence positions
    // ---------------------------------------------------------------------

    /// Positions are unique across the store.
    pub async fn positions_are_unique<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// Positions strictly increase. Gaps are permitted, so this checks ordering
    /// rather than density.
    ///
    /// The types descend across both batches, which is what makes this
    /// falsifiable at all. CF-1's own `Rejects:` names this rule and
    /// `positions_are_unique` as decorative *as they stood*: both read a
    /// quiescent store back through `read`, so both were satisfied by any store
    /// that sorted on the way out. Under descending types a store ordering by
    /// `(type, position)` returns descending positions, and this fails.
    /// `positions_are_unique` needed no change — `SharedBatchPositionStore`
    /// assigns one position to a whole batch, which is a duplicate in any order.
    pub async fn positions_are_strictly_monotonic<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Dee"), event("Cee")]).await;
        append_ok(&store, &[event("Bee"), event("Ay")]).await;

        let positions = positions_of(&read_ok(&store, &Query::all(), ReadOptions::new()).await);
        assert_eq!(
            positions.len(),
            4,
            "all four appended events must come back before their order means \
             anything, got {positions:?}"
        );
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "sequence positions must increase monotonically (gaps are fine), got {positions:?}"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Append
    // ---------------------------------------------------------------------

    /// `append` returns the position of the last event written.
    ///
    /// Strengthened opportunistically where the fixture can open a second
    /// handle: the same claim, read back through a connection that did not make
    /// the write. That is what catches an adapter returning a position from a
    /// per-session cache rather than from the store. It is a *strengthening*,
    /// not a requirement — the base assertion runs against every fixture, so
    /// this rule reports `Ran` either way.
    pub async fn append_returns_last_written_position<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let returned = append_ok(&store, &[event("A"), event("B"), event("C")]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            Some(returned),
            all.last().map(|event| event.position),
            "`append` must return the position assigned to the LAST event in the batch"
        );

        if F::SECOND_HANDLE.is_supported() {
            let observer = fixture.connect().await;
            let elsewhere = read_ok(&observer, &Query::all(), ReadOptions::new()).await;
            assert_eq!(
                Some(returned),
                elsewhere.last().map(|event| event.position),
                "and a second handle must agree about which position that was"
            );
        }

        RuleOutcome::Ran
    }

    /// A batch lands entirely or not at all.
    pub async fn append_is_atomic<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// A batch interrupted by a fault part way through lands whole or not at all.
    ///
    /// ES-18's second rule, and the **complement** of `append_is_atomic` rather
    /// than its successor. That one reaches the write path through the
    /// *condition*, which rejects a store that writes before it decides
    /// (`WriteThenCheckStore` is the compiled version). This one reaches the case
    /// a condition can never produce: two rows written, the third failing, and
    /// the store obliged to undo the first two. Whoever is tempted to merge them
    /// should read ES-18, where the merge was made once and reversed by compiling
    /// a store.
    ///
    /// It rejects a per-row `INSERT` loop with the `BEGIN` forgotten — the same
    /// shape `a_concurrent_reader_never_sees_a_partial_batch` catches from the
    /// *visibility* side, where every row does eventually land. Here a row does
    /// not land, and rollback is the only thing that can save the store.
    ///
    /// # Why it is capability-gated, and why the capability is the fixture's
    ///
    /// Nothing an outside caller holds can reach between two rows of one
    /// `append`; that is the property under test. So the injection belongs to the
    /// adapter — a trigger that raises on the third insert, a constraint armed
    /// for one write — and
    /// [`MID_BATCH_FAULT`](crate::Fixture::MID_BATCH_FAULT) is where a store says
    /// whether it has one. A store with no way to fail a single row declines and
    /// this rule reports a skip, which is an honest hole rather than a silent
    /// pass.
    ///
    /// # Both answers are checked, and neither is assumed
    ///
    /// A store may swallow the fault and commit the batch anyway — retrying the
    /// row, say. That is conformant. What is not conformant is disagreeing with
    /// itself: `Err` and a partial log, or `Ok` and a partial log.
    pub async fn append_is_atomic_under_a_mid_batch_fault<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: MID_BATCH_FAULT);

        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Existing")]).await;

        let before = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&before),
            ["Existing"],
            "the anchor: without a row already in the store, \"unchanged\" and \
             \"empty\" are the same observation and a store that lost everything \
             would pass"
        );

        // Fail while writing the third event of the batch below, so that two rows
        // are already down when the fault arrives.
        fixture.arm_mid_batch_fault(2).await;
        let batch = [event("X"), event("Y"), event("Z")];
        let outcome = store.append(&batch, None).await;

        let after = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        if outcome.is_err() {
            assert_eq!(
                snapshot_of(&after),
                snapshot_of(&before),
                "a refused append must leave the store byte-identical (ES-18); a \
                 per-row INSERT loop with no transaction around it leaves the \
                 rows it managed to write. Got {:?}",
                types_of(&after)
            );
        } else {
            assert_eq!(
                types_of(&after),
                ["Existing", "X", "Y", "Z"],
                "a store that answers `Ok` after swallowing the fault must hold \
                 the whole batch: `Ok` over a partial log is the same violation \
                 as `Err` over one, with the caller misled the other way"
            );
        }

        RuleOutcome::Ran
    }

    /// An empty batch is refused.
    pub async fn append_rejects_empty_batch<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let result = store.append(&[], None).await;

        assert!(
            matches!(result, Err(AppendError::NoEvents)),
            "the specification defines a batch as non-empty; expected AppendError::NoEvents"
        );

        RuleOutcome::Ran
    }

    /// An empty batch is refused **before** the condition is evaluated.
    ///
    /// ES-20 `[FROZEN]`, and **the rule whose first casualty was the reference
    /// implementation** — which is the opposite of decorative. `MemoryEventStore`
    /// evaluated the condition first, so `append(&[], Some(&c))` answered
    /// `NoEvents` or `ConditionViolated` depending on what the store happened to
    /// hold. Two conformant adapters could disagree about the same call, and the
    /// disagreement is not cosmetic: `ConditionViolated` is the DCB concurrency
    /// signal and its documented meaning is *rebuild the decision model and
    /// retry*, so a caller whose retry loop branches on
    /// [`is_condition_violated`](happenstance_core::AppendError::is_condition_violated)
    /// never terminates. An empty batch will still be empty next time.
    ///
    /// `append_rejects_empty_batch` cannot see any of that: it only ever calls
    /// `append(&[], None)`, where there is no condition to be evaluated in the
    /// wrong order.
    pub async fn empty_batch_is_refused_before_the_condition_is_evaluated<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("Blocker")]).await;

        // The condition matches, so a store that evaluates it first answers
        // ConditionViolated and a store that checks its argument first answers
        // NoEvents. Both are refusals; only one names the actual fault.
        let result = store
            .append(&[], Some(&condition(query_of_types(&["Blocker"]))))
            .await;

        assert!(
            matches!(result, Err(AppendError::NoEvents)),
            "an empty batch is the caller's own bug and the emptiness check MUST \
             precede the condition check (ES-20): `ConditionViolated` means \
             `retry`, and retrying an empty batch never succeeds. Got {result:?}"
        );

        RuleOutcome::Ran
    }

    /// Payload, tags and metadata survive the round trip unchanged.
    pub async fn append_preserves_event_payload<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Value edges
    //
    // Every rule in this group appends a value that is legal, uninteresting to
    // read, and at the exact edge of what some column can hold. They exist
    // because the *middle* of a range is what every other rule in the suite
    // exercises: `append_preserves_event_payload` builds one event with a
    // thirteen-byte payload and eight bytes of metadata, which are precisely the
    // two values that make a lossy mapping look total.
    // ---------------------------------------------------------------------

    /// A zero-length payload round-trips as a zero-length payload.
    ///
    /// VT-1. `Event::data` is a `Bytes`, never an `Option<Bytes>`, so "no
    /// payload" is not a value the type can hold — and an adapter whose column is
    /// nullable has to decide what an empty slice means on the way in. The
    /// decision that is easy to write and wrong is `(!data.is_empty()).then_some(data)`,
    /// applied by one helper to both blob columns because `metadata` genuinely is
    /// optional. Against a `data BLOB NOT NULL` column that is a constraint
    /// violation on a perfectly legal event; against a nullable one it is silent
    /// loss.
    ///
    /// `EmptyPayloadIsNullStore` is the compiled version, and it is the *only*
    /// one this rule can have. The rule reads twice — the payload is zero-length,
    /// and the rest of the event is untouched — and no store can fail the second
    /// half while passing the first: `data` is a `Bytes`, so "absent" is not a
    /// value it can be conflated with the way `MetadataConflatingStore` conflates
    /// the nullable column, and truncating a zero-length payload is a no-op.
    /// Refusing is the whole observable defect. Those two assertions are here
    /// because they say what the rule means, not because a registered store
    /// fails them.
    pub async fn append_preserves_an_empty_payload<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Tagged, and every value-edge event below is, for one reason worth
        // stating once: an *untagged* event is its own edge, `untagged_events_match_query_all`
        // owns it, and leaving these untagged would make every rule here fail
        // against `InnerJoinTagStore` at its setup anchor rather than at the
        // property it is named for. One tag per event also keeps a fan-out join
        // invisible, which is `TagJoinFanOutStore`'s axis and not this one's.
        let original =
            event_with_payload("Zeroed", b"").with_tags(tags(&[("edge", "empty-payload")]));
        append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            1,
            "an event with an empty payload must still be an event: {:?}",
            types_of(&all)
        );
        assert!(
            all[0].event.data().is_empty(),
            "a zero-length payload must read back as zero-length, not as a \
             substituted default: got {} byte(s)",
            all[0].event.data().len()
        );
        assert_eq!(
            all[0].event, original,
            "and the rest of the event must be untouched by whatever the empty \
             payload was mapped to"
        );

        RuleOutcome::Ran
    }

    /// Absent metadata and present-but-empty metadata are two different values.
    ///
    /// VT-1, and the half of it a round-trip rule over one event cannot reach.
    /// `metadata: None` means *the writer attached none*; `Some(&[])` means *the
    /// writer attached a zero-length blob* — a codec that emits nothing for an
    /// empty struct produces the second, and a consumer branching on
    /// `metadata.is_some()` reads the two differently.
    ///
    /// The wrong implementation is a nullable `BLOB` column plus a driver that
    /// maps a zero-length value to `NULL`, or an `Option<Vec<u8>>` normalised
    /// with `filter(|m| !m.is_empty())` on the way in. Both collapse the pair;
    /// `MetadataConflatingStore` is the second, and `DropsMetadataStore` — which
    /// forgets the column entirely — fails this rule too, one column further
    /// along the same road.
    ///
    /// The two events are found by type rather than by index. This rule owns the
    /// metadata distinction and `read_defaults_to_ascending_order` owns order; a
    /// store that returned them the other way round would otherwise fail this
    /// rule for a reason that is not this rule's.
    pub async fn metadata_distinguishes_absent_from_empty<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(
            &store,
            &[
                tagged_event("Absent", &[("edge", "absent")]),
                tagged_event("Empty", &[("edge", "empty")]).with_metadata(&b""[..]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let find = |wanted: &str| {
            all.iter()
                .find(|event| event.event_type().as_str() == wanted)
                .unwrap_or_else(|| panic!("the store must hold the `{wanted}` event"))
        };

        assert!(
            find("Absent").event.metadata().is_none(),
            "an event appended with no metadata must read back with none"
        );

        let carried = find("Empty").event.metadata();
        assert!(
            carried.is_some(),
            "an event appended with zero-length metadata must read back with \
             metadata present: a nullable column that maps an empty blob to NULL \
             makes `None` and `Some(&[])` indistinguishable"
        );
        assert!(
            matches!(carried, Some(metadata) if metadata.is_empty()),
            "and that metadata must still be zero-length, not a substituted \
             default"
        );

        RuleOutcome::Ran
    }

    /// An identifier at the documented maximum length survives, and still
    /// matches itself.
    ///
    /// **VT-14 and VT-20, deliberately.** There is one round trip here and two
    /// clauses with a stake in it: VT-14 owns the boundary of *validity* — this
    /// is the longest value the constructors accept — and VT-20 owns the boundary
    /// of the *constant*, which is 255 bytes and stays there. A second rule under
    /// a second name would buy two tests that can only ever fail together.
    ///
    /// What the rule rejects is a column **narrower than the constant** —
    /// `VARCHAR(64)`, picked because nobody in the author's domain had a longer
    /// event type. Under `MySQL`'s non-strict mode that truncates rather than
    /// refuses, and the truncated tag then matches no query, including the one
    /// built from the value that was written. That is why the rule reads twice:
    /// once for the bytes and once through a query, because an adapter that keeps
    /// the blob and truncates only the index passes the first read.
    ///
    /// The tag is multi-byte at the limit for a *second* reason, and it is worth
    /// separating from the first because the obvious framing does not survive
    /// contact with SQL. The obvious framing is "`MAX_TAG_LEN` counts bytes and a
    /// `VARCHAR(255)` counts characters, so a multi-byte value at the limit is
    /// truncated" — and no `VARCHAR(255)` truncates a 255-byte value: it is 85
    /// characters under character semantics and 255 bytes under byte semantics,
    /// and it fits either way. What the multi-byte value actually earns is
    /// `Latin1IdentifierStore`, the *charset*-lossy column, which fails this rule
    /// as well as its own — one boundary, two ways for a column to be wrong about
    /// it, and the registry records both.
    ///
    /// `NarrowIdentifierColumnStore` is the compiled version.
    pub async fn store_accepts_a_max_length_identifier<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let long_type = "E".repeat(MAX_EVENT_TYPE_LEN);
        // "kk" + ":" is three bytes, and 'क' is three, so 84 of them land the
        // joined tag on exactly MAX_TAG_LEN with no ASCII character inside it.
        let long_value = "क".repeat(84);
        let pairs = [("kk", long_value.as_str())];

        let original = tagged_event(&long_type, &pairs);
        // The non-vacuity anchors: without them a change to either constant, or
        // to `Tag::key_value`'s joining, would quietly move this rule off the
        // boundary it is named for and it would go on passing.
        assert_eq!(
            original.event_type().as_str().len(),
            MAX_EVENT_TYPE_LEN,
            "this rule must sit on the event-type boundary, not near it"
        );
        assert_eq!(
            original.tags().as_slice()[0].as_str().len(),
            MAX_TAG_LEN,
            "and on the tag boundary"
        );

        append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1, "the store must hold the event it accepted");
        assert_eq!(
            all[0].event, original,
            "an identifier at the documented maximum must round-trip byte for \
             byte: an adapter that truncates it to fit its own column stores a \
             value that is not the one the caller wrote, with no error anywhere"
        );

        let by_tag = read_ok(&store, &query_tagged(&pairs), ReadOptions::new()).await;
        assert_eq!(
            positions_of(&by_tag),
            positions_of(&all),
            "and it must still match a query built from the same tag — a \
             truncated index entry matches nothing, including itself"
        );

        RuleOutcome::Ran
    }

    /// Non-ASCII identifiers survive, and are not folded into each other.
    ///
    /// VT-14. `EventType::new` and `Tag::new` reject `char::is_control`, which is
    /// Unicode general category `Cc` and **not** ASCII — four doc comments in
    /// `happenstance-core` say ASCII and are wrong about their own code. Category
    /// `Cf` is deliberately *not* rejected, because U+200C and U+200D are load
    /// bearing in Persian, in Devanagari and in every emoji ZWJ sequence, so all
    /// three appear here.
    ///
    /// The store's half is a separate obligation from the constructors': a value
    /// the constructor accepts still has to survive a column. What this rejects
    /// is an ASCII-only one — `VARCHAR` under a `latin1` collation, SQL Server's
    /// non-`N` `VARCHAR`, or a `CHECK` written against `[[:ascii:]]` — where the
    /// driver transcodes and every codepoint outside the target charset becomes
    /// `?`. `Latin1IdentifierStore` is the compiled version.
    ///
    /// The two Devanagari types differing only by a vowel sign are the folding
    /// half: a collation that normalises or case-folds makes them one type, and
    /// the query that should select one selects both.
    pub async fn store_accepts_non_ascii_identifiers<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Persian, carrying U+200C ZERO WIDTH NON-JOINER; Devanagari; and an
        // emoji ZWJ sequence carrying U+200D. All three are `Cf`, all three are
        // legal, and a validator that banned `Cf` wholesale would reject all of
        // them.
        let persian = "سفارش\u{200c}ثبت";
        let devanagari = "आदेश";
        let family = "👨\u{200d}👩\u{200d}👧";

        let ordered = tagged_event(persian, &[("مشتری", devanagari)]);
        let acknowledged = tagged_event(devanagari, &[("emoji", family)]);
        let first = append_ok(&store, core::slice::from_ref(&ordered)).await;
        append_ok(&store, core::slice::from_ref(&acknowledged)).await;

        // Read by type rather than through `Query::all()`, because the second
        // read is the one that matters here: a store that keeps the bytes and
        // mangles the index passes a round trip and fails this.
        let by_type = read_ok(&store, &query_of_types(&[persian]), ReadOptions::new()).await;
        assert_eq!(
            by_type.len(),
            1,
            "a non-ASCII event type must match itself, and only itself: got {:?}",
            types_of(&by_type)
        );
        assert_eq!(
            by_type[0].event, ordered,
            "and the event must round-trip byte for byte — a latin1 column \
             transcodes every codepoint outside its charset to `?`, silently"
        );

        let by_tag = read_ok(
            &store,
            &query_tagged(&[("مشتری", devanagari)]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            positions_of(&by_tag),
            [first.get()],
            "as must a non-ASCII tag"
        );

        let by_emoji = read_ok(&store, &query_of_types(&[devanagari]), ReadOptions::new()).await;
        assert_eq!(
            by_emoji.len(),
            1,
            "and the second event must be reachable too: {:?}",
            types_of(&by_emoji)
        );
        assert_eq!(
            by_emoji[0].event, acknowledged,
            "including the emoji ZWJ sequence in its tag, whose U+200D is \
             category `Cf` and legal"
        );

        RuleOutcome::Ran
    }

    /// A payload at the guaranteed minimum is accepted whole.
    ///
    /// VT-21, which states a **floor** rather than a ceiling: there is no
    /// `MAX_EVENT_DATA_LEN`, every store MUST accept 65,536 bytes, and a store
    /// that refuses more MUST say so with a distinguishable error rather than
    /// truncating. This rule owns the floor; the refusal is
    /// `append_reports_exceeded_store_limits`, which needs the
    /// `AppendError::ExceedsStoreLimit` variant VT-25 introduces at phase 4.
    ///
    /// What it rejects is an undocumented row-size ceiling — a payload stored
    /// inline in a fixed-width column, or a KV backend with a per-value cap the
    /// adapter never states. Both of the engine's answers to that are compiled,
    /// and the rule needs both because they fail at different places:
    /// `PayloadCeilingStore` refuses, which the `append_ok` above catches before
    /// the round trip runs at all, and `TruncatingPayloadStore` accepts and
    /// stores a prefix, which is what the length assertion below is for. VT-21's
    /// MUST names the second — "refuse an oversized payload … rather than
    /// truncating" — and until stage 6's review nothing rejected it. Both sit at
    /// 4 KiB, which is the shape rather than a specific product.
    ///
    /// **The clause is `[PROVISIONAL]` and phase 4 freezes it.** The number is
    /// read from VT-21, not from `happenstance-core`, because the constant it
    /// mandates does not exist yet.
    pub async fn store_accepts_the_guaranteed_minimum_payload<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // A repeating non-zero pattern rather than zeroes: a column that stores
        // the length and nothing else, or a driver that treats a run of NULs as a
        // terminator, both survive an all-zero payload.
        let payload: Vec<u8> = (0..MIN_SUPPORTED_EVENT_DATA_LEN)
            .map(|byte| u8::try_from(byte % 251).unwrap_or(0))
            .collect();
        let original =
            event_with_owned_payload("Bulk", payload).with_tags(tags(&[("edge", "min-payload")]));

        append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1, "the store must hold the event it accepted");
        assert_eq!(
            all[0].event.data().len(),
            MIN_SUPPORTED_EVENT_DATA_LEN,
            "VT-21 makes {MIN_SUPPORTED_EVENT_DATA_LEN} bytes a floor every store \
             MUST accept, and a store that truncates rather than refusing loses \
             data the caller was told had landed"
        );
        assert_eq!(
            all[0].event, original,
            "and the bytes must be the ones that were written"
        );

        RuleOutcome::Ran
    }

    /// An event carrying the guaranteed minimum number of tags keeps all of them.
    ///
    /// VT-22, the same shape as VT-21 one field over: no `MAX_TAGS`, `Tags`
    /// enforces no count, and every store MUST accept 64 tags on one event.
    ///
    /// What it rejects is an adapter that packs tags into a fixed-width column —
    /// a comma-joined `VARCHAR(255)` is the usual one, chosen to avoid a side
    /// table and a join — and silently drops whatever does not fit. That
    /// reproduces VT-17's map-shaped-index failure from a different cause: the
    /// event is in the store, and a query on the tag that fell off the end does
    /// not find it. `PackedTagColumnStore` is the compiled version.
    ///
    /// It deliberately does **not** also read back through a query built from the
    /// sixty-fourth tag, which would be the natural second half. That assertion
    /// is `query_item_tags_match_supersets`' — an item constraining one tag of a
    /// sixty-four-tag event is a superset match, and adding it here would buy a
    /// second failure on `ExactTagMatchReadStore` and no new coverage. What this
    /// rule owns is that all sixty-four tags are *stored*.
    ///
    /// **The clause is `[PROVISIONAL]` and phase 4 freezes it.**
    pub async fn store_accepts_the_guaranteed_minimum_tag_count<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let owned: Vec<(String, String)> = (0..MIN_SUPPORTED_TAGS_PER_EVENT)
            .map(|n| (format!("k{n:03}"), format!("v{n:03}")))
            .collect();
        let pairs: Vec<(&str, &str)> = owned
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let original = tagged_event("Rich", &pairs);
        assert_eq!(
            original.tags().len(),
            MIN_SUPPORTED_TAGS_PER_EVENT,
            "the anchor: `Tags` deduplicates, so a generator that collided would \
             leave this rule testing a smaller number than it claims"
        );

        append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1, "the store must hold the event it accepted");
        assert_eq!(
            all[0].event.tags(),
            original.tags(),
            "VT-22 makes {MIN_SUPPORTED_TAGS_PER_EVENT} tags a floor every store \
             MUST accept, and an adapter that packs them into one column drops \
             the overflow silently"
        );

        RuleOutcome::Ran
    }

    /// A query at the guaranteed minimum item count is evaluated in full.
    ///
    /// VT-23. There is no `MAX_QUERY_ITEMS` and `Query::from_items` enforces no
    /// count; every store MUST evaluate 128 items. A store or an ingest policy
    /// MAY *refuse* a larger query, which is a different thing from silently
    /// evaluating part of one.
    ///
    /// What it rejects is an adapter that emits one bound parameter per item and
    /// discovers a driver limit — `SQLITE_MAX_VARIABLE_NUMBER`, a Postgres
    /// 65,535-parameter cap — by chunking and then forgetting to union the
    /// chunks. `ChunkedQueryStore` is the compiled version, and it evaluates the
    /// first chunk only, which is why the item that matches is the **last** one.
    ///
    /// **The clause is `[PROVISIONAL]` and phase 4 freezes it.**
    pub async fn store_evaluates_a_query_at_the_guaranteed_minimum_item_count<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let wanted = append_ok(&store, &[tagged_event("Wanted", &[("q", "1")])]).await;
        append_ok(&store, &[tagged_event("Ignored", &[("q", "2")])]).await;

        // 127 items that match nothing, then the one that matches. A store that
        // evaluates a prefix of the list returns nothing; a store that evaluates
        // all of it returns exactly the first event.
        //
        // The 127 are constrained by *tag* rather than by type, and that is not
        // arbitrary. `UninternedTypeStore` drops a type clause naming only types
        // nobody has ever written — the interned id list comes back empty and the
        // clause is a syntax error — so 127 type-only items for unwritten types
        // would degenerate to 127 items matching everything, and this rule would
        // reject a store whose defect is somewhere else entirely.
        let mut items: Vec<_> = (0..MIN_SUPPORTED_QUERY_ITEMS - 1)
            .map(|n| item_tagged(&[("absent", format!("{n:03}").as_str())]))
            .collect();
        items.push(item_of_types(&["Wanted"]));
        let query = query_of_items(items);
        assert!(
            query
                .items()
                .is_some_and(|items| items.len() == MIN_SUPPORTED_QUERY_ITEMS),
            "the anchor: a query that lost items on construction would leave this \
             rule testing a smaller number than it claims"
        );

        let found = read_ok(&store, &query, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&found),
            [wanted.get()],
            "VT-23 makes {MIN_SUPPORTED_QUERY_ITEMS} items a floor every store \
             MUST evaluate, and an adapter that sends only its first chunk of \
             bound parameters returns an answer that is wrong rather than an \
             error that is honest"
        );

        RuleOutcome::Ran
    }

    /// A batch at the guaranteed minimum size is accepted whole.
    ///
    /// VT-24. There is no `MAX_EVENTS_PER_BATCH` and no `EventBatch` newtype to
    /// carry one; every store MUST accept an append of 128 events.
    ///
    /// What it rejects is an adapter that builds a multi-row `INSERT` with one
    /// parameter set per event and per tag and meets its driver's parameter
    /// ceiling at write time — that is, after the caller has already made its
    /// decision and taken its side effects. `BatchParameterCeilingStore` is the
    /// compiled version, and `ChunkLosingBatchStore` is the fix for it applied
    /// wrongly: `&events[..CEILING]` where `events.chunks(CEILING)` was meant, so
    /// the store answers `Ok` with a real position and the tail was never
    /// written. The two fail this rule at different assertions — the first at the
    /// `append_ok` above, the second at the length check below — and the second
    /// is the reason the round trip is here rather than a bare `is_ok()`.
    ///
    /// Atomicity is ES-18's and the return value is ES-19's; neither is
    /// re-asserted here, and the omission is deliberate rather than an oversight.
    /// What this rule owns is that a batch of 128 is accepted at all and comes
    /// back whole.
    ///
    /// The final comparison is by *value and order*, and the order half is an
    /// anchor rather than a claim: `read_defaults_to_ascending_order` owns
    /// ordering and `SortByEventTypeStore` is its mutant. Nothing is registered
    /// that returns all 128 in the wrong order and passes that rule, so the order
    /// half of this assertion is deliberately un-mutated — comparing the whole
    /// slice is simply cheaper than comparing a multiset.
    ///
    /// **The clause is `[PROVISIONAL]` and phase 4 freezes it.**
    pub async fn store_accepts_the_guaranteed_minimum_batch_size<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let batch: Vec<_> = (0..MIN_SUPPORTED_EVENTS_PER_BATCH)
            .map(|n| tagged_event("Bulk", &[("n", format!("{n:03}").as_str())]))
            .collect();
        append_ok(&store, &batch).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            MIN_SUPPORTED_EVENTS_PER_BATCH,
            "VT-24 makes {MIN_SUPPORTED_EVENTS_PER_BATCH} events a floor every \
             store MUST accept in one append"
        );

        let stored: Vec<&Event> = all.iter().map(|event| &event.event).collect();
        let written: Vec<&Event> = batch.iter().collect();
        assert_eq!(
            stored, written,
            "and every event of the batch must come back, in the order it was \
             written: an adapter that splits the batch to fit its driver's \
             parameter limit loses a chunk rather than reporting one"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Append conditions
    // ---------------------------------------------------------------------

    /// Without `after`, any matching event rejects the append.
    pub async fn condition_without_after_rejects_any_match<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// A condition that matches nothing lets the append through.
    pub async fn condition_without_after_allows_non_match<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    /// `after` is exclusive: an event exactly at the boundary was already seen
    /// by the caller and must not reject the append.
    pub async fn condition_after_ignores_events_at_the_boundary<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let boundary = append_ok(&store, &[event("Blocker")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            result.is_ok(),
            "`after` is exclusive: the event AT that position was already \
             accounted for and must not reject the append, got {result:?}"
        );

        RuleOutcome::Ran
    }

    /// A matching event beyond the boundary rejects the append.
    pub async fn condition_after_rejects_events_beyond_the_boundary<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let boundary = append_ok(&store, &[event("Seen")]).await;
        append_ok(&store, &[event("Blocker")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            matches!(result, Err(AppendError::ConditionViolated(_))),
            "a matching event AFTER the boundary is one the caller never saw \
             and must reject the append"
        );

        RuleOutcome::Ran
    }

    /// Non-matching events beyond the boundary are irrelevant.
    pub async fn condition_after_ignores_non_matching_events<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let boundary = append_ok(&store, &[event("Seen")]).await;
        append_ok(&store, &[event("Irrelevant")]).await;

        let condition = condition_after(query_of_types(&["Blocker"]), boundary.get());
        let result = store.append(&[event("New")], Some(&condition)).await;

        assert!(
            result.is_ok(),
            "only events matching the condition's query may reject an append, got {result:?}"
        );

        RuleOutcome::Ran
    }

    /// A condition's verdict depends on **tags**, not on types alone.
    ///
    /// ES-27 and CF-7, and it closes the widest hole the audit measured. Every
    /// condition the suite built came from `query_of_types`; the two that carried
    /// tags — `racing_conditional_appends_elect_one_winner` and
    /// `two_handles_observe_each_others_appends` — ran against stores where a
    /// type-only probe returns the identical verdict. **No rule's verdict
    /// depended on the condition path matching tags at all.**
    ///
    /// The store holds two events of one type differing in tags, and the one the
    /// condition names carries an **extra** tag it does not. That second detail
    /// is what separates the two wrong probes: a probe that compares serialised
    /// tags with `=` rather than as a subset requirement finds no violation here,
    /// while a probe that drops the tag join entirely finds one — correctly, by
    /// accident, which is why CF-8's mirror is a separate rule and neither is
    /// sufficient alone.
    pub async fn condition_matches_on_tags<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(
            &store,
            &[tagged_event(
                "Enrolled",
                &[("course", "c1"), ("student", "s1")],
            )],
        )
        .await;
        append_ok(&store, &[tagged_event("Enrolled", &[("course", "c2")])]).await;

        let result = store
            .append(
                &[event("Enrolled")],
                Some(&condition(query_of(&["Enrolled"], &[("course", "c1")]))),
            )
            .await;

        assert!(
            matches!(result, Err(AppendError::ConditionViolated(_))),
            "a condition's query is evaluated by exactly the read path's rules, \
             so a stored event whose tags are a SUPERSET of the condition's must \
             violate it. Got {result:?}"
        );

        RuleOutcome::Ran
    }

    /// The mirror: a condition carrying a tag no event holds must not reject,
    /// even when a stored event matches its types.
    ///
    /// ES-27 and CF-8. CF-7 alone catches only the fail-open direction; an
    /// adapter tuned to be "safe" by treating a condition's tags as advisory and
    /// rejecting on type alone is equally non-conformant and equally invisible
    /// without this. It is also the more dangerous of the two in production —
    /// **the canonical DCB uniqueness shape**, where dropping the expensive half
    /// of the probe rejects every command touching any entity of that type. A
    /// total-availability failure, certified as conformant.
    pub async fn condition_with_an_unheld_tag_does_not_reject<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Enrolled", &[("course", "c1")])]).await;

        let result = store
            .append(
                &[event("Enrolled")],
                Some(&condition(query_of(&["Enrolled"], &[("course", "c9")]))),
            )
            .await;

        assert!(
            result.is_ok(),
            "no stored event carries `course:c9`, so the condition matches \
             nothing and the append must be admitted — an adapter that drops the \
             tag join from its probe rejects every command touching any course. \
             Got {result:?}"
        );

        // The liveness mirror. Without it the rule's whole content is
        // `is_ok()`, which a store whose probe returns `None` unconditionally —
        // or whose seeding append silently did nothing — satisfies for free.
        // The same probe, one tag changed to the one the store *does* hold, must
        // *not* be admitted; so the acceptance above is evidence the probe ran
        // rather than evidence it is inert. (The same shape as
        // `read_defaults_to_ascending_order`'s length check and
        // `condition_rejection_leaves_store_unchanged`'s `before` guard.)
        //
        // `is_err`, deliberately, and not `ConditionViolated`. *Which* error a
        // rejection is reported as is ES-25's MUST and
        // `condition_rejection_is_reported_as_condition_violated` owns it; a
        // mirror demanding the discriminant here would make
        // `ViolationAsStoreErrorStore` fail a rule about tag matching for a
        // reason that has nothing to do with tags. The claim this assertion
        // needs is only that the probe distinguished the two conditions.
        let held = store
            .append(
                &[event("Enrolled")],
                Some(&condition(query_of(&["Enrolled"], &[("course", "c1")]))),
            )
            .await;
        assert!(
            held.is_err(),
            "and the acceptance above must be a verdict rather than an absence: \
             the same probe against `course:c1`, which the store does hold, must \
             not admit the append. Got {held:?}"
        );

        RuleOutcome::Ran
    }

    /// A condition evaluated against an **empty** store admits the append.
    ///
    /// ES-28 and CF-10. Every other condition rule seeds the store first, so the
    /// degenerate case had never run — and it is degenerate in SQL rather than in
    /// the abstract: `MAX(position)` over no rows is `NULL`, `NULL > ?` is
    /// *unknown* rather than false, and whether `WHERE` discards an unknown or a
    /// wrapping `NOT` turns it into a rejection depends on how the predicate is
    /// nested. `NullAggregateProbeStore` is the version that rejects.
    ///
    /// This is also every adapter's very first conditional append, which is what
    /// makes the case worth a rule of its own rather than a note.
    pub async fn condition_against_an_empty_store_admits_the_append<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let result = store
            .append(
                &[event("First")],
                Some(&condition(query_of_types(&["Anything"]))),
            )
            .await;

        assert!(
            result.is_ok(),
            "an empty store holds no event matching anything, so a condition \
             over it must admit the append. Got {result:?}"
        );

        // The liveness mirror, and here it does more work than elsewhere: the
        // store was *empty*, so an inert probe and a correct one are
        // indistinguishable on the assertion above. Re-running the same shape
        // now that the store holds `First` separates them.
        let occupied = store
            .append(
                &[event("Second")],
                Some(&condition(query_of_types(&["First"]))),
            )
            .await;
        // `is_err` rather than `ConditionViolated`, for the reason given on
        // `condition_with_an_unheld_tag_does_not_reject`'s mirror: the
        // discriminant is ES-25's rule's business, not this one's.
        assert!(
            occupied.is_err(),
            "and the admission above must be the probe's verdict rather than its \
             absence: the same condition shape over a store that now holds a \
             matching event must not be admitted. Got {occupied:?}"
        );

        RuleOutcome::Ran
    }

    /// A condition whose `after` is **beyond** the store's head admits the
    /// append.
    ///
    /// ES-28's second half. The *at* half needs no rule of its own and this is
    /// worth stating so nobody writes one:
    /// `condition_after_ignores_events_at_the_boundary` sets `after` to the
    /// position of the only event in the store, which is the head.
    ///
    /// The anchor is one past a position the store assigned, never a literal —
    /// and on a store that allocates in steps, one past the head is a position it
    /// will never assign, which is exactly the input a peer resuming after a gap
    /// supplies. `AfterValidatedAgainstHeadStore` refuses it as unknown, which is
    /// plausible, defensible and fatal.
    pub async fn condition_after_beyond_head_admits_the_append<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        // Two matching events rather than one, so that the mirror below has a
        // boundary the store actually assigned to sit under. `after` is a
        // `SequencePosition` and positions start at 1, so "one below the head"
        // is not expressible when the head is the first event.
        let earlier = append_ok(&store, &[event("Blocker")]).await;
        let head = append_ok(&store, &[event("Blocker")]).await;

        let beyond = head.get().saturating_add(1);
        let result = store
            .append(
                &[event("New")],
                Some(&condition_after(query_of_types(&["Blocker"]), beyond)),
            )
            .await;

        assert!(
            result.is_ok(),
            "`after` beyond the head names history the store does not have, \
             which is not a violation and not an error. Got {result:?}"
        );

        // The liveness mirror: the same query, `after` at the *earlier*
        // blocker, leaves the head blocker above the boundary and must reject.
        // Both boundaries come from positions the store assigned (CF-6).
        let mirror = store
            .append(
                &[event("New")],
                Some(&condition_after(
                    query_of_types(&["Blocker"]),
                    earlier.get(),
                )),
            )
            .await;
        assert!(
            mirror.is_err(),
            "and the admission above must be a comparison the probe performed \
             rather than one it skipped: with `after` under the head blocker the \
             same condition must not be admitted. Got {mirror:?}"
        );

        RuleOutcome::Ran
    }

    /// `after` past the last **matching** position admits the append, even while
    /// later non-matching events exist.
    ///
    /// ES-25's "if and only if" seen from the side nothing covered, and the case
    /// the DCB specification singles out in a Note.
    /// `condition_after_ignores_non_matching_events` reaches it only with the
    /// matching set *empty*, so an adapter could pass every rule in the suite
    /// without ever comparing a matching event's position against `after`.
    ///
    /// What it rejects is the probe that ANDs two uncorrelated predicates —
    /// *does any event match the query* and *is the head above `after`* — which
    /// is what the check becomes when the existence test and the position test
    /// are written as separate subqueries. It is correct on every case the rest
    /// of the suite exercises, and it rejects every command whose caller read to
    /// a boundary above the last event touching its own entity: the steady state
    /// of a quiet entity in a busy store, reported as contention.
    pub async fn condition_after_beyond_the_last_matching_position_admits_the_append<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Two matching events, because the mirror below needs a boundary
        // *under* a match and positions start at 1, so `last_match - 1` is not
        // expressible when `last_match` is the first event.
        let earlier_match = append_ok(&store, &[event("Blocker")]).await;
        let last_match = append_ok(&store, &[event("Blocker")]).await;
        // Busy store, quiet entity: two later events, neither of them the
        // condition's business.
        append_ok(&store, &[event("Irrelevant")]).await;
        append_ok(&store, &[event("Irrelevant")]).await;

        let result = store
            .append(
                &[event("New")],
                Some(&condition_after(
                    query_of_types(&["Blocker"]),
                    last_match.get(),
                )),
            )
            .await;

        assert!(
            result.is_ok(),
            "the caller has seen every matching event, and the events above its \
             boundary match nothing the condition asks about — so there is \
             nothing to invalidate its decision. Got {result:?}"
        );

        // The liveness mirror, and this is the rule where it matters most: ES-25
        // is the clause the DCB specification singles out with a Note, and as
        // `is_ok()` alone the rule could not fail for being too *permissive*.
        // Moved back one matching event, the same query must reject.
        let mirror = store
            .append(
                &[event("New")],
                Some(&condition_after(
                    query_of_types(&["Blocker"]),
                    earlier_match.get(),
                )),
            )
            .await;
        assert!(
            mirror.is_err(),
            "and the admission above must be the position comparison's verdict \
             rather than a probe that never matched: with `after` below the last \
             matching event the same condition must not be admitted. Got \
             {mirror:?}"
        );

        RuleOutcome::Ran
    }

    /// A rejected append changes nothing observable.
    ///
    /// Strengthened where a second handle exists: "nothing observable" is a
    /// claim about the *store*, not about the connection that made the attempt,
    /// and an adapter that rolls back its own session while leaving a shared
    /// cache dirty passes the single-handle version. As above this is a
    /// strengthening, so the rule reports `Ran` either way.
    ///
    /// # The `before` guard, and why it belongs to this rule
    ///
    /// Stage 3 left this open: the rule compares two reads for equality, and two
    /// *empty* reads are equal, so a store whose reads return nothing passed it —
    /// `InnerJoinTagStore` did. The argument for leaving it was that the rule's
    /// subject is the rejection rather than the read.
    ///
    /// The guard lands here anyway, because the subject is not "the append was
    /// rejected", it is "**the store is unchanged**", and an unchanged store is
    /// only observable if there was something there to be unchanged. Equality
    /// over nothing is the same vacuity `read_defaults_to_ascending_order`'s
    /// length check exists to remove — `windows(2)` over an empty slice is
    /// vacuously ordered — and it is one line in both places.
    pub async fn condition_rejection_leaves_store_unchanged<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[event("A"), event("B")]).await;
        let before = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&before),
            ["A", "B"],
            "both appended events must be visible before `unchanged` can mean \
             anything: two empty reads compare equal, and a store that shows \
             nothing would satisfy every assertion below without holding still"
        );

        let _ = store
            .append(&[event("C")], Some(&condition(query_of_types(&["A"]))))
            .await;

        let after = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            before, after,
            "a rejected append must leave the store byte-identical"
        );

        if F::SECOND_HANDLE.is_supported() {
            let observer = fixture.connect().await;
            let elsewhere = read_ok(&observer, &Query::all(), ReadOptions::new()).await;
            assert_eq!(
                before, elsewhere,
                "and a second handle must see the same store the first one does"
            );
        }

        RuleOutcome::Ran
    }

    /// The rejection is reported as the specification's concurrency signal, not
    /// as an adapter-specific failure.
    pub async fn condition_rejection_is_reported_as_condition_violated<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
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
    /// It is written sequentially **and through one handle**, which is less than
    /// it sounds like. An earlier version of this note claimed a stronger
    /// version would need `Send + Sync + 'static` bounds that [`EventStore`]
    /// deliberately does not carry; that is true of a genuinely *parallel*
    /// successor and false of the *two-handle sequential* one the fixture
    /// contract now makes writable —
    /// [`two_handles_observe_each_others_appends`] runs exactly this shape
    /// across two connections with no extra bound anywhere.
    ///
    /// # The parallel family exists, and this rule is retained beside it
    ///
    /// This note used to end "adapters that are `Send + Sync` should add a
    /// multi-threaded stress test of their own". Do not. That family now ships:
    /// `event_store_concurrency_conformance!`, eight contenders on real OS
    /// threads against one backing store. Invoke it rather than writing your
    /// own, and read the `concurrency` module's documentation before you do.
    ///
    /// Both names are spelled plainly rather than linked, and that is the same
    /// discipline the model family's three names are held to at the crate root:
    /// the `concurrency` module is `#[cfg(not(target_arch = "wasm32"))]`, this
    /// file is not, and rustdoc treats an unresolved link as a hard error — so a
    /// link here would break `cargo doc --target wasm32-unknown-unknown`, which
    /// is the target this crate's whole two-flavour story exists for.
    ///
    /// Two corrections come with it, both measured rather than reasoned.
    ///
    /// The **bound is `F::Store: EventStore + Send`**, not `Send + Sync`. Three
    /// of the four obligations that sounded necessary — the `Send` flavour of
    /// the port, `Sync`, and `'static` — turned out not to be, because each
    /// contender owns its handle and drives its own future on its own scoped
    /// thread. `concurrency.rs`'s bound section names what the compiler said
    /// about each part.
    ///
    /// And the family is **additive: this rule is retained, not retired.** The
    /// plan said stage 5 would retire it and the plan was wrong. What this rule
    /// fixes is the *semantics* a race must have — which of two decisions taken
    /// from one snapshot is allowed to land, which is ES-25's *iff* seen as two
    /// decisions from one snapshot, and two of the three registered mutants it
    /// rejects are shapes ES-25's own `Rejects:` paragraph names by hand. What
    /// it structurally cannot do is separate an atomic check-and-write from a
    /// probe followed by an insert, because with one caller at a time nothing
    /// can get between the two halves. The parallel family supplies the second
    /// caller. Neither subsumes the other, and deleting this one would leave the
    /// semantics unpinned for every `!Send` adapter, which cannot run the
    /// parallel family at all.
    ///
    /// The old note's remaining worry — that a parallel version risks a flaky
    /// suite, which is worse than none — was taken seriously rather than
    /// dismissed: the parallel rules synchronise on a
    /// [`Barrier`](std::sync::Barrier) and on the other threads, never on a
    /// clock, and CF-33 forbids the watchdog that would have been the tempting
    /// alternative.
    pub async fn racing_conditional_appends_elect_one_winner<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
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

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Re-entrancy
    //
    // ES-36. Both rules were `#[tokio::test]`s in the testkit's own
    // `tests/local_conformance.rs` until phase 3; promoting them is what lets
    // the two stores that fail them stop being `#[should_panic]` drivers — the
    // shape CF-2 rejects by name — and become registry rows that say which rule
    // each one fails.
    //
    // The clause describes the first as a `tokio::join!` on a `current_thread`
    // runtime. It is spelled here as hand-polling instead, because the suite
    // must run under three emitters and `block_on` is one of them: a rule that
    // needed `tokio` would be a rule the runtime-free harness could not carry.
    // The two are the same choreography — both futures exist, and each is polled
    // once, before either is allowed to finish.
    // ---------------------------------------------------------------------

    /// Two `append` futures from one handle both complete, and exactly one wins.
    ///
    /// ES-36. This is `racing_conditional_appends_elect_one_winner`'s question
    /// asked of a store that is genuinely re-entered rather than called twice in
    /// sequence, and `MemoryEventStore` cannot surface the difference: its
    /// `append` body has no suspension point, so it holds no lock across one.
    ///
    /// # What it rejects, and the asymmetry that matters in production
    ///
    /// A `RefCell`-backed adapter that holds its borrow across an awaited storage
    /// call — `AwaitAcrossBorrowStore` — panics here on the second poll.
    /// **For a pooled SQL adapter holding one connection the same defect is a
    /// deadlock, not a panic**, and that is the one place in this catalogue where
    /// a real adapter hangs where the mutant falls over. A hung test reports a
    /// timeout and names nothing; this rule is why the workspace's instrument for
    /// the shape is a `RefCell` rather than a pool.
    ///
    /// More subtly it also rejects a store that *drops* the borrow around the
    /// await and thereby leaves a window between its condition probe and its
    /// write, so that both writers see a store missing the other's row and both
    /// commit. `PreCommitPositionStore` is that store, and the winner assertion
    /// is what catches it.
    pub async fn interleaved_appends_on_one_handle_elect_one_winner<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let boundary = append_ok(
            &store,
            &[tagged_event("CourseDefined", &[("course", "c1")])],
        )
        .await;

        // Both handlers decided from the same snapshot, as in
        // `racing_conditional_appends_elect_one_winner` — the difference is
        // entirely in how the two appends are driven.
        let query = query_of(&["StudentSubscribed"], &[("course", "c1")]);
        let handler_a = condition_after(query.clone(), boundary.get());
        let handler_b = condition_after(query, boundary.get());
        let subscribe = tagged_event("StudentSubscribed", &[("course", "c1")]);

        // Both futures exist before either is polled: an `async fn` body runs
        // nothing until its first `poll`, so this is what puts two appends
        // genuinely in flight on one `&store` with no executor and no `Send`.
        let mut a = pin!(store.append(core::slice::from_ref(&subscribe), Some(&handler_a)));
        let mut b = pin!(store.append(core::slice::from_ref(&subscribe), Some(&handler_b)));
        let mut a_done = false;
        let mut b_done = false;

        // One poll each, then finish whatever is left. Asserting that one poll
        // was enough would be over-specification — a store with real I/O under
        // it may take any number — but the second poll of `a` can only happen
        // after `b` has been entered, which is the interleaving.
        let started_a = poll_once(a.as_mut(), &mut a_done).await;
        let started_b = poll_once(b.as_mut(), &mut b_done).await;

        let first = match started_a {
            Some(outcome) => outcome,
            None => a.await,
        };
        let second = match started_b {
            Some(outcome) => outcome,
            None => b.await,
        };

        // Which of the two wins is not the store's to have an opinion about, so
        // the rule does not have one either: it counts.
        let outcomes = [first, second];
        assert_eq!(
            outcomes.iter().filter(|result| result.is_ok()).count(),
            1,
            "two handlers that decided from the same snapshot cannot both \
             commit, and cannot both fail: exactly one must win. Got {outcomes:?}"
        );
        assert!(
            outcomes
                .iter()
                .any(|result| matches!(result, Err(AppendError::ConditionViolated(_)))),
            "and the loser must learn that it lost, as a condition violation \
             rather than as an adapter error. Got {outcomes:?}"
        );

        let landed = read_ok(
            &store,
            &query_of_types(&["StudentSubscribed"]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            landed.len(),
            1,
            "and exactly one of the two batches may be in the store"
        );

        RuleOutcome::Ran
    }

    /// A read stream that is still alive does not block an append.
    ///
    /// ES-36's read/write pairing. The stream is built and **not drained**, an
    /// append is made through the same handle, and the stream is drained
    /// afterwards.
    ///
    /// It rejects a store whose `read` hands back a stream holding a borrow of
    /// the store — a rusqlite adapter yielding rows from a live statement, an
    /// `Rc`-shared cursor, or `BorrowHoldingStore`, which is that shape in twenty
    /// lines. RPITIT lets an implementer return exactly that, so the *port*
    /// cannot forbid it and only a rule will catch it.
    ///
    /// The rule deliberately stops short of ES-11's snapshot claim. It asserts
    /// that the pre-existing event is still yielded — which is what makes it
    /// non-vacuous, since a store returning nothing would otherwise satisfy a
    /// rule about a stream surviving — and says nothing about whether the *new*
    /// event appears. `read_result_is_stable_under_concurrent_append` is where
    /// that belongs, and it is ES-11's, which is still provisional.
    pub async fn a_live_read_stream_does_not_block_an_append<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        let seeded = append_ok(&store, &[event("Seeded")]).await;

        // Bound to a named local rather than passed as a temporary: under
        // edition 2024 `read`'s RPITIT captures every in-scope lifetime, so the
        // stream borrows the query even though its hidden type owns all of its
        // events. Inlining is `error[E0716]`, which is also why ES-13 forbids
        // "fixing" it by taking `Query` by value.
        let query = Query::all();
        let stream = store.read(&query, ReadOptions::new());

        let appended = store.append(&[event("Later")], None).await;
        assert!(
            appended.is_ok(),
            "a read stream that has not been drained must not hold a borrow, a \
             lock or a connection that an append needs. Got {appended:?}"
        );

        let drained = match collect(stream).await {
            Ok(events) => events,
            Err(err) => panic!("the stream must still drain after the append, got {err:?}"),
        };
        assert!(
            drained.iter().any(|event| event.position == seeded),
            "and it must still yield what was visible when it was opened; \
             otherwise a store that returns nothing passes this rule for free"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Position visibility
    //
    // ES-10's invariant. Its three helpers are below it rather than with the
    // others at the top of the module, because the poll schedule reads better
    // beside the thing it schedules — `poll_once` is shared with the
    // re-entrancy rule above, which is the only other one that drives two
    // `append` futures by hand.
    // ---------------------------------------------------------------------

    /// Nothing becomes visible *below* a position a reader has already
    /// observed.
    ///
    /// ES-10 states it: once any reader has observed an event at position *P*,
    /// no subsequent read against that store may yield an event at a position
    /// ≤ *P* that was not already visible. It is the property that makes
    /// `AppendCondition::after` mean anything — `is_violated_by` compares
    /// position *values*, so a caller conditioning on `after: 100` while 99 is
    /// still invisible gets no violation and no error, and the consistency
    /// boundary silently stops enforcing.
    ///
    /// # The shape, and why it is single-threaded
    ///
    /// Two `append` futures are created from **one handle** and neither is
    /// polled by being created: an `async fn` body runs nothing until its first
    /// `poll`. That is what puts two appends genuinely in flight with no
    /// executor, no `Send` bound and no clock — CF-33 forbids a clock in a
    /// rule, and a wall-clock deadline in a conformance rule is a flake in a
    /// conformance rule. The alternative on the table was a `Send + Sync`
    /// sub-trait to bind parallel rules on; it was not needed, and CF-13
    /// records that.
    ///
    /// The schedule is **A, B, B, A**, and plain alternation is not a
    /// substitute. An adapter that allocates its position before committing but
    /// commits in allocation order is indistinguishable from a correct one; the
    /// reversal on resumption is what models the slow transaction that took a
    /// low number and published it late.
    ///
    /// # Why `MemoryEventStore` passes it trivially, and why that is right
    ///
    /// Its `append` body contains no `.await` at all, so the first poll runs it
    /// to completion and there is no interleaving to have. The rule is not
    /// weaker for that: a store with no window cannot have a window bug, and
    /// the schedule below skips a future that has already finished rather than
    /// polling it again.
    ///
    /// # A requirement this rule adds, which no clause states
    ///
    /// `observe` below runs a full `read` at four points where one or both
    /// `append` futures are **pinned, entered and unfinished**. So passing this
    /// rule requires that a `read` issued while an `append` on the same handle
    /// is suspended completes. ES-36 states the append-versus-append direction
    /// and `a_live_read_stream_does_not_block_an_append` the read-then-append
    /// one; neither states this, and ES-36's prose records that.
    ///
    /// **If your adapter hangs here, that is the reason.** An adapter holding
    /// one exclusive resource across its append's suspension point — a pooled
    /// connection, a `futures::lock::Mutex`, a Durable Object storage
    /// transaction — blocks the read on what the suspended append still holds,
    /// and the suspended future cannot be re-polled because this rule is inside
    /// the read on the same stack. CF-33 forbids a clock in a rule, so there is
    /// deliberately no watchdog to turn that into a message: it is a hung job
    /// naming no rule, which is why the diagnosis is written here.
    ///
    /// # What it rejects
    ///
    /// A Postgres adapter allocating positions with `nextval()` outside the
    /// transaction — a number taken before the work and released at commit, so
    /// a transaction that started earlier can become visible later.
    /// `PreCommitPositionStore` in `tests/mutation_coverage/mutants.rs` is that
    /// store, compiled and registered: polled A, B, B, A it publishes B's row
    /// first, a reader sees B's higher position, and then A's lower one appears
    /// underneath it. It is the one mutant in the registry whose bug is a
    /// faithful model of a real database rather than an implementation slip.
    pub async fn nothing_below_an_observed_position_appears_later<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Tagged, and that is not decoration. A store whose reads return
        // nothing satisfies a rule about *what appears* for free, and
        // `InnerJoinTagStore` — tags in a side table, joined with `INNER` — is
        // exactly that store for an untagged event. The tags are what keep the
        // closing assertion able to fail.
        let slow_batch = [tagged_event("SlowWriter", &[("writer", "slow")])];
        let fast_batch = [tagged_event("FastWriter", &[("writer", "fast")])];

        // Both futures exist before either is polled, and both hold `&store`.
        // On the `Send` flavour that would need `Sync`; here it needs nothing.
        let mut slow = pin!(store.append(&slow_batch, None));
        let mut fast = pin!(store.append(&fast_batch, None));
        let mut slow_done = false;
        let mut fast_done = false;

        // Every position ever observed, in the order it first became visible.
        let mut seen: Vec<u64> = Vec::new();

        // Step 1 — the slow writer starts, and takes whatever position its
        // adapter hands out.
        assert_appended(poll_once(slow.as_mut(), &mut slow_done).await);
        observe(&store, &mut seen, "after the slow writer's first poll").await;

        // Step 2 — the fast writer starts, and takes the next one. A store that
        // allocates outside its transaction has now handed out two numbers and
        // published neither row.
        assert_appended(poll_once(fast.as_mut(), &mut fast_done).await);
        observe(&store, &mut seen, "after the fast writer's first poll").await;

        // Step 3 — THE INTERLEAVING. The writer that started *second* is
        // resumed first and commits first, so its higher position becomes
        // visible while the lower one is still in flight.
        assert_appended(poll_once(fast.as_mut(), &mut fast_done).await);
        observe(&store, &mut seen, "after the fast writer committed").await;

        // Step 4 — and only now does the slow writer's row land, carrying the
        // position underneath the one a reader has already seen.
        assert_appended(poll_once(slow.as_mut(), &mut slow_done).await);
        observe(&store, &mut seen, "after the slow writer committed").await;

        // Whatever the schedule left unfinished, finish — in the same hostile
        // order, so an adapter needing three polls is not let off. Asserting
        // "two polls was enough" would be over-specification: a store with real
        // I/O under it may take any number, and a rule a legal adapter fails is
        // a finding about the rule (CF-6).
        if !fast_done {
            assert_appended(Some(fast.await));
        }
        if !slow_done {
            assert_appended(Some(slow.await));
        }
        observe(&store, &mut seen, "after both appends completed").await;

        // The non-vacuity anchor. Without it a store that returned nothing from
        // every read would satisfy every assertion above by never showing a
        // position at all — the same shape as `read_defaults_to_ascending_order`'s
        // length check, and here for the same reason.
        assert_eq!(
            seen.len(),
            2,
            "both appends were acknowledged, so both of their events must be \
             visible; the positions ever observed were {seen:?}"
        );

        RuleOutcome::Ran
    }

    /// Polls `future` exactly once unless it has already finished, and records
    /// when it has.
    ///
    /// `poll_fn`'s own future is `Ready` whatever the inner one said, which is
    /// what makes this *poll once* rather than *await*: awaiting the append
    /// would drive it to completion and there would be no interleaving left to
    /// observe. `block_on` therefore never parks here, so a store that returns
    /// `Pending` cannot hang the caller at this line.
    ///
    /// `finished` is not bookkeeping. Polling a future after it has returned
    /// `Ready` is a contract violation, and every store whose `append` has no
    /// `.await` in it finishes on the first poll — so without the flag, steps 3
    /// and 4 would poll a completed future.
    async fn poll_once<T>(
        mut future: Pin<&mut impl Future<Output = T>>,
        finished: &mut bool,
    ) -> Option<T> {
        if *finished {
            return None;
        }
        match core::future::poll_fn(|cx| Poll::Ready(future.as_mut().poll(cx))).await {
            Poll::Ready(value) => {
                *finished = true;
                Some(value)
            }
            Poll::Pending => None,
        }
    }

    /// Fails the test if an append that has completed did not succeed.
    ///
    /// `None` is a future that has not finished yet, which is legal at every
    /// step of the schedule and says nothing either way.
    fn assert_appended<E: core::fmt::Debug>(
        outcome: Option<Result<SequencePosition, AppendError<E>>>,
    ) {
        if let Some(result) = outcome {
            assert!(
                result.is_ok(),
                "an unconditional append must succeed, got {result:?}"
            );
        }
    }

    /// Reads the whole store and checks that nothing has appeared underneath
    /// something already observed.
    ///
    /// The ceiling is taken **before** anything from this read is recorded, so
    /// two positions becoming visible in the same step are judged against what
    /// was visible before the step rather than against each other — a store
    /// committing a batch is not violating anything.
    async fn observe<S: EventStore>(store: &S, seen: &mut Vec<u64>, step: &str) {
        let visible = positions_of(&read_ok(store, &Query::all(), ReadOptions::new()).await);
        let ceiling = seen.iter().copied().max();

        for position in visible {
            if seen.contains(&position) {
                continue;
            }
            // Compared against a position this store actually assigned and a
            // reader actually saw, never against a literal: the specification
            // permits gaps and any starting value (CF-6).
            if let Some(highest) = ceiling {
                assert!(
                    position > highest,
                    "ES-10: a reader has already observed position {highest}, so \
                     no later read may yield an event at or below it that was \
                     not already visible — but {step} the store made position \
                     {position} visible underneath it. That is what allocating \
                     a position outside the transaction buys: a caller that \
                     conditioned on `after: {highest}` never saw {position}, \
                     `is_violated_by` compares it against the boundary and \
                     reports no violation, and the consistency boundary stops \
                     enforcing with no error anywhere. Observed so far: {seen:?}"
                );
            }
            seen.push(position);
        }
    }
}
