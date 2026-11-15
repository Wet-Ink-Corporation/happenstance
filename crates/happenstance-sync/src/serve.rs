//! The server half of a network peer: envelope bytes in, envelope bytes out.
//!
//! [`serve_sync`] is what a transport mounts. It knows nothing of HTTP, sockets
//! or Durable Objects: the transport receives a body, says which
//! [`Route`] it came in on, and sends back what this returns. Everything
//! between — the version check, the decode, the store, the encode — is here,
//! once, so every transport answers with the same bytes.

use alloc::vec::Vec;
use core::convert::Infallible;

use crate::classify::{Classify, FailureClass};
use crate::ingest::IngestStore;
use crate::peer::{Pulled, PushBatch, SyncPeer};
use crate::runner::Adjudicate;
use crate::store_peer::{ForeignToken, StorePeerError, StoreResume, StoreSyncPeer};
use crate::wire::{CodecError, PullRequest, ResumeToken, Route, decode, encode};
use crate::write_group::WriteGroupStore;

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
    S: WriteGroupStore,
    I: IngestStore,
{
    serve_sync_adjudicated(peer, route, request).await
}

/// [`serve_sync`] through a store peer that judges what it is pushed: one
/// request answered through `peer`, whose merge rule `A` lands each pushed
/// group with its compensation (SY-7, SY-10).
///
/// What a hub served over a transport mounts when it is the adjudicator of
/// what its spokes push: build the peer with
/// [`StoreSyncPeer::with_adjudicator`] and serve it here. A sibling rather
/// than a third type parameter on [`serve_sync`], so code that names
/// `serve_sync::<S, I>` keeps compiling. Every other word of [`serve_sync`]'s
/// documentation holds here, and the bytes are the same.
///
/// # Errors
///
/// As [`serve_sync`], and [`ServeError::Peer`] holding
/// [`StorePeerError::Adjudicate`] if the rule cannot judge a push, with
/// nothing written.
pub async fn serve_sync_adjudicated<S, I, A>(
    peer: &StoreSyncPeer<S, I, A>,
    route: Route,
    request: &[u8],
) -> Result<Vec<u8>, ServeError<S::Error, I::Error, A::Error>>
where
    S: WriteGroupStore,
    I: IngestStore,
    A: Adjudicate,
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
            // A literal, not `new` and builders: a field added to `Pulled` is a
            // compile error here until this carries it onto the wire.
            let reply = Pulled {
                batch: pulled.batch,
                resume: ResumeToken::from(pulled.resume),
                waiting: pulled.waiting,
                responder: pulled.responder,
            };
            Ok(encode(route, &reply)?)
        }
        Route::Push => {
            let batch: PushBatch = decode(route, request)?;
            let ack = peer.push(&batch).await.map_err(ServeError::Peer)?;
            Ok(encode(route, &ack)?)
        }
    }
}

/// Why [`serve_sync`] or [`serve_sync_adjudicated`] could not answer.
///
/// Generic over the store's read error `R`, its ingest error `W` and the
/// peer's merge rule's error `A`, as [`StorePeerError`] is, and bounded
/// nowhere on the enum, so it is `Send` exactly when they are. `A` defaults
/// to [`Infallible`], so `ServeError<R, W>` is still [`serve_sync`]'s error.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ServeError<R, W, A = Infallible> {
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
    Peer(StorePeerError<R, W, A>),
}

impl<R: Classify, W: Classify, A: Classify> Classify for ServeError<R, W, A> {
    fn class(&self) -> FailureClass {
        match self {
            Self::Codec(error) => error.class(),
            Self::Token(error) => error.class(),
            Self::Peer(error) => error.class(),
        }
    }
}
