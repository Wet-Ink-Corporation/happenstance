//! The measurement, driven by `event_store_benchmarks!` **verbatim**.
//!
//! The workloads are the testkit's, not this crate's, and that is the whole
//! point: a later `event_store_benchmarks!(SqliteFixture::new())` re-derives the
//! same three scenarios against the real adapter, so the figures below can be
//! reproduced by the thing they were measured *for*. A private timing loop would
//! produce numbers nobody can check against anything.
//!
//! # What this file supplies, and what it may not
//!
//! CF-23 makes the per-scenario wrapper a parameter and CF-33 forbids the
//! testkit from reading a clock, so the harness reports **counts** and the
//! caller's emitter reports **durations**. [`emit_measured`] is that emitter:
//! it runs each scenario `RUNS + 1` times, discards the first as warm-up,
//! reports the median, and prints one tab-separated row per arm and scenario.
//! Warm-up and repetition belong here rather than in the harness for the same
//! reason the clock does.
//!
//! # What the timer covers, stated because it changes how a figure reads
//!
//! Each scenario builds its own fixture, so the timed region includes creating
//! a SQLite file, running migration 1 and opening every handle the scenario
//! asks for — at `k = 64` that is sixty-five `connect()` calls. That cost is
//! **common to every arm**, because all five share one schema and one open
//! path, so the arms remain comparable to each other; what the figure is not is
//! an absolute throughput number for the adapter. The record says so where it
//! quotes them, and `results/README` repeats it, because a figure whose
//! measured region is unstated is a figure that gets quoted for something else.
//!
//! Run it with `--test-threads=1`; `run.sh` does. Timing five arms in parallel
//! on one disk measures the scheduler.

mod support;

use append_condition_probes::{
    BeginImmediateProbe, CanonicalBlob, ConditionalInsert, JoinTable, Json1, MonotonicGuard,
};
use happenstance_testkit::bench::BenchmarkParams;
use support::CandidateFixture;

/// Timed repetitions per scenario, after the discarded warm-up.
///
/// Five, and reported as a median rather than a mean: a single sample on a
/// machine with a page cache and a background scheduler is a number about the
/// last thirty seconds, and a mean lets one outlier decide an ADR.
const RUNS: usize = 5;

/// The workload sizes every arm is measured at.
///
/// * `n = 512` events in one batch — large enough that the append dominates the
///   fixture construction inside the timed region.
/// * `k = 64` contenders — the number **both** of phase 8's stated proof
///   artefacts read, while `concurrency::CONTENDERS` is 8. Measuring at 64 is
///   what turns that raise from an assumed claim into a supportable one.
/// * `N = 5000` events replayed, once unfiltered and once behind a tag filter.
///   One event in three carries the filtered tag, so the filtered pass selects
///   a proper subset rather than everything or nothing.
fn workload() -> BenchmarkParams {
    BenchmarkParams::new(512, 64, 5_000)
}

/// A caller-written emitter that owns the clock, the warm-up and the median.
///
/// It prints one tab-separated row per arm and scenario:
/// `MEASURE <arm> <scenario> <median_us> <min_us> <max_us> <record summary>`.
/// `run.sh` turns those rows into `results/`.
macro_rules! emit_measured {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let mut samples: Vec<u128> = Vec::with_capacity(RUNS);
                let mut summary = String::new();

                for run in 0..=RUNS {
                    let started = std::time::Instant::now();
                    let record = happenstance_testkit::block_on(
                        happenstance_testkit::bench::scenarios::$name(
                            __benchmark_fixture,
                            __benchmark_params(),
                        ),
                    );
                    let took = started.elapsed();

                    assert!(
                        record.is_well_formed(),
                        "{}: every pass must account for every attempt it made, got {record:?}",
                        ::core::stringify!($name)
                    );

                    // The first run is discarded: it pays for a cold page cache
                    // and a cold SQLite prepared-statement path, and a warm-up
                    // sample folded into a median is an outlier the median was
                    // chosen to resist.
                    if run > 0 {
                        samples.push(took.as_micros());
                    }
                    summary = record.summary();
                }

                samples.sort_unstable();
                println!(
                    "MEASURE\t{}\t{}\t{}\t{}\t{}\t{}",
                    ::core::module_path!(),
                    ::core::stringify!($name),
                    samples[samples.len() / 2],
                    samples[0],
                    samples[samples.len() - 1],
                    summary
                );
            }
        )*
    };
}

// --- the append-condition axis: three strategies, one tag storage ------------

happenstance_testkit::event_store_benchmarks!(
    mod_name = begin_immediate_probe_join_table,
    emit = emit_measured,
    params = workload(),
    fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new()
);

happenstance_testkit::event_store_benchmarks!(
    mod_name = conditional_insert_join_table,
    emit = emit_measured,
    params = workload(),
    fixture = CandidateFixture::<ConditionalInsert, JoinTable>::new()
);

happenstance_testkit::event_store_benchmarks!(
    mod_name = monotonic_guard_join_table,
    emit = emit_measured,
    params = workload(),
    fixture = CandidateFixture::<MonotonicGuard, JoinTable>::new()
);

// --- the tag-storage axis: three storages, one strategy ----------------------

happenstance_testkit::event_store_benchmarks!(
    mod_name = begin_immediate_probe_canonical_blob,
    emit = emit_measured,
    params = workload(),
    fixture = CandidateFixture::<BeginImmediateProbe, CanonicalBlob>::new()
);

happenstance_testkit::event_store_benchmarks!(
    mod_name = begin_immediate_probe_json1,
    emit = emit_measured,
    params = workload(),
    fixture = CandidateFixture::<BeginImmediateProbe, Json1>::new()
);
