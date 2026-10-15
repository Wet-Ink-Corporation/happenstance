//! A fixture peer with a cap on each value: Turnstile's peer D.
//!
//! SY-18's `[DEFERRED]` marker names the experiment: a KV-backed store with a
//! 128 KiB value cap, against an origin that has already committed a 340 KB
//! payload. This is that store, modelled rather than bound: the cap is checked
//! on a value length computed from the event, and nothing is encoded.

use alloc::format;
use alloc::sync::Arc;
use alloc::vec;
use core::fmt;

use happenstance_core::{
    Event, EventId, EventType, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH,
    MIN_SUPPORTED_TAGS_PER_EVENT, RecordedAt, SequencePosition, StoreId,
};
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, MemoryPeerError, MemoryResume, MemorySyncPeer,
    PeerLimits, Pulled, PushBatch, ReplicatedEvent, SendSyncPeer,
};

use crate::{Refusal, RefusalsOf, SyncFixture, Violation, oversized_push_refusal};

/// The per-value cap of Turnstile peer D's KV store: 128 KiB.
pub const KV_VALUE_CAP: usize = 128 * 1024;

/// Bytes of a value that are not the event's type, tags or data: the origin
/// `StoreId` (16), its position (8) and the recorded time (8).
const KEY_OVERHEAD: usize = 32;

/// What the envelope may occupy at the contract's floors: a type of
/// `MAX_EVENT_TYPE_LEN` bytes and `MIN_SUPPORTED_TAGS_PER_EVENT` tags of
/// `MAX_TAG_LEN` bytes each, and the key. 16,607 bytes.
const ENVELOPE_ALLOWANCE: usize =
    MAX_EVENT_TYPE_LEN + MIN_SUPPORTED_TAGS_PER_EVENT * MAX_TAG_LEN + KEY_OVERHEAD;

/// The payload this peer declares: what a value holds once the envelope is
/// reserved. 114,465 bytes.
const DECLARED_EVENT_BYTES: usize = KV_VALUE_CAP - ENVELOPE_ALLOWANCE;

/// 1 MiB, above the 340 KB seat map, so that what refuses the seat map is the
/// value cap and not the batch limit.
const DECLARED_BATCH_BYTES: usize = 1024 * 1024;

/// The bytes Turnstile's committed seat map carries: 340,000.
pub(crate) const TURNSTILE_SEAT_MAP_BYTES: usize = 340_000;

const SEAT_MAP_PUBLISHED: EventType = EventType::from_static("SeatMapPublished");

/// The origin of the seat map [`KvCappedFixture::refusals`] pushes.
const REFUSAL_ORIGIN: StoreId = StoreId::from_bytes([0xD0; 16]);

/// What a KV value holding `event` would occupy: its data and metadata, its
/// type, its tags and [`KEY_OVERHEAD`].
///
/// Saturating, because a sum that wrapped would admit what it should refuse.
fn value_len(event: &ReplicatedEvent) -> usize {
    let tags = event
        .event
        .tags()
        .iter()
        .map(|tag| tag.as_str().len())
        .fold(0, usize::saturating_add);
    event
        .payload_len()
        .saturating_add(event.event.event_type().as_str().len())
        .saturating_add(tags)
        .saturating_add(KEY_OVERHEAD)
}

/// Turnstile's committed seat map: [`TURNSTILE_SEAT_MAP_BYTES`] of opaque zero
/// bytes (SY-35), the first event in `origin`'s log.
///
/// # Errors
///
/// A [`Violation`] of `clause` if the event cannot be built. Unreachable: the
/// type is a valid literal.
pub(crate) fn turnstile_seat_map(
    origin: StoreId,
    clause: &'static str,
) -> Result<ReplicatedEvent, Violation> {
    let event = Event::new(SEAT_MAP_PUBLISHED, vec![0_u8; TURNSTILE_SEAT_MAP_BYTES])
        .map_err(|error| Violation::new(clause, format!("the seat map is invalid: {error}")))?;
    Ok(ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        event,
    ))
}

/// What a [`KvCappedPeer`] declares.
fn declared() -> PeerLimits {
    let mut limits = PeerLimits::minimum();
    limits.max_event_bytes = DECLARED_EVENT_BYTES;
    limits.max_batch_bytes = DECLARED_BATCH_BYTES;
    limits.max_batch_events = MIN_SUPPORTED_EVENTS_PER_BATCH;
    limits
}

/// A [`SyncFixture`] whose peer stores each event as one KV value of at most
/// [`KV_VALUE_CAP`] bytes: Turnstile's peer D, the shape SY-18's marker names.
///
/// # What it declares, and why
///
/// A value holds the event's type, its tags and a 32-byte key as well as its
/// payload, and [`PeerLimits`] has a field for the payload alone. So the peer
/// reserves the envelope at the contract's floors and declares what is left:
///
/// | | bytes |
/// |---|---:|
/// | value cap | 131,072 |
/// | type, at `MAX_EVENT_TYPE_LEN` | 255 |
/// | 64 tags, at `MAX_TAG_LEN` each | 16,320 |
/// | key: origin, position, recorded time | 32 |
/// | **declared `max_event_bytes`** | **114,465** |
///
/// An event at every floor at once fills a value exactly, and is accepted. The
/// declaration clears VT-21's 65,536-byte floor with 48,929 bytes to spare, and
/// sits in the peer set's spread as: floor 65,536 = [`MemorySyncPeer`] < this
/// peer 114,465 < the Neon store's 131,072 < 1 MiB for SQLite, Postgres and
/// Cloudflare.
///
/// # What it refuses
///
/// A push holding any event whose value would exceed the cap is refused whole,
/// before anything is stored, with [`KvPeerError::ValueTooLarge`], class
/// [`FailureClass::Park`]. It never truncates. A push over `max_batch_bytes`
/// is refused by the store behind the cap, class [`FailureClass::Split`].
#[derive(Debug, Clone)]
pub struct KvCappedFixture {
    peer: Arc<MemorySyncPeer>,
}

impl KvCappedFixture {
    /// A fixture over a fresh capped peer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            peer: Arc::new(MemorySyncPeer::with_limits(
                StoreId::from_bytes([0xC4; 16]),
                declared(),
            )),
        }
    }
}

impl Default for KvCappedFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncFixture for KvCappedFixture {
    type Peer = KvCappedPeer;

    async fn connect(&self) -> Self::Peer {
        KvCappedPeer {
            inner: Arc::clone(&self.peer),
        }
    }

    /// The oversized push, declared `Split`, and the Turnstile seat map,
    /// declared `Park`: 340,000 bytes is under the batch limit and over a
    /// value, so it is the value cap that refuses it.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        let mut refusals = oversized_push_refusal(peer).await?;
        let seat_map = turnstile_seat_map(REFUSAL_ORIGIN, "SY-36")?;
        let batch = PushBatch::new(vec![EventGroup::new(None, vec![seat_map])]);
        refusals.push(Refusal {
            provoked_by: "the Turnstile seat map, 340,000 bytes against a 131,072-byte value",
            declared: FailureClass::Park,
            outcome: peer.push(&batch).await.map(drop),
        });
        Ok(refusals)
    }
}

/// One handle onto a [`KvCappedFixture`]'s peer.
///
/// `Arc` for the reason [`MemorySyncHandle`](crate::MemorySyncHandle) gives:
/// many handles share one log, and the peer stays `Send + Sync`.
#[derive(Debug, Clone)]
pub struct KvCappedPeer {
    inner: Arc<MemorySyncPeer>,
}

impl SendSyncPeer for KvCappedPeer {
    type Error = KvPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        SendSyncPeer::pull(&*self.inner, from)
            .await
            .map_err(KvPeerError::Store)
    }

    /// Checks every value before storing any, so a refused push stores nothing
    /// and a group is never split by the cap.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        if let Some((event, len)) = batch
            .events()
            .map(|event| (event, value_len(event)))
            .find(|(_, len)| *len > KV_VALUE_CAP)
        {
            return Err(KvPeerError::ValueTooLarge {
                id: event.id,
                value_len: len,
                cap: KV_VALUE_CAP,
            });
        }
        SendSyncPeer::push(&*self.inner, batch)
            .await
            .map_err(KvPeerError::Store)
    }

    fn limits(&self) -> PeerLimits {
        SendSyncPeer::limits(&*self.inner)
    }
}

/// How a [`KvCappedPeer`] fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum KvPeerError {
    /// One event's value would exceed the cap. Nothing in the push was stored.
    ValueTooLarge {
        /// The event that cannot land here.
        id: EventId,
        /// What its value would occupy.
        value_len: usize,
        /// What a value may occupy.
        cap: usize,
    },
    /// The in-memory store behind the cap refused: an oversized batch, or a
    /// poisoned lock.
    Store(MemoryPeerError),
}

impl fmt::Display for KvPeerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValueTooLarge { id, value_len, cap } => write!(
                f,
                "event {id} needs a value of {value_len} bytes, over this peer's cap of {cap}"
            ),
            Self::Store(_) => f.write_str("the store behind the value cap refused"),
        }
    }
}

impl core::error::Error for KvPeerError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::ValueTooLarge { .. } => None,
            Self::Store(inner) => Some(inner),
        }
    }
}

impl Classify for KvPeerError {
    fn class(&self) -> FailureClass {
        match self {
            // No retry or split makes this event fit a value (VT-25).
            Self::ValueTooLarge { .. } => FailureClass::Park,
            Self::Store(inner) => inner.class(),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::vec;
    use alloc::vec::Vec;

    use happenstance_core::{
        Event, EventId, EventType, MAX_EVENT_TYPE_LEN, MAX_TAG_LEN, MIN_SUPPORTED_EVENT_DATA_LEN,
        MIN_SUPPORTED_TAGS_PER_EVENT, RecordedAt, SequencePosition, StoreId, Tag, Tags,
    };
    use happenstance_sync::{
        Classify, EventGroup, FailureClass, MemoryPeerError, PushBatch, ReplicatedEvent,
        SendSyncPeer,
    };
    use happenstance_testkit::block_on;

    use super::{
        KV_VALUE_CAP, KvCappedFixture, KvCappedPeer, KvPeerError, TURNSTILE_SEAT_MAP_BYTES,
        turnstile_seat_map,
    };
    use crate::{SyncFixture, batch_over_the_limit};

    const ORIGIN: StoreId = StoreId::from_bytes([0xE1; 16]);
    const SMALL_ORIGIN: StoreId = StoreId::from_bytes([0xE2; 16]);

    fn peer() -> KvCappedPeer {
        block_on(KvCappedFixture::new().connect())
    }

    /// An event at every envelope floor the contract sets: a type of
    /// `MAX_EVENT_TYPE_LEN` bytes and `MIN_SUPPORTED_TAGS_PER_EVENT` distinct
    /// tags of `MAX_TAG_LEN` bytes, carrying `size` bytes of data.
    fn at_the_floors(size: usize) -> ReplicatedEvent {
        let event_type =
            EventType::new("T".repeat(MAX_EVENT_TYPE_LEN)).expect("255 bytes is a valid type");
        let tags: Tags = (0..MIN_SUPPORTED_TAGS_PER_EVENT)
            .map(|i| Tag::new(format!("{i:0>255}")).expect("255 digits is a valid tag"))
            .collect();
        assert_eq!(tags.len(), MIN_SUPPORTED_TAGS_PER_EVENT);
        let event = Event::new(event_type, vec![0_u8; size])
            .expect("the type is valid")
            .with_tags(tags);
        ReplicatedEvent::new(
            EventId::new(ORIGIN, SequencePosition::FIRST),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        )
    }

    fn small() -> ReplicatedEvent {
        ReplicatedEvent::new(
            EventId::new(SMALL_ORIGIN, SequencePosition::FIRST),
            RecordedAt::from_millis(1_800_000_000_000),
            Event::new(EventType::from_static("Small"), vec![1_u8; 8]).expect("valid"),
        )
    }

    fn held(peer: &KvCappedPeer) -> Vec<EventGroup> {
        let pulled = block_on(peer.pull(None)).expect("an in-memory pull succeeds");
        pulled.batch.groups
    }

    #[test]
    fn the_declaration_reserves_the_envelope_at_the_contract_floors() {
        let limits = peer().limits();
        assert_eq!(limits.max_event_bytes, 114_465);
        assert!(limits.max_event_bytes >= MIN_SUPPORTED_EVENT_DATA_LEN);
        assert_eq!(
            MAX_TAG_LEN, 255,
            "the arithmetic below assumes the contract's tag bound"
        );
    }

    #[test]
    fn an_event_at_every_floor_fills_a_value_exactly() {
        let peer = peer();
        let event = at_the_floors(peer.limits().max_event_bytes);
        assert_eq!(super::value_len(&event), KV_VALUE_CAP);

        let batch = PushBatch::new(vec![EventGroup::new(None, vec![event.clone()])]);
        let ack = block_on(peer.push(&batch)).expect("exactly the cap is accepted");
        assert_eq!(ack.appended, 1);
        assert_eq!(held(&peer), vec![EventGroup::new(None, vec![event])]);
    }

    #[test]
    fn one_byte_more_is_refused_whole_and_parked() {
        let peer = peer();
        let event = at_the_floors(peer.limits().max_event_bytes + 1);
        let id = event.id;
        let batch = PushBatch::new(vec![EventGroup::new(None, vec![small(), event])]);

        let error = block_on(peer.push(&batch)).expect_err("one byte over the cap is refused");
        assert!(
            matches!(
                error,
                KvPeerError::ValueTooLarge { id: refused, value_len: 131_073, cap: 131_072 }
                    if refused == id
            ),
            "{error:?}"
        );
        assert_eq!(error.class(), FailureClass::Park);
        assert_eq!(
            held(&peer),
            Vec::new(),
            "nothing in a refused push is stored"
        );
    }

    #[test]
    fn the_seat_map_is_refused_for_its_value_not_the_batch() {
        let peer = peer();
        let seat_map = turnstile_seat_map(ORIGIN, "SY-18").expect("the seat map builds");
        let batch = PushBatch::new(vec![EventGroup::new(None, vec![seat_map])]);
        assert!(batch.payload_len() <= peer.limits().max_batch_bytes);

        // 340,000 of data, a 16-byte type, no tags and the 32-byte key.
        let expected = TURNSTILE_SEAT_MAP_BYTES + "SeatMapPublished".len() + 32;
        assert_eq!(expected, 340_048);

        let error = block_on(peer.push(&batch)).expect_err("340 KB is over a 128 KiB value");
        assert!(
            matches!(
                error,
                KvPeerError::ValueTooLarge { value_len, cap: KV_VALUE_CAP, .. }
                    if value_len == expected
            ),
            "{error:?}"
        );
    }

    #[test]
    fn an_oversized_batch_is_split() {
        let peer = peer();
        let batch = batch_over_the_limit(&peer.limits()).expect("1 MiB is under the ceiling");

        let error = block_on(peer.push(&batch)).expect_err("one byte over the batch limit");
        assert!(
            matches!(
                error,
                KvPeerError::Store(MemoryPeerError::BatchTooLarge { .. })
            ),
            "{error:?}"
        );
        assert_eq!(error.class(), FailureClass::Split);
    }
}
