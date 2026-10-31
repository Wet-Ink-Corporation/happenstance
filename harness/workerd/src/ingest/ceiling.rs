//! The ingest ceiling: the largest single `ingest` call the runtime commits,
//! per event shape, below the adapter's declared per-call ceiling.
//!
//! The adapter refuses a call over its declared 1,024 events **before** any
//! SQL, so an attempt through it can only rediscover that declaration at the
//! top. What is open is below it: does the real runtime commit every call the
//! adapter accepts, inside one `transactionSync`, at the shapes the per-event
//! ceilings allow?
//!
//! The search is **not** here. Each attempt is one request on its own object,
//! and the runner (`ingest-ceiling.mjs`) bisects, because a shape near the top
//! can exhaust memory or CPU, which traps or resets the object rather than
//! answering `Err`: an in-object bisection would die with its first trap.

use core::fmt;
use core::str::FromStr;

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

/// The bytes of a minimal or a maximally tagged event.
const SMALL_DATA_LEN: usize = 8;

/// The ceiling events' type.
const CEILING_EVENT: EventType = EventType::from_static("IngestCeiling");

/// What each event of an attempt looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Eight bytes of data and one tag.
    Minimal,
    /// Eight bytes of data and the declared maximum of distinct tags.
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
    /// A tag the shape builds was refused; the refusal's text.
    Tag(String),
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
            Self::Tag(error) => write!(f, "a ceiling tag: {error}"),
            Self::Event(error) => write!(f, "a ceiling event: {error}"),
        }
    }
}

impl core::error::Error for ShapeError {}

/// `events` foreign events from [`CEILING_ORIGIN`] at origin positions
/// `1..=events`, all of `shape`, for one group with no compensation.
///
/// The data is one buffer shared by every event (`Bytes` clones are reference
/// bumps), so the harness holds one copy of it; the tags are each event's own.
///
/// # Errors
///
/// [`ShapeError`]: zero events, more than [`MAX_EVENTS_PER_CALL`], or a shape
/// the fixture declares no ceiling for. Checked before anything is allocated.
pub fn foreign_batch(shape: Shape, events: usize) -> Result<Vec<ReplicatedEvent>, ShapeError> {
    if events == 0 {
        return Err(ShapeError::Position);
    }
    if events > MAX_EVENTS_PER_CALL {
        return Err(ShapeError::TooMany { events });
    }
    let (data_len, tag_count) = shape.dimensions()?;
    let data = Bytes::from(vec![0xE3_u8; data_len]);
    let tags = (0..tag_count)
        .map(|n| Tag::new(format!("ceiling:{n}")))
        .collect::<Result<Tags, _>>()
        .map_err(|error| ShapeError::Tag(error.to_string()))?;
    let template = Event::new(CEILING_EVENT, data)
        .map_err(|error| ShapeError::Event(error.to_string()))?
        .with_tags(tags);
    (1..=events)
        .map(|n| {
            let position = u64::try_from(n)
                .ok()
                .and_then(SequencePosition::new)
                .ok_or(ShapeError::Position)?;
            // Each event owns its tags, so this duplicates them (H-08); the
            // data inside is a shared `Bytes`.
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
