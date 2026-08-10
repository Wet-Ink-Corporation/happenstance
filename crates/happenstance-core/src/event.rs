//! Events, their types, and their positions in the store's total order.

use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt;
use core::num::NonZeroU64;

use bytes::Bytes;

use crate::error::InvalidEventType;
use crate::identity::{EventId, RecordedAt};
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
    ///
    /// An invalid value at a free `const` site is a compile error:
    ///
    /// ```compile_fail
    /// use happenstance_core::EventType;
    ///
    /// const EMPTY: EventType = EventType::from_static("");
    /// # let _ = EMPTY;
    /// ```
    ///
    /// Spelled bare `compile_fail` rather than `compile_fail,E0080`: rustdoc on
    /// 1.97.1 silently ignores an error-code annotation it cannot match, so the
    /// stricter-looking spelling is the weaker check. It is also deliberately
    /// weaker than a `trybuild` snapshot — it does not pin the diagnostic — and
    /// ADR-0015 records that phase 6 owns the `trybuild` dependency decision.
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
    /// **This is the resume idiom.** A consumer that has processed up to
    /// `checkpoint` resumes with `ReadOptions::from(checkpoint.next()?)`, and
    /// that is sound on a store with gaps: `from` is a threshold rather than a
    /// seek, so if nothing occupies `checkpoint + 1` the read yields the next
    /// event above it. This is not the caller doing arithmetic on an opaque
    /// ordering key — it is the one method on this type whose whole purpose is
    /// to advance past a position without the caller knowing what positions
    /// mean.
    ///
    /// Do not read `Some` as a promise that an event exists there, or that the
    /// next append will land there. The specification permits gaps everywhere.
    pub const fn next(self) -> Option<Self> {
        // `checked_add`, not `saturating_add`. Saturating made the one method
        // whose documented purpose is signalling overflow incapable of it:
        // at `u64::MAX` it returned `Some(u64::MAX)`, so a consumer resuming
        // from the last representable position would re-read it forever instead
        // of being told it had run out of key space.
        match self.0.get().checked_add(1) {
            Some(next) => Self::new(next),
            None => None,
        }
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

    /// Decomposes the event into its owned parts.
    ///
    /// **Not a clone-avoidance route for adapters**, which is what this line
    /// used to claim. [`EventStore::append`](crate::EventStore::append) takes
    /// `&[Event]`, so no store implementation ever owns an `Event` and none can
    /// reach this method at all; an owning adapter clones instead, and that
    /// clone is cheap — the expensive fields are [`Bytes`], so it bumps a
    /// refcount rather than copying the payload, leaving one `Box<str>` and one
    /// boxed tag slice. This exists for the code that genuinely does own an
    /// event: callers, and the wire encoders in the typed layer.
    ///
    /// Returns a struct rather than a tuple so that the *number* of an event's
    /// parts is not public API. Every `let (ty, data, tags, meta) = …` would
    /// break the day a fifth part existed; reading fields by name, or
    /// destructuring with `..`, survives it.
    #[must_use]
    pub fn into_parts(self) -> EventParts {
        EventParts {
            event_type: self.event_type,
            data: self.data,
            tags: self.tags,
            metadata: self.metadata,
        }
    }
}

/// The owned pieces of an [`Event`], from [`Event::into_parts`].
///
/// `#[non_exhaustive]`, so a later part is additive: downstream destructures
/// with `..` or reads fields by name.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct EventParts {
    /// The event's type.
    pub event_type: EventType,
    /// The opaque payload.
    pub data: Bytes,
    /// The tags the writer attached.
    pub tags: Tags,
    /// The opaque client metadata, if any.
    pub metadata: Option<Bytes>,
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

/// An [`Event`] together with the three facts its store assigned it.
///
/// # Why `position` and `id.position()` are both here
///
/// They are the same number for a locally appended event, and different numbers
/// for one that arrived through replication. `position` is **arrival order in
/// this store**; `id.position()` is **authorship order in the store that first
/// accepted it**. Keeping only one of them would be correct for local appends
/// and would lose either the local ordering or the origin identity for
/// replicated ones.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SequencedEvent {
    /// Where the event sits in **this** store's total order.
    pub position: SequencePosition,
    /// Which store first accepted the event, and where it sat there.
    ///
    /// Minted by the store at `append` for a local write, and preserved
    /// unchanged for an event accepted through ingest.
    pub id: EventId,
    /// When the store accepted it. Not an ordering key — see [`RecordedAt`].
    pub recorded_at: RecordedAt,
    /// The event itself.
    pub event: Event,
}

impl SequencedEvent {
    /// Pairs an event with the facts its store assigned it.
    ///
    /// This constructor went from two arguments to four in one commit, with no
    /// deprecated two-argument arm. That is deliberate: a compatibility shim
    /// would have to invent a [`StoreId`](crate::StoreId) and a time, which is precisely the
    /// wrong implementation the specification rejects by name — an adapter that
    /// makes identity up rather than persisting it.
    ///
    /// A *defaultable* field added later needs no change here; it lands as a
    /// `with_*` builder, in the shape [`with_recorded_at`](Self::with_recorded_at)
    /// establishes. A further **required** store-assigned fact would supersede
    /// this constructor again, and there is no signature that avoids that.
    pub const fn new(
        position: SequencePosition,
        id: EventId,
        recorded_at: RecordedAt,
        event: Event,
    ) -> Self {
        Self {
            position,
            id,
            recorded_at,
            event,
        }
    }

    /// Replaces the recorded time.
    ///
    /// For adapters reconstructing a stored event, and for tests that need a
    /// fixed clock. It is the shape a later *defaultable* field follows.
    #[must_use]
    pub const fn with_recorded_at(mut self, recorded_at: RecordedAt) -> Self {
        self.recorded_at = recorded_at;
        self
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
    //! Wire mirrors. **Every field is written on every serialisation** — no
    //! `skip_serializing_if`, no `#[serde(default)]` — and neither attribute may
    //! come back (WF-2; ADR-0016 §3 and §4 hold the measurements).
    //!
    //! Skipping a field makes the encoding *positional* in a format that is not
    //! self-describing. In postcard the decoder resynchronises against the wrong
    //! field: three of `Event`'s four shapes then error loudly and the fourth
    //! decodes a neighbour's bytes silently. What that bought was one byte per
    //! absent field; in JSON, restoring both skipped fields costs 26.
    //!
    //! `#[serde(default)]` goes for a different reason. It is inert on the write
    //! side — postcard output is byte-identical with and without it — so its only
    //! effect is to widen what the *decoder* accepts, giving a second and
    //! undocumented format: one shape written, a strictly larger set accepted,
    //! nothing describing the difference, and a peer's bug surviving the round
    //! trip as a plausible default value.
    //!
    //! The obligation binds the **encoder** only (ADR-0016 §5). serde's derive
    //! still routes an absent `Option` field through `missing_field`, which
    //! yields `None` with no attribute at all; that asymmetry is accepted rather
    //! than closed with a hand-written visitor, because `None` fails closed
    //! everywhere this crate reads it.
    //!
    //! Payloads carry a second obligation, which [`payload`] holds: opaque bytes
    //! encode as base64 where a human will read them and as raw bytes where
    //! nobody will (WF-11, ADR-0016 §10).
    //!
    //! ```
    //! use happenstance_core::Event;
    //!
    //! let event = Event::new("SeatMapPublished", &b"\xde\xad\xbe\xef"[..])?;
    //! let json = serde_json::to_string(&event)?;
    //!
    //! // Base64 text, not the array of decimal integers `bytes` would emit —
    //! // 2.6786x smaller on a 340 KiB seat map, and legible in a log.
    //! assert!(json.contains(r#""data":"3q2+7w==""#), "{json}");
    //! assert!(json.contains(r#""metadata":null"#), "{json}");
    //!
    //! // An empty payload is not an absent one: ADR-0003 promises byte-for-byte
    //! // forwarding, and "the peer sent nothing" is not "the peer sent none".
    //! let empty = event.clone().with_metadata(&b""[..]);
    //! assert!(serde_json::to_string(&empty)?.contains(r#""metadata":"""#));
    //!
    //! // postcard is not human-readable: the four bytes travel as themselves.
    //! let binary = postcard::to_stdvec(&event)?;
    //! assert!(binary.windows(4).any(|w| w == b"\xde\xad\xbe\xef"));
    //! assert_eq!(postcard::from_bytes::<Event>(&binary)?, event);
    //! # Ok::<(), Box<dyn core::error::Error>>(())
    //! ```
    use super::{Event, EventType, SequencePosition, SequencedEvent};
    use alloc::string::String;
    use bytes::Bytes;
    use core::num::NonZeroU64;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// The opaque-payload encoding: base64 in human-readable formats, raw bytes
    /// otherwise (WF-11, ADR-0016 §10).
    ///
    /// `bytes`' own impl renders a JSON array of decimal integers, which is
    /// 3.5715x the raw size and unreadable; standard-alphabet base64 with
    /// padding is 1.3333x and copies out of a log in one selection.
    ///
    /// # The wrong implementation this exists to reject
    ///
    /// An **inverted** `is_human_readable` branch: raw bytes in the JSON arm,
    /// base64 in the postcard one. Each arm is internally consistent, so
    /// encode-then-decode agrees with itself whichever one ran and a round-trip
    /// test passes in **both** formats — measured, on a deliberately inverted
    /// newtype over `[de ad be ef]`, in
    /// `docs/experiments/wire-format/tests/decorative_inverted_branch.rs`. That
    /// is why WF-11 names two rules asserting the bytes actually on the wire
    /// (`wire::payload_is_base64_in_json` and `wire::payload_is_raw_in_postcard`)
    /// rather than one asserting a round trip.
    ///
    /// The human-readable arm materialises the whole payload, and no encoding
    /// avoids that: serde's data model has no streaming entry point for a string,
    /// so `serialize_str` and `collect_str` both write the entire rendering.
    /// WF-11's falsifier is about that property, not about base64.
    mod payload {
        use super::{Bytes, Deserialize, Deserializer, Serialize, Serializer};
        use alloc::string::String;
        use base64::Engine as _;
        use base64::engine::general_purpose::STANDARD;

        pub(super) fn serialize<S: Serializer>(
            value: &Bytes,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                serializer.serialize_str(&STANDARD.encode(value))
            } else {
                serializer.serialize_bytes(value)
            }
        }

        pub(super) fn deserialize<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Bytes, D::Error> {
            if deserializer.is_human_readable() {
                let text = String::deserialize(deserializer)?;
                STANDARD
                    .decode(text)
                    .map(Bytes::from)
                    .map_err(serde::de::Error::custom)
            } else {
                Bytes::deserialize(deserializer)
            }
        }

        /// The same branch for `Option<Bytes>`.
        ///
        /// The `Option` layer stays serde's, rather than being folded into one
        /// impl, because that is what keeps `Some(Bytes::new())` — the JSON
        /// string `""` — apart from `None`, which is `null`.
        pub(super) mod optional {
            use super::{Bytes, Deserialize, Deserializer, Serialize, Serializer};

            /// Routes one payload through the branch above. Borrowed on the way
            /// out so an `Option<Bytes>` is not cloned to be written.
            struct Encode<'a>(&'a Bytes);

            impl Serialize for Encode<'_> {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    super::serialize(self.0, serializer)
                }
            }

            /// The same, on the way in, where the bytes must be owned.
            struct Decode(Bytes);

            impl<'de> Deserialize<'de> for Decode {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    super::deserialize(deserializer).map(Self)
                }
            }

            // `Option<&Bytes>` would be the idiomatic parameter, and serde's
            // `with` attribute does not offer it: the derive passes the field by
            // reference, so the signature is `&Option<T>` or nothing.
            #[allow(
                clippy::ref_option,
                reason = "the signature serde's `with` attribute calls"
            )]
            pub(in crate::event::serde_impls) fn serialize<S: Serializer>(
                value: &Option<Bytes>,
                serializer: S,
            ) -> Result<S::Ok, S::Error> {
                match value {
                    Some(payload) => serializer.serialize_some(&Encode(payload)),
                    None => serializer.serialize_none(),
                }
            }

            pub(in crate::event::serde_impls) fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Option<Bytes>, D::Error> {
                Ok(Option::<Decode>::deserialize(deserializer)?.map(|decoded| decoded.0))
            }
        }
    }

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
        #[serde(with = "payload")]
        data: Bytes,
        tags: crate::Tags,
        /// `Some(Bytes::new())` and `None` must stay distinguishable on the wire
        /// — ADR-0003 promises byte-for-byte forwarding of an opaque payload, and
        /// "the peer sent an empty metadata blob" is not "the peer sent none".
        /// Writing the field unconditionally is what keeps them apart: `null`
        /// against `""` in JSON, `[00]` against `[01 00]` in postcard.
        #[serde(with = "payload::optional")]
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

    /// Field-for-field mirror of [`SequencedEvent`].
    ///
    /// All four fields are unconditional, and WF-5 is why they must stay that
    /// way. `id` is the one under pressure: it looks derivable from `position`,
    /// and it is not. An event replicated A→B→C keeps the identity **A**
    /// assigned it, while B and C assign it their own positions — so a mirror
    /// that drops `id`, or skips it when it happens to match the local store,
    /// forges a fresh identity at every hop.
    ///
    /// ADR-0014 landed this shape already; it is documented rather than changed
    /// so that the next reader does not re-derive it as a saving.
    #[derive(Serialize, Deserialize)]
    #[serde(rename = "SequencedEvent")]
    struct SequencedEventWire {
        position: SequencePosition,
        id: crate::identity::EventId,
        recorded_at: crate::identity::RecordedAt,
        event: Event,
    }

    impl Serialize for SequencedEvent {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            SequencedEventWire {
                position: self.position,
                id: self.id,
                recorded_at: self.recorded_at,
                event: self.event.clone(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for SequencedEvent {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = SequencedEventWire::deserialize(deserializer)?;
            Ok(Self::new(
                wire.position,
                wire.id,
                wire.recorded_at,
                wire.event,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;

    /// `next()` returns `None` at the top of the key space rather than
    /// repeating itself.
    ///
    /// VT-13. `saturating_add` clamped, `u64::MAX` is non-zero, and `new`
    /// wrapped it back in `Some` — so the one method whose documented purpose is
    /// signalling overflow was incapable of it, and the resume idiom
    /// `checkpoint.next()` re-read the same event forever at exactly the
    /// position nobody tests. `NonZeroU64::checked_add` is `const`, so the
    /// repair costs neither the `const` nor a byte: the newtype's forbidden zero
    /// is the niche `Option` spends on `None`.
    #[test]
    fn position_next_signals_overflow() {
        // Declared first, not beside the assertion that reads it, because
        // `clippy::items_after_statements` is on and is right to be: an item
        // declared mid-function is in scope from the top of the function
        // regardless of where it is written.
        //
        // It is evaluated by the compiler rather than at run time, which is what
        // makes it the `const`-ness assertion. `const` is load bearing on
        // `next`: it is what lets a checkpoint advance in a `const` initialiser,
        // and it is the reason the body is a `match` rather than `?` or `map` —
        // neither is available to a `const fn` on this toolchain.
        const OVERFLOW: Option<SequencePosition> = match SequencePosition::new(u64::MAX) {
            Some(last) => last.next(),
            None => None,
        };

        let last = SequencePosition::new(u64::MAX).unwrap();
        assert_eq!(
            last.next(),
            None,
            "there is no position above the last representable one, and the \
             signature has a way to say so"
        );

        // The step below it still advances, which is what keeps the assertion
        // above from being satisfied by a `next` that returns `None` always.
        let second = SequencePosition::FIRST.next().unwrap();
        assert!(second > SequencePosition::FIRST);

        assert!(OVERFLOW.is_none());
    }

    /// `from_static` and `new` accept exactly the same values.
    ///
    /// VT-32's MUST, and the divergence a byte walk appears to force and does
    /// not: there is one validator, `validate::check`, and both constructors
    /// call it. This asserts the agreement rather than describing it, because
    /// the day a second `const`-only path is added is the day the two stop
    /// agreeing and nothing else notices.
    #[test]
    fn from_static_and_new_agree() {
        // One value from each boundary `validate::check` decides: ordinary, the
        // format characters scripts need, the invisible one VT-14 keeps legal,
        // and the four neighbours of the two closed bidirectional runs.
        for value in [
            "CourseDefined",
            "a\u{200C}b",
            "a\u{200D}b",
            "a\u{200B}b",
            "a\u{2029}b",
            "a\u{202F}b",
            "a\u{2065}b",
            "a\u{206A}b",
        ] {
            let runtime = EventType::new(value).unwrap();
            assert_eq!(EventType::from_static(value).as_str(), runtime.as_str());
        }
    }

    /// `from_static` refuses everything `new` refuses — by panicking, which is a
    /// compile error at a free `const` site.
    ///
    /// One `#[should_panic]` per refusal class, because a panic is the only
    /// channel a `const fn` has. The bidirectional class is the one VT-14 added
    /// at phase 4 and is therefore the one most likely to be dropped from a
    /// future edit of the validator.
    #[test]
    #[should_panic(expected = "bidirectional formatting controls")]
    fn from_static_rejects_a_bidirectional_control() {
        let _ = EventType::from_static("Order\u{202E}Placed");
    }

    #[test]
    #[should_panic(expected = "control characters")]
    fn from_static_rejects_a_c1_control() {
        // U+0085 NEL is `Cc` and is not ASCII — the case four doc comments used
        // to claim was out of scope.
        let _ = EventType::from_static("Order\u{85}Placed");
    }

    /// `Borrow<str>` hashes and compares as the owner does, so a map keyed by
    /// `EventType` can be probed with a `&str`.
    ///
    /// VT-33 requires this asserted rather than claimed. `Borrow` carries a
    /// documented extra obligation — the borrowed form must hash and compare
    /// exactly as the owner — and `Eq`/`Hash` are hand-written here (a derive
    /// would be correct today and quietly wrong the moment a second field
    /// lands). A violation loses `HashMap` entries with no diagnostic anywhere,
    /// which is why the assertion is a real map lookup rather than a comparison
    /// of two hashes.
    ///
    /// Gated on `std` rather than written against a hand-rolled `Hasher`:
    /// `happenstance-core` is `#![cfg_attr(not(feature = "std"), no_std)]` and
    /// `HashMap` is `std`'s, so an ungated test would break the
    /// `--no-default-features` arm the gate builds. The claim is about a
    /// `HashMap`, so the test may honestly need one.
    #[cfg(feature = "std")]
    #[test]
    fn a_map_keyed_by_event_type_is_probed_by_str() {
        let mut registry = std::collections::HashMap::new();
        registry.insert(EventType::from_static("CourseDefined"), "decode");
        // The probe that does not allocate and does not re-validate, which is
        // the whole reason the impl exists.
        assert_eq!(registry.get("CourseDefined"), Some(&"decode"));
        // And an owned key inserted the other way is the same key.
        assert!(registry.contains_key(&EventType::new("CourseDefined").unwrap()));
    }

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
