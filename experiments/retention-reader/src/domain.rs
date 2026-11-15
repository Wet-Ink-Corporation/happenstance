//! The domain the scenarios write, and the two readers built on it.
//!
//! Claims are registered and then noted. [`Counter`] is the projection
//! (reader R1): it counts how many times the runner handed it each
//! [`EventId`], because counting rows would hide an event applied twice.
//! [`ClaimFile`] is the decision model (reader R3): it folds whether a claim's
//! history registers it, and R3 stages "register the claim unless it is
//! registered" over it, a decision that refuses on the *presence* of history
//! and so admits more when history is missing (ES-40).

use core::fmt;
use std::collections::BTreeMap;

use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, DecisionModel, Delivered, DomainEvent, Event, EventId, EventType,
    InvalidEventType, InvalidProjectionId, InvalidTag, Json, MemoryProjectionBatch,
    MemoryProjectionStore, MemoryProjectionStoreError, Projection, ProjectionId, Tag, Tags,
};
use serde::{Deserialize, Serialize};

/// A claim identifier, validated once into the tag its events carry.
///
/// `DomainEvent::tags` is infallible and `Tag::key_value` can refuse, so the
/// validated tag is held rather than rebuilt on every call (the shape
/// `examples/transfers-on-sqlite` uses).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Claim {
    id: String,
    tag: Tag,
}

impl Claim {
    /// Validates `id` into the `claim:<id>` tag.
    ///
    /// # Errors
    ///
    /// [`InvalidTag`] if `id` cannot be a tag value.
    pub fn new(id: &str) -> Result<Self, InvalidTag> {
        Ok(Self {
            id: id.to_owned(),
            tag: Tag::key_value("claim", id)?,
        })
    }

    /// The claim's events' tag, as a one-tag set: the scope a decision about
    /// this claim reads.
    #[must_use]
    pub fn scope(&self) -> Tags {
        // The set stores the tag: a cheap copy of a short string.
        [self.tag.clone()].into_iter().collect()
    }
}

impl TryFrom<String> for Claim {
    type Error = InvalidTag;

    fn try_from(id: String) -> Result<Self, InvalidTag> {
        Self::new(&id)
    }
}

impl From<Claim> for String {
    fn from(claim: Claim) -> Self {
        claim.id
    }
}

/// Everything that can happen to a claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimEvent {
    /// The claim exists and may be noted.
    Registered {
        /// The claim registered.
        claim: Claim,
    },
    /// A note was added to the claim.
    Noted {
        /// The claim noted.
        claim: Claim,
        /// Which note, so two notes are two distinct payloads.
        note: u32,
    },
}

const REGISTERED: EventType = EventType::from_static("ClaimRegistered");
const NOTED: EventType = EventType::from_static("ClaimNoted");

impl DomainEvent for ClaimEvent {
    const EVENT_TYPES: &'static [EventType] = &[REGISTERED, NOTED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Registered { .. } => REGISTERED,
            Self::Noted { .. } => NOTED,
        }
    }

    fn tags(&self) -> Tags {
        match self {
            Self::Registered { claim } | Self::Noted { claim, .. } => claim.scope(),
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

/// Why a [`ClaimEvent`] could not be written down as a port [`Event`].
#[derive(Debug)]
pub enum ToEventError {
    /// The JSON codec refused the value.
    Codec(CodecError),
    /// The event type was refused.
    EventType(InvalidEventType),
}

impl fmt::Display for ToEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Codec(_) => f.write_str("encoding the claim event failed"),
            Self::EventType(_) => f.write_str("the claim event's type was refused"),
        }
    }
}

impl core::error::Error for ToEventError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Codec(error) => Some(error),
            Self::EventType(error) => Some(error),
        }
    }
}

impl ClaimEvent {
    /// A registration of `claim`.
    #[must_use]
    pub const fn registered(claim: Claim) -> Self {
        Self::Registered { claim }
    }

    /// Note number `note` on `claim`.
    #[must_use]
    pub const fn noted(claim: Claim, note: u32) -> Self {
        Self::Noted { claim, note }
    }

    /// This event as the port carries it: JSON payload, type and tags.
    ///
    /// # Errors
    ///
    /// [`ToEventError`] if the codec or the event type refuses it.
    pub fn to_event(&self) -> Result<Event, ToEventError> {
        let data = self.encode(&Json).map_err(ToEventError::Codec)?;
        let event = Event::new(self.event_type(), data).map_err(ToEventError::EventType)?;
        Ok(event.with_tags(self.tags()))
    }
}

/// Reader R1's read model: how many times each event was applied.
///
/// The counts live in the projection as well as in its store, so a scenario
/// can ask which events were applied more than once without decoding keys.
#[derive(Debug)]
pub struct Counter {
    id: ProjectionId,
    scope: Tags,
    applied: BTreeMap<EventId, u64>,
}

impl Counter {
    /// A counter over every claim event, under the projection name `name`.
    ///
    /// # Errors
    ///
    /// [`InvalidProjectionId`] if `name` is not a valid projection id.
    pub fn new(name: &str) -> Result<Self, InvalidProjectionId> {
        Ok(Self {
            id: ProjectionId::new(name)?,
            scope: Tags::empty(),
            applied: BTreeMap::new(),
        })
    }

    /// Every event applied, with the number of times it was applied.
    #[must_use]
    pub const fn applied(&self) -> &BTreeMap<EventId, u64> {
        &self.applied
    }

    /// The events applied more than once.
    pub fn applied_twice(&self) -> impl Iterator<Item = EventId> + '_ {
        self.applied
            .iter()
            .filter(|&(_, &count)| count > 1)
            .map(|(&id, _)| id)
    }
}

impl Projection for Counter {
    type Event = ClaimEvent;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        delivered: Delivered<ClaimEvent>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Self::Error> {
        let count = self.applied.entry(delivered.id()).or_insert(0);
        *count = count.saturating_add(1);
        batch.write(delivered.id().to_string(), *count);
        Ok(())
    }
}

/// Reader R3's decision model: what one claim's history says about it.
#[derive(Debug, Clone)]
pub struct ClaimFile {
    scope: Tags,
    registered: bool,
}

impl ClaimFile {
    /// The file of `claim`, before anything is folded into it.
    #[must_use]
    pub fn of(claim: &Claim) -> Self {
        Self {
            scope: claim.scope(),
            registered: false,
        }
    }

    /// Whether the history folded in registered the claim.
    #[must_use]
    pub const fn is_registered(&self) -> bool {
        self.registered
    }
}

impl DecisionModel for ClaimFile {
    type Event = ClaimEvent;

    fn scope(&self) -> &Tags {
        &self.scope
    }

    fn apply(&mut self, event: ClaimEvent) {
        match event {
            ClaimEvent::Registered { .. } => self.registered = true,
            ClaimEvent::Noted { .. } => {}
        }
    }
}
