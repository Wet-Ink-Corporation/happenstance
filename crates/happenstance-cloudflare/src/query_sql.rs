//! Turning a [`Query`] into the SQL that names the positions matching it.
//!
//! One question, asked in two places: **which positions match this query?** The
//! read path asks it per replayed event, and `append`'s condition probe asks it
//! immediately before the insert. So the translation lives in one module and
//! both callers reach it through one entry point, [`chunks`] — because two
//! spellings of one question is how a write path and a read path come to
//! disagree about what "matches" means, and the disagreement shows up as a
//! conformance failure in a rule that names neither.
//!
//! # The walls are `workerd`'s, and an item binds a constant number of parameters
//!
//! A `Query` bounds nothing by design: VT-23 requires every store to evaluate at
//! least 128 items and puts no ceiling above that, and nothing in
//! `happenstance-core`'s `query.rs` bounds tags per query item at all. A Durable
//! Object's SQLite pushes back hard, because `workerd` lowers four
//! `sqlite3_limit`s on every database it opens. Measured inside it by
//! `harness/workerd/src/probe.rs` (ADR-0079):
//!
//! * **5 compound `SELECT` terms.** [`chunks`] joins one arm per item, so a
//!   statement carries at most five items, and a 128-item query is 26 statements.
//! * **100 bound parameters.** An arm binds **0 to 3**, decided by the item's
//!   *shape* and never by how many tags or types it carries: an item's tags and
//!   its types each travel as **one** JSON array, unpacked by `json_each(?)`.
//! * **100,000-byte statements** and **an expression depth of 100.** An arm's
//!   text is likewise independent of its tag and type counts, so neither grows
//!   with the query. The `AND position IN (…)` chain this replaced nested one
//!   level per tag and was refused at 46 tags in one item.
//!
//! The partition still prices **both** axes, and the price is [`Arm::parameters`]
//! — the same value [`Arm::render`] spends, so the two cannot drift. A wide query
//! becomes several statements the caller merges rather than a refusal at the
//! pushdown limit. The refusal is what VT-23 names as its wrong implementation,
//! and on the append path it would arrive as `AppendError::Store` carrying a raw
//! driver string, inside the turn, with the caller's decision already taken.
//!
//! **What this crate does not do is keep a second, unchunked spelling beside the
//! chunked one.** `happenstance-sqlite`'s own `query_sql.rs` records that as
//! exactly how its write path stayed unchunked while its module doc claimed the
//! translation was shared. A single-chunk plan is the narrow case of the wide
//! one, so there is nothing a second spelling could say that this cannot.
//!
//! # Where this diverges from the sibling, and why
//!
//! The widths are **not** the sibling's. `happenstance-sqlite` runs on SQLite's
//! compiled defaults and partitions at 400 arms and 30,000 parameters; a
//! Durable Object's walls are `workerd`'s, so the two adapters need not agree.
//! The **merge** diverges too. `happenstance-sqlite` collects every chunk's
//! page and truncates once at the end, which makes the resident row count
//! `chunks x page`; a Durable Object is a single isolate with a real memory
//! ceiling — `tests/wf11_memory_ceiling.rs` walks it — so the read path here
//! sorts and truncates **after every chunk**, bounding residency at one page
//! plus one chunk however wide the query is. That is exact rather than an
//! approximation: the smallest *n* positions of a union are still the smallest
//! *n* after any prefix of it is truncated to *n*, because adding a chunk can
//! only push a discarded row further out.
//!
//! # Adapter-private, deliberately
//!
//! `happenstance-core` exposes `Query::items()`, `QueryItem::types()` and
//! `QueryItem::tags()` and nothing resembling a SQL plan, and it should stay
//! that way until two unlike storage shapes independently need the same
//! decomposition. One implementor is not a spread (`CLAUDE.md`, *the rule that
//! matters*), and `happenstance-sqlite` reached the same conclusion for itself.
//!
//! # Why matching goes through `event_tag`
//!
//! A tag set is canonically sorted, so it *could* be matched as a delimited blob
//! on `event`. The join table is chosen instead for one reason that survives on
//! this runtime: a Durable Object bills by rows read, and `(tag, position)` as a
//! primary key turns "which positions carry this tag" into a seek over exactly
//! the matching rows rather than a scan that reads the whole log and discards
//! most of it. `event_type` rides along as a covering column so an item
//! constraining both type and tags never has to join back to `event`.

use happenstance_core::{EventType, Query, QueryItem, Tag};

use crate::namespace::Tables;
use crate::sql_storage::SqlValue;

/// One `SELECT position …` statement per chunk, bounded by **both** pushdown
/// limits: at most `max_arms` items and at most `max_parameters` bound
/// parameters.
///
/// Each statement is a *subquery body*: the caller wraps it in the bound, the
/// ordering and the budget it needs — `SELECT max(position) FROM (…)` on the
/// append path, a paged window on the read path — which is what keeps the two
/// callers' statements the same shape underneath, and what makes the per-chunk
/// results mergeable. Bindings travel with the statement they belong to, in the
/// order that statement's `?` placeholders occur.
///
/// **Chunk and merge, never refuse.** The specification requires every store to
/// evaluate at least 128 items and puts no ceiling above that, so an adapter
/// that returned an error at its own pushdown limit would be inventing a refusal
/// the contract has no way to report — and on the append path
/// `crates/happenstance-core/src/limits.rs` gives that refusal no variant to
/// travel in. The merge is the caller's; what belongs here is only the
/// decomposition. It is never empty: a `Query::all` is one chunk.
///
/// **One item is the atom of the partition and is never split.** An item's arm
/// is an intersection over its tags, and the halves of an intersection cannot be
/// recombined by the caller's `UNION` or its `max()`. Since an arm binds a
/// constant number of parameters, what can still refuse one item is the length
/// of a single JSON parameter: SQLite's `SQLITE_LIMIT_LENGTH`, about 8,500 tags
/// of the maximum 255 bytes under `workerd` 1.20260815.1 and about 32,500 on a
/// deployed object — against a store that accepts 1,024 on an event.
pub(crate) fn chunks(
    tables: &Tables,
    query: &Query,
    max_arms: usize,
    max_parameters: usize,
) -> Vec<(String, Vec<SqlValue>)> {
    match query.items() {
        // `Query::all` short-circuits to `event` rather than going through the
        // tag index, and that is the definition rather than an optimisation:
        // `all` matches every event *including an untagged one*, and an untagged
        // event has no row in `event_tag` at all.
        None => vec![(format!("SELECT position FROM {}", tables.event), Vec::new())],
        Some(items) => {
            let arms: Vec<Arm<'_>> = items.iter().map(Arm::of).collect();
            let lengths = partition(&arms, max_arms.max(1), max_parameters.max(1));
            let mut arms = arms.into_iter();
            lengths
                .into_iter()
                .map(|length| {
                    let mut bindings = Vec::new();
                    let sql = union(tables, arms.by_ref().take(length), &mut bindings);
                    (sql, bindings)
                })
                .collect()
        }
    }
}

/// The lengths of the runs `arms` is cut into, in order, each satisfying both
/// limits.
///
/// Greedy and order-preserving. A chunk is only ever merged by `UNION` or by
/// `max()`, neither of which cares which chunk an item landed in, so packing
/// tighter by reordering would buy nothing and would make the partition depend
/// on the query's shape rather than on its prefix — which is the property that
/// lets a caller compute
/// [`planned_statement_count`](crate::event_store::CloudflareEventStore::planned_statement_count)
/// for itself.
///
/// The `length > 0` guard is what stops an arm too wide for `max_parameters` on
/// its own from emitting an empty chunk forever; it gets a chunk to itself
/// instead, which is the honest outcome. See the note on splitting in
/// [`chunks`].
fn partition(arms: &[Arm<'_>], max_arms: usize, max_parameters: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut length = 0;
    let mut parameters: usize = 0;

    for arm in arms {
        let cost = arm.parameters();
        if length > 0 && (length == max_arms || parameters.saturating_add(cost) > max_parameters) {
            out.push(length);
            length = 0;
            parameters = 0;
        }
        length += 1;
        parameters = parameters.saturating_add(cost);
    }

    out.push(length);
    out
}

/// The `UNION` of the rendered arms.
///
/// `UNION` and not `UNION ALL`: an event matching two items of one query is one
/// event, and de-duplicating here is what makes that true by construction rather
/// than by a `DISTINCT` bolted on by whichever caller remembered.
fn union<'q>(
    tables: &Tables,
    arms: impl Iterator<Item = Arm<'q>>,
    bindings: &mut Vec<SqlValue>,
) -> String {
    let rendered: Vec<String> = arms.map(|arm| arm.render(tables, bindings)).collect();
    if rendered.is_empty() {
        // `Query::from_items` refuses an empty list, so this is unreachable
        // through the public builders — but a `SELECT` with no arms is a syntax
        // error rather than an empty result, so the case is spelled rather than
        // assumed.
        return format!("SELECT position FROM {} WHERE 0", tables.event);
    }
    rendered.join(" UNION ")
}

/// `event_type IN (…)` over one JSON array parameter.
const TYPE_IN: &str = "event_type IN (SELECT value FROM json_each(?))";

/// `tag IN (…)` over one JSON array parameter.
const TAG_IN: &str = "tag IN (SELECT value FROM json_each(?))";

/// One item's arm, decided once from its shape, so that the partition prices
/// exactly what the renderer binds.
///
/// An enum rather than the two functions it replaced — one rendering the SQL,
/// one counting the parameters the first would spend — because two readings of
/// one shape are how a count drifts from the thing it counts: add a bound
/// parameter to the renderer and the counter undercounts, and the partition goes
/// back to being wrong past a driver limit without saying so. Here the variant is
/// the only input to both [`parameters`](Self::parameters) and
/// [`render`](Self::render).
///
/// Types within an item are OR — `event_type IN (…)`. Tags within an item are
/// AND, with **superset** matching: an event matches when it carries *at least*
/// the item's tags. For two or more tags that is `GROUP BY position HAVING
/// count(*) = n`, and the count is exact because `event_tag`'s primary key is
/// `(tag, position)` — a position carrying *k* of the item's *n* tags contributes
/// exactly *k* rows. Every `event_tag` row of one position carries that
/// position's own `event_type`, so the type filter keeps all of a position's
/// rows or none of them and the count stays exact under it.
#[derive(Debug, PartialEq, Eq)]
enum Arm<'q> {
    /// No tags and no types. Unconstructible through the public builders —
    /// `QueryItem::new` refuses an item constraining nothing — and cheaper to
    /// spell than to reason about.
    Everything,
    /// No tags.
    Types {
        /// A JSON array of the item's types.
        types: String,
    },
    /// One tag, which is the commonest DCB item and the cheapest: a seek on the
    /// primary key, with no `GROUP BY`.
    Tag {
        /// The tag's text, borrowed from the item.
        tag: &'q str,
        /// A JSON array of the item's types, when it constrains any.
        types: Option<String>,
    },
    /// Two or more tags.
    Tags {
        /// A JSON array of the item's tags.
        tags: String,
        /// How many tags the array holds, and so how many rows a matching
        /// position contributes.
        count: usize,
        /// A JSON array of the item's types, when it constrains any.
        types: Option<String>,
    },
}

impl<'q> Arm<'q> {
    /// The arm for `item`. Borrows the tag text and allocates only the JSON.
    fn of(item: &'q QueryItem) -> Self {
        let types = item.types();
        let types = (!types.is_empty()).then(|| json_array(types.iter().map(EventType::as_str)));
        match (distinct_tags(item).as_slice(), types) {
            ([], None) => Self::Everything,
            ([], Some(types)) => Self::Types { types },
            ([tag], types) => Self::Tag { tag, types },
            (tags, types) => Self::Tags {
                tags: json_array(tags.iter().copied()),
                count: tags.len(),
                types,
            },
        }
    }

    /// The bound parameters [`render`](Self::render) will push: 0 to 3.
    fn parameters(&self) -> usize {
        match self {
            Self::Everything => 0,
            Self::Types { .. } | Self::Tag { types: None, .. } => 1,
            Self::Tag { types: Some(_), .. } | Self::Tags { types: None, .. } => 2,
            Self::Tags { types: Some(_), .. } => 3,
        }
    }

    /// The arm's SQL, pushing its bindings in the order its `?`s occur.
    ///
    /// By value, so each JSON array moves into its [`SqlValue`] rather than
    /// being copied there.
    fn render(self, tables: &Tables, bindings: &mut Vec<SqlValue>) -> String {
        match self {
            Self::Everything => format!("SELECT position FROM {}", tables.event),
            Self::Types { types } => {
                bindings.push(SqlValue::Text(types));
                format!("SELECT position FROM {} WHERE {TYPE_IN}", tables.event)
            }
            Self::Tag { tag, types } => {
                // `SqlValue::Text` owns its string, and the binding outlives the
                // item it was read from.
                bindings.push(SqlValue::Text(tag.to_owned()));
                let mut sql = format!("SELECT position FROM {} WHERE tag = ?", tables.event_tag);
                if let Some(types) = types {
                    bindings.push(SqlValue::Text(types));
                    sql.push_str(" AND ");
                    sql.push_str(TYPE_IN);
                }
                sql
            }
            Self::Tags { tags, count, types } => {
                bindings.push(SqlValue::Text(tags));
                let mut sql = format!("SELECT position FROM {} WHERE {TAG_IN}", tables.event_tag);
                if let Some(types) = types {
                    bindings.push(SqlValue::Text(types));
                    sql.push_str(" AND ");
                    sql.push_str(TYPE_IN);
                }
                sql.push_str(" GROUP BY position HAVING count(*) = ?");
                // `count` is a `Vec` length, at most `isize::MAX`, which always
                // fits an `i64`: the fallback is unreachable, not a hidden failure.
                bindings.push(SqlValue::Integer(i64::try_from(count).unwrap_or(i64::MAX)));
                sql
            }
        }
    }
}

/// The item's tags, deduplicated, as borrowed text.
///
/// `Tags` is already sorted and deduplicated by construction, so the `dedup`
/// is a guard rather than work: `count` is the number of rows a matching
/// position contributes, and a repeated tag would make it unreachable.
fn distinct_tags(item: &QueryItem) -> Vec<&str> {
    let mut out: Vec<&str> = item.tags().iter().map(Tag::as_str).collect();
    out.dedup();
    out
}

/// `values` as a JSON array of strings, for `json_each` to unpack.
///
/// Hand-written rather than `serde_json`, which this crate does not depend on:
/// the escaping JSON requires of a string is three rules. `"` and `\` are
/// escaped, U+0000 to U+001F become `\u00XX`, and everything else — non-ASCII,
/// U+2028 included — passes through raw, which `json_each` reads back
/// byte-exactly. `Tag::new` and `EventType::new` already refuse category Cc, so
/// the control-character branch is defensive, and tested directly.
fn json_array<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::from("[");
    for (index, value) in values.into_iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('"');
        for c in value.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                c if u32::from(c) < 0x20 => {
                    out.push_str("\\u00");
                    // Two nibbles, each below 16, so `from_digit` is always
                    // `Some`; extending by the `Option` needs no unwrap.
                    for nibble in [u32::from(c) >> 4, u32::from(c) & 0xf] {
                        out.extend(char::from_digit(nibble, 16));
                    }
                }
                c => out.push(c),
            }
        }
        out.push('"');
    }
    out.push(']');
    out
}

/// The host half, reachable by a plain `cargo test -p happenstance-cloudflare`
/// with no wasm toolchain installed — the pattern `event_store.rs`'s
/// `source_chain_tests` and `lib.rs`'s own `mod tests` already use.
///
/// **Why the pushdown walls are tested here rather than under the runner.**
/// The translation below is pure Rust: it takes a [`Query`] and returns a
/// string and a binding list, and it reaches no Durable Object, no JavaScript
/// heap and no `SqlStorage`. So the property these cases assert — *how many
/// arms and how many bound parameters does one statement of the plan carry* —
/// is fully observable on the host, and the wasm runner would add a JS boundary
/// to a question that has nothing to do with one.
///
/// What that costs, stated rather than left implicit: these cases do **not**
/// execute the statement, so they do not watch `prepare` refuse it. They assert
/// against `workerd`'s measured walls — 5 compound terms and 100 bound
/// parameters, from `harness/workerd/src/probe.rs` — which are facts about the
/// runtime rather than about this adapter's chosen widths. The shim enforces the
/// same four walls, and `tests/wide_query_ceiling.rs` watches it refuse a
/// statement one past each of them.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use happenstance_core::{Query, QueryItem, Tag, Tags};

    use super::*;
    use crate::event_store::{
        CloudflareEventStore, MAX_WRAPPER_BINDINGS, WORKERD_BOUND_PARAMETERS,
    };
    use crate::namespace::TableNamespace;

    /// `workerd`'s compound-`SELECT` limit: how many terms one compound
    /// `SELECT` may carry inside a Durable Object.
    const WORKERD_COMPOUND_SELECT_TERMS: usize = 5;

    /// The plan for `query`, as the shipped translation produces it.
    fn plan(query: &Query) -> Vec<(String, Vec<SqlValue>)> {
        chunks(
            &Tables::UNPREFIXED,
            query,
            CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
            CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        )
    }

    /// Arms across the whole plan.
    fn total_arms(plan: &[(String, Vec<SqlValue>)]) -> usize {
        plan.iter().map(|(sql, _)| arms_in(sql)).sum()
    }

    /// Arms in one statement: a `UNION` of *n* arms is *n* compound terms.
    fn arms_in(sql: &str) -> usize {
        sql.matches(" UNION ").count() + 1
    }

    /// A query of `items` items, each carrying `tags` tags unique to it.
    fn query_of(items: usize, tags: usize) -> Query {
        Query::from_items((0..items).map(|item| {
            let pairs: Vec<(String, String)> = (0..tags)
                .map(|tag| (format!("k{item}"), format!("v{tag}")))
                .collect();
            QueryItem::tagged(
                Tags::from_pairs(pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                    .expect("the fixture's tags are well formed"),
            )
            .expect("an item carrying tags is constructible")
        }))
        .expect("a non-empty item list is a query")
    }

    /// An item of exactly these types and tags.
    fn item(types: &[&str], tags: &[&str]) -> QueryItem {
        QueryItem::new(
            types.iter().copied(),
            tags.iter()
                .map(|tag| Tag::new(*tag).expect("the fixture's tags are well formed"))
                .collect(),
        )
        .expect("the fixture's item constrains something")
    }

    /// An item of `types` distinct types and `tags` distinct tags.
    fn item_of_width(types: usize, tags: usize) -> QueryItem {
        let types: Vec<String> = (0..types).map(|n| format!("T{n}")).collect();
        let tags: Vec<String> = (0..tags).map(|n| format!("k:v{n}")).collect();
        item(
            &types.iter().map(String::as_str).collect::<Vec<_>>(),
            &tags.iter().map(String::as_str).collect::<Vec<_>>(),
        )
    }

    // -- json_array -------------------------------------------------------

    #[test]
    fn an_empty_list_encodes_as_an_empty_array() {
        assert_eq!(json_array(Vec::<&str>::new()), "[]");
    }

    #[test]
    fn one_and_two_values_encode_exactly() {
        assert_eq!(json_array(["a"]), r#"["a"]"#);
        assert_eq!(json_array(["a", "b:c"]), r#"["a","b:c"]"#);
    }

    #[test]
    fn a_quote_and_a_backslash_are_escaped() {
        assert_eq!(json_array([r#"k:"a\b""#]), r#"["k:\"a\\b\""]"#);
    }

    #[test]
    fn non_ascii_passes_through_raw() {
        assert_eq!(
            json_array(["é", "😀", "\u{2028}"]),
            "[\"é\",\"😀\",\"\u{2028}\"]"
        );
    }

    /// Unreachable through `Tag::new` and `EventType::new`, which refuse
    /// category Cc, and so tested directly.
    #[test]
    fn control_characters_encode_as_unicode_escapes() {
        assert_eq!(
            json_array(["\u{0}", "\u{1f}", "\t"]),
            r#"["\u0000","\u001f","\u0009"]"#
        );
    }

    // -- Arm::of ----------------------------------------------------------

    /// The shape each item gets, and its price.
    #[test]
    fn an_items_shape_decides_its_arm() {
        let cases: [(QueryItem, Arm<'static>, usize); 5] = [
            (
                item(&["B", "A"], &[]),
                Arm::Types {
                    types: r#"["A","B"]"#.to_owned(),
                },
                1,
            ),
            (
                item(&[], &["a:1"]),
                Arm::Tag {
                    tag: "a:1",
                    types: None,
                },
                1,
            ),
            (
                item(&["A"], &["a:1"]),
                Arm::Tag {
                    tag: "a:1",
                    types: Some(r#"["A"]"#.to_owned()),
                },
                2,
            ),
            (
                item(&[], &["b:2", "a:1"]),
                Arm::Tags {
                    tags: r#"["a:1","b:2"]"#.to_owned(),
                    count: 2,
                    types: None,
                },
                2,
            ),
            (
                item(&["A", "B"], &["a:1", "b:2"]),
                Arm::Tags {
                    tags: r#"["a:1","b:2"]"#.to_owned(),
                    count: 2,
                    types: Some(r#"["A","B"]"#.to_owned()),
                },
                3,
            ),
        ];
        for (item, expected, parameters) in &cases {
            let arm = Arm::of(item);
            assert_eq!(&arm, expected, "{item:?}");
            assert_eq!(arm.parameters(), *parameters, "{item:?}");
        }
    }

    /// The declared tag ceiling is one arm of two or three parameters.
    #[test]
    fn an_item_of_the_declared_tag_ceiling_is_one_tags_arm() {
        let wide = crate::event_store::Ceilings::DECLARED.tags_per_event;
        let bare = item_of_width(0, wide);
        let arm = Arm::of(&bare);
        assert!(
            matches!(arm, Arm::Tags { count, types: None, .. } if count == wide),
            "{arm:?}"
        );
        assert_eq!(arm.parameters(), 2);

        let typed = item_of_width(82, wide);
        let arm = Arm::of(&typed);
        assert!(
            matches!(arm, Arm::Tags { count, types: Some(_), .. } if count == wide),
            "{arm:?}"
        );
        assert_eq!(arm.parameters(), 3);
    }

    /// A repeated tag cannot reach `Tags`: `Tags` deduplicates on construction,
    /// so an item built from the same tag twice holds it once and is a `Tag` arm.
    /// That is why `count` is always reachable.
    #[test]
    fn a_repeated_tag_is_one_tag() {
        let repeated = item(&[], &["a:1", "a:1"]);
        assert_eq!(
            Arm::of(&repeated),
            Arm::Tag {
                tag: "a:1",
                types: None
            }
        );
    }

    /// Invariant 1: the price is a function of the shape, never of the width.
    ///
    /// The wrong implementation is the one this replaced, which bound one
    /// parameter per tag and per type: at 46 tags it bound 46, and at 1,024 it
    /// could not fit one item in `workerd`'s 100.
    #[test]
    fn an_items_bound_parameters_do_not_grow_with_its_tags_or_types() {
        let widths = [1, 2, 45, 46, 1_024];
        for types in widths {
            assert_eq!(
                Arm::of(&item_of_width(types, 0)).parameters(),
                Arm::of(&item_of_width(2, 0)).parameters(),
                "{types} types, no tags"
            );
            assert_eq!(
                Arm::of(&item_of_width(types, 1)).parameters(),
                Arm::of(&item_of_width(2, 1)).parameters(),
                "{types} types, one tag"
            );
        }
        for tags in widths.into_iter().filter(|width| *width >= 2) {
            assert_eq!(
                Arm::of(&item_of_width(0, tags)).parameters(),
                Arm::of(&item_of_width(0, 2)).parameters(),
                "{tags} tags, no types"
            );
            for types in widths {
                assert_eq!(
                    Arm::of(&item_of_width(types, tags)).parameters(),
                    Arm::of(&item_of_width(2, 2)).parameters(),
                    "{tags} tags, {types} types"
                );
            }
        }
    }

    // -- Arm::render: text -----------------------------------------------

    /// One arm's SQL and bindings, unprefixed.
    fn rendered(item: &QueryItem) -> (String, Vec<SqlValue>) {
        let mut bindings = Vec::new();
        let sql = Arm::of(item).render(&Tables::UNPREFIXED, &mut bindings);
        (sql, bindings)
    }

    #[test]
    fn the_everything_arm_reads_the_event_table() {
        let mut bindings = Vec::new();
        assert_eq!(
            Arm::Everything.render(&Tables::UNPREFIXED, &mut bindings),
            "SELECT position FROM event"
        );
        assert!(bindings.is_empty());
    }

    #[test]
    fn the_arm_text_of_each_shape_is_exact() {
        let cases = [
            (
                item(&["A"], &[]),
                "SELECT position FROM event WHERE event_type IN (SELECT value FROM json_each(?))",
            ),
            (
                item(&[], &["a:1"]),
                "SELECT position FROM event_tag WHERE tag = ?",
            ),
            (
                item(&["A"], &["a:1"]),
                "SELECT position FROM event_tag WHERE tag = ? \
                 AND event_type IN (SELECT value FROM json_each(?))",
            ),
            (
                item(&[], &["a:1", "b:2"]),
                "SELECT position FROM event_tag WHERE tag IN (SELECT value FROM json_each(?)) \
                 GROUP BY position HAVING count(*) = ?",
            ),
            (
                item(&["A"], &["a:1", "b:2"]),
                "SELECT position FROM event_tag WHERE tag IN (SELECT value FROM json_each(?)) \
                 AND event_type IN (SELECT value FROM json_each(?)) \
                 GROUP BY position HAVING count(*) = ?",
            ),
        ];
        for (item, expected) in &cases {
            assert_eq!(rendered(item).0, *expected, "{item:?}");
        }
    }

    /// Namespacing reaches every table an arm names. Each literal table name
    /// in [`Arm::render`] is a mutant this case catches.
    #[test]
    fn a_namespaced_arm_names_the_namespaced_tables() {
        let tables = Tables::namespaced(&TableNamespace::new("ns").expect("a valid namespace"));
        let query = Query::from_items([
            item(&["A"], &[]),
            item(&["A"], &["a:1"]),
            item(&["A"], &["a:1", "b:2"]),
        ])
        .expect("a non-empty item list is a query");
        let plan = chunks(&tables, &query, 5, 90);
        assert_eq!(
            plan.iter().map(|(sql, _)| sql.as_str()).collect::<Vec<_>>(),
            [
                "SELECT position FROM ns_event WHERE event_type IN (SELECT value FROM json_each(?)) \
                 UNION SELECT position FROM ns_event_tag WHERE tag = ? \
                 AND event_type IN (SELECT value FROM json_each(?)) \
                 UNION SELECT position FROM ns_event_tag WHERE tag IN (SELECT value FROM json_each(?)) \
                 AND event_type IN (SELECT value FROM json_each(?)) \
                 GROUP BY position HAVING count(*) = ?"
            ]
        );
    }

    /// Invariant 1, on the text: constant length, and so constant expression
    /// depth, whatever the item's width.
    #[test]
    fn the_arm_text_does_not_grow_with_the_query() {
        let wide = crate::event_store::Ceilings::DECLARED.tags_per_event;
        assert_eq!(
            rendered(&item_of_width(0, 2)).0,
            rendered(&item_of_width(0, wide)).0
        );
        assert_eq!(
            rendered(&item_of_width(2, 2)).0,
            rendered(&item_of_width(82, wide)).0
        );
        assert_eq!(
            rendered(&item_of_width(2, 0)).0,
            rendered(&item_of_width(82, 0)).0
        );
    }

    // -- Arm::render: bindings --------------------------------------------

    /// Invariant 2: the renderer binds exactly what the partition priced, one
    /// per `?`, in textual order.
    #[test]
    fn render_binds_exactly_its_parameters_in_textual_order() {
        let text = |value: &str| SqlValue::Text(value.to_owned());
        let cases = [
            (item(&["B", "A"], &[]), vec![text(r#"["A","B"]"#)]),
            (item(&[], &["a:1"]), vec![text("a:1")]),
            (item(&["A"], &["a:1"]), vec![text("a:1"), text(r#"["A"]"#)]),
            (
                item(&[], &["a:1", "b:2"]),
                vec![text(r#"["a:1","b:2"]"#), SqlValue::Integer(2)],
            ),
            (
                item(&["A"], &["a:1", "b:2", "c:3"]),
                vec![
                    text(r#"["a:1","b:2","c:3"]"#),
                    text(r#"["A"]"#),
                    SqlValue::Integer(3),
                ],
            ),
        ];
        for (item, expected) in &cases {
            let parameters = Arm::of(item).parameters();
            let (sql, bindings) = rendered(item);
            assert_eq!(bindings.len(), parameters, "{item:?}");
            assert_eq!(sql.matches('?').count(), bindings.len(), "{item:?}");
            assert_eq!(&bindings, expected, "{item:?}");
        }
    }

    // -- the partition ----------------------------------------------------

    /// The arm axis. `union` joins with `" UNION "`, so a plan whose statements
    /// carried more items than `workerd` allows compound terms would be refused
    /// at `prepare`.
    ///
    /// A `Query` bounds nothing by design and VT-23 requires every store to
    /// evaluate at least 128 items with no ceiling above that, so a decision
    /// model wider than the compound-`SELECT` limit is one a conformant caller
    /// may build.
    #[test]
    fn no_statement_of_the_plan_exceeds_the_compound_select_ceiling() {
        let query = query_of(500 * 2, 1);
        for (sql, _) in plan(&query) {
            assert!(
                arms_in(&sql) <= WORKERD_COMPOUND_SELECT_TERMS,
                "one statement carries {} compound terms against workerd's limit \
                 of {WORKERD_COMPOUND_SELECT_TERMS}; a wide query must be chunked \
                 and merged, never refused at the pushdown limit",
                arms_in(&sql)
            );
        }
    }

    /// The parameter axis, which is independent of the arm one, and which
    /// the caller's wrapper spends from too.
    ///
    /// 400 items carrying this store's own declared `tags_per_event` apiece:
    /// the shape that bound 409,600 parameters under the old rendering.
    #[test]
    fn no_statement_of_the_plan_exceeds_the_bound_parameter_ceiling() {
        let wide = crate::event_store::Ceilings::DECLARED.tags_per_event;
        let query = query_of(400, wide);
        for (_, bindings) in plan(&query) {
            assert!(
                bindings.len() + MAX_WRAPPER_BINDINGS <= WORKERD_BOUND_PARAMETERS,
                "one statement binds {} parameters, plus up to \
                 {MAX_WRAPPER_BINDINGS} for the caller's wrapper, against \
                 workerd's limit of {WORKERD_BOUND_PARAMETERS}",
                bindings.len()
            );
        }
    }

    /// And the caller has to be able to find out before deploying.
    ///
    /// This is the half of the finding that is not a clause violation: VT-23
    /// says *"a store or an ingest policy MAY refuse a larger one"* and imposes
    /// no documentation obligation, unlike VT-21, VT-22 and VT-24. The defect is
    /// that two adapters published under one contract at the same version have
    /// materially different query capability and neither front page says so, so
    /// an application developed against `happenstance-sqlite` — the pairing this
    /// workspace's own local-first story recommends — finds out on deploy.
    ///
    /// A source scan, in the shape `tests/fixture_contract.rs`'s
    /// `the_three_store_limits_are_stated_here` already uses: a ceiling that is
    /// declared in code and absent from the page a reader lands on is a ceiling
    /// nobody can discover.
    #[test]
    fn the_front_page_declares_the_query_ceilings() {
        const FRONT_PAGE: &str = include_str!("lib.rs");

        for name in [
            "MAX_QUERY_ARMS_PER_STATEMENT",
            "MAX_QUERY_PARAMETERS_PER_STATEMENT",
        ] {
            assert!(
                FRONT_PAGE.contains(name),
                "`{name}` is not named on the crate's front page. The three \
                 refusal ceilings are documented there and the two query \
                 ceilings are not, so a caller sizing a decision model against \
                 this adapter has nothing to read."
            );
        }
    }

    /// The plan is a partition, not a sample: every arm survives it.
    ///
    /// The half of VT-23's named wrong implementation that is not a refusal is
    /// silent truncation — *"an adapter that sends only its first chunk"* — and
    /// it is the one a green suite cannot see, because a store that answers from
    /// the first chunk answers *something*. A partition's arms sum to the item
    /// count, in order, with nothing dropped and nothing repeated.
    #[test]
    fn the_plan_partitions_the_items_rather_than_sampling_them() {
        for (items, tags) in [(1, 1), (128, 1), (1_000, 1), (400, 1_024), (37, 900)] {
            let query = query_of(items, tags);
            let plan = plan(&query);
            assert_eq!(
                total_arms(&plan),
                items,
                "a plan over {items} items of {tags} tags carries {} arms; a partition drops nothing",
                total_arms(&plan)
            );
        }
    }

    /// `Query::all` is one chunk, straight to the `event` table.
    ///
    /// Never zero statements: a plan of none is a read that returns nothing,
    /// which is the emptiest possible way to be wrong.
    #[test]
    fn a_query_of_everything_is_one_statement() {
        let plan = plan(&Query::all());
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].0, "SELECT position FROM event");
        assert!(plan[0].1.is_empty());
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&Query::all()),
            1
        );
    }

    /// Statements in the plan of `items` at five arms and `budget` parameters.
    fn statements_at(budget: usize, items: &[QueryItem]) -> usize {
        let query = Query::from_items(items.iter().cloned()).expect("a non-empty item list");
        chunks(&Tables::UNPREFIXED, &query, 5, budget).len()
    }

    /// The boundary itself: one parameter under the budget, exactly at it, and
    /// one over — at a synthetic budget, because the shipped one is never
    /// reached by five arms of at most three.
    ///
    /// Off-by-one at a partition boundary is the defect this class of fix
    /// reintroduces. A ceiling is a promise about the statement that *is*
    /// issued: at exactly the budget the plan is one statement, and one
    /// parameter over it is two.
    #[test]
    fn the_parameter_partition_splits_one_over_the_budget_and_not_before() {
        let two = || item(&[], &["a:1", "b:2"]);
        let three = || item(&["A"], &["a:1", "b:2"]);
        assert_eq!(Arm::of(&two()).parameters(), 2);
        assert_eq!(Arm::of(&three()).parameters(), 3);

        assert_eq!(
            statements_at(6, &[two(), three()]),
            1,
            "one parameter under the budget is one statement"
        );
        assert_eq!(
            statements_at(6, &[three(), three()]),
            1,
            "exactly the budget is one statement: the budget is the largest a \
             statement may carry, not the smallest it may not"
        );
        assert_eq!(
            statements_at(6, &[two(), two(), three()]),
            2,
            "one parameter over the budget is two statements, and exactly two: a \
             partition that restarted its parameter count without restarting its \
             chunk would report more"
        );
    }

    /// An arm wider than the budget on its own gets a chunk to itself, rather
    /// than an empty chunk forever or a refusal here.
    #[test]
    fn an_arm_wider_than_the_budget_gets_a_chunk_to_itself() {
        let one = || item(&["A"], &[]);
        let three = || item(&["A"], &["a:1", "b:2"]);
        assert_eq!(statements_at(2, &[three()]), 1);
        assert_eq!(statements_at(2, &[one(), three(), one()]), 3);
    }

    /// The arm axis still binds where it is the tighter of the two.
    ///
    /// The regression this rejects is a partition that replaced one limit with
    /// the other rather than taking both: at one tag per item, eleven items is
    /// eleven parameters — nowhere near the budget — and must still be three
    /// statements.
    #[test]
    fn the_arm_partition_still_binds_on_narrow_items() {
        let arms = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
        let items = arms * 2 + 1;
        assert!(items < CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT);
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&query_of(items, 1)),
            items.div_ceil(arms)
        );
    }

    /// VT-23's floor, at the count the CHANGELOG and ADR-0079 publish.
    ///
    /// A literal, not `div_ceil` of the constant, because the number is a
    /// published value change of `planned_statement_count`: 128 one-tag items
    /// were one statement at 400 arms and are 26 at `workerd`'s five. A change
    /// to either constant moves it, and should have to say so.
    #[test]
    fn vt_23s_floor_plans_twenty_six_statements() {
        let floor = happenstance_core::MIN_SUPPORTED_QUERY_ITEMS;
        assert_eq!(floor, 128);
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&query_of(floor, 1)),
            26
        );
    }

    /// The merge every multi-statement page runs, driven directly.
    ///
    /// The conformance suite now builds plans of many statements — VT-23's
    /// 128-item rule is 26 of them — so the merge is exercised end to end. These
    /// cases still drive it directly, because they name the wrong
    /// implementations one at a time where an end-to-end rule sees only their
    /// sum.
    ///
    /// Four wrong implementations, each rejected by a named case below:
    /// concatenating without ordering; ordering forwards under `backwards`;
    /// truncating before de-duplicating, which spends the page budget on
    /// duplicates and returns a short page that reads as the end of the result
    /// set; and truncating only once at the end, which is *not* wrong in its
    /// answer but is the residency the divergence from the sibling exists to
    /// avoid — so it is pinned as an equivalence rather than as a defect.
    mod merge {
        use crate::event_store::absorb;

        /// One chunk's worth of rows, as `(position, tag)` pairs.
        fn chunk(positions: &[i64], tag: char) -> Vec<(i64, char)> {
            positions.iter().map(|p| (*p, tag)).collect()
        }

        /// The merge, fed chunk by chunk, as `drain_plan` feeds it.
        fn merge(chunks: &[Vec<(i64, char)>], backwards: bool, want: usize) -> Vec<i64> {
            let mut merged: Vec<(i64, char)> = Vec::new();
            for incoming in chunks {
                merged.extend(incoming.iter().copied());
                absorb(&mut merged, backwards, want);
            }
            merged.into_iter().map(|(position, _)| position).collect()
        }

        /// Rows from different chunks interleave by position, not by chunk.
        ///
        /// The wrong implementation is a concatenation: the caller's stream
        /// yields in position order and resumes the next page from the last
        /// position it yielded, so a page that hands back chunk 2's rows after
        /// chunk 1's would resume from the wrong place and skip the rest of
        /// chunk 1 entirely.
        #[test]
        fn the_page_is_ordered_across_chunks_not_within_them() {
            let plan = [chunk(&[1, 4, 7], 'a'), chunk(&[2, 3, 9], 'b')];
            assert_eq!(merge(&plan, false, 10), vec![1, 2, 3, 4, 7, 9]);
        }

        /// And in the caller's direction.
        #[test]
        fn a_backwards_page_is_ordered_descending() {
            let plan = [chunk(&[1, 4, 7], 'a'), chunk(&[2, 3, 9], 'b')];
            assert_eq!(merge(&plan, true, 10), vec![9, 7, 4, 3, 2, 1]);
        }

        /// An event matching items in two chunks is one row.
        ///
        /// `UNION` removes a duplicate within a statement and can say nothing
        /// about two statements. Without this the caller sees the same event
        /// twice, which is the property `duplicate_items_do_not_duplicate_events`
        /// exists to forbid — and which the suite checks only inside one
        /// statement, because it never builds two.
        #[test]
        fn an_event_matched_by_two_chunks_is_yielded_once() {
            let plan = [chunk(&[1, 5, 9], 'a'), chunk(&[5, 9, 11], 'b')];
            assert_eq!(merge(&plan, false, 10), vec![1, 5, 9, 11]);
        }

        /// De-duplication happens before truncation, so a full page is `want`
        /// **distinct** rows.
        ///
        /// The other order returns three rows for a page of four, and the read
        /// stream reads a short page as the end of the result set — so the
        /// duplicates would not merely waste the budget, they would truncate the
        /// caller's replay.
        #[test]
        fn a_page_of_want_rows_is_want_distinct_rows() {
            let plan = [chunk(&[1, 2, 3, 4], 'a'), chunk(&[2, 3, 5, 6], 'b')];
            let page = merge(&plan, false, 4);
            assert_eq!(page, vec![1, 2, 3, 4]);
            assert_eq!(
                page.len(),
                4,
                "the page is full, not short by its duplicates"
            );
        }

        /// Truncating after every chunk gives the same answer as truncating once
        /// at the end — which is the claim the divergence from the sibling rests
        /// on, and the one that would be quietly wrong if it were false.
        ///
        /// Checked over every arrangement of a small universe rather than on one
        /// example: three chunks drawn from twelve positions, forwards and
        /// backwards, at every page size from one to six.
        #[test]
        fn truncating_after_every_chunk_agrees_with_truncating_once() {
            let universe: Vec<i64> = (1..=12).collect();
            for seed in 0..64u32 {
                let plan: Vec<Vec<(i64, char)>> = (0..3)
                    .map(|c| {
                        let positions: Vec<i64> = universe
                            .iter()
                            .copied()
                            .filter(|p| (seed.rotate_left(c * 5) >> (p % 12)) & 1 == 1)
                            .collect();
                        chunk(&positions, char::from(b'a' + u8::try_from(c).unwrap_or(0)))
                    })
                    .collect();

                for backwards in [false, true] {
                    for want in 1..=6 {
                        let mut once: Vec<(i64, char)> =
                            plan.iter().flat_map(|c| c.iter().copied()).collect();
                        absorb(&mut once, backwards, want);
                        let once: Vec<i64> = once.into_iter().map(|(p, _)| p).collect();

                        assert_eq!(
                            merge(&plan, backwards, want),
                            once,
                            "incremental truncation must equal one truncation at the end (seed {seed}, backwards {backwards}, want {want})"
                        );
                    }
                }
            }
        }
    }
}
