//! The worked example's domain, plus one new fact the fold has not been taught.
//!
//! A mirror of `examples/course-subscriptions/src/main.rs:194-207` (the domain
//! enum) and `:341-379` (the `Seats` decision model and its fold). It restates
//! them rather than importing them, for two mechanical reasons: a variant
//! cannot be added to an imported enum, and the example is a binary crate with
//! no lib target to import from in any case. The mirror is the thing most
//! likely to drift, so it is named here — the *guarantee* being pinned,
//! exhaustiveness of a fold over `Self::Event`, stays true under drift.
//!
//! Two simplifications, both there to keep this file's failure surface at
//! exactly one error: the variants carry only what the fold reads, and `tags`
//! returns the empty set. The example's own tags come from validated identity
//! newtypes (`src/main.rs:112-181`), and restating those here would add
//! imports, serde attributes and conversions to a file whose entire value is
//! that the first thing rustc prints is the missing arm.
//!
//! `event_type` below **is** extended for the new variant, and only `apply` is
//! not. That is deliberate: `EVENT_TYPES` and `event_type` agreeing is not a
//! property the compiler checks, so leaving that arm out too would print a
//! second error and bury the one this fixture exists to pin.

use happenstance::bytes::Bytes;
use happenstance::{Codec, CodecError, DecisionModel, DomainEvent, EventType, Tags};
use serde::{Deserialize, Serialize};

/// Everything that can happen to an enrolment — plus `CourseClosed`, which is
/// the fact the application has just grown.
#[derive(Debug, Serialize, Deserialize)]
enum Enrolment {
    CourseDefined { capacity: u32 },
    StudentSubscribed,
    StudentUnsubscribed,
    CourseClosed,
}

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("CourseDefined"),
        EventType::from_static("StudentSubscribed"),
        EventType::from_static("StudentUnsubscribed"),
        EventType::from_static("CourseClosed"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::CourseDefined { .. } => Self::EVENT_TYPES[0].clone(),
            Self::StudentSubscribed => Self::EVENT_TYPES[1].clone(),
            Self::StudentUnsubscribed => Self::EVENT_TYPES[2].clone(),
            Self::CourseClosed => Self::EVENT_TYPES[3].clone(),
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
        _event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

/// The course's capacity, and everyone currently holding a seat.
#[derive(Debug, Clone)]
struct Seats {
    scope: Tags,
    capacity: Option<u32>,
    taken: u32,
}

impl DecisionModel for Seats {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    // No `_ =>` arm. That absence *is* the protection this fixture pins.
    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::CourseDefined { capacity } => {
                self.capacity = Some(capacity);
            }
            Enrolment::StudentSubscribed => self.taken += 1,
            Enrolment::StudentUnsubscribed => {
                self.taken = self.taken.saturating_sub(1);
            }
            Enrolment::CourseClosed => self.capacity = None,
        }
    }
}

fn main() {
    let mut seats = Seats {
        scope: Tags::empty(),
        capacity: None,
        taken: 0,
    };

    for event in [
        Enrolment::CourseClosed,
        Enrolment::CourseDefined { capacity: 2 },
        Enrolment::StudentSubscribed,
        Enrolment::StudentUnsubscribed,
    ] {
        seats.apply(event);
    }

    assert_eq!(seats.capacity, Some(2));
    assert_eq!(seats.taken, 0);
}
