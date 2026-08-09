//! W5 — what does a payload cost, encoded four different ways?
//!
//! Backs ADR-0016 decision §7/§11 (WF-11): the plan's assumption about a raw
//! `serde_json` array-of-ints encoding, against base64 and hex as candidate
//! human-readable encodings, against `postcard`'s raw-bytes baseline.
//!
//! **The seed is in the source, not described.** A prior draft of this ADR
//! cited "348,160 bytes from a seeded xorshift64" with no seed — reproducible
//! in form only. `SEED` below is the whole of what is needed to regenerate
//! the payload; nothing else in this file is nondeterministic.
//!
//! **No coupling to `happenstance-core`'s own wire decision.** This measures
//! four *candidate encodings of a byte buffer* — `bytes::Bytes` itself,
//! whose `Serialize` impl the `bytes` crate owns and this workspace cannot
//! change (it is an external crate; there is no orphan-rule route to
//! override it), plus base64-as-string, hex-as-string, and `postcard`'s raw
//! form. Whatever `happenstance-core` ends up doing for `Event::data`
//! specifically (a wrapper type, a `#[serde(with = "...")]` field attribute)
//! is a *choice among* these four primitives, not a fifth thing this file
//! would need to track. These numbers do not move when phase 5 lands.

use base64::Engine as _;
use happenstance_core::bytes::Bytes;

/// xorshift64*, seeded by this constant. Deterministic, dependency-free.
/// `0x2545_F491_4F6C_DD1D` is the commonly-used xorshift64* multiplier
/// constant reused here as a seed for convenience — not a secret, just fixed.
const SEED: u64 = 0x2545_F491_4F6C_DD1D;

fn deterministic_payload(len: usize) -> Vec<u8> {
    let mut state: u64 = SEED;
    let mut out = Vec::with_capacity(len);
    while out.len() < len {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.extend_from_slice(&state.to_le_bytes());
    }
    out.truncate(len);
    out
}

fn hex_string(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn w5_payload_encodings() {
    const RAW: usize = 340 * 1024;
    println!("=== W5 payload encodings at {RAW} bytes ({} KiB), seed = {SEED:#018x} ===", RAW / 1024);

    let data = deterministic_payload(RAW);
    assert_eq!(data.len(), RAW);
    let bytes = Bytes::from(data.clone());

    println!("first 16 raw bytes = [{}]", hex_string(&data[..16]));
    let checksum = data.iter().fold(0u32, |a, b| a.wrapping_add(u32::from(*b)));
    println!("checksum (sum of bytes mod 2^32) = {checksum}");
    println!("(re-running this test regenerates byte-identical data and this same checksum)");

    let json_array = serde_json::to_string(&bytes).unwrap();
    let b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    let b64_json = serde_json::to_string(&b64).unwrap();
    let hex_str: String = data.iter().map(|b| format!("{b:02x}")).collect();
    let hex_json = serde_json::to_string(&hex_str).unwrap();
    let pc = postcard::to_stdvec(&bytes).unwrap();

    for (name, len) in [
        ("(a) serde_json of bytes::Bytes (array of decimal ints)", json_array.len()),
        ("(b) base64 STANDARD as a JSON string (with quotes)", b64_json.len()),
        ("(c) lowercase hex as a JSON string (with quotes)", hex_json.len()),
        ("(d) postcard of bytes::Bytes", pc.len()),
    ] {
        println!(
            "{name:<58} = {len:>9} bytes = {:>8.2} KiB = {:>6.4}x raw",
            len as f64 / 1024.0,
            len as f64 / RAW as f64
        );
    }

    assert!(json_array.len() > pc.len() * 3, "the array encoding is grossly larger than raw postcard");
    let ratio_b64 = b64_json.len() as f64 / RAW as f64;
    let ratio_hex = hex_json.len() as f64 / RAW as f64;
    let ratio_pc = pc.len() as f64 / RAW as f64;
    println!();
    println!("ratio (b) base64/raw  = {ratio_b64:.4}x   (expect close to 4/3 = 1.3333, base64's fixed overhead)");
    println!("ratio (c) hex/raw     = {ratio_hex:.4}x   (expect close to 2.0, hex's fixed overhead)");
    println!("ratio (d) postcard/raw= {ratio_pc:.4}x   (expect close to 1.0, plus a short length varint)");
    assert!((ratio_b64 - 4.0 / 3.0).abs() < 0.01, "base64 overhead is a fixed ~4/3, independent of content");
    assert!((ratio_hex - 2.0).abs() < 0.01, "hex overhead is a fixed 2x, independent of content");
    assert!((ratio_pc - 1.0).abs() < 0.01, "postcard is within 1% of raw at this size");
    let ratio_array_to_b64 = json_array.len() as f64 / b64_json.len() as f64;
    println!("ratio (a) array : (b) base64 = {ratio_array_to_b64:.3}x  (how much worse the array encoding is)");

    println!();
    println!("(a) first 60 chars: {}", &json_array[..60]);
    println!("(b) first 60 chars: {}", &b64_json[..60]);
    println!("(c) first 60 chars: {}", &hex_json[..60]);
    println!("(d) first 8 bytes:  [{}]  (varint length prefix + raw)", hex_string(&pc[..8]));

    println!();
    println!(
        "for reference against ADR prose that cites 'roughly 1.3 MB' for (a): measured {} bytes = {:.3} MB (10^6) = {:.3} MiB",
        json_array.len(),
        json_array.len() as f64 / 1_000_000.0,
        json_array.len() as f64 / 1_048_576.0
    );
}
