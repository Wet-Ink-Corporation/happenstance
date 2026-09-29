//! Does a foreign identity reach a store without `happenstance-core` changing?
//!
//! The warning this file answers is that deferring the sync port *"leaks
//! `EventId` and a tail seam back into `EventStore`"*. The claimed discharge is
//! that identity arrives through an `IngestStore` defined in this crate rather
//! than through `EventStore::append`, and coherence is the mechanism. Compiling
//! it is the only way to know.
//!
//! There are three crates in the argument and each gets a different answer, so
//! all three were exercised:
//!
//! 1. **The crate that defines the trait** — `happenstance-sync` — writing
//!    `impl SendIngestStore for happenstance_core::MemoryEventStore`. Local
//!    trait, foreign type. Allowed, and it compiled; it lived in
//!    `src/ingest.rs` until phase 17 deleted it, because every body was
//!    `todo!()` for a reason no amount of time would fix — the trait can be
//!    *added* to a foreign store and cannot reach *inside* it.
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
//! # The residue, and where it closed
//!
//! This file first recorded a **value type** residue: the stand-in had to keep
//! a side table of foreign identities because
//! [`SequencedEvent`](happenstance_core::SequencedEvent) had no field for one.
//! Phase 4 closed that — `SequencedEvent` carries `id` and `recorded_at` — and
//! the stand-in below keeps the identity *in* its log, as every real schema in
//! this workspace does.
//!
//! What remained was the **write path**: some operation has to accept an
//! identity the store did not mint, and `append` never will. Case 2 is where it
//! lives. The adapter owns its store's internals, so it can generalise the row
//! writer `append` already uses to take a per-row origin, and
//! `crates/happenstance-sqlite/src/ingest_spike.rs` does exactly that for a real
//! store. `EventStore::append` did not change, `happenstance-core` did not grow,
//! and the leak the warning predicted has nowhere left to arrive.

#![allow(clippy::unwrap_used)]

use std::sync::Mutex;

use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{IngestGroup, IngestStore, Ingested, ReplicatedEvent, Watermark};

/// Stands in for a store adapter crate's own type.
///
/// Each row carries the identity and recorded time it was written with, beside
/// the local position that orders it here — the shape of every real schema,
/// where origin is kept apart from position.
#[derive(Debug)]
struct HypotheticalAdapterStore {
    identity: StoreId,
    log: Mutex<Vec<Row>>,
}

#[derive(Debug, Clone)]
struct Row {
    position: SequencePosition,
    id: EventId,
    recorded_at: RecordedAt,
}

#[derive(Debug, thiserror::Error)]
enum AdapterError {
    #[error("the store's log lock was poisoned")]
    Poisoned,
    #[error("the store has issued every position it can")]
    PositionSpaceExhausted,
}

impl HypotheticalAdapterStore {
    fn new(identity: StoreId) -> Self {
        Self {
            identity,
            log: Mutex::default(),
        }
    }

    fn rows(&self) -> Vec<Row> {
        self.log.lock().unwrap().clone()
    }
}

/// The next local position after `log`'s tail.
fn next_position(log: &[Row]) -> Result<SequencePosition, AdapterError> {
    match log.last() {
        None => Ok(SequencePosition::FIRST),
        Some(row) => row
            .position
            .next()
            .ok_or(AdapterError::PositionSpaceExhausted),
    }
}

impl IngestStore for HypotheticalAdapterStore {
    type Error = AdapterError;

    fn store_id(&self) -> StoreId {
        self.identity
    }

    async fn ingest(&self, groups: &[IngestGroup<'_>]) -> Result<Ingested, Self::Error> {
        // One lock over the whole batch: the stand-in's transaction. The
        // duplicate check and the write happen inside it, which is the property
        // a real adapter buys from its unique index on the origin pair.
        let mut log = self.log.lock().map_err(|_| AdapterError::Poisoned)?;
        let mut outcome = Ingested::default();

        for group in groups {
            let before = outcome.appended;
            for event in group.events {
                if log.iter().any(|row| row.id == event.id) {
                    outcome.skipped += 1;
                    continue;
                }

                // At the tail, always. Placing an ingested event at the position
                // its origin gave it would rewrite history underneath a
                // projection that has already read past it.
                let position = next_position(&log)?;
                log.push(Row {
                    position,
                    id: event.id,
                    recorded_at: event.recorded_at,
                });
                outcome.appended += 1;
            }

            // Compensation answers new events only: a redelivered group already
            // had its answer written the first time it landed.
            if outcome.appended == before {
                continue;
            }
            for _ in group.compensation {
                let position = next_position(&log)?;
                log.push(Row {
                    position,
                    id: EventId::new(self.identity, position),
                    recorded_at: RecordedAt::from_millis(1_800_000_000_000),
                });
                outcome.compensated += 1;
            }
        }

        Ok(outcome)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        let log = self.log.lock().map_err(|_| AdapterError::Poisoned)?;
        let mut watermark = Watermark::new();
        for row in log.iter() {
            watermark.advance(row.id.store(), row.id.position());
        }
        Ok(watermark)
    }
}

fn replicated(origin: StoreId, position: u64, recorded_at: i64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::new(position).unwrap()),
        RecordedAt::from_millis(recorded_at),
        Event::new("StudentSubscribed", "{}").unwrap(),
    )
}

#[tokio::test]
async fn a_foreign_identity_reaches_a_store_through_a_trait_this_crate_owns() {
    let origin = StoreId::from_bytes([7; 16]);
    let store = HypotheticalAdapterStore::new(StoreId::from_bytes([9; 16]));

    // One recording before 1970, which the contract's `i64` carries and the
    // `u64` placeholder this crate used to define could not.
    let events = [
        replicated(origin, 4, -86_400_000),
        replicated(origin, 5, 1_700_000_000_000),
    ];
    let compensation = [Event::new("SubscriptionReversed", "{}").unwrap()];
    let batch = [IngestGroup::new(&events, &compensation)];

    let first = store.ingest(&batch).await.unwrap();
    assert_eq!(first.appended, 2);
    assert_eq!(first.skipped, 0);
    assert_eq!(first.compensated, 1);

    // Idempotence: re-delivery is a no-op, not an error and not a duplicate —
    // and the compensation is not written twice either.
    let again = store.ingest(&batch).await.unwrap();
    assert_eq!(again.appended, 0);
    assert_eq!(again.skipped, 2);
    assert_eq!(again.compensated, 0);

    // The foreign identity and recorded time land unchanged; the compensation
    // carries this store's identity.
    let rows = store.rows();
    assert_eq!(rows.len(), 3);
    for (row, event) in rows.iter().zip(&events) {
        assert_eq!(row.id, event.id);
        assert_eq!(row.recorded_at, event.recorded_at);
    }
    assert_eq!(rows[2].id.store(), store.store_id());

    // Arrival order here is unrelated to authorship order there, and the
    // compensation lands after the events it answers.
    assert!(rows[0].position < rows[1].position);
    assert!(rows[1].position < rows[2].position);

    let watermark = store.watermark().await.unwrap();
    assert_eq!(watermark.get(origin), Some(events[1].id.position()));
    assert_eq!(watermark.get(store.store_id()), Some(rows[2].id.position()));
    assert_ne!(store.store_id(), origin);
}
