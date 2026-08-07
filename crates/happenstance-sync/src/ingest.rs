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
//! # What compiling it actually proved
//!
//! Two things, and it is worth being precise about which:
//!
//! * **The trait reaches a foreign store.** This crate writes
//!   `impl IngestStore for happenstance_core::MemoryEventStore` — local trait,
//!   foreign type — and it compiles with `happenstance-core` untouched. An
//!   adapter crate can write the mirror image, foreign trait for its own local
//!   type, and that also compiles. Both directions coherence allows are open.
//! * **A third crate cannot.** `happenstance-sync-testkit`, which will define
//!   neither the trait nor the store, is barred from writing the impl on an
//!   adapter's behalf — see the `compile_fail` example below. That is not a
//!   problem for a conformance suite, which takes an implementation rather than
//!   supplying one, but it does mean there is no blanket
//!   `impl<S: EventStore> IngestStore for S` waiting to make this free.
//!
//! And one thing it did **not** prove, which is the finding rather than the
//! reassurance: the impl below cannot be written *truthfully*.
//! [`SequencedEvent`](happenstance_core::SequencedEvent) carries a position and
//! an event and nothing else, so `MemoryEventStore` has nowhere to put an
//! [`EventId`] once it has accepted one, and nowhere to read one back from to
//! answer [`holds`](IngestStore::holds). The body is `todo!()` and would still
//! be `todo!()` with unlimited time.
//!
//! So the trait seam is genuinely discharged and the **value type** seam is not.
//! `EventStore::append` does not need to change; `SequencedEvent` does. That is
//! a smaller leak than the warning claimed and a real one, and it is a claim
//! about a struct's fields rather than about a port's signature — which is the
//! cheaper of the two to land.

use happenstance_core::SequencePosition;

use crate::identity::{EventId, ReplicatedEvent, StoreId, Watermark};

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
    /// peer minted", which is the first check ingest makes and the one that
    /// stops a replication loop.
    fn store_id(&self) -> StoreId;

    /// Records events another store minted, preserving their identity.
    ///
    /// Each group lands atomically or not at all, and events land **at the
    /// tail**: an ingested event is assigned the next local position like any
    /// other write. Inserting it at the position its origin gave it would
    /// rewrite history under a projection that has already read past it.
    ///
    /// Re-delivery is a no-op. A peer will offer the same event more than once —
    /// after a dropped connection, after a resume token that did not advance,
    /// after an operator re-seeds a device — and each repeat must be skipped
    /// rather than appended again or rejected.
    ///
    /// # Errors
    ///
    /// The adapter's error, for storage failures only. An event this store
    /// already holds is a skip and not an error; disagreeing with an event's
    /// content is not available as an option, because the event is already
    /// durable somewhere else and refusing it only guarantees the two logs never
    /// converge.
    async fn ingest(&self, events: &[ReplicatedEvent]) -> Result<Ingested, Self::Error>;

    /// Whether this store already holds `id`.
    ///
    /// A dedicated operation rather than a [`Query`](happenstance_core::Query)
    /// dimension, and that is deliberate. Identity is outside the query
    /// language: a decision model that could filter on an `EventId` would be a
    /// decision model whose consistency boundary depends on which store it is
    /// running against, which is precisely what a replicated system cannot
    /// afford.
    ///
    /// # Errors
    ///
    /// The adapter's error if the lookup fails.
    async fn holds(&self, id: &EventId) -> Result<bool, Self::Error>;

    /// The highest position this store has ingested from each origin store.
    ///
    /// This is what a spoke sends as its resume position and what a hub answers
    /// a pull against. It is derived from what was actually ingested rather than
    /// stored as a separate counter, so it cannot drift from the log.
    ///
    /// # Errors
    ///
    /// The adapter's error if the watermark cannot be computed.
    async fn watermark(&self) -> Result<Watermark, Self::Error>;
}

/// What one [`ingest`](IngestStore::ingest) did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Ingested {
    /// Events appended to the local log.
    pub appended: usize,
    /// Events skipped because this store already held their [`EventId`].
    pub skipped: usize,
    /// The local position of the last appended event, if any were appended.
    ///
    /// Local, and unrelated to the position inside the events' own
    /// [`EventId`]s: this is arrival order here, that is authorship order there.
    pub last_local: Option<SequencePosition>,
}

/// The coherence proof, compiled.
///
/// `MemoryEventStore` belongs to `happenstance-core` and [`IngestStore`] belongs
/// here, so this impl is the "local trait, foreign type" half of the orphan
/// rule. It is what demonstrates that a foreign identity can reach a store
/// without `happenstance-core` changing a line.
///
/// The bodies are `todo!()` and they are not merely unfinished. See this
/// module's documentation: `SequencedEvent` has no field an `EventId` fits in,
/// so no honest body exists until the contract crate grows one. That is the
/// residue of the leak, and it is a struct field rather than a port signature.
#[cfg(feature = "memory")]
mod memory_store_ingest {
    use happenstance_core::MemoryEventStore;

    use super::{EventId, Ingested, ReplicatedEvent, SendIngestStore, StoreId, Watermark};

    impl SendIngestStore for MemoryEventStore {
        type Error = happenstance_core::MemoryStoreError;

        fn store_id(&self) -> StoreId {
            todo!("MemoryEventStore has no incarnation identifier to report")
        }

        async fn ingest(&self, _events: &[ReplicatedEvent]) -> Result<Ingested, Self::Error> {
            todo!("SequencedEvent carries no EventId, so identity cannot be preserved")
        }

        async fn holds(&self, _id: &EventId) -> Result<bool, Self::Error> {
            todo!("SequencedEvent carries no EventId, so there is nothing to match on")
        }

        async fn watermark(&self) -> Result<Watermark, Self::Error> {
            todo!("SequencedEvent carries no EventId, so no origin can be attributed")
        }
    }
}
