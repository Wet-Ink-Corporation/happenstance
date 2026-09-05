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
    /// **The shape under test.** The intersection chain with each chained
    /// membership test rewritten from an uncorrelated
    /// `position IN (SELECT position FROM event_tag WHERE tag = ?)` into a
    /// **correlated** `EXISTS (… WHERE tag = ? AND position = <seed>.position)`.
    ///
    /// The boundary stays in Rust, so this is directly comparable to
    /// [`Chain`](Self::Chain): one edit, one variable.
    ChainExists,
    /// [`ChainExists`](Self::ChainExists) with the guard's boundary additionally
    /// bound into the seed arm, so the two remediations can be seen apart and
    /// together.
    ChainExistsBoundedSeed,
}

impl Shape {
    /// Every shape, in the order a results table prints them.
    pub const ALL: [Self; 6] = [
        Self::Chain,
        Self::ChainBoundedSeed,
        Self::ChainBoundedAllArms,
        Self::Grouped,
        Self::ChainExists,
        Self::ChainExistsBoundedSeed,
    ];

    /// How the shape names itself in a results table.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Chain => "chain-as-shipped",
            Self::ChainBoundedSeed => "chain-bounded-seed",
            Self::ChainBoundedAllArms => "chain-bounded-all-arms",
            Self::Grouped => "grouped-adr0022",
            Self::ChainExists => "chain-exists",
            Self::ChainExistsBoundedSeed => "chain-exists-bounded-seed",
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
        matches!(
            self,
            Self::ChainBoundedSeed | Self::ChainBoundedAllArms | Self::ChainExistsBoundedSeed
        )
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
        Shape::ChainBoundedAllArms => chain_sql(item, selectivity, Some(boundary), true, params),
        Shape::Grouped => grouped_sql(item, boundary, params),
        Shape::ChainExists => exists_sql(item, selectivity, None, params),
        Shape::ChainExistsBoundedSeed => exists_sql(item, selectivity, Some(boundary), params),
    }
}

/// The intersection chain with its chained membership tests **correlated**.
///
/// # The one edit, and why it should matter
///
/// `chain_sql` emits, per chained tag:
///
/// ```sql
/// AND position IN (SELECT position FROM event_tag WHERE tag = ?)
/// ```
///
/// That subquery references nothing from the enclosing row, so it is
/// **uncorrelated**: SQLite is free to evaluate it once and materialise the
/// result, and
/// `experiments/shipped-append-condition-sql/results/query-plans.md:28` records
/// that it does — `LIST SUBQUERY 1` over every row carrying that tag, with the
/// seed arm demoted to the probe side. The cost is therefore proportional to
/// the *chained* tag's cardinality, and the seed's selectivity buys nothing.
///
/// This emits instead:
///
/// ```sql
/// AND EXISTS (SELECT 1 FROM event_tag AS m0 WHERE m0.tag = ? AND m0.position = seed.position)
/// ```
///
/// which references the outer row, so it cannot be hoisted out of the loop.
/// `(tag, position)` is the primary key of a `WITHOUT ROWID` table, so each
/// evaluation should be one point seek. The predicted cost is
/// `|seed tag| x log(|chained tag|)` rather than `|chained tag|` — and if that
/// holds, the seed's selectivity becomes the thing that decides, which would
/// make `event_store.rs:91-96`'s most-selective-first requirement correct on
/// the shape that ships rather than only on the aggregate that does not.
///
/// **That is a prediction, and this crate exists to find out whether it is
/// true.** It is not obviously right: SQLite may flatten the `EXISTS` into a
/// join and materialise anyway, and a correlated subquery pays a fresh b-tree
/// descent per outer row where a materialised list pays one hash probe.
///
/// # The alias is load-bearing
///
/// The correlation has to name the outer row's position, and both sides are the
/// same table. Without an alias, `position = event_tag.position` resolves to
/// the *inner* table and the predicate becomes the tautology
/// `position = position` — not a compile error, not a runtime error, and
/// silently turns a two-tag guard into a single-tag one that is both fast and
/// wrong. `tests/arms_are_equivalent.rs` is what catches that, and it is why
/// equivalence is asserted before any clock starts.
fn exists_sql(
    item: &QueryItem,
    selectivity: &Selectivity,
    bound: Option<i64>,
    params: &mut Vec<Value>,
) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();

    // The tagless shapes have no chained membership test to correlate, so they
    // are delegated rather than duplicated: a second transcription of them
    // would be a second thing to keep in step for no measurement.
    if tags.is_empty() {
        return chain_sql(item, selectivity, bound, false, params);
    }

    let mut sql =
        String::from("SELECT seed.position AS position FROM event_tag AS seed WHERE seed.tag = ?");
    params.push(Value::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        sql.push_str(" AND seed.event_type IN (");
        sql.push_str(&placeholders(types.len()));
        sql.push(')');
    }
    if let Some(boundary) = bound {
        params.push(Value::Integer(boundary));
        sql.push_str(" AND seed.position > ?");
    }
    for (index, tag) in tags[1..].iter().enumerate() {
        params.push(Value::Text(tag.clone()));
        sql.push_str(&format!(
            " AND EXISTS (SELECT 1 FROM event_tag AS m{index} WHERE m{index}.tag = ? \
             AND m{index}.position = seed.position)"
        ));
    }
    sql
}

/// The read's own window, as an arm can carry it.
///
/// `fetch_page` applies all four of these *outside* the membership test —
/// `AND position >= ? AND position <= ? ORDER BY position … LIMIT ?` — where
/// none of them can reach the arm that builds the matched set. This is the same
/// four, in a form an arm can carry.
#[derive(Clone, Copy, Debug)]
pub struct Window {
    /// `resume_from`, inclusive. The lower bound in position order whichever
    /// direction the read runs.
    pub lo: i64,
    /// The ceiling, composed with `to`. The upper bound in position order.
    pub hi: i64,
    /// `min(remaining, PAGE_SIZE)` — the page budget, not `ReadOptions::limit`.
    pub budget: i64,
    /// Whether the read runs backwards, which decides the arm's `ORDER BY` and
    /// therefore *which* `budget` positions the arm keeps.
    pub backwards: bool,
}

/// The `UNION` of one **windowed** arm per item.
///
/// # What this is testing
///
/// Every candidate in `results/read-path.md` argues about which side of the
/// join to drive from, and takes the crossover between them as given. None of
/// them asks why the read's window stops at the subquery boundary — the adapter
/// *builds* that subquery, so it could push the window in instead of applying
/// it outside.
///
/// The soundness argument is one `fetch_page` already relies on: **the merged
/// top *b* of a union is a subset of the union of the per-arm top *b***, which
/// is exactly why it already bounds each *chunk* by the page budget and merges.
/// Bounding each *arm* is the same claim one level down. An event in the page
/// is in some arm, and its rank within that arm is no worse than its rank in
/// the union, so no arm can drop a row the page needed.
///
/// If it holds, the matched set is at most `budget x arms` **whatever the
/// corpus**, and the `IN` wrapper stops being bad on a broad query — which
/// would mean the crossover the conditional rule exists to navigate does not
/// arise, rather than being easier to navigate.
///
/// # The inner `SELECT` is required, not stylistic
///
/// SQLite rejects a bare `LIMIT` on a compound arm — *"LIMIT clause should come
/// after UNION not before"* — so each arm is wrapped in a subquery that carries
/// its own `ORDER BY` and `LIMIT`.
#[must_use]
pub fn windowed_arms_sql(
    items: &[QueryItem],
    selectivity: &Selectivity,
    window: Window,
    params: &mut Vec<Value>,
) -> String {
    if items.is_empty() {
        return "SELECT position FROM event WHERE 0".to_owned();
    }
    items
        .iter()
        .map(|item| windowed_item_sql(item, selectivity, window, params))
        .collect::<Vec<_>>()
        .join(" UNION ")
}

/// One windowed arm.
///
/// The parameter order is the textual order of the `?`s and nothing else: seed
/// tag, types, `lo`, `hi`, chained tags, budget. Getting that wrong binds a tag
/// string to a position comparison, which is not an error — it is zero rows.
/// This crate has already made that mistake once, on `Wrapper::CorrelatedExists`.
fn windowed_item_sql(
    item: &QueryItem,
    selectivity: &Selectivity,
    window: Window,
    params: &mut Vec<Value>,
) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();
    let direction = if window.backwards { "DESC" } else { "ASC" };

    // An item with no tags scans `event` rather than the tag index, and takes
    // the window there. Not exercised by this crate's corpora, and spelled
    // anyway: an arm shape that carried the window on some arms and not others
    // would return the wrong page, not a slower one.
    if tags.is_empty() {
        let mut inner = String::from("SELECT position FROM event WHERE 1");
        if !types.is_empty() {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            inner.push_str(&format!(
                " AND event_type IN ({})",
                placeholders(types.len())
            ));
        }
        params.push(Value::Integer(window.lo));
        params.push(Value::Integer(window.hi));
        params.push(Value::Integer(window.budget));
        inner.push_str(&format!(
            " AND position >= ? AND position <= ? ORDER BY position {direction} LIMIT ?"
        ));
        return format!("SELECT position FROM ({inner})");
    }

    let mut inner =
        String::from("SELECT seed.position AS position FROM event_tag AS seed WHERE seed.tag = ?");
    params.push(Value::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        inner.push_str(&format!(
            " AND seed.event_type IN ({})",
            placeholders(types.len())
        ));
    }

    // The window on the **seed**, which is what turns
    // `SEARCH seed USING PRIMARY KEY (tag=?)` into
    // `(tag=? AND position>? AND position<?)` — a seek into the interior of one
    // contiguous `(tag, position)` range rather than a walk from its start.
    params.push(Value::Integer(window.lo));
    params.push(Value::Integer(window.hi));
    inner.push_str(" AND seed.position >= ? AND seed.position <= ?");

    for (index, tag) in tags[1..].iter().enumerate() {
        params.push(Value::Text(tag.clone()));
        inner.push_str(&format!(
            " AND EXISTS (SELECT 1 FROM event_tag AS m{index} WHERE m{index}.tag = ? \
             AND m{index}.position = seed.position)"
        ));
    }

    params.push(Value::Integer(window.budget));
    inner.push_str(&format!(" ORDER BY seed.position {direction} LIMIT ?"));
    format!("SELECT position FROM ({inner})")
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

/// The columns `fetch_page` selects, transcribed from
/// `crates/happenstance-sqlite/src/row.rs:39-40`.
///
/// `pub(crate)` there, so this is a copy. It is here rather than inlined because
/// the read path's cost is partly row materialisation and partly matching, and
/// the two are separated by selecting either this or `position` alone —
/// `tests/read_path.rs` measures both.
pub const PAGE_COLUMNS: &str = "position, event_type, data, metadata, tags,                                 origin_store, origin_position, recorded_at";

/// One page of a read, in the shape `fetch_page` emits.
///
/// # Why the read path needs its own wrapper
///
/// [`guard_sql`] wraps the matched set in `SELECT max(position) FROM (…)`. That
/// `max()` is an aggregate over a column the seed arm is already ordered by, so
/// SQLite may walk the seed descending and **stop at the first satisfying row** —
/// and `results/unselective-pair.md` found evidence it does: a seed 97x larger
/// cost the same. Every `chain-exists` figure in this crate's other tables may
/// therefore be a figure about an early exit rather than about the join.
///
/// The read path has no `max()`. `crates/happenstance-sqlite/src/event_store.rs:1313-1375`
/// **enumerates**:
///
/// ```sql
/// SELECT <columns> FROM event WHERE position IN (<matched>)
///   AND position >= ?   -- resume_from
///   AND position <= ?   -- the sampled ceiling
///   ORDER BY position ASC
///   LIMIT ?             -- min(remaining, PAGE_SIZE), 512
/// ```
///
/// It has an early exit of its own — the `LIMIT` — but a different one: it stops
/// after 512 *output* rows rather than at the first row satisfying a predicate.
/// Whether the correlated rewrite still pays under that shape is the question
/// `tests/read_path.rs` exists to answer, and it is the one the sibling tables
/// cannot.
///
/// The parameters this appends, in order: `resume_from`, then `ceiling`, then
/// `budget`. `to` is omitted — a full replay does not carry one, and adding it
/// would put a fourth bound on a statement whose shape is the variable under
/// test.
#[must_use]
pub fn page_sql(matched: &str, columns: &str) -> String {
    page_sql_with(Wrapper::InSubquery, matched, columns)
}

/// Which shape the read path's **outer** wrapper takes.
///
/// [`page_sql`] is the shipped one, and `results/read-path.md` shows it is the
/// read path's floor: it materialises the whole matched set as `LIST SUBQUERY 2`
/// whatever the inner chain does, and the `LIMIT` cannot help because the list is
/// built in full before a row is emitted. That is finding I-3
/// (`references/evaluation/review-pre-publication-2026-09-03.md:2652`).
///
/// These are the candidate replacements. **Neither is shipped**, and choosing
/// between them is ADR-0022's rather than this crate's, because they are good at
/// opposite ends of the selectivity axis and the adapter would have to decide,
/// per query, which to emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wrapper {
    /// `WHERE position IN (<matched>)` — what ships.
    InSubquery,
    /// `FROM event JOIN (<matched>) AS m ON m.position = event.position`.
    ///
    /// Gives SQLite the option of running `<matched>` as a co-routine and
    /// streaming it rather than materialising it into a list. Whether it takes
    /// that option is the measurement.
    Join,
    /// `WHERE EXISTS (<matched, correlated to event.position>)`.
    ///
    /// The same trick as the inner fix, applied one level out: drive the
    /// **rowid range scan** over `event` — already in position order, so it can
    /// honour `ORDER BY position ASC LIMIT ?` by stopping — and test membership
    /// per row. Nothing is materialised at all.
    ///
    /// The trade is visible from the shape alone: it walks `event` until the
    /// page is full, so it is best when the query matches a large fraction of
    /// the log and worst when it matches a handful. That is the opposite of what
    /// [`InSubquery`](Self::InSubquery) is good at, which is why the choice may
    /// have to be cardinality-conditional.
    CorrelatedExists,
}

impl Wrapper {
    /// Every wrapper, in the order a results table prints them.
    pub const ALL: [Self; 3] = [Self::InSubquery, Self::Join, Self::CorrelatedExists];

    /// How the wrapper names itself in a results table.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::InSubquery => "wrapper-in-subquery",
            Self::Join => "wrapper-join",
            Self::CorrelatedExists => "wrapper-exists",
        }
    }

    /// Whether this wrapper needs [`correlated_arms_sql`] rather than
    /// [`arms_sql`].
    #[must_use]
    pub const fn needs_correlated_body(self) -> bool {
        matches!(self, Self::CorrelatedExists)
    }
}

/// One page of a read under `wrapper`.
///
/// `matched` must be an arms SQL whose projection is named `position`, except
/// for [`Wrapper::CorrelatedExists`], which needs [`correlated_arms_sql`]'s
/// output instead.
///
/// The bound parameters a caller appends after this, in order: `resume_from`,
/// `ceiling`, `budget` — the same three in every shape, so one parameter list
/// builds all three statements.
#[must_use]
pub fn page_sql_with(wrapper: Wrapper, matched: &str, columns: &str) -> String {
    page_sql_directed(wrapper, matched, columns, false)
}

/// [`page_sql_with`], with the read's direction.
///
/// `backwards` flips only the outer `ORDER BY`. It is a separate entry point
/// rather than a fourth argument on the old one because every table in this
/// crate before it read forwards, and a silent direction parameter would let a
/// shape be re-timed in the other direction without its table saying so.
///
/// A caller pairing this with [`windowed_arms_sql`] must pass the **same**
/// direction to both. An arm ordered `ASC` under a page ordered `DESC` keeps
/// the wrong `budget` positions — the oldest rather than the newest — and
/// returns a page that is *short* rather than obviously wrong, which is why
/// the control in `tests/windowed_arms.rs` compares whole pages and not counts.
#[must_use]
pub fn page_sql_directed(
    wrapper: Wrapper,
    matched: &str,
    columns: &str,
    backwards: bool,
) -> String {
    let direction = if backwards { "DESC" } else { "ASC" };
    match wrapper {
        Wrapper::InSubquery => format!(
            "SELECT {columns} FROM event WHERE position IN ({matched}) \
             AND position >= ? AND position <= ? ORDER BY position {direction} LIMIT ?"
        ),
        // The projection has to be qualified here and only here: the join puts
        // two `position` columns in scope, and a bare one is
        // `ambiguous column name: position` — a *prepare* error, so it surfaces
        // immediately rather than as a wrong answer. The other two wrappers have
        // one `position` in scope and need no prefix.
        Wrapper::Join => format!(
            "SELECT {} FROM event JOIN ({matched}) AS m \
             ON m.position = event.position \
             WHERE event.position >= ? AND event.position <= ? \
             ORDER BY event.position {direction} LIMIT ?",
            qualified(columns, "event")
        ),
        // `EXISTS` **first**, and that is about parameter order rather than
        // about the plan — SQLite reorders a conjunction freely. Every caller
        // binds the matched set's parameters, then `resume_from`, `ceiling`,
        // `budget`, which is the order the other two wrappers already impose. A
        // first version put the two bounds ahead of the body and bound a tag
        // string to `position >= ?`: no error, a plan that looked right, and
        // **zero rows** — the fastest and most wrong arm in the file. The
        // returned-page control in `tests/read_path.rs` is what caught it.
        Wrapper::CorrelatedExists => format!(
            "SELECT {columns} FROM event WHERE EXISTS ({matched}) \
             AND position >= ? AND position <= ? ORDER BY position {direction} LIMIT ?"
        ),
    }
}

/// A comma-separated column list with every name prefixed by `table.`.
///
/// Only [`Wrapper::Join`] needs it; see the comment at that arm.
fn qualified(columns: &str, table: &str) -> String {
    columns
        .split(',')
        .map(|column| format!("{table}.{}", column.trim()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The matched set for `items`, **correlated to `event.position`**.
///
/// [`Wrapper::CorrelatedExists`] cannot use [`arms_sql`]'s output: that is a
/// self-contained `SELECT position …`, and putting it inside `EXISTS` unchanged
/// would test *"does anything match at all"* rather than *"does this row
/// match"* — true for every row of the log, returning the whole table. That is
/// the fastest and most wrong shape this file could produce, so
/// `tests/read_path.rs` compares the returned page across wrappers before any
/// clock starts.
///
/// Arms are joined with `UNION ALL` rather than `UNION`: inside `EXISTS` only
/// the existence of a row matters, so deduplicating them would be work with no
/// observable effect.
///
/// # Panics
///
/// Panics if `items` is empty, which `Query::from_items` refuses upstream.
#[must_use]
pub fn correlated_arms_sql(
    items: &[QueryItem],
    selectivity: &Selectivity,
    params: &mut Vec<Value>,
) -> String {
    assert!(!items.is_empty(), "an empty item list is refused upstream");
    items
        .iter()
        .map(|item| {
            let tags = selectivity.most_selective_first(distinct_tags(item));
            let types = item.types();
            let mut sql =
                String::from("SELECT 1 FROM event_tag AS c0 WHERE c0.position = event.position");
            if let Some(first) = tags.first() {
                params.push(Value::Text(first.clone()));
                sql.push_str(" AND c0.tag = ?");
            }
            if !types.is_empty() {
                for event_type in types {
                    params.push(Value::Text(event_type.as_str().to_owned()));
                }
                sql.push_str(" AND c0.event_type IN (");
                sql.push_str(&placeholders(types.len()));
                sql.push(')');
            }
            for (index, tag) in tags.iter().skip(1).enumerate() {
                let alias = index + 1;
                params.push(Value::Text(tag.clone()));
                sql.push_str(&format!(
                    " AND EXISTS (SELECT 1 FROM event_tag AS c{alias} \
                     WHERE c{alias}.tag = ? AND c{alias}.position = event.position)"
                ));
            }
            sql
        })
        .collect::<Vec<_>>()
        .join(" UNION ALL ")
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
