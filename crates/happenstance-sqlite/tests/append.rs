//! `append`, driven through the port and inspected through a connection the
//! port never touched.
//!
//! Every append rule but two in the conformance suite reads its result back
//! *through* `read`, so none of them can answer the one question this target
//! exists for: **what is in the file after a refusal?** A store whose `read` is
//! also wrong satisfies a port-only round trip perfectly. So every claim about a
//! refusal here is made through a second raw [`rusqlite::Connection`], against
//! the three tables migration 1 created.
//!
//! Nothing is doubled: real files, the real driver, the real port. The two
//! testkit rules that need only `append` are invoked by name rather than
//! re-written locally, so AC-001 is checked against the sibling's own contract.
//!
//! There is no timeout, watchdog, `sleep` or retry loop anywhere in this file,
//! and there must not be: CF-33 keeps a clock out of the suite, and a hang here
//! is evidence about ADR-0022's busy-timeout paragraph rather than something to
//! paper over.

#![cfg(feature = "event-store")]
#![allow(clippy::unwrap_used)]

use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    AppendCondition, AppendError, Event, Query, QueryItem, SendEventStore, StoreLimit, Tags,
};
use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};
use happenstance_testkit::{Capability, Fixture};
use rusqlite::Connection;

// ---------------------------------------------------------------------------
// The temporary file, and the smallest fixture the two borrowed rules need
// ---------------------------------------------------------------------------

/// A temporary database path that deletes itself, and its WAL sidecars, on drop.
#[derive(Debug)]
struct TempDb(PathBuf);

impl TempDb {
    fn new(label: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "happenstance-sqlite-append-{label}-{}-{ordinal}.db",
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

    /// A connection the store never held, for asking what is actually in the
    /// file.
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

/// The smallest real [`Fixture`] the two borrowed rules can be driven through.
///
/// Deliberately minimal, and deliberately not the adapter's fixture: the one the
/// whole suite runs against is `SqliteFixture`, in
/// `crates/happenstance-sqlite/tests/conformance.rs`, and it declares
/// capabilities and ceilings this one has no rule to answer. This exists because
/// `append_rejects_empty_batch` and
/// `empty_batch_is_refused_before_the_condition_is_evaluated` are the only two
/// append rules that never read their result back through `read` — so they are
/// runnable here, one story before the read path exists, and running them
/// against the sibling's own contract is stronger than restating their
/// assertions locally.
#[derive(Debug)]
struct AppendOnlyFixture(TempDb);

impl AppendOnlyFixture {
    fn new() -> Self {
        Self(TempDb::new("rules"))
    }
}

impl Fixture for AppendOnlyFixture {
    type Store = SqliteEventStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "this minimal fixture exists only to drive the two append rules that do \
         not read their result back; the reopen rules belong to SqliteFixture in \
         tests/conformance.rs, which is the fixture this adapter is measured by",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(self.0.open())
    }
}

// ---------------------------------------------------------------------------
// Builders
// ---------------------------------------------------------------------------

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

/// Row counts in the three tables one append writes to.
fn row_counts(connection: &Connection) -> (i64, i64, i64) {
    let count = |table: &str| -> i64 {
        connection
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |row| {
                row.get(0)
            })
            .unwrap()
    };
    (count("event"), count("event_tag"), count("tag_cardinality"))
}

/// Every position the store has actually assigned, ascending.
fn assigned_positions(connection: &Connection) -> Vec<i64> {
    let mut statement = connection
        .prepare("SELECT position FROM event ORDER BY position")
        .unwrap();
    let rows = statement.query_map([], |row| row.get(0)).unwrap();
    rows.map(Result::unwrap).collect()
}

/// The `detail` column of `EXPLAIN QUERY PLAN`, one string per plan row.
///
/// The same helper `tests/migration.rs` carries, for the same reason: the plan
/// is asserted on by *name* — whether a table is searched or scanned — and never
/// by pattern-matching SQLite's exact phrasing, which changes between versions.
fn query_plan(connection: &Connection, sql: &str) -> Vec<String> {
    let mut statement = connection
        .prepare(&format!("EXPLAIN QUERY PLAN {sql}"))
        .unwrap();
    let rows = statement
        .query_map([], |row| row.get::<_, String>(3))
        .unwrap();
    rows.map(Result::unwrap).collect()
}

// ---------------------------------------------------------------------------
// AC-001 — an empty batch is refused, and refused first
// ---------------------------------------------------------------------------

/// AC-001 — `ConditionViolated` means *rebuild and retry*, so answering it for
/// an empty batch puts a conforming caller into a loop that never terminates.
#[tokio::test]
async fn empty_batch_is_refused_before_any_condition_is_looked_at() {
    let db = TempDb::new("empty-batch");
    let store = db.open();

    store.append(&[event("Blocker")], None).await.unwrap();

    // The condition matches what is already in the store, so a store that
    // evaluates it first answers ConditionViolated and one that checks its
    // argument first answers NoEvents. Both refuse; only one names the fault.
    let refused = store
        .append(
            &[],
            Some(&AppendCondition::new(query_of_types(&["Blocker"]))),
        )
        .await;

    assert!(
        matches!(refused, Err(AppendError::NoEvents)),
        "an empty batch is the caller's own bug and the emptiness check MUST \
         precede the condition check; got {refused:?}"
    );
}

/// AC-001 — the same claim, checked against the testkit's own wording rather
/// than a local restatement of it.
#[tokio::test]
async fn the_two_borrowed_empty_batch_rules_pass() {
    happenstance_testkit::rules::append_rejects_empty_batch(async || AppendOnlyFixture::new())
        .await
        .report("append_rejects_empty_batch");

    happenstance_testkit::rules::empty_batch_is_refused_before_the_condition_is_evaluated(
        async || AppendOnlyFixture::new(),
    )
    .await
    .report("empty_batch_is_refused_before_the_condition_is_evaluated");
}

// ---------------------------------------------------------------------------
// AC-002 — the three ceilings, exact at both ends
// ---------------------------------------------------------------------------

/// AC-002 — a value at exactly the declared ceiling is accepted; one unit larger
/// is refused as `ExceedsStoreLimit` with the matching variant.
///
/// The at-the-ceiling acceptance is the anchor. Without it, a store that refused
/// *everything* would satisfy every other assertion here.
#[tokio::test]
async fn each_ceiling_is_exact_at_both_ends() {
    let db = TempDb::new("ceilings");
    let store = db.open();

    // EventDataLen, measured on `Event::data`'s byte length — not on an encoded
    // row and not on `data + metadata`.
    let at_limit = Event::new("Sized", vec![b'x'; SqliteEventStore::MAX_EVENT_DATA_LEN]).unwrap();
    store.append(&[at_limit], None).await.unwrap();
    let over = Event::new(
        "Sized",
        vec![b'x'; SqliteEventStore::MAX_EVENT_DATA_LEN + 1],
    )
    .unwrap();
    assert!(
        matches!(
            store.append(&[over], None).await,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                len
            }) if len == SqliteEventStore::MAX_EVENT_DATA_LEN + 1
        ),
        "an oversized payload must be ExceedsStoreLimit {{ EventDataLen }}, \
         never AppendError::Store and never a truncation"
    );

    // TagsPerEvent.
    let at_limit = event("Tagged").with_tags(distinct_tags(SqliteEventStore::MAX_TAGS_PER_EVENT));
    store.append(&[at_limit], None).await.unwrap();
    let over = event("Tagged").with_tags(distinct_tags(SqliteEventStore::MAX_TAGS_PER_EVENT + 1));
    assert!(
        matches!(
            store.append(&[over], None).await,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::TagsPerEvent,
                len
            }) if len == SqliteEventStore::MAX_TAGS_PER_EVENT + 1
        ),
        "too many tags must be ExceedsStoreLimit {{ TagsPerEvent }}"
    );

    // EventsPerBatch.
    let at_limit: Vec<Event> = (0..SqliteEventStore::MAX_EVENTS_PER_BATCH)
        .map(|_| event("Batched"))
        .collect();
    store.append(&at_limit, None).await.unwrap();
    let over: Vec<Event> = (0..=SqliteEventStore::MAX_EVENTS_PER_BATCH)
        .map(|_| event("Batched"))
        .collect();
    assert!(
        matches!(
            store.append(&over, None).await,
            Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len
            }) if len == SqliteEventStore::MAX_EVENTS_PER_BATCH + 1
        ),
        "too many events must be ExceedsStoreLimit {{ EventsPerBatch }}"
    );
}

/// `n` distinct tags.
fn distinct_tags(n: usize) -> Tags {
    let pairs: Vec<(String, String)> = (0..n)
        .map(|index| ("k".to_owned(), format!("v{index}")))
        .collect();
    Tags::from_pairs(
        pairs
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str())),
    )
    .unwrap()
}

// ---------------------------------------------------------------------------
// AC-003 — a refusal costs the log nothing
// ---------------------------------------------------------------------------

/// AC-003 — read through a connection the store never held, because "nothing of
/// the refused value survives" is a claim about the *file*.
#[tokio::test]
async fn a_refused_append_leaves_the_file_unchanged() {
    let db = TempDb::new("refusal");
    let store = db.open();

    store
        .append(&[tagged("Seed", &[("course", "c1")])], None)
        .await
        .unwrap();

    let inspector = db.raw();
    let before = row_counts(&inspector);
    let positions_before = assigned_positions(&inspector);

    // 1. An empty batch.
    assert!(store.append(&[], None).await.is_err());

    // 2. A ceiling.
    let over = Event::new(
        "TooBig",
        vec![b'x'; SqliteEventStore::MAX_EVENT_DATA_LEN + 1],
    )
    .unwrap();
    assert!(store.append(&[over], None).await.is_err());

    // 3. A violated guard, whose batch is large enough to be split across more
    //    than one insert statement had it been allowed to land.
    let batch: Vec<Event> = (0..8)
        .map(|_| tagged("Rejected", &[("course", "c1")]))
        .collect();
    let refused = store
        .append(
            &batch,
            Some(&AppendCondition::new(query_tagged(&[("course", "c1")]))),
        )
        .await;
    assert!(
        matches!(refused, Err(AppendError::ConditionViolated(_))),
        "a violated guard is ConditionViolated, never Store — the Store answer \
         is what a BEGIN DEFERRED probe-then-insert produces; got {refused:?}"
    );

    let after = row_counts(&inspector);
    assert_eq!(before, after, "a refusal wrote rows into the file");
    assert_eq!(positions_before, assigned_positions(&inspector));
}

// ---------------------------------------------------------------------------
// AC-004 — the probe and the insert are one decision
// ---------------------------------------------------------------------------

/// AC-004 — two handles onto one file deciding from the same state: exactly one
/// wins, and the loser learns it lost as `ConditionViolated` rather than as a
/// driver error.
///
/// `AppendError::Store` here is the signature of the named wrong implementation:
/// `BEGIN DEFERRED` starts as a reader and upgrades at the first write, so the
/// loser gets `SQLITE_BUSY` instead of a refusal it can act on.
#[tokio::test]
async fn two_handles_racing_one_condition_yield_one_winner_and_one_rejection() {
    let db = TempDb::new("race");
    let first = db.open();
    let second = db.open();

    let condition = AppendCondition::new(query_tagged(&[("course", "c1")]));

    let winner = first
        .append(&[tagged("Enrolled", &[("course", "c1")])], Some(&condition))
        .await;
    assert!(winner.is_ok(), "the first decision must land: {winner:?}");

    // The second handle decided from the same (empty) state and is now stale.
    let loser = second
        .append(&[tagged("Enrolled", &[("course", "c1")])], Some(&condition))
        .await;
    assert!(
        matches!(loser, Err(AppendError::ConditionViolated(_))),
        "the loser must be told to rebuild and retry, not handed a driver \
         error; got {loser:?}"
    );

    let events: i64 = db
        .raw()
        .query_row("SELECT count(*) FROM event", [], |row| row.get(0))
        .unwrap();
    assert_eq!(events, 1, "exactly one of the two decisions landed");
}

// ---------------------------------------------------------------------------
// AC-005 — guard semantics
// ---------------------------------------------------------------------------

/// AC-005 — `after` is **exclusive**, and it is one field away in the port from
/// `ReadOptions::from`, which is inclusive.
#[tokio::test]
async fn guard_after_is_exclusive_at_the_boundary() {
    let db = TempDb::new("after-boundary");
    let store = db.open();

    let boundary = store
        .append(&[tagged("Enrolled", &[("course", "c1")])], None)
        .await
        .unwrap();

    // A guard bounded at exactly that position must NOT be violated by the
    // event sitting at it.
    let admitted = store
        .append(
            &[tagged("Enrolled", &[("course", "c1")])],
            Some(&AppendCondition::new(query_tagged(&[("course", "c1")])).after(boundary)),
        )
        .await;
    assert!(
        admitted.is_ok(),
        "`after` is exclusive: an event AT the boundary does not violate the \
         guard; got {admitted:?}"
    );

    // And the event just written, which is strictly above it, does.
    let refused = store
        .append(
            &[tagged("Enrolled", &[("course", "c1")])],
            Some(&AppendCondition::new(query_tagged(&[("course", "c1")])).after(boundary)),
        )
        .await;
    assert!(matches!(refused, Err(AppendError::ConditionViolated(_))));
}

/// AC-005 — a guard with no `after` checks the entire log.
#[tokio::test]
async fn a_guard_without_after_sees_the_whole_log() {
    let db = TempDb::new("unbounded-guard");
    let store = db.open();

    store
        .append(&[tagged("Enrolled", &[("course", "c1")])], None)
        .await
        .unwrap();
    for _ in 0..3 {
        store.append(&[event("Noise")], None).await.unwrap();
    }

    let refused = store
        .append(
            &[event("Enrolled")],
            Some(&AppendCondition::new(query_tagged(&[("course", "c1")]))),
        )
        .await;
    assert!(
        matches!(refused, Err(AppendError::ConditionViolated(_))),
        "an unbounded guard is violated by a match anywhere in the log, however \
         old; got {refused:?}"
    );
}

/// AC-005 — any one violated guard refuses the whole append.
#[tokio::test]
async fn any_violated_guard_refuses_the_whole_batch() {
    let db = TempDb::new("many-guards");
    let store = db.open();

    store.append(&[event("Second")], None).await.unwrap();

    let condition = AppendCondition::new(query_of_types(&["First"]))
        .and_guard(query_of_types(&["Second"]), None)
        .and_guard(query_of_types(&["Third"]), None);

    let refused = store.append(&[event("Anything")], Some(&condition)).await;
    assert!(
        matches!(refused, Err(AppendError::ConditionViolated(_))),
        "the middle guard matched, so the whole append is refused; got {refused:?}"
    );
}

/// AC-005 — ES-21: a batch is never evaluated against its own events.
#[tokio::test]
async fn a_batch_never_conflicts_with_itself() {
    let db = TempDb::new("self-conflict");
    let store = db.open();

    let batch = vec![
        tagged("Enrolled", &[("course", "c1")]),
        tagged("Enrolled", &[("course", "c1")]),
    ];
    let landed = store
        .append(
            &batch,
            Some(&AppendCondition::new(query_tagged(&[("course", "c1")]))),
        )
        .await;

    assert!(
        landed.is_ok(),
        "the condition is evaluated against what the store already held, and it \
         held nothing; got {landed:?}"
    );
    assert_eq!(assigned_positions(&db.raw()).len(), 2);
}

// ---------------------------------------------------------------------------
// AC-006 — the returned position is the caller's own
// ---------------------------------------------------------------------------

/// AC-006 — the position of the last event of *this* batch, not the store head.
///
/// A second handle commits in between, which is exactly when returning
/// `head()` stops being the same number.
#[tokio::test]
async fn append_returns_the_callers_own_last_position() {
    let db = TempDb::new("returned-position");
    let first = db.open();
    let second = db.open();

    let mine = first
        .append(&[event("Mine"), event("MineToo")], None)
        .await
        .unwrap();
    second.append(&[event("Theirs")], None).await.unwrap();

    let inspector = db.raw();
    let mut statement = inspector
        .prepare("SELECT position FROM event WHERE event_type = 'MineToo'")
        .unwrap();
    let my_last: i64 = statement.query_row([], |row| row.get(0)).unwrap();

    assert_eq!(
        i64::try_from(mine.get()).unwrap(),
        my_last,
        "append returns the position it assigned to the caller's own last \
         event, compared against what the store actually stored — never a \
         literal, and never the head another connection has since moved"
    );
}

/// AC-006 — positions follow slice order and ascend strictly. No literals: gaps
/// are permitted and `AUTOINCREMENT` produces them.
#[tokio::test]
async fn batch_positions_follow_slice_order() {
    let db = TempDb::new("slice-order");
    let store = db.open();

    let types = ["A", "B", "C", "D"];
    let batch: Vec<Event> = types.iter().map(|name| event(name)).collect();
    let last = store.append(&batch, None).await.unwrap();

    let inspector = db.raw();
    let mut statement = inspector
        .prepare("SELECT event_type, position FROM event ORDER BY position")
        .unwrap();
    let rows: Vec<(String, i64)> = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();

    let observed: Vec<&str> = rows.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(observed, types, "position order must follow slice order");
    assert!(
        rows.windows(2).all(|pair| pair[0].1 < pair[1].1),
        "positions ascend strictly; nothing here assumes they ascend by one"
    );
    assert_eq!(i64::try_from(last.get()).unwrap(), rows[rows.len() - 1].1);
}

// ---------------------------------------------------------------------------
// AC-007 — identity, stamp, covering column and cardinality
// ---------------------------------------------------------------------------

/// AC-007 — every appended row carries this store's incarnation, its own
/// position as the origin, and a `recorded_at` stamped exactly once here.
#[tokio::test]
async fn every_appended_row_carries_its_identity_and_stamp() {
    let db = TempDb::new("identity");
    let store = db.open();
    let store_id = store.store_id();

    let before = now_millis();
    store
        .append(
            &[tagged("Enrolled", &[("course", "c1"), ("student", "s1")])],
            None,
        )
        .await
        .unwrap();
    let after = now_millis();

    let inspector = db.raw();
    let (origin_store, origin_position, position, recorded_at): (Vec<u8>, i64, i64, i64) =
        inspector
            .query_row(
                "SELECT origin_store, origin_position, position, recorded_at FROM event",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .unwrap();

    assert_eq!(origin_store, store_id.to_bytes().to_vec());
    assert_eq!(
        origin_position, position,
        "a locally appended event's origin position is the position it was just \
         given"
    );
    assert!(
        (before..=after).contains(&recorded_at),
        "recorded_at is stamped once, at append: {recorded_at} outside \
         [{before}, {after}]"
    );

    // The covering column: every event_tag row carries its event's type, which
    // is what keeps a tag-plus-type probe off `event` and out from under the
    // write lock.
    let mut statement = inspector
        .prepare("SELECT tag, position, event_type FROM event_tag ORDER BY tag")
        .unwrap();
    let rows: Vec<(String, i64, String)> = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert_eq!(rows.len(), 2);
    for (_, tag_position, event_type) in &rows {
        assert_eq!(*tag_position, position);
        assert_eq!(event_type, "Enrolled");
    }
}

/// AC-007 — `tag_cardinality` is created by migration 1 and **written by
/// `append`**. A table created and never updated silently restores the
/// serialising plan the schema amendment was made to avoid.
#[tokio::test]
async fn tag_cardinality_is_maintained_by_append() {
    let db = TempDb::new("cardinality");
    let store = db.open();

    store
        .append(
            &[
                tagged("Enrolled", &[("course", "c1"), ("student", "s1")]),
                tagged("Enrolled", &[("course", "c1"), ("student", "s2")]),
                tagged("Enrolled", &[("course", "c1"), ("student", "s3")]),
            ],
            None,
        )
        .await
        .unwrap();

    let inspector = db.raw();
    let mut statement = inspector
        .prepare("SELECT tag, events FROM tag_cardinality ORDER BY tag")
        .unwrap();
    let counts: Vec<(String, i64)> = statement
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();

    assert_eq!(
        counts,
        vec![
            ("course:c1".to_owned(), 3),
            ("student:s1".to_owned(), 1),
            ("student:s2".to_owned(), 1),
            ("student:s3".to_owned(), 1),
        ],
        "every tag written is counted; a zero here is the most-selective-tag \
         probe with nothing to order by"
    );
}

/// AC-007 and NF-001 — the identity stamp touches the batch's own rows and
/// nothing else, so its cost does not grow with the log it is appended to.
///
/// The stamp runs inside the `BEGIN IMMEDIATE` transaction, which is where
/// NF-001's whole budget is: `WHERE origin_position IS NULL` on its own has no
/// index to seek — the only index over the column is
/// `UNIQUE (origin_store, origin_position)` and `origin_store` leads it — so it
/// is a scan of the entire `event` table, walked with every other writer queued
/// behind the write lock. That is the same defect class ADR-0022 §7's schema
/// amendment exists to remove, named in the module documentation, and it lands
/// on the concurrency family rather than here.
///
/// It is observable through the port because the bound changes *behaviour*, not
/// only cost: a row that already carries a NULL origin — the shape a replication
/// ingest will one day write — is re-stamped under this store's incarnation by
/// the unbounded predicate, and left exactly as found by the bounded one. This
/// is the assertion that pins the production statement; the plan assertion below
/// is what names the reason.
#[tokio::test]
async fn the_identity_stamp_touches_only_the_batch_it_wrote() {
    let db = TempDb::new("stamp-bounds");
    let store = db.open();

    // A row the store did not write, carrying no origin, planted through a
    // connection the port never held so that nothing about it is arranged by the
    // code under test. `X'1f'` is one UNIT byte: the canonical encoding of the
    // empty tag set.
    let planted = {
        let inspector = db.raw();
        inspector
            .execute(
                "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
                 VALUES ('Planted', X'7b7d', NULL, X'1f', 0)",
                [],
            )
            .unwrap();
        inspector.last_insert_rowid()
    };

    store.append(&[event("Enrolled")], None).await.unwrap();

    let inspector = db.raw();
    let planted_origin: Option<i64> = inspector
        .query_row(
            "SELECT origin_position FROM event WHERE position = ?",
            [planted],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        planted_origin, None,
        "the stamp rewrote a row outside its own batch. An unbounded \
         `WHERE origin_position IS NULL` scans the whole log under BEGIN \
         IMMEDIATE and claims another origin's events as this store's own"
    );

    // The anchor: a stamp bounded so tightly that it stamps nothing would
    // satisfy the assertion above.
    let appended_origin: Option<i64> = inspector
        .query_row(
            "SELECT origin_position FROM event WHERE event_type = 'Enrolled'",
            [],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        appended_origin,
        Some(planted + 1),
        "the batch's own row must still be stamped with the position it was \
         just given"
    );
}

/// AC-007 and NF-001 — the bound is a rowid seek, and the predicate it replaced
/// is the scan.
///
/// Asserted by plan *name* rather than by matching SQLite's phrasing, the way
/// `tests/migration.rs` asserts the tag-plus-type probe never joins back to
/// `event`. The unbounded arm is the negative control: without it this test
/// would pass against a planner that had simply stopped reporting scans.
#[test]
fn the_identity_stamp_seeks_by_position_rather_than_scanning() {
    let db = TempDb::new("stamp-plan");
    drop(db.open());
    let connection = db.raw();

    let bounded = query_plan(
        &connection,
        "UPDATE event SET origin_store = X'00', origin_position = position \
         WHERE position >= 1 AND origin_position IS NULL",
    );
    assert!(!bounded.is_empty(), "EXPLAIN QUERY PLAN returned no rows");
    for detail in &bounded {
        assert!(
            !detail.contains("SCAN"),
            "the identity stamp scans `event` instead of seeking to its own \
             batch, under the BEGIN IMMEDIATE write lock. Plan: {bounded:?}"
        );
    }

    let unbounded = query_plan(
        &connection,
        "UPDATE event SET origin_store = X'00', origin_position = position \
         WHERE origin_position IS NULL",
    );
    assert!(
        unbounded.iter().any(|detail| detail.contains("SCAN")),
        "the negative control no longer scans, so the assertion above proves \
         nothing about the bound. Plan: {unbounded:?}"
    );
}

// ---------------------------------------------------------------------------
// AC-008 — the statements are chunked, the transaction is not
// ---------------------------------------------------------------------------

/// AC-008 — a batch at exactly the declared ceiling, with the maximum tags per
/// event, lands whole.
///
/// At those numbers a single multi-row tag insert would bind far more than
/// SQLite's 32,766 bound parameters, so this exercises the chunking on every
/// run rather than at a boundary somebody else finds.
#[tokio::test]
async fn a_batch_at_the_declared_ceiling_lands_whole() {
    let db = TempDb::new("full-ceiling");
    let store = db.open();

    let batch: Vec<Event> = (0..SqliteEventStore::MAX_EVENTS_PER_BATCH)
        .map(|index| {
            let pairs: Vec<(String, String)> = (0..SqliteEventStore::MAX_TAGS_PER_EVENT)
                .map(|tag| (format!("k{tag}"), format!("v{index}")))
                .collect();
            let tags = Tags::from_pairs(
                pairs
                    .iter()
                    .map(|(key, value)| (key.as_str(), value.as_str())),
            )
            .unwrap();
            event("Bulk").with_tags(tags)
        })
        .collect();

    let last = store.append(&batch, None).await.unwrap();

    let inspector = db.raw();
    let (events, tag_rows, _) = row_counts(&inspector);
    assert_eq!(
        events,
        i64::try_from(SqliteEventStore::MAX_EVENTS_PER_BATCH).unwrap()
    );
    assert_eq!(
        tag_rows,
        i64::try_from(
            SqliteEventStore::MAX_EVENTS_PER_BATCH * SqliteEventStore::MAX_TAGS_PER_EVENT
        )
        .unwrap(),
        "every tag row of every event landed; a chunk lost here would be \
         invisible to a port-level read-back of the events alone"
    );

    let positions = assigned_positions(&inspector);
    assert_eq!(
        i64::try_from(last.get()).unwrap(),
        *positions.last().unwrap()
    );
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}

/// AC-008 — a failure part way through a chunked insert rolls back **every**
/// chunk. The unit of reversal is the batch, never the statement.
///
/// The fault is a real one: a trigger installed through a second connection
/// that raises once the fourth row of a batch is written. That is the shape
/// `Fixture::MID_BATCH_FAULT` describes, and it is the only way to reach between
/// two rows of one transaction — nothing a caller holds can.
#[tokio::test]
async fn a_failure_mid_batch_leaves_nothing() {
    let db = TempDb::new("mid-batch-fault");
    let store = db.open();

    let inspector = db.raw();
    inspector
        .execute_batch(
            "CREATE TRIGGER fail_mid_batch AFTER INSERT ON event
             WHEN (SELECT count(*) FROM event) > 3
             BEGIN SELECT RAISE(ABORT, 'injected mid-batch fault'); END;",
        )
        .unwrap();

    let batch: Vec<Event> = (0..16).map(|_| tagged("Bulk", &[("k", "v")])).collect();

    let failed = store.append(&batch, None).await;
    assert!(
        matches!(failed, Err(AppendError::Store(_))),
        "an injected driver fault is honestly AppendError::Store; got {failed:?}"
    );

    let (events, tag_rows, cardinality) = row_counts(&inspector);
    assert_eq!(
        (events, tag_rows, cardinality),
        (0, 0, 0),
        "committing between chunks would leave a partially applied batch that a \
         single-threaded read-back would happily confirm"
    );

    inspector
        .execute_batch("DROP TRIGGER fail_mid_batch;")
        .unwrap();
}

// ---------------------------------------------------------------------------
// AC-009 — contention is a wait
// ---------------------------------------------------------------------------

/// AC-009 — a contended `BEGIN IMMEDIATE` waits on the configured busy timeout
/// rather than returning `AppendError::Store`.
///
/// No timeout, watchdog, `sleep` or retry is introduced to make this pass. If it
/// ever hangs, that is a finding about ADR-0022's busy-timeout paragraph.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn contention_waits_rather_than_erroring() {
    let db = TempDb::new("contention");
    let path = db.path().to_path_buf();

    let handles: Vec<_> = (0..4)
        .map(|worker| {
            let path = path.clone();
            tokio::spawn(async move {
                let store = SqliteEventStore::open(&path).unwrap();
                let mut outcomes = Vec::new();
                for round in 0..8 {
                    outcomes.push(
                        store
                            .append(&[tagged("Contended", &[("worker", "w")])], None)
                            .await
                            .map(|position| (worker, round, position)),
                    );
                }
                outcomes
            })
        })
        .collect();

    let mut committed = 0;
    for handle in handles {
        for outcome in handle.await.unwrap() {
            match outcome {
                Ok(_) => committed += 1,
                Err(err) => panic!(
                    "contention must be absorbed by the busy timeout, not \
                     reported as a store failure: {err:?}"
                ),
            }
        }
    }

    assert_eq!(committed, 32);
    let (events, _, _) = row_counts(&db.raw());
    assert_eq!(events, 32);
}

/// Milliseconds since the Unix epoch, as the adapter stamps them.
fn now_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|since| i64::try_from(since.as_millis()).ok())
        .unwrap_or(0)
}

/// The error type is still nameable and still carries what a caller needs, which
/// is the half of AC-010 a test can hold.
#[test]
fn the_store_error_is_still_the_named_type() {
    fn assert_error<T: core::error::Error + Send + Sync + 'static>() {}

    assert_error::<SqliteEventStoreError>();
}
