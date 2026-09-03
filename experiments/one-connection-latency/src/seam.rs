//! Instrument (c): the three synchronous bodies behind an `in_blocking_task`
//! seam.
//!
//! # What this is
//!
//! `SqliteProjectionStore::in_blocking_task`
//! (`crates/happenstance-sqlite/src/projection_store.rs:342-361`) is the seam
//! the *projection* store already has and the event store does not. Its own doc
//! calls it "the one seam every SQL-touching body goes through, so that the lock
//! is taken and released inside a `'static` closure and no guard is ever live
//! across an `await`."
//!
//! This module is that seam, transcribed, with `append`, `head` and
//! `contains_event_id` routed through it — the fix J-2, F2-1 and I-4 all
//! propose, built here so that the *residual* after it can be measured rather
//! than argued about.
//!
//! It shares [`Replica`]'s connection rather than opening a second one
//! ([`Replica::connection`]), because a seam arm on a different mutex and a
//! different file would be a different experiment.
//!
//! # What the seam costs, stated because the figure does not show it
//!
//! `spawn_blocking` demands a `'static` closure, so the batch has to be **owned**
//! by it: `events.to_vec()` deep-clones every [`Event`], and by AE-1's mechanism
//! that is `t + 2` allocations each. The shipped `append` takes `&[Event]` and
//! clones nothing. That is a real cost the remediation has to book, and it is
//! why the fix is a decision rather than an obvious win — though `Bytes` is
//! refcounted, so the payload itself does not copy.
//!
//! # What it does not fix, by construction
//!
//! `ReadCursor::sample_ceiling` (`event_store.rs:1239-1253`) takes the same
//! mutex from inside `poll_next`, on the polling thread, and ES-11 requires the
//! sample to be taken no later than the first poll. No seam on the *write* side
//! moves it. `tests/reactor_stall.rs` measures that residual as its own arm.

use std::sync::{Arc, Mutex};

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, SequencePosition, StoreId,
};
use rusqlite::Connection;
use tokio::runtime::Handle;

use crate::replica::{Replica, ReplicaError};

/// The event store's three synchronous bodies, behind the projection store's
/// seam.
#[derive(Debug, Clone)]
pub struct SeamStore {
    connection: Arc<Mutex<Connection>>,
    store_id: StoreId,
    runtime: Option<Handle>,
}

impl SeamStore {
    /// Wraps `replica`'s connection, so both arms contend for one mutex.
    #[must_use]
    pub fn over<const PAGE: usize>(replica: &Replica<PAGE>) -> Self {
        Self {
            connection: replica.connection(),
            store_id: replica.store_id(),
            runtime: Handle::try_current().ok(),
        }
    }

    /// The runtime this store's blocking work hops onto
    /// (`projection_store.rs:330-336`).
    fn runtime(&self) -> Result<Handle, ReplicaError> {
        match &self.runtime {
            Some(runtime) => Ok(runtime.clone()),
            None => Ok(Handle::try_current()?),
        }
    }

    /// Runs `work` against this store's connection on a blocking thread
    /// (`projection_store.rs:342-361`), transcribed.
    ///
    /// # Errors
    ///
    /// `work`'s error, or [`ReplicaError::ConnectionPoisoned`],
    /// [`ReplicaError::Worker`] or [`ReplicaError::NoRuntime`].
    async fn in_blocking_task<T, F>(&self, work: F) -> Result<T, ReplicaError>
    where
        F: FnOnce(&mut Connection) -> Result<T, ReplicaError> + Send + 'static,
        T: Send + 'static,
    {
        let connection = Arc::clone(&self.connection);
        let runtime = self.runtime()?;
        let joined = runtime
            .spawn_blocking(move || {
                let mut guard = connection
                    .lock()
                    .map_err(|_| ReplicaError::ConnectionPoisoned)?;
                work(&mut guard)
            })
            .await;
        match joined {
            Ok(outcome) => outcome,
            Err(join) => Err(ReplicaError::from(join)),
        }
    }

    /// `append`, with the transaction on a blocking thread.
    ///
    /// Steps 1 and 2 stay on the caller's task, exactly as the shipped body has
    /// them: they run no SQL, and moving them would delay `NoEvents` and the
    /// ceiling refusals behind a thread hop for nothing.
    ///
    /// # Errors
    ///
    /// As `SqliteEventStore::append`.
    pub async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<ReplicaError>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        crate::replica::check_ceilings(events)?;

        let recorded_at = crate::replica::now();
        let store_id = self.store_id;
        // The `'static` closure's price. See the module documentation.
        let owned: Vec<Event> = events.to_vec();
        let condition = condition.cloned();

        // The seam's error channel is `ReplicaError` and `append`'s is
        // `AppendError<ReplicaError>`, so the outcome travels as a value and is
        // unwrapped on the far side. That is the same shape the remediation
        // notes describe, and it is why `projection_store.rs`'s seam cannot
        // simply be called: it is hard-wired to one error type.
        let outcome = self
            .in_blocking_task(move |connection| {
                Ok(crate::replica::append_locked(
                    connection,
                    store_id,
                    &owned,
                    condition.as_ref(),
                    recorded_at,
                ))
            })
            .await
            .map_err(AppendError::Store)?;
        outcome
    }

    /// `head`, on a blocking thread.
    ///
    /// # Errors
    ///
    /// As `SqliteEventStore::head`.
    pub async fn head(&self) -> Result<Option<SequencePosition>, ReplicaError> {
        self.in_blocking_task(|connection| {
            let highest: Option<i64> =
                connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
            Ok(highest.and_then(|value| SequencePosition::new(value.unsigned_abs())))
        })
        .await
    }

    /// `contains_event_id`, on a blocking thread.
    ///
    /// # Errors
    ///
    /// As `SqliteEventStore::contains_event_id`.
    pub async fn contains_event_id(&self, id: EventId) -> Result<bool, ReplicaError> {
        let store = id.store().to_bytes();
        let position = i64::try_from(id.position().get()).unwrap_or(i64::MAX);
        self.in_blocking_task(move |connection| {
            let found: Option<i64> = connection
                .query_row(
                    "SELECT 1 FROM event WHERE origin_store = ? AND origin_position = ? LIMIT 1",
                    rusqlite::params![&store[..], position],
                    |row| row.get(0),
                )
                .or_else(|err| match err {
                    rusqlite::Error::QueryReturnedNoRows => Ok(None),
                    other => Err(other),
                })?;
            Ok(found.is_some())
        })
        .await
    }
}
