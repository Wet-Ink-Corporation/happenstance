//! Payload encoding: the vocabulary [`DomainEvent`] is written in.
//!
//! Three concrete codecs, and the framing region that lets a store hold more
//! than one of them at a time. The region lives in `Event::metadata` and no
//! adapter parses it — [ADR-0021].
//!
//! [`DomainEvent`]: crate::DomainEvent
//! [ADR-0021]: https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0021-payload-evolution-and-codec-tag.md

use happenstance_core::EventType;
use happenstance_core::SequencedEvent;
use happenstance_core::bytes::Bytes;

use crate::domain::DomainEvent;

/// Turns a serialisable value into payload bytes, and bytes back into a value.
///
/// A codec is chosen by the caller and passed by reference, so one store can
/// hold more than one encoding at a time. `Json` is on by default, and
/// `Postcard` and `Cbor` arrive with the features that gate them. Those three
/// are named in plain text rather than linked, because a link that resolves
/// only when a feature is on is a rustdoc error in the build where it is off.
///
/// The trait is **not sealed**: a codec of your own is a legitimate thing to
/// write, which is why this is a trait rather than an enum of the three below.
/// An enum would have been shorter and would have forbidden it.
///
/// **`Codec` carries no associated `Error` type, and that is deliberate.** An
/// associated error would add a third type parameter to every downstream
/// signature that already carries a store error and a domain error — the
/// command loop's error enum, [`Boundary::absorb`], the projection runner. A
/// single [`CodecError`] carrying the underlying error as a typed source keeps
/// those signatures at two.
///
/// The worked example lives on each concrete codec's own page, where it can
/// be written against a type that exists in that build.
///
/// [`Boundary::absorb`]: crate::Boundary::absorb
pub trait Codec {
    /// The value written into an event so a reader knows how to decode it.
    ///
    /// It is written into a framing region at the front of the event's
    /// metadata, which no store parses and no adapter has to understand. The
    /// other admissible home — a [`Tag`](happenstance_core::Tag) — lost
    /// because a tag participates in query matching, so re-encoding an event
    /// would silently change which consistency boundaries it belongs to.
    ///
    /// Must be non-empty and free of the byte `0xFF`, which terminates it in
    /// the framing region. A `&'static str` is UTF-8, so the second half is
    /// free: `0xFF` cannot appear in one.
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
    ///
    /// It means exactly one thing: a tag *was* written and this build cannot
    /// honour it. An event carrying no framing region at all is not this — it
    /// decodes with the codec already in hand, because refusing it would make
    /// every log written before the typed layer existed unreadable.
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

// ---------------------------------------------------------------------------
// The concrete codecs
// ---------------------------------------------------------------------------

/// JSON payloads, on by default. Readable in a database console.
///
/// The encoding a first program gets without naming one, because a reader who
/// has to name a codec before naming a domain has been charged for a choice
/// they had no basis to make yet.
///
/// # Example
///
/// ```
/// use happenstance::{Codec, Json};
///
/// #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
/// struct Seat { row: u32 }
///
/// let bytes = Json.encode(&Seat { row: 3 })?;
/// assert_eq!(Json.decode::<Seat>(&bytes)?, Seat { row: 3 });
/// # Ok::<(), happenstance::CodecError>(())
/// ```
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Json;

#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
impl Codec for Json {
    const TAG: &'static str = "json";

    fn encode<T: serde::Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
        serde_json::to_vec(value)
            .map(Bytes::from)
            .map_err(|err| CodecError::Encode(Box::new(err)))
    }

    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
        serde_json::from_slice(data).map_err(|err| CodecError::Decode(Box::new(err)))
    }
}

/// Compact binary payloads, for an edge deployment that pays per byte.
///
/// Not self-describing: the bytes carry no field names, so a decoder that
/// disagrees with the encoder about a type's shape resynchronises against the
/// wrong field rather than failing. Version the payload, not the event type.
#[cfg(feature = "postcard")]
#[cfg_attr(docsrs, doc(cfg(feature = "postcard")))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Postcard;

#[cfg(feature = "postcard")]
#[cfg_attr(docsrs, doc(cfg(feature = "postcard")))]
impl Codec for Postcard {
    const TAG: &'static str = "postcard";

    fn encode<T: serde::Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
        postcard::to_stdvec(value)
            .map(Bytes::from)
            .map_err(|err| CodecError::Encode(Box::new(err)))
    }

    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
        postcard::from_bytes(data).map_err(|err| CodecError::Decode(Box::new(err)))
    }
}

/// CBOR payloads: binary, and self-describing where postcard is not.
///
/// It ships because `cargo deny check licenses` cleared `ciborium`'s whole
/// subtree before the dependency was added. Had it refused, this type would
/// not exist and the manifest would carry the reason instead.
#[cfg(feature = "cbor")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbor")))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cbor;

#[cfg(feature = "cbor")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbor")))]
impl Codec for Cbor {
    const TAG: &'static str = "cbor";

    fn encode<T: serde::Serialize>(&self, value: &T) -> Result<Bytes, CodecError> {
        let mut out = Vec::new();
        ciborium::into_writer(value, &mut out)
            .map(|()| Bytes::from(out))
            .map_err(|err| CodecError::Encode(Box::new(err)))
    }

    fn decode<T: serde::de::DeserializeOwned>(&self, data: &[u8]) -> Result<T, CodecError> {
        ciborium::from_reader(data).map_err(|err| CodecError::Decode(Box::new(err)))
    }
}

// ---------------------------------------------------------------------------
// The framing region
// ---------------------------------------------------------------------------

/// The framing region's magic bytes, without the version that follows them.
///
/// Metadata that does not begin with these four bytes has no framing region:
/// every byte of it is the application's. That is what makes the region
/// distinguishable from metadata the typed layer never wrote.
const MAGIC: &[u8] = b"hpst";

/// The magic bytes and the format version, as one prefix.
///
/// A later revision changes the version byte, and an older build then meets
/// magic it recognises followed by a version it does not — which is a refusal
/// here rather than a silent misread of the application's own bytes.
const FRAMING: &[u8] = b"hpst\x01";

/// The byte that ends the tag and begins the application's own metadata.
///
/// `0xFF` cannot appear in UTF-8, and [`Codec::TAG`] is a `&'static str`, so no
/// tag can contain it. A length prefix lost to it: a byte-counted tag caps the
/// tag at 255 and needs a truncating cast to write, and neither buys anything a
/// terminator that cannot collide does not.
const TAG_END: u8 = 0xFF;

/// The claim a signature cannot make: a codec's tag can be written down.
///
/// The claim lives in a `const` **item** so that reading it is what evaluates
/// it, once per codec type — a module-scope `const _: () = …` cannot see an
/// implementor's associated const and would assert nothing at all.
pub(crate) struct TagIsWritable<C: Codec>(core::marker::PhantomData<C>);

impl<C: Codec> TagIsWritable<C> {
    pub(crate) const CHECKED: () = assert!(
        !C::TAG.is_empty(),
        "a Codec::TAG must not be empty; it is what a reader resolves an encoding by",
    );
}

/// Builds the metadata for an event this codec encoded.
///
/// `application` is whatever metadata the caller wanted on the event —
/// causation, correlation — and it is copied through after the region,
/// untouched and never parsed.
pub(crate) fn frame<C: Codec>(application: Option<&Bytes>) -> Bytes {
    let () = TagIsWritable::<C>::CHECKED;

    let tag = C::TAG.as_bytes();
    let application = application.map_or(&[][..], |bytes| bytes.as_ref());

    let mut out = Vec::with_capacity(FRAMING.len() + tag.len() + 1 + application.len());
    out.extend_from_slice(FRAMING);
    out.extend_from_slice(tag);
    out.push(TAG_END);
    out.extend_from_slice(application);
    Bytes::from(out)
}

/// The codec tag an event carries, or `None` when it carries no region.
///
/// # Errors
///
/// Returns [`CodecError::UnknownTag`] when the region is present but this
/// build cannot read it — a framing version it does not know, a tag that never
/// terminates, or bytes that are not UTF-8. Rendering what was actually there
/// is the point: a refusal that names a category tells a reader nothing they
/// can act on.
fn recover(metadata: Option<&Bytes>) -> Result<Option<&str>, CodecError> {
    let Some(metadata) = metadata else {
        return Ok(None);
    };
    // Not ours. Every byte belongs to the application, including the ones an
    // event written before this crate existed put there.
    let Some(rest) = metadata.strip_prefix(FRAMING) else {
        if metadata.starts_with(MAGIC) {
            return Err(unreadable(metadata));
        }
        return Ok(None);
    };

    let Some(end) = rest.iter().position(|byte| *byte == TAG_END) else {
        return Err(unreadable(metadata));
    };
    match rest.get(..end).map(core::str::from_utf8) {
        Some(Ok(tag)) => Ok(Some(tag)),
        _ => Err(unreadable(metadata)),
    }
}

/// A framing region that is present and cannot be read, rendered as found.
fn unreadable(metadata: &[u8]) -> CodecError {
    // Bounded, because a refusal is a message a human reads and the
    // application's own metadata may be arbitrarily long.
    let seen = metadata.get(..32).unwrap_or(metadata);
    CodecError::UnknownTag {
        tag: String::from_utf8_lossy(seen).into_owned().into_boxed_str(),
    }
}

/// Decodes one read event under the codec **its own tag** names.
///
/// The codec in hand is used when the event carries no tag, and when the tag
/// is the codec's own. Any other tag is resolved against the codecs this build
/// carries — which is what a feature flag decides, and why turning `postcard`
/// off turns a postcard-tagged event into a refusal rather than a guess.
///
/// # Errors
///
/// Returns [`CodecError::UnknownTag`] when no codec in this build answers to
/// the tag, and the domain type's own failure otherwise.
pub(crate) fn decode_event<E: DomainEvent, C: Codec>(
    codec: &C,
    event: &SequencedEvent,
) -> Result<E, CodecError> {
    let data = event.event.data();
    let event_type = event.event.event_type();

    match recover(event.event.metadata())? {
        None => E::decode(codec, event_type, data),
        Some(tag) if tag == C::TAG => E::decode(codec, event_type, data),
        Some(tag) => decode_by_tag(tag, event_type, data),
    }
}

/// Resolves a tag against the codecs this build was compiled with.
fn decode_by_tag<E: DomainEvent>(
    tag: &str,
    event_type: &EventType,
    data: &Bytes,
) -> Result<E, CodecError> {
    // Every arm below is gated, so a build carrying no codec at all reaches
    // none of them — and would otherwise be the one configuration `-D warnings`
    // rejects for two unused parameters.
    #[cfg(not(any(feature = "json", feature = "postcard", feature = "cbor")))]
    let (_, _) = (event_type, data);

    #[cfg(feature = "json")]
    if tag == <Json as Codec>::TAG {
        return E::decode(&Json, event_type, data);
    }
    #[cfg(feature = "postcard")]
    if tag == <Postcard as Codec>::TAG {
        return E::decode(&Postcard, event_type, data);
    }
    #[cfg(feature = "cbor")]
    if tag == <Cbor as Codec>::TAG {
        return E::decode(&Cbor, event_type, data);
    }

    Err(CodecError::UnknownTag {
        tag: tag.to_owned().into_boxed_str(),
    })
}
