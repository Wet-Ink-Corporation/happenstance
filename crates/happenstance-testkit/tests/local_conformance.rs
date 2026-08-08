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
//! 3. that it survives the re-entrancy rules of E2E-09, which are now **in the
//!    suite** rather than beside it.
//!
//! # Where the re-entrancy tests went
//!
//! `interleaved_appends_on_one_handle_elect_one_winner` and
//! `a_live_read_stream_does_not_block_an_append` used to live here as a
//! `reentrancy` module, with the two wrong stores they reject under
//! `#[should_panic]` beside them. Both are ES-36 rules and both are now in
//! `happenstance-testkit`'s suite, so every one of the four harnesses below runs
//! them against this store rather than one hand-written `#[tokio::test]` doing
//! it once. The two wrong stores moved with them, into
//! `tests/mutation_coverage/mutants.rs`, where the registry says which rule each
//! fails instead of a `#[should_panic]` recording only that *something* did —
//! which is the shape CF-2 rejects by name, and it survived here only because a
//! mutant cannot be committed before the rule it fails exists.
//!
//! # What running the suite here settled, and what it refuted
//!
//! The runbook assumed the testkit's shipped `#[tokio::test]` emitter could not
//! drive a `!Send` store, and that phase 1 therefore *had* to introduce a
//! single-threaded harness before CF-28 was reachable. That is false, and the
//! distinction is worth keeping because it is easy to get backwards:
//! `tokio::spawn` requires `Send`, but `Runtime::block_on` does not, and
//! `#[tokio::test]` expands to `block_on`. Every rule passes against this store
//! under the *default multi-threaded* attribute, unchanged. CF-23's case for
//! making the wrapper a parameter is real, but its reason is wasm portability
//! rather than `Send`-ness.
//!
//! # What the fixture contract added here
//!
//! `LocalFixture` is the `!Send` half of CF-20's evidence. The
//! [`Fixture`](happenstance_testkit::Fixture) trait carries no `Send` bound and
//! is not `trait_variant`-derived, and this file is where that is load-bearing
//! rather than merely tidy: an `Rc`-holding fixture cannot satisfy a `Send`
//! bound, so a second flavour of the trait would have excluded precisely the
//! adapters ADR-0001 exists for.

#![allow(clippy::unwrap_used)]

use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_testkit::Capability;

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
/// empty. `LocalFixture` is what cashes that in: its `connect` is the clone, and
/// that is why it declares `SECOND_HANDLE` supported.
#[derive(Debug, Clone, Default)]
struct LocalMemoryEventStore {
    events: Rc<RefCell<Vec<SequencedEvent>>>,
}

/// This store's incarnation.
///
/// A constant rather than a per-instance value, and the reason is the `Clone`
/// above: two handles onto one backing log must agree about the identities that
/// log holds, and a field would be copied by `Clone` rather than shared. A
/// durable adapter mints one per database; this one has no database.
const LOCAL_STORE: StoreId = StoreId::from_bytes([0x1C; 16]);

/// A fixed recorded time, for `no_clock`'s reason: a rule must not depend on
/// wall time, and nothing here asserts the value.
const LOCAL_RECORDED_AT: RecordedAt = RecordedAt::from_millis(1_700_000_000_000);

impl LocalMemoryEventStore {
    /// Creates an empty store.
    fn new() -> Self {
        Self::default()
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
            selected.truncate(limit);
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
        // Emptiness first, and before the borrow — ES-20, and the same order
        // `MemoryEventStore` now uses. This copies the reference store
        // deliberately, which is why the comment moved rather than being
        // deleted: it used to say "condition before emptiness, matching the
        // reference store", and it was faithfully copying the reference store's
        // bug (D8). `append(&[], Some(&c))` answered `NoEvents` or
        // `ConditionViolated` depending on what the store held, and a caller
        // whose retry loop branches on `is_condition_violated()` never
        // terminates. Emptiness is a precondition on the *argument*, so nothing
        // behind the borrow can change the answer.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // There is no `.await` in this body, and that is load-bearing rather
        // than incidental: the exclusive borrow below is therefore acquired and
        // released inside a single `poll`, so two `append` futures on one handle
        // can never observe each other's borrow. The compiled demonstration of
        // what happens to an adapter that gets this wrong — and why the *port*
        // still permits one — is `AwaitAcrossBorrowStore` in
        // `tests/mutation_coverage/mutants.rs`, registered against the
        // re-entrancy rules this store passes.
        let mut stored = self
            .events
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LocalStoreError::AlreadyBorrowed))?;

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

        let first_index = stored.len();
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            let position = position_at(first_index + offset);
            SequencedEvent::new(
                position,
                EventId::new(LOCAL_STORE, position),
                LOCAL_RECORDED_AT,
                event.clone(),
            )
        }));

        Ok(position_at(first_index + events.len() - 1))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // `MemoryEventStore::head`'s body, through this store's borrow
        // discipline: `try_borrow` so that a conflicting borrow is reported in
        // `Self::Error` rather than panicking, and the `Ref` released before the
        // value leaves the function — the same rule `select` states, and for the
        // same reason. There is no `.await` here to hold it across.
        let borrowed = self
            .events
            .try_borrow()
            .map_err(|_| LocalStoreError::AlreadyBorrowed)?;

        let head = borrowed.last().map(|event| event.position);
        drop(borrowed);
        Ok(head)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        // A scan, like the reference store: this log has no index, and the point
        // of both is to be obviously correct rather than fast.
        let borrowed = self
            .events
            .try_borrow()
            .map_err(|_| LocalStoreError::AlreadyBorrowed)?;

        let found = borrowed.iter().any(|event| event.id == id);
        drop(borrowed);
        Ok(found)
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
// The fixture
// =====================================================================

/// One `RefCell`-backed log, and any number of handles onto it.
///
/// # Why there is no separate handle newtype here
///
/// [`MemoryFixture`](happenstance_testkit::fixtures::MemoryFixture) needs a
/// `MemoryHandle` because `Arc<MemoryEventStore>` does not itself implement the
/// port, and adding a blanket `impl EventStore for Arc<S>` would collide with
/// the blanket impl `trait_variant` emits. Nothing like that applies here:
/// `LocalMemoryEventStore` *is* the handle. It holds its log through an `Rc`, so
/// a clone is a second handle onto one backing store, which is the same
/// refcount-not-lifetime shape `MemoryHandle` has — just without needing a type
/// to carry it.
///
/// That refcount is also why [`Fixture::Store`](happenstance_testkit::Fixture)
/// can be an ordinary associated type rather than a GAT: the handle owns a share
/// of the store instead of borrowing the fixture.
#[derive(Debug, Default)]
struct LocalFixture(LocalMemoryEventStore);

impl LocalFixture {
    fn new() -> Self {
        // Spelled through the store's own constructor rather than
        // `Self::default()` so that `LocalMemoryEventStore::new` has a caller at
        // all: it is now the only one on every target.
        Self(LocalMemoryEventStore::new())
    }
}

impl happenstance_testkit::Fixture for LocalFixture {
    type Store = LocalMemoryEventStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The same honest answer `MemoryFixture` gives, for the same reason one
    // level down: an `Rc<RefCell<Vec<_>>>` has no durable medium behind it, so
    // "reopen" could only mean either doing nothing — which passes
    // `acknowledged_writes_survive_a_reopen` vacuously — or dropping the log,
    // which fails it while the store is perfectly conformant.
    const REOPEN: Capability = Capability::declined(
        "LocalMemoryEventStore is a Vec behind an Rc<RefCell<_>>, so there is no \
         durable medium to reopen over",
    );

    // Spelled `async fn` deliberately, where the trait declares
    // `-> impl Future`. They are the same signature after desugaring, and the
    // `async_fn_in_trait` lint fires only on a public trait's *declaration* —
    // so an implementer keeps the ergonomic form. Having one of the workspace's
    // two fixtures written each way is what keeps that claim checked.
    async fn connect(&self) -> Self::Store {
        self.0.clone()
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
///
/// Native-only, matching its single consumer below. On `wasm32` the probe is
/// three `dead_code` warnings, which CI's ambient `-D warnings` turns into a
/// failure of the very step that type-checks this file for that target.
#[cfg(not(target_arch = "wasm32"))]
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
    fixture = LocalFixture::new()
);

// Harness 2 — the testkit's macro verbatim, i.e. the default `#[tokio::test]`
// on the *multi-threaded* runtime. This is the literal text of CF-28 — "the
// existing suite, invoked against a `RefCell`-backed store" — and it passes,
// which is what refutes the premise recorded in this file's module docs.
#[cfg(not(target_arch = "wasm32"))]
happenstance_testkit::event_store_conformance!(
    mod_name = local_tokio_default,
    fixture = LocalFixture::new()
);

// Harness 3 — a caller-supplied emitter the testkit has never heard of.
//
// This is the half of CF-23 that the three shipped emitters cannot demonstrate:
// a runtime, or a runtime *flavour*, that the testkit does not enumerate. It
// needs no release of `happenstance-testkit` to exist. `current_thread` is also
// the closest native analogue of the single-threaded executor a Worker runs.
#[cfg(not(target_arch = "wasm32"))]
mod local_current_thread {
    use super::LocalFixture;

    // The emitter's whole contract, written out by a third party: hoist the
    // fixture behind an `async fn`, hand that function to each rule as an
    // `impl AsyncFn() -> F` so the *rule* decides how many instances it needs,
    // and report what comes back.
    //
    // The `.report(…)` is not politeness. `RuleOutcome` is `#[must_use]`, so an
    // emitter that drops it warns — and the workspace denies warnings, which is
    // what turns "an emitter must report the skip" from prose into a build
    // failure even for an emitter the testkit has never seen.
    async fn __conformance_fixture() -> impl happenstance_testkit::Fixture {
        LocalFixture::new()
    }

    macro_rules! emit_current_thread {
        ($($name:ident),* $(,)?) => {
            $(
                #[tokio::test(flavor = "current_thread")]
                async fn $name() {
                    happenstance_testkit::rules::$name(__conformance_fixture)
                        .await
                        .report(stringify!($name));
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
    fixture = LocalFixture::new()
);
