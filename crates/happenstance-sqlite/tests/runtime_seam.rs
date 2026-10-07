//! ADR-0022 §9's reproduction: a store built on one runtime and driven after
//! that runtime is gone.
//!
//! Both stores in this crate hop onto a blocking thread through a
//! [`tokio::runtime::Handle`]. They capture one at construction, so that the
//! concurrency family's bare OS threads, which have no runtime of their own, still
//! have one to hop onto. The hazard §9 left open is the other half: a store that
//! outlives the runtime it was built in. A `static` store initialised inside the
//! first `#[tokio::test]` is the realistic shape, because every later test owns
//! a different runtime.
//!
//! **Remedy B** below is ADR-0081's name for the fix: at use time prefer the
//! runtime the caller is executing on (`Handle::try_current`), and fall back to
//! the captured handle only when there is none. Each case says what it did
//! before that fix.
//!
//! # Why plain `#[test]`
//!
//! Each case builds runtime A, drops it, and builds runtime B. Dropping a
//! `Runtime` inside `#[tokio::test]` panics ("Cannot drop a runtime in a context
//! where blocking is not allowed"), so the runtimes are built by hand.
//!
//! # What each case rejects
//!
//! * Every case seeds an event or a checkpoint on A first, so a stranded call
//!   that ends as an empty `Ok`, which looks exactly like an empty log, fails.
//! * The store is always constructed **inside** A, so the handle under test is a
//!   captured one. A store constructed outside every runtime captures nothing and
//!   tests the fallback instead; `tests/read.rs` and `tests/concurrency.rs`
//!   already cover that.
//! * Construction and use happen on **different** runtimes. One runtime for both
//!   is `examples/transfers-on-sqlite/tests/contention.rs` and proves nothing
//!   about the strand.
//!
//! The no-runtime-anywhere control for the event store is
//! `read_polled_outside_a_runtime_yields_an_error_item` in `tests/read.rs` and
//! `a_store_with_no_runtime_anywhere_reports_no_runtime` in
//! `tests/concurrency.rs`. The projection store had none, so it is here.

#![cfg(not(target_arch = "wasm32"))]

use tokio::runtime::{Builder, Runtime};

/// Runtime A: the one the store is built in, and the one that goes away.
fn first_runtime() -> Runtime {
    Builder::new_current_thread()
        .build()
        .expect("a current-thread runtime builds with no driver enabled")
}

/// Runtime B: the one the store is driven from after A has gone.
///
/// Multi-threaded on purpose. A different flavour from A rules out any reading
/// in which B merely happens to resemble A.
fn second_runtime() -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(2)
        .build()
        .expect("a multi-thread runtime builds with no driver enabled")
}

#[cfg(feature = "event-store")]
mod event_store {
    use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencePosition, collect};
    use happenstance_sqlite::event_store::{SqliteEventStore, SqliteEventStoreError};

    use super::{first_runtime, second_runtime};

    fn event(event_type: &str) -> Event {
        Event::new(event_type, &b"{}"[..]).expect("a non-empty type and a small payload")
    }

    /// Builds and seeds a store inside runtime A, then drops A.
    ///
    /// Returns the store and the position A's append was assigned.
    fn stranded_store() -> (SqliteEventStore, SequencePosition) {
        let first = first_runtime();
        let (store, seeded) = first.block_on(async {
            let store = SqliteEventStore::open_in_memory().expect("an in-memory store opens");
            let seeded = store
                .append(&[event("Seeded")], None)
                .await
                .expect("the seeding append runs on A, which is alive");
            (store, seeded)
        });
        drop(first);
        (store, seeded)
    }

    /// The read §9 is about: built on A, polled from B after A is gone.
    ///
    /// Before remedy B, the store spawned onto the handle it captured from A,
    /// whose blocking pool had shut down, and the read yielded one
    /// `Worker(JoinError::Cancelled)`.
    #[test]
    fn a_read_from_the_next_runtime_sees_the_log() {
        let (store, seeded) = stranded_store();
        let second = second_runtime();

        let outcome = second.block_on(collect(store.read(&Query::all(), ReadOptions::new())));

        match outcome {
            Ok(events) => assert_eq!(
                events.iter().map(|e| e.position).collect::<Vec<_>>(),
                vec![seeded],
                "the read from B must return exactly the event A appended"
            ),
            Err(SqliteEventStoreError::Worker(join)) => panic!(
                "ADR-0022 §9's stranded read: the read hopped onto the dead runtime \
                 and its task was {} ({join})",
                if join.is_cancelled() {
                    "cancelled"
                } else {
                    "not cancelled"
                }
            ),
            Err(other) => panic!("the read from B failed as {other:?}"),
        }
    }

    /// The write path and `head` run inline (ADR-0058), so they never touched the
    /// captured handle. Pinned so a remedy cannot regress them, and so that the
    /// read afterwards sees both events.
    #[test]
    fn append_and_head_from_the_next_runtime_and_a_read_sees_both() {
        let (store, seeded) = stranded_store();
        let second = second_runtime();

        second.block_on(async {
            let appended = store
                .append(&[event("FromB")], None)
                .await
                .expect("an append from B runs inline and needs no runtime");
            assert!(appended > seeded, "B's append lands after A's");

            let head = store.head().await.expect("head from B runs inline");
            assert_eq!(head, Some(appended), "head names B's append");

            let positions: Vec<SequencePosition> =
                match collect(store.read(&Query::all(), ReadOptions::new())).await {
                    Ok(events) => events.iter().map(|e| e.position).collect(),
                    Err(error) => panic!("the read from B after B's append failed as {error:?}"),
                };
            assert_eq!(positions, vec![seeded, appended]);
        });
    }
}

#[cfg(feature = "projection-store")]
mod projection_store {
    use happenstance_core::{
        Authority, Checkpoint, ProjectionId, ProjectionStore, SequencePosition,
    };
    use happenstance_sqlite::projection_store::{
        SqliteProjectionStore, SqliteProjectionStoreError,
    };

    use super::{first_runtime, second_runtime};

    fn id() -> ProjectionId {
        ProjectionId::new("runtime-seam")
    }

    fn position(raw: u64) -> SequencePosition {
        SequencePosition::new(raw).expect("a non-zero literal")
    }

    /// Builds a projection store inside runtime A, commits a checkpoint through
    /// it, then drops A.
    fn stranded_store() -> SqliteProjectionStore {
        let first = first_runtime();
        let store = first.block_on(async {
            let store = SqliteProjectionStore::open_in_memory().expect("an in-memory store opens");
            let batch = store.begin().await.expect("begin on A");
            store
                .commit(batch, &id(), position(3), Authority::Live)
                .await
                .expect("the seeding commit runs on A, which is alive");
            store
        });
        drop(first);
        store
    }

    /// Every SQL-touching method of the projection store goes through the one
    /// seam, so before remedy B the whole store was unusable from B, not just one
    /// method.
    #[test]
    fn checkpoint_and_commit_from_the_next_runtime_succeed() {
        let store = stranded_store();
        let second = second_runtime();

        second.block_on(async {
            match store.checkpoint(&id()).await {
                Ok(checkpoint) => assert_eq!(
                    checkpoint,
                    Checkpoint::Live {
                        through: position(3)
                    },
                    "the checkpoint from B is the one A committed"
                ),
                Err(SqliteProjectionStoreError::Worker(join)) => panic!(
                    "ADR-0022 §9's strand on the projection store: checkpoint hopped onto \
                     the dead runtime and its task was {} ({join})",
                    if join.is_cancelled() {
                        "cancelled"
                    } else {
                        "not cancelled"
                    }
                ),
                Err(other) => panic!("checkpoint from B failed as {other:?}"),
            }

            let batch = store.begin().await.expect("begin from B");
            store
                .commit(batch, &id(), position(7), Authority::Live)
                .await
                .expect("a commit from B must land");
            let after = store.checkpoint(&id()).await.expect("checkpoint from B");
            assert_eq!(
                after,
                Checkpoint::Live {
                    through: position(7)
                }
            );
        });
    }

    /// The projection store's `NoRuntime` control: constructed **and** driven with
    /// no runtime anywhere. Remedy B must keep this variant reachable.
    #[test]
    fn a_projection_store_with_no_runtime_anywhere_reports_no_runtime() {
        let store = SqliteProjectionStore::open_in_memory().expect("an in-memory store opens");

        let outcome = happenstance_testkit::block_on(store.checkpoint(&id()));

        match outcome {
            Err(SqliteProjectionStoreError::NoRuntime(_)) => {}
            other => panic!(
                "a store constructed and driven with no runtime anywhere must report \
                 NoRuntime; got {other:?}"
            ),
        }
    }
}
