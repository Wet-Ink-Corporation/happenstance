//! **Instrument (a) and (c).** How long does an `append` stall the reactor, and
//! what is left after the seam?
//!
//! # The construction
//!
//! One `current_thread` tokio runtime — the flavour `#[tokio::test]` defaults
//! to, the flavour `examples/transfers-on-sqlite` uses, and the flavour a
//! local-first deployment is most likely to be on. On it, a task ticks a 1 ms
//! `tokio::time::interval` and records every gap. Beside it, a bare OS thread
//! opens a **second connection** onto the same file through the shipped
//! adapter's own `open_configured` and holds `BEGIN IMMEDIATE` for
//! [`HOLD`]. Each arm then does one thing and the ticker's largest gap is the
//! answer.
//!
//! # Two different contentions, because WAL makes them different
//!
//! Under WAL a writer does not block readers. So:
//!
//! * **The write lock** stalls `append`, which opens `BEGIN IMMEDIATE`. This is
//!   the contention the finding names, and it is what arms 2, 3 and 4 measure.
//! * **The connection mutex** stalls everything else on the same handle. `head`,
//!   `contains_event_id` and `ReadCursor::sample_ceiling` all run plain
//!   `SELECT`s that WAL would happily serve concurrently — they wait because the
//!   *adapter* serialises them, not because SQLite does. Arms 5, 6 and 7 hold an
//!   `append` in flight on another thread and measure that.
//!
//! Measuring only the first would have produced two rows of zeroes and the wrong
//! conclusion about `head` and about the read's first poll.
//!
//! # The controls
//!
//! * **idle** — the same ticker, the same duration, no contention and no store
//!   call. On Windows the default system timer resolution is 15.6 ms, so this
//!   row is not zero and nothing below it is measurable. Every other row is read
//!   against it.
//! * **CONTROL 1** — the *real* `SqliteProjectionStore::commit`, which already
//!   routes through `in_blocking_task`. Same file, same holder, same write lock,
//!   and the conformant answer to the same contention.
//!
//! Run with `cargo test --release --test reactor_stall -- --nocapture`.

use std::time::{Duration, Instant};

use happenstance_core::{
    Authority, Event, ProjectionId, Query, ReadOptions, SendEventStore, SendProjectionStore,
    SequencePosition,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use happenstance_sqlite::projection_store::SqliteProjectionStore;
use one_connection_latency::blocking::poll_to_completion;
use one_connection_latency::holder::WriteLockHolder;
use one_connection_latency::replica::Replica;
use one_connection_latency::seam::SeamStore;
use one_connection_latency::ticker::{TickReport, Ticker};
use one_connection_latency::workload::{Scratch, seeded_event};

/// How long the second connection holds `BEGIN IMMEDIATE`.
///
/// Well under `BUSY_TIMEOUT_MS = 5_000` (`connection.rs:62`), which is the point:
/// this is an arm that *succeeds*, so the figure is what a contended append
/// costs on a good day rather than what the timeout costs on a bad one. The
/// 5,000 ms ceiling is the same defect's worst case and is stated in README.md
/// as arithmetic rather than measured, because measuring it means measuring a
/// failure.
const HOLD: Duration = Duration::from_millis(750);

/// The tick the reactor is asked to keep.
const TICK: Duration = Duration::from_millis(1);

/// How long the reactor is left alone before a contended read arm asks for the
/// mutex, so that the sibling thread's `append` has certainly reached it.
const SETTLE: Duration = Duration::from_millis(150);

/// One row of the table.
#[derive(Debug)]
struct Arm {
    name: &'static str,
    /// What the arm's own call took, wall-clock.
    call_ms: f64,
    ticks: TickReport,
}

impl Arm {
    fn print(&self) {
        println!(
            "{:<52} call={:>8.1} ms   max_gap={:>9.3} ms   p99={:>8.3} ms   p50={:>7.3} ms   ticks={}",
            self.name,
            self.call_ms,
            self.ticks.max_gap_us / 1_000.0,
            self.ticks.p99_gap_us / 1_000.0,
            self.ticks.p50_gap_us / 1_000.0,
            self.ticks.ticks,
        );
    }
}

/// Runs `body` with a ticker beside it and reports the largest gap it left.
///
/// Two details are load-bearing, and the first draft of this file got both
/// wrong — with the consequence that three arms reported a stall of about
/// 600 ms that was entirely the harness's own.
///
/// * **The trailing `sleep`.** The ticker cannot record the gap that covers the
///   stall until it is polled again, and on a `current_thread` runtime nothing
///   polls it until the test's own task yields. Without it every stall arm
///   reports the floor.
/// * **`body` returns its scenery instead of tearing it down.** A
///   [`WriteLockHolder`] joins its OS thread on `Drop`, and `JoinHandle::join`
///   is a *blocking* call: dropping the holder inside the measured region stalls
///   the reactor for the rest of the hold, and the arm then reports that instead
///   of what it called. So every arm hands its holder and its sibling thread
///   back, and they are dropped **after** the ticker has been stopped and its
///   gaps collected.
async fn stall_arm<F, Fut, C>(name: &'static str, body: F) -> Arm
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = C>,
{
    let ticker = Ticker::start(TICK);
    // Let the ticker establish a rhythm before the arm perturbs it.
    tokio::time::sleep(Duration::from_millis(50)).await;

    let started = Instant::now();
    let scenery = body().await;
    let call = started.elapsed();

    tokio::time::sleep(Duration::from_millis(20)).await;
    let ticks = ticker.stop().await;
    // Outside the measurement, deliberately. See above.
    drop(scenery);

    Arm {
        name,
        call_ms: call.as_secs_f64() * 1_000.0,
        ticks,
    }
}

/// A sibling `append` thread, joined on `Drop` so that no arm's teardown lands
/// inside another arm's window.
#[derive(Debug)]
struct Sibling(Option<std::thread::JoinHandle<()>>);

impl Drop for Sibling {
    fn drop(&mut self) {
        if let Some(thread) = self.0.take() {
            thread.join().expect("the sibling append thread panicked");
        }
    }
}

/// A batch small enough that the arm measures the wait and not the write.
fn batch() -> Vec<Event> {
    (0..4).map(|i| seeded_event(i, 128)).collect()
}

/// Starts an `append` on a bare OS thread and returns once it is in flight.
///
/// The thread drives the future with [`poll_to_completion`] rather than a
/// runtime, because the whole point is that it is *not* the reactor. It blocks
/// inside `BEGIN IMMEDIATE`, holding the connection mutex for as long as the
/// write lock is held elsewhere — which is what makes the mutex contention this
/// function creates deterministic rather than a race.
///
/// Generic over the store so that the seam arm can put its sibling on the
/// **same** connection the seam wraps. A seam measured against contention on a
/// different mutex measures nothing.
fn append_in_flight<S>(store: S) -> Sibling
where
    S: SendEventStore + Send + 'static,
{
    Sibling(Some(std::thread::spawn(move || {
        let events = batch();
        // The result is discarded on purpose: this append is scenery. If the
        // holder outlives the busy timeout it fails, and that failure is not
        // this arm's subject.
        let _ = poll_to_completion(store.append(&events, None));
    })))
}

#[tokio::test(flavor = "current_thread")]
async fn a_contended_append_stalls_the_whole_reactor() {
    let scratch = Scratch::new("reactor-stall");
    let path = scratch.path().to_path_buf();

    let store = SqliteEventStore::open(&path).expect("opening the shipped event store");
    let projection =
        SqliteProjectionStore::open(&path).expect("opening the shipped projection store");
    let replica = Replica::<512>::open(&path).expect("opening the replica");
    let seam = SeamStore::over(&replica);

    // Something to read, so that the read arm's first poll has a ceiling to
    // sample and a page to want.
    let warmup: Vec<Event> = (0..64).map(|i| seeded_event(i, 128)).collect();
    store
        .append(&warmup, None)
        .await
        .expect("seeding the reactor-stall database");

    println!();
    println!("== instrument (a): reactor stall on one `current_thread` runtime ==");
    println!(
        "   tick = {} ms, second connection holds BEGIN IMMEDIATE for {} ms",
        TICK.as_millis(),
        HOLD.as_millis()
    );
    println!("   settings, read back off the live connection: {:?}", {
        let settings = store.settings().expect("reading the connection settings");
        format!(
            "journal_mode={} synchronous={} busy_timeout_ms={}",
            settings.journal_mode(),
            settings.synchronous(),
            settings.busy_timeout_ms()
        )
    });
    println!();

    let mut arms = Vec::new();

    // --- the floor -----------------------------------------------------------
    arms.push(
        stall_arm("0. idle: the ticker alone, no contention, no call", || {
            tokio::time::sleep(HOLD)
        })
        .await,
    );

    // --- the defect ----------------------------------------------------------
    arms.push(
        stall_arm(
            "1. SHIPPED SqliteEventStore::append, write lock held",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let events = batch();
                store
                    .append(&events, None)
                    .await
                    .expect("the contended append should still succeed");
                holder
            },
        )
        .await,
    );

    // --- CONTROL 1 -----------------------------------------------------------
    arms.push(
        stall_arm(
            "2. CONTROL 1: SHIPPED SqliteProjectionStore::commit, write lock held",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let id = ProjectionId::new("one-connection-latency");
                let position = SequencePosition::new(1).expect("1 is a position");
                projection
                    .commit(projection.begin(), &id, position, Authority::Live)
                    .await
                    .expect("the contended commit should still succeed");
                holder
            },
        )
        .await,
    );

    // --- instrument (c) ------------------------------------------------------
    arms.push(
        stall_arm(
            "3. (c) the same append through an in_blocking_task seam",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let events = batch();
                seam.append(&events, None)
                    .await
                    .expect("the seam append should still succeed");
                holder
            },
        )
        .await,
    );

    // --- the connection mutex, not the file lock -----------------------------
    //
    // The sibling `append` is what holds the mutex; the holder is what keeps the
    // sibling inside `BEGIN IMMEDIATE` long enough for the arm to be about
    // waiting rather than about scheduling. Neither is torn down here — see
    // `stall_arm`.
    arms.push(
        stall_arm(
            "4. SHIPPED head(), behind an in-flight append on the same handle",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let sibling = append_in_flight(store.clone());
                tokio::time::sleep(SETTLE).await;
                store.head().await.expect("head should still answer");
                (holder, sibling)
            },
        )
        .await,
    );

    arms.push(
        stall_arm(
            "5. SHIPPED read(): first poll samples the ceiling on this thread",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let sibling = append_in_flight(store.clone());
                tokio::time::sleep(SETTLE).await;
                // `read` itself runs nothing; the ceiling sample is in the first
                // `poll_next`, on the polling thread, by ES-11's requirement.
                let query = Query::all();
                let stream = store.read(&query, ReadOptions::new());
                let mut stream = Box::pin(stream);
                let first = futures_util::StreamExt::next(&mut stream).await;
                assert!(first.is_some(), "the store is not empty");
                (holder, sibling)
            },
        )
        .await,
    );

    // The seam arm's sibling is on the **replica's** connection, which is the
    // one `SeamStore::over` wraps. A sibling on the shipped store's connection
    // would leave this arm uncontended and it would report the floor for the
    // wrong reason.
    arms.push(
        stall_arm(
            "6. (c) head() through the seam, behind an in-flight inline append",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let sibling = append_in_flight(replica.clone());
                tokio::time::sleep(SETTLE).await;
                seam.head().await.expect("the seam head should still answer");
                (holder, sibling)
            },
        )
        .await,
    );

    // The residual, stated as its own arm rather than left to the reader: the
    // seam moves `head` off the reactor and cannot move `sample_ceiling`, which
    // ES-11 requires to be taken no later than the first poll — on the polling
    // thread, by construction.
    arms.push(
        stall_arm(
            "7. (c) residual: replica read() first poll, behind the same append",
            || async {
                let holder = WriteLockHolder::hold(&path, HOLD);
                let sibling = append_in_flight(replica.clone());
                tokio::time::sleep(SETTLE).await;
                let query = Query::all();
                let stream = replica.read(&query, ReadOptions::new());
                let mut stream = Box::pin(stream);
                let first = futures_util::StreamExt::next(&mut stream).await;
                assert!(first.is_some(), "the store is not empty");
                (holder, sibling)
            },
        )
        .await,
    );

    for arm in &arms {
        arm.print();
    }
    println!();

    // The one assertion this file makes, and it is deliberately weak: it fails
    // only if the instrument itself stopped working. The *figures* are the
    // result, and a threshold on them would be this crate quietly becoming a
    // gate step (CF-34).
    let idle = arms[0].ticks.max_gap_us;
    let contended = arms[1].ticks.max_gap_us;
    assert!(
        contended > idle,
        "the contended append arm did not exceed the idle floor \
         ({contended:.0} us vs {idle:.0} us) — the holder is not holding, or the \
         ticker is not ticking, and neither is a result"
    );
}
