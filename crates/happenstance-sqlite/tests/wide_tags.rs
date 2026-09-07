//! A query is partitioned by **bound parameters** as well as by arms.
//!
//! `tests/wide_query.rs` crosses the *arm* axis — `SQLITE_MAX_COMPOUND_SELECT`,
//! 500 terms, guarded by `MAX_QUERY_ARMS_PER_STATEMENT` — and its own module doc
//! records that the conformance suite cannot reach even that, because VT-23's
//! floor is 128 items at one tag each. This target crosses the *other* axis, the
//! one nothing in the crate bounded: `SQLITE_MAX_VARIABLE_NUMBER`, 32,766 bound
//! parameters, of which `query_sql::item_sql` pushes one per tag and one per
//! type of every item in the chunk.
//!
//! The two axes are independent, and a partition on one is not a partition on
//! the other. 400 items is `MAX_QUERY_ARMS_PER_STATEMENT` exactly — one chunk,
//! by the crate's own arithmetic — and at `MAX_TAGS_PER_EVENT` tags per item
//! that one chunk binds 51,200 parameters. The wrong implementation VT-23 names
//! in terms is *"an adapter that generates one SQL parameter per item and
//! silently fails past a driver limit"*, and it is reached here at the adapter's
//! own documented chunk width carrying the adapter's own documented maximum tag
//! count.
//!
//! **Three observable outcomes, and they are not one outcome three times.**
//!
//! * `query_sql::chunks` partitions on arms alone, so a 400-item query of
//!   128-tag items is planned as one unpreparable statement.
//! * `Selectivity::read_for` is not partitioned at all: it accumulates every
//!   distinct tag of every multi-tag item across the *whole* query and binds
//!   them in a single `IN (…)`. It needs no wide item to fail — 16,500 items of
//!   two tags each is 33,000 parameters — and it fails **first**, because it
//!   runs before `chunks` on both callers.
//! * `SqliteEventStore::planned_statement_count` is public and reports that
//!   unpreparable plan as `1`. A caller sizing a query against it is told the
//!   plan is fine.
//!
//! **Both callers, deliberately.** The read path plans at
//! `event_store.rs::fetch_page`; the append-condition guard plans at
//! `event_store.rs::evaluate`, *inside* `BEGIN IMMEDIATE` with the write lock
//! held and the caller's decision already made. VT-24 rejects that timing by
//! name — *"discovers `SQLITE_MAX_VARIABLE_NUMBER` at write time"* — and
//! `crates/happenstance-core/src/limits.rs:46-52` gives the resulting refusal no
//! variant to travel in, so it arrives as `AppendError::Store` wrapping a raw
//! driver string.
//!
//! Every shape below is one a conformant caller may construct: nothing in
//! `happenstance-core`'s `query.rs` bounds tags per query item, and 128 is this
//! store's own `MAX_TAGS_PER_EVENT`.

#![cfg(feature = "event-store")]
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, Event, EventStore, Query, QueryItem, ReadOptions, SequencedEvent, Tags,
};
use happenstance_sqlite::event_store::SqliteEventStore;

/// Tags every item of the wide-tag query shares.
///
/// Shared rather than distinct so that `Selectivity::read_for`'s accumulation
/// stays small — 127 shared plus one unique per item is 527 distinct tags for
/// the whole query — and the failure this fixture reaches is `chunks`' and only
/// `chunks`'. The two sites are tested apart because they fail apart.
const SHARED_TAGS: usize = SqliteEventStore::MAX_TAGS_PER_EVENT - 1;

/// Items in the wide-tag query: the crate's own chunk width, exactly.
///
/// At this many items the arm axis says *one statement*, which is the point.
const WIDE_TAG_ITEMS: usize = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT;

/// Two-tag items enough that their distinct tags exceed `SQLITE_MAX_VARIABLE_NUMBER`.
///
/// 16,500 × 2 = 33,000 against 32,766. Every item is narrow, so `chunks`
/// partitions this shape correctly on the arm axis and `read_for` is the only
/// site that can fail.
const NARROW_ITEMS: usize = 16_500;

#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-wide-tags-{label}-{}-{ordinal}.db",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn open(&self) -> SqliteEventStore {
        SqliteEventStore::open(self.path()).unwrap()
    }
}

impl Drop for TempDb {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let mut path = self.0.clone().into_os_string();
            path.push(suffix);
            let _ = std::fs::remove_file(PathBuf::from(path));
        }
    }
}

async fn drain<S: EventStore>(store: &S, query: &Query, options: ReadOptions) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    let mut stream = Box::pin(store.read(query, options));
    let mut out = Vec::new();
    while let Some(item) = core::future::poll_fn(|context| stream.as_mut().poll_next(context)).await
    {
        out.push(item.expect("the read stream failed"));
    }
    out
}

/// The 127 tags every wide item carries, as owned pairs.
fn shared_pairs() -> Vec<(String, String)> {
    (0..SHARED_TAGS)
        .map(|n| ("shared".to_owned(), format!("s{n}")))
        .collect()
}

fn tags_from(pairs: &[(String, String)]) -> Tags {
    Tags::from_pairs(
        pairs
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str())),
    )
    .unwrap()
}

/// An item carrying `MAX_TAGS_PER_EVENT` tags: the 127 shared, plus `unique:u<index>`.
fn wide_tag_item(index: usize) -> QueryItem {
    let mut pairs = shared_pairs();
    pairs.push(("unique".to_owned(), format!("u{index}")));
    QueryItem::tagged(tags_from(&pairs)).unwrap()
}

/// `WIDE_TAG_ITEMS` items of `MAX_TAGS_PER_EVENT` tags each.
fn wide_tag_query() -> Query {
    Query::from_items((0..WIDE_TAG_ITEMS).map(wide_tag_item)).unwrap()
}

/// An event carrying the shared tags plus `unique:u<index>` — so it matches
/// exactly `wide_tag_item(index)` and no other item of the wide-tag query.
fn wide_tag_event(index: usize) -> Event {
    let mut pairs = shared_pairs();
    pairs.push(("unique".to_owned(), format!("u{index}")));
    Event::new("Subject", &b"{}"[..])
        .unwrap()
        .with_tags(tags_from(&pairs))
}

/// A query whose items are narrow but whose *distinct* tags are not.
fn many_distinct_tags_query() -> Query {
    Query::from_items((0..NARROW_ITEMS).map(|index| {
        let left = format!("a{index}");
        let right = format!("b{index}");
        QueryItem::tagged(
            Tags::from_pairs([("left", left.as_str()), ("right", right.as_str())]).unwrap(),
        )
        .unwrap()
    }))
    .unwrap()
}

// ---------------------------------------------------------------------------
// The `chunks` partition — one statement per 400 arms, however wide the arms
// ---------------------------------------------------------------------------

/// The public seam reports the plan honestly.
///
/// `planned_statement_count` exists so that a test can observe the boundary
/// rather than guess at it, and its own doc calls it *"the same call the read
/// path makes"*. That is true and insufficient: the call it makes partitions on
/// arms alone, so for this query both agree on `1` and the statement neither of
/// them can prepare is counted as a plan that will run.
#[test]
fn planned_statement_count_does_not_report_an_unpreparable_plan_as_one() {
    let query = wide_tag_query();
    let parameters = WIDE_TAG_ITEMS * SqliteEventStore::MAX_TAGS_PER_EVENT;
    assert!(
        parameters > 32_766,
        "the fixture must exceed SQLITE_MAX_VARIABLE_NUMBER or it proves nothing"
    );
    assert!(
        SqliteEventStore::planned_statement_count(&query) > 1,
        "a plan of {parameters} bound parameters cannot be one prepared \
         statement; reporting it as one tells a caller sizing its query against \
         this seam that the plan is fine"
    );
}

/// The read path serves it, and serves it from every chunk.
///
/// The two matching items sit at opposite ends of the item list, so a store that
/// partitioned by parameters and then answered from the first chunk alone would
/// return one event rather than two — which is the silent-truncation half of
/// VT-23's named wrong implementation, as distinct from the refusal half.
#[tokio::test]
async fn a_read_of_wide_items_is_served_from_every_chunk() {
    let db = TempDb::new("read-wide-items");
    let store = db.open();
    store
        .append(
            &[wide_tag_event(0), wide_tag_event(WIDE_TAG_ITEMS - 1)],
            None,
        )
        .await
        .unwrap();

    let seen = drain(&store, &wide_tag_query(), ReadOptions::new()).await;
    assert_eq!(
        seen.len(),
        2,
        "both matching events must be returned: one lies in the first parameter \
         chunk and one in the last"
    );
}

/// And the append path, where the failure arrives under the write lock.
///
/// Every item names a `unique` tag no event carries, so the guard is satisfied
/// and the append must land. What fails today is not the guard's answer — it is
/// `prepare`, with SQLite's own *"too many SQL variables"*, wrapped as
/// `AppendError::Store` inside `BEGIN IMMEDIATE`.
#[tokio::test]
async fn a_guard_of_wide_items_is_not_refused() {
    let db = TempDb::new("guard-wide-items");
    let store = db.open();
    store
        .append(&[Event::new("Seed", &b"{}"[..]).unwrap()], None)
        .await
        .unwrap();

    store
        .append(
            &[Event::new("Subject", &b"{}"[..]).unwrap()],
            Some(&AppendCondition::new(wide_tag_query())),
        )
        .await
        .expect("a guard wide in tags is chunked and merged, never refused");
}

// ---------------------------------------------------------------------------
// The `Selectivity::read_for` partition — the site with no width at all
// ---------------------------------------------------------------------------

/// The selectivity lookup is partitioned too.
///
/// It has no `max_arms` to tune because it has no concept of a chunk: one
/// `IN (…)` over every distinct tag of every multi-tag item in the query. It is
/// reached before `chunks` on both callers, so it is the site that fails first,
/// and no item here is wide — this is a query of ordinary two-tag items.
#[tokio::test]
async fn a_read_whose_distinct_tags_exceed_the_variable_limit_is_served() {
    let db = TempDb::new("read-many-tags");
    let store = db.open();
    let matching = Event::new("Subject", &b"{}"[..])
        .unwrap()
        .with_tags(Tags::from_pairs([("left", "a7"), ("right", "b7")]).unwrap());
    store.append(&[matching], None).await.unwrap();

    let seen = drain(&store, &many_distinct_tags_query(), ReadOptions::new()).await;
    assert_eq!(
        seen.len(),
        1,
        "a query whose distinct tags exceed SQLITE_MAX_VARIABLE_NUMBER must \
         still be evaluated; the selectivity lookup is a chunkable statement \
         like any other"
    );
}

// ---------------------------------------------------------------------------
// The partition boundary itself — one parameter under, exactly at, one over
// ---------------------------------------------------------------------------

/// A query of `items` items whose tag counts are `widths`, item *i* carrying
/// `widths[i]` distinct tags.
///
/// Every tag is unique to its item and its position, so the parameter cost of
/// the query is exactly `widths.iter().sum()` and the arm count is
/// `widths.len()`.
fn query_of_widths(widths: &[usize]) -> Query {
    Query::from_items(widths.iter().enumerate().map(|(item, width)| {
        let pairs: Vec<(String, String)> = (0..*width)
            .map(|tag| (format!("k{item}"), format!("v{tag}")))
            .collect();
        QueryItem::tagged(tags_from(&pairs)).unwrap()
    }))
    .unwrap()
}

/// Off-by-one at a partition boundary is the defect this class of fix
/// reintroduces, so the boundary is asserted from below, on it, and above it.
///
/// The arm axis is deliberately held slack — every case is
/// `MAX_QUERY_ARMS_PER_STATEMENT` items, never more — so that what moves the
/// answer is the parameter count and nothing else. A ceiling is a promise about
/// the statement that *is* issued: at exactly the budget the plan is one
/// statement, and one parameter over it is two.
#[test]
fn the_parameter_partition_splits_one_parameter_over_the_budget_and_not_before() {
    let arms = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
    let budget = SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT;
    // What the *statement* binds beside the items' own tags, and it is not
    // zero: `planned_statement_count` counts the read path, which puts the
    // window's two ends on every arm and the page budget once on the compound.
    // A case that means to sit exactly on the ceiling has to leave room for
    // that, and saying so here is the point — the number this test pins is a
    // promise about the statement that is issued, not about the query that
    // asked for it.
    let overhead = 2 * arms + 1;
    let items = budget - overhead;
    // A flat width plus a remainder on the last item, so the total is exact.
    let flat = items / arms;
    let mut widths = vec![flat; arms];
    widths[arms - 1] += items - flat * arms;
    assert_eq!(widths.iter().sum::<usize>() + overhead, budget);

    let at = query_of_widths(&widths);
    assert_eq!(
        SqliteEventStore::planned_statement_count(&at),
        1,
        "a plan of exactly {budget} parameters is one statement: the budget is \
         the largest a statement may carry, not the smallest it may not"
    );

    let mut under = widths.clone();
    under[0] -= 1;
    assert_eq!(
        SqliteEventStore::planned_statement_count(&query_of_widths(&under)),
        1,
        "one parameter under the budget is still one statement"
    );

    let mut over = widths.clone();
    over[0] += 1;
    assert_eq!(
        SqliteEventStore::planned_statement_count(&query_of_widths(&over)),
        2,
        "one parameter over the budget is two statements, and exactly two: a \
         partition that restarted its parameter count without restarting its \
         chunk would report more"
    );
}

/// The arm axis still binds where it is the tighter of the two.
///
/// The regression this rejects is a partition that replaced one limit with the
/// other rather than taking both: at one tag per item, 900 items is 900
/// parameters — nowhere near the budget — and must still be three statements.
#[test]
fn the_arm_partition_still_binds_on_narrow_items() {
    let arms = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
    let items = arms * 2 + 100;
    let query = query_of_widths(&vec![1; items]);
    assert!(
        items < SqliteEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT,
        "the fixture must sit below the parameter budget or it proves nothing"
    );
    assert_eq!(
        SqliteEventStore::planned_statement_count(&query),
        items.div_ceil(arms)
    );
}

/// And on the append path, inside the write transaction.
#[tokio::test]
async fn a_guard_whose_distinct_tags_exceed_the_variable_limit_is_not_refused() {
    let db = TempDb::new("guard-many-tags");
    let store = db.open();
    store
        .append(&[Event::new("Seed", &b"{}"[..]).unwrap()], None)
        .await
        .unwrap();

    store
        .append(
            &[Event::new("Subject", &b"{}"[..]).unwrap()],
            Some(&AppendCondition::new(many_distinct_tags_query())),
        )
        .await
        .expect("the selectivity lookup must not refuse a guard under the write lock");
}
