//! Round-robin paired sampling: the only place in this crate a **ratio** may
//! come from.
//!
//! # Why criterion cannot produce a ratio here
//!
//! criterion runs the members of a `benchmark_group` one after another, each to
//! completion. That is fine for an absolute figure and wrong for a comparison,
//! because anything that drifts across the run — a thermal ceiling, another
//! process, a page cache filling — lands entirely on whichever arm was running
//! at the time.
//!
//! This repository has already paid for that lesson twice, at a cost larger than
//! the effects being measured:
//!
//! * `RUNBOOK.md:1628-1634` — the position-visibility experiment's sequential
//!   design *failed*. The baseline moved **2.7× at one client and 3.0× at 64**,
//!   larger than two of the three effects it was meant to measure. The published
//!   numbers came from a paired design that re-measured the baseline between
//!   every pair of arms, and the discarded pass is kept as evidence rather than
//!   deleted. The instruction left behind: *"any later benchmark in this
//!   repository should assume the same instability."*
//! * `references/adr/0022-append-condition-strategy.md:120-144` — the testkit's
//!   harness emits one `#[test]` per scenario, so each arm is timed in its own
//!   slot; two runs an hour apart disagreed by up to **45%**, and one arm's
//!   *unconditional* append — a path no strategy participates in — varied by
//!   **4×** between slots. The two figures that record decides on come from
//!   caller-side controls that interleave the arms round-robin in one process.
//!
//! So: **absolutes from criterion, ratios from here, and never mixed in one
//! table.**
//!
//! # What "paired" means concretely
//!
//! One round is one operation from every arm. Arms rotate their position within
//! the round, so no arm is permanently first — the first operation in a round
//! pays for whatever the last one left cold, and a fixed order would hand that
//! cost to the same arm every time.
//!
//! A host that gets busy therefore affects every arm equally, and the ratio
//! survives a drift that would destroy the absolutes.
//!
//! # The drift figure is reported, not assumed away
//!
//! Interleaving bounds drift; it does not prove it was small. So every arm's
//! samples are split in half by round, and [`ArmReport::drift`] is the ratio of
//! the second half's median to the first's. A run in which that number is far
//! from 1.0 measured a host that was changing under it, and
//! [`PairedReport::is_stable`] says so in the report rather than leaving a
//! reader to assume it was fine.
//!
//! # The timer's own cost is measured, and arms below it are labelled
//!
//! Every sample here is bracketed by two `Instant::now()` calls, and on Windows
//! that pair costs a few hundred nanoseconds — which is the same order as a
//! `MemoryEventStore` append. An arm whose median sits near that floor is
//! measuring the clock.
//!
//! This is not hypothetical: the first run of
//! `tests/instruments_work.rs::the_paired_sampler_sees_a_difference_it_was_given`
//! reported a spin loop at 500 ns and a loop doing eight times the work at
//! **2 ns**, because the compiler had folded one of them — and the 500 ns was
//! the timer, not the work. So [`run`] measures the empty-region cost at the
//! start of every run, [`PairedReport::timer_overhead_nanos`] carries it, and
//! [`ArmReport::is_timer_dominated`] says of each arm whether its median
//! cleared it by the [`TIMER_HEADROOM`] factor.
//!
//! An arm that did not is not a wrong number, it is *not a number*: use
//! criterion for that scale, which amortises the clock across a batch of
//! iterations instead of paying for it per operation.
//!
//! **Nothing here asserts on any of it.** CF-34 forbids a benchmark result
//! gating a merge, and a drift threshold would be exactly that. The number is
//! printed and a human reads it.

use std::fmt;
use std::time::Instant;

use hdrhistogram::Histogram;

/// How many rounds are discarded before recording starts, unless a caller says
/// otherwise.
///
/// Enough to fault in the code path, fill a prepared-statement cache and let
/// the allocator reach a steady state, and small enough that a 200-round run
/// still spends most of itself recording.
pub const DEFAULT_WARMUP_ROUNDS: usize = 16;

/// How many times an arm's median must exceed the timer overhead before the
/// arm is reporting its own cost rather than the clock's.
///
/// Ten, so that at worst a tenth of a reported median is instrument. Below it
/// the figure is still printed - suppressing it would hide that the arm was
/// attempted - but it is labelled, and no ratio drawn from it should be quoted.
pub const TIMER_HEADROOM: u64 = 10;

/// The widest span one sample may record: one hour, in nanoseconds.
///
/// Stated as an explicit bound rather than left to `Histogram::new`, and that
/// is not a style choice. `Histogram::new(sigfig)` starts with a recordable
/// range of **1 to 2**, and `saturating_record` does exactly what its name says
/// — so every sample this crate took was silently clamped to `2`, and the first
/// run of `tests/instruments_work.rs::the_paired_sampler_sees_a_difference_it_was_given`
/// reported an arm doing eight times the work at a ratio of **1.00x**, with a
/// median, p95, p99 and max all equal to 2 ns.
///
/// The tell was that the *max* equalled the *median*. A histogram whose extreme
/// and centre agree to the digit across 400 samples is not measuring a
/// distribution, and that is the shape to look for if this ever recurs.
const MAX_RECORDABLE_NANOS: u64 = 3_600_000_000_000;

/// A histogram sized for a wall-clock span, in nanoseconds.
///
/// Three significant figures because this crate does not quote a third
/// significant figure anyway — one machine, one run per cell — so storing more
/// would be precision the report is forbidden from using.
fn nanosecond_histogram() -> Histogram<u64> {
    let mut histogram = Histogram::<u64>::new_with_bounds(1, MAX_RECORDABLE_NANOS, 3)
        .expect("1ns to 1h at three significant figures is a valid HdrHistogram");
    // Belt as well as braces: a span longer than an hour grows the histogram
    // rather than being clamped to it. A clamped sample is a wrong number that
    // looks like a real one.
    histogram.auto(true);
    histogram
}

/// How many samples the timer-overhead probe takes.
///
/// Enough for a stable median on a host whose scheduler quantum is 15.6 ms, and
/// small enough to be invisible against a run's own cost.
const TIMER_PROBE_SAMPLES: usize = 2_048;

/// The cost of the two clock reads that bracket every sample, in nanoseconds.
///
/// # Why this is not "time an empty region"
///
/// That was the first version, and it reported **0 ns** — which made
/// [`ArmReport::is_timer_dominated`] a check that could never fire, because
/// nothing is below `0 * TIMER_HEADROOM`. A guard that cannot fire is
/// decoration, and this repository already applies that test to conformance
/// rules (`CLAUDE.md`: *"a rule that no adapter can fail is decorative"*).
///
/// The reason it read zero is that the delta between `Instant::now()` and
/// `started.elapsed()` excludes most of what the pair costs: the first `now()`
/// has already returned before the span opens, and the second read's own cost
/// falls after it closes. The delta measures the *gap*, not the *calls*.
///
/// So the probe amortises instead: it times a batch of clock reads and divides.
/// One sample pays for two reads — the `now()` that opens it and the `now()`
/// inside `elapsed()` that closes it — hence the doubling. On the reference
/// host this lands in the tens of nanoseconds, which is the same order as a
/// `MemoryEventStore` append and exactly why the guard is worth having.
fn timer_overhead_nanos() -> u64 {
    let started = Instant::now();
    for _ in 0..TIMER_PROBE_SAMPLES {
        let _ = std::hint::black_box(Instant::now());
    }
    let elapsed = started.elapsed();

    let per_call = u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX)
        / u64::try_from(TIMER_PROBE_SAMPLES).unwrap_or(1);
    // Two reads per sample. `max(1)` so a host whose clock is too coarse to
    // resolve its own cost still yields a floor of one nanosecond rather than
    // zero — which would silently restore the un-fireable guard this function
    // exists to have fixed.
    (per_call * 2).max(1)
}

/// One arm's samples.
///
/// The name is not held here: it lives on the caller's [`Operation`], and the
/// report is built by zipping the two. One owner for one string.
struct Arm {
    /// Every recorded sample, in nanoseconds.
    all: Histogram<u64>,
    /// Samples from the first half of the recorded rounds.
    first_half: Histogram<u64>,
    /// Samples from the second half.
    second_half: Histogram<u64>,
}

impl Arm {
    fn new() -> Self {
        Self {
            all: nanosecond_histogram(),
            first_half: nanosecond_histogram(),
            second_half: nanosecond_histogram(),
        }
    }

    fn record(&mut self, nanos: u64, in_first_half: bool) {
        self.all.saturating_record(nanos);
        if in_first_half {
            self.first_half.saturating_record(nanos);
        } else {
            self.second_half.saturating_record(nanos);
        }
    }
}

/// What one arm cost, across the whole run and across each half of it.
#[derive(Debug, Clone, Copy)]
pub struct ArmReport {
    /// The arm's name, as the caller gave it.
    pub name: &'static str,
    /// How many operations were recorded.
    pub samples: u64,
    /// Median nanoseconds per operation.
    pub median_nanos: u64,
    /// 95th percentile.
    pub p95_nanos: u64,
    /// 99th percentile.
    pub p99_nanos: u64,
    /// The slowest single operation.
    pub max_nanos: u64,
    /// Second-half median over first-half median.
    ///
    /// 1.0 is a host that did not change under the run. Far from 1.0 in either
    /// direction means the absolutes below are about a moving target, and the
    /// ratio between arms is the only thing worth reading.
    pub drift: f64,
    /// The run's measured timer overhead, carried on every arm so a row is
    /// self-describing wherever it is copied to.
    pub timer_overhead_nanos: u64,
}

impl ArmReport {
    /// Whether this arm's median cleared the timer overhead by
    /// [`TIMER_HEADROOM`].
    ///
    /// `true` means the figure is mostly instrument. Reach for criterion at
    /// that scale: it amortises the clock across a batch instead of paying for
    /// it once per operation.
    pub const fn is_timer_dominated(&self) -> bool {
        self.median_nanos < self.timer_overhead_nanos.saturating_mul(TIMER_HEADROOM)
    }
}

impl fmt::Display for ArmReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<28} n={:<7} median={:>12}ns p95={:>12}ns p99={:>12}ns max={:>12}ns              drift={:.3}{}",
            self.name,
            self.samples,
            self.median_nanos,
            self.p95_nanos,
            self.p99_nanos,
            self.max_nanos,
            self.drift,
            if self.is_timer_dominated() {
                " TIMER-DOMINATED"
            } else {
                ""
            }
        )
    }
}

/// Every arm of one paired run.
#[derive(Debug, Clone)]
pub struct PairedReport {
    /// The arms, in the order they were given.
    pub arms: Vec<ArmReport>,
    /// Rounds actually recorded, after warm-up.
    pub rounds: usize,
    /// The median cost of the `Instant::now()` pair that brackets every sample,
    /// measured on this host at the start of this run.
    pub timer_overhead_nanos: u64,
}

/// How far a per-arm drift may sit from 1.0 before a run is called unstable.
///
/// Not a threshold that fails anything — see the module docs; it is the line
/// below which the report stops printing a caveat, and 15% is chosen against
/// the 45% and 4× swings ADR-0022 recorded on this class of host.
pub const STABLE_DRIFT_MARGIN: f64 = 0.15;

impl PairedReport {
    /// One arm by name.
    pub fn arm(&self, name: &str) -> Option<&ArmReport> {
        self.arms.iter().find(|arm| arm.name == name)
    }

    /// `arm`'s median over `baseline`'s — what the arm costs above the floor.
    ///
    /// `None` if either name is absent or the baseline's median is zero, rather
    /// than an infinity that would read as a finding.
    pub fn ratio(&self, arm: &str, baseline: &str) -> Option<f64> {
        let arm = self.arm(arm)?;
        let baseline = self.arm(baseline)?;
        if baseline.median_nanos == 0 {
            return None;
        }
        #[allow(clippy::cast_precision_loss)]
        Some(arm.median_nanos as f64 / baseline.median_nanos as f64)
    }

    /// Whether every arm cleared the timer overhead by [`TIMER_HEADROOM`].
    ///
    /// A `false` means at least one row is mostly instrument, and the report
    /// says which.
    pub fn is_above_the_timer(&self) -> bool {
        self.arms.iter().all(|arm| !arm.is_timer_dominated())
    }

    /// Whether every arm's drift sits inside [`STABLE_DRIFT_MARGIN`].
    ///
    /// A `false` does not invalidate the ratios — interleaving is what protects
    /// those — but it does invalidate quoting any absolute from this run.
    pub fn is_stable(&self) -> bool {
        self.arms
            .iter()
            .all(|arm| (arm.drift - 1.0).abs() <= STABLE_DRIFT_MARGIN)
    }
}

impl fmt::Display for PairedReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "paired run: {} recorded rounds, timer overhead {}ns/sample",
            self.rounds, self.timer_overhead_nanos
        )?;
        for arm in &self.arms {
            writeln!(f, "  {arm}")?;
        }
        if !self.is_above_the_timer() {
            writeln!(
                f,
                "  NOTE: an arm marked TIMER-DOMINATED has a median below \
                 {TIMER_HEADROOM}x the timer overhead, so most of what it reports is the \
                 clock. Measure that scale with criterion, which amortises the clock \
                 across a batch of iterations instead of paying for it per operation."
            )?;
        }
        if !self.is_stable() {
            writeln!(
                f,
                "  NOTE: at least one arm's drift is outside \u{b1}{:.0}%. The ratios between \
                 arms still hold — that is what interleaving buys — but no absolute from this \
                 run should be quoted.",
                STABLE_DRIFT_MARGIN * 100.0
            )?;
        }
        Ok(())
    }
}

/// One arm: a name and the operation to time.
///
/// The closure is `FnMut` because an arm normally owns a store and mutates it.
/// It returns a value, discarded through [`std::hint::black_box`], so an arm
/// whose result is unused cannot be optimised away — which at `opt-level = 3`
/// is what would happen to a read whose rows nobody looks at, reporting a scan
/// as free.
pub struct Operation<'a> {
    name: &'static str,
    run: Box<dyn FnMut() + 'a>,
}

impl<'a> Operation<'a> {
    /// Names an operation and the closure that performs it once.
    pub fn new(name: &'static str, run: impl FnMut() + 'a) -> Self {
        Self {
            name,
            run: Box::new(run),
        }
    }
}

impl fmt::Debug for Operation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operation")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// Runs every operation `rounds` times, round-robin, and reports what each cost.
///
/// Rounds are one operation per arm. The starting arm rotates each round, so no
/// arm is permanently first.
///
/// # Panics
///
/// Panics if `operations` is empty, or if `rounds` is zero — a paired run with
/// nothing to pair, or with nothing to record, produces a report a reader would
/// take at face value.
pub fn run(rounds: usize, warmup: usize, operations: &mut [Operation<'_>]) -> PairedReport {
    assert!(
        !operations.is_empty(),
        "a paired run needs at least one arm; with none it would report an \
         empty table that reads like a measurement"
    );
    assert!(rounds > 0, "a paired run must record at least one round");

    let timer_overhead_nanos = timer_overhead_nanos();
    let width = operations.len();

    for round in 0..warmup {
        for slot in 0..width {
            (operations[(round + slot) % width].run)();
        }
    }

    let mut arms: Vec<Arm> = (0..width).map(|_| Arm::new()).collect();
    let midpoint = rounds.div_ceil(2);

    for round in 0..rounds {
        let in_first_half = round < midpoint;
        for slot in 0..width {
            let index = (round + slot) % width;
            let started = Instant::now();
            (operations[index].run)();
            let elapsed = started.elapsed();
            arms[index].record(
                u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX),
                in_first_half,
            );
        }
    }

    let reported = arms
        .iter()
        .zip(operations.iter())
        .map(|(arm, operation)| {
            #[allow(clippy::cast_precision_loss)]
            let drift = {
                let first = arm.first_half.value_at_quantile(0.5) as f64;
                let second = arm.second_half.value_at_quantile(0.5) as f64;
                if first == 0.0 { 1.0 } else { second / first }
            };
            ArmReport {
                name: operation.name,
                samples: arm.all.len(),
                median_nanos: arm.all.value_at_quantile(0.5),
                p95_nanos: arm.all.value_at_quantile(0.95),
                p99_nanos: arm.all.value_at_quantile(0.99),
                max_nanos: arm.all.max(),
                drift,
                timer_overhead_nanos,
            }
        })
        .collect();

    PairedReport {
        arms: reported,
        rounds,
        timer_overhead_nanos,
    }
}
