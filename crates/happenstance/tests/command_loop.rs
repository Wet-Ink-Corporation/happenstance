//! Read, decide, append, retry — driven from **outside** the crate.
//!
//! The store below is a recording, violating wrapper around
//! [`MemoryEventStore`]. It is private to this test target on purpose: an
//! application must not carry a test double in its dependency graph, and the
//! public instrument (`FaultyStore<S>`) is `misbehaving-testkit-stores`', in
//! `happenstance-testkit`. Delete this wrapper when that lands.

use core::cell::{Cell, RefCell};
use core::convert::Infallible;

use futures_core::Stream;
use happenstance::bytes::Bytes;
use happenstance::{
    AppendCondition, AppendError, Boundary, Codec, CodecError, CommandError, ConditionViolated,
    DecisionModel, DomainEvent, Event, EventStore, EventType, Json, MemoryEventStore,
    MemoryStoreError, Query, ReadOptions, Retry, SequencePosition, SequencedEvent, Tags, commit,
    commit_with,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// One two-variant domain
// ---------------------------------------------------------------------------

const DEFINED: EventType = EventType::from_static("CourseDefined");
const SUBSCRIBED: EventType = EventType::from_static("StudentSubscribed");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Enrolment {
    Defined { capacity: u32 },
    Subscribed { student: String },
}

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[DEFINED, SUBSCRIBED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Defined { .. } => DEFINED,
            Self::Subscribed { .. } => SUBSCRIBED,
        }
    }

    fn tags(&self) -> Tags {
        course_tags()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if !Self::EVENT_TYPES.contains(event_type) {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seats {
    scope: Tags,
    capacity: Option<u32>,
    taken: u32,
}

impl Seats {
    fn for_course() -> Self {
        Self {
            scope: course_tags(),
            capacity: None,
            taken: 0,
        }
    }
}

impl DecisionModel for Seats {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::Defined { capacity } => self.capacity = Some(capacity),
            Enrolment::Subscribed { .. } => self.taken += 1,
        }
    }
}

fn course_tags() -> Tags {
    Tags::from_pairs([("course", "c1")]).expect("a valid tag pair")
}

fn subscribed(student: &str) -> Enrolment {
    Enrolment::Subscribed {
        student: student.to_owned(),
    }
}

/// One domain event, encoded and tagged the way a command would have left it.
fn landed(event: &Enrolment) -> Event {
    let payload = event.encode(&Json).expect("the fixture encodes");
    Event::new(event.event_type(), payload)
        .expect("a valid event type")
        .with_tags(event.tags())
}

/// A domain refusal, so `CommandError::Refused` carries a real caller type.
#[derive(Debug, PartialEq, Eq)]
struct Full;

impl core::fmt::Display for Full {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("course c1 is full")
    }
}

impl core::error::Error for Full {}

// ---------------------------------------------------------------------------
// The recording, violating store
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Watch {
    /// The last position each `read` actually observed, per attempt.
    anchors: Vec<Option<SequencePosition>>,
    /// Every condition submitted, in order.
    conditions: Vec<AppendCondition>,
    /// Every batch submitted, in order.
    appended: Vec<Vec<Event>>,
}

/// Three knobs and no more: violate the next *n* appends, record what was
/// submitted, and land a competing writer's event between read and append.
#[derive(Debug)]
struct Contended {
    inner: MemoryEventStore,
    watch: RefCell<Watch>,
    violate: Cell<u32>,
    interlopers: RefCell<Vec<Enrolment>>,
}

impl Contended {
    fn new() -> Self {
        Self {
            inner: MemoryEventStore::new(),
            watch: RefCell::new(Watch::default()),
            violate: Cell::new(0),
            interlopers: RefCell::new(Vec::new()),
        }
    }

    fn violate_next(&self, n: u32) -> &Self {
        self.violate.set(n);
        self
    }

    /// A competing writer lands `event` just before the next append.
    fn interlope(&self, event: Enrolment) -> &Self {
        self.interlopers.borrow_mut().push(event);
        self
    }

    /// An event a *previous* command left behind, in place before this one runs.
    ///
    /// It goes straight to the inner store, bypassing the watch on purpose: it
    /// is not one of the loop's own appends and must not be counted as one. Its
    /// job is to give the fold a non-zero state, so that a test can tell a
    /// pristine re-fold from a stale one.
    async fn seed(&self, event: Enrolment) {
        EventStore::append(&self.inner, &[landed(&event)], None)
            .await
            .expect("the fixture seeds");
    }

    fn positions(&self) -> Vec<SequencePosition> {
        self.inner
            .snapshot()
            .iter()
            .map(|event| event.position)
            .collect()
    }
}

impl EventStore for Contended {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // The anchor this read returns, computed the way the read computes it:
        // the last position among the events the query nominates.
        let observed = self
            .inner
            .snapshot()
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()))
            .map(|event| event.position)
            .next_back();
        self.watch.borrow_mut().anchors.push(observed);

        EventStore::read(&self.inner, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        {
            let mut watch = self.watch.borrow_mut();
            if let Some(condition) = condition {
                watch.conditions.push(condition.clone());
            }
            watch.appended.push(events.to_vec());
        }

        let interloper = if self.interlopers.borrow().is_empty() {
            None
        } else {
            Some(self.interlopers.borrow_mut().remove(0))
        };
        if let Some(event) = interloper {
            EventStore::append(&self.inner, &[landed(&event)], None).await?;
        }

        let remaining = self.violate.get();
        if remaining > 0 {
            self.violate.set(remaining - 1);
            // `None`, always: a store reached over one-shot HTTP has no
            // interactive transaction and cannot name the conflicting event.
            // A loop that branches on `Some` never retries against this store.
            return Err(AppendError::ConditionViolated(
                ConditionViolated::unspecified(),
            ));
        }

        EventStore::append(&self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(&self.inner).await
    }

    async fn contains_event_id(&self, id: happenstance::EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(&self.inner, id).await
    }
}

fn three() -> Retry {
    Retry::attempts(3.try_into().expect("3 is not zero"))
}

// ---------------------------------------------------------------------------
// AC-001 — the `after` anchor comes from the read, and only from the read
// ---------------------------------------------------------------------------

#[tokio::test]
async fn after_anchor_comes_from_the_read() {
    let store = Contended::new();
    // A competing writer lands between attempt 1's read and its append, so
    // attempt 1 is violated and attempt 2 reads a store that has moved.
    store.interlope(subscribed("interloper"));

    let done = commit_with(&store, Seats::for_course(), &Json, three(), |_: &Seats| {
        Ok::<_, Infallible>(vec![subscribed("s1")])
    })
    .await
    .expect("the second attempt commits");

    assert_eq!(done.attempts, 2, "the interloper did not force a retry");

    let watch = store.watch.borrow();
    assert_eq!(watch.conditions.len(), 2);
    assert_eq!(watch.anchors.len(), 2);
    for (attempt, condition) in watch.conditions.iter().enumerate() {
        for guard in condition.guards() {
            assert_eq!(
                guard.after,
                watch.anchors[attempt],
                "attempt {} anchored on something its own read did not return",
                attempt + 1
            );
        }
    }
    // And the second attempt's anchor really did move, so the assertion above
    // is not comparing two `None`s.
    assert!(watch.anchors[0].is_none());
    assert!(watch.anchors[1].is_some());
}

#[test]
fn threads_append_return_is_rejected() {
    // The wrong implementation, written out: `after` taken from what `append`
    // returned rather than from what the read observed. Seeded so the two
    // differ — positions may have gaps, and another writer may hold one
    // *below* the returned value that this caller never saw.
    let observed = SequencePosition::new(5);
    let returned = SequencePosition::new(9);
    let query = Seats::for_course().query().expect("a constrained boundary");

    let sound = AppendCondition::new(query.clone()).after_opt(observed);
    let unsound = AppendCondition::new(query).after_opt(returned);

    let unseen = SequencePosition::new(7).expect("7 is not zero");
    assert!(
        sound.is_violated_by(unseen, &SUBSCRIBED, &course_tags()),
        "the read's anchor must catch an event this caller never saw"
    );
    assert!(
        !unsound.is_violated_by(unseen, &SUBSCRIBED, &course_tags()),
        "append's return silently excludes exactly what a condition exists to catch"
    );
}

// ---------------------------------------------------------------------------
// AC-002 — a refusal appends nothing
// ---------------------------------------------------------------------------

#[tokio::test]
async fn refusal_appends_nothing() {
    let store = Contended::new();
    let before = store.positions();

    let refused = commit_with(&store, Seats::for_course(), &Json, three(), |_: &Seats| {
        Err::<Vec<Enrolment>, _>(Full)
    })
    .await
    .expect_err("the decision refused");

    assert!(
        matches!(&refused, CommandError::Refused(Full)),
        "expected the caller's own error, got {refused:?}"
    );
    // Compared by the positions the store actually assigned, never by literals.
    assert_eq!(store.positions(), before);
    assert!(
        store.watch.borrow().appended.is_empty(),
        "a refusal reached the port"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — a retry re-decides against the world as it now is
// ---------------------------------------------------------------------------

/// The wrong implementation this rejects: `let mut model = boundary.clone();`
/// hoisted **out** of the retry loop, so attempt 2 folds its own read onto
/// attempt 1's model instead of onto a pristine one.
///
/// That shape needs a non-zero state to be visible at all. Against an empty
/// store both the sound loop and the hoisted one record `[0, 1]` — attempt 1
/// folds nothing either way — so the empty-store version of this test rejected
/// only *"the loop never re-reads"*. One subscription already in the store
/// separates them: a pristine re-fold records `[1, 2]` and a stale one
/// double-counts the seeded event to record `[1, 3]`.
#[tokio::test]
async fn retry_refolds_from_pristine_state() {
    let store = Contended::new();
    // A previous command's event, so attempt 1 has something to fold.
    store.seed(subscribed("s0")).await;
    store.interlope(subscribed("interloper"));

    let folded: RefCell<Vec<u32>> = RefCell::new(Vec::new());
    let done = commit_with(
        &store,
        Seats::for_course(),
        &Json,
        three(),
        |seats: &Seats| {
            folded.borrow_mut().push(seats.taken);
            Ok::<_, Infallible>(vec![subscribed("s1")])
        },
    )
    .await
    .expect("the second attempt commits");

    assert_eq!(done.attempts, 2);
    assert_eq!(
        *folded.borrow(),
        vec![1, 2],
        "attempt 2 folded a store that had moved onto a model that had not been \
         rebuilt: a hoisted clone records [1, 3] here"
    );
}

#[tokio::test]
async fn retry_does_not_resubmit_the_previous_batch() {
    let store = Contended::new();
    store.interlope(subscribed("interloper"));

    let attempt = Cell::new(0_u32);
    let done = commit_with(&store, Seats::for_course(), &Json, three(), |_: &Seats| {
        attempt.set(attempt.get() + 1);
        Ok::<_, Infallible>(vec![subscribed(&format!("s{}", attempt.get()))])
    })
    .await
    .expect("the second attempt commits");

    assert_eq!(done.attempts, 2);
    let watch = store.watch.borrow();
    assert_eq!(watch.appended.len(), 2);
    let second = Json.encode(&subscribed("s2")).expect("the fixture encodes");
    assert_eq!(
        watch.appended[1].first().map(Event::data),
        Some(&second),
        "attempt 2 submitted attempt 1's events"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — the retry predicate, and the hint it declines to branch on
// ---------------------------------------------------------------------------

#[tokio::test]
async fn retries_when_conflicting_position_is_none() {
    let store = Contended::new();
    store.violate_next(1);

    let done = commit_with(&store, Seats::for_course(), &Json, three(), |_: &Seats| {
        Ok::<_, Infallible>(vec![subscribed("s1")])
    })
    .await
    .expect("a violation with no named conflict still retries");

    assert_eq!(done.attempts, 2);
}

#[test]
fn branching_on_some_is_rejected() {
    // The wrong predicate, written out. It works against an in-process store
    // and stops working against one reached over one-shot HTTP.
    fn wrong<E>(err: &AppendError<E>) -> bool {
        match err {
            AppendError::ConditionViolated(violation) => violation.conflicting_position.is_some(),
            _ => false,
        }
    }

    let reported: AppendError<MemoryStoreError> =
        AppendError::ConditionViolated(ConditionViolated::unspecified());

    assert!(
        reported.is_condition_violated(),
        "the sound predicate retries"
    );
    assert!(
        !wrong(&reported),
        "a loop that gates on `Some` never retries against a conformant remote store"
    );
}

#[tokio::test]
async fn exhausted_carries_the_violation() {
    let store = Contended::new();
    store.violate_next(9);

    let err = commit_with(
        &store,
        Seats::for_course(),
        &Json,
        Retry::attempts(2.try_into().expect("2 is not zero")),
        |_: &Seats| Ok::<_, Infallible>(vec![subscribed("s1")]),
    )
    .await
    .expect_err("every attempt was violated");

    let CommandError::Exhausted { attempts, source } = &err else {
        panic!("expected Exhausted, got {err:?}");
    };
    assert_eq!(*attempts, 2);
    // Carried for reporting, never consulted for control flow.
    assert_eq!(source.conflicting_position, None);
    assert!(
        core::error::Error::source(&err).is_some(),
        "the violation is not reachable through the error chain"
    );
}

// ---------------------------------------------------------------------------
// AC-006 — the bound is visible, and running out is its own answer
// ---------------------------------------------------------------------------

#[tokio::test]
async fn exhaustion_is_bounded_and_named() {
    for (retry, expected) in [
        (Retry::attempts(2.try_into().expect("2 is not zero")), 2),
        (Retry::once(), 1),
    ] {
        let store = Contended::new();
        store.violate_next(99);

        let err = commit_with(&store, Seats::for_course(), &Json, retry, |_: &Seats| {
            Ok::<_, Infallible>(vec![subscribed("s1")])
        })
        .await
        .expect_err("every attempt was violated");

        assert!(
            matches!(err, CommandError::Exhausted { attempts, .. } if attempts == expected),
            "expected Exhausted after {expected} attempts, got {err:?}"
        );
        assert_eq!(
            store.watch.borrow().appended.len(),
            expected as usize,
            "the loop attempted a different number of appends than it reported"
        );
    }
}

// ---------------------------------------------------------------------------
// The JSON convenience is the same loop
// ---------------------------------------------------------------------------

#[tokio::test]
async fn commit_is_commit_with_json() {
    let store = MemoryEventStore::new();

    let done = commit(&store, Seats::for_course(), three(), |_: &Seats| {
        Ok::<_, Infallible>(vec![Enrolment::Defined { capacity: 2 }])
    })
    .await
    .expect("an uncontended commit");

    assert_eq!(done.attempts, 1);
    let held = store.snapshot();
    let first = held.first().expect("one event landed");
    assert_eq!(first.position, done.position);
    assert_eq!(
        first.event.data(),
        &Json
            .encode(&Enrolment::Defined { capacity: 2 })
            .expect("the fixture encodes")
    );
}
