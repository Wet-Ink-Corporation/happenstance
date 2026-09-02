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
