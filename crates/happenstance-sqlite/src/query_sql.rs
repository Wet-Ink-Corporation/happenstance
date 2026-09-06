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
//! tag**, and `tag_cardinality` is what orders it. ADR-0022 §8 makes that
//! ordering a requirement rather than a tuning knob, because SQLite cannot
//! supply per-value cardinality on its own — `ANALYZE` stores an *average*,
//! which is exactly wrong for a tag set where one value matches a third of the
//! log and another matches one percent.
//!
//! # The chain is **correlated**, and that is the whole of why the ordering pays
//!
//! Each chained tag is an `EXISTS (… WHERE tag = ? AND position = seed.position)`,
//! not a `position IN (SELECT position FROM event_tag WHERE tag = ?)`. The
//! difference is not stylistic and it was measured before it was written
//! (`experiments/correlated-exists-guard/`).
//!
//! An uncorrelated `IN (…)` references nothing from the enclosing row, so SQLite
//! materialises it once and in full — `LIST SUBQUERY` over every row carrying
//! the chained tag — and then drives from that list, one `(tag, position)` seek
//! per entry. **That demotes the seed arm to the probe side**, which has two
//! consequences the shape does not advertise: the cost tracks the *chained*
//! tag's cardinality rather than the seed's, and most-selective-first therefore
//! puts the expensive tag in the expensive place. Measured on that shape, the
//! ordering this module requires cost **36x–43x** rather than earning anything
//! (`experiments/correlated-exists-guard/results/seed-ordering.md`).
//!
//! `EXISTS` correlated to `seed.position` cannot be hoisted out of the loop, so
//! the seed arm is the outer loop and its selectivity is what decides. The same
//! ordering then **earns 2.0x–2.2x**. At 10^6 events a two-tag guard went from
//! 579,883 µs to 129 µs, and the plan lost its `LIST SUBQUERY` entirely
//! (`.../results/guard-cost.md`, `.../results/query-plans.md`).
//!
//! The alias is load-bearing rather than tidy: without it,
//! `position = event_tag.position` binds to the *inner* table and becomes the
//! tautology `position = position` — no syntax error, no runtime error, and a
//! multi-tag item silently behaving as a single-tag one. The conformance suite
//! is what catches that, which is why the experiment ran all 89 rules against
//! every candidate shape before timing any of them.
//!
//! # The guard's boundary reaches SQLite
//!
//! [`chunks`] takes a `bound`, and on the append path it is `Some(after)`, so
//! every arm carries `position > ?`. `evaluate` keeps its `highest > boundary`
//! comparison in Rust; the bound makes that trivially true rather than
//! load-bearing, which is what lets the two be changed independently.
//!
//! It is worth 2x–4x on top of the correlated chain (129 µs to 33 µs at the
//! measurement above) and is flat across boundary positions. It is *not* a
//! substitute for it: bound into every arm of the **uncorrelated** chain, the
//! same push is excellent when the boundary sits at the head of the log
//! (137 µs) and worth nothing at all when the guard is unanchored
//! (561,838 µs, no better than no push), because there is then no
//! materialisation for it to shrink. `AppendCondition::new(query)` with no
//! anchor — a unique name, an idempotency key — is exactly that case.
//!
//! # The read path merges rather than wrapping
//!
//! Both callers go through this module, and they ask different questions of it.
//! The guard asks *"is there any matching position above the boundary?"* and
//! takes `max(position)` over [`chunks`]. A paged read asks for **the next
//! `budget` matching positions in order**, which is not a set operation at all,
//! and [`page_statements`] is where that difference finally lives.
//!
//! It used to be papered over: `fetch_page` took `chunks`' output and wrapped it
//! in a second, uncorrelated `WHERE position IN (<matched>)`, so the matched set
//! was produced in full and *then* paged. That is finding I-3, and it cost the
//! read path everything the correlated chain won — 1.7x there against
//! 1,617x–1,681x on the guard.
//!
//! Three replacements were measured before this one and each won at one end of
//! some axis and lost at another, because all three still produced the matched
//! set first. [`page_statements`] instead gives each arm the read's window,
//! leaves the budget on the compound, and joins the result to `event` — which is
//! the shape SQLite merges as co-routines with early termination. It wins
//! sixteen cells out of sixteen across both of VT-23's arm-count floors, both
//! selectivity extremes at each, both directions and three replay depths:
//! `experiments/correlated-exists-guard/results/merge-join.md`.
//!
//! The merge is a **planner choice**, so it is asserted rather than assumed —
//! see `page_statements`, and `event_store.rs`'s
//! `the_page_plan_is_a_merge_and_not_a_sort`.

use std::collections::HashMap;
use std::fmt::Write as _;

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
/// For the same reason there is no separate `statement_count` beside it. A
/// `ceil(arms / max_arms)` of its own agreed with this partition by arithmetic
/// rather than by construction, so it would have gone on reporting a boundary
/// after the read path stopped taking one — a merge that never executes, behind
/// a number saying it did.
/// [`planned_statement_count`](crate::event_store::SqliteEventStore::planned_statement_count)
/// counts *this* call instead.
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
///
/// # `bound` is the guard's boundary, and only the guard has one
///
/// `Some(after)` on the append path, `None` on the read path. When it is
/// `Some`, every arm carries `position > ?` and the chunk therefore returns only
/// positions already above the boundary.
///
/// It is an optimisation and **not** a change of contract: `evaluate` keeps its
/// `highest > boundary` comparison in Rust, which the bound makes trivially true
/// rather than redundant-and-load-bearing. A future edit that removed the
/// boundary from one arm shape and not the others would still be correct for
/// that reason, only slower — which is why the comparison stays where it is.
///
/// Measured before it was written, at 10^6 events with a two-tag guard:
/// `experiments/correlated-exists-guard/results/guard-cost.md`.
pub(crate) fn chunks(
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
    bound: Option<i64>,
) -> Vec<(String, Vec<Value>)> {
    let max_arms = max_arms.max(1);
    match query.items() {
        None => match bound {
            None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
            Some(boundary) => vec![(
                "SELECT position FROM event WHERE position > ?".to_owned(),
                vec![Value::Integer(boundary)],
            )],
        },
        Some(items) => items
            .chunks(max_arms)
            .map(|chunk| {
                let mut params = Vec::new();
                let sql = arms_sql(chunk, selectivity, bound, &mut params);
                (sql, params)
            })
            .collect(),
    }
}

/// The read's own bounds, as the arms of a paged read can carry them.
///
/// `fetch_page` used to apply all four of these *outside* the membership test —
/// `AND position >= ? AND position <= ? ORDER BY … LIMIT ?` — where none of them
/// could reach the arm that produced the matched set. That is finding I-3, and
/// [`page_statements`] is where they go instead.
///
/// `lo` and `hi` are in **position order**, not in read order: under `backwards`
/// the caller's `resume_from` is the *upper* bound and `to` the lower one, and
/// resolving that is the caller's job because only it knows which of `from`,
/// `to` and the ceiling are present.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Window {
    /// Inclusive lower bound in position order. `0` means the start of the log —
    /// a position is a `NonZeroU64`, so the clause is then vacuous and is still
    /// emitted, because an arm that carried the window on some reads and not
    /// others would be two shapes wearing one name.
    pub(crate) lo: i64,
    /// Inclusive upper bound in position order. Always at least as tight as the
    /// ceiling ADR-0011 requires, so ES-12 is discharged by this field and there
    /// is deliberately no second mechanism for it.
    pub(crate) hi: i64,
    /// `min(remaining, PAGE_SIZE)` — the page budget, beneath
    /// [`ReadOptions::limit`](happenstance_core::ReadOptions::limit) and never
    /// equal to it.
    pub(crate) budget: i64,
    /// Whether the read runs newest-first, which decides both the compound's
    /// `ORDER BY` and therefore *which* `budget` positions the merge keeps.
    pub(crate) backwards: bool,
}

/// One statement per chunk for a paged read: the arms **merged**, not unioned
/// into a set and then paged.
///
/// # Why this is not `chunks` with a wrapper
///
/// The other three shapes this replaced all answer *"which positions match?"*
/// and then take a page from the answer — materialising the matched set,
/// testing it per row, or truncating it per arm. Each is best at one end of some
/// axis, and every axis added produced a new crossover: selectivity in
/// `results/read-path.md`, then arm count in `results/wide-arms.md`.
///
/// A paged read does not ask that question. It asks for **the next `budget`
/// matching positions in order**, and the answer to that is a merge of ordered
/// streams with early termination. SQLite has one, for compound `SELECT`s:
///
/// > An alternative method of computing a compound is to run each subquery as a
/// > co-routine, arrange for their outputs to appear in sorted order, and merge
/// > the results together. […] because the co-routine doesn't need to run to
/// > completion before the outer query begins, the first rows appear sooner, and
/// > if the overall query is abandoned before finishing, less work is done
/// > overall.
/// > — <https://sqlite.org/lang_select.html>
///
/// PostgreSQL's planner does the same thing under the name `MergeAppend`.
/// Neither engine can be *asked* for it; the statement has to be shaped so that
/// it is available, and three details are what make it so.
///
/// * **The `LIMIT` sits on the compound, once.** SQLite's grammar allows it
///   nowhere else, and that restriction is the feature: a per-arm limit bounds
///   the work at `budget × arms` — 65,536 rows at VT-23's 128-item floor — where
///   the merge bounds it at `budget`.
/// * **Each arm carries the window and no limit.** Without `position >= ?` every
///   co-routine rewinds to the start of its tag range, so a page late in a
///   replay pays for the whole prefix beneath it.
/// * **`JOIN`, not `position IN (…)`.** An `IN` list is uncorrelated, so SQLite
///   materialises the compound before emitting a row and the early exit is
///   thrown away. That is finding I-3 exactly.
///
/// # The plan is a promise, and it is asserted
///
/// The merge is a *planner choice*. `EXPLAIN QUERY PLAN` says `MERGE (UNION)`
/// when it is taken; a `USE TEMP B-TREE FOR ORDER BY` **inside the co-routine**
/// means SQLite declined and the statement has quietly become a sort of the
/// whole matched set — same rows, same order, and the cost class this function
/// exists to avoid. `the_page_plan_is_a_merge_and_not_a_sort` in this module is
/// what holds that, and it is the falsifier for the whole shape.
///
/// # Chunking is unchanged
///
/// A query wider than `max_arms` still becomes several statements merged by the
/// caller, and each carries the full `budget`: the merged top *b* of a union is
/// a subset of the union of the per-chunk tops. That argument is the same one
/// that makes the compound's single `LIMIT` sound one level down.
///
/// Measured against the three shapes it replaces, sixteen cells out of sixteen:
/// `experiments/correlated-exists-guard/results/merge-join.md`.
pub(crate) fn page_statements(
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
    window: Window,
    columns: &str,
) -> Vec<(String, Vec<Value>)> {
    let max_arms = max_arms.max(1);
    let direction = if window.backwards { "DESC" } else { "ASC" };

    // No items is every event, so there is no membership test to merge and the
    // page is a bounded scan of `event` itself. Emitting the general shape here
    // would join `event` to a subquery over `event` — finding I-3's tautology in
    // a new costume.
    let Some(items) = query.items() else {
        return vec![(
            format!(
                "SELECT {columns} FROM event WHERE position >= ? AND position <= ? \
                 ORDER BY position {direction} LIMIT ?"
            ),
            vec![
                Value::Integer(window.lo),
                Value::Integer(window.hi),
                Value::Integer(window.budget),
            ],
        )];
    };

    items
        .chunks(max_arms)
        .map(|chunk| {
            let mut params = Vec::new();
            let compound = windowed_arms_sql(chunk, selectivity, window, &mut params);
            let sql = format!(
                "SELECT {} FROM event JOIN ({compound}) AS m \
                 ON m.position = event.position ORDER BY event.position {direction}",
                qualified(columns)
            );
            (sql, params)
        })
        .collect()
}

/// The compound [`page_statements`] merges: one windowed arm per item, `UNION`ed,
/// with the budget on the compound.
///
/// `UNION` rather than `UNION ALL` because two items may match one event and the
/// page must not carry it twice. The merge deduplicates as it goes, which is why
/// that costs nothing here and a `DISTINCT` would.
fn windowed_arms_sql(
    items: &[QueryItem],
    selectivity: &Selectivity,
    window: Window,
    params: &mut Vec<Value>,
) -> String {
    let direction = if window.backwards { "DESC" } else { "ASC" };
    let arms = if items.is_empty() {
        // Unconstructable — `Query::from_items` refuses an empty list — but a
        // `SELECT` with no arms is a syntax error rather than an empty result,
        // so the case is spelled rather than assumed.
        "SELECT position FROM event WHERE 0".to_owned()
    } else {
        items
            .iter()
            .map(|item| windowed_item_sql(item, selectivity, window, params))
            .collect::<Vec<_>>()
            .join(" UNION ")
    };
    params.push(Value::Integer(window.budget));
    format!("{arms} ORDER BY position {direction} LIMIT ?")
}

/// One arm of the compound: [`item_sql`]'s shape with the window and no limit.
///
/// The parameter order is the textual order of the `?`s and nothing else — seed
/// tag, types, `lo`, `hi`, chained tags. Getting that wrong binds a tag string
/// to a position comparison, which is not an error and not a wrong plan: it is
/// zero rows.
fn windowed_item_sql(
    item: &QueryItem,
    selectivity: &Selectivity,
    window: Window,
    params: &mut Vec<Value>,
) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();

    if tags.is_empty() {
        let mut sql =
            String::from("SELECT position FROM event WHERE position >= ? AND position <= ?");
        params.push(Value::Integer(window.lo));
        params.push(Value::Integer(window.hi));
        if !types.is_empty() {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            write!(sql, " AND event_type IN ({})", placeholders(types.len()))
                .expect("writing to a String cannot fail");
        }
        return sql;
    }

    let mut sql =
        String::from("SELECT seed.position AS position FROM event_tag AS seed WHERE seed.tag = ?");
    params.push(Value::Text(tags[0].clone()));
    if !types.is_empty() {
        for event_type in types {
            params.push(Value::Text(event_type.as_str().to_owned()));
        }
        write!(
            sql,
            " AND seed.event_type IN ({})",
            placeholders(types.len())
        )
        .expect("writing to a String cannot fail");
    }

    // The window on the seed, which is what turns
    // `SEARCH seed USING PRIMARY KEY (tag=?)` into
    // `(tag=? AND position>? AND position<?)` — a seek into the interior of one
    // contiguous `(tag, position)` range rather than a walk from its start.
    params.push(Value::Integer(window.lo));
    params.push(Value::Integer(window.hi));
    sql.push_str(" AND seed.position >= ? AND seed.position <= ?");

    for (index, tag) in tags[1..].iter().enumerate() {
        params.push(Value::Text(tag.clone()));
        write!(
            sql,
            " AND EXISTS (SELECT 1 FROM event_tag AS m{index} WHERE m{index}.tag = ? \
             AND m{index}.position = seed.position)"
        )
        .expect("writing to a String cannot fail");
    }
    sql
}

/// A comma-separated column list with every name prefixed by `event.`.
///
/// The join puts two `position` columns in scope and a bare one is
/// `ambiguous column name: position` — a *prepare* error, so it surfaces as a
/// failure rather than as a wrong answer, which is the only reason this is safe
/// to do by string manipulation.
fn qualified(columns: &str) -> String {
    columns
        .split(',')
        .map(|column| format!("event.{}", column.trim()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The `UNION` of one arm per item.
///
/// An empty item list cannot be constructed — `Query::from_items` refuses it —
/// but a `SELECT` with no arms would be a syntax error rather than an empty
/// result, so the case is spelled rather than assumed.
pub(crate) fn arms_sql(
    items: &[QueryItem],
    selectivity: &Selectivity,
    bound: Option<i64>,
    params: &mut Vec<Value>,
) -> String {
    if items.is_empty() {
        return "SELECT position FROM event WHERE 0".to_owned();
    }
    items
        .iter()
        .map(|item| item_sql(item, selectivity, bound, params))
        .collect::<Vec<_>>()
        .join(" UNION ")
}

/// SQL selecting the positions matching one item.
///
/// Types within an item are OR — `event_type IN (…)`. Tags within an item are
/// AND, with **superset** matching: an event matches when it carries *at least*
/// the item's tags.
fn item_sql(
    item: &QueryItem,
    selectivity: &Selectivity,
    bound: Option<i64>,
    params: &mut Vec<Value>,
) -> String {
    let tags = selectivity.most_selective_first(distinct_tags(item));
    let types = item.types();

    if tags.is_empty() {
        if types.is_empty() {
            // Unconstructible through the public builders, and cheaper to spell
            // than to reason about: an item constraining nothing matches
            // everything.
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
    // one position carries that position's own type in its covering column —
    // which is the whole point of the covering column, and what keeps this off
    // `event` and out from under the write lock.
    //
    // It is **aliased**, and the alias is load-bearing: the chained tests below
    // correlate against `seed.position`, and both sides of that comparison are
    // the same table. Without the alias `position = event_tag.position` binds to
    // the *inner* table and becomes the tautology `position = position` — no
    // syntax error, no runtime error, and a multi-tag item silently behaving as
    // a single-tag one. `AS position` on the projection is for the callers,
    // which wrap this in `max(position)` and `position IN (…)`.
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
    // The boundary goes on the seed and only on the seed. The chained tests are
    // correlated to a `seed.position` that already satisfies it, so repeating it
    // inside them would restrict nothing.
    if let Some(boundary) = bound {
        params.push(Value::Integer(boundary));
        sql.push_str(" AND seed.position > ?");
    }
    for (index, tag) in tags[1..].iter().enumerate() {
        params.push(Value::Text(tag.clone()));
        // **Correlated, not `position IN (…)`.** An uncorrelated subquery
        // references nothing from the enclosing row, so SQLite is free to
        // materialise it once in full — `LIST SUBQUERY` over every row carrying
        // the chained tag — and drive from that list, which demotes the seed arm
        // to the probe side and makes the seed's selectivity worthless. This
        // references `seed.position`, so it cannot be hoisted, and
        // `(tag, position)` is the primary key of a `WITHOUT ROWID` table, so
        // each evaluation is a point seek.
        //
        // `write!` rather than `push_str(&format!(…))`, which is what clippy's
        // `format_push_string` is about and what it costs here: the latter
        // allocates a second `String` per chained tag, and at VT-22's 64-tag
        // floor that is 63 throwaway allocations inside the write transaction
        // for a statement already being built in place.
        write!(
            sql,
            " AND EXISTS (SELECT 1 FROM event_tag AS m{index} \
             WHERE m{index}.tag = ? AND m{index}.position = seed.position)"
        )
        .expect("writing to a String cannot fail");
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

#[cfg(test)]
mod tests {
    use happenstance_core::{Query, QueryItem, Tags};

    use super::{Selectivity, Value, Window, chunks, page_statements};

    const COLUMNS: &str = "position, event_type";

    fn tagged_query() -> Query {
        let tags = Tags::from_pairs([("subject", "s1")]).expect("two non-empty strings");
        Query::from_items(vec![QueryItem::tagged(tags).expect("a non-empty tag set")])
            .expect("a non-empty item list")
    }

    fn window() -> Window {
        Window {
            lo: 7,
            hi: 900,
            budget: 512,
            backwards: false,
        }
    }

    /// A read over every event joins nothing: it is a bounded scan of `event`.
    ///
    /// The general shape would join `event` to a compound over `event`, which is
    /// finding I-3's tautology in a new costume — the wrapper removed from the
    /// outside and reintroduced as a self-join.
    #[test]
    fn an_all_query_pages_without_a_join() {
        let plan = page_statements(&Query::all(), &Selectivity::default(), 8, window(), COLUMNS);

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].0,
            "SELECT position, event_type FROM event \
             WHERE position >= ? AND position <= ? ORDER BY position ASC LIMIT ?"
        );
        assert_eq!(plan[0].1.len(), 3);
    }

    /// The three details that make the merge available, asserted as text
    /// because each is invisible in a result and decisive in a plan.
    ///
    /// `the_page_plan_is_a_merge_and_not_a_sort` in `event_store.rs` asserts
    /// that SQLite then *takes* the merge. This one asserts that it was offered.
    #[test]
    fn a_tagged_page_carries_the_window_per_arm_and_the_budget_on_the_compound() {
        let plan = page_statements(
            &tagged_query(),
            &Selectivity::default(),
            8,
            window(),
            COLUMNS,
        );

        assert_eq!(plan.len(), 1);
        let sql = &plan[0].0;

        // The window is on the arm, where the index can use it.
        assert!(
            sql.contains("seed.position >= ? AND seed.position <= ?"),
            "the arm lost its window: {sql}"
        );
        // The budget is on the compound, once. A per-arm `LIMIT` would bound the
        // work at `budget x arms` and cost the merge — measured at 17x-48x on a
        // broad 128-item query in `results/merge-join.md`.
        assert_eq!(sql.matches(" LIMIT ?").count(), 1, "{sql}");
        assert!(sql.contains("ORDER BY position ASC LIMIT ?"), "{sql}");
        // A join, not a membership test. `IN` would materialise the compound
        // before emitting a row and throw the early exit away.
        assert!(
            sql.contains(") AS m ON m.position = event.position"),
            "{sql}"
        );
        assert!(!sql.contains("position IN ("), "{sql}");
        // Qualified, because the join puts two `position` columns in scope.
        assert!(sql.starts_with("SELECT event.position, event.event_type FROM event JOIN ("));
    }

    /// Backwards flips both orderings together, and that is the whole of it.
    ///
    /// The arm's direction decides *which* `budget` positions the merge keeps.
    /// An arm ordered `ASC` under a compound ordered `DESC` keeps the oldest
    /// rather than the newest and returns a page that is **short** rather than
    /// obviously wrong — which is why there is one `backwards` field and not
    /// two.
    #[test]
    fn backwards_orders_the_compound_and_the_page_alike() {
        let mut window = window();
        window.backwards = true;
        let plan = page_statements(&tagged_query(), &Selectivity::default(), 8, window, COLUMNS);

        let sql = &plan[0].0;
        assert!(sql.contains("ORDER BY position DESC LIMIT ?"), "{sql}");
        assert!(sql.ends_with("ORDER BY event.position DESC"), "{sql}");
        assert_eq!(sql.matches("ASC").count(), 0, "{sql}");
    }

    /// A query wider than the pushdown limit becomes several statements, each
    /// carrying the **whole** budget.
    ///
    /// Not a share of it: the merged top *b* of a union is a subset of the union
    /// of the per-chunk tops, so a chunk given `budget / n` could drop a row the
    /// page needed whenever the matches are unevenly spread across chunks.
    #[test]
    fn a_wide_query_chunks_and_every_chunk_keeps_the_full_budget() {
        let items = (0..5)
            .map(|index| {
                let value = format!("s{index}");
                let tags =
                    Tags::from_pairs([("subject", value.as_str())]).expect("non-empty strings");
                QueryItem::tagged(tags).expect("a non-empty tag set")
            })
            .collect::<Vec<_>>();
        let query = Query::from_items(items).expect("a non-empty item list");

        let plan = page_statements(&query, &Selectivity::default(), 2, window(), COLUMNS);

        assert_eq!(plan.len(), 3);
        for (sql, params) in &plan {
            assert_eq!(sql.matches(" LIMIT ?").count(), 1, "{sql}");
            assert_eq!(params.last(), Some(&Value::Integer(512)), "{sql}");
        }
    }

    /// The guard path is untouched by any of this, and its boundary still
    /// reaches SQLite.
    ///
    /// `Query::all` under a guard is `position > ?` — a real restriction, not a
    /// tautology. A change that treated the two paths alike would drop that
    /// boundary, and the result is an **accepted append that should have been
    /// rejected** rather than a slow read.
    #[test]
    fn the_guard_still_binds_its_boundary() {
        let unbounded = chunks(&Query::all(), &Selectivity::default(), 8, None);
        assert_eq!(unbounded[0].0, "SELECT position FROM event");
        assert!(unbounded[0].1.is_empty());

        let bounded = chunks(&Query::all(), &Selectivity::default(), 8, Some(41));
        assert_eq!(
            bounded[0].0,
            "SELECT position FROM event WHERE position > ?"
        );
        assert_eq!(bounded[0].1.len(), 1);
    }
}
