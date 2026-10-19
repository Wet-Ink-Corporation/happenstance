//! The ingest rules, their one enumeration, and the two emitters.
//!
//! Private: what a caller reaches is [`ingest_rules`], re-exported at the
//! crate root, and the macros, which `#[macro_export]` puts there regardless.

/// The `capability` SY-38's rule reports when the store did not put the
/// rule's compensation where its arrangement needed it.
///
/// Not a fixture constant, which is why it is a description rather than an
/// identifier: SY-37's [`NO_SECOND_PAGE`](crate::NO_SECOND_PAGE) is the
/// precedent. Gaps are legal, so a store may place a compensation anywhere
/// above the head.
pub const NO_HIDDEN_CLAIM: &str = "the position an ingest's compensation takes";

/// The reason SY-38's rule reports when it could not arrange a claim hidden
/// by the call's own compensation.
///
/// Written by this crate rather than by the fixture, because it is the same
/// sentence for every store that earns it.
pub const NO_HIDDEN_CLAIM_REASON: &str = "this store did not put the rule's compensation on the identity the rule \
     claimed, two positions past the head it assigned, so a claim the call's own compensation would \
     hide was never arranged; SY-38's unheld-claim half held, and its compensation-hidden half is \
     unverified for this store";

/// The `capability` SY-18's ingest rule reports when the fixture declares no
/// per-event ceiling it can exercise.
///
/// Names all three [`IngestFixture`](crate::IngestFixture) constants, because
/// the rule needs every one of them absent, or too large to allocate, before
/// it has nothing to do: `happenstance_testkit::NO_STORE_LIMITS` is the
/// precedent.
pub const NO_PER_EVENT_LIMITS: &str = "MAX_EVENT_DATA_LEN, MAX_TAGS_PER_EVENT, MAX_METADATA_LEN";

/// The reason SY-18's ingest rule reports when it exercised no ceiling.
///
/// Written by this crate rather than by the fixture, because it is the same
/// sentence for every store that earns it.
pub const NO_PER_EVENT_LIMITS_REASON: &str = "this fixture declares no per-event ceiling the rule can exercise, none \
     at all or only ones at or above the sizes this crate will allocate, so a \
     store that checks only the head of each group could not be told from one \
     that checks every event; the rule is unverified for this store";

/// The most tags SY-18's ingest rule puts on one event, whatever ceiling the
/// fixture declares (H-13).
///
/// A declared `MAX_TAGS_PER_EVENT` at or above this is not exercised: the
/// over-ceiling event carries one tag more than the ceiling, and the rule does
/// not allocate a number it was handed. Byte ceilings are capped by
/// [`MAX_PROVOKED_BATCH_BYTES`](crate::MAX_PROVOKED_BATCH_BYTES) for the same
/// reason.
pub const MAX_PROVOKED_TAGS: usize = 4_096;

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
///
/// **Assume an identity the store did not state.** Each rule awaits the
/// store's [`store_id`](happenstance_sync::IngestStore::store_id) before it
/// arranges anything, and a failed read is a violation of the rule's own
/// clause rather than a reason to carry on.
pub mod ingest_rules {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec::Vec;

    use alloc::vec;

    use happenstance_core::{
        Event, EventId, EventStore, EventType, Query, ReadOptions, RecordedAt, SequencePosition,
        SequencedEvent, StoreId, StoreLimit, Tag, Tags, collect,
    };
    use happenstance_sync::{
        Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
        Watermark,
    };
    use happenstance_testkit::RuleOutcome;

    use super::{
        MAX_PROVOKED_TAGS, NO_HIDDEN_CLAIM, NO_HIDDEN_CLAIM_REASON, NO_PER_EVENT_LIMITS,
        NO_PER_EVENT_LIMITS_REASON,
    };

    use crate::violation::{Violation, ensure};
    use crate::{IngestFixture, MAX_PROVOKED_BATCH_BYTES};

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

    /// Bytes that are not UTF-8 and parse as no codec: two bytes UTF-8 never
    /// uses, a NUL and a two-byte sequence cut off by a non-continuation byte. A store that had to
    /// decode a payload to dedupe it fails on these (SY-35).
    const UNDECODABLE: &[u8; 5] = b"\xFF\xFE\x00\xC3\x28";

    /// SY-38's origin, for the foreign events around each claim.
    const CLAIM_ORIGIN: StoreId = StoreId::from_bytes([0x75; 16]);

    /// SY-30's origin, for the group a fault is armed inside.
    const ATOMIC_ORIGIN: StoreId = StoreId::from_bytes([0x77; 16]);

    /// SY-18's origin, for the events held to the declared ceilings.
    const CEILING_ORIGIN: StoreId = StoreId::from_bytes([0x78; 16]);

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
        ensure_foreign(&store, &[IDENTITY_ORIGIN], CLAUSE).await?;
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
            ensure(holds(&store, event.id, CLAUSE).await?, CLAUSE, || {
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
        ensure_foreign(&store, &[TAIL_ORIGIN], CLAUSE).await?;
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
    /// identical fact as though it were the first again; one that decodes a
    /// payload to reach its dedupe key, and skips bytes it cannot parse; and
    /// one that writes a group's compensation again when the group was a
    /// re-delivery. `tests/wrong_ingest_stores.rs` carries all three.
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
        ensure_foreign(&store, &[DEDUPE_ORIGIN, TWIN_ORIGIN], CLAUSE).await?;
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

    /// SY-38: an event claiming the store's own identity at a position the
    /// store did not hold when the call began refuses the **whole** call as
    /// [`IngestError::UnheldOwnIdentity`], class [`FailureClass::Stop`], and
    /// writes nothing. A held own identity is a re-delivery, and a skip.
    ///
    /// Three arrangements, each on a fresh fixture, and every claimed position
    /// computed from the head the store assigned:
    ///
    /// 1. **The unheld claim.** After one local event, two groups arrive, and
    ///    the claim, three positions past the head, is the second event of the
    ///    second. The call is refused naming the claim; the log, head and
    ///    watermark are as they were; and the store holds none of the three.
    /// 2. **The held echo.** The store's own local event, sent back to it, is
    ///    skipped, and nothing changes.
    /// 3. **The claim the call's own compensation would hide.** The first group
    ///    carries a compensation, which a dense store mints two positions past
    ///    the head; the second group claims exactly that identity. The call is
    ///    refused naming it, and writes nothing. The rule then ingests the
    ///    first group alone to see where its compensation lands. A store that
    ///    leaves a gap there was never shown the hidden claim, which the
    ///    specification permits, so the rule reports
    ///    [`Skipped`](RuleOutcome::Skipped) with [`NO_HIDDEN_CLAIM`] rather
    ///    than a pass.
    ///
    /// This refusal is the one SY-33's rule, when it is written, must exempt
    /// (ADR-0098): it is about the store's own identity, never about an
    /// event's content.
    ///
    /// # Rejects
    ///
    /// A store that drops the claim as a re-delivery and writes the rest; one
    /// that checks only the batch's first event; one that commits group by
    /// group and keeps the groups before the claim; one that refuses a claim
    /// only when it lands, so a claim meeting the call's own compensation is
    /// skipped and two events share one identity; and one that refuses a held
    /// echo too. `tests/wrong_ingest_stores.rs` carries all five, and the
    /// conformant store that leaves a gap, which this rule must skip.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-38 if any arrangement's ingest is accepted, or
    /// refused other than as the unheld claim it carries, or writes anything,
    /// or if the echo is not a skip.
    pub async fn an_unheld_own_identity_stops_the_ingest<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        an_unheld_claim_is_refused(&open().await).await?;
        a_held_echo_is_a_skip(&open().await).await?;
        a_claim_the_compensation_would_hide(&open().await).await
    }

    /// SY-38, the clause every arrangement below reports under.
    const SY_38: &str = "SY-38";

    /// SY-38's first arrangement: the unheld claim.
    async fn an_unheld_claim_is_refused<F: IngestFixture>(fixture: &F) -> Result<(), Violation> {
        let store = fixture.connect().await;
        let own = ensure_foreign(&store, &[CLAIM_ORIGIN], SY_38).await?;
        let head = append(&store, &[opaque(LOCAL, b"sy38-a-local", SY_38)?], SY_38).await?;
        let claim = EventId::new(own, positions_past(head, 3)?);
        let first = [shipped(
            CLAIM_ORIGIN,
            1,
            1_800_000_000_000,
            b"sy38-a-1",
            SY_38,
        )?];
        let second = [
            shipped(CLAIM_ORIGIN, 2, 1_800_000_000_001, b"sy38-a-2", SY_38)?,
            forged(claim, b"sy38-a-claim")?,
        ];
        let before = observe(&store, SY_38).await?;

        let refused = store
            .ingest(&[
                IngestGroup::new(&first, &[]),
                IngestGroup::new(&second, &[]),
            ])
            .await;

        ensure_refused(refused, claim, "a claim three positions past the head")?;
        ensure_unchanged(
            &store,
            &before,
            "an ingest refused for an unheld claim",
            SY_38,
        )
        .await?;
        for id in [first[0].id, second[0].id, claim] {
            ensure(!holds(&store, id, SY_38).await?, SY_38, || {
                format!("the refused ingest left {id} held")
            })?;
        }
        Ok(())
    }

    /// SY-38's second arrangement: the held echo.
    async fn a_held_echo_is_a_skip<F: IngestFixture>(fixture: &F) -> Result<(), Violation> {
        let store = fixture.connect().await;
        append(&store, &[opaque(LOCAL, b"sy38-c-local", SY_38)?], SY_38).await?;
        let before = observe(&store, SY_38).await?;
        let held = before
            .log
            .last()
            .ok_or_else(|| Violation::new(SY_38, "the store holds nothing after an append"))?;
        let own = own_identity(&store, SY_38).await?;
        ensure(held.id.store() == own, SY_38, || {
            format!(
                "a local append was minted as {}, not under the store's own identity {own}",
                held.id
            )
        })?;
        let echo = [ReplicatedEvent::new(
            held.id,
            held.recorded_at,
            held.event.clone(),
        )];

        let echoed = store.ingest(&[IngestGroup::new(&echo, &[])]).await;

        let echoed = echoed.map_err(|error| {
            Violation::new(
                SY_38,
                format!(
                    "the store refused an echo of {}, which it holds: {error}; a held own \
                     identity is a re-delivery, not a claim",
                    held.id
                ),
            )
        })?;
        ensure_counts(echoed, (0, 1, 0), "an echo of a held local event", SY_38)?;
        ensure_unchanged(&store, &before, "an echo of a held event", SY_38).await
    }

    /// SY-38's third arrangement: the claim the call's own compensation would
    /// hide, and the check that the store let the rule arrange it.
    async fn a_claim_the_compensation_would_hide<F: IngestFixture>(
        fixture: &F,
    ) -> Result<RuleOutcome, Violation> {
        let store = fixture.connect().await;
        let own = ensure_foreign(&store, &[CLAIM_ORIGIN], SY_38).await?;
        let head = append(&store, &[opaque(LOCAL, b"sy38-b-local", SY_38)?], SY_38).await?;
        let claimed = EventId::new(own, positions_past(head, 2)?);
        let first = [shipped(
            CLAIM_ORIGIN,
            11,
            1_800_000_000_011,
            b"sy38-b-1",
            SY_38,
        )?];
        let answer = [opaque(COMPENSATED, b"sy38-b-compensation", SY_38)?];
        let second = [
            shipped(CLAIM_ORIGIN, 12, 1_800_000_000_012, b"sy38-b-2", SY_38)?,
            forged(claimed, b"sy38-b-claim")?,
        ];
        let before = observe(&store, SY_38).await?;

        let refused = store
            .ingest(&[
                IngestGroup::new(&first, &answer),
                IngestGroup::new(&second, &[]),
            ])
            .await;

        ensure_refused(
            refused,
            claimed,
            "a claim on the position the call's compensation takes",
        )?;
        ensure_unchanged(
            &store,
            &before,
            "an ingest refused for a hidden claim",
            SY_38,
        )
        .await?;

        ingest(&store, &[IngestGroup::new(&first, &answer)], SY_38).await?;
        let log = read_all(&store, SY_38).await?;
        if held_once(&log, &answer[0], SY_38)?.id == claimed {
            Ok(RuleOutcome::Ran)
        } else {
            Ok(RuleOutcome::Skipped {
                capability: NO_HIDDEN_CLAIM,
                reason: NO_HIDDEN_CLAIM_REASON,
            })
        }
    }

    /// The position `count` past `head`.
    fn positions_past(head: SequencePosition, count: usize) -> Result<SequencePosition, Violation> {
        (0..count).try_fold(head, |position, _| {
            position
                .next()
                .ok_or_else(|| Violation::new(SY_38, "the store's head has no position past it"))
        })
    }

    /// An event claiming `id`, which the rule chose, carrying `data`.
    fn forged(id: EventId, data: &[u8]) -> Result<ReplicatedEvent, Violation> {
        Ok(ReplicatedEvent::new(
            id,
            RecordedAt::from_millis(1_800_000_000_000),
            opaque(LOCAL, data, SY_38)?,
        ))
    }

    /// `outcome` is the refusal naming `claim`, and states the `Stop` class.
    fn ensure_refused<E: core::error::Error + Classify>(
        outcome: Result<Ingested, IngestError<E>>,
        claim: EventId,
        arrangement: &str,
    ) -> Result<(), Violation> {
        let error = match outcome {
            Ok(ingested) => {
                return Err(Violation::new(
                    SY_38,
                    format!(
                        "the store accepted {arrangement} as {ingested:?}; {claim} claims its own \
                         identity at a position it did not hold, so the call must write nothing"
                    ),
                ));
            }
            Err(error) => error,
        };
        let named = match &error {
            IngestError::UnheldOwnIdentity { id, .. } => Some(*id),
            _ => None,
        };
        ensure(named == Some(claim), SY_38, || {
            format!("{arrangement} was refused as `{error}`, not as the unheld claim {claim}")
        })?;
        ensure(error.class() == FailureClass::Stop, SY_38, || {
            format!(
                "the refusal of {claim} states the class {:?}; an unheld own identity is Stop",
                error.class()
            )
        })
    }

    /// SY-30: an [`IngestGroup`] lands whole or not at all, even when the
    /// store fails between two of its rows.
    ///
    /// SY-30 rejects a receiver that, after a crash mid-ingest, holds four
    /// events of a seven-event batch. A conformant store decides every refusal
    /// before it writes, so the case no other rule reaches is a fault inside
    /// the write, and only the adapter can inject one:
    /// [`IngestFixture::MID_GROUP_FAULT`]. A fixture that declines it is
    /// reported [`Skipped`](RuleOutcome::Skipped) with its stated reason.
    ///
    /// Three arrangements, each on a fresh fixture, one per cut inside a
    /// four-row group of three foreign events and one compensation event:
    /// after the first, the second and the third row the store writes. Each
    /// appends a local event, so "unchanged" is not "empty", and ingests an
    /// unarmed control group of the same shape, which must land as three
    /// appended and one compensation. Then it arms the fault and ingests the
    /// armed group. The ingest must return `Err`. The log, head and watermark
    /// must be as they were, and the store must hold none of the group's
    /// events.
    ///
    /// On the ingest path a losing event and its compensation are one group,
    /// so a cut between them is the state SY-2 forbids a reader to see. The
    /// claim is SY-30's because SY-2 is frozen.
    ///
    /// # Rejects
    ///
    /// A store that decides the whole call and then writes it in separately
    /// committed chunks; one that commits a group's foreign rows and its
    /// compensation as two writes; one that rolls the group back and answers
    /// `Ok`, so a runner advances past a group that never landed; and a
    /// fixture whose arm does nothing. `tests/wrong_ingest_stores.rs` carries
    /// all four. A store that absorbs every fault its fixture can arm, and
    /// commits the group anyway, must decline the capability: this rule
    /// requires the armed ingest to fail.
    ///
    /// # Does not reject
    ///
    /// A store that commits group by group, each group whole. That is SY-38's
    /// mistake and its rule's to catch (every arm of `IngestError` means
    /// nothing in the call was written), and this rule faults one group only.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-30 if the fixture cannot arm the fault, if the
    /// control group is refused or miscounted, if the armed ingest returns
    /// `Ok`, or if it leaves anything of the group behind.
    pub async fn ingest_group_is_atomic_under_a_mid_group_fault<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        if let Some(reason) = F::MID_GROUP_FAULT.reason() {
            return Ok(RuleOutcome::Skipped {
                capability: "MID_GROUP_FAULT",
                reason,
            });
        }
        // Every cut inside a four-row group, so the rule does not depend on
        // the store writing the compensation before or after the foreign rows.
        for after in 1..=3 {
            a_fault_inside_the_group(&open().await, after).await?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// SY-30, the clause rule 1's arrangement reports under.
    const SY_30: &str = "SY-30";

    /// One arrangement of SY-30's rule: a group of three foreign rows and one
    /// compensation row, faulted while its `after + 1`-th row is written.
    async fn a_fault_inside_the_group<F: IngestFixture>(
        fixture: &F,
        after: usize,
    ) -> Result<(), Violation> {
        let store = fixture.connect().await;
        ensure_foreign(&store, &[ATOMIC_ORIGIN], SY_30).await?;
        append(&store, &[opaque(LOCAL, b"sy30-local", SY_30)?], SY_30).await?;
        // The control: the same shape, unarmed, lands whole.
        let control = [
            shipped(
                ATOMIC_ORIGIN,
                1,
                1_800_000_000_001,
                b"sy30-control-1",
                SY_30,
            )?,
            shipped(
                ATOMIC_ORIGIN,
                2,
                1_800_000_000_002,
                b"sy30-control-2",
                SY_30,
            )?,
            shipped(
                ATOMIC_ORIGIN,
                3,
                1_800_000_000_003,
                b"sy30-control-3",
                SY_30,
            )?,
        ];
        let control_answer = [opaque(COMPENSATED, b"sy30-control-compensation", SY_30)?];
        let landed = ingest(
            &store,
            &[IngestGroup::new(&control, &control_answer)],
            SY_30,
        )
        .await?;
        ensure_counts(landed, (3, 0, 1), "the unarmed control group", SY_30)?;

        let armed = [
            shipped(ATOMIC_ORIGIN, 4, 1_800_000_000_004, b"sy30-armed-1", SY_30)?,
            shipped(ATOMIC_ORIGIN, 5, 1_800_000_000_005, b"sy30-armed-2", SY_30)?,
            shipped(ATOMIC_ORIGIN, 6, 1_800_000_000_006, b"sy30-armed-3", SY_30)?,
        ];
        let answer = [opaque(COMPENSATED, b"sy30-armed-compensation", SY_30)?];
        let before = observe(&store, SY_30).await?;
        fixture.arm_mid_group_fault(after).await?;

        let outcome = store.ingest(&[IngestGroup::new(&armed, &answer)]).await;

        let row = after.saturating_add(1);
        ensure(outcome.is_err(), SY_30, || {
            format!(
                "the fixture armed a fault at row {row} of a four-row group and the ingest \
                 returned Ok; a fault that never fires shows nothing about what a failed group \
                 leaves, so a store that absorbs every fault must decline MID_GROUP_FAULT"
            )
        })?;
        ensure_unchanged(
            &store,
            &before,
            &format!("an ingest faulted at row {row} of one group"),
            SY_30,
        )
        .await?;
        for event in &armed {
            ensure(!holds(&store, event.id, SY_30).await?, SY_30, || {
                format!(
                    "an ingest faulted at row {row} of one group left {} held; a group lands \
                     whole or not at all",
                    event.id
                )
            })?;
        }
        Ok(())
    }

    /// SY-18: a store holds **every** event an ingest writes to the
    /// per-event ceilings it declares, not the head of each group, and an
    /// event over one refuses the whole call.
    ///
    /// SY-18 says an event that cannot land is discovered at ingest, so the
    /// refusal has to be a true one: [`IngestError::ExceedsStoreLimit`]
    /// naming the limit and the magnitude, class [`FailureClass::Park`], with
    /// nothing written. The rule reads the ceilings from the fixture's
    /// [`MAX_EVENT_DATA_LEN`](IngestFixture::MAX_EVENT_DATA_LEN),
    /// [`MAX_TAGS_PER_EVENT`](IngestFixture::MAX_TAGS_PER_EVENT) and
    /// [`MAX_METADATA_LEN`](IngestFixture::MAX_METADATA_LEN), and for each
    /// declared one, on a fresh fixture, after one local event:
    ///
    /// 1. **The control.** Two groups, the second carrying an event exactly
    ///    at the ceiling behind its head and a compensation whose second event
    ///    is exactly at it too, land as three appended and two compensations,
    ///    each sized event found by its bytes.
    /// 2. **Over, in the foreign events.** Two groups, the second's second
    ///    event one over the ceiling: refused, the log, head and watermark as
    ///    they were, and none of the three events held.
    /// 3. **Over, in the compensation.** One group whose compensation's second
    ///    event is one over: refused in the same way.
    ///
    /// A ceiling at or above [`MAX_PROVOKED_TAGS`]
    /// tags or [`MAX_PROVOKED_BATCH_BYTES`]
    /// bytes is not exercised, because the over-ceiling event is allocated
    /// whole (H-13). A fixture that leaves the rule nothing to exercise is
    /// reported [`Skipped`](RuleOutcome::Skipped) with
    /// [`NO_PER_EVENT_LIMITS`].
    ///
    /// # Rejects
    ///
    /// A store that holds only each group's first event to its ceilings; one
    /// that never checks the compensation it writes itself; and one that
    /// refuses an event exactly at a ceiling, which only the control sees.
    /// `tests/wrong_ingest_stores.rs` carries all three.
    ///
    /// # Does not reject
    ///
    /// A head-only check of the store's own identity. That is SY-38's rule's,
    /// whose unheld claim already sits behind a group's head, so a mutant
    /// that checked only each group's head for it would fail that rule too.
    /// Nor the batch limit, which is `Split`, not a fact about one event.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-18 if the control is refused or miscounted, or if
    /// an over-ceiling call is accepted, refused as anything but that limit
    /// and magnitude or with a class other than `Park`, or leaves anything
    /// behind. The wrong-arm message cites VT-25.
    pub async fn ingest_validates_every_event_not_the_head<F: IngestFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        let declared = [
            (
                StoreLimit::EventDataLen,
                F::MAX_EVENT_DATA_LEN,
                MAX_PROVOKED_BATCH_BYTES,
            ),
            (
                StoreLimit::TagsPerEvent,
                F::MAX_TAGS_PER_EVENT,
                MAX_PROVOKED_TAGS,
            ),
            (
                StoreLimit::MetadataLen,
                F::MAX_METADATA_LEN,
                MAX_PROVOKED_BATCH_BYTES,
            ),
        ];
        let mut exercised = false;
        for (limit, ceiling, cap) in declared {
            // Below the cap, so one over the ceiling is still inside it (H-13).
            let Some(ceiling) = ceiling.filter(|ceiling| *ceiling < cap) else {
                continue;
            };
            every_event_is_held_to(&open().await, limit, ceiling).await?;
            exercised = true;
        }
        Ok(if exercised {
            RuleOutcome::Ran
        } else {
            RuleOutcome::Skipped {
                capability: NO_PER_EVENT_LIMITS,
                reason: NO_PER_EVENT_LIMITS_REASON,
            }
        })
    }

    /// SY-18, the clause rule 2's arrangements report under.
    const SY_18: &str = "SY-18";

    /// SY-18's arrangements for one declared `ceiling` on `limit`.
    async fn every_event_is_held_to<F: IngestFixture>(
        fixture: &F,
        limit: StoreLimit,
        ceiling: usize,
    ) -> Result<(), Violation> {
        let over = ceiling
            .checked_add(1)
            .ok_or_else(|| Violation::new(SY_18, "the declared ceiling has no size past it"))?;
        let store = fixture.connect().await;
        ensure_foreign(&store, &[CEILING_ORIGIN], SY_18).await?;
        append(&store, &[opaque(LOCAL, b"sy18-local", SY_18)?], SY_18).await?;

        // The control: the same shapes, each sized event exactly at the
        // ceiling, land. Found by their bytes, never their identities.
        let head_group = [shipped(
            CEILING_ORIGIN,
            1,
            1_800_000_000_001,
            b"sy18-control-1",
            SY_18,
        )?];
        let at_ceiling = [
            shipped(
                CEILING_ORIGIN,
                2,
                1_800_000_000_002,
                b"sy18-control-2",
                SY_18,
            )?,
            sized_shipped(CEILING_ORIGIN, 3, AT_FOREIGN, limit, ceiling)?,
        ];
        let at_answer = [
            opaque(COMPENSATED, b"sy18-control-compensation", SY_18)?,
            sized(AT_COMPENSATION, limit, ceiling)?,
        ];
        let landed = ingest(
            &store,
            &[
                IngestGroup::new(&head_group, &[]),
                IngestGroup::new(&at_ceiling, &at_answer),
            ],
            SY_18,
        )
        .await?;
        ensure_counts(
            landed,
            (3, 0, 2),
            "two groups whose sized events are exactly at the ceiling",
            SY_18,
        )?;
        let log = read_all(&store, SY_18).await?;
        held_once(&log, &at_ceiling[1].event, SY_18)?;
        held_once(&log, &at_answer[1], SY_18)?;
        let before = observe(&store, SY_18).await?;

        // A: an event over the ceiling behind the second group's head, so a
        // store that holds only each group's first event to it writes it.
        let first = [shipped(
            CEILING_ORIGIN,
            4,
            1_800_000_000_004,
            b"sy18-a-1",
            SY_18,
        )?];
        let second = [
            shipped(CEILING_ORIGIN, 5, 1_800_000_000_005, b"sy18-a-2", SY_18)?,
            sized_shipped(CEILING_ORIGIN, 6, OVER_FOREIGN, limit, over)?,
        ];
        let refused = store
            .ingest(&[
                IngestGroup::new(&first, &[]),
                IngestGroup::new(&second, &[]),
            ])
            .await;
        let what = "a group whose foreign event is one over the ceiling";
        ensure_exceeds(refused, limit, over, what)?;
        ensure_unchanged(&store, &before, what, SY_18).await?;
        for id in [first[0].id, second[0].id, second[1].id] {
            ensure(!holds(&store, id, SY_18).await?, SY_18, || {
                format!("{what} was refused and left {id} held")
            })?;
        }

        // B: an event over the ceiling behind the head of a group's
        // compensation, which the store writes itself.
        let lone = [shipped(
            CEILING_ORIGIN,
            7,
            1_800_000_000_007,
            b"sy18-b-1",
            SY_18,
        )?];
        let answer = [
            opaque(COMPENSATED, b"sy18-b-compensation", SY_18)?,
            sized(OVER_COMPENSATION, limit, over)?,
        ];
        let refused = store.ingest(&[IngestGroup::new(&lone, &answer)]).await;
        let what = "a group whose compensation is one over the ceiling";
        ensure_exceeds(refused, limit, over, what)?;
        ensure_unchanged(&store, &before, what, SY_18).await?;
        ensure(!holds(&store, lone[0].id, SY_18).await?, SY_18, || {
            format!("{what} was refused and left {} held", lone[0].id)
        })
    }

    /// What a sized event's bytes are filled with. Never parsed (SY-35), and
    /// ASCII so a store that decodes a payload fails SY-12's rule and not this
    /// one.
    const FILLER: u8 = b'.';

    // The event types that tell the rule's sized events apart, since their
    // sized field is filler.
    const AT_FOREIGN: EventType = EventType::from_static("CeilingAtForeign");
    const AT_COMPENSATION: EventType = EventType::from_static("CeilingAtCompensation");
    const OVER_FOREIGN: EventType = EventType::from_static("CeilingOverForeign");
    const OVER_COMPENSATION: EventType = EventType::from_static("CeilingOverCompensation");

    /// An event of `event_type` whose `limit` field measures `len` in that
    /// limit's unit: `len` opaque bytes of data or metadata, or `len` distinct
    /// tags. Nothing else is set, so no other ceiling is approached.
    fn sized(event_type: EventType, limit: StoreLimit, len: usize) -> Result<Event, Violation> {
        let invalid = |error: &dyn core::fmt::Display| {
            Violation::new(SY_18, format!("the rule's own event is invalid: {error}"))
        };
        let data = if limit == StoreLimit::EventDataLen {
            vec![FILLER; len]
        } else {
            Vec::new()
        };
        let event = Event::new(event_type, data).map_err(|error| invalid(&error))?;
        match limit {
            StoreLimit::EventDataLen => Ok(event),
            StoreLimit::MetadataLen => Ok(event.with_metadata(vec![FILLER; len])),
            StoreLimit::TagsPerEvent => {
                let tags = (0..len)
                    .map(|i| Tag::new(format!("t:{i}")))
                    .collect::<Result<Tags, _>>()
                    .map_err(|error| invalid(&error))?;
                Ok(event.with_tags(tags))
            }
            // The batch limit is not about one event, and `StoreLimit` is
            // `#[non_exhaustive]`: the rule only ever builds the three above.
            _ => Err(Violation::new(
                SY_18,
                format!("the rule cannot size an event against the {limit} limit"),
            )),
        }
    }

    /// A foreign event from `origin` at origin `position`, sized by [`sized`].
    fn sized_shipped(
        origin: StoreId,
        position: u64,
        event_type: EventType,
        limit: StoreLimit,
        len: usize,
    ) -> Result<ReplicatedEvent, Violation> {
        let position = SequencePosition::new(position)
            .ok_or_else(|| Violation::new(SY_18, "the rule's origin position is zero"))?;
        Ok(ReplicatedEvent::new(
            EventId::new(origin, position),
            RecordedAt::from_millis(1_800_000_000_000),
            sized(event_type, limit, len)?,
        ))
    }

    /// `outcome` is the refusal of `len` against `limit`, and states `Park`.
    fn ensure_exceeds<E: core::error::Error + Classify>(
        outcome: Result<Ingested, IngestError<E>>,
        limit: StoreLimit,
        len: usize,
        arrangement: &str,
    ) -> Result<(), Violation> {
        let error = match outcome {
            Ok(ingested) => {
                return Err(Violation::new(
                    SY_18,
                    format!(
                        "the store accepted {arrangement} as {ingested:?}; it declares that \
                         ceiling, so every event an ingest writes is held to it, not the head of \
                         each group"
                    ),
                ));
            }
            Err(error) => error,
        };
        let named = match &error {
            IngestError::ExceedsStoreLimit {
                limit: refused,
                len: measured,
                ..
            } => Some((*refused, *measured)),
            _ => None,
        };
        ensure(named == Some((limit, len)), SY_18, || {
            format!(
                "{arrangement} was refused as `{error}`, not as the {limit} limit exceeded by \
                 {len}; VT-25 makes a capacity refusal one a runner can tell from a retry"
            )
        })?;
        ensure(error.class() == FailureClass::Park, SY_18, || {
            format!(
                "the refusal of {arrangement} states {:?}; a per-event ceiling is Park",
                error.class()
            )
        })
    }

    /// What a refused or skipped ingest must leave as it found it.
    #[derive(Debug, PartialEq)]
    struct Observed {
        log: Vec<SequencedEvent>,
        head: Option<SequencePosition>,
        watermark: Watermark,
    }

    async fn observe<S: EventStore + IngestStore>(
        store: &S,
        clause: &'static str,
    ) -> Result<Observed, Violation> {
        Ok(Observed {
            log: read_all(store, clause).await?,
            head: head(store, clause).await?,
            watermark: watermark(store, clause).await?,
        })
    }

    /// The store's log, head and watermark are `before`'s after `what`.
    async fn ensure_unchanged<S: EventStore + IngestStore>(
        store: &S,
        before: &Observed,
        what: &str,
        clause: &'static str,
    ) -> Result<(), Violation> {
        let after = observe(store, clause).await?;
        ensure(after == *before, clause, || {
            format!(
                "{what} changed the store: it held {} events at head {:?} before and holds {} at \
                 head {:?} after, or its watermark moved",
                before.log.len(),
                before.head,
                after.log.len(),
                after.head
            )
        })
    }

    /// Whether the store holds `id`.
    async fn holds<S: EventStore>(
        store: &S,
        id: EventId,
        clause: &'static str,
    ) -> Result<bool, Violation> {
        store.contains_event_id(id).await.map_err(|error| {
            Violation::new(
                clause,
                format!("the store refused contains_event_id: {error}"),
            )
        })
    }

    /// The store's own identity, or a violation of `clause` when it cannot
    /// read it: a rule must not arrange a claim, or call an origin foreign,
    /// against an identity the store never stated.
    async fn own_identity<S: IngestStore>(
        store: &S,
        clause: &'static str,
    ) -> Result<StoreId, Violation> {
        store.store_id().await.map_err(|error| {
            Violation::new(
                clause,
                format!("the store could not read its own identity: {error}"),
            )
        })
    }

    /// Every origin a rule uses is foreign to the store under test, or the
    /// rule would be testing SY-38 by accident. Answers the store's own
    /// identity, which the SY-38 arrangements claim.
    async fn ensure_foreign<S: IngestStore>(
        store: &S,
        origins: &[StoreId],
        clause: &'static str,
    ) -> Result<StoreId, Violation> {
        let own = own_identity(store, clause).await?;
        ensure(!origins.contains(&own), clause, || {
            format!(
                "the fixture's store has the incarnation {own}, which this rule uses as a \
                 foreign origin; a fixture must give each store a fresh one"
            )
        })?;
        Ok(own)
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
/// assert!(names.contains(&"an_unheld_own_identity_stops_the_ingest"));
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

            // --- An unheld own identity stops the ingest (SY-38) ------------
            an_unheld_own_identity_stops_the_ingest,

            // --- A group lands whole under a fault (SY-30) ------------------
            ingest_group_is_atomic_under_a_mid_group_fault,

            // --- Every event is held to the declared ceilings (SY-18) -------
            ingest_validates_every_event_not_the_head,
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

#[cfg(test)]
mod tests {
    use core::future::Future;

    use happenstance_testkit::{RuleOutcome, block_on};

    use super::{MAX_PROVOKED_TAGS, NO_PER_EVENT_LIMITS, NO_PER_EVENT_LIMITS_REASON, ingest_rules};
    use crate::{IngestFixture, MemoryIngestFixture, MemoryIngestHandle};

    /// The oracle, declaring a tag ceiling at the rule's cap and nothing else.
    struct TagsAtTheCap(MemoryIngestFixture);

    impl IngestFixture for TagsAtTheCap {
        type Store = MemoryIngestHandle;
        const MAX_TAGS_PER_EVENT: Option<usize> = Some(MAX_PROVOKED_TAGS);

        fn connect(&self) -> impl Future<Output = Self::Store> {
            self.0.connect()
        }
    }

    /// A ceiling the rule would have to allocate past its cap to exceed is
    /// not exercised (H-13), and a fixture with nothing else declared is a
    /// skip, not a pass. The oracle enforces no ceiling, so an exercised one
    /// would fail here.
    #[test]
    fn a_ceiling_at_the_cap_is_not_exercised() {
        let outcome = block_on(ingest_rules::ingest_validates_every_event_not_the_head(
            async || TagsAtTheCap(MemoryIngestFixture::new()),
        ));

        assert_eq!(
            outcome.map_err(|violation| violation.clause()),
            Ok(RuleOutcome::Skipped {
                capability: NO_PER_EVENT_LIMITS,
                reason: NO_PER_EVENT_LIMITS_REASON,
            })
        );
    }
}
