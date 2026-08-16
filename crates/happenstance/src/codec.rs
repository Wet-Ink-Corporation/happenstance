//! Payload encoding: the vocabulary [`DomainEvent`] is written in.
//!
//! [`DomainEvent`]: crate::DomainEvent

use happenstance_core::EventType;
use happenstance_core::bytes::Bytes;

/// Turns a serialisable value into payload bytes, and bytes back into a value.
///
/// A codec is chosen by the caller and passed by reference, so one store can
/// hold more than one encoding at a time. The concrete codecs — JSON and
/// friends — arrive with the feature flags that gate them; this is the trait
/// they satisfy and the bound every encoding signature in this crate is
/// written against.
///
/// **`Codec` carries no associated `Error` type, and that is deliberate.** An
/// associated error would add a third type parameter to every downstream
/// signature that already carries a store error and a domain error — the
/// command loop's error enum, [`Boundary::absorb`], the projection runner. A
/// single [`CodecError`] carrying the underlying error as a typed source keeps
/// those signatures at two.
///
/// [`Boundary::absorb`]: crate::Boundary::absorb
pub trait Codec {
    /// The value written into an event so a reader knows how to decode it.
    ///
    /// *Where* it is written — the event's metadata, or a tag — is not settled
    /// here, and this constant is invariant under that choice.
    const TAG: &'static str;

    /// Encodes `value` into payload bytes.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::Encode`] when the value cannot be serialised —
    /// a map with non-string keys under a JSON codec, for instance.
    fn encode<T: serde::Serialize>(&self, value: &T) -> Result<Bytes, CodecError>;

    /// Decodes payload bytes back into a `T`.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::Decode`] when the bytes are not a valid `T`:
    /// truncated, written by a different codec, or written by an older version
    /// of the type.
    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError>;
}

/// Why a payload could not be turned into bytes, or bytes into a value.
///
/// Every variant carries what failed rather than a category name, and the two
/// codec-side variants carry the codec's own error as a typed source. A
/// `String` would lose the chain a caller needs to report the failure; an
/// associated error type on [`Codec`] would spread a third type parameter
/// through every signature downstream of it.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CodecError {
    /// An event was written by a codec no reader here can name.
    #[error("no codec is registered for tag `{tag}`")]
    UnknownTag {
        /// The tag read off the event.
        tag: Box<str>,
    },
    /// A nominated event's type is not one this domain type declares.
    ///
    /// This is a real disagreement between a `DomainEvent`'s `EVENT_TYPES` and
    /// the fold that interprets them, not a routine miss: an event the query
    /// never nominated is skipped without ever reaching a codec.
    #[error("`{event_type}` is not an event of this domain type")]
    UnknownEventType {
        /// The event type that was nominated and could not be decoded.
        event_type: EventType,
    },
    /// The codec refused to serialise the value.
    ///
    /// The message names the condition and the codec's own error is the
    /// [`source`](core::error::Error::source). `#[error(transparent)]` — which
    /// is what the design wrote — cannot carry an explicit source: thiserror
    /// refuses the pair outright, and transparent alone forwards the *inner*
    /// error's source, so the box itself would drop out of the chain a caller
    /// reports from.
    #[error("the codec could not encode the payload")]
    Encode(#[source] Box<dyn core::error::Error + Send + Sync>),
    /// The codec refused to deserialise the bytes.
    #[error("the codec could not decode the payload")]
    Decode(#[source] Box<dyn core::error::Error + Send + Sync>),
}
