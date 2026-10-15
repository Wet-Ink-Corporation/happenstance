//! The sync rules, their one enumeration, and the three emitters.
//!
//! Private: what a caller reaches is [`rules`], re-exported at the crate root,
//! and the macros, which `#[macro_export]` puts there regardless.

/// The `capability` SY-37's rule reports when the peer never showed it a
/// second page: the two declared limits that decide how much one pull carries.
///
/// Not a fixture constant, which is why it is spelled with its path: the
/// limits are the peer's own declaration, and they are what an adapter author
/// changes (or a fixture narrows) to let the rule see a page boundary.
pub const NO_SECOND_PAGE: &str = "PeerLimits::max_batch_events, PeerLimits::max_batch_bytes";

/// The reason SY-37's rule reports when the peer never showed it a second page.
///
/// Written by this crate rather than by the fixture, as
/// `happenstance_testkit::NO_CEILING_REASON` is, because it is the same
/// sentence for every peer that earns it. What the skip leaves unverified is
/// the whole of SY-37: the rule cannot fail a peer whose every answer fits in
/// one pull.
pub const NO_SECOND_PAGE_REASON: &str = "this peer's declared limits let one pull carry everything the rule will \
     write, so no page boundary was observed and a `Nothing` answered at one \
     could not be checked; SY-37 is unverified for this peer";

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
        AppendCondition, Event, EventId, EventType, Query, QueryItem, RecordedAt, SequencePosition,
        StoreId, Tag, Tags,
    };
    use happenstance_sync::{
        Classify, EventGroup, PeerLimits, PushBatch, ReplicatedEvent, SyncPeer, Waiting,
    };
    use happenstance_testkit::RuleOutcome;

    use crate::SyncFixture;
    use crate::violation::{Violation, ensure};

    /// The most pulls a rule makes before it decides the peer will never say
    /// it has caught up. Far above what two or three tiny groups need under the
    /// minimum conformant limits; its job is to turn a peer that loops into a
    /// failure rather than a hang (H-13).
    const MAX_PULLS: usize = 64;

    /// The most groups past one that SY-37's rule writes to a peer in one log,
    /// whatever limits the peer declares (H-13). Twice the minimum conformant
    /// `max_batch_events`, so the reference peer is filled one past its page.
    const FILL_CAP: usize = 256;

    const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
    const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");
    const READING_TAKEN: EventType = EventType::from_static("ReadingTaken");

    /// Two origins and an adjudicating hub, as raw store identities.
    const ORIGIN_A: StoreId = StoreId::from_bytes([0x11; 16]);
    const ORIGIN_B: StoreId = StoreId::from_bytes([0x22; 16]);
    const HUB: StoreId = StoreId::from_bytes([0x33; 16]);

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

    /// SY-37: a peer that answers a pull with [`Waiting::Nothing`] hands back
    /// an empty batch on the next pull from the token that answer carried.
    ///
    /// The rule writes two logs, each to a fresh peer from `open`, and each one
    /// group longer than one pull may carry under the peer's own declared
    /// limits:
    ///
    /// 1. **Cut by count**: [`PeerLimits::max_batch_events`] plus one
    ///    single-event groups of a few bytes (the count capped at 256).
    /// 2. **Cut by bytes**: single-event groups whose payloads are sized so
    ///    [`PeerLimits::max_batch_bytes`] ends a page before the event count
    ///    does. Written only where the bytes do cut first, and the log fits in
    ///    257 groups and 64 MiB.
    ///
    /// On each it follows the peer's tokens from the start, and on the first
    /// `Nothing` pulls once more from that token. That pull must be empty. A
    /// peer that never says `Nothing` passes: it costs its runner the empty
    /// pull, and never a missed event.
    ///
    /// If neither log showed a second non-empty page, the rule could not have
    /// failed the peer, and it reports a skip naming [`NO_SECOND_PAGE`]
    /// rather than a pass.
    ///
    /// The bound on pulls — one more than the log has groups — is the suite's
    /// liveness guard, not SY-37's claim: a peer that ignores its token and
    /// keeps handing back groups is reported here, and fails SY-1's and SY-2's
    /// rules too.
    ///
    /// [`NO_SECOND_PAGE`]: crate::NO_SECOND_PAGE
    ///
    /// # Rejects
    ///
    /// A peer that says `Nothing` because its page ran out rather than its log
    /// — the natural slip for an adapter that reports "this page is the last
    /// one I fetched" — so a runner that believes it stops short and the rest
    /// of the log waits for the next wake. `tests/wrong_peers.rs` carries two:
    /// a peer that says `Nothing` on every pull, which the count-cut log
    /// catches, and one that says it whenever a page carried fewer events than
    /// `max_batch_events`, which only the byte-cut log can catch.
    ///
    /// # Does not reject
    ///
    /// A wrong [`Waiting::Events`] count: it is advisory, and checking it would
    /// need a snapshot of the far side the port does not offer. Nor a peer
    /// that says `Nothing` while stalled behind a group too large for any pull:
    /// SY-37 forbids it, but a conformant push cannot put such a group on the
    /// far side, so the rule cannot build the case. Nor a peer whose single
    /// pull carries all of both logs, which it reports as a skip; the caps
    /// bound the work against a peer that declares huge limits (H-13), at that
    /// price.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-37 if the peer refuses a push inside its own
    /// limits, refuses a pull, keeps handing back groups past the log the rule
    /// wrote, or hands back events on the pull after it said nothing was
    /// waiting.
    pub async fn nothing_waiting_means_the_next_pull_is_empty<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-37";

        let paged_by_count = {
            let fixture = open().await;
            let peer = fixture.connect().await;
            let limits = peer.limits();
            let fill = Fill::cut_by_count(&limits);
            fill_with_readings(&peer, &limits, fill, CLAUSE).await?;
            follow_tokens(&peer, fill.groups, CLAUSE).await?
        };

        let fixture = open().await;
        let peer = fixture.connect().await;
        let limits = peer.limits();
        let paged_by_bytes = match Fill::cut_by_bytes(&limits) {
            Some(fill) => {
                fill_with_readings(&peer, &limits, fill, CLAUSE).await?;
                follow_tokens(&peer, fill.groups, CLAUSE).await?
            }
            None => false,
        };

        Ok(if paged_by_count || paged_by_bytes {
            RuleOutcome::Ran
        } else {
            RuleOutcome::Skipped {
                capability: crate::NO_SECOND_PAGE,
                reason: crate::NO_SECOND_PAGE_REASON,
            }
        })
    }

    /// One log SY-37's rule writes: how many single-event groups, how many
    /// payload bytes each carries at least, and how many groups go in a push.
    #[derive(Debug, Clone, Copy)]
    struct Fill {
        groups: usize,
        payload: usize,
        per_push: usize,
    }

    impl Fill {
        /// One group more than `max_batch_events`, capped, with tiny payloads.
        fn cut_by_count(limits: &PeerLimits) -> Self {
            Self {
                groups: limits.max_batch_events.min(FILL_CAP).saturating_add(1),
                payload: 0,
                per_push: limits.max_batch_events.max(1),
            }
        }

        /// One group more than `max_batch_bytes` admits at the largest payload
        /// the peer takes, when that is fewer events than `max_batch_events`;
        /// `None` when the count cuts first or the log would pass either cap.
        fn cut_by_bytes(limits: &PeerLimits) -> Option<Self> {
            let payload = limits.max_event_bytes.min(limits.max_batch_bytes);
            let per_page = limits.max_batch_bytes.checked_div(payload)?;
            let groups = per_page.checked_add(1)?;
            let bytes = groups.checked_mul(payload)?;
            let fits = per_page < limits.max_batch_events
                && groups <= FILL_CAP.saturating_add(1)
                && bytes <= crate::MAX_PROVOKED_BATCH_BYTES;
            fits.then_some(Self {
                groups,
                payload,
                per_push: per_page.max(1),
            })
        }
    }

    /// Follows `peer`'s tokens from the start of a log of `groups` groups, and
    /// checks the pull after the first `Nothing`. Answers whether a second
    /// non-empty page was seen, which is when a `Nothing` could have been a
    /// lie about a page boundary.
    async fn follow_tokens<P: SyncPeer>(
        peer: &P,
        groups: usize,
        clause: &'static str,
    ) -> Result<bool, Violation> {
        let refused = |error| Violation::new(clause, format!("the peer refused a pull: {error}"));
        let mut resume = None;
        let mut pages = 0_usize;
        // The suite's liveness guard: a log of `groups` groups cannot fill more
        // non-empty pulls than that unless the peer repeats itself; one more for
        // the pull that ends it.
        for _ in 0..=groups {
            let pulled = peer.pull(resume.as_ref()).await.map_err(refused)?;
            if !pulled.batch.groups.is_empty() {
                pages = pages.saturating_add(1);
            }
            match pulled.waiting {
                Waiting::Nothing => {
                    let next = peer.pull(Some(&pulled.resume)).await.map_err(refused)?;
                    let missed = next.batch.events().count();
                    ensure(missed == 0, clause, || {
                        format!(
                            "the peer said nothing was waiting, and the next pull from the token \
                             it returned carried {missed} event(s); a runner that believed it \
                             stopped short of them"
                        )
                    })?;
                    return Ok(pages >= 2);
                }
                Waiting::Unknown | Waiting::Events(_) if pulled.batch.groups.is_empty() => {
                    return Ok(pages >= 2);
                }
                Waiting::Unknown | Waiting::Events(_) => resume = Some(pulled.resume),
            }
        }
        Err(Violation::new(
            clause,
            format!(
                "the peer was still handing back groups of a {groups}-group log after {} pulls",
                groups.saturating_add(1)
            ),
        ))
    }

    /// Pushes `fill.groups` single-event groups from [`ORIGIN_A`], at most
    /// `fill.per_push` to a push, each push checked against `limits` before it
    /// is offered.
    async fn fill_with_readings<P: SyncPeer>(
        peer: &P,
        limits: &PeerLimits,
        fill: Fill,
        clause: &'static str,
    ) -> Result<(), Violation> {
        let Fill {
            groups: count,
            payload,
            per_push: chunk,
        } = fill;
        let mut position = SequencePosition::FIRST;
        let mut groups = Vec::with_capacity(chunk.min(count));
        for reading in 1..=count {
            // Distinct opaque bytes per event, padded to the fill's payload
            // size, and never parsed (SY-35).
            let mut data = reading.to_string().into_bytes();
            data.resize(data.len().max(payload), 0);
            let event = Event::new(READING_TAKEN, data).map_err(|error| {
                Violation::new(clause, format!("the rule's own event is invalid: {error}"))
            })?;
            groups.push(EventGroup::new(
                None,
                vec![ReplicatedEvent::new(
                    EventId::new(ORIGIN_A, position),
                    RecordedAt::from_millis(1_800_000_000_000),
                    event,
                )],
            ));
            position = position
                .next()
                .ok_or_else(|| Violation::new(clause, "the rule's log ran out of positions"))?;

            if groups.len() == chunk || reading == count {
                let batch = PushBatch::new(core::mem::take(&mut groups));
                ensure(limits.admits(&batch), clause, || {
                    format!(
                        "the peer's declared limits do not admit a batch of {} single-event \
                         groups the rule sized from those limits ({} payload bytes): {limits:?}",
                        batch.events().count(),
                        batch.payload_len()
                    )
                })?;
                peer.push(&batch).await.map_err(|error| {
                    Violation::new(
                        clause,
                        format!("the peer refused a push inside its own limits: {error}"),
                    )
                })?;
            }
        }
        Ok(())
    }

    /// Pulls from the start of the peer's log until it hands back an empty
    /// batch, which the port defines as caught up.
    ///
    /// It ignores [`Pulled::waiting`](happenstance_sync::Pulled::waiting) on
    /// purpose, so a peer wrong about it fails SY-37's rule and only that one.
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

            // --- Every refusal states its class (SY-36) ---------------------
            every_refusal_states_its_class,

            // --- Nothing waiting means nothing waiting (SY-37) --------------
            nothing_waiting_means_the_next_pull_is_empty,
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
