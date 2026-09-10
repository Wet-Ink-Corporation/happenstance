//! Pin the calling thread to one CPU, and put the mask back afterwards.
//!
//! # Why a test needs this at all
//!
//! `instruments_work.rs`'s processor-clock control measures a busy region and
//! asserts the processor reads busy. On this repository's Linux measurement host
//! it read *idle* — `elapsed=0.256s processor=0.000s` — roughly one run in five,
//! and the cause is not the control:
//!
//! The kernel there has marked the TSC unstable (`ops/host/README.md` records
//! why, and why firmware cannot fix it), so `sched_clock` runs on a per-CPU
//! fallback. A task that **migrates between CPUs mid-region** comes back with
//! its accrued runtime mis-accounted, and `CLOCK_PROCESS_CPUTIME_ID` reports a
//! busy quarter-second as zero.
//!
//! Measured, ten runs per configuration:
//!
//! | affinity | failures |
//! | --- | --- |
//! | unpinned | 2 of 10 |
//! | four CPUs — the `bench.sh` mask | 4 of 10 |
//! | **one CPU** | **0 of 10** |
//!
//! Pinning is therefore the mechanism, not a hedge. A bounded retry was tried
//! first and rejected: standalone it cleared the flake, but inside a full
//! `run.sh` the mis-accounting persisted across all three attempts (0.12, 0.00,
//! 0.00) and the run still aborted. It can persist, not merely flicker.
//!
//! # What this does not do
//!
//! It does not make the assertion unfireable, which is the trap. Pinning removes
//! a *confound* — a clock that genuinely could not tell working from waiting
//! would still read idle on one CPU, and the control still fails on it. The
//! wrong implementation it rejects is unchanged; what changed is that a host
//! property stopped being able to fake one.
//!
//! Linux only. Everywhere else this is a no-op that reports itself as such, so
//! the control runs identically and simply has nothing to remove.

#![allow(dead_code)] // Each test binary links only what it uses.

/// A thread pinned to a single CPU for as long as this value lives.
pub(crate) struct PinnedToOneCpu {
    #[cfg(target_os = "linux")]
    previous: libc::cpu_set_t,
    /// Whether the pin actually took. `false` on a platform without affinity, or
    /// where the calls declined — see [`PinnedToOneCpu::describe`].
    pinned: bool,
    cpu: i32,
}

impl PinnedToOneCpu {
    /// Pins to whichever CPU the caller is already running on.
    ///
    /// Never fails: a refused or unavailable pin yields a value that reports
    /// itself as unpinned rather than an error, because the caller's job is to
    /// measure either way and to say which it did.
    #[cfg(target_os = "linux")]
    pub(crate) fn here() -> Self {
        // SAFETY: every call below is passed a correctly sized, fully
        // initialised `cpu_set_t` that outlives the call, and pid 0 names the
        // calling thread. `unsafe` is available here for the same reason
        // `src/bin/allocations.rs` has it and `crates/` does not: this crate is
        // outside the workspace, so the root's `unsafe_code = "forbid"` — which
        // cannot be waived from inside a crate — does not reach it.
        unsafe {
            let mut previous: libc::cpu_set_t = std::mem::zeroed();
            let size = size_of::<libc::cpu_set_t>();

            if libc::sched_getaffinity(0, size, &mut previous) != 0 {
                return Self {
                    previous,
                    pinned: false,
                    cpu: -1,
                };
            }

            let cpu = libc::sched_getcpu();
            if cpu < 0 {
                return Self {
                    previous,
                    pinned: false,
                    cpu,
                };
            }

            let mut one: libc::cpu_set_t = std::mem::zeroed();
            libc::CPU_ZERO(&mut one);
            libc::CPU_SET(cpu as usize, &mut one);
            let pinned = libc::sched_setaffinity(0, size, &one) == 0;

            Self {
                previous,
                pinned,
                cpu,
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    pub(crate) fn here() -> Self {
        Self {
            pinned: false,
            cpu: -1,
        }
    }

    /// One line for the control to print, so a reader of the output knows
    /// whether the confound was removed or merely absent.
    pub(crate) fn describe(&self) -> String {
        if self.pinned {
            format!("pinned to cpu {}", self.cpu)
        } else if cfg!(target_os = "linux") {
            "NOT pinned — affinity call declined".to_owned()
        } else {
            "not pinned — affinity is Linux-only here".to_owned()
        }
    }
}

#[cfg(target_os = "linux")]
impl Drop for PinnedToOneCpu {
    fn drop(&mut self) {
        if !self.pinned {
            return;
        }
        // SAFETY: `previous` was filled by a successful `sched_getaffinity`
        // above and is still owned by this value.
        //
        // The result is deliberately ignored. This runs on the unwind path of a
        // failing assertion as well as the happy one, and a panic in `drop`
        // during a panic aborts the process — which would replace a legible test
        // failure with one that says nothing.
        unsafe {
            let size = size_of::<libc::cpu_set_t>();
            let _ = libc::sched_setaffinity(0, size, &self.previous);
        }
    }
}
