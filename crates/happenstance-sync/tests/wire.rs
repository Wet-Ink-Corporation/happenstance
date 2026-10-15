//! WF-8's two rules, then the message set's tests: an envelope at a version this build does
//! not implement is **refused distinguishably**, and the refusal happens
//! **before the message's `Deserialize` is ever called**.
//!
//! # What each half proves, and why it takes two
//!
//! The first is a claim about the *result* of decoding, and it is cheap. The
//! second is a claim about what did **not** happen on the way there, and it is
//! the expensive one — because ADR-0016 §11 measured that nothing observable in
//! the result distinguishes the implementation WF-8's MUST NOT forbids from the
//! one it requires. A derived `Deserialize` plus a version check run afterwards
//! returns the *same* `Err` in both formats, and produces *byte-identical*
//! postcard framing: `[01 07]` at version 1, `[80 03 07]` at 384, the same
//! `take_from_bytes::<u16>` value and the same `[07]` remainder. So neither the
//! error nor the buffer separates them, and a rule asserting on either would
//! pass against the forbidden implementation — the definition of decorative.
//!
//! The one separating observation is whether `T::deserialize` ran at all. That
//! is what [`version_is_readable_before_the_message`] witnesses, and it witnesses
//! it in both directions in the same test: zero calls through the real
//! [`Envelope`], one call through a locally-declared derived envelope decoding
//! the *same bytes*. A reader sees the discrimination rather than taking it on
//! trust.
//!
//! # Why the tests live in `mod wire`
//!
//! `SPECIFICATION.md` names both rules as `wire::<name>`, and a rule name that
//! `cargo test --list` cannot print is a citation that resolves to nothing. At
//! the file's top level these would list as `<name>`. Same reason, same trick as
//! `happenstance-testkit/tests/mutation_coverage.rs`.

#![allow(clippy::unwrap_used, reason = "test code, per the house style")]

mod wire {
    use core::cell::Cell;

    use happenstance_sync::wire::{Envelope, FORMAT_VERSION, WireError, check_format_version};
    // One import covers both namespaces: with `serde`'s `derive` feature on,
    // `serde::Deserialize` names the trait *and* the derive macro, so the
    // hand-written impl on `Witness` and the derive on `DerivedEnvelope` — the
    // two halves this file exists to contrast — read at the same spelling.
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// A version no build ever speaks, written as an offset from the one this
    /// build does speak. A literal would quietly become *the* supported version
    /// the day `FORMAT_VERSION` is bumped past it, and both tests would then
    /// assert the opposite of what they say.
    const UNSUPPORTED_VERSION: u16 = FORMAT_VERSION.wrapping_add(1);

    /// The payload every envelope in this file carries, so that "the message was
    /// decoded" and "the message came back" are claims about the same value.
    const SENTINEL: u32 = 7;

    const MESSAGE: &str = "hello";

    // =====================================================================
    // The foreign encoder, and why a test needs one
    // =====================================================================

    /// What a peer one format version ahead puts on the wire.
    ///
    /// `Envelope` deliberately has no constructor that stamps an arbitrary
    /// version — a sender that can name a version it does not implement can send
    /// one — so the buffers these tests decode cannot come from the type under
    /// test. They come from here instead. The framing is byte-identical to the
    /// real envelope's in both formats; measured in
    /// `experiments/wire-format/tests/decorative_envelope_witness.rs`, and
    /// re-proved at the top of [`rejects_an_unknown_format_version`] so that a
    /// refusal can never be a framing mismatch wearing a version refusal's face.
    #[derive(Serialize)]
    struct ForeignEnvelope<T> {
        format_version: u16,
        message: T,
    }

    fn foreign_json<T: Serialize>(format_version: u16, message: T) -> String {
        serde_json::to_string(&ForeignEnvelope {
            format_version,
            message,
        })
        .unwrap()
    }

    fn foreign_postcard<T: Serialize>(format_version: u16, message: T) -> Vec<u8> {
        postcard::to_stdvec(&ForeignEnvelope {
            format_version,
            message,
        })
        .unwrap()
    }

    // =====================================================================
    // Rule: wire::rejects_an_unknown_format_version
    // =====================================================================

    /// An envelope at a version this build does not implement is refused in both
    /// formats, and the refusal is distinguishable from a malformed message.
    ///
    /// **Wrong implementation it rejects:** an envelope that reads the version
    /// and does not act on it — the field present, decoded, and inert. Such an
    /// envelope returns `Ok` from every decode below.
    ///
    /// The distinguishability half is not decoration either. "Refused, park this
    /// and retry after an upgrade" and "malformed, drop this" are opposite
    /// instructions, and parking bytes that will never decode is a leak.
    ///
    /// **It is spelled differently in each format, and that is a measurement
    /// rather than a preference.** `serde`'s only channel out of a `Deserialize`
    /// is `Error::custom`, which takes a `Display` and erases the type;
    /// `serde_json` keeps the string, so in JSON the answer is
    /// [`WireError::UNSUPPORTED_FORMAT_VERSION`]'s text — public for exactly that
    /// reason. `postcard::Error` is a fieldless `#[non_exhaustive]` enum, so its
    /// `custom` **discards the message** and every refusal arrives as
    /// `SerdeDeCustom`. The text therefore cannot be the postcard spelling, and
    /// the typed answer there comes from peeking the version off the front of the
    /// buffer and asking [`check_format_version`] — which is what that function
    /// is public for.
    #[test]
    fn rejects_an_unknown_format_version() {
        // The foreign encoder speaks the real envelope's framing: at the
        // supported version its bytes decode, and the message survives.
        let supported_json = foreign_json(FORMAT_VERSION, MESSAGE);
        let supported_postcard = foreign_postcard(FORMAT_VERSION, MESSAGE);
        assert_eq!(
            serde_json::from_str::<Envelope<String>>(&supported_json)
                .unwrap()
                .message(),
            MESSAGE,
            "the foreign encoder's JSON is the real envelope's JSON, so anything \
             refused below is refused for its version and not for its shape"
        );
        assert_eq!(
            postcard::from_bytes::<Envelope<String>>(&supported_postcard)
                .unwrap()
                .message(),
            MESSAGE,
            "the same, positionally: the foreign encoder's postcard framing is \
             the real envelope's postcard framing"
        );

        // ---- JSON: the marker text is what reaches the receiver ----
        let unknown_json = foreign_json(UNSUPPORTED_VERSION, MESSAGE);
        let refusal = serde_json::from_str::<Envelope<String>>(&unknown_json)
            .unwrap_err()
            .to_string();
        assert!(
            refusal.contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "in JSON, an envelope from a version this build does not implement is \
             refused with a marker a receiver can park on; got {refusal:?}"
        );

        // A malformed message at a *supported* version must not carry that
        // marker, or "park this" and "drop this" become the same signal.
        let wrong_type = serde_json::from_str::<Envelope<String>>(&foreign_json(
            FORMAT_VERSION,
            [SENTINEL, SENTINEL],
        ))
        .unwrap_err()
        .to_string();
        assert!(
            !wrong_type.contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "a JSON message of the wrong type is malformed, not from the future; \
             parking it would leak bytes that will never decode. got {wrong_type:?}"
        );

        // ---- postcard: the marker does not survive the format's error type ----
        let unknown_postcard = foreign_postcard(UNSUPPORTED_VERSION, MESSAGE);
        let refusal = postcard::from_bytes::<Envelope<String>>(&unknown_postcard).unwrap_err();
        assert_eq!(
            refusal,
            postcard::Error::SerdeDeCustom,
            "postcard refuses the unknown version too — an envelope that read the \
             version and did nothing with it would return Ok here"
        );
        assert!(
            !refusal
                .to_string()
                .contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "measured, and the reason this half of the rule is spelled \
             differently: `postcard::Error` carries no payload, so its \
             `de::Error::custom` discards the message. The text is dropped by the \
             format, not by us, and pinning that here stops a future reader \
             assuming the JSON spelling generalises. got {refusal}"
        );

        // Refusal and malformed-ness stay apart in band even so, because they
        // arrive as different variants. Truncated to exactly the version, using
        // postcard's own answer for where the version ends rather than a byte
        // count a varint would make a lie: `1 => [01]` but `384 => [80 03]`.
        let (found, after_version) = postcard::take_from_bytes::<u16>(&supported_postcard).unwrap();
        let version_only = &supported_postcard[..supported_postcard.len() - after_version.len()];
        assert_eq!(found, FORMAT_VERSION);
        assert_eq!(
            postcard::from_bytes::<Envelope<String>>(version_only).unwrap_err(),
            postcard::Error::DeserializeUnexpectedEnd,
            "a buffer that stops after the version is malformed, and postcard says \
             so with a different variant than it uses for a refusal"
        );

        // And out of band, the typed answer a postcard receiver actually routes
        // on: peek the version, ask the decoder's own question of it.
        let (found, _) = postcard::take_from_bytes::<u16>(&unknown_postcard).unwrap();
        assert_eq!(
            check_format_version(found),
            Err(WireError::UnsupportedFormatVersion {
                found: UNSUPPORTED_VERSION,
                supported: FORMAT_VERSION,
            }),
            "the version is legible on the front of the buffer without decoding \
             the message, and `check_format_version` gives the receiver the typed \
             refusal postcard's error type cannot carry"
        );
    }

    // =====================================================================
    // Rule: wire::version_is_readable_before_the_message
    // =====================================================================

    // A thread-local rather than a static, so that `cargo test` running this
    // file's tests on separate threads cannot interleave counts — which is also
    // why no mutex is needed: every decode below happens on the test's own
    // thread, and each assertion is preceded by a reset it cannot race.
    thread_local! {
        static MESSAGE_DECODES: Cell<u32> = const { Cell::new(0) };
    }

    /// A message that records having been decoded.
    ///
    /// This is the whole instrument. `Serialize` is written out rather than
    /// derived so the encoded form is a bare `u32` in both formats with no
    /// newtype wrapper to reason about; `Deserialize` is written out because
    /// counting is the point.
    #[derive(Debug, PartialEq, Eq)]
    struct Witness(u32);

    impl Serialize for Witness {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            self.0.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Witness {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            MESSAGE_DECODES.with(|calls| calls.set(calls.get() + 1));
            u32::deserialize(deserializer).map(Witness)
        }
    }

    /// Runs one decode with the counter zeroed, and reports what it observed.
    fn while_counting<R>(decode: impl FnOnce() -> R) -> (R, u32) {
        MESSAGE_DECODES.with(|calls| calls.set(0));
        let result = decode();
        (result, MESSAGE_DECODES.with(Cell::get))
    }

    /// The implementation WF-8's MUST NOT forbids, declared locally so the test
    /// can measure it rather than describe it: `#[derive(Deserialize)]` reads
    /// **every** field into a local `Option` and only then constructs the
    /// struct, so the version check below — the very same
    /// [`check_format_version`] the real envelope calls, moved and nothing else
    /// — runs after the message has already been decoded.
    #[derive(Deserialize)]
    struct DerivedEnvelope<T> {
        format_version: u16,
        message: T,
    }

    fn decode_derived_json(json: &str) -> Result<Witness, String> {
        let envelope: DerivedEnvelope<Witness> =
            serde_json::from_str(json).map_err(|error| error.to_string())?;
        check_format_version(envelope.format_version).map_err(|error| error.to_string())?;
        Ok(envelope.message)
    }

    fn decode_derived_postcard(bytes: &[u8]) -> Result<Witness, String> {
        let envelope: DerivedEnvelope<Witness> =
            postcard::from_bytes(bytes).map_err(|error| error.to_string())?;
        check_format_version(envelope.format_version).map_err(|error| error.to_string())?;
        Ok(envelope.message)
    }

    /// On an unsupported version the message's `Deserialize` never runs — and
    /// the forbidden implementation, on the same bytes, runs it once.
    ///
    /// **Wrong implementation it rejects:** a derived `Deserialize` plus a
    /// post-hoc version check. `DerivedEnvelope` above *is* that implementation,
    /// and the second half of this test is it failing.
    ///
    /// This is a witness test and not a byte-layout test on purpose. ADR-0016
    /// §11 measured that at an unknown version both implementations return `Err`
    /// in both formats and encode to identical bytes, so the only rule that
    /// rejects the forbidden one asks a question about execution rather than
    /// about data.
    #[test]
    fn version_is_readable_before_the_message() {
        let unknown_json = foreign_json(UNSUPPORTED_VERSION, Witness(SENTINEL));
        let unknown_postcard = foreign_postcard(UNSUPPORTED_VERSION, Witness(SENTINEL));

        // What WF-8 requires: refused, with the message untouched.
        let (refused, decodes) =
            while_counting(|| serde_json::from_str::<Envelope<Witness>>(&unknown_json));
        assert!(refused.is_err(), "an unknown version is refused in JSON");
        assert_eq!(
            decodes, 0,
            "in JSON, no part of the message may be decoded before the version \
             has been checked and accepted"
        );

        let (refused, decodes) =
            while_counting(|| postcard::from_bytes::<Envelope<Witness>>(&unknown_postcard));
        assert!(
            refused.is_err(),
            "an unknown version is refused in postcard"
        );
        assert_eq!(
            decodes, 0,
            "in postcard, likewise — and here it is the declaration order of \
             `format_version` that puts the version in front of the message"
        );

        // The same bytes through the forbidden implementation. Both `Err`, which
        // is exactly why the assertion above had to be about the count.
        let (refused, decodes) = while_counting(|| decode_derived_json(&unknown_json));
        assert!(
            refused.is_err(),
            "a derive plus a post-hoc check refuses too — the error is not the \
             discriminator, which is this test's reason for existing"
        );
        assert_eq!(
            decodes, 1,
            "a derived `Deserialize` decodes the whole message before any check \
             can look at the version: the partial decode WF-8 forbids"
        );

        let (refused, decodes) = while_counting(|| decode_derived_postcard(&unknown_postcard));
        assert!(
            refused.is_err(),
            "and in postcard, same refusal, same bytes"
        );
        assert_eq!(
            decodes, 1,
            "and in postcard, the same forbidden decode — 1 against the real \
             envelope's 0 is the whole of the discrimination"
        );

        // Finally: the witness can count at all. Without this, a `Witness` whose
        // `Deserialize` was never reachable would report 0 forever and the two
        // assertions above would be vacuous.
        let supported_json = serde_json::to_string(&Envelope::new(Witness(SENTINEL))).unwrap();
        let (accepted, decodes) =
            while_counting(|| serde_json::from_str::<Envelope<Witness>>(&supported_json));
        assert_eq!(accepted.unwrap().message(), &Witness(SENTINEL));
        assert_eq!(
            decodes, 1,
            "at the supported version the message is decoded exactly once, so a \
             count of 0 above is a refusal and not a broken instrument"
        );

        let supported_postcard = postcard::to_stdvec(&Envelope::new(Witness(SENTINEL))).unwrap();
        let (accepted, decodes) =
            while_counting(|| postcard::from_bytes::<Envelope<Witness>>(&supported_postcard));
        assert_eq!(accepted.unwrap().message(), &Witness(SENTINEL));
        assert_eq!(decodes, 1, "and the same in postcard");
    }

    // =====================================================================
    // The message set (#114, ADR-0097)
    //
    // Each message is its own type in `Envelope<T>`. These tests are about
    // how those types encode, so they are wire tests and not conformance
    // rules: a peer adapter does not implement them, it inherits them.
    // =====================================================================

    use core::fmt::Debug;
    use core::num::NonZeroU64;

    use happenstance_core::{
        AppendCondition, Event, EventId, EventType, Query, QueryItem, RecordedAt, SequencePosition,
        StoreId, Tag, Tags,
    };
    use happenstance_sync::wire::{PullRequest, ResumeToken};
    use happenstance_sync::{
        Ack, EventGroup, Pulled, PushBatch, ReplicatedEvent, Waiting, Watermark,
    };
    use serde::de::DeserializeOwned;

    const ORIGIN: StoreId = StoreId::from_bytes([0x5E; 16]);
    const OTHER_ORIGIN: StoreId = StoreId::from_bytes([0x0A; 16]);
    const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");

    /// WF-11's measured example: base64 `3q2+7w==`.
    const PAYLOAD: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];

    fn seat() -> Tags {
        [Tag::from_static("seat:s1")].into_iter().collect()
    }

    /// The second position a store assigns, without writing it as a literal.
    fn second() -> SequencePosition {
        SequencePosition::FIRST.next().unwrap()
    }

    fn replicated(origin: StoreId, position: SequencePosition) -> ReplicatedEvent {
        let event = Event::new(SEAT_CLAIMED, PAYLOAD.to_vec())
            .unwrap()
            .with_tags(seat());
        ReplicatedEvent::new(
            EventId::new(origin, position),
            RecordedAt::from_millis(1_800_000_000_000),
            event,
        )
    }

    /// Two groups a receiver must be told apart: one guarded event, then two
    /// unguarded events that landed together.
    fn two_groups() -> PushBatch {
        let guard = AppendCondition::new(Query::from_item(
            QueryItem::new([SEAT_CLAIMED], seat()).unwrap(),
        ));
        PushBatch::new(vec![
            EventGroup::new(
                Some(guard),
                vec![replicated(ORIGIN, SequencePosition::FIRST)],
            ),
            EventGroup::new(
                None,
                vec![
                    replicated(OTHER_ORIGIN, SequencePosition::FIRST),
                    replicated(OTHER_ORIGIN, second()),
                ],
            ),
        ])
    }

    // The wrong implementations, one per test below.

    /// `#[derive(Deserialize)]` over a real message: reads the message before
    /// anything can look at the version.
    #[derive(Debug, Deserialize)]
    struct DerivedPushEnvelope {
        format_version: u16,
        message: PushBatch,
    }

    /// A message carrying the payload in a field of its own, outside `Event`.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct RewrappedPayload {
        data: Vec<u8>,
    }

    /// A push flattened to its events.
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct FlattenedPush {
        events: Vec<ReplicatedEvent>,
    }

    /// `PullRequest` without `#[serde(deny_unknown_fields)]`.
    #[derive(Debug, PartialEq, Deserialize)]
    struct PermissivePullRequest {
        from: Option<ResumeToken>,
    }

    /// The derive `Watermark`'s hand-written `Deserialize` replaces.
    #[derive(Debug, Deserialize)]
    #[serde(transparent)]
    struct DerivedWatermark(Vec<(StoreId, SequencePosition)>);

    fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(message: &T) {
        let envelope = Envelope::new(message);
        let json = serde_json::to_string(&envelope).unwrap();
        let from_json: Envelope<T> = serde_json::from_str(&json).unwrap();
        assert_eq!(from_json.message(), message, "JSON: {json}");

        let bytes = postcard::to_stdvec(&envelope).unwrap();
        let from_postcard: Envelope<T> = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(from_postcard.message(), message, "postcard: {bytes:02x?}");
    }

    /// The four messages, each in its own envelope, in both formats.
    ///
    /// It has no control, and no compiled wrong implementation fails it: a
    /// derived pair round-trips by construction. It is a smoke test that each
    /// message type encodes and decodes at all; the tests below carry the
    /// controls.
    #[test]
    fn the_message_set_round_trips_in_both_formats() {
        round_trip(&two_groups());
        round_trip(&PushBatch::empty());

        // Fixed-width on the wire (ADR-0097 §2): a count a 64-bit sender can
        // write, a 32-bit receiver can hold.
        let mut ack = Ack::default();
        ack.appended = u64::MAX;
        ack.skipped = 1;
        ack.confirmed.advance(ORIGIN, SequencePosition::FIRST);
        ack.confirmed.advance(OTHER_ORIGIN, second());
        round_trip(&ack);

        round_trip(&PullRequest::from_start());
        round_trip(&PullRequest::after(ResumeToken::new("group:2")));
        round_trip(&Pulled::new(two_groups(), ResumeToken::new("group:2")));
        for waiting in [
            Waiting::Nothing,
            Waiting::Unknown,
            Waiting::Events(NonZeroU64::MAX),
        ] {
            round_trip(
                &Pulled::new(two_groups(), ResumeToken::new("group:2")).with_waiting(waiting),
            );
        }
    }

    /// `Pulled::waiting` on the wire (SY-37): always present, declared last,
    /// externally tagged. Postcard's variant index is the declaration order, so
    /// the trailing bytes are pinned — reordering the variants would re-mean
    /// every buffer a peer already sent.
    #[test]
    fn pulled_carries_waiting_in_both_formats() {
        let three = NonZeroU64::new(3).unwrap();
        let pulled = Pulled::new(PushBatch::empty(), ResumeToken::new("group:0"));
        let nothing = pulled.clone().with_waiting(Waiting::Nothing);
        let counted = pulled.clone().with_waiting(Waiting::Events(three));

        assert_eq!(
            serde_json::to_string(&nothing).unwrap(),
            r#"{"batch":{"groups":[]},"resume":"group:0","waiting":"nothing"}"#
        );
        assert_eq!(
            serde_json::to_string(&counted).unwrap(),
            r#"{"batch":{"groups":[]},"resume":"group:0","waiting":{"events":3}}"#
        );
        assert_eq!(
            serde_json::to_string(&pulled).unwrap(),
            r#"{"batch":{"groups":[]},"resume":"group:0","waiting":"unknown"}"#,
            "the default travels too, never as a missing key (WF-2)"
        );

        // The fields before `waiting`, encoded positionally as postcard
        // encodes the struct, so only the trailing bytes are asserted here.
        let before =
            postcard::to_stdvec(&(PushBatch::empty(), ResumeToken::new("group:0"))).unwrap();
        let tail = |pulled: &Pulled<ResumeToken>| {
            let bytes = postcard::to_stdvec(pulled).unwrap();
            assert!(bytes.starts_with(&before), "{bytes:02x?}");
            bytes.get(before.len()..).unwrap().to_vec()
        };
        assert_eq!(tail(&nothing), [0x00]);
        assert_eq!(tail(&pulled), [0x01]);
        assert_eq!(tail(&counted), [0x02, 0x03]);
    }

    /// `Events(0)` would be a second spelling of `Nothing`, so the decoder
    /// refuses a zero count from the wire in both formats (WF-10). The control
    /// is the smallest count that is allowed, at the same bytes otherwise.
    #[test]
    fn a_zero_event_count_is_refused() {
        let refused = serde_json::from_str::<Waiting>(r#"{"events":0}"#)
            .unwrap_err()
            .to_string();
        assert!(refused.contains("expected a nonzero u64"), "{refused}");
        assert_eq!(
            postcard::from_bytes::<Waiting>(&[0x02, 0x00]).unwrap_err(),
            postcard::Error::SerdeDeCustom,
            "postcard drops the text, as it does for every refusal"
        );

        let one = Waiting::Events(NonZeroU64::MIN);
        assert_eq!(
            serde_json::from_str::<Waiting>(r#"{"events":1}"#).unwrap(),
            one
        );
        assert_eq!(postcard::from_bytes::<Waiting>(&[0x02, 0x01]).unwrap(), one);
    }

    /// WF-8 over a real message. The message is malformed *and* the version is
    /// unknown, so the error says which was looked at first.
    ///
    /// The derived envelope beside it is the control: it reads the message
    /// before it can see the version, and reports the malformed message
    /// instead. Postcard drops error text, so there the version is read off the
    /// front of the buffer, as `check_format_version`'s documentation says.
    #[test]
    fn a_real_message_is_refused_before_it_decodes() {
        let malformed = serde_json::json!({ "groups": "not a list of groups" });

        let ahead = foreign_json(UNSUPPORTED_VERSION, &malformed);
        let refused = serde_json::from_str::<Envelope<PushBatch>>(&ahead).unwrap_err();
        assert!(
            refused
                .to_string()
                .contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "the version is refused before the malformed batch is read: {refused}"
        );

        let current = foreign_json(FORMAT_VERSION, &malformed);
        let malformed_error = serde_json::from_str::<Envelope<PushBatch>>(&current).unwrap_err();
        assert!(
            !malformed_error
                .to_string()
                .contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "at a supported version the same bytes fail as malformed, not refused: \
             {malformed_error}"
        );
        let sane: DerivedPushEnvelope =
            serde_json::from_str(&foreign_json(FORMAT_VERSION, PushBatch::empty())).unwrap();
        assert_eq!(
            (sane.format_version, sane.message),
            (FORMAT_VERSION, PushBatch::empty()),
            "the control decodes a well-formed message, so its failure below is the order's"
        );
        let derived = serde_json::from_str::<DerivedPushEnvelope>(&ahead).unwrap_err();
        assert!(
            !derived
                .to_string()
                .contains(WireError::UNSUPPORTED_FORMAT_VERSION),
            "the control decodes the message first and never reaches the version: {derived}"
        );

        // Postcard, the same shape: the version, then a batch that claims five
        // groups and ends. A decoder that read the message first would run off
        // the end and say `DeserializeUnexpectedEnd`; the real envelope stops at
        // the version and says `SerdeDeCustom`, which is all postcard can say.
        let truncated_batch = |version: u16| {
            let mut bytes = postcard::to_stdvec(&version).unwrap();
            bytes.push(5);
            bytes
        };
        let bytes = truncated_batch(UNSUPPORTED_VERSION);
        assert_eq!(
            postcard::from_bytes::<Envelope<PushBatch>>(&bytes).unwrap_err(),
            postcard::Error::SerdeDeCustom,
            "the version is refused before the truncated batch is read: {bytes:02x?}"
        );
        assert_eq!(
            postcard::from_bytes::<Envelope<PushBatch>>(&truncated_batch(FORMAT_VERSION))
                .unwrap_err(),
            postcard::Error::DeserializeUnexpectedEnd,
            "at a supported version the same batch fails as truncated, not refused"
        );
        assert_eq!(
            postcard::from_bytes::<DerivedPushEnvelope>(&bytes).unwrap_err(),
            postcard::Error::DeserializeUnexpectedEnd,
            "the control reads the batch first and runs off its end"
        );
        let (found, _) = postcard::take_from_bytes::<u16>(&bytes).unwrap();
        assert_eq!(
            check_format_version(found),
            Err(WireError::UnsupportedFormatVersion {
                found: UNSUPPORTED_VERSION,
                supported: FORMAT_VERSION,
            })
        );
    }

    /// WF-11 through the message set: a forwarded payload renders through
    /// `happenstance-core`'s own codec, base64 in JSON and raw in postcard.
    ///
    /// The control is a message that carries the payload in a field of its
    /// own rather than inside `Event` — the "simplification" a message set
    /// invites. It round-trips equal in both formats, and renders the JSON
    /// array of integers WF-11 exists to forbid, which only an assertion on
    /// the rendered bytes can see.
    #[test]
    fn payload_bytes_render_through_the_core_codec() {
        let batch = PushBatch::new(vec![EventGroup::new(
            None,
            vec![replicated(ORIGIN, SequencePosition::FIRST)],
        )]);
        let envelope = Envelope::new(&batch);

        let json = serde_json::to_string(&envelope).unwrap();
        assert!(json.contains(r#""3q2+7w==""#), "base64 in JSON: {json}");
        assert!(!json.contains("[222,173,190,239]"), "{json}");

        let bytes = postcard::to_stdvec(&envelope).unwrap();
        assert!(
            bytes.windows(PAYLOAD.len()).any(|window| window == PAYLOAD),
            "four raw bytes in postcard: {bytes:02x?}"
        );
        assert!(!bytes.windows(8).any(|window| window == b"3q2+7w=="));
        let rewrapped = RewrappedPayload {
            data: PAYLOAD.to_vec(),
        };
        let control = serde_json::to_string(&Envelope::new(&rewrapped)).unwrap();
        let back: Envelope<RewrappedPayload> = serde_json::from_str(&control).unwrap();
        assert_eq!(back.message(), &rewrapped, "the control round-trips");
        assert!(
            !control.contains(r#""3q2+7w==""#),
            "and fails on the rendered bytes: {control}"
        );
    }

    /// SY-30 on the wire: a receiver is told the groups, with each guard on
    /// its own events, in both formats. The control flattens a guard-free
    /// batch to its events, as a smaller encoding would; it round-trips every
    /// event equal and cannot say which landed together, so the count of
    /// groups a receiver rebuilds is not the count the origin wrote.
    #[test]
    fn push_batch_preserves_group_boundaries_on_the_wire() {
        let batch = two_groups();
        let json: serde_json::Value = serde_json::to_value(Envelope::new(&batch)).unwrap();
        let groups = json["message"]["groups"].as_array().unwrap();
        assert_eq!(groups.len(), batch.groups.len());
        assert!(
            !groups[0]["guard"].is_null(),
            "the first group's guard travels"
        );
        assert!(
            groups[1]["guard"].is_null(),
            "the second group's absent guard travels as null, not as a missing key (WF-2)"
        );
        assert_eq!(groups[1]["events"].as_array().unwrap().len(), 2);

        let bytes = postcard::to_stdvec(&Envelope::new(&batch)).unwrap();
        let back: Envelope<PushBatch> = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(
            group_shape(back.message()),
            [(true, 1), (false, 2)],
            "postcard keeps each group, its guard and its events: {bytes:02x?}"
        );

        // The control has no guard to lean on: every group's guard is `None`,
        // so only the boundaries themselves can tell the batches apart.
        let unguarded = PushBatch::new(
            batch
                .groups
                .iter()
                .map(|group| EventGroup::new(None, group.events.clone()))
                .collect(),
        );
        let bytes = postcard::to_stdvec(&Envelope::new(&unguarded)).unwrap();
        let back: Envelope<PushBatch> = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(group_shape(back.message()), [(false, 1), (false, 2)]);

        let flattened = FlattenedPush {
            events: unguarded.events().cloned().collect(),
        };
        let bytes = postcard::to_stdvec(&Envelope::new(&flattened)).unwrap();
        let back: Envelope<FlattenedPush> = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(
            back.message(),
            &flattened,
            "the control round-trips its events"
        );
        let regrouped = PushBatch::new(
            back.into_message()
                .events
                .into_iter()
                .map(|event| EventGroup::new(None, vec![event]))
                .collect(),
        );
        assert_ne!(
            regrouped.groups.len(),
            unguarded.groups.len(),
            "and a receiver regrouping it publishes groups the origin never wrote"
        );
    }

    /// Each group as (guarded?, how many events), in forwarding order.
    fn group_shape(batch: &PushBatch) -> Vec<(bool, usize)> {
        batch
            .groups
            .iter()
            .map(|group| (group.guard.is_some(), group.events.len()))
            .collect()
    }

    /// WF-10 for the one hand-written impl the message set adds: a watermark
    /// decodes canonical whatever order the sender wrote, and two marks for one
    /// store keep the higher, in both formats. The control is the derive this
    /// impl replaces, which keeps the sender's order and both marks.
    #[test]
    fn a_watermark_decodes_canonical() {
        let (low, high) = if OTHER_ORIGIN < ORIGIN {
            (OTHER_ORIGIN, ORIGIN)
        } else {
            (ORIGIN, OTHER_ORIGIN)
        };
        let unsorted = [
            (high, second()),
            (low, SequencePosition::FIRST),
            (high, SequencePosition::FIRST),
        ];
        let mut expected = Watermark::new();
        expected.advance(low, SequencePosition::FIRST);
        expected.advance(high, second());

        let json = serde_json::to_string(&unsorted).unwrap();
        let decoded: Watermark = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, expected);
        assert_eq!(decoded.get(high), Some(second()));
        let derived: DerivedWatermark = serde_json::from_str(&json).unwrap();
        assert_eq!(
            derived.0.len(),
            unsorted.len(),
            "the control keeps both marks"
        );
        assert!(
            !derived.0.is_sorted_by_key(|(store, _)| *store),
            "and the sender's order, which a binary search then misses"
        );

        // A slice, not the array: postcard writes a fixed-size array without
        // the length prefix a sequence carries, and a watermark is a sequence.
        let bytes = postcard::to_stdvec(unsorted.as_slice()).unwrap();
        let decoded: Watermark = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(decoded, expected, "postcard: {bytes:02x?}");
        assert_eq!(decoded.get(high), Some(second()));
        let derived: DerivedWatermark = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(derived.0.len(), unsorted.len(), "the control, positionally");
        assert!(!derived.0.is_sorted_by_key(|(store, _)| *store));

        // The other way round: the lower mark first. A decoder that kept the
        // first mark per store passes the input above and fails this one.
        let lower_first = [
            (high, SequencePosition::FIRST),
            (low, SequencePosition::FIRST),
            (high, second()),
        ];
        let json = serde_json::to_string(&lower_first).unwrap();
        let decoded: Watermark = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, expected, "JSON, lower mark first: {json}");
        let bytes = postcard::to_stdvec(lower_first.as_slice()).unwrap();
        let decoded: Watermark = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(
            decoded, expected,
            "postcard, lower mark first: {bytes:02x?}"
        );
    }

    /// Every ordering of a small input decodes to the watermark that folding
    /// the same pairs through [`Watermark::advance`] builds, in both formats.
    ///
    /// **Wrong implementation it rejects:** a decoder that keeps the first
    /// mark it sees per store, `dedup_by_key` on the store or the fold without
    /// its comparison. Some ordering puts a store's lower mark first, and that
    /// decoder keeps it.
    #[test]
    fn a_watermark_decodes_as_advance_folds_it() {
        let third = second().next().unwrap();
        let marks = [
            (ORIGIN, SequencePosition::FIRST),
            (ORIGIN, third),
            (OTHER_ORIGIN, second()),
            (OTHER_ORIGIN, SequencePosition::FIRST),
            (ORIGIN, second()),
        ];
        let orderings = permutations(&marks);
        assert_eq!(orderings.len(), 120, "every ordering of five marks");

        for ordering in &orderings {
            let mut expected = Watermark::new();
            for (store, position) in ordering {
                expected.advance(*store, *position);
            }

            let json = serde_json::to_string(ordering).unwrap();
            let decoded: Watermark = serde_json::from_str(&json).unwrap();
            assert_eq!(decoded, expected, "JSON: {json}");

            let bytes = postcard::to_stdvec(ordering).unwrap();
            let decoded: Watermark = postcard::from_bytes(&bytes).unwrap();
            assert_eq!(decoded, expected, "postcard, from {ordering:?}");
        }
    }

    /// Every ordering of `items`, each as its own vector.
    fn permutations<T: Copy>(items: &[T]) -> Vec<Vec<T>> {
        let Some((first, rest)) = items.split_first() else {
            return vec![Vec::new()];
        };
        permutations(rest)
            .into_iter()
            .flat_map(|ordering| {
                (0..=ordering.len()).map(move |at| {
                    let mut with_first = ordering.clone();
                    with_first.insert(at, *first);
                    with_first
                })
            })
            .collect()
    }

    /// The worst order for a decoder that inserts each mark where it belongs:
    /// strictly descending, every insert at the front, with a repeated store
    /// whose lower mark arrives after its higher one. It decodes to the same
    /// canonical watermark as ascending input. It is about the order of stores,
    /// not about which of two marks wins: that is
    /// [`a_watermark_decodes_as_advance_folds_it`]'s.
    #[test]
    fn a_descending_watermark_decodes_canonical() {
        let stores: Vec<StoreId> = (1..=8_u8)
            .map(|byte| StoreId::from_bytes([byte; 16]))
            .collect();
        let mut descending: Vec<(StoreId, SequencePosition)> = stores
            .iter()
            .rev()
            .map(|store| (*store, second()))
            .collect();
        descending.push((stores[3], SequencePosition::FIRST));

        let mut expected = Watermark::new();
        for store in &stores {
            expected.advance(*store, second());
        }

        let json = serde_json::to_string(&descending).unwrap();
        let decoded: Watermark = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded, expected, "JSON: {json}");
        assert!(
            decoded
                .iter()
                .map(|(store, _)| store)
                .eq(stores.iter().copied())
        );

        let bytes = postcard::to_stdvec(&descending).unwrap();
        let decoded: Watermark = postcard::from_bytes(&bytes).unwrap();
        assert_eq!(decoded, expected, "postcard: {bytes:02x?}");
    }

    // =====================================================================
    // A mis-routed buffer (ADR-0097 §1)
    //
    // A buffer does not say which message it holds; the transport does. These
    // two tests pin what happens when the transport is wrong.
    // =====================================================================

    /// In JSON, a push handed to a pull's decoder is refused rather than read
    /// as a pull from the start of the log.
    ///
    /// **Wrong implementation it rejects:** `PullRequest` without
    /// `#[serde(deny_unknown_fields)]`. Its one field is an `Option`, which a
    /// derive defaults to `None` when the key is absent, so it ignores
    /// `groups` and decodes `PullRequest { from: None }` — fail-open, a full
    /// re-pull answered to a push. `PermissivePullRequest` is that
    /// implementation, compiled, and decodes the same push to `Ok`.
    #[test]
    fn a_json_push_is_refused_as_a_pull_request() {
        let push = serde_json::to_string(&Envelope::new(two_groups())).unwrap();
        let refused = serde_json::from_str::<Envelope<PullRequest>>(&push)
            .unwrap_err()
            .to_string();
        assert!(
            refused.contains("unknown field `groups`"),
            "a push is not a pull from the start, and the error says why: {refused}"
        );

        let empty = serde_json::to_string(&Envelope::new(PushBatch::empty())).unwrap();
        let refused = serde_json::from_str::<Envelope<PullRequest>>(&empty)
            .unwrap_err()
            .to_string();
        assert!(refused.contains("unknown field `groups`"), "{refused}");

        // The control: the same bytes through the derive without the
        // attribute decode as a pull from the start of the log.
        let permissive = serde_json::from_str::<Envelope<PermissivePullRequest>>(&push).unwrap();
        assert_eq!(
            permissive.into_message(),
            PermissivePullRequest { from: None },
            "without `deny_unknown_fields` the push is read as a full re-pull"
        );

        // The real pull still decodes, so the refusal above is the push's.
        let pull = serde_json::to_string(&Envelope::new(PullRequest::from_start())).unwrap();
        assert_eq!(
            serde_json::from_str::<Envelope<PullRequest>>(&pull)
                .unwrap()
                .into_message(),
            PullRequest::from_start()
        );
    }

    /// In postcard, an empty push and a first pull are the same two bytes, so
    /// a mis-routed buffer decodes silently as the other message. Postcard
    /// carries no field names, so it has no `deny_unknown_fields` to refuse
    /// with: the route is the only thing that tells them apart.
    ///
    /// Pinned rather than prevented, so that a reader of ADR-0097 sees the
    /// fail-open case measured, and so that a change which made the two
    /// encodings differ is noticed and the ADR's text revisited.
    #[test]
    fn a_misrouted_postcard_buffer_decodes_as_another_message() {
        let push = postcard::to_stdvec(&Envelope::new(PushBatch::empty())).unwrap();
        let pull = postcard::to_stdvec(&Envelope::new(PullRequest::from_start())).unwrap();
        assert_eq!(push, pull, "an empty push is a first pull, byte for byte");
        assert_eq!(
            push,
            [postcard::to_stdvec(&FORMAT_VERSION).unwrap(), vec![0x00]].concat(),
            "the version, then a zero: no groups, or no token"
        );
        assert_eq!(
            postcard::from_bytes::<Envelope<PullRequest>>(&push)
                .unwrap()
                .into_message(),
            PullRequest::from_start(),
            "and the push decodes as a pull from the start of the log"
        );
    }
}
