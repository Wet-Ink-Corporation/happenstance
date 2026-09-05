// The module documentation is `src/overview.md`, included rather than written
// here so the reader who is sent to it and the reader who runs `cargo doc` meet
// the same bytes by construction. Same mechanism, and the same argument, as
// `crates/happenstance/src/lib.rs:10`.
#![doc = include_str!("overview.md")]
#![allow(clippy::print_stdout)]

use core::num::NonZeroU32;

use anyhow::{Result, bail};
use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, CommandError, DecisionModel, DomainEvent, EventType, InvalidTag,
    MemoryEventStore, MemoryStoreError, Retry, Tag, Tags, commit,
};
use serde::{Deserialize, Serialize};

/// How many times a command may be attempted before it gives up.
///
/// Spelled at every call site rather than defaulted inside the loop, because a
/// loop whose only exit is success is a hang with better manners. This run is
/// uncontended — one process, one writer — so every commit reports a single
/// attempt and the bound is never spent: a visible bound on an unexercised
/// path. Built at compile time: `attempts` is `const fn`, so there is no
/// fallible conversion to spell at the call site.
const ATTEMPTS: NonZeroU32 = NonZeroU32::new(3).unwrap();

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let store = MemoryEventStore::new();

    println!("== defining course c1 with capacity 2 ==");
    define_course(&store, "c1", 2).await?;
    println!("   defined");

    println!("\n== defining course c1 again ==");
    match define_course(&store, "c1", 5).await {
        Ok(()) => bail!("a duplicate course definition should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== subscribing s1 and s2 ==");
    subscribe(&store, "c1", "s1").await?;
    subscribe(&store, "c1", "s2").await?;
    println!("   both subscribed");

    println!("\n== subscribing s1 again ==");
    match subscribe(&store, "c1", "s1").await {
        Ok(()) => bail!("a duplicate subscription should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== subscribing s3, which would exceed capacity ==");
    match subscribe(&store, "c1", "s3").await {
        Ok(()) => bail!("exceeding capacity should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== s1 unsubscribes, freeing a seat ==");
    unsubscribe(&store, "c1", "s1").await?;
    subscribe(&store, "c1", "s3").await?;
    println!("   s3 subscribed into the freed seat");

    println!("\n== final log ==");
    for event in store.snapshot() {
        println!(
            "   {:>3}  {:<22} {:?}",
            event.position.get(),
            event.event_type().as_str(),
            event.tags()
        );
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Identity, validated once at the edge
// ---------------------------------------------------------------------------

/// A course identifier, validated once into the tag its events carry.
///
/// `DomainEvent::tags` is infallible and `Tags` has no infallible constructor —
/// `Tag::key_value` is the only way in and it can refuse. So a domain type
/// whose tags come from runtime values has to *hold* the validated form rather
/// than rebuild it on every call, and the `?` is paid here, at the edge, where
/// the handler is already writing one. That is the resolution `DecisionModel`
/// makes for a model's own scope, applied to the event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct CourseId {
    id: String,
    tag: Tag,
}

impl CourseId {
    /// Validates `id` into the tag every event for this course is written with.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("course", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for CourseId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<CourseId> for String {
    fn from(course: CourseId) -> Self {
        course.id
    }
}

/// A student identifier, validated the same way and for the same reason.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
struct StudentId {
    id: String,
    tag: Tag,
}

impl StudentId {
    /// Validates `id` into the tag every event for this student carries.
    fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("student", id)?,
        })
    }

    /// The identifier as the application wrote it.
    fn as_str(&self) -> &str {
        &self.id
    }
}

impl TryFrom<String> for StudentId {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<StudentId> for String {
    fn from(student: StudentId) -> Self {
        student.id
    }
}

// ---------------------------------------------------------------------------
// The domain: one enum, named once
// ---------------------------------------------------------------------------

/// Everything that can happen to an enrolment.
///
/// One enum, so a query and a fold cannot name different sets: the query is
/// derived from `EVENT_TYPES` below plus the model's scope, and every fold is a
/// `match` over these variants with no wildcard arm. A fourth variant is a
/// compile error in every decision model that has not said what it means.
#[derive(Debug, Serialize, Deserialize)]
enum Enrolment {
    /// A course exists, and how many seats it has.
    CourseDefined { course: CourseId, capacity: u32 },
    /// A student holds one of those seats.
    StudentSubscribed {
        course: CourseId,
        student: StudentId,
    },
    /// A student gave one back.
    StudentUnsubscribed {
        course: CourseId,
        student: StudentId,
    },
}

impl DomainEvent for Enrolment {
    const EVENT_TYPES: &'static [EventType] = &[
        EventType::from_static("CourseDefined"),
        EventType::from_static("StudentSubscribed"),
        EventType::from_static("StudentUnsubscribed"),
    ];

    fn event_type(&self) -> EventType {
        match self {
            Self::CourseDefined { .. } => Self::EVENT_TYPES[0].clone(),
            Self::StudentSubscribed { .. } => Self::EVENT_TYPES[1].clone(),
            Self::StudentUnsubscribed { .. } => Self::EVENT_TYPES[2].clone(),
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::CourseDefined { course, .. } => [course.tag.clone()].into_iter().collect(),
            Self::StudentSubscribed { course, student }
            | Self::StudentUnsubscribed { course, student } => {
                [course.tag.clone(), student.tag.clone()]
                    .into_iter()
                    .collect()
            }
        }
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

/// Why a decision refused.
///
/// Every variant carries the value the transcript prints — `course c1 is full
/// (2/2)`, never `capacity exceeded` — so a reader can act on the refusal where
/// it is printed instead of opening a second document to learn what happened.
#[derive(Debug, thiserror::Error)]
enum Refusal {
    /// Invariant 1: a course may not be defined twice.
    #[error("course {course} is already defined")]
    AlreadyDefined {
        /// The course that already exists.
        course: String,
    },
    /// A course has to exist before anyone can hold a seat in it.
    #[error("course {course} does not exist")]
    NotDefined {
        /// The course nobody has defined.
        course: String,
    },
    /// Invariant 3: a student may not subscribe to the same course twice.
    #[error("student {student} is already subscribed to {course}")]
    AlreadySubscribed {
        /// The student already holding a seat.
        student: String,
        /// The course they hold it in.
        course: String,
    },
    /// Invariant 2: a course may not exceed its capacity.
    #[error("course {course} is full ({taken}/{capacity})")]
    Full {
        /// The full course.
        course: String,
        /// Seats currently held.
        taken: u32,
        /// Seats the course was defined with.
        capacity: u32,
    },
    /// A seat cannot be released by someone who is not holding one.
    #[error("student {student} is not subscribed to {course}")]
    NotSubscribed {
        /// The student who holds no seat.
        student: String,
        /// The course they are not in.
        course: String,
    },
}

// ---------------------------------------------------------------------------
// The decision models: one per consistency concern
// ---------------------------------------------------------------------------

/// Whether this course has been defined.
///
/// `define_course`'s whole boundary. The query it derives is every event tagged
/// with this course — the smallest set that could invalidate the decision.
#[derive(Debug, Clone)]
struct CourseDefinition {
    scope: Tags,
    defined: bool,
}

impl CourseDefinition {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(course: &CourseId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("course", course.as_str())])?,
            defined: false,
        })
    }
}

impl DecisionModel for CourseDefinition {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::CourseDefined { .. } => self.defined = true,
            Enrolment::StudentSubscribed { .. } | Enrolment::StudentUnsubscribed { .. } => {}
        }
    }
}

/// The course's capacity, and everyone currently holding a seat.
///
/// The capacity arrives by *decoding* the `CourseDefined` variant, not by
/// scraping the payload's bytes: a course with no capacity is a decode failure
/// naming its position, never a course with capacity zero.
#[derive(Debug, Clone)]
struct Seats {
    scope: Tags,
    capacity: Option<u32>,
    taken: u32,
}

impl Seats {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(course: &CourseId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("course", course.as_str())])?,
            capacity: None,
            taken: 0,
        })
    }
}

impl DecisionModel for Seats {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::CourseDefined { capacity, .. } => self.capacity = Some(capacity),
            Enrolment::StudentSubscribed { .. } => self.taken += 1,
            Enrolment::StudentUnsubscribed { .. } => self.taken = self.taken.saturating_sub(1),
        }
    }
}

/// One student's own history with one course.
///
/// The second consistency concern in `subscribe`, and the whole of
/// `unsubscribe` — which is the demonstration that a decision model is a
/// reusable value rather than a per-handler blob. Its scope carries both tags,
/// so the query it derives nominates this student's events and nobody else's.
#[derive(Debug, Clone)]
struct StudentSeat {
    scope: Tags,
    subscribed: bool,
}

impl StudentSeat {
    /// Validates the scope once, here, where the caller already writes a `?`.
    fn new(course: &CourseId, student: &StudentId) -> Result<Self, InvalidTag> {
        Ok(Self {
            scope: Tags::from_pairs([("course", course.as_str()), ("student", student.as_str())])?,
            subscribed: false,
        })
    }
}

impl DecisionModel for StudentSeat {
    type Event = Enrolment;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            Enrolment::StudentSubscribed { .. } => self.subscribed = true,
            Enrolment::StudentUnsubscribed { .. } => self.subscribed = false,
            // A course definition carries no `student` tag, so this model's
            // query never nominates one. The arm exists because the fold is
            // exhaustive over the enum, and that exhaustiveness is the whole
            // guarantee.
            Enrolment::CourseDefined { .. } => {}
        }
    }
}

// ---------------------------------------------------------------------------
// The three command handlers
// ---------------------------------------------------------------------------

/// Defines a course, rejecting a second definition of the same one.
async fn define_course(store: &MemoryEventStore, course: &str, capacity: u32) -> Result<()> {
    let course = CourseId::new(course)?;
    let boundary = CourseDefinition::new(&course)?;

    commit(
        store,
        boundary,
        Retry::attempts(ATTEMPTS),
        |defined: &CourseDefinition| {
            if defined.defined {
                return Err(Refusal::AlreadyDefined {
                    course: course.id.clone(),
                });
            }
            Ok(vec![Enrolment::CourseDefined {
                course: course.clone(),
                capacity,
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Subscribes a student, enforcing capacity and no-double-subscription together.
///
/// This is the case that motivates DCB, and it is the case that motivates
/// composition. *The course's capacity and everyone holding a seat* is one
/// consistency concern; *this student's own history* is another. The tuple
/// below is both of them, OR'd into one query and checked by one append
/// condition — with no macro invocation and no builder in the way. A tuple of
/// decision models is a boundary.
async fn subscribe(store: &MemoryEventStore, course: &str, student: &str) -> Result<()> {
    let course = CourseId::new(course)?;
    let student = StudentId::new(student)?;
    let boundary = (Seats::new(&course)?, StudentSeat::new(&course, &student)?);

    commit(
        store,
        boundary,
        Retry::attempts(ATTEMPTS),
        |(seats, seat): &(Seats, StudentSeat)| {
            let Some(capacity) = seats.capacity else {
                return Err(Refusal::NotDefined {
                    course: course.id.clone(),
                });
            };
            if seat.subscribed {
                return Err(Refusal::AlreadySubscribed {
                    student: student.id.clone(),
                    course: course.id.clone(),
                });
            }
            if seats.taken >= capacity {
                return Err(Refusal::Full {
                    course: course.id.clone(),
                    taken: seats.taken,
                    capacity,
                });
            }
            Ok(vec![Enrolment::StudentSubscribed {
                course: course.clone(),
                student: student.clone(),
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Releases a student's seat.
async fn unsubscribe(store: &MemoryEventStore, course: &str, student: &str) -> Result<()> {
    let course = CourseId::new(course)?;
    let student = StudentId::new(student)?;
    let boundary = StudentSeat::new(&course, &student)?;

    commit(
        store,
        boundary,
        Retry::attempts(ATTEMPTS),
        |seat: &StudentSeat| {
            if !seat.subscribed {
                return Err(Refusal::NotSubscribed {
                    student: student.id.clone(),
                    course: course.id.clone(),
                });
            }
            Ok(vec![Enrolment::StudentUnsubscribed {
                course: course.clone(),
                student: student.clone(),
            }])
        },
    )
    .await
    .map(|_| ())
    .map_err(rejected)
}

/// Renders a command failure as the message the transcript prints.
///
/// `CommandError::Refused` carries the handler's *own* refusal, and that is
/// what a reader must see: `CommandError`'s own `Display` says "the decision
/// refused", which is a category rather than a value. Every other variant is a
/// real failure and travels out with its source chain intact.
fn rejected(err: CommandError<MemoryStoreError, Refusal>) -> anyhow::Error {
    match err {
        CommandError::Refused(refusal) => refusal.into(),
        other => other.into(),
    }
}
