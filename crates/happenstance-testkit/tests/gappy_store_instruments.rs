//! The read-model handler that assumes `position + 1`, and the one that does
//! not.
//!
//! Positions are an opaque ordering key and the specification permits gaps
//! (VT-11), but no store a consumer can reach today leaves one: the reference
//! store allocates densely from 1, so the input that separates a correct
//! handler from an incorrect one never occurs. `GappyMemoryStore` is that
//! input.
//!
//! No assertion here names a literal position. Every claim is a *relation* —
//! strictly increasing, some consecutive pair separated by more than one —
//! measured against what the store returned, which is the discipline this crate
//! teaches adapter authors and would otherwise be violating in the file that
//! teaches it.
//!
//! Two conditions on the file, and neither is redundant. `memory` gates the
//! type itself, so `--no-default-features` must drop this file cleanly rather
//! than fail to build it; `not(wasm32)` is the `#[tokio::test]` harness.

#![cfg(all(not(target_arch = "wasm32"), feature = "memory"))]
#![allow(clippy::unwrap_used)]

use core::num::NonZeroU64;

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencePosition, collect};
use happenstance_testkit::GappyMemoryStore;

fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).unwrap()
}

/// Prime, and greater than one, so that an off-by-one in a caller and a genuine
/// gap can never be confused.
fn stride() -> NonZeroU64 {
    NonZeroU64::new(7).unwrap()
}

// ---------------------------------------------------------------------------
// AC-008 — the gaps are the permitted freedom, stated as a relation
// ---------------------------------------------------------------------------

/// Positions are unique, strictly increasing, and not dense.
#[tokio::test]
async fn assigned_positions_are_strictly_increasing_with_at_least_one_gap() {
    let store = GappyMemoryStore::with_stride(stride());

    let mut assigned = Vec::new();
    for event_type in ["Ay", "Bee", "Cee"] {
        assigned.push(store.append(&[event(event_type)], None).await.unwrap());
    }

    assert!(
        assigned.windows(2).all(|pair| pair[0] < pair[1]),
        "VT-11: positions are strictly increasing; got {assigned:?}"
    );
    assert!(
        assigned
            .windows(2)
            .any(|pair| pair[1].get() > pair[0].get().saturating_add(1)),
        "this instrument exists to leave a hole, and left none: {assigned:?}"
    );

    // And the log reads back in the same order the store assigned.
    let held = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    let read_back: Vec<SequencePosition> = held.iter().map(|event| event.position).collect();
    assert_eq!(read_back, assigned);
}

// ---------------------------------------------------------------------------
// AC-007 — the two handlers
// ---------------------------------------------------------------------------

/// The wrong handler's arithmetic, isolated so the test can name it.
///
/// "The next event is at the position after this one" is true of every store
/// most people have used, and is not true of any store the specification
/// describes.
fn next_position_by_addition(previous: SequencePosition) -> u64 {
    previous.get().saturating_add(1)
}

/// The computed position is not the position the store assigned.
#[tokio::test]
async fn handler_assuming_position_plus_one_disagrees_with_the_store() {
    let store = GappyMemoryStore::with_stride(stride());

    let first = store.append(&[event("Ay")], None).await.unwrap();
    let second = store.append(&[event("Bee")], None).await.unwrap();

    let computed = next_position_by_addition(first);

    assert_ne!(
        computed,
        second.get(),
        "the handler predicted where the next event would be and the store put \
         it somewhere else — which is what every conformant gapped adapter does"
    );

    let held = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .unwrap();
    assert!(
        !held.iter().any(|event| event.position.get() == computed),
        "and the position it computed belongs to no event at all: {computed}"
    );
}

/// The conformant sibling: resume past an **inclusive** checkpoint, and answer
/// "am I caught up?" by comparing with `head()` rather than by subtracting.
///
/// `ReadOptions::from` is inclusive, so resuming means advancing past the
/// checkpoint with `SequencePosition::next` — whose `None` arm is key-space
/// exhaustion and is treated as "there is nothing above this" rather than
/// unwrapped.
async fn process_every_event_once<S: EventStore>(store: &S) -> Vec<SequencePosition> {
    let mut checkpoint: Option<SequencePosition> = None;
    let mut processed = Vec::new();

    loop {
        let options = match checkpoint {
            None => ReadOptions::new().limit(1),
            Some(seen) => match seen.next() {
                Some(beyond) => ReadOptions::new().from(beyond).limit(1),
                None => return processed,
            },
        };

        let Ok(batch) = collect(store.read(&Query::all(), options)).await else {
            return processed;
        };
        if batch.is_empty() {
            return processed;
        }

        for event in &batch {
            processed.push(event.position);
            checkpoint = Some(event.position);
        }

        // Equality against the head, never `head - checkpoint`: the difference
        // between two positions is not a count of anything.
        if store.head().await.ok().flatten() == checkpoint {
            return processed;
        }
    }
}

#[tokio::test]
async fn handler_resuming_past_an_inclusive_checkpoint_sees_every_event_once() {
    let store = GappyMemoryStore::with_stride(stride());

    let mut assigned = Vec::new();
    for event_type in ["Ay", "Bee", "Cee", "Dee"] {
        assigned.push(store.append(&[event(event_type)], None).await.unwrap());
    }

    let processed = process_every_event_once(&store).await;

    assert_eq!(
        processed, assigned,
        "every event exactly once, in the order the store assigned them"
    );
}
