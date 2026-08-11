//! W3 — how much does a `StoreId` cost in JSON as an array of ints, versus
//! quoted hex?
//!
//! Backs ADR-0016 decision §6 (WF-6): `StoreId::serialize` at HEAD is
//! `self.0.serialize(serializer)` where `self.0: [u8; 16]`
//! (`crates/happenstance-core/src/identity.rs:194-198`) — a JSON array of up
//! to sixteen decimal integers. `StoreId::Display` (`identity.rs:64-77`) is
//! already the proposed replacement's rendering: thirty-two lowercase hex
//! characters, no dashes.
//!
//! **Why the array-size measurement does not touch the real `StoreId`
//! type.** Phase 5 changes `StoreId`'s `Serialize` impl to emit the hex
//! string, in place — same type, same public API, different wire shape. A
//! test that measures "the array encoding" by calling
//! `serde_json::to_string(&StoreId::from_bytes(..))` measures whatever
//! `StoreId` currently does, which is exactly the one thing this file must
//! not assume stays true. `[u8; 16]` directly is what the array shape *is*,
//! independent of whether `StoreId` still produces it, so the array-size
//! assertions below serialise a bare `[u8; 16]` rather than a `StoreId`.

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

const RAW: [u8; 16] = [
    0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0,
];

#[test]
fn w3_store_id_json_size() {
    println!("=== W3(a) the array shape: [u8; 16] directly, decoupled from StoreId ===");
    let array_json = serde_json::to_string(&RAW).unwrap();
    let quoted_hex = format!("\"{}\"", hex(&RAW));
    println!("array JSON      = {array_json}");
    println!("array JSON len  = {}", array_json.len());
    println!("quoted hex      = {quoted_hex}");
    println!("quoted hex len  = {}", quoted_hex.len());
    let ratio = array_json.len() as f64 / quoted_hex.len() as f64;
    println!("ratio array:hex = {} : {} = {ratio:.4}x", array_json.len(), quoted_hex.len());
    assert_eq!(quoted_hex.len(), 34, "32 hex chars + 2 quote marks");

    // The two extremes for the array's per-element width: every byte a
    // single decimal digit (narrowest) and every byte three digits (widest).
    let small: [u8; 16] = [1; 16];
    let big: [u8; 16] = [255; 16];
    let sj = serde_json::to_string(&small).unwrap();
    let bj = serde_json::to_string(&big).unwrap();
    println!(
        "all-0x01 array JSON = {sj} ({} bytes, ratio {:.4}x against 34-byte quoted hex)",
        sj.len(),
        sj.len() as f64 / 34.0
    );
    println!(
        "all-0xff array JSON = {bj} ({} bytes, ratio {:.4}x against 34-byte quoted hex)",
        bj.len(),
        bj.len() as f64 / 34.0
    );
    assert_eq!(sj.len(), 16 + 15 + 2, "sixteen 1-digit numbers, fifteen commas, two brackets");
    assert_eq!(bj.len(), 16 * 3 + 15 + 2, "sixteen 3-digit numbers, fifteen commas, two brackets");
    println!();

    println!("=== W3(b) postcard, same [u8; 16], for contrast ===");
    let pc = postcard::to_stdvec(&RAW).unwrap();
    println!("array postcard     = [{}]", hex(&pc));
    println!("array postcard len = {}", pc.len());
    assert_eq!(pc.len(), 16, "postcard of a fixed-size [u8; 16] is the sixteen raw bytes, no framing");
    println!();

    println!("=== W3(c) informational only: the REAL happenstance_core::StoreId at this build's HEAD ===");
    println!("(not asserted — expected to change once WF-6 lands; see README.md)");
    let id = happenstance_core::StoreId::from_bytes(RAW);
    let real_json = serde_json::to_string(&id).unwrap();
    let display = id.to_string();
    println!("Display              = {display}");
    println!("real JSON            = {real_json}");
    println!("real JSON len        = {}", real_json.len());
    if real_json == array_json {
        println!("=> byte-identical to the bare-array mirror: WF-6 has NOT landed yet in this build.");
    } else if real_json == format!("\"{display}\"") {
        println!("=> quoted-hex, matching Display: WF-6 HAS landed in this build.");
    } else {
        println!("=> neither shape: something else changed. Look again.");
    }

    println!();
    println!("=== W3(d) EventId, for context (unaffected by WF-6's StoreId question alone) ===");
    let eid = happenstance_core::EventId::new(id, happenstance_core::SequencePosition::new(42).unwrap());
    let ej = serde_json::to_string(&eid).unwrap();
    println!("EventId JSON       = {ej} ({} bytes)", ej.len());
    println!("EventId postcard   = [{}] ({} bytes)", hex(&postcard::to_stdvec(&eid).unwrap()), postcard::to_stdvec(&eid).unwrap().len());
    println!("EventId Display    = {eid}");
}
