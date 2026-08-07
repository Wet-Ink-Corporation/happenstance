//! The two real peers, stood in for, and pointed at the port.
//!
//! This file is the experiment. `SyncPeer` was not written and then admired; it
//! was written and then attacked with the two peers the deployment actually has,
//! which disagree about every property a Rust trait can express:
//!
//! | | Durable Object | Postgres over one-shot HTTP |
//! |---|---|---|
//! | `Send` | no — `Rc`, `JsValue`, single-threaded | yes |
//! | ambient runtime | none to spawn into | tokio |
//! | state between calls | a live socket | nothing at all |
//! | cursor | possible | impossible |
//! | error | wraps a `JsValue` string behind `Rc` | wraps a status code |
//!
//! Neither is the real adapter. Both are stand-ins with the *type* properties of
//! the real thing and none of its dependencies — deliberately, because a real
//! HTTP client drags a TLS stack into the workspace and the gate's licence
//! allowlist has opinions about those. A stand-in's `Send`-ness is asserted by
//! whoever wrote it rather than by `reqwest`, and that is the honest limit of
//! this evidence.
//!
//! Being an integration test, this is a **separate crate** from
//! `happenstance-sync`. That is not incidental: it is the same relationship a
//! real peer adapter has to the port, so an impl that compiles here is an impl
//! that compiles in a sibling crate.

#![allow(clippy::unwrap_used)]
// Both stand-ins are `todo!()` by construction — the type checker is the
// instrument here, not the runtime. Scoped to this file for the same reason the
// crate root scopes it: the phase that implements replication deletes both.
#![allow(clippy::todo)]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use happenstance_core::{Event, SequencePosition};
// The three identity types are spelled through `identity::` on purpose: they are
// phase 4's to define, and this import is one of the sites phase 4 rewrites.
use happenstance_sync::identity::{EventId, RecordedAt, StoreId};
use happenstance_sync::{
    Ack, EventGroup, MemorySyncPeer, PeerLimits, Pulled, PushBatch, ReplicatedEvent, SendSyncPeer,
    SyncPeer,
};

// ---------------------------------------------------------------------------
// Peer A: Durable Object shaped. !Send throughout.
// ---------------------------------------------------------------------------

/// Stands in for `worker::Error`, whose payload is a string pulled out of a
/// `JsValue`.
///
/// `Rc<str>` rather than `String` on purpose: it is what makes this type
/// genuinely `!Send`, which is the property under test. A `String` here would
/// make the whole experiment vacuous — the peer would be accidentally `Send` and
/// the port would never be asked the question.
#[derive(Debug, Clone)]
enum DurableObjectError {
    /// A `JsValue` that came back from the Workers runtime, stringified.
    JsError(Rc<str>),
    /// `SqlStorage` refused the statement.
    SqlStorage { sql: Rc<str>, message: Rc<str> },
    /// The socket to the far side closed mid-exchange.
    SocketClosed,
}

impl core::fmt::Display for DurableObjectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::JsError(message) => write!(f, "javascript error: {message}"),
            Self::SqlStorage { sql, message } => write!(f, "sql storage refused {sql}: {message}"),
            Self::SocketClosed => f.write_str("the peer socket closed"),
        }
    }
}

impl core::error::Error for DurableObjectError {}

/// The resume token a Durable Object hands out: an opaque string its own
/// `SqlStorage` understands.
///
/// `Rc<str>`, so this token is `!Send` too. `SyncPeer::Resume` carries no `Send`
/// bound, which is what admits it.
#[derive(Debug, Clone)]
struct DurableObjectResume(Rc<str>);

/// A peer that lives inside a Durable Object.
///
/// `Rc<RefCell<..>>` rather than `Arc<Mutex<..>>` because a Durable Object is
/// single-threaded by construction: there is exactly one instance of the object,
/// requests to it are serialised by the platform, and paying for an atomic would
/// be paying for a race that cannot happen.
struct DurableObjectPeer {
    store: StoreId,
    /// Stands in for `SqlStorage`. `!Send` and deliberately so.
    storage: Rc<RefCell<Vec<EventGroup>>>,
    /// The socket handle the Workers runtime gave us — `!Send`, and the reason
    /// this peer cannot be moved to another thread even in principle.
    socket: Rc<str>,
}

impl SyncPeer for DurableObjectPeer {
    type Error = DurableObjectError;
    type Resume = DurableObjectResume;

    async fn pull(
        &self,
        _from: Option<&Self::Resume>,
    ) -> Result<Pulled<Self::Resume>, Self::Error> {
        // The body would `SELECT` out of `SqlStorage` and await a socket read.
        // Both are `!Send` futures with no runtime to spawn into.
        let _ = (self.store, &self.storage, &self.socket);
        todo!("durable object peer: pull over the socket")
    }

    async fn push(&self, _batch: &PushBatch) -> Result<Ack, Self::Error> {
        todo!("durable object peer: push over the socket")
    }

    fn limits(&self) -> PeerLimits {
        PeerLimits::minimum()
    }
}

// ---------------------------------------------------------------------------
// Peer B: one-shot HTTP shaped. Send, and owns nothing between calls.
// ---------------------------------------------------------------------------

/// Stands in for a Neon `/sql` failure.
///
/// Every variant is owned and `Send`, which is what an HTTP client's errors
/// look like. The absence of a connection variant is the point: there is no
/// connection to lose.
#[derive(Debug)]
enum OneShotHttpError {
    /// The endpoint answered with a status this peer does not accept.
    Status { code: u16, body: Box<str> },
    /// The request never completed.
    Transport(Box<str>),
    /// The response body was larger than the endpoint's 64 MB ceiling, which is
    /// a hard limit of the deployment rather than a configuration.
    ResponseTooLarge { bytes: usize },
    /// The response parsed but did not mean anything.
    Malformed(Box<str>),
}

impl core::fmt::Display for OneShotHttpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Status { code, body } => write!(f, "http {code}: {body}"),
            Self::Transport(message) => write!(f, "transport failure: {message}"),
            Self::ResponseTooLarge { bytes } => {
                write!(f, "response of {bytes} bytes exceeds the 64 MB ceiling")
            }
            Self::Malformed(message) => write!(f, "malformed response: {message}"),
        }
    }
}

impl core::error::Error for OneShotHttpError {}

/// A server-issued continuation token.
///
/// Owned bytes the caller stores and hands back. This is the one place the port
/// is deliberately vague, and the vagueness is load-bearing rather than lazy:
/// the token has to be meaningful to the far side and opaque to the runner, or
/// the runner would start comparing two peers' tokens, which is exactly what
/// makes no sense across two independently-ordered logs.
///
/// It is also the place to be suspicious of this whole exercise. A port whose
/// types are all `Vec<u8>` names no transport and proves nothing; this one has
/// exactly one such type, and it earns it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct HttpResume(Box<[u8]>);

/// A peer reached over one-shot HTTP.
///
/// Holds an endpoint and a bearer token and **nothing else**. No connection, no
/// pool, no session, no cursor. Every field here survives being serialised into
/// a config file, which is the test for whether a peer is holding state it
/// should not.
struct OneShotHttpPeer {
    endpoint: Box<str>,
    bearer: Box<str>,
    /// The ceiling the deployment imposes on a single response. Not a choice.
    max_response_bytes: usize,
}

impl OneShotHttpPeer {
    /// One request, one response, no handle left over.
    ///
    /// The real body would be a single `POST` carrying a statement and reading
    /// the whole result set into memory, because there is no cursor to page
    /// with.
    async fn round_trip(&self, _statement: &str) -> Result<Box<[u8]>, OneShotHttpError> {
        let _ = (&self.endpoint, &self.bearer, self.max_response_bytes);
        todo!("one-shot http peer: POST /sql and read the whole response")
    }
}

impl SendSyncPeer for OneShotHttpPeer {
    type Error = OneShotHttpError;
    type Resume = HttpResume;

    async fn pull(
        &self,
        _from: Option<&Self::Resume>,
    ) -> Result<Pulled<Self::Resume>, Self::Error> {
        // One statement, one round trip. `from` becomes a `WHERE` clause rather
        // than a cursor position, because there is no cursor.
        let _ = self.round_trip("SELECT ... LIMIT ...").await?;
        todo!("one-shot http peer: decode the response into a batch")
    }

    async fn push(&self, _batch: &PushBatch) -> Result<Ack, Self::Error> {
        // Ingest is one multi-row INSERT in one statement. There is no
        // interactive transaction to open, so the atomicity `EventGroup`
        // requires has to be bought inside the single statement or not at all —
        // a capability limit, not a type error, and the port cannot see it.
        let _ = self.round_trip("INSERT ... ON CONFLICT DO NOTHING").await?;
        todo!("one-shot http peer: decode the ack")
    }

    fn limits(&self) -> PeerLimits {
        let mut limits = PeerLimits::minimum();
        limits.max_batch_bytes = self.max_response_bytes;
        limits
    }
}

// ---------------------------------------------------------------------------
// The claim: one body of generic code drives both, and names neither transport.
// ---------------------------------------------------------------------------

/// A runner step, written once against the bare flavour.
///
/// Bound on [`SyncPeer`] rather than [`SendSyncPeer`], which is the weaker
/// requirement and therefore accepts both peers. If this function had needed to
/// know which peer it held — a socket to close, a transaction to commit, a
/// cursor to advance — the port would have failed its exam here.
async fn exchange<P: SyncPeer>(peer: &P, outbound: &PushBatch) -> Result<(usize, usize), P::Error> {
    let limits = peer.limits();
    if !limits.admits(outbound) {
        // A capability check the runner can make *before* the round trip, which
        // is the only moment at which it is worth anything.
        return Ok((0, 0));
    }

    let ack = peer.push(outbound).await?;
    let pulled = peer.pull(None).await?;
    Ok((ack.appended, pulled.batch.groups.len()))
}

/// Resumes twice, holding the token across the second await.
///
/// This is the shape a real runner has — pull, checkpoint, pull again — and it
/// is the shape that discovers whether `Resume` can cross an await. It can, on
/// the bare flavour, because nothing there has to be `Send`.
async fn drain<P: SyncPeer>(peer: &P, rounds: usize) -> Result<usize, P::Error> {
    let mut resume: Option<P::Resume> = None;
    let mut seen = 0;

    for _ in 0..rounds {
        let pulled = peer.pull(resume.as_ref()).await?;
        seen += pulled.batch.groups.len();
        resume = Some(pulled.resume);
    }

    Ok(seen)
}

/// The same drain, spawned onto tokio.
///
/// `P::Resume: Send` is written at this bound and it is **load-bearing**.
/// `trait_variant` marks the derived futures `Send` and leaves associated types
/// alone, so without it the `Option<P::Resume>` held across the second `await`
/// makes this future `!Send` and `tokio::spawn` refuses with `error[E0277]`.
/// Removing the bound and asking the compiler is how that was established rather
/// than assumed.
///
/// The gap is the same one `happenstance-core` carries on `EventStore::Error`,
/// arriving somewhere it hurts more: an error can be collapsed to a `usize`
/// before the next await, and a resume token is the thing the next call needs.
fn spawn_drain<P>(peer: Arc<P>, rounds: usize) -> tokio::task::JoinHandle<usize>
where
    P: SendSyncPeer + Send + Sync + 'static,
    P::Resume: Send,
{
    tokio::spawn(async move {
        let mut resume: Option<P::Resume> = None;
        let mut seen = 0;

        for _ in 0..rounds {
            let Ok(pulled) = SendSyncPeer::pull(&*peer, resume.as_ref()).await else {
                break;
            };
            seen += pulled.batch.groups.len();
            resume = Some(pulled.resume);
        }

        seen
    })
}

/// Passing an item to a call is what makes clippy accept it as having an effect;
/// a `let _ = ..` binding is `no_effect_underscore_binding`.
fn instantiate<T>(_item: T) {}

/// The only way to state "this type crosses threads" as a checked obligation.
fn is_send<T: Send>(_value: &T) {}

fn store(byte: u8) -> StoreId {
    StoreId::from_bytes([byte; 16])
}

fn replicated(origin: StoreId, position: u64, payload: &'static str) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::new(position).unwrap()),
        RecordedAt::from_millis(1_700_000_000_000 + position),
        Event::new("CourseCapacityChanged", payload).unwrap(),
    )
}

/// Instantiating the generic functions is the assertion; there is nothing to
/// run, because both stand-ins are `todo!()` by design.
///
/// Naming a generic function monomorphises it, which is what forces the compiler
/// to check the bodies against each peer's real associated types. A bound that
/// merely *parses* proves nothing until it is instantiated.
#[test]
fn both_real_peer_shapes_satisfy_one_generic_runner() {
    instantiate(exchange::<DurableObjectPeer>);
    instantiate(drain::<DurableObjectPeer>);

    instantiate(exchange::<OneShotHttpPeer>);
    instantiate(drain::<OneShotHttpPeer>);

    // The `Send` flavour implies the bare one, so the HTTP peer satisfies both
    // and the memory fixture does too. The Durable Object peer satisfies only
    // the bare flavour, which is the whole reason there are two.
    instantiate(spawn_drain::<OneShotHttpPeer>);
    instantiate(spawn_drain::<MemorySyncPeer>);
    instantiate(exchange::<MemorySyncPeer>);
}

/// The two error types are real values, not shapes.
///
/// Constructing every variant is what stops `dead_code` from being the only
/// thing holding this file honest, and the `Send` assertion at the end is the
/// property the whole two-flavour scheme rests on: one of these peers can cross
/// a thread and the other cannot, and it is the *error type* that decides it.
#[test]
fn the_two_peers_disagree_about_send_and_the_error_types_are_why() {
    let js = DurableObjectError::JsError(Rc::from("TypeError: cannot read property"));
    let sql = DurableObjectError::SqlStorage {
        sql: Rc::from("SELECT * FROM event"),
        message: Rc::from("no such table"),
    };
    let closed = DurableObjectError::SocketClosed;
    let resume = DurableObjectResume(Rc::from("opaque-do-cursor"));

    assert!(js.to_string().contains("TypeError"));
    assert!(sql.to_string().contains("no such table"));
    assert!(closed.to_string().contains("socket"));
    assert!(resume.clone().0.contains("cursor"));

    let status = OneShotHttpError::Status {
        code: 429,
        body: Box::from("too many requests"),
    };
    let transport = OneShotHttpError::Transport(Box::from("dns failure"));
    let too_large = OneShotHttpError::ResponseTooLarge { bytes: 67_108_865 };
    let malformed = OneShotHttpError::Malformed(Box::from("expected an array"));

    assert!(status.to_string().contains("429"));
    assert!(transport.to_string().contains("dns"));
    assert!(too_large.to_string().contains("64 MB"));
    assert!(malformed.to_string().contains("array"));

    // The asymmetry, asserted rather than asserted-in-prose. There is no way to
    // write the negative — `DurableObjectError: !Send` is not a bound Rust can
    // state — so the `Rc` in its fields is what enforces it, and the fact that
    // `DurableObjectPeer` implements only the bare flavour is the consequence.
    is_send(&status);
    is_send(&HttpResume(Box::from(&b"cursor"[..])));
}

/// The fixture peer, driven for real, so that at least one path through the port
/// executes rather than merely type-checking.
#[tokio::test]
async fn memory_peer_round_trips_and_dedupes() {
    let origin = store(1);
    let peer = MemorySyncPeer::new(store(2));

    let batch = PushBatch::new(vec![EventGroup::new(
        None,
        vec![
            replicated(origin, 1, "{\"capacity\":10}"),
            replicated(origin, 2, "{\"capacity\":12}"),
        ],
    )]);

    let first = exchange(&peer, &batch).await.unwrap();
    assert_eq!(first.0, 2, "both events are new to this peer");

    // Re-delivery must be a no-op: a peer sees the same event more than once
    // whenever a resume token fails to advance.
    //
    // Fully qualified, and not by preference. This file needs both flavour names
    // in scope — one peer implements each — and method-call syntax on a type
    // that satisfies both is `error[E0034]: multiple applicable items in scope`.
    // The two-flavour scheme carries its ambiguity hazard to the third port
    // exactly as it does on the other two.
    let second = SendSyncPeer::push(&peer, &batch).await.unwrap();
    assert_eq!(second.appended, 0);
    assert_eq!(second.skipped, 2);
    assert_eq!(second.confirmed.get(origin), SequencePosition::new(2));

    let drained = drain(&peer, 2).await.unwrap();
    assert!(drained > 0, "the pull side sees what the push side wrote");

    let spawned = spawn_drain(Arc::new(MemorySyncPeer::new(store(3))), 1)
        .await
        .unwrap();
    assert_eq!(spawned, 0, "a fresh peer has nothing to hand back");
}
