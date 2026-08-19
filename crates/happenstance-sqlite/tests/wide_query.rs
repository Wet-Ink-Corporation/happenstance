//! A wide `Query` is chunked and merged, never refused.
//!
//! The conformance suite **cannot reach the chunk boundary on its own**: VT-23's
//! floor is 128 items, and at one tag per item that is nowhere near SQLite's
//! compound-`SELECT` ceiling. Ship the chunking with a width of 400 and every
//! rule in the suite runs one statement, the merge is dead code, and the crate
//! acquires a green suite over an unexercised path — the exact failure mode this
//! initiative exists to retire.
//!
//! So this target crosses the boundary deliberately and **observes that it was
//! crossed**, through `SqliteEventStore::planned_statement_count` — the same
//! function the read path plans with, so it cannot report a boundary the code
//! does not take.
//!
//! It is an **integration** target on purpose: it can only reach public API, so
//! anything it needs that is not public would fail to compile. The boundary
//! observation therefore had to be designed rather than smuggled in behind
//! `#[doc(hidden)]` or `cfg(test)`.
//!
//! **Both callers, not just `read`.** The same translation runs inside the
//! append transaction, where an `AppendCondition` guard asks the identical
//! question under the write lock. That path was *not* chunked until the slice
//! review found it, so a wide guard refused at the pushdown limit — the wrong
//! implementation, on the write path, where the contract has no way to report
//! it. `::a_wide_append_condition_guard_is_not_refused` and
//! `::a_wide_guard_answers_from_every_chunk_not_the_first` are the standing
//! guards.

#![cfg(feature = "event-store")]
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, MIN_SUPPORTED_QUERY_ITEMS, Query, QueryItem,
    ReadOptions, SequencePosition, SequencedEvent, Tags,
};
use happenstance_sqlite::event_store::SqliteEventStore;

/// Enough items that `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT) > 1`.
///
/// Derived from the crate's own width rather than written as a literal, so that
/// changing the width cannot silently stop this file crossing the boundary.
const WIDE: usize = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT * 2 + 100;

/// How many of the wide query's items actually match something.
///
/// Above `PAGE_SIZE` (512), so a full drain of a multi-chunk query also crosses
/// a **page** boundary — which is where the per-chunk resume state could leak
/// out of one hop and reintroduce the historical `resume_after` bug in a new
/// disguise.
const MATCHING: usize = 600;

#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-wide-{label}-{}-{ordinal}.db",
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

fn tags_of(pairs: &[(&str, &str)]) -> Tags {
    Tags::from_pairs(pairs.iter().copied()).unwrap()
}

fn tagged(event_type: &str, key: &str, value: &str) -> Event {
    Event::new(event_type, &b"{}"[..])
        .unwrap()
        .with_tags(tags_of(&[(key, value)]))
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

/// Seeds `count` events, each carrying the single tag `subject:sN`.
async fn seed(store: &SqliteEventStore, count: usize) -> Vec<SequencePosition> {
    let mut appended = 0;
    while appended < count {
        let size = (count - appended).min(SqliteEventStore::MAX_EVENTS_PER_BATCH);
        let batch: Vec<Event> = (appended..appended + size)
            .map(|index| tagged("Subject", "subject", &format!("s{index}")))
            .collect();
        store.append(&batch, None).await.unwrap();
        appended += size;
    }
    drain(store, &Query::all(), ReadOptions::new())
        .await
        .into_iter()
        .map(|event| event.position)
        .collect()
}

/// A query of `total` single-tag items, the first `matching` of which name a
/// seeded subject and the rest of which name nothing.
fn wide_query(total: usize, matching: usize) -> Query {
    let items: Vec<QueryItem> = (0..total)
        .map(|index| {
            let value = if index < matching {
                format!("s{index}")
            } else {
                format!("absent-{index}")
            };
            QueryItem::tagged(tags_of(&[("subject", &value)])).unwrap()
        })
        .collect();
    Query::from_items(items).unwrap()
}

// ---------------------------------------------------------------------------
// AC-001 — a wide query is served, and there is no refusal path to reach
// ---------------------------------------------------------------------------

/// AC-001 — the 128 items every store must evaluate, with the **only** matching
/// item last, so a store that served the first chunk and stopped would return
/// nothing.
#[tokio::test]
async fn a_query_at_the_guaranteed_minimum_item_count_is_served() {
    let db = TempDb::new("minimum");
    let store = db.open();
    store
        .append(&[tagged("Subject", "subject", "needle")], None)
        .await
        .unwrap();

    let mut items: Vec<QueryItem> = (0..MIN_SUPPORTED_QUERY_ITEMS - 1)
        .map(|index| {
            QueryItem::tagged(tags_of(&[("subject", &format!("absent-{index}"))])).unwrap()
        })
        .collect();
    items.push(QueryItem::tagged(tags_of(&[("subject", "needle")])).unwrap());
    let query = Query::from_items(items).unwrap();

    let seen = drain(&store, &query, ReadOptions::new()).await;
    assert_eq!(
        seen.len(),
        1,
        "a query at the guaranteed minimum item count must be served, not \
         refused and not silently truncated"
    );
}

/// AC-001 — and far above it, where more than one statement is genuinely needed.
#[tokio::test]
async fn a_query_far_above_the_minimum_is_served() {
    let db = TempDb::new("far-above");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;

    let seen = drain(&store, &wide_query(WIDE, MATCHING), ReadOptions::new()).await;
    assert_eq!(
        seen.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned
    );
}

/// AC-001 — and on the **write** path, where the same translation runs under
/// the write lock.
///
/// The read path and the append-condition guard ask one question — *which
/// positions match this query?* — and `query_sql`'s module doc advertises the
/// translation as shared by both. It was not: `read` chunked and `evaluate` did
/// not, so a guard carrying more than `MAX_QUERY_ARMS_PER_STATEMENT` arms failed
/// with SQLite's own *"too many terms in compound SELECT"* wrapped as
/// `AppendError::Store` — a refusal at the pushdown limit, which is AC-008's
/// named wrong implementation, arriving on the write path instead of the read
/// one. VT-23 forbids the refusal wherever it appears, and
/// `crates/happenstance-core/src/limits.rs:46-52` gives it no variant to be
/// reported through.
///
/// The conformance suite cannot reach this: `MIN_SUPPORTED_QUERY_ITEMS` is 128,
/// well below a chunk width of 400.
#[tokio::test]
async fn a_wide_append_condition_guard_is_not_refused() {
    let db = TempDb::new("guard-wide");
    let store = db.open();
    seed(&store, 3).await;

    // Every item names a subject nothing carries, so the guard is satisfied and
    // the append must land.
    let query = wide_query(WIDE, 0);
    assert!(
        SqliteEventStore::planned_statement_count(&query) > 1,
        "at one statement this would be testing the single-statement guard and \
         the write path's merge would be dead code"
    );

    store
        .append(
            &[tagged("Subject", "subject", "new")],
            Some(&AppendCondition::new(query)),
        )
        .await
        .expect("a wide guard is chunked and merged, never refused");
}

/// AC-001 / AC-003 — the guard's answer is the highest match across **every**
/// chunk, not the first chunk's.
///
/// A guard asks an inequality on the *highest* matching position, so chunking it
/// is exact only if the per-chunk `max(position)` results are merged by `max`.
/// This arranges the two matching items either side of the chunk partition and
/// sets the boundary at the **lower** one: an implementation that answered from
/// the first chunk alone would find nothing above the boundary and let the
/// append through, which is the silent wrong answer rather than the loud one.
#[tokio::test]
async fn a_wide_guard_answers_from_every_chunk_not_the_first() {
    let db = TempDb::new("guard-merge");
    let store = db.open();
    let assigned = seed(&store, 3).await;

    let mut items: Vec<QueryItem> = Vec::with_capacity(WIDE);
    items.push(QueryItem::tagged(tags_of(&[("subject", "s0")])).unwrap());
    for index in 1..WIDE - 1 {
        items.push(QueryItem::tagged(tags_of(&[("subject", &format!("absent-{index}"))])).unwrap());
    }
    items.push(QueryItem::tagged(tags_of(&[("subject", "s2")])).unwrap());
    let query = Query::from_items(items).unwrap();
    assert!(SqliteEventStore::planned_statement_count(&query) > 1);

    let refused = store
        .append(
            &[tagged("Subject", "subject", "new")],
            Some(&AppendCondition::new(query.clone()).after(assigned[0])),
        )
        .await;
    assert!(
        matches!(
            &refused,
            Err(AppendError::ConditionViolated(violated))
                if violated.conflicting_position == Some(assigned[2])
        ),
        "the guard must name the highest matching position across the whole \
         query, not the highest one the first statement happened to see; got \
         {refused:?}"
    );

    // The boundary still applies across the merge: at the highest match there is
    // nothing above it, so the same guard is satisfied.
    store
        .append(
            &[tagged("Subject", "subject", "new")],
            Some(&AppendCondition::new(query).after(assigned[2])),
        )
        .await
        .expect("nothing matches above the highest match, so the guard holds");
}

// ---------------------------------------------------------------------------
// AC-002 — the boundary is crossed, and observed to have been
// ---------------------------------------------------------------------------

/// AC-002 — the merge runs, and the test would fail if the read were served by a
/// single statement.
#[tokio::test]
async fn a_wide_query_actually_crosses_the_chunk_boundary() {
    let db = TempDb::new("boundary");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;

    let query = wide_query(WIDE, MATCHING);
    let statements = SqliteEventStore::planned_statement_count(&query);
    assert!(
        statements > 1,
        "this file exists to exercise the merge; at {statements} statement(s) it \
         would be testing the single-statement path and the merge would be dead \
         code behind a green suite"
    );
    assert_eq!(
        statements,
        WIDE.div_ceil(SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT),
        "the reported plan is ceil(arms / width)"
    );

    // A narrow query is still one statement, so the number means something.
    assert_eq!(SqliteEventStore::planned_statement_count(&Query::all()), 1);

    let seen = drain(&store, &query, ReadOptions::new()).await;
    assert_eq!(
        seen.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned,
        "and the merged answer is the whole union, across {statements} statements"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — the union, exactly once each, in position order
// ---------------------------------------------------------------------------

/// AC-003 — two items in different chunks that both match the same event yield
/// it once.
#[tokio::test]
async fn overlapping_arms_yield_each_event_once() {
    let db = TempDb::new("overlap");
    let store = db.open();
    let assigned = seed(&store, 3).await;

    // The same item repeated across the chunk boundary, plus filler.
    let mut items: Vec<QueryItem> = Vec::with_capacity(WIDE);
    for index in 0..WIDE {
        let value = if index % SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT < 3 {
            format!(
                "s{}",
                index % SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT
            )
        } else {
            format!("absent-{index}")
        };
        items.push(QueryItem::tagged(tags_of(&[("subject", &value)])).unwrap());
    }
    let query = Query::from_items(items).unwrap();
    assert!(SqliteEventStore::planned_statement_count(&query) > 1);

    let seen = drain(&store, &query, ReadOptions::new()).await;
    assert_eq!(
        seen.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned,
        "each event exactly once, ascending, regardless of how the items were \
         partitioned into chunks"
    );
}

/// AC-003 — the same items shuffled produce the identical output vector.
#[tokio::test]
async fn item_order_does_not_change_the_result_set() {
    let db = TempDb::new("order");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;

    let ordered = wide_query(WIDE, MATCHING);

    // A deterministic shuffle: reverse, which puts every matching item into a
    // different chunk from the one it was in.
    let mut items: Vec<QueryItem> = (0..WIDE)
        .map(|index| {
            let value = if index < MATCHING {
                format!("s{index}")
            } else {
                format!("absent-{index}")
            };
            QueryItem::tagged(tags_of(&[("subject", &value)])).unwrap()
        })
        .collect();
    items.reverse();
    let shuffled = Query::from_items(items).unwrap();

    let first = drain(&store, &ordered, ReadOptions::new()).await;
    let second = drain(&store, &shuffled, ReadOptions::new()).await;

    assert_eq!(
        first.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned
    );
    assert_eq!(
        second
            .iter()
            .map(|event| event.position)
            .collect::<Vec<_>>(),
        assigned
    );
}

/// AC-003 — the answer is every chunk's matches, not the first chunk's.
#[tokio::test]
async fn the_union_is_every_chunk_not_the_first() {
    let db = TempDb::new("every-chunk");
    let store = db.open();
    let assigned = seed(&store, 5).await;

    // Every matching item sits in the **last** chunk, so a store that served
    // only the first would return nothing at all.
    let mut items: Vec<QueryItem> = (0..WIDE - 5)
        .map(|index| {
            QueryItem::tagged(tags_of(&[("subject", &format!("absent-{index}"))])).unwrap()
        })
        .collect();
    for index in 0..5 {
        items.push(QueryItem::tagged(tags_of(&[("subject", &format!("s{index}"))])).unwrap());
    }
    let query = Query::from_items(items).unwrap();
    assert!(SqliteEventStore::planned_statement_count(&query) > 1);

    let seen = drain(&store, &query, ReadOptions::new()).await;
    assert_eq!(
        seen.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned
    );
}

/// AC-003 — two items whose type lists look alike but whose tags differ are not
/// collapsed.
#[tokio::test]
async fn items_with_similar_types_but_different_tags_are_not_collapsed() {
    let db = TempDb::new("not-collapsed");
    let store = db.open();
    store
        .append(
            &[
                tagged("Subject", "subject", "left"),
                tagged("Subject", "subject", "right"),
                tagged("Subject", "subject", "third"),
            ],
            None,
        )
        .await
        .unwrap();

    let mut items = vec![
        QueryItem::new(["Subject"], tags_of(&[("subject", "left")])).unwrap(),
        QueryItem::new(["Subject"], tags_of(&[("subject", "right")])).unwrap(),
    ];
    for index in 0..WIDE {
        items.push(
            QueryItem::new(
                ["Subject"],
                tags_of(&[("subject", &format!("absent-{index}"))]),
            )
            .unwrap(),
        );
    }
    let query = Query::from_items(items).unwrap();

    let seen = drain(&store, &query, ReadOptions::new()).await;
    assert_eq!(
        seen.len(),
        2,
        "same types, different tags: two distinct arms, and neither may be \
         dropped as a duplicate of the other"
    );
}

// ---------------------------------------------------------------------------
// AC-004 — `limit` applies across the merged output
// ---------------------------------------------------------------------------

/// AC-004 — `n` across the merge, never `n` per chunk.
#[tokio::test]
async fn limit_applies_across_chunks_not_per_chunk() {
    let db = TempDb::new("limit-chunks");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING);
    let statements = SqliteEventStore::planned_statement_count(&query);
    assert!(statements > 1);

    let limited = drain(&store, &query, ReadOptions::new().limit(10)).await;
    assert_eq!(
        limited.len(),
        10,
        "a per-chunk LIMIT would have returned up to {} events",
        10 * statements
    );
    assert_eq!(
        limited
            .iter()
            .map(|event| event.position)
            .collect::<Vec<_>>(),
        assigned[..10].to_vec()
    );
}

/// AC-004 — counted after filtering, forwards.
#[tokio::test]
async fn limit_applies_after_filtering_forwards() {
    let db = TempDb::new("limit-forwards");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;

    // Only the first 20 subjects are named, so a limit of 50 is bounded by the
    // matches rather than by the log.
    let query = wide_query(WIDE, 20);
    let seen = drain(&store, &query, ReadOptions::new().limit(50)).await;
    assert_eq!(seen.len(), 20);
    assert_eq!(
        seen.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned[..20].to_vec()
    );
}

/// AC-004 — and backwards, where a per-statement `LIMIT` would keep whichever
/// rows a chunk happened to hold rather than the highest matching positions.
#[tokio::test]
async fn limit_applies_after_filtering_backwards() {
    let db = TempDb::new("limit-backwards");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING);

    let newest = drain(&store, &query, ReadOptions::new().backwards().limit(10)).await;
    let mut expected = assigned[MATCHING - 10..].to_vec();
    expected.reverse();
    assert_eq!(
        newest
            .iter()
            .map(|event| event.position)
            .collect::<Vec<_>>(),
        expected,
        "a backwards limited read returns the highest matching positions"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — one ceiling over every chunk and every hop
// ---------------------------------------------------------------------------

/// AC-005 — an event appended after the read began is above the ceiling and
/// absent from the whole of it, however many statements it took.
#[tokio::test]
async fn a_concurrent_append_between_chunks_is_not_seen() {
    let db = TempDb::new("ceiling-chunks");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING + 1);

    let mut stream = Box::pin(store.read(&query, ReadOptions::new()));
    let mut observed = Vec::new();
    for _ in 0..100 {
        match next(&mut stream).await {
            Some(event) => observed.push(event.position),
            None => break,
        }
    }

    // A second real connection commits an event the query names.
    let second = db.open();
    second
        .append(
            &[tagged("Subject", "subject", &format!("s{MATCHING}"))],
            None,
        )
        .await
        .unwrap();

    while let Some(event) = next(&mut stream).await {
        observed.push(event.position);
    }

    assert_eq!(
        observed, assigned,
        "one query is one sample: every chunk statement of every hop carries the \
         same ceiling"
    );
}

/// AC-005 — the same claim across a **page** boundary rather than a chunk one.
#[tokio::test]
async fn a_concurrent_append_between_pages_is_not_seen() {
    let db = TempDb::new("ceiling-pages");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING + 1);

    let mut stream = Box::pin(store.read(&query, ReadOptions::new()));
    let mut observed = Vec::new();
    // Past the first page of 512, so the next fetch is a second hop.
    for _ in 0..520 {
        match next(&mut stream).await {
            Some(event) => observed.push(event.position),
            None => break,
        }
    }

    let second = db.open();
    second
        .append(
            &[tagged("Subject", "subject", &format!("s{MATCHING}"))],
            None,
        )
        .await
        .unwrap();

    while let Some(event) = next(&mut stream).await {
        observed.push(event.position);
    }
    assert_eq!(observed, assigned);
}

async fn next<S>(stream: &mut std::pin::Pin<Box<S>>) -> Option<SequencedEvent>
where
    S: Stream<
        Item = Result<SequencedEvent, happenstance_sqlite::event_store::SqliteEventStoreError>,
    >,
{
    core::future::poll_fn(|context| stream.as_mut().poll_next(context))
        .await
        .map(|item| item.unwrap())
}

// ---------------------------------------------------------------------------
// AC-006 — paging a wide query repeats no row and drops none
// ---------------------------------------------------------------------------

/// AC-006 — more than `PAGE_SIZE` matching events across a multi-chunk query.
///
/// The historical `resume_after` bug — an inclusive seed advanced by an
/// exclusive step, re-reading one row per page boundary — must not return in
/// per-chunk form: per-chunk state stays **inside** one hop, and `advance()`
/// folds only the merged page into `resume_from`.
#[tokio::test]
async fn paging_a_wide_query_repeats_no_row_and_drops_none() {
    let db = TempDb::new("paging");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    const { assert!(MATCHING > 512, "this criterion needs a page boundary") };

    let query = wide_query(WIDE, MATCHING);
    assert!(SqliteEventStore::planned_statement_count(&query) > 1);

    let seen: Vec<SequencePosition> = drain(&store, &query, ReadOptions::new())
        .await
        .into_iter()
        .map(|event| event.position)
        .collect();
    assert_eq!(seen, assigned);

    let mut deduplicated = seen.clone();
    deduplicated.dedup();
    assert_eq!(deduplicated.len(), seen.len(), "no row repeated");
}

/// AC-006 — and backwards.
#[tokio::test]
async fn paging_backwards_repeats_no_row_and_drops_none() {
    let db = TempDb::new("paging-backwards");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING);

    let seen: Vec<SequencePosition> = drain(&store, &query, ReadOptions::new().backwards())
        .await
        .into_iter()
        .map(|event| event.position)
        .collect();
    let mut expected = assigned;
    expected.reverse();
    assert_eq!(seen, expected);
}

// ---------------------------------------------------------------------------
// AC-008 — the constrained callers
// ---------------------------------------------------------------------------

/// AC-008 — `read` of a wide query still executes nothing off a runtime.
///
/// It cannot: `spawn_blocking` panics with no runtime, which is why the
/// absent-runtime case is an error item rather than a panic.
#[test]
fn read_of_a_wide_query_executes_nothing_off_runtime() {
    let db = TempDb::new("off-runtime");
    let store = db.open();

    let query = wide_query(WIDE, 4);
    let stream = store.read(&query, ReadOptions::new());
    drop(stream);
}

/// AC-008 — the stream is held across an await inside a real `tokio::spawn`,
/// which is the shape that would fail if it were not `Send`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_wide_read_streams_from_a_tokio_spawn() {
    let db = TempDb::new("spawned");
    let store = db.open();
    let assigned = seed(&store, MATCHING).await;
    let query = wide_query(WIDE, MATCHING);

    let handle = tokio::spawn(async move {
        let mut stream = Box::pin(store.read(&query, ReadOptions::new()));
        let mut out = Vec::new();
        while let Some(event) = next(&mut stream).await {
            out.push(event.position);
            // An await point with the stream alive across it.
            tokio::task::yield_now().await;
        }
        out
    });

    assert_eq!(handle.await.unwrap(), assigned);
}
