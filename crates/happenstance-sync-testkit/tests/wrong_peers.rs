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
use std::sync::Arc;

use happenstance_core::{SequencePosition, StoreId};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, MemoryPeerError, MemoryResume, MemorySyncPeer,
    PeerLimits, Pulled, PushBatch, SendSyncPeer, Waiting,
};
use happenstance_sync_testkit::{
    NO_SECOND_PAGE, NO_SECOND_PAGE_REASON, RefusalsOf, SyncFixture, oversized_push_refusal, rules,
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
}

/// The reference peer's error, classified through the wrong peer's mistake.
#[derive(Debug)]
struct WrongError {
    inner: MemoryPeerError,
    mistake: Mistake,
    /// Whether [`Classify::class`] has answered once already. A `Cell`, not an
    /// atomic: the error is never shared across threads, and only
    /// [`Mistake::ClassChangesBetweenReads`] reads it.
    read: Cell<bool>,
}

impl core::fmt::Display for WrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl core::error::Error for WrongError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.inner)
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
                    self.inner.class()
                }
            }
            Mistake::Faithful
            | Mistake::EvaluatesTheGuard
            | Mistake::AcksThenDropsOnTheGuard
            | Mistake::FlattensGroups
            | Mistake::DeclaresNoRefusal
            | Mistake::AcceptsTheOversizedPush
            | Mistake::AlwaysSaysNothingWaiting
            | Mistake::SaysNothingOnAShortPage => self.inner.class(),
        }
    }
}

#[derive(Debug)]
struct WrongPeer {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
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
        WrongError {
            inner,
            mistake: self.mistake,
            read: Cell::new(false),
        }
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
            | Mistake::SaysNothingOnAShortPage => self
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
        }
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = self.inner.limits();
        if self.mistake == Mistake::AcceptsTheOversizedPush {
            // Declares half of what the inner peer enforces, so a batch one
            // byte over the declaration still fits under the real limit.
            limits.max_batch_bytes /= 2;
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
