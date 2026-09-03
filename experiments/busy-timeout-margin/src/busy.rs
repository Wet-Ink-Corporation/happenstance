//! A counting replacement for SQLite's default busy handler.
//!
//! # Why this file exists at all
//!
//! `references/adr/0022-append-condition-strategy.md:615-616` names its own
//! falsifier: *"Re-open the busy timeout if any run ever reports `busy > 0`,
//! which would mean five seconds stopped being generous."* Nothing anywhere in
//! the tree reports a nonzero busy count, and — more to the point — nothing
//! reports a busy count that is *approaching* the cap either. `busy = 0` is a
//! one-bit answer to a continuous question: it says the unluckiest contender
//! waited less than 5,000 ms, and it cannot distinguish 40 ms from 4,999 ms.
//! Every margin claim in the corpus is therefore inferred from *race wall time*,
//! which is an upper bound on one contender's wait rather than a measurement of
//! it.
//!
//! This handler measures the thing directly: per contender, per lock event, the
//! accumulated milliseconds spent asleep inside the busy handler, and the
//! maximum of that over every lock event the contender lived through.
//!
//! # It reproduces SQLite's own schedule rather than inventing one
//!
//! `sqlite3_busy_timeout` installs `sqliteDefaultBusyCallback`, and replacing it
//! with a handler that backs off differently would measure a different adapter.
//! [`DELAYS`] and [`TOTALS`] are that function's own tables, and
//! [`counting_busy_handler`] is its own control flow, transcribed. The one thing
//! that is added is the accounting.
//!
//! It matters that this is a **poll**, not a queue. SQLite grants the write lock
//! to whichever contender happens to retry at the moment the previous one
//! releases it; there is no FIFO ordering and no fairness. So the unluckiest
//! contender's wait is not `63 × T_commit` — it is the tail of a distribution,
//! which is why this crate reports a maximum and a histogram and never a mean.
//!
//! # Why the state is thread-local
//!
//! `rusqlite::Connection::busy_handler` takes `Option<fn(i32) -> bool>` — a bare
//! function pointer, not a closure, because rusqlite `transmute`s the pointer
//! into SQLite's `void*` argument slot. There is nowhere to hang per-connection
//! state. A thread local is exact here rather than a workaround: the handler
//! runs on the thread that is blocked, one contender is one OS thread with one
//! connection for its whole life, and [`take`] is called on that thread before
//! it exits.
//!
//! # What replacing the handler costs, and how it is disclosed
//!
//! `sqlite3_busy_handler()` sets `db->busyTimeout = 0` as a side effect, so
//! `PRAGMA busy_timeout` reads back **0** on a connection carrying this handler
//! even though the effective cap is unchanged at [`BUSY_TIMEOUT_MS`]. That is a
//! true reading of a misleading number, so [`crate::Durability::conditions`] is
//! never printed alone in this crate: every row prints
//! `busy_handler=counting cap_ms=5000` beside it. Reading a pragma back off the
//! live connection is this experiment family's rule and it is kept; what is
//! added is the note that the pragma is no longer where the answer lives.

use std::cell::Cell;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

/// Which busy handler the connections in this process carry.
///
/// # Why this switch exists
///
/// Installing a handler to measure a handler is a measurement that can perturb
/// the thing measured, and here it plainly might: SQLite's own handler sleeps
/// through `sqlite3_win32_sleep` → `Sleep()`, while this one sleeps through
/// `std::thread::sleep`, and the two do not have to have the same timer
/// resolution. A tighter sleep is a tighter poll, and a tighter poll changes how
/// a sixty-four-way thundering herd resolves.
///
/// So the experiment carries the control for its own instrument.
/// `HS_HANDLER=default` runs the identical harness with
/// `Connection::busy_timeout` — SQLite's handler, no accounting, exactly what
/// `experiments/append-condition` measured under — and `run.sh` runs the release
/// control both ways. The difference between those two rows is what the
/// instrument costs, stated in `results/` rather than assumed away.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Handler {
    /// This module's counting transcription of SQLite's schedule.
    Counting,
    /// SQLite's own, installed by `Connection::busy_timeout`. Reports nothing.
    SqliteDefault,
}

impl Handler {
    /// How the choice names itself in a results row.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counting => "counting",
            Self::SqliteDefault => "sqlite-default",
        }
    }
}

/// Which handler this process installs, read once from `HS_HANDLER`.
///
/// Counting unless `HS_HANDLER=default`: the accounting is the point, so the
/// unmeasured arm has to be asked for by name.
#[must_use]
pub fn handler() -> Handler {
    static CHOICE: OnceLock<Handler> = OnceLock::new();
    *CHOICE.get_or_init(|| match std::env::var("HS_HANDLER").as_deref() {
        Ok("default") => Handler::SqliteDefault,
        _ => Handler::Counting,
    })
}

/// The cap this handler enforces, in milliseconds.
///
/// Deliberately the same literal as
/// `crates/happenstance-sqlite/src/connection.rs:62`'s `BUSY_TIMEOUT_MS`, and
/// the same value `experiments/append-condition` measured under. The whole
/// question is whether *this number* is enough, so the experiment may not quietly
/// run under a different one.
pub const BUSY_TIMEOUT_MS: u64 = 5_000;

/// `sqliteDefaultBusyCallback`'s delay table, in milliseconds.
const DELAYS: [u64; 12] = [1, 2, 5, 10, 15, 20, 25, 25, 25, 50, 50, 100];

/// `sqliteDefaultBusyCallback`'s prefix sums of [`DELAYS`], in milliseconds.
///
/// Carried rather than computed because it is carried in SQLite: `totals[n]` is
/// the wall time already spent when the handler is entered for the `n`th time,
/// and the cap is applied against *that* rather than against a clock. Deriving
/// it here would be a second implementation of the same table with a second
/// chance to be wrong.
const TOTALS: [u64; 12] = [0, 1, 3, 8, 18, 33, 53, 78, 103, 128, 178, 228];

/// What one contender's connection did inside the busy handler.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BusyStats {
    /// The largest accumulated wait, in milliseconds, over every lock event.
    ///
    /// **This is the number the experiment exists to produce.** One *lock
    /// event* is one contended acquisition — SQLite restarts the invocation
    /// count at zero for each — so a contender that waited 300 ms three times
    /// reports 300, not 900. That is the right grain: the cap is applied per
    /// lock event, so it is a per-lock-event wait that exhausts it.
    pub max_wait_ms: u64,
    /// Every millisecond this thread spent asleep in the handler, summed.
    pub total_wait_ms: u64,
    /// How many times the handler was entered — i.e. how many `SQLITE_BUSY`
    /// retries this contender absorbed.
    pub retries: u64,
    /// The same maximum, but of **measured** sleep rather than scheduled sleep.
    ///
    /// SQLite applies its cap against [`TOTALS`], a table, not against a clock —
    /// so a `sleep(1)` that the operating system serves in 15.6 ms costs the
    /// contender 15.6 ms of real waiting and one millisecond of budget. The two
    /// numbers answer different questions: [`max_wait_ms`](Self::max_wait_ms) is
    /// how much of the 5,000 ms *budget* the unluckiest contender burned, and
    /// this is how long it actually stood still. Reporting only the first would
    /// understate the wall clock; reporting only the second would compare a
    /// measurement against a budget it is not measured in.
    pub max_real_wait_ms: u64,
    /// How many distinct contended acquisitions occurred.
    pub lock_events: u64,
    /// How many times the handler gave up and let `SQLITE_BUSY` reach the
    /// caller.
    ///
    /// Nonzero is the falsifier firing: it is exactly ADR-0022's `busy > 0`.
    pub exhausted: u64,
}

impl BusyStats {
    /// Folds another contender's row into this one.
    ///
    /// `max_wait_ms` maxes rather than sums, for the reason the field's own
    /// documentation gives — the distribution's tail is the measurement, and a
    /// sum of tails is not a tail.
    pub fn merge(&mut self, other: Self) {
        self.max_wait_ms = self.max_wait_ms.max(other.max_wait_ms);
        self.max_real_wait_ms = self.max_real_wait_ms.max(other.max_real_wait_ms);
        self.total_wait_ms += other.total_wait_ms;
        self.retries += other.retries;
        self.lock_events += other.lock_events;
        self.exhausted += other.exhausted;
    }
}

thread_local! {
    /// This thread's accumulation. See the module docs for why thread-local is
    /// the exact scope rather than a compromise.
    static STATS: Cell<BusyStats> = const { Cell::new(BusyStats {
        max_wait_ms: 0,
        total_wait_ms: 0,
        max_real_wait_ms: 0,
        retries: 0,
        lock_events: 0,
        exhausted: 0,
    }) };
    /// The budget accumulated so far within the current lock event.
    static CURRENT: Cell<u64> = const { Cell::new(0) };
    /// The **measured** wait accumulated so far within the current lock event.
    static CURRENT_REAL: Cell<u64> = const { Cell::new(0) };
}

/// Clears this thread's accumulation and returns what it held.
///
/// Called on the contender thread immediately before its timed append and again
/// immediately after, so the row a contender reports covers its own attempt and
/// nothing else — seeding, migration and `head()` are all outside it.
pub fn take() -> BusyStats {
    CURRENT.with(|current| current.set(0));
    CURRENT_REAL.with(|current| current.set(0));
    STATS.with(Cell::take)
}

/// SQLite's `sqliteDefaultBusyCallback`, with the accounting this experiment
/// needs bolted on.
///
/// Returning `true` asks SQLite to retry the lock; returning `false` makes it
/// hand `SQLITE_BUSY` to the caller, which is where
/// `crates/happenstance-testkit/src/concurrency.rs:263`'s `Attempt::Failed`
/// comes from and therefore where a red rule that is not about the adapter's
/// logic comes from.
///
/// # Panics
///
/// Never. `count` arrives from SQLite as a non-negative invocation counter and a
/// negative value would be a driver bug; it is clamped rather than asserted on,
/// because a panic unwinding into C is worse than a slightly wrong sleep.
pub fn counting_busy_handler(count: i32) -> bool {
    let count = usize::try_from(count).unwrap_or(0);

    // A fresh locking event: SQLite restarts the counter at zero for each one.
    if count == 0 {
        CURRENT.with(|current| current.set(0));
        CURRENT_REAL.with(|current| current.set(0));
        STATS.with(|stats| {
            let mut held = stats.get();
            held.lock_events += 1;
            stats.set(held);
        });
    }

    let last = DELAYS.len() - 1;
    let (delay, prior) = if count < DELAYS.len() {
        (DELAYS[count], TOTALS[count])
    } else {
        (
            DELAYS[last],
            TOTALS[last] + DELAYS[last] * (count - last) as u64,
        )
    };

    // SQLite's own cap arithmetic: the remaining budget is measured against the
    // schedule's prefix sum, not against a clock, so a handler that oversleeps
    // does not get to overshoot the cap on the *next* entry.
    let delay = delay.min(BUSY_TIMEOUT_MS.saturating_sub(prior));
    if delay == 0 {
        // The wait recorded here is the *measured* accumulation, not `prior`.
        // The schedule's prefix sum overshoots the cap on its final entry — at
        // 5,000 ms it reads 5,028 — while the sleeps actually taken sum to
        // exactly the cap, because the previous entry was truncated to the
        // remaining budget. Reporting `prior` would print a wait longer than
        // the timeout that produced it.
        let accumulated = CURRENT.with(Cell::get);
        let real = CURRENT_REAL.with(Cell::get);
        STATS.with(|stats| {
            let mut held = stats.get();
            held.exhausted += 1;
            held.max_wait_ms = held.max_wait_ms.max(accumulated);
            held.max_real_wait_ms = held.max_real_wait_ms.max(real);
            stats.set(held);
        });
        return false;
    }

    // Measured around the sleep, not inferred from the argument to it. Windows'
    // default timer resolution is 15.625 ms, so `sleep(1)` is very often not one
    // millisecond, and the first eleven entries of the schedule ask for less
    // than a tick.
    let began = Instant::now();
    std::thread::sleep(Duration::from_millis(delay));
    let slept = u64::try_from(began.elapsed().as_millis()).unwrap_or(u64::MAX);

    let accumulated = CURRENT.with(|current| {
        let now = current.get() + delay;
        current.set(now);
        now
    });
    let real = CURRENT_REAL.with(|current| {
        let now = current.get() + slept;
        current.set(now);
        now
    });
    STATS.with(|stats| {
        let mut held = stats.get();
        held.retries += 1;
        held.total_wait_ms += delay;
        held.max_wait_ms = held.max_wait_ms.max(accumulated);
        held.max_real_wait_ms = held.max_real_wait_ms.max(real);
        stats.set(held);
    });
    true
}

#[cfg(test)]
mod tests {
    use super::{BUSY_TIMEOUT_MS, DELAYS, TOTALS};

    /// The two tables have to agree, or the cap is applied against a prefix sum
    /// of a schedule nobody is running.
    #[test]
    fn totals_is_the_prefix_sum_of_delays() {
        let mut running = 0;
        for (index, total) in TOTALS.iter().enumerate() {
            assert_eq!(*total, running, "totals[{index}]");
            running += DELAYS[index];
        }
    }

    /// The schedule must reach the cap in a bounded number of entries, or
    /// `run.sh` cannot promise to terminate unattended.
    #[test]
    fn the_schedule_reaches_the_cap() {
        let last = DELAYS.len() - 1;
        let mut count = DELAYS.len();
        loop {
            let prior = TOTALS[last] + DELAYS[last] * (count - last) as u64;
            if prior >= BUSY_TIMEOUT_MS {
                break;
            }
            count += 1;
            assert!(count < 1_000, "the back-off schedule never reaches the cap");
        }
        // 228 ms of table, then 100 ms a step: entry 59 is the first whose
        // prefix sum (5,028 ms) is past the cap, so entry 58 sleeps a truncated
        // 72 ms and entry 59 gives up. Sixty entries, about five seconds.
        assert_eq!(count, 59);
    }
}
