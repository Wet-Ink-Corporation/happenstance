//! M-1's probe: does a `block_on` nested inside a `block_on` swallow the outer
//! future's wakeup?
//!
//! # Why it is in this crate and not beside `block_on`
//!
//! It belongs here because of what it shares with the rest of the directory:
//! **from outside, a hang and a timeout exhaustion are the same event.** Both
//! end as a CI job that stopped, naming no rule. CF-33 is `[FROZEN]` and forbids
//! the conformance suite from carrying the watchdog that could tell them apart
//! (`crates/happenstance-testkit/src/concurrency.rs:36-44`), and
//! `crates/happenstance-sqlite/tests/concurrency.rs:41-49` restates the
//! prohibition in terms. So an adapter author who sees the job stop has two
//! candidate causes and no instrument. This crate measures one of them; leaving
//! the other unmeasured in the same session would let either finding be answered
//! with "that was probably the other one".
//!
//! An experiment may carry the watchdog the suite may not: nothing here is a
//! conformance rule, `run.sh` is not a gate step (CF-34), and NF-003 requires
//! this directory to terminate unattended. So the probe runs on a thread that is
//! joined through a channel with a deadline, and a hang is *reported* rather than
//! waited on.
//!
//! # The mechanism, and why it is not hypothetical
//!
//! `crates/happenstance-testkit/src/registry.rs:330-344` is a park loop:
//!
//! ```text
//! Poll::Pending => std::thread::park(),
//! ```
//!
//! and `ParkWaker::wake` (`:302-308`) is a bare `self.0.unpark()`. There is no
//! notified flag. `park`/`unpark` carries **one** token, and it is per *thread*,
//! not per `block_on` — so any park on that thread will consume it, including a
//! park belonging to a different, inner `block_on`.
//!
//! That nesting is not invented for this file. The concurrency family does it:
//! `concurrency.rs:890` calls `incomplete_batches` on the rule's own thread and
//! `incomplete_batches` opens a second `crate::block_on` at `:980`, while the
//! rule itself is being driven by `crate::block_on` from
//! `__emit_concurrency_blocking` at `:1130`.
//!
//! # The three cases, and the third is what makes it a probe
//!
//! 1. [`a_plain_block_on_is_woken_by_another_thread`] — the baseline. If this
//!    fails, nothing below means anything.
//! 2. [`nesting_alone_is_harmless`] — a nested `block_on` with no concurrent
//!    outer wake completes. This is what stops the probe from indicting nesting
//!    as such.
//! 3. [`an_outer_wake_delivered_during_a_nested_block_on`] — the hazard, with the
//!    wake timed to land while the inner loop is parked. A named wrong
//!    implementation, arranged deliberately, which is the bar `CLAUDE.md` sets
//!    for an instrument: *a rule that no adapter can fail is decorative*.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use happenstance_testkit::block_on;

/// How long the watchdog waits before calling it a hang.
///
/// Generously above every delay the probes schedule (the longest is 200 ms) and
/// far enough below any plausible CI job timeout that `run.sh` still terminates
/// on its own.
const DEADLINE: Duration = Duration::from_secs(10);

/// Runs `body` on its own thread and reports whether it finished.
///
/// The thread is deliberately **not** joined on the failure path. A thread parked
/// with no token cannot be woken, so joining it is the hang this function exists
/// to report; it is left parked and the process reclaims it at exit.
fn completes_within(deadline: Duration, body: impl FnOnce() + Send + 'static) -> bool {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        body();
        // A closed channel is the normal case when the deadline already passed,
        // so the send's failure is not an error here.
        let _ = sender.send(());
    });
    receiver.recv_timeout(deadline).is_ok()
}

/// A future that is `Pending` until a helper thread flips its flag.
///
/// The waker is handed to the helper on the first poll, which is the ordinary
/// contract and the only way the hazard can arise at all: a future that never
/// escapes its waker can never have that waker fire at an awkward moment.
struct WokenByThread {
    done: Arc<AtomicBool>,
    delay: Duration,
    armed: bool,
}

impl WokenByThread {
    fn new(delay: Duration) -> Self {
        Self {
            done: Arc::new(AtomicBool::new(false)),
            delay,
            armed: false,
        }
    }
}

impl Future for WokenByThread {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
        if self.done.load(Ordering::Acquire) {
            return Poll::Ready(());
        }
        if !self.armed {
            self.armed = true;
            let waker = context.waker().clone();
            let done = Arc::clone(&self.done);
            let delay = self.delay;
            std::thread::spawn(move || {
                std::thread::sleep(delay);
                done.store(true, Ordering::Release);
                waker.wake();
            });
        }
        Poll::Pending
    }
}

/// The outer future: it hands its waker out, then runs a nested `block_on` on
/// the same thread.
///
/// `nested` is what separates case 2 from case 3, and `outer_wake_after` is what
/// times the collision: the outer wake is scheduled to land *while* the inner
/// loop is parked, which is the only ordering in which the token can be stolen.
struct NestingOuter {
    outer_wake_after: Duration,
    inner_ready_after: Option<Duration>,
    armed: bool,
    done: Arc<AtomicBool>,
}

impl Future for NestingOuter {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
        if self.done.load(Ordering::Acquire) {
            return Poll::Ready(());
        }
        if !self.armed {
            self.armed = true;

            let waker: Waker = context.waker().clone();
            let done = Arc::clone(&self.done);
            let after = self.outer_wake_after;
            std::thread::spawn(move || {
                std::thread::sleep(after);
                done.store(true, Ordering::Release);
                waker.wake();
            });

            // The nesting. On this same OS thread, and therefore sharing this
            // thread's single park token with the loop that is driving *us*.
            if let Some(delay) = self.inner_ready_after {
                block_on(WokenByThread::new(delay));
            }
        }
        Poll::Pending
    }
}

/// Baseline: `block_on` is woken by another thread at all.
#[test]
fn a_plain_block_on_is_woken_by_another_thread() {
    let completed = completes_within(DEADLINE, || {
        block_on(WokenByThread::new(Duration::from_millis(20)));
    });
    println!("WAKEUP\tcase=plain\tcompleted={completed}");
    assert!(
        completed,
        "the baseline must complete, or nothing below is evidence about nesting"
    );
}

/// Nesting on its own is harmless, so the probe cannot be read as indicting it.
///
/// The outer wake is scheduled for 200 ms and the inner future completes at
/// 20 ms, so the inner `block_on` has returned long before the outer waker
/// fires: there is no park of the inner loop for the outer's token to land in.
#[test]
fn nesting_alone_is_harmless() {
    let completed = completes_within(DEADLINE, || {
        block_on(NestingOuter {
            outer_wake_after: Duration::from_millis(200),
            inner_ready_after: Some(Duration::from_millis(20)),
            armed: false,
            done: Arc::new(AtomicBool::new(false)),
        });
    });
    println!("WAKEUP\tcase=nested-no-collision\tcompleted={completed}");
    assert!(
        completed,
        "nesting with no concurrent outer wake must complete — if this fails the \
         probe is measuring something other than the lost wakeup"
    );
}

/// The hazard: the outer wake is delivered while the nested `block_on` is
/// parked.
///
/// The inner future completes at 200 ms; the outer waker fires at 20 ms, by
/// which time the inner loop has polled once, seen `Pending` and parked. The
/// inner park consumes the outer's token. When the inner loop finally returns and
/// the outer poll answers `Pending`, the outer park has nothing left to consume.
///
/// This test **asserts nothing**. It is a measurement, and the finding it
/// belongs to is about `happenstance-testkit`, which this directory may not
/// change; a failing assertion here would read as this experiment being broken.
/// The row it prints is the result.
#[test]
fn an_outer_wake_delivered_during_a_nested_block_on() {
    let completed = completes_within(DEADLINE, || {
        block_on(NestingOuter {
            outer_wake_after: Duration::from_millis(20),
            inner_ready_after: Some(Duration::from_millis(200)),
            armed: false,
            done: Arc::new(AtomicBool::new(false)),
        });
    });
    println!(
        "WAKEUP\tcase=nested-collision\tcompleted={completed}\tdeadline_s={}\t\
         verdict={}",
        DEADLINE.as_secs(),
        if completed {
            "no-lost-wakeup-observed"
        } else {
            "HUNG-past-deadline"
        }
    );
}
