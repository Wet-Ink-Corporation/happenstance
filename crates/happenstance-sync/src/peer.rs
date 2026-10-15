//! The peer port, in two flavours.
//!
//! # Status: sketch
//!
//! This is a phase-2 instrument, not the protocol. It exists to answer one
//! falsifiable question — **can a peer be stated without naming a transport?** —
//! and it is settled against the two peers the deployment actually has, which
//! are as unlike each other as the story allows:
//!
//! * a Cloudflare **Durable Object** reached over a socket: `!Send` throughout,
//!   errors that carry `Rc`, no ambient runtime to spawn into;
//! * a **Postgres over one-shot HTTP**: `Send`, owns nothing between calls,
//!   cannot hold a cursor open, one round trip per operation.
//!
//! Both are implemented against this trait in this crate's `tests/`. What that
//! proved, and what it did not, is written down at the bottom of this page.
//!
//! # Two flavours, for the third time
//!
//! [`SyncPeer`] is written without any `Send` requirement and `trait_variant`
//! derives [`SendSyncPeer`] from it, exactly as `happenstance-core` does for its
//! two ports. The scheme is not being copied for symmetry: the Durable Object
//! peer *cannot* satisfy a `Send` bound and the HTTP peer wants one, so a single
//! trait would exclude one of the two peers this crate exists to serve.
//!
//! As everywhere else: bind [`SyncPeer`] in generic code — it is the weaker
//! requirement and accepts both — and import only one of the two names per
//! module, or method calls go ambiguous with `error[E0034]`.
//!
//! # Why `pull` returns a batch and not a stream
//!
//! [`EventStore::read`](happenstance_core::EventStore::read) returns
//! `impl Stream` at the top level, and that is load-bearing there: it is what
//! lets the `Send` flavour mark the *stream* `Send`, and laziness is what buys a
//! million-event replay without buffering.
//!
//! It is the wrong shape here. A stream is a cursor, a cursor is state held
//! between polls, and a peer reached over one-shot HTTP has nothing to hold it
//! *in*: no connection, no session, no transaction. So [`pull`](SyncPeer::pull)
//! returns a bounded batch and an owned resume token, and the caller loops. One
//! round trip per call, by construction rather than by documentation.
//!
//! This is not a free choice, and **the type checker does not make it for us**.
//! The cursor shape was attempted, pointed at both peers, and compiled against
//! both: the one-shot HTTP peer satisfies `impl Stream` by buffering a whole
//! response into a `Vec` and replaying it. Legal, `Send`, and a lie.
//! `tests/cursor_shape_probe.rs` is that experiment, kept compiling so the
//! result cannot rot. What it means for this port is that one round trip with no
//! held state has to be checked by a fixture peer that counts its own round
//! trips, in the conformance suite — the port itself cannot express it.

use alloc::boxed::Box;
use alloc::vec::Vec;

use happenstance_core::{AppendCondition, EventId};

use crate::identity::{ReplicatedEvent, Watermark};

/// One peer relationship.
///
/// This is the `!Send` flavour and the one to use in generic bounds; adapters
/// that can cross threads should implement [`SendSyncPeer`] and get this for
/// free.
///
/// # One peer, not a peer set
///
/// Every method describes a single relationship. Fan-out across several peers,
/// ordering between them, and what to do when two disagree are *policy*, and
/// policy on the port would make every adapter author — including whoever writes
/// the 200-line HTTP client — inherit the merge problem. They belong to a runner
/// above this trait, in the same division of labour that puts the projection
/// runner above `ProjectionStore`. "Add a second peer" is then a runner
/// configuration and not a breaking change to a published trait.
///
/// # Hub-ness is not on the port
///
/// Nothing here says whether the far side is a hub or a spoke. A store in the
/// middle of a chain — a Durable Object that is a spoke to a cloud estate store
/// and a hub to 138 tablets — is one peer wearing each hat, and a `is_hub()` on
/// the port would force it to answer a question that has two answers.
#[trait_variant::make(SendSyncPeer: Send)]
pub trait SyncPeer {
    /// How this peer's transport fails; each failure states its [class](crate::Classify).
    ///
    /// Carries no `Send` bound. That is what admits the Durable Object peer,
    /// whose errors wrap a `JsValue`-derived string behind an `Rc` and are
    /// `!Send` for reasons no adapter author can do anything about. Requiring
    /// `Send + Sync` here was attempted; the Durable Object stand-in in
    /// `tests/real_peer_shapes.rs` fails it with two `error[E0277]`s — ``Rc<str>``
    /// cannot be sent between threads safely, and the same again for `Sync`.
    type Error: core::error::Error + crate::Classify + 'static;

    /// Where to resume from: opaque to the caller, owned by the caller.
    ///
    /// An associated type rather than a [`Watermark`] or a
    /// [`SequencePosition`](happenstance_core::SequencePosition), for two
    /// reasons that pull in the same direction. A position is meaningful only
    /// inside one store, so it cannot cross this boundary at all; and whether
    /// replication is whole-log or scoped is still open, and a scalar cannot
    /// distinguish "not yet received" from "filtered out". An opaque associated
    /// type defers that question into the adapter without deferring the port.
    ///
    /// **Owned and `Clone`, never a handle the peer holds.** A resume token that
    /// lives inside the peer object is a resume token that does not exist at the
    /// edge: a Worker is cancelled mid-flight, a Durable Object is evicted, a
    /// tablet loses signal. The normal termination path out there *is* the
    /// handle going away, so the token has to survive the handle being dropped
    /// and reconstructed.
    ///
    /// It is not required to be `Send`, and that is a gap rather than a
    /// decision: `trait_variant` marks the derived futures `Send` and leaves
    /// associated types alone, so a runner that wants to `tokio::spawn` a
    /// per-peer task and carry the token out of it needs to write
    /// `S::Resume: Send` at its own bound. `E0277` when it does not; the same
    /// gap `happenstance-core` has on `EventStore::Error`, arriving on a type
    /// that carries *data* rather than a failure, where it cannot be collapsed
    /// to a `usize` before the next await.
    type Resume: Clone + 'static;

    /// Fetches the next batch of events the far side holds and this side does
    /// not, plus the token to resume after them.
    ///
    /// `from` is `None` on a first exchange. The returned token is always the
    /// one to use next, even when the batch is empty — an empty batch means
    /// "caught up for now", not "start over".
    ///
    /// # One round trip
    ///
    /// The whole operation must complete in a single exchange. There is no
    /// `open` / `next_batch` / `close`, which is the shape every socket-based
    /// design produces and is unimplementable on a peer that reaches its store
    /// over one-shot HTTP.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error for transport and authorisation failures.
    /// Nothing about the *content* of the far side's log is an error here.
    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error>;

    /// Offers a batch of events to the far side.
    ///
    /// # Errors
    ///
    /// Transport- or authorisation-level refusals only. A peer may not refuse a
    /// push because it disagrees with the events in it: an ingested event is a
    /// fact another store has already durably committed, and refusing it does
    /// not un-commit it — it only guarantees the two logs never converge.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error>;

    /// What this peer can accept, readable *before* a push rather than
    /// discovered as an error after one.
    ///
    /// Not `async`, and that is the whole point: a runner that has to await a
    /// round trip to learn a size cap will skip the check, and a limit nobody
    /// checks is a limit discovered at ingest — which is after the write has
    /// already committed somewhere else, the one moment at which nothing useful
    /// can be done about it.
    fn limits(&self) -> PeerLimits;
}

/// What one [`pull`](SyncPeer::pull) returned.
///
/// A named struct rather than `(PushBatch, R, Waiting)` because each field is
/// worth naming at the call site, and a tuple of a batch and a token invites
/// exactly the transposition it cannot catch.
///
/// [`new`](Self::new) says [`Waiting::Unknown`], which costs a runner one more
/// pull and is never a lie. A peer that can tell, inside the same round trip,
/// whether anything lies past its token says so with
/// [`with_waiting`](Self::with_waiting).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct Pulled<R> {
    /// The events, in the far side's forwarding order.
    pub batch: PushBatch,
    /// Where to resume the next `pull` from.
    pub resume: R,
    /// What the far side still holds past `resume` (SY-37). Declared last:
    /// postcard is positional, and the two fields above keep their places.
    pub waiting: Waiting,
}

/// A batch of events crossing the boundary, decomposed into the groups that
/// must land atomically.
///
/// The decomposition is **on the wire and explicit**, not left for the receiver
/// to infer. Each group is a set of events that one origin-side decision wrote
/// together under one guard; a receiver that flattened the batch and appended
/// event-by-event would publish a state the origin never had, and no amount of
/// ordering fixes that.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct PushBatch {
    /// The groups, in forwarding order.
    pub groups: Vec<EventGroup>,
}

impl PushBatch {
    /// An empty batch: the far side has nothing new.
    #[must_use]
    pub const fn empty() -> Self {
        Self { groups: Vec::new() }
    }

    /// Assembles a batch from its groups.
    #[must_use]
    pub const fn new(groups: Vec<EventGroup>) -> Self {
        Self { groups }
    }

    /// Every event in the batch, flattened, in forwarding order.
    ///
    /// For counting and for size checks. Ingest must not use this: flattening is
    /// exactly what [`EventGroup`] exists to prevent.
    pub fn events(&self) -> impl Iterator<Item = &ReplicatedEvent> + '_ {
        self.groups.iter().flat_map(|group| group.events.iter())
    }

    /// Total payload bytes, for checking against
    /// [`PeerLimits::max_batch_bytes`].
    #[must_use]
    pub fn payload_len(&self) -> usize {
        self.events().map(ReplicatedEvent::payload_len).sum()
    }
}

/// Events that one origin-side decision appended together, and the condition it
/// appended them under.
///
/// The `guard` travels as **evidence, not as an instruction**. It says what the
/// origin checked before it decided; it is not a condition for the receiver to
/// re-evaluate against its own log, because the receiver's log contains events
/// the origin never saw and re-checking would reject a fact that is already
/// durable elsewhere. SY-6 settles what it is for: an adjudicator may read it,
/// and a guard carrying `after` is refused at ingest. Phase 13 builds the refusal.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct EventGroup {
    /// What the origin checked before appending. Evidence only.
    pub guard: Option<AppendCondition>,
    /// The events, which must land atomically or not at all.
    pub events: Vec<ReplicatedEvent>,
}

impl EventGroup {
    /// Groups events that were appended together under `guard`.
    #[must_use]
    pub const fn new(guard: Option<AppendCondition>, events: Vec<ReplicatedEvent>) -> Self {
        Self { guard, events }
    }
}

/// What the far side did with a [`push`](SyncPeer::push).
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct Ack {
    /// How many events the far side appended, excluding ones it already held.
    pub appended: u64,
    /// How many events the far side already held and skipped.
    ///
    /// Re-delivery is a no-op, not an error: a peer *will* see the same event
    /// more than once, and the count is here so a runner can tell a healthy
    /// overlap from a resume token that is not advancing.
    pub skipped: u64,
    /// What the far side has now durably seen, per origin store.
    ///
    /// A confirmation, deliberately **not** an event. Writing "peer X has seen
    /// up to Y" into the log would make every confirmation replicable, and two
    /// peers confirming each other's confirmations is a log that grows without
    /// anyone doing anything.
    pub confirmed: Watermark,
}

/// What a peer can accept.
///
/// Read before pushing. The contract bounds an event type and a tag at 255 bytes
/// each and leaves the payload unbounded, so an event that is durable at its
/// origin can be structurally unrepresentable at a peer — a KV-backed store with
/// a 128 KiB value cap against an origin that has already committed 340 KB.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct PeerLimits {
    /// Largest single event payload, in bytes.
    ///
    /// A peer below the contract's own floor is non-conformant, not merely
    /// small.
    pub max_event_bytes: usize,
    /// Largest total payload in one [`push`](SyncPeer::push), in bytes.
    ///
    /// The one-shot HTTP peer's ceiling is a response size, and it is the reason
    /// this is separate from `max_event_bytes`: a peer can be fine with a large
    /// event and unable to carry sixty of them.
    pub max_batch_bytes: usize,
    /// Largest number of events in one [`push`](SyncPeer::push).
    pub max_batch_events: usize,
    /// The lowest resume point this peer can still satisfy, if it has pruned.
    ///
    /// A resume token pointing below this can never be satisfied, and a runner
    /// that keeps retrying it never converges. It says nothing about whether
    /// everything above it is retained (SY-32, ADR-0028). `None`: no pruning.
    pub retention_floor: Option<EventId>,
}

impl PeerLimits {
    /// The smallest limits a conformant peer may declare.
    ///
    /// 64 KiB per event is the contract's own floor for what a store must
    /// accept; a peer that cannot manage that is not a peer.
    #[must_use]
    pub const fn minimum() -> Self {
        Self {
            max_event_bytes: 65_536,
            max_batch_bytes: 65_536,
            max_batch_events: 128,
            retention_floor: None,
        }
    }

    /// Whether `batch` fits inside these limits.
    ///
    /// Advisory. A peer still has to check on arrival, because limits read
    /// before a push can be stale by the time it lands.
    #[must_use]
    pub fn admits(&self, batch: &PushBatch) -> bool {
        batch.payload_len() <= self.max_batch_bytes
            && batch.events().count() <= self.max_batch_events
            && batch
                .events()
                .all(|event| event.payload_len() <= self.max_event_bytes)
    }
}

/// A running budget for filling one [`pull`](SyncPeer::pull) response.
///
/// Exists because "one round trip" is a size constraint as much as a protocol
/// one: an adapter has to stop adding groups *before* it exceeds what it
/// declared, and every adapter would otherwise write the same three comparisons
/// slightly differently.
#[derive(Debug, Clone, Copy)]
pub struct PullBatchLimit {
    bytes_left: usize,
    events_left: usize,
    max_event_bytes: usize,
}

impl PullBatchLimit {
    /// Whether `group` still fits.
    ///
    /// An oversized single event returns `false` for ever, which is correct and
    /// is why [`PeerLimits::retention_floor`] exists: the runner has to be able
    /// to tell "wait" from "this will never move".
    #[must_use]
    pub fn admits(&self, group: &EventGroup) -> bool {
        let bytes: usize = group.events.iter().map(ReplicatedEvent::payload_len).sum();
        group.events.len() <= self.events_left
            && bytes <= self.bytes_left
            && group
                .events
                .iter()
                .all(|event| event.payload_len() <= self.max_event_bytes)
    }

    /// Deducts `group` from the budget.
    ///
    /// Saturating rather than wrapping: a budget that underflows to `usize::MAX`
    /// would admit everything after it, which is the opposite of what a limit is
    /// for.
    pub fn charge(&mut self, group: &EventGroup) {
        let bytes: usize = group.events.iter().map(ReplicatedEvent::payload_len).sum();
        self.bytes_left = self.bytes_left.saturating_sub(bytes);
        self.events_left = self.events_left.saturating_sub(group.events.len());
    }
}

impl From<&PeerLimits> for PullBatchLimit {
    fn from(limits: &PeerLimits) -> Self {
        Self {
            bytes_left: limits.max_batch_bytes,
            events_left: limits.max_batch_events,
            max_event_bytes: limits.max_event_bytes,
        }
    }
}

/// Why a peer exchange failed for reasons no adapter owns.
///
/// Adapters report their *own* failures through
/// [`SyncPeer::Error`]. This is for the runner above them, which has failure
/// modes of its own: a token that no longer resolves, a batch the far side
/// declared it cannot take.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SyncError {
    /// The resume token points below the far side's retention floor.
    ///
    /// Not retryable. The gap has to be closed by a full re-seed or accepted.
    #[error("resume token is below the peer's retention floor at {floor}")]
    BelowRetentionFloor {
        /// The lowest resume point the peer can still satisfy.
        floor: EventId,
    },
    /// The batch exceeds what the peer declared it can accept.
    ///
    /// Retryable only after splitting; retrying the same batch will fail
    /// identically for ever.
    #[error("batch of {actual} bytes exceeds the peer's limit of {limit}")]
    BatchTooLarge {
        /// What was offered.
        actual: usize,
        /// What the peer declared.
        limit: usize,
    },
    /// A peer named in the runner's configuration is not reachable by any
    /// configured transport.
    #[error("no transport is configured for peer {name}")]
    UnknownPeer {
        /// The runner's name for the peer, which is configuration and not an
        /// identity the port knows about.
        name: Box<str>,
    },
}

// Declared here, at the end, so the lines above keep the numbers cited
// elsewhere. What a pull says about the rest of the far side's log (SY-37).

/// What the far side still holds past the token a [`pull`](SyncPeer::pull)
/// returned, as [`Pulled::waiting`] reports it.
///
/// Three cases, because a runner faces three: it knows there is nothing more,
/// it knows how much more there is, or it was not told. Spelled as an enum
/// rather than an `Option<u64>`, which cannot say "not told" without a
/// sentinel, and rather than a flag beside an optional count, which can say
/// "nothing more, and here are three".
///
/// The count is a [`NonZeroU64`](core::num::NonZeroU64), so "nothing" has one
/// spelling: `Events(0)` cannot be written, and a runner matching on
/// [`Nothing`](Self::Nothing) misses no second form of it. Its decoder refuses
/// a zero from the wire for the same reason. Not `#[non_exhaustive]`: the three
/// cases are the whole space, and a runner should get the compiler's
/// exhaustiveness check when it matches.
///
/// # On the wire
///
/// Externally tagged, in snake case: `"nothing"`, `"unknown"`, or
/// `{"events":3}`. In postcard the variant index is the declaration order —
/// `Nothing` 0, `Unknown` 1, `Events` 2 — so the variants are never reordered.
///
/// # Examples
///
/// A runner that believes [`Nothing`](Self::Nothing) stops after the pull that
/// says it, rather than spending one more round trip on a pull that comes back
/// empty — the round trip SY-15 says is the expensive one:
///
/// ```
/// # #[cfg(not(feature = "memory"))]
/// # fn main() {}
/// # #[cfg(feature = "memory")]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
/// use happenstance_sync::{
///     EventGroup, MemorySyncPeer, ReplicatedEvent, SyncPeer, Waiting,
/// };
///
/// let tablet = StoreId::from_bytes([1; 16]);
/// let hub = MemorySyncPeer::new(StoreId::from_bytes([2; 16]));
/// hub.seed(EventGroup::new(None, vec![ReplicatedEvent::new(
///     EventId::new(tablet, SequencePosition::FIRST),
///     RecordedAt::from_millis(1_700_000_000_000),
///     Event::new("ReadingTaken", &b"{\"celsius\":4}"[..])?,
/// )]))?;
///
/// let mut from = None;
/// let mut pulls = 0;
/// loop {
///     let pulled = hub.pull(from.as_ref()).await?;
///     pulls += 1;
///     // ... ingest `pulled.batch` here, then keep the token ...
///     let caught_up = pulled.waiting == Waiting::Nothing || pulled.batch.groups.is_empty();
///     from = Some(pulled.resume);
///     if caught_up {
///         break;
///     }
/// }
/// assert_eq!(pulls, 1, "told nothing is waiting, the runner skips the empty pull");
/// # Ok(())
/// # }
/// ```
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Waiting {
    /// Nothing: no group lies past this answer's token, so the next pull from
    /// it returns an empty batch unless the far side takes a write in between
    /// (SY-37). A peer stalled behind a group it cannot carry says
    /// [`Events`](Self::Events), never this.
    Nothing,
    /// The peer did not say. The caller pulls again to find out, which is what
    /// every pull cost before this field existed.
    #[default]
    Unknown,
    /// This many events lay past the token when the far side answered.
    ///
    /// Advisory: more may have arrived since, and a group too large for the
    /// next pull's budget can hold that pull empty while this is non-zero.
    Events(core::num::NonZeroU64),
}

impl Waiting {
    /// [`Nothing`](Self::Nothing) for zero, [`Events`](Self::Events)
    /// otherwise: the count an adapter's `count(*)`, or its limit-plus-one
    /// probe, produced.
    #[must_use]
    pub const fn counted(remaining: u64) -> Self {
        match core::num::NonZeroU64::new(remaining) {
            Some(count) => Self::Events(count),
            None => Self::Nothing,
        }
    }
}

// `Pulled`'s constructors, declared here rather than beside the struct for the
// same reason as `Waiting`: the struct gained a field, and the lines below it
// keep their numbers.
impl<R> Pulled<R> {
    /// Pairs a batch with the token that follows it, saying
    /// [`Waiting::Unknown`].
    #[must_use]
    pub const fn new(batch: PushBatch, resume: R) -> Self {
        Self {
            batch,
            resume,
            waiting: Waiting::Unknown,
        }
    }

    /// Says what the far side still holds past [`resume`](Self::resume).
    ///
    /// A builder step rather than a third argument to [`new`](Self::new), so a
    /// peer that cannot tell keeps calling `new` and says
    /// [`Waiting::Unknown`], which is never a lie.
    #[must_use]
    pub const fn with_waiting(mut self, waiting: Waiting) -> Self {
        self.waiting = waiting;
        self
    }
}

#[cfg(test)]
mod waiting_tests {
    use core::num::NonZeroU64;

    use super::{Pulled, PushBatch, Waiting};

    #[test]
    fn counted_zero_is_nothing() {
        assert_eq!(Waiting::counted(0), Waiting::Nothing);
    }

    #[test]
    fn counted_n_is_events_n() {
        let three = NonZeroU64::new(3).expect("three is not zero");

        assert_eq!(Waiting::counted(3), Waiting::Events(three));
    }

    /// The default a stand-in peer gets for free costs a round trip and is
    /// never a lie; `Nothing` would be cheaper and, from a peer that did not
    /// look, false.
    #[test]
    fn pulled_new_says_unknown() {
        let pulled = Pulled::new(PushBatch::empty(), 7_u8);

        assert_eq!(pulled.waiting, Waiting::Unknown);
    }

    #[test]
    fn with_waiting_replaces_only_waiting() {
        let pulled = Pulled::new(PushBatch::empty(), 7_u8).with_waiting(Waiting::Nothing);

        assert_eq!(
            pulled,
            Pulled {
                batch: PushBatch::empty(),
                resume: 7_u8,
                waiting: Waiting::Nothing,
            }
        );
    }
}
