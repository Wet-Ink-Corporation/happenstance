//! The workspace's first `!Send` event store, and the suite run against it.
//!
//! ADR-0001 was provisional because "**no `!Send` implementation of these ports
//! exists anywhere**, not even a reference one", and named this store as the
//! cheapest proof that would lift it. This file is that proof; the marker came
//! off on 2026-08-06. CF-28 `[FROZEN]` makes the store mandatory and puts it
//! here. ES-7 rides along: `happenstance-testkit` is
//! a genuinely downstream crate, so a direct `impl EventStore for` a local type
//! here is the first evidence that the blanket impl `trait_variant` emits
//! (`impl<T: SendEventStore> EventStore for T`) does not foreclose it.
//!
//! Three things are proved here that `MemoryEventStore` cannot prove:
//!
//! 1. the bare flavour has an implementer at all;
//! 2. it is genuinely `!Send` (see `send_probe`), not merely un-annotated;
//! 3. the re-entrancy question of E2E-09 — see `reentrancy` and `mutants`.
//!
//! # What running the suite here settled, and what it refuted
//!
//! The runbook assumed the testkit's shipped `#[tokio::test]` emitter could not
//! drive a `!Send` store, and that phase 1 therefore *had* to introduce a
//! single-threaded harness before CF-28 was reachable. That is false, and the
//! distinction is worth keeping because it is easy to get backwards:
//! `tokio::spawn` requires `Send`, but `Runtime::block_on` does not, and
//! `#[tokio::test]` expands to `block_on`. All twenty-seven rules pass against
//! this store under the *default multi-threaded* attribute, unchanged. CF-23's
//! case for making the wrapper a parameter is real, but its reason is wasm
//! portability rather than `Send`-ness.

#![allow(clippy::unwrap_used)]

use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventStore, Query, ReadOptions,
    SequencePosition, SequencedEvent,
};

// =====================================================================
// The store
// =====================================================================

/// A `RefCell`-backed reference event store that is deliberately `!Send`.
///
/// # Why `Rc` and not a bare `RefCell`
///
/// `RefCell<T>: Send where T: Send` — `RefCell` gives up `Sync`, not `Send`. So
/// `RefCell<Vec<SequencedEvent>>` on its own is perfectly `Send` and would prove
/// nothing about the bare flavour. `Rc` is what actually removes `Send`, and it
/// is also the honest shape: a Durable Object holds its store through an `Rc` on
/// a single-threaded executor.
///
/// `Clone` is derived, so two handles onto one backing log are expressible here
/// — the far end of the "handle multiplicity" axis the specification records as
/// empty. Nothing below depends on it; it is available for a later rule.
#[derive(Debug, Clone, Default)]
struct LocalMemoryEventStore {
    events: Rc<RefCell<Vec<SequencedEvent>>>,
}

impl LocalMemoryEventStore {
    /// Creates an empty store.
    fn new() -> Self {
        Self::default()
    }

    /// A snapshot of every event held, in position order.
    fn snapshot(&self) -> Vec<SequencedEvent> {
        self.events.borrow().clone()
    }

    /// Filters, orders and truncates under a shared borrow, then releases it.
    ///
    /// The borrow must not outlive this function. `MemoryEventStore` reaches the
    /// same conclusion for a different reason: an `RwLockReadGuard` held across
    /// a poll would be a liveness problem, whereas a `Ref` held past the return
    /// would be a *panic* the first time anyone appended. Same discipline,
    /// sharper consequence.
    fn select(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LocalStoreError> {
        let borrowed = self
            .events
            .try_borrow()
            .map_err(|_| LocalStoreError::AlreadyBorrowed)?;

        let matched = borrowed
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));

        let mut selected: Vec<SequencedEvent> = if options.backwards {
            matched
                .rev()
                .filter(|event| options.from.is_none_or(|from| event.position <= from))
                .cloned()
                .collect()
        } else {
            matched
                .filter(|event| options.from.is_none_or(|from| event.position >= from))
                .cloned()
                .collect()
        };

        if let Some(limit) = options.limit {
            selected.truncate(limit.get());
        }

        drop(borrowed);
        Ok(selected)
    }
}

/// The dense position for a zero-based index.
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index)
        .unwrap_or(u64::MAX - 1)
        .saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// [`LocalMemoryEventStore`]'s error type.
///
/// Inhabited, unlike `MemoryStoreError`, and on purpose. A `RefCell` store has
/// exactly one failure mode of its own — a conflicting borrow — and the choice
/// between `try_borrow` and `borrow` is the choice between reporting it and
/// panicking. Making the failure representable in `Self::Error` is how an
/// adapter avoids panicking without relying on never being re-entered.
///
/// Written by hand rather than with `thiserror`, which the testkit does not
/// depend on. Worth noticing, because every adapter author meets it:
/// `EventStore::Error: core::error::Error` obliges each of them to bring a
/// derive or write these ten lines, and `happenstance-core` re-exports neither
/// `thiserror` nor a helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalStoreError {
    /// Another borrow of the log was live. Only reachable if a `Ref` escapes a
    /// method, which nothing here allows.
    AlreadyBorrowed,
}

impl core::fmt::Display for LocalStoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyBorrowed => f.write_str("the event log was already borrowed"),
        }
    }
}

impl core::error::Error for LocalStoreError {}

// This is the ES-7 evidence: a direct `impl EventStore for` a local type in a
// downstream crate, alongside the blanket `impl<T: SendEventStore> EventStore
// for T` that `trait_variant` emits into `happenstance-core`. No `error[E0119]`.
impl EventStore for LocalMemoryEventStore {
    type Error = LocalStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // Note the absence of `+ Send`. That is the whole point of the bare
        // flavour, and the reason this file exists.
        Snapshot::new(self.select(query, options))
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // There is no `.await` in this body, and that is load-bearing rather
        // than incidental: the exclusive borrow below is therefore acquired and
        // released inside a single `poll`, so two `append` futures on one handle
        // can never observe each other's borrow. See `mutants` for the compiled
        // demonstration of what happens to an adapter that gets this wrong, and
        // for why the *port* still permits one.
        let mut stored = self
            .events
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LocalStoreError::AlreadyBorrowed))?;

        // Condition before emptiness, matching the reference store: a caller
        // that raced and lost should learn that, not that its batch was empty.
        if let Some(condition) = condition {
            let conflict = stored.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });

            if let Some(conflict) = conflict {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }

        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let first_index = stored.len();
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            SequencedEvent::new(position_at(first_index + offset), event.clone())
        }));

        Ok(position_at(first_index + events.len() - 1))
    }
}

/// The stream returned by [`LocalMemoryEventStore::read`].
///
/// Owns its events, so it borrows nothing from the `RefCell`. It is `!Send` only
/// because it is not required to be; nothing in it would object to `Send`.
#[derive(Debug)]
struct Snapshot {
    /// Surfaced as the stream's first item, so a failure is lazy like every
    /// other read failure rather than arriving before the first poll.
    error: Option<LocalStoreError>,
    events: std::vec::IntoIter<SequencedEvent>,
}

impl Snapshot {
    fn new(selected: Result<Vec<SequencedEvent>, LocalStoreError>) -> Self {
        match selected {
            Ok(events) => Self {
                error: None,
                events: events.into_iter(),
            },
            Err(err) => Self {
                error: Some(err),
                events: Vec::new().into_iter(),
            },
        }
    }
}

impl Stream for Snapshot {
    type Item = Result<SequencedEvent, LocalStoreError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Some(err) = this.error.take() {
            return Poll::Ready(Some(Err(err)));
        }
        Poll::Ready(this.events.next().map(Ok))
    }
}

// =====================================================================
// Compiled proof that the store is `!Send`
// =====================================================================

/// Autoref specialisation, the only way to observe the *absence* of an auto
/// trait on stable.
///
/// Method resolution tries inherent candidates before trait candidates and
/// discards an inherent candidate whose bounds do not hold. So `is_send()`
/// resolves to the inherent method when `T: Send` and falls through to the trait
/// method when it does not — a compile-time decision reported as a runtime bool.
mod send_probe {
    use core::marker::PhantomData;

    #[derive(Debug)]
    pub(crate) struct Probe<T>(pub(crate) PhantomData<T>);

    pub(crate) trait NotSend {
        fn is_send(&self) -> bool {
            false
        }
    }

    impl<T> NotSend for Probe<T> {}

    impl<T: Send> Probe<T> {
        // `&self` is not unused, it is the entire mechanism: an associated
        // function would be resolved by path and would never fall through to the
        // trait candidate.
        #[allow(clippy::unused_self)]
        pub(crate) fn is_send(&self) -> bool {
            true
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_store_is_not_send() {
    use core::marker::PhantomData;
    use send_probe::{NotSend as _, Probe};

    // Positive control. Without it this assertion would also pass if the probe
    // were simply broken and always answered `false` — which is the shape of
    // vacuity this whole phase exists to remove.
    assert!(
        Probe::<happenstance_core::MemoryEventStore>(PhantomData).is_send(),
        "probe is broken: MemoryEventStore is Send"
    );

    assert!(
        !Probe::<LocalMemoryEventStore>(PhantomData).is_send(),
        "LocalMemoryEventStore must be !Send, or it proves nothing about the \
         bare flavour"
    );
}

// =====================================================================
// The suite, three ways
// =====================================================================

// Harness 1 — the testkit's runtime-free emitter, driven by its `block_on`.
// No async runtime is involved at all.
#[cfg(not(target_arch = "wasm32"))]
happenstance_testkit::event_store_conformance!(
    mod_name = local_blocking,
    emit = happenstance_testkit::__emit_blocking,
    factory = LocalMemoryEventStore::new()
);

// Harness 2 — the testkit's macro verbatim, i.e. the default `#[tokio::test]`
// on the *multi-threaded* runtime. This is the literal text of CF-28 — "the
// existing suite, invoked against a `RefCell`-backed store" — and it passes,
// which is what refutes the premise recorded in this file's module docs.
#[cfg(not(target_arch = "wasm32"))]
happenstance_testkit::event_store_conformance!(
    mod_name = local_tokio_default,
    factory = LocalMemoryEventStore::new()
);

// Harness 3 — a caller-supplied emitter the testkit has never heard of.
//
// This is the half of CF-23 that the three shipped emitters cannot demonstrate:
// a runtime, or a runtime *flavour*, that the testkit does not enumerate. It
// needs no release of `happenstance-testkit` to exist. `current_thread` is also
// the closest native analogue of the single-threaded executor a Worker runs.
#[cfg(not(target_arch = "wasm32"))]
mod local_current_thread {
    use super::LocalMemoryEventStore;

    macro_rules! emit_current_thread {
        ($($name:ident),* $(,)?) => {
            $(
                #[tokio::test(flavor = "current_thread")]
                async fn $name() {
                    happenstance_testkit::rules::$name(LocalMemoryEventStore::new).await;
                }
            )*
        };
    }

    happenstance_testkit::for_each_event_store_rule!(emit_current_thread);
}

// Harness 4 — the same rules under `wasm-bindgen-test`, against the `!Send`
// store, on the target the two-flavour design exists for. Type-checked by
// `cargo xtask ci`'s wasm32 harness step on every run; executed by CI's
// `wasm-conformance` job.
#[cfg(target_arch = "wasm32")]
happenstance_testkit::event_store_conformance!(
    mod_name = local_wasm,
    emit = happenstance_testkit::__emit_wasm,
    factory = LocalMemoryEventStore::new()
);

// =====================================================================
// E2E-09 — re-entrancy
// =====================================================================

/// Two `append` futures created from one handle and polled alternately.
///
/// This is the question `MemoryEventStore` cannot carry — `memory.rs`'s append
/// body holds no lock across a suspension point because it contains no `.await`
/// at all — and the reason CF-28 asks for a `RefCell` store specifically.
#[cfg(not(target_arch = "wasm32"))]
mod reentrancy {
    use super::{EventStore, LocalMemoryEventStore};
    use happenstance_core::{AppendError, Query, ReadOptions, collect};
    use happenstance_testkit::fixtures::{condition_after, query_of, tagged_event};

    #[tokio::test(flavor = "current_thread")]
    async fn interleaved_appends_on_one_handle_elect_one_winner() {
        let store = LocalMemoryEventStore::new();
        let boundary = store
            .append(&[tagged_event("CourseDefined", &[("course", "c1")])], None)
            .await
            .unwrap();

        let query = query_of(&["StudentSubscribed"], &[("course", "c1")]);
        let condition_a = condition_after(query.clone(), boundary.get());
        let condition_b = condition_after(query, boundary.get());
        let subscribe = tagged_event("StudentSubscribed", &[("course", "c1")]);

        // Both futures are created before either is polled, so both hold `&self`
        // simultaneously. On the `Send` flavour that would need `Sync`; here it
        // needs nothing, which is the point.
        let a = store.append(core::slice::from_ref(&subscribe), Some(&condition_a));
        let b = store.append(core::slice::from_ref(&subscribe), Some(&condition_b));
        let (first, second) = tokio::join!(a, b);

        assert!(first.is_ok(), "the first to commit must succeed: {first:?}");
        assert!(
            matches!(second, Err(AppendError::ConditionViolated(_))),
            "the second decided from a stale snapshot and must be rejected: {second:?}"
        );
        assert_eq!(
            store.snapshot().len(),
            2,
            "exactly one of the two conditional appends may land"
        );
    }

    /// A read stream held open across an append.
    ///
    /// The `Snapshot` stream owns its events, so this is fine. The shape that
    /// would *not* be fine — a stream holding a `Ref` — is
    /// `mutants::BorrowHoldingStore`, which passes all twenty-seven existing
    /// rules and panics here.
    #[tokio::test(flavor = "current_thread")]
    async fn a_live_read_stream_does_not_block_an_append() {
        let store = LocalMemoryEventStore::new();
        store
            .append(&[tagged_event("A", &[("k", "v")])], None)
            .await
            .unwrap();

        // `Query::all()` must be bound to a local. `read`'s RPITIT captures
        // every in-scope lifetime under edition 2024, so the returned stream
        // borrows the *query* even though it owns all of its events —
        // otherwise `error[E0716]: temporary value dropped while borrowed`.
        let query = Query::all();
        let stream = store.read(&query, ReadOptions::new());

        // The stream is alive across this append.
        let appended = store
            .append(&[tagged_event("B", &[("k", "v")])], None)
            .await;
        assert!(
            appended.is_ok(),
            "a live read stream must not hold a borrow: {appended:?}"
        );

        let drained = collect(stream).await.unwrap();
        assert_eq!(drained.len(), 1, "the stream is a snapshot taken at read");
    }
}

// =====================================================================
// The wrong implementations the two tests above exist to reject
// =====================================================================

/// "A rule that no adapter can fail is decorative." Both stores below are
/// plausible, both pass **all twenty-seven** existing rules, and both panic at
/// runtime. They are the evidence that the two tests in `reentrancy` are worth
/// promoting into the suite proper — which is phase 3's call, not this file's.
///
/// The suite misses both for one reason: every rule drains a read via `collect`
/// before appending again, and `racing_conditional_appends_elect_one_winner` is
/// sequential and single-handle. Nothing in it ever holds two live things at
/// once.
#[cfg(not(target_arch = "wasm32"))]
mod mutants {
    use super::{
        AppendCondition, AppendError, ConditionViolated, Context, Event, EventStore, Pin, Poll,
        Query, Rc, ReadOptions, RefCell, SequencePosition, SequencedEvent, Stream, position_at,
    };
    use core::cell::Ref;
    use core::future::Future;
    use happenstance_testkit::fixtures::event;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Never {}

    impl core::fmt::Display for Never {
        fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match *self {}
        }
    }

    impl core::error::Error for Never {}

    /// Shared by both mutants: correct filtering, ordering and truncation.
    fn select_indices(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Vec<usize> {
        let matched = events
            .iter()
            .enumerate()
            .filter(|(_, event)| query.matches(event.event_type(), event.tags()));

        let mut indices: Vec<usize> = if options.backwards {
            matched
                .rev()
                .filter(|(_, event)| options.from.is_none_or(|from| event.position <= from))
                .map(|(index, _)| index)
                .collect()
        } else {
            matched
                .filter(|(_, event)| options.from.is_none_or(|from| event.position >= from))
                .map(|(index, _)| index)
                .collect()
        };

        if let Some(limit) = options.limit {
            indices.truncate(limit.get());
        }
        indices
    }

    /// Shared by both mutants: the correct append body, minus borrow discipline.
    fn apply_append(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Never>> {
        if let Some(condition) = condition {
            let conflict = stored.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });
            if let Some(conflict) = conflict {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        let first = stored.len();
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            SequencedEvent::new(position_at(first + offset), event.clone())
        }));
        Ok(position_at(first + events.len() - 1))
    }

    // -----------------------------------------------------------------
    // Mutant 1 — `read` returns a stream that keeps the borrow alive
    // -----------------------------------------------------------------

    #[derive(Debug, Clone, Default)]
    struct BorrowHoldingStore {
        events: Rc<RefCell<Vec<SequencedEvent>>>,
    }

    #[derive(Debug)]
    struct BorrowingStream<'a> {
        borrowed: Ref<'a, Vec<SequencedEvent>>,
        indices: std::vec::IntoIter<usize>,
    }

    impl Stream for BorrowingStream<'_> {
        type Item = Result<SequencedEvent, Never>;
        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            let next = this.indices.next();
            Poll::Ready(next.map(|index| Ok(this.borrowed[index].clone())))
        }
    }

    impl EventStore for BorrowHoldingStore {
        type Error = Never;

        // This type-checks, and that is the finding. RPITIT lets an implementer
        // return a stream that borrows from the store, so the port cannot
        // express "your stream must not hold a borrow" — and nothing but a rule
        // will catch it.
        fn read(
            &self,
            query: &Query,
            options: ReadOptions,
        ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
            let borrowed = self.events.borrow();
            let indices = select_indices(&borrowed, query, options);
            BorrowingStream {
                borrowed,
                indices: indices.into_iter(),
            }
        }

        async fn append(
            &self,
            events: &[Event],
            condition: Option<&AppendCondition>,
        ) -> Result<SequencePosition, AppendError<Self::Error>> {
            apply_append(&mut self.events.borrow_mut(), events, condition)
        }
    }

    #[tokio::test(flavor = "current_thread")]
    #[should_panic(expected = "already borrowed")]
    async fn borrow_holding_read_stream_panics_on_a_concurrent_append() {
        let store = BorrowHoldingStore::default();
        store.append(&[event("A")], None).await.unwrap();

        let query = Query::all();
        let stream = store.read(&query, ReadOptions::new());
        let _ = store.append(&[event("B")], None).await;
        drop(stream);
    }

    // -----------------------------------------------------------------
    // Mutant 2 — `append` holds the exclusive borrow across an `.await`
    // -----------------------------------------------------------------

    /// Stands in for `SqlStorage::exec(..).await` in a Durable Object.
    struct YieldOnce(bool);

    impl Future for YieldOnce {
        type Output = ();
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.0 {
                Poll::Ready(())
            } else {
                self.0 = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    struct AwaitAcrossBorrowStore {
        events: Rc<RefCell<Vec<SequencedEvent>>>,
    }

    #[derive(Debug)]
    struct Snapshot(std::vec::IntoIter<SequencedEvent>);

    impl Stream for Snapshot {
        type Item = Result<SequencedEvent, Never>;
        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.get_mut().0.next().map(Ok))
        }
    }

    impl EventStore for AwaitAcrossBorrowStore {
        type Error = Never;

        fn read(
            &self,
            query: &Query,
            options: ReadOptions,
        ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
            let borrowed = self.events.borrow();
            let selected: Vec<SequencedEvent> = select_indices(&borrowed, query, options)
                .into_iter()
                .map(|index| borrowed[index].clone())
                .collect();
            drop(borrowed);
            Snapshot(selected.into_iter())
        }

        // Two findings ride on this `allow`, both worth carrying into the ADR.
        // `clippy::await_holding_refcell_ref` catches this defect *statically*,
        // and it fires under the workspace's own `-D warnings` — so for any
        // adapter that adopts this lint policy the defect never reaches a test.
        // But it is `warn`-by-default, so a downstream adapter on stock settings
        // gets a warning it can ignore, which is why a runtime rule still earns
        // its place. The lint does not catch mutant 1 at all.
        #[allow(clippy::await_holding_refcell_ref)]
        async fn append(
            &self,
            events: &[Event],
            condition: Option<&AppendCondition>,
        ) -> Result<SequencePosition, AppendError<Self::Error>> {
            // THE DEFECT: the borrow is taken, then the future suspends.
            let mut stored = self.events.borrow_mut();
            YieldOnce(false).await;
            apply_append(&mut stored, events, condition)
        }
    }

    #[tokio::test(flavor = "current_thread")]
    #[should_panic(expected = "already borrowed")]
    async fn borrow_across_await_panics_when_two_appends_interleave() {
        let store = AwaitAcrossBorrowStore::default();
        let batch_a = [event("A")];
        let batch_b = [event("B")];
        let a = store.append(&batch_a, None);
        let b = store.append(&batch_b, None);
        let _ = tokio::join!(a, b);
    }
}
