//! **The headline figure: what happenstance costs over the database you
//! already run.**
//!
//! # Why this is a binary and not a criterion group
//!
//! Because it reports **ratios**, and criterion cannot produce an honest one
//! here. It runs the members of a group sequentially, each to completion, so
//! anything that drifts across a run lands entirely on whichever arm was
//! running at the time. This repository has measured that drift twice, and both
//! times it was larger than the effect: **2.7× at one client and 3.0× at 64**
//! in the position-visibility experiment (`RUNBOOK.md:1628-1634`), and **45%
//! between two runs an hour apart** with a **4× swing on a path no arm
//! participated in** in ADR-0022 (`:120-144`).
//!
//! So every arm here is interleaved round-robin in one process by
//! [`happenstance_benchmarks::paired`], which also reports each arm's own drift
//! so a reader can see whether the run was stable rather than assume it.
//!
//! # The question, in the seed's own words
//!
//! `references/seeds/measured-not-claimed.md:100-104`:
//!
//! > The question an adopter actually asks is not how fast happenstance is; it
//! > is **what happenstance costs over the database they already run**. That
//! > number — the overhead above a raw insert on the same file or the same
//! > instance — is the one nobody can dispute and nobody can be sold, and it
//! > has never been taken.
//!
//! # Two floors, and both ratios are published
//!
//! * **against `raw/same-schema`** — hand-written SQL on a file the *adapter*
//!   migrated. This isolates the cost of the adapter's Rust: query planning,
//!   tag dedup, the connection mutex, the paged read, the `spawn_blocking` hop.
//! * **against `raw/hand-rolled`** — the single-table event log an engineer
//!   writes on day one, with no `event_tag` join table. This prices the library
//!   *and its schema* together.
//!
//! Publishing only the first would price the adapter against a schema chosen
//! for capabilities the hand-rolled table does not have. Publishing only the
//! second would charge the library for a schema decision ADR-0022 took
//! deliberately. `src/fixtures/raw.rs` argues both at length.
//!
//! **Both floors are deliberately cheaper than a correct store** — no event id,
//! no recorded time, no `tag_cardinality` upsert, no append condition. That
//! makes each a genuine lower bound, and therefore makes the ratio above it an
//! *upper* bound on what happenstance costs. A "fair" floor that re-implemented
//! half the adapter would produce a smaller, more flattering number and a
//! weaker claim.
//!
//! # Usage
//!
//! ```console
//! cargo run --release --bin overhead
//! ```
//!
//! Writes CSV to stdout. `run.sh` tees it into `results/raw/overhead.csv`.
//! Nothing here asserts on a ratio; CF-34 forbids a benchmark result gating a
//! merge, and this binary exits zero whatever it measures.

use std::cell::RefCell;

use happenstance_benchmarks::corpus::{Corpus, Regime, Shape};
use happenstance_benchmarks::fixtures::memory::MemoryFixture;
use happenstance_benchmarks::fixtures::raw::{Floor, RawStore};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::paired::{self, Operation, PairedReport};
use happenstance_benchmarks::report::{Row, RunRecord};
use happenstance_benchmarks::runtime;
use happenstance_core::{EventStore, Query, ReadOptions, collect};
use happenstance_testkit::Fixture;

/// Rounds each paired scenario records, after warm-up.
///
/// Two hundred: enough for a stable median and for the drift halves to have
/// a hundred samples each, and few enough that the whole binary stays inside
/// `run.sh`'s stated budget on the SQLite arms, where one round is a real
/// `fsync`.
const ROUNDS: usize = 200;

/// The batch every append arm writes.
///
/// Eight events, three tags, a 256-byte payload, in the **owned** regime — the
/// regime a caller is actually in. An interned corpus would make the library's
/// arms 66 heap operations per event cheaper and the floors unaffected, which
/// would flatter the ratio for a reason that has nothing to do with the store.
const BATCH: usize = 8;

/// How many events the replay arms read back.
const REPLAY: usize = 2_000;

/// The corpus every arm here uses.
fn corpus() -> Corpus {
    Corpus::distinct(Shape::new(256, 3, Regime::Owned))
}

fn main() {
    let mut record = RunRecord::new();
    eprintln!("{}\n", record.conditions);

    append_overhead(&mut record);
    replay_overhead(&mut record);
    cross_adapter(&mut record);

    print!("{}", record.to_csv());
}

/// Turns a finished paired report into rows, and prints it for the raw log.
///
/// The ratio rows are the point; the per-arm absolutes are carried too, marked
/// **not representative** when the run drifted or when an arm sat inside the
/// timer's own noise. A row that cannot be a headline is still committed — it
/// is what makes the ratio's provenance checkable — but a reader meets it
/// labelled.
fn record_paired(
    record: &mut RunRecord,
    group: &str,
    shape: &str,
    report: &PairedReport,
    baselines: &[&str],
) {
    eprintln!("[{group}] {report}");

    for arm in &report.arms {
        let quotable = report.is_stable() && !arm.is_timer_dominated();
        for (metric, value) in [
            ("median", arm.median_nanos),
            ("p95", arm.p95_nanos),
            ("p99", arm.p99_nanos),
        ] {
            record.push(Row {
                group: group.to_owned(),
                arm: arm.name.to_owned(),
                shape: shape.to_owned(),
                metric: metric.to_owned(),
                #[allow(clippy::cast_precision_loss)]
                value: value as f64,
                unit: "ns".to_owned(),
                representative: quotable,
            });
        }

        // Every baseline this scenario reports against, from the **one**
        // interleaved run. Two `paired::run` calls would give two ratios that
        // could have drifted apart from each other, which is the failure this
        // whole binary exists to avoid.
        for baseline in baselines {
            if arm.name == *baseline {
                continue;
            }
            let Some(ratio) = report.ratio(arm.name, baseline) else {
                continue;
            };
            eprintln!("[{group}] {} / {baseline} = {ratio:.2}x", arm.name);
            record.push(Row {
                group: group.to_owned(),
                arm: arm.name.to_owned(),
                shape: shape.to_owned(),
                metric: format!("ratio-over/{baseline}"),
                value: ratio,
                // A ratio survives drift — that is what interleaving buys — so
                // it is representative even when the absolutes beside it are
                // not. It is *not* representative when the baseline itself sat
                // in the timer's noise, because then the denominator is
                // instrument.
                unit: "x".to_owned(),
                representative: report
                    .arm(baseline)
                    .is_some_and(|base| !base.is_timer_dominated()),
            });
        }
    }
}

/// `SqliteEventStore::append` against both raw floors.
///
/// The single most important measurement in this crate.
fn append_overhead(record: &mut RunRecord) {
    let corpus = corpus();
    let batch = corpus.batch(BATCH);

    // Each arm owns its own fixture and store for the whole run, so no arm pays
    // another's setup. `RefCell` because `paired::Operation` takes `FnMut` and
    // the raw store needs `&mut self` to append.
    let sqlite_fixture = SqliteFixture::new();
    let sqlite_store = runtime::block_on(sqlite_fixture.connect());

    let same_schema_fixture = SqliteFixture::new();
    let same_schema = RefCell::new(
        RawStore::over(&same_schema_fixture, Floor::SameSchema).expect("the floor opens"),
    );

    let hand_rolled_fixture = SqliteFixture::new();
    let hand_rolled = RefCell::new(
        RawStore::over(&hand_rolled_fixture, Floor::HandRolled).expect("the floor opens"),
    );

    let mut operations = [
        Operation::new("raw/same-schema", || {
            same_schema
                .borrow_mut()
                .append(&batch)
                .expect("the floor appends");
        }),
        Operation::new("raw/hand-rolled", || {
            hand_rolled
                .borrow_mut()
                .append(&batch)
                .expect("the floor appends");
        }),
        Operation::new("happenstance-sqlite", || {
            runtime::block_on(sqlite_store.append(&batch, None)).expect("the append lands");
        }),
    ];

    let report = paired::run(ROUNDS, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    record_paired(
        record,
        "overhead/append",
        &format!("owned/256B/3tags/batch-{BATCH}"),
        &report,
        &["raw/same-schema", "raw/hand-rolled"],
    );
}

/// An unfiltered replay of a seeded log, against the same-schema floor.
///
/// The hand-rolled floor is left out here on purpose: its read is a `SELECT`
/// over one table where the adapter's is a paged read through
/// `spawn_blocking`, and the two differ in what they *return* — the adapter
/// yields whole `SequencedEvent`s with identity and recorded time, the floor
/// yields three columns. Comparing them on the append path is fair because
/// both write a row; comparing them on the read path would be comparing two
/// different results.
fn replay_overhead(record: &mut RunRecord) {
    let corpus = corpus();

    let sqlite_fixture = SqliteFixture::new();
    let sqlite_store = runtime::block_on(sqlite_fixture.connect());
    let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
    runtime::block_on(async {
        let mut written = 0;
        while written < REPLAY {
            let take = ceiling.min(REPLAY - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            sqlite_store
                .append(&chunk, None)
                .await
                .expect("seeding must land");
            written += take;
        }
    });
    sqlite_fixture.checkpoint();

    let floor_fixture = SqliteFixture::new();
    let floor =
        RefCell::new(RawStore::over(&floor_fixture, Floor::SameSchema).expect("the floor opens"));
    {
        let mut floor = floor.borrow_mut();
        let mut written = 0;
        while written < REPLAY {
            let take = 128.min(REPLAY - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            floor.append(&chunk).expect("the floor seeds");
            written += take;
        }
    }
    floor_fixture.checkpoint();

    let mut operations = [
        Operation::new("raw/same-schema", || {
            let seen = floor.borrow().read_all().expect("the floor replays");
            debug_assert_eq!(seen, REPLAY);
        }),
        Operation::new("happenstance-sqlite", || {
            let events = runtime::block_on(collect(
                sqlite_store.read(&Query::all(), ReadOptions::new()),
            ))
            .expect("the replay succeeds");
            debug_assert_eq!(events.len(), REPLAY);
        }),
    ];

    let report = paired::run(ROUNDS, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    record_paired(
        record,
        "overhead/replay",
        &format!("owned/256B/3tags/{REPLAY}-events"),
        &report,
        &["raw/same-schema"],
    );
}

/// `MemoryEventStore` against `SqliteEventStore`, on the same workload.
///
/// The cross-adapter comparison CF-34 names — *"benchmarks are published per
/// adapter and compared against that adapter's own history"* — and the one that
/// separates what the **contract** costs from what **storage** costs. The
/// memory arm does no I/O at all, so the gap between them is the price of
/// durability rather than the price of the abstraction.
fn cross_adapter(record: &mut RunRecord) {
    let corpus = corpus();
    let batch = corpus.batch(BATCH);

    let memory_fixture = MemoryFixture::new();
    let memory_store = runtime::block_on(memory_fixture.connect());

    let sqlite_fixture = SqliteFixture::new();
    let sqlite_store = runtime::block_on(sqlite_fixture.connect());

    let mut operations = [
        Operation::new("memory", || {
            runtime::block_on(memory_store.append(&batch, None))
                .expect("MemoryEventStore's error type is uninhabited");
        }),
        Operation::new("sqlite", || {
            runtime::block_on(sqlite_store.append(&batch, None)).expect("the append lands");
        }),
    ];

    let report = paired::run(ROUNDS, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    record_paired(
        record,
        "cross-adapter/append",
        &format!("owned/256B/3tags/batch-{BATCH}"),
        &report,
        &["memory"],
    );
}
