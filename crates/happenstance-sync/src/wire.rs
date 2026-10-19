//! The versioned envelope every replication message travels in.
//!
//! # What is here
//!
//! The envelope, per [ADR-0016](../../../../.kb/decisions/0016-the-wire-format.md)
//! §11 — [`FORMAT_VERSION`], [`Envelope`], its hand-written [`Deserialize`] impl
//! and [`WireError`] — and, since #114, the message set at the end of the file.
//!
//! [`Envelope`] is generic in `T` and is **not** an enum of message kinds, and
//! the message set did not make it one. Each message is its own type, wrapped as
//! `Envelope<PushBatch>`, `Envelope<Ack>`, `Envelope<PullRequest>` or
//! `Envelope<Pulled<ResumeToken>>`: an enum of kinds inside the versioned type
//! would make adding a kind a shape change, and so a version bump every peer not
//! yet redeployed refuses. Which message a buffer holds is the transport's to say
//! — the route, or the request it answers (ADR-0097). [`SyncError`](crate::SyncError)
//! is still not extended.
//!
//! # The obligation, and why it is discharged differently in each format
//!
//! WF-8 says a receiver must read and check the version **before any part of the
//! message is decoded**. That single sentence has two spellings, because the two
//! formats this workspace encodes into do not agree on what "first" means.
//!
//! * In a positional format — postcard — fields are written in declaration
//!   order and nothing else is recoverable, so `format_version` being declared
//!   first is what makes `postcard::take_from_bytes::<u16>` on the front of the
//!   buffer yield it. That is a property of the *struct declaration*, which is
//!   why the field order below is load-bearing rather than cosmetic.
//! * In a self-describing format — JSON — key order carries no meaning at all.
//!   Measured: `{"message":null,"format_version":999}` decodes to `Ok` under a
//!   derived impl. So position cannot be the obligation there, and it is
//!   discharged instead by the explicit check in [`Envelope`]'s hand-written
//!   `Deserialize`, before the message is touched.
//!
//! The measurements behind both statements are in
//! `experiments/wire-format/tests/w6_envelope_varint.rs` and
//! `decorative_envelope_witness.rs`.

use core::fmt;
use core::marker::PhantomData;

use serde::Serialize;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

/// The version stamped on every envelope this build encodes, and the only one
/// it decodes.
///
/// # What bumping it means
///
/// Carried on every message, never negotiated per connection, and version 2 is
/// this envelope and the message set below it (ADR-0097). It is bumped whenever
/// the **shape** of any wire type changes: a field added, removed, retyped or
/// moved between types — per-type versioning cannot say VT-30's move of `after`
/// into `Guard`, which changed no single type. The set this version names
/// includes [`Pulled::waiting`](crate::Pulled::waiting) on the pulled message.
///
/// # What must never bump it
///
/// A change to a **capacity bound**. That is WF-9's territory, and it is not
/// this constant's: an over-capacity value decodes to `Ok` and is refused by the
/// store that cannot hold it, with an error naming the store. Conflating the two
/// makes every peer in a heterogeneous deployment unreachable the moment one of
/// them raises a limit — the raiser bumps the version, and every peer that has
/// not been redeployed refuses every message from it, including the ones well
/// inside the old bound.
pub const FORMAT_VERSION: u16 = 2;

/// The field names `Envelope`'s `Deserialize` announces to the format, in
/// declaration order.
const FIELDS: &[&str] = &["format_version", "message"];

/// A replication message, wrapped with the wire format version that produced it.
///
/// Field order is part of the contract: `format_version` is declared first so
/// that a positional encoder writes it first. See the module documentation for
/// why that is only half of WF-8's obligation.
///
/// ```
/// use happenstance_sync::wire::{Envelope, FORMAT_VERSION};
///
/// let envelope = Envelope::new("hello");
/// assert_eq!(envelope.format_version(), FORMAT_VERSION);
///
/// let json = serde_json::to_string(&envelope)?;
/// assert_eq!(json, r#"{"format_version":2,"message":"hello"}"#);
///
/// let decoded: Envelope<String> = serde_json::from_str(&json)?;
/// assert_eq!(decoded.message(), "hello");
/// # Ok::<(), serde_json::Error>(())
/// ```
///
/// A version this build does not implement is refused, and the message inside is
/// never decoded:
///
/// ```
/// use happenstance_sync::wire::{Envelope, WireError};
///
/// // What a peer one format version ahead would send.
/// let json = r#"{"format_version":999,"message":"hello"}"#;
/// let error = serde_json::from_str::<Envelope<String>>(json).unwrap_err();
/// assert!(error.to_string().contains(WireError::UNSUPPORTED_FORMAT_VERSION));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Envelope<T> {
    format_version: u16,
    message: T,
}

impl<T> Envelope<T> {
    /// Wraps `message` and stamps [`FORMAT_VERSION`].
    ///
    /// There is deliberately no constructor that stamps an arbitrary version: a
    /// sender that can name a version it does not implement can send one, and
    /// nothing downstream could tell that from a peer that genuinely speaks it.
    #[must_use]
    pub const fn new(message: T) -> Self {
        Self {
            format_version: FORMAT_VERSION,
            message,
        }
    }

    /// The version this envelope was encoded at.
    ///
    /// Always [`FORMAT_VERSION`] — a decoded envelope carrying anything else
    /// does not exist, because decoding refuses it.
    #[must_use]
    pub const fn format_version(&self) -> u16 {
        self.format_version
    }

    /// The message the envelope carries.
    #[must_use]
    pub const fn message(&self) -> &T {
        &self.message
    }

    /// Unwraps the envelope, discarding the version.
    #[must_use]
    pub fn into_message(self) -> T {
        self.message
    }
}

/// Checks a version read off the wire against the one this build speaks.
///
/// Exposed because a receiver that reads the version itself — peeking the front
/// of a postcard buffer, say, to route a message it will not decode — needs the
/// same answer as the decoder, and needs it as a typed error rather than as
/// prose.
///
/// # Errors
///
/// [`WireError::UnsupportedFormatVersion`] when `found` is not
/// [`FORMAT_VERSION`].
pub const fn check_format_version(found: u16) -> Result<(), WireError> {
    if found == FORMAT_VERSION {
        Ok(())
    } else {
        Err(WireError::UnsupportedFormatVersion {
            found,
            supported: FORMAT_VERSION,
        })
    }
}

/// Why a wire-level decode was refused.
///
/// Separate from [`SyncError`](crate::SyncError) on purpose. That enum is
/// ADR-0026's and describes how a peer *exchange* failed; a version refusal is a
/// decoding failure that happens before any exchange semantics apply. Folding
/// this in later is a phase-13 decision about the message set, and it supersedes
/// nothing to make it — whereas adding a variant to `SyncError` now would take
/// that decision inside an encoding change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
#[non_exhaustive]
pub enum WireError {
    /// The sender speaks a format version this build does not implement.
    ///
    /// The whole message is refused; nothing inside it has been decoded. A
    /// receiver may park the bytes and retry after an upgrade, which is the
    /// reason this must be distinguishable from a malformed message — those
    /// bytes will never decode, and parking them is a leak.
    #[error(
        "{} {found}, and this build speaks {supported}",
        WireError::UNSUPPORTED_FORMAT_VERSION
    )]
    UnsupportedFormatVersion {
        /// The version the sender stamped.
        found: u16,
        /// The version this build implements, i.e. [`FORMAT_VERSION`].
        supported: u16,
    },
}

impl WireError {
    /// The stable text that marks a version refusal in a decoder's error.
    ///
    /// `serde`'s `Deserialize` has exactly one channel out — `Error::custom`,
    /// which takes a `Display` and erases the type — so a typed [`WireError`]
    /// cannot cross the boundary of [`Envelope`]'s `Deserialize`. What crosses
    /// is this text, and it is public precisely so that "refused, park this"
    /// stays distinguishable from "malformed, drop this" without callers
    /// matching on a message they had to copy out of the source.
    ///
    /// # It crosses `serde_json`, and it does not cross `postcard`
    ///
    /// Measured by `wire::rejects_an_unknown_format_version`, which pins both
    /// halves. `postcard::Error` is a fieldless enum, so its `Error::custom`
    /// discards the `Display` it is given and every refusal arrives as
    /// `SerdeDeCustom` — the text is dropped by the format, not by this module,
    /// and no wording here could survive it. A postcard receiver gets its typed
    /// answer the other way round: read the version off the front of the buffer
    /// with `postcard::take_from_bytes::<u16>` and hand it to
    /// [`check_format_version`], which is the reason that function is public.
    pub const UNSUPPORTED_FORMAT_VERSION: &'static str = "unsupported wire format version";
}

/// The hand-written half of the format, and the reason this module exists.
///
/// **Do not replace this with `#[derive(Deserialize)]`.** The derive is a code
/// generator with no interception point: it reads *every* field into a local
/// `Option` and only then constructs the struct, so a derived `Envelope<T>`
/// decodes the message before anything examines the version — the partial decode
/// WF-8's MUST NOT forbids. `default`, `deny_unknown_fields` and `flatten` are
/// the hooks the derive offers, and none of them is "stop here and decide".
///
/// A derive plus a version check afterwards looks identical from outside and is
/// not: measured, it returns the same `Err` in both formats and produces
/// byte-identical postcard framing — `[01 07]` at version 1, `[80 03 07]` at 384,
/// the same `take_from_bytes::<u16>` value and the same `[07]` remainder. The
/// one observation that separates them is whether `T::deserialize` ran at all,
/// which is what `wire::version_is_readable_before_the_message` witnesses.
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Envelope<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_struct("Envelope", FIELDS, EnvelopeVisitor(PhantomData))
    }
}

struct EnvelopeVisitor<T>(PhantomData<T>);

impl<'de, T: Deserialize<'de>> Visitor<'de> for EnvelopeVisitor<T> {
    type Value = Envelope<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a versioned replication envelope")
    }

    /// The positional arm. The version is read and checked before
    /// `next_element::<T>` is called, so an unsupported version returns without
    /// the message's `Deserialize` ever running.
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let found: u16 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        check_format_version(found).map_err(de::Error::custom)?;

        let message: T = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;

        Ok(Envelope {
            format_version: found,
            message,
        })
    }

    /// The self-describing arm, where key order is not observable and the
    /// obligation is therefore this refusal rather than a position: `message` is
    /// not read until a version has been seen *and* accepted, whichever order
    /// the sender's encoder happened to emit the keys in.
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut format_version: Option<u16> = None;
        let mut message: Option<T> = None;

        while let Some(field) = map.next_key::<Field>()? {
            match field {
                Field::FormatVersion => {
                    if format_version.is_some() {
                        return Err(de::Error::duplicate_field(FIELDS[0]));
                    }
                    let found: u16 = map.next_value()?;
                    check_format_version(found).map_err(de::Error::custom)?;
                    format_version = Some(found);
                }
                Field::Message => {
                    if message.is_some() {
                        return Err(de::Error::duplicate_field(FIELDS[1]));
                    }
                    if format_version.is_none() {
                        return Err(de::Error::custom(
                            "the message arrived before an accepted format_version",
                        ));
                    }
                    message = Some(map.next_value()?);
                }
                Field::Unknown => {
                    map.next_value::<de::IgnoredAny>()?;
                }
            }
        }

        Ok(Envelope {
            format_version: format_version.ok_or_else(|| de::Error::missing_field(FIELDS[0]))?,
            message: message.ok_or_else(|| de::Error::missing_field(FIELDS[1]))?,
        })
    }
}

/// One key of the envelope, resolved without allocating.
///
/// Written out rather than taken as a `String` because a compact self-describing
/// format may send field indices instead of names, and because borrowing a key
/// out of the input is not available to every deserializer.
enum Field {
    FormatVersion,
    Message,
    Unknown,
}

impl<'de> Deserialize<'de> for Field {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

struct FieldVisitor;

impl Visitor<'_> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("`format_version` or `message`")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(match value {
            "format_version" => Field::FormatVersion,
            "message" => Field::Message,
            _ => Field::Unknown,
        })
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<Self::Value, E> {
        Ok(match value {
            b"format_version" => Field::FormatVersion,
            b"message" => Field::Message,
            _ => Field::Unknown,
        })
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        Ok(match value {
            0 => Field::FormatVersion,
            1 => Field::Message,
            _ => Field::Unknown,
        })
    }
}

// ===========================================================================
// The message set (#114, ADR-0097)
//
// Below the envelope rather than beside it at the top of the file, so the
// lines the specification cites above keep their numbers.
// ===========================================================================

/// Where the far side should resume a pull from, as the far side minted it.
///
/// Opaque: the caller stores it and hands it back, and never reads it (SY-15).
/// Text rather than bytes, because a token is what an HTTP peer puts in a query
/// string or a Durable Object in a cursor column, and because bytes would render
/// in JSON as an array of integers that no operator can read. It is the wire
/// form of a peer's [`SyncPeer::Resume`](crate::SyncPeer::Resume); turning one
/// into the other is the peer adapter's, and nothing here assumes how.
///
/// # Examples
///
/// ```
/// use happenstance_sync::wire::ResumeToken;
///
/// let token = ResumeToken::new("group:42");
/// assert_eq!(token.as_str(), "group:42");
/// assert_eq!(serde_json::to_string(&token)?, r#""group:42""#);
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ResumeToken(Box<str>);

impl ResumeToken {
    /// Wraps the text a peer minted.
    #[must_use]
    pub fn new(token: impl Into<Box<str>>) -> Self {
        Self(token.into())
    }

    /// The text, to hand back to the peer that minted it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Asks the far side for one round trip's worth of what it holds.
///
/// `from` is `None` on a first pull and the last [`Pulled`](crate::Pulled)'s
/// token after that. Always present on the wire, `None` included (WF-2).
///
/// # The whole message set
///
/// One type per message, each travelling as `Envelope<T>`; this is the only
/// one of the four that exists for the wire alone.
///
/// | Message | Type | Answers |
/// |---|---|---|
/// | push | [`PushBatch`](crate::PushBatch) | — |
/// | ack | [`Ack`](crate::Ack) | a push |
/// | pull | [`PullRequest`] | — |
/// | pulled | [`Pulled<ResumeToken>`](crate::Pulled) | a pull |
///
/// Four messages for the two exchanges [`SyncPeer`](crate::SyncPeer) has. A
/// peer's [`limits`](crate::SyncPeer::limits) is not one of them: how a remote
/// peer's limits reach the caller is the transport's (#117), and a fifth
/// message is additive because no enum holds the four.
///
/// Pulled carries [`waiting`](crate::Pulled::waiting) after its token, always
/// present (SY-37, WF-2): what the far side still holds, so a
/// runner told [`Nothing`](crate::Waiting::Nothing) skips the empty pull.
///
/// # A mis-routed buffer
///
/// A buffer does not say which message it holds, so a transport that routes one
/// to the wrong decoder gets whatever that decoder makes of it. In JSON this
/// type refuses unknown fields, so a push handed to it is an error rather than
/// `PullRequest { from: None }`, which its one optional field would otherwise
/// default to. Postcard has no equivalent: it carries no field names, and an
/// empty push and a first pull are both the version and then `00`
/// (ADR-0097 §1).
///
/// # Examples
///
/// A first pull, and the one after it:
///
/// ```
/// use happenstance_sync::wire::{Envelope, PullRequest, ResumeToken};
///
/// let first = Envelope::new(PullRequest::from_start());
/// assert_eq!(
///     serde_json::to_string(&first)?,
///     r#"{"format_version":2,"message":{"from":null}}"#
/// );
///
/// let next = PullRequest::after(ResumeToken::new("group:42"));
/// assert_eq!(next.from.as_ref().map(ResumeToken::as_str), Some("group:42"));
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct PullRequest {
    /// Where to resume from; `None` for the start of the far side's log.
    pub from: Option<ResumeToken>,
}

impl PullRequest {
    /// A pull from the start of the far side's log.
    #[must_use]
    pub const fn from_start() -> Self {
        Self { from: None }
    }

    /// A pull that resumes where `token` says.
    #[must_use]
    pub const fn after(token: ResumeToken) -> Self {
        Self { from: Some(token) }
    }
}
