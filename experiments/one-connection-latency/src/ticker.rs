//! The reactor-stall instrument: a 1 ms tick, and the gaps it did not keep.
//!
//! # Why a missed tick is the right measurement
//!
//! A stall is not a slow call. A slow `append` is the caller's problem; a
//! **stalled reactor** is everyone else's — the timers that stop firing, the
//! in-flight HTTP responses that stop being written, the health check that
//! misses. `tokio::time::interval` at 1 ms is the cheapest honest proxy for all
//! of them: it is exactly a timer that should fire, and the largest gap between
//! two consecutive ticks is how long the executor was unavailable to fire it.
//!
//! [`MissedTickBehavior::Delay`] rather than the default `Burst`: under `Burst`
//! a 700 ms stall is followed by 700 immediate catch-up ticks, all of them
//! separated by nanoseconds, and the *maximum* gap is still right while every
//! percentile below it is a fiction about the recovery rather than about the
//! stall. `Delay` re-bases the schedule, so the histogram is a histogram of the
//! reactor's behaviour rather than of the timer wheel's backlog.
//!
//! # The floor is a measurement, not an assumption
//!
//! On Windows the default system timer resolution is 15.6 ms unless something in
//! the process has called `timeBeginPeriod`, so a 1 ms interval on an *idle*
//! current-thread reactor does **not** produce 1 ms gaps. Every table in this
//! crate therefore carries an `idle` row taken under exactly the same
//! construction, and no arm's number means anything except against it. A stall
//! figure quoted without its floor is a figure that has silently attributed the
//! operating system's timer granularity to the adapter.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use tokio::task::JoinHandle;
use tokio::time::MissedTickBehavior;

/// What one ticking task observed.
#[derive(Debug, Clone, Copy, Default)]
pub struct TickReport {
    /// How many gaps were recorded.
    pub ticks: usize,
    /// The largest gap between two consecutive ticks, in microseconds.
    pub max_gap_us: f64,
    /// The 99th-percentile gap, in microseconds.
    pub p99_gap_us: f64,
    /// The median gap, in microseconds.
    pub p50_gap_us: f64,
}

/// A running ticker.
#[derive(Debug)]
pub struct Ticker {
    stop: Arc<AtomicBool>,
    task: JoinHandle<Vec<u64>>,
}

impl Ticker {
    /// Starts ticking at `period` on the current runtime.
    #[must_use]
    pub fn start(period: Duration) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let flag = Arc::clone(&stop);

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(period);
            interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
            // The first tick of a `tokio::time::interval` completes
            // immediately, so it is consumed here rather than recorded: it
            // would otherwise contribute a gap measured from before the timer
            // existed.
            interval.tick().await;

            let mut gaps = Vec::new();
            let mut last = Instant::now();
            loop {
                interval.tick().await;
                let now = Instant::now();
                gaps.push(u64::try_from((now - last).as_nanos()).unwrap_or(u64::MAX));
                last = now;
                if flag.load(Ordering::Relaxed) {
                    break;
                }
            }
            gaps
        });

        Self { stop, task }
    }

    /// Stops the ticker and summarises what it saw.
    ///
    /// The caller must have yielded to the reactor at least once since the
    /// measured operation finished, or the final — and largest — gap has not
    /// been recorded yet. `stall_arm` does that yield; this method does not do
    /// it for you, because doing it here would add a sleep to every arm's
    /// timeline whether or not it needed one.
    ///
    /// # Panics
    ///
    /// Panics if the ticking task panicked, which it can only do by failing to
    /// allocate.
    pub async fn stop(self) -> TickReport {
        self.stop.store(true, Ordering::Relaxed);
        let mut gaps = self.task.await.expect("the ticking task panicked");
        gaps.sort_unstable();

        TickReport {
            ticks: gaps.len(),
            max_gap_us: crate::probe::percentile_us(&gaps, 1.0),
            p99_gap_us: crate::probe::percentile_us(&gaps, 0.99),
            p50_gap_us: crate::probe::percentile_us(&gaps, 0.50),
        }
    }
}
