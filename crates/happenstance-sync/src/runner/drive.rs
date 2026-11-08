//! [`Edge::run`]: steps until the edge is synced or halts.
//!
//! Its own file so the step's control flow and the loop that paces it read
//! apart: the step decides what a failure does to the edge, and the loop only
//! decides whether to wait, retry or return.

use core::num::NonZeroU64;

use super::backoff::{Backoff, Sleep};
use super::{Edge, EdgeState, StepFailure, Stepped};
use crate::classify::{Classify, FailureClass};
use crate::ingest::IngestStore;
use crate::peer::{SyncPeer, Waiting};
use crate::write_group::WriteGroupStore;

impl<R> Edge<R> {
    /// Steps until the edge is synced, or quiet, or halts: a caller-owned
    /// loop, driven on whatever runtime the caller has. Nothing is spawned.
    ///
    /// It returns `Ok` after a step that left the edge
    /// [synced](super::SyncStatus::is_synced), or that left it quiet: the step
    /// pushed nothing, ingested nothing it lacked, and did not read through
    /// echoes toward something waiting. So a far side that cannot say
    /// [`Waiting::Nothing`] still lets the run end.
    /// Call it again to sync again; how often is the caller's.
    ///
    /// Echoes are this edge's own events coming back, as they do on a pull from
    /// the start of the far side's log. A page of them moves nothing, yet the
    /// run reads on while the far side's
    /// [`Waiting::Events`] count falls, because what is
    /// waiting behind them may be new. A count that holds or rises is no
    /// progress, so a far side stuck on one page cannot hold the run.
    ///
    /// After a [`Transient`](FailureClass::Transient) failure it waits the
    /// backoff's next delay through `sleep` and steps again, and a successful
    /// step resets the backoff. After a [`Split`](FailureClass::Split) the edge
    /// could halve, it steps again at once, with the smaller batch.
    ///
    /// Every pass of the loop either moved an event, lowered the far side's
    /// waiting count, halved the batch, or spent a backoff attempt, so the run
    /// ends; and it can be dropped between
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
        // The far side's last `Events` count, which a page of echoes must
        // lower to count as progress. Starting from the edge's status, so a run
        // called again against a stuck far side ends at its first step.
        let mut waiting_before = events_waiting(self.status.waiting);
        loop {
            // The error lives only inside this `match`, which ends before the
            // sleep's await: a `!Send` error (ADR-0009 leaves it unbounded)
            // must not sit in the future across that await, or a run over
            // `Send`-flavour ports could not be spawned. An explicit `drop`
            // before the await was not enough for the compiler's analysis;
            // `tests/flavours.rs`'s `edge_runs_from_generic` is what found it.
            let delay = match self.step(peer, store).await {
                Ok(stepped) => {
                    backoff.reset();
                    let waiting_now = events_waiting(self.status.waiting);
                    let drained = read_through_echoes(&stepped, waiting_before, waiting_now);
                    waiting_before = waiting_now;
                    if self.status.is_synced() || (moved_nothing_new(&stepped) && !drained) {
                        return Ok(());
                    }
                    continue;
                }
                Err(error) => {
                    if self.status.state != EdgeState::Running {
                        return Err(error);
                    }
                    match error.class() {
                        FailureClass::Transient => match backoff.next_delay() {
                            Some(delay) => delay,
                            None => return Err(error),
                        },
                        // The step halved the batch; the next one offers fewer
                        // groups.
                        FailureClass::Split => continue,
                        // The step parked or stopped the edge for these, so
                        // the state check above has already returned; kept
                        // total all the same.
                        FailureClass::Park | FailureClass::Stop => return Err(error),
                    }
                }
            };
            sleep.sleep(delay).await;
        }
    }
}

/// Pushed nothing, and ingested nothing the local store lacked.
const fn moved_nothing_new(stepped: &Stepped) -> bool {
    stepped.pushed == 0 && stepped.ingested.appended == 0 && stepped.ingested.compensated == 0
}

/// The count of a [`Waiting::Events`], and `None` for any other answer.
const fn events_waiting(waiting: Waiting) -> Option<NonZeroU64> {
    match waiting {
        Waiting::Events(count) => Some(count),
        Waiting::Nothing | Waiting::Unknown => None,
    }
}

/// The step skipped echoes and the far side's count of what waits behind them
/// fell: strictly below the last count, or first stated now. Strictly, so the
/// run ends against a far side whose count holds.
fn read_through_echoes(
    stepped: &Stepped,
    before: Option<NonZeroU64>,
    now: Option<NonZeroU64>,
) -> bool {
    stepped.ingested.skipped > 0
        && match (before, now) {
            (Some(before), Some(now)) => now < before,
            (None, Some(_)) => true,
            (_, None) => false,
        }
}
