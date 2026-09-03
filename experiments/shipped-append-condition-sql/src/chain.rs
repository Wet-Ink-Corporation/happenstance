//! The four guard SQL shapes, one of which is the one that ships.
//!
//! # Why any of this is transcribed at all
//!
//! `crates/happenstance-sqlite/src/query_sql.rs` is a **private module**
//! (`lib.rs:89`, `mod query_sql;`), so `item_sql`, `arms_sql`, `chunks` and
//! `Selectivity` cannot be called from outside the crate. Three of the four
//! shapes measured here do not exist in the adapter at all, so they would have
//! to be written somewhere regardless; the fourth is transcribed so that all
//! four are built by one function and differ in exactly one `match`.
//!
//! A transcription is a claim, so it is checked rather than asserted:
//! `tests/emitted_sql.rs` installs a `sqlite3_trace_v2` callback on the
//! connection it hands a real [`SqliteEventStore`](happenstance_sqlite::event_store::SqliteEventStore),
//! drives a real conditional append and a real paged replay, and asserts byte
//! for byte that [`Shape::Chain`]'s output is the string the adapter emitted.
//! If the adapter's SQL changes, that test fails; it is the standing guard on
//! everything in this file.
//!
//! # The four shapes
//!
//! * [`Shape::Chain`] — what ships today. `query_sql.rs:195-233`: a seed arm
//!   `SELECT position FROM event_tag WHERE tag = ?`, most-selective tag first,
//!   then one `AND position IN (SELECT position FROM event_tag WHERE tag = ?)`
//!   per remaining tag. No aggregate anywhere.
//! * [`Shape::ChainBoundedSeed`] — the same chain with `AND position > ?`, the
//!   guard's own boundary, bound into the **seed** arm. This shape does not
//!   exist in the adapter. It is the counterfactual finding I-2 names: the
//!   boundary is currently applied only in Rust, at
//!   `event_store.rs:667-672`, after `SELECT max(position) FROM (…)` has
//!   already returned.
//! * [`Shape::ChainBoundedAllArms`] — the same chain with the boundary bound
//!   into the seed arm **and** into every chained membership subquery. Present
//!   because the seed-only shape turned out not to answer the question: the
//!   chained subquery is uncorrelated, so SQLite materialises it once in full,
//!   and a boundary the seed carries cannot reach that materialisation. This is
//!   the shape that tests whether pushdown is worth anything *at all* here.
//! * [`Shape::Grouped`] — `GROUP BY position HAVING COUNT(DISTINCT tag) = n`,
//!   which is the form ADR-0022 §8 measured and the form the crate's own module
//!   documentation quotes its 1.97x and ~200x figures from. It is
//!   `experiments/append-condition/src/tags.rs`'s `JoinTable::item_sql`
//!   transcribed, single-tag fast path included, because that is the arm the
//!   42–66 ms two-tag figure was produced by.
//!
//! # What is held fixed across the four
//!
//! One schema (migration 1, applied by the shipped adapter itself — see
//! [`crate::store`]), one `event_tag` table, one set of rows, one chunk width,
//! one `Selectivity` lookup, one `SELECT max(position) FROM (…)` wrapper. The
//! only thing that varies is the string between the parentheses.

use std::collections::HashMap;

use happenstance_core::{Query, QueryItem};
use rusqlite::Connection;
use rusqlite::types::Value;

/// Which SQL shape a guard is evaluated with.
///
/// A runtime enum rather than four types implementing a trait, because every
/// caller here dispatches on it once per statement and a trait would put the
/// choice in a type parameter that the results table then has to re-derive.
/// [`crate::probe_store::ProbeStore`] carries it as a field for exactly that reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// The intersection chain the adapter emits today.
    Chain,
    /// The intersection chain with the guard's boundary bound into the seed arm.
    ChainBoundedSeed,
    /// The intersection chain with the boundary bound into **every** arm — the
    /// seed and each chained membership subquery.
    ChainBoundedAllArms,
    /// `GROUP BY position HAVING COUNT(DISTINCT tag) = n` — ADR-0022 §8's form.
    Grouped,
}

impl Shape {
    /// Every shape, in the order a results table prints them.
    pub const ALL: [Self; 4] = [
        Self::Chain,
        Self::ChainBoundedSeed,
        Self::ChainBoundedAllArms,
        Self::Grouped,
    ];

    /// How the shape names itself in a results table.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Chain => "chain-as-shipped",
            Self::ChainBoundedSeed => "chain-bounded-seed",
            Self::ChainBoundedAllArms => "chain-bounded-all-arms",
            Self::Grouped => "grouped-adr0022",
        }
    }

    /// Whether the guard's boundary reaches SQLite at all.
    ///
    /// `false` for the two shapes that compare `highest > boundary` in Rust,
    /// which is what `event_store.rs:667-672` does today. When it is `true` the
    /// statement returns only positions already above the boundary, so *any*
    /// row is a violation and the Rust comparison becomes `> 0` — a position is
    /// a `NonZeroU64`, so that is trivially satisfied and the two spellings
    /// agree by construction.
    #[must_use]
    pub const fn boundary_in_sql(self) -> bool {
        matches!(self, Self::ChainBoundedSeed | Self::ChainBoundedAllArms)
    }
}

/// How many positions each tag of the query matches, for the tags that need it.
///
/// A transcription of `crates/happenstance-sqlite/src/query_sql.rs:69-119`,
/// including its `Vec::contains` accumulation — which finding I-5 is about, and
/// which is therefore preserved rather than improved. `tests/selectivity_cost.rs`
/// times both this and its `BTreeSet` counterfactual.
#[derive(Debug, Default)]
pub struct Selectivity(HashMap<String, i64>);

impl Selectivity {
    /// Reads the counts for every tag any multi-tag item of `query` names.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the statement fails.
    pub fn read_for(connection: &Connection, query: &Query) -> rusqlite::Result<Self> {
        let wanted = Self::wanted_tags(query);
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

    /// The `wanted` accumulation of `read_for`, lifted out so it can be timed
    /// without a database.
    ///
    /// This is the quadratic finding I-5 names: `wanted.contains` is a linear
    /// scan of a `Vec<String>` performed once per tag of every multi-tag item,
    /// so the whole thing is O(T²) in the query's total multi-tag tag count.
    /// It is transcribed exactly, `Vec::contains` and all.
    #[must_use]
    pub fn wanted_tags(query: &Query) -> Vec<String> {
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
        wanted
    }

    /// The same accumulation with the linear scan replaced by an ordered set.
    ///
    /// The counterfactual, present so the finding's remediation has a figure
    /// rather than an expectation. It preserves first-seen order, which the
    /// `Vec` version also does, so the two produce identical output.
    #[must_use]
    pub fn wanted_tags_via_set(query: &Query) -> Vec<String> {
        let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut wanted: Vec<String> = Vec::new();
        for item in query.items().unwrap_or_default() {
            let tags = distinct_tags(item);
            if tags.len() > 1 {
                for tag in tags {
                    if seen.insert(tag.clone()) {
                        wanted.push(tag);
                    }
                }
            }
        }
        wanted
    }

    /// `tags`, most selective first.
    ///
    /// `query_sql.rs:121-128`. A tag with no row in `tag_cardinality` has
    /// matched nothing, so it sorts first — correct rather than a default.
    #[must_use]
    pub fn most_selective_first(&self, mut tags: Vec<String>) -> Vec<String> {
        tags.sort_by_key(|tag| self.0.get(tag).copied().unwrap_or(0));
        tags
    }

    /// The same counts with their sign flipped, so that
    /// [`most_selective_first`](Self::most_selective_first) orders the tags
    /// **least** selective first.
    ///
    /// The counterfactual for a claim the crate's own module documentation makes
    /// in terms — *"multi-tag items must be probed most-selective-tag-first"*
    /// (`crates/happenstance-sqlite/src/event_store.rs:91-96`). Ordering cannot
    /// change a conjunction's *result*, so this is not a second shape needing its
    /// own conformance run; it is the same [`Shape::Chain`] SQL with its two tags
    /// swapped, and `tests/seed_ordering.rs` asserts both orderings return the
    /// identical answer before it reports either one's time.
    ///
    /// A tag absent from `tag_cardinality` still sorts first under
    /// `most_selective_first` (it maps to `0`, and every present count is
    /// negative here), which is the *same* placement the un-inverted order gives
    /// it. That asymmetry is deliberate: an absent tag has matched nothing, so it
    /// belongs first under either policy, and moving it would be varying two
    /// things at once.
    #[must_use]
    pub fn inverted(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|(tag, count)| (tag.clone(), -count))
                .collect(),
        )
    }

    /// The count this lookup found for `tag`, if any.
    #[must_use]
    pub fn count_of(&self, tag: &str) -> Option<i64> {
        self.0.get(tag).copied()
    }
}

/// One `SELECT position …` subquery per chunk of at most `max_arms` items.
///
/// `crates/happenstance-sqlite/src/query_sql.rs:154-171`, with `shape` and
/// `boundary` added. `boundary` is ignored by every shape except
/// [`Shape::ChainBoundedSeed`].
///
/// `Query::all` short-circuits to the `event` table rather than going through
/// the tag index, which is the definition and not an optimisation: `all` matches
/// every event *including an untagged one*, and an untagged event has no row in
/// `event_tag` at all. The bounded shape has to carry its boundary onto that arm
/// too, or a guard over `Query::all` would return a position at or below the
/// boundary and be misread as a violation.
#[must_use]
pub fn chunks(
    shape: Shape,
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
    boundary: i64,
) -> Vec<(String, Vec<Value>)> {
    let max_arms = max_arms.max(1);
    match query.items() {
        None => {
            if shape.boundary_in_sql() {
                vec![(
                    "SELECT position FROM event WHERE position > ?".to_owned(),
                    vec![Value::Integer(boundary)],
                )]
            } else {
                vec![("SELECT position FROM event".to_owned(), Vec::new())]
            }
        }
        Some(items) => items
            .chunks(max_arms)
            .map(|chunk| {
                let mut params = Vec::new();
                let sql = arms_sql(shape, chunk, selectivity, boundary, &mut params);
                (sql, params)
            })
            .collect(),
    }
}

/// The `UNION` of one arm per item — `query_sql.rs:173-192`.
#[must_use]
pub fn arms_sql(
    shape: Shape,
    items: &[QueryItem],
    selectivity: &Selectivity,
    boundary: i64,
    params: &mut Vec<Value>,
) -> String {
    if items.is_empty() {
        return "SELECT position FROM event WHERE 0".to_owned();
    }
    items
        .iter()
        .map(|item| item_sql(shape, item, selectivity, boundary, params))
        .collect::<Vec<_>>()
        .join(" UNION ")
}

/// SQL selecting the positions matching one item, in the requested shape.
///
/// The [`Shape::Chain`] branch is `query_sql.rs:195-233` verbatim.
#[must_use]
pub fn item_sql(
    shape: Shape,
    item: &QueryItem,
    selectivity: &Selectivity,
    boundary: i64,
    params: &mut Vec<Value>,
) -> String {
    match shape {
        Shape::Chain => chain_sql(item, selectivity, None, false, params),
        Shape::ChainBoundedSeed => chain_sql(item, selectivity, Some(boundary), false, params),
        Shape::ChainBoundedAllArms => {
            chain_sql(item, selectivity, Some(boundary), true, params)
        }
        Shape::Grouped => grouped_sql(item, boundary, params),
    }
}

/// The shipped intersection chain, optionally carrying the boundary in its seed.
///
/// `bound` is `None` for the shape that ships. When it is `Some`, the seed arm
/// gains `AND position > ?` — and every *other* arm shape reachable here (the
/// tagless ones) gains the equivalent, because a shape that binds the boundary
/// on some arms and not others would return positions below it and be misread.
///
/// `bound_chained` additionally pushes the boundary into each chained membership
/// subquery. That is redundant *as a filter* — the seed has already excluded
/// every position at or below the boundary, so the intersection cannot gain or
/// lose a row — and it is not redundant as a **plan**: the chained subquery is
/// uncorrelated, so SQLite materialises it once, in full, and a predicate inside
/// it is the only thing that can shrink that materialisation.
fn chain_sql(
    item: &QueryItem,
    selectivity: &Selectivity,
    bound: Option<i64>,
    bound_chained: bool,
    params: &mut Vec<Value>,
) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();

    if tags.is_empty() {
        if types.is_empty() {
            return match bound {
                None => "SELECT position FROM event".to_owned(),
                Some(boundary) => {
                    params.push(Value::Integer(boundary));
                    "SELECT position FROM event WHERE position > ?".to_owned()
                }
            };
        }
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        let mut sql = format!(
            "SELECT position FROM event WHERE event_type IN ({})",
            placeholders(types.len())
        );
        if let Some(boundary) = bound {
            params.push(Value::Integer(boundary));
            sql.push_str(" AND position > ?");
        }
        return sql;
    }

    // The seed carries the type constraint, because every `event_tag` row for
    // one position carries that position's own type in its covering column.
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
    // **The one edit under test.** `(tag, position)` is the primary key of a
    // `WITHOUT ROWID` table, so `tag = ? AND position > ?` is a seek into the
    // interior of one contiguous range rather than a scan of the whole of it.
    // It goes on the seed and only on the seed: the chained subqueries are
    // membership tests over the same table, and restricting them would restrict
    // nothing the seed has not already restricted.
    if let Some(boundary) = bound {
        params.push(Value::Integer(boundary));
        sql.push_str(" AND position > ?");
    }
    for tag in &tags[1..] {
        params.push(Value::Text(tag.clone()));
        if let (Some(boundary), true) = (bound, bound_chained) {
            params.push(Value::Integer(boundary));
            sql.push_str(
                " AND position IN (SELECT position FROM event_tag WHERE tag = ? AND position > ?)",
            );
        } else {
            sql.push_str(" AND position IN (SELECT position FROM event_tag WHERE tag = ?)");
        }
    }
    sql
}

/// ADR-0022 §8's `GROUP BY … HAVING COUNT(DISTINCT tag)` form.
///
/// `experiments/append-condition/src/tags.rs`'s `JoinTable::item_sql`,
/// transcribed including its single-tag fast path, because that is the arm
/// whose two-tag rejection cost was recorded as 42,399 µs
/// (`results/append-condition.md` §1, monotonic-guard at 50,000 events) and
/// which this experiment re-runs as its calibration.
///
/// `boundary` is accepted and ignored, which is the honest signature: `GROUP BY`
/// is the optimisation barrier ADR-0022 §8 names, so there is nowhere useful to
/// put it and putting it in the `WHERE` would change the shape being compared.
fn grouped_sql(item: &QueryItem, boundary: i64, params: &mut Vec<Value>) -> String {
    let _ = boundary;
    let tags = distinct_tags(item);
    let types = item.types();

    if tags.is_empty() {
        if types.is_empty() {
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

    for tag in &tags {
        params.push(Value::Text(tag.clone()));
    }
    let type_clause = if types.is_empty() {
        String::new()
    } else {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        format!(" AND event_type IN ({})", placeholders(types.len()))
    };

    if tags.len() == 1 {
        return format!("SELECT position FROM event_tag WHERE tag = ?{type_clause}");
    }

    format!(
        "SELECT position FROM event_tag WHERE tag IN ({}){} \
         GROUP BY position HAVING COUNT(DISTINCT tag) = {}",
        placeholders(tags.len()),
        type_clause,
        tags.len()
    )
}

/// The wrapper both the shipped `evaluate` and every timed run here put around a
/// chunk — `crates/happenstance-sqlite/src/event_store.rs:658-663`.
#[must_use]
pub fn guard_sql(matched: &str) -> String {
    format!("SELECT max(position) FROM ({matched})")
}

/// The item's tags, deduplicated — `query_sql.rs:236-245`, `Vec::contains` and
/// all. Preserved rather than improved for the reason [`Selectivity::wanted_tags`]
/// is.
#[must_use]
pub fn distinct_tags(item: &QueryItem) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(item.tags().len());
    for tag in item.tags() {
        let value = tag.as_str().to_owned();
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

/// `?,?,?` for `n` bound parameters — `query_sql.rs:248-258`.
#[must_use]
pub fn placeholders(n: usize) -> String {
    let mut out = String::with_capacity(n * 2);
    for i in 0..n {
        if i > 0 {
            out.push(',');
        }
        out.push('?');
    }
    out
}
