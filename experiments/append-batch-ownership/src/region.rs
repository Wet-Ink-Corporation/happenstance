//! One measuring region at a time, per process.
//!
//! The counting allocator's counters are process-global
//! (`event-clone-allocations/src/counting.rs`), so under the default parallel
//! test harness one test's allocations land in whichever region another test
//! has open, and an exact-count assertion fails on somebody else's work. A test
//! that counts takes [`hold`] first, measures through the guard it gets back,
//! and keeps that guard to its last line, so no other counting test in the same
//! binary allocates while it runs.
//!
//! The lock holds a [`Region`], a zero-sized handle whose only method is the
//! measurement, rather than guarding `()` beside it: a test that counts through
//! [`Region`] cannot do so without holding the lock (OWN-12). Counting does not
//! have to go through it — `measured::measure` remains, for the single-threaded
//! wasm32 leg. The static is the one place the rule "global
//! counters, so one reader at a time" can live, because the counters themselves
//! are a global in another crate (ARC-09). It does not silence the test
//! harness's own threads, which still allocate when a test finishes or the next
//! one is spawned; `run.sh` passes `--test-threads=1` for the recorded run for
//! that reason. On wasm32 there is one thread and the lock is never contended.

use std::sync::{Mutex, MutexGuard, PoisonError};

use event_clone_allocations::counting::{self, Counts};

/// The right to read the process-global counters. Only [`hold`] hands one out.
#[derive(Debug)]
pub struct Region(());

impl Region {
    /// Runs `f` and counts what it allocated, as `counting::measure` does.
    pub fn measure<T>(&mut self, f: impl FnOnce() -> T) -> (T, Counts) {
        counting::measure(f)
    }
}

/// The one region.
static REGION: Mutex<Region> = Mutex::new(Region(()));

/// Takes the region, waiting for any test that holds it.
///
/// A poisoned lock is taken anyway. Poisoning means another test panicked while
/// holding it, and that panic is already that test's reported failure; a
/// [`Region`] has no state to leave half-written, and refusing would fail every
/// later test for a reason that is not its own.
pub fn hold() -> MutexGuard<'static, Region> {
    REGION.lock().unwrap_or_else(PoisonError::into_inner)
}
