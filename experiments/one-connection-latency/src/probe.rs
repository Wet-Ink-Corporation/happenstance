//! What the reproduced page fetch reports, and the summary the harness prints.
//!
//! Three numbers per page, and they answer three different questions:
//!
//! * **wait** — how long `fetch_page`'s `connection.lock()` blocked. This is the
//!   cost the *page* pays for sharing a handle, and it is zero on a handle
//!   nobody else is using.
//! * **hold** — how long the guard was live: from the lock returning to the end
//!   of the function. This is the cost the page imposes on *everyone else*, and
//!   it is the number J-5 is about.
//! * **peak** — live bytes at the worst instant inside the hold, which is the
//!   number R-1 is about.
//!
//! Separating wait from hold matters. A single histogram of "time in
//! `fetch_page`" would fold the two together, and on a busy handle the wait can
//! be the larger half — at which point the figure says nothing about `PAGE_SIZE`
//! at all, because waiting is what the *other* page's `PAGE_SIZE` bought.
//!
//! # Global, and deliberately so
//!
//! The reproduced cursor is a value the harness does not own — it lives inside a
//! `spawn_blocking` closure, behind a `Stream`, exactly as the real one does —
//! so there is no place to thread a `&mut Histogram` through that would not
//! change the shape being measured. `run.sh` passes `--test-threads=1`, and
//! [`reset`] is called at the top of every configuration.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};

/// Per-page samples, in nanoseconds: `(wait, hold)`.
static SAMPLES: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());

/// The largest residency any page of the current configuration reached.
static PEAK_BYTES: AtomicI64 = AtomicI64::new(0);

/// How many rows the widest merge of the current configuration held at once,
/// **before** `truncate(budget)`.
///
/// This is the `ceil(arms / 400) × PAGE_SIZE` multiplier made observable rather
/// than assumed: it is the merge buffer's length at its maximum, which is what
/// the page actually pays for.
static PEAK_MERGED_ROWS: AtomicU64 = AtomicU64::new(0);

/// How many statements the widest page of the current configuration prepared.
static STATEMENTS_PER_PAGE: AtomicU64 = AtomicU64::new(0);

/// Whether the harness wants a residency region taken around each page.
static WANT_PEAK: AtomicBool = AtomicBool::new(false);

/// Clears every accumulator. Called once per configuration.
///
/// # Panics
///
/// Panics if a previous holder of the samples mutex panicked, which in this
/// crate means a page fetch panicked — and a measurement taken after that is
/// not one to salvage.
pub fn reset() {
    SAMPLES.lock().expect("probe samples poisoned").clear();
    PEAK_BYTES.store(0, Ordering::Relaxed);
    PEAK_MERGED_ROWS.store(0, Ordering::Relaxed);
    STATEMENTS_PER_PAGE.store(0, Ordering::Relaxed);
}

/// Turns the residency region on or off. See [`crate::counting`].
pub fn set_want_peak(on: bool) {
    WANT_PEAK.store(on, Ordering::Relaxed);
}

/// Whether the page fetch should open a residency region.
#[must_use]
pub fn want_peak() -> bool {
    WANT_PEAK.load(Ordering::Relaxed)
}

/// Records one page.
///
/// # Panics
///
/// As [`reset`].
pub fn record_page(wait_nanos: u64, hold_nanos: u64) {
    SAMPLES
        .lock()
        .expect("probe samples poisoned")
        .push((wait_nanos, hold_nanos));
}

/// Records one page's residency, in bytes.
pub fn record_peak_bytes(bytes: i64) {
    PEAK_BYTES.fetch_max(bytes, Ordering::Relaxed);
}

/// Records the merge buffer's length before truncation, and how many statements
/// filled it.
pub fn record_shape(merged_rows: usize, statements: usize) {
    PEAK_MERGED_ROWS.fetch_max(merged_rows as u64, Ordering::Relaxed);
    STATEMENTS_PER_PAGE.fetch_max(statements as u64, Ordering::Relaxed);
}

/// What one configuration measured.
#[derive(Debug, Clone, Copy, Default)]
pub struct PageSummary {
    /// How many pages were sampled.
    pub pages: usize,
    /// Median lock-hold, in microseconds.
    pub hold_p50_us: f64,
    /// 99th-percentile lock-hold, in microseconds.
    pub hold_p99_us: f64,
    /// Longest lock-hold, in microseconds.
    pub hold_max_us: f64,
    /// Median wait for the lock, in microseconds.
    pub wait_p50_us: f64,
    /// Longest wait for the lock, in microseconds.
    pub wait_max_us: f64,
    /// Largest residency any page reached, in bytes. Zero when the residency
    /// region was off.
    pub peak_bytes: i64,
    /// Longest merge buffer, in rows, before truncation.
    pub peak_merged_rows: u64,
    /// Most statements one page prepared.
    pub statements_per_page: u64,
}

/// Summarises everything recorded since [`reset`].
///
/// # Panics
///
/// As [`reset`].
#[must_use]
pub fn summarise() -> PageSummary {
    let samples = SAMPLES.lock().expect("probe samples poisoned").clone();
    let mut holds: Vec<u64> = samples.iter().map(|(_, hold)| *hold).collect();
    let mut waits: Vec<u64> = samples.iter().map(|(wait, _)| *wait).collect();
    holds.sort_unstable();
    waits.sort_unstable();

    PageSummary {
        pages: samples.len(),
        hold_p50_us: percentile_us(&holds, 0.50),
        hold_p99_us: percentile_us(&holds, 0.99),
        hold_max_us: percentile_us(&holds, 1.0),
        wait_p50_us: percentile_us(&waits, 0.50),
        wait_max_us: percentile_us(&waits, 1.0),
        peak_bytes: PEAK_BYTES.load(Ordering::Relaxed),
        peak_merged_rows: PEAK_MERGED_ROWS.load(Ordering::Relaxed),
        statements_per_page: STATEMENTS_PER_PAGE.load(Ordering::Relaxed),
    }
}

/// The `q`-th percentile of a **sorted** nanosecond slice, in microseconds.
///
/// Nearest-rank rather than interpolated, so every figure printed is a sample
/// that was actually taken. An empty slice is zero rather than a panic: a
/// configuration whose budget expired before one page completed is a result, and
/// it is reported as `pages = 0` beside it.
#[must_use]
pub fn percentile_us(sorted_nanos: &[u64], q: f64) -> f64 {
    if sorted_nanos.is_empty() {
        return 0.0;
    }
    // `allow` rather than `expect`: this crate's gate is rustc under the
    // repository's ambient `-D warnings`, and rustc does not evaluate a tool
    // lint's expectation, so an `expect` here would be a promise nothing checks.
    // Both casts are of a slice length and of a nanosecond count, each far below
    // 2^53.
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    let rank = ((q * sorted_nanos.len() as f64).ceil() as usize).clamp(1, sorted_nanos.len()) - 1;
    #[allow(clippy::cast_precision_loss)]
    let value = sorted_nanos[rank] as f64;
    value / 1_000.0
}
