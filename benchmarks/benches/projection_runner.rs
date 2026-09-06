//! The projection runner: catch-up throughput, and the one knob it has.
//!
//! # Why chunk size is the whole sweep
//!
//! `run_projection` does one read for the whole run and pulls the stream item
//! by item, holding at most `chunk` events between `begin` and `commit`
//! (`crates/happenstance/src/runner.rs:416`). That is its **only** throughput
//! knob, and PS-13/PS-14 require the *result* to be identical at every value of
//! it — `rebuild_is_chunk_size_invariant` is a conformance rule.
//!
//! So the correctness question is settled and the cost question is not. The
//! sizes swept here are E2E-22's own — *"replayed at chunk sizes 1, 100 and
//! 5,000"* (`spec/E2E-CASES.md:577-598`) — because that case rejects *"any
//! runner whose chunk size is a tuning parameter"*, and a benchmark that swept
//! different numbers could not speak to it.
//!
//! `references/scenarios/README.md:665-670` is the deployment that makes it
//! urgent: Wattline's 210-million-event backfill opened a **40-minute
//! transaction** that froze the visibility watermark for every handler in the
//! cluster. *"Chunking it into bounded transactions is the mitigation, and
//! `begin`/`commit` is the whole port surface, so chunk size is the runner's
//! business and every adapter's default is wrong for this."*
//!
//! # Two stores, because one batch shape is not the axis
//!
//! `SqliteProjectionStore`'s batch is a **buffer of deferred SQL** applied at
//! commit inside `spawn_blocking`; `MemoryProjectionStore`'s is a map written
//! into directly. A runner figure taken against only one of them is a figure
//! about that batch shape rather than about the runner — which is CF-25's
//! anti-monoculture argument applied one port over
//! (`spec/SPECIFICATION.md:8461`).
//!
//! # The idle poll is measured, but the fan-out is not
//!
//! ES-32 (`spec/SPECIFICATION.md:4087`) keeps `EventStore` without a tail or
//! subscribe method at 0.1 — **consumers poll** — and its `[PROVISIONAL]`
//! marker names the fan-out runner's staleness as the falsifier.
//! `experiments/polling-cost/` has already answered the fan-out half:
//! **delivery amplification of 32.00 at 32 views**, exactly the fan-out, with
//! no economy of scale anywhere in the range.
//!
//! What is *not* answered there and is measured here is the single-runner idle
//! cost: what one `run_projection` costs when the query nominates nothing above
//! the checkpoint. That is the figure an operator multiplies by their poll
//! interval to size a background loop, and it is the one number in this file a
//! reader is likely to act on directly.

use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use happenstance::{DomainEvent, Json, Progressed, run_projection};
use happenstance_benchmarks::domain::{MemoryTotals, Recorded, SqliteTotals};
use happenstance_benchmarks::fixtures::memory::{MemoryFixture, MemoryHandle};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{Event, EventStore, MemoryProjectionStore};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use happenstance_testkit::Fixture;

/// Chunk sizes swept. E2E-22's own three, and no others — see the module docs.
const CHUNK_SIZES: [usize; 3] = [1, 100, 5_000];

/// How many events a catch-up run has to get through.
///
/// Ten thousand: twice the largest chunk, so the largest chunk is exercised
/// more than once rather than degenerating into a single transaction; and small
/// enough that the SQLite arm's seeding stays inside `run.sh`'s stated budget.
const LOG: usize = 10_000;

/// The account every projected event is tagged for.
const ACCOUNT: &str = "a1";

/// Samples per group.
///
/// Ten, and the **event** store is seeded once per group rather than once per
/// sample. `store_replay.rs`'s module documentation records what the other
/// arrangement cost: criterion calls `iter_custom` once per sample, so a
/// ten-thousand-event seed inside it is paid ten times over for no gain — a
/// catch-up only ever reads the log.
///
/// The **projection** store is still fresh per iteration, and that is not an
/// oversight: a run that reused it would find its checkpoint already at the
/// head and measure an idle poll, which is the next group's figure and not this
/// one's.
const SEEDED_SAMPLES: usize = 10;

/// One contract-level event carrying an encoded [`Recorded`].
///
/// Built through `DomainEvent::encode` rather than by hand, so the runner
/// decodes bytes the typed layer actually writes — a corpus encoded some other
/// way would measure a decode of a shape no application produces.
///
/// # Panics
///
/// Panics if the benchmark's own event cannot be built or encoded.
fn projected_event(amount: u32) -> Event {
    let event = Recorded::new(ACCOUNT, amount, 64);
    Event::new(
        event.event_type(),
        event.encode(&Json).expect("json encodes"),
    )
    .expect("the benchmark's own event type is valid")
    .with_tags(event.tags())
}

/// Seeds [`LOG`] events into `store`, chunked to `ceiling`.
async fn seed(store: &impl EventStore, ceiling: usize) {
    let mut written = 0;
    while written < LOG {
        let take = ceiling.min(LOG - written);
        let batch: Vec<Event> = (written..written + take)
            .map(|n| projected_event(u32::try_from(n % 1_000).expect("the modulus fits a u32")))
            .collect();
        store
            .append(&batch, None)
            .await
            .unwrap_or_else(|_| panic!("seeding {take} events must land"));
        written += take;
    }
}

/// A memory event store seeded to [`LOG`], built once per group.
fn seeded_memory_events() -> MemoryHandle {
    runtime::block_on(async {
        let fixture = MemoryFixture::new();
        let store = fixture.connect().await;
        seed(&store, 128).await;
        store
    })
}

/// A SQLite event store seeded to [`LOG`], with its write-ahead log folded in.
///
/// The fixture is returned alongside the store and must outlive it: dropping it
/// removes the database file.
fn seeded_sqlite_events() -> (SqliteFixture, SqliteEventStore) {
    let fixture = SqliteFixture::new();
    let store = runtime::block_on(async {
        let store = fixture.connect().await;
        let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
        seed(&store, ceiling).await;
        store
    });
    fixture.checkpoint();
    (fixture, store)
}

/// A fresh SQLite projection store, with the application's own read-model table
/// applied.
///
/// A projection store's migration covers the **checkpoint** table and not an
/// application's read model — exactly the split
/// `examples/transfers-on-sqlite/src/main.rs:129` demonstrates, with three
/// handles onto one file.
fn fresh_projection_store() -> (SqliteFixture, SqliteProjectionStore) {
    let fixture = SqliteFixture::new();
    let models = SqliteProjectionStore::open(fixture.path()).expect("the projection store opens");
    rusqlite::Connection::open(fixture.path())
        .expect("the read model's own connection opens")
        .execute_batch(SqliteTotals::READ_MODEL_DDL)
        .expect("the read model's table is created");
    (fixture, models)
}

/// Catch-up over a seeded log, swept by chunk size, on both projection stores.
///
/// Each iteration builds a **fresh** projection store, so every run starts from
/// `Checkpoint::NeverRun` and replays the whole log.
fn catch_up(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("projection/catch-up");
    group.sample_size(SEEDED_SAMPLES);
    group.throughput(Throughput::Elements(LOG as u64));

    // Both event stores, seeded once. A catch-up reads and never writes, so one
    // seeded log serves every chunk size and every sample.
    let memory_events = seeded_memory_events();
    let (_sqlite_fixture, sqlite_events) = seeded_sqlite_events();

    for chunk in CHUNK_SIZES {
        let chunk_size = NonZeroUsize::new(chunk).expect("every swept chunk size is non-zero");

        group.bench_function(
            BenchmarkId::new("memory-events/memory-models", chunk),
            |bencher| {
                bencher.iter_custom(|iters| {
                    runtime::block_on(async {
                        let mut total = Duration::ZERO;
                        for _ in 0..iters {
                            let models = MemoryProjectionStore::new();
                            let mut projection = MemoryTotals::new(ACCOUNT)
                                .expect("the benchmark's account is valid");

                            let started = Instant::now();
                            let progressed = run_projection(
                                &memory_events,
                                &models,
                                &mut projection,
                                &Json,
                                chunk_size,
                            )
                            .await
                            .expect("the catch-up completes");
                            total += started.elapsed();
                            assert_applied(progressed);
                        }
                        total
                    })
                });
            },
        );

        group.bench_function(
            BenchmarkId::new("sqlite-events/sqlite-models", chunk),
            |bencher| {
                bencher.iter_custom(|iters| {
                    runtime::block_on(async {
                        let mut total = Duration::ZERO;
                        for _ in 0..iters {
                            // A fresh file per iteration: the read model and the
                            // checkpoint both have to start empty, and a
                            // `DELETE FROM` between iterations would leave the
                            // pages hot in a way a first run never sees.
                            let (_models_fixture, models) = fresh_projection_store();
                            let mut projection = SqliteTotals::new(ACCOUNT)
                                .expect("the benchmark's account is valid");

                            let started = Instant::now();
                            let progressed = run_projection(
                                &sqlite_events,
                                &models,
                                &mut projection,
                                &Json,
                                chunk_size,
                            )
                            .await
                            .expect("the catch-up completes");
                            total += started.elapsed();
                            assert_applied(progressed);
                        }
                        total
                    })
                });
            },
        );
    }

    group.finish();
}

/// What a poll costs when there is nothing new.
///
/// The figure an operator multiplies by their poll interval. The projection is
/// already caught up, so the run reads its checkpoint, issues a query bounded
/// below by it, finds nothing, and returns `Progressed { through: None,
/// applied: 0 }`.
///
/// `experiments/polling-cost/` measured the *fan-out* half of this — 32.00×
/// amplification at 32 views, no economy of scale — and this is the per-runner
/// half it multiplies.
fn idle_poll(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("projection/idle-poll");
    group.sample_size(SEEDED_SAMPLES);

    let chunk_size = NonZeroUsize::new(100).expect("100 is non-zero");
    let memory_events = seeded_memory_events();
    let (_sqlite_fixture, sqlite_events) = seeded_sqlite_events();

    group.bench_function("memory-events/memory-models", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let models = MemoryProjectionStore::new();
                let mut projection = MemoryTotals::new(ACCOUNT).expect("valid");
                // Catch up first, outside the timed region. What is measured is
                // the *second* run, which finds nothing.
                let caught_up =
                    run_projection(&memory_events, &models, &mut projection, &Json, chunk_size)
                        .await
                        .expect("the initial catch-up completes");
                assert_applied(caught_up);

                let started = Instant::now();
                for _ in 0..iters {
                    let progressed =
                        run_projection(&memory_events, &models, &mut projection, &Json, chunk_size)
                            .await
                            .expect("the idle poll completes");
                    black_box(progressed.applied);
                }
                started.elapsed()
            })
        });
    });

    group.bench_function("sqlite-events/sqlite-models", |bencher| {
        bencher.iter_custom(|iters| {
            runtime::block_on(async {
                let (models_fixture, models) = fresh_projection_store();
                let mut projection = SqliteTotals::new(ACCOUNT).expect("valid");
                let caught_up =
                    run_projection(&sqlite_events, &models, &mut projection, &Json, chunk_size)
                        .await
                        .expect("the initial catch-up completes");
                assert_applied(caught_up);
                models_fixture.checkpoint();

                let started = Instant::now();
                for _ in 0..iters {
                    let progressed =
                        run_projection(&sqlite_events, &models, &mut projection, &Json, chunk_size)
                            .await
                            .expect("the idle poll completes");
                    black_box(progressed.applied);
                }
                started.elapsed()
            })
        });
    });

    group.finish();
}

/// A catch-up that applied nothing has measured nothing.
///
/// Not a threshold — CF-34 forbids one — and not a timing. It fires when the
/// *instrument* has stopped working: a projection whose scope no longer matches
/// the corpus would complete instantly, report zero applied, and post the best
/// figure in the file.
fn assert_applied(progressed: Progressed) {
    assert_eq!(
        progressed.applied, LOG,
        "a catch-up that applied {} of {LOG} events is a measurement of a \
         projection whose scope stopped matching the corpus, not of a runner",
        progressed.applied
    );
}

criterion_group!(benches, catch_up, idle_poll);
criterion_main!(benches);
