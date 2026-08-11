//! W2 — is `Option<Query>` (and `Query::All` itself) distinguishable from
//! plain absence?
//!
//! Backs ADR-0016 decision §4 and the WF-3 amendment: at HEAD,
//! `Query::All` serialises via `serializer.serialize_none()`
//! (`query.rs:375-383`) and `Query::Items` via `serialize_some`, so `Query`
//! piggybacks on `Option`'s own wire shape. Wrapped in a real `Option<Query>`
//! (or in a struct field with `#[serde(default, skip_serializing_if =
//! "Option::is_none")]`, WF-4's `Guard::after` shape), `Some(Query::All)` and
//! `None::<Query>` become the same bytes.
//!
//! **Why a local mirror.** The proposed fix is an externally tagged
//! `Query` (`"All"` / `{"Items": [...]}`), which is a different Rust-level
//! `Serialize`/`Deserialize` impl for the same public type. The moment phase
//! 5 lands it, testing `Some(happenstance_core::Query::all())` against
//! `None::<happenstance_core::Query>` would stop demonstrating the bug it
//! demonstrates today — it would demonstrate the fix instead, silently, and
//! this file would keep passing while measuring something else entirely. The
//! pre-fix ambiguity is reproduced here on `QueryMirrorPreFix`, a type this
//! crate owns and that never changes underneath the assertion.

use serde::{Deserialize, Serialize};

/// A permanent reproduction of `Query`'s pre-fix `Serialize`/`Deserialize`
/// (`crates/happenstance-core/src/query.rs:375-392` at HEAD): `All` writes
/// `serialize_none`, `Items` writes `serialize_some`.
#[derive(Debug, Clone, PartialEq)]
enum QueryMirrorPreFix {
    All,
    Items(Vec<QueryItemMirror>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct QueryItemMirror {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    types: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
}

impl Serialize for QueryMirrorPreFix {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_none(),
            Self::Items(items) => serializer.serialize_some(items),
        }
    }
}

impl<'de> Deserialize<'de> for QueryMirrorPreFix {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match Option::<Vec<QueryItemMirror>>::deserialize(deserializer)? {
            None => Ok(Self::All),
            Some(items) => Ok(Self::Items(items)),
        }
    }
}

/// The proposed fix: externally tagged. Decoupled from the crate on purpose
/// — this is a target shape, not a measurement of HEAD, so it carries no
/// coupling risk either way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum QueryWireProposed {
    All,
    Items(Vec<QueryItemMirror>),
}

#[test]
fn w2_option_query() {
    println!("=== W2(a) the pre-fix ambiguity: Some(QueryMirrorPreFix::All) vs None ===");
    let some_all = serde_json::to_string(&Some(QueryMirrorPreFix::All)).unwrap();
    let none_q = serde_json::to_string(&None::<QueryMirrorPreFix>).unwrap();
    println!("Some(QueryMirrorPreFix::All) = {some_all}");
    println!("None::<QueryMirrorPreFix>    = {none_q}");
    println!("equal?                       = {}", some_all == none_q);
    assert_eq!(
        some_all, none_q,
        "the pre-fix shape's whole defect: Some(All) and None are the same bytes"
    );

    let items = QueryMirrorPreFix::Items(vec![QueryItemMirror {
        types: vec!["Enrolled".into()],
        tags: vec!["course:c1".into()],
    }]);
    let items_json = serde_json::to_string(&items).unwrap();
    println!("QueryMirrorPreFix::Items(..) = {items_json}");
    println!(
        "Some(Items(..))              = {}",
        serde_json::to_string(&Some(items.clone())).unwrap()
    );
    println!();

    println!("=== W2(b) decoding null back: which variant does it become? ===");
    let back: Option<QueryMirrorPreFix> = serde_json::from_str("null").unwrap();
    println!("from_str::<Option<QueryMirrorPreFix>>(\"null\") = {back:?}");
    assert_eq!(back, None, "the decoder cannot tell None from an encoded Query::All either");
    let round = serde_json::from_str::<Option<QueryMirrorPreFix>>(&some_all).unwrap();
    println!("round-trip of the Some(All) bytes             = {round:?}  (decodes to None, not Some(All))");
    assert_eq!(
        round, None,
        "round-tripping Some(QueryMirrorPreFix::All) yields None: information is lost"
    );
    println!();

    println!("=== W2(c) the proposed externally tagged fix distinguishes them ===");
    let none_json = serde_json::to_string(&None::<QueryWireProposed>).unwrap();
    let some_all_json = serde_json::to_string(&Some(QueryWireProposed::All)).unwrap();
    println!("JSON  None::<QueryWireProposed>        = {none_json}");
    println!("JSON  Some(QueryWireProposed::All)     = {some_all_json}");
    println!("JSON distinguishable?                  = {}", none_json != some_all_json);
    assert_ne!(none_json, some_all_json, "externally tagged: None and Some(All) differ");

    let none_pc = postcard::to_stdvec(&None::<QueryWireProposed>).unwrap();
    let some_all_pc = postcard::to_stdvec(&Some(QueryWireProposed::All)).unwrap();
    println!(
        "postcard None::<QueryWireProposed>     = [{}]",
        hex(&none_pc)
    );
    println!(
        "postcard Some(QueryWireProposed::All)  = [{}]",
        hex(&some_all_pc)
    );
    println!("postcard distinguishable?               = {}", none_pc != some_all_pc);
    assert_ne!(none_pc, some_all_pc, "externally tagged: None and Some(All) differ in postcard too");

    let round_back: Option<QueryWireProposed> =
        serde_json::from_str(&some_all_json).unwrap();
    println!("round-trip of Some(QueryWireProposed::All) = {round_back:?}");
    assert_eq!(round_back, Some(QueryWireProposed::All), "the proposed shape round-trips correctly");

    println!();
    println!("=== W2(d) informational only: the REAL happenstance_core types at this build's HEAD ===");
    println!("(not asserted — expected to change once WF-3/WF-4's amendments land; see README.md)");
    let real_some_all = serde_json::to_string(&Some(happenstance_core::Query::all())).unwrap();
    let real_none = serde_json::to_string(&None::<happenstance_core::Query>).unwrap();
    println!("real Some(Query::all()) = {real_some_all}");
    println!("real None::<Query>      = {real_none}");
    if real_some_all == real_none {
        println!("=> still ambiguous: WF-3/WF-4 have NOT landed yet in this build.");
    } else {
        println!("=> distinguishable: the fix has landed (or the shape changed some other way).");
    }

    println!();
    println!("=== W2(e) informational only: real AppendCondition against degenerate JSON ===");
    for input in [
        "{}",
        "[]",
        r#"{"guards":[]}"#,
        r#"{"guards":[{}]}"#,
        r#"{"guards":[{"query":null}]}"#,
    ] {
        match serde_json::from_str::<happenstance_core::AppendCondition>(input) {
            Ok(v) => println!("  {input:<32} => Ok({v:?})"),
            Err(e) => println!("  {input:<32} => Err({e})"),
        }
    }
    let cond = happenstance_core::AppendCondition::new(happenstance_core::Query::all());
    println!(
        "  AppendCondition::new(Query::all()) serialises as {}",
        serde_json::to_string(&cond).unwrap()
    );
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect::<Vec<_>>().join(" ")
}
