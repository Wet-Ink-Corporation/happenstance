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
//! tag**, which keeps the boundary pushable for the same reason and is what
//! `tag_cardinality` exists to order. ADR-0022 §8 makes that ordering a
//! requirement rather than a tuning knob: a two-tag boundary over a
//! 50,000-event log costs roughly 200x a single-tag one, and SQLite cannot
//! supply per-value cardinality on its own — `ANALYZE` stores an *average*,
//! which is exactly wrong for a tag set where one value matches a third of the
//! log and another matches one percent.

use std::collections::{BTreeSet, HashMap};

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
pub(crate) fn chunks(
    query: &Query,
    selectivity: &Selectivity,
    max_arms: usize,
    max_parameters: usize,
) -> Vec<(String, Vec<Value>)> {
    match query.items() {
        None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
        Some(items) => partition(items, max_arms.max(1), max_parameters.max(1))
            .into_iter()
            .map(|chunk| {
                let mut params = Vec::new();
                let sql = arms_sql(chunk, selectivity, &mut params);
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
fn partition(items: &[QueryItem], max_arms: usize, max_parameters: usize) -> Vec<&[QueryItem]> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut arms = 0;
    let mut parameters = 0;

    for (index, item) in items.iter().enumerate() {
        let cost = item_parameters(item);
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
    let tags = selectivity.most_selective_first(owned_tags(item));
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
}
