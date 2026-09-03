//! `experiments/append-condition/tests/contention_at_64.rs`'s race, re-run under
//! the configuration `cargo xtask ci` actually uses, with the busy handler
//! counted.
//!
//! # What is the same, and it has to be
//!
//! The race is the same race: *n* `rusqlite::Connection`s on one file, opened
//! before the timed region; one head snapshot; one append condition anchored
//! there so exactly one contender may win; a `std::sync::Barrier` rather than a
//! sleep so the release is exact on a one-core host and a twenty-core one alike;
//! every contender's result collapsed to committed / rejected / busy / failed.
//! That sameness is what makes the release control below checkable against
//! ADR-0022 §11's recorded row rather than merely comparable to it.
//!
//! # What is different, and why each difference is there
//!
//! **The build.** `run.sh` runs the matrix in `--release` once — the control —
//! and in **debug** everywhere else, because `xtask/src/main.rs`'s tests step is
//! `cargo test --locked --workspace --all-features` with no `--release`.
//!
//! **The core count.** Set by process affinity mask at creation, verified twice,
//! and refused rather than mislabelled — see [`busy_timeout_margin_probes::cores`].
//!
//! **Three racing tests, concurrently.** `--test-threads` is left at its default
//! so libtest overlaps them exactly as it overlaps the real family's rules in
//! one binary. Three and not five: `event_store_concurrency_conformance!`
//! generates five `#[test]` fns in `crates/happenstance-sqlite/tests/concurrency.rs`,
//! but only **three** of them open `CONTENDERS` handles — `concurrency.rs:394`,
//! `:611` and `:691`. `k_disjoint_boundaries_admit_exactly_k_commits` opens
//! `BOUNDARIES * PER_BOUNDARY` = 12 (`:508-509`) and
//! `a_concurrent_reader_never_sees_a_partial_batch` opens `WRITERS + 1` = 5
//! (`:775`, `:795`). Reproducing five would over-provision the load by two
//! rules and produce a number the gate cannot reach, so the three are named
//! after the three rules they stand in for.
//!
//! **A counting busy handler.** The whole point. Each contender clears its
//! thread-local accounting immediately before its append and reads it back
//! immediately after, so the row covers that contender's own attempt and not the
//! seeding that preceded it.
//!
//! # What the numbers mean in the gate's own currency
//!
//! `busy > 0` is `Attempt::Failed` at
//! `crates/happenstance-testkit/src/concurrency.rs:263`, and
//! `exactly_one_of_n_contenders_commits` panics on any of them at `:409-414`.
//! So a nonzero `busy` in a row below is not a slow run: it is the gate red, on
//! a message that tells the adapter author their store is wrong when nothing
//! about their store is. It is also, exactly, ADR-0022's own stated re-open
//! trigger (`references/adr/0022-append-condition-strategy.md:615-616`).
//!
//! # There is no assertion that a race elected a winner
//!
//! `contention_at_64.rs` asserts `committed == ROUNDS`. This file deliberately
//! does not, and the omission is the measurement: under exhaustion the *winner*
//! can be the contender that gives up, and a run that panicked there would stop
//! before reporting the distribution that explains why. The correctness
//! assertion that is kept is the one that cannot mask anything —
//! `committed + rejected + busy + failed == contenders`.

mod support;

use std::sync::Barrier;

use busy_timeout_margin_probes::{
    AppendStrategy, BeginImmediateProbe, BusyStats, CandidateStore, ConditionalInsert, JoinTable,
    MonotonicGuard, SqliteProbeError, busy, cores,
};
use happenstance_core::{AppendError, Event, EventStore};
use happenstance_testkit::block_on;
use happenstance_testkit::fixtures::{condition_after, query_of, tagged_event};
use support::CandidateFixture;

/// Events per seeding append.
const CHUNK: usize = 100;

/// Reads a positive `usize` out of the environment, or falls back.
///
/// Every knob is defaulted to `contention_at_64.rs`'s own value, so running this
/// binary with an empty environment runs the recorded shape. `run.sh` overrides
/// them per configuration and every row prints back what it ran under, which is
/// what stops a tuned run being read as the default one.
fn setting(name: &str, fallback: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(fallback)
}

/// Which of the three append-condition strategies this run races.
///
/// A runtime choice rather than three test targets, because the arm has to be
/// held **equal to the control's** for a debug figure to be a delta from it.
/// `begin-immediate-probe` is the default for exactly that reason: it is the arm
/// ADR-0022 §11's recorded row was produced by, and it is also the slowest of
/// the three at 64, so a margin computed from it is the conservative one.
fn arm() -> String {
    std::env::var("HS_ARM").unwrap_or_else(|_| "begin-immediate-probe".to_owned())
}

/// The label `run.sh` gives this configuration, carried into every row.
fn label() -> String {
    std::env::var("HS_LABEL").unwrap_or_else(|_| "unlabelled".to_owned())
}

/// What one contender's attempt collapsed to.
///
/// `Rejected` and `Busy` are kept apart deliberately, and the distinction is the
/// whole reason this file can say anything: a `ConditionViolated` is the DCB
/// retry signal and a *correct* outcome for a loser, while `SQLITE_BUSY` is the
/// adapter running out of patience and reaches a conformance rule as
/// `Attempt::Failed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Committed,
    Rejected,
    Busy,
    Failed,
}

/// What one race produced.
#[derive(Debug, Default, Clone)]
struct Tally {
    committed: usize,
    rejected: usize,
    busy: usize,
    failed: usize,
    /// Every contender's own maximum busy wait, in milliseconds, one entry per
    /// contender per round.
    ///
    /// Kept per contender rather than summarised per race, because the question
    /// is about the unluckiest contender and a per-race summary has already
    /// thrown that away.
    waits_ms: Vec<u64>,
    /// The busy accounting of every contender, folded.
    busy_stats: BusyStats,
    /// Rounds in which nobody committed at all.
    no_winner: usize,
}

impl Tally {
    fn count(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Committed => self.committed += 1,
            Outcome::Rejected => self.rejected += 1,
            Outcome::Busy => self.busy += 1,
            Outcome::Failed => self.failed += 1,
        }
    }

    fn total(&self) -> usize {
        self.committed + self.rejected + self.busy + self.failed
    }

    fn absorb(&mut self, other: &Self) {
        self.committed += other.committed;
        self.rejected += other.rejected;
        self.busy += other.busy;
        self.failed += other.failed;
        self.no_winner += other.no_winner;
        self.waits_ms.extend_from_slice(&other.waits_ms);
        self.busy_stats.merge(other.busy_stats);
    }
}

/// Classifies one contender's result without letting a non-`Send` error escape
/// its thread.
fn classify(
    result: Result<happenstance_core::SequencePosition, AppendError<SqliteProbeError>>,
) -> Outcome {
    match result {
        Ok(_) => Outcome::Committed,
        Err(AppendError::ConditionViolated(_)) => Outcome::Rejected,
        Err(AppendError::Store(SqliteProbeError::Sqlite(rusqlite::Error::SqliteFailure(
            error,
            _,
        )))) if error.code == rusqlite::ErrorCode::DatabaseBusy
            || error.code == rusqlite::ErrorCode::DatabaseLocked =>
        {
            Outcome::Busy
        }
        Err(_) => Outcome::Failed,
    }
}

/// Appends events until the log holds `to` of them.
fn fill<A: AppendStrategy>(store: &CandidateStore<A, JoinTable>, to: usize) {
    let mut written = 0;
    while written < to {
        let take = CHUNK.min(to - written);
        let batch: Vec<Event> = (0..take)
            .map(|offset| {
                let row = format!("r{}", (written + offset) % 97);
                tagged_event("Seeded", &[("shard", "cold"), ("row", &row)])
            })
            .collect();
        block_on(store.append(&batch, None)).expect("seeding must succeed");
        written += take;
    }
}

/// Runs one race and returns its wall time in microseconds and its tally.
fn race<A: AppendStrategy>(
    fixture: &CandidateFixture<A, JoinTable>,
    contenders: usize,
    round: usize,
) -> (u128, Tally) {
    let handles: Vec<CandidateStore<A, JoinTable>> =
        (0..contenders).map(|_| fixture.open()).collect();

    let head = block_on(handles[0].head())
        .expect("head must succeed")
        .expect("the store is not empty");
    let boundary = query_of(&["Contended"], &[("race", "one")]);
    let condition = condition_after(boundary, head.get());
    let event = tagged_event("Contended", &[("race", "one")]);

    let gate = Barrier::new(contenders);
    let started = std::time::Instant::now();

    let results: Vec<(Outcome, BusyStats)> = std::thread::scope(|scope| {
        let running: Vec<_> = handles
            .iter()
            .map(|store| {
                let gate = &gate;
                let condition = &condition;
                let event = &event;
                scope.spawn(move || {
                    gate.wait();
                    // Clear whatever this thread accumulated before the timed
                    // attempt — nothing, on a fresh scoped thread, but stated so
                    // that the reading below can only be this append's.
                    let _ = busy::take();
                    let outcome = classify(block_on(
                        store.append(core::slice::from_ref(event), Some(condition)),
                    ));
                    (outcome, busy::take())
                })
            })
            .collect();

        running
            .into_iter()
            .map(|thread| match thread.join() {
                Ok(pair) => pair,
                Err(payload) => std::panic::resume_unwind(payload),
            })
            .collect()
    });

    let elapsed = started.elapsed().as_micros();
    let mut tally = Tally::default();
    for (outcome, stats) in results {
        tally.count(outcome);
        tally.waits_ms.push(stats.max_wait_ms);
        tally.busy_stats.merge(stats);
    }
    if tally.committed == 0 {
        tally.no_winner = 1;
    }

    assert_eq!(
        tally.total(),
        contenders,
        "round {round}: every contender must be accounted for"
    );
    (elapsed, tally)
}

/// The percentile of a sorted slice, by nearest rank.
fn percentile(sorted: &[u64], numerator: usize, denominator: usize) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let index = (sorted.len() * numerator / denominator).min(sorted.len() - 1);
    sorted[index]
}

/// Runs one rule's worth of races and prints its row.
fn measure<A: AppendStrategy>(rule: &str) {
    // Authoritative, and cheap: one process spawn. It aborts rather than let a
    // twenty-core run be filed under a two-core label.
    let constraint = cores::verify();
    constraint.require_honest();

    let contenders = setting("HS_CONTENDERS", 64);
    let rounds = setting("HS_ROUNDS", 10);
    let seed = setting("HS_SEED", 5_000);

    let fixture = CandidateFixture::<A, JoinTable>::new();
    // Seeding also proves the *n*-connection open works before anything is
    // timed: a file-descriptor or connection ceiling surfaces here as a panic
    // naming the platform rather than as a number measured quietly at 32.
    let seeded = fixture.open();
    fill(&seeded, seed);
    let durability = seeded.durability().conditions();
    drop(seeded);

    let mut samples: Vec<u128> = Vec::with_capacity(rounds);
    let mut totals = Tally::default();
    for round in 0..rounds {
        let (elapsed, tally) = race(&fixture, contenders, round);
        samples.push(elapsed);
        totals.absorb(&tally);
    }

    samples.sort_unstable();
    totals.waits_ms.sort_unstable();

    let arm = arm();
    let label = label();
    println!(
        "BUSYWAIT\tlabel={label}\trule={rule}\tarm={arm}\tcontenders={contenders}\t\
         rounds={rounds}\tseeded={seed}\t\
         wait_ms_max={}\twait_ms_p99={}\twait_ms_p90={}\twait_ms_p50={}\t\
         race_us_median={}\trace_us_min={}\trace_us_max={}\t\
         committed={}\trejected={}\tbusy={}\tfailed={}\tno_winner={}\t\
         retries={}\tlock_events={}\texhausted={}\treal_wait_ms_max={}\t\
         {}\t{durability}\tbusy_handler={}\tcap_ms={}",
        totals.waits_ms.last().copied().unwrap_or(0),
        percentile(&totals.waits_ms, 99, 100),
        percentile(&totals.waits_ms, 90, 100),
        percentile(&totals.waits_ms, 50, 100),
        samples[samples.len() / 2],
        samples[0],
        samples[samples.len() - 1],
        totals.committed,
        totals.rejected,
        totals.busy,
        totals.failed,
        totals.no_winner,
        totals.busy_stats.retries,
        totals.busy_stats.lock_events,
        totals.busy_stats.exhausted,
        totals.busy_stats.max_real_wait_ms,
        constraint.conditions(),
        busy::handler().as_str(),
        busy::BUSY_TIMEOUT_MS,
    );

    dump_waits(&label, rule, &arm, &totals.waits_ms);
}

/// Writes every per-contender wait out, when `HS_WAITS_DIR` names a directory.
///
/// The printed row carries four percentiles; the distribution is what the
/// question is actually about, so the whole vector is kept and `results/*.md` is
/// written from it by hand. A failure here is a missing raw file rather than a
/// wrong figure, so it is reported and not panicked on.
fn dump_waits(label: &str, rule: &str, arm: &str, waits: &[u64]) {
    let Ok(dir) = std::env::var("HS_WAITS_DIR") else {
        return;
    };
    let path = std::path::Path::new(&dir).join(format!("waits-{label}-{arm}-{rule}.txt"));
    let body: String = waits.iter().map(|wait| format!("{wait}\n")).collect();
    if let Err(err) = std::fs::create_dir_all(&dir).and_then(|()| std::fs::write(&path, body)) {
        println!("NOTE\tcould not write {}: {err}", path.display());
    }
}

/// Dispatches to the arm named by `HS_ARM`.
fn measure_arm(rule: &str) {
    match arm().as_str() {
        "begin-immediate-probe" => measure::<BeginImmediateProbe>(rule),
        "conditional-insert" => measure::<ConditionalInsert>(rule),
        "monotonic-guard" => measure::<MonotonicGuard>(rule),
        other => panic!(
            "HS_ARM={other} is not one of the three strategies \
             `crates/happenstance-sqlite/src/lib.rs:56-62` names"
        ),
    }
}

/// The core constraint, measured rather than assumed.
///
/// Run by `run.sh` in a launch of its own, `--exact`, under the same affinity
/// mask as the races that follow. It is a separate launch because
/// [`cores::establish`]'s speedup probe saturates every core it is allowed, and a
/// probe that ran beside three 64-contender races would be measuring them.
///
/// It asserts nothing about the *measured* count — a loaded host depresses it
/// legitimately — and everything about the reported one, which is the number
/// `run.sh` set and therefore the number a mismatch would indict.
#[test]
fn the_core_constraint_is_what_was_asked_for() {
    let constraint = cores::establish();
    constraint.require_honest();
    println!(
        "CORES\tlabel={}\texpected_mask={:?}\t{}",
        label(),
        constraint.expected,
        constraint.conditions(),
    );
}

/// Stands in for `concurrency.rs:394`.
#[test]
fn exactly_one_of_n_contenders_commits() {
    measure_arm("exactly_one_of_n_contenders_commits");
}

/// Stands in for `concurrency.rs:611`.
#[test]
fn positions_are_unique_under_concurrent_appends() {
    measure_arm("positions_are_unique_under_concurrent_appends");
}

/// Stands in for `concurrency.rs:691`.
#[test]
fn append_returns_the_callers_own_last_position() {
    measure_arm("append_returns_the_callers_own_last_position");
}
