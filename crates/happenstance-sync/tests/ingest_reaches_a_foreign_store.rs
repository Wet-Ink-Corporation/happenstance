//! Does a foreign identity reach a store without `happenstance-core` changing?
//!
//! The warning this file answers is that deferring the sync port *"leaks
//! `EventId` and a tail seam back into `EventStore`"*. The claimed discharge is
//! that identity arrives through an `IngestStore` defined in this crate rather
//! than through `EventStore::append`, and coherence is the mechanism. Compiling
//! it is the only way to know.
//!
//! There are three crates in the argument and each gets a different answer, so
//! all three are exercised:
//!
//! 1. **The crate that defines the trait** — `happenstance-sync` — writing
//!    `impl SendIngestStore for happenstance_core::MemoryEventStore`. Local
//!    trait, foreign type. Allowed, and it is in `src/ingest.rs`.
//! 2. **The crate that defines the store** — an adapter — writing the impl for
//!    its own type. Foreign trait, local type. Allowed, and this file is that
//!    crate: an integration test is a separate compilation unit, so
//!    `HypotheticalAdapterStore` below stands in exactly the relationship a real
//!    adapter crate stands in.
//! 3. **Any third crate** — the future `happenstance-sync-testkit`, say.
//!    **Refused.** Attempted and captured:
//!
//! ```text
//! error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
//!  --> crates\happenstance-sync\tests\orphan_probe.rs:4:1
//!   |
//! 4 | impl IngestStore for happenstance_core::MemoryEventStore {
//!   | ^^^^^^^^^^^^^^^^^^^^^-----------------------------------
//!   |                      |
//!   |                      `MemoryEventStore` is not defined in the current crate
//!   |
//!   = note: impl doesn't have any local type before any uncovered type parameters
//!   = note: define and implement a trait or new type instead
//! ```
//!
//! That third answer is fine for a conformance suite, which takes an
//! implementation rather than supplying one. It does rule out the shortcut
//! somebody will eventually propose — a blanket
//! `impl<S: EventStore> IngestStore for S` in a testkit — and it rules it out
//! for a reason no amount of design can route around.
//!
//! # The residue
//!
//! The *trait* seam holds. The **value type** seam does not: this store has to
//! invent a place to keep an `EventId` because
//! [`SequencedEvent`](happenstance_core::SequencedEvent) has no field for one,
//! and every real adapter will have to invent the same place. So
//! `EventStore::append` does not need to change and `SequencedEvent` does — a
//! smaller leak than the warning claimed, arriving on a struct's fields rather
//! than on a port's signature, which is the cheaper of the two to land.

#![allow(clippy::unwrap_used)]

use std::collections::BTreeMap;
use std::sync::Mutex;

use happenstance_core::{Event, SequencePosition};
// The three identity types are spelled through `identity::` on purpose: they are
// phase 4's to define, and this import is one of the sites phase 4 rewrites.
use happenstance_sync::identity::{EventId, RecordedAt, StoreId};
use happenstance_sync::{IngestStore, Ingested, ReplicatedEvent, Watermark};

/// Stands in for a store adapter crate's own type.
///
/// The side table is the finding. `SequencedEvent` carries a position and an
/// event, so an adapter that wants to preserve a foreign `EventId` has to keep
/// it *beside* the log — and now two structures have to stay consistent through
/// every crash, which is the sort of thing a contract type exists to prevent.
#[derive(Debug)]
struct HypotheticalAdapterStore {
    identity: StoreId,
    /// Local position → the identity the origin store minted. The thing
    /// `SequencedEvent` has nowhere to put.
    minted_elsewhere: Mutex<BTreeMap<u64, EventId>>,
    next_position: Mutex<u64>,
}

#[derive(Debug, thiserror::Error)]
enum AdapterError {
    #[error("the store's side table lock was poisoned")]
    Poisoned,
    #[error("the store has issued every position it can")]
    PositionSpaceExhausted,
}

impl IngestStore for HypotheticalAdapterStore {
    type Error = AdapterError;

    fn store_id(&self) -> StoreId {
        self.identity
    }

    async fn ingest(&self, events: &[ReplicatedEvent]) -> Result<Ingested, Self::Error> {
        let mut table = self
            .minted_elsewhere
            .lock()
            .map_err(|_| AdapterError::Poisoned)?;
        let mut next = self
            .next_position
            .lock()
            .map_err(|_| AdapterError::Poisoned)?;

        let mut outcome = Ingested::default();
        for event in events {
            if table.values().any(|held| *held == event.id) {
                outcome.skipped += 1;
                continue;
            }

            // At the tail, always. Placing an ingested event at the position its
            // origin gave it would rewrite history underneath a projection that
            // has already read past it.
            *next = next
                .checked_add(1)
                .ok_or(AdapterError::PositionSpaceExhausted)?;
            table.insert(*next, event.id);
            outcome.appended += 1;
            outcome.last_local = SequencePosition::new(*next);
        }

        Ok(outcome)
    }

    async fn holds(&self, id: &EventId) -> Result<bool, Self::Error> {
        let table = self
            .minted_elsewhere
            .lock()
            .map_err(|_| AdapterError::Poisoned)?;
        Ok(table.values().any(|held| held == id))
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        let table = self
            .minted_elsewhere
            .lock()
            .map_err(|_| AdapterError::Poisoned)?;
        let mut watermark = Watermark::new();
        for id in table.values() {
            watermark.advance(id.store(), id.position());
        }
        Ok(watermark)
    }
}

fn replicated(origin: StoreId, position: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::new(position).unwrap()),
        RecordedAt::from_millis(1_700_000_000_000 + position),
        Event::new("StudentSubscribed", "{}").unwrap(),
    )
}

#[tokio::test]
async fn a_foreign_identity_reaches_a_store_through_a_trait_this_crate_owns() {
    let origin = StoreId::from_bytes([7; 16]);
    let store = HypotheticalAdapterStore {
        identity: StoreId::from_bytes([9; 16]),
        minted_elsewhere: Mutex::default(),
        next_position: Mutex::default(),
    };

    let batch = [replicated(origin, 4), replicated(origin, 5)];
    let first = store.ingest(&batch).await.unwrap();
    assert_eq!(first.appended, 2);
    assert_eq!(first.skipped, 0);

    // Idempotence: re-delivery is a no-op, not an error and not a duplicate.
    let again = store.ingest(&batch).await.unwrap();
    assert_eq!(again.appended, 0);
    assert_eq!(again.skipped, 2);

    assert!(store.holds(&batch[0].id).await.unwrap());
    assert_eq!(
        store.watermark().await.unwrap().get(origin),
        SequencePosition::new(5)
    );

    // Arrival order here is unrelated to authorship order there: positions 4
    // and 5 at the origin landed at 1 and 2 locally.
    assert_eq!(first.last_local, SequencePosition::new(2));
    assert_ne!(store.store_id(), origin);
}
