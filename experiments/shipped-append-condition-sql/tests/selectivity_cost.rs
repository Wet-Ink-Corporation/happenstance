//! Findings I-5 and X-1, and neither of them needs a row of data.
//!
//! # I-5 — the quadratic that runs once per read page and once per append guard
//!
//! `query_sql::distinct_tags` deduplicates with `Vec::contains`, which is O(t²)
//! in `String` comparisons for one item's tags, and `Selectivity::read_for`
//! accumulates across items with the same pattern, which is O(T²) in the query's
//! total multi-tag tag count. Both run on **every** call, `read_for` is called
//! once per read page (`event_store.rs:1301`) and once per append guard
//! (`event_store.rs:645`), and the append one runs inside `BEGIN IMMEDIATE`.
//!
//! The shape is the specification's own floor, not a corner: **VT-23** requires
//! every store to evaluate at least 128 query items, nothing bounds tags per
//! query item, and `MAX_TAGS_PER_EVENT` is 128 — so 128 items × 128 tags is a
//! query a conformant caller may build, and it presents 16,384 tags.
//!
//! Three seams, and they say different things:
//!
//! * [`SqliteEventStore::planned_statement_count`] is **public and shipped**, not
//!   a transcription, and calls straight through `chunks` → `arms_sql` →
//!   `item_sql` → `distinct_tags`. It covers the per-item half.
//! * [`Selectivity::wanted_tags`] is this crate's transcription of `read_for`'s
//!   accumulation, lifted out so it can be timed with no database at all. It
//!   covers the cross-item half.
//! * [`Selectivity::wanted_tags_via_set`] is the **counterfactual**: the same
//!   accumulation with the linear scan replaced by an ordered set. The finding's
//!   remediation deserves a figure rather than an expectation.
//!
//! The `shared-tags` row is the control. Its 128 items name the *same* 128 tags,
//! so `wanted` stops growing after the first item and the cross-item quadratic
//! disappears — while the per-item one does not. Without that row a reader could
//! not tell which of the two loops the `distinct-tags` figure is about.
//!
//! # X-1 — chunked by item count, never by parameter count
//!
//! The same two call sites are where SQLite's *second* pushdown limit is
//! unbounded. `SQLITE_MAX_VARIABLE_NUMBER` is 32,766;
//! `MAX_QUERY_ARMS_PER_STATEMENT` bounds arms and nothing bounds parameters. Both
//! sites are shown here as a `prepare` that actually fails, with the parameter
//! count that did it — on the write path, inside the write lock, after the caller
//! has already made its decision.
//!
//! Run it with `cargo test --release --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test selectivity_cost --
//! --nocapture`.

mod support;

use std::time::Instant;

use happenstance_core::{Query, QueryItem, Tag, Tags};
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;
use shipped_append_condition_sql::chain::{Selectivity, Shape, chunks};
use shipped_append_condition_sql::probe_store;

/// `SQLITE_MAX_VARIABLE_NUMBER`, which SQLite has defaulted to since 3.32.
const SQLITE_MAX_VARIABLE_NUMBER: usize = 32_766;

/// VT-23's floor for query items.
const ITEMS: usize = 128;

/// `SqliteEventStore::MAX_TAGS_PER_EVENT`, used here as tags per query *item*
/// because nothing bounds that at all.
const TAGS: usize = 128;

/// Samples per figure.
const ROUNDS: usize = 25;

/// A query of `ITEMS` items each carrying `TAGS` tags.
///
/// When `distinct` the tags differ between items, which is what makes
/// `read_for`'s `wanted` accumulate; when not, every item names the same set and
/// the cross-item half collapses to a single pass.
fn wide_query(distinct: bool) -> Query {
    let built: Vec<QueryItem> = (0..ITEMS)
        .map(|item| {
            let prefix = if distinct { item } else { 0 };
            let set: Tags = (0..TAGS)
                .map(|tag| Tag::new(format!("k{prefix}x{tag}:v")).expect("a valid tag"))
                .collect();
            QueryItem::new(["Seeded"], set).expect("a valid query item")
        })
        .collect();
    Query::from_items(built).expect("a non-empty query")
}

/// The median of `rounds` samples of `body`, in microseconds.
fn median_us<T>(rounds: usize, mut body: impl FnMut() -> T) -> u128 {
    // Warm once: the allocation pattern is identical on every call, so a
    // first-call figure would be reporting the allocator's growth.
    drop(body());
    let mut samples = Vec::with_capacity(rounds);
    for _ in 0..rounds {
        let started = Instant::now();
        let value = body();
        samples.push(started.elapsed().as_micros());
        drop(value);
    }
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// An in-memory database carrying the two tables the query planning touches.
fn schema_only() -> Connection {
    let connection = Connection::open_in_memory().expect("an in-memory database");
    connection
        .execute_batch(probe_store::MIGRATION_1)
        .expect("the schema is applied");
    connection
}

#[test]
fn the_query_planning_quadratics_at_the_specifications_own_floor() {
    for (label, distinct) in [("distinct-tags", true), ("shared-tags", false)] {
        let query = wide_query(distinct);
        let total = Selectivity::wanted_tags(&query).len();

        let planned = median_us(ROUNDS, || {
            SqliteEventStore::planned_statement_count(&query)
        });
        let accumulation = median_us(ROUNDS, || Selectivity::wanted_tags(&query));
        let via_set = median_us(ROUNDS, || Selectivity::wanted_tags_via_set(&query));

        assert_eq!(
            Selectivity::wanted_tags(&query),
            Selectivity::wanted_tags_via_set(&query),
            "the counterfactual must produce identical output, or it is a \
             different function wearing the same name"
        );

        println!(
            "PLAN\titems={ITEMS}\ttags_per_item={TAGS}\tshape={label}\t\
             distinct_tags_in_query={total}\tplanned_statement_count_us_median={planned}\t\
             read_for_accumulation_us_median={accumulation}\t\
             read_for_via_btreeset_us_median={via_set}\trounds={ROUNDS}"
        );
    }
}

#[test]
fn the_selectivity_lookup_is_not_chunked_against_the_parameter_limit() {
    // X-1(a). 300 items × 128 distinct tags is 38,400 bound parameters in one
    // `tag IN (…)`, above SQLite's 32,766. `read_for` runs **inside**
    // `BEGIN IMMEDIATE` on the append path, so this arrives as
    // `AppendError::Store` with the write lock held and the caller's decision
    // already made.
    let connection = schema_only();
    let items = 300;
    let built: Vec<QueryItem> = (0..items)
        .map(|item| {
            let set: Tags = (0..TAGS)
                .map(|tag| Tag::new(format!("k{item}x{tag}:v")).expect("a valid tag"))
                .collect();
            QueryItem::new(["Seeded"], set).expect("a valid query item")
        })
        .collect();
    let query = Query::from_items(built).expect("a non-empty query");
    let parameters = Selectivity::wanted_tags(&query).len();
    assert!(parameters > SQLITE_MAX_VARIABLE_NUMBER);

    let outcome = Selectivity::read_for(&connection, &query);
    let message = match outcome {
        Ok(_) => "accepted".to_owned(),
        Err(err) => format!("{err}"),
    };
    println!(
        "PARAMLIMIT\tsite=Selectivity::read_for\titems={items}\ttags_per_item={TAGS}\t\
         parameters={parameters}\tlimit={SQLITE_MAX_VARIABLE_NUMBER}\toutcome={message}"
    );
}

#[test]
fn a_full_width_chunk_is_not_chunked_against_the_parameter_limit_either() {
    // X-1(b). `chunks` partitions by `items.chunks(MAX_QUERY_ARMS_PER_STATEMENT)`
    // — 400 — but `item_sql` binds one parameter per tag *and* one per type, so a
    // 400-item chunk carrying 128 tags each binds 400 × 129.
    let width = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
    let built: Vec<QueryItem> = (0..width)
        .map(|item| {
            let set: Tags = (0..TAGS)
                .map(|tag| Tag::new(format!("k{item}x{tag}:v")).expect("a valid tag"))
                .collect();
            QueryItem::new(["Seeded"], set).expect("a valid query item")
        })
        .collect();
    let query = Query::from_items(built).expect("a non-empty query");

    let plan = chunks(Shape::Chain, &query, &Selectivity::default(), width, 0);
    assert_eq!(plan.len(), 1, "one chunk, because it is bounded by item count");

    let (sql, params) = &plan[0];
    let connection = schema_only();
    let outcome = connection.prepare(sql);
    let message = match outcome {
        Ok(_) => "prepared".to_owned(),
        Err(err) => format!("{err}"),
    };
    println!(
        "PARAMLIMIT\tsite=query_sql::chunks\titems={width}\ttags_per_item={TAGS}\t\
         parameters={}\tlimit={SQLITE_MAX_VARIABLE_NUMBER}\tsql_bytes={}\toutcome={message}",
        params.len(),
        sql.len(),
    );

    // And the shipped seam agrees the plan is one statement, which is the half of
    // the claim that does not depend on this crate's transcription at all.
    assert_eq!(SqliteEventStore::planned_statement_count(&query), 1);
}
