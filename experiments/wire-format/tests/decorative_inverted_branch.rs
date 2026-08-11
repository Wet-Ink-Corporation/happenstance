//! Decorative measurement: does an inverted `is_human_readable` branch
//! survive `round_trips_in_json` / `round_trips_in_postcard`?
//!
//! Backs ADR-0016 decisions §6 and §7 (WF-6, WF-11): the plausible wrong
//! implementation of a format-dependent encoding branch is one where the two
//! arms are swapped — base64 when the format is *binary*, raw bytes when the
//! format is human-readable. Because both encodings are internally
//! consistent (encode-then-decode agrees with itself whichever arm ran), a
//! round-trip rule cannot see the inversion in EITHER format. Only a rule
//! that asserts the actual on-the-wire SHAPE — the JSON string literal, or
//! the absence of base64 text in postcard — can.

use happenstance_core::bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, PartialEq)]
struct PayloadInverted(Bytes);

impl Serialize for PayloadInverted {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use base64::Engine as _;
        // DELIBERATELY INVERTED: base64 in the binary arm, raw in the human arm.
        if s.is_human_readable() {
            s.serialize_bytes(&self.0)
        } else {
            s.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.0))
        }
    }
}
impl<'de> Deserialize<'de> for PayloadInverted {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use base64::Engine as _;
        if d.is_human_readable() {
            let v = Vec::<u8>::deserialize(d)?;
            Ok(PayloadInverted(Bytes::from(v)))
        } else {
            let s = String::deserialize(d)?;
            let v = base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(serde::de::Error::custom)?;
            Ok(PayloadInverted(Bytes::from(v)))
        }
    }
}

/// The correct (non-inverted) reference, for contrast: base64 when
/// human-readable, raw otherwise — WF-11's actual proposal.
#[derive(Debug, PartialEq)]
struct PayloadCorrect(Bytes);

impl Serialize for PayloadCorrect {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use base64::Engine as _;
        if s.is_human_readable() {
            s.serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.0))
        } else {
            s.serialize_bytes(&self.0)
        }
    }
}
impl<'de> Deserialize<'de> for PayloadCorrect {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use base64::Engine as _;
        if d.is_human_readable() {
            let s = String::deserialize(d)?;
            let v = base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(serde::de::Error::custom)?;
            Ok(PayloadCorrect(Bytes::from(v)))
        } else {
            let v = Vec::<u8>::deserialize(d)?;
            Ok(PayloadCorrect(Bytes::from(v)))
        }
    }
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

#[test]
fn decorative_inverted_human_readable_branch() {
    println!("=== decorative: WF-6/WF-11, an INVERTED is_human_readable branch ===");
    let payload = Bytes::from_static(&[0xde, 0xad, 0xbe, 0xef]);

    let inverted = PayloadInverted(payload.clone());
    let ij = serde_json::to_string(&inverted).unwrap();
    let ib = postcard::to_stdvec(&inverted).unwrap();
    println!("INVERTED  json     = {ij}");
    println!("INVERTED  postcard = [{}]  (ascii: {:?})", hex(&ib), String::from_utf8_lossy(&ib[1..]));

    let ij_ok = serde_json::from_str::<PayloadInverted>(&ij).map(|v| v == inverted).unwrap_or(false);
    let ib_ok = postcard::from_bytes::<PayloadInverted>(&ib).map(|v| v == inverted).unwrap_or(false);
    println!("INVERTED  json round-trip     = {ij_ok}");
    println!("INVERTED  postcard round-trip = {ib_ok}");
    assert!(ij_ok, "round_trips_in_json is BLIND to the inversion: it round-trips fine");
    assert!(ib_ok, "round_trips_in_postcard is equally blind");

    println!();
    let correct = PayloadCorrect(payload.clone());
    let cj = serde_json::to_string(&correct).unwrap();
    let cb = postcard::to_stdvec(&correct).unwrap();
    println!("CORRECT   json     = {cj}");
    println!("CORRECT   postcard = [{}]", hex(&cb));

    println!();
    println!("=== the discriminating assertions: shape, not round-trip ===");
    println!("payload_is_base64_in_json:  CORRECT json is the base64 string \"3q2+7w==\": {}", cj == "\"3q2+7w==\"");
    println!("                             INVERTED json is instead an array of ints: {ij}");
    assert_eq!(cj, "\"3q2+7w==\"", "the correct arm produces base64 text in JSON");
    assert_ne!(ij, "\"3q2+7w==\"", "the inverted arm does NOT — this is the only assertion that catches it");

    let inverted_has_b64_text = ib.len() > 4 && ib[1..].iter().all(|b| b.is_ascii());
    println!(
        "payload_is_raw_in_postcard: CORRECT postcard is 4 raw bytes + 1-byte length prefix = 5 bytes: {}",
        cb.len() == 5
    );
    println!("                             INVERTED postcard instead carries ASCII base64 text: {inverted_has_b64_text}");
    assert_eq!(cb.len(), 5, "correct: length-prefixed raw bytes, no base64 expansion");
    assert!(ib.len() > 5, "inverted: base64 text is longer than the 4 raw bytes it replaces");

    println!();
    println!("=> ADR-0016 must name payload_is_base64_in_json and payload_is_raw_in_postcard explicitly.");
    println!("   round_trips_in_json / round_trips_in_postcard passing proves nothing about which arm ran.");
}
