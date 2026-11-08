//! [`Edge::run`]: steps until the edge is synced or halts.
//!
//! Its own file so the step's control flow and the loop that paces it read
//! apart: the step decides what a failure does to the edge, and the loop only
//! decides whether to wait, retry or return.

use super::backoff::{Backoff, Sleep};
use super::{Edge, EdgeState, StepFailure, Stepped};
use crate::classify::{Classify, FailureClass};
use crate::ingest::IngestStore;
use crate::peer::SyncPeer;
use crate::write_group::WriteGroupStore;

impl<R> Edge<R> {
    /// Steps until the edge is synced, or quiet, or halts: a caller-owned
    /// loop, driven on whatever runtime the caller has. Nothing is spawned.
    ///
    /// It returns `Ok` after a step that left the edge
    /// [synced](super::SyncStatus::is_synced), or that moved nothing new: it
    /// pushed nothing and ingested nothing it lacked, so a far side that cannot
    /// say [`Waiting::Nothing`](crate::Waiting::Nothing) still lets the run end.
    /// Call it again to sync again; how often is the caller's.
    ///
    /// After a [`Transient`](FailureClass::Transient) failure it waits the
    /// backoff's next delay through `sleep` and steps again, and a successful
    /// step resets the backoff. After a [`Split`](FailureClass::Split) the edge
    /// could halve, it steps again at once, with the smaller batch.
    ///
    /// Every pass of the loop either moved an event, halved the batch, or
    /// spent a backoff attempt, so the run ends; and it can be dropped between
    /// any two awaits without losing anything a later run does not redo.
    ///
    /// # Errors
    ///
    /// The step error that ended the run: one that parked or stopped the
    /// edge, including [`StepError::Incarnation`](super::StepError::Incarnation),
    /// or the last transient failure once the backoff's attempts are spent.
    /// In that last case the edge is still running, and a later run retries:
    /// the remedy for a far side that stays down is the caller's.
    ///
    /// # Examples
    ///
    /// A run against a reachable hub ends synced, without waiting at all:
    ///
    /// ```
    /// # #[cfg(not(feature = "memory"))]
    /// # fn main() {}
    /// # #[cfg(feature = "memory")]
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
    /// use core::time::Duration;
    ///
    /// use happenstance_core::{Event, EventStore, StoreId};
    /// use happenstance_sync::runner::{Backoff, Edge, Sleep};
    /// use happenstance_sync::{MemoryIngestStore, MemorySyncPeer};
    ///
    /// /// The caller's clock. A real one awaits its runtime's timer here —
    /// /// `tokio::time::sleep`, a Durable Object alarm — which the library never
    /// /// names; this run never fails, so it is never called.
    /// struct Clock;
    ///
    /// impl Sleep for Clock {
    ///     async fn sleep(&self, _duration: Duration) {}
    /// }
    ///
    /// let hub = MemorySyncPeer::new(StoreId::from_bytes([2; 16]));
    /// let tablet = MemoryIngestStore::new();
    /// tablet
    ///     .append(&[Event::new("ReadingTaken", &b"{\"celsius\":4}"[..])?], None)
    ///     .await?;
    /// let mut backoff = Backoff::new(Duration::from_millis(100), Duration::from_secs(30), 7)?;
    ///
    /// let mut edge = Edge::new();
    /// edge.run(&hub, &tablet, &mut backoff, &Clock).await?;
    ///
    /// assert!(edge.status().is_synced());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run<P, S, Z>(
        &mut self,
        peer: &P,
        store: &S,
        backoff: &mut Backoff,
        sleep: &Z,
    ) -> Result<(), StepFailure<P, S>>
    where
        P: SyncPeer<Resume = R>,
        S: WriteGroupStore + IngestStore,
        Z: Sleep,
    {
        loop {
            let error = match self.step(peer, store).await {
                Ok(stepped) => {
                    backoff.reset();
                    if self.status.is_synced() || moved_nothing_new(&stepped) {
                        return Ok(());
                    }
                    continue;
                }
                Err(error) => error,
            };
            if self.status.state != EdgeState::Running {
                return Err(error);
            }
            match error.class() {
                FailureClass::Transient => {
                    let Some(delay) = backoff.next_delay() else {
                        return Err(error);
                    };
                    // Dropped before the wait, so a `!Send` error never sits
                    // in a `Send` future across the await (ADR-0009).
                    drop(error);
                    sleep.sleep(delay).await;
                }
                // The step halved the batch; the next one offers fewer groups.
                FailureClass::Split => {}
                // The step parked or stopped the edge for these, so the state
                // check above has already returned; kept total all the same.
                FailureClass::Park | FailureClass::Stop => return Err(error),
            }
        }
    }
}

/// Pushed nothing, and ingested nothing the local store lacked.
const fn moved_nothing_new(stepped: &Stepped) -> bool {
    stepped.pushed == 0 && stepped.ingested.appended == 0 && stepped.ingested.compensated == 0
}
