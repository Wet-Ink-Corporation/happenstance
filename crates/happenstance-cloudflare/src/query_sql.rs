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
//! # Two pushdown limits, and neither implies the other
//!
//! A `Query` bounds nothing by design: VT-23 requires every store to evaluate at
//! least 128 items and puts no ceiling above that, and nothing in
//! `happenstance-core`'s `query.rs` bounds tags per query item at all. SQLite
//! pushes back in two units and both are reachable from the contract's own
//! floor.
//!
//! * `SQLITE_MAX_COMPOUND_SELECT` — **500 terms**. [`arms`] joins one term per
//!   item, so a decision model of 1,000 boundaries is one statement of 1,000
//!   terms.
//! * `SQLITE_MAX_VARIABLE_NUMBER` — **32,766 bound parameters**. [`item_sql`]
//!   spends one per tag and one per type, so 400 items — inside any plausible
//!   arm width — carrying this store's own declared `tags_per_event` of 1,024
//!   apiece bind 409,600.
//!
//! Neither number is derivable from the other, so [`chunks`] partitions on
//! **both**, and a wide query becomes several statements the caller merges
//! rather than a refusal at the pushdown limit. The refusal is what VT-23 names
//! as its wrong implementation, and on the append path it would arrive as
//! `AppendError::Store` carrying a raw driver string, inside the turn, with the
//! caller's decision already taken.
//!
//! **What this crate does not do is keep a second, unchunked spelling beside the
//! chunked one.** `happenstance-sqlite`'s own `query_sql.rs` records that as
//! exactly how its write path stayed unchunked while its module doc claimed the
//! translation was shared. A single-chunk plan is the narrow case of the wide
//! one, so there is nothing a second spelling could say that this cannot.
//!
//! # Where this diverges from the sibling, and why
//!
//! The widths are the sibling's, because the storage underneath is the same
//! SQLite and the two constants are properties of *it* rather than of either
//! adapter. The **merge** is not. `happenstance-sqlite` collects every chunk's
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

use happenstance_core::{Query, QueryItem};

use crate::sql_storage::SqlValue;

/// One `SELECT position …` statement per chunk, bounded by **both** of SQLite's
/// pushdown limits: at most `max_arms` items and at most `max_parameters` bound
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
/// is an intersection — `tag = ? AND position IN (…) AND position IN (…)` — and
/// the halves of an intersection cannot be recombined by the caller's `UNION` or
/// its `max()`. An item whose own tags exceed `max_parameters` therefore still
/// gets a chunk to itself and would still be refused by the driver; reaching
/// that needs 32,766 tags on a single query item, against a store that accepts
/// 1,024 on an event.
pub(crate) fn chunks(
    query: &Query,
    max_arms: usize,
    max_parameters: usize,
) -> Vec<(String, Vec<SqlValue>)> {
    match query.items() {
        // `Query::all` short-circuits to `event` rather than going through the
        // tag index, and that is the definition rather than an optimisation:
        // `all` matches every event *including an untagged one*, and an untagged
        // event has no row in `event_tag` at all.
        None => vec![("SELECT position FROM event".to_owned(), Vec::new())],
        Some(items) => partition(items, max_arms.max(1), max_parameters.max(1))
            .into_iter()
            .map(|chunk| {
                let mut bindings = Vec::new();
                let sql = arms(chunk, &mut bindings);
                (sql, bindings)
            })
            .collect(),
    }
}

/// `items` cut into runs that satisfy both limits, in order.
///
/// Greedy and order-preserving. A chunk is only ever merged by `UNION` or by
/// `max()`, neither of which cares which chunk an item landed in, so packing
/// tighter by reordering would buy nothing and would make the partition depend
/// on the query's shape rather than on its prefix — which is the property that
/// lets a caller compute
/// [`planned_statement_count`](crate::event_store::CloudflareEventStore::planned_statement_count)
/// for itself.
///
/// The `arms > 0` guard is what stops an item too wide for `max_parameters` on
/// its own from emitting an empty chunk forever; it gets a chunk to itself
/// instead, which is the honest outcome. See the note on splitting in
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

/// Bound parameters one item's arm will cost, counted the way [`item_sql`]
/// spends them.
///
/// One per distinct tag and one per type, in every branch: the tagless branch
/// binds its types and nothing else, and the tagged branch binds the seed tag,
/// then the types, then one per remaining tag.
///
/// This is a second reading of [`item_sql`], which is the shape that drifts —
/// add a bound parameter there and this undercounts, and the partition goes back
/// to being wrong past a driver limit without saying so. What catches that is
/// the boundary case in this module's own tests, which computes the expected
/// chunk count from the declared ceilings and the query it built and compares it
/// against the partition: an undercount moves one of those and not the other.
/// The alternative — returning the count from [`item_sql`] itself — would mean
/// building every statement twice, once to size it and once to use it, on the
/// path that runs inside the append turn.
fn item_parameters(item: &QueryItem) -> usize {
    distinct_tags(item).len() + item.types().len()
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
    use crate::event_store::CloudflareEventStore;

    /// `SQLITE_MAX_COMPOUND_SELECT`, the default this runtime's SQLite is built
    /// with: how many terms one compound `SELECT` may carry.
    const COMPOUND_SELECT_TERMS: usize = 500;

    /// `SQLITE_MAX_VARIABLE_NUMBER`, likewise: bound parameters per statement.
    const BOUND_PARAMETERS: usize = 32_766;

    /// The plan for `query`, as the shipped translation produces it.
    ///
    /// This helper is the **only** line the partition moved: before it, the
    /// translation returned one statement and this wrapped it in a one-element
    /// vector; after it, the translation returns the plan itself. Every
    /// assertion below is stated against the plan rather than against the
    /// spelling that produced it, so the two failing cases that named this
    /// defect assert today exactly what they asserted when they were red.
    fn plan(query: &Query) -> Vec<(String, Vec<SqlValue>)> {
        chunks(
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
                "a plan over {items} items of {tags} tags carries                  {} arms; a partition drops nothing",
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

    /// The boundary itself: one parameter under the budget, exactly at it, and
    /// one over.
    ///
    /// Off-by-one at a partition boundary is the defect this class of fix
    /// reintroduces. The arm axis is held slack — every case is
    /// `MAX_QUERY_ARMS_PER_STATEMENT` items, never more — so what moves the
    /// answer is the parameter count and nothing else. A ceiling is a promise
    /// about the statement that *is* issued: at exactly the budget the plan is
    /// one statement, and one parameter over it is two.
    #[test]
    fn the_parameter_partition_splits_one_over_the_budget_and_not_before() {
        let arms = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
        let budget = CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT;
        let flat = budget / arms;
        let remainder = budget - flat * arms;

        // Item 0 carries the remainder, so the total is exactly the budget and
        // the under/over cases move item 0 alone.
        let widths = |delta: isize| -> Vec<usize> {
            let mut widths = vec![flat; arms];
            widths[0] = widths[0]
                .saturating_add(remainder)
                .saturating_add_signed(delta);
            widths
        };
        let of_widths = |widths: &[usize]| -> Query {
            Query::from_items(widths.iter().enumerate().map(|(item, width)| {
                let pairs: Vec<(String, String)> = (0..*width)
                    .map(|tag| (format!("k{item}"), format!("v{tag}")))
                    .collect();
                QueryItem::tagged(
                    Tags::from_pairs(pairs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
                        .expect("the fixture's tags are well formed"),
                )
                .expect("an item carrying tags is constructible")
            }))
            .expect("a non-empty item list is a query")
        };

        assert_eq!(widths(0).iter().sum::<usize>(), budget);
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&of_widths(&widths(0))),
            1,
            "a plan of exactly {budget} parameters is one statement: the budget              is the largest a statement may carry, not the smallest it may not"
        );
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&of_widths(&widths(-1))),
            1,
            "one parameter under the budget is still one statement"
        );
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&of_widths(&widths(1))),
            2,
            "one parameter over the budget is two statements, and exactly two:              a partition that restarted its parameter count without restarting              its chunk would report more"
        );
    }

    /// The arm axis still binds where it is the tighter of the two.
    ///
    /// The regression this rejects is a partition that replaced one limit with
    /// the other rather than taking both: at one tag per item, 900 items is 900
    /// parameters — nowhere near the budget — and must still be three
    /// statements.
    #[test]
    fn the_arm_partition_still_binds_on_narrow_items() {
        let arms = CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
        let items = arms * 2 + 100;
        assert!(items < CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT);
        assert_eq!(
            CloudflareEventStore::planned_statement_count(&query_of(items, 1)),
            items.div_ceil(arms)
        );
    }
}
