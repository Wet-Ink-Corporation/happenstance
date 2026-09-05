//! What the shipped intersection chain costs, at three log sizes, interleaved.
//!
//! # Why interleaved, and why one file
//!
//! `experiments/append-condition/README.md` records that on this host two runs an
//! hour apart disagreed by up to 45%, and that one arm's *unconditional* append —
//! a path no arm plays any part in — varied 4x between time slots. So the shapes
//! are measured round-robin inside one process, and the figure that decides
//! anything is a **ratio taken in one run**, never a number carried across runs.
//!
//! Which shape goes first **rotates every round**. Without that, one shape always
//! follows another's page-cache churn on a 184 MB database and the whole
//! difference could be read as an artefact of going second.
//!
//! They also share **one database file**. The four shapes differ only in the
//! guard SQL — one schema, one write path, one identity story — so seeding one
//! log rather than four removes "which million events" from the list of things a
//! difference between two figures could be. It also makes the fifth arm possible.
//!
//! # The fifth arm is the real adapter
//!
//! `query_sql::item_sql` is private, so [`Shape::Chain`] is a transcription of
//! it. `tests/emitted_sql.rs` is the standing guard on that transcription — it
//! asserts the string against one captured off a `sqlite3_trace_v2` callback.
//! This file adds the other half: the real
//! [`SqliteEventStore`](happenstance_sqlite::event_store::SqliteEventStore) is
//! opened on the same seeded file and its whole `append` is timed beside the
//! probe's, so the end-to-end figure is not only about SQL that matches but about
//! a store that behaves the same.
//!
//! The real adapter cannot supply the three counterfactual shapes, which is the
//! whole reason the transcription exists.
//!
//! # The two instruments, and why both
//!
//! * `GUARD` rows — **the guard alone**. `BEGIN IMMEDIATE`, the
//!   `tag_cardinality` lookup, the `SELECT max(position) FROM (…)`, `ROLLBACK`:
//!   exactly what `append_locked` does minus the write. Nothing commits, so the
//!   log length is constant across every round and every shape sees identical
//!   rows. `txn_only_us` — `BEGIN IMMEDIATE; ROLLBACK` with no guard at all — is
//!   the subtrahend, so `guard_us` is the probe rather than a transaction with a
//!   probe somewhere in it.
//! * `APPEND` rows — **the whole call**, through `EventStore::append`, on the
//!   rejection path only. A rejected append rolls back, so the log still does not
//!   move; what this adds over `GUARD` is the driver round trip, the error
//!   construction and — for the real adapter — `check_ceilings` and the store's
//!   own mutex.
//!
//! # The five scenarios
//!
//! | scenario | tags | boundary | what it is |
//! | --- | --- | --- | --- |
//! | `accepted-1tag-at-head` | 1 | head | the common write: a guard that holds |
//! | `accepted-2tag-at-head` | 2 | head | **finding I-2's cell** |
//! | `rejected-1tag-unbounded` | 1 | 0 | ADR-0022 §8's fast-path cell |
//! | `rejected-2tag-unbounded` | 2 | 0 | **ADR-0022 §8's 42-66 ms cell** |
//! | `rejected-2tag-midlog` | 2 | size/2 | a boundary with something to push |
//!
//! The last one exists because the other four are the two extremes: at `head`
//! nothing is above the boundary, and at `0` everything is. Half way down the log
//! is where a pushed-down predicate has half a range to discard, which is the
//! ordinary case and the one neither extreme reports.
//!
//! Run it with `cargo test --release --manifest-path
//! experiments/shipped-append-condition-sql/Cargo.toml --test guard_cost --
//! --nocapture --test-threads=1`.

mod support;

use std::time::Instant;

use correlated_exists_guard::chain::{Selectivity, Shape, arms_sql, guard_sql};
use correlated_exists_guard::{Conditions, ProbeStore, seed};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, SequencePosition, StoreId,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::block_on;
use happenstance_testkit::fixtures::{condition, condition_after, query_of, tagged_event};
use rusqlite::Connection;
use rusqlite::types::Value;

/// The log sizes, and how many rounds each is measured for.
///
/// 50,000 first, and it is a **calibration** rather than a result: ADR-0022 §8's
/// two-tag figure of 42-66 ms was taken at exactly that length against the
/// `grouped-adr0022` shape, so that shape landing in the same region is what says
/// the rest of the table is measuring the same thing.
///
/// The round counts fall as the log grows because the cost rises with it.
const SIZES: [(u64, usize); 3] = [(50_000, 120), (500_000, 30), (1_000_000, 15)];

/// Where a scenario's boundary sits.
#[derive(Debug, Clone, Copy)]
enum Boundary {
    /// Anchored at the head — the accepted path.
    Head,
    /// No boundary at all, which is a boundary of zero.
    Unbounded,
    /// Half way down the log.
    MidLog,
}

impl Boundary {
    /// The integer this boundary is at a log of `size` events.
    #[allow(clippy::cast_possible_wrap)]
    const fn at(self, size: u64) -> i64 {
        match self {
            Self::Head => size as i64,
            Self::Unbounded => 0,
            Self::MidLog => (size / 2) as i64,
        }
    }

    /// The condition the end-to-end instrument passes to `append`.
    fn condition(self, query: Query, size: u64) -> AppendCondition {
        match self {
            Self::Head => condition_after(query, size),
            Self::Unbounded => condition(query),
            Self::MidLog => condition_after(query, size / 2),
        }
    }
}

/// One scenario: a label, how many tags the boundary names, and where it sits.
const SCENARIOS: [(&str, usize, Boundary); 5] = [
    ("accepted-1tag-at-head", 1, Boundary::Head),
    ("accepted-2tag-at-head", 2, Boundary::Head),
    ("rejected-1tag-unbounded", 1, Boundary::Unbounded),
    ("rejected-2tag-unbounded", 2, Boundary::Unbounded),
    ("rejected-2tag-midlog", 2, Boundary::MidLog),
];

/// The boundary query of `tags` tags.
///
/// `row:r7` matches about one event in ninety-seven and `shard:cold` matches all
/// of them, which is the contrast `tag_cardinality` exists to order and the shape
/// ADR-0022 §8's ~200x figure was taken on.
fn boundary_query(tags: usize) -> Query {
    if tags == 1 {
        query_of(&[seed::SEED_TYPE], &[("row", "r7")])
    } else {
        query_of(&[seed::SEED_TYPE], &[("shard", "cold"), ("row", "r7")])
    }
}

/// The median, the tenth percentile and the maximum, in microseconds.
fn spread(samples: &mut [u128]) -> (u128, u128, u128) {
    samples.sort_unstable();
    let last = samples.len() - 1;
    (
        samples[samples.len() / 2],
        samples[last / 10],
        samples[last],
    )
}

/// `BEGIN IMMEDIATE`, the guard, `ROLLBACK` — `append_locked` minus the write.
///
/// The selectivity lookup is **inside** the timed region because it is inside the
/// transaction in the adapter: `evaluate` (`event_store.rs:645-646`) calls
/// `Selectivity::read_for` per guard, under the write lock, so a figure that
/// excluded it would be a figure for code that does not exist.
fn timed_guard(
    connection: &Connection,
    shape: Shape,
    query: &Query,
    boundary: i64,
) -> (u128, Option<i64>) {
    let started = Instant::now();
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .expect("the write lock must be available");
    let selectivity = Selectivity::read_for(connection, query).expect("the lookup must succeed");
    let items = query.items().expect("the query has items");
    let mut params: Vec<Value> = Vec::new();
    let sql = guard_sql(&arms_sql(shape, items, &selectivity, boundary, &mut params));
    let highest: Option<i64> = connection
        .query_row(&sql, rusqlite::params_from_iter(params.iter()), |row| {
            row.get(0)
        })
        .expect("the guard must run");
    connection.execute_batch("ROLLBACK").expect("rollback");
    let elapsed = started.elapsed().as_micros();

    // The **verdict**, not the raw maximum. A bounded shape returns `NULL` where
    // an unbounded one returns a position at or below the boundary, and those are
    // the same answer — *not violated* — spelled two ways. Comparing the raw
    // value would report a disagreement that is not one; comparing the verdict is
    // what catches a shape that has actually stopped noticing conflicts.
    (elapsed, highest.filter(|value| *value > boundary))
}

/// `BEGIN IMMEDIATE; ROLLBACK` and nothing else — the subtrahend.
fn timed_transaction(connection: &Connection) -> u128 {
    let started = Instant::now();
    connection
        .execute_batch("BEGIN IMMEDIATE")
        .expect("the write lock must be available");
    connection.execute_batch("ROLLBACK").expect("rollback");
    started.elapsed().as_micros()
}

/// A store whose rejection path can be timed, with its error type erased.
trait RejectingStore {
    /// Appends `event` under `condition`, requiring the condition to be violated,
    /// and returns the position it named.
    fn reject(&self, event: &Event, condition: &AppendCondition) -> Option<SequencePosition>;
}

impl RejectingStore for ProbeStore {
    fn reject(&self, event: &Event, condition: &AppendCondition) -> Option<SequencePosition> {
        expect_violation(block_on(EventStore::append(
            self,
            core::slice::from_ref(event),
            Some(condition),
        )))
    }
}

impl RejectingStore for SqliteEventStore {
    fn reject(&self, event: &Event, condition: &AppendCondition) -> Option<SequencePosition> {
        expect_violation(block_on(EventStore::append(
            self,
            core::slice::from_ref(event),
            Some(condition),
        )))
    }
}

/// Unwraps the one outcome the rejection path is allowed to have.
fn expect_violation<E: core::fmt::Debug>(
    outcome: Result<SequencePosition, AppendError<E>>,
) -> Option<SequencePosition> {
    match outcome {
        Err(AppendError::ConditionViolated(at)) => at.conflicting_position,
        other => panic!(
            "the rejection control must actually be rejected, or it is timing the \
             wrong path: {other:?}"
        ),
    }
}

/// Reads the incarnation the adapter minted for this file.
fn store_id_of(connection: &Connection) -> StoreId {
    connection
        .query_row(
            "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = 'store_id'",
            [],
            |row| {
                let raw: Vec<u8> = row.get(0)?;
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&raw[..16]);
                Ok(StoreId::from_bytes(bytes))
            },
        )
        .expect("the adapter minted an incarnation")
}

#[test]
fn the_guard_at_three_log_sizes() {
    let path = tempdb("guard-cost");

    // Migrate through the **adapter**, so the file's identity and schema version
    // are the adapter's own rather than this crate's idea of them.
    drop(SqliteEventStore::open(&path).expect("migrating must succeed"));

    let mut writer =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let store_id = store_id_of(&writer);

    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let conditions = Conditions::require(&connection);
    println!(
        "CONDITIONS\t{} max_arms={}",
        conditions.line(),
        SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT
    );

    let probes: Vec<ProbeStore> = Shape::ALL
        .iter()
        .map(|shape| ProbeStore::open(&path, *shape).expect("opening must succeed"))
        .collect();
    let real = SqliteEventStore::open(&path).expect("opening must succeed");
    let event = tagged_event(seed::SEED_TYPE, &[("shard", "cold"), ("row", "r7")]);

    for (size, rounds) in SIZES {
        seed::seed(&mut writer, store_id, size).expect("seeding must succeed");
        let check = seed::verify(&connection, size).expect("verify must succeed");
        assert!(
            check.is_sound(size),
            "the seed must be sound before anything is timed: {}",
            check.line()
        );
        println!(
            "SEED\tsize={size}\t{}\tdb_bytes={}",
            check.line(),
            std::fs::metadata(&path).map(|meta| meta.len()).unwrap_or(0),
        );

        // ---- instrument A: the guard alone ---------------------------------
        for (label, tags, boundary) in SCENARIOS {
            let query = boundary_query(tags);
            let at = boundary.at(size);
            let mut samples: Vec<Vec<u128>> = vec![Vec::new(); Shape::ALL.len()];
            let mut transactions: Vec<Vec<u128>> = vec![Vec::new(); Shape::ALL.len()];

            for round in 0..rounds {
                let mut verdicts: Vec<(Shape, Option<i64>)> = Vec::new();
                // The starting shape rotates, so each shape runs first in a
                // quarter of the rounds and no median is entirely about going
                // second.
                for offset in 0..Shape::ALL.len() {
                    let index = (round + offset) % Shape::ALL.len();
                    let shape = Shape::ALL[index];
                    transactions[index].push(timed_transaction(&connection));
                    let (us, verdict) = timed_guard(&connection, shape, &query, at);
                    samples[index].push(us);
                    verdicts.push((shape, verdict));
                }
                let first = verdicts[0];
                for (shape, verdict) in &verdicts {
                    assert_eq!(
                        *verdict,
                        first.1,
                        "{label} at {size}: {} disagreed with {} — one of these \
                         shapes is deciding less",
                        shape.name(),
                        first.0.name(),
                    );
                }
            }

            for (index, shape) in Shape::ALL.iter().enumerate() {
                let (median, p10, max) = spread(&mut samples[index]);
                let (txn, _, _) = spread(&mut transactions[index]);
                println!(
                    "GUARD\tsize={size}\tshape={}\tscenario={label}\tboundary={at}\t\
                     us_median={median}\tus_p10={p10}\tus_max={max}\t\
                     txn_only_us_median={txn}\tguard_us_median={}\trounds={rounds}\t{}",
                    shape.name(),
                    median.saturating_sub(txn),
                    conditions.line(),
                );
            }
        }

        // ---- instrument B: the whole call, rejection path only --------------
        for (label, tags, boundary) in SCENARIOS {
            if !label.starts_with("rejected") {
                continue;
            }
            let guard = boundary.condition(boundary_query(tags), size);
            let mut names: Vec<&str> = Shape::ALL.iter().map(|shape| shape.name()).collect();
            names.push("real-adapter");
            let mut arms: Vec<&dyn RejectingStore> = probes
                .iter()
                .map(|probe| probe as &dyn RejectingStore)
                .collect();
            arms.push(&real as &dyn RejectingStore);

            let mut samples: Vec<Vec<u128>> = vec![Vec::new(); arms.len()];
            for round in 0..rounds {
                let mut named: Vec<Option<SequencePosition>> = vec![None; arms.len()];
                for offset in 0..arms.len() {
                    let index = (round + offset) % arms.len();
                    let started = Instant::now();
                    let at = arms[index].reject(&event, &guard);
                    samples[index].push(started.elapsed().as_micros());
                    named[index] = at;
                }
                for (index, at) in named.iter().enumerate() {
                    assert_eq!(
                        *at, named[0],
                        "{label} at {size}: {} named a different conflicting \
                         position from {}",
                        names[index], names[0],
                    );
                }
            }

            for (index, name) in names.iter().enumerate() {
                let (median, p10, max) = spread(&mut samples[index]);
                println!(
                    "APPEND\tsize={size}\tshape={name}\tscenario={label}\t\
                     us_median={median}\tus_p10={p10}\tus_max={max}\trounds={rounds}\t{}",
                    conditions.line(),
                );
            }
        }
    }

    // Nothing above committed, so the log is still exactly the last size seeded —
    // which the real adapter is asked to confirm, because a probe store that had
    // quietly written a different schema would have failed here rather than in
    // `results/`.
    let head = block_on(EventStore::head(&real))
        .expect("head must succeed")
        .expect("the store is not empty");
    let last = SIZES[SIZES.len() - 1].0;
    assert_eq!(
        head.get(),
        last,
        "the timed runs must not have moved the log"
    );
    println!("READBACK\tadapter=happenstance-sqlite\thead={}", head.get());

    drop(real);
    drop(probes);
    drop(connection);
    drop(writer);
    remove(&path);
}

fn tempdb(name: &str) -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "happenstance-shipped-guard-{}-{name}.sqlite3",
        std::process::id()
    ));
    remove(&path);
    path
}

fn remove(path: &std::path::Path) {
    for suffix in ["", "-wal", "-shm"] {
        let mut candidate = path.to_path_buf().into_os_string();
        candidate.push(suffix);
        let _ = std::fs::remove_file(std::path::PathBuf::from(candidate));
    }
}
