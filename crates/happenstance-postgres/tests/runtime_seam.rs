//! ADR-0022 §9's reproduction against a live Postgres: a store built on one
//! runtime and driven after that runtime is gone, or while it sits undriven.
//!
//! `happenstance-sqlite/tests/runtime_seam.rs` is the same reproduction one
//! layer down. Postgres has two things SQLite does not, and they are why this
//! file has more cases than its sibling:
//!
//! * **Every** server-touching method hops through the handle, `append` and
//!   `head` included, because `sqlx` needs a runtime and the concurrency
//!   family's contenders are bare threads. SQLite's write path runs inline.
//! * There is a second strand under the handle. A pooled connection's socket is
//!   registered with the I/O driver of the runtime that **opened** it, so a store
//!   whose work moves to another runtime may still be holding connections that
//!   belong to the old one. The cases that open the pool on B rather than A
//!   isolate the handle from that second strand, and
//!   `a_pool_connection_opened_on_a_dropped_runtime_does_not_serve_the_next`
//!   measures the second strand with no store involved at all.
//!
//! **Remedy B** is ADR-0081's term for the fix, as in the sibling: prefer the
//! executing runtime at use time and fall back to the captured handle. It
//! removes the first strand and cannot touch the second.
//!
//! # How this target is gated
//!
//! Exactly as `projection.rs` and `live_projection.rs` are, for the reasons
//! `postgres_conformance.rs` carries in full: every case needs the server and is
//! `#[ignore]`d with a reason, so the default gate compiles and lints it and
//! reports it as `ignored`, and CI's live-postgres job runs it with
//! `-- --ignored`.
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features --test runtime_seam -- --ignored --list
//! cargo test -p happenstance-postgres --all-features --test runtime_seam -- --ignored --show-output --test-threads=1
//! ```
//!
//! # Why plain `#[test]`, and how a hang is bounded
//!
//! Each case builds runtime A by hand and drops it, or leaves it undriven, and
//! drives the store from runtime B. Dropping a `Runtime` inside `#[tokio::test]`
//! panics, so the runtimes are built here rather than by the macro.
//!
//! Two bounds, because there are two kinds of hang and only one of them can be
//! ended from outside:
//!
//! * **Work queued on an undriven A** (Prediction 4). The read runs on a named
//!   worker thread that reports through a `std::sync::mpsc` channel, and the
//!   test waits with `recv_timeout`. On a timeout the test drops A, which
//!   cancels what was queued there, and then joins the worker, so the failure
//!   names what the read yielded afterwards rather than surfacing as a CI
//!   timeout.
//! * **A connection whose I/O driver is gone.** Dropping a runtime cannot end
//!   this one: the runtime is already gone. The operation is wrapped in
//!   `tokio::time::timeout` on B instead, which drops the future. `time` is a
//!   feature of this crate's own tokio dependency, so it costs the
//!   dev-dependency nothing.

#![cfg(not(target_arch = "wasm32"))]

mod support;

use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use happenstance_core::{
    AppendError, Event, EventStore, Query, ReadOptions, SequencePosition, SequencedEvent, collect,
};
use happenstance_postgres::error::PostgresEventStoreError;
use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::sqlx;
use happenstance_testkit::Fixture;
use support::PostgresFixture;
use tokio::runtime::{Builder, Handle, Runtime};

/// How long an operation is given before it is called a hang.
///
/// Three times the fixture pool's five-second `acquire_timeout`, so an operation
/// that is merely waiting for a connection reports the pool's own error rather
/// than being classified as a hang.
const HANG_LIMIT: Duration = Duration::from_secs(15);

/// Runtime A: the one the store captures, and the one that goes away or idles.
///
/// Current-thread on purpose. Once its `block_on` has returned nothing drives it,
/// which is the state Prediction 4 needs and a multi-thread runtime never
/// reaches: its workers keep running whether or not anyone is blocked on it.
fn first_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("a current-thread runtime with its drivers builds")
}

/// Runtime B: the one the store is driven from afterwards.
fn second_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("a multi-thread runtime with its drivers builds")
}

fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).expect("a non-empty type and a small payload")
}

fn positions(events: &[SequencedEvent]) -> Vec<SequencePosition> {
    events.iter().map(|event| event.position).collect()
}

type ReadOutcome = Result<Vec<SequencedEvent>, PostgresEventStoreError>;

/// A read of the whole log, driven from runtime B on its own thread.
struct BoundedRead {
    outcome: Receiver<ReadOutcome>,
    worker: JoinHandle<()>,
}

impl BoundedRead {
    /// Starts the read. It runs on `runtime` through [`Handle::block_on`], so
    /// the executing runtime is B even though the worker thread is not one of
    /// B's own.
    fn start(runtime: Handle, store: PostgresEventStore) -> Self {
        let (send, outcome) = mpsc::sync_channel(1);
        let worker = thread::Builder::new()
            .name("runtime-seam-read".to_owned())
            .spawn(move || {
                let read = runtime.block_on(collect(store.read(&Query::all(), ReadOptions::new())));
                send.send(read)
                    .expect("the test thread holds the receiver until it joins this one");
            })
            .expect("the OS starts a test thread");
        Self { outcome, worker }
    }

    /// The read's outcome, or `None` if it has not finished within
    /// [`HANG_LIMIT`].
    fn within_limit(&self) -> Option<ReadOutcome> {
        match self.outcome.recv_timeout(HANG_LIMIT) {
            Ok(outcome) => Some(outcome),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                panic!("the read's worker thread exited without reporting; it panicked")
            }
        }
    }

    /// Waits for the outcome and joins the worker.
    ///
    /// # Panics
    ///
    /// If the worker has still not reported after [`HANG_LIMIT`]. The worker is
    /// then left running until the process exits, which is the one case in which
    /// it is not joined: joining a hung thread hangs the test.
    fn finish(self) -> ReadOutcome {
        let outcome = self
            .within_limit()
            .unwrap_or_else(|| panic!("the read did not finish within {HANG_LIMIT:?}"));
        self.worker
            .join()
            .expect("the worker reported, so it did not panic");
        outcome
    }
}

/// Opens `fixture`'s pool on `pool_runtime`, then constructs a store inside
/// `capturing`, so that the handle the store captures is `capturing`'s and
/// every connection its pool holds is `pool_runtime`'s.
fn store_captured_on(
    fixture: &PostgresFixture,
    pool_runtime: &Runtime,
    capturing: &Runtime,
) -> PostgresEventStore {
    pool_runtime.block_on(async {
        sqlx::query("SELECT 1")
            .execute(&fixture.pool_for_test().await)
            .await
            .expect("the pool opens a connection on its own runtime");
    });
    // The pool is open, so this `connect` only clones it and calls
    // `PostgresEventStore::new`, which is where the handle is captured.
    capturing.block_on(fixture.connect())
}

/// Seeds one event through a store constructed on `runtime`, which is alive.
fn seed_on(fixture: &PostgresFixture, runtime: &Runtime) -> SequencePosition {
    runtime.block_on(async {
        fixture
            .connect()
            .await
            .append(&[event("Seeded")], None)
            .await
            .expect("a store built on a live runtime appends from it")
    })
}

/// Prediction 3 with the second strand removed: the pool is opened on B, so the
/// only thing tying the store to A is the handle it captured. Every method that
/// hops through it is driven from B after A is gone.
///
/// Before remedy B, `read` yielded `Worker(JoinError::Cancelled)`, and so did
/// every `on_runtime` call: the work was spawned onto A's closed task list.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn a_handle_captured_on_a_dropped_runtime_strands_nothing() {
    let fixture = PostgresFixture::new();
    let second = second_runtime();
    let first = first_runtime();
    let store = store_captured_on(&fixture, &second, &first);
    let seeded = seed_on(&fixture, &second);
    drop(first);

    let appended = second.block_on(async {
        let appended = match store.append(&[event("FromB")], None).await {
            Ok(appended) => appended,
            Err(error) => panic!("ADR-0022 §9: an append from B failed as {error:?}"),
        };
        assert!(appended > seeded, "B's append lands after the seed");
        match store.head().await {
            Ok(head) => assert_eq!(head, Some(appended), "head names B's append"),
            Err(error) => panic!("ADR-0022 §9: head from B failed as {error:?}"),
        }
        appended
    });

    let read = BoundedRead::start(second.handle().clone(), store);
    match read.finish() {
        Ok(events) => assert_eq!(positions(&events), vec![seeded, appended]),
        Err(error) => panic!("ADR-0022 §9: the read from B failed as {error:?}"),
    }
}

/// Prediction 4: A is kept alive but nothing drives it, and the store's pool is
/// on B, so only the captured handle can make this read wait.
///
/// Before remedy B, the read was spawned into A's queue, which nobody polls, and
/// it hung for the whole bound; dropping A then cancelled it.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn a_read_from_b_does_not_wait_on_an_undriven_capturing_runtime() {
    let fixture = PostgresFixture::new();
    let second = second_runtime();
    let first = first_runtime();
    let store = store_captured_on(&fixture, &second, &first);
    let seeded = seed_on(&fixture, &second);

    let read = BoundedRead::start(second.handle().clone(), store);
    if let Some(outcome) = read.within_limit() {
        read.worker
            .join()
            .expect("the worker reported, so it did not panic");
        drop(first);
        match outcome {
            Ok(events) => assert_eq!(positions(&events), vec![seeded]),
            Err(error) => panic!("the read from B failed as {error:?}"),
        }
    } else {
        drop(first);
        let after = read.finish();
        panic!(
            "ADR-0022 §9, Prediction 4: the read hung for {HANG_LIMIT:?} on the undriven \
             runtime it captured; once that runtime was dropped it yielded {after:?}"
        );
    }
}

/// Prediction 5: does a pooled connection opened on A survive A's drop?
///
/// No store is involved, so that what the next case observes can be attributed
/// to the pool rather than to the handle. **It does not reliably survive, and
/// remedy B cannot make it**: the socket is still a good OS socket, but its
/// readiness is registered with A's I/O driver, which went with A, and `sqlx`
/// does not see such a socket as broken. A read that finds its reply already in
/// the socket buffer succeeds; a read that has to wait for one waits for a
/// readiness event nobody will deliver.
///
/// With `SELECT 1` that is a race, measured against `sqlx` 0.8.6 and tokio
/// 1.53.1 at three outcomes across five runs: success, `PoolTimedOut` (the
/// `test_before_acquire` ping waits out the five-second `acquire_timeout`), and
/// an unbounded hang in the query once the ping has raced through. So the query
/// here is `pg_sleep`, whose reply cannot be in the buffer when the read is first
/// polled, and the outcome is no longer a race: the ping either times the
/// acquire out or races through, and then the query waits. A success means this
/// strand has gone and the record describing it is wrong.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn a_pool_connection_opened_on_a_dropped_runtime_does_not_serve_the_next() {
    let fixture = PostgresFixture::new();
    let first = first_runtime();
    let pool = first.block_on(async {
        let pool = fixture.pool_for_test().await;
        sqlx::query("SELECT 1")
            .execute(&pool)
            .await
            .expect("a connection opened on A, which is alive, answers");
        pool
    });
    drop(first);
    let second = second_runtime();

    // Built inside the `async` block: `timeout` arms its timer when it is
    // constructed, and outside B there is no timer to arm.
    let outcome = second.block_on(async {
        tokio::time::timeout(
            HANG_LIMIT,
            sqlx::query("SELECT pg_sleep(0.2)").execute(&pool),
        )
        .await
    });

    match outcome {
        Err(tokio::time::error::Elapsed { .. }) | Ok(Err(sqlx::Error::PoolTimedOut)) => {}
        Ok(Ok(_)) => panic!(
            "a pooled connection opened on a dropped runtime served the next one; the \
             second strand under ADR-0022 §9 has gone, and its record is now wrong"
        ),
        Ok(Err(other)) => panic!("the first use from B failed in an unmeasured way: {other:?}"),
    }
}

/// Prediction 3, the realistic shape: pool, store and seed all on A, A dropped,
/// and the store driven from B. A `static` store initialised inside the first
/// `#[tokio::test]` is exactly this.
///
/// Before remedy B, the first call from B failed as `Worker(JoinError::Cancelled)`:
/// the handle strand. Remedy B removes that, and what remains is the pool strand
/// the previous case pins. The store's work now runs on B, but the connection it
/// is handed was opened on A, so whether `head` answers is the previous case's
/// race: it answers, reports the pool's `PoolTimedOut`, or does not finish. All
/// three are accepted here, an answer only if it is the right one. What this
/// case rejects is the handle strand, and an empty answer that would look like
/// an empty log. **The pool strand is what remedy B does not fix**: an
/// application whose pool can outlive the runtime that opened it must open the
/// pool on a runtime that lives as long as the pool does.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn a_store_built_wholly_on_a_dropped_runtime_is_stranded_by_its_pool_not_its_handle() {
    let fixture = PostgresFixture::new();
    let first = first_runtime();
    let (store, seeded) = first.block_on(async {
        let store = fixture.connect().await;
        let seeded = store
            .append(&[event("Seeded")], None)
            .await
            .expect("the seeding append runs on A, which is alive");
        (store, seeded)
    });
    drop(first);
    let second = second_runtime();

    let outcome = second.block_on(async { tokio::time::timeout(HANG_LIMIT, store.head()).await });

    match outcome {
        Err(tokio::time::error::Elapsed { .. })
        | Ok(Err(PostgresEventStoreError::Driver(sqlx::Error::PoolTimedOut))) => {}
        Ok(Err(PostgresEventStoreError::Worker(join))) => panic!(
            "ADR-0022 §9's handle strand: head from B was spawned onto the dropped runtime \
             and its task was {} ({join})",
            if join.is_cancelled() {
                "cancelled"
            } else {
                "not cancelled"
            }
        ),
        Ok(Ok(head)) => assert_eq!(
            head,
            Some(seeded),
            "a head that raced through a connection opened on A must name A's append"
        ),
        Ok(Err(other)) => panic!("head from B failed in an unmeasured way: {other:?}"),
    }
}

/// What a store's three operations yielded when called from a runtime without
/// drivers.
struct DriverlessOutcome {
    head: Result<Option<SequencePosition>, PostgresEventStoreError>,
    append: Result<SequencePosition, AppendError<PostgresEventStoreError>>,
    read: ReadOutcome,
}

/// The price of remedy B on Postgres alone (ADR-0081 §6): the calling runtime
/// now runs the store's work, so it needs tokio's drivers.
///
/// The pool, the store and a seed are all on a full multi-thread runtime that
/// stays alive throughout, so neither strand above is in play. The store is then
/// called from a `current_thread` runtime built **without** `enable_all`. Remedy
/// B spawns the work there, and `sqlx` acquires every connection under
/// `tokio::time::timeout`, which panics on a runtime with no timer
/// (`sqlx-core-0.8.6/src/rt/mod.rs:29`, *"timers are disabled"*): each of `head`,
/// `append` and `read` ends as `Worker(JoinError::Panic)`. With the captured
/// handle put back first, as before remedy B, the same `head` succeeded.
///
/// Measured once and not pinned: the same calls from a runtime with
/// `enable_time` alone succeeded, because they reused an idle connection whose
/// socket belongs to the full runtime. The I/O driver is needed, by reading and
/// not by running, when a call has to open a new connection, which `sqlx` does
/// inside `acquire`, on the caller's runtime.
///
/// The driverless runtime has no timer to bound a hang with, so the calls run on
/// a named worker thread that reports through a channel, as [`BoundedRead`] does.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn a_call_from_a_runtime_without_drivers_fails_as_a_worker_panic() {
    let fixture = PostgresFixture::new();
    let full = second_runtime();
    let (store, seeded) = full.block_on(async {
        let store = fixture.connect().await;
        let seeded = store
            .append(&[event("Seeded")], None)
            .await
            .expect("a store built on a live, full runtime appends from it");
        (store, seeded)
    });

    let (send, outcome) = mpsc::sync_channel(1);
    // A second handle onto the same store, moved to the worker; the pool and the
    // identity cell are both shared, so this clones two `Arc`s and no state.
    let caller = store.clone();
    let worker = thread::Builder::new()
        .name("runtime-seam-driverless".to_owned())
        .spawn(move || {
            let driverless = Builder::new_current_thread()
                .build()
                .expect("a current-thread runtime with no drivers builds");
            let outcome = driverless.block_on(async {
                DriverlessOutcome {
                    head: caller.head().await,
                    append: caller.append(&[event("Driverless")], None).await,
                    read: collect(caller.read(&Query::all(), ReadOptions::new())).await,
                }
            });
            send.send(outcome)
                .expect("the test thread holds the receiver until it joins this one");
        })
        .expect("the OS starts a test thread");
    let outcome = outcome
        .recv_timeout(HANG_LIMIT)
        .unwrap_or_else(|error| panic!("the driverless calls did not report: {error:?}"));
    worker
        .join()
        .expect("the worker reported, so it did not panic");

    match outcome.head {
        Err(PostgresEventStoreError::Worker(join)) if join.is_panic() => {}
        other => panic!("ADR-0081 §6: head from a driverless runtime yielded {other:?}"),
    }
    match outcome.append {
        Err(AppendError::Store(PostgresEventStoreError::Worker(join))) if join.is_panic() => {}
        other => panic!("ADR-0081 §6: append from a driverless runtime yielded {other:?}"),
    }
    match outcome.read {
        Err(PostgresEventStoreError::Worker(join)) if join.is_panic() => {}
        other => panic!("ADR-0081 §6: read from a driverless runtime yielded {other:?}"),
    }

    // The failure is the caller's runtime, not the store: from the full runtime
    // the same store still answers, and the driverless append did not land.
    let head = full
        .block_on(store.head())
        .expect("the store answers from the runtime that has its drivers");
    assert_eq!(head, Some(seeded), "only the seed is in the log");
}
