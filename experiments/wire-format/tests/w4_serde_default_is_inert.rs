//! W4 — does `#[serde(default)]`, on its own, change what a decoder accepts?
//!
//! Backs ADR-0016 decision §3: whether `#[serde(default)]` (kept, on the ADR's
//! own account, at three of D1's five sites once `skip_serializing_if` is
//! deleted) does anything observable by itself, and whether the two
//! `Option`-typed sites (`Event::metadata`, `Guard::after`) are already
//! permissive on the decode side with **no** attribute at all — the
//! `deserialize_option`/`missing_field` mechanism that makes `#[serde(default)]`
//! redundant there specifically.
//!
//! No coupling risk here: every struct in this file is a local fixture built
//! to isolate one variable at a time (plain / per-field `default` /
//! whole-struct `default` / `default` + `skip_serializing_if`), not a mirror
//! of any type this crate will change. All assertions are load-bearing.

use serde::{Deserialize, Serialize};

// A: plain, no attributes.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct A {
    id: u32,
    count: Option<u32>,
}

// B: #[serde(default)] on one field.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct B {
    id: u32,
    #[serde(default)]
    count: Option<u32>,
}

// C: #[serde(default)] on the whole struct.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(default)]
struct C {
    id: u32,
    count: Option<u32>,
}

// D: #[serde(default, skip_serializing_if)] on one field — the WF-2/D1 shape.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct D {
    id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<u32>,
}

// E / F isolate the Option<T> auto-default special case with a plain,
// non-Option field: E has no attribute, F has #[serde(default)]. Neither
// field is Option-typed, so this pair targets `#[serde(default)]`'s own
// effect rather than the effect Option already has for free.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct E {
    id: u32,
    count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct F {
    id: u32,
    #[serde(default)]
    count: u32,
}

/// No serde attributes anywhere — the shape WF-2/WF-4's amendments leave
/// behind once `skip_serializing_if` is gone and `#[serde(default)]` is
/// judged redundant on the `Option`-typed fields.
#[derive(Serialize, Deserialize, Debug)]
struct NoAttributesAtAll {
    a: u8,
    opt: Option<u8>,
    seq: Vec<u8>,
    tags: std::collections::BTreeSet<String>,
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}

#[test]
fn w4_serde_default_is_inert() {
    println!("=== W4(a) postcard byte length/content, default vs non-default field value ===");
    let a_default = A { id: 1, count: None };
    let a_nondef = A { id: 1, count: Some(7) };
    let b_default = B { id: 1, count: None };
    let c_default = C { id: 1, count: None };
    let d_default = D { id: 1, count: None };
    let d_nondef = D { id: 1, count: Some(7) };

    let pa0 = postcard::to_stdvec(&a_default).unwrap();
    let pb0 = postcard::to_stdvec(&b_default).unwrap();
    let pc0 = postcard::to_stdvec(&c_default).unwrap();
    let pd0 = postcard::to_stdvec(&d_default).unwrap();
    println!("A (plain)                default(count=None) : len={} bytes={:?}", pa0.len(), pa0);
    println!("B (#[default] one field) default(count=None) : len={} bytes={:?}", pb0.len(), pb0);
    println!("C (#[default] struct)    default(count=None) : len={} bytes={:?}", pc0.len(), pc0);
    println!("D (default+skip, WF-2)   default(count=None) : len={} bytes={:?}", pd0.len(), pd0);
    assert_eq!(pa0, pb0, "A and B are byte-identical: #[serde(default)] does not change ENCODING");
    assert_eq!(pa0, pc0, "A and C are byte-identical too, whole-struct default included");
    assert_ne!(pa0, pd0, "D differs: skip_serializing_if is the one attribute that changes bytes");

    let pa1 = postcard::to_stdvec(&a_nondef).unwrap();
    let pd1 = postcard::to_stdvec(&d_nondef).unwrap();
    println!("A nondefault(count=Some(7)): len={} bytes={:?}", pa1.len(), pa1);
    println!("D nondefault(count=Some(7)): len={} bytes={:?}", pd1.len(), pd1);
    assert_eq!(pa1, pd1, "at a non-default value, D's skip has nothing to skip: identical to A");
    println!();

    println!("=== W4(b) struct+trailer, take_from_bytes: does decode consume the RIGHT byte count? ===");
    println!("(a trailer is the simplest possible 'neighbour' — one arbitrary value appended after");
    println!(" the struct's own bytes, standing in for whatever comes next in a real batch)");
    for (label, buf_len, decode_ok, expect_ok) in [
        ("A default", trailer_len(&a_default), trailer_ok::<A>(&a_default), true),
        ("B default", trailer_len(&b_default), trailer_ok::<B>(&b_default), true),
        ("C default", trailer_len(&c_default), trailer_ok::<C>(&c_default), true),
        ("D default", trailer_len(&d_default), trailer_ok::<D>(&d_default), false),
    ] {
        println!("  {label}: struct_len={buf_len}, remainder_is_trailer={decode_ok}");
        assert_eq!(
            decode_ok, expect_ok,
            "{label}: A/B/C always write BOTH fields (skip is absent), so the trailer is untouched. \
             D omits `count` on the wire when it is default, so `take_from_bytes::<D>` reads the \
             trailer's own first byte(s) as if they were `count` — this is D1's bug, and unlike W1 it \
             needs no crafted neighbour: ANY trailing bytes reproduce it."
        );
    }
    println!();

    println!("=== W4(c) does #[serde(default)] change JSON decode of a document MISSING the field? ===");
    let json_missing = r#"{"id":42}"#;
    let r_a: Result<A, _> = serde_json::from_str(json_missing);
    let r_b: Result<B, _> = serde_json::from_str(json_missing);
    let r_c: Result<C, _> = serde_json::from_str(json_missing);
    let r_d: Result<D, _> = serde_json::from_str(json_missing);
    println!("  input: {json_missing}");
    println!("  A (no attr):            {r_a:?}");
    println!("  B (#[default] field):   {r_b:?}");
    println!("  C (#[default] struct):  {r_c:?}");
    println!("  D (WF-2 shape):         {r_d:?}");
    assert!(r_a.is_ok(), "A accepts the missing field too — Option's own deserialize_option handles it");
    assert!(r_b.is_ok());
    assert!(r_c.is_ok());
    assert!(r_d.is_ok());
    assert_eq!(r_a.unwrap(), A { id: 42, count: None }, "A, with NO attribute, already defaults an absent Option field");
    println!();

    println!("=== W4(d) isolating the real effect with a NON-Option field (E: none, F: #[default]) ===");
    let e = E { id: 42, count: 7 };
    let f = F { id: 42, count: 7 };
    let r_e: Result<E, _> = serde_json::from_str(json_missing);
    let r_f: Result<F, _> = serde_json::from_str(json_missing);
    println!("  E (plain, non-Option, no attr):   {r_e:?}");
    println!("  F (#[default], non-Option):       {r_f:?}");
    assert!(r_e.is_err(), "a non-Option field with NO default attribute rejects absence");
    assert!(r_f.is_ok(), "the same field WITH #[serde(default)] accepts it: this is the attribute's real, isolated effect");
    assert_eq!(r_f.unwrap(), F { id: 42, count: 0 });
    println!("  => #[serde(default)]'s only observable effect is on a field that is NOT already Option-shaped.");
    let _ = (e, f);
    println!();

    println!("=== W4(e) an Option-typed field with NO serde attribute whatsoever ===");
    let full = NoAttributesAtAll {
        a: 1,
        opt: None,
        seq: vec![],
        tags: Default::default(),
    };
    println!("encoder output for the all-defaults value = {}", serde_json::to_string(&full).unwrap());
    for input in [
        r#"{"a":1,"opt":null,"seq":[],"tags":[]}"#,
        r#"{"a":1,"seq":[],"tags":[]}"#,
        r#"{"a":1,"opt":null,"tags":[]}"#,
        r#"{"a":1,"opt":null,"seq":[]}"#,
    ] {
        let result = serde_json::from_str::<NoAttributesAtAll>(input);
        println!("  {input:40} => {:?}", result.as_ref().map_err(std::string::ToString::to_string));
    }
    let opt_absent: Result<NoAttributesAtAll, _> = serde_json::from_str(r#"{"a":1,"seq":[],"tags":[]}"#);
    assert!(
        opt_absent.is_ok(),
        "an Option field accepts total absence with NO attribute at all: \
         serde's derive routes a missing field through `missing_field`, which \
         succeeds for any type whose Deserialize calls deserialize_option"
    );
    let seq_absent: Result<NoAttributesAtAll, _> = serde_json::from_str(r#"{"a":1,"opt":null,"tags":[]}"#);
    assert!(seq_absent.is_err(), "a NON-Option field (seq) with no attribute still rejects absence");
    println!("=> two of D1's five sites (Event::metadata, Guard::after) are Option-typed, so deleting");
    println!("   #[serde(default)] there closes nothing on the decode side that was not already open.");

    println!();
    println!("=== W4(f) postcard: does whole-struct #[serde(default)] rescue a truncated buffer? ===");
    println!("(the JSON analogue of this — missing-key tolerance — is what W4(c)/(d) measured; this");
    println!(" asks whether the SAME tolerance exists for postcard's length-implied, seq-based decode)");
    let c_full = C { id: 42, count: Some(7) };
    let full_buf = postcard::to_stdvec(&c_full).unwrap();
    let id_only_len = postcard::to_stdvec(&42u32).unwrap().len();
    let truncated = &full_buf[..id_only_len];
    println!("full encoder-produced buffer  = [{}] ({} bytes)", hex(&full_buf), full_buf.len());
    println!("truncated (count field ENTIRELY absent) = [{}] ({} bytes)", hex(truncated), truncated.len());
    let a_from_truncated = postcard::from_bytes::<A>(truncated);
    let c_from_truncated = postcard::from_bytes::<C>(truncated);
    println!(
        "  A::from_bytes(truncated) = {:?}",
        a_from_truncated.as_ref().map(|_| "ok").map_err(std::string::ToString::to_string)
    );
    println!(
        "  C::from_bytes(truncated) = {:?}",
        c_from_truncated.as_ref().map(|v| format!("{v:?}")).map_err(std::string::ToString::to_string)
    );
    assert!(a_from_truncated.is_err(), "A (no default): the encoder-impossible buffer is rejected");
    assert!(
        c_from_truncated.is_err(),
        "C (whole-struct default) is ALSO rejected: postcard has no map keys to be missing, so \
         running out of bytes is a hard parse error regardless of #[serde(default)] — the JSON-only \
         'second, undocumented format' argument for deleting the attribute does not reach postcard. \
         Both fail with the same error kind (DeserializeUnexpectedEnd / \"Hit the end of buffer\")."
    );
    assert_eq!(
        a_from_truncated.unwrap_err().to_string(),
        c_from_truncated.unwrap_err().to_string(),
        "A and C fail identically — #[serde(default)] makes no observable difference in postcard here"
    );
}

fn trailer_len<T: Serialize>(v: &T) -> usize {
    postcard::to_stdvec(v).unwrap().len()
}

fn trailer_ok<T>(v: &T) -> bool
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq,
{
    let mut buf = postcard::to_stdvec(v).unwrap();
    let trailer = postcard::to_stdvec(&0xDEAD_BEEFu32).unwrap();
    buf.extend_from_slice(&trailer);
    match postcard::take_from_bytes::<T>(&buf) {
        Ok((decoded, rest)) => rest == trailer.as_slice() && decoded == *v,
        Err(_) => false,
    }
}
