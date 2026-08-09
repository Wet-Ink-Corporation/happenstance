//! The **conformant** controls, and the capability instrument.
//!
//! Mutants catch under-specification: a rule that asserts too little lets a
//! wrong store through. Conformant variants catch the opposite and rarer
//! failure — **over-specification**, a rule that asserts more than the
//! specification requires, passes against `MemoryEventStore`, and fails a
//! perfectly legal adapter in the field. Nothing in the workspace catches that
//! today, which is CF-5's whole argument.
//!
//! [`GappedPositionStore`] is the first one: positions in **steps of seven from
//! 4096**. Every clause of the specification it touches permits that — positions
//! must be unique and strictly increasing, and gaps are explicitly allowed — so
//! it MUST pass every registered rule. Any rule it fails is a rule that quietly
//! assumed density or a literal starting value, and CF-6 says the *rule* is
//! wrong.
//!
//! [`PagedStreamStore`] is the second, on a different axis: its `read` stream
//! returns [`Poll::Pending`] between every item. Nothing in the specification
//! says a read stream is ready on first poll, so a rule it fails is a rule that
//! assumed a store with no network under it. It also exercises
//! `happenstance_testkit::block_on`'s parking path, which nothing else in the
//! workspace reaches.
//!
//! This file also holds [`DecliningFixture`], which is not a variant at all: it
//! is the instrument `capability_skips_are_reported` needs, and it is
//! deliberately absent from `for_each_mutant!`. See its documentation.

use core::cell::{Cell, RefCell};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MIN_SUPPORTED_EVENT_DATA_LEN,
    MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_TAGS_PER_EVENT, Query, ReadOptions,
    SequencePosition, SequencedEvent, StoreLimit,
};
use happenstance_testkit::{Capability, Fixture};

use crate::correct::{Log, LogError, LogStore, Snapshot, dense};
use crate::harness::Subject;

// =====================================================================
// CF-5, CF-6 — the gapped conformant variant
// =====================================================================

/// The first position a [`GappedPositionStore`] hands out.
///
/// Chosen well clear of 1 so that a rule which happens to compare against a
/// literal `1` fails loudly rather than by one.
const FIRST_GAPPED: u64 = 4096;

/// The distance between consecutive positions.
///
/// Seven rather than two, so that an off-by-one in a rule and a genuine gap
/// cannot be confused, and prime so that no position is ever a multiple of
/// another.
const GAP: u64 = 7;

/// Allocates positions in steps of [`GAP`] starting at [`FIRST_GAPPED`].
///
/// This is a legal policy, not a defect. A real adapter reaches it by allocating
/// from a sequence with `CACHE 7`, by encoding a shard id in the low bits, or by
/// using a transaction id as the position — all three leave gaps and none of
/// them starts at 1.
fn gapped(previous: Option<SequencePosition>) -> SequencePosition {
    let raw = match previous {
        Some(previous) => previous.get().saturating_add(GAP),
        None => FIRST_GAPPED,
    };
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// A completely conformant store that assigns sparse positions and survives a
/// reopen.
///
/// Two things are separated here that a naive fixture conflates. `live` is the
/// process state a reopen throws away; `committed` is the durable medium a
/// reopen replays. Because [`gapped`] is a pure function of the previous
/// position, replaying the committed events reproduces the *positions* and not
/// merely the payloads — which is the half
/// `acknowledged_writes_survive_a_reopen`'s second assertion checks, and the
/// reason this variant can support `REOPEN` honestly where `MemoryFixture`
/// cannot.
///
/// # What replaying does *not* reproduce, and why it is invisible here
///
/// Positions are a pure function of the previous position; a `RecordedAt` is
/// not. A durable side of `Event` can only be reopened by replaying, and a
/// replay **restamps** — which is exactly the shape
/// `recorded_time_survives_a_reopen` exists to reject, and which
/// `tests/fixture_instruments.rs`'s `DurableFixture` avoids by carrying
/// `SequencedEvent` on its own durable side and restoring rather than
/// replaying.
///
/// This variant passes that rule anyway, and only because `correct::stamp`
/// spends a **constant** — `TEST_RECORDED_AT`, fixed rather than read from a
/// clock because CF-33 forbids one. So the restamp lands on the same value it
/// replaced and nothing can see it. That is a property of the instrument, not
/// of the store: the day this binary's stamp varies, this variant starts
/// failing `recorded_time_survives_a_reopen`, and the answer will be to give
/// the durable side `SequencedEvent`s rather than to weaken the rule. Recorded
/// here so that the failure arrives with its cause attached.
///
/// It carries the **fault-injection** capability too, and that is a second axis
/// on one instrument rather than an accident. `capability_skips_are_reported`'s
/// closing assertion is that a fixture supporting everything skips nothing —
/// which catches a rule whose `require!` gate reads the wrong const, and which
/// needs one fixture that genuinely supports everything. Splitting the fault onto
/// a fourth variant would leave that assertion with nothing to run against, and
/// the two axes do not interact: the fault fires *inside* the transaction, so
/// neither the live log nor the durable record moves, and every position stays
/// where [`gapped`] put it.
#[derive(Debug, Clone)]
pub(crate) struct GappedPositionStore {
    live: Rc<RefCell<Log>>,
    committed: Rc<RefCell<Vec<Event>>>,
    fault: Fault,
}

impl EventStore for GappedPositionStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.live
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|log| log.select(query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Fires once, and inside the transaction: whatever rows this store had
        // begun writing go back with it, so the caller's `Err` and the log agree.
        // `mutants::NoTransactionStore` is this store with the `BEGIN` removed.
        if let Some(after) = self.fault.get()
            && events.len() > after
        {
            self.fault.set(None);
            return Err(AppendError::Store(LogError::WriteFailed));
        }

        // The capacity refusal, reported through the variant VT-25 introduces
        // rather than through `Self::Error`. This is a second conformant
        // difference from `MemoryEventStore` and it is deliberate: VT-21 – VT-24
        // require every store to document a limit and to refuse beyond it
        // distinguishably.
        //
        // It precedes the write and returns before `self.committed` is extended,
        // so a refused append moves neither the live log nor the durable record
        // and `acknowledged_writes_survive_a_reopen` is untouched. The comparisons
        // are `>`, so the three guaranteed-minimum rules — which write exactly the
        // floor — are still accepted.
        if let Some(event) = events
            .iter()
            .find(|event| event.data().len() > MIN_SUPPORTED_EVENT_DATA_LEN)
        {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventDataLen,
                len: event.data().len(),
            });
        }
        if let Some(event) = events
            .iter()
            .find(|event| event.tags().len() > MIN_SUPPORTED_TAGS_PER_EVENT)
        {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::TagsPerEvent,
                len: event.tags().len(),
            });
        }
        if events.len() > MIN_SUPPORTED_EVENTS_PER_BATCH {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: events.len(),
            });
        }

        let position = {
            let mut log = self
                .live
                .try_borrow_mut()
                .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
            log.append(events, condition)?
        };

        // Recorded only on success, and only after the live store has accepted
        // the batch. Acknowledging before this line is `LosingFixture`'s defect
        // in `tests/fixture_instruments.rs`; this store is the correct sibling.
        self.committed
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?
            .extend(events.iter().cloned());

        Ok(position)
    }

    // Both forwarded to the correct store over the same log rather than written
    // out again: `Log`'s events are private to `correct`, and a second copy of
    // these bodies would be a second way for a *conformant control* to differ
    // from correct — which is the one thing this variant is not allowed to have.
    // Note what forwarding buys here specifically: `head_of` answers with the
    // last position *assigned*, which under [`gapped`] is 4096 + 7n and never a
    // count of events. `live` and not `committed`, because head and identity are
    // properties of what this handle can read back, and a reopen is what replays
    // one into the other.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        LogStore::over(&self.live).head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        LogStore::over(&self.live).contains_event_id(id).await
    }
}

/// One backing store for [`GappedPositionStore`], and any number of handles onto
/// it.
#[derive(Debug)]
pub(crate) struct GappedPositionFixture {
    committed: Rc<RefCell<Vec<Event>>>,
    /// Replaced wholesale by `reopen`, which is why it is a `RefCell` around the
    /// `Rc` rather than the other way round: a handle taken before the call
    /// keeps the old log, exactly as a real connection would keep talking to a
    /// closed file.
    live: RefCell<Rc<RefCell<Log>>>,
    fault: Fault,
}

impl Fixture for GappedPositionFixture {
    type Store = GappedPositionStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    // CF-40. Stated at the floors rather than above them, so that this variant is
    // simultaneously the tightest legal store and a passing one: VT-21 – VT-24
    // make these numbers the minimum every store must accept, and a store that
    // accepts exactly the minimum and refuses beyond it is conformant. Choosing
    // anything larger would leave the refusal path exercised at a number no clause
    // has an opinion about.
    //
    // This is why the variant is the one that states them: it is already the
    // fixture that supports every capability, and `capability_skips_are_reported`
    // asserts it skips *nothing* — so a portfolio in which no conformant variant
    // has a ceiling is a portfolio in which
    // `append_reports_exceeded_store_limits` never runs.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(MIN_SUPPORTED_EVENT_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(MIN_SUPPORTED_TAGS_PER_EVENT);
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(MIN_SUPPORTED_EVENTS_PER_BATCH);

    async fn connect(&self) -> Self::Store {
        GappedPositionStore {
            live: Rc::clone(&self.live.borrow()),
            committed: Rc::clone(&self.committed),
            fault: Rc::clone(&self.fault),
        }
    }

    async fn arm_mid_batch_fault(&self, after: usize) {
        self.fault.set(Some(after));
    }

    async fn reopen(&self) {
        let committed = self.committed.borrow().clone();

        let mut log = Log::new(gapped);
        if !committed.is_empty() {
            // One batch rather than one per original append: `gapped` allocates
            // from the previous position alone, so batching cannot change the
            // positions. A store whose allocator depended on batch boundaries
            // would need the boundaries recorded too — worth knowing before
            // phase 8 copies this shape.
            let replayed = log.append(&committed, None);
            assert!(
                replayed.is_ok(),
                "replaying the committed log must not be rejected: {replayed:?}"
            );
        }

        *self.live.borrow_mut() = Rc::new(RefCell::new(log));
    }
}

impl Subject for GappedPositionFixture {
    const NAME: &'static str = "GappedPositionStore";

    fn open() -> Self {
        Self {
            committed: Rc::new(RefCell::new(Vec::new())),
            live: RefCell::new(Rc::new(RefCell::new(Log::new(gapped)))),
            fault: Rc::new(Cell::new(None)),
        }
    }
}

// =====================================================================
// CF-5 — the network-shaped conformant variant
// =====================================================================

/// A stream that is never ready on first poll.
///
/// Between every item it returns [`Poll::Pending`] and wakes itself, modelling a
/// store that fetches a page at a time over a network. Nothing in the
/// specification says a read stream must be ready immediately, so a rule this
/// fails is a rule that assumed it.
///
/// It is also the only thing in the workspace that reaches
/// `happenstance_testkit::block_on`'s **parking** path. Every other harness
/// drives streams that are ready on first poll, so `block_on`'s `Poll::Pending`
/// arm — `std::thread::park` woken by the `ParkWaker` — has never executed. The
/// testkit's own machinery gets a positive control here.
#[derive(Debug)]
pub(crate) struct PagedStream {
    /// Surfaced first, like [`Snapshot`]'s, so a read failure arrives lazily.
    error: Option<LogError>,
    events: std::vec::IntoIter<SequencedEvent>,
    /// Whether the next poll is the one that fetches rather than the one that
    /// waits.
    ready: bool,
}

impl PagedStream {
    /// A paging stream over `selected`, or over the failure that prevented
    /// reading it.
    fn new(selected: Result<Vec<SequencedEvent>, LogError>) -> Self {
        match selected {
            Ok(events) => Self {
                error: None,
                events: events.into_iter(),
                ready: false,
            },
            Err(err) => Self {
                error: Some(err),
                events: Vec::new().into_iter(),
                ready: false,
            },
        }
    }
}

impl Stream for PagedStream {
    type Item = Result<SequencedEvent, LogError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if !this.ready {
            this.ready = true;
            // Waking before returning `Pending` is what makes this a *slow*
            // stream rather than a hung one: `block_on` parks, the unpark token
            // is already granted, and the park returns immediately. A stream
            // that returned `Pending` without waking would hang the whole binary
            // with no message, which is the failure `catch_unwind` cannot
            // rescue.
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
        this.ready = false;

        if let Some(err) = this.error.take() {
            return Poll::Ready(Some(Err(err)));
        }
        Poll::Ready(this.events.next().map(Ok))
    }
}

/// A conformant store whose `read` stream yields one item per two polls.
#[derive(Debug, Clone)]
pub(crate) struct PagedStreamStore(Rc<RefCell<Log>>);

impl EventStore for PagedStreamStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        PagedStream::new(
            self.0
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|log| log.select(query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let mut log = self
            .0
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        log.append(events, condition)
    }

    // Forwarded, as [`GappedPositionStore`]'s are. Deliberately *not* made to
    // pend first: this variant's axis is stream readiness, and neither of these
    // returns a stream, so a `Poll::Pending` future here would be a second
    // difference on a control that is only allowed one.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        LogStore::over(&self.0).head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        LogStore::over(&self.0).contains_event_id(id).await
    }
}

/// One log, and any number of paging handles onto it.
#[derive(Debug)]
pub(crate) struct PagedStreamFixture(Rc<RefCell<Log>>);

impl Fixture for PagedStreamFixture {
    type Store = PagedStreamStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    // Declined honestly, and the cost is bounded: `conformant_variants_pass_everything`
    // requires every rule to have *executed* against at least one variant, and
    // `GappedPositionFixture` supports `REOPEN`, so the control is still over
    // every rule, which is the property the meta-test asserts rather than a count.
    const REOPEN: Capability = Capability::declined(
        "this variant exists for the stream-readiness axis; its log is a Vec \
         behind an Rc with no durable medium",
    );

    async fn connect(&self) -> Self::Store {
        PagedStreamStore(Rc::clone(&self.0))
    }
}

impl Subject for PagedStreamFixture {
    const NAME: &'static str = "PagedStreamStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Log::new(dense))))
    }
}

// =====================================================================
// ES-18 — the fault-injection axis
// =====================================================================

/// How far into the next batch an armed fault fires.
///
/// A `Cell` rather than a flag on the store, because the fixture arms it and a
/// *handle* meets it: the two are different objects, and the arming has to reach
/// whichever handle the rule happens to be holding.
///
/// It lives here rather than beside either store because both ends of the axis
/// need it — [`GappedPositionStore`] rolls back when it fires and
/// `mutants::NoTransactionStore` does not — and a defect is only evidence when
/// the two differ in one thing.
pub(crate) type Fault = Rc<Cell<Option<usize>>>;

// =====================================================================
// CF-18 — the capability instrument
// =====================================================================

/// A fixture that declines **every** capability, including the one that is a
/// MUST.
///
/// # Why this is not registered in `for_each_mutant!`
///
/// It is not a mutant and it is not a variant. A mutant is a store that fails a
/// named rule; this one fails nothing — it *skips*. Registering it would make
/// `mutants_fail_exactly_their_declared_rules` assert over a store whose whole
/// content is the absence of assertions.
///
/// It exists so `capability_skips_are_reported` can check the half of CF-18 that
/// only an unco-operative fixture can show: that a capability-gated rule is
/// still **emitted and answered**, with the fixture's own reason attached,
/// rather than silently omitted.
///
/// It is also the wrong implementation that the `SECOND_HANDLE` MUST is shown to
/// reject. `two_handles_observe_each_others_appends` uses `must!` rather than
/// `require!`, so against this fixture it **fails**, quoting
/// [`SECOND_HANDLE_REASON`](Self::SECOND_HANDLE_REASON) — which is what stops
/// that assertion being one no implementation can fire.
///
/// # `connect` panics on the second call, on purpose
///
/// Declining `SECOND_HANDLE` is a claim, and a claim a rule could ignore. If a
/// capability-gated rule ran anyway — because someone deleted its `require!`
/// gate, or because `require!` stopped reading the right const — it would open a
/// second handle here and *pass quietly*, since a `LogStore` clone is perfectly
/// correct. The panic converts that silent pass into a loud
/// `Verdict::Panicked` the meta-test rejects.
#[derive(Debug)]
pub(crate) struct DecliningFixture {
    store: LogStore,
    connects: Cell<usize>,
}

impl DecliningFixture {
    /// The reason this fixture gives for declining `SECOND_HANDLE`.
    ///
    /// A `const` rather than a literal repeated in the test, because the point
    /// of the assertion is that the *fixture's own* words reach the report. Two
    /// copies of the string would let the report carry someone else's.
    pub(crate) const SECOND_HANDLE_REASON: &'static str = "this instrument declines everything so that a skip has something to be \
         reported about; it is not a conformant fixture";

    /// The reason this fixture gives for declining `REOPEN`.
    pub(crate) const REOPEN_REASON: &'static str =
        "this instrument holds a Vec behind an Rc and has no durable medium";

    /// The reason this fixture gives for declining `MID_BATCH_FAULT`.
    ///
    /// Stated rather than inherited from the trait's default, because the whole
    /// content of this fixture is that a skip carries **its own** words: a
    /// default reason reaching the report would be the testkit's, and
    /// `capability_skips_are_reported` would then be checking a string this
    /// fixture never said.
    pub(crate) const MID_BATCH_FAULT_REASON: &'static str = "this instrument declines everything, and it has no transaction to fail \
         part way through in any case";

    /// A fresh instrument over a completely correct, densely-allocating store.
    pub(crate) fn new() -> Self {
        Self {
            store: LogStore::new(dense),
            connects: Cell::new(0),
        }
    }
}

impl Fixture for DecliningFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::declined(Self::SECOND_HANDLE_REASON);
    const REOPEN: Capability = Capability::declined(Self::REOPEN_REASON);
    const MID_BATCH_FAULT: Capability = Capability::declined(Self::MID_BATCH_FAULT_REASON);

    async fn connect(&self) -> Self::Store {
        let taken = self.connects.get() + 1;
        self.connects.set(taken);
        assert!(
            taken == 1,
            "`DecliningFixture` declines SECOND_HANDLE, so a rule that opened a \
             second handle ignored the capability gate"
        );
        self.store.clone()
    }
}

impl Subject for DecliningFixture {
    const NAME: &'static str = "DecliningFixture";

    fn open() -> Self {
        Self::new()
    }
}
