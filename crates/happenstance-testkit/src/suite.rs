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

    // The two read-isolation rules poll a stream by hand rather than through
    // `collect`, so they name the trait. `futures_core` is already this crate's
    // dependency — `fixtures.rs` imports `Stream` from it — so this costs no
    // manifest change.
    use futures_core::Stream;

    use happenstance_core::{
        AppendError, Event, EventId, EventStore, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN, Query,
        ReadOptions, SequencePosition, SequencedEvent, StoreId, StoreLimit, collect,
    };

    use crate::fixtures::{
        condition, condition_after, event, event_with_owned_payload, event_with_payload,
        item_of_types, item_tagged, query_of, query_of_items, query_of_types, query_tagged,
        tagged_event, tags,
    };
    use crate::{Fixture, NO_CEILING_REASON, NO_STORE_LIMITS, RuleOutcome};

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

    /// Reads the head and unwraps, failing the test with context on error.
    async fn head_ok<S: EventStore>(store: &S) -> Option<SequencePosition> {
        match store.head().await {
            Ok(head) => head,
            Err(err) => panic!("head should succeed, got {err:?}"),
        }
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

    // ---------------------------------------------------------------------
    // Query algebra — the union law (VT-31, ES-15)
    // ---------------------------------------------------------------------

    /// The match set of concatenated items is the union of their match sets.
    ///
    /// VT-31, and the rule ES-15's `Rejects:` names as still owed: *an adapter
    /// that sorts and deduplicates a query's items as an optimisation.*
    /// `query_item_order_does_not_change_the_result_set` cannot catch that —
    /// sorting the items is precisely what makes their order stop mattering, so
    /// an adapter that sorts them passes an order-invariance rule by
    /// construction. What catches it is a **match-set** claim over an item whose
    /// presence changes the set.
    ///
    /// The two items constrain **tags alone**, so both carry the same (empty)
    /// type list — which is the key a deduplicating adapter interns them under.
    /// `QueryItem::new` already sorts and deduplicates *types*, so interning
    /// items by their type list to emit one `type_id IN (…)` clause per distinct
    /// set is the natural next step, and it silently drops the tag half of every
    /// item it swallows. `ItemDedupByTypeStore` is that adapter.
    ///
    /// A shared *non-empty* type list would demonstrate the same collapse, and
    /// was rejected for a reason worth recording: an item carrying both a type
    /// and a tag is `ClauseJoinerStore`'s blind spot, so that mutant would fail
    /// this rule at its anchor for a reason
    /// `query_item_combines_types_and_tags_with_and` already owns.
    ///
    /// The two single-item reads are the non-vacuity anchor, and they carry more
    /// weight here than usual: without them a store returning *both* events for
    /// *every* query would satisfy the union assertion, and so would one
    /// returning neither for any.
    ///
    /// **What this rule cannot state**, recorded so nobody looks for it. VT-31's
    /// second sentence — any query unioned with `Query::All` matches everything —
    /// is not expressible through the constructors. A `Query` is `All` *or*
    /// `Items`, `Query::All` is not a `QueryItem`, and there is no union
    /// operator, so the concatenation this rule performs cannot include it.
    pub async fn query_union_is_item_concatenation<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let one = append_ok(&store, &[tagged_event("Enrolled", &[("course", "c1")])]).await;
        let two = append_ok(&store, &[tagged_event("Enrolled", &[("course", "c2")])]).await;

        // Tag-only items, so both intern under the same empty type list. An
        // adapter deduplicating by that key keeps whichever it sees first and
        // drops the other's tag constraint with it.
        let course_one = item_tagged(&[("course", "c1")]);
        let course_two = item_tagged(&[("course", "c2")]);

        let left = query_of_items([course_one.clone()]);
        let right = query_of_items([course_two.clone()]);
        let concatenated = query_of_items([course_one, course_two]);

        let left_set = read_ok(&store, &left, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&left_set),
            [one.get()],
            "the first query alone must select exactly its own event — otherwise \
             the union assertion below holds for the wrong reason"
        );

        let right_set = read_ok(&store, &right, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&right_set),
            [two.get()],
            "and the second query alone must select exactly its own"
        );

        let union = read_ok(&store, &concatenated, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&union),
            [one.get(), two.get()],
            "the match set of a query whose items are the CONCATENATION of two \
             queries' items must be the union of their match sets. An adapter \
             that interns items by their type list collapses these two into one, \
             passes every order-invariance rule by construction, and breaks the \
             fan-out projection runner — one read of the union query, then a \
             local re-filter per projection through `Query::matches`"
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

    /// Reading forwards **from** a cursor **with** a budget spends the budget.
    ///
    /// CF-12's SHOULD, ES-14 and VT-28 all name this one composition as the
    /// consumer that motivates them — a paging loop that resumes carries `from`,
    /// and `ReadOptions`' own documentation writes the idiom out as
    /// `.limit(budget - fetched)` — and until this rule the suite issued it
    /// nowhere. `from` was composed with `to`, with `backwards`, and with
    /// `backwards` **and** `limit`; forwards `from` with `limit` appeared at no
    /// call site among the eighty-nine rules that preceded this one.
    ///
    /// It rejects `ForwardPagingBudgetStore`: `limit` applied only where `from`
    /// is absent, because the resume branch was written after the paging query
    /// and nobody threaded the budget into it. That is the order every SQL
    /// adapter in this workspace will be written in, and the backwards branch is
    /// left correct in it — so `read_backwards_from_with_limit` passes and this
    /// is the only rule that sees it. The caller it protects is a projection
    /// runner asking for five hundred events from its checkpoint and being handed
    /// the whole stream, with no error anywhere.
    ///
    /// # Why the query has two items, and why there is no `to` here
    ///
    /// Two items because CF-12's finding is about the *generated SQL*, where the
    /// cursor and the disjunction are built by different hands:
    /// `UnparenthesisedPredicateStore` binds the bound to the last item alone and
    /// fails here for that reason, and `LimitPerItemStore` writes the budget onto
    /// one statement per item and fails here for its own. A single-item query
    /// would see neither.
    ///
    /// No `to` here, and the reason given for that was **half right and was
    /// acted on as though it were whole**. `from` cuts the front of the read
    /// while `to` and `limit` both cut the back, so in read order the two
    /// commute exactly — that much is true, and it is why the wrong
    /// implementation an adversarial review proposed for the pair (the budget
    /// applied before the bound) is not a defect at all and is not registered.
    ///
    /// What it does not follow is that no rule was owed. Commuting is an
    /// argument about the *order* two options are applied in;
    /// `WindowedPagingBudgetStore` does not reorder them, it **drops** the budget
    /// because a bound is present, and no amount of commutation reaches that. It
    /// passed all ninety-two rules that preceded `read_to_composes_with_limit`,
    /// which is the rule that owns the pair now.
    ///
    /// The two items' matches are contiguous blocks rather than interleaved, for
    /// `limit_applies_across_items_not_per_item`'s reason: interleaving makes
    /// item order and position order disagree, and `ItemOrderedUnionStore` and
    /// `SortByEventTypeStore` would then fail this rule for reasons
    /// `query_item_order_does_not_change_the_result_set` and
    /// `read_defaults_to_ascending_order` already own. The window the assertion
    /// names **straddles the boundary between the blocks**, which is what makes
    /// it a statement about the merged result rather than about either item: a
    /// store that pages each item separately cannot produce it.
    ///
    /// The assertion names three positions the store itself assigned, so a store
    /// that returns nothing fails it and so does one that returns everything.
    /// That is the non-vacuity anchor, and it is why there is no separate one.
    pub async fn read_from_composes_with_limit<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // One distinct tag each, and every tag distinct, for
        // `limit_applies_across_items_not_per_item`'s three reasons: untagged
        // events are invisible to `InnerJoinTagStore`, two tags on one event make
        // `TagJoinFanOutStore` return it twice, and byte-identical events are one
        // event to `PayloadDedupStore`. The types ascend across the blocks so
        // that `SortByEventTypeStore`'s sort is the identity here and it fails
        // the rule that owns it instead.
        append_ok(&store, &[tagged_event("Left", &[("page", "l1")])]).await;
        let left_mid = append_ok(&store, &[tagged_event("Left", &[("page", "l2")])]).await;
        let left_high = append_ok(&store, &[tagged_event("Left", &[("page", "l3")])]).await;
        let right_low = append_ok(&store, &[tagged_event("Right", &[("page", "r1")])]).await;
        append_ok(&store, &[tagged_event("Right", &[("page", "r2")])]).await;
        append_ok(&store, &[tagged_event("Right", &[("page", "r3")])]).await;

        let query = query_of_items([item_of_types(&["Left"]), item_of_types(&["Right"])]);

        let page = read_ok(&store, &query, ReadOptions::new().from(left_mid).limit(3)).await;
        assert_eq!(
            positions_of(&page),
            [left_mid.get(), left_high.get(), right_low.get()],
            "a forward read composing `from` with `limit` must yield the first \
             THREE events at or above the cursor, across the whole merged result. \
             An adapter that branches on `from` for its resume path and never \
             threads the budget into that branch hands back the entire tail: the \
             caller asked for a page, got the log, and nothing errored"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Read options — the upper bound (ES-16, VT-29)
    // ---------------------------------------------------------------------

    /// `ReadOptions::to` includes the event at that position.
    ///
    /// ES-16 and VT-29. The upper bound is **inclusive**, and both wrong
    /// implementations ES-16's `Rejects:` names are here: an adapter that accepts
    /// `ReadOptions` by value, matches the fields it knows and ignores the rest —
    /// `ToBoundIgnoredStore`, which returns the whole log and turns a bounded
    /// backfill into an unbounded one with no error anywhere — and an adapter
    /// that reads `to` as exclusive — `ToIsExclusiveStore`, which yields a window
    /// one event short at every chunk boundary and is invisible until the chunks
    /// are reassembled.
    ///
    /// The events are **tagged** and their types **ascend**, and neither is
    /// decoration. Untagged events would let `InnerJoinTagStore` fail this rule
    /// for a reason `untagged_events_match_query_all` already owns; descending
    /// types would let `SortByEventTypeStore` fail it for a reason
    /// `read_defaults_to_ascending_order` already owns. Neither would be
    /// coverage; both would be inflation.
    ///
    /// The assertion names two specific events the store assigned, so a store
    /// returning nothing fails it. That is the non-vacuity anchor, and it is why
    /// there is no separate one.
    pub async fn read_to_is_inclusive<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let first = append_ok(&store, &[tagged_event("Ay", &[("window", "closed")])]).await;
        let second = append_ok(&store, &[tagged_event("Bee", &[("window", "closed")])]).await;
        append_ok(&store, &[tagged_event("Cee", &[("window", "closed")])]).await;

        let bounded = read_ok(&store, &Query::all(), ReadOptions::new().to(second)).await;
        assert_eq!(
            positions_of(&bounded),
            [first.get(), second.get()],
            "`to` is an INCLUSIVE upper bound: the event at that position must be \
             the last one returned, and nothing above it may be"
        );

        RuleOutcome::Ran
    }

    /// `from` and `to` together name a closed window, and nothing outside it is
    /// yielded.
    ///
    /// ES-16 and VT-29's headline capability: a backfill worker owning `[1, H]`
    /// while a tail worker owns everything above it. Neither bound alone can
    /// express that, which is why this is its own rule rather than a second
    /// assertion in `read_to_is_inclusive`.
    ///
    /// It rejects three stores, and the third is why the window has a *lower*
    /// bound at all. `ToBoundIgnoredStore` returns everything from the lower
    /// bound to the end; `ToIsExclusiveStore` drops the window's top event; and
    /// `FromIsAnOffsetStore` — `from` bound to the `OFFSET` already in the paging
    /// query — slides the window by the lower bound's numeric *value*, which is a
    /// different window entirely on any store whose positions do not start at
    /// one.
    ///
    /// The window is the middle three of five, so a store that got either end
    /// wrong returns a different set rather than merely a longer one.
    pub async fn read_from_and_to_bound_a_closed_window<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(&store, &[tagged_event("Ay", &[("window", "closed")])]).await;
        let lower = append_ok(&store, &[tagged_event("Bee", &[("window", "closed")])]).await;
        let middle = append_ok(&store, &[tagged_event("Cee", &[("window", "closed")])]).await;
        let upper = append_ok(&store, &[tagged_event("Dee", &[("window", "closed")])]).await;
        append_ok(&store, &[tagged_event("Ee", &[("window", "closed")])]).await;

        let window = read_ok(
            &store,
            &Query::all(),
            ReadOptions::new().from(lower).to(upper),
        )
        .await;
        assert_eq!(
            positions_of(&window),
            [lower.get(), middle.get(), upper.get()],
            "`from` and `to` are both inclusive and both bound the same read: a \
             closed window must yield exactly the events between them, ends \
             included, and nothing on either side"
        );

        RuleOutcome::Ran
    }

    /// Under `backwards`, `to` bounds the **older** end.
    ///
    /// ES-16 and VT-29: `from` remains the starting (higher) bound and `to` the
    /// stopping (lower) one — the two swap roles in position order, not in
    /// meaning. `from` is deliberately left unset so that this rule turns on the
    /// swap alone; `read_backwards_from_with_limit` already owns the other half.
    ///
    /// It rejects `BackwardsToIsAnUpperBoundStore`, which is `WHERE position <= ?`
    /// copied verbatim into the backwards branch. ES-8's `Rejects:` describes
    /// exactly that shape for `from`; this is the same mistake one bound over.
    /// That store is **correct reading forwards**, so it passes both rules above
    /// and this is the only thing in the suite that sees it —
    /// which is what earns this rule its place beside them.
    /// `ToBoundIgnoredStore`, `ToIsExclusiveStore` and `BackwardsIgnoredStore`
    /// fail it too, from three other directions.
    pub async fn read_to_under_backwards_bounds_the_older_end<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(&store, &[tagged_event("Ay", &[("window", "closed")])]).await;
        append_ok(&store, &[tagged_event("Bee", &[("window", "closed")])]).await;
        let stop = append_ok(&store, &[tagged_event("Cee", &[("window", "closed")])]).await;
        let middle = append_ok(&store, &[tagged_event("Dee", &[("window", "closed")])]).await;
        let newest = append_ok(&store, &[tagged_event("Ee", &[("window", "closed")])]).await;

        let found = read_ok(
            &store,
            &Query::all(),
            ReadOptions::new().to(stop).backwards(),
        )
        .await;
        assert_eq!(
            positions_of(&found),
            [newest.get(), middle.get(), stop.get()],
            "under `backwards`, `to` is the STOPPING bound and therefore the \
             older end: the read starts at the newest event and stops at the one \
             `to` names, inclusive. An adapter that reads `to` as an upper bound \
             in position order irrespective of direction returns the wrong end of \
             the log entirely — and it is correct reading forwards, so nothing \
             else in the suite sees it"
        );

        RuleOutcome::Ran
    }

    /// The upper bound composes with a **multi-item** query.
    ///
    /// CF-12's shape, one bound over. `to` commutes with filtering semantically
    /// and not in generated SQL: `WHERE a OR b AND position <= ?` conjoins the
    /// window's top with the *last* disjunct alone, so every event matching an
    /// earlier item comes back regardless of where the window ends. The caller
    /// it protects is a bounded backfill worker owning `[1, H]` while a tail
    /// worker owns everything above it — the backfill reads past its own window
    /// and re-delivers events the tail worker has already processed, with no
    /// error anywhere.
    ///
    /// It rejects `UnparenthesisedToPredicateStore`, and that store is the
    /// reason this rule exists rather than an illustration of it. It is
    /// `UnparenthesisedPredicateStore`'s twin with the *lower* bound conjoined
    /// correctly, so `read_from_composes_with_multi_item_query` passes it; the
    /// three `to` rules above all issue `Query::all()`, where there is nothing
    /// for the `OR` to bind wrongly across, so they pass it too. Until this rule
    /// the only thing in the tree that could see it was the **model** family,
    /// which is `proptest`-gated and `cfg(not(target_arch = "wasm32"))` — so a
    /// Cloudflare or Neon adapter carrying the bug passed every rule it actually
    /// runs. It was registered as a model-only mutant for exactly that reason,
    /// and it is an ordinary one now.
    ///
    /// # Why the read is forwards, unbudgeted, and one assertion
    ///
    /// A backwards read here would also reject
    /// `BackwardsToIsAnUpperBoundStore` and `BackwardsIgnoredStore`, which
    /// `read_to_under_backwards_bounds_the_older_end` and
    /// `read_backwards_reverses_order` already own, and a budget would take
    /// `read_to_composes_with_limit`'s. Neither would be coverage; both would be
    /// inflation. The axis this rule owns is the query shape.
    ///
    /// # What the log and the query are shaped around
    ///
    /// Five decisions, each of them a mutant this rule must **not** reject for a
    /// reason another rule already owns. The suite has measured every one of
    /// them: an earlier draft of this rule used a three-type item and
    /// `TypesAreAndStore` failed it, which is
    /// `query_item_types_are_or`'s finding arriving here under a different name.
    ///
    /// - Every event is **tagged**, so `InnerJoinTagStore` cannot fail this rule
    ///   for `untagged_events_match_query_all`'s reason.
    /// - Every event carries exactly **one** tag, so `TagJoinFanOutStore` has no
    ///   second tag to return it twice over.
    /// - The types **ascend** with position, so `SortByEventTypeStore`'s sort is
    ///   the identity here and `read_defaults_to_ascending_order` keeps it.
    /// - The first item is a **single-tag** item and the second a **single-type**
    ///   one. A multi-type item would let `TypesAreAndStore` fail this rule; two
    ///   tag items would carry the same (empty) type list and let
    ///   `ItemDedupByTypeStore` intern one into the other, which is
    ///   `query_union_is_item_concatenation`'s.
    /// - The two items' matches **inside the window** are contiguous — the tag
    ///   item's two, then the type item's one — for
    ///   `read_from_composes_with_limit`'s reason: interleaving them would make
    ///   item order and position order disagree inside the window and
    ///   `ItemOrderedUnionStore` would fail this rule for a reason
    ///   `query_item_order_does_not_change_the_result_set` owns.
    ///
    /// What makes the assertion a statement about the *merged* result is
    /// therefore not the interleaving but the **leak**: the event above the
    /// bound matches the **first** item, which is the one a dangling upper bound
    /// never reaches.
    ///
    /// The assertion names three positions the store itself assigned, so a store
    /// that returns nothing fails it and so does one that returns everything.
    /// That is the non-vacuity anchor, and it is why there is no separate one.
    pub async fn read_to_composes_with_multi_item_query<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Matched by the tag item, the tag item, the type item, nothing, and
        // the tag item. The window ends at the third; the fifth is the one an
        // unparenthesised upper bound lets through, because it matches the item
        // the bound never reaches.
        let by_tag_low = append_ok(&store, &[tagged_event("Ay", &[("side", "left")])]).await;
        let by_tag_high = append_ok(&store, &[tagged_event("Bee", &[("side", "left")])]).await;
        let stop = append_ok(&store, &[tagged_event("Cee", &[("edge", "top")])]).await;
        append_ok(&store, &[tagged_event("Dee", &[("edge", "past")])]).await;
        append_ok(&store, &[tagged_event("Ee", &[("side", "left")])]).await;

        let query = query_of_items([item_tagged(&[("side", "left")]), item_of_types(&["Cee"])]);

        let window = read_ok(&store, &query, ReadOptions::new().to(stop)).await;
        assert_eq!(
            positions_of(&window),
            [by_tag_low.get(), by_tag_high.get(), stop.get()],
            "`to` must bound EVERY item of the query. An unparenthesised \
             `WHERE a OR b AND position <= ?` conjoins the window's top with \
             the last disjunct alone, and the first item's matches come back \
             from above the window every time: a bounded backfill re-delivers \
             events the tail worker has already processed, with no error \
             anywhere"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Read options — the budget (VT-28, ES-14)
    // ---------------------------------------------------------------------

    /// `limit(0)` yields nothing.
    ///
    /// VT-28, and **phase 4 is the phase that can express it**: until `limit`
    /// became `Option<usize>` the builder stored `NonZeroUsize::new(limit)`, so
    /// zero became `None` — unlimited — before any adapter saw it, and the input
    /// this rule needs was not representable through the API.
    ///
    /// The caller it protects is the one writing `.limit(budget - fetched)`. Under
    /// the wrong implementation a paging loop that reaches parity does not read
    /// zero events; it reads **the entire log, unbounded, silently** — no error,
    /// no rejection, and a memory ceiling that stops being a ceiling.
    ///
    /// It rejects `LimitZeroIsUnlimitedStore`, which is the DCB reference
    /// implementation's `if (limit)` falsiness ported to a language where `0` is
    /// not falsy, and `FetchOneExtraStore`, whose `LIMIT n + 1` cursor probe
    /// hands back one event when it was asked for none.
    ///
    /// The `limit(1)` read at the end is the non-vacuity anchor, and this rule
    /// cannot do without it: every assertion above it is `is_empty()`, which a
    /// store returning nothing at all satisfies for free.
    pub async fn read_limit_zero_yields_nothing<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // One distinct tag each, for the reason
        // `limit_applies_across_items_not_per_item` states in full: untagged
        // events, two-tag events and byte-identical events each break a different
        // registered mutant for a reason that is not this rule's.
        let first = append_ok(&store, &[tagged_event("Budget", &[("run", "z1")])]).await;
        append_ok(&store, &[tagged_event("Budget", &[("run", "z2")])]).await;
        append_ok(&store, &[tagged_event("Budget", &[("run", "z3")])]).await;

        let none = read_ok(&store, &Query::all(), ReadOptions::new().limit(0)).await;
        assert!(
            none.is_empty(),
            "`limit(0)` must yield nothing. A budget of zero read as \"unlimited\" \
             turns a paging loop that has spent its budget into a full scan of the \
             log, silently; got {:?}",
            positions_of(&none)
        );

        // The same question through the filtering, backwards path, because that
        // is the path a paging loop actually takes and a store may spend its
        // budget somewhere else there.
        let none_backwards = read_ok(
            &store,
            &query_of_types(&["Budget"]),
            ReadOptions::new().backwards().limit(0),
        )
        .await;
        assert!(
            none_backwards.is_empty(),
            "and it must yield nothing reading backwards through a filtering \
             query too; got {:?}",
            positions_of(&none_backwards)
        );

        // The anchor. A store returning nothing whatever it was asked for passes
        // both assertions above and is not conformant.
        let one = read_ok(&store, &Query::all(), ReadOptions::new().limit(1)).await;
        assert_eq!(
            positions_of(&one),
            [first.get()],
            "and `limit(1)` must still yield exactly the first event — otherwise a \
             store that answered every read with nothing would pass a rule about \
             a budget of zero"
        );

        RuleOutcome::Ran
    }

    /// `limit` is a budget for the whole result, not one per query item.
    ///
    /// ES-14: `limit(n)` MUST yield the first *n* events of the ordered result
    /// set, not *n* per query item. The two items match three events each and the
    /// budget is four, so a store applying it per item returns six.
    ///
    /// `LimitPerItemStore` is that store — one statement per `QueryItem`, each
    /// carrying `LIMIT n`, unioned client-side with the budget never re-applied
    /// to the merged result. It is the same adapter shape ES-12 rejects, failing
    /// here for an independent reason, which is why both rules are worth having.
    ///
    /// **The two items' matches are contiguous blocks rather than interleaved,
    /// and that is deliberate.** Interleaving would make item order and position
    /// order disagree, so `ItemOrderedUnionStore` and `SortByEventTypeStore`
    /// would both fail this rule for reasons
    /// `query_item_order_does_not_change_the_result_set` and
    /// `read_defaults_to_ascending_order` already own. Blocks keep this rule
    /// about the budget.
    ///
    /// The backwards mirror is here for `read_limit_applies_after_filtering`'s
    /// reason: a store truncates at whichever end it scans from, and a per-item
    /// budget is written into the same clause either way.
    pub async fn limit_applies_across_items_not_per_item<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Exactly one tag per event, and every tag distinct. Both halves are
        // load bearing and they pull against each other. Untagged events are
        // invisible to `InnerJoinTagStore`; **two** tags on one event make
        // `TagJoinFanOutStore` return it twice; and three byte-identical events
        // are one event to `PayloadDedupStore`. One distinct tag each is the only
        // seeding that avoids all three, and it costs nothing here because the
        // query below selects on **type**.
        let left_low = append_ok(&store, &[tagged_event("Left", &[("half", "l1")])]).await;
        let left_mid = append_ok(&store, &[tagged_event("Left", &[("half", "l2")])]).await;
        let left_high = append_ok(&store, &[tagged_event("Left", &[("half", "l3")])]).await;
        let right_low = append_ok(&store, &[tagged_event("Right", &[("half", "r1")])]).await;
        let right_mid = append_ok(&store, &[tagged_event("Right", &[("half", "r2")])]).await;
        let right_high = append_ok(&store, &[tagged_event("Right", &[("half", "r3")])]).await;

        let query = query_of_items([item_of_types(&["Left"]), item_of_types(&["Right"])]);

        let budgeted = read_ok(&store, &query, ReadOptions::new().limit(4)).await;
        assert_eq!(
            positions_of(&budgeted),
            [
                left_low.get(),
                left_mid.get(),
                left_high.get(),
                right_low.get()
            ],
            "`limit(4)` over a two-item query must yield the first four events of \
             the MERGED result, not four per item. A store that writes the budget \
             into one statement per item hands a caller who asked for four a page \
             it did not size — and the caller's own paging arithmetic is built on \
             the number it asked for"
        );

        let newest = read_ok(&store, &query, ReadOptions::new().backwards().limit(4)).await;
        assert_eq!(
            positions_of(&newest),
            [
                right_high.get(),
                right_mid.get(),
                right_low.get(),
                left_high.get()
            ],
            "and the same reading backwards, because a store truncates at \
             whichever end it scans from"
        );

        RuleOutcome::Ran
    }

    /// A window and a budget on the same read, with the **budget** the smaller.
    ///
    /// ES-16 and ES-14 together, and it is the last unread pair of read options:
    /// until this rule the suite composed `to` with `from`, with `backwards` and
    /// (since `read_to_composes_with_multi_item_query`) with a filtering query,
    /// and issued `to` beside a `limit` at no call site at all.
    ///
    /// It rejects `WindowedPagingBudgetStore`: the budget threaded into the
    /// paging statement and not into the windowed one, because a closed window is
    /// a different statement from a page and the paging clause was already on the
    /// other one. That store is `ForwardPagingBudgetStore` one read option over,
    /// and **it passed all ninety-two rules that preceded this one** — measured,
    /// with an empty `fails` list, rather than argued. The caller it breaks is the
    /// backfill worker ES-16 exists for, writing `.limit(budget - fetched)`
    /// against the window it was given and being handed the whole window instead:
    /// a batch nobody sized, and arithmetic about a number the store ignored.
    ///
    /// # Why the budget is smaller than the window, and why there is one read
    ///
    /// Because the other arrangement has no wrong implementation to name. With a
    /// budget *larger* than the window the answer is the one `to` alone
    /// determines, so `ToBoundIgnoredStore` and `ToIsExclusiveStore` would fail
    /// here for `read_to_is_inclusive`'s reason and nothing else would fail at
    /// all. With the budget smaller, both of those stores answer **correctly** —
    /// the budget masks the bound — which is precisely why neither the bound's
    /// rules nor the budget's can see the store this rule is for.
    ///
    /// # `to` and `limit` commute, and that is why this rule is shaped as it is
    ///
    /// `read_from_composes_with_limit`'s documentation used the commutation to
    /// argue that **no** rule was owed here. The commutation is real: in read
    /// order `from` cuts the front while `to` and `limit` both cut the back, and
    /// two prefix operations compose to the shorter prefix whichever order they
    /// run in. So the wrong implementation an adversarial review proposed — the
    /// row budget applied *before* the upper bound,
    /// `.take(n).take_while(|e| e.position <= to)` — answers every read exactly
    /// as the reference implementation does, forwards and backwards alike. It is
    /// not a defect, and it is deliberately not registered.
    ///
    /// What the argument missed is that commuting is not the only way two options
    /// interact. `WindowedPagingBudgetStore` does not reorder them; it **drops**
    /// one because the other is present, which no amount of commutation reaches.
    /// Order and applicability are different questions and only the first was
    /// answered.
    ///
    /// The read is forwards. The backwards composition stays with
    /// `read_to_under_backwards_bounds_the_older_end` and
    /// `read_backwards_from_with_limit`: the only wrong implementation that needs
    /// backwards *and* a window *and* a budget is a store whose windowed
    /// statement is written ascending with the `LIMIT` inside it and the
    /// direction applied by an outer `ORDER BY` — a narrower hypothesis, needing
    /// two statements and one specific SQL shape, and asserting it here would
    /// make `BackwardsToIsAnUpperBoundStore`, `BackwardsIgnoredStore` and
    /// `FetchOneExtraStore` fail this rule for reasons three other rules own. It
    /// is recorded as an extension rather than taken.
    ///
    /// Every event is tagged, carries one distinct tag, and the types ascend, for
    /// `limit_applies_across_items_not_per_item`'s reasons: `InnerJoinTagStore`
    /// needs a tag to join to, `TagJoinFanOutStore` needs a second tag to fan out
    /// over, `PayloadDedupStore` needs two byte-identical events, and
    /// `SortByEventTypeStore`'s sort has to be the identity here or
    /// `read_defaults_to_ascending_order` loses its store.
    ///
    /// The assertion names two positions the store itself assigned, so a store
    /// that returns nothing fails it and so does one that returns the window.
    /// That is the non-vacuity anchor.
    pub async fn read_to_composes_with_limit<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let first = append_ok(&store, &[tagged_event("Ay", &[("chunk", "c1")])]).await;
        let second = append_ok(&store, &[tagged_event("Bee", &[("chunk", "c2")])]).await;
        append_ok(&store, &[tagged_event("Cee", &[("chunk", "c3")])]).await;
        // The window's top: three events above its floor and one below the head,
        // so the budget is smaller than the window and the window is smaller than
        // the log. Neither bound is left doing all the work.
        let window_top = append_ok(&store, &[tagged_event("Dee", &[("chunk", "c4")])]).await;
        append_ok(&store, &[tagged_event("Ee", &[("chunk", "c5")])]).await;

        let page = read_ok(
            &store,
            &Query::all(),
            ReadOptions::new().to(window_top).limit(2),
        )
        .await;
        assert_eq!(
            positions_of(&page),
            [first.get(), second.get()],
            "a window and a budget bound the SAME read, and the smaller of them \
             is what the caller gets. An adapter that answers a closed window \
             with its own statement and threads the row budget only into the \
             paged one hands back the whole window: the backfill worker asked \
             for a page of its window and got the window, with no error anywhere"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Read options — `from` over a gap (ES-9)
    // ---------------------------------------------------------------------

    /// `from` is a range predicate over assigned positions, not a seek.
    ///
    /// ES-9. Reading from a position **nothing occupies** must yield the next
    /// matching event above it, or below it reading backwards, and must neither
    /// error nor come back empty on that ground alone.
    ///
    /// # Where the gap comes from, and why it is the one above the head
    ///
    /// The port has no delete, and a store allocating densely leaves no interior
    /// gap for a rule to aim at — so a rule that needed one could only run
    /// against an adapter that is already sparse, which is not the adapter under
    /// test. The position **immediately above the head** is unoccupied on *every*
    /// store, whatever it allocates and wherever it starts, because nothing has
    /// been appended since. Reading backwards from it is therefore the portable
    /// half of ES-9, and it is the half no existing rule reaches: **six** rules
    /// pass `from` — `read_from_is_inclusive`, `read_backwards_from_with_limit`,
    /// `read_from_composes_with_multi_item_query`,
    /// `read_from_composes_with_limit`, `read_from_and_to_bound_a_closed_window`
    /// and this one — and every one of the other five hands it a position the
    /// store actually assigned. This sentence read *"the only two rules that pass
    /// `from` at all"* until phase 12's pre-publication review counted them; the
    /// count is written out here rather than left implicit because it is the
    /// claim that decayed.
    ///
    /// The interior half runs only where the fixture's own allocator left a hole
    /// — `GappedPositionStore`, and any adapter allocating from a sequence with
    /// `CACHE 7`, from a transaction id, or with a shard id in the low bits. On a
    /// dense store the branch is skipped rather than faked, which is the honest
    /// answer: the assertion above it is what every store owes.
    ///
    /// It rejects `FromIsAnOffsetStore` — `from` bound to the `OFFSET` already in
    /// the paging query, which skips by a *count* and so yields nothing at all
    /// from a position above the head — and `BackwardsIgnoredStore`, whose
    /// backwards branch never reaches the SQL, so the same predicate becomes a
    /// floor and the read comes back empty.
    pub async fn read_from_a_gap_position<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let first = append_ok(&store, &[tagged_event("Ay", &[("run", "gap")])]).await;
        let second = append_ok(&store, &[tagged_event("Bee", &[("run", "gap")])]).await;
        let third = append_ok(&store, &[tagged_event("Cee", &[("run", "gap")])]).await;

        // The one unoccupied position every store has. `next()` is fallible
        // because it signals key-space exhaustion (VT-13) and a rule may not
        // unwrap — but a fixture whose head is the last representable position is
        // not reachable through the port, so this is a fixture fault and says so.
        let Some(beyond) = third.next() else {
            panic!(
                "this fixture's head is the last representable position, so there \
                 is no unoccupied position above it for this rule to read from"
            );
        };

        let above = read_ok(&store, &Query::all(), ReadOptions::new().from(beyond)).await;
        assert!(
            above.is_empty(),
            "reading forwards from a position above the head must yield nothing, \
             and must not error; got {:?}",
            positions_of(&above)
        );

        let below = read_ok(
            &store,
            &Query::all(),
            ReadOptions::new().from(beyond).backwards(),
        )
        .await;
        assert_eq!(
            positions_of(&below),
            [third.get(), second.get(), first.get()],
            "`from` names a position, not an index: reading backwards from a \
             position NOTHING OCCUPIES must yield the next matching event below \
             it. An equality seek or a `rowid` offset returns empty here, and a \
             projection resuming across a gap then stalls forever with no error \
             anywhere"
        );

        // The interior half, where the fixture's own allocator supplies the hole.
        // Skipped rather than faked on a dense store: the port has no way to make
        // one, and pretending otherwise would be a rule that tested nothing.
        if let Some(interior) = first.next().filter(|candidate| *candidate < second) {
            let higher = read_ok(&store, &Query::all(), ReadOptions::new().from(interior)).await;
            assert_eq!(
                positions_of(&higher),
                [second.get(), third.get()],
                "forwards from a position inside a gap must yield the next \
                 matching event ABOVE it"
            );

            let lower = read_ok(
                &store,
                &Query::all(),
                ReadOptions::new().from(interior).backwards(),
            )
            .await;
            assert_eq!(
                positions_of(&lower),
                [first.get()],
                "and backwards from the same position must yield the one below it"
            );
        }

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
    // Head (ES-30)
    //
    // Its one helper is below the section heading rather than with the others at
    // the top of the module, because what it encodes is a *decision* — ADR-0013 §4
    // on what `head()` may be asserted to equal — and that decision reads better
    // beside the three rules that rest on it.
    // ---------------------------------------------------------------------

    /// ES-30's portable relation: `head()` is not below the highest position a
    /// read of the whole store through the same handle just yielded.
    ///
    /// # Why this is a bound and not an equality against what `append` returned
    ///
    /// ADR-0013 §4 decides it, and the reason is that the equality form rejects
    /// a *conformant* adapter. An adapter buying ES-10's visibility invariant
    /// with `xid8` + `pg_snapshot_xmin` — the only arm phase 2 measured that
    /// both holds the invariant and leaves writers unserialised — makes
    /// visibility a **predicate** rather than an identity: a row is visible when
    /// its writing transaction's `xid8` is below `pg_snapshot_xmin`, however low
    /// its *position* is. `head()` on such a store reports that **frontier**, so
    /// `append` returning `Ok(P)` does not promise the next `head()` is at or
    /// above `P`. Read-your-own-writes does not hold, staleness is bounded by
    /// the longest open write transaction anywhere in the cluster — 0.688 ms
    /// unloaded and 4010.719 ms behind an unrelated five-second write in an
    /// unrelated database, both measured — and CF-33 forbids this rule from
    /// waiting for it to pass.
    ///
    /// The frontier is not a softening of ES-30's MUST. "The highest position
    /// currently **visible**" is the clause's own wording, and where visibility
    /// is a predicate the frontier is what those words denote. A `head()`
    /// answering `max(position)` there names a position no read will yield,
    /// which is the pagination failure ES-11 describes.
    ///
    /// # The order of the two calls is load bearing
    ///
    /// The read runs **first** and the head **second**. Visibility only grows,
    /// so a head sampled after a read cannot legitimately be below what that
    /// read saw; sampling the head first and reading afterwards would make a
    /// conformant store fail whenever anything became visible in between.
    ///
    /// # It cannot anchor itself
    ///
    /// A store whose reads yield nothing satisfies the relation for free, so the
    /// `None` arm below asserts nothing at all. That is deliberate: the caller
    /// gets the read back and anchors on it in the terms of its own arrangement,
    /// which is what makes each of the three rules' failure messages name the
    /// thing that rule set up.
    async fn head_not_below_the_visible_log<S: EventStore>(store: &S) -> Vec<SequencedEvent> {
        let visible = read_ok(store, &Query::all(), ReadOptions::new()).await;
        let highest = visible.iter().map(|event| event.position).max();
        let head = head_ok(store).await;

        if let Some(highest) = highest {
            assert!(
                head.is_some_and(|head| head >= highest),
                "ES-30: `head()` must be the highest position currently visible, \
                 and a read of the whole store through this handle had just \
                 yielded {highest} — so a head of {head:?} names a store that \
                 has already shown a caller more than it admits to holding. A \
                 consumer comparing its checkpoint against this head is told it \
                 is ahead of the log it is reading, and stops."
            );
        }

        visible
    }

    /// A store holding nothing reports no head.
    ///
    /// ES-30. `head()` returns `Option<SequencePosition>` precisely so that the
    /// empty store has an answer that is not a position, and this is the rule
    /// that makes the `None` mean what the signature says.
    ///
    /// # What it rejects
    ///
    /// `EmptyHeadIsFirstStore`: `SELECT IFNULL(MAX(position), 0)` decoded into a
    /// non-nullable integer, then turned back into a `SequencePosition` with
    /// `SequencePosition::new(value).unwrap_or(SequencePosition::FIRST)` —
    /// because `MAX` over no rows is `NULL`, because the driver's scalar decode
    /// wants a non-nullable column type, and because this workspace's own house
    /// rule forbids `unwrap` in library code and `unwrap_or` is the shortest
    /// thing that satisfies it. A `NonZero` newtype makes the fallback look
    /// forced. The store then reports a position no event occupies on the one
    /// state every adapter is in on its first run, and the paginating adapter
    /// ES-11 prescribes bounds its first window by it.
    ///
    /// # The anchor, and why it is not `head() == append`
    ///
    /// Without the second half, a store whose `head` is `Ok(None)`
    /// unconditionally passes this rule forever — and that store also passes
    /// ES-31's "am I caught up?" comparison by never being caught up. The anchor
    /// is the portable relation rather than an equality against what `append`
    /// returned, for the reason `head_not_below_the_visible_log` documents.
    pub async fn head_of_an_empty_store_is_none<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let empty = head_ok(&store).await;
        assert!(
            empty.is_none(),
            "a store holding no events has no highest visible position, so \
             `head()` must be `None` rather than a position no event occupies: \
             got {empty:?}"
        );

        // The event is tagged for the reason every value-edge rule's is: a store
        // whose reads drop untagged events (`InnerJoinTagStore`) would otherwise
        // fail the anchor rather than the property, and a *head* scoped to a tag
        // join (`DefaultQueryHeadStore`) would fail this rule instead of the one
        // written for it.
        let written = append_ok(&store, &[tagged_event("Seeded", &[("edge", "head")])]).await;

        let visible = head_not_below_the_visible_log(&store).await;
        assert!(
            visible.iter().any(|event| event.position == written),
            "the anchor: once an event has been appended the read above must \
             yield it, or `head()` is being judged against an empty log and a \
             store that answers `None` to everything passes this rule forever. \
             Positions visible: {:?}",
            positions_of(&visible)
        );

        RuleOutcome::Ran
    }

    /// `head()` covers the whole store, not the part some default query matches.
    ///
    /// ES-30, in the portable form ADR-0013 §4 amends the clause to: append a
    /// batch holding at least one event a narrower default query would not
    /// match, read the whole store with `Query::all()`, and assert `head()` is
    /// not below the highest position that read yielded. It deliberately does
    /// **not** assert read-your-own-writes;
    /// `head_not_below_the_visible_log` carries the whole argument for why.
    ///
    /// # The seeding is the rule
    ///
    /// A rule that appended one uniform batch could not fail: whatever query a
    /// scoped `head` is written against, one batch of identical events either
    /// matches it entirely or not at all, and in both cases the scoped answer
    /// and the true answer agree. So the **highest** event here is untagged and
    /// carries a second type, and that is what a scoped head has to miss.
    ///
    /// # What it rejects
    ///
    /// `DefaultQueryHeadStore`: `SELECT max(e.position) FROM event e JOIN tag t
    /// ON t.event_id = e.id` — the head statement written against the same
    /// joined view the read path is built around, because there is one view and
    /// reusing it is the obvious move. An untagged event at the top of the log
    /// is invisible to it, so the store under-reports its head to every
    /// projection runner and every paginating caller, and the events past the
    /// under-reported head are exactly the ones nothing else in the deployment
    /// tags.
    ///
    /// ES-30's other two rejected implementations are not this rule's and were
    /// never going to be. A cached last-written position returns the right
    /// answer against a single handle and is `head_advances_across_two_handles`'
    /// to reject; the `backwards().limit(1)` workaround as a universal answer is
    /// hung on E2E-13 by ES-30's own text.
    pub async fn head_is_the_highest_visible_position<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(&store, &[tagged_event("Tagged", &[("edge", "head")])]).await;
        // Untagged, of a second type, and appended last — the three things that
        // give a query-scoped head something to miss.
        let untagged = append_ok(&store, &[event("Untagged")]).await;

        let visible = head_not_below_the_visible_log(&store).await;
        assert!(
            visible.iter().any(|event| event.position == untagged),
            "the anchor: `Query::all()` matches untagged events (ES-13), so the \
             read this rule judges `head()` against must have yielded the \
             untagged one — otherwise a scoped `head` is being compared with an \
             equally scoped read and the two agree for the wrong reason. \
             Positions visible: {:?}",
            positions_of(&visible)
        );

        RuleOutcome::Ran
    }

    /// A head read through one handle covers what another handle wrote.
    ///
    /// ES-30's third rule, and the one that needs ES-33's fixture. It is
    /// `two_handles_observe_each_others_appends` asked of the *head* rather than
    /// of the read and the condition probe: the same shared consistency
    /// boundary, reached through the one method whose answer an adapter is most
    /// tempted to keep in a field.
    ///
    /// # What it rejects
    ///
    /// `LastWrittenHeadStore`: a handle that remembers the position its own last
    /// `append` returned and answers `head()` from it. It is the "cached
    /// last-written position" ES-30's `Rejects:` names, and it is the rule's
    /// alone — against a single handle that cache is exactly right, so it passes
    /// both of the other head rules and every other rule in this suite. Whether
    /// the cache is a field, a session variable or a `currval()` is an
    /// implementation detail; what makes it wrong is that a store is not a
    /// connection, and every deployment with a pool reaches one store two ways.
    ///
    /// # Two details that would make it pass for the wrong reason
    ///
    /// The observer connects **before** the write, because a handle that samples
    /// the head when it opens would sample the right answer afterwards. And the
    /// assertion is the portable relation rather than an equality against the
    /// position `append` returned, for
    /// `head_not_below_the_visible_log`'s reason: this rule is about a stale
    /// handle, not about how fresh a frontier is.
    ///
    /// The anchor rests on ES-33 and ES-34, both `[FROZEN]`: two handles are two
    /// handles onto **one** backing store, and an append through one is visible
    /// through the other. If the read below comes back short, the finding is
    /// there and not here.
    pub async fn head_advances_across_two_handles<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // `must!`, not `require!`: CF-16 makes a second handle an obligation on
        // every fixture rather than a trade the suite may record as a skip.
        must!(F: SECOND_HANDLE);

        let fixture = open().await;
        let writer = fixture.connect().await;
        let observer = fixture.connect().await;

        // The types **ascend**, and it is not decoration. `SortByEventTypeStore`
        // orders by `(type, position)`, so a descending pair would make it fail
        // this rule at the anchor for a reason `read_defaults_to_ascending_order`
        // already owns — inflation rather than coverage. Every event is tagged
        // for `InnerJoinTagStore`'s sake, one rule over.
        let low = append_ok(&writer, &[tagged_event("CaseOpened", &[("case", "head")])]).await;
        let high = append_ok(&writer, &[tagged_event("CaseSettled", &[("case", "head")])]).await;

        let visible = head_not_below_the_visible_log(&observer).await;
        assert_eq!(
            positions_of(&visible),
            [low.get(), high.get()],
            "the anchor: ES-33 makes the second handle a handle onto the *same* \
             backing store and ES-34 makes the writer's committed appends \
             visible through it, so both must come back here before anything \
             this rule says about `head()` is a statement about a shared log"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // Identity, recorded time and membership
    //
    // Every fact this group is about is assigned by the **store**; the caller
    // supplies none of them. That makes the group structurally different from
    // everything above it: there is no input to compare an output against, so
    // each assertion below is either a relationship between two store-assigned
    // facts, or a claim that one of them is *stable*.
    //
    // Two prohibitions bound what may be written here, and both are easy to
    // violate by writing the obvious stronger rule.
    //
    // **VT-9's third MUST** forbids the *contract* from stating any relationship
    // between `RecordedAt` order and `SequencePosition` order. The conformance
    // suite is the contract's executable form — an adapter author reads a
    // failing rule as a requirement — so a rule that asserts an order states
    // one. No rule below compares two events' recorded times in either
    // direction, or compares one against a position. A store on a machine whose
    // clock steps backwards under an NTP correction is conformant, and
    // `RUNBOOK.md`'s "a rule that it is non-decreasing with position" was struck
    // by ADR-0014 for exactly that reason.
    //
    // **CF-33** forbids reading a clock, so no rule below checks a recorded time
    // for plausibility against the harness's own either.
    //
    // One thing the group cannot do, stated once here rather than in six doc
    // comments: it can never name a `StoreId` *value*. `EventStore` has no
    // `store_id()` accessor and ES-19's prose says why one was refused, so the
    // only incarnation a rule may speak of is the one it read back off an event.
    // ---------------------------------------------------------------------

    /// VT-4 — the four facts on a `SequencedEvent` are **persisted**, not
    /// produced by the read that returned them.
    ///
    /// VT-4's `Rejects:` line names the adapter that makes identity or time up
    /// at read time, and observes that it "passes every rule that reads back
    /// what it just wrote inside one process". That is true of a rule that reads
    /// *once*, and it stops being true the moment one event is reached two
    /// different ways: an identity derived from the row's ordinal in the result
    /// set, or a time taken from the connection's clock in the row mapper, both
    /// answer differently when the result set is a different shape — while
    /// agreeing with themselves perfectly under any single query.
    ///
    /// So this reads one event through `Query::all()` and again through a query
    /// that selects it alone, and compares the whole `SequencedEvent`.
    /// `RowOrdinalIdentityStore` and `ReadTimeClockStore` in
    /// `tests/mutation_coverage/mutants.rs` are the two compiled versions, and
    /// the first of them is *right* under `Query::all()` on a densely-allocated
    /// store, which is what makes it survivable and what makes the second read
    /// the whole rule.
    ///
    /// **The middle event, not the first**, so that neither an ordinal-derived
    /// identity nor a first-row special case can coincide with the right answer.
    /// The types ascend on purpose: `SortByEventTypeStore` orders by `(type,
    /// position)` and would otherwise reorder the batch, which is
    /// `read_defaults_to_ascending_order`'s subject and not this one's.
    ///
    /// The *local shape* of the identity — that `id.position()` is the position
    /// this store assigned — belongs to `append_stamps_a_local_event_id`, and
    /// the time's own claim to `append_stamps_a_recorded_time`. Asserting either
    /// here as well would be one rule wearing three names.
    pub async fn append_stamps_identity_and_time<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[
                tagged_event("Ay", &[("seat", "s1")]),
                tagged_event("Bee", &[("seat", "s2")]),
                tagged_event("Cee", &[("seat", "s3")]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            3,
            "the anchor: all three events must come back before \"the same event \
             reached two ways\" means anything, got {:?}",
            types_of(&all)
        );
        let through_all = all[1].clone();

        let narrowed = read_ok(
            &store,
            &query_of(&["Bee"], &[("seat", "s2")]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            narrowed.len(),
            1,
            "the second anchor: the narrow query must select exactly the event it \
             was written for, or the comparison below holds because there is \
             nothing to compare. Got {:?}",
            types_of(&narrowed)
        );

        assert_eq!(
            narrowed[0], through_all,
            "every field of a `SequencedEvent` is a fact the store assigned at \
             append and persisted, so reaching one event two ways must produce \
             one value. An identity synthesised from the row's ordinal in the \
             result set, or a time read from the connection's clock in the row \
             mapper, agrees with itself under any single query and differs here"
        );

        RuleOutcome::Ran
    }

    /// VT-5 — a locally appended event's `EventId` is **this** store's
    /// incarnation paired with the position **this** store just assigned.
    ///
    /// Two assertions, rejecting two different stores.
    ///
    /// That `id.position()` equals `position` rejects any identity computed from
    /// something other than the assigned position: a content hash
    /// (`ContentHashIdentityStore`), or a batch-wide `RETURNING` value read once
    /// and applied to every row (`SharedBatchIdentityStore`). It is sound here
    /// and only here because every event this rule writes is a **local** append
    /// — for an event that arrived through ingest the two numbers are
    /// deliberately different, which is what `SequencedEvent`'s own
    /// documentation explains and why the field is not redundant.
    ///
    /// That every event of one open shares one `id.store()` rejects
    /// `PerEventStoreIdStore`, which mints a fresh incarnation per event. That
    /// store satisfies VT-6's literal MUST — no pair is ever reissued — while
    /// destroying everything the type is for: every event becomes its own
    /// origin, a peer's `Watermark` grows one row per event rather than one per
    /// incarnation, and VT-5's peer-independent sort degenerates to comparing
    /// 128 opaque bits.
    ///
    /// Two appends rather than one batch, because the incarnation is a property
    /// of the store and not of the call.
    pub async fn append_stamps_a_local_event_id<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Ay", &[("seat", "s1")])]).await;
        append_ok(
            &store,
            &[
                tagged_event("Bee", &[("seat", "s2")]),
                tagged_event("Cee", &[("seat", "s3")]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            3,
            "the anchor: both appends must have landed before anything is said \
             about what they were stamped with, got {:?}",
            types_of(&all)
        );

        for event in &all {
            assert_eq!(
                event.id.position(),
                event.position,
                "a locally appended event's identity is (this store's \
                 incarnation, the position this store assigned it), so \
                 `id.position()` and `position` are the same number here. They \
                 part company only for an event that arrived through ingest, \
                 which no rule in this suite can produce. Event at {} carries {}",
                event.position,
                event.id
            );
        }

        let incarnations: Vec<_> = all.iter().map(|event| event.id.store()).collect();
        assert!(
            incarnations.windows(2).all(|pair| pair[0] == pair[1]),
            "every event a store accepts in one open carries that store's own \
             incarnation. Minting a fresh `StoreId` per *event* never reissues a \
             pair and so satisfies VT-6's letter, and it makes every event its \
             own origin — a peer's watermark then grows one row per event and \
             VT-5's peer-independent sort has nothing left to sort by. Got \
             {incarnations:?}"
        );

        RuleOutcome::Ran
    }

    /// VT-5 and VT-8 — a store holds at most one event per `EventId`, and it is
    /// the store's job to make that true rather than the caller's.
    ///
    /// The arrangement is a one-event append followed by a two-event one, which
    /// is the shape that separates the two ways this goes wrong. A store that
    /// binds one identity for a whole multi-row `INSERT` is wrong only *within*
    /// a batch (`SharedBatchIdentityStore`, and `SharedBatchPositionStore`
    /// arriving at the same place through the position column); a store that
    /// resets something between calls is wrong only *across* batches. One
    /// single-event append and one multi-event append reach both, and a single
    /// batch of three would reach only the first.
    ///
    /// Uniqueness is checked by sorting and deduplicating rather than by
    /// pairwise comparison because `EventId` is `Ord` — deliberately, so that
    /// the convergent fold VT-5 argues for can sort merged events without a
    /// hash — and using that order here is free.
    pub async fn event_ids_are_unique_within_a_store<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Ay", &[("seat", "s1")])]).await;
        append_ok(
            &store,
            &[
                tagged_event("Bee", &[("seat", "s2")]),
                tagged_event("Cee", &[("seat", "s3")]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            3,
            "the anchor: three events were appended and three must come back, or \
             \"their identities are distinct\" is a statement about a shorter \
             list. Got {:?}",
            types_of(&all)
        );

        let observed: Vec<_> = all.iter().map(|event| event.id).collect();
        let mut distinct = observed.clone();
        distinct.sort_unstable();
        distinct.dedup();
        assert_eq!(
            distinct.len(),
            observed.len(),
            "a store holds at most one event per `EventId`, and enforcing that is \
             the store's obligation rather than the caller's — an ingest path \
             that establishes idempotence by reading first and appending second \
             has an unclosed race between two concurrent ingests. Got \
             {observed:?}"
        );

        RuleOutcome::Ran
    }

    /// VT-2 — structural equality is not identity.
    ///
    /// Rejects a content-hash identity scheme, and any store that deduplicates
    /// on payload equality: a refrigeration engineer consuming two of the same
    /// part on one work order writes two byte-identical `VanStockConsumed`
    /// events, a content hash collapses them into one, and the van's stock
    /// balance is permanently one unit high with nothing reporting it.
    ///
    /// **One batch rather than two appends**, which is deliberate on two counts.
    /// It is the arrangement a content-addressed scheme collapses — an
    /// `INSERT … ON CONFLICT (content_hash) DO NOTHING` inside one statement —
    /// and it exercises ES-19's slice-order assignment on the identity path at
    /// the same time.
    ///
    /// `ContentHashIdentityStore` in `tests/mutation_coverage/mutants.rs` is the
    /// compiled version, and it is a real adapter shape rather than a saboteur:
    /// content-addressed identity is what anyone reaching for idempotent ingest
    /// proposes first, and it is exactly what VT-8 forbids by making uniqueness
    /// the store's obligation. Its identities are unique across *distinct*
    /// events, stable across a reopen and never reissued, so it survives every
    /// other identity rule here.
    pub async fn appending_equal_events_yields_two_events<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let consumed = tagged_event("VanStockConsumed", &[("part", "p1")]);
        let again = consumed.clone();
        assert_eq!(
            consumed, again,
            "the anchor, and it is the whole premise of the rule: `Event` \
             compares structurally, so these two are equal by every measure a \
             store can take of them without consulting what it assigned"
        );

        append_ok(&store, &[consumed, again]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            2,
            "two structurally equal events are two events. A store that \
             deduplicates on payload equality returns one here, and the fact it \
             dropped is one nothing will ever report missing. Got {:?}",
            types_of(&all)
        );
        assert_ne!(
            all[0].position, all[1].position,
            "and they must occupy two distinct positions"
        );
        assert_ne!(
            all[0].id, all[1].id,
            "and carry two distinct identities. This is the conjunct that could \
             not be written before `SequencedEvent` carried an `EventId`: two \
             equal events already got two positions, so a rule written earlier \
             would have asserted two thirds of the clause and silently skipped \
             the third"
        );

        RuleOutcome::Ran
    }

    /// VT-7 — an `EventId` is outside the query language, and in particular it
    /// is not a `Tag`.
    ///
    /// Two halves, and the second is the one that bites. The first is that the
    /// event's own tags come back as the caller wrote them, with nothing added.
    /// The second is that the identity is not *matchable* — because the
    /// dangerous implementation does not put the tag on the event at all. It
    /// writes an extra row into the tag side table so that a membership question
    /// can be answered by the index the adapter already has, leaving
    /// `Event::tags()` round-tripping untouched and every payload-fidelity rule
    /// in the suite still passing. `IdentityMatchableAsTagStore` in
    /// `tests/mutation_coverage/mutants.rs` is that store.
    ///
    /// What it costs is structural rather than merely expensive. `Query`'s
    /// algebra is types-OR within an item, tags-AND with superset matching,
    /// items-OR across the query, and VT-31 freezes it because E2E-32's fan-out
    /// runner is correct only if `Items(a) ∪ Items(b) == Items(a ++ b)`. An
    /// identity axis is not a set-superset predicate; it is a point lookup on a
    /// unique key, and grafting it onto `QueryItem::matches` gives the item a
    /// third semantic with different composition rules.
    ///
    /// The probes cover the four keys an adapter would actually choose and both
    /// renderings — the whole `store:position` identity and the incarnation
    /// alone — because an adapter indexing only the origin store is the same
    /// mistake at lower cardinality.
    pub async fn event_id_is_not_matchable_by_query<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Issued", &[("unit", "u1")])]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            1,
            "the anchor: the event must be in the store before anything is said \
             about what does not match it, got {:?}",
            types_of(&all)
        );
        let stored = &all[0];

        assert_eq!(
            stored.tags(),
            &tags(&[("unit", "u1")]),
            "an event's tags are the caller's, and a store adds none of its own: \
             a materialised identity tag makes identity writer-forgeable and \
             enters every `contains_all` merge-scan on every query in the system"
        );

        // The anchor for the probes: a tag query this store *does* answer, so
        // that "nothing matched" below cannot be a store that matches nothing.
        let matched = read_ok(&store, &query_tagged(&[("unit", "u1")]), ReadOptions::new()).await;
        assert_eq!(
            matched.len(),
            1,
            "the second anchor: a tag query the store answers, without which \
             every assertion below holds for a store whose reads return nothing"
        );

        let identity = stored.id.to_string();
        let incarnation = stored.id.store().to_string();
        for key in ["event_id", "id", "_id", "origin"] {
            for value in [identity.as_str(), incarnation.as_str()] {
                let found =
                    read_ok(&store, &query_tagged(&[(key, value)]), ReadOptions::new()).await;
                assert!(
                    found.is_empty(),
                    "`{key}:{value}` matched {} event(s). Identity is answered by \
                     a dedicated port operation — `contains_event_id` — and not \
                     by the query language: a store that also indexes it as a tag \
                     puts a maximally high-cardinality entry in the one column \
                     adapters are told to index, and gives `QueryItem` a point \
                     lookup where its algebra has a set-superset predicate",
                    found.len()
                );
            }
        }

        RuleOutcome::Ran
    }

    /// VT-6 — reopening a store does not reissue an `EventId`.
    ///
    /// This rule **replaces** VT-6's originally named
    /// `store_id_is_stable_across_reopen`, which was invalid: it would fail every
    /// adapter that takes the second of the two mechanisms the clause permits —
    /// mint a fresh incarnation on every open — and a rule that forbids a
    /// permitted implementation is worse than a decorative one. What is asserted
    /// instead is the invariant *both* mechanisms satisfy.
    ///
    /// **The first assertion is not padding.** Replacing a rule loses whatever
    /// the replaced rule caught by accident, and `store_id_is_stable_across_reopen`
    /// rejected mint-per-*append* as a side effect of demanding stability. The
    /// reissue assertion alone does not, because per-append minting produces
    /// `EventId`s that all differ. "Two events in one open share a `StoreId`" is
    /// the weakest assertion that rejects it, it is true under both permitted
    /// mechanisms, and it costs one extra append.
    ///
    /// The wrong implementation the second assertion rejects is sharp, and it is
    /// the restore failure in miniature: **a store that keeps its `StoreId`
    /// across a reopen and restarts its position counter at 1.** That is what a
    /// restored backup looks like from the inside, and `LosingFixture` in
    /// `tests/mutation_coverage/mutants.rs` is it — a store with nothing durable
    /// behind it, so a reopen finds an empty medium and the next append mints an
    /// identity the store has already issued to a different event.
    ///
    /// What this rule cannot reach is **detection**: nothing in the tree can
    /// present an adapter with a restore it must notice, and `DurableFixture`
    /// reopens by instruction rather than by fault. The harm VT-6 names is
    /// reissue, and reissue is observable by instruction; detection stays
    /// phase 13's, in `happenstance-sync-testkit`'s
    /// `restored_peer_does_not_reissue_identities`.
    pub async fn reopened_store_does_not_reissue_an_event_id<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: REOPEN);

        let fixture = open().await;

        // The handle is dropped with the block, before the reopen: a handle that
        // carried state across would make the rule pass for the wrong reason.
        let before = {
            let store = fixture.connect().await;
            append_ok(
                &store,
                &[
                    tagged_event("Ay", &[("seat", "s1")]),
                    tagged_event("Bee", &[("seat", "s2")]),
                ],
            )
            .await;

            let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
            assert_eq!(
                all.len(),
                2,
                "the anchor: both events must be in the store before the reopen, \
                 or there is no identity for a later one to collide with. Got \
                 {:?}",
                types_of(&all)
            );
            all.iter().map(|event| event.id).collect::<Vec<_>>()
        };

        assert_eq!(
            before[0].store(),
            before[1].store(),
            "two events appended in one open share one incarnation. A store that \
             mints a fresh `StoreId` per append never reissues a pair — so it \
             passes the assertion below — and makes every event its own origin, \
             which is what the rule this one replaces used to catch by accident. \
             Got {before:?}"
        );

        fixture.reopen().await;

        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Cee", &[("seat", "s3")])]).await;

        let after = read_ok(&store, &query_of_types(&["Cee"]), ReadOptions::new()).await;
        assert_eq!(
            after.len(),
            1,
            "the second anchor: the post-reopen append must be readable, or the \
             collision check below has nothing to check. Got {:?}",
            types_of(&after)
        );
        let minted = after[0].id;

        assert!(
            !before.contains(&minted),
            "a store must never issue an `(StoreId, SequencePosition)` pair it \
             has already issued for a different event. A store that keeps its \
             incarnation across a reopen and restarts its position counter is \
             what a restored backup looks like from the inside: every peer's \
             deduplication then treats genuinely new events as already-seen and \
             **real facts are silently dropped** — the one failure mode in the \
             replication design with no error path and no observable symptom. \
             Minted {minted} again, having already issued {before:?}"
        );

        RuleOutcome::Ran
    }

    /// VT-9 — the store stamps a time at append, and reports that same value on
    /// every read of that event.
    ///
    /// **Read the prohibitions before strengthening this rule.** VT-9's third
    /// MUST forbids the contract from stating any relationship between
    /// `RecordedAt` order and `SequencePosition` order, and the conformance suite
    /// is the contract's executable form: an adapter author reads a failing rule
    /// as a requirement, so a rule that asserts an order states one. Therefore no
    /// comparison of two events' recorded times in either direction, no
    /// comparison of a recorded time against a position, and no plausibility
    /// check against the harness's own clock — which CF-33 forbids independently.
    /// `RUNBOOK.md`'s "a rule that it is non-decreasing with position" was struck
    /// by ADR-0014 for precisely this reason, and a store on a machine whose
    /// clock steps backwards under an NTP correction is conformant.
    ///
    /// What is left is that the value is a **fact about the append** rather than
    /// about the read, and that is not nothing. `ReadTimeClockStore` in
    /// `tests/mutation_coverage/mutants.rs` fills the field from the
    /// connection's clock in its row mapper, which is what an adapter does when
    /// `recorded_at` arrives on the port after its schema was written and the
    /// field has to be given *something*. That store is correct in every other
    /// respect and invisible to any rule that reads once.
    pub async fn append_stamps_a_recorded_time<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Issued", &[("unit", "u1")])]).await;

        let first = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            first.len(),
            1,
            "the anchor: the event must come back at all, got {:?}",
            types_of(&first)
        );

        let second = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            second.len(),
            1,
            "the second anchor: and it must come back again, unchanged in \
             number, got {:?}",
            types_of(&second)
        );
        assert_eq!(
            first[0].position, second[0].position,
            "and it must be the same event both times, or the comparison below \
             is between two different rows"
        );

        assert_eq!(
            first[0].recorded_at, second[0].recorded_at,
            "a `RecordedAt` records when the store accepted the event, so it is \
             fixed at append and reported unchanged afterwards. A row mapper that \
             fills the field from the connection's clock answers a different \
             question every time it is asked, and the audit answer the field \
             exists to give — which side of midnight did this land — becomes a \
             reading of when somebody last looked"
        );

        RuleOutcome::Ran
    }

    /// VT-9 — a reopen does not restamp an event's recorded time.
    ///
    /// The durability half of VT-9, and the half that catches the adapter VT-4's
    /// `Rejects:` line describes: one that makes the value up rather than
    /// persisting it "passes every rule that reads back what it just wrote
    /// inside one process and fails the first reopen".
    ///
    /// It asserts only that **the same event's** value is unchanged. It may not
    /// assert anything else: comparing it against another event's, against a
    /// position, or against the harness's own clock are all forbidden — see
    /// `append_stamps_a_recorded_time` for which prohibition forbids which.
    ///
    /// `LosingFixture` in `tests/mutation_coverage/mutants.rs` is the registered
    /// mutant, and it fails at the *survival* anchor rather than at the closing
    /// comparison: nothing was written to a durable medium, so the reopen finds
    /// an empty one. Nothing in this workspace can yet fail the closing
    /// comparison alone — that needs a durable medium that keeps an event and
    /// loses its stamp, which is a *real* store rather than a fixture, and phase
    /// 8 is where the first one arrives.
    ///
    /// # An instrument obligation this rule carries
    ///
    /// A `REOPEN` fixture passes it only if its reopen restores the event's
    /// stamps and not merely its payload. `DurableFixture` in
    /// `tests/fixture_instruments.rs` must carry `SequencedEvent` values on its
    /// durable side for that to be true; a durable side of `Event` cannot, and
    /// the replay restamps.
    pub async fn recorded_time_survives_a_reopen<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: REOPEN);

        let fixture = open().await;

        // The handle is dropped with the block, so nothing carries the value
        // across the reopen in memory.
        let (position, recorded_at) = {
            let store = fixture.connect().await;
            append_ok(&store, &[tagged_event("Issued", &[("unit", "u1")])]).await;

            let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
            assert_eq!(
                all.len(),
                1,
                "the anchor: the event must be readable before the reopen, got \
                 {:?}",
                types_of(&all)
            );
            (all[0].position, all[0].recorded_at)
        };

        fixture.reopen().await;

        let store = fixture.connect().await;
        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            1,
            "the second anchor: the acknowledged event must have survived the \
             reopen at all, or \"its time is unchanged\" is a claim about \
             nothing. Got {:?}",
            types_of(&all)
        );
        assert_eq!(
            all[0].position, position,
            "and it must be the same event, at the position the store assigned it \
             before the reopen"
        );

        assert_eq!(
            all[0].recorded_at, recorded_at,
            "a `RecordedAt` is persisted alongside the event, not recomputed when \
             the store is opened. An adapter that restamps on replay hands every \
             auditor the time of the last restart, and the one clock reading whose \
             provenance the log itself attested is gone with no error and no \
             symptom"
        );

        RuleOutcome::Ran
    }

    /// ES-41 (VT-7) — `contains_event_id` answers about an **identity**, not
    /// about a position.
    ///
    /// The membership operation VT-7 requires, and the whole reason it is a
    /// dedicated port operation rather than a query: dedup is a membership test,
    /// so it gets a membership test.
    ///
    /// The wrong implementation is the natural one — `SELECT 1 FROM events WHERE
    /// position = ?`, which is what an adapter writes when its events table has a
    /// position column and no origin columns yet, which is every adapter before
    /// it implements ingest. It passes every single-store rule in the suite,
    /// because a store that has ingested nothing only ever holds its own
    /// incarnation, and it fails the first time a peer asks: a foreign event is
    /// reported as already present whenever the local log happens to be at least
    /// that long, and the batch carrying it is silently dropped.
    /// `PositionOnlyMembershipStore` in `tests/mutation_coverage/mutants.rs` is
    /// it.
    ///
    /// **The foreign incarnation is built by flipping every byte of the store's
    /// own**, which is the only way a rule can name an incarnation this store
    /// certainly does not have: `EventStore` has no `store_id()` accessor
    /// (ES-19's prose says why one was refused), so the only `StoreId` a rule can
    /// obtain is the one it read back, and `!b == b` holds for no byte. A
    /// hard-coded sentinel would be a value some real adapter could one day mint.
    pub async fn contains_event_id_reports_membership<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[
                tagged_event("Ay", &[("seat", "s1")]),
                tagged_event("Bee", &[("seat", "s2")]),
            ],
        )
        .await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            2,
            "the anchor: both events must be in the store, or \"it reports what \
             it holds\" is a claim about an empty log. Got {:?}",
            types_of(&all)
        );

        for event in &all {
            match store.contains_event_id(event.id).await {
                Ok(true) => {}
                other => panic!(
                    "the store minted {} itself and must report holding it; got \
                     {other:?}",
                    event.id
                ),
            }
        }

        for event in &all {
            // Every byte flipped, so this incarnation is certainly not the
            // store's own — `!b == b` holds for no byte — while the position is
            // one the store definitely assigned. That pair is exactly what a
            // position-only lookup cannot tell apart from a local event.
            let elsewhere = EventId::new(
                StoreId::from_bytes(event.id.store().to_bytes().map(|byte| !byte)),
                event.id.position(),
            );
            match store.contains_event_id(elsewhere).await {
                Ok(false) => {}
                other => panic!(
                    "the store does not hold {elsewhere}, which names another \
                     incarnation at a position this store happens to have \
                     assigned. `SELECT 1 FROM events WHERE position = ?` is the \
                     natural implementation and it passes every single-store rule \
                     in this suite; against a peer it reports a foreign event as \
                     already held whenever the local log is long enough, and the \
                     ingest that trusted it drops a real fact. Got {other:?}"
                ),
            }
        }

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

    // ---------------------------------------------------------------------
    // 4. CF-39 — an armed mid-batch fault must actually fire
    // ---------------------------------------------------------------------

    /// A fixture that declares `MID_BATCH_FAULT` supported must arm a fault the
    /// store **cannot absorb**, so the append returns `Err`.
    ///
    /// CF-39, and it exists to make `append_is_atomic_under_a_mid_batch_fault`
    /// non-vacuous. That rule asserts the store holds all of a faulted batch or
    /// none of it, *with which of the two decided by what `append` answered* — so
    /// a fixture whose arm does nothing passes it for free: no fault, `Ok`, every
    /// row present, all-or-nothing satisfied. A capability can then be declared
    /// supported, contribute nothing, and produce a green atomicity result for a
    /// store nothing has ever faulted. That is the failure CF-18 exists to
    /// prevent one level up, where a rule `#[cfg]`-ed out of the binary is
    /// indistinguishable from a rule that passed.
    ///
    /// # The wrong implementation is a *fixture*, and it is a precise one
    ///
    /// The trait's provided `arm_mid_batch_fault` **panics**, and its message
    /// names this exact mistake — so a fixture that declares the capability and
    /// simply forgets the override does not pass vacuously, it aborts loudly. The
    /// vacuous pass needs a fixture that overrides `arm_mid_batch_fault` with an
    /// *empty body*: honest-looking code, no panic, no fault, green.
    /// `NoopFaultFixture` in `tests/mutation_coverage/mutants.rs` is that fixture,
    /// over a completely correct store, and it fails this rule and nothing else.
    ///
    /// # What this costs, stated rather than discovered later
    ///
    /// ES-18 permits a **store** to swallow a fault and commit the batch anyway,
    /// and that sentence is untouched. What CF-39 constrains is the **fixture**:
    /// a store that can absorb every fault its fixture is able to arm MUST
    /// decline the capability with that as its stated reason, rather than declare
    /// it and contribute an `Ok`. The weaker alternative is not a weaker rule, it
    /// is no rule at all — firing-and-being-absorbed and never-arming produce the
    /// same `Ok` over the same full log, so an anti-vacuity clause that stops
    /// short of demanding `Err` demands nothing a test can see.
    ///
    /// The control append is the anchor. Without it an `Err` here could be the
    /// store refusing a three-event batch for reasons of its own, and the rule
    /// would certify a fault that never fired.
    pub async fn arming_a_mid_batch_fault_makes_the_append_fail<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: MID_BATCH_FAULT);

        let fixture = open().await;
        let store = fixture.connect().await;

        // The anchor: the same shape of batch, unarmed, must be accepted.
        let control = [
            tagged_event("Control", &[("row", "one")]),
            tagged_event("Control", &[("row", "two")]),
            tagged_event("Control", &[("row", "three")]),
        ];
        append_ok(&store, &control).await;

        // Fail while writing the third event, so that two rows are already down
        // when the fault arrives — the same arming
        // `append_is_atomic_under_a_mid_batch_fault` uses, so that the two rules
        // are talking about the same event.
        fixture.arm_mid_batch_fault(2).await;
        let batch = [
            tagged_event("Armed", &[("row", "one")]),
            tagged_event("Armed", &[("row", "two")]),
            tagged_event("Armed", &[("row", "three")]),
        ];
        let outcome = store.append(&batch, None).await;

        assert!(
            outcome.is_err(),
            "CF-39: a fixture declaring `MID_BATCH_FAULT` supported MUST, when \
             armed at k < events.len(), cause the write of the k-th event to \
             fail inside the store's own write path, by a mechanism the store \
             cannot absorb, so that the append returns `Err`. This one armed a \
             fault at row 2 of a three-event batch and the append succeeded — \
             which is what a fixture whose `arm_mid_batch_fault` has an empty \
             body does, and it turns \
             `append_is_atomic_under_a_mid_batch_fault` into a green result \
             about a store nothing has faulted. A fixture whose store can absorb \
             every fault it is able to arm must DECLINE the capability with that \
             as its stated reason. Got {outcome:?}"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // 5. The read path's error arm — a fetch failure is not end-of-stream
    // ---------------------------------------------------------------------

    /// A fixture that declares `READ_FAULT` supported must arm a fault the
    /// store surfaces as an `Err` **item**, not as the end of the stream.
    ///
    /// [`EventStore::read`] yields
    /// `Result<SequencedEvent, Self::Error>` per item, and the whole value of
    /// that `Err` arm is that a caller can tell *the log ended* from *the fetch
    /// failed*. The wrong implementation is one line, and it is the most natural
    /// way to get a fallible fetch past a `poll_next` that must return a value:
    ///
    /// ```text
    /// let Ok(page) = fetch().await else { return Poll::Ready(None) };
    /// ```
    ///
    /// `SwallowedReadFaultStore` in `tests/mutation_coverage/mutants.rs` is that
    /// store, and it is not exotic: both adapters that will need a fallible
    /// fetch are already in the workspace — `happenstance-cloudflare` over
    /// `SqlStorage` and `happenstance-neon` over one-shot HTTP, neither of which
    /// can hold a cursor open across polls.
    ///
    /// # What the caller loses, which is why this is worth a rule
    ///
    /// Everything downstream reads `Ok`. A projection runner sees a short
    /// replay, commits its checkpoint at the truncation point, and the events
    /// above it are never applied — with no error anywhere to log. Whoever finds
    /// out is whoever reconciles the read model against the log, months later.
    /// The consumer half of the contract is already correct: `collect` returns
    /// `Poll::Ready(Err(..))` on a mid-stream `Err`, so a store that uses the
    /// arm it was given is reported faithfully.
    ///
    /// # Where the fault fires is the fixture's, and why
    ///
    /// [`arm_read_fault`](crate::Fixture::arm_read_fault) takes no index. A
    /// `read` is one call whose granularity — page, chunk, item — belongs to the
    /// adapter, and demanding a fault "after the *k*-th event" would be this
    /// suite asserting a paging model the port does not have. What is asserted
    /// is the one thing the port makes observable: an `Err` reaches the caller.
    ///
    /// # The control read is the anchor
    ///
    /// Without it, an `Err` here could be a store that refuses every read of
    /// four events for reasons of its own, and the rule would certify a fault
    /// that never fired. With it, the same read is known to succeed unarmed.
    ///
    /// CF-39's argument one path over: a fixture whose `arm_read_fault` does
    /// nothing passes vacuously, so a store that can absorb every read fault its
    /// fixture is able to arm MUST **decline** the capability with that as its
    /// stated reason rather than declare it and contribute a full, successful
    /// read.
    pub async fn arming_a_read_fault_makes_the_stream_yield_an_error<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        require!(F: READ_FAULT);

        let fixture = open().await;
        let store = fixture.connect().await;

        // Four events rather than one, so that a store paging in twos has a page
        // boundary to fail at and a swallowed failure is a *short* read rather
        // than an empty one — the shape that is indistinguishable from a
        // complete read of a smaller store.
        let seeded = [event("Read"), event("Read"), event("Read"), event("Read")];
        append_ok(&store, &seeded).await;

        // The anchor: unarmed, this exact read succeeds and returns all four.
        let control = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            control.len(),
            seeded.len(),
            "the anchor: without a read that succeeds unarmed, an `Err` below \
             could be a store refusing this read for reasons of its own"
        );

        // Armed after `connect`, deliberately: an adapter whose arming is
        // applied only when a handle is opened arms nothing a rule already
        // holding one can see, and the trait says so.
        fixture.arm_read_fault().await;
        let outcome = collect(store.read(&Query::all(), ReadOptions::new())).await;

        assert!(
            outcome.is_err(),
            "a fixture declaring `READ_FAULT` supported MUST cause the next \
             read's stream to yield an `Err` ITEM, and this one ended without \
             error. That is `let Ok(page) = fetch().await else {{ return \
             Poll::Ready(None) }};` — a fetch failure reported as the end of the \
             log, which every consumer downstream reads as `Ok`: a projection \
             runner replays a short prefix, checkpoints at the truncation point \
             and never applies the rest. If instead this fixture's store absorbs \
             every fault it can arm, it must DECLINE the capability with that as \
             its stated reason rather than contribute a successful read. Got {} \
             event(s) and no error, against {} appended",
            outcome.map_or(0, |events| events.len()),
            seeded.len()
        );

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
    // 1. ES-19 — positions within one batch follow slice order
    // ---------------------------------------------------------------------

    /// Positions within one batch are assigned in **slice order**, strictly
    /// ascending.
    ///
    /// ES-19's second sentence, which `append_returns_last_written_position`
    /// structurally cannot see: that rule infers the batch's order from
    /// `all.last()`, so it asks only *which* position came back and never *which
    /// event* got it. On a quiescent store the highest position is the last row
    /// however the rows were ordered, so a store that writes the batch backwards
    /// and returns the maximum answers it correctly.
    ///
    /// `ReverseOrderBatchStore` is the compiled version — a multi-row `INSERT`
    /// built from the batch reversed, which is what an adapter produces when it
    /// pushes rows onto a stack, or when it sorts a batch to group rows by an
    /// interned type id and forgets that grouping is also reordering. The
    /// consequence is not cosmetic: a decision replays its own batch in the order
    /// the store returns it, so `Held` then `Released` comes back as `Released`
    /// then `Held` and the projection is wrong with no error anywhere.
    /// `SharedBatchPositionStore` fails it too, one bind parameter over: one
    /// position bound for every row of a multi-row insert is not *ascending*, and
    /// "strictly" is the word that rejects it.
    ///
    /// # Two things it deliberately does not assert
    ///
    /// It does not re-assert the returned position.
    /// `append_returns_last_written_position` owns ES-19's first sentence and
    /// `ReturnsFirstOfBatchStore` is its mutant; making the claim here would buy
    /// a second failure on that store and no new coverage.
    ///
    /// And it compares positions **found by type**, never the order the read
    /// returned them in. Read ordering is `read_defaults_to_ascending_order`'s
    /// and `SortByEventTypeStore` is its mutant, so an order comparison here
    /// would fail every store whose defect is the read path — which is what keeps
    /// a mutant a scalpel rather than a shotgun.
    pub async fn batch_positions_follow_slice_order<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let batch = [
            tagged_event("Aleph", &[("row", "first")]),
            tagged_event("Beth", &[("row", "second")]),
            tagged_event("Gimel", &[("row", "third")]),
        ];
        append_ok(&store, &batch).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            3,
            "all three events of the batch must be readable back before their \
             positions mean anything, got {:?}",
            types_of(&all)
        );

        let position_of = |wanted: &str| {
            all.iter()
                .find(|event| event.event_type().as_str() == wanted)
                .unwrap_or_else(|| {
                    panic!(
                        "the store must hold the `{wanted}` event it was handed, \
                         got {:?}",
                        types_of(&all)
                    )
                })
                .position
        };
        let first = position_of("Aleph");
        let second = position_of("Beth");
        let third = position_of("Gimel");

        assert!(
            first < second && second < third,
            "ES-19: positions within one batch MUST be assigned in SLICE order \
             and MUST strictly ascend. The batch was Aleph, Beth, Gimel and the \
             store assigned {first:?}, {second:?}, {third:?}. A store that writes \
             the batch backwards, or that binds one position for every row of a \
             multi-row insert, still returns the highest position from a \
             quiescent store and passes `append_returns_last_written_position`"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // 2. ES-21 — a batch is not evaluated against its own condition
    // ---------------------------------------------------------------------

    /// A condition is evaluated only against events the store **already held**.
    ///
    /// ES-21 `[FROZEN]`. A batch can never conflict with itself, and the sentence
    /// has to be written down because the reference store answers it correctly
    /// only by accident of ordering — `MemoryEventStore` checks `stored` before
    /// it extends, and nothing said it had to.
    ///
    /// What it rejects is the per-row conditional
    /// `INSERT … SELECT … WHERE NOT EXISTS`, which the decision ledger carries as
    /// a live candidate for the append-condition SQL strategy. Carried per row,
    /// the guard travels with every statement, so the second row of a batch is
    /// checked against a store that already holds the first — and on the
    /// canonical DCB uniqueness shape, where the condition names the very type
    /// being written, it self-rejects. An adapter built that way refuses **every**
    /// conditional append of more than one matching event and passes every other
    /// rule in this suite. `PerRowConditionStore` is the compiled version, and it
    /// rolls back on rejection precisely so that it fails *this* rule and not
    /// `append_is_atomic`. `WriteThenCheckStore` fails it from the other side:
    /// writing before deciding puts the batch into the set its own probe reads.
    ///
    /// # The reissue at the end is the anchor, and it is doing real work
    ///
    /// Without it the rule's whole content is `is_ok()`, which a store whose
    /// probe answers `None` to everything satisfies for free — and an inert probe
    /// is exactly what this suite exists to catch. The identical batch under the
    /// identical condition must now be refused, which is evidence that the
    /// condition's query really does match these events and that the admission
    /// above was a verdict rather than an absence.
    ///
    /// `is_err`, deliberately, and not `ConditionViolated`: which error a
    /// rejection is reported as is ES-25's MUST and
    /// `condition_rejection_is_reported_as_condition_violated` owns it.
    pub async fn batch_is_not_evaluated_against_its_own_condition<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // Seeded so that the store is non-empty — `condition_against_an_empty_store_admits_the_append`
        // owns the degenerate case and `NullAggregateProbeStore` is its mutant —
        // and so that the boundary below is a position the store assigned. The
        // seed matches nothing the condition asks about.
        let boundary = append_ok(&store, &[tagged_event("Seen", &[("course", "c1")])]).await;

        // Both events of the batch are matched by the condition's own query, and
        // the boundary sits past everything the store holds — so the only events
        // that could violate it are the two being appended.
        //
        // They carry **one** tag each and are told apart by their payloads rather
        // than by a second tag. Both are forced: the condition's query names
        // `course:c1`, so both events must carry it, and two tags on one event
        // makes `TagJoinFanOutStore` return it twice while two byte-identical
        // events are one event to `PayloadDedupStore`. Distinct payloads under a
        // shared tag is the only seeding that is neither.
        let batch = [
            event_with_payload("Enrolled", &b"{\"student\":\"s1\"}"[..])
                .with_tags(tags(&[("course", "c1")])),
            event_with_payload("Enrolled", &b"{\"student\":\"s2\"}"[..])
                .with_tags(tags(&[("course", "c1")])),
        ];
        let guard = condition_after(query_of(&["Enrolled"], &[("course", "c1")]), boundary.get());

        let landed = store.append(&batch, Some(&guard)).await;
        assert!(
            landed.is_ok(),
            "ES-21: a condition is evaluated only against events the store \
             already held, so a batch can never conflict with itself. Both of \
             these events match the condition's query and would sit above its \
             boundary, and the append MUST still be admitted — a per-row \
             `INSERT … SELECT … WHERE NOT EXISTS` checks the second row against a \
             store that already holds the first and refuses it. Got {landed:?}"
        );

        let stored = read_ok(&store, &query_of_types(&["Enrolled"]), ReadOptions::new()).await;
        assert_eq!(
            stored.len(),
            2,
            "and both events of the batch must have landed: a store that \
             self-rejects and rolls back answers `Err`, and one that keeps the \
             row it managed to write answers `Ok` with half a batch. Got {:?}",
            positions_of(&stored)
        );

        // The anchor.
        let again = store.append(&batch, Some(&guard)).await;
        assert!(
            again.is_err(),
            "and the admission above must be a verdict rather than an absence: \
             the identical batch under the identical condition must be refused \
             now that the store really does hold matching events above the \
             boundary. Got {again:?}"
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // 3. ES-22 — a dropped append future leaves no partial batch
    // ---------------------------------------------------------------------

    /// Dropping an `append` future leaves the store fully applied or unchanged,
    /// never partially applied.
    ///
    /// ES-22 `[FROZEN]`. In Rust, cancellation *is* dropping the future: there is
    /// no cancel token, the caller simply stops polling and the state machine is
    /// destroyed at whatever suspension point it had reached. Nothing runs
    /// afterwards except `Drop` impls, and the call produces no `Result` at all —
    /// which is why ES-23's *outcome* is unspecified while ES-22's *shape* is
    /// not. The store may have committed, and may commit afterwards; what it may
    /// never do is hold two rows of a three-row batch.
    ///
    /// What it rejects is an adapter that executes a batch as several statements
    /// with a suspension point between them and relies on running to completion.
    /// `YieldingRowAtATimeStore` is the compiled version: one row per statement,
    /// awaiting between rows, so a drop after the first poll leaves exactly one
    /// row down. **It is not a rename of `RowAtATimeStore`**, which
    /// `a_concurrent_reader_never_sees_a_partial_batch` owns — that one tests the
    /// *visibility* of rows that all land in the end, this one tests rows that
    /// never land at all. Neither substitutes for the other.
    ///
    /// `MemoryEventStore` passes trivially, because its `append` body contains no
    /// `.await` at all and the first poll runs it to completion — which is
    /// precisely why the reference store cannot answer this question and a real
    /// adapter must.
    ///
    /// # Why the future is polled once rather than not at all
    ///
    /// An `async fn` body runs nothing until its first `poll`, so a future that
    /// is built and dropped has provably done nothing and this rule would be
    /// asserting over an untouched store. One poll is what puts the append in
    /// flight. `poll_once` answers `None` when the store suspended and `Some`
    /// when it finished, and ES-22 permits both.
    pub async fn dropped_append_future_leaves_no_partial_batch<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(&store, &[tagged_event("Existing", &[("row", "seed")])]).await;

        let before = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            types_of(&before),
            ["Existing"],
            "the anchor: without a row already in the store, \"unchanged\" and \
             \"empty\" are the same observation and a store that lost everything \
             would pass"
        );

        let batch = [
            tagged_event("Ex", &[("row", "one")]),
            tagged_event("Why", &[("row", "two")]),
            tagged_event("Zed", &[("row", "three")]),
        ];

        // The future is created, entered once, and dropped at the end of this
        // block. Nothing polls it again, which is the whole of what cancellation
        // means here.
        {
            let mut appending = pin!(store.append(&batch, None));
            let mut finished = false;
            let outcome = poll_once(appending.as_mut(), &mut finished).await;
            // A store with no suspension point in its `append` finishes on the
            // first poll; ES-22 permits "fully applied", so that is not a
            // failure. What it may not do is fail.
            assert_appended(outcome);
        }

        let after = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        let landed = after.len().saturating_sub(before.len());
        assert!(
            landed == 0 || landed == batch.len(),
            "ES-22: dropping an `append` future must leave the store fully \
             applied or unchanged — never partially applied. {} of {} events \
             survived the drop, which is a batch the caller was never told about \
             and can never resolve. Got {:?}",
            landed,
            batch.len(),
            types_of(&after)
        );

        if landed == 0 {
            assert_eq!(
                snapshot_of(&after),
                snapshot_of(&before),
                "and \"unchanged\" means byte-identical, not merely the same \
                 number of rows"
            );
        } else {
            // Membership rather than order. Slice order within a batch is
            // ES-19's and `batch_positions_follow_slice_order` owns it, so an
            // order comparison here would fail `ReverseOrderBatchStore` for a
            // reason that is not this rule's.
            let landed_types = types_of(&after);
            for event in &batch {
                assert!(
                    landed_types.contains(&event.event_type().as_str()),
                    "and \"fully applied\" means every event of the batch: `{}` \
                     is missing from {landed_types:?}",
                    event.event_type().as_str()
                );
            }
        }

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // 5-7. ES-24 — reissue: one guarantee and two limits
    // ---------------------------------------------------------------------

    /// A conditional append whose condition matches its **own** events is
    /// at-most-once under verbatim reissue.
    ///
    /// ES-24 `[FROZEN]`, and the guarantee costs nothing to provide because it
    /// falls out of the condition the caller already wrote. After a dropped
    /// future (ES-22, ES-23) the caller does not know whether its append landed.
    /// Reissuing the identical batch resolves it: a refusal means the first
    /// attempt landed, `Ok` means it had not and now has. Either way the store
    /// holds exactly one copy, and the caller needs no identity, no idempotency
    /// key and no new API — which is what makes this clause severable from
    /// `EventId`.
    ///
    /// What it rejects is a store that writes before it decides.
    /// `WriteThenCheckStore` — autocommit plus a separate probe — extends its log
    /// and *then* evaluates the condition, so the batch's own events are in the
    /// set the probe reads and the **first** attempt is refused while its rows
    /// stay down. A caller following the documented resolution procedure reads
    /// that refusal as "my write already landed", stops, and has written nothing
    /// at all on the one path where the answer matters.
    ///
    /// # Two deliberate choices in the arrangement
    ///
    /// The store is **seeded first**, so that no assertion here runs against an
    /// empty store: `condition_against_an_empty_store_admits_the_append` owns
    /// that degenerate case and `NullAggregateProbeStore` is its mutant, and a
    /// rule that started empty would fail it for a reason that is not this
    /// rule's.
    ///
    /// The second attempt is asserted with `is_err`, not with a match on
    /// `ConditionViolated`. Which error a rejection is reported as is ES-25's
    /// MUST and `condition_rejection_is_reported_as_condition_violated` owns it;
    /// demanding the discriminant here would make `ViolationAsStoreErrorStore`
    /// fail a rule about reissue for a reason that has nothing to do with
    /// reissue.
    pub async fn reissued_conditional_batch_lands_once<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;
        append_ok(
            &store,
            &[tagged_event("CourseDefined", &[("course", "c1")])],
        )
        .await;

        // The canonical shape: the condition's query matches the very event being
        // appended, which is what buys the guarantee.
        // One tag, not two: `TagJoinFanOutStore` returns an event once per tag
        // it carries, so a two-tag event would make the closing "exactly one
        // copy" assertion fail against a store whose defect is the read join.
        let subscribe = tagged_event("StudentSubscribed", &[("enrolment", "c1-s1")]);
        let query = query_of(&["StudentSubscribed"], &[("enrolment", "c1-s1")]);
        let guard = condition(query.clone());

        let first = store
            .append(core::slice::from_ref(&subscribe), Some(&guard))
            .await;
        assert!(
            first.is_ok(),
            "a conditional append whose condition matches its own events must \
             still be admitted the first time — a store that writes the batch \
             and probes afterwards finds its own rows and refuses. Got {first:?}"
        );

        // The verbatim reissue: the identical batch under the identical
        // condition, which is what a caller does when a dropped future left the
        // outcome unknown.
        let second = store
            .append(core::slice::from_ref(&subscribe), Some(&guard))
            .await;
        assert!(
            second.is_err(),
            "ES-24: reissuing the identical batch must be refused once the first \
             attempt has landed, because the condition matches the events it \
             wrote. That refusal is how a caller learns its append already \
             happened, and it is the whole recovery mechanism for an unknown \
             outcome. Got {second:?}"
        );

        let all = read_ok(&store, &query, ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            1,
            "and the store must hold exactly one copy: at-most-once is the \
             property, and a second copy makes the refusal above a lie about \
             what is in the log. Got {:?}",
            positions_of(&all)
        );

        RuleOutcome::Ran
    }

    /// An **unconditional** append has no at-most-once property: a reissue
    /// appends a second copy.
    ///
    /// ES-24's first stated limit, and this rule pins a *non*-guarantee. That is
    /// unusual enough to say why: a guarantee whose limits are unstated is read
    /// as universal, and callers write retry loops against the adapter they
    /// happened to test on. If one adapter deduplicates and another does not, the
    /// same retry loop double-charges on one and not on the other, and nothing in
    /// either adapter's CI says so. So duplicate-landing is a **MUST** here, and
    /// a store with natural payload dedup is non-conformant. That is deliberate:
    /// a contract whose idempotency varies silently by adapter is worse than one
    /// with none.
    ///
    /// What it rejects is `PayloadDedupStore` — a store that hashes
    /// `(event_type, tags, data)` and refuses a duplicate. Not a strawman: a
    /// content-addressed store, or one built to be safe under at-least-once
    /// ingest, gets that behaviour for free and would ship it as a feature.
    ///
    /// The two events are asserted **equal** as well as counted, which is the
    /// non-vacuity anchor: without it a store could satisfy the count by mangling
    /// one of them, and the rule would have proved that two rows exist rather
    /// than that the same event landed twice.
    pub async fn reissued_unconditional_batch_lands_twice<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let noted = tagged_event("Noted", &[("ledger", "l1")]);
        append_ok(&store, core::slice::from_ref(&noted)).await;
        append_ok(&store, core::slice::from_ref(&noted)).await;

        let all = read_ok(&store, &query_of_types(&["Noted"]), ReadOptions::new()).await;
        assert_eq!(
            all.len(),
            2,
            "ES-24: an unconditional append has no at-most-once property, and \
             nothing in the port can give it one. Reissuing a byte-identical \
             batch MUST append a second copy — a store that hashes the event and \
             quietly refuses the duplicate strengthens a guarantee the contract \
             disclaims, and callers then depend on behaviour the next adapter \
             does not have. Got {:?}",
            positions_of(&all)
        );
        assert_eq!(
            all[0].event, all[1].event,
            "the anchor: both copies must be the same event twice — otherwise \
             this rule has counted two rows rather than observed one event \
             landing twice"
        );
        assert_eq!(
            all[0].event, noted,
            "and that event must be the one the caller wrote"
        );

        RuleOutcome::Ran
    }

    /// A **conditional** append whose condition does not match its own events has
    /// no at-most-once property either.
    ///
    /// ES-24's second stated limit — the one the clause states in prose and gives
    /// no rule. It is the *common* shape rather than an exotic corner: a decision
    /// that reads one thing and writes another, conditioning on
    /// `CourseCapacityChanged` while appending `StudentSubscribed`, leaves the
    /// retry indistinguishable from a first attempt, because nothing the retry
    /// wrote is in the set its own condition looks at.
    ///
    /// This is the rule that stops a caller reading
    /// `reissued_conditional_batch_lands_once` as "conditional appends are
    /// idempotent". They are not; *conditions that match their own events* are,
    /// and the difference is one line in the caller's decision model. Callers
    /// needing at-most-once here must supply their own dedup in the domain — a
    /// natural key in the **tags**, which is queryable, and not in `metadata`,
    /// which is structurally not.
    ///
    /// `PayloadDedupStore` is the compiled version, and it passes
    /// `reissued_conditional_batch_lands_once` while failing this rule and its
    /// sibling — which is exactly the shape of the mistake: an adapter that gets
    /// the *documented* guarantee right and silently extends it.
    ///
    /// The condition's boundary sits above the matching event on purpose. With
    /// `after` at the match this rule would also reject a probe reading `after`
    /// as inclusive, which `condition_after_ignores_events_at_the_boundary` owns.
    pub async fn reissued_batch_conditioned_on_other_events_lands_twice<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        append_ok(
            &store,
            &[tagged_event("CourseCapacityChanged", &[("course", "c1")])],
        )
        .await;
        // A non-matching marker, so the boundary is a position the store assigned
        // and sits strictly above every event the condition can match.
        let boundary = append_ok(&store, &[tagged_event("Marker", &[("course", "c1")])]).await;

        let subscribe = tagged_event("StudentSubscribed", &[("course", "c1")]);
        // The condition reads capacity and the batch writes a subscription: the
        // events being appended are matched by nothing the condition asks about.
        let guard = condition_after(
            query_of(&["CourseCapacityChanged"], &[("course", "c1")]),
            boundary.get(),
        );

        let first = store
            .append(core::slice::from_ref(&subscribe), Some(&guard))
            .await;
        assert!(
            first.is_ok(),
            "nothing has changed the capacity since the caller read it, so the \
             append must be admitted. Got {first:?}"
        );

        let second = store
            .append(core::slice::from_ref(&subscribe), Some(&guard))
            .await;
        assert!(
            second.is_ok(),
            "ES-24: a conditional append whose query does not match its own \
             events has no at-most-once property. The reissue is \
             indistinguishable from a first attempt — nothing the first attempt \
             wrote is in the set this condition looks at — so it MUST be \
             admitted. Got {second:?}"
        );

        let all = read_ok(
            &store,
            &query_of_types(&["StudentSubscribed"]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            all.len(),
            2,
            "and both copies must be in the store: this is the limit the clause \
             states, and a store that deduplicates it away leaves callers \
             depending on an idempotency the contract disclaims. Got {:?}",
            positions_of(&all)
        );
        assert_eq!(
            all[0].event, all[1].event,
            "the anchor: the two copies must be the same event, or this rule has \
             counted rows rather than observed a reissue"
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

    /// Two tags that differ only by Unicode normal form are two tags.
    ///
    /// VT-15 `[FROZEN]`: equality is byte equality over the UTF-8 encoding, and
    /// the contract — and every adapter — applies no normalisation, no case
    /// folding and no trimming. `"café"` in NFC is five bytes ending U+00E9;
    /// in NFD it is six, ending `e` + U+0301. They render identically in every
    /// console, every editor and every log.
    ///
    /// # What it rejects
    ///
    /// `NormalisingTagStore`: a tag column under a **nondeterministic** ICU
    /// collation — `CREATE COLLATION … (provider = icu, deterministic = false)`
    /// is one line and is exactly what somebody reaches for when a search
    /// stops matching — or an adapter calling a normaliser on the way in
    /// because "tags should be canonical". Both rewrite a caller's identifier,
    /// so the tag read back is not the tag written, byte-faithful replication
    /// stops being byte-faithful, and two consistency boundaries silently
    /// become one. A macOS client writes NFD, a Linux client writes NFC, and
    /// the two stop conflicting where the application intended them to.
    ///
    /// # Why the failure is a *query* assertion rather than a round trip
    ///
    /// Both rows survive a normalising column; what is lost is that they are
    /// distinguishable. So the round trip is the anchor and the two selective
    /// reads are the rule: under a merging collation the query built from
    /// either form returns both events, and a caller that thought it was
    /// looking at one tenant, one turbine or one shop is looking at two.
    /// `append_preserves_event_type_and_tags_byte_for_byte` owns the byte
    /// fidelity of the stored value; this rule owns the index's opinion of it.
    pub async fn tags_differing_only_by_unicode_normalisation_are_distinct<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let composed = "caf\u{e9}";
        let decomposed = "cafe\u{301}";
        assert_ne!(
            composed, decomposed,
            "the anchor: these must be two different byte strings, or this rule \
             is one tag written twice and nothing can fail it"
        );

        let first = append_ok(&store, &[tagged_event("Ordered", &[("shop", composed)])]).await;
        let second = append_ok(&store, &[tagged_event("Ordered", &[("shop", decomposed)])]).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            positions_of(&all),
            [first.get(), second.get()],
            "both events must be in the store before their tags can be compared"
        );

        let by_composed = read_ok(
            &store,
            &query_tagged(&[("shop", composed)]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            positions_of(&by_composed),
            [first.get()],
            "a query built from the composed tag must select the event written \
             with it and no other: VT-15 makes equality byte equality, and a \
             normalising column merges two consistency boundaries with no error \
             and no visible cue anywhere"
        );

        let by_decomposed = read_ok(
            &store,
            &query_tagged(&[("shop", decomposed)]),
            ReadOptions::new(),
        )
        .await;
        assert_eq!(
            positions_of(&by_decomposed),
            [second.get()],
            "and the decomposed tag must select the other one, for the same \
             reason read from the other end"
        );

        RuleOutcome::Ran
    }

    /// One key, two values, two tags — and both are queryable.
    ///
    /// VT-17 `[FROZEN]`: `key:value` is a convention the contract does not
    /// enforce. `Tags` deduplicates on the **whole** tag string, so
    /// `Tags::from_pairs([("tenant", "a"), ("tenant", "b")])` is a two-element
    /// set and both elements are the caller's data.
    ///
    /// # What it rejects
    ///
    /// `KeyedTagMapStore`: tags stored as a map — a `JSONB` object, a
    /// `HashMap<String, String>` column, or a side table under
    /// `UNIQUE (event_id, key)` with `ON CONFLICT (event_id, key) DO UPDATE`.
    /// Every one of those is a natural schema for something the contract itself
    /// invites you to read as a pair (`Tag::key`, `Tag::value`), and every one
    /// keeps exactly one value per key. The event stays in the store and stops
    /// matching one of the two queries that should select it. On Wattline's
    /// 4,200-tenant shared log that is a cross-tenant correctness failure
    /// produced entirely by an indexing choice, and the tenant whose tag was
    /// dropped simply stops seeing its own events.
    ///
    /// # The anchor is on the type, not on the store
    ///
    /// If `Tags` ever collapsed the pair at construction, this rule would be
    /// asserting a store's behaviour over an input it never received — passing
    /// while VT-17's actual subject went unchecked. The length check is what
    /// keeps the rule on the value it claims to be about.
    pub async fn tags_may_repeat_a_key<F: Fixture>(open: impl AsyncFn() -> F) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let shared = tagged_event(
            "MeterRead",
            &[("tenant", "a"), ("tenant", "b"), ("meter", "m1")],
        );
        assert_eq!(
            shared.tags().len(),
            3,
            "the anchor: `Tags` deduplicates on the whole `key:value` string, so \
             two values under one key are two tags. If this is ever 2, the rule \
             has stopped being about VT-17 and nothing says so"
        );

        let written = append_ok(&store, core::slice::from_ref(&shared)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1, "the store must hold the event it accepted");
        assert_eq!(
            all[0].event, shared,
            "an event carrying two values under one key must round-trip with \
             both of them: a tag index shaped as a key-to-value map keeps one \
             and reports nothing"
        );

        for tenant in ["a", "b"] {
            let found = read_ok(
                &store,
                &query_tagged(&[("tenant", tenant)]),
                ReadOptions::new(),
            )
            .await;
            assert_eq!(
                positions_of(&found),
                [written.get()],
                "and the event must be selected by a query on *either* value — \
                 the tag that a map-shaped index dropped is the one whose owner \
                 stops seeing its own events"
            );
        }

        RuleOutcome::Ran
    }

    /// An event type and a tag survive the round trip byte for byte, whitespace
    /// included.
    ///
    /// VT-1 and VT-15. `append_preserves_event_payload` already compares a whole
    /// `Event` after a round trip and cannot reach this: the identifiers it
    /// writes are `"A"` and `"course:c1"`, which are fixed points of every
    /// transformation an adapter might apply. What this rule writes is an
    /// identifier at the one edge a store is tempted to tidy.
    ///
    /// # What it rejects
    ///
    /// `TrimmingIdentifierStore`: `TRIM()` in the insert statement, or
    /// `value.trim()` in the row mapper, because a trailing space in an
    /// identifier "must be a typo". Kestrel Rotor replicated a
    /// `SerialisedUnitConsumed` carrying `turbine:HW2-A14 `; `Tag::new` accepts
    /// it, `Tags` sorts it adjacent to the unpadded tag, and `contains_all` is a
    /// strict merge-scan on equality that does not match it. A lot-recall query
    /// silently missed a turbine. An adapter that trims makes the *store* the
    /// place that decides, which is worse than the application's bug it was
    /// trying to fix: the value the caller wrote is no longer in the log, so
    /// nothing downstream can even detect what happened.
    ///
    /// # Both directions are asserted, and they fail differently
    ///
    /// The padded value must still match a query built from the same bytes — a
    /// rewritten index entry matches nothing, including itself — and the trimmed
    /// value must match **nothing**, because whitespace is significant and the
    /// two strings are two consistency boundaries. A store that trims on the way
    /// in fails the first; a store that trims only in its query builder fails
    /// the second.
    pub async fn append_preserves_event_type_and_tags_byte_for_byte<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let padded_type = "SerialisedUnitConsumed ";
        let padded_value = "HW2-A14 ";
        let trimmed_value = "HW2-A14";
        assert_ne!(
            padded_value, trimmed_value,
            "the anchor: the padded and the trimmed value must differ, or the \
             closing assertion cannot tell a trimming adapter from a faithful one"
        );

        let original = tagged_event(padded_type, &[("turbine", padded_value)]);
        let written = append_ok(&store, core::slice::from_ref(&original)).await;

        let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(all.len(), 1, "the store must hold the event it accepted");
        assert_eq!(
            all[0].event, original,
            "VT-15 makes equality byte equality and the contract normalises \
             nothing: an adapter that trims, folds case or normalises an \
             identifier stores a value that is not the one the caller wrote, \
             with no error anywhere"
        );

        let by_type = read_ok(&store, &query_of_types(&[padded_type]), ReadOptions::new()).await;
        assert_eq!(
            positions_of(&by_type),
            [written.get()],
            "and the event must still match a query built from the same bytes — \
             a rewritten index entry matches nothing, including itself"
        );

        let trimmed = read_ok(
            &store,
            &query_tagged(&[("turbine", trimmed_value)]),
            ReadOptions::new(),
        )
        .await;
        assert!(
            trimmed.is_empty(),
            "while the trimmed tag must match nothing: whitespace is \
             significant everywhere, so `turbine:HW2-A14 ` and \
             `turbine:HW2-A14` are two consistency boundaries and a store that \
             conflates them answers a lot-recall query with somebody else's \
             turbine. Got {:?}",
            positions_of(&trimmed)
        );

        RuleOutcome::Ran
    }

    // ---------------------------------------------------------------------
    // 8. VT-25 / CF-40 — a capacity refusal is distinguishable
    // ---------------------------------------------------------------------

    /// A store that refuses an over-capacity append reports it as
    /// `AppendError::ExceedsStoreLimit`, and never by truncating.
    ///
    /// VT-25 `[FROZEN]`, and the refusal half of the four floors this section's
    /// other rules cover. VT-21 – VT-24 say what every store MUST accept; this
    /// says what a store MUST do about anything larger. The distinction the
    /// variant exists for is a sync runner's: "this event will never fit here,
    /// park it and tell a human" against "the disk is full, retry". A runner that
    /// cannot tell them apart guesses, and a runner that guesses wrong drops an
    /// event permanently (E2E-42).
    ///
    /// # Why the ceilings come from the fixture (CF-40)
    ///
    /// There is no `MAX_EVENT_DATA_LEN` constant and there must not be one — a
    /// single number is a straitjacket on Postgres and a lie on a KV-backed peer.
    /// So the ceiling is a **fact about this store**, and the fixture states it:
    /// [`MAX_EVENT_DATA_LEN`](crate::Fixture::MAX_EVENT_DATA_LEN),
    /// [`MAX_TAGS_PER_EVENT`](crate::Fixture::MAX_TAGS_PER_EVENT) and
    /// [`MAX_EVENTS_PER_BATCH`](crate::Fixture::MAX_EVENTS_PER_BATCH), each
    /// defaulting to `None`.
    ///
    /// They are `Option<usize>` and **not** [`Capability`](crate::Capability),
    /// and the difference is not cosmetic. A declined `Capability` is a *trade*:
    /// the fixture could have co-operated and chose not to, and the reason it
    /// gives is the record of that choice. `None` here is a store reporting a
    /// fact about itself — that it has no ceiling — and there is nothing to trade
    /// away. It routes through the same skipped-with-a-reason path because the
    /// reporting obligation is identical: a rule that cannot run must say so, or
    /// a green suite is indistinguishable from a green suite minus one rule.
    ///
    /// # What it rejects
    ///
    /// Four compiled stores, in two pairs, and each pair is one column with the
    /// engine configured two ways. `PayloadCeilingStore` and
    /// `BatchParameterCeilingStore` refuse through `AppendError::Store` — which
    /// is what every adapter does today, because until this variant existed there
    /// was nowhere else to put it. `TruncatingPayloadStore` and
    /// `ChunkLosingBatchStore` do not refuse at all: they store what fits, answer
    /// `Ok` with a real position, and the caller is told the whole write landed.
    /// The second pair is why each limit is checked twice — once for the answer,
    /// once for what is in the store afterwards.
    ///
    /// # The at-the-ceiling append is the anchor
    ///
    /// Without it a store that refused *everything* would satisfy every assertion
    /// below, and the rule would certify a ceiling nobody had located. A value at
    /// exactly the stated limit must be accepted; a value one larger must be
    /// refused. Nothing here asserts what the limit *is* — that is the store's to
    /// document — only that it is where the fixture says it is. Nor does anything
    /// assert the `len` field's value: VT-25 says it carries "the value that
    /// exceeded" the limit, and an adapter that reports its own ceiling instead
    /// is answering a caller's question about magnitude either way.
    pub async fn append_reports_exceeded_store_limits<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        // CF-40's gate. Written out rather than behind `require!` because the
        // three constants are `Option<usize>` rather than `Capability`: a store
        // with no ceiling has nothing to refuse and no trade to record.
        if F::MAX_EVENT_DATA_LEN.is_none()
            && F::MAX_TAGS_PER_EVENT.is_none()
            && F::MAX_EVENTS_PER_BATCH.is_none()
        {
            return RuleOutcome::Skipped {
                capability: NO_STORE_LIMITS,
                reason: NO_CEILING_REASON,
            };
        }

        let fixture = open().await;
        let store = fixture.connect().await;

        if let Some(ceiling) = F::MAX_EVENT_DATA_LEN {
            payload_ceiling_is_honoured(&store, ceiling).await;
        }
        if let Some(ceiling) = F::MAX_TAGS_PER_EVENT {
            tag_ceiling_is_honoured(&store, ceiling).await;
        }
        if let Some(ceiling) = F::MAX_EVENTS_PER_BATCH {
            batch_ceiling_is_honoured(&store, ceiling).await;
        }

        RuleOutcome::Ran
    }

    /// One ceiling of `append_reports_exceeded_store_limits`, checked at both
    /// ends: exactly the stated limit is accepted, one more is refused as
    /// `ExceedsStoreLimit`, and nothing of the refused value survives.
    ///
    /// Three helpers rather than one rule body, and the reason is a lint that is
    /// right: `clippy::too_many_lines` fires at a hundred, and one rule over
    /// three limits is a hundred and forty-seven. Splitting on the limit rather
    /// than on the assertion keeps each half readable on its own — the failure
    /// message a reader lands on names one ceiling, and the helper it is in
    /// contains only that ceiling's arrangement.
    ///
    /// It is one *rule* over three limits for the reason
    /// `append_reports_exceeded_store_limits`'s own documentation gives: three
    /// rules would cost three registry entries, three changelog entries and three
    /// clause `Rule:` lines for three failures that can only arrive together in
    /// an adapter that got its capacity reporting wrong once.
    async fn payload_ceiling_is_honoured<S: EventStore>(store: &S, ceiling: usize) {
        // A repeating non-zero pattern rather than zeroes: a column that
        // stores the length and nothing else, or a driver that treats a run
        // of NULs as a terminator, both survive an all-zero payload.
        let filler = |len: usize| -> Vec<u8> {
            (0..len)
                .map(|byte| u8::try_from(byte % 251).unwrap_or(0))
                .collect()
        };

        let at = event_with_owned_payload("AtTheCeiling", filler(ceiling))
            .with_tags(tags(&[("edge", "data-at")]));
        append_ok(store, core::slice::from_ref(&at)).await;

        let over = event_with_owned_payload("OverTheCeiling", filler(ceiling + 1))
            .with_tags(tags(&[("edge", "data-over")]));
        let refused = store.append(core::slice::from_ref(&over), None).await;
        assert!(
            matches!(
                refused,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    ..
                })
            ),
            "VT-25: this fixture states a payload ceiling of {ceiling} bytes, \
             so a payload of {} must be refused as \
             `AppendError::ExceedsStoreLimit {{ limit: EventDataLen, .. }}` — \
             not as `AppendError::Store`, which a caller cannot tell from a \
             transient failure, and not by succeeding. Got {refused:?}",
            ceiling + 1
        );

        let leftovers = read_ok(
            store,
            &query_tagged(&[("edge", "data-over")]),
            ReadOptions::new(),
        )
        .await;
        assert!(
            leftovers.is_empty(),
            "and a store MUST NOT truncate instead: the refused event must \
             not be in the log in any form. Got {:?}",
            positions_of(&leftovers)
        );
    }

    /// The tag-count ceiling. See [`payload_ceiling_is_honoured`].
    async fn tag_ceiling_is_honoured<S: EventStore>(store: &S, ceiling: usize) {
        let owned: Vec<(String, String)> = (0..=ceiling)
            .map(|n| (format!("k{n:05}"), format!("v{n:05}")))
            .collect();
        let pairs: Vec<(&str, &str)> = owned
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let at = tagged_event("TagsAtTheCeiling", &pairs[..ceiling]);
        assert_eq!(
            at.tags().len(),
            ceiling,
            "the anchor: `Tags` deduplicates, so a generator that collided \
             would leave this rule testing a smaller number than the fixture \
             claims"
        );
        append_ok(store, core::slice::from_ref(&at)).await;

        let over = tagged_event("TagsOverTheCeiling", &pairs);
        assert_eq!(
            over.tags().len(),
            ceiling + 1,
            "and the over-limit event must really carry one tag more than the \
             ceiling"
        );
        let refused = store.append(core::slice::from_ref(&over), None).await;
        assert!(
            matches!(
                refused,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::TagsPerEvent,
                    ..
                })
            ),
            "VT-25: this fixture states a ceiling of {ceiling} tags per \
             event, so {} tags must be refused as \
             `AppendError::ExceedsStoreLimit {{ limit: TagsPerEvent, .. }}` \
             rather than by packing what fits into one column and dropping \
             the rest. Got {refused:?}",
            ceiling + 1
        );

        let leftovers = read_ok(
            store,
            &query_of_types(&["TagsOverTheCeiling"]),
            ReadOptions::new(),
        )
        .await;
        assert!(
            leftovers.is_empty(),
            "and it MUST NOT store the event with the overflow dropped: a \
             tag that fell off the end is an event a query on that tag will \
             never find, with no error anywhere. Got {} event(s)",
            leftovers.len()
        );
    }

    /// The batch-size ceiling. See [`payload_ceiling_is_honoured`].
    async fn batch_ceiling_is_honoured<S: EventStore>(store: &S, ceiling: usize) {
        let at: Vec<Event> = (0..ceiling)
            .map(|n| tagged_event("BatchAtTheCeiling", &[("n", format!("{n:05}").as_str())]))
            .collect();
        append_ok(store, &at).await;

        let over: Vec<Event> = (0..=ceiling)
            .map(|n| tagged_event("BatchOverTheCeiling", &[("n", format!("{n:05}").as_str())]))
            .collect();
        let refused = store.append(&over, None).await;
        assert!(
            matches!(
                refused,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventsPerBatch,
                    ..
                })
            ),
            "VT-25: this fixture states a ceiling of {ceiling} events per \
             append, so a batch of {} must be refused as \
             `AppendError::ExceedsStoreLimit {{ limit: EventsPerBatch, .. }}` \
             — a refusal arriving as an adapter error leaves a sync runner \
             nothing to switch on. Got {refused:?}",
            ceiling + 1
        );

        let leftovers = read_ok(
            store,
            &query_of_types(&["BatchOverTheCeiling"]),
            ReadOptions::new(),
        )
        .await;
        assert!(
            leftovers.is_empty(),
            "and it MUST NOT clamp the batch to what one statement can \
             carry: `&events[..ceiling]` where `events.chunks(ceiling)` was \
             meant answers `Ok` with a real position and never writes the \
             tail. Got {} event(s) of a refused batch in the store",
            leftovers.len()
        );
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
    // 9-10. VT-30 — guards carry independent boundaries
    // ---------------------------------------------------------------------

    /// Each guard of an `AppendCondition` is evaluated against **its own**
    /// boundary.
    ///
    /// VT-30. A decision model assembled from fragments read separately produces
    /// one boundary per fragment, and there is no single boundary that is correct
    /// for all of them. VT-27 is `[FROZEN]` and refuses per-item bounds inside a
    /// `Query`, telling such an application to issue one read per fragment — and
    /// that refusal is only *sound* if the resulting boundaries can be carried
    /// into one condition. Without guards the prescribed workaround is the
    /// `min(p₁…p₄)` collapse, and a collapse is a liveness failure: the quiet
    /// fragment's stale boundary governs the busy one, so appends that never
    /// conflicted are refused and the deployment reads the rejection rate as
    /// contention.
    ///
    /// `MinCollapseStore` is that workaround promoted into an adapter, and it is
    /// the compiled version. It passes the **entire** existing `condition_after_*`
    /// family, because every rule in it carries a single guard and the minimum
    /// over one boundary is that boundary — which is exactly why this rule has to
    /// exist rather than be inferred.
    ///
    /// # The arrangement, and why the boundaries sit where they do
    ///
    /// The store holds a quiet event, a marker, a busy event and a second marker,
    /// in that order. The quiet guard's boundary is the **first** marker — above
    /// its own last match and *below* the busy event — and the busy guard's is
    /// the second. Under correct per-guard evaluation neither guard has a match
    /// above its own boundary and the append is admitted; under a collapse to the
    /// minimum, the busy event sits above it and the append is refused.
    ///
    /// Each boundary is strictly above its own guard's last match rather than at
    /// it, so that this rule does not also reject a probe reading `after` as
    /// inclusive — `condition_after_ignores_events_at_the_boundary` owns that and
    /// `AfterIsInclusiveStore` is its mutant.
    ///
    /// The mirror re-runs the same shape with the busy guard **unbounded**, which
    /// must be refused. It is the non-vacuity anchor: without it a store whose
    /// probe never fires satisfies the admission above for free, and the rule
    /// would certify an inert condition.
    ///
    /// Nothing here reads the store back, and that is deliberate: every position
    /// comes from the `append` that assigned it, so a store whose *read* path is
    /// wrong fails the rules that own the read path and not this one.
    pub async fn condition_guards_carry_independent_boundaries<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let quiet_query = query_of(&["TariffPublished"], &[("tariff", "t1")]);
        let busy_query = query_of(&["MeterReadingTaken"], &[("meter", "m1")]);

        append_ok(
            &store,
            &[tagged_event("TariffPublished", &[("tariff", "t1")])],
        )
        .await;
        // The markers match neither guard, and are distinguishable from each
        // other so that a content-addressed store has nothing to deduplicate.
        let quiet_boundary = append_ok(&store, &[tagged_event("Marker", &[("n", "1")])]).await;
        append_ok(
            &store,
            &[tagged_event("MeterReadingTaken", &[("meter", "m1")])],
        )
        .await;
        let busy_boundary = append_ok(&store, &[tagged_event("Marker", &[("n", "2")])]).await;

        // The caller read the quiet fragment to its own boundary and the busy one
        // to its own, and carries both into one condition. The event it writes
        // matches neither guard.
        let settled = [tagged_event("SettlementRun", &[("meter", "m1")])];
        let per_guard = condition_after(quiet_query.clone(), quiet_boundary.get())
            .and_guard(busy_query.clone(), Some(busy_boundary));

        let admitted = store.append(&settled, Some(&per_guard)).await;
        assert!(
            admitted.is_ok(),
            "VT-30: each guard MUST be evaluated against its OWN boundary. \
             Neither fragment has moved since the caller read it — no \
             `TariffPublished` above {quiet_boundary:?}, no `MeterReadingTaken` \
             above {busy_boundary:?} — so the append MUST be admitted. A store \
             that collapses the guards to `min(after)` re-admits every event \
             above that minimum for EVERY guard, so the quiet fragment's stale \
             boundary governs the busy one and a consistency boundary that never \
             conflicted starts refusing. Got {admitted:?}"
        );

        // The anchor: the same two fragments with the busy guard unbounded must
        // be refused, so the admission above is a verdict rather than a probe
        // that never fired.
        let unbounded =
            condition_after(quiet_query, quiet_boundary.get()).and_guard(busy_query, None);
        let refused = store.append(&settled, Some(&unbounded)).await;
        assert!(
            refused.is_err(),
            "and the admission above must be the guards' verdict rather than \
             their absence: with the busy guard unbounded, the \
             `MeterReadingTaken` the store already holds violates it and the \
             append MUST be refused. Got {refused:?}"
        );

        RuleOutcome::Ran
    }

    /// A one-guard condition is evaluated by the same rules as any other guard.
    ///
    /// VT-30's compatibility half: `AppendCondition::new(query)` produces a single
    /// unbounded guard, `after`/`after_opt` apply to every guard, and every
    /// existing call site keeps its meaning as the single-guard case. What that
    /// obliges of a *store* is one sentence — **the one-guard path is not exempt
    /// from the boundary semantics** — and it is the sentence a store can get
    /// wrong on its own.
    ///
    /// # What this rule adds that the `condition_after_*` family does not
    ///
    /// Every other append-condition rule in this suite builds a condition with
    /// exactly one guard, so all of them already exercise the single-guard path,
    /// and any store *uniformly* wrong about `after` is caught by one of them.
    /// What none of them can see is a store wrong about `after` on **one path
    /// only**, because none of them ever builds the other path.
    ///
    /// That is the regression VT-30's refactor invites, and it is what this rule
    /// owns. An adapter generalising to N guards keeps a `guards.len() == 1` fast
    /// path — the overwhelmingly common case, and the one where a `UNION` per
    /// guard is pure overhead — and that fast path is the *old* statement,
    /// written before boundaries existed and never re-reviewed, while the general
    /// path is new and was. `SingleGuardFastPathStore` is the compiled version:
    /// correct for two guards, and for one guard it asks only whether any event
    /// matches the query at all. `MinCollapseStore` is its mirror image — right
    /// on one guard, wrong on N — and
    /// `condition_guards_carry_independent_boundaries` catches that one.
    ///
    /// # Why two fixture instances
    ///
    /// The two spellings must be compared against stores in the **same state**,
    /// and an admitted append is a write: measuring both on one store would let
    /// the second spelling see an event the first one added, so a store whose
    /// probe is sensitive to unrelated events — `ExistenceProbeStore`,
    /// `UncorrelatedProbeStore` — would fail this rule for a reason that has
    /// nothing to do with guard arity. Each instance is driven to completion
    /// before the next is opened, so a fixture that failed to isolate them fails
    /// `two_fixture_instances_observe_none_of_each_others_appends`, which owns
    /// that, rather than this.
    pub async fn condition_with_one_guard_behaves_as_today<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let first = open().await;
        let one_guard = probe_one_and_two_guard_spellings(&first.connect().await, 1).await;

        let second = open().await;
        let two_guards = probe_one_and_two_guard_spellings(&second.connect().await, 2).await;

        assert_eq!(
            one_guard.stale_admitted,
            two_guards.stale_admitted,
            "VT-30: a one-guard condition is evaluated by the same rules as any \
             other guard. With a boundary BELOW a matching event, the one-guard \
             spelling {} the append and the identical condition written twice {} \
             it — a store with a `guards.len() == 1` fast path that predates the \
             boundary semantics has two answers to one question, and the answer \
             a caller gets depends on how many fragments its decision model \
             happened to read",
            if one_guard.stale_admitted {
                "admitted"
            } else {
                "refused"
            },
            if two_guards.stale_admitted {
                "admitted"
            } else {
                "refused"
            }
        );
        assert_eq!(
            one_guard.current_admitted,
            two_guards.current_admitted,
            "and with a boundary ABOVE every matching event: one guard {} the \
             append, two identical guards {} it",
            if one_guard.current_admitted {
                "admitted"
            } else {
                "refused"
            },
            if two_guards.current_admitted {
                "admitted"
            } else {
                "refused"
            }
        );

        // The anchor. Two spellings agree trivially against a store that answers
        // the same thing to everything, so the boundary has to make a difference
        // for the agreement to mean anything.
        assert!(
            !one_guard.stale_admitted && one_guard.current_admitted,
            "the anchor: the two boundaries must be told apart at all. A matching \
             event above the boundary MUST refuse the append and a boundary above \
             every match MUST admit it — otherwise the agreement asserted above \
             is agreement about nothing. Stale boundary admitted: {}; current \
             boundary admitted: {}",
            one_guard.stale_admitted,
            one_guard.current_admitted
        );

        RuleOutcome::Ran
    }

    /// What one spelling of a condition answered at each of two boundaries.
    ///
    /// Booleans rather than the `Result`s themselves, because the two spellings
    /// are measured against two different stores and the claim is only that the
    /// **verdicts** agree; carrying `AppendError<S::Error>` out of the helper
    /// would put the fixture's error type into this rule's signature for a value
    /// it never inspects.
    struct GuardSpellingVerdicts {
        /// Whether the append was admitted with the boundary below a matching
        /// event.
        stale_admitted: bool,
        /// Whether it was admitted with the boundary above every matching event.
        current_admitted: bool,
    }

    /// Seeds a store, then asks the boundary question twice with a condition
    /// written as `guards` copies of one clause.
    ///
    /// One copy is the spelling every caller uses; two is the same condition said
    /// twice, which is semantically identical because a conjunction of a clause
    /// with itself is that clause. A store that answers them differently has two
    /// code paths that disagree.
    ///
    /// The refusing case is asked **first**, because a refusal writes nothing and
    /// leaves the store in exactly the state the seeding produced — which is the
    /// state the admitting case below has to be measured against.
    async fn probe_one_and_two_guard_spellings<S: EventStore>(
        store: &S,
        guards: usize,
    ) -> GuardSpellingVerdicts {
        let query = query_of_types(&["Blocker"]);
        // Two matching events distinguishable by tag, so that a content-addressed
        // store has nothing to deduplicate, and a non-matching filler whose
        // position is a boundary strictly above every match.
        let stale = append_ok(store, &[tagged_event("Blocker", &[("n", "1")])]).await;
        append_ok(store, &[tagged_event("Blocker", &[("n", "2")])]).await;
        let current = append_ok(store, &[tagged_event("Filler", &[("n", "3")])]).await;

        let build = |boundary: SequencePosition| {
            let mut built = condition_after(query.clone(), boundary.get());
            for _ in 1..guards {
                built = built.and_guard(query.clone(), Some(boundary));
            }
            built
        };

        // Both conditions are bound to locals rather than passed as temporaries:
        // `append` borrows them across an await, and a temporary living only to
        // the end of the statement is the shape that reads as fine and is one
        // refactor from `error[E0716]`.
        let stale_condition = build(stale);
        let stale_admitted = store
            .append(
                &[tagged_event("Attempt", &[("n", "stale")])],
                Some(&stale_condition),
            )
            .await
            .is_ok();

        let current_condition = build(current);
        let current_admitted = store
            .append(
                &[tagged_event("Attempt", &[("n", "current")])],
                Some(&current_condition),
            )
            .await
            .is_ok();

        GuardSpellingVerdicts {
            stale_admitted,
            current_admitted,
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
    // Read isolation (ES-11, ES-12)
    //
    // Both rules poll the stream **once before they append**, and the two
    // helpers below are what let them. `poll_once`, beside the visibility rule,
    // does the same for two `append` futures; the pause point has to be one the
    // rule controls, or it has to come from a fixture nothing in the tree can
    // build.
    // ---------------------------------------------------------------------

    /// One `read` is one sample: an append made after the stream has been polled
    /// does not appear in it.
    ///
    /// ES-11. The store is read once to fix the expected answer, a stream is
    /// opened over the same query, **polled once**, an event is appended, and the
    /// stream is then drained. What comes out must be exactly what was visible
    /// before the append.
    ///
    /// # The leading poll is the whole reason this rule is portable
    ///
    /// It reads as an incidental detail and it is not. The contract fixes the
    /// sample **no later than the first poll**, and deliberately does not say
    /// whether it is taken at call time or deferred: `MemoryEventStore` filters,
    /// orders and truncates under the read lock when `read` is called, and
    /// `happenstance-sqlite` cannot — `spawn_blocking` panics with no runtime in
    /// scope and `read` is not `async`, so its work has to move into `poll_next`.
    /// Both are conformant. A rule that appended **before** the first poll would
    /// therefore fail the deferring adapter at random while passing the buffering
    /// one, and the failure would read as a flake rather than as a rule asking
    /// the wrong question. One poll first, and both shapes answer the same.
    ///
    /// A stream that answers `Poll::Pending` on that first poll has still been
    /// polled, and nothing in the specification says a read stream is ready
    /// immediately — `PagedStreamStore` is the conformant control for exactly
    /// that. So the leading poll's *result* is kept where there is one and
    /// ignored where there is not.
    ///
    /// # What it rejects
    ///
    /// `RefetchingPagedStore`: a store with no cursor issuing an independent
    /// statement per page against whatever it holds *now*. That is the natural
    /// shape for a one-shot-HTTP adapter, and it is what ES-11's ceiling — capture
    /// a position ceiling *H* no later than the first poll and bound every later
    /// statement by `position <= H` — exists to require. It also rejects
    /// `BorrowHoldingStore`, whose stream holds the store open so the append in
    /// the middle of this rule cannot happen at all.
    ///
    /// # What it deliberately does not assert
    ///
    /// Nothing about *laziness*. Whether the adapter did its work at call time or
    /// at the first poll is unobservable through the port and is the adapter's
    /// business; a MUST no rule can check is the decorative-rule failure with the
    /// arrow reversed.
    pub async fn read_result_is_stable_under_concurrent_append<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        // One distinct tag each, for the reason
        // `limit_applies_across_items_not_per_item` states in full.
        let one = append_ok(&store, &[tagged_event("Seeded", &[("writer", "b1")])]).await;
        let two = append_ok(&store, &[tagged_event("Seeded", &[("writer", "b2")])]).await;
        let three = append_ok(&store, &[tagged_event("Seeded", &[("writer", "b3")])]).await;

        let before = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert_eq!(
            positions_of(&before),
            [one.get(), two.get(), three.get()],
            "all three seeded events must be readable back before a claim about \
             what a later read may not add means anything"
        );

        // ES-13: the returned stream captures the query's lifetime, so whatever
        // owns the stream must own the query. Inside one function that is a named
        // local; a stream that escaped this function would need a parameter,
        // because a local does not outlive the function that declared it.
        let query = Query::all();
        let mut stream = pin!(store.read(&query, ReadOptions::new()));

        let mut drained: Vec<SequencedEvent> = Vec::new();
        match poll_stream_once(stream.as_mut()).await {
            Poll::Ready(Some(Ok(event))) => drained.push(event),
            Poll::Ready(Some(Err(err))) => {
                panic!("the first poll of a read stream must not fail, got {err:?}")
            }
            // Neither is a violation: a stream may legally not be ready on its
            // first poll, and one that ends there is caught by the comparison
            // below rather than here.
            Poll::Ready(None) | Poll::Pending => {}
        }

        let later = append_ok(&store, &[tagged_event("Later", &[("writer", "after")])]).await;

        drained.extend(drain_rest(stream.as_mut()).await);

        assert_eq!(
            snapshot_of(&drained),
            snapshot_of(&before),
            "one `read` is evaluated against ONE state of the store, fixed no \
             later than the first poll of its stream. A store taking a fresh \
             sample per page grows under the caller's feet — and the caller then \
             derives its append condition's boundary from a maximum position \
             sitting above an event it never saw, so the condition tells the store \
             to ignore exactly what it missed and a write that should have been \
             rejected is accepted"
        );
        assert!(
            !drained.iter().any(|event| event.position == later),
            "and the event appended after the first poll must not be among them"
        );

        // The anchor for the append itself: without it a store whose `append`
        // silently did nothing would satisfy every assertion above.
        let after = read_ok(&store, &Query::all(), ReadOptions::new()).await;
        assert!(
            after.iter().any(|event| event.position == later),
            "the appended event must really have landed — a read issued after the \
             stream was drained must see it"
        );

        RuleOutcome::Ran
    }

    /// Every item of one `Query` is evaluated against the same sample.
    ///
    /// ES-12, and it is strictly harder than ES-11: an adapter issuing one
    /// statement per `QueryItem` satisfies ES-11 *per item* and still fails this.
    ///
    /// The shape is `read_result_is_stable_under_concurrent_append`'s with two
    /// changes, and both of them are the rule: the query has **two items**, and
    /// the event appended after the leading poll matches the **later** one. A
    /// store evaluating item 1's statement at the first poll and item 2's at some
    /// later poll picks the new event up in the second statement.
    ///
    /// # Why the failure is quieter than a tear looks
    ///
    /// The missed event's position sits *below* the maximum the read observed.
    /// `read_decision_model` hands that maximum to `AppendCondition::after_opt`,
    /// and the condition then says "reject if anything matched after *P*" —
    /// which is precisely the instruction to ignore the event that was missed. A
    /// torn read does not become a rejected append; it becomes an **accepted**
    /// one.
    ///
    /// # The two items carry distinct type lists on purpose
    ///
    /// `ItemDedupByTypeStore` collapses items sharing a type list, so two
    /// tag-only items would make it fail this rule at its anchor for a reason
    /// `query_union_is_item_concatenation` already owns. "Alpha" before "Omega"
    /// for the matching reason: item order and position order agree, so no
    /// ordering mutant is caught here either.
    ///
    /// It rejects `RefetchingPagedStore`, the same store ES-11's rule rejects —
    /// re-sampling per page is re-sampling per item at a different granularity,
    /// which is ADR-0011's finding that **ES-12 is discharged by ES-11's ceiling
    /// rather than by a second mechanism**. `BorrowHoldingStore` fails it too,
    /// for its own reason.
    pub async fn query_items_share_one_snapshot<F: Fixture>(
        open: impl AsyncFn() -> F,
    ) -> RuleOutcome {
        let fixture = open().await;
        let store = fixture.connect().await;

        let alpha = append_ok(&store, &[tagged_event("Alpha", &[("item", "first")])]).await;
        let omega = append_ok(&store, &[tagged_event("Omega", &[("item", "last")])]).await;

        // Two disjoint items, and the second is what the late append will match.
        let query = query_of_items([item_of_types(&["Alpha"]), item_of_types(&["Omega"])]);

        let before = read_ok(&store, &query, ReadOptions::new()).await;
        assert_eq!(
            positions_of(&before),
            [alpha.get(), omega.get()],
            "both items must select their own event before a claim about the \
             sample they share means anything"
        );

        let mut stream = pin!(store.read(&query, ReadOptions::new()));

        let mut drained: Vec<SequencedEvent> = Vec::new();
        match poll_stream_once(stream.as_mut()).await {
            Poll::Ready(Some(Ok(event))) => drained.push(event),
            Poll::Ready(Some(Err(err))) => {
                panic!("the first poll of a read stream must not fail, got {err:?}")
            }
            Poll::Ready(None) | Poll::Pending => {}
        }

        // Matches the LAST item of the query. A store emitting one statement per
        // item has not reached that item yet.
        // A distinct tag, so this is a *different* event from the seeded Omega
        // while still matching the query's last item by type. Repeating the seed
        // byte for byte would make `PayloadDedupStore` collapse the two, and this
        // rule would then fail it for a reason
        // `reissued_unconditional_batch_lands_twice` already owns.
        let late = append_ok(&store, &[tagged_event("Omega", &[("item", "late")])]).await;

        drained.extend(drain_rest(stream.as_mut()).await);

        assert_eq!(
            snapshot_of(&drained),
            snapshot_of(&before),
            "every item of one `Query` must be evaluated against the same sample \
             as every other item of that same `read`. An adapter emitting one \
             statement per `QueryItem` picks the late event up in a later \
             statement, and the caller's condition boundary — derived from the \
             maximum position this read observed — then tells the store to ignore \
             exactly what was missed"
        );
        assert!(
            !drained.iter().any(|event| event.position == late),
            "and the event matching the query's last item must not be among them"
        );

        let after = read_ok(&store, &query, ReadOptions::new()).await;
        assert!(
            after.iter().any(|event| event.position == late),
            "the appended event must really have landed and really have matched \
             the query — otherwise this rule tested nothing"
        );

        RuleOutcome::Ran
    }

    /// Polls a stream exactly once, whatever it answers.
    ///
    /// `poll_fn`'s own future is `Ready` whatever the inner one said, which is
    /// what makes this *poll once* rather than *await* — and what keeps
    /// `block_on` from parking here, so a store that answers `Pending` cannot
    /// hang the caller at this line. It is the stream-shaped sibling of
    /// [`poll_once`], and the two isolation rules need it for the reason the
    /// visibility rule needs that one: the pause point has to be one the rule
    /// controls.
    async fn poll_stream_once<S: Stream>(mut stream: Pin<&mut S>) -> Poll<Option<S::Item>> {
        core::future::poll_fn(|cx| Poll::Ready(stream.as_mut().poll_next(cx))).await
    }

    /// Drains what is left of a stream that has already been polled.
    ///
    /// `collect` cannot serve here and the reason is its signature rather than
    /// its body: it takes the stream **by value** and pins it itself, and these
    /// two rules hold their stream across an append, so what they have is a
    /// `Pin<&mut _>` they must keep. The loop is `collect`'s, with the error arm
    /// turned into the panic a rule wants rather than a `Result` every call site
    /// would have to unwrap.
    async fn drain_rest<S, E>(mut stream: Pin<&mut S>) -> Vec<SequencedEvent>
    where
        S: Stream<Item = Result<SequencedEvent, E>>,
        E: core::fmt::Debug,
    {
        let mut rest = Vec::new();
        core::future::poll_fn(|cx| {
            loop {
                match stream.as_mut().poll_next(cx) {
                    Poll::Ready(Some(Ok(event))) => rest.push(event),
                    Poll::Ready(Some(Err(err))) => {
                        panic!("the stream must still drain after the append, got {err:?}")
                    }
                    Poll::Ready(None) => return Poll::Ready(()),
                    Poll::Pending => return Poll::Pending,
                }
            }
        })
        .await;
        rest
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
