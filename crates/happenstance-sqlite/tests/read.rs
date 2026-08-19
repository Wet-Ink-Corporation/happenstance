//! The read path: lazy, paged, and bounded by one snapshot ceiling.
//!
//! This target asks three things the conformance suite cannot ask, because the
//! suite has no way to know how many statements a `read` issued: whether a drain
//! crosses more than two **pages** correctly, whether the ceiling survives a
//! write from a genuinely separate `rusqlite::Connection` half way through, and
//! whether `read` itself is inert outside a runtime.
//!
//! Everything is seeded through the adapter's own `append` — the same row codec
//! the decode half reads back — on a real file. Nothing is doubled, and there is
//! no `cfg(test)` page-size knob: the multi-page criteria seed strictly more than
//! `2 × PAGE_SIZE` rows so that at least three page statements run, because a
//! knob would leave the shipped paging path untested.

#![cfg(feature = "event-store")]
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll, Waker};

use futures_core::Stream;
use happenstance_core::{
    Event, EventStore, Query, QueryItem, ReadOptions, SequencePosition, SequencedEvent, Tags,
};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use rusqlite::Connection;

/// Strictly more than two pages, so at least three page statements run.
///
/// `PAGE_SIZE` is 512 and deliberately private; this is the number of events a
/// multi-page criterion seeds, and it is above `2 × 512` on purpose.
const PAST_TWO_PAGES: usize = 1_300;

/// A temporary database path that deletes itself, and its WAL sidecars, on drop.
#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-read-{label}-{}-{ordinal}.db",
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

    fn raw(&self) -> Connection {
        Connection::open(self.path()).unwrap()
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

fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).unwrap()
}

fn tags(pairs: &[(&str, &str)]) -> Tags {
    Tags::from_pairs(pairs.iter().copied()).unwrap()
}

fn tagged(event_type: &str, pairs: &[(&str, &str)]) -> Event {
    event(event_type).with_tags(tags(pairs))
}

fn query_of_types(types: &[&str]) -> Query {
    Query::from_item(QueryItem::of_types(types.iter().copied()).unwrap())
}

fn query_tagged(pairs: &[(&str, &str)]) -> Query {
    Query::from_item(QueryItem::tagged(tags(pairs)).unwrap())
}

/// Drains a stream to a vector.
///
/// Bound on the weaker `EventStore`, per the workspace's constraint 4: it
/// accepts both flavours, and only one of the two names may be in scope here.
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

/// Appends `count` events of `event_type` in ceiling-sized batches, returning
/// every position the store actually assigned.
async fn seed(store: &SqliteEventStore, event_type: &str, count: usize) -> Vec<SequencePosition> {
    let mut positions = Vec::with_capacity(count);
    let mut remaining = count;
    while remaining > 0 {
        let batch_size = remaining.min(SqliteEventStore::MAX_EVENTS_PER_BATCH);
        let batch: Vec<Event> = (0..batch_size).map(|_| event(event_type)).collect();
        store.append(&batch, None).await.unwrap();
        remaining -= batch_size;
    }
    let drained = drain(store, &Query::all(), ReadOptions::new()).await;
    positions.extend(drained.iter().map(|event| event.position));
    positions
}

// ---------------------------------------------------------------------------
// AC-001 — `read` executes nothing, and a missing runtime is an item
// ---------------------------------------------------------------------------

/// AC-001 — building the stream takes no lock and runs no SQL.
///
/// The proof is that the **same store** can append while the un-polled stream is
/// alive: if `read` had taken the connection mutex and parked the guard in the
/// cursor, this would deadlock rather than fail.
#[tokio::test]
async fn read_executes_nothing_until_polled() {
    let db = TempDb::new("inert");
    let store = db.open();
    store.append(&[event("Seed")], None).await.unwrap();

    let everything = Query::all();
    let stream = store.read(&everything, ReadOptions::new());

    store.append(&[event("WhileOpen")], None).await.unwrap();

    drop(stream);
    let seen = drain(&store, &Query::all(), ReadOptions::new()).await;
    assert_eq!(seen.len(), 2);
}

/// AC-001 — a store constructed **and** polled with no runtime anywhere reports
/// the absence as one `Err` item rather than panicking.
///
/// This is what keeps `SqliteEventStoreError::NoRuntime` meaningful under the
/// option ADR-0022 §9 chose: the store prefers a `Handle` captured at
/// construction, so the variant is reachable only when there was no runtime at
/// construction *or* at poll — which is exactly the state this test builds.
#[test]
fn read_polled_outside_a_runtime_yields_an_error_item() {
    let db = TempDb::new("no-runtime");
    let store = db.open();

    let everything = Query::all();
    let mut stream = store.read(&everything, ReadOptions::new());
    let mut context = Context::from_waker(Waker::noop());

    match Pin::new(&mut stream).poll_next(&mut context) {
        Poll::Ready(Some(Err(SqliteEventStoreError::NoRuntime(_)))) => {}
        other => panic!(
            "a poll with no runtime anywhere must arrive as one NoRuntime item, \
             never as a panic; got {other:?}"
        ),
    }
}

// ---------------------------------------------------------------------------
// AC-002 — a multi-page drain repeats and drops nothing
// ---------------------------------------------------------------------------

/// AC-002 — more than `2 × PAGE_SIZE` events, so at least three page statements
/// run, in both directions.
///
/// The historical bug this rejects is precise and quiet: `resume_from` seeded
/// from an inclusive bound and advanced by an exclusive step re-reads one row at
/// every page boundary. Nothing under `PAGE_SIZE` events can see it.
#[tokio::test]
async fn a_multi_page_drain_repeats_and_drops_nothing() {
    let db = TempDb::new("multi-page");
    let store = db.open();
    let assigned = seed(&store, "Paged", PAST_TWO_PAGES).await;
    assert_eq!(assigned.len(), PAST_TWO_PAGES);

    let forwards: Vec<SequencePosition> = drain(&store, &Query::all(), ReadOptions::new())
        .await
        .into_iter()
        .map(|event| event.position)
        .collect();
    assert_eq!(
        forwards, assigned,
        "every matching event exactly once, in ascending position order — \
         compared against the positions the store actually assigned, never \
         against literals"
    );

    let backwards: Vec<SequencePosition> =
        drain(&store, &Query::all(), ReadOptions::new().backwards())
            .await
            .into_iter()
            .map(|event| event.position)
            .collect();
    let mut expected = assigned.clone();
    expected.reverse();
    assert_eq!(backwards, expected);
}

// ---------------------------------------------------------------------------
// AC-003 — the ceiling is a ceiling
// ---------------------------------------------------------------------------

/// AC-003 — a genuinely separate connection appends while the stream is half
/// drained, and none of it appears.
///
/// A paged read with no ceiling **is** the torn log the application author's
/// stated fear names, and the consequence is not a wrong read: it is an accepted
/// append that should have been rejected, because the caller derives its
/// condition's boundary from the maximum position the read observed.
#[tokio::test]
async fn a_concurrent_append_mid_drain_is_not_observed() {
    let db = TempDb::new("ceiling");
    let store = db.open();
    let before = seed(&store, "Original", PAST_TWO_PAGES).await;

    let everything = Query::all();
    let mut stream = Box::pin(store.read(&everything, ReadOptions::new()));

    // Drain one page's worth, which forces the ceiling to be captured.
    let mut observed = Vec::new();
    for _ in 0..600 {
        match next(&mut stream).await {
            Some(event) => observed.push(event.position),
            None => break,
        }
    }
    assert!(!observed.is_empty());

    // A second handle — a second real connection onto the same file — commits.
    let second = db.open();
    second
        .append(&[event("Interloper"), event("Interloper")], None)
        .await
        .unwrap();

    while let Some(event) = next(&mut stream).await {
        observed.push(event.position);
    }

    assert_eq!(
        observed, before,
        "the half-drained read must observe exactly what was there when it \
         captured its ceiling, and nothing committed after it"
    );

    // And a fresh read does see them, so the ceiling bounded this read rather
    // than the store.
    let after = drain(&store, &Query::all(), ReadOptions::new()).await;
    assert_eq!(after.len(), before.len() + 2);
}

/// AC-003, at the sharpest point: **one** poll, then an append.
///
/// ES-11 says the sample is taken *no later than the first poll*, and this is
/// the shape that makes "no later than" mean something. A store that defers the
/// sample into its own `spawn_blocking` hop takes it **after** the first poll
/// returns — so an append that lands in between is below the ceiling and the
/// read observes it.
///
/// This adapter did exactly that, and failed `read_result_is_stable_under_concurrent_append`
/// and `query_items_share_one_snapshot` roughly one conformance run in two until
/// the sample moved onto the polling thread. Drained-then-appended tests cannot
/// see it — by the time they append, the sample is long taken — which is why
/// this one is written separately and polls exactly once.
#[tokio::test]
async fn an_append_after_a_single_poll_is_not_observed() {
    let db = TempDb::new("first-poll");
    let store = db.open();
    let before = seed(&store, "Seeded", 3).await;

    let everything = Query::all();
    let mut stream = Box::pin(store.read(&everything, ReadOptions::new()));

    // Exactly one poll. `Pending` is a legal answer and is not a violation.
    let mut observed = Vec::new();
    let first =
        core::future::poll_fn(|context| Poll::Ready(stream.as_mut().poll_next(context))).await;
    if let Poll::Ready(Some(item)) = first {
        observed.push(item.unwrap().position);
    }

    store.append(&[event("Later")], None).await.unwrap();

    while let Some(event) = next(&mut stream).await {
        observed.push(event.position);
    }

    assert_eq!(
        observed, before,
        "the sample is taken no later than the FIRST POLL: an append that lands \
         after that poll returned must be above the ceiling, however little of \
         the stream had been drained"
    );
}

/// One item from a pinned stream.
async fn next<S>(stream: &mut Pin<Box<S>>) -> Option<SequencedEvent>
where
    S: Stream<Item = Result<SequencedEvent, SqliteEventStoreError>>,
{
    core::future::poll_fn(|context| stream.as_mut().poll_next(context))
        .await
        .map(|item| item.unwrap())
}

// ---------------------------------------------------------------------------
// AC-004 — every item of one query shares that one ceiling
// ---------------------------------------------------------------------------

/// AC-004 — a two-item query drained across a page boundary while an event
/// matching only one item lands: the read is all-or-nothing on it, and the
/// answer is nothing.
#[tokio::test]
async fn query_items_share_the_one_ceiling() {
    let db = TempDb::new("shared-ceiling");
    let store = db.open();

    for _ in 0..700 {
        store
            .append(&[tagged("Left", &[("side", "l")])], None)
            .await
            .unwrap();
    }
    for _ in 0..700 {
        store
            .append(&[tagged("Right", &[("side", "r")])], None)
            .await
            .unwrap();
    }

    let query = Query::from_items([
        QueryItem::tagged(tags(&[("side", "l")])).unwrap(),
        QueryItem::tagged(tags(&[("side", "r")])).unwrap(),
    ])
    .unwrap();

    let mut stream = Box::pin(store.read(&query, ReadOptions::new()));
    let mut observed = Vec::new();
    for _ in 0..600 {
        match next(&mut stream).await {
            Some(event) => observed.push(event.position),
            None => break,
        }
    }

    let second = db.open();
    second
        .append(&[tagged("Right", &[("side", "r")])], None)
        .await
        .unwrap();

    while let Some(event) = next(&mut stream).await {
        observed.push(event.position);
    }

    assert_eq!(
        observed.len(),
        1_400,
        "the second item must not pick up an event the first item could not \
         have seen: one query is one sample, and the ceiling is the mechanism"
    );
}

// ---------------------------------------------------------------------------
// AC-005 — the bounds compose in both directions
// ---------------------------------------------------------------------------

/// AC-005 — `from` and `to` are inclusive in both directions, `from` stays the
/// *starting* bound under `backwards`, and the ceiling bounds the starting end
/// rather than replacing `to`.
///
/// The wrong shape this rejects is ES-16's named one: `WHERE position <= ?`
/// copied verbatim into the descending branch, which is correct forwards, passes
/// two of the three rules, and returns the oldest events where the newest were
/// asked for.
#[tokio::test]
async fn bounds_compose_in_both_directions() {
    let db = TempDb::new("bounds");
    let store = db.open();
    let assigned = seed(&store, "Bounded", 12).await;

    let low = assigned[3];
    let high = assigned[8];

    let window: Vec<SequencePosition> =
        drain(&store, &Query::all(), ReadOptions::new().from(low).to(high))
            .await
            .into_iter()
            .map(|event| event.position)
            .collect();
    assert_eq!(
        window,
        assigned[3..=8].to_vec(),
        "a closed forward window returns its endpoints and everything between"
    );

    let backwards: Vec<SequencePosition> = drain(
        &store,
        &Query::all(),
        ReadOptions::new().backwards().from(high).to(low),
    )
    .await
    .into_iter()
    .map(|event| event.position)
    .collect();
    let mut expected = assigned[3..=8].to_vec();
    expected.reverse();
    assert_eq!(
        backwards, expected,
        "under backwards, `from` is the higher starting bound and `to` the \
         lower stopping one — they swap roles in position order, not in meaning"
    );

    // A `to` above everything the store holds is narrowed to the ceiling rather
    // than ignored: the read still ends at the highest visible position.
    let beyond = SequencePosition::new(assigned[11].get() + 10_000).unwrap();
    let clamped: Vec<SequencePosition> =
        drain(&store, &Query::all(), ReadOptions::new().to(beyond))
            .await
            .into_iter()
            .map(|event| event.position)
            .collect();
    assert_eq!(clamped, assigned);

    // And a backwards read whose `from` is above the ceiling starts at the
    // ceiling rather than yielding nothing.
    let from_beyond: Vec<SequencePosition> = drain(
        &store,
        &Query::all(),
        ReadOptions::new().backwards().from(beyond),
    )
    .await
    .into_iter()
    .map(|event| event.position)
    .collect();
    let mut newest_first = assigned.clone();
    newest_first.reverse();
    assert_eq!(from_beyond, newest_first);
}

// ---------------------------------------------------------------------------
// AC-006 — the empty store, and a gap
// ---------------------------------------------------------------------------

/// AC-006 — the state every adapter is in before it works.
///
/// The registered defect this rejects is a ceiling computed as *arithmetic* on a
/// `None` head, which errors or panics on a store whose only fault is being new.
#[tokio::test]
async fn an_empty_store_yields_nothing_and_does_not_error() {
    let db = TempDb::new("empty");
    let store = db.open();

    let seen = drain(&store, &Query::all(), ReadOptions::new()).await;
    assert!(seen.is_empty());

    let backwards = drain(&store, &Query::all(), ReadOptions::new().backwards()).await;
    assert!(backwards.is_empty());

    assert_eq!(store.head().await.unwrap(), None);
}

/// AC-006 — `from` is a threshold, not a seek: a read from a position nothing
/// occupies yields the next matching event above it.
///
/// The gap is a real one, made by deleting a row — which `AUTOINCREMENT`
/// guarantees does not reuse the position afterwards.
#[tokio::test]
async fn read_from_a_gap_position_yields_the_next_event() {
    let db = TempDb::new("gap");
    let store = db.open();
    let assigned = seed(&store, "Gapped", 6).await;

    let missing = assigned[2];
    {
        let raw = db.raw();
        raw.execute(
            "DELETE FROM event_tag WHERE position = ?",
            [i64::try_from(missing.get()).unwrap()],
        )
        .unwrap();
        raw.execute(
            "DELETE FROM event WHERE position = ?",
            [i64::try_from(missing.get()).unwrap()],
        )
        .unwrap();
    }

    let forwards: Vec<SequencePosition> =
        drain(&store, &Query::all(), ReadOptions::new().from(missing))
            .await
            .into_iter()
            .map(|event| event.position)
            .collect();
    assert_eq!(
        forwards,
        assigned[3..].to_vec(),
        "a read from an unoccupied position yields the next matching event \
         above it, rather than erroring or coming back empty"
    );

    let backwards: Vec<SequencePosition> = drain(
        &store,
        &Query::all(),
        ReadOptions::new().backwards().from(missing),
    )
    .await
    .into_iter()
    .map(|event| event.position)
    .collect();
    let mut below = assigned[..2].to_vec();
    below.reverse();
    assert_eq!(backwards, below);
}

// ---------------------------------------------------------------------------
// AC-007 — `limit` is a whole-read budget
// ---------------------------------------------------------------------------

/// AC-007 — counted after filtering and across every item, never per page and
/// never per item; `Some(0)` yields nothing.
#[tokio::test]
async fn limit_is_a_whole_read_budget() {
    let db = TempDb::new("limit");
    let store = db.open();
    let assigned = seed(&store, "Limited", PAST_TWO_PAGES).await;

    // Smaller than one page.
    let few = drain(&store, &Query::all(), ReadOptions::new().limit(7)).await;
    assert_eq!(few.len(), 7);
    assert_eq!(
        few.iter().map(|event| event.position).collect::<Vec<_>>(),
        assigned[..7].to_vec()
    );

    // Spanning several page boundaries: the per-page budget is an
    // implementation detail beneath this number.
    let across = drain(&store, &Query::all(), ReadOptions::new().limit(1_100)).await;
    assert_eq!(across.len(), 1_100);

    // Larger than the log.
    let all = drain(
        &store,
        &Query::all(),
        ReadOptions::new().limit(PAST_TWO_PAGES * 2),
    )
    .await;
    assert_eq!(all.len(), PAST_TWO_PAGES);

    // Zero yields nothing — a deliberate divergence from the reference
    // implementation's JavaScript falsiness.
    let none = drain(&store, &Query::all(), ReadOptions::new().limit(0)).await;
    assert!(none.is_empty());

    // And across items rather than per item.
    let two_items = Query::from_items([
        QueryItem::of_types(["Limited"]).unwrap(),
        QueryItem::of_types(["Limited"]).unwrap(),
    ])
    .unwrap();
    let shared = drain(&store, &two_items, ReadOptions::new().limit(5)).await;
    assert_eq!(shared.len(), 5);
}

// ---------------------------------------------------------------------------
// AC-008 — `head` is a fresh query
// ---------------------------------------------------------------------------

/// AC-008 — `None` on a new file, the assigned maximum after a seed, and the
/// position the *other* handle just wrote.
///
/// A memoised head passes every single-handle test and fails only once a second
/// connection exists — which is this adapter's whole point.
#[tokio::test]
async fn head_is_a_fresh_query_across_two_handles() {
    let db = TempDb::new("head");
    let first = db.open();
    assert_eq!(first.head().await.unwrap(), None);

    let last = first
        .append(&[event("One"), event("Two")], None)
        .await
        .unwrap();
    assert_eq!(first.head().await.unwrap(), Some(last));

    let second = db.open();
    let theirs = second.append(&[event("Theirs")], None).await.unwrap();
    assert_eq!(
        first.head().await.unwrap(),
        Some(theirs),
        "the first handle must report the position the second handle just \
         wrote; a cached field would report a value that predates the commit"
    );
}

// ---------------------------------------------------------------------------
// AC-009 — query semantics, and the row that comes back
// ---------------------------------------------------------------------------

/// AC-009 — types OR within an item, tags AND with superset matching, items OR,
/// duplicate items not duplicating, and `Query::all` matching untagged events.
#[tokio::test]
async fn query_semantics_are_served_by_the_page_sql() {
    let db = TempDb::new("semantics");
    let store = db.open();

    store.append(&[event("Untagged")], None).await.unwrap();
    store
        .append(&[tagged("Enrolled", &[("course", "c1")])], None)
        .await
        .unwrap();
    store
        .append(
            &[tagged("Enrolled", &[("course", "c1"), ("student", "s1")])],
            None,
        )
        .await
        .unwrap();
    store
        .append(&[tagged("Dropped", &[("course", "c2")])], None)
        .await
        .unwrap();

    // `Query::all` matches every event, including the untagged one.
    assert_eq!(
        drain(&store, &Query::all(), ReadOptions::new()).await.len(),
        4
    );

    // Types within an item are OR.
    let either = drain(
        &store,
        &query_of_types(&["Enrolled", "Dropped"]),
        ReadOptions::new(),
    )
    .await;
    assert_eq!(either.len(), 3);

    // Tags within an item are AND, and matching is a **superset** test: the
    // two-tag event carries both, the one-tag event does not.
    let both = drain(
        &store,
        &query_tagged(&[("course", "c1"), ("student", "s1")]),
        ReadOptions::new(),
    )
    .await;
    assert_eq!(both.len(), 1);
    let course = drain(
        &store,
        &query_tagged(&[("course", "c1")]),
        ReadOptions::new(),
    )
    .await;
    assert_eq!(course.len(), 2);

    // Items are OR, and duplicate items do not duplicate events.
    let duplicated = Query::from_items([
        QueryItem::tagged(tags(&[("course", "c1")])).unwrap(),
        QueryItem::tagged(tags(&[("course", "c1")])).unwrap(),
    ])
    .unwrap();
    assert_eq!(
        drain(&store, &duplicated, ReadOptions::new()).await.len(),
        2
    );

    // Type-plus-tag, which is the shape the covering column exists for.
    let constrained =
        Query::from_item(QueryItem::new(["Enrolled"], tags(&[("course", "c1")])).unwrap());
    assert_eq!(
        drain(&store, &constrained, ReadOptions::new()).await.len(),
        2
    );
}

/// AC-009 — the decode half agrees with the encode half, byte for byte.
#[tokio::test]
async fn a_decoded_row_matches_what_was_written_byte_for_byte() {
    let db = TempDb::new("codec");
    let store = db.open();
    let store_id = store.store_id();

    let payload: Vec<u8> = (0..=255u8).collect();
    let with_metadata = Event::new("Rich", payload.clone())
        .unwrap()
        .with_tags(tags(&[("course", "c1"), ("student", "s1")]))
        .with_metadata(vec![1u8, 2, 3]);
    let absent_metadata = Event::new("Plain", payload.clone()).unwrap();
    let empty_metadata = Event::new("Empty", payload.clone())
        .unwrap()
        .with_metadata(Vec::new());

    store
        .append(
            &[with_metadata.clone(), absent_metadata, empty_metadata],
            None,
        )
        .await
        .unwrap();

    let seen = drain(&store, &Query::all(), ReadOptions::new()).await;
    assert_eq!(seen.len(), 3);

    assert_eq!(seen[0].event.event_type().as_str(), "Rich");
    assert_eq!(&seen[0].event.data()[..], &payload[..]);
    assert_eq!(seen[0].event.tags(), with_metadata.tags());
    assert_eq!(
        seen[0].event.metadata().map(|bytes| bytes.to_vec()),
        Some(vec![1u8, 2, 3])
    );

    assert_eq!(
        seen[1].event.metadata(),
        None,
        "absent metadata must stay absent"
    );
    assert_eq!(
        seen[2].event.metadata().map(|bytes| bytes.to_vec()),
        Some(Vec::new()),
        "empty metadata is a different value from absent metadata, and a store \
         that folds them has lost one"
    );

    // The identity comes from the **stored origin pair**, not from the local
    // incarnation plus the local position — correct for a locally appended
    // event, and wrong for every ingested one, which is why it is read back.
    for observed in &seen {
        assert_eq!(observed.id.store(), store_id);
        assert_eq!(observed.id.position(), observed.position);
    }

    // `recorded_at` is returned as stored, never re-stamped on read.
    let stored: i64 = db
        .raw()
        .query_row(
            "SELECT recorded_at FROM event WHERE event_type = 'Rich'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(seen[0].recorded_at.as_millis(), stored);
}

/// AC-009 — `contains_event_id` answers from the stored origin pair.
#[tokio::test]
async fn contains_event_id_answers_from_the_stored_origin_pair() {
    let db = TempDb::new("contains");
    let store = db.open();
    store.append(&[event("Known")], None).await.unwrap();

    let known = drain(&store, &Query::all(), ReadOptions::new()).await;
    assert!(store.contains_event_id(known[0].id).await.unwrap());

    let elsewhere = happenstance_core::EventId::new(
        happenstance_core::StoreId::from_bytes([9u8; 16]),
        known[0].position,
    );
    assert!(!store.contains_event_id(elsewhere).await.unwrap());
}
