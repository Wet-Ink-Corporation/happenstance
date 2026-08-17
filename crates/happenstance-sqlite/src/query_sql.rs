//! Turning a [`Query`] into the SQL that names the positions matching it.
//!
//! One question, asked in two places: **which positions match this query?** It
//! is asked on the read path, where the cost is per replayed event, and inside
//! the append transaction, where the probe runs *while the write lock is held*
//! and every other writer waits behind it. So the translation lives in one
//! module and both callers go through **one** entry point, [`chunks`] — which
//! is the property this module doc used to assert and the code did not have:
//! `read` chunked, `evaluate` did not, and a guard wider than the pushdown limit
//! failed with SQLite's own *"too many terms in compound SELECT"*.
//!
//! # Adapter-private, deliberately
//!
//! `RUNBOOK.md:4203-4206` writes the work item as *"handle
//! `Query::index_arms()` exceeding SQLite's pushdown limits"*, and **that API
//! does not exist**: `happenstance-core`'s `query.rs` has `Query::items()`,
//! `QueryItem::types()` and `QueryItem::tags()`, and nothing else. ADR-0022 §10
//! rejects minting it, on CLAUDE.md's own rule — a port is only as
//! well-designed as the spread of what implements it, and one implementor is not
//! a spread. The decomposition therefore stays here, private, and the re-open
//! trigger is named rather than left to memory: if `postgres-and-neon-stores`
//! independently needs the same thing, *that* is two unlike storage shapes
//! agreeing, and that is the evidence to mint a public name — in its own ADR.
//!
//! # Why matching goes through `event_tag` rather than the `tags` column
//!
//! ADR-0022 §6 measured all three storages the crate's own docs named. On a
//! selective read — 516 events out of 50,050, which is the shape a consistency
//! boundary actually has — the join table cost 10.7 ms against a canonical
//! blob's 34.0 and JSON1's 49.8, against a ±7% noise floor. It pays for that
//! with a write about twice as expensive, which is the trade an adapter whose
//! callers read before every write should be making.
//!
//! # The single-tag fast path is a measured correction, not a micro-optimisation
//!
//! The general superset test is `GROUP BY position HAVING COUNT(DISTINCT tag) =
//! n`. **A `GROUP BY` is an optimisation barrier**: SQLite cannot push the
//! enclosing `position > ?` predicate — the boundary every append-condition
//! guard carries — through an aggregate, so the probe materialises *every*
//! matching position in the log and then discards the ones below the boundary.
//! With exactly one tag the aggregate asserts nothing, because *"carries at
//! least this one tag"* is membership and the `(tag, position)` key answers it
//! with a seek. Dropping it measured 1,093 µs → 556 µs against its own negative
//! control (ADR-0022 §8), and a single-tag item is the overwhelmingly common
//! shape of a consistency boundary.
//!
//! Multi-tag items take an **intersection chain seeded by the most selective
//! tag**, which keeps the boundary pushable for the same reason and is what
//! `tag_cardinality` exists to order. ADR-0022 §8 makes that ordering a
//! requirement rather than a tuning knob: a two-tag boundary over a
//! 50,000-event log costs roughly 200x a single-tag one, and SQLite cannot
//! supply per-value cardinality on its own — `ANALYZE` stores an *average*,
//! which is exactly wrong for a tag set where one value matches a third of the
//! log and another matches one percent.

use std::collections::HashMap;

use happenstance_core::{Query, QueryItem};
use rusqlite::Connection;
use rusqlite::types::Value;

/// How many positions each tag of the query matches, for the tags that need it.
///
/// Only multi-tag items need it, so the lookup is skipped entirely — no
/// statement at all — for a query whose every item names at most one tag, which
/// is the common case.
#[derive(Debug, Default)]
pub(crate) struct Selectivity(HashMap<String, i64>);

impl Selectivity {
    /// Reads the counts for every tag any multi-tag item of `query` names.
    ///
    /// One statement for the whole query rather than one per item: a 128-item
    /// query is exactly the shape that makes a per-item lookup expensive, and it
    /// is the shape VT-23 requires every store to evaluate.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the statement fails.
    pub(crate) fn read_for(connection: &Connection, query: &Query) -> rusqlite::Result<Self> {
        let mut wanted: Vec<String> = Vec::new();
        for item in query.items().unwrap_or_default() {
            let tags = distinct_tags(item);
            if tags.len() > 1 {
                for tag in tags {
                    if !wanted.contains(&tag) {
                        wanted.push(tag);
                    }
                }
            }
        }
        if wanted.is_empty() {
            return Ok(Self::default());
        }

        let mut counts = HashMap::with_capacity(wanted.len());
        let sql = format!(
            "SELECT tag, events FROM tag_cardinality WHERE tag IN ({})",
            placeholders(wanted.len())
        );
        let mut statement = connection.prepare(&sql)?;
        let mut rows = statement.query(rusqlite::params_from_iter(wanted.iter()))?;
        while let Some(row) = rows.next()? {
            counts.insert(row.get::<_, String>(0)?, row.get::<_, i64>(1)?);
        }
        Ok(Self(counts))
    }

    /// `tags`, most selective first.
    ///
    /// A tag with no row in `tag_cardinality` has matched nothing, so it is the
    /// most selective thing there is — sorting it first is correct rather than a
    /// default.
    fn most_selective_first(&self, mut tags: Vec<String>) -> Vec<String> {
        tags.sort_by_key(|tag| self.0.get(tag).copied().unwrap_or(0));
        tags
    }
}

/// How many statements one page of `query` takes at `max_arms` arms each.
///
/// Never zero: `Query::all` is one arm.
pub(crate) fn statement_count(query: &Query, max_arms: usize) -> usize {
    let arms = query.items().map_or(1, <[QueryItem]>::len).max(1);
    arms.div_ceil(max_arms.max(1))
}

/// One `SELECT position …` subquery per chunk of at most `max_arms` items.
///
/// **Chunk and merge, never refuse.** A `Query` bounds nothing by design and the
/// specification requires every store to evaluate at least 128 items, so an
/// adapter that returned an error at its own pushdown limit would be inventing a
/// refusal the contract has no way to report. The merge over these cursors is
/// the caller's; what belongs here is only the decomposition.
///
/// This is the **only** entry point, deliberately. It replaced a second,
/// unchunked spelling that the append-condition path used: two spellings of one
/// question is how the write path came to refuse at the pushdown limit while the
/// module doc above claimed the translation was shared. A single-chunk plan is
/// the narrow case of the wide one, so there is nothing the removed spelling
/// could say that this cannot.
///
/// [`Query::all`] short-circuits to the `event` table rather than going through
/// the tag index, and that is the definition rather than an optimisation: `all`
/// matches every event *including an untagged one*, and an untagged event has no
/// row in `event_tag` at all. Within a chunk, items are combined with `UNION`
/// rather than `UNION ALL`, which is what makes
/// `duplicate_items_do_not_duplicate_events` pass by construction rather than by
/// a `DISTINCT` bolted on afterwards.
///
/// The caller wraps each chunk in the bounds, the ordering and the page budget —
/// or, on the append path, in `SELECT max(position) FROM (…)` — which is what
/// makes the per-chunk statements identical in shape and therefore mergeable.
pub(crate) fn chunks(
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
) -> Vec<(String, Vec<Value>)> {
    let max_arms = max_arms.max(1);
    match query.items() {
        None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
        Some(items) => items
            .chunks(max_arms)
            .map(|chunk| {
                let mut params = Vec::new();
                let sql = arms_sql(chunk, selectivity, &mut params);
                (sql, params)
            })
            .collect(),
    }
}

/// The `UNION` of one arm per item.
///
/// An empty item list cannot be constructed — `Query::from_items` refuses it —
/// but a `SELECT` with no arms would be a syntax error rather than an empty
/// result, so the case is spelled rather than assumed.
pub(crate) fn arms_sql(
    items: &[QueryItem],
    selectivity: &Selectivity,
    params: &mut Vec<Value>,
) -> String {
    if items.is_empty() {
        return "SELECT position FROM event WHERE 0".to_owned();
    }
    items
        .iter()
        .map(|item| item_sql(item, selectivity, params))
        .collect::<Vec<_>>()
        .join(" UNION ")
}

/// SQL selecting the positions matching one item.
///
/// Types within an item are OR — `event_type IN (…)`. Tags within an item are
/// AND, with **superset** matching: an event matches when it carries *at least*
/// the item's tags.
fn item_sql(item: &QueryItem, selectivity: &Selectivity, params: &mut Vec<Value>) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();

    if tags.is_empty() {
        if types.is_empty() {
            // Unconstructible through the public builders, and cheaper to spell
            // than to reason about: an item constraining nothing matches
            // everything.
            return "SELECT position FROM event".to_owned();
        }
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        return format!(
            "SELECT position FROM event WHERE event_type IN ({})",
            placeholders(types.len())
        );
    }

    // The seed carries the type constraint, because every `event_tag` row for
    // one position carries that position's own type in its covering column —
    // which is the whole point of the covering column, and what keeps this off
    // `event` and out from under the write lock.
    let mut sql = String::from("SELECT position FROM event_tag WHERE tag = ?");
    params.push(Value::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        sql.push_str(" AND event_type IN (");
        sql.push_str(&placeholders(types.len()));
        sql.push(')');
    }
    for tag in &tags[1..] {
        params.push(Value::Text(tag.clone()));
        sql.push_str(" AND position IN (SELECT position FROM event_tag WHERE tag = ?)");
    }
    sql
}

/// The item's tags, deduplicated.
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
