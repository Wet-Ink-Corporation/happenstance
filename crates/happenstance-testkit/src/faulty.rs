//! A store that refuses on demand: the input no in-process store produces.
//!
//! # Why both flavours are here, and why they are two types
//!
//! `trait_variant` emits a blanket `impl<T: SendEventStore> EventStore for T`,
//! so one type cannot carry a hand-written impl of each flavour — the
//! diagnostic is `error[E0119]`, naming the macro-generated impl. The two
//! wrappers are therefore siblings over one private core, and every delegating
//! call below is written in **fully-qualified** form (`EventStore::read(&self
//! .inner, …)`) rather than as method-call syntax: both flavour names are in
//! scope in this file, and `self.inner.read(..)` would be `error[E0034]` for an
//! inner store that satisfies both.

use core::pin::Pin;
use core::sync::atomic::{AtomicU32, Ordering};
use core::task::{Context, Poll};
use std::sync::Arc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, SendEventStore, SequencePosition, SequencedEvent,
};

// ---------------------------------------------------------------------------
// The shared core
// ---------------------------------------------------------------------------

/// How many failures of each kind are still armed.
///
/// `AtomicU32` rather than `Cell` because `append` and `read` take `&self` and
/// the `Send` flavour has to stay `Sync`. Held behind an `Arc` so that cloning
/// a wrapper produces a second **handle onto the same arming**, which is what
/// makes the count a property of the fixture rather than of one binding.
#[derive(Debug, Default)]
struct Armed {
    violate: AtomicU32,
    fail_read: AtomicU32,
}

impl Armed {
    /// Consumes one armed failure, reporting whether there was one to consume.
    ///
    /// `fetch_update` with a checked subtraction, so the count never wraps
    /// below zero and the **total** number of injected failures across every
    /// handle is at most the number armed — whatever order threads arrive in.
    fn take(counter: &AtomicU32) -> bool {
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |remaining| {
                remaining.checked_sub(1)
            })
            .is_ok()
    }
}

// ---------------------------------------------------------------------------
// The error
// ---------------------------------------------------------------------------

/// Why a read or an append through a faulty wrapper failed.
///
/// The wrapper cannot report the inner store's error as its own. The reference
/// store's is `pub enum MemoryStoreError {}` — uninhabited — so there is no
/// value of it to yield for an injected failure. Projecting this type is what
/// lets the wrapper compile over a store that cannot fail, and what lets a
/// caller tell *the fixture broke this on purpose* from *my store failed*,
/// which is the one question a test against a fixture has to answer.
///
/// # Examples
///
/// ```
/// use happenstance_core::{EventStore, MemoryEventStore};
/// use happenstance_core::{Query, ReadOptions};
/// use happenstance_testkit::{FaultyStore, FaultyStoreError};
///
/// let store = FaultyStore::new(MemoryEventStore::new()).fail_next_read(1);
/// let first = happenstance_testkit::block_on(async {
///     happenstance_core::collect(
///         store.read(&Query::all(), ReadOptions::new()),
///     )
///     .await
/// });
/// assert!(matches!(first, Err(FaultyStoreError::Injected)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FaultyStoreError<E> {
    /// The wrapper produced this failure because it was armed to.
    Injected,
    /// The wrapped store failed for its own reasons.
    ///
    /// The inner error is carried, never rendered: it stays reachable through
    /// [`source`](core::error::Error::source) so a caller reports the real
    /// failure rather than this wrapper's description of it.
    Store(E),
}

impl<E: core::fmt::Display> core::fmt::Display for FaultyStoreError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Injected => f.write_str(
                "the test fixture injected this failure; nothing underneath it went wrong",
            ),
            Self::Store(err) => write!(f, "the wrapped store failed: {err}"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for FaultyStoreError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Injected => None,
            Self::Store(err) => Some(err),
        }
    }
}

// ---------------------------------------------------------------------------
// The read stream
// ---------------------------------------------------------------------------

/// The stream a faulty wrapper hands back.
///
/// The inner stream is boxed so that it can be projected without `unsafe` and
/// without a pin-projection dependency; `Pin<Box<St>>` is `Unpin` whatever `St`
/// is, which is what lets `poll_next` take `&mut Self` at all, and it is `Send`
/// exactly when `St` is — so the `Send` flavour keeps its stream `Send`.
#[derive(Debug)]
struct InjectedRead<St> {
    injected: bool,
    inner: Pin<Box<St>>,
}

impl<St, E> Stream for InjectedRead<St>
where
    St: Stream<Item = Result<SequencedEvent, E>>,
{
    type Item = Result<SequencedEvent, FaultyStoreError<E>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // At the *first poll*, which is where a real store fails: a read that
        // errored at call time would be a fixture demonstrating a shape the port
        // does not have.
        if this.injected {
            this.injected = false;
            return Poll::Ready(Some(Err(FaultyStoreError::Injected)));
        }

        this.inner
            .as_mut()
            .poll_next(cx)
            .map(|item| item.map(|event| event.map_err(FaultyStoreError::Store)))
    }
}

// ---------------------------------------------------------------------------
// The two wrappers
// ---------------------------------------------------------------------------

/// Wraps a store so it can be made to fail on demand — the `!Send` flavour.
///
/// **Rejects: a retry loop that branches on `conflicting_position` being
/// `Some`.** An injected violation reports `None`, which is what a store with
/// no interactive transaction legitimately reports, so a loop written for an
/// in-process store never retries here — in the author's own test suite, in
/// under a second, with no database.
///
/// One instance is one arming. Cloning a wrapper produces a second handle onto
/// the **same** counters, so the total number of injected failures across every
/// clone is at most the number armed; which clone receives which failure is
/// deliberately unspecified.
///
/// Use [`SendFaultyStore`] where the inner store implements
/// [`SendEventStore`] and the wrapper has to
/// cross a thread boundary. Two types rather than one impl: `trait_variant`'s
/// blanket impl makes one type carrying both flavours `error[E0119]`.
///
/// # Examples
///
/// ```
/// use happenstance_core::{AppendError, Event, EventStore};
/// use happenstance_core::MemoryEventStore;
/// use happenstance_testkit::{FaultyStore, block_on};
///
/// let store = FaultyStore::new(MemoryEventStore::new()).violate_next(1);
/// let seat = Event::new("Seated", &b"{}"[..])?;
///
/// let refused = block_on(store.append(&[seat.clone()], None));
/// match refused {
///     Err(AppendError::ConditionViolated(violated)) => {
///         assert!(violated.conflicting_position.is_none());
///     }
///     other => panic!("expected a violation, got {other:?}"),
/// }
///
/// // The arming is spent, so the next append delegates.
/// assert!(block_on(store.append(&[seat], None)).is_ok());
/// # Ok::<(), happenstance_core::InvalidEventType>(())
/// ```
#[derive(Debug, Clone)]
pub struct FaultyStore<S: EventStore> {
    inner: S,
    armed: Arc<Armed>,
}

/// Wraps a store so it can be made to fail on demand — the `Send` flavour.
///
/// [`FaultyStore`]'s sibling, and everything on that page applies here: the
/// same arming, the same injected `conflicting_position: None`, the same wrong
/// caller rejected. What this one adds is that the wrapper is `Send + Sync`
/// and its read stream stays `Send`, so it can be held across an await inside
/// a spawned task.
///
/// **Rejects: a retry loop that branches on `conflicting_position` being
/// `Some`.**
///
/// # Examples
///
/// ```
/// use happenstance_core::{Event, MemoryEventStore, SendEventStore};
/// use happenstance_testkit::{SendFaultyStore, block_on};
///
/// let store = SendFaultyStore::new(MemoryEventStore::new());
/// let seat = Event::new("Seated", &b"{}"[..])?;
///
/// let landed = block_on(SendEventStore::append(&store, &[seat], None));
/// assert!(landed.is_ok());
/// # Ok::<(), happenstance_core::InvalidEventType>(())
/// ```
#[derive(Debug, Clone)]
pub struct SendFaultyStore<S: SendEventStore> {
    inner: S,
    armed: Arc<Armed>,
}

impl<S: EventStore> FaultyStore<S> {
    /// Wraps `inner` with nothing armed.
    ///
    /// Until something is armed the wrapper is transparent: it delegates every
    /// call and is indistinguishable from `inner`, which the conformance suite
    /// asserts rather than assumes.
    #[must_use]
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            armed: Arc::new(Armed::default()),
        }
    }

    /// Fails the next `n` appends with a violated condition naming **no**
    /// conflicting event.
    ///
    /// The inner store is never called for an armed append, so nothing is
    /// written, no position is allocated and the head does not move. Each armed
    /// call consumes one; when the count reaches zero the wrapper delegates
    /// again, so a fixture is never permanently poisoned.
    ///
    /// `n = 0` arms nothing and is legal — a caller writing
    /// `violate_next(attempts - 1)` is not surprised.
    #[must_use]
    pub fn violate_next(self, n: u32) -> Self {
        self.armed.violate.store(n, Ordering::Relaxed);
        self
    }

    /// Fails the next `n` reads at their **first polled item**.
    ///
    /// The count is consumed where the stream is *produced*, not where it is
    /// first polled, so a read whose stream is dropped undrained still spends
    /// one. The failure itself arrives lazily, as
    /// [`FaultyStoreError::Injected`], because that is where a real store fails
    /// — `read` is not `async` and has no call-time channel for one.
    ///
    /// `n = 0` arms nothing and is legal.
    #[must_use]
    pub fn fail_next_read(self, n: u32) -> Self {
        self.armed.fail_read.store(n, Ordering::Relaxed);
        self
    }
}

impl<S: SendEventStore> SendFaultyStore<S> {
    /// Wraps `inner` with nothing armed.
    ///
    /// See [`FaultyStore::new`]; this flavour differs only in the bound.
    #[must_use]
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            armed: Arc::new(Armed::default()),
        }
    }

    /// Fails the next `n` appends with a violated condition naming **no**
    /// conflicting event.
    ///
    /// See [`FaultyStore::violate_next`]. `n = 0` arms nothing and is legal.
    #[must_use]
    pub fn violate_next(self, n: u32) -> Self {
        self.armed.violate.store(n, Ordering::Relaxed);
        self
    }

    /// Fails the next `n` reads at their **first polled item**.
    ///
    /// See [`FaultyStore::fail_next_read`]. `n = 0` arms nothing and is legal.
    #[must_use]
    pub fn fail_next_read(self, n: u32) -> Self {
        self.armed.fail_read.store(n, Ordering::Relaxed);
        self
    }
}

impl<S: EventStore> EventStore for FaultyStore<S> {
    type Error = FaultyStoreError<S::Error>;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        InjectedRead {
            injected: take_read_failure(&self.armed),
            inner: Box::pin(EventStore::read(&self.inner, query, options)),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Above the delegation, and returning without it: the inner `append` is
        // never called for an armed failure, so nothing is written and no
        // position is allocated. A wrapper that appended and then reported
        // failure would be a fixture lying about the atomicity the port
        // promises.
        if take_violation(&self.armed) {
            return Err(injected_violation());
        }

        EventStore::append(&self.inner, events, condition)
            .await
            .map_err(|err| err.map_store(FaultyStoreError::Store))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(&self.inner)
            .await
            .map_err(FaultyStoreError::Store)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(&self.inner, id)
            .await
            .map_err(FaultyStoreError::Store)
    }
}

// `+ Sync` is the compiler's demand, not a preference: every method of the
// `Send` flavour holds `&self` across an await, and `&S: Send` is `S: Sync`.
// `SendEventStore` promises only `Send`, so the wrapper has to ask for the rest
// itself — RS-21's whole point, that the `Send` flavour's obligations are not
// inherited. `MemoryHandle` never had to write it because it is concrete and
// already `Sync`.
impl<S: SendEventStore + Sync> SendEventStore for SendFaultyStore<S> {
    type Error = FaultyStoreError<S::Error>;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        InjectedRead {
            injected: take_read_failure(&self.armed),
            inner: Box::pin(SendEventStore::read(&self.inner, query, options)),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // See the bare flavour's `append`: the inner store is never reached for
        // an armed failure.
        if take_violation(&self.armed) {
            return Err(injected_violation());
        }

        SendEventStore::append(&self.inner, events, condition)
            .await
            .map_err(|err| err.map_store(FaultyStoreError::Store))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        SendEventStore::head(&self.inner)
            .await
            .map_err(FaultyStoreError::Store)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        SendEventStore::contains_event_id(&self.inner, id)
            .await
            .map_err(FaultyStoreError::Store)
    }
}

/// The injected violation, spelled once for both flavours.
fn injected_violation<E>() -> AppendError<E> {
    AppendError::ConditionViolated(ConditionViolated::unspecified())
}

/// Whether an armed read failure was consumed, spelled once for both flavours.
fn take_read_failure(armed: &Armed) -> bool {
    Armed::take(&armed.fail_read)
}

/// Whether an armed append failure was consumed — once, for both flavours.
fn take_violation(armed: &Armed) -> bool {
    Armed::take(&armed.violate)
}
