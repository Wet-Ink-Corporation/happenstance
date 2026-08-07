//! The most ambitious signature this sketch attempted, and what happened to it.
//!
//! `SyncPeer::pull` returns a bounded batch and an owned token. The obvious
//! alternative — and the one the workspace already reaches for, because
//! [`EventStore::read`](happenstance_core::EventStore::read) has exactly this
//! shape — is a **cursor**:
//!
//! ```text
//! fn pull(&self, from: Option<&Self::Resume>)
//!     -> impl Stream<Item = Result<ReplicatedEvent, Self::Error>>;
//! ```
//!
//! That shape is forbidden by the deployment: a stream is a cursor, a cursor is
//! state held between polls, and a peer reached over one-shot HTTP has no
//! connection, no session and no transaction to hold it in.
//!
//! So the experiment was: **does the type checker agree?** Point the shape at
//! both real peer shapes and see which one it rejects.
//!
//! # Result: it rejects neither, and that is the finding
//!
//! The module below is the cursor-shaped port with both peers implementing it,
//! and it compiles clean under
//! `cargo clippy -p happenstance-sync --all-targets --all-features -- -D warnings`.
//! The one-shot HTTP peer satisfies `impl Stream` by buffering one whole
//! response into a `Vec` and replaying it — legal, `Send`, and a lie. A peer
//! that can never page will be indistinguishable, at the type level, from one
//! that pages perfectly, right up until a log exceeds the 64 MB a single
//! response can carry.
//!
//! **"One round trip, no held state" is a capability constraint and not a type
//! constraint.** Nothing in the port can enforce it; only a fixture peer that
//! counts its own round trips and panics on the second can, and that belongs to
//! the conformance suite. A table of `error[E….]` rows would have shown the
//! cursor shape as compatible with every peer in the workspace, which is the
//! precise opposite of the truth.
//!
//! # Two things the type checker *did* reject
//!
//! **Nesting the stream inside the future.** Writing the cursor shape as
//! `async fn pull(..) -> Result<impl Stream<..>, Self::Error>` and marking the
//! stream `Send` in the one-shot HTTP impl:
//!
//! ```text
//! error: impl trait in impl method signature does not match trait method signature
//!   --> crates\happenstance-sync\src\probe_stream.rs:67:76
//!    |
//! 12 | #[trait_variant::make(SendStreamPeer: Send)]
//!    | -------------------------------------------- return type from trait method defined here
//! ...
//! 67 |     ) -> Result<impl Stream<Item = Result<ReplicatedEvent, Self::Error>> + Send, Self::Error> {
//!    |                                                                            ^^^^ this bound is stronger than that defined on the trait
//!    |
//!    = note: add `#[allow(refining_impl_trait)]` if it is intended for this to be part of the public API of this crate
//!    = note: `-D refining-impl-trait-reachable` implied by `-D warnings`
//! ```
//!
//! and then, with that `+ Send` removed, asking whether the derived `Send`
//! flavour hands the caller a `Send` stream:
//!
//! ```text
//! error: future cannot be sent between threads safely
//!    --> crates\happenstance-sync\src\probe_stream.rs:129:5
//!     |
//! 129 | /     is_send(&async move {
//! 130 | |         let stream = SendStreamPeer::pull(peer, None).await;
//! 131 | |         core::future::ready(()).await;
//! 132 | |         drop(stream);
//! 133 | |     });
//!     | |______^ future created by async block is not `Send`
//!     |
//!     = help: within `{async block@..}`, the trait `std::marker::Send` is not
//!             implemented for `impl Stream<Item = Result<ReplicatedEvent, ...>>`
//! ```
//!
//! It does not. `trait_variant` marks the outermost `impl Future` `Send` and
//! nothing inside it. That is ADR-0008's finding, reproduced independently on
//! the third port, against a trait ADR-0008 never saw — which is about as strong
//! as corroboration gets. The version below keeps the stream at the top level
//! and is `Send` on the derived flavour, which is why
//! [`assert_the_send_flavour_yields_a_send_stream`] compiles.

#![allow(clippy::unwrap_used)]

use std::rc::Rc;

use futures_core::Stream;
use happenstance_sync::ReplicatedEvent;

/// The cursor-shaped port. Not the one that ships.
#[trait_variant::make(SendStreamPeer: Send)]
trait StreamPeer {
    type Error: core::error::Error + 'static;
    type Resume: Clone + 'static;

    /// Stream at the top level and not `async`, which is the only arrangement
    /// under which the derived flavour can mark the *stream* `Send`.
    fn pull(
        &self,
        from: Option<&Self::Resume>,
    ) -> impl Stream<Item = Result<ReplicatedEvent, Self::Error>>;
}

// --- Peer B: one-shot HTTP. No cursor exists, so it fakes one. --------------

#[derive(Debug)]
struct HttpError(Box<str>);

impl core::fmt::Display for HttpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for HttpError {}

struct OneShotHttpPeer {
    endpoint: Box<str>,
}

/// The only "stream" a peer with no cursor can produce: one buffered response,
/// replayed item by item.
///
/// This type is the whole finding in miniature. It is a perfectly good `Stream`,
/// it is `Send`, it satisfies the port — and it has already read everything into
/// memory before the first `poll_next`, which is exactly what the streaming
/// shape exists to avoid.
struct BufferedResponse {
    buffered: Vec<ReplicatedEvent>,
    next: usize,
}

impl Stream for BufferedResponse {
    type Item = Result<ReplicatedEvent, HttpError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let item = this.buffered.get(this.next).cloned();
        this.next += 1;
        core::task::Poll::Ready(item.map(Ok))
    }
}

impl SendStreamPeer for OneShotHttpPeer {
    type Error = HttpError;
    type Resume = Box<[u8]>;

    fn pull(
        &self,
        _from: Option<&Self::Resume>,
    ) -> impl Stream<Item = Result<ReplicatedEvent, Self::Error>> + Send {
        let _ = &self.endpoint;
        BufferedResponse {
            buffered: Vec::new(),
            next: 0,
        }
    }
}

// --- Peer A: Durable Object. A real cursor, and !Send throughout. -----------

#[derive(Debug)]
struct DurableObjectError(Rc<str>);

impl core::fmt::Display for DurableObjectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

impl core::error::Error for DurableObjectError {}

struct DurableObjectPeer {
    socket: Rc<str>,
}

/// A genuine cursor: it holds the socket open between polls, which is the thing
/// the other peer cannot do and the type system cannot tell apart.
struct SocketCursor {
    socket: Rc<str>,
}

impl Stream for SocketCursor {
    type Item = Result<ReplicatedEvent, DurableObjectError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        let _ = &self.socket;
        core::task::Poll::Ready(None)
    }
}

impl StreamPeer for DurableObjectPeer {
    type Error = DurableObjectError;
    type Resume = Rc<str>;

    fn pull(
        &self,
        _from: Option<&Self::Resume>,
    ) -> impl Stream<Item = Result<ReplicatedEvent, Self::Error>> {
        SocketCursor {
            socket: Rc::clone(&self.socket),
        }
    }
}

/// The control for the rejected nested shape: with the stream at the top level,
/// the derived flavour really does hand back a `Send` stream.
///
/// Holding it across an `await` is the assertion. Under
/// `async fn pull(..) -> Result<impl Stream, _>` this is `error[E0277]`; under
/// the shape below it compiles, which is the whole reason ADR-0008 exists.
fn assert_the_send_flavour_yields_a_send_stream<P>(peer: &P)
where
    P: SendStreamPeer + Sync,
    P::Error: Send,
    P::Resume: Send,
{
    fn is_send<T: Send>(_value: &T) {}
    is_send(&async move {
        let stream = SendStreamPeer::pull(peer, None);
        core::future::ready(()).await;
        drop(stream);
    });
}

/// Instantiating is the assertion; there is nothing to run.
#[test]
fn a_cursor_shaped_port_admits_a_peer_that_cannot_hold_a_cursor() {
    fn instantiate<T>(_item: T) {}

    // The peer that *can* hold a cursor, on the bare flavour.
    let durable = DurableObjectPeer {
        socket: Rc::from("ws://do"),
    };
    instantiate(StreamPeer::pull(&durable, None));

    // The peer that *cannot*, on the derived flavour — and the compiler has no
    // objection whatsoever.
    let http = OneShotHttpPeer {
        endpoint: Box::from("https://ep.neon.tech/sql"),
    };
    instantiate(SendStreamPeer::pull(&http, None));
    instantiate(assert_the_send_flavour_yields_a_send_stream::<OneShotHttpPeer>);
}
