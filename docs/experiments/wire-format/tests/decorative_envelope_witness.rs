//! Decorative measurement: does either byte-layout rule proposed for WF-8
//! actually reject the implementation WF-8's MUST NOT forbids?
//!
//! WF-8 forbids decoding any part of a message before its `format_version`
//! has been checked. The plausible wrong implementation is a **derived**
//! `Deserialize` plus a version check run *after* `serde` has already
//! decoded the whole message — exactly the partial decode the MUST NOT
//! names. This file builds both the derived-plus-post-hoc implementation and
//! a hand-written one that refuses before decoding the message, and checks
//! every rule ADR-0016 considers against both.
//!
//! **Result, up front:** neither the JSON/postcard error shape nor the
//! postcard front-of-buffer byte layout (W6) tells them apart — both
//! implementations produce the identical bytes and the identical `Err`. The
//! only observation that separates them is whether the message type's own
//! `Deserialize` was ever invoked. A `Witness` type that counts its own
//! `deserialize` calls is what CLAUDE.md's decorative-rule bar asks for: a
//! named wrong implementation, actually written down, actually rejected —
//! by exactly one of the candidate rules, not by all of them.

use core::marker::PhantomData;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::Cell;

const FORMAT_VERSION: u16 = 1;

thread_local! {
    static MESSAGE_DECODES: Cell<u32> = const { Cell::new(0) };
}

#[derive(Debug, PartialEq)]
struct Witness(u32);

impl Serialize for Witness {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(s)
    }
}
impl<'de> Deserialize<'de> for Witness {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        MESSAGE_DECODES.with(|c| c.set(c.get() + 1));
        u32::deserialize(d).map(Witness)
    }
}

/// The implementation WF-8's MUST NOT forbids: a derive, plus a version
/// check run after `serde_json`/`postcard` has already decoded the whole
/// message.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct EnvelopeDerived<T> {
    format_version: u16,
    message: T,
}

impl<T: DeserializeOwned> EnvelopeDerived<T> {
    fn from_json(s: &str) -> Result<Self, String> {
        let e: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if e.format_version != FORMAT_VERSION {
            return Err(format!("UnknownFormatVersion({})", e.format_version));
        }
        Ok(e)
    }
    fn from_postcard(b: &[u8]) -> Result<Self, String> {
        let e: Self = postcard::from_bytes(b).map_err(|e| e.to_string())?;
        if e.format_version != FORMAT_VERSION {
            return Err(format!("UnknownFormatVersion({})", e.format_version));
        }
        Ok(e)
    }
}

/// The implementation ADR-0016 specifies: hand-written, refuses before
/// `T::deserialize` is ever called.
#[derive(Serialize, Debug, PartialEq)]
struct EnvelopeHand<T> {
    format_version: u16,
    message: T,
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for EnvelopeHand<T> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<T>(PhantomData<T>);
        impl<'de, T: Deserialize<'de>> serde::de::Visitor<'de> for V<T> {
            type Value = EnvelopeHand<T>;
            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("a versioned envelope")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                let v: u16 = a.next_element()?.ok_or_else(|| serde::de::Error::custom("no version"))?;
                if v != FORMAT_VERSION {
                    return Err(serde::de::Error::custom(format!("UnknownFormatVersion({v})")));
                }
                let m: T = a.next_element()?.ok_or_else(|| serde::de::Error::custom("no message"))?;
                Ok(EnvelopeHand { format_version: v, message: m })
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                let mut version: Option<u16> = None;
                let mut message: Option<T> = None;
                while let Some(k) = a.next_key::<String>()? {
                    match k.as_str() {
                        "format_version" => {
                            let v: u16 = a.next_value()?;
                            if v != FORMAT_VERSION {
                                return Err(serde::de::Error::custom(format!("UnknownFormatVersion({v})")));
                            }
                            version = Some(v);
                        }
                        "message" => {
                            if version.is_none() {
                                return Err(serde::de::Error::custom("message before format_version"));
                            }
                            message = Some(a.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = a.next_value()?;
                        }
                    }
                }
                Ok(EnvelopeHand {
                    format_version: version.ok_or_else(|| serde::de::Error::custom("no version"))?,
                    message: message.ok_or_else(|| serde::de::Error::custom("no message"))?,
                })
            }
        }
        d.deserialize_struct("Envelope", &["format_version", "message"], V(PhantomData))
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

#[test]
fn decorative_envelope_witness_count() {
    println!("=== decorative: WF-8, what do the two proposed rules actually reject? ===");

    let bad_derived = EnvelopeDerived { format_version: 999u16, message: Witness(7) };
    let bad_json = serde_json::to_string(&bad_derived).unwrap();
    let bad_post = postcard::to_stdvec(&bad_derived).unwrap();
    println!("over-version message: json     = {bad_json}");
    println!("over-version message: postcard = [{}]", hex(&bad_post));

    println!();
    println!("--- rule: wire::rejects_an_unknown_format_version ---");
    MESSAGE_DECODES.with(|c| c.set(0));
    let r = EnvelopeDerived::<Witness>::from_json(&bad_json);
    let derived_json_calls = MESSAGE_DECODES.with(Cell::get);
    println!("JSON,     DERIVED (+ post-hoc check) => {r:?}  (message decodes = {derived_json_calls})");
    MESSAGE_DECODES.with(|c| c.set(0));
    let r2 = serde_json::from_str::<EnvelopeHand<Witness>>(&bad_json).map_err(|e| e.to_string());
    let hand_json_calls = MESSAGE_DECODES.with(Cell::get);
    println!("JSON,     HAND-WRITTEN              => {r2:?}  (message decodes = {hand_json_calls})");
    assert!(r.is_err() && r2.is_err(), "both Err: the rule as an error-shape check does NOT distinguish them");

    MESSAGE_DECODES.with(|c| c.set(0));
    let r = EnvelopeDerived::<Witness>::from_postcard(&bad_post);
    let derived_post_calls = MESSAGE_DECODES.with(Cell::get);
    println!("postcard, DERIVED (+ post-hoc check) => {r:?}  (message decodes = {derived_post_calls})");
    MESSAGE_DECODES.with(|c| c.set(0));
    let r2 = postcard::from_bytes::<EnvelopeHand<Witness>>(&bad_post).map_err(|e| e.to_string());
    let hand_post_calls = MESSAGE_DECODES.with(Cell::get);
    println!("postcard, HAND-WRITTEN               => {r2:?}  (message decodes = {hand_post_calls})");
    assert!(r.is_err() && r2.is_err());

    println!();
    println!("--- rule: version_is_readable_before_the_message (byte-position spelling) ---");
    for (name, bytes) in [
        ("DERIVED, version 1", postcard::to_stdvec(&EnvelopeDerived { format_version: 1u16, message: Witness(7) }).unwrap()),
        ("HAND,    version 1", postcard::to_stdvec(&EnvelopeHand { format_version: 1u16, message: Witness(7) }).unwrap()),
        ("DERIVED, version 384", postcard::to_stdvec(&EnvelopeDerived { format_version: 384u16, message: Witness(7) }).unwrap()),
        ("HAND,    version 384", postcard::to_stdvec(&EnvelopeHand { format_version: 384u16, message: Witness(7) }).unwrap()),
    ] {
        let (v, rest) = postcard::take_from_bytes::<u16>(&bytes).unwrap();
        println!("  {name:22} bytes=[{}] take u16 => {v}, remainder [{}]", hex(&bytes), hex(rest));
    }
    let derived_v1 = postcard::to_stdvec(&EnvelopeDerived { format_version: 1u16, message: Witness(7) }).unwrap();
    let hand_v1 = postcard::to_stdvec(&EnvelopeHand { format_version: 1u16, message: Witness(7) }).unwrap();
    assert_eq!(derived_v1, hand_v1, "byte-identical: the position spelling does NOT distinguish them either");

    println!();
    println!("--- the ONLY observation that separates them: was T::deserialize called? ---");
    println!("  DERIVED,      JSON,     unknown version => T::deserialize calls = {derived_json_calls}");
    println!("  HAND-WRITTEN, JSON,     unknown version => T::deserialize calls = {hand_json_calls}");
    println!("  DERIVED,      postcard, unknown version => T::deserialize calls = {derived_post_calls}");
    println!("  HAND-WRITTEN, postcard, unknown version => T::deserialize calls = {hand_post_calls}");
    assert_eq!(derived_json_calls, 1, "the derived+post-hoc form decodes the message BEFORE checking the version");
    assert_eq!(hand_json_calls, 0, "the hand-written form never touches the message on an unknown version");
    assert_eq!(derived_post_calls, 1);
    assert_eq!(hand_post_calls, 0);
    println!();
    println!("=> the rule that actually rejects the forbidden implementation has to assert on the WITNESS COUNT,");
    println!("   not on an error shape and not on a byte position. Neither is what ADR-0016's decision §8 specifies.");
}
