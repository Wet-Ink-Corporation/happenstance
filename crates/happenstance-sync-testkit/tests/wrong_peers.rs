//! Every sync rule, failing the wrong implementation it names.
//!
//! A rule that no peer can fail is decorative. Each wrong peer here is the
//! reference peer with one plausible mistake added, and each test asserts that
//! the rule written against that mistake reports a violation of its own clause.
//! The faithful wrapper is the control: the same plumbing with no mistake
//! passes every rule, so a red result below is the mistake's and not the
//! wrapper's.

#![cfg(not(target_arch = "wasm32"))]

use std::sync::Arc;

use happenstance_core::{SequencePosition, StoreId};
use happenstance_sync::{
    Ack, EventGroup, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch,
    SendSyncPeer,
};
use happenstance_sync_testkit::{SyncFixture, Violation, rules};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a wrong peer makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// SY-1's wrong implementation: re-evaluate the guard that travelled with
    /// a group against what this peer already holds, and drop the group when
    /// it is violated.
    EvaluatesTheGuard,
    /// SY-2's wrong implementation: store every event as its own group, so a
    /// losing event and its compensation come back apart.
    FlattensGroups,
}

#[derive(Debug)]
struct WrongPeer {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
}

impl WrongPeer {
    /// Everything the inner peer holds, flattened, for the guard check.
    async fn held(&self) -> Result<Vec<EventGroup>, MemoryPeerError> {
        let mut groups = Vec::new();
        let mut resume = None;
        loop {
            let pulled = self.inner.pull(resume.as_ref()).await?;
            if pulled.batch.groups.is_empty() {
                return Ok(groups);
            }
            groups.extend(pulled.batch.groups);
            resume = Some(pulled.resume);
        }
    }
}

impl SendSyncPeer for WrongPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.inner.pull(from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        match self.mistake {
            Mistake::Faithful => self.inner.push(batch).await,
            Mistake::FlattensGroups => {
                let singletons = batch
                    .events()
                    .map(|event| EventGroup::new(None, vec![event.clone()]))
                    .collect();
                self.inner.push(&PushBatch::new(singletons)).await
            }
            Mistake::EvaluatesTheGuard => {
                let mut ack = Ack::default();
                for group in &batch.groups {
                    let held = self.held().await?;
                    let violated = group.guard.as_ref().is_some_and(|guard| {
                        held.iter().flat_map(|held| &held.events).any(|event| {
                            guard.is_violated_by(
                                SequencePosition::FIRST,
                                event.event.event_type(),
                                event.event.tags(),
                            )
                        })
                    });
                    if violated {
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
    }

    fn limits(&self) -> PeerLimits {
        self.inner.limits()
    }
}

#[derive(Debug)]
struct WrongFixture {
    inner: Arc<MemorySyncPeer>,
    mistake: Mistake,
}

impl WrongFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            inner: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0x77; 16]))),
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
}

fn clause_of(result: Result<RuleOutcome, Violation>) -> Option<&'static str> {
    result.err().map(|violation| violation.clause())
}

#[test]
fn the_faithful_wrapper_passes_every_rule() {
    let open = async || WrongFixture::new(Mistake::Faithful);
    assert_eq!(
        block_on(rules::ingest_never_rejects(open)),
        Ok(RuleOutcome::Ran)
    );
    assert_eq!(
        block_on(rules::compensation_is_atomic_with_the_losing_event(open)),
        Ok(RuleOutcome::Ran)
    );
}

#[test]
fn ingest_never_rejects_fails_a_peer_that_evaluates_the_guard() {
    let open = async || WrongFixture::new(Mistake::EvaluatesTheGuard);
    assert_eq!(
        clause_of(block_on(rules::ingest_never_rejects(open))),
        Some("SY-1")
    );
}

#[test]
fn compensation_is_atomic_fails_a_peer_that_flattens_groups() {
    let open = async || WrongFixture::new(Mistake::FlattensGroups);
    assert_eq!(
        clause_of(block_on(
            rules::compensation_is_atomic_with_the_losing_event(open)
        )),
        Some("SY-2")
    );
}
