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
//! [`StoreResume`] carries the [`first`](crate::WriteGroup::first)
//! [`SequencePosition`](happenstance_core::SequencePosition) of the last group a
//! pull forwarded, and the next pull reads the groups above it. A position is
//! meaningful only inside the
//! store that assigned it, which is exactly where it goes: a token is handed
//! back only to the peer that minted it (SY-15), and that peer reads the store
//! that numbered it. Nothing here can tell a token minted by a *different*
//! store peer from one minted by this one — both are numbers — so a caller that
//! swaps the far side under a saved token resumes at the wrong place. That is
//! the caller's to prevent, the same way it is for any opaque token.
//!
//! # Groups: one write is one group
//!
//! A pull reads the far store through
//! [`read_groups`](WriteGroupStore::read_groups) and hands back **one**
//! [`EventGroup`] per write group the store wrote: an `append`, or an ingested
//! group with the compensation written beside it (SY-2). Each arrives whole,
//! and an ingested group carries the [`guard`](EventGroup::guard) it was pushed
//! with, so a relay forwards the origin's evidence (SY-6, SY-30). A page never
//! splits a group and never merges two: a group that would take the page past
//! its budget waits, whole, for the next pull. Rows the store wrote before it
//! recorded groups come back one group each, without a guard.
//!
//! # Waiting
//!
//! A pull reads one event past what its budget can carry, so it knows in the
//! same call whether it reached the end of the far log. It says
//! [`Waiting::Nothing`] only when the read ended with every group sent, and
//! [`Waiting::Unknown`] otherwise. It never counts: an exact count is a scan of
//! the rest of the log on every pull.
//!
//! # The responder
//!
//! Every [`Pulled`] and every [`Ack`] names the store that answered
//! (ADR-0104): the ingest half's [`store_id`](IngestStore::store_id), read
//! after the page or after the watermark. A runner that sees it change between
//! two answers knows the far store was restored or re-minted (VT-6).
//!
//! Read after, not before: a re-mint that lands between the page and the read
//! labels an old incarnation's page with the new identity. That errs toward
//! stopping the edge, which is the safe direction for SY-34; read first, the
//! same race would label the new incarnation's page with the old identity, and
//! the edge would carry on.
//!
//! **It is read on every call, and never cached.** A cached identity is the
//! one a re-mint does not reach, which is the change the field exists to
//! report. The cost is one more read of the store per pull and per push. On a
//! store that keeps its identity in memory that is a field load; on a SQL
//! store it is one indexed read of a one-row table; on Neon, whose store reads
//! its meta row on every call (ADR-0086), it is one more read-only HTTP round
//! trip **to the database**. It adds none on the sync wire: a remote pull or
//! push is still one exchange, and the identity travels in its reply.

use alloc::vec::Vec;
use core::num::NonZeroUsize;

use happenstance_core::{EventId, StoreId};

use crate::classify::{Classify, FailureClass};
use crate::ingest::{IngestError, IngestGroup, IngestStore};
use crate::peer::{
    Ack, EventGroup, PeerLimits, PullBatchLimit, Pulled, PushBatch, SyncPeer, Waiting,
};
use crate::write_group::WriteGroupStore;

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
/// The declaration binds pulls too, and that includes groups the far store
/// appended locally, which no push limit checked. A group at the head of the
/// unread log that the declaration cannot carry whole — an event over the
/// declared [`max_event_bytes`](PeerLimits::max_event_bytes), or more events or
/// bytes than one batch takes — cannot travel in any page, because a group is
/// never split: every pull from there on returns an empty page,
/// [`Waiting::Unknown`] and the token it was given, and replication **stalls at
/// that group** until the far side declares larger limits. Declare
/// `max_batch_events` at least the store's per-call ceiling, and
/// `max_event_bytes` and `max_batch_bytes` at least the largest payload and the
/// largest write the store's own writers accept.
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
/// #     use happenstance_sync::*;
/// #     impl SendWriteGroupStore for super::Shared {
/// #         type Error = write_group::GroupOrderError;
/// #         async fn read_groups(&self, after: Option<SequencePosition>,
/// #             max_events: core::num::NonZeroUsize) -> Result<GroupPage, Self::Error> {
/// #             SendWriteGroupStore::read_groups(&*self.0, after, max_events).await
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
    S: WriteGroupStore,
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

    /// The ingest half's identity, read now and never cached, so a re-mint
    /// shows on the next answer (ADR-0104).
    async fn responder(&self) -> Result<StoreId, StorePeerError<S::Error, I::Error>> {
        self.ingest
            .store_id()
            .await
            .map_err(StorePeerError::Identity)
    }
}

impl<S, I> SyncPeer for StoreSyncPeer<S, I>
where
    S: WriteGroupStore,
    I: IngestStore,
{
    type Error = StorePeerError<S::Error, I::Error>;
    type Resume = StoreResume;

    /// Reads the far store's write groups above `from`, each whole, until the
    /// declared budget is spent or the log ends, and names the store that
    /// answered.
    ///
    /// # Errors
    ///
    /// [`StorePeerError::Read`] if the store's group read fails, and
    /// [`StorePeerError::Identity`] if its identity cannot be read; either way
    /// nothing was forwarded, and the same pull can be retried.
    async fn pull(&self, from: Option<&StoreResume>) -> Result<Pulled<StoreResume>, Self::Error> {
        let from = from.copied().unwrap_or_default();
        // One past the most events a page can carry: the group that does not
        // fit is the proof the log goes on, read in the same call (SY-37).
        // Total: a declared `usize::MAX` saturates rather than wraps.
        let probe = NonZeroUsize::MIN.saturating_add(self.limits.max_batch_events);
        let page = self
            .events
            .read_groups(from.last_forwarded(), probe)
            .await
            .map_err(StorePeerError::Read)?;

        let mut budget = PullBatchLimit::from(&self.limits);
        let mut groups = Vec::with_capacity(page.groups.len());
        let mut resume = from;
        let mut stalled = false;
        for group in page.groups {
            let first = group.first();
            let group = EventGroup::from(group);
            // Never split and never skipped: a group the budget cannot admit
            // ends the page, and one at the head stalls it (D-2).
            if !budget.admits(&group) {
                stalled = true;
                break;
            }
            budget.charge(&group);
            groups.push(group);
            resume = StoreResume::after(first);
        }
        let waiting = if stalled {
            Waiting::Unknown
        } else {
            page.waiting
        };
        // Read after the page, not before (ADR-0104): see the module docs.
        let responder = self.responder().await?;
        Ok(Pulled::new(PushBatch::new(groups), resume)
            .with_waiting(waiting)
            .with_responder(responder))
    }

    /// Ingests each pushed group as one atomic group with no compensation, and
    /// acknowledges with the store's watermark after the write.
    ///
    /// The travelling guards are not evaluated (SY-1): each is stored with its
    /// group and forwarded on pull (SY-30). Nothing here detects a conflict or
    /// writes a compensation, so this peer cannot be SY-7's adjudicator; the
    /// hook for that is planned with SY-7 (#136).
    ///
    /// # Errors
    ///
    /// [`StorePeerError::EventTooLarge`], [`TooManyEvents`](StorePeerError::TooManyEvents)
    /// or [`BatchTooLarge`](StorePeerError::BatchTooLarge) for a push the
    /// declared limits do not admit, with nothing written;
    /// [`StorePeerError::Ingest`] if the store refuses or fails the write; and
    /// [`StorePeerError::Watermark`] or [`StorePeerError::Identity`] if the
    /// write landed and the confirmation, or the identity that names the
    /// store which took it, could not be read; in either case re-delivery is a
    /// skip.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.admit(batch)?;
        let groups: Vec<IngestGroup<'_>> = batch
            .groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
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
        let responder = self.responder().await?;
        Ok(Ack {
            appended: saturating_u64(ingested.appended),
            skipped: saturating_u64(ingested.skipped),
            confirmed,
            responder: Some(responder),
        })
    }

    fn limits(&self) -> PeerLimits {
        self.limits.clone()
    }
}

/// A count for the wire, which carries `u64`. Saturating, and only in theory:
/// no target this crate builds for has a `usize` wider than 64 bits.
fn saturating_u64(count: usize) -> u64 {
    u64::try_from(count).unwrap_or(u64::MAX)
}

mod resume;
pub use resume::{ForeignToken, StoreResume};

/// How a [`StoreSyncPeer`] fails: its store's group-read error `R`, its ingest
/// error `W`, or a push over the limits it declared.
///
/// The store's errors are type parameters, never strings (RS-30-2), and each
/// states its own class (SY-36). No bounds on the enum, so it is `Send` exactly
/// when both store errors are (ADR-0009).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum StorePeerError<R, W> {
    /// The far store's group read failed; the store's error states the class.
    #[error("the far store's group read failed")]
    Read(#[source] R),

    /// The far store refused or failed the ingest. Nothing in the push was
    /// written, save the unknown outcome [`IngestError::Store`] describes.
    #[error(transparent)]
    Ingest(IngestError<W>),

    /// The ingest landed and the watermark to confirm it could not be read.
    /// Re-delivering the push converges: every event in it is now a skip.
    #[error("the far store's watermark could not be read after the ingest")]
    Watermark(#[source] W),

    /// The far store's identity, which every answer names (ADR-0104), could
    /// not be read. After a push the ingest has landed, and re-delivering it
    /// converges as for [`Watermark`](Self::Watermark); after a pull nothing
    /// was forwarded.
    #[error("the far store's identity could not be read")]
    Identity(#[source] W),

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
            Self::Watermark(error) | Self::Identity(error) => error.class(),
            Self::EventTooLarge { .. } => FailureClass::Park,
            Self::TooManyEvents { .. } | Self::BatchTooLarge { .. } => FailureClass::Split,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StorePeerError;
    use crate::{Classify, FailureClass};

    /// A store error that answers whatever class it was built with.
    #[derive(Debug)]
    struct Answers(FailureClass);

    impl Classify for Answers {
        fn class(&self) -> FailureClass {
            self.0
        }
    }

    /// An identity the store could not read is classed by the store (SY-36).
    /// Rejects an `Identity` arm fixed at one class: a dropped connection is
    /// `Transient`, a missing meta row is not.
    #[test]
    fn an_unread_identity_states_the_store_errors_class() {
        for class in [
            FailureClass::Transient,
            FailureClass::Split,
            FailureClass::Park,
            FailureClass::Stop,
        ] {
            let error = StorePeerError::<Answers, Answers>::Identity(Answers(class));

            assert_eq!(error.class(), class);
        }
    }
}
