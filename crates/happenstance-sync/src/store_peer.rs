//! A [`SyncPeer`] whose far side is a store the caller holds.
//!
//! # What it is for
//!
//! [`StoreSyncPeer`] is the far side of a network peer with the network taken
//! out: a pull reads the store, a push ingests into it. A server mounts it
//! behind [`serve_sync`](crate::serve_sync) — the Durable Object over its
//! SQLite, a Postgres-backed service over its pool — and a client reaches it
//! through [`RemoteSyncPeer`](crate::RemoteSyncPeer). In-process, it is also a
//! peer in its own right: two stores in one program replicate through it with
//! no transport at all.
//!
//! # The token is the far store's own position
//!
//! [`StoreResume`] carries the last
//! [`SequencePosition`](happenstance_core::SequencePosition) a pull forwarded,
//! and the next pull reads above it. A position is meaningful only inside the
//! store that assigned it, which is exactly where it goes: a token is handed
//! back only to the peer that minted it (SY-15), and that peer reads the store
//! that numbered it. Nothing here can tell a token minted by a *different*
//! store peer from one minted by this one — both are numbers — so a caller that
//! swaps the far side under a saved token resumes at the wrong place. That is
//! the caller's to prevent, the same way it is for any opaque token.
//!
//! # Groups: one page is one group
//!
//! [`EventStore::read`] yields
//! [`SequencedEvent`](happenstance_core::SequencedEvent)s, and nothing on one
//! says which `append` or [`ingest`](IngestStore::ingest) group it was written
//! in: **the boundaries the far store wrote are not recoverable through the
//! port** (#420). So a pull hands back its whole page as **one** [`EventGroup`].
//! Every origin group that fits inside one page arrives whole, and the reader
//! ingests the page atomically.
//!
//! **A group that crosses a page boundary is split.** When the budget ends
//! inside a group the far store wrote, the page carries its head and the next
//! pull its tail. That includes a losing event and the compensation written
//! beside it (SY-2): a relay that pulls from this peer can deliver the two in
//! separate pages. Keeping such a group whole needs the far store to expose its
//! write boundaries, which is #420.
//!
//! The pulled group's [`guard`](EventGroup::guard) is `None`. No store keeps
//! the condition an event was appended under, so there is nothing to forward;
//! the guard is evidence (SY-6), and its absence costs an adjudicator that
//! evidence and nothing else.
//!
//! # Waiting
//!
//! A pull reads one event past what its budget can carry, so it knows in the
//! same call whether it reached the end of the far log. It says
//! [`Waiting::Nothing`] only when the read ended with every event sent, and
//! [`Waiting::Unknown`] otherwise. It never counts: an exact count is a scan of
//! the rest of the log on every pull.

use alloc::vec;
use alloc::vec::Vec;
use core::pin::Pin;

use happenstance_core::futures_core::Stream;
use happenstance_core::{EventId, EventStore, Query, ReadOptions};

use crate::classify::{Classify, FailureClass};
use crate::identity::ReplicatedEvent;
use crate::ingest::{IngestError, IngestGroup, IngestStore};
use crate::peer::{
    Ack, EventGroup, PeerLimits, PullBatchLimit, Pulled, PushBatch, SyncPeer, Waiting,
};

/// A peer over a store the caller holds: `events` to read, `ingest` to write.
///
/// Two handles rather than one value, because an adapter's read half and its
/// ingest half may be separate handles onto one store — and **they must be the
/// same store**. A peer whose pushes land somewhere its pulls never read
/// acknowledges events it will never forward.
///
/// # It never adjudicates
///
/// A push ingests with **no compensation**: the store peer only replicates.
/// A hub served by this peer, directly or behind
/// [`serve_sync`](crate::serve_sync), therefore cannot author a compensation,
/// and cannot be the peer SY-7 makes the adjudicator. An adjudication hook is
/// planned with SY-7 (#136).
///
/// # Limits
///
/// The [`PeerLimits`] it declares are the ones it enforces on a push and fills
/// a pull to. Declare no more than the store accepts: an event the declaration
/// admits and the store's ingest refuses is the failure SY-18 exists to move
/// ahead of the push.
///
/// The declaration binds pulls too, and that includes events the far store
/// appended locally, which no push limit checked. An event at the head of the
/// unread log whose payload is over the declared
/// [`max_event_bytes`](PeerLimits::max_event_bytes) cannot travel in any page:
/// every pull from there on returns an empty page, [`Waiting::Unknown`] and the
/// token it was given, and replication **stalls at that event** until the far
/// side declares a larger limit. Declare at least the largest payload the
/// store's own writers accept.
///
/// # Send
///
/// Implements the bare [`SyncPeer`], so a `!Send` store, such as the Durable
/// Object's, is a valid far side. Bind it as `SyncPeer` in generic code
/// (RS-20-2). For concrete `Send` stores its futures are `Send` too, but the
/// type does not implement [`SendSyncPeer`](crate::SendSyncPeer), and
/// coherence forbids adding it beside the bare impl: `trait_variant`'s
/// blanket impl already gives every `SendSyncPeer` the bare trait. A runner that spawns and binds
/// `SendSyncPeer` needs a separate Send-flavour type, which can be added later
/// without a break (#421).
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "memory"))]
/// # fn main() {}
/// # #[cfg(feature = "memory")]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
/// use happenstance_sync::{
///     EventGroup, MemoryIngestStore, PushBatch, ReplicatedEvent, StoreSyncPeer, SyncPeer,
///     Waiting,
/// };
///
/// // The far side's store; `&MemoryIngestStore` is not itself a store, so the
/// // example hands the peer a handle type of its own.
/// let hub = Shared(std::sync::Arc::new(MemoryIngestStore::new()));
/// let peer = StoreSyncPeer::new(hub.clone(), hub);
///
/// let tablet = StoreId::from_bytes([1; 16]);
/// let reading = ReplicatedEvent::new(
///     EventId::new(tablet, SequencePosition::FIRST),
///     RecordedAt::from_millis(1_700_000_000_000),
///     Event::new("ReadingTaken", &b"{\"celsius\":4}"[..])?,
/// );
/// let batch = PushBatch::new(vec![EventGroup::new(None, vec![reading])]);
///
/// assert_eq!(peer.push(&batch).await?.appended, 1);
/// let pulled = peer.pull(None).await?;
/// assert_eq!(pulled.batch, batch);
/// assert_eq!(pulled.waiting, Waiting::Nothing);
/// # Ok(())
/// # }
/// # #[cfg(feature = "memory")]
/// # #[derive(Clone)]
/// # struct Shared(std::sync::Arc<happenstance_sync::MemoryIngestStore>);
/// # #[cfg(feature = "memory")]
/// # mod shared {
/// #     use happenstance_core::*;
/// #     use happenstance_core::futures_core::Stream;
/// #     use happenstance_sync::*;
/// #     impl SendEventStore for super::Shared {
/// #         type Error = MemoryStoreError;
/// #         fn read(&self, query: &Query, options: ReadOptions)
/// #             -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
/// #             SendEventStore::read(&*self.0, query, options)
/// #         }
/// #         async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
/// #             -> Result<SequencePosition, AppendError<Self::Error>> {
/// #             SendEventStore::append(&*self.0, events, condition).await
/// #         }
/// #         async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
/// #             SendEventStore::head(&*self.0).await
/// #         }
/// #         async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
/// #             SendEventStore::contains_event_id(&*self.0, id).await
/// #         }
/// #     }
/// #     impl SendIngestStore for super::Shared {
/// #         type Error = MemoryStoreError;
/// #         async fn store_id(&self) -> Result<StoreId, Self::Error> {
/// #             SendIngestStore::store_id(&*self.0).await
/// #         }
/// #         async fn ingest(&self, groups: &[IngestGroup<'_>])
/// #             -> Result<Ingested, IngestError<Self::Error>> {
/// #             SendIngestStore::ingest(&*self.0, groups).await
/// #         }
/// #         async fn watermark(&self) -> Result<Watermark, Self::Error> {
/// #             SendIngestStore::watermark(&*self.0).await
/// #         }
/// #     }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct StoreSyncPeer<S, I> {
    events: S,
    ingest: I,
    limits: PeerLimits,
}

impl<S, I> StoreSyncPeer<S, I> {
    /// A peer over `events` and `ingest`, declaring
    /// [`PeerLimits::minimum`].
    #[must_use]
    pub const fn new(events: S, ingest: I) -> Self {
        Self::with_limits(events, ingest, PeerLimits::minimum())
    }

    /// A peer over `events` and `ingest` that declares, and enforces, `limits`.
    #[must_use]
    pub const fn with_limits(events: S, ingest: I, limits: PeerLimits) -> Self {
        Self {
            events,
            ingest,
            limits,
        }
    }
}

impl<S, I> StoreSyncPeer<S, I>
where
    S: EventStore,
    I: IngestStore,
{
    /// Refuses a push the declared limits do not admit, before anything is
    /// written: an event that can never land first, then the batch's count and
    /// bytes, which a split can cure.
    fn admit(&self, batch: &PushBatch) -> Result<(), StorePeerError<S::Error, I::Error>> {
        let limits = &self.limits;
        if let Some(event) = batch
            .events()
            .find(|event| event.payload_len() > limits.max_event_bytes)
        {
            return Err(StorePeerError::EventTooLarge {
                id: event.id,
                len: event.payload_len(),
                limit: limits.max_event_bytes,
            });
        }
        let count = batch.events().count();
        if count > limits.max_batch_events {
            return Err(StorePeerError::TooManyEvents {
                actual: count,
                limit: limits.max_batch_events,
            });
        }
        let bytes = batch.payload_len();
        if bytes > limits.max_batch_bytes {
            return Err(StorePeerError::BatchTooLarge {
                actual: bytes,
                limit: limits.max_batch_bytes,
            });
        }
        Ok(())
    }
}

impl<S, I> SyncPeer for StoreSyncPeer<S, I>
where
    S: EventStore,
    S::Error: Classify,
    I: IngestStore,
{
    type Error = StorePeerError<S::Error, I::Error>;
    type Resume = StoreResume;

    /// Reads the far store above `from`, one event at a time, until the
    /// declared budget is spent or the log ends.
    ///
    /// # Errors
    ///
    /// [`StorePeerError::Read`] if the store's read fails.
    async fn pull(&self, from: Option<&StoreResume>) -> Result<Pulled<StoreResume>, Self::Error> {
        let from = from.copied().unwrap_or_default();
        let Some(start) = from.next_position() else {
            // The token names the last position a store can number, so nothing
            // can lie above it.
            return Ok(Pulled::new(PushBatch::empty(), from).with_waiting(Waiting::Nothing));
        };

        // One past the most events a page can carry: the event that does not
        // fit is the proof the log goes on, read in the same call (SY-37).
        let probe = self.limits.max_batch_events.saturating_add(1);
        let query = Query::all();
        let mut stream = core::pin::pin!(
            self.events
                .read(&query, ReadOptions::new().from(start).limit(probe))
        );
        let mut budget = PullBatchLimit::from(&self.limits);
        let mut page = Vec::new();
        let mut resume = from;

        let waiting = loop {
            let Some(next) = next_item(stream.as_mut()).await else {
                break Waiting::Nothing;
            };
            let sequenced = next.map_err(StorePeerError::Read)?;
            let position = sequenced.position;
            let single = EventGroup::new(
                None,
                vec![ReplicatedEvent::new(
                    sequenced.id,
                    sequenced.recorded_at,
                    sequenced.event,
                )],
            );
            if !budget.admits(&single) {
                break Waiting::Unknown;
            }
            budget.charge(&single);
            page.extend(single.events);
            resume = StoreResume::after(position);
        };

        let groups = if page.is_empty() {
            Vec::new()
        } else {
            vec![EventGroup::new(None, page)]
        };
        Ok(Pulled::new(PushBatch::new(groups), resume).with_waiting(waiting))
    }

    /// Ingests each pushed group as one atomic group with no compensation, and
    /// acknowledges with the store's watermark after the write.
    ///
    /// The travelling guards are not evaluated (SY-1) and not stored: no store
    /// keeps one. Nothing here detects a conflict or writes a compensation, so
    /// this peer cannot be SY-7's adjudicator; the hook for that is planned
    /// with SY-7 (#136).
    ///
    /// # Errors
    ///
    /// [`StorePeerError::EventTooLarge`], [`TooManyEvents`](StorePeerError::TooManyEvents)
    /// or [`BatchTooLarge`](StorePeerError::BatchTooLarge) for a push the
    /// declared limits do not admit, with nothing written;
    /// [`StorePeerError::Ingest`] if the store refuses or fails the write; and
    /// [`StorePeerError::Watermark`] if the write landed and the confirmation
    /// could not be read, in which case re-delivery is a skip.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.admit(batch)?;
        let groups: Vec<IngestGroup<'_>> = batch
            .groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]))
            .collect();
        let ingested = self
            .ingest
            .ingest(&groups)
            .await
            .map_err(StorePeerError::Ingest)?;
        let confirmed = self
            .ingest
            .watermark()
            .await
            .map_err(StorePeerError::Watermark)?;
        Ok(Ack {
            appended: saturating_u64(ingested.appended),
            skipped: saturating_u64(ingested.skipped),
            confirmed,
        })
    }

    fn limits(&self) -> PeerLimits {
        self.limits.clone()
    }
}

/// The next item of `stream`, without a `StreamExt` dependency.
async fn next_item<St: Stream>(mut stream: Pin<&mut St>) -> Option<St::Item> {
    core::future::poll_fn(|cx| stream.as_mut().poll_next(cx)).await
}

/// A count for the wire, which carries `u64`. Saturating, and only in theory:
/// no target this crate builds for has a `usize` wider than 64 bits.
fn saturating_u64(count: usize) -> u64 {
    u64::try_from(count).unwrap_or(u64::MAX)
}

mod resume;
pub use resume::{ForeignToken, StoreResume};

/// How a [`StoreSyncPeer`] fails: its store's read error `R`, its ingest
/// error `W`, or a push over the limits it declared.
///
/// The store's errors are type parameters, never strings (RS-30-2), and each
/// states its own class (SY-36). No bounds on the enum, so it is `Send` exactly
/// when both store errors are (ADR-0009).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StorePeerError<R, W> {
    /// The far store's read failed; the store's error states the class.
    #[error("the far store's read failed")]
    Read(#[source] R),

    /// The far store refused or failed the ingest. Nothing in the push was
    /// written, save the unknown outcome [`IngestError::Store`] describes.
    #[error(transparent)]
    Ingest(IngestError<W>),

    /// The ingest landed and the watermark to confirm it could not be read.
    /// Re-delivering the push converges: every event in it is now a skip.
    #[error("the far store's watermark could not be read after the ingest")]
    Watermark(#[source] W),

    /// One event is larger than the declared
    /// [`max_event_bytes`](PeerLimits::max_event_bytes), so no split can carry
    /// it: [`Park`](FailureClass::Park).
    #[error(
        "event {id} carries {len} payload bytes, over this peer's declared max_event_bytes of \
         {limit}"
    )]
    #[non_exhaustive]
    EventTooLarge {
        /// The first such event in the push.
        id: EventId,
        /// Its payload bytes.
        len: usize,
        /// What this peer declared.
        limit: usize,
    },

    /// The push carries more events than the declared
    /// [`max_batch_events`](PeerLimits::max_batch_events):
    /// [`Split`](FailureClass::Split).
    #[error("push of {actual} events exceeds this peer's declared max_batch_events of {limit}")]
    #[non_exhaustive]
    TooManyEvents {
        /// What was offered.
        actual: usize,
        /// What this peer declared.
        limit: usize,
    },

    /// The push carries more payload bytes than the declared
    /// [`max_batch_bytes`](PeerLimits::max_batch_bytes):
    /// [`Split`](FailureClass::Split).
    #[error(
        "push of {actual} payload bytes exceeds this peer's declared max_batch_bytes of {limit}"
    )]
    #[non_exhaustive]
    BatchTooLarge {
        /// What was offered.
        actual: usize,
        /// What this peer declared.
        limit: usize,
    },
}

/// The store's arms ask the store's error; the limit arms are fixed by the arm
/// (SY-36).
impl<R: Classify, W: Classify> Classify for StorePeerError<R, W> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Read(error) => error.class(),
            Self::Ingest(error) => error.class(),
            Self::Watermark(error) => error.class(),
            Self::EventTooLarge { .. } => FailureClass::Park,
            Self::TooManyEvents { .. } | Self::BatchTooLarge { .. } => FailureClass::Split,
        }
    }
}
