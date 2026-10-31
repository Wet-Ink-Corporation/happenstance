//! Every sync rule, failing the wrong implementation it names — and only that
//! rule.
//!
//! A rule that no peer can fail is decorative. Each wrong peer here is the
//! reference peer with one plausible mistake added, and each test runs the
//! **whole** enumeration against it: the rule written against that mistake must
//! report a violation of its own clause, and every other rule must pass. That
//! second half is what makes "fails exactly its rule" a test rather than a
//! sentence, and it is what keeps a mutant from passing for the wrong reason —
//! a peer that broke two rules would prove neither check is needed.
//!
//! The faithful wrapper is the control: the same plumbing with no mistake
//! passes every rule, so a red result below is the mistake's and not the
//! wrapper's.

#![cfg(not(target_arch = "wasm32"))]

use std::cell::Cell;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};

use happenstance_core::{EventId, SequencePosition, StoreId};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, MemoryPeerError, MemoryResume, MemorySyncPeer,
    PeerLimits, Pulled, PushBatch, ReplicatedEvent, SendSyncPeer, Waiting, Watermark,
};
use happenstance_sync_testkit::{
    BYTE_LOG_REFUSED_REASON, COUNT_LOG_REFUSED_REASON, LOG_REFUSED, NO_CONFIRMATION,
    NO_CONFIRMATION_REASON, NO_SECOND_PAGE, NO_SECOND_PAGE_REASON, RefusalsOf, SyncFixture,
    oversized_push_refusal, rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a [`WrongPeer`] makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// SY-1's first wrong implementation: re-evaluate the guard that travelled
    /// with a group against what this peer already holds, and refuse the group
    /// — acknowledging it as nothing appended — when it is violated.
    EvaluatesTheGuard,
    /// SY-1's second: the same evaluation, made *after* acknowledging. The ack
    /// says `appended: 1` and the group is dropped. It is the shape a peer
    /// takes when it acknowledges on receipt and adjudicates later, and only
    /// the rule's pull of the whole log can see it — so it is the mutant that
    /// isolates that check.
    AcksThenDropsOnTheGuard,
    /// SY-2's first wrong implementation: store every event as its own group,
    /// so a losing event and its compensation come back apart.
    FlattensGroups,
    /// SY-36's wrong implementation: state one class for every failure, so a
    /// batch that can only ever be split is retried as if it were transient.
    OneClassForEverything,
    /// SY-36, the fixture's half: declare no refusal at all, so the rule would
    /// compare classes over an empty list and pass having checked nothing.
    DeclaresNoRefusal,
    /// SY-36, a refusal that never happens: declare a smaller batch limit than
    /// the peer enforces, so the push built one byte over the declared limit
    /// is accepted. The fixture's declaration names a refusal the peer does
    /// not make.
    AcceptsTheOversizedPush,
    /// SY-36, a class that is not a function of the error: the first read
    /// answers the reference peer's class and every later read `Transient`,
    /// so a runner that reads it twice handles one failure two ways.
    ClassChangesBetweenReads,
    /// SY-37's wrong implementation: answer every pull with "nothing is
    /// waiting", whether the page ran out or the log did. A runner that
    /// believes it stops one page in.
    AlwaysSaysNothingWaiting,
    /// SY-37's second: say "nothing is waiting" whenever a page carried fewer
    /// events than `max_batch_events`, as an adapter does that reads a short
    /// page as the end of the log. Honest when the event count cuts the page,
    /// and a lie when `max_batch_bytes` cuts it first.
    SaysNothingOnAShortPage,
    /// A peer that declares more events to a push than it takes: it refuses
    /// any push of more than [`TAKES_EVENTS`], whatever its declaration says.
    /// SY-37's rule must report the refused count-cut log as refused, a lie
    /// about the batch limits, which SY-18's batch rule judges.
    TakesFewerEventsThanItDeclares,
    /// A peer that declares larger events than it takes: it refuses any event
    /// over half its declared `max_event_bytes`. SY-37's rule must report the
    /// refused byte-cut log as refused, and as SY-18's subject.
    TakesSmallerEventsThanItDeclares,
    /// Both of the two above at once, so SY-37's rule sees both logs refused
    /// and must name the count-cut one, the batch-limit lie.
    TakesLessOfBothThanItDeclares,
    /// A peer that declares twice the `max_batch_bytes` it enforces.
    /// `peer_declares_its_own_limits` judges per-event limits only, so SY-37's
    /// rule must not blame it: its byte-cut log goes one event to a push,
    /// takes it, and pages at the limit the peer really imposes. SY-18's
    /// batch rule is the one that fails it.
    DeclaresMoreBatchBytesThanItTakes,
    /// SY-31's wrong implementation: a write-behind cache. It acknowledges a
    /// push on admission, with the ack the reference peer would have sent,
    /// and commits the batch only on this handle's next push or pull. A
    /// handle dropped in between loses what it confirmed, which is a crash
    /// between the ack and the commit.
    AcksOnAdmission,
    /// Not a mistake: a peer that stores everything and confirms nothing.
    /// Safe, because a spoke re-pushes, and SY-31's rule cannot hold an empty
    /// confirmation to anything, so it must say so rather than pass.
    ConfirmsNothing,
    /// SY-18's batch rule, an off-by-one on the count: refuse a push of
    /// `max_batch_events` or more, as an adapter does that compares with `>=`.
    /// The declaration is honest by one event and the corner is refused. Run
    /// under [`both_logs_written`], whose 1,000-event declaration is far above
    /// the 257 events SY-37's count-cut log puts in a push; under the minimum
    /// declaration that log's 128-event pushes would be refused too.
    RefusesAtTheDeclaredBatchEvents,
    /// SY-18's batch rule, an off-by-one on the bytes: refuse a push carrying
    /// `max_batch_bytes` or more. Run under [`both_logs_written`]'s 128 KiB,
    /// because the limits are load-bearing: under the minimum declaration,
    /// `peer_declares_its_own_limits`'s 64 KiB boundary event equals
    /// `max_batch_bytes`, and that rule would fail the peer too.
    RefusesAtTheDeclaredBatchBytes,
    /// SY-18's batch rule, a peer that acknowledges more than it keeps: a push
    /// of more than [`HEAD_EVENTS`] events has only its first `HEAD_EVENTS`
    /// groups stored, and is acknowledged as every event appended. Only the
    /// rule's pull-back can see it, so it is the mutant that isolates that
    /// check.
    KeepsOnlyTheHeadOfALargePush,
}

/// How many events a push to [`Mistake::TakesFewerEventsThanItDeclares`] may
/// carry: the reference peer's minimum declaration, and fewer than the 257 the
/// count-cut log puts in one push when the declaration admits them.
const TAKES_EVENTS: usize = 128;

/// How many groups of a large push [`Mistake::KeepsOnlyTheHeadOfALargePush`]
/// stores: 257, the whole of SY-37's count-cut log (its `FILL_CAP` plus one),
/// so that log, pushed in one push under a large declaration, is stored whole
/// and SY-37's rule sees the reference peer. If `FILL_CAP` moves, this moves
/// with it, or the mutant fails two rules.
const HEAD_EVENTS: usize = 257;

/// What a [`WrongError`] is about.
#[derive(Debug)]
enum Cause {
    /// The reference peer's own refusal.
    Peer(MemoryPeerError),
    /// A push carrying more events than the wrong peer takes. Its own case
    /// rather than a [`MemoryPeerError::BatchTooLarge`], whose message says
    /// bytes.
    TooManyEvents { actual: usize, limit: usize },
}

impl Cause {
    /// The reference peer's class, and `Split` for a push with too many
    /// events, which is the class the reference peer gives an oversized push.
    fn class(&self) -> FailureClass {
        match self {
            Self::Peer(inner) => inner.class(),
            Self::TooManyEvents { .. } => FailureClass::Split,
        }
    }
}

/// The reference peer's error, classified through the wrong peer's mistake.
#[derive(Debug)]
struct WrongError {
    cause: Cause,
    mistake: Mistake,
    /// Whether [`Classify::class`] has answered once already. A `Cell`, not an
    /// atomic: the error is never shared across threads, and only
    /// [`Mistake::ClassChangesBetweenReads`] reads it.
    read: Cell<bool>,
}

impl core::fmt::Display for WrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.cause {
            Cause::Peer(inner) => inner.fmt(f),
            Cause::TooManyEvents { actual, limit } => write!(
                f,
                "push of {actual} events exceeds the {limit} this peer takes in one push"
            ),
        }
    }
}

impl core::error::Error for WrongError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self.cause {
            Cause::Peer(inner) => Some(inner),
            Cause::TooManyEvents { .. } => None,
        }
    }
}

impl Classify for WrongError {
    fn class(&self) -> FailureClass {
        match self.mistake {
            Mistake::OneClassForEverything => FailureClass::Transient,
            Mistake::ClassChangesBetweenReads => {
                if self.read.replace(true) {
                    FailureClass::Transient
                } else {
                    self.cause.class()
                }
            }
            Mistake::Faithful
            | Mistake::EvaluatesTheGuard
            | Mistake::AcksThenDropsOnTheGuard
            | Mistake::FlattensGroups
            | Mistake::DeclaresNoRefusal
            | Mistake::AcceptsTheOversizedPush
            | Mistake::AlwaysSaysNothingWaiting
            | Mistake::SaysNothingOnAShortPage
            | Mistake::TakesFewerEventsThanItDeclares
            | Mistake::TakesSmallerEventsThanItDeclares
            | Mistake::TakesLessOfBothThanItDeclares
            | Mistake::DeclaresMoreBatchBytesThanItTakes
            | Mistake::AcksOnAdmission
            | Mistake::ConfirmsNothing
            | Mistake::RefusesAtTheDeclaredBatchEvents
            | Mistake::RefusesAtTheDeclaredBatchBytes
            | Mistake::KeepsOnlyTheHeadOfALargePush => self.cause.class(),
        }
    }
}

#[derive(Debug)]
struct WrongPeer {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
    /// What [`Mistake::AcksOnAdmission`] has acknowledged and not yet
    /// committed. One per handle, so dropping the handle loses it. A
    /// `std::sync::Mutex` because the peer's futures must be `Send`; no guard
    /// is held across an `.await`.
    admitted: Mutex<Vec<PushBatch>>,
}

/// Everything `peer` holds, from the start of its log.
async fn held_by(peer: &MemorySyncPeer) -> Result<Vec<EventGroup>, MemoryPeerError> {
    let mut groups = Vec::new();
    let mut resume = None;
    loop {
        let pulled = peer.pull(resume.as_ref()).await?;
        if pulled.batch.groups.is_empty() {
            return Ok(groups);
        }
        groups.extend(pulled.batch.groups);
        resume = Some(pulled.resume);
    }
}

impl WrongPeer {
    fn wrap(&self, inner: MemoryPeerError) -> WrongError {
        self.refuse(Cause::Peer(inner))
    }

    fn refuse(&self, cause: Cause) -> WrongError {
        WrongError {
            cause,
            mistake: self.mistake,
            read: Cell::new(false),
        }
    }

    /// Why a peer that takes less than it declares refuses `batch`, if it
    /// does: more than [`TAKES_EVENTS`] events, or an event over half the
    /// declared `max_event_bytes`, as its mistake says.
    fn takes_less(&self, batch: &PushBatch) -> Option<Cause> {
        let counts = matches!(
            self.mistake,
            Mistake::TakesFewerEventsThanItDeclares | Mistake::TakesLessOfBothThanItDeclares
        );
        let sizes = matches!(
            self.mistake,
            Mistake::TakesSmallerEventsThanItDeclares | Mistake::TakesLessOfBothThanItDeclares
        );
        let events = batch.events().count();
        if counts && events > TAKES_EVENTS {
            return Some(Cause::TooManyEvents {
                actual: events,
                limit: TAKES_EVENTS,
            });
        }
        let limit = self.inner.limits().max_event_bytes / 2;
        let largest = batch.events().map(ReplicatedEvent::payload_len).max()?;
        (sizes && largest > limit).then_some(Cause::Peer(MemoryPeerError::BatchTooLarge {
            actual: largest,
            limit,
        }))
    }

    /// Why a peer that refuses at its own declared batch limit refuses
    /// `batch`, if it does: `>=` where the declaration means `>`.
    fn refuses_at_the_limit(&self, batch: &PushBatch) -> Option<Cause> {
        let limits = self.inner.limits();
        let events = batch.events().count();
        let bytes = batch.payload_len();
        match self.mistake {
            Mistake::RefusesAtTheDeclaredBatchEvents if events >= limits.max_batch_events => {
                Some(Cause::TooManyEvents {
                    actual: events,
                    limit: limits.max_batch_events,
                })
            }
            Mistake::RefusesAtTheDeclaredBatchBytes if bytes >= limits.max_batch_bytes => {
                Some(Cause::Peer(MemoryPeerError::BatchTooLarge {
                    actual: bytes,
                    limit: limits.max_batch_bytes,
                }))
            }
            _ => None,
        }
    }

    /// [`Mistake::KeepsOnlyTheHeadOfALargePush`]'s push: store the first
    /// [`HEAD_EVENTS`] groups, and acknowledge every event as appended.
    async fn keep_the_head(&self, batch: &PushBatch) -> Result<Ack, MemoryPeerError> {
        if batch.groups.len() <= HEAD_EVENTS {
            return self.inner.push(batch).await;
        }
        let head = batch.groups.iter().take(HEAD_EVENTS).cloned().collect();
        let mut ack = self.inner.push(&PushBatch::new(head)).await?;
        ack.appended = u64::try_from(batch.events().count()).unwrap_or(u64::MAX);
        ack.skipped = 0;
        Ok(ack)
    }

    /// Whether `group`'s travelling guard is violated by what the inner peer
    /// already holds.
    async fn guard_is_violated(&self, group: &EventGroup) -> Result<bool, MemoryPeerError> {
        let Some(guard) = &group.guard else {
            return Ok(false);
        };
        let held = held_by(&self.inner).await?;
        Ok(held.iter().flat_map(|held| &held.events).any(|event| {
            guard.is_violated_by(
                SequencePosition::FIRST,
                event.event.event_type(),
                event.event.tags(),
            )
        }))
    }

    /// Commits everything [`Mistake::AcksOnAdmission`] has acknowledged on
    /// this handle.
    async fn flush(&self) -> Result<(), MemoryPeerError> {
        let pending = core::mem::take(
            &mut *self
                .admitted
                .lock()
                .map_err(|_| MemoryPeerError::Poisoned)?,
        );
        for batch in &pending {
            self.inner.push(batch).await?;
        }
        Ok(())
    }

    /// [`Mistake::AcksOnAdmission`]'s push: refuse what the reference peer
    /// refuses, predict the ack it would send, and buffer the batch.
    async fn admit(&self, batch: &PushBatch) -> Result<Ack, MemoryPeerError> {
        let limit = self.inner.limits().max_batch_bytes;
        let offered = batch.payload_len();
        if offered > limit {
            return Err(MemoryPeerError::BatchTooLarge {
                actual: offered,
                limit,
            });
        }
        self.flush().await?;
        let mut held: BTreeSet<EventId> = held_by(&self.inner)
            .await?
            .iter()
            .flat_map(|group| &group.events)
            .map(|event| event.id)
            .collect();
        // The reference peer's confirmation before this batch, advanced by
        // each event it would append: the ack a faithful peer sends.
        let mut ack = self.inner.push(&PushBatch::empty()).await?;
        for event in batch.events() {
            if held.insert(event.id) {
                ack.appended += 1;
                ack.confirmed.advance(event.id.store(), event.id.position());
            } else {
                ack.skipped += 1;
            }
        }
        self.admitted
            .lock()
            .map_err(|_| MemoryPeerError::Poisoned)?
            .push(batch.clone());
        Ok(ack)
    }

    /// Pushes one group at a time, letting `refused` decide what a violated
    /// guard is acknowledged as.
    async fn push_guarded(
        &self,
        batch: &PushBatch,
        refused: fn(&EventGroup) -> u64,
    ) -> Result<Ack, MemoryPeerError> {
        let mut ack = Ack::default();
        for group in &batch.groups {
            if self.guard_is_violated(group).await? {
                ack.appended += refused(group);
                continue;
            }
            let one = self
                .inner
                .push(&PushBatch::new(vec![group.clone()]))
                .await?;
            ack.appended += one.appended;
            ack.skipped += one.skipped;
            ack.confirmed = one.confirmed;
        }
        Ok(ack)
    }
}

impl SendSyncPeer for WrongPeer {
    type Error = WrongError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        // A write-behind cache serves its own handle's reads, so on one handle
        // it is indistinguishable from the faithful peer.
        if self.mistake == Mistake::AcksOnAdmission {
            self.flush().await.map_err(|error| self.wrap(error))?;
        }
        let pulled = self
            .inner
            .pull(from)
            .await
            .map_err(|error| self.wrap(error))?;
        let short_page = pulled.batch.events().count() < self.inner.limits().max_batch_events;
        Ok(match self.mistake {
            Mistake::AlwaysSaysNothingWaiting => pulled.with_waiting(Waiting::Nothing),
            Mistake::SaysNothingOnAShortPage if short_page => pulled.with_waiting(Waiting::Nothing),
            _ => pulled,
        })
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        match self.mistake {
            Mistake::Faithful
            | Mistake::OneClassForEverything
            | Mistake::DeclaresNoRefusal
            | Mistake::AcceptsTheOversizedPush
            | Mistake::ClassChangesBetweenReads
            | Mistake::AlwaysSaysNothingWaiting
            | Mistake::SaysNothingOnAShortPage
            | Mistake::DeclaresMoreBatchBytesThanItTakes => self
                .inner
                .push(batch)
                .await
                .map_err(|error| self.wrap(error)),
            Mistake::FlattensGroups => {
                let singletons = batch
                    .events()
                    .map(|event| EventGroup::new(None, vec![event.clone()]))
                    .collect();
                self.inner
                    .push(&PushBatch::new(singletons))
                    .await
                    .map_err(|error| self.wrap(error))
            }
            Mistake::EvaluatesTheGuard => self
                .push_guarded(batch, |_| 0)
                .await
                .map_err(|error| self.wrap(error)),
            Mistake::AcksThenDropsOnTheGuard => self
                .push_guarded(batch, |group| {
                    u64::try_from(group.events.len()).unwrap_or(u64::MAX)
                })
                .await
                .map_err(|error| self.wrap(error)),
            Mistake::AcksOnAdmission => self.admit(batch).await.map_err(|error| self.wrap(error)),
            Mistake::ConfirmsNothing => {
                let mut ack = self
                    .inner
                    .push(batch)
                    .await
                    .map_err(|error| self.wrap(error))?;
                ack.confirmed = Watermark::new();
                Ok(ack)
            }
            Mistake::RefusesAtTheDeclaredBatchEvents | Mistake::RefusesAtTheDeclaredBatchBytes => {
                if let Some(cause) = self.refuses_at_the_limit(batch) {
                    return Err(self.refuse(cause));
                }
                self.inner
                    .push(batch)
                    .await
                    .map_err(|error| self.wrap(error))
            }
            Mistake::KeepsOnlyTheHeadOfALargePush => self
                .keep_the_head(batch)
                .await
                .map_err(|error| self.wrap(error)),
            Mistake::TakesFewerEventsThanItDeclares
            | Mistake::TakesSmallerEventsThanItDeclares
            | Mistake::TakesLessOfBothThanItDeclares => {
                if let Some(cause) = self.takes_less(batch) {
                    return Err(self.refuse(cause));
                }
                self.inner
                    .push(batch)
                    .await
                    .map_err(|error| self.wrap(error))
            }
        }
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = self.inner.limits();
        match self.mistake {
            // Declares half of what the inner peer enforces, so a batch one
            // byte over the declaration still fits under the real limit.
            Mistake::AcceptsTheOversizedPush => limits.max_batch_bytes /= 2,
            // Declares twice what the inner peer enforces, so a push of two
            // events at the per-event limit is refused although declared.
            Mistake::DeclaresMoreBatchBytesThanItTakes => limits.max_batch_bytes *= 2,
            _ => {}
        }
        limits
    }
}

#[derive(Debug)]
struct WrongFixture {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
}

impl WrongFixture {
    fn new(mistake: Mistake) -> Self {
        Self::with_limits(mistake, PeerLimits::minimum())
    }

    /// The reference peer behind the wrapper, declaring `limits`.
    fn with_limits(mistake: Mistake, limits: PeerLimits) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::with_limits(
                StoreId::from_bytes([0x77; 16]),
                limits,
            )),
            mistake,
        }
    }
}

impl SyncFixture for WrongFixture {
    type Peer = WrongPeer;

    async fn connect(&self) -> Self::Peer {
        WrongPeer {
            inner: Arc::clone(&self.inner),
            mistake: self.mistake,
            admitted: Mutex::new(Vec::new()),
        }
    }

    /// The reference fixture's declaration, unchanged, except for the one
    /// mistake that lives in the fixture rather than the peer.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        if self.mistake == Mistake::DeclaresNoRefusal {
            return Ok(Vec::new());
        }
        oversized_push_refusal(peer).await
    }
}

/// How many events one [`PagingPeer`] pull carries.
///
/// Two, because that is the smallest page that ends SY-2's pull *between* the
/// losing claim and its reversal: the rule's log is one winning claim, then the
/// pair, so the first page is the winner and the loser.
const PAGE_EVENTS: usize = 2;

/// Where a [`PagingPeer`] pull left off: a group, and an event inside it.
///
/// The event offset is the mistake made visible. The reference peer's token is a
/// group index alone, because it never ends a pull inside a group.
#[derive(Debug, Clone, Copy, Default)]
struct PageCursor {
    group: usize,
    event: usize,
}

/// SY-2's second wrong implementation: page by event count, cutting a group at
/// the page boundary and resuming inside it.
///
/// Every event comes back, in order, and each push is stored whole — the log is
/// right and the transport is wrong. It is the natural cut for an adapter whose
/// backing store pages by row, and it is a different type from [`WrongPeer`]
/// because its resume token has to carry the offset the reference peer's has no
/// room for.
#[derive(Debug)]
struct PagingPeer {
    inner: Arc<MemorySyncPeer>,
}

impl SendSyncPeer for PagingPeer {
    type Error = MemoryPeerError;
    type Resume = PageCursor;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        let held = held_by(&self.inner).await?;
        let mut at = from.copied().unwrap_or_default();
        let mut room = PAGE_EVENTS;
        let mut page = Vec::new();

        while room > 0 {
            let Some(group) = held.get(at.group) else {
                break;
            };
            let rest = group.events.get(at.event..).unwrap_or_default();
            let taken: Vec<_> = rest.iter().take(room).cloned().collect();
            room = room.saturating_sub(taken.len());
            at.event += taken.len();
            if at.event >= group.events.len() {
                at = PageCursor {
                    group: at.group + 1,
                    event: 0,
                };
            }
            page.push(EventGroup::new(group.guard.clone(), taken));
        }

        Ok(Pulled::new(PushBatch::new(page), at))
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.inner.push(batch).await
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

#[derive(Debug)]
struct PagingFixture {
    inner: Arc<MemorySyncPeer>,
}

impl SyncFixture for PagingFixture {
    type Peer = PagingPeer;

    async fn connect(&self) -> Self::Peer {
        PagingPeer {
            inner: Arc::clone(&self.inner),
        }
    }

    /// The reference fixture's declaration: the mistake is in how this peer
    /// pages, and its push and its errors are the reference peer's own.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

/// Every rule in the enumeration against one fixture, by name, each verdict
/// reduced to the clause it reports.
///
/// Driven by `for_each_sync_peer_rule!` rather than a list written here, so a
/// rule added to the suite is run against every mutant below without an edit —
/// and a mutant that also fails the new rule is reported, rather than silently
/// counted as a single-rule proof.
fn verdicts<F, O>(open: O) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)>
where
    F: SyncFixture,
    O: AsyncFn() -> F + Copy,
{
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$(
                (
                    stringify!($name),
                    block_on(rules::$name(open)).map_err(|violation| violation.clause()),
                )
            ),*]
        };
    }
    happenstance_sync_testkit::for_each_sync_peer_rule!(run_each)
}

/// `rule` reports `clause`, and every other rule passes.
fn fails_exactly(
    verdicts: &[(&'static str, Result<RuleOutcome, &'static str>)],
    rule: &str,
    clause: &'static str,
) {
    assert!(
        verdicts.iter().any(|(name, _)| *name == rule),
        "`{rule}` is not in the enumeration: {verdicts:?}"
    );
    for (name, verdict) in verdicts {
        if *name == rule {
            assert_eq!(*verdict, Err(clause), "`{name}` must reject its mutant");
        } else {
            assert_eq!(
                *verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` failed `{rule}`'s mutant, so that mutant proves nothing about \
                 `{rule}` alone"
            );
        }
    }
}

#[test]
fn the_faithful_wrapper_passes_every_rule() {
    let all = verdicts(async || WrongFixture::new(Mistake::Faithful));
    assert!(all.len() >= 3, "the enumeration ran no rules: {all:?}");
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

#[test]
fn ingest_never_rejects_fails_a_peer_that_evaluates_the_guard() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::EvaluatesTheGuard)),
        "ingest_never_rejects",
        "SY-1",
    );
}

/// The mutant that only the log-presence half of SY-1 can see: its ack is the
/// one a correct peer sends.
#[test]
fn ingest_never_rejects_fails_a_peer_that_acks_and_then_drops() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::AcksThenDropsOnTheGuard)),
        "ingest_never_rejects",
        "SY-1",
    );
}

#[test]
fn compensation_is_atomic_fails_a_peer_that_flattens_groups() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::FlattensGroups)),
        "compensation_is_atomic_with_the_losing_event",
        "SY-2",
    );
}

#[test]
fn compensation_is_atomic_fails_a_peer_that_ends_a_pull_inside_a_group() {
    fails_exactly(
        &verdicts(async || PagingFixture {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x78; 16]))),
        }),
        "compensation_is_atomic_with_the_losing_event",
        "SY-2",
    );
}

/// The mutant only a fresh handle can see: on the handle that pushed, its
/// buffer is flushed before every read.
#[test]
fn ack_never_confirms_a_non_durable_event_fails_a_peer_that_acks_on_admission() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::AcksOnAdmission)),
        "ack_never_confirms_a_non_durable_event",
        "SY-31",
    );
}

/// A peer that confirms nothing has given the rule no confirmation to hold to
/// the log, so the rule cannot have failed it. It says so, and every other rule
/// runs.
#[test]
fn ack_never_confirms_a_non_durable_event_skips_a_peer_that_confirms_nothing() {
    for (name, verdict) in verdicts(async || WrongFixture::new(Mistake::ConfirmsNothing)) {
        let expected = if name == "ack_never_confirms_a_non_durable_event" {
            RuleOutcome::Skipped {
                capability: NO_CONFIRMATION,
                reason: NO_CONFIRMATION_REASON,
            }
        } else {
            RuleOutcome::Ran
        };
        assert_eq!(verdict, Ok(expected), "`{name}`");
    }
}

#[test]
fn every_refusal_states_its_class_fails_a_peer_with_one_class() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::OneClassForEverything)),
        "every_refusal_states_its_class",
        "SY-36",
    );
}

#[test]
fn every_refusal_states_its_class_fails_a_fixture_that_declares_no_refusal() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::DeclaresNoRefusal)),
        "every_refusal_states_its_class",
        "SY-36",
    );
}

#[test]
fn every_refusal_states_its_class_fails_a_peer_that_accepts_the_oversized_push() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::AcceptsTheOversizedPush)),
        "every_refusal_states_its_class",
        "SY-36",
    );
}

#[test]
fn every_refusal_states_its_class_fails_an_error_whose_class_changes_between_reads() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::ClassChangesBetweenReads)),
        "every_refusal_states_its_class",
        "SY-36",
    );
}

#[test]
fn nothing_waiting_means_the_next_pull_is_empty_fails_a_peer_that_always_says_nothing() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::AlwaysSaysNothingWaiting)),
        "nothing_waiting_means_the_next_pull_is_empty",
        "SY-37",
    );
}

/// The mutant only a page cut by `max_batch_bytes` can see: when the event
/// count cuts the page, its short page really is the last one.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_fails_a_peer_that_reads_a_short_page_as_the_end() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::SaysNothingOnAShortPage)),
        "nothing_waiting_means_the_next_pull_is_empty",
        "SY-37",
    );
}

/// A faithful peer whose declared limits let one pull carry everything the
/// rule is willing to write has shown the rule no second page, so the rule
/// cannot have failed it. It says so, rather than reporting a pass.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_skips_a_peer_that_never_pages() {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;
    limits.max_batch_bytes = 64 * 1024 * 1024;

    let outcome = block_on(rules::nothing_waiting_means_the_next_pull_is_empty(
        async || WrongFixture::with_limits(Mistake::Faithful, limits.clone()),
    ));

    assert_eq!(
        outcome.map_err(|violation| violation.clause()),
        Ok(RuleOutcome::Skipped {
            capability: NO_SECOND_PAGE,
            reason: NO_SECOND_PAGE_REASON,
        })
    );
}

/// SY-18's verdict alone against `mistake` behind a peer declaring `limits`:
/// whether the attribution SY-37's skip makes is true.
fn sy18_against(mistake: Mistake, limits: &PeerLimits) -> Result<RuleOutcome, &'static str> {
    block_on(rules::peer_declares_its_own_limits(async || {
        WrongFixture::with_limits(mistake, limits.clone())
    }))
    .map_err(|violation| violation.clause())
}

/// SY-18's batch rule's verdict alone against `mistake` behind a peer
/// declaring `limits`: whether the attribution SY-37's count-cut skip makes is
/// true.
fn sy18_batch_against(mistake: Mistake, limits: &PeerLimits) -> Result<RuleOutcome, &'static str> {
    block_on(rules::peer_takes_a_batch_at_its_declared_limits(
        async || WrongFixture::with_limits(mistake, limits.clone()),
    ))
    .map_err(|violation| violation.clause())
}

/// A faithful peer whose `max_batch_bytes` is more than its event count can
/// carry: 128 events of at most 64 KiB is 8 MiB, under the 16 MiB declared.
/// The full corner is sized by what the events can carry, not by the batch
/// bytes, or no event of it would fit `max_event_bytes` and the declaration
/// would admit no corner at all.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_runs_when_the_event_bytes_bind_the_corner() {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_bytes = 16 * 1024 * 1024;

    assert_eq!(
        sy18_batch_against(Mistake::Faithful, &limits),
        Ok(RuleOutcome::Ran)
    );
}

/// SY-37's verdict alone against `mistake` behind a peer declaring `limits`.
fn sy37_against(mistake: Mistake, limits: &PeerLimits) -> Result<RuleOutcome, &'static str> {
    block_on(rules::nothing_waiting_means_the_next_pull_is_empty(
        async || WrongFixture::with_limits(mistake, limits.clone()),
    ))
    .map_err(|violation| violation.clause())
}

/// A peer that refuses the count-cut log has lied about its batch limits,
/// which SY-18's batch rule judges. The skip says so, and not that its limits
/// are too large to page: the fix is the opposite one.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_reports_a_refused_count_cut_log() {
    // 1,000 events to a push and 64 MiB: the count-cut log goes in one push of
    // 257, and the byte-cut log would need 1,025 groups, so it is not written.
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;
    limits.max_batch_bytes = 64 * 1024 * 1024;

    assert_eq!(
        sy37_against(Mistake::Faithful, &limits),
        Ok(RuleOutcome::Skipped {
            capability: NO_SECOND_PAGE,
            reason: NO_SECOND_PAGE_REASON,
        }),
        "the control: the same limits, honoured, are a peer that never pages"
    );
    assert_eq!(
        sy37_against(Mistake::TakesFewerEventsThanItDeclares, &limits),
        Ok(RuleOutcome::Skipped {
            capability: LOG_REFUSED,
            reason: COUNT_LOG_REFUSED_REASON,
        })
    );
    assert_eq!(
        sy18_batch_against(Mistake::TakesFewerEventsThanItDeclares, &limits),
        Err("SY-18"),
        "the skip says SY-18's batch rule judges this peer, so it must"
    );
}

/// A peer that refuses the byte-cut log, where the count-cut log showed no
/// page, is reported as refusing that log: SY-18's subject, not a limit too
/// large to page.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_reports_a_refused_byte_cut_log() {
    // 1,000 events and 128 KiB: the count-cut log's 257 tiny events fit one
    // pull, and the byte-cut log is three 64 KiB events, two to a page.
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;
    limits.max_batch_bytes = 128 * 1024;

    assert_eq!(
        sy37_against(Mistake::Faithful, &limits),
        Ok(RuleOutcome::Ran),
        "the control: the same limits, honoured, page on the byte-cut log"
    );
    assert_eq!(
        sy37_against(Mistake::TakesSmallerEventsThanItDeclares, &limits),
        Ok(RuleOutcome::Skipped {
            capability: LOG_REFUSED,
            reason: BYTE_LOG_REFUSED_REASON,
        })
    );
    assert_eq!(
        sy18_against(Mistake::TakesSmallerEventsThanItDeclares, &limits),
        Err("SY-18"),
        "the skip says SY-18's rule judges this peer, so it must"
    );
}

/// 1,000 events and 128 KiB: the count-cut log's one push of 257 events is
/// over [`TAKES_EVENTS`], and the byte-cut log is three 64 KiB events, two to
/// a page.
fn both_logs_written() -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;
    limits.max_batch_bytes = 128 * 1024;
    limits
}

/// Both logs refused: the count-cut refusal is the one named, because
/// `peer_declares_its_own_limits` already reports the byte-cut one, and the
/// count-cut one is the batch-limit lie.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_names_the_count_cut_log_when_both_are_refused() {
    assert_eq!(
        sy37_against(Mistake::TakesLessOfBothThanItDeclares, &both_logs_written()),
        Ok(RuleOutcome::Skipped {
            capability: LOG_REFUSED,
            reason: COUNT_LOG_REFUSED_REASON,
        })
    );
}

/// A refused count-cut log does not outrank a byte-cut log that paged: the
/// rule checked a `Nothing` at a boundary, so it ran.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_runs_when_only_the_count_cut_log_is_refused() {
    assert_eq!(
        sy37_against(
            Mistake::TakesFewerEventsThanItDeclares,
            &both_logs_written()
        ),
        Ok(RuleOutcome::Ran)
    );
}

/// A peer that overstates only `max_batch_bytes` tells a lie
/// `peer_declares_its_own_limits` does not judge, so SY-37's rule must not
/// report it as that rule's. Its byte-cut log goes one event to a push, which
/// the peer takes, and its pulls page at the limit it really imposes: the rule
/// runs. SY-18's batch rule is the one that fails it.
#[test]
fn nothing_waiting_means_the_next_pull_is_empty_runs_against_a_peer_overstating_its_batch_bytes() {
    // The inner peer enforces 64 KiB a push and declares 128 KiB: the
    // byte-cut log is three 64 KiB events, and the count-cut log fits one
    // pull of 1,000 events.
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;

    assert_eq!(
        sy37_against(Mistake::DeclaresMoreBatchBytesThanItTakes, &limits),
        Ok(RuleOutcome::Ran)
    );
    assert_eq!(
        sy18_against(Mistake::DeclaresMoreBatchBytesThanItTakes, &limits),
        Ok(RuleOutcome::Ran),
        "`peer_declares_its_own_limits` judges per-event limits only; the batch lie \
         is its sibling's, which is why SY-37's must not lay it on either"
    );
    assert_eq!(
        sy18_batch_against(Mistake::DeclaresMoreBatchBytesThanItTakes, &limits),
        Err("SY-18"),
        "SY-18's batch rule judges a batch-limit lie, so it must fail this peer"
    );
}

/// A peer that refuses more than [`TAKES_EVENTS`] events to a push, while it
/// declares 1,000: the count corner, 1,000 empty events, is refused.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_fails_a_peer_that_takes_fewer_events_than_it_declares()
{
    fails_exactly(
        &verdicts(async || {
            WrongFixture::with_limits(Mistake::TakesFewerEventsThanItDeclares, both_logs_written())
        }),
        "peer_takes_a_batch_at_its_declared_limits",
        "SY-18",
    );
}

/// A peer that declares 128 KiB to a push and enforces 64 KiB: the count
/// corner carries no bytes and lands, and the full corner, 1,000 events
/// carrying 128 KiB between them, is refused.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_fails_a_peer_that_declares_more_batch_bytes_than_it_takes()
 {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_events = 1_000;

    fails_exactly(
        &verdicts(async || {
            WrongFixture::with_limits(Mistake::DeclaresMoreBatchBytesThanItTakes, limits.clone())
        }),
        "peer_takes_a_batch_at_its_declared_limits",
        "SY-18",
    );
}

/// The mutant that shows the count corner must be the declared count exactly:
/// a corner of one event fewer lands.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_fails_a_peer_that_refuses_at_its_declared_event_limit()
{
    fails_exactly(
        &verdicts(async || {
            WrongFixture::with_limits(
                Mistake::RefusesAtTheDeclaredBatchEvents,
                both_logs_written(),
            )
        }),
        "peer_takes_a_batch_at_its_declared_limits",
        "SY-18",
    );
}

/// The mutant that shows the full corner must total the declared bytes
/// exactly: a corner of one byte fewer lands.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_fails_a_peer_that_refuses_at_its_declared_byte_limit()
{
    fails_exactly(
        &verdicts(async || {
            WrongFixture::with_limits(Mistake::RefusesAtTheDeclaredBatchBytes, both_logs_written())
        }),
        "peer_takes_a_batch_at_its_declared_limits",
        "SY-18",
    );
}

/// The mutant only the pull-back can see: its acknowledgement is the one a
/// faithful peer sends.
#[test]
fn peer_takes_a_batch_at_its_declared_limits_fails_a_peer_that_acks_a_large_push_and_keeps_its_head()
 {
    fails_exactly(
        &verdicts(async || {
            WrongFixture::with_limits(Mistake::KeepsOnlyTheHeadOfALargePush, both_logs_written())
        }),
        "peer_takes_a_batch_at_its_declared_limits",
        "SY-18",
    );
}
