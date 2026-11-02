//! The ingest ceiling: the largest single `ingest` call the runtime commits,
//! per event shape, below the adapter's declared per-call ceiling.
//!
//! The adapter refuses a call over its declared 1,024 events or 262,144 tags
//! **before** any SQL, so an attempt through it can only rediscover those
//! declarations at the top. What is open is below it: does the real runtime
//! commit every call the adapter accepts, inside one `transactionSync`, at the
//! shapes the per-event ceilings allow?
//!
//! The search is **not** here. Each attempt is one request on its own object,
//! and the runner (`ingest-ceiling.mjs`) bisects, because a shape near the top
//! can exhaust memory or CPU, which traps or resets the object rather than
//! answering `Err`: an in-object bisection would die with its first trap.

use core::fmt;
use core::str::FromStr;
use std::sync::LazyLock;

use happenstance_cloudflare::CloudflareIngestStore;
use happenstance_cloudflare::happenstance_sync::{
    Classify, FailureClass, IngestGroup, IngestStore, ReplicatedEvent,
};
use happenstance_core::bytes::Bytes;
use happenstance_core::{
    Event, EventId, EventType, RecordedAt, SequencePosition, StoreId, Tag, Tags,
};
use happenstance_testkit::Fixture;

use crate::fixture::WorkerdFixture;

/// The origin every ceiling event claims, distinct from every rule's.
pub const CEILING_ORIGIN: StoreId = StoreId::from_bytes([0xE3; 16]);

/// The most events one attempt may ask for: one past the adapter's declared
/// batch ceiling, so the top of a search reaches the adapter's own refusal,
/// and no further, so a request cannot ask for unbounded memory (H-13).
pub const MAX_EVENTS_PER_CALL: usize =
    match <WorkerdFixture<'static> as Fixture>::MAX_EVENTS_PER_BATCH {
        Some(ceiling) => ceiling + 1,
        None => 0,
    };

/// The most `max-tags` events one call carries before the adapter refuses it
/// for its total tags: the declared tags-per-batch ceiling over the declared
/// tags per event, 262,144 / 1,024 = 256 (ADR-0100). Falls back to
/// [`MAX_EVENTS_PER_CALL`] when either is undeclared, as then no total wall
/// is the adapter's.
pub const MAX_TAGS_EVENTS_AT_CEILING: usize = match (
    <WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_BATCH,
    <WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_EVENT,
) {
    (Some(per_batch), Some(per_event)) => match per_batch.checked_div(per_event) {
        Some(events) => events,
        None => MAX_EVENTS_PER_CALL,
    },
    _ => MAX_EVENTS_PER_CALL,
};

/// The bytes of a minimal or a maximally tagged event.
const SMALL_DATA_LEN: usize = 8;

/// The ceiling events' type.
const CEILING_EVENT: EventType = EventType::from_static("IngestCeiling");

/// The names of the ceiling's tags, `ceiling:0` up to the declared maximum
/// per event, and at least one for the one-tag shapes, built once per isolate
/// and never written again.
///
/// Why a static (OWN-11): a [`Tag`] borrows only `'static` text, and every
/// event of every attempt names the same tags, so the names have to outlive
/// any one attempt and be shared by all of them; each event's tags borrow
/// them rather than own a copy. This is immutable after its one
/// initialisation, under 40 KB, and bounded by the fixture's declaration
/// rather than by a request. Leaking a per-attempt buffer to get `'static` is
/// forbidden (H-07), and a `const` table would not follow the declaration.
static CEILING_TAG_NAMES: LazyLock<Box<[String]>> = LazyLock::new(|| {
    let declared = <WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_EVENT.unwrap_or(0);
    (0..declared.max(1))
        .map(|n| format!("ceiling:{n}"))
        .collect()
});

/// What each event of an attempt looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Eight bytes of data and one tag.
    Minimal,
    /// Eight bytes of data and the declared maximum of distinct tags.
    ///
    /// Its wall is the adapter's own refusal at
    /// [`MAX_TAGS_EVENTS_AT_CEILING`]` + 1` events, as `TagsPerBatch`, below
    /// the deployed object's CPU and storage-time resets (ADR-0100).
    MaxTags,
    /// The declared maximum of data and one tag.
    MaxData,
}

impl Shape {
    /// Every shape, in the order a report prints them.
    pub const ALL: [Self; 3] = [Self::Minimal, Self::MaxTags, Self::MaxData];

    /// The name a route and a report use.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::MaxTags => "max-tags",
            Self::MaxData => "max-data",
        }
    }

    /// `(data bytes, tags)` of each event of this shape, at the ceilings the
    /// adapter declares.
    fn dimensions(self) -> Result<(usize, usize), ShapeError> {
        let declared = |ceiling: Option<usize>| ceiling.ok_or(ShapeError::Undeclared);
        match self {
            Self::Minimal => Ok((SMALL_DATA_LEN, 1)),
            Self::MaxTags => Ok((
                SMALL_DATA_LEN,
                declared(<WorkerdFixture<'static> as Fixture>::MAX_TAGS_PER_EVENT)?,
            )),
            Self::MaxData => Ok((
                declared(<WorkerdFixture<'static> as Fixture>::MAX_EVENT_DATA_LEN)?,
                1,
            )),
        }
    }
}

impl FromStr for Shape {
    type Err = ShapeError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|shape| shape.name() == name)
            .ok_or_else(|| ShapeError::Unknown(name.to_owned()))
    }
}

/// Why an attempt's batch could not be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShapeError {
    /// No shape has this name.
    Unknown(String),
    /// Zero events: an origin position is never zero.
    Position,
    /// More events than [`MAX_EVENTS_PER_CALL`].
    TooMany {
        /// How many were asked for.
        events: usize,
    },
    /// The adapter's fixture declares no ceiling for this shape's dimension.
    Undeclared,
    /// The shape asks for more tags than there are names built.
    Tag {
        /// Tags the shape asked for.
        asked: usize,
        /// Names built, from the fixture's declaration.
        built: usize,
    },
    /// The event the shape builds was refused; the refusal's text.
    Event(String),
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(name) => write!(
                f,
                "no ingest ceiling shape {name}; known: {}",
                Shape::ALL.map(Shape::name).join(", ")
            ),
            Self::Position => f.write_str("an attempt needs at least one event"),
            Self::TooMany { events } => write!(
                f,
                "{events} events is past the {MAX_EVENTS_PER_CALL} an attempt may ask for"
            ),
            Self::Undeclared => f.write_str("the fixture declares no ceiling for this shape"),
            Self::Tag { asked, built } => {
                write!(f, "{asked} ceiling tags is past the {built} names built")
            }
            Self::Event(error) => write!(f, "a ceiling event: {error}"),
        }
    }
}

impl core::error::Error for ShapeError {}

/// `events` foreign events from [`CEILING_ORIGIN`] at origin positions
/// `1..=events`, all of `shape`, for one group with no compensation.
///
/// The data is one buffer shared by every event (`Bytes` clones are reference
/// bumps), so the harness holds one copy of it. The tags borrow
/// one static set of names, so the names exist once however many events and
/// attempts name them; each event owns only its slice of borrowed tags.
///
/// # Errors
///
/// [`ShapeError`]: zero events, more than [`MAX_EVENTS_PER_CALL`], a shape
/// the fixture declares no ceiling for, or more tags than names built
/// ([`ShapeError::Tag`]). Checked before any event is allocated.
pub fn foreign_batch(shape: Shape, events: usize) -> Result<Vec<ReplicatedEvent>, ShapeError> {
    if events == 0 {
        return Err(ShapeError::Position);
    }
    if events > MAX_EVENTS_PER_CALL {
        return Err(ShapeError::TooMany { events });
    }
    let (data_len, tag_count) = shape.dimensions()?;
    let data = Bytes::from(vec![0xE3_u8; data_len]);
    let names = CEILING_TAG_NAMES
        .get(..tag_count)
        .ok_or_else(|| ShapeError::Tag {
            asked: tag_count,
            built: CEILING_TAG_NAMES.len(),
        })?;
    // Borrowed, so a clone of these tags copies pointers, not names. Each name
    // is `ceiling:` and a decimal, which `from_static`'s checks accept: the
    // panic it documents cannot be reached from text this module writes.
    let tags: Tags = names.iter().map(|name| Tag::from_static(name)).collect();
    let template = Event::new(CEILING_EVENT, data)
        .map_err(|error| ShapeError::Event(error.to_string()))?
        .with_tags(tags);
    (1..=events)
        .map(|n| {
            let position = u64::try_from(n)
                .ok()
                .and_then(SequencePosition::new)
                .ok_or(ShapeError::Position)?;
            // Each event owns its tag slice, so this duplicates it (H-08):
            // one allocation of borrowed tags, no names. The data inside is a
            // shared `Bytes`.
            Ok(ReplicatedEvent::new(
                EventId::new(CEILING_ORIGIN, position),
                RecordedAt::from_millis(1_800_000_000_000),
                template.clone(),
            ))
        })
        .collect()
}

/// What one attempt came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attempt {
    /// The call committed.
    Accepted {
        /// Events sent.
        events: usize,
        /// Events the store reports appended.
        appended: usize,
    },
    /// The call was refused.
    Refused {
        /// The class the adapter gave the refusal.
        class: FailureClass,
        /// The refusal, verbatim.
        error: String,
    },
}

impl Attempt {
    /// The response body: `accepted events=<n> appended=<m>`, or
    /// `refused class=<Class>` and the error on the next line.
    #[must_use]
    pub fn line(&self) -> String {
        match self {
            Self::Accepted { events, appended } => {
                format!("accepted events={events} appended={appended}")
            }
            Self::Refused { class, error } => format!("refused class={class:?}\n{error}"),
        }
    }
}

/// Ingests `batch` as one group through `store`, and says what came of it.
pub async fn attempt(store: &CloudflareIngestStore, batch: &[ReplicatedEvent]) -> Attempt {
    match store.ingest(&[IngestGroup::new(batch, &[])]).await {
        Ok(ingested) => Attempt::Accepted {
            events: batch.len(),
            appended: ingested.appended,
        },
        Err(error) => Attempt::Refused {
            class: error.class(),
            error: error.to_string(),
        },
    }
}

#[cfg(test)]
mod tests;
