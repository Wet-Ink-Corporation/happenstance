//! The write-group family (#420): a store reads its log back in the writes
//! that made it (SY-39).
//!
//! Mounted with [`write_group_store_conformance!`](crate::write_group_store_conformance)
//! on the same [`IngestFixture`](crate::IngestFixture) the ingest family takes,
//! whose store must also implement
//! [`WriteGroupStore`](happenstance_sync::WriteGroupStore). No new fixture
//! trait and no bound added to `IngestFixture::Store`, so an adapter that has
//! not implemented the port yet still mounts the ingest family unchanged.

/// The write-group rules. See the [crate documentation](crate) for what each
/// covers.
///
/// Each rule is an async function taking `impl AsyncFn() -> F`, how to make a
/// fixture, and returning `Result<RuleOutcome, Violation>`, as the ingest
/// family's do.
///
/// # What no rule here does
///
/// **Assert a local position it did not read back.** Every `first` compared
/// is the position the store assigned to the group's first event, read through
/// [`EventStore::read`](happenstance_core::EventStore::read). The literal
/// positions below are **origin** positions: facts a peer supplies.
///
/// **Decode a payload** (SY-35). Groups are told apart by their opaque bytes.
pub mod write_group_rules {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventStore, EventType, Query, QueryItem, ReadOptions,
        RecordedAt, SequencePosition, StoreId, collect,
    };
    use happenstance_sync::{
        GroupPage, IngestGroup, IngestStore, ReplicatedEvent, Waiting, WriteGroup, WriteGroupStore,
    };
    use happenstance_testkit::RuleOutcome;

    use crate::IngestFixture;
    use crate::violation::{Violation, ensure};

    /// Every rule's clause.
    const CLAUSE: &str = "SY-39";

    /// A page large enough for every arrangement here: 64 events.
    const WHOLE_LOG: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

    /// The most pages the chaining rule reads before it decides the store will
    /// never reach the end (H-13).
    const MAX_PAGES: usize = 64;

    const APPENDED: EventType = EventType::from_static("GroupAppended");
    const SHIPPED: EventType = EventType::from_static("GroupShipped");
    const REVERSED: EventType = EventType::from_static("GroupReversed");

    /// The origin of every foreign group here.
    const ORIGIN: StoreId = StoreId::from_bytes([0x79; 16]);

    /// SY-39: one `append` reads back as one write group, named by the position
    /// the store gave its first event, carrying no guard.
    ///
    /// Two appends, of three events and of one, then one page of the whole log:
    /// two groups, `[3, 1]`, in order, each `first` equal to the position the
    /// store assigned that append's first event.
    ///
    /// # Rejects
    ///
    /// A store that records no groups, so every row reads as its own, and one
    /// that merges adjacent appends into one group. Both are in
    /// `tests/wrong_write_group_stores.rs`.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses a write or a read, or
    /// reads back other groups.
    pub async fn an_append_reads_back_as_one_group<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        append(&store, &["append-a1", "append-a2", "append-a3"]).await?;
        append(&store, &["append-b1"]).await?;

        let page = read(&store, None, WHOLE_LOG).await?;

        expect_shape(
            &page,
            &[&["append-a1", "append-a2", "append-a3"], &["append-b1"]],
        )?;
        let expected = [
            position_of(&store, "append-a1").await?,
            position_of(&store, "append-b1").await?,
        ];
        let firsts: Vec<SequencePosition> = page.groups.iter().map(WriteGroup::first).collect();
        ensure(firsts == expected, CLAUSE, || {
            format!(
                "the store named its two appends {firsts:?}; the positions it gave their first \
                 events are {expected:?}"
            )
        })?;
        ensure(
            page.groups.iter().all(|g| g.guard().is_none()),
            CLAUSE,
            || String::from("a local append read back with a guard; a local append records none"),
        )?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-39: an ingested group keeps its boundary, the origin's identities,
    /// and the guard it arrived with.
    ///
    /// One `ingest` of two groups, `[x1, x2]` under a position-free guard and
    /// `[y1]` under none, read back as two groups, with the origin's
    /// identities and the guards `[Some(guard), None]`.
    ///
    /// # Rejects
    ///
    /// A store that drops the guard, and one that merges what one call
    /// ingested into one group.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses the ingest or a read, or
    /// reads back other groups, identities or guards.
    pub async fn an_ingested_group_keeps_its_boundary_and_guard<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store).await?;
        let guard = guard()?;
        let x = [shipped(4, "ingest-x1")?, shipped(5, "ingest-x2")?];
        let y = [shipped(6, "ingest-y1")?];

        ingest(
            &store,
            &[
                IngestGroup::new(&x, &[]).with_guard(Some(&guard)),
                IngestGroup::new(&y, &[]),
            ],
        )
        .await?;
        let page = read(&store, None, WHOLE_LOG).await?;

        expect_shape(&page, &[&["ingest-x1", "ingest-x2"], &["ingest-y1"]])?;
        let ids: Vec<EventId> = events(&page).map(|event| event.id).collect();
        let sent: Vec<EventId> = x.iter().chain(&y).map(|event| event.id).collect();
        ensure(ids == sent, CLAUSE, || {
            format!("the store read back identities {ids:?} for the origin's {sent:?}")
        })?;
        let guards: Vec<Option<&AppendCondition>> =
            page.groups.iter().map(WriteGroup::guard).collect();
        ensure(guards == [Some(&guard), None], CLAUSE, || {
            format!(
                "the store read back guards {guards:?}; the first group arrived under \
                 {guard:?} and the second under none"
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-39, with SY-2: a compensation is one write group with the ingested
    /// group it answers, after it.
    ///
    /// One `ingest` of `[losing]` answered by `[reversal]`: one group holding
    /// both, the losing event first.
    ///
    /// # Rejects
    ///
    /// A store that records the compensation as a group of its own, which a
    /// relay would forward in a separate atomic unit (SY-2).
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses the ingest or a read, or
    /// reads back other groups.
    pub async fn compensation_joins_its_group<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store).await?;
        let losing = [shipped(9, "compensated-losing")?];
        let reversal = [opaque(REVERSED, "compensated-reversal")?];

        ingest(&store, &[IngestGroup::new(&losing, &reversal)]).await?;
        let page = read(&store, None, WHOLE_LOG).await?;

        expect_shape(&page, &[&["compensated-losing", "compensated-reversal"]])?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-39, with SY-11: a group whose first event is a re-delivery is named
    /// by the first row it **landed**, and keeps its guard.
    ///
    /// `[held]` is ingested alone, then a local append of two, then `[held,
    /// new1, new2]` under a position-free guard. The re-delivered `held` is
    /// skipped, so the third write lands two rows: three groups,
    /// `[held]`, `[local1, local2]` and `[new1, new2]`, the last carrying the
    /// guard, with `first` equal to the position the store gave `new1`.
    ///
    /// # Rejects
    ///
    /// A store that names a group by the first position it **allocated** for
    /// it rather than the first row it wrote. On a store whose sequence burns
    /// a value for each skipped re-delivery, as a Postgres sequence and
    /// SQLite's `AUTOINCREMENT` both do, that position belongs to no row of
    /// the group. Only the in-memory stores burn nothing and cannot make the
    /// mistake. The wrong store is in
    /// `tests/wrong_write_group_stores.rs`.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses a write or a read, reads
    /// back other groups, names the guarded group by another position, or
    /// drops its guard.
    pub async fn a_group_after_a_redelivery_is_named_by_its_first_landed_row<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store).await?;
        let guard = guard()?;
        let held = [shipped(21, "redelivered-held")?];
        let redelivered = [
            shipped(21, "redelivered-held")?,
            shipped(22, "redelivered-new1")?,
            shipped(23, "redelivered-new2")?,
        ];

        ingest(&store, &[IngestGroup::new(&held, &[])]).await?;
        append(&store, &["redelivered-local1", "redelivered-local2"]).await?;
        ingest(
            &store,
            &[IngestGroup::new(&redelivered, &[]).with_guard(Some(&guard))],
        )
        .await?;
        let page = read(&store, None, WHOLE_LOG).await?;

        expect_shape(
            &page,
            &[
                &["redelivered-held"],
                &["redelivered-local1", "redelivered-local2"],
                &["redelivered-new1", "redelivered-new2"],
            ],
        )?;
        let landed = position_of(&store, "redelivered-new1").await?;
        let named = page.groups.last().map(WriteGroup::first);
        ensure(named == Some(landed), CLAUSE, || {
            format!(
                "the store named the group that landed after a re-delivery {named:?}; the first \
                 row it wrote is at {landed}"
            )
        })?;
        let kept = page.groups.last().and_then(WriteGroup::guard);
        ensure(kept == Some(&guard), CLAUSE, || {
            format!(
                "the group that landed after a re-delivery read back with the guard {kept:?}; \
                 it arrived under {guard:?}"
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-39: a read never returns part of a group. It stops before the group
    /// that would take the page past `max_events`, and returns a first group
    /// whole however large it is.
    ///
    /// Appends of two, three and two events. `read_groups(None, 4)` returns the
    /// first group alone, because the second would take the page to five; then
    /// `read_groups(after = its first, 1)` returns the three-event group whole.
    ///
    /// # Rejects
    ///
    /// A store that fills a page by event and cuts the group that crosses it,
    /// one that skips a first group larger than the page, and one that resumes
    /// above a row position rather than a group's first, which hands back a
    /// group's tail as if it were a group.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses a write or a read, or a
    /// page carries other groups.
    pub async fn a_read_never_cuts_a_group<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        append(&store, &["cut-a1", "cut-a2"]).await?;
        append(&store, &["cut-b1", "cut-b2", "cut-b3"]).await?;
        append(&store, &["cut-c1", "cut-c2"]).await?;

        let head = read(&store, None, budget(4)?).await?;
        expect_shape(&head, &[&["cut-a1", "cut-a2"]])?;

        let after = head.groups.first().map(WriteGroup::first);
        let oversized = read(&store, after, budget(1)?).await?;
        expect_shape(&oversized, &[&["cut-b1", "cut-b2", "cut-b3"]])?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-39: pages chained on `after = last.first()` return every group
    /// exactly once, in order, and only the read that reached the end says
    /// [`Waiting::Nothing`].
    ///
    /// Appends of two, one, two and one events, read two events at a time:
    /// each page stops before a group that would overflow it, so every page
    /// but the last ends at a group boundary with the log going on.
    ///
    /// # Rejects
    ///
    /// A store that says nothing is waiting on a page its budget cut, one that
    /// resumes above a row position, and one that cuts a group to fill a page.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-39 if the store refuses a write or a read, hands
    /// back a group twice or never, or says [`Waiting::Nothing`] before the end
    /// or not at it.
    pub async fn pages_chain_on_first_and_end_with_nothing<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: IngestFixture,
        F::Store: WriteGroupStore,
    {
        let fixture = open().await;
        let store = fixture.connect().await;
        let written: [&[&str]; 4] = [
            &["chain-a1", "chain-a2"],
            &["chain-b1"],
            &["chain-c1", "chain-c2"],
            &["chain-d1"],
        ];
        for group in written {
            append(&store, group).await?;
        }

        let pages = chain(&store, budget(2)?).await?;

        let read: Vec<Vec<String>> = pages.iter().flat_map(shape).collect();
        let expected: Vec<Vec<String>> = written.iter().map(|group| owned(group)).collect();
        ensure(read == expected, CLAUSE, || {
            format!("chained pages read back groups {read:?}; the log holds {expected:?}")
        })?;
        let waiting: Vec<Waiting> = pages.iter().map(|page| page.waiting).collect();
        let Some((last, before)) = waiting.split_last() else {
            return Err(Violation::new(CLAUSE, "the store returned no page at all"));
        };
        ensure(
            *last == Waiting::Nothing && !before.contains(&Waiting::Nothing),
            CLAUSE,
            || {
                format!(
                    "the pages said {waiting:?}; only the last, which reached the end of the \
                     log, may say Nothing, and it must"
                )
            },
        )?;
        Ok(RuleOutcome::Ran)
    }

    /// Every non-empty page from the start, each read above the last group's
    /// first; stops at the first empty page.
    async fn chain<S>(store: &S, max: NonZeroUsize) -> Result<Vec<GroupPage>, Violation>
    where
        S: WriteGroupStore,
    {
        let mut pages = Vec::new();
        let mut after = None;
        for _ in 0..MAX_PAGES {
            let page = read(store, after, max).await?;
            let Some(last) = page.groups.last() else {
                return Ok(pages);
            };
            after = Some(last.first());
            pages.push(page);
        }
        Err(Violation::new(
            CLAUSE,
            format!("the store was still handing back groups after {MAX_PAGES} pages"),
        ))
    }

    /// The page's groups match `expected`, payload by payload.
    fn expect_shape(page: &GroupPage, expected: &[&[&str]]) -> Result<(), Violation> {
        let read = shape(page);
        let expected: Vec<Vec<String>> = expected.iter().map(|group| owned(group)).collect();
        ensure(read == expected, CLAUSE, || {
            format!("the store read back groups {read:?} where it wrote {expected:?}")
        })
    }

    /// Each group's payloads, as text: every payload here is ASCII.
    fn shape(page: &GroupPage) -> Vec<Vec<String>> {
        page.groups
            .iter()
            .map(|group| {
                group
                    .events()
                    .iter()
                    .map(|event| String::from_utf8_lossy(event.event.data()).into_owned())
                    .collect()
            })
            .collect()
    }

    fn owned(group: &[&str]) -> Vec<String> {
        group.iter().map(|payload| String::from(*payload)).collect()
    }

    fn events(page: &GroupPage) -> impl Iterator<Item = &ReplicatedEvent> + '_ {
        page.groups.iter().flat_map(|group| group.events().iter())
    }

    fn budget(events: usize) -> Result<NonZeroUsize, Violation> {
        NonZeroUsize::new(events).ok_or_else(|| Violation::new(CLAUSE, "a page budget of zero"))
    }

    /// One `append` of an event per payload.
    async fn append<S: EventStore>(store: &S, payloads: &[&str]) -> Result<(), Violation> {
        let events = payloads
            .iter()
            .map(|payload| opaque(APPENDED, payload))
            .collect::<Result<Vec<Event>, Violation>>()?;
        EventStore::append(store, &events, None)
            .await
            .map(|_| ())
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the store refused an append: {error}"))
            })
    }

    async fn ingest<S: IngestStore>(
        store: &S,
        groups: &[IngestGroup<'_>],
    ) -> Result<(), Violation> {
        IngestStore::ingest(store, groups)
            .await
            .map(|_| ())
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the store refused an ingest: {error}"))
            })
    }

    async fn read<S: WriteGroupStore>(
        store: &S,
        after: Option<SequencePosition>,
        max: NonZeroUsize,
    ) -> Result<GroupPage, Violation> {
        WriteGroupStore::read_groups(store, after, max)
            .await
            .map_err(|error| {
                Violation::new(CLAUSE, format!("the store refused a group read: {error}"))
            })
    }

    /// The position the store gave the event carrying `payload`.
    async fn position_of<S: EventStore>(
        store: &S,
        payload: &str,
    ) -> Result<SequencePosition, Violation> {
        let log = collect(EventStore::read(store, &Query::all(), ReadOptions::new()))
            .await
            .map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!("the store refused a read of its log: {error}"),
                )
            })?;
        log.iter()
            .find(|row| row.event.data().as_ref() == payload.as_bytes())
            .map(|row| row.position)
            .ok_or_else(|| Violation::new(CLAUSE, format!("{payload} is not in the log")))
    }

    /// Fails a fixture whose store has the rules' origin as its own identity.
    async fn ensure_foreign<S: IngestStore>(store: &S) -> Result<(), Violation> {
        let own = IngestStore::store_id(store).await.map_err(|error| {
            Violation::new(CLAUSE, format!("the store refused its identity: {error}"))
        })?;
        ensure(own != ORIGIN, CLAUSE, || {
            format!("the fixture's store has the incarnation {own}, the rules' foreign origin")
        })
    }

    /// A position-free guard: evidence that travels, never refused (SY-6).
    fn guard() -> Result<AppendCondition, Violation> {
        let item = QueryItem::of_types([SHIPPED]).map_err(|error| {
            Violation::new(CLAUSE, format!("the rule's own query is invalid: {error}"))
        })?;
        Ok(AppendCondition::new(Query::from_item(item)))
    }

    fn shipped(position: u64, payload: &str) -> Result<ReplicatedEvent, Violation> {
        let at = SequencePosition::new(position)
            .ok_or_else(|| Violation::new(CLAUSE, "the rule's origin position is zero"))?;
        Ok(ReplicatedEvent::new(
            EventId::new(ORIGIN, at),
            RecordedAt::from_millis(1_800_000_000_000),
            opaque(SHIPPED, payload)?,
        ))
    }

    fn opaque(event_type: EventType, payload: &str) -> Result<Event, Violation> {
        Event::new(event_type, payload.as_bytes().to_vec()).map_err(|error| {
            Violation::new(CLAUSE, format!("the rule's own event is invalid: {error}"))
        })
    }
}

/// Hands the complete write-group rule set to `$callback`.
///
/// This family's single enumeration: `$callback` is the path of a macro that
/// accepts a comma-separated list of identifiers, invoked once with every rule
/// in [`write_group_rules`].
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_write_group_rule!(rule_names);
/// assert!(names.contains(&"a_read_never_cuts_a_group"));
/// ```
#[macro_export]
macro_rules! for_each_write_group_rule {
    // Raw token trees rather than `$cb:path`, for the reason the event-store
    // family's enumeration records.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- One write reads back as one group (SY-39) ------------------
            an_append_reads_back_as_one_group,
            an_ingested_group_keeps_its_boundary_and_guard,
            compensation_joins_its_group,
            a_group_after_a_redelivery_is_named_by_its_first_landed_row,

            // --- A page never cuts a group, and pages chain (SY-39) ---------
            a_read_never_cuts_a_group,
            pages_chain_on_first_and_end_with_nothing,
        }
    };
}

/// Emits one `#[tokio::test]` per write-group rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`. The default emitter of
/// [`write_group_store_conformance!`](crate::write_group_store_conformance).
#[macro_export]
macro_rules! emit_write_group_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::write_group_rules::$name(__write_group_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one plain `#[test]` per write-group rule, driven by
/// [`happenstance_testkit::block_on`]: no runtime and no `Send` bound.
#[macro_export]
macro_rules! emit_write_group_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::block_on(
                    $crate::__private::write_group_rules::$name(__write_group_fixture),
                )?
                .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one `#[wasm_bindgen_test]` per write-group rule.
///
/// The caller's crate needs `wasm-bindgen-test` in its `dev-dependencies`,
/// gated on `cfg(target_arch = "wasm32")`. A skip is logged through
/// `console_log!` rather than reported on stdout, which writes nowhere on that
/// target: [`emit_ingest_wasm!`](crate::emit_ingest_wasm)'s twin, and the
/// harness the Durable Object's ingest store runs this family under.
#[macro_export]
macro_rules! emit_write_group_wasm {
    ($($name:ident),* $(,)?) => {
        $(
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                let __outcome =
                    $crate::__private::write_group_rules::$name(__write_group_fixture).await?;
                if let Some(__line) = __outcome.skip_line(::core::stringify!($name)) {
                    ::wasm_bindgen_test::console_log!("{}", __line);
                }
                Ok(())
            }
        )*
    };
}

/// Expands to a `[&str; N]` of the rule names, for the orphan meta-test.
///
/// Not an emitter and not promised; exported only because `macro_rules!` has no
/// other way to reach another crate's tests.
#[doc(hidden)]
#[macro_export]
macro_rules! __write_group_rule_names {
    ($($name:ident),* $(,)?) => { [ $( ::core::stringify!($name) ),* ] };
}

/// No rule exists without appearing in the enumeration, and none is
/// enumerated without existing: `no_orphan_ingest_rules`, one family over.
#[cfg(test)]
#[test]
fn no_orphan_write_group_rules() {
    const SOURCE: &str = include_str!("write_group_suite.rs");
    const PREFIX: &str = "    pub async fn ";

    let registered = crate::for_each_write_group_rule!(crate::__write_group_rule_names);
    let mut declared: Vec<&str> = SOURCE
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .filter_map(|rest| {
            rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
        })
        .collect();

    assert!(
        !declared.is_empty(),
        "the source scan matched no rule at all"
    );
    declared.sort_unstable();
    let mut registered_sorted = registered.to_vec();
    registered_sorted.sort_unstable();
    assert_eq!(
        declared, registered_sorted,
        "`for_each_write_group_rule!` and the rules declared in `write_group_suite.rs` disagree"
    );
}
