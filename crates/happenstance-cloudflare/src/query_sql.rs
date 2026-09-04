//! Turning a [`Query`] into the SQL that names the positions matching it.
//!
//! One question, asked in two places: **which positions match this query?** The
//! read path asks it per replayed event, and `append`'s condition probe asks it
//! immediately before the insert. So the translation lives in one module and
//! both callers reach it through one entry point — because two spellings of one
//! question is how a write path and a read path come to disagree about what
//! "matches" means, and the disagreement shows up as a conformance failure in a
//! rule that names neither.
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

use happenstance_core::{Query, QueryItem};

use crate::sql_storage::SqlValue;

/// A `SELECT position …` statement naming every position `query` matches.
///
/// The result is a *subquery body*: the caller wraps it in the bound, the
/// ordering and the budget it needs — `SELECT max(position) FROM (…)` on the
/// append path, a paged window on the read path — which is what keeps the two
/// callers' statements the same shape underneath.
///
/// Bindings are appended to `bindings` in the order the statement's `?`
/// placeholders occur.
pub(crate) fn positions_matching(query: &Query, bindings: &mut Vec<SqlValue>) -> String {
    match query.items() {
        // `Query::all` short-circuits to `event` rather than going through the
        // tag index, and that is the definition rather than an optimisation:
        // `all` matches every event *including an untagged one*, and an untagged
        // event has no row in `event_tag` at all.
        None => "SELECT position FROM event".to_owned(),
        Some(items) => arms(items, bindings),
    }
}

/// The `UNION` of one arm per item.
///
/// `UNION` and not `UNION ALL`: an event matching two items of one query is one
/// event, and de-duplicating here is what makes that true by construction rather
/// than by a `DISTINCT` bolted on by whichever caller remembered.
fn arms(items: &[QueryItem], bindings: &mut Vec<SqlValue>) -> String {
    if items.is_empty() {
        // `Query::from_items` refuses an empty list, so this is unreachable
        // through the public builders — but a `SELECT` with no arms is a syntax
        // error rather than an empty result, so the case is spelled rather than
        // assumed.
        return "SELECT position FROM event WHERE 0".to_owned();
    }
    items
        .iter()
        .map(|item| item_sql(item, bindings))
        .collect::<Vec<_>>()
        .join(" UNION ")
}

/// SQL selecting the positions matching one item.
///
/// Types within an item are OR — `event_type IN (…)`. Tags within an item are
/// AND, with **superset** matching: an event matches when it carries *at least*
/// the item's tags, which is why the extra tags become `position IN (…)`
/// intersections rather than an equality on a tag set.
fn item_sql(item: &QueryItem, bindings: &mut Vec<SqlValue>) -> String {
    let tags = distinct_tags(item);
    let types = item.types();

    if tags.is_empty() {
        if types.is_empty() {
            // Unconstructible through the public builders — `QueryItem::new`
            // refuses an item constraining nothing — and cheaper to spell than
            // to reason about.
            return "SELECT position FROM event".to_owned();
        }
        for event_type in types {
            bindings.push(SqlValue::Text(event_type.as_str().to_owned()));
        }
        return format!(
            "SELECT position FROM event WHERE event_type IN ({})",
            placeholders(types.len())
        );
    }

    // The seed carries the type constraint, because every `event_tag` row for
    // one position also carries that position's own type — which is the whole
    // point of the covering column.
    let mut sql = String::from("SELECT position FROM event_tag WHERE tag = ?");
    bindings.push(SqlValue::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            bindings.push(SqlValue::Text(event_type.as_str().to_owned()));
        }
        sql.push_str(" AND event_type IN (");
        sql.push_str(&placeholders(types.len()));
        sql.push(')');
    }
    for tag in &tags[1..] {
        bindings.push(SqlValue::Text(tag.clone()));
        sql.push_str(" AND position IN (SELECT position FROM event_tag WHERE tag = ?)");
    }
    sql
}

/// The item's tags, deduplicated.
///
/// A repeated tag would add an intersection that can never narrow anything, and
/// on a metered runtime a redundant subquery is a redundant row read.
fn distinct_tags(item: &QueryItem) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(item.tags().len());
    for tag in item.tags() {
        let value = tag.as_str().to_owned();
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

/// `?,?,?` for `n` bound parameters.
pub(crate) fn placeholders(n: usize) -> String {
    let mut out = String::with_capacity(n * 2);
    for i in 0..n {
        if i > 0 {
            out.push(',');
        }
        out.push('?');
    }
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
/// against SQLite's own documented walls — `SQLITE_MAX_COMPOUND_SELECT`'s 500
/// terms and `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766 bound parameters — which are
/// facts about the driver rather than about this adapter's chosen widths, and
/// which the sibling adapter's `tests/wide_tags.rs` has separately watched a
/// real SQLite enforce. Executing them here would need a
/// `#[wasm_bindgen_test]` case, and every wasm harness this crate runs is
/// enumerated by hand in `xtask/src/proof.rs`.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use happenstance_core::{Query, QueryItem, Tags};

    use super::*;

    /// `SQLITE_MAX_COMPOUND_SELECT`, the default this runtime's SQLite is built
    /// with: how many terms one compound `SELECT` may carry.
    const COMPOUND_SELECT_TERMS: usize = 500;

    /// `SQLITE_MAX_VARIABLE_NUMBER`, likewise: bound parameters per statement.
    const BOUND_PARAMETERS: usize = 32_766;

    /// The plan for `query`, as the shipped translation produces it.
    ///
    /// One statement today, because nothing partitions. This is the only line
    /// the change under test moves; every assertion below is stated against the
    /// plan rather than against the spelling that produced it.
    fn plan(query: &Query) -> Vec<(String, Vec<SqlValue>)> {
        let mut bindings = Vec::new();
        let sql = positions_matching(query, &mut bindings);
        vec![(sql, bindings)]
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

    /// The arm axis. `arms` joins with `" UNION "` and nothing bounds the join.
    ///
    /// A `Query` bounds nothing by design and VT-23 requires every store to
    /// evaluate at least 128 items with no ceiling above that, so a decision
    /// model wider than SQLite's compound-`SELECT` limit is one a conformant
    /// caller may build — and the sibling adapter chunks at 400 precisely
    /// because it is.
    #[test]
    fn no_statement_of_the_plan_exceeds_the_compound_select_ceiling() {
        let query = query_of(COMPOUND_SELECT_TERMS * 2, 1);
        for (sql, _) in plan(&query) {
            assert!(
                arms_in(&sql) <= COMPOUND_SELECT_TERMS,
                "one statement carries {} compound terms against SQLite's limit \
                 of {COMPOUND_SELECT_TERMS}; a wide query must be chunked and \
                 merged, never refused at the pushdown limit",
                arms_in(&sql)
            );
        }
    }

    /// The parameter axis, which is independent of the arm one.
    ///
    /// `item_sql` binds one parameter per tag and one per type, so 400 items —
    /// comfortably inside any plausible arm width — carrying this store's own
    /// declared `tags_per_event` apiece is 409,600 bound parameters. Nothing in
    /// `happenstance-core`'s `query.rs` bounds tags per query item, and the
    /// number a caller reads off this adapter's own front page is 1,024.
    #[test]
    fn no_statement_of_the_plan_exceeds_the_bound_parameter_ceiling() {
        let wide = crate::event_store::Ceilings::DECLARED.tags_per_event;
        let query = query_of(400, wide);
        for (_, bindings) in plan(&query) {
            assert!(
                bindings.len() <= BOUND_PARAMETERS,
                "one statement binds {} parameters against SQLite's limit of \
                 {BOUND_PARAMETERS}; the arm count says nothing about the \
                 parameter count, and a partition on one is not a partition on \
                 the other",
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
}
