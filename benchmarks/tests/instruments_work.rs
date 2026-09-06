//! The instruments, forced to fire.
//!
//! # Why this file exists, in one sentence from the house rules
//!
//! `experiments/gate-vacuity/README.md:59` — *"Controls before counts."* An
//! instrument that has quietly stopped working reports zero, or reports a
//! constant, and both look like findings. Every check here fails when the
//! *instrument* is broken and never when a number is large, which is the line
//! CF-34 draws (`spec/SPECIFICATION.md:8747`): a benchmark result may not gate
//! a merge, but a benchmark harness that cannot measure may.
//!
//! # This binary installs the counting allocator; the criterion targets do not
//!
//! A `#[global_allocator]` is a per-binary singleton, and each file under
//! `tests/` is its own binary. Installing it here costs the criterion targets
//! nothing and gives the allocator a place to be proven.
//!
//! # Run this single-threaded
//!
//! The counters are process-global, so a second test thread's allocations land
//! inside whatever region happens to be open. `run.sh` passes
//! `--test-threads=1`; a parallel run of this file will report counts that are
//! too high and is not a finding about anything.

use happenstance_benchmarks::corpus::{Corpus, FLOOR_TAGS_PER_EVENT, Regime, Shape};
use happenstance_benchmarks::counting::{self, Counting};
use happenstance_benchmarks::cpu;
use happenstance_benchmarks::paired::{self, Operation};

/// The counting allocator, installed for this test binary and nothing else.
#[global_allocator]
static ALLOCATOR: Counting = Counting;

/// CONTROL — the allocator counts an allocation that certainly happened.
///
/// A `Vec` grown to a known capacity is one `alloc`. If this reports zero, the
/// allocator is installed but not counting, and every allocation figure in
/// `results/` is a zero nobody would question.
#[test]
fn the_allocator_counts_a_known_allocation() {
    let (value, region) = counting::measure(|| Vec::<u64>::with_capacity(1024));

    assert_eq!(
        region.counts.allocs, 1,
        "one `Vec::with_capacity` is exactly one `alloc`; got {region}"
    );
    assert!(
        region.counts.bytes >= 1024 * 8,
        "1,024 u64s request at least 8,192 bytes; got {region}"
    );
    assert_eq!(
        value.capacity(),
        1024,
        "the measured value is returned, not dropped inside the region"
    );
}

/// CONTROL — the peak is a high-water mark, not the residue.
///
/// A region that allocates a large buffer and frees it before returning must
/// report a peak far above what it retained. An instrument that only sampled
/// live bytes at the end would report ~0 here, and would then price a
/// scenario's transient working set at nothing.
#[test]
fn the_peak_sees_a_buffer_that_was_freed_before_the_region_closed() {
    const BIG: usize = 4 * 1024 * 1024;

    let ((), region) = counting::measure(|| {
        let transient = vec![0_u8; BIG];
        drop(std::hint::black_box(transient));
    });

    let big = i64::try_from(BIG).expect("4 MiB fits an i64");
    assert!(
        region.peak_above_baseline >= big,
        "a {BIG}-byte buffer was live inside the region, so the peak must see \
         it even though it was freed before the region closed; got {region}"
    );
    assert!(
        region.retained < big,
        "and it was freed, so almost none of it is retained; got {region}"
    );
}

/// CONTROL — regions refuse to nest, by name.
///
/// The peak is one pair of statics. An inner region would reset the outer's
/// high-water mark and the outer would then report the inner's peak as its own
/// — two figures that disagree with no way to tell which is wrong.
#[test]
#[should_panic(expected = "do not nest")]
fn a_nested_region_is_refused() {
    let _ = counting::measure(|| counting::measure(|| ()));
}

/// **THE control this whole suite turns on** — the two regimes are separated by
/// more than an order of magnitude, and the interned arm does not respond to
/// tag count at all.
///
/// `references/evaluation/review-pre-publication-2026-09-03.md:2724` measured
/// 66 heap operations against 1 at VT-22's 64-tag floor. If this test ever goes
/// quiet — if the two arms converge — then either the contract changed or
/// `crate::corpus` stopped building what it says it builds, and every figure in
/// `results/` carrying a regime column has become decoration.
///
/// The numbers asserted are deliberately loose (an order of magnitude, and
/// *flat* meaning "within one operation"). The exact counts belong in
/// `results/`, where they are reported rather than gated.
#[test]
fn the_two_allocation_regimes_are_separated_by_an_order_of_magnitude() {
    let clone_cost = |regime: Regime, tags: usize| {
        let corpus = Corpus::uniform(Shape::new(1024, tags, regime));
        let event = corpus.event(0);
        // The *second* clone, not the first: a payload built with
        // `Bytes::from(Vec<u8>)` starts promotable and allocates a shared
        // header on its first clone only. Measuring the first would report 67
        // where the steady state is 66, and the difference would look like
        // noise rather than like the documented behaviour it is.
        let warm = event.clone();
        let (_kept, region) = counting::measure(|| warm.clone());
        region.counts.heap_ops()
    };

    let owned = clone_cost(Regime::Owned, FLOOR_TAGS_PER_EVENT);
    let interned = clone_cost(Regime::Interned, FLOOR_TAGS_PER_EVENT);

    println!(
        "regime separation at {FLOOR_TAGS_PER_EVENT} tags: owned={owned} heap ops, \
         interned={interned} heap ops"
    );

    assert!(
        owned >= interned * 10,
        "the owned regime must cost at least an order of magnitude more than \
         the interned one at the {FLOOR_TAGS_PER_EVENT}-tag conformance floor \
         — the review measured 66 against 1. Got owned={owned}, \
         interned={interned}. If these have converged, every regime column in \
         results/ is decoration"
    );

    let interned_at_one = clone_cost(Regime::Interned, 1);
    assert!(
        interned.abs_diff(interned_at_one) <= 1,
        "the interned regime is flat in tag count — that is what makes it the \
         arm a benchmark author writes by accident and then reports as the \
         library's cost. Got {interned} at {FLOOR_TAGS_PER_EVENT} tags and \
         {interned_at_one} at 1"
    );

    let owned_at_one = clone_cost(Regime::Owned, 1);
    assert!(
        owned > owned_at_one,
        "and the owned regime is *not* flat: it must respond to tag count. Got \
         {owned} at {FLOOR_TAGS_PER_EVENT} tags and {owned_at_one} at 1"
    );
}

/// CONTROL — the processor clock distinguishes working from waiting.
///
/// Two arms with the same wall-clock span and opposite processor spans. If both
/// report the same ratio the CPU column is measuring nothing, and the
/// `spawn_blocking` asymmetry this suite exists partly to watch would be
/// invisible.
///
/// The margins are enormous on purpose — 0.5 against 0.5 — because this must
/// not go red on a loaded host. It fires when the clock is broken, not when the
/// machine is busy.
#[test]
fn the_processor_clock_tells_working_from_waiting() {
    let span = cpu::RESOLVABLE_FLOOR + std::time::Duration::from_millis(100);

    let ((), busy) = cpu::measure(|| {
        let deadline = std::time::Instant::now() + span;
        let mut turns = 0_u64;
        while std::time::Instant::now() < deadline {
            turns = turns.wrapping_add(1);
        }
        let _ = std::hint::black_box(turns);
    });

    let ((), waiting) = cpu::measure(|| std::thread::sleep(span));

    println!("busy: {busy}");
    println!("waiting: {waiting}");

    assert!(
        busy.is_resolvable() && waiting.is_resolvable(),
        "both regions were built to exceed the resolvable floor; if they did \
         not, the floor or the wall clock is wrong. busy={busy} waiting={waiting}"
    );
    assert!(
        busy.ratio() > 0.5,
        "a spin loop must show the processor busy for most of its span; got {busy}"
    );
    assert!(
        waiting.ratio() < 0.5,
        "a sleep must show the processor idle for most of its span; got {waiting}"
    );
}

/// CONTROL — the paired sampler reports a ratio it was given.
///
/// One arm does a fixed amount of work; the other does roughly eight times as
/// much. The sampler must see it. A sampler that reported 1.0 here would report
/// 1.0 for the overhead above the raw-SQL floor too, and that is the headline
/// figure of this whole suite.
///
/// Wide bounds — anything from 2× to 40× passes — because the point is that the
/// instrument *responds*, not that a spin loop scales linearly.
#[test]
fn the_paired_sampler_sees_a_difference_it_was_given() {
    // `black_box` on the *input*, not only on the result. With a literal
    // iteration count the whole loop constant-folds at `opt-level = 3` — the
    // first version of this control reported the eight-times-heavier arm at
    // **2 ns**, because there was no loop left to run. Blackboxing the output
    // does not help: it stops the value being discarded, not the value being
    // computed at compile time.
    fn spin(iterations: u64) {
        let iterations = std::hint::black_box(iterations);
        let mut accumulator = 0_u64;
        for step in 0..iterations {
            accumulator = accumulator.wrapping_add(step).wrapping_mul(2_654_435_761);
        }
        let _ = std::hint::black_box(accumulator);
    }

    let mut operations = [
        Operation::new("light", || spin(20_000)),
        Operation::new("heavy", || spin(160_000)),
    ];
    let report = paired::run(400, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    println!("{report}");

    let ratio = report
        .ratio("heavy", "light")
        .expect("both arms were named and both recorded samples");
    assert!(
        (2.0..=40.0).contains(&ratio),
        "an arm doing eight times the work must read as materially more \
         expensive than the light one; got {ratio:.2}×. A sampler that cannot \
         see this cannot see the overhead above the raw-SQL floor either"
    );

    assert!(
        report.is_above_the_timer(),
        "both arms were sized to clear the timer overhead by {}x. If they no \
         longer do, the host's clock got slower or the arms got faster, and \
         the iteration counts in this control need revisiting — a control that \
         is itself timer-dominated proves nothing about the sampler:\n{report}",
        paired::TIMER_HEADROOM
    );

    for arm in &report.arms {
        assert_eq!(
            arm.samples, 400,
            "every arm records exactly one sample per round: {arm}"
        );
        assert!(
            arm.p99_nanos >= arm.median_nanos,
            "percentiles must be ordered; got {arm}"
        );
    }
}

/// CONTROL — the paired sampler reports ~1.0 for two identical arms.
///
/// The negative control for the test above. Without it, a sampler that always
/// reported a large ratio would pass that one and be wrong everywhere.
#[test]
fn the_paired_sampler_reports_no_difference_where_there_is_none() {
    // See the sibling control above for why the *input* is blackboxed.
    fn spin() {
        let iterations = std::hint::black_box(20_000_u64);
        let mut accumulator = 0_u64;
        for step in 0..iterations {
            accumulator = accumulator.wrapping_add(step).wrapping_mul(2_654_435_761);
        }
        let _ = std::hint::black_box(accumulator);
    }

    let mut operations = [Operation::new("left", spin), Operation::new("right", spin)];
    let report = paired::run(400, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    println!("{report}");

    let ratio = report
        .ratio("right", "left")
        .expect("both arms were named and both recorded samples");
    assert!(
        (0.5..=2.0).contains(&ratio),
        "two arms running the same function must not differ by more than a \
         factor of two even on a busy host; got {ratio:.2}×. A sampler that \
         manufactures a difference here would manufacture one everywhere"
    );
}

/// CONTROL — a paired run with no arms is refused rather than reported.
///
/// An empty table reads exactly like a measurement of nothing, and nothing is
/// what it would be.
#[test]
#[should_panic(expected = "at least one arm")]
fn a_paired_run_with_no_arms_is_refused() {
    let _ = paired::run(10, 0, &mut []);
}

/// CONTROL — the timer-domination guard fires.
///
/// The guard exists because a `MemoryEventStore` append is the same order of
/// magnitude as two clock reads, and a per-operation timer cannot see something
/// that small. A guard that could never fire would be decoration — which is
/// exactly what it was in the first version of `crate::paired`, where the
/// overhead probe read **0 ns** and so nothing could ever fall below it.
///
/// So this arm does as close to nothing as an arm can, and must be labelled.
#[test]
fn the_timer_domination_guard_fires_on_an_arm_that_does_nothing() {
    let mut operations = [Operation::new("empty", || {
        let _ = std::hint::black_box(0_u64);
    })];
    let report = paired::run(400, paired::DEFAULT_WARMUP_ROUNDS, &mut operations);
    println!("{report}");

    assert!(
        report.timer_overhead_nanos > 0,
        "the overhead probe must report a positive floor; a zero floor makes          the guard unfireable, which is what it was before the probe started          amortising over a batch. Got {report}"
    );
    assert!(
        !report.is_above_the_timer(),
        "an arm that does nothing must be labelled TIMER-DOMINATED. If it is          not, the guard is decoration and every sub-microsecond figure this          crate prints is unlabelled instrument. Got {report}"
    );
}
