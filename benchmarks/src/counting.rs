//! The allocation instrument: one counting allocator answering both questions.
//!
//! # Why one allocator and not the two this repository already has
//!
//! `experiments/event-clone-allocations/src/counting.rs` counts *events* —
//! allocs, reallocs, bytes requested — because its question is "how many times
//! does this touch the heap".
//! `experiments/one-connection-latency/src/counting.rs` tracks *live* bytes and
//! their peak, because its question is "how much is resident at once". A
//! benchmark suite asks both, of the same scenario, in the same run, and a
//! `#[global_allocator]` is a per-binary singleton — so there can only be one.
//! This is those two merged, with each half kept separable at the read side:
//! [`Counts`] is the cumulative half, [`Region`] the residency half.
//!
//! # Why nothing here reads a clock
//!
//! Because the counts are the reproducible half. On the host in `README.md` the
//! allocation columns were identical to the digit across four separate runs
//! while wall-clock medians moved by up to 40% between them, which is why
//! `references/evaluation/review-pre-publication-2026-09-03.md:2836` says to
//! *"quote the allocation columns and not the timings"*. Timing lives in
//! `criterion` and in [`crate::paired`]; this module is deliberately mute about it.
//!
//! # Why it cannot recurse
//!
//! Every counter is an atomic static. Nothing on the counting path allocates,
//! so the allocator never re-enters itself — which is why they are not
//! `thread_local!`s, whose lazy initialisation would allocate on first touch
//! from inside the very call it is trying to count.
//!
//! They are process-global rather than per-thread, so a background thread's
//! allocation lands in whatever region happens to be open. Every binary that
//! installs this allocator runs its arms with `--test-threads=1` and allocates
//! nothing off the measuring thread; `run.sh` passes the flag and says why.
//!
//! # Where it is installed, and where it is not
//!
//! **Not here, and not in `lib.rs`.** Both experiments declare theirs in their
//! `lib.rs`, which installs the allocator into every binary that links the
//! crate — here that would be every `criterion` target, whose timings would
//! then carry an atomic increment on every heap operation in the measured
//! region, and whose figures would then be about the instrument.
//!
//! So the type is public and the attribute is not. A binary opts in with one
//! line, and exactly two do: `src/bin/allocations.rs`, which is the reporting
//! instrument, and `tests/instruments_work.rs`, which is the control proving
//! this file counts what it says it counts. Every other target in the crate —
//! all seven `criterion` benches, the paired runner, the collector — links the
//! type and never installs it.

use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};

/// Calls to `alloc` and `alloc_zeroed`.
static ALLOCS: AtomicU64 = AtomicU64::new(0);
/// Calls to `realloc` — a buffer that grew in place or moved.
static REALLOCS: AtomicU64 = AtomicU64::new(0);
/// Calls to `dealloc`.
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
/// Bytes requested, cumulative and never decremented.
static BYTES: AtomicU64 = AtomicU64::new(0);

/// Bytes allocated and not yet freed.
///
/// Signed because the process is already some way through its life when the
/// allocator is installed on the first `alloc` it sees, so an early `dealloc`
/// of memory this allocator never handed out drives it below zero. A `u64`
/// would wrap and report about eighteen exabytes resident.
static LIVE: AtomicI64 = AtomicI64::new(0);
/// The high-water mark of [`LIVE`] since the open region began.
static PEAK: AtomicI64 = AtomicI64::new(0);
/// Whether a region is open. The `fetch_max` on the hot path is skipped
/// entirely while it is not, which keeps the cost of the instrument off every
/// allocation the suite is not asking about.
static PEAK_ENABLED: AtomicBool = AtomicBool::new(false);

/// A pass-through allocator that counts before delegating to [`System`].
#[derive(Debug)]
pub struct Counting;

// SAFETY: every method forwards to `System`'s implementation of the same method
// with the same arguments and returns its pointer unchanged, so the memory
// contract is `System`'s throughout. The counting itself touches only atomic
// statics, which allocate nothing and so cannot re-enter the allocator.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        add_live(as_delta(layout.size()));
        // SAFETY: `layout` is forwarded unchanged from the caller, which
        // established its validity.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        add_live(as_delta(layout.size()));
        // SAFETY: as `alloc`.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCS.fetch_add(1, Ordering::Relaxed);
        // A free never raises the peak, so this side skips `fetch_max`
        // unconditionally rather than going through `add_live`.
        LIVE.fetch_sub(as_delta(layout.size()), Ordering::Relaxed);
        // SAFETY: `ptr` and `layout` are forwarded unchanged; the caller
        // established that they name a live allocation from this allocator, and
        // this allocator's allocations are `System`'s.
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        REALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(
            new_size.saturating_sub(layout.size()) as u64,
            Ordering::Relaxed,
        );
        add_live(as_delta(new_size) - as_delta(layout.size()));
        // SAFETY: as `dealloc`, plus `new_size` forwarded unchanged. Delegating
        // to `System::realloc` rather than to the `GlobalAlloc` default body is
        // deliberate: the default body calls `self.alloc`, which would count the
        // same growth twice.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// A layout size as a signed delta.
///
/// The cast cannot wrap. Rust's own allocation contract bounds `Layout::size()`
/// at `isize::MAX`, so the value fits an `i64` exactly on every target this
/// crate builds for. A `try_from` here would put a branch on the allocator's
/// hot path to re-check something the language already guarantees — and this
/// function is called on every single heap operation in the process.
#[allow(clippy::cast_possible_wrap)]
const fn as_delta(size: usize) -> i64 {
    size as i64
}

/// Moves [`LIVE`] and raises [`PEAK`] if a region is open.
fn add_live(delta: i64) {
    let live = LIVE.fetch_add(delta, Ordering::Relaxed) + delta;
    if PEAK_ENABLED.load(Ordering::Relaxed) {
        PEAK.fetch_max(live, Ordering::Relaxed);
    }
}

/// Four cumulative counters read at one instant, or the difference between two
/// such reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Calls to `alloc` and `alloc_zeroed`.
    pub allocs: u64,
    /// Calls to `realloc` — a buffer that grew in place or moved.
    pub reallocs: u64,
    /// Calls to `dealloc`.
    pub deallocs: u64,
    /// Bytes *requested*: `layout.size()` on each fresh allocation and the
    /// growth delta on each `realloc`. **Not** bytes resident, and not the size
    /// the system allocator actually reserved. A lower bound on resident
    /// growth, quoted as one.
    pub bytes: u64,
}

impl Counts {
    /// Fresh allocations plus reallocations — the number a reader means by "how
    /// many times did this touch the heap".
    ///
    /// The two are counted apart rather than summed at the source because
    /// `Box<[Tag]>::clone` and `Cow::Owned(String)::clone` both take the
    /// `alloc` path while a `Vec` that grows takes `realloc`, and a measurement
    /// that folds them together cannot tell "cloned sixty-four strings" from
    /// "grew one buffer six times".
    pub const fn heap_ops(&self) -> u64 {
        self.allocs + self.reallocs
    }
}

impl std::ops::Sub for Counts {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            allocs: self.allocs - rhs.allocs,
            reallocs: self.reallocs - rhs.reallocs,
            deallocs: self.deallocs - rhs.deallocs,
            bytes: self.bytes - rhs.bytes,
        }
    }
}

impl fmt::Display for Counts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "allocs={} reallocs={} heap_ops={} bytes={} deallocs={}",
            self.allocs,
            self.reallocs,
            self.heap_ops(),
            self.bytes,
            self.deallocs
        )
    }
}

/// What one measured region cost, cumulative and resident together.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Region {
    /// Heap operations and bytes requested across the region.
    pub counts: Counts,
    /// Bytes live at the moment the region opened.
    pub live_before: i64,
    /// The highest [`LIVE`] reached while the region was open, **relative to
    /// `live_before`**. This is the number that answers "how much did this need
    /// resident at once".
    pub peak_above_baseline: i64,
    /// Bytes still live when the region closed, relative to `live_before`. A
    /// scenario that returns its result leaves this positive by the size of
    /// that result; one that leaks leaves it positive by more.
    pub retained: i64,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} peak={} retained={}",
            self.counts, self.peak_above_baseline, self.retained
        )
    }
}

/// Reads the four cumulative counters.
pub fn snapshot() -> Counts {
    Counts {
        allocs: ALLOCS.load(Ordering::Relaxed),
        reallocs: REALLOCS.load(Ordering::Relaxed),
        deallocs: DEALLOCS.load(Ordering::Relaxed),
        bytes: BYTES.load(Ordering::Relaxed),
    }
}

/// Bytes allocated and not yet freed, right now.
pub fn live_bytes() -> i64 {
    LIVE.load(Ordering::Relaxed)
}

/// Holds an open region and closes it even if `f` unwinds.
///
/// Not a convenience. Without it, a panic inside the measured closure leaves
/// [`PEAK_ENABLED`] set, and **every later `measure` in the same process then
/// panics with "regions do not nest"** — so one failing scenario cascades into
/// every scenario after it and the run reports a nesting bug that is not there.
/// `tests/instruments_work.rs` found exactly that: the `#[should_panic]` control
/// for nesting poisoned four unrelated tests in the same binary.
struct OpenRegion;

impl Drop for OpenRegion {
    fn drop(&mut self) {
        PEAK_ENABLED.store(false, Ordering::SeqCst);
    }
}

/// Runs `f` and returns what it produced alongside what it cost.
///
/// The value is returned rather than dropped inside the region, so the figures
/// are the cost of *building* it and never of tearing it down.
/// [`std::hint::black_box`] is on both sides so an arm whose result is unused
/// cannot be optimised out between the two reads — which is exactly what would
/// happen to `let _ = event.clone();` at `opt-level = 3`, and would report a
/// clone as free.
///
/// # Panics
///
/// Panics if a region is already open. Regions do not nest: the peak is one
/// pair of statics, so an inner region would reset the outer one's high-water
/// mark and the outer would then report the inner's peak as its own. Two
/// figures that disagree silently are worse than one call that stops.
pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Region) {
    assert!(
        !PEAK_ENABLED.swap(true, Ordering::SeqCst),
        "measurement regions do not nest: the peak is one pair of statics, so          an inner region would reset the outer one's high-water mark and the          outer would then report the inner's peak as its own"
    );
    let _open = OpenRegion;

    let live_before = LIVE.load(Ordering::Relaxed);
    PEAK.store(live_before, Ordering::Relaxed);

    // `black_box` on the first read as well as on the value: it forces the
    // counters to be materialised *before* `f` runs, so a relaxed load cannot
    // sink past the call it is meant to bracket.
    let before = std::hint::black_box(snapshot());
    let value = std::hint::black_box(f());
    let after = std::hint::black_box(snapshot());

    let peak = PEAK.load(Ordering::Relaxed);
    let live_after = LIVE.load(Ordering::Relaxed);

    let region = Region {
        counts: after - before,
        live_before,
        peak_above_baseline: peak - live_before,
        retained: live_after - live_before,
    };
    (value, region)
}
