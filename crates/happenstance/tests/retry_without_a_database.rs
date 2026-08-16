//! A caller's retry loop, driven against contention that cannot otherwise be
//! reproduced on demand.
//!
//! Nothing here opens a connection, binds a socket or writes a file: the store
//! is `MemoryEventStore` behind `happenstance-testkit`'s reference fixture, and
//! the contention comes from `FaultyStore`, which refuses an append **without
//! naming the conflicting event** — the answer a store reached over one-shot
//! HTTP gives conformantly, and the one no in-process store gives at all.
//!
//! That is the whole point of the file. A loop that branches on
//! `ConditionViolated::conflicting_position` being `Some` works against every
//! store an author can reach today and stops working in production; here it
//! stops working in a test that runs in a millisecond.

#![cfg(all(feature = "memory", feature = "json"))]
#![allow(clippy::unwrap_used)]

use core::sync::atomic::{AtomicU32, Ordering};

use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventType, Json, Retry, Tags,
    commit_with,
};
use happenstance_testkit::fixtures::MemoryFixture;
use happenstance_testkit::{FaultyStore, FaultyStoreError, Fixture};

const SUBSCRIBED: EventType = EventType::from_static("StudentSubscribed");

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Subscribed;

impl DomainEvent for Subscribed {
    const EVENT_TYPES: &'static [EventType] = &[SUBSCRIBED];

    fn event_type(&self) -> EventType {
        SUBSCRIBED
    }

    fn tags(&self) -> Tags {
        scope()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(codec: &C, _t: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

#[derive(Debug, Clone)]
struct Course {
    scope: Tags,
    subscribed: u32,
}

impl DecisionModel for Course {
    type Event = Subscribed;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, _event: Self::Event) {
        self.subscribed += 1;
    }
}

fn scope() -> Tags {
    Tags::from_pairs([("course", "c1")]).expect("a valid tag pair")
}

fn course() -> Course {
    Course {
        scope: scope(),
        subscribed: 0,
    }
}

/// One `MemoryEventStore` behind one handle, wrapped and armed.
async fn armed(violations: u32) -> FaultyStore<happenstance_testkit::fixtures::MemoryHandle> {
    let fixture = MemoryFixture::new();
    FaultyStore::new(fixture.connect().await).violate_next(violations)
}

// ---------------------------------------------------------------------------
// AC-009 — the loop survives a violation that names no conflict
// ---------------------------------------------------------------------------

#[tokio::test]
async fn retry_succeeds_after_an_injected_violation() {
    let store = armed(1).await;
    let retry = Retry::attempts(3.try_into().unwrap());

    let done = commit_with(&store, course(), &Json, retry, |_: &Course| {
        Ok::<_, core::convert::Infallible>(vec![Subscribed])
    })
    .await
    .expect("the loop re-decided and committed");

    assert_eq!(
        done.attempts, 2,
        "one injected violation, then success — and the injected violation \
         reported `conflicting_position: None`, so a loop that branched on \
         `Some` would have surfaced it instead of retrying"
    );
}

#[tokio::test]
async fn retry_refolds_from_a_pristine_model() {
    let fixture = MemoryFixture::new();
    let store = FaultyStore::new(fixture.connect().await);
    let retry = Retry::attempts(3.try_into().unwrap());

    // Seed one event *before* arming, so the fold has a state that could be
    // stale and the injection is spent on the attempt this test is about.
    let seeded = commit_with(&store, course(), &Json, Retry::once(), |_: &Course| {
        Ok::<_, core::convert::Infallible>(vec![Subscribed])
    })
    .await
    .expect("the seeding commit");
    assert_eq!(seeded.attempts, 1, "the seeding commit was uncontended");

    let store = store.violate_next(1);

    let attempts = AtomicU32::new(0);
    let done = commit_with(&store, course(), &Json, retry, |held: &Course| {
        attempts.fetch_add(1, Ordering::Relaxed);
        assert_eq!(
            held.subscribed, 1,
            "the loop re-used a stale fold: the model was applied twice over \
             one seeded event"
        );
        Ok::<_, core::convert::Infallible>(vec![Subscribed])
    })
    .await
    .expect("the retried commit");

    assert_eq!(done.attempts, 2);
    assert_eq!(
        attempts.load(Ordering::Relaxed),
        2,
        "the decision was taken again from a fresh read, not re-submitted"
    );
}

#[tokio::test]
async fn retry_is_bounded() {
    let store = armed(9).await;
    let retry = Retry::attempts(3.try_into().unwrap());

    let outcome = commit_with(&store, course(), &Json, retry, |_: &Course| {
        Ok::<_, core::convert::Infallible>(vec![Subscribed])
    })
    .await;

    match outcome {
        Err(CommandError::Exhausted { attempts, .. }) => assert_eq!(
            attempts, 3,
            "the bound the caller passed is the bound the loop honoured"
        ),
        other => panic!("a bounded retry must give up rather than hang: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// EC-005 — a read failure is not the concurrency signal
// ---------------------------------------------------------------------------

#[tokio::test]
async fn an_injected_read_failure_is_not_retried_as_contention() {
    let fixture = MemoryFixture::new();
    let store = FaultyStore::new(fixture.connect().await).fail_next_read(1);
    let retry = Retry::attempts(3.try_into().unwrap());

    let outcome = commit_with(&store, course(), &Json, retry, |_: &Course| {
        Ok::<_, core::convert::Infallible>(vec![Subscribed])
    })
    .await;

    match outcome {
        Err(CommandError::Read(FaultyStoreError::Injected)) => {}
        other => panic!(
            "a store failure during the read is not the DCB concurrency signal \
             and must not be retried as one: {other:?}"
        ),
    }
}
