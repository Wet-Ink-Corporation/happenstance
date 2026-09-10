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
///
/// # It deliberately does not implement `PartialOrd` or `Ord`
///
/// The paragraph above is a rule, and until `0.2.0` this type carried a derive
/// that handed a caller the exact operation the rule forbids — sixteen lines
/// apart, with the affordance winning, because a derive is reachable and a
/// sentence is not. Nothing in this workspace ever ordered one.
///
/// A reader who wants events in the order they happened wants
/// [`SequencePosition`], and now gets told so by the compiler rather than by a
/// doc comment. Removing a derive is a breaking change, so this was the last
/// release at which it was free: nothing was published at a compatible version,
/// and no consumer was pinned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    //! A `StoreId` has two encodings and the *format* chooses between them
    //! (WF-6, ADR-0016 §9): thirty-two lowercase hex digits where a human will
    //! read them, sixteen raw bytes where nobody will. So human-readability is
    //! part of this format's identity — the same value is not the same bytes in
    //! JSON and in postcard.
    //!
    //! # The wrong implementation this exists to reject
    //!
    //! An **inverted** `is_human_readable` branch: hex in the binary arm, raw
    //! bytes in the human-readable one. Each arm is internally consistent, so
    //! encode-then-decode agrees with itself whichever one ran and a round-trip
    //! test passes in **both** formats — measured, on a deliberately inverted
    //! newtype, in `experiments/wire-format/tests/decorative_inverted_branch.rs`.
    //! That is why WF-6 names two rules asserting the bytes actually on the wire
    //! (`wire::store_id_encodes_as_hex_in_json` and
    //! `wire::store_id_encodes_as_bytes_in_postcard`) rather than one asserting
    //! a round trip.
    //!
    //! ```
    //! use happenstance_core::StoreId;
    //!
    //! let store = StoreId::from_bytes([
    //!     0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78,
    //!     0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0,
    //! ]);
    //!
    //! // JSON is human-readable: the `Display` rendering, quoted.
    //! let json = serde_json::to_string(&store)?;
    //! assert_eq!(json, "\"0f1e2d3c4b5a69788796a5b4c3d2e1f0\"");
    //! assert_eq!(serde_json::from_str::<StoreId>(&json)?, store);
    //!
    //! // postcard is not: sixteen raw bytes, and no length prefix.
    //! let binary = postcard::to_stdvec(&store)?;
    //! assert_eq!(binary, store.to_bytes());
    //!
    //! // Uppercase hex is refused rather than accepted quietly, so that one
    //! // value has exactly one human-readable spelling.
    //! assert!(serde_json::from_str::<StoreId>("\"0F1E2D3C4B5A69788796A5B4C3D2E1F0\"").is_err());
    //! // And a UUID rendering of the same bytes is not a `StoreId`.
    //! assert!(serde_json::from_str::<StoreId>("\"0f1e2d3c-4b5a-6978-8796-a5b4c3d2e1f0\"").is_err());
    //! # Ok::<(), Box<dyn core::error::Error>>(())
    //! ```

    use super::{EventId, RecordedAt, StoreId};
    use crate::event::SequencePosition;
    use alloc::string::String;
    use serde::de::{Error as _, Unexpected};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Thirty-two hex digits, which is `size_of::<StoreId>() * 2`.
    const HEX_LEN: usize = 32;

    /// One **lowercase** hex digit's value, or `None`.
    ///
    /// Uppercase is a rejection rather than an oversight. `u8::from_str_radix`
    /// and every hex helper in the ecosystem accept `A`–`F` silently, so a peer
    /// emitting `0F1E…` would round-trip perfectly and no round-trip test could
    /// see it — while two spellings of one `StoreId` reached the log, where
    /// anything comparing identities as text would call them different stores.
    /// One value, one rendering; the encoder only ever emits lowercase, so the
    /// decoder only ever accepts it.
    const fn nibble(digit: u8) -> Option<u8> {
        match digit {
            b'0'..=b'9' => Some(digit - b'0'),
            b'a'..=b'f' => Some(digit - b'a' + 10),
            _ => None,
        }
    }

    impl Serialize for StoreId {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            if serializer.is_human_readable() {
                // `collect_str` over the `Display` impl, which already renders
                // exactly the thirty-two lowercase digits — one definition of
                // the rendering, and no `String` allocated to reach it.
                serializer.collect_str(self)
            } else {
                self.0.serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for StoreId {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            if !deserializer.is_human_readable() {
                return <[u8; 16]>::deserialize(deserializer).map(Self);
            }

            let text = String::deserialize(deserializer)?;
            // The length check is also what refuses a UUID rendering: separators
            // make it thirty-six characters, and a `StoreId` has no version
            // nibble or variant bits to justify them.
            if text.len() != HEX_LEN {
                return Err(D::Error::invalid_length(text.len(), &"32 hex digits"));
            }

            let mut bytes = [0u8; 16];
            let mut digits = text.bytes();
            for slot in &mut bytes {
                // Both `next()` calls are `Some` because the length is checked
                // above; `nibble` is what rejects a separator or an uppercase
                // digit that slipped in at the right length.
                let high = digits.next().and_then(nibble);
                let low = digits.next().and_then(nibble);
                match (high, low) {
                    (Some(high), Some(low)) => *slot = (high << 4) | low,
                    _ => {
                        return Err(D::Error::invalid_value(
                            Unexpected::Str(&text),
                            &"32 lowercase hex digits, with no separators",
                        ));
                    }
                }
            }
            Ok(Self(bytes))
        }
    }

    /// The field is `store`, not WF-6's `origin`.
    ///
    /// [`EventId`]'s own field and its [`store`](EventId::store) accessor already
    /// agree on the word, and ADR-0014 froze that type; a third name for one
    /// value, on the wire, would be the only place in the crate using it. The
    /// frozen clause is amended to match rather than the type renamed
    /// (ADR-0016 §9).
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
