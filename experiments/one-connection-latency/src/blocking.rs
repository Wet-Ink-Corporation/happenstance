//! Driving a future to completion on a bare OS thread, with no runtime.
//!
//! # Why this exists rather than a nested runtime
//!
//! Two arms of instrument (a) need an `append` **in flight on another thread**
//! while the reactor tries to do something else, so that the thing being
//! measured is contention for the connection `Mutex` rather than for SQLite's
//! file write lock. The appending thread must not be a tokio task: on a
//! `current_thread` runtime a task cannot run while the reactor is stalled,
//! which is the very condition under measurement.
//!
//! A nested `current_thread` runtime on that OS thread would work and would
//! bring its own timer, its own blocking pool and its own `Handle::try_current`,
//! all of which are shape the measurement does not want. `Waker::noop` plus a
//! bounded spin is smaller and says exactly what is true of the futures it
//! drives: **they contain no `await`**, so they complete on the first poll.
//!
//! That is not an assumption. `SqliteEventStore::append`
//! (`crates/happenstance-sqlite/src/event_store.rs:1018-1053`) is an `async fn`
//! whose body has no `.await` in it at all — which is precisely the defect J-2
//! reports — so one poll returns `Ready`. [`poll_to_completion`] therefore
//! refuses to spin more than a bounded number of times, and a future that did
//! yield would fail the run loudly instead of hanging it (NF-003: it terminates
//! unattended, and there is no watchdog anywhere to rescue it).

use std::future::Future;
use std::task::{Context, Poll, Waker};

/// How many polls a future gets before this gives up on it.
const POLL_BUDGET: usize = 1_024;

/// Drives `future` to completion on the calling thread.
///
/// # Panics
///
/// Panics if `future` returns `Pending` more than [`POLL_BUDGET`] times. That
/// means it awaited something, which no future this is used on does — and a
/// silent spin would turn a wrong assumption into a hung run naming no rule.
pub fn poll_to_completion<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..POLL_BUDGET {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
        std::thread::yield_now();
    }
    panic!(
        "a future this crate drives on a bare thread returned Pending {POLL_BUDGET} times; \
         it awaits something, and the arm driving it is measuring the wrong thing"
    );
}
