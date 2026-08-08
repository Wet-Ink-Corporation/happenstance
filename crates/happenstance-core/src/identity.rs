//! Who assigned an event, and when it was recorded.
//!
//! Three store-assigned values. None of them is ever supplied by a caller
//! through [`append`](crate::EventStore::append): a foreign identity arrives
//! through the ingest port in the replication crate, which is the seam that
//! keeps the contract's write path from having to distinguish "I am writing
//! this" from "someone else wrote this and I am copying it".

use core::fmt;

use crate::event::SequencePosition;

/// Identifies one **incarnation** of a store's persistent state.
///
/// Not a device, and not a peer. The distinction is the whole clause: a tablet
/// restored from a backup, or a disk image cloned onto a second machine, will
/// reissue the same origin positions to different events. If a `StoreId`
/// survived that, two genuinely different events would carry one `EventId`, and
/// every peer's deduplication would silently drop real facts — no error, no
/// conflict, just missing history.
///
/// So it is minted when a store's persistent state is created, and **re-minted
/// when that state is restored or cloned**. An adapter that cannot detect a
/// restore must offer an out-of-band operation to re-mint, and must say so.
///
/// Never derive one from a hostname, a device identifier, a peer name, or
/// anything else that survives a restore.
///
/// # Representation
///
/// `[u8; 16]` rather than `u128`, which is the choice a reader is most likely to
/// assume was made by default. Three reasons, none about size:
///
/// * A `u128` has a byte order and an array does not. Every adapter persisting
///   one must choose `to_be_bytes` or `to_le_bytes`, and two adapters choosing
///   differently produce logs that disagree about identity while both pass every
///   in-process rule.
/// * `Ord` on `[u8; 16]` is lexicographic over exactly the bytes an adapter
///   stores, so it agrees with the `BLOB`/`BYTEA` collation a database already
///   applies. `Ord` on a `u128` is numeric and agrees only under one endianness.
/// * An integer invites arithmetic — incrementing, ranging, comparing as a
///   magnitude — on a value whose 128 bits are opaque by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreId([u8; 16]);

impl StoreId {
    /// Wraps sixteen bytes minted by the adapter.
    ///
    /// There is deliberately no random constructor here. `happenstance-core` has
    /// no entropy source, is `no_std`-capable, and must not acquire a `getrandom`
    /// dependency to mint a value only an adapter is in a position to persist.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// The sixteen bytes, in the order an adapter should store them.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

impl fmt::Display for StoreId {
    /// Thirty-two lowercase hex characters, with no dashes.
    ///
    /// Deliberately **not** UUID-formatted. A `StoreId` has no version nibble
    /// and no variant bits, and formatting it with dashes invites an operator —
    /// or a downstream tool — to parse it as a UUID and read structure into it
    /// that is not there.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }
        Ok(())
    }
}

/// A store-assigned event identity: which store first accepted the event, and
/// at what position it did so.
///
/// Stable under replication. A store that accepts an event through ingest
/// preserves the `EventId` it arrived with, and mints one only for events
/// appended to it locally. That is what makes deduplication possible without
/// anybody parsing a payload.
///
/// # Why the fields are private
///
/// The two halves are one fact, not two, and a transparent pair would let the
/// compiler accept `EventId { store: theirs, position: mine }` without a word —
/// exactly the value this type exists to make deliberate. Both accessors return
/// by value; both halves are `Copy`.
///
/// This is not in tension with [`SequencedEvent`](crate::SequencedEvent)'s
/// public fields: those are four independent facts a reader wants by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId {
    store: StoreId,
    position: SequencePosition,
}

impl EventId {
    /// Pairs the minting store with the position it assigned.
    #[must_use]
    pub const fn new(store: StoreId, position: SequencePosition) -> Self {
        Self { store, position }
    }

    /// The incarnation that first accepted this event.
    #[must_use]
    pub const fn store(self) -> StoreId {
        self.store
    }

    /// The position the minting store assigned.
    ///
    /// **Authorship order, not arrival order.** For a locally appended event
    /// this equals the enclosing [`SequencedEvent`](crate::SequencedEvent)'s
    /// `position`; for an event that arrived through ingest it does not, and the
    /// two carry different information — this one orders the event in its origin
    /// store, the other orders it here.
    #[must_use]
    pub const fn position(self) -> SequencePosition {
        self.position
    }
}

impl fmt::Display for EventId {
    /// `store:position`, with the store as thirty-two hex characters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.store, self.position)
    }
}

/// When the store accepted an event: milliseconds since the Unix epoch, UTC.
///
/// # This is not an ordering key
///
/// Nothing may derive an order from it, and the contract states no relationship
/// between `RecordedAt` order and [`SequencePosition`]
/// order. Two events in one batch may share a millisecond; a store whose clock
/// steps backwards may stamp a later event with an earlier time. Positions order
/// events. This records a fact about the world for an auditor, and answers "when
/// did this land", which no position can.
///
/// Preserved unchanged through ingest, so a replicated event keeps the time its
/// *origin* recorded rather than acquiring its arrival time here.
///
/// # Why `i64` and not `SystemTime`
///
/// `SystemTime` is not `no_std`, and this crate is `no_std`-capable. A `chrono`
/// or `jiff` dependency would be the largest in the contract crate and would buy
/// nothing the store needs — the store is the only thing that ever sets this,
/// and an adapter that has a clock can produce an integer from it. Signed rather
/// than unsigned because times before 1970 are representable rather than
/// wrapping, which costs nothing and removes a class of surprise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RecordedAt(i64);

impl RecordedAt {
    /// Wraps a count of milliseconds since the Unix epoch.
    #[must_use]
    pub const fn from_millis(millis: i64) -> Self {
        Self(millis)
    }

    /// Milliseconds since the Unix epoch.
    #[must_use]
    pub const fn as_millis(self) -> i64 {
        self.0
    }
}

impl fmt::Display for RecordedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}ms", self.0)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
mod serde_impls {
    //! The derives ship here; the *encoding* is phase 5's.
    //!
    //! Whether a `StoreId` crosses the wire as hex text or as a byte array, and
    //! a `RecordedAt` as a number or a string, is the wire format's question.
    //! Shipping the derives now means that phase changes an encoding rather than
    //! a type.

    use super::{EventId, RecordedAt, StoreId};
    use crate::event::SequencePosition;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for StoreId {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for StoreId {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            <[u8; 16]>::deserialize(deserializer).map(Self)
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "EventId")]
    struct EventIdWire {
        store: StoreId,
        position: SequencePosition,
    }

    impl Serialize for EventId {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            EventIdWire {
                store: self.store,
                position: self.position,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for EventId {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = EventIdWire::deserialize(deserializer)?;
            Ok(Self::new(wire.store, wire.position))
        }
    }

    impl Serialize for RecordedAt {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for RecordedAt {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            i64::deserialize(deserializer).map(Self)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::{EventId, RecordedAt, StoreId};
    use crate::event::SequencePosition;
    use alloc::string::ToString;

    fn store_id(first: u8) -> StoreId {
        let mut bytes = [0u8; 16];
        bytes[0] = first;
        bytes[15] = 0xff;
        StoreId::from_bytes(bytes)
    }

    #[test]
    fn store_id_renders_as_thirty_two_hex_characters() {
        let rendered = store_id(0x0a).to_string();
        assert_eq!(rendered.len(), 32);
        assert!(rendered.starts_with("0a"));
        assert!(rendered.ends_with("ff"));
        assert!(
            !rendered.contains('-'),
            "no dashes: a StoreId is not a UUID and must not invite being parsed as one"
        );
    }

    #[test]
    fn store_id_round_trips_through_bytes() {
        let bytes = [7u8; 16];
        assert_eq!(StoreId::from_bytes(bytes).to_bytes(), bytes);
    }

    #[test]
    fn store_id_orders_lexicographically_over_its_bytes() {
        // The property that lets a database's BLOB collation agree with `Ord`,
        // and the one a `u128` would only have under one endianness.
        let low = StoreId::from_bytes([0x00; 16]);
        let mut raised = [0x00; 16];
        raised[0] = 0x01;
        assert!(low < StoreId::from_bytes(raised));

        let mut last_byte = [0x00; 16];
        last_byte[15] = 0x01;
        assert!(low < StoreId::from_bytes(last_byte));
        assert!(StoreId::from_bytes(last_byte) < StoreId::from_bytes(raised));
    }

    #[test]
    fn event_id_keeps_its_halves_distinguishable() {
        let id = EventId::new(store_id(1), SequencePosition::new(42).unwrap());
        assert_eq!(id.store(), store_id(1));
        assert_eq!(id.position().get(), 42);
        assert!(id.to_string().ends_with(":42"));
    }

    #[test]
    fn event_ids_from_different_stores_at_one_position_are_different() {
        // The whole reason the pair is the identity: position alone is not
        // unique across stores, and a peer that deduplicates on it would drop
        // real facts.
        let position = SequencePosition::new(9).unwrap();
        assert_ne!(
            EventId::new(store_id(1), position),
            EventId::new(store_id(2), position)
        );
    }

    #[test]
    fn recorded_at_round_trips_and_admits_times_before_the_epoch() {
        assert_eq!(
            RecordedAt::from_millis(1_700_000_000_000).as_millis(),
            1_700_000_000_000
        );
        assert_eq!(RecordedAt::from_millis(-1).as_millis(), -1);
    }
}
