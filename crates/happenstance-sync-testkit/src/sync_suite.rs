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
///
/// A peer that refused one of the rule's logs earns [`LOG_REFUSED`] instead,
/// because the fix is the opposite one: not narrower limits, but a push that
/// honours the limits already declared.
pub const NO_SECOND_PAGE_REASON: &str = "this peer showed no page boundary: one pull carried everything the rule \
     wrote, so a `Nothing` answered at a boundary could not be checked; narrow \
     the declared batch limits to let the rule see a page. SY-37 is unverified \
     for this peer";

/// The `capability` SY-37's rule reports when the peer showed no page boundary
/// and refused a push of one of its logs that its own declaration admits.
///
/// Distinct from [`NO_SECOND_PAGE`] because the two need opposite fixes: that
/// one asks for smaller limits, this one for a peer that takes what its limits
/// already say it takes. Which log was refused is in the reason:
/// [`COUNT_LOG_REFUSED_REASON`] or [`BYTE_LOG_REFUSED_REASON`].
pub const LOG_REFUSED: &str = "SyncPeer::push, within the peer's own PeerLimits";

/// The reason SY-37's rule reports when the peer refused its count-cut log.
///
/// That log pushes at most [`PeerLimits::max_batch_events`] events of at most
/// three payload bytes to a push, so a refusal is the peer stating batch
/// limits it does not impose. **SY-18's batch rule judges that:**
/// [`peer_takes_a_batch_at_its_declared_limits`] pushes `max_batch_events`
/// events at once, at least as many as any push of this log carries, since
/// [`MAX_PROVOKED_BATCH_EVENTS`] is above the log's 257, and fails a peer that
/// refuses them. This skip names the refusal for SY-37's own sake, and only
/// when the byte-cut log did not page: a byte-cut log that paged makes the
/// rule [`RuleOutcome::Ran`]. When both logs are refused this is the reason
/// given, because a refused byte-cut log fails `peer_declares_its_own_limits`
/// as well.
///
/// [`RuleOutcome::Ran`]: happenstance_testkit::RuleOutcome::Ran
/// [`peer_takes_a_batch_at_its_declared_limits`]: rules::peer_takes_a_batch_at_its_declared_limits
/// [`MAX_PROVOKED_BATCH_EVENTS`]: crate::MAX_PROVOKED_BATCH_EVENTS
///
/// [`PeerLimits::max_batch_events`]: happenstance_sync::PeerLimits::max_batch_events
pub const COUNT_LOG_REFUSED_REASON: &str = "this peer refused a push of the count-cut log, whose pushes carry at \
     most `max_batch_events` events of at most three bytes each: its declared \
     batch limits are not the limits it imposes, which SY-18's \
     `peer_takes_a_batch_at_its_declared_limits` judges, and the log showed no \
     page; SY-37 is unverified for this peer";

/// The reason SY-37's rule reports when the peer refused its byte-cut log, and
/// the count-cut log showed no page.
///
/// That log pushes one event to a push, its payload
/// [`PeerLimits::max_event_bytes`] capped by `max_batch_bytes`: no larger than
/// the boundary event `peer_declares_its_own_limits` pushes alone. So a
/// refusal is the peer taking smaller events than it declares, SY-18's
/// subject, and a peer that refuses it on size fails that rule too. Because
/// each push holds one event, a peer that overstates only `max_batch_bytes`
/// takes the log and pages it, which no rule could otherwise judge.
///
/// [`PeerLimits::max_event_bytes`]: happenstance_sync::PeerLimits::max_event_bytes
pub const BYTE_LOG_REFUSED_REASON: &str = "this peer refused a one-event push of the byte-cut log, whose events are \
     sized from its declared `max_event_bytes`: it takes smaller events than \
     it declares (SY-18's subject, judged by its rule), and the count-cut log \
     showed no page; SY-37 is unverified for this peer";

/// The `capability` SY-31's rule reports when the peer's
/// acknowledgement confirmed none of what the rule pushed.
///
/// Not a fixture constant, as [`NO_SECOND_PAGE`] is not: it names the field
/// whose value left the rule nothing to check.
pub const NO_CONFIRMATION: &str = "Ack::confirmed";

/// The reason SY-31's rule reports when the peer confirmed none of what it
/// pushed.
///
/// Written by this crate rather than by the fixture, because it is the same
/// sentence for every peer that earns it. Confirming nothing is safe, since a
/// spoke re-pushes and a re-delivery is a no-op, but it leaves the rule no
/// confirmation to hold to the log.
pub const NO_CONFIRMATION_REASON: &str = "this peer acknowledged the rule's push and confirmed none of the events \
     in it, so no confirmation could be held to what a fresh handle reads \
     back; confirming nothing is safe, because a spoke re-pushes and a \
     re-delivery is a no-op, but SY-31's durability half is unverified for \
     this peer";

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
    use alloc::collections::BTreeMap;
    use alloc::format;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventType, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN,
        MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_TAGS_PER_EVENT, Query, QueryItem, RecordedAt,
        SequencePosition, StoreId, Tag, Tags,
    };
    use happenstance_sync::{
        Ack, Classify, EventGroup, PeerLimits, PushBatch, ReplicatedEvent, SyncPeer, Waiting,
    };
    use happenstance_testkit::RuleOutcome;

    use crate::kv_capped::turnstile_seat_map;
    use crate::violation::{Violation, ensure};
    use crate::{MAX_PROVOKED_BATCH_BYTES, MAX_PROVOKED_BATCH_EVENTS, SyncFixture};

    /// The most pulls a rule makes before it decides the peer will never say
    /// it has caught up. Far above what two or three tiny groups need under the
    /// minimum conformant limits; its job is to turn a peer that loops into a
    /// failure rather than a hang (H-13).
    const MAX_PULLS: usize = 64;

    /// The most groups past one that SY-37's rule writes to a peer in one log,
    /// whatever limits the peer declares (H-13). Twice the minimum conformant
    /// `max_batch_events`, so the reference peer pages more than once.
    const FILL_CAP: usize = 256;

    const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
    const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");
    const READING_TAKEN: EventType = EventType::from_static("ReadingTaken");

    /// Two origins and an adjudicating hub, as raw store identities.
    const ORIGIN_A: StoreId = StoreId::from_bytes([0x11; 16]);
    const ORIGIN_B: StoreId = StoreId::from_bytes([0x22; 16]);
    const HUB: StoreId = StoreId::from_bytes([0x33; 16]);

    /// SY-18's three origins: the one that committed the seat map, and the two
    /// whose events sit at the declared boundary, in data and in metadata.
    const SEAT_MAP_ORIGIN: StoreId = StoreId::from_bytes([0x44; 16]);
    const BOUNDARY_ORIGIN: StoreId = StoreId::from_bytes([0x55; 16]);
    const METADATA_ORIGIN: StoreId = StoreId::from_bytes([0x66; 16]);

    /// SY-31's two origins, whose events the rule pushes in one batch.
    const DURABLE_ORIGIN: StoreId = StoreId::from_bytes([0x81; 16]);
    const DURABLE_TWIN: StoreId = StoreId::from_bytes([0x82; 16]);

    /// The type of every event SY-18's batch rule pushes.
    const BATCH_FILLED: EventType = EventType::from_static("BatchFilled");

    /// SY-18's batch rule's two origins: the push at the declared event count
    /// with empty payloads, and the push at that count and the declared bytes.
    const COUNT_CORNER_ORIGIN: StoreId = StoreId::from_bytes([0x91; 16]);
    const BYTE_CORNER_ORIGIN: StoreId = StoreId::from_bytes([0x92; 16]);

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
    /// Nor a lie about `max_batch_bytes` or `max_batch_events` alone. This rule
    /// holds the limits a peer imposes on *an event*, and reads the batch
    /// limits only to size the boundary event. A batch at them that is refused
    /// is [`peer_takes_a_batch_at_its_declared_limits`]'s to judge, and a
    /// batch over them is SY-36's `Split`, not a fact that can never land.
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

    /// SY-37: a peer that answers a pull with [`Waiting::Nothing`] hands back
    /// an empty batch on the next pull from the token that answer carried.
    ///
    /// The rule writes two logs, each to a fresh peer from `open`:
    ///
    /// 1. **Cut by count**: 257 single-event groups of a few bytes, pushed at
    ///    most [`PeerLimits::max_batch_events`] to a push, so a peer that pages
    ///    at any event count up to 256 shows a page boundary.
    /// 2. **Cut by bytes**: single-event groups whose payloads are sized from
    ///    the declared limits so [`PeerLimits::max_batch_bytes`] ends a page
    ///    before the event count does, one group past that page, pushed one
    ///    event to a push so the pull alone draws the boundary. Written only
    ///    where the bytes do cut first, and the log fits in 257 groups and
    ///    64 MiB.
    ///
    /// On each it follows the peer's tokens from the start, and on the first
    /// `Nothing` pulls once more from that token. That pull must be empty. A
    /// peer that never says `Nothing` passes: it costs its runner the empty
    /// pull, and never a missed event.
    ///
    /// A log the peer refuses to take is not built, and shows no page. Whose
    /// failure the refusal is depends on the log. The byte-cut log goes one
    /// event to a push, each sized from [`PeerLimits::max_event_bytes`] and no
    /// larger than SY-18's boundary event, so refusing it is SY-18's subject,
    /// and `peer_declares_its_own_limits` judges it; a peer that overstates
    /// only `max_batch_bytes` takes it. The count-cut log is tiny events, at
    /// most `max_batch_events` to a push, so refusing it is a lie about the
    /// batch limits, which that rule says it does not judge and
    /// [`peer_takes_a_batch_at_its_declared_limits`] judges.
    ///
    /// If neither log showed a second non-empty page, the rule could not have
    /// failed the peer, and it reports a skip rather than a pass: naming
    /// [`LOG_REFUSED`] if the peer refused a log, with
    /// [`COUNT_LOG_REFUSED_REASON`] or [`BYTE_LOG_REFUSED_REASON`] saying which
    /// (the count-cut one if both), and [`NO_SECOND_PAGE`] if it took every log
    /// the rule wrote and one pull carried each. The two need opposite fixes: a
    /// push that honours the declared limits, and narrower limits. One log that
    /// paged is enough for the rule to have run, so a refusal of the other is
    /// not reported over it.
    ///
    /// The bound on pulls — one more than the log has groups — is the suite's
    /// liveness guard, not SY-37's claim: a peer that ignores its token and
    /// keeps handing back groups is reported here, and fails SY-1's and SY-2's
    /// rules too.
    ///
    /// [`NO_SECOND_PAGE`]: crate::NO_SECOND_PAGE
    /// [`LOG_REFUSED`]: crate::LOG_REFUSED
    /// [`COUNT_LOG_REFUSED_REASON`]: crate::COUNT_LOG_REFUSED_REASON
    /// [`BYTE_LOG_REFUSED_REASON`]: crate::BYTE_LOG_REFUSED_REASON
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
    /// A [`Violation`] of SY-37 if the peer refuses a pull, keeps handing back
    /// groups past the log the rule wrote, or hands back events on the pull
    /// after it said nothing was waiting.
    pub async fn nothing_waiting_means_the_next_pull_is_empty<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-37";

        let by_count = {
            let fixture = open().await;
            let peer = fixture.connect().await;
            write_log(&peer, Fill::cut_by_count(&peer.limits()), CLAUSE).await?
        };

        let fixture = open().await;
        let peer = fixture.connect().await;
        let by_bytes = match Fill::cut_by_bytes(&peer.limits()) {
            Some(fill) => write_log(&peer, fill, CLAUSE).await?,
            // Not written: the count cuts first, or the log would pass a cap.
            None => Log::NoPage,
        };

        Ok(match (by_count, by_bytes) {
            (Log::Paged, _) | (_, Log::Paged) => RuleOutcome::Ran,
            // The count-cut refusal first: it is the batch-limit lie, which
            // SY-18's batch rule judges, while its per-event rule reports the
            // byte-cut one.
            (Log::Refused, _) => RuleOutcome::Skipped {
                capability: crate::LOG_REFUSED,
                reason: crate::COUNT_LOG_REFUSED_REASON,
            },
            (Log::NoPage, Log::Refused) => RuleOutcome::Skipped {
                capability: crate::LOG_REFUSED,
                reason: crate::BYTE_LOG_REFUSED_REASON,
            },
            (Log::NoPage, Log::NoPage) => RuleOutcome::Skipped {
                capability: crate::NO_SECOND_PAGE,
                reason: crate::NO_SECOND_PAGE_REASON,
            },
        })
    }

    /// SY-31: an [`Ack`] confirms only what the far side still holds once the
    /// handle that acknowledged is gone.
    ///
    /// SY-31's confirmation watermark is advanced to [`Ack::confirmed`], "what
    /// the far side has now durably seen", and SY-24's forwarding never offers
    /// anything at or below it again. So a confirmation sent before the write
    /// is durable loses that write for good: the spoke's watermark has passed
    /// it, and nothing re-sends it.
    ///
    /// The rule pushes one batch of two groups from two origins, at origin
    /// positions 3 and 7 and 2, drops the handle that pushed, connects a fresh
    /// one onto the same fixture and pulls everything. Then every pushed event
    /// at or below the confirmed mark for its origin is held, byte for byte,
    /// and every confirmed mark names an event the fresh handle holds.
    /// `confirmed` is read as a high-water mark, never as everything up to: no
    /// position below a mark that the rule did not push is assumed held.
    ///
    /// "Not durable" is observed as "gone once the acking handle is dropped",
    /// which is the strongest crash a [`SyncFixture`] can express today: one
    /// fixture is one backing peer, and each `connect` one handle onto it. A
    /// peer that buffers in fixture-wide memory and loses it to a process crash
    /// passes here, until the fixture contract gains a reopen capability with
    /// the first durable peer adapter.
    ///
    /// A peer whose ack confirms none of the pushed events has given the rule
    /// nothing to check, so the rule reports
    /// [`Skipped`](RuleOutcome::Skipped) with
    /// [`NO_CONFIRMATION`](crate::NO_CONFIRMATION) rather than a pass.
    ///
    /// [`Ack`]: happenstance_sync::Ack
    /// [`Ack::confirmed`]: happenstance_sync::Ack::confirmed
    ///
    /// # Rejects
    ///
    /// A write-behind cache that acknowledges a push on admission and commits
    /// it later, so a handle dropped in between has confirmed what the peer
    /// never held. `tests/wrong_peers.rs` carries it, and the peer that
    /// confirms nothing, which this rule must skip.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-31 if the peer refuses the push or a pull, if a
    /// confirmed event is not held by a fresh handle, or if a confirmed mark
    /// names no event the fresh handle holds.
    pub async fn ack_never_confirms_a_non_durable_event<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-31";
        let fixture = open().await;
        let first = durable_reading(DURABLE_ORIGIN, 3, b"sy31-p-3", CLAUSE)?;
        let second = durable_reading(DURABLE_ORIGIN, 7, b"sy31-p-7", CLAUSE)?;
        let other = durable_reading(DURABLE_TWIN, 2, b"sy31-q-2", CLAUSE)?;
        let batch = PushBatch::new(vec![
            EventGroup::new(None, vec![first, second]),
            EventGroup::new(None, vec![other]),
        ]);

        let ack = {
            let peer = fixture.connect().await;
            peer.push(&batch).await.map_err(|error| {
                Violation::new(
                    CLAUSE,
                    format!("the peer refused a batch well inside the minimum limits: {error}"),
                )
            })?
            // The acking handle is dropped here: what it confirmed must
            // outlive it.
        };

        let fresh = fixture.connect().await;
        let held: Vec<ReplicatedEvent> = pull_everything(&fresh, CLAUSE)
            .await?
            .into_iter()
            .flat_map(|group| group.events)
            .collect();
        let covered: Vec<&ReplicatedEvent> = batch
            .events()
            .filter(|event| {
                ack.confirmed
                    .get(event.id.store())
                    .is_some_and(|confirmed| event.id.position() <= confirmed)
            })
            .collect();
        if covered.is_empty() {
            return Ok(RuleOutcome::Skipped {
                capability: crate::NO_CONFIRMATION,
                reason: crate::NO_CONFIRMATION_REASON,
            });
        }
        for event in covered {
            ensure(held.contains(event), CLAUSE, || {
                format!(
                    "the peer confirmed {} and a fresh handle onto it does not hold it, byte for \
                     byte; a spoke's watermark passes it, so it is never pushed again",
                    event.id
                )
            })?;
        }
        // The high-water reading: each mark names a position the peer holds,
        // and nothing is inferred about the positions below it.
        for (origin, position) in ack.confirmed.iter() {
            let id = EventId::new(origin, position);
            ensure(held.iter().any(|event| event.id == id), CLAUSE, || {
                format!(
                    "the peer confirmed {id} as the highest position it holds from {origin}, \
                     and a fresh handle onto it holds no event with that identity"
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// SY-18: a peer's declared batch limits are the limits it imposes: a push
    /// at the declared corner, which the declaration admits, lands whole.
    ///
    /// [`peer_declares_its_own_limits`] holds the declaration to one event at a
    /// time. This rule holds its other half, [`PeerLimits::max_batch_events`]
    /// and [`PeerLimits::max_batch_bytes`], to a push as large as they say. A
    /// runner that honours them and is refused anyway has been told a limit
    /// that is not the one imposed, and learns the real one at ingest. The
    /// rule plays that runner. It reads [`SyncPeer::limits`] once, and then:
    ///
    /// 1. takes the corner's event count, `max_batch_events` capped by
    ///    [`MAX_PROVOKED_BATCH_EVENTS`], and its bytes, `max_batch_bytes`
    ///    capped by [`MAX_PROVOKED_BATCH_BYTES`] and by what that many events
    ///    can carry with none over `max_event_bytes`;
    /// 2. pushes the **count corner**: that many single-event groups with
    ///    empty payloads. The declaration must admit it, and it must land,
    ///    every event acknowledged as appended;
    /// 3. pushes the **full corner**, when it has any bytes: as many
    ///    single-event groups again, from a second origin, whose payloads
    ///    total exactly the corner's bytes and differ by at most one byte. The
    ///    same must hold;
    /// 4. pulls the whole log back: every event of both pushes is held, byte
    ///    for byte, which rejects a peer that acknowledges what it dropped.
    ///
    /// Two pushes rather than one, so a refusal names the limit that was not
    /// imposed: the count corner carries no bytes. And the events are as small
    /// as the corner allows, with no guard, tags or metadata, so a lie about a
    /// per-event limit, which is [`peer_declares_its_own_limits`]'s to judge,
    /// cannot fail this rule as well.
    ///
    /// [`PeerLimits::max_batch_events`]: happenstance_sync::PeerLimits::max_batch_events
    /// [`PeerLimits::max_batch_bytes`]: happenstance_sync::PeerLimits::max_batch_bytes
    /// [`MAX_PROVOKED_BATCH_EVENTS`]: crate::MAX_PROVOKED_BATCH_EVENTS
    /// [`MAX_PROVOKED_BATCH_BYTES`]: crate::MAX_PROVOKED_BATCH_BYTES
    ///
    /// # Rejects
    ///
    /// `tests/wrong_peers.rs` carries five, each over the reference peer:
    /// a peer that takes fewer events to a push than it declares, and one that
    /// declares twice the `max_batch_bytes` it enforces, which SY-37's rule
    /// also uses to attribute its skips; two that refuse *at* the declared
    /// limit, `>=` where the declaration means `>`, one on the event count and
    /// one on the bytes, which pin each corner to the declaration exactly; and
    /// a peer that stores only the head of a large push and acknowledges all
    /// of it, which only the pull-back sees.
    ///
    /// # Does not reject
    ///
    /// A peer that *accepts* more than it declares: a runner that reads the
    /// declaration never sends more, so nothing is lost. Nor the refusal of a
    /// push one byte over the declaration, which is SY-36's `Split` and
    /// [`every_refusal_states_its_class`]'s to judge. Nor a pull larger than
    /// the declaration: the batch limits bind a push. Nor a declaration of
    /// `max_batch_events` of zero, which [`peer_declares_its_own_limits`]
    /// fails and this rule has no corner to push into, so it pushes nothing
    /// and runs. Nor a peer whose limits are true up to the caps and false
    /// above them, because the corner is built at the caps.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-18 if the declaration does not admit one of the
    /// corners, if the peer refuses either push or acknowledges it as anything
    /// but every event appended, or if an event of either is not handed back
    /// whole; or if the peer refuses a pull or keeps handing back groups past
    /// one pull per event pushed.
    pub async fn peer_takes_a_batch_at_its_declared_limits<F: SyncFixture>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-18";
        let fixture = open().await;
        let peer = fixture.connect().await;

        let limits = peer.limits();
        let events = limits.max_batch_events.min(MAX_PROVOKED_BATCH_EVENTS);
        if events == 0 {
            // A declaration that admits no event: `peer_declares_its_own_limits`
            // fails it, and there is no corner here to push into.
            return Ok(RuleOutcome::Ran);
        }

        // The most bytes `events` events carry with none over `max_event_bytes`.
        let bytes = limits
            .max_batch_bytes
            .min(MAX_PROVOKED_BATCH_BYTES)
            .min(events.saturating_mul(limits.max_event_bytes));

        let mut landed =
            vec![push_the_corner(&peer, &limits, COUNT_CORNER_ORIGIN, events, 0, CLAUSE).await?];
        if bytes > 0 {
            landed.push(
                push_the_corner(&peer, &limits, BYTE_CORNER_ORIGIN, events, bytes, CLAUSE).await?,
            );
        }

        // One pull per event pushed, and the empty one that ends the log: a
        // peer that pages one event at a time still fits (H-13).
        let pushed = landed
            .iter()
            .map(|batch| batch.events().count())
            .sum::<usize>();
        let held: BTreeMap<EventId, ReplicatedEvent> =
            pull_up_to(&peer, pushed.saturating_add(1), CLAUSE)
                .await?
                .into_iter()
                .flat_map(|group| group.events)
                .map(|event| (event.id, event))
                .collect();
        for expected in landed.iter().flat_map(PushBatch::events) {
            ensure(held.get(&expected.id) == Some(expected), CLAUSE, || {
                format!(
                    "event {} of {} bytes, in a push at the declared batch limits, was \
                     acknowledged and is not in the peer's log, byte for byte",
                    expected.id,
                    expected.payload_len()
                )
            })?;
        }
        Ok(RuleOutcome::Ran)
    }

    /// What one log SY-37's rule wrote showed it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Log {
        /// Built, and a second non-empty page was seen: a `Nothing` at a
        /// boundary could have been a lie, and was checked.
        Paged,
        /// Built, or not written at all, and no page boundary was seen.
        NoPage,
        /// The peer refused a push of it, so it was not built.
        Refused,
    }

    /// Whether the peer took every push of a log.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Pushed {
        All,
        Refused,
    }

    /// Writes `fill` to `peer` and, if the peer took it, follows its tokens.
    async fn write_log<P: SyncPeer>(
        peer: &P,
        fill: Fill,
        clause: &'static str,
    ) -> Result<Log, Violation> {
        Ok(match fill_with_readings(peer, fill, clause).await? {
            Pushed::Refused => Log::Refused,
            Pushed::All if follow_tokens(peer, fill.groups, clause).await? => Log::Paged,
            Pushed::All => Log::NoPage,
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
        /// The cap plus one groups with tiny payloads, whatever the peer
        /// declares: a peer that pages at any count up to the cap shows a
        /// second page, including one whose declaration understates it.
        fn cut_by_count(limits: &PeerLimits) -> Self {
            Self {
                groups: FILL_CAP.saturating_add(1),
                payload: 0,
                per_push: limits.max_batch_events.max(1),
            }
        }

        /// One group more than `max_batch_bytes` admits at the largest payload
        /// the peer takes, when that is fewer events than `max_batch_events`;
        /// `None` when the count cuts first or the log would pass either cap.
        ///
        /// Pushed one event at a time. The page boundary is the pull's to
        /// draw, and a push of one event no larger than SY-18's boundary event
        /// can be refused only for a per-event limit, which SY-18's rule
        /// judges. A push of a full page would also be refused by a peer that
        /// overstates `max_batch_bytes` alone, which is SY-18's batch rule's
        /// to judge, not this log's.
        fn cut_by_bytes(limits: &PeerLimits) -> Option<Self> {
            let payload = limits.max_event_bytes.min(limits.max_batch_bytes);
            let per_page = limits.max_batch_bytes.checked_div(payload)?;
            let groups = per_page.checked_add(1)?;
            let bytes = groups.checked_mul(payload)?;
            let fits = per_page < limits.max_batch_events
                && groups <= FILL_CAP.saturating_add(1)
                && bytes <= MAX_PROVOKED_BATCH_BYTES;
            fits.then_some(Self {
                groups,
                payload,
                per_push: 1,
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
    /// `fill.per_push` to a push. Answers whether the peer took them all.
    async fn fill_with_readings<P: SyncPeer>(
        peer: &P,
        fill: Fill,
        clause: &'static str,
    ) -> Result<Pushed, Violation> {
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
                // A refusal here is the peer disagreeing with its own
                // declaration. Whose failure that is depends on the log: for
                // the byte-cut log, one event sized from `max_event_bytes` to a
                // push, it is SY-18's per-event rule's to judge; for the
                // count-cut log, tiny events at most `max_batch_events` to a
                // push, it is a lie about batch limits, SY-18's batch rule's
                // to judge. The caller reports which, as a skip, unless the
                // other log paged.
                if peer.push(&batch).await.is_err() {
                    return Ok(Pushed::Refused);
                }
            }
        }
        Ok(Pushed::All)
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
        pull_up_to(peer, MAX_PULLS, clause).await
    }

    /// [`pull_everything`], giving up after `max_pulls` pulls rather than
    /// [`MAX_PULLS`]: for a rule whose log is too long for that bound.
    async fn pull_up_to<P: SyncPeer>(
        peer: &P,
        max_pulls: usize,
        clause: &'static str,
    ) -> Result<Vec<EventGroup>, Violation> {
        let mut groups = Vec::new();
        let mut resume: Option<P::Resume> = None;
        for _ in 0..max_pulls {
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
            format!("the peer was still handing back groups after {max_pulls} pulls"),
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
        let ack = push_or_report(
            peer,
            batch,
            what,
            "the failure was relocated to ingest, not prevented",
            clause,
        )
        .await?;
        ensure(ack.appended == 1 && ack.skipped == 0, clause, || {
            format!(
                "the peer acknowledged {what} as {} appended and {} skipped; it held nothing \
                 with that identity, so it must append it",
                ack.appended, ack.skipped
            )
        })
    }

    /// Pushes `batch`, which the peer's declaration admits, and reports a
    /// refusal as `what` refused, followed by `meaning`: what the refusal says
    /// about the peer.
    async fn push_or_report<P: SyncPeer>(
        peer: &P,
        batch: &PushBatch,
        what: &str,
        meaning: &str,
        clause: &'static str,
    ) -> Result<Ack, Violation> {
        peer.push(batch).await.map_err(|error| {
            Violation::new(
                clause,
                format!(
                    "the declared limits admit {what} and the push refused it ({error}, class \
                     {:?}): {meaning}",
                    error.class()
                ),
            )
        })
    }

    /// Builds the batch at a corner of `limits` — `events` single-event
    /// groups carrying `bytes` payload bytes between them — checks that the
    /// declaration admits it, and pushes it: it must land whole, every event
    /// acknowledged as appended. Answers the batch, for the pull-back.
    async fn push_the_corner<P: SyncPeer>(
        peer: &P,
        limits: &PeerLimits,
        origin: StoreId,
        events: usize,
        bytes: usize,
        clause: &'static str,
    ) -> Result<PushBatch, Violation> {
        let batch = batch_at_the_corner(origin, events, bytes, clause)?;
        let what = format!("a push of {events} events and {bytes} payload bytes");
        ensure(limits.admits(&batch), clause, || {
            format!(
                "the declaration {limits:?} admits no {what}, the batch its own limits \
                 state it accepts"
            )
        })?;
        let ack = push_or_report(
            peer,
            &batch,
            &what,
            "a declared batch limit this peer does not impose",
            clause,
        )
        .await?;
        let expected = u64::try_from(events).map_err(|error| {
            Violation::new(
                clause,
                format!("the rule's own event count is too large: {error}"),
            )
        })?;
        ensure(ack.appended == expected && ack.skipped == 0, clause, || {
            format!(
                "the peer acknowledged {what} as {} appended and {} skipped; it held none of \
                 those identities, so it must append all {expected}",
                ack.appended, ack.skipped
            )
        })?;
        Ok(batch)
    }

    /// `events` single-event groups from `origin`, at its positions from the
    /// first, whose payloads total exactly `bytes`: each `bytes / events`
    /// long, and the first `bytes % events` one byte longer. The bytes are
    /// zeros and are never parsed (SY-35).
    ///
    /// Single-event groups with no guard, no tags and no metadata, so a peer
    /// has nothing to cut, evaluate or cap but the batch itself.
    fn batch_at_the_corner(
        origin: StoreId,
        events: usize,
        bytes: usize,
        clause: &'static str,
    ) -> Result<PushBatch, Violation> {
        let no_events = || Violation::new(clause, "the rule built a corner of no events");
        let base = bytes.checked_div(events).ok_or_else(no_events)?;
        let longer = bytes.checked_rem(events).ok_or_else(no_events)?;

        let mut position = SequencePosition::FIRST;
        let mut groups = Vec::with_capacity(events);
        for index in 0..events {
            let size = if index < longer {
                base.saturating_add(1)
            } else {
                base
            };
            let event = Event::new(BATCH_FILLED, vec![0_u8; size]).map_err(|error| {
                Violation::new(clause, format!("the rule's own event is invalid: {error}"))
            })?;
            groups.push(EventGroup::new(
                None,
                vec![ReplicatedEvent::new(
                    EventId::new(origin, position),
                    RecordedAt::from_millis(1_800_000_000_000),
                    event,
                )],
            ));
            position = position
                .next()
                .ok_or_else(|| Violation::new(clause, "the rule's corner ran out of positions"))?;
        }
        Ok(PushBatch::new(groups))
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

    /// A reading `origin` recorded at its own `position`, carrying `data` as
    /// opaque bytes (SY-35).
    fn durable_reading(
        origin: StoreId,
        position: u64,
        data: &[u8],
        clause: &'static str,
    ) -> Result<ReplicatedEvent, Violation> {
        let position = SequencePosition::new(position)
            .ok_or_else(|| Violation::new(clause, "the rule's origin position is zero"))?;
        let event = Event::new(READING_TAKEN, data.to_vec()).map_err(|error| {
            Violation::new(clause, format!("the rule's own event is invalid: {error}"))
        })?;
        Ok(ReplicatedEvent::new(
            EventId::new(origin, position),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        ))
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

    #[cfg(test)]
    mod tests {
        use alloc::collections::BTreeSet;

        use happenstance_sync::ReplicatedEvent;

        use super::{BYTE_CORNER_ORIGIN, batch_at_the_corner};

        /// The corner's arithmetic at its edges: no bytes, an exact multiple,
        /// a remainder of one, fewer bytes than events, and a large corner.
        #[test]
        fn a_corner_carries_exactly_its_events_and_bytes_spread_evenly() {
            let cases: [(usize, usize); 6] = [
                (1, 0),
                (128, 0),
                (128, 65_536),
                (128, 65_537),
                (3, 2),
                (10, 1_048_576),
            ];
            for (events, bytes) in cases {
                let batch = batch_at_the_corner(BYTE_CORNER_ORIGIN, events, bytes, "SY-18")
                    .expect("a corner of at least one event builds");
                let sizes: Vec<usize> = batch.events().map(ReplicatedEvent::payload_len).collect();
                let ids: BTreeSet<_> = batch.events().map(|event| event.id).collect();
                let case = format!("{events} events, {bytes} bytes");

                assert_eq!(sizes.len(), events, "{case}: the event count");
                assert_eq!(batch.payload_len(), bytes, "{case}: the payload total");
                let spread = sizes.iter().max().zip(sizes.iter().min());
                assert!(
                    spread.is_some_and(|(max, min)| max - min <= 1),
                    "{case}: sizes {sizes:?} differ by more than one byte"
                );
                assert_eq!(ids.len(), events, "{case}: every identity is distinct");
                assert!(
                    batch.groups.iter().all(|group| group.events.len() == 1),
                    "{case}: every group carries one event"
                );
            }
        }

        /// The remainder goes to the first events, one byte each.
        #[test]
        fn a_corner_with_a_remainder_lengthens_its_first_events() {
            let batch = batch_at_the_corner(BYTE_CORNER_ORIGIN, 128, 65_537, "SY-18")
                .expect("a corner of 128 events builds");
            let sizes: Vec<usize> = batch.events().map(ReplicatedEvent::payload_len).collect();
            assert_eq!(sizes.first(), Some(&513));
            assert!(sizes.iter().skip(1).all(|size| *size == 512), "{sizes:?}");
        }

        /// A corner of no events is the rule's own mistake, reported as a
        /// violation rather than a division by zero.
        #[test]
        fn a_corner_of_no_events_is_a_violation_not_a_panic() {
            for bytes in [0, 1, 65_536] {
                let built = batch_at_the_corner(BYTE_CORNER_ORIGIN, 0, bytes, "SY-18");
                assert_eq!(
                    built
                        .map(|batch| batch.payload_len())
                        .map_err(|v| v.clause()),
                    Err("SY-18"),
                    "0 events, {bytes} bytes"
                );
            }
        }
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
            peer_takes_a_batch_at_its_declared_limits,

            // --- Every refusal states its class (SY-36) ---------------------
            every_refusal_states_its_class,

            // --- Nothing waiting means nothing waiting (SY-37) --------------
            nothing_waiting_means_the_next_pull_is_empty,

            // --- An ack confirms only what is durable (SY-31) ---------------
            ack_never_confirms_a_non_durable_event,
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
