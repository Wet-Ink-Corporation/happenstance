//! Processor time, and the ratio that makes it worth reporting.
//!
//! # Why a CPU column at all, when there is already a wall clock
//!
//! Because wall time cannot distinguish *working* from *waiting*, and the one
//! open question about this workspace's SQLite adapter is exactly that
//! distinction. `SqliteEventStore::append`, `head` and `contains_event_id` take
//! the connection mutex and run rusqlite synchronously on whatever task polled
//! them, while the sibling module in the same crate routes every SQL-touching
//! body through `SqliteProjectionStore::in_blocking_task`
//! (`crates/happenstance-sqlite/src/projection_store.rs:342`). Findings J-2,
//! F2-1 and I-4 of `references/evaluation/review-pre-publication-2026-09-03.md`
//! all name that asymmetry, and `experiments/one-connection-latency/` has
//! already priced its effect on the reactor at 885.8 ms against a same-crate
//! control's 27.5 ms.
//!
//! What this module adds is the cheap, always-on companion to that experiment:
//! a [`Utilisation`] beside every scenario, so a change in *where* the work
//! happens shows up in the standing suite rather than waiting for someone to
//! re-run a one-off harness.
//!
//! # What the ratio means, and the two ways to misread it
//!
//! [`Utilisation::ratio`] is processor time over elapsed time.
//!
//! * **≈ 1.0** — one thread, busy the whole time. Every single-threaded arm
//!   here should land near it, and an arm that does not is either blocking on
//!   I/O or has found a thread nobody put there.
//! * **&lt; 1.0** — the scenario spent part of its span not running: a real
//!   `await`, a lock wait, a disk flush under `synchronous = NORMAL`.
//! * **&gt; 1.0** — more than one thread ran. Under `spawn_blocking` that is the
//!   point; on an arm documented as single-threaded it is a finding.
//!
//! The two misreadings. First, **this is process-wide**, not per-scenario:
//! `cpu-time`'s `ProcessTime` reads the whole process's user and kernel time,
//! so any other thread alive in the same binary is counted. Every arm that
//! reports a ratio runs alone, `run.sh` passes `--test-threads=1`, and the
//! criterion targets do not report a ratio at all. Second, **the resolution is
//! the scheduler's**, not the timer's: on Windows the accounting granularity is
//! around 15.6 ms, so a scenario shorter than a few hundred milliseconds
//! produces a ratio quantised into visible steps. [`Utilisation::is_resolvable`]
//! says whether a given span cleared that floor, and nothing should quote a
//! ratio for which it is `false`.

use std::fmt;
use std::time::{Duration, Instant};

use cpu_time::ProcessTime;

/// The span below which a ratio is scheduler quantisation rather than a
/// measurement.
///
/// Ten times the ~15.6 ms Windows accounting tick, which is the coarsest of the
/// three platforms the repository builds on. Stated as one constant rather than
/// per-platform on purpose: a threshold that moves with the host makes two runs
/// on two machines disagree about which rows are quotable, and this suite's
/// figures are already only comparable within one run.
pub const RESOLVABLE_FLOOR: Duration = Duration::from_millis(156);

/// Elapsed and processor time for one region, and the ratio between them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utilisation {
    /// Wall-clock time the region took.
    pub elapsed: Duration,
    /// User plus kernel processor time the **process** accrued over the same
    /// span, summed across every thread.
    pub processor: Duration,
}

impl Utilisation {
    /// Processor time over elapsed time. See the module docs for how to read it.
    ///
    /// Returns `0.0` for a zero-length span rather than an infinity, so a
    /// degenerate region cannot poison a mean.
    pub fn ratio(self) -> f64 {
        if self.elapsed.is_zero() {
            return 0.0;
        }
        self.processor.as_secs_f64() / self.elapsed.as_secs_f64()
    }

    /// Whether the region ran long enough for [`ratio`](Self::ratio) to be a
    /// measurement rather than scheduler quantisation.
    pub fn is_resolvable(self) -> bool {
        self.elapsed >= RESOLVABLE_FLOOR
    }
}

impl fmt::Display for Utilisation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "elapsed={:.3}s processor={:.3}s ratio={:.2}{}",
            self.elapsed.as_secs_f64(),
            self.processor.as_secs_f64(),
            self.ratio(),
            if self.is_resolvable() {
                ""
            } else {
                " (below the resolvable floor — do not quote)"
            }
        )
    }
}

/// Runs `f` and returns what it produced alongside what it cost in both clocks.
///
/// Both clocks are started before `f` and read after it, in the order
/// processor-then-wall on entry and wall-then-processor on exit, so the
/// processor span is nested inside the wall span rather than overlapping it. A
/// ratio built the other way can exceed 1.0 on a single thread purely from the
/// two reads' own cost.
pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Utilisation) {
    let processor_start = ProcessTime::now();
    let wall_start = Instant::now();

    let value = std::hint::black_box(f());

    let elapsed = wall_start.elapsed();
    let processor = processor_start.elapsed();

    (value, Utilisation { elapsed, processor })
}
