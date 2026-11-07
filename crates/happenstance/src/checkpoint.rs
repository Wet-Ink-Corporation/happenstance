//! The id a projection's checkpoint is read and committed under (PS-25).
//!
//! The runner does not key a checkpoint on the projection's name alone. It
//! keys it on the name and a digest of the query the projection derives, so
//! a projection whose query changes starts a new checkpoint and replays from
//! the beginning, rather than resuming past events it was never handed.

use happenstance_core::{
    InvalidProjectionId, InvalidQuery, MAX_PROJECTION_ID_LEN, ProjectionId, Query, QueryItem,
};

use crate::boundary::{AtLeastOneType, derive_query};
use crate::domain::DomainEvent;
use crate::runner::Projection;

/// The digest's rendering: `@` and 16 lowercase hex digits.
const SUFFIX_LEN: usize = 1 + 16;

/// The longest projection name a derived checkpoint id can carry.
///
/// 238 bytes, so that the name, `@` and 16 hex digits fit
/// [`MAX_PROJECTION_ID_LEN`]'s 255 (ADR-0082 §D5). A longer name is refused
/// by [`checkpoint_id`], never truncated.
pub const MAX_CHECKPOINT_NAME_LEN: usize = MAX_PROJECTION_ID_LEN - SUFFIX_LEN;

/// Terminates each event type and each tag (ASCII US).
const US: u8 = 0x1F;
/// Terminates an item's type section (ASCII RS).
const RS: u8 = 0x1E;
/// Terminates an item (ASCII GS).
const GS: u8 = 0x1D;
/// The whole encoding of [`Query::All`] (ASCII FS).
const FS: u8 = 0x1C;

/// FNV-1a's 64-bit offset basis.
const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
/// FNV-1a's 64-bit prime.
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

/// Why a checkpoint id could not be derived.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidCheckpointId {
    /// The projection's query could not be derived.
    ///
    /// Absorbed rather than unwrapped, as
    /// [`ProjectionError::Boundary`](crate::ProjectionError::Boundary) is:
    /// unreachable for a projection whose event type declares at least one
    /// type, which is a compile-time check.
    #[error(transparent)]
    Query(#[from] InvalidQuery),
    /// The name is longer than [`MAX_CHECKPOINT_NAME_LEN`].
    ///
    /// Refused, never truncated: a truncated name would let two projections
    /// share one checkpoint.
    #[error(
        "a projection name must be at most {max} bytes to derive a checkpoint id, got {len}",
        max = MAX_CHECKPOINT_NAME_LEN
    )]
    NameTooLong {
        /// The rejected name's length, in bytes.
        len: usize,
    },
    /// [`ProjectionId::new`] refused the derived id.
    ///
    /// In practice a name that is itself a `sync/` watermark id, which no
    /// derived id may begin with (ADR-0082 §D5).
    #[error("the derived checkpoint id was refused")]
    Refused(#[source] InvalidProjectionId),
}

/// The id `run_projection` reads and commits this projection's checkpoint
/// under.
///
/// It is the projection's [`id`](Projection::id), its name, followed by `@`
/// and 16 lowercase hex digits: a digest of the [`Query`] the runner derives
/// from the projection's event types and [`scope`](Projection::scope). A
/// projection whose query changes therefore derives a new id, reads
/// [`Checkpoint::NeverRun`] under it and replays from the start, rather than
/// resuming past events its old query never nominated.
///
/// Read a runner's checkpoint through this id. Reading it through
/// `projection.id()` reads a row the runner never writes.
///
/// ```
/// use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// use happenstance::{EventType, Tags, MemoryProjectionBatch};
/// use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// use happenstance::{Delivered, Projection, ProjectionId};
/// use happenstance::checkpoint_id;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// enum Seat { Taken }
///
/// const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// impl DomainEvent for Seat {
///     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
///     fn event_type(&self) -> EventType { SEAT_TAKEN }
///     fn tags(&self) -> Tags { Tags::empty() }
///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
///         c.encode(self)
///     }
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
/// }
///
/// struct Sold { id: ProjectionId, scope: Tags }
///
/// impl Projection for Sold {
///     type Event = Seat;
///     type Store = MemoryProjectionStore;
///     type Error = MemoryProjectionStoreError;
///     fn id(&self) -> &ProjectionId { &self.id }
///     fn scope(&self) -> &Tags { &self.scope }
///     async fn apply(
///         &mut self,
///         _: Delivered<Seat>,
///         _: &mut MemoryProjectionBatch,
///     ) -> Result<(), MemoryProjectionStoreError> {
///         Ok(())
///     }
/// }
///
/// let sold = Sold {
///     id: ProjectionId::new("seats_sold")?,
///     scope: Tags::empty(),
/// };
/// let key = checkpoint_id(&sold)?;
/// assert_eq!(key.as_str(), "seats_sold@f3ff331ff7883985");
/// # Ok(()) }
/// ```
///
/// # The encoding, and why it is injective
///
/// The digest is 64-bit FNV-1a over a byte encoding of the canonical query.
/// Each item is its event types in the item's sorted order, each followed by
/// ASCII US (`0x1F`); then RS (`0x1E`); then its tags in their sorted order,
/// each followed by US; then GS (`0x1D`). The query is its items' encodings,
/// deduplicated, sorted bytewise and concatenated, so neither item order nor
/// a repeated item changes the digest. [`Query::All`] is the one byte FS
/// (`0x1C`).
///
/// VT-14 keeps every control character out of every event type and tag, and
/// in UTF-8 a byte below `0x80` only ever encodes that ASCII character, so no
/// field contains a terminator. Each item therefore parses back uniquely, an
/// item ends at its only GS, so a concatenation of items parses back
/// uniquely, and FS begins no item. Two distinct canonical queries encode to
/// two distinct byte strings. Plain concatenation would not: it collides the
/// types `ab`, `c` with `a`, `bc`, and a type `x` with a tag `x`.
///
/// Terminators rather than length prefixes, because a `usize` length hashes
/// differently on `wasm32` and a 64-bit host, and the id must be the same on
/// both. One item over two types and two items over one type each match the
/// same events and digest differently. The runner only ever derives one item,
/// and the cost of the difference would be a rebuild, never a wrong resume.
///
/// # Why FNV-1a, and what is promised
///
/// FNV-1a is a dozen lines and no dependency. It is not a cryptographic hash
/// and does not need to be one: nothing adversarial chooses the queries of
/// one projection name, and two of them collide with odds of about one in
/// 2^64. The function, the encoding and the `@` separator are a promise:
/// changing any of them changes every derived id, and every deployment would
/// rebuild every view.
///
/// # Adopting a checkpoint written under the bare name
///
/// A runner before this one committed under `projection.id()`, and that row
/// reads [`Checkpoint::NeverRun`] under the derived id, so the view rebuilds.
/// To keep its position instead, copy the old checkpoint across with an
/// empty commit, which PS-21 permits. Do it **only when the query did not
/// change**, and **before the new runner first runs**: adopting across a
/// changed query re-creates exactly the resume this id exists to prevent.
/// The orphaned bare-name row is harmless, and a
/// [`reset`](happenstance_core::ProjectionStore::reset) clears it.
///
/// ```
/// # use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// # use happenstance::{Event, EventStore, EventType, Json, Tags};
/// # use happenstance::{MemoryEventStore, MemoryProjectionBatch};
/// # use happenstance::{MemoryProjectionStore, MemoryProjectionStoreError};
/// # use happenstance::{Delivered, Projection, ProjectionId};
/// # use happenstance::{Authority, Checkpoint, Chunk, ProjectionStore};
/// # use happenstance::{checkpoint_id, run_projection};
/// # use std::error::Error;
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
/// # #[derive(serde::Serialize, serde::Deserialize)]
/// # enum Seat { Taken }
/// # const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// # impl DomainEvent for Seat {
/// #     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
/// #     fn event_type(&self) -> EventType { SEAT_TAKEN }
/// #     fn tags(&self) -> Tags { Tags::empty() }
/// #     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
/// #         c.encode(self)
/// #     }
/// #     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
/// #         -> Result<Self, CodecError> { c.decode(d) }
/// # }
/// # struct Sold { id: ProjectionId, scope: Tags }
/// # impl Projection for Sold {
/// #     type Event = Seat;
/// #     type Store = MemoryProjectionStore;
/// #     type Error = MemoryProjectionStoreError;
/// #     fn id(&self) -> &ProjectionId { &self.id }
/// #     fn scope(&self) -> &Tags { &self.scope }
/// #     async fn apply(
/// #         &mut self,
/// #         _: Delivered<Seat>,
/// #         _: &mut MemoryProjectionBatch,
/// #     ) -> Result<(), MemoryProjectionStoreError> {
/// #         Ok(())
/// #     }
/// # }
/// let events = MemoryEventStore::new();
/// let taken = Event::new("SeatTaken", Json.encode(&Seat::Taken)?)?;
/// let written = events.append(&[taken], None).await?;
/// let models = MemoryProjectionStore::new();
/// let mut sold = Sold {
///     id: ProjectionId::new("seats_sold")?,
///     scope: Tags::empty(),
/// };
/// // What an earlier runner left behind, under the bare name.
/// let batch = models.begin().await?;
/// models.commit(batch, sold.id(), written, Authority::Live).await?;
///
/// // The adoption: the old position and authority, under the new id.
/// let key = checkpoint_id(&sold)?;
/// let (through, authority) = match models.checkpoint(sold.id()).await? {
///     Checkpoint::Live { through } => (through, Authority::Live),
///     Checkpoint::Rebuilding { through } => {
///         (through, Authority::Rebuilding)
///     }
///     _ => return Ok(()), // never run: nothing to adopt
/// };
/// let empty = models.begin().await?;
/// models.commit(empty, &key, through, authority).await?;
///
/// // The runner resumes where the old one stopped.
/// let chunk = Chunk::of(64.try_into()?);
/// let done =
///     run_projection(&events, &models, &mut sold, &Json, chunk)
///         .await?;
/// assert_eq!(done.applied, 0);
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns [`InvalidCheckpointId::NameTooLong`] when the name is longer than
/// [`MAX_CHECKPOINT_NAME_LEN`], [`InvalidCheckpointId::Refused`] when the
/// name is a `sync/` watermark id, and [`InvalidCheckpointId::Query`] when
/// the projection's query cannot be derived.
///
/// [`Checkpoint::NeverRun`]: happenstance_core::Checkpoint::NeverRun
pub fn checkpoint_id<P: Projection>(projection: &P) -> Result<ProjectionId, InvalidCheckpointId> {
    // Reading the `const` is what evaluates it, as the runner does.
    let () = AtLeastOneType::<P::Event>::CHECKED;
    let query = derive_query(P::Event::EVENT_TYPES, projection.scope())?;
    derive(projection.id(), &query)
}

/// The derivation the runner and [`checkpoint_id`] share.
///
/// # Errors
///
/// [`InvalidCheckpointId::NameTooLong`] past [`MAX_CHECKPOINT_NAME_LEN`], and
/// [`InvalidCheckpointId::Refused`] for whatever [`ProjectionId::new`] still
/// refuses after that check, which is only a reserved prefix.
pub(crate) fn derive(
    name: &ProjectionId,
    query: &Query,
) -> Result<ProjectionId, InvalidCheckpointId> {
    let len = name.as_str().len();
    if len > MAX_CHECKPOINT_NAME_LEN {
        return Err(InvalidCheckpointId::NameTooLong { len });
    }
    // Through the checking door and no other: an unchecked one would let a
    // `sync/` watermark name mint an id in the replication namespace.
    let digest = fnv1a64(&encode_query(query));
    ProjectionId::new(format!("{name}@{digest:016x}")).map_err(InvalidCheckpointId::Refused)
}

/// 64-bit FNV-1a over `bytes`.
///
/// The multiplication wraps because FNV is defined modulo 2^64: wrapping is
/// the function, not an overflow being hidden.
fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(FNV_OFFSET_BASIS, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(FNV_PRIME)
    })
}

/// One item's pinned encoding: `(type US)* RS (tag US)* GS`.
fn encode_item(item: &QueryItem) -> Vec<u8> {
    let mut out = Vec::new();
    for event_type in item.types() {
        out.extend_from_slice(event_type.as_str().as_bytes());
        out.push(US);
    }
    out.push(RS);
    for tag in item.tags() {
        out.extend_from_slice(tag.as_str().as_bytes());
        out.push(US);
    }
    out.push(GS);
    out
}

/// The query's pinned encoding.
///
/// FS for [`Query::All`]; otherwise its items' encodings, deduplicated,
/// sorted bytewise and concatenated.
fn encode_query(query: &Query) -> Vec<u8> {
    let Some(items) = query.items() else {
        return vec![FS];
    };
    let mut encoded: Vec<Vec<u8>> = items.iter().map(encode_item).collect();
    // The encoding is injective, so equal encodings are equal items and
    // deduplicating the bytes deduplicates the items.
    encoded.sort_unstable();
    encoded.dedup();
    encoded.concat()
}

#[cfg(test)]
mod tests;
