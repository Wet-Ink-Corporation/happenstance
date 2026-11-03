//! The digest, the encoding and the refusals, pinned.
//!
//! Every golden here was computed by an independent model of the encoding
//! table, outside Rust, and two models agreed before any was pinned. A value
//! that disagrees with the Rust is a question about which side is wrong; it
//! is never fixed by pasting the Rust output over it.

use std::collections::HashMap;

use happenstance_core::{
    EventType, InvalidProjectionId, ProjectionId, Query, QueryItem, StoreId, Tag, Tags,
};

use super::{
    InvalidCheckpointId, MAX_CHECKPOINT_NAME_LEN, derive, encode_item, encode_query, fnv1a64,
};

fn types(names: &[&str]) -> Query {
    Query::from_item(QueryItem::of_types(names.iter().copied()).expect("a typed item"))
}

fn tagged(names: &[&str], tags: &[&str]) -> Query {
    let tags: Tags = tags
        .iter()
        .map(|tag| Tag::new(*tag).expect("a valid tag"))
        .collect();
    Query::from_item(QueryItem::new(names.iter().copied(), tags).expect("a valid item"))
}

fn digest(query: &Query) -> String {
    format!("{:016x}", fnv1a64(&encode_query(query)))
}

fn name(value: &str) -> ProjectionId {
    ProjectionId::new(value).expect("a valid name")
}

/// The published FNV-1a 64 vectors. Rejects FNV-1 (multiply before xor),
/// the 32-bit variant and a wrong offset basis, each of which misses all
/// three.
#[test]
fn fnv1a64_matches_the_published_vectors() {
    assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(fnv1a64(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(fnv1a64(b"foobar"), 0x8594_4171_f739_67e8);
}

#[test]
fn an_item_encodes_to_its_pinned_bytes() {
    let seat = QueryItem::of_types(["SeatTaken"]).expect("a typed item");
    assert_eq!(encode_item(&seat), b"SeatTaken\x1f\x1e\x1d");

    let tag =
        QueryItem::tagged(Tags::from_iter([Tag::new("x").expect("a tag")])).expect("a tagged item");
    assert_eq!(encode_item(&tag), b"\x1ex\x1f\x1d");
}

/// Rejects plain concatenation, and one terminator for both sections.
#[test]
fn concatenation_cannot_collide() {
    let ab_c = types(&["ab", "c"]);
    let a_bc = types(&["a", "bc"]);
    assert_eq!(encode_query(&ab_c), b"ab\x1fc\x1f\x1e\x1d");
    assert_eq!(encode_query(&a_bc), b"a\x1fbc\x1f\x1e\x1d");
    assert_ne!(digest(&ab_c), digest(&a_bc));

    assert_ne!(digest(&types(&["x"])), digest(&tagged(&[], &["x"])));
}

/// Rejects an encoding that neither sorts nor deduplicates the items.
#[test]
fn item_order_and_duplicates_do_not_change_the_digest() {
    let a = QueryItem::of_types(["A"]).expect("a typed item");
    let b = QueryItem::of_types(["B"]).expect("a typed item");

    let once = Query::from_items([a.clone()]).expect("one item");
    let twice = Query::from_items([a.clone(), a.clone()]).expect("two items");
    assert_eq!(digest(&once), digest(&twice));

    let forwards = Query::from_items([a.clone(), b.clone()]).expect("two items");
    let backwards = Query::from_items([b, a]).expect("two items");
    assert_eq!(digest(&forwards), digest(&backwards));
}

/// Every canonical item over a small universe, every query of one or two of
/// them, and `Query::All`: no two distinct queries share an encoding.
///
/// Rejects a missing field terminator, which is plain concatenation: the
/// types `a`, `b` and the one type `ab` then share an encoding. Measured by
/// deleting the push of US after each type.
#[test]
fn encoding_is_injective_over_a_small_universe() {
    let type_sets = subsets(&["a", "b", "ab"]);
    let tag_sets = subsets(&["a", "b", "k:v"]);
    let mut items = Vec::new();
    for type_set in &type_sets {
        for tag_set in &tag_sets {
            if type_set.is_empty() && tag_set.is_empty() {
                continue;
            }
            let tags: Tags = tag_set
                .iter()
                .map(|tag| Tag::new(*tag).expect("a valid tag"))
                .collect();
            items.push(QueryItem::new(type_set.iter().copied(), tags).expect("a constrained item"));
        }
    }

    let mut queries = vec![Query::all()];
    for (i, first) in items.iter().enumerate() {
        queries.push(Query::from_item(first.clone()));
        for second in items.iter().skip(i.saturating_add(1)) {
            queries.push(Query::from_items([first.clone(), second.clone()]).expect("two items"));
        }
    }

    let mut seen: HashMap<Vec<u8>, Query> = HashMap::new();
    for query in queries {
        let encoded = encode_query(&query);
        if let Some(earlier) = seen.get(&encoded) {
            panic!("{earlier:?} and {query:?} share an encoding");
        }
        seen.insert(encoded, query);
    }
    assert!(seen.len() > 1000, "the universe was not enumerated");
}

/// Every subset of `values` of size zero to two.
fn subsets<'a>(values: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut out = vec![Vec::new()];
    for (i, first) in values.iter().enumerate() {
        out.push(vec![*first]);
        for second in values.iter().skip(i.saturating_add(1)) {
            out.push(vec![*first, *second]);
        }
    }
    out
}

/// The digests a deployment's checkpoint ids are made of. `[T93]` is the
/// zero-padding case: `{:x}` without `016` renders a 14-digit suffix.
#[test]
fn digest_goldens() {
    let a = QueryItem::of_types(["A"]).expect("a typed item");
    let b = QueryItem::of_types(["B"]).expect("a typed item");
    let goldens = [
        (Query::all(), "af63d14c8601d9db"),
        (types(&["SeatTaken"]), "f3ff331ff7883985"),
        (types(&["ab", "c"]), "41f8ad03bffb24b0"),
        (types(&["a", "bc"]), "15a81d66dc2d5016"),
        (types(&["x"]), "df6bf452cd1fda8d"),
        (tagged(&[], &["x"]), "c7164236dee9f2e3"),
        (types(&["A"]), "e8a7b08cf787c878"),
        (
            Query::from_items([a, b]).expect("two items"),
            "74b77bcfe9f82f4e",
        ),
        (types(&["A", "B"]), "a11fa1a735aa2c93"),
        (tagged(&["A"], &["k:v"]), "85abbde9e5c9dc6e"),
        (tagged(&["A"], &["a:b", "k:v"]), "ed52bc42ae29c6a6"),
        (types(&["PartShredded"]), "5e53ecae2d67f8d5"),
        (
            tagged(&["PartShredded"], &["course:c1"]),
            "a898b50c56100e9f",
        ),
        (types(&["T93"]), "0093fbc1e0e24aed"),
    ];
    for (query, golden) in goldens {
        assert_eq!(digest(&query), golden, "{query:?}");
    }
}

#[test]
fn the_derived_id_is_the_name_an_at_sign_and_the_digest() {
    let id = derive(&name("seats_sold"), &types(&["SeatTaken"])).expect("derivable");
    assert_eq!(id.as_str(), "seats_sold@f3ff331ff7883985");

    let padded = derive(&name("t"), &types(&["T93"])).expect("derivable");
    assert_eq!(padded.as_str(), "t@0093fbc1e0e24aed");
}

#[test]
fn a_238_byte_name_derives_a_255_byte_id() {
    assert_eq!(MAX_CHECKPOINT_NAME_LEN, 238);
    let long = "n".repeat(MAX_CHECKPOINT_NAME_LEN);
    let id = derive(&name(&long), &Query::all()).expect("derivable");
    assert_eq!(id.as_str().len(), 255);
    assert_eq!(id.as_str(), format!("{long}@af63d14c8601d9db"));
}

/// Rejects a check against 255, which lets `ProjectionId::new` report
/// `TooLong { len: 256 }` about an id the caller never wrote, and rejects
/// truncation.
#[test]
fn a_239_byte_name_is_refused_not_truncated() {
    let long = name(&"n".repeat(239));
    assert_eq!(
        derive(&long, &Query::all()),
        Err(InvalidCheckpointId::NameTooLong { len: 239 })
    );
}

/// Rejects a derivation through `from_static` or any door that skips
/// `ProjectionId::new` (ADR-0082 §D5).
#[test]
fn a_sync_watermark_name_is_refused() {
    let watermark = ProjectionId::sync_watermark(StoreId::from_bytes([0xab; 16]));
    assert_eq!(
        derive(&watermark, &Query::all()),
        Err(InvalidCheckpointId::Refused(
            InvalidProjectionId::Reserved { prefix: "sync/" }
        ))
    );
}

/// `@` inside a name is harmless: the digest is always the last 17 bytes.
#[test]
fn a_name_containing_the_separator_still_derives_uniquely() {
    let query = types(&["SeatTaken"]);
    let plain = derive(&name("a"), &query).expect("derivable");
    let at = derive(&name("a@b"), &query).expect("derivable");
    assert_ne!(plain, at);

    let (head, suffix) = at.as_str().split_at(at.as_str().len() - 17);
    assert_eq!(head, "a@b");
    assert_eq!(suffix, "@f3ff331ff7883985");
    assert!(
        suffix
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
}

/// The query's event types decide the id, and `EventType` reaches it only
/// through the item's canonical order.
#[test]
fn type_order_does_not_change_the_digest() {
    let forwards = types(&["A", "B"]);
    let backwards = Query::from_item(QueryItem::of(
        EventType::from_static("B"),
        [EventType::from_static("A")],
        Tags::empty(),
    ));
    assert_eq!(digest(&forwards), digest(&backwards));
}
