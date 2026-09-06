//! Query shape: the arms where a wide query stops being free.
//!
//! # What this measures that `store_replay.rs` does not
//!
//! That file sweeps *log length* against a handful of fixed queries. This one
//! holds the log fixed and sweeps the **query** — because two of the three
//! costliest findings in the pre-publication review live in the query itself
//! and appear only at the specification's own floors.
//!
//! * **I-5, the quadratic tag dedup.** At VT-23's floor of 128 items with 128
//!   tags each — 16,384 tags in one query —
//!   `references/evaluation/review-pre-publication-2026-09-03.md:2685` measured
//!   `Selectivity::read_for` at **257,690 µs against a `BTreeSet`'s 6,424 µs
//!   (40.1×)**. The call happens **inside `BEGIN IMMEDIATE`** with the write
//!   lock held, so a quarter-second of pure-Rust planning is a quarter-second
//!   every other writer waits.
//! * **X-1, the parameter ceiling.** The query plan is chunked by *item* count
//!   and never by *parameter* count, so 400 items × 128 tags binds **51,600
//!   parameters against SQLite's 32,766** (`:1036`, `:2837`).
//!
//! Neither is asserted on. This suite has no thresholds (CF-34). What it does
//! is take the figures on every run, at the floors, so a repair or a regression
//! is visible in `results/history/` rather than in a review a year later.
//!
//! # The chunking boundary is measured either side of itself
//!
//! `SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT` is **400**, and it is
//! `pub` precisely so a test can compute the boundary rather than guess at it
//! (`crates/happenstance-sqlite/src/event_store.rs:277`). The item sweep
//! straddles it — 128 (VT-23's floor), 400 (exactly one statement), 401 (two) —
//! so the cost of crossing it is a difference between adjacent points rather
//! than a slope somebody has to infer. The value is **read from the adapter**,
//! not restated, so the sweep cannot drift away from the boundary it straddles.
//!
//! # Why the log is small here
//!
//! A thousand events. The question is what the *planner* costs, not what the
//! scan costs, and a large log would bury a 40× planning cost under I/O. The
//! scan's cost against log length is `store_replay.rs`'s.

use std::hint::black_box;
use std::time::Instant;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use happenstance_benchmarks::corpus::{self, Corpus, FLOOR_QUERY_ITEMS, Regime, Shape};
use happenstance_benchmarks::fixtures::memory::{MemoryFixture, MemoryHandle};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{EventStore, Query, ReadOptions, collect};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::Fixture;

/// The log every arm here reads. Small on purpose — see the module docs.
const LOG: usize = 1_000;

/// Samples per group.
///
/// Ten, and the two stores are seeded **once for the whole file** rather than
/// once per sample. `store_replay.rs`'s module documentation records what the
/// other arrangement cost. Nothing here appends, so one seeded store serves
/// every arm.
const SEEDED_SAMPLES: usize = 10;

/// Item counts, straddling the adapter's statement-chunking boundary.
fn item_counts() -> [usize; 4] {
    let width = SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT;
    [8, FLOOR_QUERY_ITEMS, width, width + 1]
}

/// The corpus every arm reads.
fn query_corpus() -> Corpus {
    Corpus::distinct(Shape::new(256, 3, Regime::Owned))
}

/// Seeds [`LOG`] events into `store`, chunked to `ceiling`.
async fn seed(store: &impl EventStore, corpus: &Corpus, ceiling: usize) {
    let mut written = 0;
    while written < LOG {
        let take = ceiling.min(LOG - written);
        let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
        store
            .append(&chunk, None)
            .await
            .unwrap_or_else(|_| panic!("seeding {take} events must land"));
        written += take;
    }
}

/// A memory store seeded to [`LOG`], built once for the whole file.
fn seeded_memory() -> MemoryHandle {
    let corpus = query_corpus();
    runtime::block_on(async {
        let fixture = MemoryFixture::new();
        let store = fixture.connect().await;
        seed(&store, &corpus, 128).await;
        store
    })
}

/// A SQLite store seeded to [`LOG`], with its write-ahead log folded in.
///
/// The fixture is returned alongside the store and must outlive it: dropping it
/// removes the database file.
fn seeded_sqlite() -> (SqliteFixture, SqliteEventStore) {
    let corpus = query_corpus();
    let fixture = SqliteFixture::new();
    let store = runtime::block_on(async {
        let store = fixture.connect().await;
        let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
        seed(&store, &corpus, ceiling).await;
        store
    });
    fixture.checkpoint();
    (fixture, store)
}

/// Times `iters` reads of `query`.
fn time_reads(iters: u64, store: &impl EventStore, query: &Query) -> std::time::Duration {
    runtime::block_on(async {
        let started = Instant::now();
        for _ in 0..iters {
            let events = collect(store.read(query, ReadOptions::new()))
                .await
                .expect("the read succeeds");
            black_box(events.len());
        }
        started.elapsed()
    })
}

/// Item count against read cost, at one tag per item.
///
/// The cheap axis: it isolates the per-item cost of the query plan from the
/// per-tag cost the next group sweeps. A store that chunks by item shows a step
/// at the boundary; one that does not shows a straight line.
fn item_count(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query/item-count");
    group.sample_size(SEEDED_SAMPLES);

    let memory = seeded_memory();
    let (_fixture, sqlite) = seeded_sqlite();

    for items in item_counts() {
        // One tag per item, so the parameter count grows linearly and stays
        // well inside SQLite's 32,766 ceiling — this arm is about items.
        let query = corpus::query_at_the_item_floor(items, 1);

        group.bench_function(BenchmarkId::new("memory", items), |bencher| {
            bencher.iter_custom(|iters| time_reads(iters, &memory, &query));
        });

        group.bench_function(BenchmarkId::new("sqlite", items), |bencher| {
            bencher.iter_custom(|iters| time_reads(iters, &sqlite, &query));
        });
    }

    group.finish();
}

/// Tags per item against read cost, at VT-23's item floor.
///
/// **The I-5 arm.** Total tags is `items × tags_per_item`, so at 128 items this
/// sweep reaches 128, 1,024 and 8,192 tags in one query. The measured quadratic
/// is in `Selectivity::read_for`, which the *append* path also calls inside the
/// write lock — so a cost visible here is a cost every concurrent writer pays
/// there.
///
/// The sweep stops below 128 tags per item deliberately: at 128 items × 128
/// tags the same shape binds 51,600 parameters against SQLite's 32,766 (X-1),
/// which is a *failure* rather than a slow read, and a benchmark that ran into
/// it would be timing an error path.
fn tags_per_item(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query/tags-per-item");
    group.sample_size(SEEDED_SAMPLES);

    let memory = seeded_memory();
    let (_fixture, sqlite) = seeded_sqlite();

    for tags in [1_usize, 8, 64] {
        let query = corpus::query_at_the_item_floor(FLOOR_QUERY_ITEMS, tags);

        // How many bound parameters this shape implies on the SQLite arm, at
        // three per tag row. Printed rather than asserted: it is the number X-1
        // is about, and a reader comparing two runs needs to see it move.
        println!(
            "query/tags-per-item: {FLOOR_QUERY_ITEMS} items x {tags} tags = {} tags, \
             ~{} bound parameters against SQLite's 32,766",
            FLOOR_QUERY_ITEMS * tags,
            FLOOR_QUERY_ITEMS * tags * 3
        );

        group.bench_function(BenchmarkId::new("memory", tags), |bencher| {
            bencher.iter_custom(|iters| time_reads(iters, &memory, &query));
        });

        group.bench_function(BenchmarkId::new("sqlite", tags), |bencher| {
            bencher.iter_custom(|iters| time_reads(iters, &sqlite, &query));
        });
    }

    group.finish();
}

/// What building a query costs before a store is reached.
///
/// The planner half of I-5, isolated: no I/O, no store, just `Query::from_items`
/// over the same shapes the arms above read with. If the quadratic is in
/// construction rather than in evaluation, this is where it shows, and it shows
/// without a database in the way.
///
/// It also prices what an application pays *per command* to describe its own
/// consistency boundary — a cost `store_append.rs`'s arms exclude by building
/// the query once outside the timed region.
fn query_construction(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("query/construction");

    for items in item_counts() {
        group.bench_function(BenchmarkId::new("one-tag-per-item", items), |bencher| {
            bencher.iter(|| black_box(corpus::query_at_the_item_floor(items, 1)));
        });
    }

    for tags in [1_usize, 8, 64] {
        group.bench_function(BenchmarkId::new("at-the-item-floor", tags), |bencher| {
            bencher.iter(|| {
                black_box(corpus::query_at_the_item_floor(FLOOR_QUERY_ITEMS, tags));
            });
        });
    }

    // And the two the specification names by hand, so a reader can see what an
    // ordinary query costs beside a pathological one.
    group.bench_function("query-all", |bencher| {
        bencher.iter(|| black_box(Query::all()));
    });
    group.bench_function("mixed-selectivity", |bencher| {
        bencher.iter(|| black_box(corpus::query_mixed_selectivity(0)));
    });

    group.finish();
}

criterion_group!(benches, item_count, tags_per_item, query_construction);
criterion_main!(benches);
