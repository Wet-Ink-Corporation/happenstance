//! The instrument: a counting `#[global_allocator]` and the region timer that
//! reads it.
//!
//! # Why it counts four things and not one
//!
//! "Allocations" is not one number. `Box<[Tag]>::clone` and
//! `Cow::Owned(String)::clone` both take the `alloc` path, but a `Vec` that
//! grows takes `realloc`, and a measurement that folds the two together cannot
//! tell "cloned sixty-four strings" from "grew one buffer six times". Both
//! matter here and they answer different questions, so both are reported and
//! `heap_ops` is the sum, named rather than assumed.
//!
//! `bytes` is the requested size, not the size the system allocator actually
//! reserved. It is a lower bound on resident growth and is quoted as one.
//!
//! # Why it cannot recurse
//!
//! The counters are `AtomicU64` statics. Nothing on the counting path allocates,
//! so the allocator never re-enters itself — which is why they are not a
//! `thread_local!`, whose lazy initialisation would allocate on first touch from
//! inside the very call it is trying to count.
//!
//! The counters are process-global rather than per-thread, so a background
//! thread's allocation lands in whatever region happens to be open. `run.sh`
//! passes `--test-threads=1` for that reason, and the arms allocate nothing off
//! the measuring thread.

use std::alloc::{GlobalAlloc, Layout, System};
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static ALLOCS: AtomicU64 = AtomicU64::new(0);
static REALLOCS: AtomicU64 = AtomicU64::new(0);
static DEALLOCS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);

/// A pass-through allocator that counts before delegating to [`System`].
pub struct Counting;

// SAFETY: every method forwards to `System`'s implementation of the same method
// with the same arguments and returns its pointer unchanged, so the memory
// contract is `System`'s. The counting itself touches only `AtomicU64` statics,
// which allocate nothing and so cannot re-enter the allocator.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        // SAFETY: `layout` is forwarded unchanged from the caller, which
        // established its validity.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOCS.fetch_add(1, Ordering::Relaxed);
        BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        // SAFETY: as `alloc`.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DEALLOCS.fetch_add(1, Ordering::Relaxed);
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
        // SAFETY: as `dealloc`, plus `new_size` forwarded unchanged. Delegating
        // to `System::realloc` rather than to the `GlobalAlloc` default body is
        // deliberate: the default body calls `self.alloc`, which would count the
        // same growth twice.
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

/// Four counters read at one instant, or the difference between two such reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counts {
    /// Calls to `alloc` and `alloc_zeroed`.
    pub allocs: u64,
    /// Calls to `realloc` — a buffer that grew in place or moved.
    pub reallocs: u64,
    /// Calls to `dealloc`.
    pub deallocs: u64,
    /// Bytes *requested*: `layout.size()` on each fresh allocation and the
    /// growth delta on each `realloc`. Not bytes resident.
    pub bytes: u64,
}

impl Counts {
    /// Fresh allocations plus reallocations — the number a reader means by
    /// "how many times did this touch the heap".
    #[must_use]
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
            "allocs={:<10} reallocs={:<6} heap_ops={:<10} bytes={:<12} deallocs={}",
            self.allocs,
            self.reallocs,
            self.heap_ops(),
            self.bytes,
            self.deallocs
        )
    }
}

/// Reads the four counters.
#[must_use]
pub fn snapshot() -> Counts {
    Counts {
        allocs: ALLOCS.load(Ordering::Relaxed),
        reallocs: REALLOCS.load(Ordering::Relaxed),
        deallocs: DEALLOCS.load(Ordering::Relaxed),
        bytes: BYTES.load(Ordering::Relaxed),
    }
}

/// Runs `f` and returns what it produced alongside what it cost.
///
/// The value is returned rather than dropped inside the region, so the counts
/// are the cost of *building* it and never of tearing it down. `black_box` is on
/// both sides so that an arm whose result is unused cannot be optimised out
/// between the two reads — which is exactly what would happen to
/// `let _ = event.clone();` at `opt-level = 3`.
pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Counts) {
    // `black_box` on the first read as well as on the value: it forces the
    // counters to be materialised *before* `f` runs, so a relaxed load cannot
    // sink past the call it is meant to bracket.
    let before = std::hint::black_box(snapshot());
    let value = std::hint::black_box(f());
    let after = std::hint::black_box(snapshot());
    (value, after - before)
}
