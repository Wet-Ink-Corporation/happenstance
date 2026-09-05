//! Append throughput: what it costs to put *n* events into a log, and how far
//! above raw SQL that is.
//!
//! # What is varied, and why these values
//!
//! Batch size, payload size and tag count, each swept to the specification's
//! own conformance floor rather than to a convenient round number — VT-24's
//! 128 events, VT-21's 64 KiB payload, VT-22's 64 tags
//! (`crate::corpus`'s constants carry the clause citations). Every point is
//! taken in both allocation regimes, and the interned one is labelled a control
//! wherever it appears: see `crate::corpus`'s module documentation for why a
//! suite that measured only the interned regime would report a 66× cheaper
//! clone as the library's.
//!
//! # Why `iter_custom` rather than `iter` or `iter_batched`
//!
//! Three reasons, and the first is not negotiable.
//!
//! 1. **`SqliteEventStore` captures `Handle::try_current()` at construction**
//!    (`crates/happenstance-sqlite/src/event_store.rs:326-331`), so a store
//!    built in a synchronous `iter_batched` setup closure carries `None` and
//!    its first `read` fails with `NoRuntime`. `iter_custom` lets the whole
//!    block — construction included — run inside one `block_on`.
//! 2. **The timer brackets the loop, not each iteration.** At the memory arm's
//!    scale an append is around 900 ns and one `Instant::now()` pair is around
//!    50 ns; per-iteration timing would put 6% of instrument into the figure.
//!    `crate::paired` pays that cost deliberately, because a *ratio* needs
//!    interleaving; an absolute does not.
//! 3. **Setup is excluded explicitly rather than by criterion's estimate.**
//!    Opening a `SqliteEventStore` converts the journal to WAL, sets three
//!    pragmas and runs migration 1 — real work that has nothing to do with
//!    appending.
//!
//! # The log grows inside a measured block, and that is stated
//!
//! Each block starts on a fresh, empty store and appends `iters` times into it,
//! so the log is longer at the end of a block than at the start. For the memory
//! arm this changes nothing measurable — an unconditional append is a `Vec`
//! push and evaluates no condition. For the SQLite arm the index deepens
//! slightly. Both figures are therefore *"append into an initially empty log"*,
//! and the effect of log length gets its own arms in `store_replay.rs` and
//! `store_query_shapes.rs`, where the log is seeded to a stated size.
//!
//! # This file produces absolutes only
//!
//! Any ratio a reader wants — SQLite over the raw floor, memory over SQLite —
//! comes from `src/bin/overhead.rs`, which interleaves the arms round-robin in
//! one process. criterion runs group members sequentially, and on this class of
//! host sequential arms have drifted 2.7–3.0× (`RUNBOOK.md:1628-1634`).

use std::hint::black_box;
use std::time::{Duration, Instant};

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use happenstance_benchmarks::corpus::{
    Corpus, FLOOR_EVENTS_PER_BATCH, FLOOR_TAGS_PER_EVENT, Regime, Shape,
};
use happenstance_benchmarks::fixtures::memory::MemoryFixture;
use happenstance_benchmarks::fixtures::raw::{Floor, RawStore};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{Event, EventStore};
use happenstance_testkit::Fixture;

/// Batch sizes swept, ending at VT-24's conformance floor.
///
/// One because a single-event append is the commonest shape an application
/// writes; 128 because it is what every conformant store must accept, and it is
/// where the per-batch costs a store amortises stop being invisible.
const BATCH_SIZES: [usize; 4] = [1, 8, 32, FLOOR_EVENTS_PER_BATCH];

/// Payload sizes swept, ending at VT-21's conformance floor.
const PAYLOAD_SIZES: [usize; 3] = [64, 1_024, 65_536];

/// Tag counts swept, ending at VT-22's conformance floor.
const TAG_COUNTS: [usize; 3] = [0, 3, FLOOR_TAGS_PER_EVENT];

/// The batch size the payload and tag sweeps hold fixed.
///
/// Eight rather than one, so the per-batch costs a store amortises — one
/// `BEGIN IMMEDIATE`, one prepared statement, one commit — are spread the way a
/// real writer spreads them, and the axis being swept is the one that moves.
const FIXED_BATCH: usize = 8;

/// How many samples a SQLite group takes.
///
/// Below criterion's default of 100 because every iteration of the SQLite arm
/// creates a database file, converts its journal to WAL and runs migration 1 in
/// *untimed* setup — cheap per iteration, and minutes of wall clock across a
/// full sweep at the default. The confidence interval widens; the figures here
/// are quoted as orders of magnitude and ratios, never to a third significant
/// figure, so it is a cost worth paying to keep `./run.sh` inside its stated
/// budget.
const SQLITE_SAMPLES: usize = 30;

/// Appends `batch` `iters` times into a fresh in-memory store, timing only the
/// appends.
fn time_memory_appends(iters: u64, batch: &[Event]) -> Duration {
    runtime::block_on(async {
        let fixture = MemoryFixture::new();
        let store = fixture.connect().await;

        let started = Instant::now();
        for _ in 0..iters {
            let position = store
                .append(batch, None)
                .await
                .expect("MemoryEventStore's error type is uninhabited");
            black_box(position);
        }
        started.elapsed()
    })
}

/// Appends `batch` `iters` times into a fresh SQLite store, timing only the
/// appends.
///
/// The fixture is built and connected inside the `block_on`, so the store
/// captures a live runtime handle. It is dropped at the end of the block, which
/// removes the database file and both WAL sidecars — a sweep would otherwise
/// leave a file per sample in the temporary directory.
fn time_sqlite_appends(iters: u64, batch: &[Event]) -> Duration {
    runtime::block_on(async {
        let fixture = SqliteFixture::new();
        let store = fixture.connect().await;

        let started = Instant::now();
        for _ in 0..iters {
            let position = store.append(batch, None).await.expect("the append lands");
            black_box(position);
        }
        started.elapsed()
    })
}

/// Appends `batch` `iters` times through raw SQL, timing only the appends.
fn time_floor_appends(iters: u64, batch: &[Event], floor: Floor) -> Duration {
    let fixture = SqliteFixture::new();
    let mut store = RawStore::over(&fixture, floor).expect("the floor opens");

    let started = Instant::now();
    for _ in 0..iters {
        let position = store.append(batch).expect("the floor appends");
        black_box(position);
    }
    started.elapsed()
}

/// Batch size against throughput, at a fixed payload and tag count.
///
/// The headline sweep: `Throughput::Elements` makes criterion report
/// events/second rather than seconds/batch, which is the number a reader
/// actually wants and the one a batch-size sweep is otherwise easy to misread.
fn batch_size(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("append/batch-size");
    group.sample_size(SQLITE_SAMPLES);

    for regime in Regime::BOTH {
        for size in BATCH_SIZES {
            let corpus = Corpus::uniform(Shape::new(1_024, 3, regime));
            let batch = corpus.batch(size);
            group.throughput(Throughput::Elements(size as u64));

            let label = |arm: &str| format!("{arm}/{}", regime.label());

            group.bench_with_input(
                BenchmarkId::new(label("memory"), size),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_memory_appends(iters, batch)),
            );
            group.bench_with_input(
                BenchmarkId::new(label("sqlite"), size),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_sqlite_appends(iters, batch)),
            );
            for floor in Floor::BOTH {
                group.bench_with_input(
                    BenchmarkId::new(label(floor.label()), size),
                    &batch,
                    |bencher, batch| {
                        bencher.iter_custom(|iters| time_floor_appends(iters, batch, floor));
                    },
                );
            }
        }
    }

    group.finish();
}

/// Payload size against throughput, at a fixed batch size and tag count.
///
/// `Throughput::Bytes` here rather than `Elements`, so the report is MB/s — the
/// axis on which a payload sweep is legible. The interned regime is included
/// and is expected to *win*, because `Bytes::from_static` allocates nothing;
/// that gap is the measurement, not a nuisance.
fn payload_size(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("append/payload-size");
    group.sample_size(SQLITE_SAMPLES);

    for regime in Regime::BOTH {
        for payload in PAYLOAD_SIZES {
            let corpus = Corpus::uniform(Shape::new(payload, 3, regime));
            let batch = corpus.batch(FIXED_BATCH);
            group.throughput(Throughput::Bytes((payload * FIXED_BATCH) as u64));

            let label = |arm: &str| format!("{arm}/{}", regime.label());

            group.bench_with_input(
                BenchmarkId::new(label("memory"), payload),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_memory_appends(iters, batch)),
            );
            group.bench_with_input(
                BenchmarkId::new(label("sqlite"), payload),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_sqlite_appends(iters, batch)),
            );
            group.bench_with_input(
                BenchmarkId::new(label(Floor::SameSchema.label()), payload),
                &batch,
                |bencher, batch| {
                    bencher
                        .iter_custom(|iters| time_floor_appends(iters, batch, Floor::SameSchema));
                },
            );
        }
    }

    group.finish();
}

/// Tag count against throughput, at a fixed batch and payload size.
///
/// **The arm the regime axis exists for.** Every tag is a row in `event_tag`
/// and an upsert in `tag_cardinality`, both inside the write transaction — and
/// in the owned regime every tag is also `t + 2` heap operations before the
/// store is even reached. At `FLOOR_TAGS_PER_EVENT` the two regimes are 66
/// operations apart per event (`tests/instruments_work.rs` reproduces the
/// figure), so a suite that reported only the interned arm here would price the
/// tag index at a fraction of what a caller pays.
fn tag_count(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("append/tag-count");
    group.sample_size(SQLITE_SAMPLES);

    for regime in Regime::BOTH {
        for tags in TAG_COUNTS {
            let corpus = Corpus::uniform(Shape::new(1_024, tags, regime));
            let batch = corpus.batch(FIXED_BATCH);
            group.throughput(Throughput::Elements(FIXED_BATCH as u64));

            let label = |arm: &str| format!("{arm}/{}", regime.label());

            group.bench_with_input(
                BenchmarkId::new(label("memory"), tags),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_memory_appends(iters, batch)),
            );
            group.bench_with_input(
                BenchmarkId::new(label("sqlite"), tags),
                &batch,
                |bencher, batch| bencher.iter_custom(|iters| time_sqlite_appends(iters, batch)),
            );
            group.bench_with_input(
                BenchmarkId::new(label(Floor::SameSchema.label()), tags),
                &batch,
                |bencher, batch| {
                    bencher
                        .iter_custom(|iters| time_floor_appends(iters, batch, Floor::SameSchema));
                },
            );
        }
    }

    group.finish();
}

/// What building the batch costs, before any store is reached.
///
/// ES-17 (`spec/SPECIFICATION.md:3345`) keeps `append`'s batch **borrowed**, and
/// its `[PROVISIONAL]` marker is falsified by *"a measurement on a real adapter
/// showing the per-event clone is a material fraction of append cost"*. That
/// measurement needs a denominator and a numerator: the append arms above are
/// the denominator, and this is the numerator.
///
/// It is deliberately **not** an answer to ES-17. The clause's own falsifier
/// asks for two builds of one adapter differing only in `append`'s ownership,
/// measured on one harness (`.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md`),
/// and nothing here is that. What this arm supplies is the cost of the clone in
/// both regimes, at the tag floor, so that whoever does build those two arms
/// knows what fraction they are looking for — and knows that the answer differs
/// by 66× depending on a choice a benchmark author makes without noticing.
fn batch_construction(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("append/batch-construction");

    for regime in Regime::BOTH {
        for tags in TAG_COUNTS {
            let corpus = Corpus::uniform(Shape::new(1_024, tags, regime));
            group.throughput(Throughput::Elements(FLOOR_EVENTS_PER_BATCH as u64));

            group.bench_with_input(
                BenchmarkId::new(format!("build/{}", regime.label()), tags),
                &corpus,
                |bencher, corpus| {
                    bencher.iter(|| black_box(corpus.batch(FLOOR_EVENTS_PER_BATCH)));
                },
            );

            // And the clone the borrowed-batch signature is designed to avoid,
            // measured on a batch that already exists. `iter_batched` with a
            // reference setup so the clone is timed and the drop is not.
            group.bench_with_input(
                BenchmarkId::new(format!("clone/{}", regime.label()), tags),
                &corpus,
                |bencher, corpus| {
                    bencher.iter_batched(
                        || corpus.batch(FLOOR_EVENTS_PER_BATCH),
                        |batch| black_box(batch.clone()),
                        BatchSize::SmallInput,
                    );
                },
            );
        }
    }

    group.finish();
}

/// What one `connect` costs, which every other arm excludes.
///
/// `MemoryFixture::connect` is a refcount bump; `SqliteFixture::connect` opens a
/// file, converts its journal to WAL, sets three pragmas and runs migration 1.
/// Every arm above excludes that from its timed region, so it is measured here
/// once rather than left as an unstated exclusion — a cost a consumer pays on
/// every process start, and the one an edge deployment with a cold start pays
/// on every request.
fn connect(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("append/connect");
    group.sample_size(SQLITE_SAMPLES);

    group.bench_function("memory", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let fixture = MemoryFixture::new();
                let started = Instant::now();
                for _ in 0..iters {
                    black_box(fixture.connect().await);
                }
                started.elapsed()
            })
        });
    });

    // A fresh file per iteration, because the second `open` onto an
    // already-migrated file does materially less work than the first — and the
    // first is what a consumer's process start actually pays.
    group.bench_function("sqlite/first-open", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let mut total = Duration::ZERO;
                for _ in 0..iters {
                    let fixture = SqliteFixture::new();
                    let started = Instant::now();
                    black_box(fixture.connect().await);
                    total += started.elapsed();
                }
                total
            })
        });
    });

    group.bench_function("sqlite/subsequent-open", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let fixture = SqliteFixture::new();
                black_box(fixture.connect().await);
                let started = Instant::now();
                for _ in 0..iters {
                    black_box(fixture.connect().await);
                }
                started.elapsed()
            })
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    batch_size,
    payload_size,
    tag_count,
    batch_construction,
    connect
);
criterion_main!(benches);
