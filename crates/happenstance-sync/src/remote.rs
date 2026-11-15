//! The client half of a network peer, over a transport the caller supplies.
//!
//! [`Exchange`] is the transport: one request's bytes out on a [`Route`], one
//! reply's bytes back. [`RemoteSyncPeer`] is the [`SyncPeer`] over it, encoding
//! and decoding the envelopes [`serve_sync`](crate::serve_sync) answers, and
//! refusing a push over the far side's declared limits before it is sent, in
//! the order [`StoreSyncPeer`](crate::StoreSyncPeer) refuses one. Nothing
//! here names HTTP: an HTTP client, a Durable Object stub, a socket, or a
//! function call into `serve_sync` in the same process are all an `Exchange`.

use alloc::vec::Vec;

use happenstance_core::EventId;

use crate::classify::{Classify, FailureClass};
use crate::peer::{Ack, LimitBreach, PeerLimits, Pulled, PushBatch, SyncPeer};
use crate::wire::{CodecError, PullRequest, ResumeToken, Route, decode, encode};

/// Sends one request on a route and returns the reply: one round trip.
///
/// This is the `!Send` flavour and the one to bind in generic code; a
/// transport that can cross threads implements [`SendExchange`] and gets this
/// for free (ADR-0001). The Durable Object's stub, which holds a `JsValue`,
/// cannot be `Send`, and a native HTTP client wants to be.
///
/// # What an implementation promises
///
/// * One round trip. The reply is the far side's answer to *this* request,
///   paired by the transport — the response to the request it sent, or the
///   message on the matching return route — never a buffer from another call.
/// * `route` reaches the far side, which passes it to
///   [`serve_sync`](crate::serve_sync) with the bytes (ADR-0097).
/// * A failure the far side reported, or the transport met, comes back as
///   [`Error`](Self::Error) stating its own [class](Classify) (SY-36). An HTTP
///   transport reads that class back from the status `serve_sync`'s error was
///   mapped to.
/// * The reply is bounded. A transport caps the reply body it will read at
///   about the declared [`max_batch_bytes`](PeerLimits::max_batch_bytes) plus
///   the envelope's overhead, and refuses a longer one rather than buffering
///   it: the bytes come from a peer, and [`RemoteSyncPeer`] decodes whatever
///   it is handed.
/// * The round trip is bounded in time. A transport puts a timeout on the
///   request and returns a failure when it expires, so a far side that never
///   answers cannot hold a pull or a push open forever.
#[trait_variant::make(SendExchange: Send)]
pub trait Exchange {
    /// How the transport fails, each failure stating its class. No `Send`
    /// bound, for the reason [`SyncPeer::Error`] has none.
    type Error: core::error::Error + Classify + 'static;

    /// Sends `request` on `route` and returns the reply's bytes.
    ///
    /// `request` is owned because the transport consumes it: an HTTP body or a
    /// message to post takes the buffer, and a borrow would make it copy.
    ///
    /// # Errors
    ///
    /// The transport's error, for a failure to deliver the request, a failure
    /// to read the reply, or a refusal the far side reported.
    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error>;
}

/// A [`SyncPeer`] whose far side is reached through an [`Exchange`].
///
/// Its [`Resume`](SyncPeer::Resume) is ADR-0097's text [`ResumeToken`], handed
/// back verbatim to the far side that minted it.
///
/// Implements the bare [`SyncPeer`], so a `!Send` exchange is a valid
/// transport. Over a concrete `Send` exchange its futures are `Send` too, but
/// the type does not implement [`SendSyncPeer`](crate::SendSyncPeer), and
/// coherence forbids adding it beside the bare impl: `trait_variant`'s
/// blanket impl already gives every `SendSyncPeer` the bare trait. Code that
/// spawns and binds `SendSyncPeer` takes
/// [`SendPeer::new(peer)`](crate::SendPeer) instead, over a
/// [`SendExchange`]` + Sync` (#421).
#[derive(Debug, Clone)]
pub struct RemoteSyncPeer<X> {
    exchange: X,
    limits: PeerLimits,
}

impl<X> RemoteSyncPeer<X> {
    /// A peer over `exchange`, declaring the far side's `limits`.
    ///
    /// The limits are taken here because [`limits`](SyncPeer::limits) is not
    /// `async` (SY-18): a runner reads them before a push, and cannot be made
    /// to wait a round trip for them. The four wire messages do not carry them
    /// (ADR-0097), so the transport brings them out of band: configuration
    /// shared by both ends, or a request of its own at connect time, before
    /// this is built — the far side's are its
    /// [`StoreSyncPeer`](crate::StoreSyncPeer)'s
    /// [`limits`](SyncPeer::limits). This peer refuses a push over `limits`
    /// before sending it, so declaring more than the far side accepts moves
    /// the refusal from here, before the push, to the far side, after it;
    /// declaring less only costs smaller batches.
    #[must_use]
    pub const fn new(exchange: X, limits: PeerLimits) -> Self {
        Self { exchange, limits }
    }

    /// The transport.
    #[must_use]
    pub const fn exchange(&self) -> &X {
        &self.exchange
    }

    /// The transport, handed back.
    #[must_use]
    pub fn into_exchange(self) -> X {
        self.exchange
    }
}

impl<X: Exchange> SyncPeer for RemoteSyncPeer<X> {
    type Error = RemoteError<X::Error>;
    type Resume = ResumeToken;

    /// One [`Route::Pull`] exchange.
    ///
    /// # Errors
    ///
    /// [`RemoteError::Exchange`] if the transport fails; [`RemoteError::Codec`]
    /// if the reply is not a pulled message this build reads.
    async fn pull(&self, from: Option<&ResumeToken>) -> Result<Pulled<ResumeToken>, Self::Error> {
        // The token is cloned into the request because the wire type owns it;
        // it is a short text, and the caller keeps theirs.
        let request = encode(
            Route::Pull,
            &PullRequest {
                from: from.cloned(),
            },
        )?;
        let reply = self
            .exchange
            .exchange(Route::Pull, request)
            .await
            .map_err(RemoteError::Exchange)?;
        Ok(decode(Route::Pull, &reply)?)
    }

    /// One [`Route::Push`] exchange, or none when the declared limits refuse
    /// the batch.
    ///
    /// # Errors
    ///
    /// [`RemoteError::EventTooLarge`], [`TooManyEvents`](RemoteError::TooManyEvents)
    /// or [`BatchTooLarge`](RemoteError::BatchTooLarge) for a push the declared
    /// limits do not admit, checked in that order and before anything is sent;
    /// [`RemoteError::Exchange`] if the transport fails, including a refusal
    /// the far side reported through it; [`RemoteError::Codec`] if the reply
    /// is not an ack this build reads.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        if let Some(breach) = self.limits.first_breach(batch) {
            return Err(refused(breach));
        }
        let request = encode(Route::Push, batch)?;
        let reply = self
            .exchange
            .exchange(Route::Push, request)
            .await
            .map_err(RemoteError::Exchange)?;
        Ok(decode(Route::Push, &reply)?)
    }

    fn limits(&self) -> PeerLimits {
        self.limits.clone()
    }
}

/// How a [`RemoteSyncPeer`] fails: its transport `E`, bytes it could not
/// write or read, or a push over the limits it declared, which it refuses
/// without sending. The limit arms mirror
/// [`StorePeerError`](crate::StorePeerError)'s, so a runner reads one shape.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RemoteError<E> {
    /// The exchange failed; the transport's error states the class, so a
    /// refusal the far side reported keeps the class it had there.
    #[error("the exchange failed")]
    Exchange(#[source] E),

    /// The request did not encode or the reply did not decode.
    /// [`Stop`](FailureClass::Stop).
    #[error(transparent)]
    Codec(#[from] CodecError),

    /// One event is larger than the
    /// [`max_event_bytes`](PeerLimits::max_event_bytes) this client declared
    /// for its far side, so no split can carry it:
    /// [`Park`](FailureClass::Park). Refused before anything was sent.
    #[error(
        "event {id} carries {len} payload bytes, over the max_event_bytes of {limit} declared \
         for the far side; not sent"
    )]
    #[non_exhaustive]
    EventTooLarge {
        /// The first such event in the push.
        id: EventId,
        /// Its payload bytes.
        len: usize,
        /// What was declared for the far side.
        limit: usize,
    },

    /// The push carries more events than the
    /// [`max_batch_events`](PeerLimits::max_batch_events) this client declared
    /// for its far side: [`Split`](FailureClass::Split). Refused before
    /// anything was sent.
    #[error(
        "push of {actual} events exceeds the max_batch_events of {limit} declared for the far \
         side; not sent"
    )]
    #[non_exhaustive]
    TooManyEvents {
        /// What was offered.
        actual: usize,
        /// What was declared for the far side.
        limit: usize,
    },

    /// The push carries more payload bytes than the
    /// [`max_batch_bytes`](PeerLimits::max_batch_bytes) this client declared
    /// for its far side: [`Split`](FailureClass::Split). Refused before
    /// anything was sent.
    #[error(
        "push of {actual} payload bytes exceeds the max_batch_bytes of {limit} declared for the \
         far side; not sent"
    )]
    #[non_exhaustive]
    BatchTooLarge {
        /// What was offered.
        actual: usize,
        /// What was declared for the far side.
        limit: usize,
    },
}

/// The refusal a push breaking `breach` is reported as. A function rather
/// than a `From` impl, because a public `From` from a crate-private type would
/// leak it.
const fn refused<E>(breach: LimitBreach) -> RemoteError<E> {
    match breach {
        LimitBreach::Event { id, len, limit } => RemoteError::EventTooLarge { id, len, limit },
        LimitBreach::Count { actual, limit } => RemoteError::TooManyEvents { actual, limit },
        LimitBreach::Bytes { actual, limit } => RemoteError::BatchTooLarge { actual, limit },
    }
}

/// The transport's and the codec's arms ask their errors; the limit arms are
/// fixed by the arm, as [`StorePeerError`](crate::StorePeerError)'s are (SY-36).
impl<E: Classify> Classify for RemoteError<E> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Exchange(error) => error.class(),
            Self::Codec(error) => error.class(),
            Self::EventTooLarge { .. } => FailureClass::Park,
            Self::TooManyEvents { .. } | Self::BatchTooLarge { .. } => FailureClass::Split,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::cell::Cell;

    use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};

    use super::{Exchange, RemoteError, RemoteSyncPeer};
    use crate::classify::{Classify, FailureClass};
    use crate::identity::ReplicatedEvent;
    use crate::peer::{EventGroup, PeerLimits, PushBatch, SyncPeer};
    use crate::wire::Route;

    /// What every call to [`Counting`] answers.
    #[derive(Debug, PartialEq, Eq, thiserror::Error)]
    #[error("the test transport reaches nothing")]
    struct Unreachable;

    impl Classify for Unreachable {
        fn class(&self) -> FailureClass {
            FailureClass::Transient
        }
    }

    /// A transport that counts its calls and answers each with
    /// [`Unreachable`], so "reached the exchange" is a count of one and a
    /// [`RemoteError::Exchange`], with no reply bytes to build.
    #[derive(Debug, Default)]
    struct Counting {
        calls: Cell<usize>,
    }

    impl Exchange for Counting {
        type Error = Unreachable;

        async fn exchange(&self, _route: Route, _request: Vec<u8>) -> Result<Vec<u8>, Unreachable> {
            self.calls.set(self.calls.get().saturating_add(1));
            Err(Unreachable)
        }
    }

    /// Limits with three different numbers, so a refusal names which one it
    /// read: 4 bytes an event, 10 bytes a push, 3 events a push.
    fn tight() -> PeerLimits {
        PeerLimits {
            max_event_bytes: 4,
            max_batch_bytes: 10,
            max_batch_events: 3,
            retention_floor: None,
        }
    }

    fn peer() -> RemoteSyncPeer<Counting> {
        RemoteSyncPeer::new(Counting::default(), tight())
    }

    /// The id of the event at `position` in a [`batch`].
    fn id(position: u64) -> EventId {
        EventId::new(
            StoreId::from_bytes([0x45; 16]),
            SequencePosition::new(position).expect("test positions start at 1"),
        )
    }

    /// One group of events, one per entry of `lens`, each carrying that many
    /// payload bytes, numbered from position 1.
    fn batch(lens: &[usize]) -> PushBatch {
        let events = (1_u64..)
            .zip(lens)
            .map(|(position, &len)| {
                let event = Event::new("Sized", alloc::vec![0_u8; len])
                    .expect("a static event type is valid");
                ReplicatedEvent::new(id(position), RecordedAt::from_millis(0), event)
            })
            .collect();
        PushBatch::new(alloc::vec![EventGroup::new(None, events)])
    }

    /// The control the refusals are measured against: a push the limits admit
    /// is sent, once.
    #[tokio::test]
    async fn a_push_inside_the_limits_reaches_the_exchange_once() {
        let peer = peer();

        let outcome = peer.push(&batch(&[1])).await;

        assert!(
            matches!(outcome, Err(RemoteError::Exchange(Unreachable))),
            "{outcome:?}"
        );
        assert_eq!(peer.exchange().calls.get(), 1);
    }

    /// One event no split can carry is refused as `Park`, naming it, and
    /// nothing is sent. Rejects a peer that sends without checking, and one
    /// that classes the lone event `Split`, which a runner would split forever.
    #[tokio::test]
    async fn an_event_over_max_event_bytes_is_park_and_never_sent() {
        let peer = peer();

        let refused = peer
            .push(&batch(&[5]))
            .await
            .expect_err("five bytes in an event over a four-byte limit");

        assert!(
            matches!(
                refused,
                RemoteError::EventTooLarge { id: named, len: 5, limit: 4 } if named == id(1)
            ),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Park);
        assert_eq!(peer.exchange().calls.get(), 0);
    }

    /// One event more than `max_batch_events`, each of them empty, is `Split`
    /// and never sent. Rejects a peer that checks only the bytes.
    #[tokio::test]
    async fn a_push_over_max_batch_events_is_split_and_never_sent() {
        let peer = peer();

        let refused = peer
            .push(&batch(&[0, 0, 0, 0]))
            .await
            .expect_err("four events over a three-event limit");

        assert!(
            matches!(
                refused,
                RemoteError::TooManyEvents {
                    actual: 4,
                    limit: 3
                }
            ),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Split);
        assert_eq!(peer.exchange().calls.get(), 0);
    }

    /// One payload byte over `max_batch_bytes`, from events each inside
    /// `max_event_bytes` and fewer than `max_batch_events`, is `Split` and
    /// never sent. Rejects a peer that checks only each event and the count.
    #[tokio::test]
    async fn a_push_over_max_batch_bytes_is_split_and_never_sent() {
        let peer = peer();

        let refused = peer
            .push(&batch(&[4, 4, 3]))
            .await
            .expect_err("eleven bytes over a ten-byte limit");

        assert!(
            matches!(
                refused,
                RemoteError::BatchTooLarge {
                    actual: 11,
                    limit: 10
                }
            ),
            "{refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Split);
        assert_eq!(peer.exchange().calls.get(), 0);
    }

    /// Each limit is a ceiling the push may reach: an event of exactly
    /// `max_event_bytes`, exactly `max_batch_events` events, and exactly
    /// `max_batch_bytes` bytes are all sent. Rejects a `>=` in any of the
    /// three comparisons.
    #[tokio::test]
    async fn a_push_at_each_limit_is_sent() {
        for lens in [&[4][..], &[0, 0, 0], &[4, 4, 2]] {
            let peer = peer();

            let outcome = peer.push(&batch(lens)).await;

            assert!(
                matches!(outcome, Err(RemoteError::Exchange(Unreachable))),
                "{lens:?}: {outcome:?}"
            );
            assert_eq!(peer.exchange().calls.get(), 1, "{lens:?}");
        }
    }

    /// A push over the count that also carries two oversized events is
    /// refused for the first of them. Rejects a check that reads the count
    /// before the events, and one that names the last offender.
    #[tokio::test]
    async fn an_oversized_event_outranks_the_count_and_the_first_is_named() {
        let peer = peer();

        let refused = peer
            .push(&batch(&[0, 5, 0, 6]))
            .await
            .expect_err("four events, two of them oversized");

        assert!(
            matches!(
                refused,
                RemoteError::EventTooLarge { id: named, len: 5, limit: 4 } if named == id(2)
            ),
            "{refused:?}"
        );
        assert_eq!(peer.exchange().calls.get(), 0);
    }

    /// The text says the limit was this client's declaration and that nothing
    /// was sent, so a log line tells a local refusal from the far side's.
    #[tokio::test]
    async fn each_refusal_says_it_was_declared_for_the_far_side_and_not_sent() {
        let peer = peer();
        let mut texts = Vec::new();

        for lens in [&[5][..], &[0, 0, 0, 0], &[4, 4, 3]] {
            let refused = peer.push(&batch(lens)).await.expect_err("over a limit");
            texts.push(refused.to_string());
        }

        assert_eq!(
            texts,
            [
                alloc::format!(
                    "event {} carries 5 payload bytes, over the max_event_bytes of 4 declared \
                     for the far side; not sent",
                    id(1)
                ),
                "push of 4 events exceeds the max_batch_events of 3 declared for the far side; \
                 not sent"
                    .into(),
                "push of 11 payload bytes exceeds the max_batch_bytes of 10 declared for the far \
                 side; not sent"
                    .into(),
            ]
        );
    }
}
