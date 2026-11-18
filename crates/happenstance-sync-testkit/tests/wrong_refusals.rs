//! SY-33's rule, failing the peers whose refusals depend on their logs — and
//! only that rule, except where a second rule is written against the same
//! mistake from another side.
//!
//! A rule that no peer can fail is decorative. Each wrong peer here is the
//! reference peer with one condition-level refusal added: the refusal SY-33
//! forbids because it can never succeed on a retry. Each test runs the
//! **whole** enumeration, as `tests/wrong_peers.rs` does, so a mutant that
//! breaks another rule too is reported rather than counted as a proof of this
//! one.

#![cfg(not(target_arch = "wasm32"))]

use std::sync::Arc;

use happenstance_core::{EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, MemoryPeerError, MemoryResume, MemorySyncPeer,
    PeerLimits, Pulled, PushBatch, SendSyncPeer,
};
use happenstance_sync_testkit::{
    RefusalsOf, SyncFixture, Violation, oversized_push_refusal, rules,
};
use happenstance_testkit::{Capability, RuleOutcome, block_on};

/// The one condition-level refusal a [`LogKeyedPeer`] adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// Refuse a push that carries an event this peer already holds, as a
    /// "duplicate". A re-delivery is a skip (SY-11), and a refusal of it can
    /// never succeed, because the peer will hold that event for ever.
    RefusesWhatItHolds,
    /// Refuse a group, with an error, when what this peer holds violates the
    /// guard the group travelled under: the "conflict" refusal, and the
    /// literal condition-level refusal. SY-1's rule rejects it too, from the
    /// side of the fact that was refused.
    RefusesAViolatedGuard,
    /// The same "conflict" refusal, written the way a careful peer would:
    /// a group whose events this peer already holds is a re-delivery and is
    /// skipped first (SY-11), and only a group with new events has its guard
    /// checked against the log. The rule's re-sent probe is skipped here, so
    /// only its late probe, the same shape from a fresh origin, sees it.
    RefusesAViolatedGuardOnNewEvents,
    /// Refuse the oversized push as `Split` while the log is empty and `Stop`
    /// once it holds anything; a pull's refusal keeps the reference peer's
    /// class. Each error's class is fixed when it is made, so
    /// SY-36's "a function of the error alone" holds; what varies is which
    /// error the same push draws, and only the log decides that.
    ClassFollowsTheLog,
}

/// Why a [`LogKeyedPeer`] refused.
#[derive(Debug)]
enum Cause {
    /// The reference peer's own refusal.
    Peer(MemoryPeerError),
    /// The push carried an event the peer already holds.
    Held(EventId),
    /// What the peer holds violates the group's guard.
    GuardViolated,
}

/// The wrong peer's error, its class decided when it was made.
#[derive(Debug)]
struct KeyedError {
    cause: Cause,
    class: FailureClass,
}

impl core::fmt::Display for KeyedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.cause {
            Cause::Peer(inner) => inner.fmt(f),
            Cause::Held(id) => write!(f, "duplicate: this peer already holds {id}"),
            Cause::GuardViolated => f.write_str("conflict: the group's guard is violated here"),
        }
    }
}

impl core::error::Error for KeyedError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self.cause {
            Cause::Peer(inner) => Some(inner),
            Cause::Held(_) | Cause::GuardViolated => None,
        }
    }
}

impl Classify for KeyedError {
    fn class(&self) -> FailureClass {
        self.class
    }
}

#[derive(Debug)]
struct LogKeyedPeer {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
}

impl LogKeyedPeer {
    /// Everything the inner peer holds, from its floor: a compacted reference
    /// peer refuses the start (SY-32).
    async fn held(&self) -> Result<Vec<EventGroup>, MemoryPeerError> {
        let mut groups = Vec::new();
        let mut resume = Some(self.inner.floor_token()?);
        loop {
            let pulled = self.inner.pull(resume.as_ref()).await?;
            if pulled.batch.groups.is_empty() {
                return Ok(groups);
            }
            groups.extend(pulled.batch.groups);
            resume = Some(pulled.resume);
        }
    }

    /// The condition-level refusal this peer's mistake makes of `batch`, given
    /// what it holds, if it makes one.
    async fn condition_refusal(&self, batch: &PushBatch) -> Result<Option<Cause>, MemoryPeerError> {
        let held = self.held().await?;
        let events = || held.iter().flat_map(|group| &group.events);
        Ok(match self.mistake {
            Mistake::RefusesWhatItHolds => batch
                .events()
                .find(|offered| events().any(|event| event.id == offered.id))
                .map(|offered| Cause::Held(offered.id)),
            Mistake::RefusesAViolatedGuard => batch
                .groups
                .iter()
                .filter_map(|group| group.guard.as_ref())
                .any(|guard| {
                    events().any(|event| {
                        guard.is_violated_by(
                            SequencePosition::FIRST,
                            event.event.event_type(),
                            event.event.tags(),
                        )
                    })
                })
                .then_some(Cause::GuardViolated),
            Mistake::RefusesAViolatedGuardOnNewEvents => batch
                .groups
                .iter()
                .filter(|group| {
                    group
                        .events
                        .iter()
                        .any(|offered| !events().any(|event| event.id == offered.id))
                })
                .filter_map(|group| group.guard.as_ref())
                .any(|guard| {
                    events().any(|event| {
                        guard.is_violated_by(
                            SequencePosition::FIRST,
                            event.event.event_type(),
                            event.event.tags(),
                        )
                    })
                })
                .then_some(Cause::GuardViolated),
            Mistake::Faithful | Mistake::ClassFollowsTheLog => None,
        })
    }

    /// The reference peer's refusal of a pull, or of the read a push's check
    /// makes, classified as the reference peer classifies it.
    fn wrap(inner: MemoryPeerError) -> KeyedError {
        KeyedError {
            class: inner.class(),
            cause: Cause::Peer(inner),
        }
    }

    /// The reference peer's refusal of a push, classified as this peer's
    /// mistake says. A pull's refusal is left alone, so SY-32's `Reseed` stays
    /// that rule's and this mutant is SY-33's alone.
    async fn refuse_push(&self, inner: MemoryPeerError) -> KeyedError {
        let class = match self.mistake {
            Mistake::ClassFollowsTheLog if matches!(self.held().await, Ok(held) if !held.is_empty()) => {
                FailureClass::Stop
            }
            _ => inner.class(),
        };
        KeyedError {
            cause: Cause::Peer(inner),
            class,
        }
    }
}

impl SendSyncPeer for LogKeyedPeer {
    type Error = KeyedError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.inner.pull(from).await.map_err(Self::wrap)
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        match self.condition_refusal(batch).await {
            Ok(Some(cause)) => {
                return Err(KeyedError {
                    cause,
                    class: FailureClass::Stop,
                });
            }
            Ok(None) => {}
            Err(error) => return Err(Self::wrap(error)),
        }
        match self.inner.push(batch).await {
            Ok(ack) => Ok(ack),
            Err(error) => Err(self.refuse_push(error).await),
        }
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

#[derive(Debug)]
struct LogKeyedFixture {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
}

impl LogKeyedFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x7C; 16]))),
            mistake,
        }
    }
}

impl SyncFixture for LogKeyedFixture {
    type Peer = LogKeyedPeer;

    async fn connect(&self) -> Self::Peer {
        LogKeyedPeer {
            inner: Arc::clone(&self.inner),
            mistake: self.mistake,
        }
    }

    /// The reference fixture's one refusal, the oversized push.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }

    /// The reference peer's compaction, so SY-32's rule runs here too and a
    /// skip there does not stand in for a pass.
    const COMPACTION: Capability = Capability::SUPPORTED;

    async fn compact_recorded_before(
        &self,
        _peer: &Self::Peer,
        cutoff: RecordedAt,
    ) -> Result<(), Violation> {
        self.inner
            .compact_recorded_before(cutoff)
            .map(drop)
            .map_err(|error| Violation::new("SY-32", format!("could not compact: {error}")))
    }
}

/// Every rule in the enumeration against `mistake`'s fixture, each verdict
/// reduced to the clause it reports.
fn verdicts(mistake: Mistake) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)> {
    let open = async || LogKeyedFixture::new(mistake);
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

/// Each `(rule, clause)` reports its clause, and every other rule runs and
/// passes.
fn fails_exactly_these(mistake: Mistake, expected: &[(&str, &'static str)]) {
    let verdicts = verdicts(mistake);
    for (rule, _) in expected {
        assert!(
            verdicts.iter().any(|(name, _)| name == rule),
            "`{rule}` is not in the enumeration: {verdicts:?}"
        );
    }
    for (name, verdict) in &verdicts {
        if let Some((_, clause)) = expected.iter().find(|(rule, _)| rule == name) {
            assert_eq!(*verdict, Err(*clause), "`{name}` must reject {mistake:?}");
        } else {
            assert_eq!(
                *verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` failed {mistake:?}, a mutant written against {expected:?}"
            );
        }
    }
}

/// The control: the wrapper with no mistake passes every rule, so a red
/// result below is the mistake's and not the wrapper's.
#[test]
fn the_faithful_log_keyed_wrapper_passes_every_rule() {
    fails_exactly_these(Mistake::Faithful, &[]);
}

/// The "duplicate" refusal: only the rule's re-sent probe sees it, because
/// nothing else in the enumeration re-sends an event the peer holds.
#[test]
fn refusal_is_never_condition_derived_fails_a_peer_that_refuses_what_it_holds() {
    fails_exactly_these(
        Mistake::RefusesWhatItHolds,
        &[("refusal_is_never_condition_derived", "SY-33")],
    );
}

/// The same push draws `Split` on the fresh receiver and `Stop` once it holds
/// the rule's seed: only the comparison of the fixture's two runs sees it, and
/// SY-36's rule, on a fresh peer, sees `Split` as declared.
#[test]
fn refusal_is_never_condition_derived_fails_a_peer_whose_class_follows_its_log() {
    fails_exactly_these(
        Mistake::ClassFollowsTheLog,
        &[("refusal_is_never_condition_derived", "SY-33")],
    );
}

/// The "conflict" refusal, the one SY-33 was written against. SY-1's rule
/// rejects it from the side of the second claim it refused; SY-33's from the
/// side of the refusal that appeared once the log held a violating event.
#[test]
fn refusal_is_never_condition_derived_fails_a_peer_that_refuses_a_violated_guard() {
    fails_exactly_these(
        Mistake::RefusesAViolatedGuard,
        &[
            ("ingest_never_rejects", "SY-1"),
            ("refusal_is_never_condition_derived", "SY-33"),
        ],
    );
}

/// The careful "conflict" refusal, which skips a re-delivery before it checks
/// the guard. The rule's re-sent probe passes it, so this pins that the late
/// probe from a fresh origin does work of its own.
#[test]
fn refusal_is_never_condition_derived_fails_a_peer_that_refuses_a_violated_guard_on_new_events() {
    fails_exactly_these(
        Mistake::RefusesAViolatedGuardOnNewEvents,
        &[
            ("ingest_never_rejects", "SY-1"),
            ("refusal_is_never_condition_derived", "SY-33"),
        ],
    );
}
