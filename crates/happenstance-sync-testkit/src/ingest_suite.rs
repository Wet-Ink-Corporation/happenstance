//! The ingest rules, their one enumeration, and the two emitters.
//!
//! Private: what a caller reaches is [`ingest_rules`], re-exported at the
//! crate root, and the macros, which `#[macro_export]` puts there regardless.

/// The ingest rules. See the [crate documentation](crate) for what each covers.
///
/// Each rule is an async function taking `impl AsyncFn() -> F`, how to make a
/// fixture rather than a fixture, as in the event-store family, and returning
/// `Result<RuleOutcome, Violation>`.
///
/// # What no rule here does
///
/// **Decode a payload** (SY-35). Events are compared whole, identity, recorded
/// time and the opaque bytes, and never parsed.
///
/// **Assert a local position it did not read back.** The specification permits
/// gaps, so every local position compared is one the store assigned. The
/// literal positions below are **origin** positions: facts a peer supplies.
///
/// **Locate an event by identity, except VT-5's.** The other rules find what
/// they wrote by its distinct opaque bytes, so a store that re-mints on ingest
/// fails VT-5's rule and only that one.
pub mod ingest_rules {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec::Vec;

    use happenstance_core::{
        Event, EventId, EventStore, EventType, Query, ReadOptions, RecordedAt, SequencePosition,
        SequencedEvent, StoreId, Tag, collect,
    };
    use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent, Watermark};
    use happenstance_testkit::RuleOutcome;

    use crate::IngestFixture;
    use crate::violation::{Violation, ensure};

    const SHIPPED: EventType = EventType::from_static("Shipped");
    const LOCAL: EventType = EventType::from_static("Local");
    const COMPENSATED: EventType = EventType::from_static("Compensated");

    /// VT-5's origin.
    const IDENTITY_ORIGIN: StoreId = StoreId::from_bytes([0x71; 16]);
    /// SY-5's origin.
    const TAIL_ORIGIN: StoreId = StoreId::from_bytes([0x72; 16]);
    /// SY-12's two origins, which record one byte-identical fact each.
    const DEDUPE_ORIGIN: StoreId = StoreId::from_bytes([0x73; 16]);
    const TWIN_ORIGIN: StoreId = StoreId::from_bytes([0x74; 16]);

    /// Bytes that are not UTF-8 and parse as no codec: a lone continuation
    /// pair, a NUL and a truncated two-byte sequence. A store that had to
    /// decode a payload to dedupe it fails on these (SY-35).
    const UNDECODABLE: [u8; 5] = [0xFF, 0xFE, 0x00, 0xC3, 0x28];

    /// VT-5: an ingested event keeps the [`EventId`] its origin minted, and
    /// VT-9's [`RecordedAt`] with it.
    ///
    /// One group of two foreign events, at origin positions 4 and 9, one
    /// recorded before 1970 and one after. Read back, exactly one held event
    /// carries each identity, with the recorded time and the bytes that were
    /// sent; the store answers `contains_event_id` for both, and its
    /// watermark for the origin is the higher of the two origin positions.
    ///
    /// # Rejects
    ///
    /// A store that re-mints an ingested event under its own identity, keeping
    /// the origin's only for its own bookkeeping, and one that restamps it
    /// with the arrival time. `tests/wrong_ingest_stores.rs` carries both.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of VT-5 if the store refuses the ingest, miscounts it,
    /// or holds anything but the sent identity, time and bytes.
    pub async fn ingest_preserves_origin_identity<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "VT-5";
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store, &[IDENTITY_ORIGIN], CLAUSE)?;
        let sent = [
            shipped(IDENTITY_ORIGIN, 4, -86_400_000, b"vt5-before-1970", CLAUSE)?,
            shipped(IDENTITY_ORIGIN, 9, 1_700_000_000_000, b"vt5-after", CLAUSE)?,
        ];

        let ingested = ingest(&store, &[IngestGroup::new(&sent, &[])], CLAUSE).await?;

        ensure(ingested.appended == 2, CLAUSE, || {
            format!("the store reported {ingested:?} for two events it did not hold")
        })?;
        let log = read_all(&store, CLAUSE).await?;
        for event in &sent {
            let carrying: Vec<&SequencedEvent> =
                log.iter().filter(|row| row.id == event.id).collect();
            let [landed] = carrying.as_slice() else {
                return Err(Violation::new(
                    CLAUSE,
                    format!(
                        "{} held events carry {}, which was sent once; an ingested event \
                         keeps the identity its origin minted",
                        carrying.len(),
                        event.id
                    ),
                ));
            };
            ensure(landed.recorded_at == event.recorded_at, CLAUSE, || {
                format!(
                    "{} was recorded at {} by its origin and is held as recorded at {}",
                    event.id, event.recorded_at, landed.recorded_at
                )
            })?;
            ensure(landed.event == event.event, CLAUSE, || {
                format!("{} is held with bytes other than the ones sent", event.id)
            })?;
            let held = store.contains_event_id(event.id).await.map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!("the store refused contains_event_id: {error}"),
                )
            })?;
            ensure(held, CLAUSE, || {
                format!(
                    "the store holds {} and contains_event_id denies it",
                    event.id
                )
            })?;
        }
        let watermark = watermark(&store, CLAUSE).await?;
        let highest = sent.iter().map(|event| event.id.position()).max();
        ensure(watermark.get(IDENTITY_ORIGIN) == highest, CLAUSE, || {
            format!(
                "the watermark for {IDENTITY_ORIGIN} is {:?}; the highest origin position \
                 sent was {highest:?}",
                watermark.get(IDENTITY_ORIGIN)
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-5: an ingested event, and its compensation, land at the tail, above
    /// every position assigned before the call, and the log below the old
    /// head is unchanged.
    ///
    /// Three local events are appended. One group then arrives from an origin
    /// that numbered its events 1 and 2, below the local head, which is what
    /// tempts a merge, with one compensation event. Read back, every event at
    /// or below the old head is what was there before, whole, and each foreign
    /// event and the compensation is held once, above it. Events are found by
    /// their distinct bytes, never by identity.
    ///
    /// # Rejects
    ///
    /// An order-preserving merge: a store that inserts an ingested event where
    /// its origin numbered it and renumbers what follows, so a projection that
    /// had read past that position never sees it.
    /// `tests/wrong_ingest_stores.rs` carries it.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-5 if the store refuses a write, rewrites the log
    /// below the old head, or holds a written event other than once above it.
    pub async fn ingested_events_land_above_the_local_head<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-5";
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store, &[TAIL_ORIGIN], CLAUSE)?;
        let local = [
            opaque(LOCAL, b"sy5-local-1", CLAUSE)?,
            opaque(LOCAL, b"sy5-local-2", CLAUSE)?,
            opaque(LOCAL, b"sy5-local-3", CLAUSE)?,
        ];
        let head_before = append(&store, &local, CLAUSE).await?;
        let prefix = read_all(&store, CLAUSE).await?;
        let sent = [
            shipped(TAIL_ORIGIN, 1, 1_800_000_000_000, b"sy5-foreign-1", CLAUSE)?,
            shipped(TAIL_ORIGIN, 2, 1_800_000_000_001, b"sy5-foreign-2", CLAUSE)?,
        ];
        let answer = [opaque(COMPENSATED, b"sy5-compensation", CLAUSE)?];

        ingest(&store, &[IngestGroup::new(&sent, &answer)], CLAUSE).await?;

        let log = read_all(&store, CLAUSE).await?;
        let below: Vec<&SequencedEvent> = log
            .iter()
            .filter(|row| row.position <= head_before)
            .collect();
        ensure(below.iter().copied().eq(&prefix), CLAUSE, || {
            format!(
                "the log at or below the old head {head_before} changed across an ingest: it \
                 held {} events there and now holds {}, or holds them differently",
                prefix.len(),
                below.len()
            )
        })?;
        for written in sent.iter().map(|event| &event.event).chain(&answer) {
            let landed = held_once(&log, written, CLAUSE)?;
            ensure(landed.position > head_before, CLAUSE, || {
                format!(
                    "an ingested event of type {} landed at {}, at or below the head {head_before} \
                     the store had assigned before the call",
                    written.event_type(),
                    landed.position
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// SY-12: a re-delivery is decided on the event's identity alone, which
    /// the store reaches without decoding a byte of the payload. With SY-11's
    /// store half: a group that was wholly a re-delivery writes no
    /// compensation.
    ///
    /// Every payload and metadata here is bytes that are not UTF-8 and parse
    /// as no codec. One group carries two events from one origin and a third
    /// from a second origin whose type, tags, data, metadata and recorded time
    /// are byte-identical to the first's, so only the identity tells them
    /// apart, and one compensation event. The first delivery lands all three
    /// and the compensation; the same batch again lands nothing, writes no
    /// compensation and leaves the log and head as they were; and a batch of
    /// one held event and one new one lands exactly the new one.
    ///
    /// # Rejects
    ///
    /// A store that dedupes on content, which drops the second origin's
    /// identical fact as though it were the first again, and one that writes a
    /// group's compensation again when the group was a re-delivery.
    /// `tests/wrong_ingest_stores.rs` carries both.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-12 if the store refuses an ingest, miscounts one,
    /// or holds other than what the three deliveries should leave.
    pub async fn dedupe_reaches_identity_without_decoding<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-12";
        let fixture = open().await;
        let store = fixture.connect().await;
        ensure_foreign(&store, &[DEDUPE_ORIGIN, TWIN_ORIGIN], CLAUSE)?;
        let first = undecodable(DEDUPE_ORIGIN, 1, b'1', CLAUSE)?;
        let second = undecodable(DEDUPE_ORIGIN, 2, b'2', CLAUSE)?;
        // The twin shares every byte with `first`, recorded time included:
        // the duplication is the arrangement.
        let twin = ReplicatedEvent::new(
            EventId::new(TWIN_ORIGIN, first.id.position()),
            first.recorded_at,
            first.event.clone(),
        );
        let answer = [opaque(COMPENSATED, &undecodable_bytes(b'c'), CLAUSE)?];
        let batch = [first, second.clone(), twin];
        let group = [IngestGroup::new(&batch, &answer)];

        let delivered = ingest(&store, &group, CLAUSE).await?;
        ensure_counts(delivered, (3, 0, 1), "the first delivery", CLAUSE)?;
        let log = read_all(&store, CLAUSE).await?;
        let twins = log.iter().filter(|row| row.event == batch[0].event).count();
        ensure(twins == 2, CLAUSE, || {
            format!(
                "two origins each sent one byte-identical event and the store holds {twins}; \
                 only their identities differ, and both are new"
            )
        })?;
        held_once(&log, &answer[0], CLAUSE)?;
        let head_before = head(&store, CLAUSE).await?;

        let again = ingest(&store, &group, CLAUSE).await?;
        ensure_counts(again, (0, 3, 0), "the same batch again", CLAUSE)?;
        ensure(read_all(&store, CLAUSE).await? == log, CLAUSE, || {
            String::from("a re-delivery of a held batch changed the log")
        })?;
        let head_after = head(&store, CLAUSE).await?;
        ensure(head_after == head_before, CLAUSE, || {
            format!("a re-delivery moved the head from {head_before:?} to {head_after:?}")
        })?;

        let third = undecodable(DEDUPE_ORIGIN, 3, b'3', CLAUSE)?;
        let partial = [second, third];
        let landed = ingest(&store, &[IngestGroup::new(&partial, &[])], CLAUSE).await?;
        ensure_counts(landed, (1, 1, 0), "one held event and one new one", CLAUSE)?;
        let grown = read_all(&store, CLAUSE).await?;
        held_once(&grown, &partial[1].event, CLAUSE)?;
        // Identity, time and bytes, and not position: where the new event
        // lands, and whether anything moved for it, is SY-5's to judge.
        let rest: Vec<&SequencedEvent> = grown
            .iter()
            .filter(|row| row.event != partial[1].event)
            .collect();
        let kept = |before: &SequencedEvent| {
            rest.iter().any(|after| {
                after.id == before.id
                    && after.recorded_at == before.recorded_at
                    && after.event == before.event
            })
        };
        ensure(
            rest.len() == log.len() && log.iter().all(kept),
            CLAUSE,
            || {
                format!(
                    "a batch of one held event and one new one changed the log by more than the \
                 new event: {} other events were held before and {} are now",
                    log.len(),
                    rest.len()
                )
            },
        )?;
        Ok(RuleOutcome::Ran)
    }

    /// Every origin a rule uses is foreign to the store under test, or the
    /// rule would be testing SY-38 by accident.
    fn ensure_foreign<S: IngestStore>(
        store: &S,
        origins: &[StoreId],
        clause: &'static str,
    ) -> Result<(), Violation> {
        let own = store.store_id();
        ensure(!origins.contains(&own), clause, || {
            format!(
                "the fixture's store has the incarnation {own}, which this rule uses as a \
                 foreign origin; a fixture must give each store a fresh one"
            )
        })
    }

    /// A foreign event from `origin` at origin `position`, recorded at
    /// `millis`, carrying `data` as opaque bytes.
    fn shipped(
        origin: StoreId,
        position: u64,
        millis: i64,
        data: &[u8],
        clause: &'static str,
    ) -> Result<ReplicatedEvent, Violation> {
        let position = SequencePosition::new(position)
            .ok_or_else(|| Violation::new(clause, "the rule's origin position is zero"))?;
        Ok(ReplicatedEvent::new(
            EventId::new(origin, position),
            RecordedAt::from_millis(millis),
            opaque(SHIPPED, data, clause)?,
        ))
    }

    /// An event of `event_type` whose data is `data`, never parsed (SY-35).
    fn opaque(
        event_type: EventType,
        data: &[u8],
        clause: &'static str,
    ) -> Result<Event, Violation> {
        Event::new(event_type, data.to_vec()).map_err(|error| {
            Violation::new(clause, format!("the rule's own event is invalid: {error}"))
        })
    }

    /// Ingests `groups`, which the store must accept.
    async fn ingest<S: IngestStore>(
        store: &S,
        groups: &[IngestGroup<'_>],
        clause: &'static str,
    ) -> Result<Ingested, Violation> {
        store.ingest(groups).await.map_err(|error| {
            Violation::new(
                clause,
                format!("the store refused an ingest it must accept: {error}"),
            )
        })
    }

    /// [`UNDECODABLE`] followed by `suffix`, so each event's bytes are its own.
    fn undecodable_bytes(suffix: u8) -> Vec<u8> {
        let mut bytes = UNDECODABLE.to_vec();
        bytes.push(suffix);
        bytes
    }

    /// A foreign event from `origin` whose data and metadata are both
    /// undecodable, tagged so a content comparison has a tag to compare too.
    fn undecodable(
        origin: StoreId,
        position: u64,
        suffix: u8,
        clause: &'static str,
    ) -> Result<ReplicatedEvent, Violation> {
        let mut event = shipped(
            origin,
            position,
            1_800_000_000_000,
            &undecodable_bytes(suffix),
            clause,
        )?;
        event.event = event
            .event
            .with_metadata(undecodable_bytes(suffix.wrapping_add(0x80)))
            .with_tags([Tag::from_static("item:undecodable")].into_iter().collect());
        Ok(event)
    }

    /// `ingested` is `(appended, skipped, compensated)`, or a violation naming
    /// `what` was ingested.
    fn ensure_counts(
        ingested: Ingested,
        (appended, skipped, compensated): (usize, usize, usize),
        what: &str,
        clause: &'static str,
    ) -> Result<(), Violation> {
        let got = (ingested.appended, ingested.skipped, ingested.compensated);
        ensure(got == (appended, skipped, compensated), clause, || {
            format!(
                "{what} was counted as (appended, skipped, compensated) {got:?}; it should be \
                 {:?}",
                (appended, skipped, compensated)
            )
        })
    }

    /// The store's head.
    async fn head<S: EventStore>(
        store: &S,
        clause: &'static str,
    ) -> Result<Option<SequencePosition>, Violation> {
        store
            .head()
            .await
            .map_err(|error| Violation::new(clause, format!("the store refused its head: {error}")))
    }

    /// Appends `events` locally, without a condition, and returns the last
    /// position the store assigned.
    async fn append<S: EventStore>(
        store: &S,
        events: &[Event],
        clause: &'static str,
    ) -> Result<SequencePosition, Violation> {
        store.append(events, None).await.map_err(|error| {
            Violation::new(clause, format!("the store refused a local append: {error}"))
        })
    }

    /// The one event in `log` whose bytes are `written`'s, found without its
    /// identity.
    fn held_once<'log>(
        log: &'log [SequencedEvent],
        written: &Event,
        clause: &'static str,
    ) -> Result<&'log SequencedEvent, Violation> {
        let mut matching = log.iter().filter(|row| row.event == *written);
        match (matching.next(), matching.next()) {
            (Some(landed), None) => Ok(landed),
            (found, _) => Err(Violation::new(
                clause,
                format!(
                    "an event of type {} was written once and is held {}",
                    written.event_type(),
                    if found.is_some() {
                        "more than once"
                    } else {
                        "nowhere"
                    }
                ),
            )),
        }
    }

    /// Every event the store holds, in position order.
    async fn read_all<S: EventStore>(
        store: &S,
        clause: &'static str,
    ) -> Result<Vec<SequencedEvent>, Violation> {
        collect(store.read(&Query::all(), ReadOptions::new()))
            .await
            .map_err(|error| {
                Violation::new(
                    clause,
                    format!("the store refused a read of its log: {error}"),
                )
            })
    }

    /// The store's watermark.
    async fn watermark<S: IngestStore>(
        store: &S,
        clause: &'static str,
    ) -> Result<Watermark, Violation> {
        store.watermark().await.map_err(|error| {
            Violation::new(clause, format!("the store refused its watermark: {error}"))
        })
    }
}

/// Hands the complete ingest rule set to `$callback`.
///
/// This family's single enumeration. `$callback` is the path of a macro that
/// accepts a comma-separated list of identifiers, and it is invoked once, with
/// every rule in [`ingest_rules`].
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_ingest_store_rule!(rule_names);
/// assert!(names.contains(&"ingest_preserves_origin_identity"));
/// ```
#[macro_export]
macro_rules! for_each_ingest_store_rule {
    // Raw token trees rather than `$cb:path`, for the reason the event-store
    // family's enumeration records: a parsed `path` cannot sit in callee
    // position inside an expression.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- An ingested event keeps its origin's identity (VT-5) -------
            ingest_preserves_origin_identity,

            // --- An ingested event lands at the tail (SY-5) -----------------
            ingested_events_land_above_the_local_head,

            // --- Dedupe is by identity, never content (SY-12, SY-11) --------
            dedupe_reaches_identity_without_decoding,
        }
    };
}

/// Emits one `#[tokio::test]` per ingest rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`. The default emitter of
/// [`ingest_store_conformance!`](crate::ingest_store_conformance).
#[macro_export]
macro_rules! emit_ingest_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::ingest_rules::$name(__ingest_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one plain `#[test]` per ingest rule, driven by
/// [`happenstance_testkit::block_on`].
///
/// No runtime and no `Send` bound, so this is the harness a `!Send` store runs
/// under.
#[macro_export]
macro_rules! emit_ingest_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::block_on(
                    $crate::__private::ingest_rules::$name(__ingest_fixture),
                )?
                .report(::core::stringify!($name));
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
macro_rules! __ingest_rule_names {
    ($($name:ident),* $(,)?) => { [ $( ::core::stringify!($name) ),* ] };
}

/// Every `pub async fn` declared at the top level of this file's
/// `ingest_rules` module: the same four-space prefix `cargo xtask spec-trace`
/// reads.
#[cfg(test)]
fn declared_rules() -> Vec<&'static str> {
    const SOURCE: &str = include_str!("ingest_suite.rs");
    const PREFIX: &str = "    pub async fn ";

    SOURCE
        .lines()
        .filter_map(|line| line.strip_prefix(PREFIX))
        .filter_map(|rest| {
            rest.split(|c: char| !c.is_alphanumeric() && c != '_')
                .next()
        })
        .collect()
}

/// No rule exists without appearing in the enumeration, and none is
/// enumerated without existing: `no_orphan_sync_rules`, one family over.
#[cfg(test)]
#[test]
fn no_orphan_ingest_rules() {
    let registered = crate::for_each_ingest_store_rule!(crate::__ingest_rule_names);
    let declared = declared_rules();

    assert!(
        !declared.is_empty(),
        "the source scan matched no rule at all"
    );
    let orphans: Vec<_> = declared
        .iter()
        .filter(|name| !registered.contains(*name))
        .collect();
    assert!(
        orphans.is_empty(),
        "these rules are absent from `for_each_ingest_store_rule!`, so no harness runs them: \
         {orphans:?}"
    );
    let missing: Vec<_> = registered
        .iter()
        .filter(|name| !declared.contains(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "these rules are registered but not declared in `ingest_suite.rs`: {missing:?}"
    );
}
