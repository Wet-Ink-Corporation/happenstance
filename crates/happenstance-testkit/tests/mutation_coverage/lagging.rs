//! The lagging instrument: any registered fixture, with reads that trail its
//! acknowledged appends until the fixture settles the frontier.
//!
//! [`LaggingFixture`] stands where `happenstance-postgres` and
//! `happenstance-neon` stand behind `pg_snapshot_xmin`. An append that
//! returned `Ok` is held at once, and a `read` or `head` may not show it yet.
//! It declines `READ_YOUR_OWN_WRITES`, so the named rules that read back their
//! own writes reach `Fixture::settle_frontier` before they do. The
//! [`Mode`] decides what that call does:
//!
//! - [`Settles`] brings the frontier up to everything the inner store holds,
//!   which is what the Postgres and Neon fixtures do against a live server.
//! - [`Unsettling`] keeps the provided body, so every read-back rule must
//!   report a skip under `READ_YOUR_OWN_WRITES`.
//! - [`EmptySettle`] answers `Visible` and settles nothing: the lying
//!   override, `EmptyArmFrontierFixture`'s analogue one hook over.
//! - [`Tripwire`] settles as [`Settles`] does, and panics on any read or `head`
//!   issued while the lag runs. It finds the rules a lag can reach at all.
//!
//! # The lag, and what it deliberately does not hide
//!
//! The lag is a fixed function of the calls, not of a clock (CF-33, TST-10).
//! An acknowledged append starts it, and a settle ends it. While it runs, a
//! read hides every row above the frontier the last settle recorded, and every
//! row at all when no settle has happened yet.
//!
//! A **refused** append does not start the lag. A store that wrote a row and
//! then refused the batch therefore shows that row to the next read, rather
//! than having the instrument hide the defect the rule exists to find.
//!
//! The filter runs after the inner store's own `limit`, so a limited read made
//! during the lag can come back shorter than a lagging store would have
//! answered. No rule makes one: every limited read follows a settle with no
//! append between them, and [`pinned`] holds the instrument to that reading.

use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::sync::{Arc, Mutex};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, collect,
};
use happenstance_testkit::{Armed, Capability, Fixture, Settled};

use crate::harness::Subject;

// =====================================================================
// The lag
// =====================================================================

/// What every handle onto one lagging fixture shares.
#[derive(Debug, Default)]
struct LagState {
    /// Whether an acknowledged append has landed since the last settle.
    lagging: bool,
    /// The highest position the last settle saw held, or `None` before the
    /// first settle. It never moves backward.
    frontier: Option<SequencePosition>,
}

impl LagState {
    /// The rows a read issued now must hide.
    fn cap(&self) -> Cap {
        match (self.lagging, self.frontier) {
            (false, _) => Cap::ShowAll,
            (true, None) => Cap::HideAll,
            (true, Some(frontier)) => Cap::Below(frontier),
        }
    }
}

/// How much of the inner store's answer a read or `head` may show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cap {
    /// No lag: the inner answer, unchanged.
    ShowAll,
    /// A lag before any settle: no row is visible.
    HideAll,
    /// A lag after a settle: rows at or below this position are visible.
    Below(SequencePosition),
}

impl Cap {
    /// Whether a row at `position` is visible under this cap.
    fn shows(self, position: SequencePosition) -> bool {
        match self {
            Self::ShowAll => true,
            Self::HideAll => false,
            Self::Below(frontier) => position <= frontier,
        }
    }

    /// The inner store's head, capped.
    fn head(self, inner: Option<SequencePosition>) -> Option<SequencePosition> {
        match self {
            Self::ShowAll => inner,
            Self::HideAll => None,
            Self::Below(frontier) => inner.map(|head| head.min(frontier)),
        }
    }
}

/// One fixture instance's lag, shared by every handle onto it.
///
/// `Arc<Mutex<_>>` rather than `Rc<RefCell<_>>` (OWN-11), because one
/// instrument serves the concurrency family too, whose handles cross threads.
/// The state really is shared and mutable: every handle's `append` starts the
/// lag and the fixture's settle ends it. No guard is held across an `.await`.
#[derive(Debug, Default)]
struct Lag(Arc<Mutex<LagState>>);

impl Lag {
    /// Runs `step` under the lock.
    fn with<T>(&self, step: impl FnOnce(&mut LagState) -> T) -> T {
        let mut state = self
            .0
            .lock()
            .expect("no lag-state guard is held across a panic");
        step(&mut state)
    }
}

// =====================================================================
// The store
// =====================================================================

/// A handle onto the inner store, with the lag applied to what it shows.
#[derive(Debug)]
pub(crate) struct LaggingStore<S> {
    inner: S,
    lag: Lag,
    /// [`Mode::TRIPS`], copied at `connect`.
    trips: bool,
}

/// The panic [`Tripwire`] raises, which `parity.rs` reads back.
pub(crate) const TRIPPED: &str = "the lagging instrument's tripwire: a read during a lag";

impl<S> LaggingStore<S> {
    /// The cap a read or `head` issued now applies, after the tripwire.
    fn cap(&self) -> Cap {
        let cap = self.lag.with(|state| state.cap());
        assert!(!(self.trips && cap != Cap::ShowAll), "{TRIPPED}");
        cap
    }
}

/// The inner store's read stream, with the rows above the cap removed.
///
/// `Err` items always pass through: the lag hides rows, never failures.
struct Lagged<St> {
    inner: Pin<Box<St>>,
    cap: Cap,
}

impl<St, E> Stream for Lagged<St>
where
    St: Stream<Item = Result<SequencedEvent, E>>,
{
    type Item = Result<SequencedEvent, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(event))) if !this.cap.shows(event.position) => {}
                other => return other,
            }
        }
    }
}

impl<S: EventStore> EventStore for LaggingStore<S> {
    type Error = S::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Lagged {
            inner: Box::pin(self.inner.read(query, options)),
            cap: self.cap(),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let appended = self.inner.append(events, condition).await;
        if appended.is_ok() {
            self.lag.with(|state| state.lagging = true);
        }
        appended
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let inner = self.inner.head().await?;
        Ok(self.cap().head(inner))
    }

    // Held, whatever a read shows (ADR-0095), so the lag does not apply.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.inner.contains_event_id(id).await
    }
}

// =====================================================================
// What a settle does
// =====================================================================

/// What [`LaggingFixture`]'s `settle_frontier` does.
pub(crate) trait Mode {
    /// The behaviour, as data the fixture matches on.
    const SETTLE: Settle;
    /// Whether a read or `head` issued during a lag panics with [`TRIPPED`].
    const TRIPS: bool = false;
}

/// The three answers a lagging fixture can give a settle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Settle {
    /// Bring the frontier up to everything held, and answer `Visible`.
    Reveal,
    /// Keep the provided body.
    Provided,
    /// Answer `Visible` and change nothing.
    Lie,
}

/// The conformant override.
#[derive(Debug)]
pub(crate) struct Settles;

impl Mode for Settles {
    const SETTLE: Settle = Settle::Reveal;
}

/// A declining fixture with no override.
#[derive(Debug)]
pub(crate) struct Unsettling;

impl Mode for Unsettling {
    const SETTLE: Settle = Settle::Provided;
}

/// The lying override.
#[derive(Debug)]
pub(crate) struct EmptySettle;

impl Mode for EmptySettle {
    const SETTLE: Settle = Settle::Lie;
}

/// The conformant override, with a read during a lag made loud.
///
/// It finds the rules that read while the lag runs. Only those can answer a
/// lagging store differently from the store it wraps, because after a settle the
/// lag shows everything until the next acknowledged append.
#[derive(Debug)]
pub(crate) struct Tripwire;

impl Mode for Tripwire {
    const SETTLE: Settle = Settle::Reveal;
    const TRIPS: bool = true;
}

// =====================================================================
// The fixture
// =====================================================================

/// Any fixture `F`, whose reads trail its acknowledged appends.
///
/// Every capability and arming comes from `F` except two.
/// `READ_YOUR_OWN_WRITES` is declined, which is the whole subject.
/// `HELD_ABOVE_FRONTIER` is `F`'s when `F` supports it, and is otherwise
/// declined in this instrument's words, because a fixture that declines
/// read-your-own-writes owes that answer (CF-18).
#[derive(Debug)]
pub(crate) struct LaggingFixture<F, M = Settles> {
    inner: F,
    lag: Lag,
    mode: PhantomData<M>,
}

impl<F: Fixture, M: Mode> LaggingFixture<F, M> {
    /// The highest position the inner store holds, read through a fresh
    /// inner handle so that no handle the rule holds is disturbed.
    async fn held_head(&self) -> Option<SequencePosition> {
        let probe = self.inner.connect().await;
        let held = collect(probe.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the settle's own read of the inner store succeeds");
        held.iter().map(|event| event.position).max()
    }
}

impl<F: Fixture, M: Mode> Fixture for LaggingFixture<F, M> {
    type Store = LaggingStore<F::Store>;

    const SECOND_HANDLE: Capability = F::SECOND_HANDLE;
    const REOPEN: Capability = F::REOPEN;
    const READ_YOUR_OWN_WRITES: Capability = Capability::declined(
        "this instrument hides every acknowledged append from reads and head until the \
         fixture settles its frontier, which is the subject it exists for",
    );
    const MID_BATCH_FAULT: Capability = F::MID_BATCH_FAULT;
    const READ_FAULT: Capability = F::READ_FAULT;
    const MAX_EVENT_DATA_LEN: Option<usize> = F::MAX_EVENT_DATA_LEN;
    const MAX_TAGS_PER_EVENT: Option<usize> = F::MAX_TAGS_PER_EVENT;
    const MAX_EVENTS_PER_BATCH: Option<usize> = F::MAX_EVENTS_PER_BATCH;
    const MAX_METADATA_LEN: Option<usize> = F::MAX_METADATA_LEN;
    const MAX_TAGS_PER_BATCH: Option<usize> = F::MAX_TAGS_PER_BATCH;
    const HELD_ABOVE_FRONTIER: Capability = if F::HELD_ABOVE_FRONTIER.is_supported() {
        Capability::SUPPORTED
    } else {
        Capability::declined(
            "the wrapped fixture cannot hold a row above a frontier, and this instrument's \
             lag ends at every settle, so it cannot hold one there either",
        )
    };

    async fn connect(&self) -> Self::Store {
        LaggingStore {
            inner: self.inner.connect().await,
            // A cheap handle onto the one lag every handle shares.
            lag: Lag(Arc::clone(&self.lag.0)),
            trips: M::TRIPS,
        }
    }

    fn reopen(&self) -> impl Future<Output = ()> {
        self.inner.reopen()
    }

    fn arm_mid_batch_fault(&self, after: usize) -> impl Future<Output = ()> {
        self.inner.arm_mid_batch_fault(after)
    }

    fn arm_read_fault(&self) -> impl Future<Output = ()> {
        self.inner.arm_read_fault()
    }

    fn arm_held_above_frontier(&self) -> impl Future<Output = Armed> {
        self.inner.arm_held_above_frontier()
    }

    async fn settle_frontier(&self) -> Settled {
        match M::SETTLE {
            Settle::Reveal => {
                let held = self.held_head().await;
                self.lag.with(|state| {
                    state.frontier = state.frontier.max(held);
                    state.lagging = false;
                });
                Settled::Visible
            }
            // The inner fixture keeps the provided body, so this is that body,
            // not a copy of it.
            Settle::Provided => self.inner.settle_frontier().await,
            Settle::Lie => Settled::Visible,
        }
    }
}

impl<F: Subject, M: Mode> Subject for LaggingFixture<F, M> {
    const NAME: &'static str = F::NAME;

    fn open() -> Self {
        Self {
            inner: F::open(),
            lag: Lag::default(),
            mode: PhantomData,
        }
    }
}

// Which named rules read back their own writes, as data.
mod inventory;
// The wrapper that panics if a rule settles a fixture that reads its writes.
mod never_asked;
// The meta-tests that run the suite against this instrument.
mod parity;
mod settled;

// =====================================================================
// The instrument, pinned
// =====================================================================

/// The four properties every measurement in `lagging/settled.rs` rests on, held
/// directly so that none of them can quietly stop being true.
mod pinned {
    use happenstance_core::{AppendError, Event, EventStore, Query, ReadOptions, SequencePosition};
    use happenstance_testkit::fixtures::{condition, event, query_of_types};
    use happenstance_testkit::{Fixture, Settled, block_on};

    use super::{EmptySettle, LaggingFixture, Settles, Unsettling};
    use crate::harness::Subject;
    use crate::variants::GappedPositionFixture;

    type Lagging = LaggingFixture<GappedPositionFixture, Settles>;

    /// Appends unconditionally and returns the position the store assigned.
    fn appended<S: EventStore>(store: &S, events: &[Event]) -> SequencePosition {
        match block_on(store.append(events, None)) {
            Ok(position) => position,
            Err(err) => panic!("an unconditional append to the gapped store lands: {err:?}"),
        }
    }

    /// The positions a full read shows, in order.
    fn shown<S: EventStore>(store: &S) -> Vec<SequencePosition> {
        block_on(happenstance_core::collect(
            store.read(&Query::all(), ReadOptions::new()),
        ))
        .expect("the gapped store reads")
        .iter()
        .map(|event| event.position)
        .collect()
    }

    fn head<S: EventStore>(store: &S) -> Option<SequencePosition> {
        block_on(store.head()).expect("the gapped store answers head")
    }

    #[test]
    fn an_acknowledged_append_is_hidden_until_a_settle() {
        let fixture = Lagging::open();
        let store = block_on(fixture.connect());
        let at = appended(&store, &[event("A")]);

        assert_eq!(shown(&store), []);
        assert_eq!(head(&store), None);

        assert_eq!(block_on(fixture.settle_frontier()), Settled::Visible);
        assert_eq!(shown(&store), [at]);
        assert_eq!(head(&store), Some(at));
    }

    #[test]
    fn a_refused_append_does_not_start_the_lag() {
        let fixture = Lagging::open();
        let store = block_on(fixture.connect());
        let at = appended(&store, &[event("A")]);
        assert_eq!(block_on(fixture.settle_frontier()), Settled::Visible);

        let guard = condition(query_of_types(&["A"]));
        let refused = block_on(store.append(&[event("A")], Some(&guard)));
        assert!(
            matches!(refused, Err(AppendError::ConditionViolated(_))),
            "a matching condition refuses: {refused:?}"
        );
        assert_eq!(shown(&store), [at], "a refusal leaves the view as it was");
    }

    #[test]
    fn a_settle_reveals_what_is_held_and_the_frontier_never_retreats() {
        let fixture = Lagging::open();
        let store = block_on(fixture.connect());
        let settled = appended(&store, &[event("A"), event("B")]);
        assert_eq!(block_on(fixture.settle_frontier()), Settled::Visible);
        let before = shown(&store);
        assert_eq!(before.last(), Some(&settled));

        let later = appended(&store, &[event("C")]);
        assert_eq!(shown(&store), before, "the lag shows the settled rows only");
        assert_eq!(
            head(&store),
            Some(settled),
            "a lagging head stops at the frontier"
        );

        assert_eq!(block_on(fixture.settle_frontier()), Settled::Visible);
        assert_eq!(
            head(&store),
            Some(later),
            "the next settle moves it forward"
        );
    }

    #[test]
    fn the_other_modes_answer_without_revealing() {
        let unsettling = LaggingFixture::<GappedPositionFixture, Unsettling>::open();
        let store = block_on(unsettling.connect());
        appended(&store, &[event("A")]);
        assert_eq!(block_on(unsettling.settle_frontier()), Settled::Nothing);
        assert_eq!(shown(&store), []);

        let lying = LaggingFixture::<GappedPositionFixture, EmptySettle>::open();
        let store = block_on(lying.connect());
        appended(&store, &[event("A")]);
        assert_eq!(block_on(lying.settle_frontier()), Settled::Visible);
        assert_eq!(shown(&store), [], "the lie reveals nothing");
    }
}
