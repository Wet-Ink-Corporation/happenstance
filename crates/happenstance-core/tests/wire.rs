//! Phase 5's proof artefact: what this crate puts on the wire survives a round
//! trip in a **self-describing** format and in a **non-self-describing** one,
//! and neither format alone would have said so.
//!
//! `serde_json` is the first half of WF-7's matrix and `postcard` is the second,
//! and the reason there are two is a measurement rather than a preference. D1
//! was five `skip_serializing_if` attributes. Measured, all four `Event` shapes
//! round-trip **cleanly** through `serde_json` with those attributes in place —
//! a field name is on the wire, so an absent field is simply an absent field.
//! `postcard` writes no names, so a skipped field makes the encoding
//! *positional*: the decoder resynchronises against the wrong field and reads
//! its neighbour's bytes as its own. A matrix with one format certifies a format
//! that cannot be decoded, which is exactly what shipped.
//!
//! So this file proves five things, in this order:
//!
//! 1. Every envelope shape round-trips in `serde_json` — including at
//!    [`MIN_SUPPORTED_EVENT_DATA_LEN`], which is the only size at which the JSON
//!    rule rejects anything the postcard rule does not already reject.
//! 2. Every envelope shape round-trips in `postcard`, **framed** — the value is
//!    encoded with bytes after it, so a field-count desynchronisation surfaces
//!    as a wrong value rather than as a convenient parse error. See
//!    [`frame_and_take`] for why that distinction is the whole test.
//! 3. A decoder is the other door into a private invariant, and it is locked —
//!    but the lock is not a capacity limit. WF-10's three pins hold the
//!    invariants a constructor enforces; `decode_accepts_an_over_capacity_value`
//!    holds the floor under them, because "re-run the constructor's invariants"
//!    and "enforce the store's limits" are one sentence apart and different
//!    instructions.
//! 4. `ReadOptions` is on neither side of the wire, asserted at **const
//!    evaluation** rather than by a `compile_fail` doctest — measured, three of
//!    four spellings of that doctest reported green while the type was fully
//!    serialisable. See [`read_options_is_not_serialisable`].
//! 5. The wrong implementation is still wrong. [`negative_controls`] holds a
//!    mirror carrying D1's attributes and asserts it still decodes a neighbour's
//!    bytes into its own `metadata` field, silently.
//!
//! [ADR-0016](../../../.kb/decisions/0016-the-wire-format.md) is the decision;
//! `SPECIFICATION.md` §7's WF-2, WF-5, WF-7, WF-9, WF-10 and WF-12 are the
//! clauses.

#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

// Every test lives inside `mod wire` so that `cargo test --list` prints
// `wire::round_trips_in_postcard` — the name the clauses cite and the name a
// reviewer pastes back into `cargo test`. At the file's top level the same test
// would list as `round_trips_in_postcard`, and every clause naming `wire::` it
// would resolve to nothing. `happenstance-testkit`'s `mutation_coverage.rs` uses
// the same trick for the same reason.
mod wire {
    use core::fmt::Debug;
    use std::sync::LazyLock;

    use happenstance_core::bytes::Bytes;
    use happenstance_core::{
        AppendCondition, Event, EventId, EventType, InvalidQuery, MIN_SUPPORTED_EVENT_DATA_LEN,
        Query, QueryItem, RecordedAt, SequencePosition, SequencedEvent, StoreId, Tag, Tags,
    };
    use proptest::prelude::*;
    use serde::Serialize;
    use serde::de::DeserializeOwned;

    // =====================================================================
    // The shapes, and the generators that reach the edges phase 3 excluded
    // =====================================================================

    /// One sample of every shape this crate puts on the wire.
    ///
    /// A struct rather than eight separate proptests because the obligation is
    /// per-*format*, not per-type: WF-7 asks for two round-trip rules covering
    /// every shape, and sixteen rules would make the clause cite eight names per
    /// format and check the same property in each.
    #[derive(Debug, Clone)]
    struct Shapes {
        /// The four `Event` shapes, one per case.
        event: Event,
        /// An `Event` wearing the three store-assigned facts (WF-5).
        sequenced: SequencedEvent,
        /// `All` and `Items` both, because they are different variants now.
        query: Query,
        /// Reached at the top level as well as through `query`: a types-only and
        /// a tags-only item are the two an adapter's `WHERE` clause mishandles.
        item: QueryItem,
        /// Including the empty set, which is a legal `Tags` and a legal
        /// serialisation.
        tags: Tags,
        /// One to three guards, with and without `after`.
        condition: AppendCondition,
        /// The type whose encoding differs *between* the two formats (WF-6).
        store: StoreId,
        /// The pair that must not be recomputed at the receiver.
        id: EventId,
    }

    impl Shapes {
        /// Round-trips every field through `serde_json`.
        fn round_trip_in_json(&self) {
            json_round_trip(&self.event);
            json_round_trip(&self.sequenced);
            json_round_trip(&self.query);
            json_round_trip(&self.item);
            json_round_trip(&self.tags);
            json_round_trip(&self.condition);
            json_round_trip(&self.store);
            json_round_trip(&self.id);
        }

        /// Round-trips every field through `postcard`, each one framed.
        fn round_trip_in_postcard(&self) {
            postcard_round_trip_framed(&self.event);
            postcard_round_trip_framed(&self.sequenced);
            postcard_round_trip_framed(&self.query);
            postcard_round_trip_framed(&self.item);
            postcard_round_trip_framed(&self.tags);
            postcard_round_trip_framed(&self.condition);
            postcard_round_trip_framed(&self.store);
            postcard_round_trip_framed(&self.id);
        }
    }

    mod strategies {
        //! Local generators, deliberately **not**
        //! `happenstance_testkit::fixtures::strategies`.
        //!
        //! Two reasons, and the second is the decisive one.
        //!
        //! First, the cheap one: `happenstance-core` is the crate everything else
        //! depends on, so a dev-dependency on the testkit — which depends on
        //! `happenstance-core` — is a cycle cargo will not resolve.
        //!
        //! Second, and this would matter even if the cycle did not exist: the
        //! testkit's `any_event` **deliberately excludes** the two value edges
        //! this phase exists to pin (`fixtures.rs:428-437`) — the empty payload,
        //! and `metadata: Some(<empty>)` as distinct from `None`. Phase 3
        //! excluded them because the clause that owns the question had not been
        //! written and a generator producing them would have been asserting an
        //! answer through the model's round-trip comparison. The clause is
        //! written now: ADR-0003 promises byte-for-byte forwarding of an opaque
        //! payload, so "the peer sent nothing" and "the peer sent none" are two
        //! facts and the wire must keep them apart. A generator that cannot
        //! produce both cannot check it.

        use super::{
            AppendCondition, Bytes, Event, EventId, EventType, Query, QueryItem, RecordedAt,
            SequencePosition, SequencedEvent, Shapes, StoreId, Tag, Tags,
        };
        use proptest::prelude::*;

        /// A tag from a five-symbol alphabet.
        ///
        /// Small on purpose: the property under test is the *encoding*, and a
        /// wide alphabet buys collisions in the sorted-and-deduplicated set
        /// rather than coverage of the wire. The non-ASCII entry is there
        /// because a length prefix is in bytes and a `char` count is not.
        pub(super) fn any_tag() -> impl Strategy<Value = Tag> {
            prop::sample::select(vec![
                "course:c1",
                "student:s1",
                "tenant:a",
                "archived",
                "τ:1",
            ])
            .prop_map(|value| Tag::new(value).expect("a valid tag"))
        }

        /// Zero to three tags. Zero is a shape, not an absence — it is what a
        /// bare `Event` carries and what the deleted `skip_serializing_if`
        /// removed from the wire.
        pub(super) fn any_tags() -> impl Strategy<Value = Tags> {
            prop::collection::vec(any_tag(), 0..4).prop_map(|tags| tags.into_iter().collect())
        }

        /// An event type from a four-symbol alphabet, one of which is multi-byte
        /// for the same reason [`any_tag`]'s is.
        pub(super) fn any_event_type() -> impl Strategy<Value = EventType> {
            prop::sample::select(vec!["A", "CourseDefined", "seat.map.published", "τύπος"])
                .prop_map(|value| EventType::new(value).expect("a valid event type"))
        }

        /// A payload, **including the empty one**.
        ///
        /// The empty payload is one of the two edges named in this module's doc.
        /// It is not the maximum-size one: that is a fixture rather than a draw,
        /// because a 65,536-byte edge reached one case in twenty is an edge that
        /// stops being reached the day someone reweights the arm.
        pub(super) fn any_payload() -> impl Strategy<Value = Bytes> {
            prop::sample::select(vec![
                Bytes::new(),
                Bytes::from_static(b"{}"),
                Bytes::from_static(b"[1,2]"),
                Bytes::from_static(b"\xde\xad\xbe\xef"),
            ])
        }

        /// Metadata: absent, present-and-empty, or present.
        ///
        /// The middle arm is the second edge. `None` and `Some(<empty>)` are
        /// `null` against `""` in JSON and `[00]` against `[01 00]` in postcard,
        /// and a generator producing only the outer two would pass against a
        /// wire format that collapsed them.
        pub(super) fn any_metadata() -> impl Strategy<Value = Option<Bytes>> {
            prop_oneof![
                Just(None),
                Just(Some(Bytes::new())),
                prop::sample::select(vec![
                    Bytes::from_static(b"trace-id"),
                    Bytes::from_static(b"causation"),
                ])
                .prop_map(Some),
            ]
        }

        /// An [`Event`] in one of its four shapes: bare, tags-only,
        /// metadata-only, both.
        ///
        /// The two axes are drawn independently: `tags` is empty about a quarter
        /// of the time and `metadata` is `None` about a third, so no corner is
        /// rarer than one case in six and all four are visited dozens of times in
        /// a default 256-case run. They are the corners that matter because they
        /// are exactly the ones D1's attributes erased from the encoding.
        pub(super) fn any_event() -> impl Strategy<Value = Event> {
            (any_event_type(), any_tags(), any_payload(), any_metadata()).prop_map(
                |(event_type, tags, data, metadata)| {
                    // `.as_str().to_owned()` rather than the `EventType` already
                    // in hand: `Event::new` is bounded `impl TryInto<EventType,
                    // Error = InvalidEventType>`, and an existing `EventType`'s
                    // conversion is `Infallible` — a different associated type,
                    // so it does not satisfy the bound.
                    let event = Event::new(event_type.as_str().to_owned(), data)
                        .expect("an EventType is already valid")
                        .with_tags(tags);
                    match metadata {
                        Some(metadata) => event.with_metadata(metadata),
                        None => event,
                    }
                },
            )
        }

        /// A [`QueryItem`], keeping both legal empty halves.
        ///
        /// `prop_filter_map` rather than a generator steered away from the empty
        /// case: an item constraining neither types nor tags is rejected at
        /// construction, but *each* empty set on its own is legal and is the
        /// shape whose absent field the deleted attributes used to skip.
        pub(super) fn any_query_item() -> impl Strategy<Value = QueryItem> {
            (prop::collection::vec(any_event_type(), 0..3), any_tags())
                .prop_filter_map("an item must constrain types or tags", |(types, tags)| {
                    QueryItem::new(types, tags).ok()
                })
        }

        /// A [`Query`], including `All`.
        ///
        /// `All` gets a full third of the draws rather than the testkit's fifth.
        /// It is a distinct variant on the wire as of ADR-0016 §7 — it used to be
        /// `null` — so it is now a shape the encoding can get wrong, not merely a
        /// behaviour with one outcome.
        pub(super) fn any_query() -> impl Strategy<Value = Query> {
            prop_oneof![
                1 => Just(Query::all()),
                2 => prop::collection::vec(any_query_item(), 1..4)
                    .prop_map(|items| Query::from_items(items).expect("at least one item")),
            ]
        }

        /// A position anywhere in the key space, including the two ends.
        pub(super) fn any_sequence_position() -> impl Strategy<Value = SequencePosition> {
            (1u64..=u64::MAX).prop_map(|raw| SequencePosition::new(raw).expect("non-zero"))
        }

        /// Sixteen arbitrary bytes. Not sampled from a list: a `StoreId`'s
        /// encoding is per-nibble in JSON, so every value of every nibble is a
        /// case.
        pub(super) fn any_store_id() -> impl Strategy<Value = StoreId> {
            any::<[u8; 16]>().prop_map(StoreId::from_bytes)
        }

        /// A store and the position it assigned.
        pub(super) fn any_event_id() -> impl Strategy<Value = EventId> {
            (any_store_id(), any_sequence_position())
                .prop_map(|(store, position)| EventId::new(store, position))
        }

        /// A [`SequencedEvent`] whose `id` is drawn independently of its
        /// `position`.
        ///
        /// That independence is the point rather than laziness: an event
        /// replicated A→B→C keeps A's identity while B and C assign their own
        /// positions, so a generator tying the two together would agree with the
        /// wrong implementation `sequenced_event_round_trips` exists to reject.
        pub(super) fn any_sequenced_event() -> impl Strategy<Value = SequencedEvent> {
            (
                any_sequence_position(),
                any_event_id(),
                any::<i64>(),
                any_event(),
            )
                .prop_map(|(position, id, millis, event)| {
                    SequencedEvent::new(position, id, RecordedAt::from_millis(millis), event)
                })
        }

        /// One to three guards, each with its own optional boundary.
        ///
        /// `after_opt` is applied to the first guard before the others are added
        /// because it rewrites *every* guard it can see; adding the rest
        /// afterwards is what keeps their boundaries independent.
        pub(super) fn any_append_condition() -> impl Strategy<Value = AppendCondition> {
            prop::collection::vec(
                (any_query(), prop::option::of(any_sequence_position())),
                1..4,
            )
            .prop_map(|guards| {
                let mut guards = guards.into_iter();
                let (query, after) = guards.next().expect("at least one guard");
                let mut condition = AppendCondition::new(query).after_opt(after);
                for (query, after) in guards {
                    condition = condition.and_guard(query, after);
                }
                condition
            })
        }

        /// One sample of every shape at once.
        pub(super) fn any_shapes() -> impl Strategy<Value = Shapes> {
            (
                any_event(),
                any_sequenced_event(),
                any_query(),
                any_query_item(),
                any_tags(),
                any_append_condition(),
                any_store_id(),
                any_event_id(),
            )
                .prop_map(
                    |(event, sequenced, query, item, tags, condition, store, id)| Shapes {
                        event,
                        sequenced,
                        query,
                        item,
                        tags,
                        condition,
                        store,
                        id,
                    },
                )
        }
    }

    // =====================================================================
    // The harness: why every postcard round trip carries a trailer
    // =====================================================================

    /// The bytes every framed round trip writes immediately after the value.
    ///
    /// Not arbitrary, and not a marker chosen to be recognisable. These five
    /// bytes are W1's eighth neighbour
    /// (`experiments/wire-format/tests/w1_postcard_desynchronisation.rs`),
    /// which is one of the three of nine measured to make a skipped field decode
    /// to a **wrong value with no error**: `01` is read as `Some`, `02` as a
    /// two-byte length, and `AA BB` as the payload, leaving `05` behind.
    ///
    /// Choosing a trailer that merely *errors* would have been the weaker
    /// instrument — see [`frame_and_take`].
    const TRAILER: [u8; 5] = [0x01, 0x02, 0xAA, 0xBB, 0x05];

    /// Encodes `value` with [`TRAILER`] behind it, then takes the value back off
    /// the front and returns whatever bytes it left.
    ///
    /// **This is the load-bearing design decision in the file.** A *standalone*
    /// postcard round trip reports a field-count desynchronisation as
    /// `DeserializeUnexpectedEnd` — the decoder runs out of buffer looking for
    /// the field the encoder skipped. That is a real failure and it is the wrong
    /// one to test for: it says the message was truncated, and a test asserting
    /// "this errors" passes for a dozen reasons that are not the defect.
    ///
    /// D1 in a real message was not a truncated message. It was an event
    /// followed by *something* — the next event in a batch, the next field of an
    /// enclosing struct — and the decoder read that something as the skipped
    /// field's value and returned `Ok`. Measured over nine neighbours: three
    /// wrong values, two silently over-consumed, four errors. Framing the value
    /// is what makes this file reproduce the first two classes instead of only
    /// the third, and `RUNBOOK.md:3400-3401` asks for exactly that — *"a decode
    /// that returns the wrong value is a better regression test than a decode
    /// that errors."*
    ///
    /// The remainder is returned rather than asserted here so that
    /// [`negative_controls`] can assert the *opposite* of what
    /// [`postcard_round_trip_framed`] asserts, over the same framing.
    fn frame_and_take<T>(value: &T) -> Result<(T, Vec<u8>), postcard::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let buffer = postcard::to_stdvec(&(value, TRAILER))?;
        let (decoded, rest) = postcard::take_from_bytes::<T>(&buffer)?;
        Ok((decoded, rest.to_vec()))
    }

    /// The default postcard round trip: the value comes back, and so does every
    /// byte behind it.
    fn postcard_round_trip_framed<T>(value: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let (decoded, rest) = frame_and_take(value).unwrap_or_else(|error| {
            panic!("{value:?} did not survive a framed postcard round trip: {error}")
        });

        assert_eq!(
            &decoded, value,
            "a postcard decode returned a different value than was encoded"
        );
        assert_eq!(
            rest, TRAILER,
            "the decoder consumed bytes that did not belong to the value: the \
             encoding is positional, so some field is being skipped"
        );
    }

    /// The unframed round trip, for the one test that is *about* a solo encoding.
    ///
    /// Kept separate rather than made the default so that reaching for it is a
    /// visible choice. Everything else uses [`postcard_round_trip_framed`].
    fn postcard_round_trip_solo<T>(value: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let buffer = postcard::to_stdvec(value).expect("a value this crate defines must encode");
        let decoded = postcard::from_bytes::<T>(&buffer)
            .unwrap_or_else(|error| panic!("{value:?} did not decode from its own bytes: {error}"));
        assert_eq!(&decoded, value, "a solo postcard round trip lost something");
    }

    /// The `serde_json` half of the matrix.
    fn json_round_trip<T>(value: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let text = serde_json::to_string(value).expect("a value this crate defines must encode");
        let decoded = serde_json::from_str::<T>(&text)
            .unwrap_or_else(|error| panic!("{text} did not decode: {error}"));
        assert_eq!(&decoded, value, "a JSON round trip lost something");
    }

    /// An `Event` carrying exactly [`MIN_SUPPORTED_EVENT_DATA_LEN`] bytes.
    ///
    /// Built once, because 64 KiB rebuilt per proptest case is 64 KiB of
    /// generator time buying nothing — the value is a constant.
    ///
    /// The payload is a 251-byte cycle rather than a constant fill. 251 is prime,
    /// so the pattern aligns with neither base64's three-byte group nor any
    /// power-of-two chunk an encoder might buffer in: a truncation, a duplicated
    /// chunk or a dropped one moves the bytes and equality notices, where a
    /// uniform fill would have compared equal to most of its own mutations.
    static MAXIMUM_SIZE_EVENT: LazyLock<Event> = LazyLock::new(|| {
        let data: Vec<u8> = (0..MIN_SUPPORTED_EVENT_DATA_LEN)
            .map(|index| u8::try_from(index % 251).expect("251 fits in a u8"))
            .collect();
        Event::new("SeatMapPublished", Bytes::from(data)).expect("a valid event type")
    });

    // =====================================================================
    // WF-7 — the two-format matrix
    // =====================================================================

    proptest! {
        /// Every envelope shape survives `serde_json`, at every size a store
        /// must accept.
        ///
        /// # This rule takes no negative control, and must not be given one
        ///
        /// The wrong implementation WF-2 and WF-7 were written against is a
        /// `skip_serializing_if` attribute, and measured, a mirror carrying one
        /// round-trips **cleanly** in `serde_json` — all four `Event` shapes.
        /// That is not a gap in this rule; it is the fact WF-7 exists to state.
        /// A self-describing format puts the field names on the wire, so an
        /// absent field is an absent field and the decoder never loses its
        /// place. Obliging this rule to reject a skip would be writing an
        /// unsatisfiable obligation into a frozen clause. The control belongs to
        /// [`round_trips_in_postcard`] alone (ADR-0016 §6).
        ///
        /// # The wrong implementation it does reject
        ///
        /// A payload encoder that truncates, or a `Deserialize` that grew a
        /// length cap at [`MIN_SUPPORTED_EVENT_DATA_LEN`]. Both are
        /// JSON-specific — the base64 arm is where a size bound gets written —
        /// and both are invisible below 65,536 bytes, which is why the maximum
        /// -size event is asserted here unconditionally rather than drawn from
        /// the generator. Below that size this rule is a regression fixture, and
        /// says so rather than being dressed up.
        #[test]
        fn round_trips_in_json(shapes in strategies::any_shapes()) {
            shapes.round_trip_in_json();

            // WF-7's "maximum-size values" half. Equality already covers the
            // length, but a reader should not have to derive that from it.
            json_round_trip(&*MAXIMUM_SIZE_EVENT);
            prop_assert_eq!(
                MAXIMUM_SIZE_EVENT.data().len(),
                MIN_SUPPORTED_EVENT_DATA_LEN,
                "the fixture stopped being the size the rule is about"
            );
        }
    }

    proptest! {
        /// Every envelope shape survives `postcard`, framed.
        ///
        /// # The wrong implementation it rejects
        ///
        /// Any `skip_serializing_if` on any wire mirror — D1 itself, and this
        /// ADR's starting point. `postcard` writes no field names, so an absent
        /// field makes the encoding positional and the decoder reads the next
        /// value along as though it were the missing one. The control lives at
        /// [`negative_controls::skipped_event_wire_fails_the_postcard_round_trip`]
        /// and is asserted to still fail.
        ///
        /// `#[serde(default)]` is *not* what this rejects, and the distinction
        /// is worth keeping straight: measured, it is byte-identical on the
        /// write side and only widens what the decoder accepts. It is gone from
        /// the five sites for a different reason (ADR-0016 §4) and nothing here
        /// would notice its return.
        #[test]
        fn round_trips_in_postcard(shapes in strategies::any_shapes()) {
            shapes.round_trip_in_postcard();
        }
    }

    // =====================================================================
    // WF-5 — the three store-assigned facts stay on the wire
    // =====================================================================

    /// A `SequencedEvent` keeps all four of its fields through both formats.
    ///
    /// # This is a regression pin, and is stated as one
    ///
    /// `SequencedEventWire` already writes all four fields unconditionally —
    /// ADR-0014 landed that shape and ADR-0016 §8 only documented it. So no
    /// implementation in the tree fails this today, and a rule no adapter can
    /// fail is decorative unless it names the one it *could*.
    ///
    /// # The wrong implementation it pins against
    ///
    /// A wire form that drops `id` as "recomputable at the receiver": the
    /// argument is that `EventId` is a store plus a position and the receiver
    /// knows both, so why send it. It is not recomputable. An event replicated
    /// A→B→C keeps the identity **A** minted, while B and C assign it their own
    /// positions — so a receiver reconstructing `id` from its own store and its
    /// own position forges a fresh identity at every hop, and every peer's
    /// deduplication silently stops working. The saving is sixteen bytes plus a
    /// varint.
    ///
    /// The generator is what gives this teeth: `any_sequenced_event` draws `id`
    /// independently of `position`, so a receiver-side reconstruction disagrees
    /// with the encoded value in almost every case rather than in none.
    #[test]
    fn sequenced_event_round_trips() {
        let store = StoreId::from_bytes([
            0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
            0xe1, 0xf0,
        ]);
        let origin = SequencePosition::new(7).unwrap();
        let here = SequencePosition::new(4_001).unwrap();
        assert_ne!(
            origin, here,
            "the fixture is only a fixture if the two positions differ"
        );

        let sequenced = SequencedEvent::new(
            here,
            EventId::new(store, origin),
            RecordedAt::from_millis(1_700_000_000_000),
            Event::new("SeatMapPublished", &b"\xde\xad\xbe\xef"[..])
                .unwrap()
                .with_tags(Tags::from_pairs([("course", "c1")]).unwrap()),
        );

        json_round_trip(&sequenced);
        postcard_round_trip_framed(&sequenced);

        // Named individually so a failure says which fact was lost rather than
        // printing two four-field structs and leaving the reader to diff them.
        let json = serde_json::to_string(&sequenced).unwrap();
        let decoded = serde_json::from_str::<SequencedEvent>(&json).unwrap();
        assert_eq!(
            decoded.id.store(),
            store,
            "the minting store did not survive the wire"
        );
        assert_eq!(
            decoded.id.position(),
            origin,
            "the origin position was replaced by the local one — the receiver \
             recomputed an identity it was supposed to carry"
        );
        assert_eq!(decoded.position, here, "the local position did not survive");
        assert_eq!(
            decoded.recorded_at, sequenced.recorded_at,
            "the recorded time did not survive"
        );
    }

    // =====================================================================
    // The D1 golden vector
    // =====================================================================

    /// The exact bytes a bare `Event` encodes to in `postcard` today.
    ///
    /// Not a round trip — a round trip is symmetric and cannot see a change that
    /// alters both halves. This is the file's only assertion about the bytes
    /// themselves, so that a future edit to the wire format arrives as a diff
    /// somebody has to look at rather than as a silently different encoding that
    /// still round-trips with itself and no longer decodes anything a deployed
    /// peer wrote.
    ///
    /// The value is W1's, byte for byte, so the ADR's measurement and this pin
    /// describe one thing: the pre-fix encoding of this event was **7** bytes
    /// with `tags` and `metadata` skipped, and it is **9** now. Two bytes bought
    /// a format that can be decoded.
    #[test]
    fn bare_event_postcard_encoding_is_pinned() {
        let bare = Event::new("A", &b"\x11\x22\x33\x44"[..]).unwrap();
        assert!(bare.tags().is_empty());
        assert!(bare.metadata().is_none());

        let encoded = postcard::to_stdvec(&bare).unwrap();

        assert_eq!(
            encoded,
            [
                0x01, 0x41, // event_type: one byte long, "A"
                0x04, 0x11, 0x22, 0x33, 0x44, // data: four bytes, raw
                0x00, // tags: a sequence of length zero, written not skipped
                0x00, // metadata: None, written not skipped
            ],
            "the postcard encoding of a bare Event changed. If that was \
             deliberate, this vector is the diff to update — and the two \
             trailing zeroes are the whole of D1's fix, so check they are still \
             both there before assuming they are noise."
        );

        // The pinned bytes are a *solo* encoding, which is the one place in this
        // file where the unframed round trip is the thing being described.
        postcard_round_trip_solo(&bare);
    }

    // =====================================================================
    // WF-1, WF-3, WF-4 — the two values a wrong encoding cannot tell apart
    // =====================================================================

    /// `Query::All` has a form of its own in both formats, and it is not the
    /// format's null.
    ///
    /// # The wrong implementation it rejects
    ///
    /// HEAD's encoding: `All` written with `serialize_none` and `Items` with
    /// `serialize_some`. Under it `Some(Query::All)` and `None::<Query>` are
    /// **both** `null` — measured, `w2_option_query`. The control is
    /// [`negative_controls::option_shaped_query_is_indistinguishable_from_none`],
    /// which reproduces that encoding and asserts it is still ambiguous.
    ///
    /// Why that is worse than an ordinary lost distinction: in serde's data
    /// model `Option` is not a wrapper the format renders. `serialize_some(v)`
    /// is *transparent* and emits exactly the bytes `v` emits, so `Some(T)` and
    /// `T` are one encoding — unlike C#'s `Nullable<T>`, which is a distinct
    /// runtime type carrying its own `bool`. So an `Option`-shaped `Query` does
    /// not merely blur two values: it parks `All` on the format's null, and
    /// `All` with no boundary is the most destructive value in the protocol —
    /// the append condition that checks the whole log and can never be
    /// satisfied by a narrower one. A peer that emits `null` by accident emits
    /// it.
    ///
    /// The assertions are on the encoded form rather than on a round trip
    /// because a round trip is symmetric: `All ↔ null` round-trips perfectly
    /// well with itself. Only the bytes say which value the null is.
    #[test]
    fn query_all_is_unambiguous() {
        // JSON: an externally tagged unit variant is a string, and a string is
        // not null. This is the comparison the clause is about.
        assert_eq!(
            serde_json::to_string(&Query::all()).unwrap(),
            "\"All\"",
            "`Query::All` stopped having a name of its own on the JSON wire"
        );
        assert_eq!(
            serde_json::to_string(&None::<Query>).unwrap(),
            "null",
            "the format's null moved, which is the other half of the comparison"
        );
        assert_ne!(
            serde_json::to_string(&Query::all()).unwrap(),
            serde_json::to_string(&None::<Query>).unwrap(),
            "match-all and absent-query are the same JSON text again: a peer \
             that omits a query now asks for the whole log"
        );

        // The tag is on the other variant too, or `Items` would be what a bare
        // sequence decodes to and `All` would be the only tagged form.
        let item = QueryItem::new(
            [EventType::new("Enrolled").unwrap()],
            Tags::from_pairs([("course", "c1")]).unwrap(),
        )
        .unwrap();
        assert_eq!(
            serde_json::to_string(&Query::from_items([item]).unwrap()).unwrap(),
            r#"{"Items":[{"types":["Enrolled"],"tags":["course:c1"]}]}"#,
            "the `Items` variant lost its tag"
        );

        // postcard: `All` is the zero discriminant, which is also `None`'s
        // encoding — the two are told apart one level up, where the message
        // actually carries an `Option`. `[00]` against `[01 00]` is the
        // distinction that survives a format with no field names.
        assert_eq!(
            postcard::to_stdvec(&Query::all()).unwrap(),
            [0x00],
            "`Query::All` is the first variant of a two-variant enum"
        );
        assert_eq!(
            postcard::to_stdvec(&Some(Query::all())).unwrap(),
            [0x01, 0x00],
            "postcard's `Some` tag plus the `All` discriminant"
        );
        assert_ne!(
            postcard::to_stdvec(&Some(Query::all())).unwrap(),
            postcard::to_stdvec(&None::<Query>).unwrap(),
            "a present match-all query and an absent query encode alike in \
             postcard, so the `Option` layer has stopped carrying anything"
        );
    }

    /// `Some(Query::All)` comes back as `Some(Query::All)`.
    ///
    /// # The wrong implementation it rejects
    ///
    /// The same defect as [`query_all_is_unambiguous`], approached from the
    /// `Option<Query>` side, and it fails for a *different* reason — which is
    /// why WF-3 names both rules rather than picking one. Under HEAD's encoding
    /// the write side is ambiguous (both values are `null`) and the read side
    /// resolves that ambiguity the other way: `from_str::<Option<Query>>("null")`
    /// is `Ok(None)` while `from_str::<Query>("null")` is `Ok(All)`. So a peer
    /// sending a match-all query received an absent one, silently and with no
    /// byte in the message wrong.
    #[test]
    fn option_query_round_trips() {
        let some_all = Some(Query::all());

        let text = serde_json::to_string(&some_all).unwrap();
        let decoded = serde_json::from_str::<Option<Query>>(&text).unwrap();
        assert_eq!(
            decoded, some_all,
            "`Some(Query::All)` did not survive a JSON round trip"
        );
        assert!(
            decoded.is_some(),
            "`Some(Query::All)` returned as `None`: the receiver was told to \
             check the whole log and heard no condition at all"
        );

        // The neighbours, so that "it round-trips" is not satisfied by an
        // encoding that maps everything to one value.
        let some_items = Some(
            Query::from_items([QueryItem::new(
                [EventType::new("Enrolled").unwrap()],
                Tags::from_pairs([("course", "c1")]).unwrap(),
            )
            .unwrap()])
            .unwrap(),
        );
        assert_ne!(
            serde_json::to_string(&some_all).unwrap(),
            serde_json::to_string(&None::<Query>).unwrap(),
            "the two values this rule keeps apart share an encoding"
        );
        json_round_trip(&some_all);
        json_round_trip(&some_items);
        json_round_trip(&None::<Query>);

        // Framed, because the `Option` tag is exactly the kind of one-byte
        // field whose loss a solo postcard decode reports as a short buffer.
        postcard_round_trip_framed(&some_all);
        postcard_round_trip_framed(&some_items);
        postcard_round_trip_framed(&None::<Query>);
    }

    /// A guard with no `query` is not a guard, in either of the two spellings a
    /// peer can reach it by.
    ///
    /// # The wrong implementation it rejects
    ///
    /// A `Guard` whose `query` decodes an absent or `null` value to
    /// `Query::All` — which is what the tree did until this phase. Measured,
    /// **both** `{"guards":[{}]}` and `{"guards":[{"query":null}]}` returned
    /// `Ok(AppendCondition { guards: [Guard { query: All, after: None }] })`:
    /// an append condition matching every event since the beginning of the log
    /// and therefore the one condition that can never fail, conjured out of an
    /// empty object. That is a silent lost update arriving over the wire.
    ///
    /// Two mechanisms produce it and the rule needs both inputs, because a fix
    /// to either alone leaves the other live: `{}` is stopped by the absence of
    /// `#[serde(default)]` on `GuardWire::query`, and `null` is stopped by
    /// `Query` no longer being `Option`-shaped (WF-3). "Add a default so `{}`
    /// parses" restores the first, and it will look like a kindness.
    ///
    /// `"{}"`, `"[]"` and `{"guards":[]}` are asserted too. They already failed
    /// before this phase and the clause keeps them as a floor, not as the
    /// finding.
    #[test]
    fn empty_object_is_not_a_condition() {
        /// Returns the error text, and panics with what was decoded otherwise —
        /// a failure that prints the forged condition is worth more than one
        /// that prints `false`.
        fn rejected(input: &str) -> String {
            match serde_json::from_str::<AppendCondition>(input) {
                Ok(condition) => panic!(
                    "`{input}` decoded to {condition:?} instead of failing. A \
                     guard the sender never wrote is now checking the whole log."
                ),
                Err(error) => error.to_string(),
            }
        }

        // The two inputs the amended clause names.
        let missing = rejected(r#"{"guards":[{}]}"#);
        assert!(
            missing.contains("query"),
            "an empty guard object is rejected, but not for the absence of \
             `query` — the message was: {missing}"
        );
        let null_query = rejected(r#"{"guards":[{"query":null}]}"#);
        // "expected value", not a message naming `query`: the field is present,
        // so the missing-field path never runs and the refusal comes from
        // `Query` no longer having a null to be. That the two inputs fail with
        // two different messages is the evidence that two mechanisms are doing
        // the work, and that fixing either one alone leaves the other live.
        assert!(
            null_query.contains("expected value"),
            "a null query is rejected, but not by the decoder refusing to read \
             a tagged enum from a null — the message was: {null_query}"
        );
        assert_ne!(
            missing, null_query,
            "both inputs now fail through the same path, so one of the two \
             defences has stopped being load-bearing"
        );

        // The floor, retained rather than live.
        rejected("{}");
        rejected("[]");
        rejected(r#"{"guards":[]}"#);

        // The counterweight: a decoder that rejects everything would satisfy
        // every assertion above, so the well-formed spelling must still decode.
        let accepted =
            serde_json::from_str::<AppendCondition>(r#"{"guards":[{"query":"All","after":null}]}"#)
                .expect("a fully spelled guard is the form the encoder writes");
        assert_eq!(
            accepted.guards().len(),
            1,
            "the one guard that was written is the one that arrived"
        );
    }

    /// Every guard carries `after` on the JSON wire, present and readable, when
    /// it is `None` as much as when it is set.
    ///
    /// # The wrong implementation it rejects
    ///
    /// A `GuardWire` that skips `after` when it is `None` — one
    /// `skip_serializing_if` away, and the saving is sixteen characters. A
    /// replication ingest policy decides whether to re-check an append
    /// condition by looking at what boundary the sender declared, and "the
    /// sender declared no boundary" is a decision input, not a default to
    /// invent. A field that is absent leaves the policy inferring, and the
    /// inference fails open.
    ///
    /// The assertion parses the JSON back into a `serde_json::Value` and walks
    /// it, rather than asking a string whether it `contains("after")`. A
    /// substring match is satisfied by the field name appearing anywhere at all
    /// — inside a tag, in another guard — and this clause is about a policy
    /// being able to *look the field up*, which is what indexing demonstrates
    /// and a substring does not.
    ///
    /// JSON only, deliberately: postcard writes no field names, so "an ingest
    /// policy can see the field" is a property of the self-describing half of
    /// the matrix. The postcard half is [`round_trips_in_postcard`]'s framing,
    /// where a skipped field surfaces as a wrong value.
    #[test]
    fn condition_after_is_visible_to_an_ingest_policy() {
        let boundary = SequencePosition::new(4_001).unwrap();
        let condition = AppendCondition::new(Query::all())
            .after_opt(None)
            .and_guard(Query::all(), Some(boundary));

        let value: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&condition).unwrap()).unwrap();

        let guards = value["guards"]
            .as_array()
            .expect("`guards` is a sequence, and an ingest policy indexes it");
        assert_eq!(
            guards.len(),
            condition.guards().len(),
            "the wire lost a guard between the encoder and the reader"
        );

        // `.get` and not a second `[]`: `serde_json`'s `Index` yields `Null` for
        // a key that is not there, so `value["guards"][0]["after"] == Null` is
        // satisfied by exactly the wire this rule exists to reject. Presence and
        // null-ness are two questions and only `Option` distinguishes them.
        assert_eq!(
            value["guards"][0].get("after"),
            Some(&serde_json::Value::Null),
            "the guard with no boundary omitted `after` entirely, so a reader \
             cannot tell `no boundary declared` from `field not written`"
        );

        // Compared against the position the fixture set, not against a literal:
        // the number in the JSON is only correct relative to what was encoded.
        let raw = u64::from(boundary);
        assert_eq!(
            value["guards"][1].get("after"),
            Some(&serde_json::json!(raw)),
            "the guard with a boundary did not put that boundary on the wire"
        );

        // WF-4's MUST names both fields. `query` has no plausible skip — it has
        // no default to be at — but its presence is half the clause.
        assert!(
            value["guards"][0].get("query").is_some(),
            "a guard reached the wire without the query it guards"
        );
    }

    // =====================================================================
    // WF-6, WF-11 — the human-readable branch, checked by its bytes
    //
    // The wrong implementation all four rules in this section reject is one
    // shape: an *inverted* `is_human_readable` branch — hex/base64 and raw
    // bytes swapped between the two formats. Measured
    // (`decorative_inverted_human_readable_branch`,
    // experiments/wire-format/tests/decorative_inverted_branch.rs): a
    // deliberately inverted newtype over `[de ad be ef]` still round-trips
    // `Ok(true)` in **both** `serde_json` and `postcard`, because hex and
    // base64 are symmetric — an encoder that writes the wrong arm and a
    // decoder that reads it back with the same wrong arm agree with each
    // other and disagree with every other peer in the network. No round-trip
    // rule anywhere in this file, however many shapes or cases it draws, can
    // see that. Only a look at the actual bytes on one named side can, which
    // is why every test below asserts a literal encoded form instead of
    // encoding-then-decoding.
    // =====================================================================

    /// `StoreId` is thirty-two lowercase hex characters in JSON, and the
    /// decoder accepts only that spelling.
    ///
    /// # The wrong implementation it rejects
    ///
    /// See the section banner: an inverted `is_human_readable` branch that
    /// would, for example, render the byte array as JSON instead of hex. The
    /// rejections of uppercase hex and of a hyphenated, UUID-shaped rendering
    /// are ADR-0016 §9's own decisions rather than incidental behaviour of
    /// whatever crate happens to parse hex — a round trip cannot see either
    /// one, because neither uppercase hex nor a hyphenated rendering is ever
    /// produced by the encoder for a round trip to feed back in.
    #[test]
    fn store_id_encodes_as_hex_in_json() {
        let store = StoreId::from_bytes([
            0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
            0xe1, 0xf0,
        ]);

        let text = serde_json::to_string(&store).unwrap();
        assert_eq!(
            text, "\"0f1e2d3c4b5a69788796a5b4c3d2e1f0\"",
            "a StoreId's JSON encoding is not the 32-character lowercase hex \
             string ADR-0016 §9 specifies — an inverted is_human_readable \
             branch would put the byte array here instead"
        );
        assert_eq!(
            text.trim_matches('"').len(),
            32,
            "sixteen bytes render as thirty-two hex characters, not some other \
             count"
        );
        assert!(
            text.trim_matches('"')
                .chars()
                .all(|c| c.is_ascii_hexdigit()),
            "every character of a StoreId's JSON form must be a hex digit"
        );

        assert!(
            serde_json::from_str::<StoreId>("\"0F1E2D3C4B5A69788796A5B4C3D2E1F0\"").is_err(),
            "uppercase hex decoded, but ADR-0016 §9 promises the lowercase \
             Display rendering as the only accepted spelling — a round trip \
             never produces uppercase hex to notice this was ever accepted"
        );
        assert!(
            serde_json::from_str::<StoreId>("\"0f1e2d3c-4b5a-6978-8796-a5b4c3d2e1f0\"").is_err(),
            "a hyphenated, UUID-shaped rendering decoded, but a StoreId has no \
             version nibble and no variant bits and is deliberately not \
             UUID-formatted — again invisible to a round trip, which never \
             encodes with hyphens to feed back in"
        );
    }

    /// `StoreId` is sixteen raw bytes in postcard, with no length prefix and
    /// no ASCII hex anywhere in the output.
    ///
    /// # The wrong implementation it rejects
    ///
    /// The same inverted branch, other arm: a `StoreId` that mistakenly wrote
    /// hex-as-a-string in postcard would carry a length-prefixed 32-byte
    /// ASCII payload instead of the identity's own 16 bytes, and — because
    /// hex round-trips through itself — a round-trip rule would not notice.
    #[test]
    fn store_id_encodes_as_bytes_in_postcard() {
        let raw = [
            0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
            0xe1, 0xf0,
        ];
        let store = StoreId::from_bytes(raw);

        let encoded = postcard::to_stdvec(&store).unwrap();
        assert_eq!(
            encoded, raw,
            "postcard must write the StoreId's own sixteen bytes, raw and \
             unprefixed — a length byte, a different byte count, or a \
             transcoded value here is the inverted branch"
        );

        let hex_ascii = b"0f1e2d3c4b5a69788796a5b4c3d2e1f0";
        assert!(
            !encoded
                .windows(hex_ascii.len())
                .any(|window| window == hex_ascii),
            "the postcard bytes contain the ASCII of this StoreId's hex \
             rendering, which means the human-readable branch fired on a \
             non-human-readable format"
        );
    }

    /// `Event::data` is a base64 string in JSON — the ADR's own vector.
    ///
    /// # The wrong implementation it rejects
    ///
    /// The section banner's inversion, on the payload rather than the
    /// identity: a JSON encoding that wrote the raw byte array (or one that
    /// wrote base64 in postcard) round-trips through itself with no error, so
    /// only the literal string form catches it. `Some(Bytes::new())`
    /// rendering as `""` and `None` metadata rendering as `null` are asserted
    /// alongside it because ADR-0003 promises byte-for-byte forwarding of an
    /// opaque payload: "the peer sent nothing" and "the peer sent none" must
    /// stay two distinguishable facts on the wire, not collapse to one.
    #[test]
    fn payload_is_base64_in_json() {
        let event = Event::new("A", &b"\xde\xad\xbe\xef"[..]).unwrap();
        let text = serde_json::to_string(&event).unwrap();
        assert!(
            text.contains("\"3q2+7w==\""),
            "[de ad be ef] must render as the base64 string \"3q2+7w==\" \
             (ADR-0016 §10's own vector) — the encoded event was: {text}"
        );

        let empty_payload = Event::new("A", Bytes::new()).unwrap();
        let empty_text = serde_json::to_string(&empty_payload).unwrap();
        assert!(
            empty_text.contains("\"data\":\"\""),
            "an empty (but present) payload must render as the empty base64 \
             string, not be dropped or confused with an absent value — \
             encoded event was: {empty_text}"
        );

        let with_empty_metadata = Event::new("A", &b"\x01"[..])
            .unwrap()
            .with_metadata(Bytes::new());
        let with_none_metadata = Event::new("A", &b"\x01"[..]).unwrap();
        let empty_metadata_text = serde_json::to_string(&with_empty_metadata).unwrap();
        let none_metadata_text = serde_json::to_string(&with_none_metadata).unwrap();
        assert!(
            empty_metadata_text.contains("\"metadata\":\"\""),
            "Some(Bytes::new()) metadata must render as an empty base64 \
             string — encoded event was: {empty_metadata_text}"
        );
        assert!(
            none_metadata_text.contains("\"metadata\":null"),
            "None metadata must render as JSON null — encoded event was: \
             {none_metadata_text}"
        );
        assert_ne!(
            empty_metadata_text, none_metadata_text,
            "\"the peer sent nothing\" and \"the peer sent none\" collapsed \
             to the same JSON text, which is exactly the distinction \
             ADR-0003's byte-for-byte forwarding promise depends on"
        );
    }

    /// `Event::data` is raw bytes in postcard, and the ASCII of its base64
    /// rendering never appears in the output.
    ///
    /// # The wrong implementation it rejects
    ///
    /// The same inversion, other arm: a postcard encoder that (wrongly) took
    /// the human-readable branch would write the ASCII text `"3q2+7w=="`
    /// instead of the four raw bytes it stands for. Both encodings decode
    /// back into the same value under their own paired (also-wrong) decoder,
    /// so a round trip cannot tell them apart; only checking which bytes are
    /// actually on the wire can.
    #[test]
    fn payload_is_raw_in_postcard() {
        let event = Event::new("A", &b"\xde\xad\xbe\xef"[..]).unwrap();
        let encoded = postcard::to_stdvec(&event).unwrap();

        assert!(
            encoded
                .windows(4)
                .any(|window| window == [0xde, 0xad, 0xbe, 0xef]),
            "the raw payload bytes [de ad be ef] do not appear in the \
             postcard encoding of the event"
        );

        let base64_ascii = b"3q2+7w==";
        assert!(
            !encoded
                .windows(base64_ascii.len())
                .any(|window| window == base64_ascii),
            "the postcard bytes contain the ASCII of this payload's base64 \
             rendering, which means the human-readable branch fired on a \
             non-human-readable format"
        );
    }

    // =====================================================================
    // WF-10 — a decoder is the other door into a private invariant
    //
    // All three rules in this section are **already satisfied**, and all three
    // pin against one wrong implementation rather than three: a `Deserialize`
    // **derived straight onto the private fields**. That is the obvious
    // implementation — it is one line, it round-trips perfectly, and every
    // round-trip rule in this file passes under it — and it is a hole in the
    // type's invariant, because a constructor that is the only enforcement of
    // an invariant stops being the only door the moment a decoder can reach the
    // fields behind it. The three types below each hold an invariant a peer
    // would otherwise be free to violate in the receiver's own memory, and the
    // consequence differs per type; each rule names its own.
    // =====================================================================

    /// A non-canonical tag list arrives canonical, in both formats.
    ///
    /// # This rule's name says "rejects" and the type does not reject
    ///
    /// Stated rather than papered over: `Tags` **re-canonicalises** rather than
    /// erroring (`tag.rs:497-504` collects through `FromIterator`, which sorts
    /// and deduplicates), so the assertion is that a non-canonical wire value
    /// cannot survive the decode as a non-canonical `Tags` — not that the decode
    /// fails. Sorting is the stronger of the two outcomes for a wire format: an
    /// error would make a peer that merely orders its tags differently
    /// unreachable, and nothing about tag order is a peer's promise to keep.
    ///
    /// # This is a regression pin, and is stated as one
    ///
    /// Nothing in the tree fails this today. What it pins against is a
    /// `#[derive(Deserialize)]` on `Tags`' private `Box<[Tag]>`, which is what
    /// the next person to tidy `serde_impls` will reach for — it is shorter and
    /// it round-trips, because the *encoder* only ever writes canonical order
    /// and a round trip only ever feeds the encoder's own output back in.
    ///
    /// What it costs is not cosmetic. `Tags::contains` is a `binary_search` and
    /// `contains_all` is a single merge-scan over two sorted slices
    /// (`tag.rs:284-320`), so an unsorted `Tags` does not look wrong — it
    /// answers *"is this event tagged `course:c1`"* with `false` for a tag that
    /// is present, and a DCB append condition built on that reads a decision
    /// model that is missing events it was supposed to see.
    #[test]
    fn decode_rejects_a_non_canonical_tag_set() {
        // Unsorted *and* duplicated: the two ways a peer's tag list can be
        // non-canonical, and a fix that only sorts passes half of this.
        const AS_SENT: [&str; 4] = ["student:s1", "course:c1", "course:c1", "archived"];

        // The comparison value is what this crate's own constructor produces
        // from the same tags, not a hand-written ordering: the rule is that the
        // decoder agrees with the constructor, and a literal would be asserting
        // a second opinion about what canonical means.
        let expected: Tags = AS_SENT
            .into_iter()
            .map(|value| Tag::new(value).unwrap())
            .collect();

        let decoded =
            serde_json::from_str::<Tags>(r#"["student:s1","course:c1","course:c1","archived"]"#)
                .expect("a peer's tag order is not a protocol error");
        assert_eq!(
            decoded, expected,
            "a tag set sent unsorted and duplicated did not arrive as the set \
             this crate's own constructor builds from the same tags"
        );
        assert_eq!(
            decoded.len(),
            expected.len(),
            "the duplicate survived the wire, so `Tags` is holding a multiset"
        );
        assert!(
            decoded
                .as_slice()
                .windows(2)
                .all(|pair| pair[0].as_str() < pair[1].as_str()),
            "the decoded tags are not strictly increasing, so `contains`' \
             binary search and `contains_all`' merge-scan are both now \
             answering `false` for tags that are present"
        );

        // The evidence that the decode did any work at all: had the bytes
        // passed straight through, the first tag would still be the first tag
        // the peer wrote.
        assert_ne!(
            decoded.as_slice().first().map(Tag::as_str),
            AS_SENT.first().copied(),
            "the decoded order is the order the peer sent, which is what a \
             derive onto the private field would produce"
        );

        // postcard reaches the same impl through a different door: a `Vec<Tag>`
        // is a sequence, and `Tags` decodes a sequence, so a peer built on the
        // plain vector encodes bytes this crate must read.
        let as_sent: Vec<Tag> = AS_SENT
            .into_iter()
            .map(|value| Tag::new(value).unwrap())
            .collect();
        let decoded = postcard::from_bytes::<Tags>(&postcard::to_stdvec(&as_sent).unwrap())
            .expect("the same tolerance in the non-self-describing format");
        assert_eq!(
            decoded, expected,
            "the postcard decoder skipped the canonicalisation the JSON one ran"
        );
    }

    /// A query item constraining neither types nor tags is refused on the way
    /// in, by the constructor that refuses it in memory.
    ///
    /// # This is a regression pin, and is stated as one
    ///
    /// `query.rs:368-373` already routes the decode through `QueryItem::new`.
    /// The pin is against the same derive: `QueryItemWire` is `#[derive(
    /// Serialize, Deserialize)]` two lines above, and deleting the hand-written
    /// impl in favour of putting the derive on `QueryItem` itself looks like
    /// removing duplication.
    ///
    /// # What it costs
    ///
    /// An item constraining nothing matches every event, so it is `Query::all`
    /// wearing the shape of a filter. Reaching it by decode rather than by
    /// construction is how a peer turns a narrow append condition into the
    /// widest one there is — the same failure `empty_object_is_not_a_condition`
    /// guards from the guard's side, one level down.
    #[test]
    fn decode_rejects_an_unconstrained_query_item() {
        let error = serde_json::from_str::<QueryItem>(r#"{"types":[],"tags":[]}"#)
            .expect_err("an item constraining nothing is `Query::all` in disguise");

        // Compared against the constructor's own `Display`, not a literal: that
        // is what says the refusal came from `QueryItem::new` re-running its
        // invariant, rather than from the decoder tripping over the shape.
        assert!(
            error
                .to_string()
                .contains(&InvalidQuery::UnconstrainedItem.to_string()),
            "the item was rejected, but not by `QueryItem::new` — the message \
             was: {error}"
        );

        // The same value in postcard. A postcard struct is positional, so a
        // two-tuple of empty sequences is byte-identical to what a peer's
        // `QueryItemWire` writes; the check has to run there too, and postcard
        // discards the custom message, so `is_err` is all this leg can say.
        let as_sent: (Vec<EventType>, Tags) = (Vec::new(), Tags::empty());
        assert!(
            postcard::from_bytes::<QueryItem>(&postcard::to_stdvec(&as_sent).unwrap()).is_err(),
            "the unconstrained item decoded in postcard, so the invariant is \
             being enforced by the JSON representation rather than by the \
             constructor"
        );

        // The counterweight: a decoder that refused everything would satisfy
        // both assertions above, so each legal half-empty shape must still
        // decode.
        serde_json::from_str::<QueryItem>(r#"{"types":["Enrolled"],"tags":[]}"#)
            .expect("a types-only item constrains something");
        serde_json::from_str::<QueryItem>(r#"{"types":[],"tags":["course:c1"]}"#)
            .expect("a tags-only item constrains something");
    }

    /// A zero-item `Items` query is refused, and the constructor is what
    /// refuses it.
    ///
    /// # This is a regression pin, and is stated as one
    ///
    /// `query.rs:385-392` routes `Items` through `Query::from_items`. The pin is
    /// against a derive onto `Query` itself — which is *more* tempting here than
    /// anywhere else in this section, because `QueryWire` is already a derived
    /// enum with the right variants and the hand-written impl looks like a
    /// wrapper that adds nothing.
    ///
    /// # Which mechanism catches it, since this phase changed the representation
    ///
    /// The **constructor**, not the representation, and the assertion below
    /// says so by comparing against `InvalidQuery::NoItems`' own text. The
    /// externally tagged encoding this phase landed (ADR-0016 §7) refuses a
    /// great deal that the `Option`-shaped one accepted, but not this: `[]` is a
    /// well-formed sequence and `{"Items":[]}` is a well-formed externally
    /// tagged variant, so the representation hands the decoder an empty `Vec`
    /// and it is `from_items` that declines it. A reader who assumed the tag
    /// closed this hole too would delete the only thing that does.
    ///
    /// # What it costs
    ///
    /// An empty item list is not "match nothing". It is a query no event can
    /// satisfy, arriving where a caller asked for a filter — so a decision model
    /// built from it is empty and every append condition over it succeeds.
    #[test]
    fn decode_rejects_a_zero_item_query() {
        let error = serde_json::from_str::<Query>(r#"{"Items":[]}"#)
            .expect_err("a query with no items is not a query");
        assert!(
            error
                .to_string()
                .contains(&InvalidQuery::NoItems.to_string()),
            "the empty `Items` was rejected, but not by `Query::from_items` \
             re-running its invariant — the message was: {error}"
        );

        // postcard: the `Items` discriminant followed by a zero-length
        // sequence. Written as bytes rather than encoded from a value because
        // no value of `Query` can produce them — which is the point.
        assert!(
            postcard::from_bytes::<Query>(&[0x01, 0x00]).is_err(),
            "`[01 00]` — the `Items` discriminant and an empty sequence — \
             decoded to a query, so the invariant is being enforced by the JSON \
             representation rather than by the constructor"
        );

        // The counterweight, both variants, so that "it rejects" is not
        // satisfied by a decoder that rejects `Items` altogether.
        let one_item = serde_json::from_str::<Query>(
            r#"{"Items":[{"types":["Enrolled"],"tags":["course:c1"]}]}"#,
        )
        .expect("one item is enough");
        assert_eq!(
            one_item,
            Query::from_items([QueryItem::new(
                [EventType::new("Enrolled").unwrap()],
                Tags::from_pairs([("course", "c1")]).unwrap(),
            )
            .unwrap()])
            .unwrap(),
            "the one-item query did not decode to the query it encodes from"
        );
        assert_eq!(
            serde_json::from_str::<Query>("\"All\"").unwrap(),
            Query::all(),
            "the match-all query stopped decoding, which no reading of WF-10 asks for"
        );
    }

    // =====================================================================
    // WF-9, VT-19 — the one rule in this file that runs the other way
    // =====================================================================

    /// An `Event` whose payload **exceeds** [`MIN_SUPPORTED_EVENT_DATA_LEN`]
    /// decodes to `Ok`.
    ///
    /// # Why a rule asserting `Ok` earns its place
    ///
    /// Every other rule here narrows what a decoder accepts. This one holds the
    /// floor under it, and it is needed precisely *because* of its neighbours:
    /// WF-10 asks these same `Deserialize` impls to re-run their constructor's
    /// invariants, and "re-run the invariants" reads to the next contributor as
    /// "enforce the limits". They are not the same instruction.
    /// `MIN_SUPPORTED_EVENT_DATA_LEN` is a **floor every store must clear**
    /// (`limits.rs:17-22`), not a ceiling the contract crate applies — a store
    /// that will not hold more says so with
    /// `AppendError::ExceedsStoreLimit`, at append, where the caller can tell a
    /// capacity refusal from a malformed message.
    ///
    /// # The wrong implementation it rejects
    ///
    /// A `Deserialize` that grew a length check at that constant. The control is
    /// [`negative_controls::length_checked_event_wire_rejects_a_65_kib_payload`],
    /// which is that implementation, asserted to refuse the payload the real
    /// `Event` accepts here.
    ///
    /// What such a check costs is worse than a rejected event: the receiver
    /// refuses the *message*, so a payload one byte over an arbitrary line stops
    /// replication between two stores that would both have held it happily.
    #[test]
    fn decode_accepts_an_over_capacity_value() {
        // Inclusive range: one byte past the floor is the tightest witness of
        // "exceeds", and the byte a `> MIN` check refuses first.
        let data: Vec<u8> = (0..=MIN_SUPPORTED_EVENT_DATA_LEN)
            .map(|index| u8::try_from(index % 251).expect("251 fits in a u8"))
            .collect();
        assert!(
            data.len() > MIN_SUPPORTED_EVENT_DATA_LEN,
            "the fixture stopped exceeding the floor the rule is about"
        );

        let event = Event::new("SeatMapPublished", Bytes::from(data))
            .expect("the contract crate applies no capacity limit at construction");

        // Both formats: the human-readable arm is where a size bound gets
        // written (base64 materialises the whole payload), and the positional
        // arm is where a length prefix invites one.
        json_round_trip(&event);
        postcard_round_trip_framed(&event);

        // Named separately from the round trips so a failure says the payload
        // was resized rather than printing two 64 KiB events to diff.
        let decoded =
            serde_json::from_str::<Event>(&serde_json::to_string(&event).unwrap()).unwrap();
        assert_eq!(
            decoded.data().len(),
            event.data().len(),
            "the over-capacity payload changed length crossing the wire"
        );
        assert!(
            decoded.data().len() > MIN_SUPPORTED_EVENT_DATA_LEN,
            "the decoder returned a payload at or under the floor, so something \
             clamped it rather than refusing it — which is worse than either"
        );
    }

    // =====================================================================
    // WF-12 — `ReadOptions` is not serialisable, asserted at const evaluation
    // =====================================================================

    mod read_options_is_not_serialisable {
        //! WF-12's rule. It is a **`const` assertion and not a `#[test]`**, so
        //! `cargo test --list` cannot print it; what enforces it is that this
        //! target must *build*, which `xtask/src/proof.rs` already does in order
        //! to list this file's neighbours. The module carries the rule's name so
        //! that a reader arriving from the clause finds it by grep.
        //!
        //! # Why not the `compile_fail` doctest the clause originally asked for
        //!
        //! Because it was measured and it does not work. A `compile_fail`
        //! doctest passes when the snippet fails to compile **for any reason**:
        //! of four spellings of this same assertion (ADR-0016 §13, D1 – D4 in
        //! `experiments/wire-format/src/lib.rs`), the honest one correctly
        //! *failed* against a serialisable `ReadOptions` while a type-name typo,
        //! a misspelt trait and a wrong crate path all reported ok. Three of
        //! four green against a false claim. And `tests/` cannot host one at
        //! all: an integration test that fails to compile fails the build.
        //!
        //! # The mechanism, which is the Rust-specific part
        //!
        //! Rust resolves an **inherent** associated item before a trait one, so
        //! [`Detect`] is given both: a trait constant defaulted to `false` that
        //! exists for every `T`, and an inherent constant of `true` that exists
        //! only where the bound holds. `Detect::<T>::IS_SERIALIZE` therefore
        //! names the inherent `true` when `T: Serialize` and falls back to the
        //! trait's `false` when it does not — the effect one would reach for
        //! specialisation to get, without the unstable feature. A misspelt type
        //! name is then `error[E0425]`, a build failure, which is exactly the
        //! property the doctest lacked.
        //!
        //! The `Deserialize` half is **not** a transcription of the `Serialize`
        //! one. `Deserialize<'de>` is parameterised by the lifetime of the data
        //! it borrows from, so `impl<T: serde::Deserialize> Detect<T>` is
        //! `error[E0106]: missing lifetime specifier`; what is wanted is the
        //! higher-ranked bound `for<'de> Deserialize<'de>`, and
        //! [`serde::de::DeserializeOwned`] is its spelling. One honest limit,
        //! measured: a *borrowing* `Deserialize` does not answer `false` here,
        //! it fails to compile — so `Detect` answers "does this deserialise from
        //! owned data", which is the question a wire format asks anyway.

        use core::marker::PhantomData;

        use happenstance_core::{Event, ReadOptions};
        use serde::Serialize;
        use serde::de::DeserializeOwned;

        /// The marker the two answers hang off.
        #[allow(
            dead_code,
            reason = "never constructed; it exists only to be named in a const"
        )]
        struct Detect<T>(PhantomData<T>);

        /// The default answer, available for every `T`.
        trait NotSerialize {
            const IS_SERIALIZE: bool = false;
        }
        impl<T> NotSerialize for Detect<T> {}

        /// The inherent answer, available only where the bound holds — and
        /// inherent items win, which is the whole mechanism.
        impl<T: Serialize> Detect<T> {
            const IS_SERIALIZE: bool = true;
        }

        /// The same shape for the read side.
        trait NotDeserialize {
            const IS_DESERIALIZE: bool = false;
        }
        impl<T> NotDeserialize for Detect<T> {}

        impl<T: DeserializeOwned> Detect<T> {
            const IS_DESERIALIZE: bool = true;
        }

        // WF-12 itself. `from` is a store-local position with no meaning at the
        // sender, and `backwards`/`limit` are traversal choices a reader makes
        // for itself; giving them impls would make a decoder's willingness to
        // accept them a promise this crate never meant to keep.
        const _: () = assert!(
            !Detect::<ReadOptions>::IS_SERIALIZE,
            "WF-12: ReadOptions must not implement Serialize"
        );
        const _: () = assert!(
            !Detect::<ReadOptions>::IS_DESERIALIZE,
            "WF-12: ReadOptions must not implement Deserialize"
        );

        // The detector's own control, and it is not decoration: every assertion
        // above is satisfied by a `Detect` whose inherent impl never applies —
        // a mistyped bound, a missing `serde` feature, an inherent constant
        // renamed out of the way — and that failure is silent in both
        // directions. `Event` is serialisable in this build, so if these two
        // ever stop holding, the two above have stopped meaning anything.
        const _: () = assert!(
            Detect::<Event>::IS_SERIALIZE,
            "the Serialize detector answers `false` for a type that derives it, \
             so WF-12's assertion above is passing vacuously"
        );
        const _: () = assert!(
            Detect::<Event>::IS_DESERIALIZE,
            "the Deserialize detector answers `false` for a type that derives \
             it, so WF-12's assertion above is passing vacuously"
        );
    }

    // =====================================================================
    // Negative controls
    // =====================================================================

    mod negative_controls {
        //! The wrong implementations, kept alive so that "demonstrated to have
        //! failed before the change" is a standing property rather than a
        //! sentence in a commit message.
        //!
        //! Each control is a local mirror of a shape this crate used to have,
        //! plus a test asserting it **still fails**. They are the reason the
        //! rules above are not decorative: a rule that no implementation can
        //! fail certifies nothing, and the only way to know a rule discriminates
        //! is to keep something it rejects.
        //!
        //! # These are fragile, and nothing here prevents that
        //!
        //! Stated plainly rather than left to be discovered. Nothing in the
        //! compiler, the linter or the gate requires these mirrors to exist.
        //! They implement no trait the crate needs, they are called by nothing
        //! outside this module, and to a reader who has not read ADR-0016 they
        //! look exactly like dead code that someone forgot — a struct with a
        //! bug in it, asserted to have the bug. The first person who tidies this
        //! file will delete them, and every rule above will keep passing.
        //!
        //! What stands against that is `xtask/src/proof.rs`, which asserts these
        //! test names are present in `cargo test -- --list` before it runs
        //! anything, the same way it already does for the testkit's mutation
        //! registry. Deleting a control fails the gate with a message naming it.
        //! The link is recorded here as well as there because a reader arriving
        //! from this end has no other way to discover it.

        use super::{
            Bytes, Event, MIN_SUPPORTED_EVENT_DATA_LEN, Query, TRAILER, frame_and_take,
            postcard_round_trip_framed,
        };
        use serde::{Deserialize, Deserializer, Serialize, Serializer};

        /// `EventWire` as it stood before ADR-0016: `tags` and `metadata`
        /// skipped when they are at their default.
        ///
        /// A local mirror rather than a reference to the real type, for the
        /// reason `w1_postcard_desynchronisation.rs` gives: a control that
        /// stops reproducing the defect the moment the defect is fixed is not a
        /// control. This reproduces D1's shape for as long as the file exists,
        /// whatever `happenstance-core` looks like by then.
        ///
        /// `String` and `Vec<u8>` rather than `EventType` and `Bytes` because
        /// the defect is the attribute, not the field types, and the plain types
        /// encode identically — a length-prefixed string and a length-prefixed
        /// byte sequence.
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        struct SkippedEventWire {
            event_type: String,
            data: Vec<u8>,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            tags: Vec<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            metadata: Option<Vec<u8>>,
        }

        /// The same four fields with the attributes removed — what the crate
        /// writes now.
        ///
        /// Its presence is what makes the test a control rather than a
        /// demonstration: the two structs differ in the attributes and nothing
        /// else, so the difference in outcome cannot be blamed on the field
        /// types, the harness or the trailer.
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        struct UnskippedEventWire {
            event_type: String,
            data: Vec<u8>,
            tags: Vec<String>,
            metadata: Option<Vec<u8>>,
        }

        /// D1, still wrong: a skipped field makes the decoder read its
        /// neighbour's bytes and return `Ok`.
        ///
        /// This is the wrong implementation `wire::round_trips_in_postcard`
        /// exists to reject (WF-2, WF-7). The assertion is deliberately not
        /// `is_err`: with a trailer behind it the decode **succeeds**, and what
        /// comes back is an event whose metadata is two bytes that were never in
        /// it. That silent wrong value is what D1 was in a real message, and a
        /// control asserting only "this errors" would have been satisfied by a
        /// truncated buffer.
        #[test]
        fn skipped_event_wire_fails_the_postcard_round_trip() {
            let original = SkippedEventWire {
                event_type: "A".into(),
                data: vec![0x11, 0x22, 0x33, 0x44],
                tags: vec!["course:c1".into()],
                metadata: None,
            };

            let (decoded, rest) = frame_and_take(&original)
                .expect("the decode SUCCEEDS — that it does not error is the finding");

            assert_ne!(
                decoded, original,
                "the skipped-field mirror round-tripped intact, so either the \
                 trailer stopped following the value or postcard stopped being \
                 positional. Either way this control has stopped controlling \
                 anything and `wire::round_trips_in_postcard` is now unfalsified."
            );
            assert_eq!(
                decoded.metadata,
                Some(vec![0xAA, 0xBB]),
                "the decoder read the trailer's `01 02 AA BB` as `Some`, a \
                 two-byte length, and a payload — a field the encoder never wrote"
            );
            assert_eq!(
                rest,
                [TRAILER[4]],
                "and it consumed four of the trailer's five bytes doing it"
            );

            // The control's other half: the identical struct without the
            // attributes passes the identical frame.
            postcard_round_trip_framed(&UnskippedEventWire {
                event_type: original.event_type,
                data: original.data,
                tags: original.tags,
                metadata: original.metadata,
            });
        }

        /// `Query` as it stood before ADR-0016 §7: the variant carried by
        /// `Option`'s own tag rather than by one of its own.
        ///
        /// `Vec<String>` stands in for `Vec<QueryItem>` for the reason
        /// [`SkippedEventWire`] uses plain types: the defect is the *shape* of
        /// the encoding, and a sequence is a sequence in both formats.
        #[derive(Debug, Clone, PartialEq)]
        enum OptionShapedQuery {
            All,
            Items(Vec<String>),
        }

        impl Serialize for OptionShapedQuery {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {
                    Self::All => serializer.serialize_none(),
                    Self::Items(items) => serializer.serialize_some(items),
                }
            }
        }

        impl<'de> Deserialize<'de> for OptionShapedQuery {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                Ok(match Option::<Vec<String>>::deserialize(deserializer)? {
                    None => Self::All,
                    Some(items) => Self::Items(items),
                })
            }
        }

        /// HEAD's `Query`, still ambiguous: `Some(All)` and `None` are one
        /// encoding, and the receiver resolves the ambiguity as `None`.
        ///
        /// This is the wrong implementation `wire::query_all_is_unambiguous`
        /// exists to reject (WF-1, WF-3), and it is a control rather than a
        /// demonstration because the real `Query` is put through the same three
        /// comparisons below and passes each one.
        ///
        /// The reason it cannot be fixed by trying harder inside the `Option`:
        /// `serialize_some(v)` is transparent in serde's data model, so no
        /// choice of inner value makes `Some(v)` differ from `v`. Only a tag
        /// belonging to the type itself does.
        #[test]
        fn option_shaped_query_is_indistinguishable_from_none() {
            let all = serde_json::to_string(&OptionShapedQuery::All).unwrap();
            let absent = serde_json::to_string(&None::<OptionShapedQuery>).unwrap();

            assert_eq!(
                all, absent,
                "the `Option`-shaped mirror stopped being ambiguous, so either \
                 serde's `serialize_none` changed or this control has stopped \
                 controlling anything and `wire::query_all_is_unambiguous` is \
                 now unfalsified"
            );
            assert_eq!(all, "null", "and the value they share is the format's null");

            // The read side, which is what `option_query_round_trips` names:
            // the ambiguity is resolved the other way, so a match-all query
            // sent is an absent query received.
            assert_eq!(
                serde_json::from_str::<Option<OptionShapedQuery>>(&all).unwrap(),
                None,
                "`Some(All)` came back as something other than `None`, which \
                 would mean the transparent-`Some` behaviour this control \
                 depends on no longer holds"
            );

            // And the third loss, the one that needs no `Option` at all: an
            // `Items` inside a `Some` is byte-identical to a bare `Items`.
            let items = OptionShapedQuery::Items(vec!["course:c1".into()]);
            assert_eq!(
                serde_json::to_string(&Some(items.clone())).unwrap(),
                serde_json::to_string(&items).unwrap(),
                "`serialize_some` stopped being transparent"
            );

            // The control's other half: the real type, over the same three
            // comparisons, distinguishing where the mirror cannot.
            assert_ne!(
                serde_json::to_string(&Query::all()).unwrap(),
                serde_json::to_string(&None::<Query>).unwrap(),
                "the tagged encoding is as ambiguous as the one it replaced"
            );
            assert_eq!(
                serde_json::from_str::<Option<Query>>(
                    &serde_json::to_string(&Some(Query::all())).unwrap()
                )
                .unwrap(),
                Some(Query::all()),
                "`Some(Query::All)` no longer survives the wire"
            );
            postcard_round_trip_framed(&Some(Query::all()));
        }

        /// An `Event` wire form whose `Deserialize` grew a capacity check.
        ///
        /// Not a shape this crate ever had — the other two controls reproduce
        /// something that shipped, and this one reproduces something a
        /// contributor is about to write. WF-10 asks every `Deserialize` in the
        /// crate to re-run its constructor's invariants, and four lines below
        /// that instruction sits a constant named `MIN_SUPPORTED_EVENT_DATA_LEN`
        /// that reads exactly like the invariant to re-run. It is not one: it is
        /// a **floor** every store must clear, and the check below is what
        /// mistaking it for a ceiling looks like.
        #[derive(Serialize, Debug, Clone, PartialEq)]
        struct LengthCheckedEventWire {
            event_type: String,
            data: Vec<u8>,
        }

        impl<'de> Deserialize<'de> for LengthCheckedEventWire {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let wire = UncheckedEventWire::deserialize(deserializer)?;
                if wire.data.len() > MIN_SUPPORTED_EVENT_DATA_LEN {
                    return Err(serde::de::Error::custom(
                        "event data exceeds the supported length",
                    ));
                }
                Ok(Self {
                    event_type: wire.event_type,
                    data: wire.data,
                })
            }
        }

        /// The same two fields with no check — the control's other half.
        ///
        /// It is what makes the outcome attributable: the two structs share a
        /// wire encoding byte for byte, so a decode that succeeds here and fails
        /// above cannot be blamed on the bytes, the payload size or postcard.
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        struct UncheckedEventWire {
            event_type: String,
            data: Vec<u8>,
        }

        /// A capacity check in a `Deserialize`, still refusing a payload the
        /// contract requires every store to accept.
        ///
        /// This is the wrong implementation
        /// `wire::decode_accepts_an_over_capacity_value` exists to reject
        /// (WF-9, VT-19). The assertion is `is_err` here — unlike the other two
        /// controls, where the finding is that the wrong implementation returns
        /// `Ok` — because refusing the message *is* the defect: a store that
        /// cannot hold this payload is required to say so at append, with
        /// `AppendError::ExceedsStoreLimit`, which a caller can tell apart from
        /// a malformed message. A decoder that refuses it has taken that
        /// distinction away from every peer downstream of it.
        #[test]
        fn length_checked_event_wire_rejects_a_65_kib_payload() {
            // One byte past the floor, built here rather than shared with the
            // rule it backs: a control that drifts with the thing it controls is
            // not a control.
            let data: Vec<u8> = (0..=MIN_SUPPORTED_EVENT_DATA_LEN)
                .map(|index| u8::try_from(index % 251).expect("251 fits in a u8"))
                .collect();
            let length = data.len();
            assert!(
                length > MIN_SUPPORTED_EVENT_DATA_LEN,
                "the control stopped exceeding the floor it is about"
            );

            let encoded = postcard::to_stdvec(&LengthCheckedEventWire {
                event_type: "SeatMapPublished".into(),
                data: data.clone(),
            })
            .expect("the check is on the read side only");

            assert!(
                postcard::from_bytes::<LengthCheckedEventWire>(&encoded).is_err(),
                "the length-checked mirror accepted its own encoding, so this \
                 control has stopped controlling anything and \
                 `wire::decode_accepts_an_over_capacity_value` is now unfalsified"
            );

            // Same bytes, no check: the message was well formed all along.
            let unchecked = postcard::from_bytes::<UncheckedEventWire>(&encoded)
                .expect("the bytes are well formed — only the check refused them");
            assert_eq!(
                unchecked.data.len(),
                length,
                "the payload the checked mirror refused did not survive the \
                 unchecked one either, so the refusal was not the check"
            );

            // And the real type, over the same payload, doing what the rule
            // asserts.
            let event = Event::new("SeatMapPublished", Bytes::from(data))
                .expect("the contract crate applies no capacity limit");
            postcard_round_trip_framed(&event);
        }
    }
}
