//! W6 — where does a `format_version` land in `postcard`'s byte layout, and
//! does self-describing JSON care about field order at all?
//!
//! Backs ADR-0016 decision §8 (WF-8): whether "first field" is a property a
//! decoder can check by *position* in `postcard`, and whether it means
//! anything at all in `serde_json`.
//!
//! No coupling risk: `Envelope<T>` here is a local fixture for a message
//! shape that does not exist in `happenstance-core` yet — phase 5 is what
//! introduces it (`happenstance-sync`'s wire envelope), so there is no HEAD
//! behaviour to drift out from under this file. This is the proposal being
//! measured, not a fact about code that predates it.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Envelope<T> {
    format_version: u16,
    message: T,
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

#[test]
fn w6_envelope_varint() {
    println!("=== W6(a) postcard varint layout of format_version at several widths ===");
    for version in [1u16, 0x0180u16, 127u16, 128u16, 0xffffu16] {
        let env = Envelope { format_version: version, message: () };
        let pc = postcard::to_stdvec(&env).unwrap();
        println!(
            "format_version = {version} (0x{version:04x}) -> postcard [{}] ({} bytes)",
            hex(&pc),
            pc.len()
        );
        let (read, rest) = postcard::take_from_bytes::<u16>(&pc).unwrap();
        println!("    take_from_bytes::<u16> => version {read}, remainder {} bytes [{}]", rest.len(), hex(rest));
        assert_eq!(read, version, "the front-of-buffer u16 IS the version, at every width tried");
        assert!(rest.is_empty(), "message is (), so nothing should remain");
    }
    println!();

    println!("=== W6(b) with a non-empty message, so the remainder is visibly untouched ===");
    for version in [1u16, 0x0180u16] {
        let env = Envelope { format_version: version, message: vec![0xaau8, 0xbb, 0xcc] };
        let pc = postcard::to_stdvec(&env).unwrap();
        let (read, rest) = postcard::take_from_bytes::<u16>(&pc).unwrap();
        println!(
            "version {version} + message [0xaa,0xbb,0xcc] -> [{}] ({} bytes); take u16 => {read}, remainder [{}]",
            hex(&pc),
            pc.len(),
            hex(rest)
        );
        // The message is length-prefixed (a Vec), so the remainder is the
        // varint length byte followed by the three message bytes.
        assert_eq!(read, version);
        assert_eq!(rest, &[0x03, 0xaa, 0xbb, 0xcc], "remainder is the message's own encoding, untouched");
    }
    println!();

    println!("=== W6(c) the same envelope in JSON ===");
    for version in [1u16, 0x0180u16] {
        let env = Envelope { format_version: version, message: () };
        println!("version {version} -> {}", serde_json::to_string(&env).unwrap());
    }

    println!();
    println!("=== W6(d) does a self-describing decoder care about key ORDER at all? ===");
    let reordered: serde_json::Value = serde_json::from_str(r#"{"message":null,"format_version":1}"#).unwrap();
    let back: Envelope<()> = serde_json::from_str(r#"{"message":null,"format_version":1}"#).unwrap();
    println!("key-reordered JSON parses: {reordered} -> {back:?}");
    println!(
        "and re-serialises as {} (serde_json emits declaration order, but accepts any input order)",
        serde_json::to_string(&back).unwrap()
    );
    assert_eq!(back.format_version, 1, "the reordered document decodes fine — position carries no meaning in JSON");

    println!();
    println!("=== W6(e) a wrong version buried at the END of the JSON object ===");
    let bad: Result<Envelope<()>, _> = serde_json::from_str(r#"{"message":null,"format_version":999}"#);
    println!("wrong version at the end still decodes and is caught only by VALUE, not position: {bad:?}");
    match &bad {
        Ok(env) => assert_eq!(env.format_version, 999, "the derive-plus-post-hoc-check pattern reads this fine"),
        Err(e) => panic!("expected Ok (a derived Deserialize does not refuse structurally): {e}"),
    }
    println!("=> nothing about JSON's structure refuses this. A version check has to be a VALUE check");
    println!("   run before anything else is trusted, never a POSITION check — position is meaningless here.");
}
