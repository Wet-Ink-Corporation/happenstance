//! The sync rules, their one enumeration, and the three emitters.
//!
//! Private: what a caller reaches is [`rules`], re-exported at the crate root,
//! and the macros, which `#[macro_export]` puts there regardless.

/// The sync rules. See the [crate documentation](crate) for what each covers.
///
/// Each rule is an async function taking `impl AsyncFn() -> F` — not a fixture
/// but how to make one, as in the event-store family — and returning
/// `Result<RuleOutcome, Violation>`.
///
/// # What no rule here does
///
/// **Decode a payload** (SY-35). Events are compared whole — identity, recorded
/// time and the opaque bytes — and never parsed. A suite that parsed `data`
/// would certify a peer that does, and ADR-0003's guarantee is that no peer
/// needs to.
///
/// **Hold two peers** (SY-8). Where a clause speaks of two peers, the rule plays
/// one of them itself: it is the side that pushes and pulls, and the fixture's
/// peer is the far side.
pub mod rules {
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventType, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN,
        MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_TAGS_PER_EVENT, Query, QueryItem, RecordedAt,
        SequencePosition, StoreId, Tag, Tags,
    };
    use happenstance_sync::{Classify, EventGroup, PushBatch, ReplicatedEvent, SyncPeer};
    use happenstance_testkit::RuleOutcome;

    use crate::kv_capped::turnstile_seat_map;
    use crate::violation::{Violation, ensure};
    use crate::{MAX_PROVOKED_BATCH_BYTES, SyncFixture};

    /// The most pulls a rule makes before it decides the peer will never say
    /// it has caught up. Far above what two or three tiny groups need under the
    /// minimum conformant limits; its job is to turn a peer that loops into a
    /// failure rather than a hang (H-13).
    const MAX_PULLS: usize = 64;

    const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
    const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");

    /// Two origins and an adjudicating hub, as raw store identities.
    const ORIGIN_A: StoreId = StoreId::from_bytes([0x11; 16]);
    const ORIGIN_B: StoreId = StoreId::from_bytes([0x22; 16]);
    const HUB: StoreId = StoreId::from_bytes([0x33; 16]);

    /// SY-18's three origins: the one that committed the seat map, and the two
    /// whose events sit at the declared boundary, in data and in metadata.
    const SEAT_MAP_ORIGIN: StoreId = StoreId::from_bytes([0x44; 16]);
    const BOUNDARY_ORIGIN: StoreId = StoreId::from_bytes([0x55; 16]);
    const METADATA_ORIGIN: StoreId = StoreId::from_bytes([0x66; 16]);

    /// SY-1: a peer appends every replicated event it does not already hold,
    /// and never evaluates an append condition — the origin's or its own — as a
    /// precondition.
    ///
    /// Two origins each claim the same seat under **byte-identical,
    /// position-free** guards: each decision was valid where it was made, and
    /// together they conflict. The rule pushes both, as the two origins would,
    /// and then pulls the peer's whole log. Both claims must be there.
    ///
    /// # Rejects
    ///
    /// A peer that evaluates the travelling guard against what it holds and
    /// refuses — or silently drops — the second claim. It is the natural first
    /// cut, because the guard arrives on the wire already, and it produces a
    /// peer set that never converges. `tests/wrong_peers.rs` carries both
    /// spellings: one that acknowledges the dropped claim as nothing appended,
    /// which the ack check catches, and one that acknowledges it as appended
    /// and drops it anyway, which only the pull of the whole log can see.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-1 if the peer refuses either push, reports either
    /// claim as anything but newly appended, or does not hand both back.
    pub async fn ingest_never_rejects<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-1";
        let fixture = open().await;
        let peer = fixture.connect().await;
        let guard = seat_guard(CLAUSE)?;

        let mine = claim(ORIGIN_A, CLAUSE)?;
        let theirs = claim(ORIGIN_B, CLAUSE)?;
        for event in [&mine, &theirs] {
            let batch = PushBatch::new(vec![EventGroup::new(
                Some(guard.clone()),
                vec![event.clone()],
            )]);
            let ack = peer.push(&batch).await.map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!(
                        "the peer refused a replicated claim from {}: {error}",
                        event.id
                    ),
                )
            })?;
            ensure(ack.appended == 1 && ack.skipped == 0, CLAUSE, || {
                format!(
                    "the peer acknowledged the claim from {} as {} appended and {} skipped; \
                     it held nothing with that identity, so it must append it",
                    event.id, ack.appended, ack.skipped
                )
            })?;
        }

        let held: Vec<ReplicatedEvent> = pull_everything(&peer, CLAUSE)
            .await?
            .into_iter()
            .flat_map(|group| group.events)
            .collect();
        for expected in [&mine, &theirs] {
            ensure(held.contains(expected), CLAUSE, || {
                format!(
                    "the claim from {} is not in the peer's log, byte for byte; a conflicting \
                     claim already there is not a reason to drop it",
                    expected.id
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// SY-2, the transport's half: a losing event and the compensation that
    /// answers it travel as one group, and a peer hands them back as one group.
    ///
    /// A hub has adjudicated a conflict: one claim won, and the other is sent on
    /// together with the reversal the hub wrote for it, in one
    /// [`EventGroup`]. Whoever pulls that from this peer ingests a group at a
    /// time, atomically (`IngestStore::ingest`), so a peer that splits the group
    /// hands its reader a log where the losing claim stands with nothing
    /// resolving it. The store-side half — one ingest commits the group whole —
    /// rides on ES-18 and on the ingest suite.
    ///
    /// # Rejects
    ///
    /// A peer that flattens what it is pushed into one group per event, or
    /// that pages by event count and ends a pull between the two.
    /// `tests/wrong_peers.rs` carries both.
    ///
    /// # Does not reject
    ///
    /// **Compensate-after-commit**, the defect SY-2's own `Rejects` paragraph
    /// names: ingest the losing event, return, and author the compensation
    /// afterwards. That is the authoring side's mistake, made before anything
    /// reaches a peer, and a rule that holds only a peer cannot see it — this
    /// rule hands the peer a group that is already whole. The authoring half is
    /// tracked by #320; until it lands, a green run here certifies the
    /// transport and nothing upstream of it.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-2 if the peer refuses the pushes, or if the group
    /// it hands back with the losing claim does not also hold its reversal.
    pub async fn compensation_is_atomic_with_the_losing_event<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-2";
        let fixture = open().await;
        let peer = fixture.connect().await;

        let winner = claim(ORIGIN_A, CLAUSE)?;
        let losing = claim(ORIGIN_B, CLAUSE)?;
        let reversal = replicated(HUB, CLAIM_REVERSED, CLAUSE)?;
        let batch = PushBatch::new(vec![
            EventGroup::new(None, vec![winner]),
            EventGroup::new(None, vec![losing.clone(), reversal.clone()]),
        ]);
        peer.push(&batch).await.map_err(|error| {
            Violation::new(
                CLAUSE,
                format!("the peer refused a batch well inside the minimum limits: {error}"),
            )
        })?;

        let groups = pull_everything(&peer, CLAUSE).await?;
        let Some(carrying) = groups.iter().find(|group| group.events.contains(&losing)) else {
            return Err(Violation::new(
                CLAUSE,
                format!("the losing claim {} never came back", losing.id),
            ));
        };
        ensure(carrying.events.contains(&reversal), CLAUSE, || {
            format!(
                "the losing claim {} came back in a group without its reversal {}; a reader \
                 ingesting that group commits the claim with nothing resolving it",
                losing.id, reversal.id
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// SY-18: a peer declares the limits it imposes, synchronously, and the
    /// declaration is true: an event it admits lands, so a runner that read it
    /// learns of an event that cannot land *before* pushing, not at ingest.
    ///
    /// The rule plays that runner. It reads [`SyncPeer::limits`] before it
    /// pushes anything, and then:
    ///
    /// 1. reads it again, because a declaration that changes between two reads
    ///    cannot be relied on before a push;
    /// 2. holds `max_event_bytes` to VT-21's floor,
    ///    [`MIN_SUPPORTED_EVENT_DATA_LEN`]. Every peer the suite is mounted on
    ///    passes this line, which is where the floor is compared across the
    ///    peer set;
    /// 3. runs the Turnstile experiment: a 340,000-byte seat map, already
    ///    committed at its origin. If the declaration admits it, it is pushed
    ///    and must land. If not, the runner parks it without a round trip,
    ///    which is the prevention, and nothing is pushed;
    /// 4. pushes two events at the declaration's own boundary and at the
    ///    contract's envelope floors at once: `max_event_bytes` of opaque bytes
    ///    (capped by `max_batch_bytes`), carried once in data and once in
    ///    metadata, because the declaration bounds the two together; a type of
    ///    [`MAX_EVENT_TYPE_LEN`] bytes; and [`MIN_SUPPORTED_TAGS_PER_EVENT`]
    ///    tags of [`MAX_TAG_LEN`] bytes each. The declaration must admit each,
    ///    and each must land, acknowledged as appended. A limit on the envelope
    ///    or on metadata that the peer imposes without declaring it is a limit
    ///    discovered at ingest;
    /// 5. pulls the whole log back: every event that landed is held, byte for
    ///    byte, which rejects a peer that accepts and truncates.
    ///
    /// The asynchronous form of the declaration, which SY-18 also rejects,
    /// cannot be written: `limits` is not `async` on the port, so the compiler
    /// is that check and no mutant is needed for it.
    ///
    /// A peer that declares more than [`MAX_PROVOKED_BATCH_BYTES`] has its
    /// boundary tested at that ceiling rather than at its declaration, because
    /// the event is allocated whole (H-13). The rule still runs.
    ///
    /// # Rejects
    ///
    /// `tests/wrong_limits.rs` carries eight mistakes over the KV-capped
    /// reference peer, one for each check above:
    ///
    /// * declarations rewritten over unchanged behaviour: no value cap, which
    ///   admits the seat map that ingest then parks; the raw 131,072-byte value
    ///   cap, as if a value held the payload alone, which admits a boundary
    ///   event ingest refuses; a byte below VT-21's floor; a declaration that
    ///   changes between two reads; and `max_batch_events` of zero, which
    ///   admits no event at all;
    /// * behaviour changed under an honest declaration: a cap of 1,024 bytes on
    ///   metadata, Workers KV's, that the declaration does not state; data
    ///   truncated on a large push and acknowledged as appended; and a large
    ///   event stored whole and acknowledged as skipped.
    ///
    /// # Does not reject
    ///
    /// A runner that ignores `limits()`. That is the runner's defect, and #244
    /// owns it; a peer can only make its declaration true. Nor limits that go
    /// stale between the read and the push: [`PeerLimits::admits`] is advisory,
    /// and what keeps that survivable is the `Park` and `Split` classes SY-36
    /// checks.
    ///
    /// Nor a lie about `max_batch_bytes` or `max_batch_events` alone. SY-18
    /// is about the limits a peer imposes on *an event*; the batch limits are
    /// read here only to size the boundary event, and a batch over them is
    /// SY-36's `Split`, not a fact that can never land.
    ///
    /// [`PeerLimits::admits`]: happenstance_sync::PeerLimits::admits
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-18 if the declaration changes between two reads,
    /// declares below VT-21's floor, or admits no event at its own boundary;
    /// or if an event the declaration admits is refused, acknowledged as
    /// anything but one appended, or not handed back whole. The floor breach
    /// cites VT-21 in its message.
    pub async fn peer_declares_its_own_limits<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-18";
        let fixture = open().await;
        let peer = fixture.connect().await;

        let limits = peer.limits();
        let again = peer.limits();
        ensure(again == limits, CLAUSE, || {
            format!(
                "the peer declared {limits:?} and then {again:?}; a declaration that changes \
                 between two reads cannot be relied on before a push"
            )
        })?;
        ensure(
            limits.max_event_bytes >= MIN_SUPPORTED_EVENT_DATA_LEN,
            CLAUSE,
            || {
                format!(
                    "the peer declares max_event_bytes of {}, below VT-21's floor of \
                     {MIN_SUPPORTED_EVENT_DATA_LEN}",
                    limits.max_event_bytes
                )
            },
        )?;

        let mut landed = Vec::new();
        let seat_map = one_event(turnstile_seat_map(SEAT_MAP_ORIGIN, CLAUSE)?);
        if limits.admits(&seat_map) {
            push_admitted(&peer, &seat_map, "the 340,000-byte seat map", CLAUSE).await?;
            landed.push(seat_map);
        }

        let size = limits
            .max_event_bytes
            .min(limits.max_batch_bytes)
            .min(MAX_PROVOKED_BATCH_BYTES);
        for carrier in [Carrier::Data, Carrier::Metadata] {
            let boundary = one_event(event_at_the_envelope_floors(size, carrier, CLAUSE)?);
            ensure(limits.admits(&boundary), CLAUSE, || {
                format!(
                    "the declaration {limits:?} admits no event of {size} bytes in its \
                     {carrier:?}, the size it states it accepts"
                )
            })?;
            push_admitted(&peer, &boundary, carrier.describe(), CLAUSE).await?;
            landed.push(boundary);
        }

        let held: Vec<ReplicatedEvent> = pull_everything(&peer, CLAUSE)
            .await?
            .into_iter()
            .flat_map(|group| group.events)
            .collect();
        for expected in landed.iter().flat_map(PushBatch::events) {
            ensure(held.contains(expected), CLAUSE, || {
                format!(
                    "event {} of {} bytes was acknowledged and is not in the peer's log, \
                     byte for byte",
                    expected.id,
                    expected.payload_len()
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// SY-36: every error a peer reports states its failure class, and the
    /// class is the one its adapter says it is.
    ///
    /// The fixture drives a fresh handle into every refusal it can produce and
    /// declares each one's class ([`SyncFixture::refusals`]); the rule checks
    /// that each provocation was refused, and that the error's
    /// [`Classify::class`] answers what was declared. The class is read twice
    /// from the same value, because SY-36 makes it a function of the error
    /// alone.
    ///
    /// # Rejects
    ///
    /// A peer that reports every failure as one class — the opaque error
    /// ADR-0026 rejected, with a trait impl bolted on so it compiles.
    /// `tests/wrong_peers.rs` carries it, as `Transient` for everything, which
    /// retries a batch that can only ever be split.
    ///
    /// The rule's other three refusals each have a mutant there too: a fixture
    /// that declares no refusal, a peer that accepts the push its fixture says
    /// it refuses, and an error whose class changes between two reads.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-36 if the fixture declares no refusal, if a
    /// provocation succeeds, or if a refusal's class is not the declared one or
    /// changes between two reads.
    pub async fn every_refusal_states_its_class<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-36";
        let fixture = open().await;
        let peer = fixture.connect().await;

        let refusals = fixture.refusals(&peer).await?;
        ensure(!refusals.is_empty(), CLAUSE, || {
            String::from(
                "the fixture declared no refusal; every peer can be refused something, and a \
                 rule checked against nothing has checked nothing",
            )
        })?;
        for refusal in refusals {
            let Err(error) = refusal.outcome else {
                return Err(Violation::new(
                    CLAUSE,
                    format!(
                        "the peer accepted {}, which its fixture says it refuses",
                        refusal.provoked_by
                    ),
                ));
            };
            let (first, second) = (error.class(), error.class());
            ensure(first == second, CLAUSE, || {
                format!(
                    "the refusal of {} answered {first:?} and then {second:?}; the class must \
                     be a function of the error alone",
                    refusal.provoked_by
                )
            })?;
            ensure(first == refusal.declared, CLAUSE, || {
                format!(
                    "the refusal of {} ({error}) states {first:?}, and its fixture declares \
                     {:?}",
                    refusal.provoked_by, refusal.declared
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// Pulls from the start of the peer's log until it hands back an empty
    /// batch, which the port defines as caught up.
    async fn pull_everything<P: SyncPeer>(
        peer: &P,
        clause: &'static str,
    ) -> Result<Vec<EventGroup>, Violation> {
        let mut groups = Vec::new();
        let mut resume: Option<P::Resume> = None;
        for _ in 0..MAX_PULLS {
            let pulled = peer.pull(resume.as_ref()).await.map_err(|error| {
                Violation::new(clause, format!("the peer refused a pull: {error}"))
            })?;
            if pulled.batch.groups.is_empty() {
                return Ok(groups);
            }
            groups.extend(pulled.batch.groups);
            resume = Some(pulled.resume);
        }
        Err(Violation::new(
            clause,
            format!("the peer was still handing back groups after {MAX_PULLS} pulls"),
        ))
    }

    /// `event` as a batch of one group.
    fn one_event(event: ReplicatedEvent) -> PushBatch {
        PushBatch::new(vec![EventGroup::new(None, vec![event])])
    }

    /// Pushes `batch`, which the peer's declaration admits, and requires it to
    /// land as one event appended. A refusal here is the failure SY-18 exists
    /// to move: found at ingest, after the origin committed.
    async fn push_admitted<P: SyncPeer>(
        peer: &P,
        batch: &PushBatch,
        what: &str,
        clause: &'static str,
    ) -> Result<(), Violation> {
        let ack = peer.push(batch).await.map_err(|error| {
            Violation::new(
                clause,
                format!(
                    "the declared limits admit {what} and the push refused it ({error}, class \
                     {:?}): the failure was relocated to ingest, not prevented",
                    error.class()
                ),
            )
        })?;
        ensure(ack.appended == 1 && ack.skipped == 0, clause, || {
            format!(
                "the peer acknowledged {what} as {} appended and {} skipped; it held nothing \
                 with that identity, so it must append it",
                ack.appended, ack.skipped
            )
        })
    }

    /// Which of an event's two opaque fields carries a boundary event's bytes.
    ///
    /// Both, one event each, because `max_event_bytes` bounds
    /// [`ReplicatedEvent::payload_len`], which is data and metadata together:
    /// a peer that stores metadata somewhere smaller has a limit its
    /// declaration does not state.
    #[derive(Debug, Clone, Copy)]
    enum Carrier {
        Data,
        Metadata,
    }

    impl Carrier {
        /// What the violation message calls the event this carrier builds.
        const fn describe(self) -> &'static str {
            match self {
                Self::Data => "an event at the declared boundary, in its data",
                Self::Metadata => "an event at the declared boundary, in its metadata",
            }
        }

        /// Each boundary event's origin, so the two are distinct facts.
        const fn origin(self) -> StoreId {
            match self {
                Self::Data => BOUNDARY_ORIGIN,
                Self::Metadata => METADATA_ORIGIN,
            }
        }
    }

    /// An event at every envelope floor the contract sets, carrying `size`
    /// opaque bytes (SY-35) in `carrier`: a type of [`MAX_EVENT_TYPE_LEN`]
    /// bytes and [`MIN_SUPPORTED_TAGS_PER_EVENT`] distinct tags of
    /// [`MAX_TAG_LEN`] bytes.
    fn event_at_the_envelope_floors(
        size: usize,
        carrier: Carrier,
        clause: &'static str,
    ) -> Result<ReplicatedEvent, Violation> {
        let event_type = EventType::new("L".repeat(MAX_EVENT_TYPE_LEN)).map_err(|error| {
            Violation::new(clause, format!("the rule's own type is invalid: {error}"))
        })?;
        let tags = (0..MIN_SUPPORTED_TAGS_PER_EVENT)
            .map(|i| Tag::new(format!("{i:0>MAX_TAG_LEN$}")))
            .collect::<Result<Tags, _>>()
            .map_err(|error| {
                Violation::new(clause, format!("the rule's own tag is invalid: {error}"))
            })?;
        let bytes = vec![0_u8; size];
        let event = match carrier {
            Carrier::Data => Event::new(event_type, bytes),
            Carrier::Metadata => {
                Event::new(event_type, Vec::new()).map(|event| event.with_metadata(bytes))
            }
        }
        .map_err(|error| {
            Violation::new(clause, format!("the rule's own event is invalid: {error}"))
        })?
        .with_tags(tags);
        Ok(ReplicatedEvent::new(
            EventId::new(carrier.origin(), SequencePosition::FIRST),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        ))
    }

    /// The guard both claims travel under: "nobody has claimed seat s1", with
    /// no position, so it is byte-identical wherever it was written.
    fn seat_guard(clause: &'static str) -> Result<AppendCondition, Violation> {
        let item = QueryItem::new([SEAT_CLAIMED], seat()).map_err(|error| {
            Violation::new(clause, format!("the rule's own query is invalid: {error}"))
        })?;
        Ok(AppendCondition::new(Query::from_item(item)))
    }

    fn seat() -> Tags {
        [Tag::from_static("seat:s1")].into_iter().collect()
    }

    /// `origin`'s claim on the seat, the first event in its log.
    fn claim(origin: StoreId, clause: &'static str) -> Result<ReplicatedEvent, Violation> {
        replicated(origin, SEAT_CLAIMED, clause)
    }

    fn replicated(
        origin: StoreId,
        event_type: EventType,
        clause: &'static str,
    ) -> Result<ReplicatedEvent, Violation> {
        // The payload is opaque bytes and stays that way (SY-35): a distinct
        // value per origin, so "byte for byte" means something.
        let event = Event::new(event_type, origin.to_string().into_bytes())
            .map_err(|error| {
                Violation::new(clause, format!("the rule's own event is invalid: {error}"))
            })?
            .with_tags(seat());
        Ok(ReplicatedEvent::new(
            EventId::new(origin, SequencePosition::FIRST),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        ))
    }
}

/// Hands the complete sync rule set to `$callback`.
///
/// This family's single enumeration. `$callback` is the path of a macro that
/// accepts a comma-separated list of identifiers, and it is invoked once, with
/// every rule in [`rules`].
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_sync_peer_rule!(rule_names);
/// assert!(names.contains(&"ingest_never_rejects"));
/// ```
#[macro_export]
macro_rules! for_each_sync_peer_rule {
    // Raw token trees rather than `$cb:path`, for the reason the event-store
    // family's enumeration records: a parsed `path` cannot sit in callee
    // position inside an expression.
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- Ingest never refuses (SY-1) --------------------------------
            ingest_never_rejects,

            // --- A group travels whole (SY-2) -------------------------------
            compensation_is_atomic_with_the_losing_event,

            // --- A peer declares its own limits (SY-18) ---------------------
            peer_declares_its_own_limits,

            // --- Every refusal states its class (SY-36) ---------------------
            every_refusal_states_its_class,
        }
    };
}

/// Emits one `#[tokio::test]` per sync rule.
///
/// The caller's crate needs `tokio` with `macros` and `rt` in its
/// `dev-dependencies`. The default emitter of
/// [`sync_peer_conformance!`](crate::sync_peer_conformance).
#[macro_export]
macro_rules! emit_sync_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::rules::$name(__conformance_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one plain `#[test]` per sync rule, driven by
/// [`happenstance_testkit::block_on`].
///
/// No runtime and no `Send` bound, so this is the harness a `!Send` peer runs
/// under — and the one SY-17's rule names.
#[macro_export]
macro_rules! emit_sync_blocking {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() -> ::core::result::Result<(), $crate::Violation> {
                $crate::__private::block_on($crate::__private::rules::$name(__conformance_fixture))?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}

/// Emits one `#[wasm_bindgen_test]` per sync rule.
///
/// The caller's crate needs `wasm-bindgen-test` in its `dev-dependencies`,
/// gated on `cfg(target_arch = "wasm32")`. A skip is logged through
/// `console_log!` rather than reported on stdout, which writes nowhere on that
/// target — the event-store family's `emit_wasm` says why at length.
#[macro_export]
macro_rules! emit_sync_wasm {
    ($($name:ident),* $(,)?) => {
        $(
            #[::wasm_bindgen_test::wasm_bindgen_test]
            async fn $name() -> ::core::result::Result<(), $crate::Violation> {
                let __outcome = $crate::__private::rules::$name(__conformance_fixture).await?;
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
macro_rules! __sync_rule_names {
    ($($name:ident),* $(,)?) => { [ $( ::core::stringify!($name) ),* ] };
}

/// Every `pub async fn` declared at the top level of this file's `rules`
/// module — the same four-space prefix `cargo xtask spec-trace` reads.
#[cfg(test)]
fn declared_rules() -> Vec<&'static str> {
    const SOURCE: &str = include_str!("sync_suite.rs");
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
/// enumerated without existing — the event-store family's `no_orphan_rules`,
/// one family over.
#[cfg(test)]
#[test]
fn no_orphan_sync_rules() {
    let registered = crate::for_each_sync_peer_rule!(crate::__sync_rule_names);
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
        "these rules are absent from `for_each_sync_peer_rule!`, so no harness runs them: \
         {orphans:?}"
    );
    let missing: Vec<_> = registered
        .iter()
        .filter(|name| !declared.contains(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "these rules are registered but not declared in `sync_suite.rs`: {missing:?}"
    );
}
