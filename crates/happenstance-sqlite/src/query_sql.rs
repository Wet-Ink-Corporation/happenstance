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
//! # Two pushdown limits, and a partition on one is not a partition on the other
//!
//! SQLite pushes back in two units and this module has to answer both.
//! `SQLITE_MAX_COMPOUND_SELECT` bounds the **arms** of a `UNION` at 500 terms;
//! `SQLITE_MAX_VARIABLE_NUMBER` bounds the **bound parameters** of one statement
//! at 32,766, and [`item_sql`] pushes one per tag and one per type of every item
//! in the chunk. Nothing in `happenstance-core`'s `query.rs` bounds tags per
//! query item, so the two numbers move independently: 400 single-tag items is
//! 400 arms and 400 parameters, and the same 400 items at `MAX_TAGS_PER_EVENT`
//! tags apiece is still 400 arms and **51,200 parameters**.
//!
//! That second query was planned as one statement, by the adapter's own chosen
//! arm width, carrying the adapter's own documented maximum tag count — and
//! `prepare` refused it with *"too many SQL variables"*, wrapped as
//! `AppendError::Store`, inside `BEGIN IMMEDIATE` on the append path. VT-23
//! names that implementation in terms: *an adapter that generates one SQL
//! parameter per item and silently fails past a driver limit*. So [`chunks`]
//! partitions on **both** axes, and [`Selectivity::read_for`] — whose single
//! `IN (…)` over the whole query's distinct tags is the site that fails *first*,
//! because it runs before [`chunks`] on both callers — takes a width of its own
//! rather than having none.
//!
//! The parameter axis was never unknown here. `event_store.rs`'s
//! `PARAMETER_BUDGET` has carried the arithmetic and the headroom since the
//! write path was written; it simply had one consumer, the tag insert, where it
//! should have had three. `tests/wide_tags.rs` is the standing guard, and it
//! crosses the axis at both callers because only one of them holds the write
//! lock.
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

use std::collections::{BTreeSet, HashMap};
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
    /// One statement per `max_parameters` tags rather than one per item: a
    /// 128-item query is exactly the shape that makes a per-item lookup
    /// expensive, and it is the shape VT-23 requires every store to evaluate.
    ///
    /// **`max_parameters` is the width this lookup had no concept of.** The
    /// accumulation below runs across *every* multi-tag item of the whole query
    /// with nothing bounding it, so a query of ordinary two-tag items reaches
    /// `SQLITE_MAX_VARIABLE_NUMBER` at 16,384 items — no wide item required —
    /// and it reaches it *before* [`chunks`] does, on both callers. Chunking the
    /// lookup is exact rather than approximate for the reason a lookup is not a
    /// filter: the result is a map keyed by tag, so two statements over disjoint
    /// halves of `wanted` and one over all of it insert the same entries. A tag
    /// absent from `tag_cardinality` has no entry either way, which
    /// [`Selectivity::most_selective_first`] already treats as maximally
    /// selective.
    ///
    /// # Why the accumulator is a set
    ///
    /// It was a `Vec<String>` guarded by `if !wanted.contains(&tag)`, which is
    /// quadratic in the query's *distinct* tags — and the shape that reaches
    /// that quadratic is not a corner but VT-23's own floor. At 128 items
    /// carrying [`MAX_TAGS_PER_EVENT`](crate::event_store::SqliteEventStore::MAX_TAGS_PER_EVENT)
    /// tags apiece, 16,384 tags are presented here and the guard performs about
    /// 134 million string comparisons before a single statement is prepared —
    /// once per read page, and once per append guard **inside `BEGIN
    /// IMMEDIATE`**, which is a quarter of a second every other writer waits.
    /// `experiments/shipped-append-condition-sql/results/selectivity.md` §1
    /// measured it: 257,690 µs against a set's 6,424 µs, **40.1x**, outputs
    /// asserted byte-identical before either was timed.
    ///
    /// [`BTreeSet`] rather than [`HashSet`](std::collections::HashSet), and the
    /// reason is the chunking two lines down: a hash set's iteration order is
    /// unspecified, so the partition of `wanted` into statements would differ
    /// between runs of one binary. The *result* would not — a lookup keyed by
    /// tag is insensitive to which statement found each row, which is the
    /// property the paragraph above rests on — but a plan that is not
    /// reproducible is one nobody can bisect. `&str` rather than `String`
    /// because every tag here is borrowed from `query`, which outlives this
    /// call: the old accumulator allocated one `String` per tag on top of being
    /// quadratic.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if a statement fails.
    pub(crate) fn read_for(
        connection: &Connection,
        query: &Query,
        max_parameters: usize,
    ) -> rusqlite::Result<Self> {
        let mut wanted: BTreeSet<&str> = BTreeSet::new();
        for item in query.items().unwrap_or_default() {
            let tags = item.tags();
            if tags.len() > 1 {
                for tag in tags {
                    wanted.insert(tag.as_str());
                }
            }
        }
        if wanted.is_empty() {
            return Ok(Self::default());
        }

        let wanted: Vec<&str> = wanted.into_iter().collect();
        let mut counts = HashMap::with_capacity(wanted.len());
        for batch in wanted.chunks(max_parameters.max(1)) {
            let sql = format!(
                "SELECT tag, events FROM tag_cardinality WHERE tag IN ({})",
                placeholders(batch.len())
            );
            let mut statement = connection.prepare(&sql)?;
            let mut rows = statement.query(rusqlite::params_from_iter(batch.iter()))?;
            while let Some(row) = rows.next()? {
                counts.insert(row.get::<_, String>(0)?, row.get::<_, i64>(1)?);
            }
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

/// One `SELECT position …` subquery per chunk, bounded by **both** of SQLite's
/// pushdown limits: at most `max_arms` items and at most `max_parameters` bound
/// parameters.
///
/// **Chunk and merge, never refuse.** A `Query` bounds nothing by design and the
/// specification requires every store to evaluate at least 128 items, so an
/// adapter that returned an error at its own pushdown limit would be inventing a
/// refusal the contract has no way to report. The merge over these cursors is
/// the caller's; what belongs here is only the decomposition.
///
/// **Both axes, because the arm count does not imply the parameter count.**
/// Partitioning on `max_arms` alone is what let 400 items — the arm width
/// exactly — carrying `MAX_TAGS_PER_EVENT` tags apiece bind 51,200 of SQLite's
/// 32,766 parameters in a statement this function reported as a valid plan of
/// one. See the module doc; `tests/wide_tags.rs` is the standing guard.
///
/// **One item is the atom of the partition and is never split.** An item's arm
/// is an intersection — `tag = ? AND position IN (…) AND position IN (…)` — and
/// splitting an intersection across statements is not a union merge, so the two
/// halves could not be recombined by the caller's `UNION` or its `max()`. An
/// item whose own tags exceed `max_parameters` therefore still gets a chunk to
/// itself and still fails at `prepare`, which is a refusal this decomposition
/// cannot remove; it takes 32,766 tags on a single query item to reach, against
/// a store that accepts 128 on an event.
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
    max_parameters: usize,
    bound: Option<i64>,
) -> Vec<(String, Vec<Value>)> {
    match query.items() {
        None => match bound {
            None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
            Some(boundary) => vec![(
                "SELECT position FROM event WHERE position > ?".to_owned(),
                vec![Value::Integer(boundary)],
            )],
        },
        // The boundary costs one bound parameter *per arm* — `item_sql` puts it
        // on the seed of every item — so it is priced into the per-item cost
        // rather than subtracted from the budget once.
        Some(items) => partition(
            items,
            max_arms.max(1),
            max_parameters.max(1),
            usize::from(bound.is_some()),
        )
        .into_iter()
        .map(|chunk| {
            let mut params = Vec::new();
            let sql = arms_sql(chunk, selectivity, bound, &mut params);
            (sql, params)
        })
        .collect(),
    }
}

/// `items` cut into runs that satisfy both limits, in order.
///
/// Greedy and order-preserving, because a chunk is only ever merged by `UNION`
/// or by `max()` — neither of which cares which chunk an item landed in — and
/// because reordering items to pack chunks tighter would make the partition
/// depend on the query's shape rather than on its prefix, which is exactly the
/// property that makes `planned_statement_count` computable by a caller.
///
/// The `arms > 0` guard is what stops an item too wide for `max_parameters` on
/// its own from emitting an empty chunk forever. It gets a chunk to itself
/// instead, which is the honest outcome: see the note on splitting in
/// [`chunks`].
///
/// `per_arm_extra` is what the *caller's* shape spends on every arm on top of
/// the item's own tags and types, and it is a parameter rather than a constant
/// because the two callers do not spend the same. [`chunks`] binds the guard
/// boundary once per arm, so it passes `1` when there is a boundary and `0` when
/// there is not. [`page_statements`] binds `lo` and `hi` on every arm, so it
/// passes `2` — and it also spends one *per chunk* on the compound's `LIMIT`,
/// which it takes off the budget before calling rather than pricing here,
/// because a per-chunk cost is not a per-arm one and folding the two together is
/// how a ceiling ends up in the wrong unit.
///
/// That distinction is the whole reason this function has a fourth argument.
/// The read path arrived here from a rewrite that chunked on `max_arms` alone;
/// merging it without this would have left every windowed read counting arms
/// while SQLite counts parameters, which is the defect `partition` exists to
/// close — reintroduced on the one path the rewrite touched.
fn partition(
    items: &[QueryItem],
    max_arms: usize,
    max_parameters: usize,
    per_arm_extra: usize,
) -> Vec<&[QueryItem]> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut arms = 0;
    let mut parameters = 0;

    for (index, item) in items.iter().enumerate() {
        let cost = item_parameters(item) + per_arm_extra;
        if arms > 0 && (arms == max_arms || parameters + cost > max_parameters) {
            out.push(&items[start..index]);
            start = index;
            arms = 0;
            parameters = 0;
        }
        arms += 1;
        parameters += cost;
    }

    out.push(&items[start..]);
    out
}

/// Bound parameters one item's arm will cost, counted the way [`item_sql`] spends
/// them.
///
/// One per distinct tag and one per type, in every branch: the tagless branch
/// binds its types and nothing else, and the tagged branch binds the seed tag,
/// then the types, then one per remaining tag.
///
/// `item.tags().len()` **is** the distinct count, and reads it in `O(1)` rather
/// than deduplicating to find out: VT-16 `[FROZEN]` makes `Tags` canonical. See
/// [`owned_tags`], which is where that premise is stated and checked. This
/// function is called once per item by [`partition`], so the dedup it used to do
/// was the per-item quadratic paid a second time on the planning path — a third
/// time counting [`item_sql`]'s own call.
///
/// This is a second reading of [`item_sql`], which is the shape that drifts —
/// add a bound parameter there and this undercounts, and the partition silently
/// goes back to being wrong past a driver limit. What catches that is
/// `tests/wide_tags.rs`'s boundary case, which computes the expected chunk count
/// from the public ceilings and the query it built and compares it against
/// `planned_statement_count`: an undercount here moves one of those and not the
/// other. The alternative — returning the count from `item_sql` itself — would
/// mean building the SQL twice for every plan, once to size it and once to use
/// it, on the path that runs under the write lock.
fn item_parameters(item: &QueryItem) -> usize {
    item.tags().len() + item.types().len()
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
///
/// # Both ceilings, not one
///
/// This chunks on `max_parameters` as well as `max_arms`, for the reason
/// [`chunks`] does and with two costs [`chunks`] does not have: every arm binds
/// `lo` and `hi`, and every chunk binds the compound's `LIMIT`. The budget
/// handed to [`partition`] is therefore `max_parameters - 1` with `2` per arm.
///
/// It is spelled out because it was nearly lost. The merge-join shape was
/// written against a planner that chunked on arms alone, and arrived here after
/// the parameter ceiling landed; taking it as written would have moved every
/// windowed read onto a statement builder with no parameter bound at all, which
/// SQLite reports as `too many SQL variables` at prepare time on a query wide
/// enough to need chunking — the exact failure the ceiling exists to prevent,
/// on the exact path this function replaced.
pub(crate) fn page_statements(
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
    max_parameters: usize,
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

    // `saturating_sub(1)` for the compound's `LIMIT`, which is one parameter per
    // *chunk* rather than per arm; `.max(1)` so a caller that hands over a
    // ceiling of one still gets a partition rather than an empty one.
    partition(items, max_arms, max_parameters.saturating_sub(1).max(1), 2)
        .into_iter()
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
    let tags = selectivity.most_selective_first(owned_tags(item));
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
    let tags = selectivity.most_selective_first(owned_tags(item));
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

/// The item's tags, owned, so that [`Selectivity::most_selective_first`] can
/// sort them.
///
/// **It does not deduplicate, and that is a fact about the input rather than an
/// omission.** VT-16 `[FROZEN]` makes `Tags` canonical — sorted and
/// deduplicated at construction, `happenstance_core::Tags` doing the work in
/// `FromIterator` — so the `if !out.contains(&value)` guard this replaced could
/// never remove anything. It was quadratic *and* dead: measured at 4.6–5.0 ms
/// per plan at the ceilings, against a `Tags` that had already paid for the
/// property. `a_query_items_tags_are_already_canonical` is the standing check on
/// that premise, because it is a premise about a type in another crate.
///
/// The owning is not dead, and it is why this is not simply `item.tags()`:
/// `most_selective_first` sorts, `item_sql` binds each value as an owned
/// [`Value::Text`], and the seed tag is cloned out of the sorted order.
fn owned_tags(item: &QueryItem) -> Vec<String> {
    item.tags()
        .iter()
        .map(|tag| tag.as_str().to_owned())
        .collect()
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
    //! The measurement that cannot be taken from outside this crate.
    //!
    //! [`Selectivity::read_for`] is `pub(crate)` by ADR-0022 §10's decision, and
    //! the one public seam beside it —
    //! [`SqliteEventStore::planned_statement_count`](crate::event_store::SqliteEventStore::planned_statement_count)
    //! — passes `Selectivity::default()` and never calls it. No target in
    //! `tests/` can reach the function, so the measurement lives where the
    //! function does. This is the crate's only `#[cfg(test)]` module and it is
    //! here for that reason rather than by preference.
    //!
    //! **Why a clock is allowed here and forbidden in the suite.** CF-33
    //! `[FROZEN]` binds `crates/happenstance-testkit/src` — the conformance
    //! rules — and its `Rejects:` paragraph names what makes a timed assertion
    //! bad: it *"makes the suite's verdict a property of the hardware"*.
    //! Nothing below asserts a duration. The one timed assertion is a **ratio
    //! between two implementations measured back to back in one process**, on
    //! the same input and in the same build profile — the shipped accumulation
    //! against a verbatim copy of the one it replaced. That is a property of the
    //! two algorithms and not of the machine, which is exactly the objection
    //! CF-33 raises. CF-34 still holds: this is not a performance bar, and no
    //! number here is a budget.

    #![allow(clippy::unwrap_used)]

    use std::time::Instant;

    use happenstance_core::Tags;

    use super::*;

    /// VT-23's floor. It is what makes the shape below a conformance floor
    /// rather than a corner: every store must evaluate at least this many query
    /// items, and nothing anywhere bounds tags per item.
    const ITEMS: usize = 128;

    /// `SqliteEventStore::MAX_TAGS_PER_EVENT`, restated rather than imported so
    /// this module does not acquire the event store's feature gate.
    const TAGS_PER_ITEM: usize = 128;

    /// How much faster than the implementation it replaced the shipped
    /// accumulation must be, at the floor above.
    ///
    /// `experiments/shipped-append-condition-sql/results/selectivity.md` §1
    /// measured the separation at **40.1x** — 257,690 µs against 6,424 µs,
    /// medians of 25 rounds, with the two outputs asserted byte-identical before
    /// either was timed. The threshold here is **5**, and the eightfold gap
    /// between the two numbers is the slack: this fires when the quadratic comes
    /// back, not when the machine is busy.
    const MINIMUM_SPEEDUP: u32 = 5;

    /// A query at VT-23's floor whose every item is multi-tag and whose tags are
    /// distinct across items.
    ///
    /// Multi-tag because `read_for` skips single-tag items entirely, and
    /// distinct across items because that is what makes the accumulation grow:
    /// 128 x 128 is 16,384 tags presented to the planning path, every one of
    /// them a `wanted.contains` miss over everything accumulated so far.
    fn floor_query() -> Query {
        let items = (0..ITEMS).map(|item| {
            let pairs: Vec<(String, String)> = (0..TAGS_PER_ITEM)
                .map(|tag| (format!("k{item:03}"), format!("v{tag:03}")))
                .collect();
            let tags = Tags::from_pairs(
                pairs
                    .iter()
                    .map(|(key, value)| (key.as_str(), value.as_str())),
            )
            .unwrap();
            QueryItem::new(Vec::<String>::new(), tags).unwrap()
        });
        Query::from_items(items).unwrap()
    }

    /// A connection carrying migration 1's `tag_cardinality` and nothing else.
    ///
    /// Empty on purpose: the lookup's cost is in assembling what to ask for, not
    /// in the answer, and an empty table is the case
    /// [`Selectivity::most_selective_first`] already treats as maximally
    /// selective.
    fn planning_connection() -> Connection {
        let connection = Connection::open_in_memory().unwrap();
        connection
            .execute_batch(
                "CREATE TABLE tag_cardinality (tag TEXT PRIMARY KEY, events INTEGER NOT NULL) \
                 WITHOUT ROWID;",
            )
            .unwrap();
        connection
    }

    /// [`Selectivity::read_for`] as it stood at `bd11598`, kept verbatim so the
    /// claim about it is checked rather than asserted.
    ///
    /// The only edits are its name and its receiver. Everything else — the
    /// `Vec<String>` accumulator, the `contains` guard, the per-item
    /// `distinct_tags`, the chunked lookup and the map it builds — is the
    /// shipped code of that commit, which is what makes the comparison below a
    /// comparison of two implementations rather than of an implementation
    /// against a straw man.
    fn read_for_quadratic(
        connection: &Connection,
        query: &Query,
        max_parameters: usize,
    ) -> rusqlite::Result<Selectivity> {
        fn distinct_tags_quadratic(item: &QueryItem) -> Vec<String> {
            let mut out: Vec<String> = Vec::with_capacity(item.tags().len());
            for tag in item.tags() {
                let value = tag.as_str().to_owned();
                if !out.contains(&value) {
                    out.push(value);
                }
            }
            out
        }

        let mut wanted: Vec<String> = Vec::new();
        for item in query.items().unwrap_or_default() {
            let tags = distinct_tags_quadratic(item);
            if tags.len() > 1 {
                for tag in tags {
                    if !wanted.contains(&tag) {
                        wanted.push(tag);
                    }
                }
            }
        }
        if wanted.is_empty() {
            return Ok(Selectivity::default());
        }

        let mut counts = HashMap::with_capacity(wanted.len());
        for batch in wanted.chunks(max_parameters.max(1)) {
            let sql = format!(
                "SELECT tag, events FROM tag_cardinality WHERE tag IN ({})",
                placeholders(batch.len())
            );
            let mut statement = connection.prepare(&sql)?;
            let mut rows = statement.query(rusqlite::params_from_iter(batch.iter()))?;
            while let Some(row) = rows.next()? {
                counts.insert(row.get::<_, String>(0)?, row.get::<_, i64>(1)?);
            }
        }
        Ok(Selectivity(counts))
    }

    /// The shipped selectivity lookup agrees with the one it replaced, and is
    /// not quadratic in the query's distinct tags.
    ///
    /// Two assertions, and the first is the one that makes the second worth
    /// making: the outputs are compared **before** either is timed, so a faster
    /// function that answers a different question fails here rather than passing
    /// as an optimisation.
    ///
    /// The wrong implementation this rejects is the one that shipped: a
    /// `Vec<String>` accumulated with `if !wanted.contains(&tag)`, which at
    /// VT-23's own floor performs about 134 million string comparisons before a
    /// single statement is prepared — once per read page, and once per append
    /// guard **inside `BEGIN IMMEDIATE`**, with every other writer waiting.
    #[test]
    fn the_selectivity_lookup_agrees_with_the_quadratic_and_is_faster_than_it() {
        let connection = planning_connection();
        let query = floor_query();

        let expected = read_for_quadratic(
            &connection,
            &query,
            crate::event_store::SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        )
        .unwrap();
        let actual = Selectivity::read_for(
            &connection,
            &query,
            crate::event_store::SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        )
        .unwrap();
        assert_eq!(
            actual.0, expected.0,
            "the shipped lookup answers a different question from the one it replaced"
        );

        // One sample for the slow arm, because at seconds it is stable and three
        // of them is a minute; the best of three for the fast arm, because at
        // milliseconds a single sample is mostly scheduler.
        let started = Instant::now();
        read_for_quadratic(
            &connection,
            &query,
            crate::event_store::SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        )
        .unwrap();
        let quadratic = started.elapsed();

        let shipped = (0..3)
            .map(|_| {
                let started = Instant::now();
                Selectivity::read_for(
                    &connection,
                    &query,
                    crate::event_store::SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
                )
                .unwrap();
                started.elapsed()
            })
            .min()
            .unwrap();

        assert!(
            quadratic >= shipped * MINIMUM_SPEEDUP,
            "at VT-23's floor ({ITEMS} items x {TAGS_PER_ITEM} tags) the shipped selectivity \
             lookup took {shipped:?} against the quadratic's {quadratic:?}, which is under the \
             {MINIMUM_SPEEDUP}x this asserts; the accumulation is quadratic in the query's \
             distinct tags again"
        );
    }

    /// The per-item deduplication was dead work, not merely quadratic.
    ///
    /// VT-16 `[FROZEN]` makes [`Tags`] canonical — sorted and deduplicated at
    /// construction — so the `if !out.contains(&value)` guard that
    /// `distinct_tags` carried could never remove anything. That is the premise
    /// [`owned_tags`] rests on, and it is a premise about a type in another
    /// crate, so it is checked here rather than assumed: if `Tags` ever stops
    /// being canonical, this fails before the SQL does.
    ///
    /// The inputs are the three shapes that would defeat a weaker guarantee: the
    /// same pair twice, one key with two values, and two keys whose
    /// concatenations collide on a prefix.
    #[test]
    fn a_query_items_tags_are_already_canonical() {
        fn distinct_tags_quadratic(item: &QueryItem) -> Vec<String> {
            let mut out: Vec<String> = Vec::with_capacity(item.tags().len());
            for tag in item.tags() {
                let value = tag.as_str().to_owned();
                if !out.contains(&value) {
                    out.push(value);
                }
            }
            out
        }

        for pairs in [
            vec![("course", "c1"), ("course", "c1")],
            vec![("course", "c1"), ("course", "c10")],
            vec![("course", "c1"), ("student", "s1"), ("course", "c1")],
            vec![("a", "b"), ("a", "bc"), ("ab", "c")],
        ] {
            let tags = Tags::from_pairs(pairs.iter().copied()).unwrap();
            let item = QueryItem::new(Vec::<String>::new(), tags).unwrap();

            assert_eq!(
                owned_tags(&item),
                distinct_tags_quadratic(&item),
                "the dedup removed something, so it was not dead work: {pairs:?}"
            );
            assert_eq!(
                item_parameters(&item),
                distinct_tags_quadratic(&item).len() + item.types().len(),
                "the parameter count parted company with what `item_sql` will bind: {pairs:?}"
            );
        }
    }

    const COLUMNS: &str = "position, event_type";

    /// A parameter ceiling wide enough never to be the axis that cuts.
    ///
    /// These cases pin the **arm** partition and the shape of one statement,
    /// so the parameter ceiling has to be present — `page_statements` takes
    /// both — and has to be irrelevant. `wide_tags.rs` is where the other axis
    /// is the subject.
    const WIDE: usize = usize::MAX;

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
        let plan = page_statements(
            &Query::all(),
            &Selectivity::default(),
            8,
            WIDE,
            window(),
            COLUMNS,
        );

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
            WIDE,
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
        let plan = page_statements(
            &tagged_query(),
            &Selectivity::default(),
            8,
            WIDE,
            window,
            COLUMNS,
        );

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

        let plan = page_statements(&query, &Selectivity::default(), 2, WIDE, window(), COLUMNS);

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
        let unbounded = chunks(&Query::all(), &Selectivity::default(), 8, WIDE, None);
        assert_eq!(unbounded[0].0, "SELECT position FROM event");
        assert!(unbounded[0].1.is_empty());

        let bounded = chunks(&Query::all(), &Selectivity::default(), 8, WIDE, Some(41));
        assert_eq!(
            bounded[0].0,
            "SELECT position FROM event WHERE position > ?"
        );
        assert_eq!(bounded[0].1.len(), 1);
    }
}
