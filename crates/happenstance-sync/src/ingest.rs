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
//! reassurance: the impl below still cannot be written *truthfully* — though no
//! longer for the reason first recorded here, and the move is worth following.
//!
//! The original obstruction was the **value type**.
//! [`SequencedEvent`](happenstance_core::SequencedEvent) carried a position and
//! an event and nothing else, so there was nowhere to put an identity a peer had
//! minted. Phase 4 closed that: it now carries `position`, `id`, `recorded_at`
//! and `event`, where `id` is a `happenstance_core::EventId` — the same
//! `(store, position)` pair this crate's placeholder [`EventId`] sketches, in a
//! different crate.
//!
//! What remains is the **write path**, one layer in. The only `&self` operation
//! that adds to a store is
//! [`EventStore::append`](happenstance_core::EventStore::append), and
//! `MemoryEventStore` mints `EventId::new(self.store_id, position)` for every
//! event it writes. That is not an oversight to route around: `happenstance-core`
//! states that no store-assigned value is ever supplied by a caller through
//! `append`, which is exactly the property that keeps the contract's write path
//! from having to distinguish "I decided this" from "somebody else did and I am
//! copying it". `MemoryEventStore::restore` does preserve a foreign identity, and
//! it is no help here: it builds a **new** store out of an owned snapshot, while
//! [`ingest`](IngestStore::ingest) holds `&self` on an existing one. So a foreign
//! identity has a place to sit and no door to come in through, and the bodies
//! below would still be `todo!()` with unlimited time.
//!
//! So the trait seam is genuinely discharged and the **write path** seam is not.
//! `EventStore::append` still does not need to change — that is the whole result
//! — but the store's own crate has to offer *some* operation that accepts an
//! identity it did not mint, because coherence lets this crate add a trait to a
//! foreign type and never lets it reach inside one. That is a smaller leak than
//! the warning claimed and a real one, and it has shrunk twice: first from a
//! port signature to a struct's fields, and now from a struct's fields to one
//! missing store operation.
//!
//! [`holds`](IngestStore::holds) is the one method the write path does not
//! block, and naming it is what keeps the finding honest rather than sweeping.
//! `EventStore::contains_event_id` landed alongside the identity fields and
//! would answer it exactly; the only obstruction there is that this crate's
//! placeholder [`EventId`] is a *different type* from the contract's, which
//! [`crate::identity`] already records as phase 4's to remove. It is left
//! `todo!()` with the others because an [`IngestStore`] whose
//! [`ingest`](IngestStore::ingest) cannot run has nothing for `holds` to be
//! true about.

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
/// module's documentation: `SequencedEvent` does now have a field an identity
/// fits in, and `MemoryEventStore` still has no operation that puts a *foreign*
/// one there — `append` mints its own for every event it writes, and `restore`
/// builds a whole new store. That is the residue of the leak, and it is a
/// missing write path rather than a port signature.
#[cfg(feature = "memory")]
mod memory_store_ingest {
    use happenstance_core::MemoryEventStore;

    use super::{EventId, Ingested, ReplicatedEvent, SendIngestStore, StoreId, Watermark};

    impl SendIngestStore for MemoryEventStore {
        type Error = happenstance_core::MemoryStoreError;

        fn store_id(&self) -> StoreId {
            todo!("MemoryEventStore::store_id() answers this, as a happenstance_core::StoreId")
        }

        async fn ingest(&self, _events: &[ReplicatedEvent]) -> Result<Ingested, Self::Error> {
            todo!("append mints an EventId per event, so no foreign identity can be preserved")
        }

        async fn holds(&self, _id: &EventId) -> Result<bool, Self::Error> {
            todo!("contains_event_id would answer this, once the placeholder EventId is unified")
        }

        async fn watermark(&self) -> Result<Watermark, Self::Error> {
            todo!("a watermark is derived from ingested events, and nothing can be ingested")
        }
    }
}
