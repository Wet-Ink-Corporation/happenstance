//! Events, their types, and their positions in the store's total order.

use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;
use core::num::NonZeroU64;

use bytes::Bytes;

use crate::error::InvalidEventType;
use crate::tag::Tags;
use crate::validate;

/// Longest permitted event type, in bytes.
pub const MAX_EVENT_TYPE_LEN: usize = 255;

/// The identifier a store filters on: `CourseDefined`, `StudentSubscribed`, and
/// so on.
///
/// Backed by `Cow<'static, str>` so that one type expresses both identifiers
/// written in the source and baked into the binary, and identifiers that
/// arrived from a peer at run time and had to be allocated. Ingest needs the
/// second; a codec registry wants the first; a newtype over `&'static str`
/// alone could only express the first, which is why it lost.
///
/// # Examples
///
/// ```
/// use happenstance_core::EventType;
///
/// let ty = EventType::new("StudentSubscribed")?;
/// assert_eq!(ty.as_str(), "StudentSubscribed");
/// # Ok::<(), happenstance_core::InvalidEventType>(())
/// ```
///
/// Constructed in a `const`, it is validated by the compiler and allocates
/// nothing:
///
/// ```
/// use happenstance_core::EventType;
///
/// const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
/// assert_eq!(COURSE_DEFINED.as_str(), "CourseDefined");
/// ```
#[derive(Clone)]
pub struct EventType(Cow<'static, str>);

impl EventType {
    /// Creates an event type.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidEventType`] if the value is empty, longer than
    /// [`MAX_EVENT_TYPE_LEN`] bytes, contains a character in Unicode general
    /// category `Cc`, or contains one of the seven explicit bidirectional
    /// formatting controls.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidEventType> {
        let value = value.into();
        match validate::check(&value, MAX_EVENT_TYPE_LEN) {
            validate::Refusal::Accepted => Ok(Self(Cow::Owned(value))),
            validate::Refusal::Empty => Err(InvalidEventType::Empty),
            validate::Refusal::TooLong => Err(InvalidEventType::TooLong { len: value.len() }),
            validate::Refusal::ControlCharacter => Err(InvalidEventType::ControlCharacter),
            validate::Refusal::BidirectionalControl => Err(InvalidEventType::BidirectionalControl),
        }
    }

    /// Creates an event type from a string literal, validating at compile time.
    ///
    /// Enforces exactly the rules [`new`](Self::new) enforces — one function
    /// checks both — so an `EventType` is always a validated value, with some of
    /// that validation having happened before the program ran.
    ///
    /// # Panics
    ///
    /// Panics if the value would be rejected by [`new`](Self::new). **Where
    /// that panic surfaces depends on the call site, and the difference is
    /// sharp enough to be worth stating:**
    ///
    /// | Call site | When the invalid value is caught |
    /// |---|---|
    /// | a free `const` | `cargo check`, as `error[E0080]` |
    /// | an associated `const` that is read somewhere | `cargo build` |
    /// | an associated `const` that is never read | **never** |
    /// | a `let` binding | at run time, as a panic |
    ///
    /// The third row is the one to design around: an associated const is
    /// evaluated lazily, so an invalid one that nothing reads survives `check`,
    /// `clippy`, `build` and `test`. Prefer a free `const` for anything whose
    /// validity you want the compiler to guarantee.
    #[must_use]
    pub const fn from_static(value: &'static str) -> Self {
        match validate::check(value, MAX_EVENT_TYPE_LEN) {
            validate::Refusal::Accepted => Self(Cow::Borrowed(value)),
            validate::Refusal::Empty => panic!("an event type must not be empty"),
            validate::Refusal::TooLong => {
                panic!("an event type must be at most MAX_EVENT_TYPE_LEN bytes")
            }
            validate::Refusal::ControlCharacter => {
                panic!("an event type must not contain control characters")
            }
            validate::Refusal::BidirectionalControl => {
                panic!("an event type must not contain bidirectional formatting controls")
            }
        }
    }

    /// The event type as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// `Eq`, `Ord` and `Hash` are written out rather than derived. A derive on a
// single-field tuple struct produces exactly these bodies today and would
// silently produce different ones the moment a second field lands — and this
// phase is adding fields to neighbouring types for that very reason. Writing
// them here also puts the `Borrow<str>` obligation at the place it is
// discharged: `Borrow` promises the borrowed form hashes and compares
// *identically* to the owner, and a `HashMap` whose key breaks that promise
// loses entries with no diagnostic anywhere.
impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for EventType {}

impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventType {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl core::hash::Hash for EventType {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl core::borrow::Borrow<str> for EventType {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventType({:?})", self.as_str())
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for EventType {
    type Error = InvalidEventType;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for EventType {
    type Error = InvalidEventType;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl core::str::FromStr for EventType {
    type Err = InvalidEventType;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// An event's place in the store's total order.
///
/// The specification requires positions to be unique and monotonically
/// increasing, and explicitly permits gaps — so this is an opaque ordering key,
/// **not** a count of events. Never compute `end - start` and call it a length.
///
/// Backed by [`NonZeroU64`], which makes position zero unrepresentable and lets
/// `Option<SequencePosition>` occupy the same eight bytes as a bare
/// `SequencePosition`. That matters: `Option<SequencePosition>` appears in every
/// [`AppendCondition`](crate::AppendCondition) and every
/// [`ReadOptions`](crate::ReadOptions).
///
/// # Examples
///
/// ```
/// use happenstance_core::SequencePosition;
///
/// assert_eq!(SequencePosition::FIRST.get(), 1);
/// assert!(SequencePosition::new(0).is_none());
///
/// // The niche optimisation: no discriminant word is needed.
/// assert_eq!(
///     size_of::<Option<SequencePosition>>(),
///     size_of::<SequencePosition>(),
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SequencePosition(NonZeroU64);

impl SequencePosition {
    /// The lowest representable position.
    pub const FIRST: Self = Self(NonZeroU64::MIN);

    /// Creates a position, rejecting zero.
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// The underlying value.
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// The next position, or `None` on overflow.
    ///
    /// Only meaningful for adapters that allocate positions densely; the
    /// specification does not require the next append to land here.
    pub const fn next(self) -> Option<Self> {
        Self::new(self.0.get().saturating_add(1))
    }
}

impl fmt::Display for SequencePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SequencePosition> for u64 {
    fn from(position: SequencePosition) -> Self {
        position.get()
    }
}

/// An event on its way into the store.
///
/// The payload is opaque: the contract layer never parses it. That keeps
/// adapters free of domain knowledge and lets replication forward events
/// byte-for-byte without deserialising them. Encoding and decoding are the
/// typed layer's job.
///
/// # Examples
///
/// ```
/// use happenstance_core::{Event, Tags};
///
/// let event = Event::new("StudentSubscribed", &b"{\"student\":\"s1\"}"[..])?
///     .with_tags(Tags::from_pairs([("course", "c1"), ("student", "s1")])?);
///
/// assert_eq!(event.event_type().as_str(), "StudentSubscribed");
/// assert_eq!(event.tags().len(), 2);
/// assert!(event.metadata().is_none());
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
// `event_type` repeats the struct name, but it is the specification's own term
// and renaming it to `kind` or `ty` would make the mapping to DCB less obvious.
#[allow(clippy::struct_field_names)]
#[derive(Clone, PartialEq, Eq)]
pub struct Event {
    event_type: EventType,
    data: Bytes,
    tags: Tags,
    metadata: Option<Bytes>,
}

impl Event {
    /// Creates an untagged event.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidEventType`] if `event_type` fails
    /// [`EventType::new`]'s validation.
    ///
    /// # Examples
    ///
    /// The bound accepts a `&str`, which is validated here, and an
    /// already-built [`EventType`], which is not validated twice:
    ///
    /// ```
    /// use happenstance_core::{Event, EventType};
    ///
    /// const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");
    ///
    /// let from_str = Event::new("CourseDefined", &b"{}"[..])?;
    /// let from_const = Event::new(COURSE_DEFINED, &b"{}"[..])?;
    /// assert_eq!(from_str.event_type(), from_const.event_type());
    /// # Ok::<(), happenstance_core::InvalidEventType>(())
    /// ```
    pub fn new<T>(event_type: T, data: impl Into<Bytes>) -> Result<Self, InvalidEventType>
    where
        // Deliberately not `TryInto<EventType, Error = InvalidEventType>`. That
        // equality constraint excludes the *infallible* identity conversion, so
        // an already-built `EventType` — the thing a codec registry interns once
        // per domain event — could not be passed at all (`error[E0271]`). The
        // looser pair below accepts both, and is the shape `QueryItem::new`
        // already uses one file over.
        T: TryInto<EventType>,
        InvalidEventType: From<T::Error>,
    {
        Ok(Self {
            event_type: event_type.try_into()?,
            data: data.into(),
            tags: Tags::empty(),
            metadata: None,
        })
    }

    /// Attaches tags, replacing any already set.
    #[must_use]
    pub fn with_tags(mut self, tags: Tags) -> Self {
        self.tags = tags;
        self
    }

    /// Attaches opaque client metadata, replacing any already set.
    #[must_use]
    pub fn with_metadata(mut self, metadata: impl Into<Bytes>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }

    /// The event's type.
    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    /// The opaque payload.
    pub fn data(&self) -> &Bytes {
        &self.data
    }

    /// The event's tags, in canonical order.
    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    /// The opaque client metadata, if any.
    pub fn metadata(&self) -> Option<&Bytes> {
        self.metadata.as_ref()
    }

    /// Decomposes the event, avoiding a clone in adapter write paths.
    pub fn into_parts(self) -> (EventType, Bytes, Tags, Option<Bytes>) {
        (self.event_type, self.data, self.tags, self.metadata)
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Payloads are frequently large and rarely UTF-8; printing the length
        // keeps test failures legible.
        /// Renders a payload as its length rather than its contents.
        struct ByteLen(Option<usize>);

        impl fmt::Debug for ByteLen {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    Some(len) => write!(f, "<{len} bytes>"),
                    None => f.write_str("None"),
                }
            }
        }

        f.debug_struct("Event")
            .field("event_type", &self.event_type)
            .field("data", &ByteLen(Some(self.data.len())))
            .field("tags", &self.tags)
            .field("metadata", &ByteLen(self.metadata.as_ref().map(Bytes::len)))
            .finish()
    }
}

/// An [`Event`] that has been assigned a position by the store.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SequencedEvent {
    /// Where the event sits in the store's total order.
    pub position: SequencePosition,
    /// The event itself.
    pub event: Event,
}

impl SequencedEvent {
    /// Pairs an event with its assigned position.
    pub const fn new(position: SequencePosition, event: Event) -> Self {
        Self { position, event }
    }

    /// The event's type. Shorthand for `self.event.event_type()`.
    pub fn event_type(&self) -> &EventType {
        self.event.event_type()
    }

    /// The event's tags. Shorthand for `self.event.tags()`.
    pub fn tags(&self) -> &Tags {
        self.event.tags()
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{Event, EventType, SequencePosition, SequencedEvent};
    use alloc::string::String;
    use bytes::Bytes;
    use core::num::NonZeroU64;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for EventType {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = String::deserialize(deserializer)?;
            Self::new(raw).map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for SequencePosition {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.get())
        }
    }

    impl<'de> Deserialize<'de> for SequencePosition {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let raw = NonZeroU64::deserialize(deserializer)?;
            Ok(Self(raw))
        }
    }

    /// Field-for-field mirror of [`Event`], used to keep the wire format stable
    /// while the real type's fields stay private.
    #[derive(Serialize, Deserialize)]
    #[serde(rename = "Event")]
    struct EventWire {
        event_type: EventType,
        data: Bytes,
        #[serde(default, skip_serializing_if = "crate::Tags::is_empty")]
        tags: crate::Tags,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        metadata: Option<Bytes>,
    }

    impl Serialize for Event {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            EventWire {
                event_type: self.event_type().clone(),
                data: self.data().clone(),
                tags: self.tags().clone(),
                metadata: self.metadata().cloned(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Event {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = EventWire::deserialize(deserializer)?;
            let mut event = Self {
                event_type: wire.event_type,
                data: wire.data,
                tags: wire.tags,
                metadata: None,
            };
            if let Some(metadata) = wire.metadata {
                event = event.with_metadata(metadata);
            }
            Ok(event)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "SequencedEvent")]
    struct SequencedEventWire {
        position: SequencePosition,
        event: Event,
    }

    impl Serialize for SequencedEvent {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            SequencedEventWire {
                position: self.position,
                event: self.event.clone(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for SequencedEvent {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = SequencedEventWire::deserialize(deserializer)?;
            Ok(Self::new(wire.position, wire.event))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    #[test]
    fn rejects_invalid_event_types() {
        assert!(matches!(EventType::new(""), Err(InvalidEventType::Empty)));
        assert!(matches!(
            EventType::new("a\nb"),
            Err(InvalidEventType::ControlCharacter)
        ));
        assert!(matches!(
            EventType::new("x".repeat(MAX_EVENT_TYPE_LEN + 1)),
            Err(InvalidEventType::TooLong { .. })
        ));
    }

    #[test]
    fn position_zero_is_unrepresentable() {
        assert!(SequencePosition::new(0).is_none());
        assert_eq!(SequencePosition::new(1), Some(SequencePosition::FIRST));
    }

    #[test]
    fn option_position_is_niche_optimised() {
        assert_eq!(
            size_of::<Option<SequencePosition>>(),
            size_of::<SequencePosition>()
        );
    }

    #[test]
    fn builder_sets_tags_and_metadata() {
        let event = Event::new("Defined", &b"{}"[..])
            .unwrap()
            .with_tags(Tags::from_pairs([("course", "c1")]).unwrap())
            .with_metadata(&b"trace"[..]);

        assert_eq!(event.tags().len(), 1);
        assert_eq!(event.metadata().unwrap().as_ref(), b"trace");
    }
}
