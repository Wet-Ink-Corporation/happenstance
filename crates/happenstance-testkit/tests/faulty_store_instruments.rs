//! The two wrong callers `FaultyStore` exists to reject, and their conformant
//! siblings.
//!
//! Every store a consumer of this library can test against behaves perfectly:
//! `MemoryEventStore` names the conflicting event on every violation and never
//! fails a read. Two whole classes of caller bug are therefore not merely hard
//! to test — the input that separates a correct caller from an incorrect one
//! never occurs in-process at all. This file drives the inputs `FaultyStore`
//! produces on demand.
//!
//! Nothing here is asserted by a bare `#[should_panic]`. That records only that
//! *something* failed, so a wrong caller that fails for an unrelated reason
//! reads as proof — the shape ADR-0010 rejects by name. Each case asserts the
//! specific value-level disagreement it produces, and each ships beside the
//! conformant sibling that gets it right.
//!
//! Native only: the harness is `#[tokio::test]`, and the wasm32 story for this
//! crate's own tests is `memory_conformance_wasm.rs`'s.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, Query, QueryItem,
    ReadOptions, SequencePosition, SequencedEvent,
};
use happenstance_testkit::{FaultyStore, FaultyStoreError};

// ---------------------------------------------------------------------------
// Arrangement
// ---------------------------------------------------------------------------

fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).unwrap()
}

/// A condition no correct store here can violate on its own.
///
/// Load-bearing: a condition naming a type the store *does* hold would be
/// violated by the inner store too, and every assertion below about the
/// injected violation would pass for the wrong reason.
fn unviolated_condition() -> AppendCondition {
    AppendCondition::new(Query::from_item(
        QueryItem::of_types(["NeverAppendedHere"]).unwrap(),
    ))
}

/// Drains a read stream into every item it yielded, errors included.
///
/// `happenstance_core::collect` stops at the first error and discards what it
/// had, which is exactly the information these tests are about.
async fn drain<S, T, E>(stream: S) -> Vec<Result<T, E>>
where
    S: Stream<Item = Result<T, E>>,
{
    let mut stream = core::pin::pin!(stream);
    let mut items = Vec::new();

    core::future::poll_fn(|cx| {
        loop {
            match stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(item)) => items.push(item),
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => return Poll::Pending,
            }
        }
    })
    .await;

    items
}

/// The positions a drained read reported, in the order it reported them.
///
/// Never compared against a literal: every value here came out of the store.
fn positions<E>(items: &[Result<SequencedEvent, E>]) -> Vec<SequencePosition> {
    items
        .iter()
        .filter_map(|item| item.as_ref().ok())
        .map(|event| event.position)
        .collect()
}

// ---------------------------------------------------------------------------
// AC-001 — the four names resolve at the crate root
// ---------------------------------------------------------------------------

/// Every instrument is reachable by its documented public path, in one act.
///
/// The import list is the assertion: a private module in any of the four paths,
/// a `#[doc(hidden)]`, or a second crate to add would all fail here rather than
/// in a consumer's editor. Gated on `memory` only because `GappyMemoryStore` is.
#[cfg(feature = "memory")]
#[test]
fn instruments_are_reachable_from_the_crate_root() {
    use core::num::NonZeroU64;

    use happenstance_testkit::{FaultyStore, FaultyStoreError, GappyMemoryStore, SendFaultyStore};

    let bare = FaultyStore::new(MemoryEventStore::new()).violate_next(1);
    let sendable = SendFaultyStore::new(MemoryEventStore::new()).fail_next_read(1);
    let gappy = GappyMemoryStore::with_stride(NonZeroU64::new(7).unwrap());
    let injected: FaultyStoreError<happenstance_core::MemoryStoreError> =
        FaultyStoreError::Injected;

    // Constructed, not merely named: a type alias would satisfy an import and
    // not a construction.
    let _ = (&bare, &sendable, &gappy, &injected);
    assert!(format!("{injected}").contains("injected"));
}

// ---------------------------------------------------------------------------
// AC-002, EC-002 — the injected violation
// ---------------------------------------------------------------------------

/// The injected violation names no conflicting event — the whole point.
///
/// A fixture that reported `Some(head)` because it is "more informative" would
/// behave exactly like `MemoryEventStore` on the one axis it was built to vary,
/// and the retry test below would pass against the wrong caller.
#[tokio::test]
async fn injected_violation_reports_no_conflicting_position() {
    let store = FaultyStore::new(MemoryEventStore::new()).violate_next(1);

    let err = store
        .append(&[event("Seated")], Some(&unviolated_condition()))
        .await
        .unwrap_err();

    match err {
        AppendError::ConditionViolated(violation) => assert!(
            violation.conflicting_position.is_none(),
            "the fixture exists to produce the `None` a remote store reports; \
             got {:?}",
            violation.conflicting_position
        ),
        other => panic!("expected a violated condition, got {other:?}"),
    }
}

/// An armed append never reaches the inner store.
///
/// A wrapper that appended and then reported failure would be a fixture that
/// lies about atomicity — the very property `EventStore::append` promises.
#[tokio::test]
async fn injected_violation_does_not_touch_the_inner_store() {
    let inner = MemoryEventStore::new();
    inner.append(&[event("Seated")], None).await.unwrap();

    let store = FaultyStore::new(inner).violate_next(1);

    let head_before = store.head().await.unwrap();
    let read_before = drain(store.read(&Query::all(), ReadOptions::new())).await;

    let err = store
        .append(&[event("Seated")], Some(&unviolated_condition()))
        .await
        .unwrap_err();
    assert!(err.is_condition_violated(), "got {err:?}");

    let head_after = store.head().await.unwrap();
    let read_after = drain(store.read(&Query::all(), ReadOptions::new())).await;

    assert_eq!(
        head_before, head_after,
        "a rejected append must leave the store byte-identical"
    );
    assert_eq!(positions(&read_before), positions(&read_after));
    assert_eq!(
        read_before.len(),
        read_after.len(),
        "the inner `append` was called: the log grew"
    );
}

// ---------------------------------------------------------------------------
// AC-003 — the retry loop everyone writes first
// ---------------------------------------------------------------------------

/// The wrong caller: retry only when the store says *which* event conflicted.
///
/// It works against every in-process store and stops working against one
/// reached over one-shot HTTP, which reports `None` conformantly.
async fn retry_gated_on_some_conflicting_position<S: EventStore>(
    store: &S,
    attempts: u32,
) -> Result<SequencePosition, AppendError<S::Error>> {
    let mut submitted = 0_u32;

    loop {
        submitted += 1;
        match store
            .append(&[event("Seated")], Some(&unviolated_condition()))
            .await
        {
            Ok(position) => return Ok(position),
            Err(err) => {
                let retry = match &err {
                    // The defect, spelled out: `Some` is a hint, not a promise.
                    AppendError::ConditionViolated(violation) => {
                        violation.conflicting_position.is_some()
                    }
                    _ => false,
                };
                if !retry || submitted >= attempts {
                    return Err(err);
                }
            }
        }
    }
}

/// The conformant sibling: retry on the signal itself.
async fn retry_gated_on_the_signal<S: EventStore>(
    store: &S,
    attempts: u32,
) -> Result<(SequencePosition, u32), AppendError<S::Error>> {
    let mut submitted = 0_u32;

    loop {
        submitted += 1;
        match store
            .append(&[event("Seated")], Some(&unviolated_condition()))
            .await
        {
            Ok(position) => return Ok((position, submitted)),
            Err(err) => {
                if !err.is_condition_violated() || submitted >= attempts {
                    return Err(err);
                }
            }
        }
    }
}

/// The wrong caller gives up on the first attempt, and the log stays empty.
///
/// Asserted as the error it returned and the state it left, not as a panic:
/// `#[should_panic]` would pass just as well if the loop had failed for an
/// unrelated reason.
#[tokio::test]
async fn retry_gated_on_some_conflicting_position_never_retries() {
    let store = FaultyStore::new(MemoryEventStore::new()).violate_next(1);

    let err = retry_gated_on_some_conflicting_position(&store, 3)
        .await
        .unwrap_err();

    match err {
        AppendError::ConditionViolated(violation) => assert!(
            violation.conflicting_position.is_none(),
            "the loop surfaced a violation that named a conflict, so it did not \
             fail for the reason this instrument exists to produce"
        ),
        other => panic!("expected the violation to reach the caller, got {other:?}"),
    }

    let held = drain(store.read(&Query::all(), ReadOptions::new())).await;
    assert!(
        held.is_empty(),
        "the loop never retried, so nothing can have committed; got {:?}",
        positions(&held)
    );
}

/// The conformant sibling commits on its second attempt, and the fixture is
/// reversible: once the counter reaches zero the wrapper delegates again.
#[tokio::test]
async fn retry_gated_on_is_condition_violated_succeeds_on_the_second_attempt() {
    let store = FaultyStore::new(MemoryEventStore::new()).violate_next(1);

    let (committed, submitted) = retry_gated_on_the_signal(&store, 3).await.unwrap();
    assert_eq!(submitted, 2, "one injected failure, then success");

    // Compared against what the store itself reports, never a literal: the
    // specification permits gaps.
    let head = store.head().await.unwrap();
    assert_eq!(
        head,
        Some(committed),
        "the committed position must be the one the store's own `append` returned"
    );

    // The counter reached zero, so a third append delegates rather than being
    // permanently poisoned.
    let again = store
        .append(&[event("Seated")], Some(&unviolated_condition()))
        .await
        .unwrap();
    let head_again = store.head().await.unwrap();
    assert_eq!(head_again, Some(again));
    assert!(
        again > committed,
        "the delegated append landed above the first"
    );
}

/// EC-001 — arming nothing is legal and inert.
#[tokio::test]
async fn arming_zero_arms_nothing() {
    let store = FaultyStore::new(MemoryEventStore::new())
        .violate_next(0)
        .fail_next_read(0);

    let landed = store.append(&[event("Seated")], None).await.unwrap();
    let items = drain(store.read(&Query::all(), ReadOptions::new())).await;

    assert_eq!(positions(&items), vec![landed]);
    assert!(items.iter().all(Result::is_ok));
}

// ---------------------------------------------------------------------------
// AC-004 — the injected read failure is lazy
// ---------------------------------------------------------------------------

/// An armed read fails where a real store fails: at the first poll.
///
/// `read` is not `async` and hands the stream back at the top level, so there
/// is no call-time channel a failure could arrive through at all. What this
/// asserts is the positive half: the first *item* is the injected error, and
/// everything behind it is still the inner store's.
#[tokio::test]
async fn injected_read_failure_is_the_first_polled_item_not_a_call_time_error() {
    let inner = MemoryEventStore::new();
    let landed = inner.append(&[event("Seated")], None).await.unwrap();

    let store = FaultyStore::new(inner).fail_next_read(1);

    // Nothing has failed here: `read` returns a stream, not a `Result`. The
    // query is bound rather than inlined because edition 2024 RPITIT captures
    // every in-scope lifetime, so the stream borrows it; inlining is E0716.
    let query = Query::all();
    let stream = store.read(&query, ReadOptions::new());

    let items = drain(stream).await;
    match items.first() {
        Some(Err(FaultyStoreError::Injected)) => {}
        other => panic!("expected the injected failure first, got {other:?}"),
    }
    assert_eq!(
        positions(&items),
        vec![landed],
        "behind the injected item the inner store's own events must follow"
    );

    // Spent: the next read delegates.
    let after = drain(store.read(&Query::all(), ReadOptions::new())).await;
    assert!(after.iter().all(Result::is_ok));
    assert_eq!(positions(&after), vec![landed]);
}

// ---------------------------------------------------------------------------
// AC-009, EC-004, EC-005 — the two error arms
// ---------------------------------------------------------------------------

/// A store failure with somewhere for the chain to end.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Disk;

impl core::fmt::Display for Disk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the disk is full")
    }
}

impl core::error::Error for Disk {}

/// One `Err(Disk)` item, then end of stream.
struct FailingRead(bool);

impl Stream for FailingRead {
    type Item = Result<SequencedEvent, Disk>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.0 {
            this.0 = false;
            Poll::Ready(Some(Err(Disk)))
        } else {
            Poll::Ready(None)
        }
    }
}

/// A store whose every operation fails with its own error.
///
/// `!Send` by construction, and that is what lets it implement the bare
/// flavour directly: `trait_variant` emits a blanket
/// `impl<T: SendEventStore> EventStore for T`, and coherence admits this impl
/// only because an `Rc` in the type makes `SendEventStore` unimplementable
/// here. It is the same reasoning `MemoryHandle` records from the other side.
struct BrokenStore(PhantomData<Rc<()>>);

impl EventStore for BrokenStore {
    type Error = Disk;

    fn read(
        &self,
        _query: &Query,
        _options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        FailingRead(true)
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        Err(AppendError::Store(Disk))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Err(Disk)
    }

    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
        Err(Disk)
    }
}

/// The two arms are distinguishable, and the inner error survives the wrapper.
///
/// A `String` payload, or an arm that re-labelled a store failure `Injected`,
/// would make "the fixture broke this on purpose" indistinguishable from "my
/// store failed" — which is the one question a caller of a fixture has.
#[tokio::test]
async fn injected_and_store_errors_are_distinguishable_and_the_source_chain_survives() {
    // The fixture's own failure: no source, because nothing underneath failed.
    let armed = FaultyStore::new(MemoryEventStore::new()).fail_next_read(1);
    let injected = drain(armed.read(&Query::all(), ReadOptions::new())).await;
    let injected = injected.into_iter().next().unwrap().unwrap_err();
    assert!(matches!(injected, FaultyStoreError::Injected));
    assert!(
        core::error::Error::source(&injected).is_none(),
        "nothing underneath failed, so there is no source to report"
    );

    // The inner store's failure, with nothing armed: carried, never relabelled.
    let broken = FaultyStore::new(BrokenStore(PhantomData));
    let surfaced = drain(broken.read(&Query::all(), ReadOptions::new())).await;
    let surfaced = surfaced.into_iter().next().unwrap().unwrap_err();
    match &surfaced {
        FaultyStoreError::Store(inner) => assert_eq!(*inner, Disk),
        other => panic!("a store failure was relabelled: {other:?}"),
    }
    let source = core::error::Error::source(&surfaced).unwrap();
    assert!(
        source.downcast_ref::<Disk>().is_some(),
        "the inner error must stay reachable through `source()`, not be \
         flattened into a rendered string: {source}"
    );

    // And the head path carries the same distinction.
    let head = broken.head().await.unwrap_err();
    assert!(matches!(head, FaultyStoreError::Store(Disk)));
}
