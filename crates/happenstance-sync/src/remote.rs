//! The client half of a network peer, over a transport the caller supplies.
//!
//! [`Exchange`] is the transport: one request's bytes out on a [`Route`], one
//! reply's bytes back. [`RemoteSyncPeer`] is the [`SyncPeer`] over it, encoding
//! and decoding the envelopes [`serve_sync`](crate::serve_sync) answers. Nothing
//! here names HTTP: an HTTP client, a Durable Object stub, a socket, or a
//! function call into `serve_sync` in the same process are all an `Exchange`.

use alloc::vec::Vec;

use crate::classify::{Classify, FailureClass};
use crate::peer::{Ack, PeerLimits, Pulled, PushBatch, SyncPeer};
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
    /// [`limits`](SyncPeer::limits). Declaring more than the far side accepts
    /// moves its refusal from before the push to after it; declaring less only
    /// costs smaller batches.
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

    /// One [`Route::Push`] exchange.
    ///
    /// # Errors
    ///
    /// [`RemoteError::Exchange`] if the transport fails, including a refusal
    /// the far side reported through it; [`RemoteError::Codec`] if the reply
    /// is not an ack this build reads.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
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

/// How a [`RemoteSyncPeer`] fails: its transport `E`, or bytes it could not
/// write or read.
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
}

impl<E: Classify> Classify for RemoteError<E> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Exchange(error) => error.class(),
            Self::Codec(error) => error.class(),
        }
    }
}
