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
    use alloc::vec::Vec;

    use happenstance_core::{
        Event, EventId, EventStore, EventType, Query, ReadOptions, RecordedAt, SequencePosition,
        SequencedEvent, StoreId, collect,
    };
    use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent, Watermark};
    use happenstance_testkit::RuleOutcome;

    use crate::IngestFixture;
    use crate::violation::{Violation, ensure};

    const SHIPPED: EventType = EventType::from_static("Shipped");

    /// VT-5's origin.
    const IDENTITY_ORIGIN: StoreId = StoreId::from_bytes([0x71; 16]);

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
