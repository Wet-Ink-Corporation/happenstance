//! WF-8's two rules, and nothing else: an envelope at a version this build does
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
    /// `docs/experiments/wire-format/tests/decorative_envelope_witness.rs`, and
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
}
