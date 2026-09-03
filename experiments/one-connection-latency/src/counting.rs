//! A counting `#[global_allocator]` that tracks **live** bytes and their peak.
//!
//! # Why live-and-peak rather than the cumulative counters next door
//!
//! `experiments/event-clone-allocations/src/counting.rs` counts *events* —
//! allocs, reallocs, bytes requested — because its question is "how many times
//! does this touch the heap". The question here is different and needs a
//! different number: **how many bytes of one page are resident at once**. A
//! cumulative byte count answers that with the sum of everything the page ever
//! allocated, including the buffers it freed on the way, which is an
//! overstatement of exactly the thing R-1 is about.
//!
//! So this one keeps a signed running total of live bytes and a high-water mark
//! over it, and a region is `(peak - live_at_entry)`.
//!
//! # What the number is, and is not
//!
//! It is the sum of `layout.size()` over the allocations live at the worst
//! instant of the region. It is **not** resident set size: the system allocator
//! rounds every request up to a size class, keeps freed pages mapped, and
//! SQLite's own page cache is `malloc`'d through this same allocator and so
//! *is* included. It is a lower bound on the memory the page costs the process,
//! and it is quoted as one.
//!
//! # Why it cannot recurse
//!
//! The counters are `AtomicI64` statics. Nothing on the counting path
//! allocates, so the allocator never re-enters itself — which is why they are
//! not a `thread_local!`, whose lazy initialisation would allocate on first
//! touch from inside the very call it is trying to count.
//!
//! # Why the counters are process-global, and what that costs
//!
//! Being global, they see every thread. That is fatal to a peak taken while a
//! second task is appending, so the harness never does that: the residency pass
//! runs with no concurrent appender and the latency pass takes no residency
//! figure. `run.sh` passes `--test-threads=1` for the same reason.
//! [`crate::probe::want_peak`] is the switch, and it is off unless the quiet
//! pass turned it on.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

/// Bytes requested and not yet returned, summed over every live allocation.
static LIVE: AtomicI64 = AtomicI64::new(0);

/// The high-water mark of [`LIVE`] since the last [`begin_region`].
static PEAK: AtomicI64 = AtomicI64::new(0);

/// Whether a residency region is open. See the module documentation.
static PEAK_ENABLED: AtomicBool = AtomicBool::new(false);

/// A pass-through allocator that tracks live bytes before delegating to
/// [`System`].
#[derive(Debug)]
pub struct Counting;

// SAFETY: every method forwards to `System`'s implementation of the same method
// with the same arguments and returns its pointer unchanged, so the memory
// contract is `System`'s. The accounting touches only `AtomicI64` statics,
// which allocate nothing and so cannot re-enter the allocator.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as i64);
        // SAFETY: `layout` is forwarded unchanged from the caller, which
        // established its validity.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        note(layout.size() as i64);
        // SAFETY: as `alloc`.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LIVE.fetch_sub(layout.size() as i64, Ordering::Relaxed);
        // SAFETY: `ptr` and `layout` are forwarded unchanged; the caller
        // established that they name a live allocation from this allocator, and
        // this allocator's allocations are `System`'s.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        note(new_size as i64 - layout.size() as i64);
        // SAFETY: as `dealloc`, plus `new_size` forwarded unchanged. Delegating
        // to `System::realloc` rather than to the `GlobalAlloc` default body is
        // deliberate: the default body calls `self.alloc`, which would count the
        // same growth twice.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Adds `delta` to the live total and raises the high-water mark.
///
/// The `fetch_max` is skipped entirely while no region is open, which is what
/// keeps the allocator's cost on the *latency* pass down to one relaxed add.
fn note(delta: i64) {
    let live = LIVE.fetch_add(delta, Ordering::Relaxed) + delta;
    if PEAK_ENABLED.load(Ordering::Relaxed) {
        PEAK.fetch_max(live, Ordering::Relaxed);
    }
}

/// Opens a residency region and returns the baseline to subtract from its peak.
///
/// # Panics
///
/// Never. A second `begin_region` before the first is closed simply restarts
/// the high-water mark, which is what nesting would want anyway.
pub fn begin_region() -> i64 {
    let live = LIVE.load(Ordering::Relaxed);
    PEAK.store(live, Ordering::Relaxed);
    PEAK_ENABLED.store(true, Ordering::Relaxed);
    live
}

/// Closes the region opened by `begin_region` and returns its peak, in bytes.
#[must_use]
pub fn end_region(baseline: i64) -> i64 {
    let peak = PEAK.load(Ordering::Relaxed);
    PEAK_ENABLED.store(false, Ordering::Relaxed);
    peak - baseline
}

/// Live bytes right now, for a sanity print.
#[must_use]
pub fn live_bytes() -> i64 {
    LIVE.load(Ordering::Relaxed)
}
