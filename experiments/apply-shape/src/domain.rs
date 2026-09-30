//! The one domain every case folds: a tick against a key.
//!
//! Two events are special, and each exists to reach one arm of `on_error`:
//! a tick against [`POISON`] is decoded fine and **refused by `apply`** (an
//! application refusal, raised before any statement is issued), and
//! [`undecodable`] is an event of the right type whose payload is not JSON (a
//! decode failure, the crypto-shred case).

use happenstance::bytes::Bytes;
use happenstance::{Codec, CodecError, DomainEvent, Event, EventType, Json, Tags};
use serde::{Deserialize, Serialize};

/// The one event type.
pub const TICKED: EventType = EventType::from_static("Ticked");

/// The key `apply` refuses.
pub const POISON: &str = "poison";

/// A tick against a key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ticked {
    /// Which counter moves.
    pub key: String,
}

impl DomainEvent for Ticked {
    const EVENT_TYPES: &'static [EventType] = &[TICKED];

    fn event_type(&self) -> EventType {
        TICKED
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
        if *event_type != TICKED {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        codec.decode(data)
    }
}

/// A well-formed tick, as the event store holds it.
///
/// # Errors
///
/// The codec's refusal, which JSON never produces for this type.
pub fn tick(key: &str) -> Result<Event, CodecError> {
    let data = Ticked {
        key: key.to_owned(),
    }
    .encode(&Json)?;
    Event::new(TICKED, data).map_err(|error| CodecError::Encode(Box::new(error)))
}

/// An event of the right type whose payload is not JSON.
///
/// # Errors
///
/// Never, in practice: the type is a constant that validates.
pub fn undecodable() -> Result<Event, CodecError> {
    Event::new(TICKED, Bytes::from_static(b"\x00 shredded \x00"))
        .map_err(|error| CodecError::Encode(Box::new(error)))
}

/// Tags every projection here is scoped to: none. The query is constrained by
/// the event type alone.
#[must_use]
pub fn scope() -> Tags {
    Tags::empty()
}
