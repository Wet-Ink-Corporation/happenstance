//! Tuple composition, driven from **outside** the crate.
//!
//! An integration test compiles as a downstream crate, so everything here is
//! reachable only through `happenstance`'s public root. That is what a sealed
//! trait's impls have to be, and it is exactly what a `#[cfg(test)]` module
//! inside the crate cannot show.

use happenstance::bytes::Bytes;
use happenstance::{
    Boundary, Codec, CodecError, DecisionModel, DomainEvent, Event, EventId, EventType, QueryItem,
    RecordedAt, SequencePosition, SequencedEvent, StoreId, Tags,
};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// One event vocabulary, shared by every model below.
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
        Tags::empty()
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

/// One model, reused under different scopes. Two models sharing an event type
/// over disjoint tags is the shape that makes routing-by-arrival visible.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Tally {
    scope: Tags,
    seen: u32,
}

impl Tally {
    fn scoped(key: &str, value: &str) -> Self {
        Self {
            scope: Tags::from_pairs([(key, value)]).expect("a valid tag pair"),
            seen: 0,
        }
    }
}

impl DecisionModel for Tally {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::Defined { .. } | Enrolment::Subscribed { .. } => self.seen += 1,
        }
    }
}

struct Json;

impl Codec for Json {
    const TAG: &'static str = "json";

    fn encode<T: Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
        serde_json::to_vec(value)
            .map(Bytes::from)
            .map_err(|e| CodecError::Encode(Box::new(e)))
    }

    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
        serde_json::from_slice(data).map_err(|e| CodecError::Decode(Box::new(e)))
    }
}

fn read_event(position: u64, event: &Enrolment, tags: &[(&str, &str)]) -> SequencedEvent {
    let payload = event.encode(&Json).expect("the fixture encodes");
    let tags = Tags::from_pairs(tags.iter().copied()).expect("valid tag pairs");
    let event = Event::new(event.event_type(), payload)
        .expect("a valid event type")
        .with_tags(tags);
    let position = SequencePosition::new(position).expect("a non-zero position");
    SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes([3; 16]), position),
        RecordedAt::from_millis(1_700_000_000_000),
        event,
    )
}

// ---------------------------------------------------------------------------
// AC-001 — composing is writing a tuple
// ---------------------------------------------------------------------------

#[test]
fn a_tuple_is_a_boundary() {
    let course = Tally::scoped("course", "c1");
    let student = Tally::scoped("student", "s1");
    let term = Tally::scoped("term", "t1");

    // Two. No macro invocation, no builder, no extra import.
    let pair = (course.clone(), student.clone());
    assert_eq!(
        pair.query()
            .expect("a pair derives")
            .items()
            .map_or(0, <[_]>::len),
        2
    );

    // Three.
    let triple = (course.clone(), student.clone(), term.clone());
    assert_eq!(
        triple
            .query()
            .expect("a triple derives")
            .items()
            .map_or(0, <[_]>::len),
        3
    );

    // Eight — the ceiling. A nine-tuple is an ordinary "trait bound not
    // satisfied" on the caller's own line; `trybuild` has no home in this tree
    // and no substitute discriminates, so that refusal is stated, not tested.
    let mut eight = (
        Tally::scoped("a", "1"),
        Tally::scoped("b", "2"),
        Tally::scoped("c", "3"),
        Tally::scoped("d", "4"),
        Tally::scoped("e", "5"),
        Tally::scoped("f", "6"),
        Tally::scoped("g", "7"),
        Tally::scoped("h", "8"),
    );
    assert_eq!(
        eight
            .query()
            .expect("an eight-tuple derives")
            .items()
            .map_or(0, <[_]>::len),
        8
    );

    // And it absorbs: driven only through `happenstance::Boundary`.
    let event = read_event(1, &Enrolment::Defined { capacity: 2 }, &[("a", "1")]);
    eight.absorb(&event, &Json).expect("an eight-tuple absorbs");
    assert_eq!(eight.0.seen, 1);
}

// ---------------------------------------------------------------------------
// AC-002 — the composed query is a union, never a narrowing
// ---------------------------------------------------------------------------

/// The oracle, spelled in the opposite direction from the implementation: each
/// member's **own** `query()` result, concatenated in member order. It shares
/// no subroutine with the composite (RS-60-4) and is never a literal item list.
fn union_of(members: &[&Tally]) -> Vec<QueryItem> {
    let mut items = Vec::new();
    for member in members {
        let own = member.query().expect("each member derives its own query");
        items.extend_from_slice(own.items().expect("a member constrains something"));
    }
    items
}

#[test]
fn composed_query_is_the_union_of_member_queries() {
    let course = Tally::scoped("course", "c1");
    let student = Tally::scoped("student", "s1");

    let composed = (course.clone(), student.clone())
        .query()
        .expect("the composite derives");

    assert_eq!(
        composed
            .items()
            .expect("the composite constrains something"),
        union_of(&[&course, &student]).as_slice()
    );

    // And it *selects* the union: every event either member would have
    // selected alone is selected by the composite.
    for tags in [&[("course", "c1")][..], &[("student", "s1")][..]] {
        let event = read_event(
            1,
            &Enrolment::Subscribed {
                student: "s1".into(),
            },
            tags,
        );
        assert!(composed.matches(event.event.event_type(), event.event.tags()));
    }
}

#[test]
fn a_narrowing_union_is_rejected() {
    let course = Tally::scoped("course", "c1");
    let student = Tally::scoped("student", "s1");

    // The wrong implementation, hand-rolled here: "tidy" the union by dropping
    // items whose type set is already present. It compiles, it is shorter, and
    // it selects fewer events than its members did — a lost update with no
    // diagnostic anywhere.
    let mut narrowed: Vec<QueryItem> = Vec::new();
    for item in union_of(&[&course, &student]) {
        if !narrowed.iter().any(|kept| kept.types() == item.types()) {
            narrowed.push(item);
        }
    }

    let honest = (course.clone(), student.clone())
        .query()
        .expect("the composite derives");
    let honest_items = honest.items().expect("the composite constrains something");

    assert_ne!(
        narrowed.as_slice(),
        honest_items,
        "the narrowing composite must fail the union assertion"
    );

    // Concretely: the narrowed query stops selecting the second member's
    // events, which is the failure that has no diagnostic.
    let student_event = read_event(
        1,
        &Enrolment::Subscribed {
            student: "s1".into(),
        },
        &[("student", "s1")],
    );
    let ty = student_event.event.event_type();
    let tags = student_event.event.tags();
    assert!(honest.matches(ty, tags));
    assert!(
        !narrowed.iter().any(|item| item.matches(ty, tags)),
        "the narrowed union lost a member's boundary"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — absorb routes by nomination, not by arrival
// ---------------------------------------------------------------------------

#[test]
fn each_member_absorbs_only_what_it_nominated() {
    let mut both = (
        Tally::scoped("course", "c1"),
        Tally::scoped("student", "s1"),
    );

    // Same event type, disjoint tags: one for each member.
    let for_course = read_event(1, &Enrolment::Defined { capacity: 2 }, &[("course", "c1")]);
    let for_student = read_event(
        2,
        &Enrolment::Subscribed {
            student: "s1".into(),
        },
        &[("student", "s1")],
    );
    // And one inside both boundaries at once.
    let for_both = read_event(
        3,
        &Enrolment::Subscribed {
            student: "s1".into(),
        },
        &[("course", "c1"), ("student", "s1")],
    );
    // And one inside neither.
    let for_neither = read_event(4, &Enrolment::Defined { capacity: 9 }, &[("course", "c2")]);

    for event in [&for_course, &for_student, &for_both, &for_neither] {
        both.absorb(event, &Json).expect("absorption succeeds");
    }

    assert_eq!(both.0.seen, 2, "the course model folded its own two");
    assert_eq!(both.1.seen, 2, "the student model folded its own two");
}

#[test]
fn routing_by_arrival_is_rejected() {
    let course = Tally::scoped("course", "c1");
    let student = Tally::scoped("student", "s1");

    let events = [
        read_event(1, &Enrolment::Defined { capacity: 2 }, &[("course", "c1")]),
        read_event(
            2,
            &Enrolment::Subscribed {
                student: "s1".into(),
            },
            &[("student", "s1")],
        ),
    ];

    // The wrong implementation, hand-rolled here: hand every arriving event to
    // every member. It compiles, and it passes any test whose models never
    // share an event type.
    let mut by_arrival = (course.clone(), student.clone());
    for event in &events {
        let decoded = Enrolment::decode(&Json, event.event.event_type(), event.event.data())
            .expect("the fixture decodes");
        by_arrival.0.apply(decoded.clone());
        by_arrival.1.apply(decoded);
    }

    let mut by_nomination = (course, student);
    for event in &events {
        by_nomination
            .absorb(event, &Json)
            .expect("absorption succeeds");
    }

    assert_eq!((by_nomination.0.seen, by_nomination.1.seen), (1, 1));
    assert_ne!(
        (by_arrival.0.seen, by_arrival.1.seen),
        (by_nomination.0.seen, by_nomination.1.seen),
        "routing by arrival must fail the same assertion nomination passes"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — the seal holds for tuples
// ---------------------------------------------------------------------------

#[test]
fn the_seal_holds_for_tuples() {
    // Only public paths are nameable here. `use happenstance::sealed::Sealed;`
    // is `error[E0603]: module `sealed` is private`, and
    // `impl happenstance::Boundary for MyType` is `error[E0277]: the trait
    // bound `MyType: happenstance::sealed::Sealed` is not satisfied` — both
    // recorded as prose rather than as a fence, because rustdoc collects
    // doctests from the lib target only and this file is not it.
    fn only_a_boundary_composes<B: Boundary>(_boundary: &B) {}

    let pair = (
        Tally::scoped("course", "c1"),
        Tally::scoped("student", "s1"),
    );
    only_a_boundary_composes(&pair);

    // Nesting works too, and is the answer above arity 8.
    let nested = (pair, Tally::scoped("term", "t1"));
    assert_eq!(
        nested
            .query()
            .expect("a nested tuple derives")
            .items()
            .map_or(0, <[_]>::len),
        3
    );
}

// ---------------------------------------------------------------------------
// AC-008 — composing is pure, and a clone re-folds identically
// ---------------------------------------------------------------------------

#[test]
fn composing_is_pure_and_a_clone_refolds_identically() {
    let pristine = (
        Tally::scoped("course", "c1"),
        Tally::scoped("student", "s1"),
    );
    let events = [
        read_event(1, &Enrolment::Defined { capacity: 2 }, &[("course", "c1")]),
        read_event(
            2,
            &Enrolment::Subscribed {
                student: "s1".into(),
            },
            &[("course", "c1"), ("student", "s1")],
        ),
    ];

    let mut folded = pristine.clone();
    for event in &events {
        folded.absorb(event, &Json).expect("absorption succeeds");
    }

    // The clone taken first is untouched — this is what a `ConditionViolated`
    // retry re-folds from.
    assert_eq!(pristine.0.seen, 0);
    assert_eq!(pristine.1.seen, 0);
    assert_eq!(
        pristine.query().expect("derives").items(),
        folded.query().expect("derives").items(),
        "folding changes state, never the boundary"
    );

    // And a re-fold from the pristine clone reproduces the first fold exactly.
    let mut again = pristine.clone();
    for event in &events {
        again.absorb(event, &Json).expect("absorption succeeds");
    }
    assert_eq!(again, folded);

    // No store was constructed anywhere in this test.
}

#[test]
fn the_composed_query_is_inspectable() {
    let pair = (
        Tally::scoped("course", "c1"),
        Tally::scoped("student", "s1"),
    );

    // A value the caller binds, prints and asserts on — not private machinery
    // inside a read.
    let query = pair.query().expect("the composite derives");
    let rendered = format!("{query:?}");

    assert!(!query.is_all());
    assert!(rendered.contains("CourseDefined"));
    assert!(rendered.contains("StudentSubscribed"));
    assert_eq!(query.items().map_or(0, <[_]>::len), 2);
}
