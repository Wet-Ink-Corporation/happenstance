//! VT-18's two compile tests: a constructor accepts a value the caller already
//! holds, and the three validation errors compose.
//!
//! Both are artefacts of a **signature**. Each is discharged the moment this
//! target builds, and the `#[test]` bodies exist so that a reader can see what
//! is being claimed and so the arrangement cannot rot into one that compiles
//! while meaning nothing — the same posture `frozen_signatures.rs` takes, and
//! for the same reason.
//!
//! Each test names the wrong implementation it rejects, because a compile test
//! that no wrong implementation fails is decorative. Both wrong implementations
//! are the tree's own history: VT-18 was written against them and phase 4
//! discharged it.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

use happenstance_core::{
    Event, EventType, InvalidEventType, InvalidQuery, InvalidTag, Query, QueryItem, Tags,
};

// =====================================================================
// VT-18, first half — `Event::new` accepts an already-built `EventType`
// =====================================================================

/// The interned type a codec registry holds: one [`EventType`] per domain
/// event, validated at compile time and reused for every instance of it.
///
/// A free `const` rather than an associated one deliberately — `EventType::from_static`
/// records why: an associated const is evaluated lazily, so an invalid one that
/// nothing reads survives `check`, `clippy`, `build` and `test`.
const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");

/// The typed layer's encoder, in miniature: generic over what the caller holds,
/// and forwarding [`Event::new`]'s own bounds rather than restating a narrower
/// pair.
///
/// This is the case interning exists for, and the one a pair of concrete call
/// sites does not cover. Here `T` is a *type parameter*, so the body is compiled
/// once against the bound alone and both instantiations must satisfy the same
/// signature; a codec written this way never sees a string at all.
fn encode<T>(event_type: T, data: &'static [u8]) -> Result<Event, InvalidEventType>
where
    T: TryInto<EventType>,
    InvalidEventType: From<T::Error>,
{
    Event::new(event_type, data)
}

/// **Rejects:** `Event::new<T: TryInto<EventType, Error = InvalidEventType>>` —
/// the equality-constrained bound VT-18 was written against.
///
/// The blanket `impl<T, U: From<T>> TryFrom<T> for U` gives an already-built
/// `EventType` the conversion error [`Infallible`](core::convert::Infallible),
/// and `Infallible` is not `InvalidEventType`, so an equality constraint
/// excludes the one conversion that cannot fail. Every line below that passes
/// `COURSE_DEFINED` then fails with `error[E0271]: type mismatch resolving
/// <EventType as TryInto<EventType>>::Error == InvalidEventType`.
///
/// The `&str` call sites are not padding: they are what makes narrowing the
/// bound *back* to the equality form a build failure rather than a trade. One
/// argument type would be satisfiable either way.
#[test]
fn event_new_accepts_a_held_event_type() {
    let from_held = Event::new(COURSE_DEFINED, &b"{}"[..]).unwrap();
    let from_str = Event::new("CourseDefined", &b"{}"[..]).unwrap();

    assert_eq!(from_held.event_type(), from_str.event_type());

    // The same two conversions through one generic body.
    assert_eq!(
        encode(COURSE_DEFINED, b"{}").unwrap().event_type(),
        &COURSE_DEFINED
    );
    assert_eq!(
        encode("CourseDefined", b"{}").unwrap().event_type(),
        &COURSE_DEFINED
    );

    // The fallible half still refuses: accepting a held value must not cost the
    // validation a string gets.
    assert_eq!(
        Event::new("", &b"{}"[..]).unwrap_err(),
        InvalidEventType::Empty
    );
}

// =====================================================================
// VT-18, second half — the three validation errors compose under one `?`
// =====================================================================

/// A command handler in a library crate, which is the caller VT-18 is about.
///
/// The **return type is the test**. `Tags::from_pairs` fails with `InvalidTag`
/// while `QueryItem::new` and `Query::from_items` fail with `InvalidQuery`, so
/// the bare `?` on the first line compiles only because
/// `InvalidQuery: From<InvalidTag>` exists. There is no union enum here and no
/// `Box<dyn core::error::Error>` — the latter is how this crate's own doctests
/// escape the problem, and it is exactly what would hide the defect. `anyhow`
/// is not available: the house style forbids it in library code, which is the
/// situation a downstream command handler is in too.
fn subscription_decision_model(course: &str, student: &str) -> Result<Query, InvalidQuery> {
    let by_course = Tags::from_pairs([("course", course)])?;
    let by_student = Tags::from_pairs([("student", student)])?;

    let capacity = QueryItem::new(["CourseDefined", "CourseCapacityChanged"], by_course)?;
    let enrolment = QueryItem::tagged(by_student)?;

    Query::from_items([capacity, enrolment])
}

/// **Rejects:** an `InvalidQuery` without `Tag(#[from] InvalidTag)`.
///
/// Strike that variant and [`subscription_decision_model`] stops compiling with
/// `error[E0277]: ?` couldn't convert the error to `InvalidQuery`, which is the
/// diagnostic every downstream handler used to get before writing a line of
/// domain logic.
#[test]
fn command_handler_composes_validation_errors() {
    let query = subscription_decision_model("c1", "s1").unwrap();
    assert_eq!(query.items().unwrap().len(), 2);

    // Composition keeps the *cause*, which is what collapsing the three enums
    // into one `InvalidInput` would have cost: the handler can still tell which
    // of the three constructors refused, and why.
    let refused = subscription_decision_model("c1", "").unwrap_err();
    assert!(
        matches!(refused, InvalidQuery::Tag(InvalidTag::Empty)),
        "an invalid tag must arrive as `InvalidQuery::Tag`, got {refused:?}"
    );
}
