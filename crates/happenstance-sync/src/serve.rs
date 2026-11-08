//! The server half of a network peer: envelope bytes in, envelope bytes out.
//!
//! [`serve_sync`] is what a transport mounts. It knows nothing of HTTP, sockets
//! or Durable Objects: the transport receives a body, says which
//! [`Route`] it came in on, and sends back what this returns. Everything
//! between — the version check, the decode, the store, the encode — is here,
//! once, so every transport answers with the same bytes.

use alloc::vec::Vec;

use happenstance_core::EventStore;

use crate::classify::{Classify, FailureClass};
use crate::ingest::IngestStore;
use crate::peer::{Pulled, PushBatch, SyncPeer};
use crate::store_peer::{ForeignToken, StorePeerError, StoreResume, StoreSyncPeer};
use crate::wire::{CodecError, PullRequest, ResumeToken, Route, decode, encode};

/// Answers one request through `peer`, and returns the reply's bytes.
///
/// `request` is the body exactly as it arrived: a postcard
/// [`Envelope`](crate::wire::Envelope) holding a
/// [`PullRequest`] on [`Route::Pull`] or a [`PushBatch`] on [`Route::Push`].
/// The reply is an envelope holding a [`Pulled<ResumeToken>`](Pulled) or an
/// [`Ack`](crate::Ack). Which message a buffer holds is the transport's to say
/// (ADR-0097); a body sent on the wrong route can be refused as malformed, or
/// can decode as the other message, and only the transport can prevent that.
///
/// The token a pull answers with is [`StoreResume`]'s text form, so the client
/// hands back what this minted and nothing else is ever asked to read it.
///
/// # What the transport owns
///
/// **The size of `request`.** It is untrusted and fully buffered by the time it
/// arrives here, so the cap belongs where it is read: a transport refuses a
/// body much larger than the peer's
/// [`max_batch_bytes`](crate::PeerLimits::max_batch_bytes) before calling this.
///
/// **Turning an error into a reply.** There is no error message in the wire
/// set, so a transport maps [`ServeError::class`](Classify::class) to whatever
/// its client can classify again — for HTTP, say, `Transient` to 503, `Split`
/// to 413, `Park` to 422 and `Stop` to 400 — and the client's
/// [`Exchange`](crate::Exchange) error reports that class back.
///
/// # Errors
///
/// * [`ServeError::Codec`] if the request is not an envelope this build reads:
///   another format version, or malformed bytes.
/// * [`ServeError::Token`] if a pull resumes from a token no store peer minted.
/// * [`ServeError::Peer`] if the peer refuses or fails, as
///   [`StoreSyncPeer`]'s pull and push document.
pub async fn serve_sync<S, I>(
    peer: &StoreSyncPeer<S, I>,
    route: Route,
    request: &[u8],
) -> Result<Vec<u8>, ServeError<S::Error, I::Error>>
where
    S: EventStore,
    S::Error: Classify,
    I: IngestStore,
{
    match route {
        Route::Pull => {
            let request: PullRequest = decode(route, request)?;
            let from = request
                .from
                .as_ref()
                .map(StoreResume::try_from)
                .transpose()?;
            let pulled = peer.pull(from.as_ref()).await.map_err(ServeError::Peer)?;
            let reply = Pulled::new(pulled.batch, ResumeToken::from(pulled.resume))
                .with_waiting(pulled.waiting);
            Ok(encode(route, &reply)?)
        }
        Route::Push => {
            let batch: PushBatch = decode(route, request)?;
            let ack = peer.push(&batch).await.map_err(ServeError::Peer)?;
            Ok(encode(route, &ack)?)
        }
    }
}

/// Why [`serve_sync`] could not answer.
///
/// Generic over the store's read error `R` and ingest error `W`, as
/// [`StorePeerError`] is, and bounded nowhere on the enum, so it is `Send`
/// exactly when they are.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ServeError<R, W> {
    /// The request did not decode, or the reply did not encode.
    /// [`Stop`](FailureClass::Stop).
    #[error(transparent)]
    Codec(#[from] CodecError),

    /// A pull resumed from a token no store peer minted.
    /// [`Stop`](FailureClass::Stop).
    #[error(transparent)]
    Token(#[from] ForeignToken),

    /// The peer refused or failed; its error states the class.
    #[error(transparent)]
    Peer(StorePeerError<R, W>),
}

impl<R: Classify, W: Classify> Classify for ServeError<R, W> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Codec(error) => error.class(),
            Self::Token(error) => error.class(),
            Self::Peer(error) => error.class(),
        }
    }
}
