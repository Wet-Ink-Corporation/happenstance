//! How many cores the process is actually allowed to run on, verified twice.
//!
//! # `taskset` does not exist on this machine
//!
//! The finding this experiment answers proposes `taskset -c 0,1`. This host is
//! Windows 11, so the constraint is a **processor affinity mask**, applied by
//! `affinity-run.ps1` to *itself* before it starts the test binary, which then
//! inherits it at creation the way every Windows child does. Inheritance rather
//! than assignment is deliberate: `Start-Process -PassThru` followed by
//! `$p.ProcessorAffinity = …` leaves a window in which the child is already
//! running unconstrained, and a seeding phase that ran on twenty cores before
//! the mask landed would produce a "2-core" row that is nothing of the kind.
//! `affinity-run.ps1` carries the whole argument, including why `cmd /c start
//! /affinity` and an FFI `SetProcessAffinityMask` both lost.
//!
//! # Why the mask is verified rather than assumed, and why twice
//!
//! An affinity call that silently fails produces a twenty-core number wearing a
//! two-core label, which is worse than no number at all. So the binary checks
//! its own constraint before it measures anything, by two independent routes,
//! and prints both beside every row:
//!
//! 1. **What the operating system says.** [`reported_mask`] asks Windows for
//!    this process's own `ProcessorAffinity`. That is authoritative and it is
//!    also exactly the quantity `run.sh` set, so a mismatch is a launcher bug.
//! 2. **What the machine does.** [`effective_cores`] runs a fixed amount of
//!    CPU-bound work on one thread and then the same work on many threads, and
//!    reports the speedup. That is not authoritative — a busy host depresses it
//!    — but it is *independent*: it would catch a mask that was reported and not
//!    enforced, which route 1 cannot.
//!
//! Route 1 shells out to PowerShell rather than calling `GetProcessAffinityMask`
//! through FFI. The workspace forbids `unsafe` outright and this crate keeps
//! that (`#![forbid(unsafe_code)]` in `lib.rs`); one 300 ms process spawn per
//! test binary is a cheaper price than an `unsafe` block and a `windows-sys`
//! dependency in an experiment.
//!
//! `std::thread::available_parallelism` is deliberately **not** used as either
//! route. On Linux it consults `sched_getaffinity` and would be exactly right;
//! on Windows it reports the machine's processors and ignores the process
//! affinity mask entirely, so it would have answered "20" under every mask below.

use std::time::{Duration, Instant};

/// How the process's core constraint was established and checked.
#[derive(Debug, Clone)]
pub struct Constraint {
    /// The mask `run.sh` intended, from `HS_EXPECTED_AFFINITY`, or `None` when
    /// the binary was run directly.
    pub expected: Option<u64>,
    /// The mask Windows reports for this process, or `None` if it could not be
    /// read.
    pub reported: Option<u64>,
    /// Cores implied by the reported mask.
    pub reported_cores: Option<u32>,
    /// Cores implied by the measured parallel speedup, rounded to one decimal.
    pub measured_cores: f64,
}

impl Constraint {
    /// Whether the constraint that was asked for is the constraint in force.
    ///
    /// A missing expectation is *not* a mismatch — the binary is runnable by
    /// hand — but a stated expectation that the operating system contradicts is,
    /// and [`Constraint::require_honest`] turns that into a refusal to measure.
    #[must_use]
    pub fn agrees(&self) -> bool {
        match (self.expected, self.reported) {
            (Some(expected), Some(reported)) => expected == reported,
            (Some(_), None) => false,
            (None, _) => true,
        }
    }

    /// Aborts the run rather than emit a mislabelled figure.
    ///
    /// The same shape as [`crate::Durability::require_shippable`], and for the
    /// same reason: `experiments/append-condition` refuses to print a number
    /// under a pragma the adapter may not ship, and this refuses to print one
    /// under a core count it cannot demonstrate.
    ///
    /// # Panics
    ///
    /// Panics when `HS_EXPECTED_AFFINITY` was set and the operating system
    /// reports a different mask, or none.
    pub fn require_honest(&self) {
        assert!(
            self.agrees(),
            "refusing to measure: HS_EXPECTED_AFFINITY={:?} but this process's \
             reported affinity mask is {:?}. A mask that did not take effect \
             produces a twenty-core number wearing a two-core label",
            self.expected,
            self.reported,
        );
    }

    /// The one field a results row carries beside every figure.
    #[must_use]
    pub fn conditions(&self) -> String {
        format!(
            "affinity_mask={} reported_cores={} measured_cores={:.1}",
            self.reported
                .map_or_else(|| "unknown".to_owned(), |mask| format!("0x{mask:x}")),
            self.reported_cores
                .map_or_else(|| "unknown".to_owned(), |cores| cores.to_string()),
            self.measured_cores,
        )
        .replace("measured_cores=-1.0", "measured_cores=n/a")
    }
}

/// Establishes the constraint and both checks on it.
///
/// The speedup probe saturates every core it is allowed, so this must run in a
/// process of its own — `run.sh` invokes it as a separate `--exact` launch under
/// the same affinity mask, ahead of the racing tests. Calling it from inside a
/// racing test would have the probe and the races measuring each other.
#[must_use]
pub fn establish() -> Constraint {
    let mut constraint = verify();
    constraint.measured_cores = effective_cores();
    constraint
}

/// The cheap half: what the operating system says, with no probe.
///
/// This is what every racing test calls, because it is the *authoritative*
/// check and it costs one process spawn. The expensive, independent one is
/// [`establish`].
#[must_use]
pub fn verify() -> Constraint {
    let expected = std::env::var("HS_EXPECTED_AFFINITY")
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok());
    let reported = reported_mask();

    Constraint {
        expected,
        reported,
        reported_cores: reported.map(u64::count_ones),
        // Not measured here, and printed as `n/a` rather than as a zero that
        // would read as "no cores".
        measured_cores: -1.0,
    }
}

/// This process's affinity mask as Windows reports it.
///
/// `None` when PowerShell is not available or answers something unparsable,
/// which is a degraded run rather than a wrong one — [`Constraint::agrees`]
/// treats it as a mismatch whenever a mask was expected.
fn reported_mask() -> Option<u64> {
    let pid = std::process::id();
    let output = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &format!("[int64](Get-Process -Id {pid}).ProcessorAffinity"),
        ])
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    text.trim().parse::<i64>().ok().map(|mask| mask as u64)
}

/// Threads the speedup probe runs, comfortably above the widest mask measured.
const PROBE_THREADS: usize = 24;

/// Iterations of the probe's inner loop, sized so one thread takes on the order
/// of 100 ms in a **debug** build — the build every figure in this crate except
/// the control is produced under.
const PROBE_ITERATIONS: u64 = 4_000_000;

/// Cores this process can actually use, measured rather than reported.
///
/// One thread does [`PROBE_ITERATIONS`] of arithmetic the optimiser cannot elide;
/// then [`PROBE_THREADS`] threads each do the same. If the process is confined to
/// *c* cores the wall time of the second is about `PROBE_THREADS / c` times the
/// first, so the ratio recovers *c*.
///
/// It is an estimate and it is stated as one. It reads low on a loaded host and
/// it reads high where the scheduler has hyper-threads to hand out; what it is
/// good for is catching the failure this experiment must not have — a mask that
/// was set on paper and not in fact — where it would read 20 against a stated 2.
#[must_use]
pub fn effective_cores() -> f64 {
    // A warm-up pass, discarded: the first run of anything on Windows pays for
    // page faults and a frequency ramp that would be charged to the serial half.
    spin(PROBE_ITERATIONS / 4);

    let started = Instant::now();
    spin(PROBE_ITERATIONS);
    let serial = started.elapsed();

    let started = Instant::now();
    std::thread::scope(|scope| {
        for _ in 0..PROBE_THREADS {
            scope.spawn(|| spin(PROBE_ITERATIONS));
        }
    });
    let parallel = started.elapsed();

    if parallel <= Duration::ZERO || serial <= Duration::ZERO {
        return 0.0;
    }
    let ratio = serial.as_secs_f64() / parallel.as_secs_f64();
    (PROBE_THREADS as f64 * ratio * 10.0).round() / 10.0
}

/// Work the optimiser has to actually do.
///
/// `black_box` on the accumulator each iteration, because a pure arithmetic loop
/// with an unread result is legally deletable and a probe that measured nothing
/// would report a bottomless core count.
fn spin(iterations: u64) -> u64 {
    let mut accumulator = 0x9e37_79b9_7f4a_7c15_u64;
    for index in 0..iterations {
        accumulator = accumulator
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(index);
        accumulator = std::hint::black_box(accumulator);
    }
    accumulator
}
