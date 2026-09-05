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
//! # What this does not fix
//!
//! The **read** path wraps every chunk in a second, uncorrelated
//! `WHERE position IN (<matched>)` (`event_store.rs`'s `fetch_page`), and on a
//! tagged query that wrapper materialises the matched set whatever this module
//! emits. There it is the read path's floor: the correlated chain is worth 1.7x
//! against 1,617x–1,681x on the guard.
//!
//! One half of that is fixed and the other is not, and the line between them is
//! whether the adapter has to *estimate* anything. When the chunk matches every
//! event the wrapper is a tautology over the whole table, which is true by
//! construction — [`matches_every_event`] says so and `fetch_page` omits it.
//! What that was worth, and why, is on `matches_every_event` itself: the two
//! cases do not even share a failure mode, so they do not share a paragraph.
//!
//! For a **tagged** query the wrapper does materialise, and choosing between
//! that and a per-row membership test is a genuine crossover — the replacement
//! is 1,089x better on a query matching every event and 2.3x worse on one
//! matching 1 in 97. Picking between them means the adapter changing its query
//! plan on data it samples, which is ADR-0022's to decide and not this
//! module's; `experiments/correlated-exists-guard/results/read-path.md` prices
//! the candidates and `references/seeds/adr-0022-shipped-shape-drift.md`
//! records what the decision is owed.

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

/// Whether a chunk built with these arguments matches **every** event in the
/// log, so that a caller wrapping it in a membership test would be emitting a
/// tautology.
///
/// It answers for [`chunks`]'s arguments rather than for a `Query` alone,
/// because the two conditions are not the same one. `Query::all` short-circuits
/// to `SELECT position FROM event`, which is every position there is — but only
/// while `bound` is `None`. With a boundary the same query becomes
/// `… WHERE position > ?`, which is a real restriction and must keep its
/// wrapper. A predicate that read only the `Query` would be right on the read
/// path, wrong on the append path, and silently wrong rather than loudly.
///
/// # Why the read path asks, and what the wrapper actually costs
///
/// `fetch_page` wraps each chunk in `WHERE position IN (<matched>)` and then
/// appends its own `position >= ?` for `resume_from`. For `Query::all` the
/// chunk is `SELECT position FROM event`, and the cost of that is **not** the
/// materialisation it looks like: `position` is the rowid, so SQLite answers
/// the `IN` with `USING ROWID SEARCH ON TABLE event FOR IN-OPERATOR` and drives
/// the scan *from the subquery*, in its order, from its first row.
///
/// **That is where `resume_from` goes.** A range predicate on `position` can be
/// pushed into a rowid range scan; it cannot be pushed into a search driven by
/// an `IN` list, so every page re-walks the whole prefix of the log below its
/// own starting point and discards it. The cost therefore grows with how far
/// into a replay the page sits, which makes a full replay at least quadratic in
/// the length of the log — and a replay is the commonest read in the library.
/// Measured at 500,000 events, one 512-row page:
///
/// | page | wrapped | omitted |
/// | --- | ---: | ---: |
/// | first | 122 µs | 100 µs |
/// | half-way | 22,955 µs | 147 µs |
/// | 90% in | 56,310 µs | 182 µs |
///
/// Omitting the wrapper needs no cardinality estimate and no sampling: the set
/// is the whole table by construction. The tagged case is a different question
/// — there the same wrapper really does materialise, and replacing it is a
/// crossover rather than a win. This predicate deliberately does not try to
/// answer that one; see
/// `experiments/correlated-exists-guard/results/all-query-wrapper.md` for the
/// table above and `.../results/read-path.md` for the crossover.
pub(crate) fn matches_every_event(query: &Query, bound: Option<i64>) -> bool {
    query.items().is_none() && bound.is_none()
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

    use super::{Selectivity, chunks, matches_every_event};

    fn tagged_query() -> Query {
        let tags = Tags::from_pairs([("subject", "s1")]).expect("two non-empty strings");
        Query::from_items(vec![QueryItem::tagged(tags).expect("a non-empty tag set")])
            .expect("a non-empty item list")
    }

    /// The predicate and the SQL it speaks for are one fact, so they are
    /// asserted together.
    ///
    /// `matches_every_event` is true exactly when `chunks` emits the
    /// unrestricted `SELECT position FROM event`. Asserting only the boolean
    /// would let the two drift: someone tightening that chunk to carry a
    /// predicate of its own would leave `fetch_page` omitting a wrapper the
    /// plan had come to need, and every test of read *behaviour* would still
    /// pass, because a redundant wrapper is invisible and a missing one is not.
    #[test]
    fn an_unbounded_all_query_is_every_event_and_says_so_in_sql() {
        let query = Query::all();
        let plan = chunks(&query, &Selectivity::default(), 8, None);

        assert!(matches_every_event(&query, None));
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].0, "SELECT position FROM event");
        assert!(plan[0].1.is_empty());
    }

    /// The wrong implementation this predicate exists to reject: one that reads
    /// only the `Query`.
    ///
    /// `Query::all` under a guard is `position > ?` — a real restriction. A
    /// caller that dropped its membership test on the strength of the query
    /// alone would be probing the whole log against a boundary it had stopped
    /// applying, and that is an **accepted append that should have been
    /// rejected**, not a slow read.
    #[test]
    fn a_boundary_makes_the_same_query_a_restriction() {
        let query = Query::all();
        let plan = chunks(&query, &Selectivity::default(), 8, Some(41));

        assert!(!matches_every_event(&query, Some(41)));
        assert_eq!(plan[0].0, "SELECT position FROM event WHERE position > ?");
        assert_eq!(plan[0].1.len(), 1);
    }

    /// A tagged query is a restriction under either bound. This is the case the
    /// read path's crossover is about, and it stays wrapped deliberately.
    #[test]
    fn a_tagged_query_is_never_every_event() {
        let query = tagged_query();

        assert!(!matches_every_event(&query, None));
        assert!(!matches_every_event(&query, Some(0)));
    }
}
