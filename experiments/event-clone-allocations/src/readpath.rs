//! `MemoryEventStore::read`, and the one-line change H2 proposes to it.
//!
//! # Why there are replicas at all
//!
//! The fix under measurement is a change to
//! `crates/happenstance-core/src/memory.rs`, and this experiment may not edit
//! `crates/`. So the *before* shape is reproduced here character-for-character
//! from `memory.rs:302-333` at `56ef6c5` ([`read_before`]) and the *after* shape
//! is the same function with `.take(limit)` moved above `.cloned()`
//! ([`read_after`]).
//!
//! A replica is a claim, so it is checked rather than asserted:
//! `tests/arms_are_equivalent.rs` runs [`read_before`] and the **real**
//! `MemoryEventStore::read` over the same `SequencedEvent` values across a grid
//! of options and requires the outputs to be equal, and requires
//! [`read_after`]'s output to equal [`read_before`]'s. If the replica drifted
//! from `memory.rs`, that test fails and no figure is emitted.
//!
//! # Why the futures are driven by hand
//!
//! A runtime would allocate inside the measured region and its allocations would
//! be indistinguishable from the store's. [`drain`] and [`block_on_ready`] pin
//! on the stack with `core::pin::pin!` and poll with `Waker::noop()`, so driving
//! the stream costs zero heap operations and the counters see only the store.
//! `MemoryEventStore` never returns `Pending` — `read` is not `async` at all
//! (ADR-0008), and `append`'s body has no await point that can suspend — so both
//! drivers panic rather than spin if one ever does, which would be a finding of
//! its own.

use core::future::Future;
use core::task::{Context, Poll, Waker};

use futures_core::Stream;
use happenstance_core::{Event, EventStore, MemoryEventStore, Query, ReadOptions, SequencedEvent};

use crate::arms::{Payload, Regime, event_type, payload, tags};

/// Polls `future` exactly once and unwraps the result.
///
/// # Panics
///
/// Panics if the future returns `Pending`, which for `MemoryEventStore` would
/// mean the store had acquired a suspension point it does not have.
pub fn block_on_ready<F: Future>(future: F) -> F::Output {
    let mut future = core::pin::pin!(future);
    let mut cx = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("the in-memory store yielded, which it cannot do"),
    }
}

/// Drives a read stream to completion, collecting into a `Vec`.
///
/// The `Vec` starts empty and grows, rather than being pre-sized: pre-sizing it
/// would hand the *before* arm a buffer the real code path does not have, and
/// the growth is a handful of reallocations against millions of allocations, so
/// it changes no conclusion. It is disclosed in the README's caveats regardless.
///
/// # Panics
///
/// Panics if the stream yields an error or returns `Pending`.
pub fn drain<E, S>(stream: S) -> Vec<SequencedEvent>
where
    E: core::fmt::Debug,
    S: Stream<Item = Result<SequencedEvent, E>>,
{
    let mut stream = core::pin::pin!(stream);
    let mut cx = Context::from_waker(Waker::noop());
    let mut out = Vec::new();
    loop {
        match stream.as_mut().poll_next(&mut cx) {
            Poll::Ready(Some(Ok(event))) => out.push(event),
            Poll::Ready(Some(Err(error))) => panic!("the in-memory store failed: {error:?}"),
            Poll::Ready(None) => return out,
            Poll::Pending => panic!("the in-memory store yielded, which it cannot do"),
        }
    }
}

/// `memory.rs:302-333` at `56ef6c5`, reproduced.
///
/// Filter by query, filter by bounds, **clone every survivor**, collect, and
/// only then truncate to `limit`. The comment on the truncate in `memory.rs` is
/// about ES-14's *ordering* obligation and is correct; what it does not say is
/// that everything the truncate discards was cloned first.
#[must_use]
pub fn read_before(
    store: &[SequencedEvent],
    query: &Query,
    options: ReadOptions,
) -> Vec<SequencedEvent> {
    let matched = store
        .iter()
        .filter(|event| query.matches(event.event_type(), event.tags()));

    let mut selected: Vec<SequencedEvent> = if options.backwards {
        matched
            .rev()
            .filter(|event| options.from.is_none_or(|from| event.position <= from))
            .filter(|event| options.to.is_none_or(|to| event.position >= to))
            .cloned()
            .collect()
    } else {
        matched
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .filter(|event| options.to.is_none_or(|to| event.position <= to))
            .cloned()
            .collect()
    };

    if let Some(limit) = options.limit {
        selected.truncate(limit);
    }

    selected
}

/// The same function with `.take(limit)` above `.cloned()`.
///
/// `take` sits **below** both bounds filters and **above** `cloned`, and below
/// `rev()` in the backwards branch. One link earlier it would limit the
/// *scanned* set instead of the *result* set, which is the wrong implementation
/// ES-14 `[FROZEN]` forbids; one link later — where it is today — it limits the
/// result set correctly and clones the discarded tail on the way.
///
/// `unwrap_or(usize::MAX)` is how `Option<usize>` reaches `take`. A store cannot
/// hold `usize::MAX` events, so the unlimited case is unchanged, and `Some(0)`
/// still reads nothing.
#[must_use]
pub fn read_after(
    store: &[SequencedEvent],
    query: &Query,
    options: ReadOptions,
) -> Vec<SequencedEvent> {
    let matched = store
        .iter()
        .filter(|event| query.matches(event.event_type(), event.tags()));
    let limit = options.limit.unwrap_or(usize::MAX);

    if options.backwards {
        matched
            .rev()
            .filter(|event| options.from.is_none_or(|from| event.position <= from))
            .filter(|event| options.to.is_none_or(|to| event.position >= to))
            .take(limit)
            .cloned()
            .collect()
    } else {
        matched
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .filter(|event| options.to.is_none_or(|to| event.position <= to))
            .take(limit)
            .cloned()
            .collect()
    }
}

/// Reads through the real `MemoryEventStore` and collects the result.
#[must_use]
pub fn read_real(
    store: &MemoryEventStore,
    query: &Query,
    options: ReadOptions,
) -> Vec<SequencedEvent> {
    drain(store.read(query, options))
}

/// How many events one `append` call carries while filling a store.
///
/// [`happenstance_core::MIN_SUPPORTED_EVENTS_PER_BATCH`], because a fixture that
/// writes at a size no adapter is obliged to accept is a fixture that could not
/// be pointed at a real adapter later.
pub const FILL_BATCH: usize = happenstance_core::MIN_SUPPORTED_EVENTS_PER_BATCH;

/// Fills a fresh `MemoryEventStore` with `count` events and returns it together
/// with a mirror of exactly what it holds.
///
/// The mirror is read back out of the store rather than built alongside it, so
/// the replicas in this module operate on the store's own `SequencedEvent`
/// values — same positions, same `EventId`, same `RecordedAt`. Building a
/// parallel `Vec` by hand would mint a different `StoreId` and a different
/// timestamp, and the fidelity comparison would then be between two things that
/// were never equal.
///
/// # Panics
///
/// Panics if any append fails, which for an unconditioned append into the
/// in-memory store it cannot.
#[must_use]
pub fn filled(
    count: usize,
    regime: Regime,
    tags_per_event: usize,
    shape: Payload,
) -> (MemoryEventStore, Vec<SequencedEvent>) {
    let store = MemoryEventStore::new();
    let mut batch: Vec<Event> = Vec::with_capacity(FILL_BATCH);

    for index in 0..count {
        batch.push(
            Event::new(event_type(regime), payload(shape))
                .expect("the event type is valid")
                .with_tags(tags(regime, tags_per_event)),
        );
        if batch.len() == FILL_BATCH || index + 1 == count {
            block_on_ready(store.append(&batch, None)).expect("an unconditioned append succeeds");
            batch.clear();
        }
    }

    let mirror = read_real(&store, &Query::all(), ReadOptions::new());
    assert_eq!(mirror.len(), count, "the store holds what was written");
    (store, mirror)
}

/// A query that matches every event [`filled`] wrote, without being `Query::All`.
///
/// `Query::All` short-circuits `matches` (`query.rs:221`), so measuring against
/// it would measure a read path with the tag comparison compiled out. This item
/// constrains on the same tags every filled event carries, so the filter runs in
/// full and still matches everything — which is what "a fully-matching query"
/// has to mean if the number is to be about the clone rather than about the
/// filter.
///
/// # Panics
///
/// Panics if the item is unconstrained, which it is not: it carries tags.
#[must_use]
pub fn matching_query(regime: Regime, tags_per_event: usize) -> Query {
    let item = happenstance_core::QueryItem::new(
        core::iter::empty::<happenstance_core::EventType>(),
        tags(regime, tags_per_event),
    )
    .expect("an item with tags and no types is constrained");
    Query::from_item(item)
}
