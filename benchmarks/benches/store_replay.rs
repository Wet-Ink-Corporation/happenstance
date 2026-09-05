//! Replay: what it costs to read a log back, and the two places the price is
//! not what the documentation says.
//!
//! # The seeded log is the point, and it is seeded **once**
//!
//! `store_append.rs` measures into an empty log on purpose. Here the log is
//! seeded to a **stated** length before anything is timed, because every
//! interesting property of a read is a function of how much is in front of it.
//!
//! The seed is hoisted out of the criterion routine, and that is not a
//! micro-optimisation — the first version of this file put it inside
//! `iter_custom`, which criterion calls once per *sample*. Seeding 100,000
//! events through `SqliteEventStore` takes about 28 seconds, so twenty samples
//! spent nine minutes seeding to measure a read that takes milliseconds, and the
//! run reported *"Unable to complete 20 samples in 5.0s; you may wish to
//! increase target time to 263.9s"*. A read mutates nothing, so one seeded store
//! serves every sample.
//!
//! Seeding is chunked to each fixture's declared `MAX_EVENTS_PER_BATCH`, so a
//! store that states a ceiling is honoured rather than driven into a refusal —
//! which would measure the error path and report it as seeding cost.
//!
//! # Two arms that exist because a doc comment is measured false
//!
//! * **`limit(1)` against `limit(None)`.**
//!   `MemoryEventStore::read` materialises the entire matched set into a
//!   `Vec<SequencedEvent>` — cloning every event — before `limit` truncates it
//!   (`crates/happenstance-core/src/memory.rs:296-336`).
//!   `references/evaluation/review-pre-publication-2026-09-03.md:2792` measured
//!   the two at **1.0000× to four significant figures** on a million events:
//!   asking for one event costs what asking for all of them costs. The store
//!   documents itself as not built for scale and nobody is entitled to be
//!   surprised that it is slow — but it prices its snapshot as *cheap* and its
//!   `limit` as a *reduction*, and this arm is what shows neither is true.
//!   `src/bin/allocations.rs` reports the same pair in heap operations, which
//!   is the reproducible half.
//! * **`head()` against `backwards().limit(1)`.** ES-30
//!   (`spec/SPECIFICATION.md:4006`) makes `head` a required method precisely
//!   because the composed spelling is *"one cheap statement on local SQLite and
//!   one full HTTP round trip on the adapter with the smallest latency budget in
//!   the system"*. On SQLite the gap should be small; on the memory store it
//!   should be enormous, because `head` is `last()` and the composed form is a
//!   full clone. Reporting both is what makes ES-30 a measured decision rather
//!   than a plausible one.
//!
//! # `Query::all` gets its own arm
//!
//! `references/evaluation/review-pre-publication-2026-09-03.md:2652` (finding
//! I-3) measured `happenstance-sqlite` emitting
//! `WHERE position IN (SELECT position FROM event)` for `Query::all` — a
//! tautology — at **60,092 µs warm against 71 µs** for the identical statement
//! with the clause deleted, returning the identical 512 rows. It is the
//! commonest read in the library and the one a projection runner catching up
//! performs, so it is measured beside the tagged read rather than folded into
//! it.
//!
//! Nothing here asserts on any of that. The arms exist so the numbers are taken
//! again on every run and a change is visible; CF-34 forbids a threshold.

use std::hint::black_box;
use std::time::Instant;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use happenstance_benchmarks::corpus::{self, Corpus, Regime, Shape};
use happenstance_benchmarks::fixtures::memory::{MemoryFixture, MemoryHandle};
use happenstance_benchmarks::fixtures::raw::{Floor, RawStore};
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;
use happenstance_benchmarks::runtime;
use happenstance_core::{EventStore, Query, ReadOptions, collect};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_testkit::Fixture;

/// Log lengths swept.
///
/// A thousand is a small aggregate's lifetime; ten thousand is a busy tenant's
/// day. Both are inside the wall-clock budget `run.sh` states.
///
/// The hundred-thousand and million-event points are deliberately **not** here.
/// At a million events one `Query::all` replay through the real adapter takes
/// **97.2 seconds** (finding I-3), which would put hours into a run whose whole
/// budget is minutes — and `experiments/one-connection-latency/` has already
/// taken that measurement at that scale. A standing suite that nobody re-runs
/// because it takes an afternoon has stopped being a standing suite.
const LOG_LENGTHS: [usize; 2] = [1_000, 10_000];

/// Samples per group.
///
/// criterion's default is 100. Ten here because each sample is a whole replay
/// and the figures are quoted as ratios and orders of magnitude, never to a
/// third significant figure. The confidence interval widens; the run stays
/// inside its stated budget.
const SEEDED_SAMPLES: usize = 10;

/// The seeding corpus: distinct tags, owned regime.
///
/// Distinct because the tagged and selective arms need one tag that selects one
/// event and another that selects the whole log. Owned because that is the
/// regime a caller is in, and a replay of an interned corpus would report a
/// clone cost 66× below what an application pays
/// (`happenstance_benchmarks::corpus`).
fn replay_corpus() -> Corpus {
    Corpus::distinct(Shape::new(256, 3, Regime::Owned))
}

/// A memory store seeded to `length`, built once and read many times.
fn seeded_memory(length: usize) -> MemoryHandle {
    let corpus = replay_corpus();
    runtime::block_on(async {
        let fixture = MemoryFixture::new();
        let store = fixture.connect().await;
        let mut written = 0;
        while written < length {
            let take = 128.min(length - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            store
                .append(&chunk, None)
                .await
                .expect("MemoryEventStore's error type is uninhabited");
            written += take;
        }
        store
    })
}

/// A SQLite store seeded to `length`, with its write-ahead log folded in.
///
/// The fixture is returned alongside the store and must outlive it: dropping it
/// removes the database file. The checkpoint matters — a replay over a WAL
/// grown by the seeding phase measures the seeding.
fn seeded_sqlite(length: usize) -> (SqliteFixture, SqliteEventStore) {
    let corpus = replay_corpus();
    let fixture = SqliteFixture::new();
    let store = runtime::block_on(async {
        let store = fixture.connect().await;
        let ceiling = <SqliteFixture as Fixture>::MAX_EVENTS_PER_BATCH.unwrap_or(128);
        let mut written = 0;
        while written < length {
            let take = ceiling.min(length - written);
            let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
            store.append(&chunk, None).await.expect("seeding must land");
            written += take;
        }
        store
    });
    fixture.checkpoint();
    (fixture, store)
}

/// A raw-SQL floor seeded to `length`, over its own file.
fn seeded_floor(length: usize) -> (SqliteFixture, RawStore) {
    let corpus = replay_corpus();
    let fixture = SqliteFixture::new();
    let mut store = RawStore::over(&fixture, Floor::SameSchema).expect("the floor opens");
    let mut written = 0;
    while written < length {
        let take = 128.min(length - written);
        let chunk: Vec<_> = (written..written + take).map(|n| corpus.event(n)).collect();
        store.append(&chunk).expect("the floor seeds");
        written += take;
    }
    fixture.checkpoint();
    (fixture, store)
}

/// Times `iters` reads of `query` under `options` against an event store.
fn time_reads(
    iters: u64,
    store: &impl EventStore,
    query: &Query,
    options: ReadOptions,
) -> std::time::Duration {
    runtime::block_on(async {
        let started = Instant::now();
        for _ in 0..iters {
            let events = collect(store.read(query, options))
                .await
                .expect("the read succeeds");
            black_box(events.len());
        }
        started.elapsed()
    })
}

/// Unfiltered replay of a seeded log, on all three arms.
fn unfiltered(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("replay/unfiltered");
    group.sample_size(SEEDED_SAMPLES);

    for length in LOG_LENGTHS {
        group.throughput(Throughput::Elements(length as u64));

        let memory = seeded_memory(length);
        let (_sqlite_fixture, sqlite) = seeded_sqlite(length);
        let (_floor_fixture, floor) = seeded_floor(length);

        group.bench_function(BenchmarkId::new("memory", length), |bencher| {
            bencher
                .iter_custom(|iters| time_reads(iters, &memory, &Query::all(), ReadOptions::new()));
        });

        group.bench_function(BenchmarkId::new("sqlite", length), |bencher| {
            bencher
                .iter_custom(|iters| time_reads(iters, &sqlite, &Query::all(), ReadOptions::new()));
        });

        group.bench_function(
            BenchmarkId::new(Floor::SameSchema.label(), length),
            |bencher| {
                bencher.iter_custom(|iters| {
                    let started = Instant::now();
                    for _ in 0..iters {
                        black_box(floor.read_all().expect("the floor replays"));
                    }
                    started.elapsed()
                });
            },
        );
    }

    group.finish();
}

/// Tag-filtered replay, at both ends of the selectivity axis.
///
/// The unselective end matches the whole log; the selective end matches exactly
/// one event however long the log is. An adapter that seeks reports the second
/// as flat in log length; one that scans reports it as linear — and that is the
/// *"scans where it should seek"* class CF-34's preamble says no conformance
/// rule can catch.
fn filtered(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("replay/filtered");
    group.sample_size(SEEDED_SAMPLES);

    for length in LOG_LENGTHS {
        let memory = seeded_memory(length);
        let (_sqlite_fixture, sqlite) = seeded_sqlite(length);

        for (name, query, selected) in [
            ("matches-all", corpus::query_matching_all(), length),
            ("matches-one", corpus::query_matching_one(0), 1),
            (
                "mixed-selectivity",
                corpus::query_mixed_selectivity(0),
                length,
            ),
        ] {
            group.throughput(Throughput::Elements(selected as u64));

            group.bench_function(
                BenchmarkId::new(format!("memory/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| {
                        time_reads(iters, &memory, &query, ReadOptions::new())
                    });
                },
            );

            group.bench_function(
                BenchmarkId::new(format!("sqlite/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| {
                        time_reads(iters, &sqlite, &query, ReadOptions::new())
                    });
                },
            );
        }
    }

    group.finish();
}

/// `limit(1)` against `limit(None)`, and `head()` against the composed
/// spelling.
///
/// Four shapes, two comparisons, both about a claim the library makes in prose.
/// See the module documentation.
fn cheap_reads(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("replay/cheap-reads");
    group.sample_size(SEEDED_SAMPLES);

    for length in LOG_LENGTHS {
        let memory = seeded_memory(length);
        let (_sqlite_fixture, sqlite) = seeded_sqlite(length);

        // `limit(None)` is spelled out rather than defaulted so the pair reads
        // as a comparison rather than as a baseline somebody has to infer.
        let shapes: [(&str, ReadOptions); 3] = [
            ("limit-1", ReadOptions::new().limit(1)),
            ("limit-none", ReadOptions::new()),
            ("backwards-limit-1", ReadOptions::new().backwards().limit(1)),
        ];

        for (name, options) in shapes {
            group.bench_function(
                BenchmarkId::new(format!("memory/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| time_reads(iters, &memory, &Query::all(), options));
                },
            );

            group.bench_function(
                BenchmarkId::new(format!("sqlite/{name}"), length),
                |bencher| {
                    bencher.iter_custom(|iters| time_reads(iters, &sqlite, &Query::all(), options));
                },
            );
        }

        // And `head()`, the method ES-30 requires precisely so the composed
        // spelling above is never the only way to ask.
        group.bench_function(BenchmarkId::new("memory/head", length), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let started = Instant::now();
                    for _ in 0..iters {
                        black_box(memory.head().await.expect("head succeeds"));
                    }
                    started.elapsed()
                })
            });
        });

        group.bench_function(BenchmarkId::new("sqlite/head", length), |bencher| {
            bencher.iter_custom(|iters| {
                runtime::block_on(async {
                    let started = Instant::now();
                    for _ in 0..iters {
                        black_box(sqlite.head().await.expect("head succeeds"));
                    }
                    started.elapsed()
                })
            });
        });

        // The floor's answer to the same question, so the adapter's `head` has
        // something to be measured above.
        let (_floor_fixture, floor) = seeded_floor(length);
        group.bench_function(
            BenchmarkId::new(format!("{}/head", Floor::SameSchema.label()), length),
            |bencher| {
                bencher.iter_custom(|iters| {
                    let started = Instant::now();
                    for _ in 0..iters {
                        black_box(floor.head().expect("the floor reports a head"));
                    }
                    started.elapsed()
                });
            },
        );
    }

    group.finish();
}

/// What a page hop costs, swept over how many pages a replay needs.
///
/// `SqliteEventStore` reads in pages of `PAGE_SIZE = 512` rows, each hop taking
/// the connection mutex and going through `spawn_blocking`
/// (`crates/happenstance-sqlite/src/event_store.rs:141`, `:1276`). The constant
/// says of itself *"a placeholder until it is measured"*, and finding J-5 calls
/// it the only knob sizing how long `fetch_page` holds that mutex.
///
/// This does not measure `PAGE_SIZE` — the constant is private, and
/// `experiments/one-connection-latency/` already swept it at 64, 128, 512 and
/// 2048. What it does is report the *per-event* cost either side of the page
/// boundary, so a change in the paging strategy shows up in the standing suite
/// instead of waiting for someone to re-run that experiment.
fn page_hops(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("replay/page-hops");
    group.sample_size(SEEDED_SAMPLES);

    // Either side of the 512-row page: 256 is one hop, 2,048 is four, 8,192 is
    // sixteen. A per-event cost flat across them says the hop is cheap relative
    // to the row; one that falls says it is not.
    for length in [256_usize, 2_048, 8_192] {
        group.throughput(Throughput::Elements(length as u64));
        let (_fixture, sqlite) = seeded_sqlite(length);

        group.bench_function(BenchmarkId::new("sqlite", length), |bencher| {
            bencher
                .iter_custom(|iters| time_reads(iters, &sqlite, &Query::all(), ReadOptions::new()));
        });
    }

    group.finish();
}

criterion_group!(benches, unfiltered, filtered, cheap_reads, page_hops);
criterion_main!(benches);
