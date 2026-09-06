//! The benchmark family, against the reference implementation.
//!
//! `memory_concurrency_conformance.rs`'s sibling, and it exists for the reason
//! CLAUDE.md gives about adapters: a harness reachable only by `cargo check` is
//! the "compiles but never ran" failure this project exists to retire. Every
//! scenario in `happenstance_testkit::bench` is *executed* here, against
//! `MemoryFixture`, by `cargo test -p happenstance-testkit --features bench`.
//!
//! Three invocations, and each of them is carrying something:
//!
//! 1. the one-argument arm through the shipped tokio emitter, at the shipped
//!    smoke parameters — the line an adapter author actually writes;
//! 2. the shipped blocking emitter at a *second* parameter set, which is what
//!    makes *n*, *k* and *N* observably the caller's rather than constants
//!    inside the testkit;
//! 3. an emitter written **here**, in the caller's crate, which owns the clock
//!    and reports a CSV row. That is CF-23 demonstrated rather than described:
//!    the testkit never reads a clock (CF-33, and `cargo xtask lint-clock`
//!    scans its `src/`), so the only place a duration can come from is an
//!    emitter — and this file is where one is written.
//!
//! Native only, and the `cfg` is load-bearing rather than tidy. A Cargo feature
//! is **not** target-scoped, so `--all-features` sets `bench` for
//! `wasm32-unknown-unknown` too; `happenstance_testkit::bench` does not exist
//! there, and an adapter with a wasm32 harness gates its own invocation exactly
//! like this.
//!
//! Nothing here asserts on a timing. CF-34's own `Rejects:` line is a threshold
//! that can turn a merge red on a loaded runner, and the third emitter below
//! prints its number rather than judging it.

#![cfg(all(feature = "bench", not(target_arch = "wasm32")))]

use core::future::Future;

use happenstance_testkit::bench::{BenchmarkParams, BenchmarkRecord, scenarios};
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};
use happenstance_testkit::{Capability, Fixture};

/// A caller-written emitter that owns the clock.
///
/// It is defined in *this* crate, uses `std::time::Instant` — which
/// `crates/happenstance-testkit/src` may not so much as spell — and prints one
/// CSV row per scenario. An adapter author who wants `criterion`, `divan` or a
/// row in a spreadsheet writes this shape and puts the dependency in their own
/// `dev-dependencies`; the testkit's stays two crates wide.
///
/// The name is substituted verbatim into the expansion, so it resolves here
/// rather than in the testkit. Misspell it and the compile error names the
/// missing item in this file, which is the whole of EC-006.
macro_rules! emit_timed_csv {
    ($($name:ident),* $(,)?) => {
        $(
            #[test]
            fn $name() {
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
                // The other half, and available to a third-party emitter for
                // the same reason the first is: a row whose contended pass
                // never ran is a row whose number means something else, and a
                // CSV is exactly where that is unrecoverable later.
                assert!(
                    record.is_complete(),
                    "{}: every pass the scenario owes must have run, got {record:?}",
                    ::core::stringify!($name)
                );
                println!(
                    "{},{},{}",
                    ::core::stringify!($name),
                    record.summary(),
                    took.as_nanos()
                );
            }
        )*
    };
}

// 1. The one-argument arm: the line an adapter author writes.
happenstance_testkit::event_store_benchmarks!(MemoryFixture::new());

// 2. The shipped blocking emitter, at the caller's own n, k and N.
happenstance_testkit::event_store_benchmarks!(
    mod_name = dcb_benchmarks_blocking,
    emit = happenstance_testkit::__emit_benchmark_blocking,
    params = BenchmarkParams::new(6, 5, 21),
    fixture = MemoryFixture::new()
);

// 3. An emitter this crate wrote, at a third parameter set.
happenstance_testkit::event_store_benchmarks!(
    mod_name = dcb_benchmarks_timed_csv,
    emit = emit_timed_csv,
    params = BenchmarkParams::new(12, 7, 33),
    fixture = MemoryFixture::new()
);

/// A fixture that states a batch ceiling it inherits no enforcement for.
///
/// `MemoryEventStore` has no ceiling at all, so nothing in the reference
/// implementation can produce EC-004. Limits are **declared facts** about a
/// fixture rather than trades (`contract.rs`), which is what lets this one
/// state a number and be believed: the benchmark reads the declared ceiling and
/// reports the append as a *refusal* rather than as completed work.
#[derive(Debug)]
struct BatchCeilingFixture(MemoryFixture);

impl Fixture for BatchCeilingFixture {
    type Store = MemoryHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "this fixture wraps MemoryFixture, which has no durable medium to reopen over",
    );

    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(2);

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// AC-001: all three scenarios run against a real `Fixture` and each produces a
/// completed, well-formed record.
///
/// The generated tests above already prove the macro path; this proves the same
/// three scenarios called directly, so a record that is well-formed only
/// because an emitter never looked at it cannot pass.
#[test]
fn all_three_scenarios_complete_against_the_reference_fixture() {
    let params = BenchmarkParams::new(5, 3, 12);

    let records = [
        happenstance_testkit::block_on(scenarios::append_throughput(
            || async { MemoryFixture::new() },
            params,
        )),
        happenstance_testkit::block_on(scenarios::conditional_append_under_contention(
            || async { MemoryFixture::new() },
            params,
        )),
        happenstance_testkit::block_on(scenarios::replay_with_and_without_a_tag_filter(
            || async { MemoryFixture::new() },
            params,
        )),
    ];

    let names: Vec<&str> = records.iter().map(BenchmarkRecord::scenario).collect();
    assert_eq!(
        names,
        vec![
            "append_throughput",
            "conditional_append_under_contention",
            "replay_with_and_without_a_tag_filter",
        ],
        "each scenario must name itself in its own record"
    );

    for record in &records {
        assert!(
            record.is_well_formed(),
            "{}: attempts must equal committed + rejected + refused + failed in \
             every pass, got {record:?}",
            record.scenario()
        );
        assert!(
            !record.passes().is_empty(),
            "{}: a completed scenario reports at least one pass",
            record.scenario()
        );
    }
}

/// AC-005: the batch size is the caller's, and the appended events are the
/// number they asked for.
#[test]
fn the_append_scenario_appends_the_callers_batch_size() {
    for batch_size in [1_usize, 9] {
        let params = BenchmarkParams::new(batch_size, 2, 4);
        let record = happenstance_testkit::block_on(scenarios::append_throughput(
            || async { MemoryFixture::new() },
            params,
        ));

        let pass = record.pass("append").expect("the append pass is reported");
        assert_eq!(pass.attempts(), 1, "one batch, at the caller's size");
        assert_eq!(pass.committed(), 1, "MemoryEventStore accepts the batch");
        assert_eq!(pass.refused(), 0);
        assert_eq!(
            pass.events(),
            batch_size,
            "the events appended must be the n the caller asked for, not a \
             constant chosen inside the testkit"
        );
    }
}

/// AC-005: replay runs twice — once unfiltered, once behind a tag filter — and
/// reports both.
#[test]
fn replay_reports_a_filtered_and_an_unfiltered_pass() {
    let params = BenchmarkParams::new(4, 2, 20);
    let record = happenstance_testkit::block_on(scenarios::replay_with_and_without_a_tag_filter(
        || async { MemoryFixture::new() },
        params,
    ));

    let seeded = record.pass("seed").expect("the seed pass is reported");
    assert_eq!(
        seeded.events(),
        20,
        "the replay length is the caller's N, not a constant"
    );

    let all = record.pass("replay-all").expect("the unfiltered pass");
    let tagged = record.pass("replay-tagged").expect("the filtered pass");

    assert_eq!(all.events(), 20, "an unfiltered replay reads every event");
    assert!(
        tagged.events() > 0 && tagged.events() < all.events(),
        "the filtered replay must select a proper subset — a filter that \
         matched everything, or nothing, would measure the wrong thing. Got \
         {} of {}",
        tagged.events(),
        all.events()
    );
    assert_eq!(all.committed(), 1);
    assert_eq!(tagged.committed(), 1);
}

/// AC-006: every contender is accounted for, and a `ConditionViolated`
/// rejection is not the same thing as a store failure.
///
/// `MemoryEventStore` is a `Vec` behind an `RwLock`, so it serialises its
/// writers: exactly one contender commits and the rest learn that they lost.
/// A run in which everybody won would be a measurement of the wrong thing, and
/// this is the assertion that makes that visible rather than averaging it away.
#[test]
fn a_contended_run_accounts_for_every_contender() {
    for contenders in [2_usize, 8] {
        let params = BenchmarkParams::new(3, contenders, 6);
        let record =
            happenstance_testkit::block_on(scenarios::conditional_append_under_contention(
                || async { MemoryFixture::new() },
                params,
            ));

        let pass = record.pass("contend").expect("the contended pass");
        assert_eq!(
            pass.attempts(),
            contenders,
            "k is the caller's, and every contender is an attempt"
        );
        assert_eq!(
            pass.committed() + pass.rejected() + pass.refused() + pass.failed(),
            contenders,
            "committed + rejected + refused + failed must account for every \
             contender, got {pass:?}"
        );
        assert_eq!(
            pass.committed(),
            1,
            "a store that serialises its writers elects exactly one winner"
        );
        assert_eq!(
            pass.rejected(),
            contenders - 1,
            "and every loser learns it lost as a condition violation"
        );
        assert_eq!(
            pass.failed(),
            0,
            "a transport-level failure is a different number from a rejection, \
             and this run had none"
        );
    }
}

/// EC-004: a declared ceiling below the caller's *n* is reported as a refusal,
/// never as completed work and never as a panic.
#[test]
fn a_declared_batch_ceiling_is_reported_as_a_refusal() {
    let params = BenchmarkParams::new(8, 2, 4);
    let record = happenstance_testkit::block_on(scenarios::append_throughput(
        || async { BatchCeilingFixture(MemoryFixture::new()) },
        params,
    ));

    let pass = record.pass("append").expect("the append pass is reported");
    assert_eq!(pass.attempts(), 1);
    assert_eq!(
        pass.refused(),
        1,
        "a batch above the fixture's stated MAX_EVENTS_PER_BATCH is a refusal"
    );
    assert_eq!(
        pass.committed(),
        0,
        "and a refusal is never counted as completed work"
    );
    assert_eq!(pass.events(), 0);
    assert!(record.is_well_formed());
}

/// EC-003: a degenerate parameter is refused at the call, naming the parameter,
/// before any store is touched.
#[test]
#[should_panic(expected = "contenders")]
fn zero_contenders_is_refused_by_name() {
    let _ = BenchmarkParams::new(4, 0, 4);
}

/// EC-003, the other two parameters.
#[test]
#[should_panic(expected = "batch_size")]
fn an_empty_batch_is_refused_by_name() {
    let _ = BenchmarkParams::new(0, 4, 4);
}

/// EC-003, the third.
#[test]
#[should_panic(expected = "replay_events")]
fn zero_events_to_replay_is_refused_by_name() {
    let _ = BenchmarkParams::new(4, 4, 0);
}

/// AC-007, the non-occlusion half: a record's own summary is one line.
///
/// The gate's test step runs `-- --show-output` so that a declined capability's
/// `SKIP <rule>: …` line reaches a human. A chatty benchmark emitter would bury
/// exactly the output that step exists to surface, so the shipped emitters are
/// held to one line per scenario — which starts with the record being able to
/// render itself in one.
#[test]
fn a_record_summarises_itself_in_one_line() {
    let params = BenchmarkParams::new(4, 4, 8);
    let record = happenstance_testkit::block_on(scenarios::replay_with_and_without_a_tag_filter(
        || async { MemoryFixture::new() },
        params,
    ));

    let summary = record.summary();
    assert!(
        !summary.contains('\n'),
        "a scenario reports at most one line, got: {summary}"
    );
    assert!(
        summary.contains("replay-all") && summary.contains("replay-tagged"),
        "and that one line carries every pass, got: {summary}"
    );
}

/// A fixture whose very first append loses, so the boundary is never planted.
///
/// The input no in-process store produces, and the one this file has no other
/// way to reach: `MemoryEventStore` accepts an unconditional append always, so
/// nothing in the reference implementation can drive
/// `conditional_append_under_contention`'s early return. `violate_next(1)`
/// spends its single arming on the seed — a migration half-applied, a
/// permission revoked, a pool exhausted, all reported through the same seam —
/// and every handle opened afterwards is transparent, exactly as an adapter
/// whose seed failed for a transient reason would be.
///
/// The arming lives behind an `Arc` that `Clone` shares, so one prototype
/// cloned per `connect` is one fixture with one arming, as
/// `tests/faulty_store_conformance.rs` records.
#[derive(Debug)]
struct SeedViolatesFixture(happenstance_testkit::SendFaultyStore<MemoryHandle>);

impl SeedViolatesFixture {
    fn new() -> Self {
        let handle = happenstance_testkit::block_on(MemoryFixture::new().connect());
        Self(happenstance_testkit::SendFaultyStore::new(handle).violate_next(1))
    }
}

impl Fixture for SeedViolatesFixture {
    type Store = happenstance_testkit::SendFaultyStore<MemoryHandle>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "the wrapped store is a MemoryEventStore, so there is no durable medium \
         to reopen over and the wrapper adds none",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(self.0.clone())
    }
}

/// A contended run that never happened must not report as a completed one.
///
/// The scenario's early return is honest about the record it builds — one seed
/// pass, no contended pass — but nothing read it. Every counter in that record
/// adds up, so `is_well_formed` is true, and the contended pass that is the
/// entire measurement is *absent* rather than zero. The distinction between
/// "contention produced no rejections" and "contention never happened" is the
/// one the module documentation says the record exists to preserve, and it is
/// the one an adapter author comparing two releases' `BENCH` lines cannot
/// recover afterwards.
///
/// **Rejects: a `report` whose completion half is discharged by a predicate
/// that cannot be false.**
#[test]
#[should_panic(expected = "did not complete")]
fn a_contended_run_that_never_happened_is_not_reported_as_complete() {
    let record = happenstance_testkit::block_on(scenarios::conditional_append_under_contention(
        || async { SeedViolatesFixture::new() },
        BenchmarkParams::new(4, 3, 8),
    ));

    assert_eq!(
        record.passes().len(),
        1,
        "the seed pass is the whole of what the scenario produced, got {record:?}"
    );
    assert!(
        record.pass("contend").is_none(),
        "the contended pass is absent, not zero"
    );
    assert!(
        record.is_well_formed(),
        "and the record is well-formed all the same — every attempt it made is \
         accounted for, which is why well-formedness cannot carry completion"
    );

    record.report("conditional_append_under_contention");
}
