//! Conditional append: what a DCB guard costs, and what contention does to it.
//!
//! # The guard is the whole reason this library exists, and it is the expensive
//! part
//!
//! An unconditional append is an insert. A *conditional* one evaluates every
//! guard against the log first, inside `BEGIN IMMEDIATE`, with every other
//! writer queued behind it — so the guard's cost is not latency for one caller,
//! it is a serialisation point for all of them.
//!
//! Two measured figures set what these arms are looking for. Finding I-1
//! (`references/evaluation/review-pre-publication-2026-09-03.md:2540`): a
//! two-tag guard costs **296 ms at 500,000 events and 560 ms at 10^6**, inside
//! the write lock. Finding CN-1 (`:2575`): the tag ordering the adapter's own
//! rustdoc *requires* — most-selective-first — is **inverted** on the shipped
//! shape, at 511,054 µs against 11,839 µs, reproduced across six cells and two
//! runs.
//!
//! Neither is re-derived here. `experiments/shipped-append-condition-sql/` took
//! them against the SQL directly, which is the right instrument for a question
//! about a statement and is not bound by a standing suite's wall-clock budget.
//! What this file adds is the same question asked *through the port*, at log
//! lengths a `./run.sh` can afford, so a change in the guard strategy shows up
//! in the standing suite rather than waiting for someone to re-run a one-off.
//!
//! # The guard arms measure a **refused** append, and that is deliberate
//!
//! An append that lands makes the log longer, so a block of `iters` iterations
//! against a seeded log of *N* events would report the guard's cost at
//! `N + iters/2` rather than at *N*. `typed_command.rs`'s module documentation
//! records what that cost when it went unnoticed there: a factor of two
//! reported where the underlying cost differed by a factor of a thousand.
//!
//! The repair here is cheaper than a per-iteration restore, because a refused
//! append **writes nothing**. `AppendCondition::new(query)` with no anchor
//! fails if *anything* matches, so the guard is evaluated in full against the
//! whole seeded log and the store is left exactly as it was. Each figure is
//! therefore *"evaluate this guard against a log of exactly N events, and
//! refuse"* — which is also the path a losing DCB writer pays on every retry,
//! and the one this file exists to price.
//!
//! The unconditional denominator is **not** here, for the same reason: it
//! cannot avoid appending. It is `store_append.rs`'s figure at the same batch
//! shape, and the two files are comparable because both are criterion
//! absolutes from the same host and the same run.
//!
//! # Contention is interleaved on one thread, and that is deliberate
//!
//! The contended arm drives
//! `happenstance_testkit::bench::scenarios::conditional_append_under_contention`,
//! which builds *k* append futures before polling any of them and then drives
//! them round-robin on one thread. It binds `Fixture` and nothing more — no
//! `Send` — so the shape stays honest for the `!Send` flavour the whole
//! two-trait port design pays for
//! (`crates/happenstance-testkit/src/bench.rs:63-71`).
//!
//! **OS-thread contention is a different measurement and is not taken here.**
//! An adapter that wants it supplies it from its own emitter; saying so is
//! better than quietly reporting single-thread interleaving as if it were
//! thread contention.
//!
//! # The rejection mix is reported, not averaged away
//!
//! ADR-0012 asked for *"a realistic batch and rejection mix"*, and the testkit's
//! record keeps `Committed`, `Rejected` (a `ConditionViolated`, which is the DCB
//! retry signal), `Refused` (a stated ceiling) and `Failed` (a store error)
//! apart. **A run in which every contender wins is a valid measurement of the
//! wrong thing.** The `contention` group prints the mix beside the timing on
//! every run, so that case is visible instead of being averaged into a
//! throughput number.

use std::hint::black_box;
use std::time::Instant;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use happenstance_benchmarks::corpus::{self, Corpus, Regime, Shape};
use happenstance_benchmarks::fixtures::memory::MemoryFixture;
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{AppendCondition, AppendError, EventStore, Query, QueryItem, Tags};
use happenstance_testkit::Fixture;
use happenstance_testkit::bench::{BenchmarkParams, scenarios};

/// Log lengths the guard is evaluated against.
///
/// The guard's cost is a function of what the log holds, not of what the batch
/// holds, so this is the axis that matters. Both points are inside `run.sh`'s
/// stated budget; the 500,000 and 10^6 points where finding I-1 measured
/// hundreds of milliseconds belong to
/// `experiments/shipped-append-condition-sql/`.
const LOG_LENGTHS: [usize; 2] = [1_000, 10_000];

/// Contender counts.
///
/// One is the uncontended control — without it a contended figure has nothing
/// to be contended *against*. Sixty-four is the count
/// `crates/happenstance-sqlite`'s own concurrency suite runs at, and the count
/// at which `experiments/busy-timeout-margin/` found `busy > 0`.
const CONTENDERS: [usize; 3] = [1, 8, 64];

/// Samples per group.
///
/// Ten, and the seeded stores are built **once per benchmark point** rather
/// than once per sample. `store_replay.rs`'s module documentation records what
/// the other arrangement cost: criterion calls `iter_custom` once per sample,
/// so a seed inside it is paid `sample_size` times over.
///
/// The guard arms take no per-iteration restore at all, because a refused
/// append writes nothing — see the module documentation. The seeded log is
/// therefore exactly this length for every sample and every iteration.
const SEEDED_SAMPLES: usize = 10;

/// A two-tag guard: the shape finding I-1 measured at hundreds of milliseconds.
///
/// Tags are AND within an item, so this is the intersection the adapter
/// evaluates as a chain — and the shape whose probe ordering CN-1 found
/// inverted.
///
/// # Panics
///
/// Panics if the query is refused, which would mean this file is malformed.
fn two_tag_guard(ordinal: usize) -> Query {
    Query::from_item(
        QueryItem::tagged(
            Tags::from_pairs([("bench", &*format!("e{ordinal}")), ("bench", "t01")])
                .expect("the benchmark's own tags are valid"),
        )
        .expect("a two-tag item is a valid query item"),
    )
}

/// Seeds `count` events, chunked to `ceiling`.
async fn seed(store: &impl EventStore, corpus: &Corpus, count: usize, ceiling: usize) {
    let mut written = 0;
    while written < count {
        let take = ceiling.min(count - written);
        let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
        store
            .append(&chunk, None)
            .await
            .unwrap_or_else(|_| panic!("seeding {take} events must land"));
        written += take;
    }
}

/// Guard shape against log length, uncontended, and refused.
///
/// Three points on the selectivity axis. The unconditional denominator is
/// `store_append.rs`'s at the same batch shape — see the module documentation
/// for why it cannot live here.
fn guard_shape(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("conditional/guard-shape");
    group.sample_size(SEEDED_SAMPLES);

    for length in LOG_LENGTHS {
        let corpus = Corpus::distinct(Shape::new(256, 3, Regime::Owned));

        // Seeded once per length, then guarded against by every shape below.
        // Nothing here writes, so one seeded store serves them all.
        let memory_fixture = MemoryFixture::new();
        let memory = runtime::block_on(async {
            let store = memory_fixture.connect().await;
            seed(&store, &corpus, length, 128).await;
            store
        });

        let sqlite_fixture = SqliteFixture::new();
        let sqlite = runtime::block_on(async {
            let store = sqlite_fixture.connect().await;
            let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
            seed(&store, &corpus, length, ceiling).await;
            store
        });
        sqlite_fixture.checkpoint();

        // Three points on the selectivity axis. Each condition is
        // **unanchored**, so it matches against the whole seeded log, the
        // append is refused, and the guard is evaluated in full while nothing
        // is written.
        let shapes: [(&str, AppendCondition); 3] = [
            (
                "guard/selects-one",
                AppendCondition::new(corpus::query_matching_one(0)),
            ),
            (
                "guard/selects-all",
                AppendCondition::new(corpus::query_matching_all()),
            ),
            (
                "guard/two-tag-intersection",
                AppendCondition::new(two_tag_guard(0)),
            ),
        ];

        for (name, condition) in shapes {
            let batch = corpus.batch(1);

            group.bench_function(
                BenchmarkId::new(format!("memory/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| {
                        runtime::block_on(async {
                            let started = Instant::now();
                            for _ in 0..iters {
                                let outcome = memory.append(&batch, Some(&condition)).await;
                                debug_assert!(
                                    outcome
                                        .as_ref()
                                        .is_err_and(AppendError::is_condition_violated),
                                    "the guard must refuse, or the log grows under the measurement"
                                );
                                black_box(outcome.is_ok());
                            }
                            started.elapsed()
                        })
                    });
                },
            );

            group.bench_function(
                BenchmarkId::new(format!("sqlite/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| {
                        runtime::block_on(async {
                            let started = Instant::now();
                            for _ in 0..iters {
                                let outcome = sqlite.append(&batch, Some(&condition)).await;
                                debug_assert!(
                                    outcome
                                        .as_ref()
                                        .is_err_and(AppendError::is_condition_violated),
                                    "the guard must refuse, or the log grows under the measurement"
                                );
                                black_box(outcome.is_ok());
                            }
                            started.elapsed()
                        })
                    });
                },
            );
        }
    }

    group.finish();
}

/// *k* contenders racing for one consistency boundary.
///
/// Driven through the testkit's own scenario so the workload is the one every
/// adapter mounting `event_store_benchmarks!` runs — ADR-0022 §3's *"the
/// workloads are the testkit's… a private timing loop would have produced
/// figures the adapter can never reproduce."*
///
/// The scenario takes `impl AsyncFn() -> F` — **how to make a fixture**, not one
/// already made — so that every run races into a fresh consistency boundary.
/// Its setup is therefore inside the timed region, and each figure is *"one
/// contended scenario, fixture construction included"*. That is stated rather
/// than corrected: reaching past the testkit's own workload to exclude it would
/// produce a number no other adapter could reproduce, which is the failure mode
/// ADR-0022 §3 names.
///
/// The record's mix is printed once per point, **outside** the timed region. It
/// is what distinguishes a store that serialises its writers (one winner, k−1
/// rejections) from one that does not, and from one that is simply broken (a
/// non-zero `failed`).
fn contention(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("conditional/contention");
    group.sample_size(SEEDED_SAMPLES);

    for contenders in CONTENDERS {
        let params = BenchmarkParams::new(8, contenders, 16);

        let memory_mix = runtime::block_on(scenarios::conditional_append_under_contention(
            || async { MemoryFixture::new() },
            params,
        ));
        let sqlite_mix = runtime::block_on(scenarios::conditional_append_under_contention(
            || async { SqliteFixture::new() },
            params,
        ));
        println!("mix k={contenders} memory: {}", memory_mix.summary());
        println!("mix k={contenders} sqlite: {}", sqlite_mix.summary());

        group.bench_function(BenchmarkId::new("memory", contenders), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let started = Instant::now();
                    for _ in 0..iters {
                        let record = scenarios::conditional_append_under_contention(
                            || async { MemoryFixture::new() },
                            params,
                        )
                        .await;
                        black_box(record.is_well_formed());
                    }
                    started.elapsed()
                })
            });
        });

        group.bench_function(BenchmarkId::new("sqlite", contenders), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let started = Instant::now();
                    for _ in 0..iters {
                        let record = scenarios::conditional_append_under_contention(
                            || async { SqliteFixture::new() },
                            params,
                        )
                        .await;
                        black_box(record.is_well_formed());
                    }
                    started.elapsed()
                })
            });
        });
    }

    group.finish();
}

criterion_group!(benches, guard_shape, contention);
criterion_main!(benches);
