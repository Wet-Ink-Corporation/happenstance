//! `NeonSyncPeer`: a [`SyncPeer`] whose far side is a Neon store reached over
//! its HTTP endpoint (#131).
//!
//! # One request per call
//!
//! This is the peer phase 13 names as the one that **cannot hold a
//! transaction across a round trip**: no connection, no interactive
//! transaction, no cursor. So each call is one SQL request, one transaction
//! on the server:
//!
//! * a **pull** is the group read and the identity read, two statements of one
//!   read-only `REPEATABLE READ` batch. The group read fetches one row past
//!   what its budget can carry and counts what it scanned, so `waiting` is
//!   decided inside the same statement as the page (SY-37), and the identity
//!   that names the responder (ADR-0104) is read in the same snapshot;
//! * a **push** is the ingest's three statements, then the watermark and the
//!   identity, in one `REPEATABLE READ` batch. The ack's `confirmed` and
//!   `responder` are read inside the transaction that wrote the push, and
//!   arrive only if it committed. A push with nothing foreign in it writes
//!   nothing and is one read-only request of the last two.
//!
//! # Why not `StoreSyncPeer` over this store
//!
//! `NeonEventStore` already implements everything
//! [`StoreSyncPeer`](happenstance_sync::StoreSyncPeer) needs, and that peer
//! meets SY-37 over it: `read_groups` decides `waiting` in its own statement.
//! It lost on the round-trip count. It asks the store one question per port
//! method, so a pull costs two HTTP round trips (the page, then the identity)
//! and a push three (the ingest, the watermark, the identity), and the
//! identity is read in a different snapshot from the page it labels. On this
//! transport a round trip is the expensive unit (E2E-36), so the peer fuses
//! the same statements into one request rather than inventing new SQL: the
//! group read, the ingest, the watermark and the identity read are the store's
//! own, shared with its port implementations.
//!
//! # Behaviour shared with `StoreSyncPeer`
//!
//! The token is [`StoreResume`], the far store's own position, which is
//! sound for the reason it is there: a token goes back only to the peer that
//! minted it (SY-15), and this peer reads the store that numbered it. Its
//! text form is `StoreSyncPeer`'s. A page is whole write groups, one [`EventGroup`] each, and a
//! group the budget cannot carry waits whole for the next pull (D-2). A push
//! ingests with no compensation, so this peer cannot be SY-7's adjudicator,
//! and it declares and enforces the [`PeerLimits`] it was built with.

use core::num::NonZeroUsize;

use happenstance_core::EventId;
use happenstance_sync::{
    Ack, Classify, EventGroup, FailureClass, IngestError, IngestGroup, Ingested, PeerLimits,
    PullBatchLimit, Pulled, PushBatch, StoreResume, SyncPeer, Waiting,
};

use super::ingest::{identity_from, ingest_body, ingested_from, watermark_from};
use super::write_group::{decode_group_page, group_budget};
use super::{NeonEventStore, as_i64, decode_body};
use crate::error::NeonError;
use crate::transport::{IsolationLevel, SqlRequest, SqlTransport};
use crate::wire::ResultSet;

/// A [`SyncPeer`] over a [`NeonEventStore`]: one SQL request per pull and one
/// per push.
///
/// # Limits
///
/// [`new`](Self::new) declares [`PeerLimits::minimum`], which sits inside the
/// store's own ceilings ([`NeonEventStore::MAX_EVENTS_PER_BATCH`] events and
/// [`NeonEventStore::MAX_EVENT_DATA_LEN`] bytes an event). A declaration above
/// those ceilings admits pushes the ingest then refuses, which is the failure
/// SY-18 exists to move ahead of the push. A pull reads up to one more group
/// than `max_batch_events` in one response, bounded by the store's
/// `max_response_bytes`; a page of large groups that overflows it fails the
/// pull as [`NeonError::ResponseTooLarge`], so declare limits whose page fits.
///
/// # Send
///
/// The bare [`SyncPeer`] only, as the store is bare-only: a `wasm32`
/// transport need not be `Send`. Bind it as `SyncPeer` in generic code
/// (constraint 4).
///
/// # Examples
///
/// ```
/// use happenstance_neon::{NeonConfig, NeonEventStore, NeonSyncPeer, NullTransport};
/// use happenstance_neon::happenstance_sync::{PeerLimits, SyncPeer};
///
/// let store = NeonEventStore::new(NullTransport::new(), NeonConfig::default());
/// let peer = NeonSyncPeer::new(store);
/// assert_eq!(peer.limits(), PeerLimits::minimum());
/// ```
#[derive(Debug, Clone)]
pub struct NeonSyncPeer<T> {
    store: NeonEventStore<T>,
    limits: PeerLimits,
}

impl<T> NeonSyncPeer<T> {
    /// A peer over `store`, declaring [`PeerLimits::minimum`].
    #[must_use]
    pub const fn new(store: NeonEventStore<T>) -> Self {
        Self::with_limits(store, PeerLimits::minimum())
    }

    /// A peer over `store` that declares, and enforces, `limits`.
    #[must_use]
    pub const fn with_limits(store: NeonEventStore<T>, limits: PeerLimits) -> Self {
        Self { store, limits }
    }

    /// The store this peer reads and writes.
    #[must_use]
    pub const fn store(&self) -> &NeonEventStore<T> {
        &self.store
    }

    /// Refuses a push the declared limits do not admit, before anything is
    /// sent: an event that can never land first, then the batch's count and
    /// bytes, which a split can cure. `StoreSyncPeer`'s order.
    fn admit<E>(&self, batch: &PushBatch) -> Result<(), NeonPeerError<E>> {
        let limits = &self.limits;
        if let Some(event) = batch
            .events()
            .find(|event| event.payload_len() > limits.max_event_bytes)
        {
            return Err(NeonPeerError::EventTooLarge {
                id: event.id,
                len: event.payload_len(),
                limit: limits.max_event_bytes,
            });
        }
        let count = batch.events().count();
        if count > limits.max_batch_events {
            return Err(NeonPeerError::TooManyEvents {
                actual: count,
                limit: limits.max_batch_events,
            });
        }
        let bytes = batch.payload_len();
        if bytes > limits.max_batch_bytes {
            return Err(NeonPeerError::BatchTooLarge {
                actual: bytes,
                limit: limits.max_batch_bytes,
            });
        }
        Ok(())
    }
}

impl<T: SqlTransport> SyncPeer for NeonSyncPeer<T> {
    type Error = NeonPeerError<T::Error>;
    type Resume = StoreResume;

    /// Reads the store's write groups above `from`, each whole, until the
    /// declared budget is spent or the log ends, and names the store that
    /// answered: one read-only request.
    ///
    /// # Errors
    ///
    /// [`NeonPeerError::Read`] for any failure of the request or of its
    /// answer; nothing was forwarded, and the same pull can be retried.
    async fn pull(&self, from: Option<&StoreResume>) -> Result<Pulled<StoreResume>, Self::Error> {
        let from = from.copied().unwrap_or_default();
        // One past the most events a page can carry: the group that does not
        // fit is the proof the log goes on (SY-37). Total: a declared
        // `usize::MAX` saturates rather than wraps.
        let probe = NonZeroUsize::MIN.saturating_add(self.limits.max_batch_events);
        let after = from.last_forwarded().map_or(0, as_i64);
        let request = SqlRequest::batch(
            vec![
                self.store.groups_statement(after, group_budget(probe)),
                self.store.identity_statement(),
            ],
            IsolationLevel::RepeatableRead,
        )
        .read_only();
        let response = self
            .store
            .transport()
            .round_trip(request)
            .await
            .map_err(|error| NeonPeerError::Read(NeonError::Transport(error)))?;
        let body = decode_body::<T::Error>(&response, self.store.config().max_response_bytes)
            .map_err(NeonPeerError::Read)?;
        let [page, identity] = last_two(body.result_sets(), 2).map_err(NeonPeerError::Read)?;
        let page = decode_group_page(page, probe).map_err(NeonPeerError::Read)?;
        let responder = identity_from(identity).map_err(NeonPeerError::Read)?;

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
        Ok(Pulled::new(PushBatch::new(groups), resume)
            .with_waiting(waiting)
            .with_responder(responder))
    }

    /// Ingests each pushed group as one atomic group with no compensation,
    /// and acknowledges with the watermark and the identity read in the same
    /// transaction: one request.
    ///
    /// The travelling guards are not evaluated (SY-1): each is stored with its
    /// group and forwarded on pull (SY-30).
    ///
    /// # Errors
    ///
    /// [`NeonPeerError::EventTooLarge`], [`TooManyEvents`](NeonPeerError::TooManyEvents)
    /// or [`BatchTooLarge`](NeonPeerError::BatchTooLarge) for a push the
    /// declared limits do not admit, with nothing sent;
    /// [`NeonPeerError::Ingest`] if the store refuses or fails the request,
    /// which then wrote nothing, save the unknown outcome of a request that
    /// got no answer ([`IngestError::Store`]); and [`NeonPeerError::Answer`]
    /// if the request committed and its confirmation could not be read, in
    /// which case re-delivery is a skip.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.admit(batch)?;
        let groups: Vec<IngestGroup<'_>> = batch
            .groups
            .iter()
            .map(|group| IngestGroup::new(&group.events, &[]).with_guard(group.guard.as_ref()))
            .collect();
        let write = self
            .store
            .ingest_write(&groups)
            .map_err(NeonPeerError::Ingest)?;
        let confirm = [
            self.store.watermark_statement(),
            self.store.identity_statement(),
        ];
        let (request, written) = match write {
            Some(mut request) => {
                let written = request.statements.len();
                request.statements.extend(confirm);
                // ES-11, as `append` and `ingest` wait: this is a write.
                self.store.transport().reads_settled().await;
                (request, written)
            }
            None => (
                SqlRequest::batch(confirm.into(), IsolationLevel::RepeatableRead).read_only(),
                0,
            ),
        };
        let response = self
            .store
            .transport()
            .round_trip(request)
            .await
            .map_err(|error| {
                NeonPeerError::Ingest(IngestError::Store(NeonError::Transport(error)))
            })?;
        let body = ingest_body(&response, self.store.config().max_response_bytes, &groups)
            .map_err(NeonPeerError::Ingest)?;
        // The request committed, so an answer that cannot be read from here
        // on is `Answer`: re-delivery is a skip.
        let sets = body.result_sets();
        let [watermark, identity] =
            last_two(sets, written.saturating_add(2)).map_err(NeonPeerError::Answer)?;
        // A count the answer lacks is such an answer, not the ingest's
        // `Store`, whose outcome is unknown.
        let ingested = match sets.first() {
            Some(counts) if written > 0 => ingested_from(counts).map_err(|error| match error {
                IngestError::Store(error) => NeonPeerError::Answer(error),
                other => NeonPeerError::Ingest(other),
            })?,
            _ => Ingested::default(),
        };
        let mut ack = Ack::default();
        ack.appended = saturating_u64(ingested.appended);
        ack.skipped = saturating_u64(ingested.skipped);
        ack.confirmed = watermark_from(watermark).map_err(NeonPeerError::Answer)?;
        Ok(ack.with_responder(identity_from(identity).map_err(NeonPeerError::Answer)?))
    }

    fn limits(&self) -> PeerLimits {
        self.limits.clone()
    }
}

/// The last two of exactly `expected` result sets, which is how a request of
/// `expected` statements ending in two reads is answered.
///
/// # Errors
///
/// [`NeonError::ResultSets`] for any other count, naming both: never a
/// lenient read of whatever sets sit at the end.
fn last_two<E>(sets: &[ResultSet], expected: usize) -> Result<[&ResultSet; 2], NeonError<E>> {
    let shape = || NeonError::ResultSets {
        expected,
        actual: sets.len(),
    };
    if sets.len() != expected {
        return Err(shape());
    }
    match sets {
        [.., second_last, last] => Ok([second_last, last]),
        _ => Err(shape()),
    }
}

/// A count for the wire, which carries `u64`. Saturating, and only in theory:
/// no target this crate builds for has a `usize` wider than 64 bits.
fn saturating_u64(count: usize) -> u64 {
    u64::try_from(count).unwrap_or(u64::MAX)
}

/// How a [`NeonSyncPeer`] fails: the store's error, the ingest's refusal, or
/// a push over the limits it declared.
///
/// Its own type rather than `happenstance-sync`'s `StorePeerError`, whose
/// limit variants are `#[non_exhaustive]` and so cannot be built outside that
/// crate. The arms mirror it, and so do their classes (SY-36). `E` is the
/// transport's error.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NeonPeerError<E> {
    /// The pull's one request, the group read and the identity read together,
    /// failed, or its answer could not be read: the page, `waiting` or the
    /// responder. One arm for both reads, where `StorePeerError` has `Read`
    /// and `Identity`, because here they are one request and fail together.
    /// Nothing was forwarded.
    #[error("the Neon store's pull request, or its answer, failed")]
    Read(#[source] NeonError<E>),

    /// The push's request was refused or failed. Nothing in the push was
    /// written, save the unknown outcome [`IngestError::Store`] describes for
    /// a request that got no answer.
    #[error(transparent)]
    Ingest(IngestError<NeonError<E>>),

    /// The push's request committed and the watermark or the identity in its
    /// answer could not be read. Re-delivering the push converges: every
    /// event in it is now a skip.
    #[error("the Neon store's answer to a committed push could not be read")]
    Answer(#[source] NeonError<E>),

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
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
impl<E> Classify for NeonPeerError<E> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Read(error) | Self::Answer(error) => error.class(),
            Self::Ingest(error) => error.class(),
            Self::EventTooLarge { .. } => FailureClass::Park,
            Self::TooManyEvents { .. } | Self::BatchTooLarge { .. } => FailureClass::Split,
        }
    }
}
