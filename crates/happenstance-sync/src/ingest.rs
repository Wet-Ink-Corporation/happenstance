//! The ingest port: how a foreign identity reaches a local store.
//!
//! # The leak this exists to close
//!
//! Deferring the sync port was said to *"leak `EventId` and a tail seam back
//! into `EventStore`"*. The reasoning is easy to follow and it is worth stating
//! before the counter-argument: a replicated event arrives carrying an identity
//! its origin store minted, that identity must be preserved rather than
//! re-minted, and the only operation that writes to a store is
//! [`EventStore::append`](happenstance_core::EventStore::append) — so `append`
//! grows a `Option<EventId>` parameter, every local caller pays for a concept it
//! never uses, and replication has reached into the contract crate.
//!
//! The escape is **coherence**. Rust's orphan rule says an implementation may be
//! written by the crate that defines the trait or by the crate that defines the
//! type, and by nobody else. Turn that around and it is a growth mechanism: a
//! store adapter can acquire a new capability from *any* crate that defines a
//! trait for it, and the crate that defines the store port never has to hear
//! about it. [`IngestStore`] is that trait, it lives here, and `append` keeps
//! its signature.
//!
//! # Where the leak went
//!
//! It shrank three times, and the last one closed it.
//!
//! **The value type.** [`SequencedEvent`](happenstance_core::SequencedEvent)
//! once carried a position and an event and nothing else, so there was nowhere
//! to put an identity a peer had minted. Phase 4 closed that: it carries
//! `position`, `id`, `recorded_at` and `event`, and this module speaks the same
//! [`EventId`](happenstance_core::EventId) and
//! [`RecordedAt`](happenstance_core::RecordedAt) rather than placeholders of
//! them.
//!
//! **The write path.** That left an identity with a place to sit and no door to
//! come in through. The only `&self` operation that adds to a store is
//! `append`, and `append` mints — `happenstance-core` states that no
//! store-assigned value is ever supplied by a caller through it, which is the
//! property that keeps the contract's write path from having to distinguish "I
//! decided this" from "somebody else did and I am copying it". This crate used
//! to write `impl SendIngestStore for MemoryEventStore` — local trait, foreign
//! type — and it compiled, and every body was `todo!()`, because coherence lets
//! a crate add a trait to a foreign type and never lets it reach inside one.
//! `MemoryEventStore` mints `EventId::new(self.store_id, position)` for every
//! event it writes, and nothing outside `happenstance-core` can make it do
//! otherwise.
//!
//! **Whose door it is.** Read that last sentence the other way round and it is
//! the answer: the write path that accepts a foreign identity belongs to the
//! crate that owns the store's internals, which is the adapter — and the
//! adapters' schemas were built to take it. Each already keeps origin apart from
//! position: SQLite and the Durable Object insert with the origin columns `NULL`
//! and stamp local identity afterwards through an `IS NULL` marker that exists
//! so an ingested row is *not* restamped, and Postgres and Neon write the origin
//! pair explicitly under a unique index on it. So the ingest path need not be a
//! second write path beside `append`: it can be the adapter's existing row
//! writer, generalised to take a per-row origin, with what is ingest-only small
//! enough to list — the deduplicating conflict clause, a per-row origin and
//! recorded time, and the absence of any condition evaluation (SY-1).
//!
//! Two adapters show it, and not to the same standard. SQLite runs it: append
//! and ingest prepare one `INSERT` in one writer and differ only in the values
//! bound. Neon only spells it: its ingest statement is composed from the insert
//! builder its append uses, and has never been sent to a server. Postgres and
//! the Durable Object are argued from their schemas above and have not been
//! built.
//!
//! The instrument is `crates/happenstance-sqlite/src/ingest_spike.rs`: this
//! trait implemented for `SqliteEventStore`, in the adapter's own crate, over
//! the same private writer `append` uses. `happenstance-core` grows nothing for
//! it and no published signature changes. That spike is the run VT-10's
//! falsifier names — *"a store adapter cannot implement `IngestStore` without
//! duplicating append's write path"*, with SQLite as the instrument — and the
//! phase-17 record states what it found.
//!
//! # What coherence still refuses
//!
//! **A third crate cannot write the impl.** `happenstance-sync-testkit`, which
//! will define neither the trait nor the store, is barred from writing it on an
//! adapter's behalf — see the `compile_fail` example on [`IngestStore`]. That is
//! not a problem for a conformance suite, which takes an implementation rather
//! than supplying one, but it does mean there is no blanket
//! `impl<S: EventStore> IngestStore for S` waiting to make this free — and after
//! the paragraphs above, no reason to want one: a blanket impl could only call
//! `append`, and `append` re-mints.
//!
//! The in-memory store is the one this leaves without an implementation.
//! `MemoryEventStore` lives in the contract crate, so the adapter-owned answer
//! would put an ingest operation in `happenstance-core` — the letter VT-10
//! forbids even where it would be additive. Its oracle is phase 13's, as a
//! store this crate owns.
//!
//! # Membership is not on this trait
//!
//! It used to carry `holds(&EventId)`. That question is
//! [`EventStore::contains_event_id`](happenstance_core::EventStore::contains_event_id),
//! which landed with the identity fields and answers it exactly, so a runner
//! binds `S: EventStore + IngestStore` and asks the store it already has. It is
//! also not a question [`ingest`](IngestStore::ingest) needs asked first:
//! deduplication happens *inside* the write, against the store's uniqueness on
//! the origin pair (VT-8), and a probe-then-write would be the race that clause
//! forbids.

use happenstance_core::{Event, StoreId};

use crate::identity::{ReplicatedEvent, Watermark};

/// A store that can accept events another store already minted.
///
/// Separate from [`EventStore`](happenstance_core::EventStore) on purpose. The
/// two operations look similar and are not: an append is a *decision*, taken
/// now, against a condition checked now, by a store that assigns the identity.
/// An ingest is the recording of a decision somebody else already took and
/// already made durable, and the identity came with it.
///
/// Only a peer runner should hold one of these. Handing an `IngestStore` to
/// application code hands it the ability to forge history.
///
/// # A third crate cannot supply this impl
///
/// Coherence permits this trait's own crate, or a store's own crate, to write
/// the implementation. It permits nobody else, which is worth knowing before
/// someone plans on a blanket impl in a testkit:
///
/// ```compile_fail,E0117
/// use happenstance_sync::IngestStore;
///
/// // Foreign trait, foreign type, third crate. The orphan rule refuses.
/// impl IngestStore for happenstance_core::Event {
///     type Error = core::convert::Infallible;
/// }
/// ```
///
/// # Flavours
///
/// This is the `!Send` flavour, and generic code should bind it: the ingest path
/// on the Cloudflare side is single-threaded and cannot satisfy a `Send` bound
/// at all. Adapters that can cross threads implement [`SendIngestStore`] and get
/// this for free.
#[trait_variant::make(SendIngestStore: Send)]
pub trait IngestStore {
    /// How this store fails to ingest.
    type Error: core::error::Error + 'static;

    /// This store's own incarnation identifier.
    ///
    /// Needed to tell "an event I minted, coming back to me" from "an event a
    /// peer minted". The first, while this store still holds it, is a skip like
    /// any other re-delivery. One that claims this store's identity and is
    /// *not* held means the store was restored or rewound under an identity it
    /// should have re-minted (VT-6), and what ingest does with it is phase 13's
    /// to settle.
    fn store_id(&self) -> StoreId;

    /// Records events other stores minted, preserving their identity, together
    /// with any compensation this side writes for them.
    ///
    /// One call per pulled batch, not per event. **Each group lands atomically
    /// or not at all**, compensation included; a store may commit the whole
    /// batch as one unit, and one that can express it as one statement should.
    ///
    /// Events land **at the tail**: an ingested event is assigned the next local
    /// position like any other write, and keeps its origin's
    /// [`EventId`](happenstance_core::EventId) and
    /// [`RecordedAt`](happenstance_core::RecordedAt) unchanged. Inserting it at
    /// the position its origin gave it would rewrite history under a projection
    /// that has already read past it.
    ///
    /// No append condition is evaluated, the origin's or this store's (SY-1). A
    /// conflict is answered by the group's
    /// [`compensation`](IngestGroup::compensation), which is minted here, as
    /// ordinary local events, in the same transaction as the events it answers.
    ///
    /// Re-delivery is a no-op. A peer will offer the same event more than once —
    /// after a dropped connection, after a resume token that did not advance,
    /// after an operator re-seeds a device — and each repeat is skipped rather
    /// than appended again or rejected. The skip is decided inside the write,
    /// against the store's own uniqueness on the origin pair (VT-8), never by a
    /// lookup ahead of it. A group whose foreign events were all skipped writes
    /// no compensation either (SY-11): the first delivery already wrote it.
    ///
    /// # Errors
    ///
    /// The adapter's error, for storage failures and for capacity refusals —
    /// neither of which is a judgement on the events. An event this store
    /// already holds is a skip and not an error; disagreeing with an event's
    /// content is not available as an option, because the event is already
    /// durable somewhere else and refusing it only guarantees the two logs never
    /// converge.
    async fn ingest(&self, groups: &[IngestGroup<'_>]) -> Result<Ingested, Self::Error>;

    /// The highest origin position this store holds from each origin store.
    ///
    /// This is what a spoke sends as its resume position and what a hub answers
    /// a pull against. It is derived from the log rather than stored as a
    /// separate counter, so it cannot drift from what was actually written.
    ///
    /// # Errors
    ///
    /// The adapter's error if the watermark cannot be computed.
    async fn watermark(&self) -> Result<Watermark, Self::Error>;
}

/// One atomic unit of an [`ingest`](IngestStore::ingest): foreign events, and
/// the local compensation this side writes for them.
///
/// Borrowed rather than owned. The runner already holds the pulled batch, and
/// an ingest that took `Vec`s would make it clone every payload to hand them
/// over for the length of one call.
///
/// `#[non_exhaustive]`, so a group is built with [`IngestGroup::new`] outside
/// this crate. What travels beside a group is phase 13's to finish — the
/// origin's guard is evidence (SY-6) that has nowhere to go here yet — and a
/// field added then must not break every literal written before it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct IngestGroup<'a> {
    /// Events another store minted, each carrying its origin identity and
    /// recorded time.
    pub events: &'a [ReplicatedEvent],
    /// Events this store writes in answer to them, minted here.
    ///
    /// Written in the same transaction as [`events`](Self::events) (SY-2), and
    /// only when at least one of those was new (SY-11). Empty for a group that
    /// needs no answer, which is most of them.
    pub compensation: &'a [Event],
}

impl<'a> IngestGroup<'a> {
    /// A group of foreign `events`, answered by `compensation`.
    #[must_use]
    pub const fn new(events: &'a [ReplicatedEvent], compensation: &'a [Event]) -> Self {
        Self {
            events,
            compensation,
        }
    }
}

/// What one [`ingest`](IngestStore::ingest) did, summed across its groups.
///
/// Counts, not positions. A runner decides what to do next from how much landed
/// and how much was already here; where it landed locally is arrival order,
/// unrelated to anything the runner tells a peer (SY-19), and a store that
/// writes a batch in one statement should not be made to report per-row
/// positions to satisfy a field nobody reads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Ingested {
    /// Foreign events appended to the local log.
    pub appended: usize,
    /// Foreign events skipped because this store already held their identity.
    pub skipped: usize,
    /// Compensation events written, minted here.
    pub compensated: usize,
}
