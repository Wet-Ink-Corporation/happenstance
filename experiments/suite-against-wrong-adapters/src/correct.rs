//! **Copied verbatim** from `crates/happenstance-testkit/tests/mutation_coverage/correct.rs`
//! at `56ef6c5`, with two mechanical edits and nothing else:
//!
//! 1. `pub(crate)` → `pub`, because this crate's stores live in `src/` and its
//!    fixtures live in `tests/`, which is a second crate.
//! 2. One intra-doc link to `crate::mutants::Defect::select` — a type that does
//!    not exist here — demoted to prose.
//!
//! It is copied rather than depended on because `tests/` is not a library: there
//! is no way to `use` the testkit's own mutation-coverage primitives from
//! outside its test binary. Copying is what makes the comparison honest — every
//! store in this experiment is the *repository's own* correct core with one step
//! changed, exactly as the testkit's registered mutants are, so a rule that goes
//! red here went red for the declared reason.
//!
//! The *correct* primitives every store in this test binary delegates to.
//!
//! A mutant is only evidence when the defect is the **only** difference. A
//! hand-written log would also differ in position assignment, ordering and
//! condition evaluation, and a rule that went red could then be red for any of
//! those instead — which is the failure `mutants_fail_exactly_their_declared_rules`
//! exists to catch and the one it would then be catching in the *instrument*.
//! So the correct behaviour is written once, here, in steps small enough that a
//! mutant can replace exactly one of them.
//!
//! `tests/local_conformance.rs`'s `select_indices` / `apply_append` pair is the
//! seed: the repository had already discovered that two mutants sharing one
//! correct core is what makes them comparable. This generalises it, and the
//! generalisation is deliberate rather than tidy — **phase 4 freezes the port
//! immediately after phase 3**, so every store in this binary is rewritten once,
//! early. Twenty stores that each spell out filtering and allocation is twenty
//! rewrites; twenty stores that call [`select`] and [`commit`] is one file.
//!
//! # The steps, and what a mutant does to them
//!
//! | Step | Function | The wrong version it invites |
//! |---|---|---|
//! | filter | [`matching`] | drop the tag join, keep only the type |
//! | order + `from` | [`ordered`] | `from` exclusive; forwards order when backwards was asked |
//! | truncate | [`truncated`] | `LIMIT` applied before the filter |
//! | condition | [`violation`] | ignore `after`; ignore tags; per-item statements |
//! | allocate | [`Allocate`] | `COUNT(*) + 1`; a per-session cache |
//! | sequence | [`sequence`] | one position bound for every row of the batch |
//! | commit | [`commit_with`] | the condition evaluated before the emptiness check; partial writes |
//!
//! A mutant built from these calls the correct step for everything except the
//! one column it is written to be wrong in. [`commit_with`] is what makes that
//! possible for the two steps a store cannot reach from outside: it takes the
//! probe and the sequencer as parameters, so replacing one of them costs a
//! function rather than a reimplementation of the write path.

use core::cell::RefCell;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Guard, Query,
    ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};

// =====================================================================
// Position allocation
// =====================================================================

/// How a store hands out the next sequence position, given the highest one it
/// has already assigned.
///
/// A bare `fn` pointer rather than a trait, and the reason is the harness rather
/// than taste: a [`Log`] carrying a `Box<dyn Fn…>` would not be `UnwindSafe`,
/// and the probe machinery in `harness.rs` is built so that no
/// `AssertUnwindSafe` appears anywhere. A function pointer is `Copy`, `Debug`
/// and `UnwindSafe`, so it costs nothing to hold.
///
/// It takes `Option` rather than a count because *only the last position* is a
/// legal input to allocation. Handing an allocator `events.len()` is precisely
/// the `COUNT(*) + 1` mistake CF-2 names, and a signature that cannot express it
/// is one fewer thing to get wrong in the correct core.
pub type Allocate = fn(Option<SequencePosition>) -> SequencePosition;

/// Dense allocation from 1 — what `MemoryEventStore` does.
///
/// The specification permits gaps, so this is *a* conformant policy rather than
/// *the* conformant policy; `variants.rs` holds the other end of that axis.
pub fn dense(previous: Option<SequencePosition>) -> SequencePosition {
    match previous {
        Some(previous) => previous.next().unwrap_or(SequencePosition::FIRST),
        None => SequencePosition::FIRST,
    }
}

// =====================================================================
// The error every store here reports
// =====================================================================

/// The failure a store built on [`Log`] can report of its own.
///
/// One inhabited variant rather than an uninhabited `Never`, because the borrow
/// family of mutants needs somewhere to put a conflicting borrow that is *not* a
/// panic — the choice between `try_borrow` and `borrow` is the choice between
/// reporting the failure and panicking, and both halves have to be expressible
/// for a mutant to be able to pick the wrong one.
///
/// Written by hand because the testkit does not depend on `thiserror`;
/// `EventStore::Error: core::error::Error` is the obligation being discharged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    /// Another borrow of the log was live.
    AlreadyBorrowed,
    /// The database reported a uniqueness conflict.
    ///
    /// No correct store here can produce it. It exists because
    /// `ViolationAsStoreErrorStore` models the DCB uniqueness shape implemented
    /// as a partial unique index, where the driver hands back `23505
    /// unique_violation` and the adapter passes it straight through — and that
    /// defect is only expressible if the store's own error type has somewhere
    /// for a driver error to land.
    UniqueViolation,
    /// A condition named an `after` this store has never assigned.
    ///
    /// Also unproducible by any correct store here, and for the same reason as
    /// the variant above: `AfterValidatedAgainstHeadStore` models the adapter
    /// that checks an incoming `after` against its own head and refuses a
    /// position it does not recognise, and refusing needs somewhere to refuse
    /// *to*. ES-28 says a condition whose `after` is at or beyond the head must
    /// neither reject nor error, so the defect is only expressible if erroring
    /// is expressible.
    UnknownPosition,
    /// The write of one row of a batch failed.
    ///
    /// The injected fault `Fixture::MID_BATCH_FAULT` arms, and the only variant
    /// here a *conformant* store produces: `GappedPositionStore` in
    /// `variants.rs` meets it inside its transaction and rolls back,
    /// `NoTransactionStore` in `mutants.rs` meets it with no transaction to roll
    /// back. The two answer the caller identically and leave two different logs,
    /// which is the whole content of ES-18's second rule.
    WriteFailed,
    /// A `NOT NULL` column was handed `NULL`.
    ///
    /// The fourth unproducible-by-a-correct-store variant.
    /// `EmptyPayloadIsNullStore` normalises a zero-length blob to `NULL` on the
    /// way in — one helper serving both blob columns, written for `metadata`
    /// where the option is genuine — and the `data` column is `NOT NULL`, so
    /// SQLite answers `NOT NULL constraint failed: event.data` and the adapter
    /// surfaces it as its own error. Modelling that as a panic would leave
    /// `append_preserves_an_empty_payload` satisfied by the store falling over
    /// rather than by the rule's own assertion.
    NotNullViolation,
    /// A value exceeded the width the schema declared for it.
    ///
    /// `PayloadCeilingStore`'s answer: an undocumented row-size ceiling, met at
    /// write time rather than declared up front. VT-25 gives this a *contract*
    /// spelling — `AppendError::ExceedsStoreLimit` — at phase 4; until then the
    /// only channel an adapter has for it is `AppendError::Store`, which is
    /// exactly the clause's complaint and exactly what this variant models.
    ValueTooLarge,
    /// The statement carried more bound parameters than the driver accepts.
    ///
    /// `BatchParameterCeilingStore`'s answer: one parameter set per event in a
    /// multi-row `INSERT`, meeting `SQLITE_MAX_VARIABLE_NUMBER` or Postgres'
    /// 65,535-parameter cap. Like the variant above it is a capacity refusal with
    /// nowhere to go but `Store` until VT-25 lands.
    TooManyParameters,
    /// `max(position)` came back `NULL` and the column type cannot hold it.
    ///
    /// The third unproducible-by-a-correct-store variant, and it is why
    /// `Defect::select` in the testkit returns a `Result` at
    /// all. `NullHeadPagingStore` models `sqlx::query_scalar!` decoding
    /// `max(position)` into a non-nullable `i64` on an empty table. The faithful
    /// outcome there is a **decode error**, not a panic: the driver returns one
    /// and the adapter surfaces it as `Self::Error`. Modelling it as a panic
    /// would leave `reading_an_empty_store_yields_nothing` satisfied by the
    /// store falling over rather than by the rule's own assertion, which is the
    /// substitution CF-2 forbids.
    NullHead,
}

impl core::fmt::Display for LogError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyBorrowed => f.write_str("the event log was already borrowed"),
            Self::UniqueViolation => f.write_str("23505 unique_violation"),
            Self::UnknownPosition => {
                f.write_str("the condition's `after` names a position this store never assigned")
            }
            Self::WriteFailed => f.write_str("the write of one row of the batch failed"),
            Self::NotNullViolation => f.write_str("NOT NULL constraint failed: event.data"),
            Self::ValueTooLarge => f.write_str("value too large for column: event.data"),
            Self::TooManyParameters => f.write_str("too many SQL variables"),
            Self::NullHead => {
                f.write_str("max(position) returned NULL: the paging window has no upper bound")
            }
        }
    }
}

impl core::error::Error for LogError {}

// =====================================================================
// The read path, one step at a time
// =====================================================================

/// Step 1 — the events matching `query`, in insertion order.
///
/// Borrowed rather than cloned so that [`truncated`] runs over references: a
/// mutant that truncates in the wrong place should differ from the correct store
/// in *order of operations* only, and cloning early would hide the cost that
/// makes the wrong order tempting in the first place.
pub fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
    events
        .iter()
        .filter(|event| query.matches(event.event_type(), event.tags()))
        .collect()
}

/// Step 2 — apply the direction and the **inclusive** `from` and `to` bounds.
///
/// `from` is inclusive in both directions and bounds opposite ends: forwards it
/// is a floor, backwards a ceiling. Getting that backwards is the whole of
/// `read_backwards_from_with_limit`'s subject matter.
///
/// `to` is the *stopping* bound and sits on the other side of the position order
/// from `from` in whichever direction the read runs — so backwards it is a floor
/// where `from` is a ceiling. Both are inclusive in both directions (ES-16).
// The lifetime is elided rather than named: `matched` is the only input carrying
// one, so elision ties the output to it and `clippy::needless_lifetimes` denies
// spelling it out. [`matching`] above keeps its `'a` for the opposite reason —
// it takes two references and elision would tie the output to the wrong one.
pub fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
    // The log is appended to in position order, so `matching` returns an already
    // ascending run and reversing it is a complete descending sort. A store
    // whose backing order is not insertion order — every SQL adapter — has to
    // sort here instead, which is why this is its own step.
    if options.backwards {
        matched
            .into_iter()
            .rev()
            .filter(|event| options.from.is_none_or(|from| event.position <= from))
            .filter(|event| options.to.is_none_or(|to| event.position >= to))
            .collect()
    } else {
        matched
            .into_iter()
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .filter(|event| options.to.is_none_or(|to| event.position <= to))
            .collect()
    }
}

/// Step 3 — truncate to `limit`, **after** filtering and ordering.
///
/// The order matters and is the point of the split: `LIMIT` applied before the
/// tag filter is one of the four wrong implementations a reviewer measured
/// passing the suite, and it is a SQL bug with no in-memory analogue unless the
/// in-memory store is written in the same two steps.
pub fn truncated(mut selected: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
    if let Some(limit) = options.limit {
        selected.truncate(limit);
    }
    selected
}

/// The three read steps composed, cloned out of the log.
pub fn select(
    events: &[SequencedEvent],
    query: &Query,
    options: ReadOptions,
) -> Vec<SequencedEvent> {
    truncated(ordered(matching(events, query), options), options)
        .into_iter()
        .cloned()
        .collect()
}

// =====================================================================
// The append path, one step at a time
// =====================================================================

/// The position of the first event violating `condition`, if any.
///
/// Delegates the whole verdict to [`AppendCondition::is_violated_by`], which is
/// where `after` and the tag join both live. A mutant that wants to ignore
/// tags — the sqlite shape, where tags are a second table and the join is the
/// first thing dropped from the probe — reimplements *this* function and nothing
/// else.
pub fn violation(
    events: &[SequencedEvent],
    condition: &AppendCondition,
) -> Option<SequencePosition> {
    events
        .iter()
        .find(|existing| {
            condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
        })
        .map(|existing| existing.position)
}

/// Assigns positions to a batch, given the highest position already in the log.
///
/// Each event's position is allocated from the previous event's, so a batch of
/// three under a stepped allocator is three steps rather than one — which is
/// what makes a gapped store's `append` return value and its read-back agree.
pub fn sequence(
    events: &[Event],
    head: Option<SequencePosition>,
    allocate: Allocate,
) -> Vec<SequencedEvent> {
    let mut previous = head;
    let mut sequenced = Vec::with_capacity(events.len());
    for event in events {
        let position = allocate(previous);
        previous = Some(position);
        sequenced.push(stamp(position, event.clone()));
    }
    sequenced
}

/// Finds the first event that any guard rejects, given a per-guard predicate.
///
/// Every condition mutant in this binary models a defect in how **one** guard is
/// evaluated, and each one uses this so the defect is applied to *every* guard.
/// Reaching for `condition.guards()[0]` instead would be shorter and would give
/// each mutant a second, undeclared defect — ignoring guards 2..n — so
/// `mutants_fail_exactly_their_declared_rules` would start catching the
/// instrument rather than the implementation. That is the failure this whole
/// file exists to prevent, arriving through the newest field.
pub fn first_violation(
    events: &[SequencedEvent],
    condition: &AppendCondition,
    violated: impl Fn(&Guard, &SequencedEvent) -> bool,
) -> Option<SequencePosition> {
    events
        .iter()
        .find(|event| {
            condition
                .guards()
                .iter()
                .any(|guard| violated(guard, event))
        })
        .map(|event| event.position)
}

/// The incarnation every store in this binary mints identities under.
///
/// One constant rather than one per store, deliberately. Identity is not what
/// these mutants are instruments for, and giving each store its own would make
/// every mutant's output vary between runs for a reason unrelated to its defect
/// — which is exactly the confound `correct.rs` exists to remove. A mutant that
/// needs a *different* incarnation to express its defect overrides `stamp`, and
/// that override is then the only difference, which is the property the registry
/// checks.
pub const TEST_STORE: StoreId = StoreId::from_bytes([0xA1; 16]);

/// A fixed recorded time.
///
/// Fixed rather than read from a clock because `no_clock` (CF-33) forbids a rule
/// depending on wall time, and a store whose output moves between runs cannot be
/// compared against a snapshot. The value is arbitrary and nothing asserts it.
pub const TEST_RECORDED_AT: RecordedAt = RecordedAt::from_millis(1_700_000_000_000);

/// Pairs a position with the identity and time a correct store would assign.
pub fn stamp(position: SequencePosition, event: Event) -> SequencedEvent {
    SequencedEvent::new(
        position,
        EventId::new(TEST_STORE, position),
        TEST_RECORDED_AT,
        event,
    )
}

/// The condition probe, as a value.
///
/// A `fn` pointer for the same reason [`Allocate`] is one: it is `Copy`,
/// `UnwindSafe` and free to hold, so the harness never needs
/// `AssertUnwindSafe`.
pub type Probe = fn(&[SequencedEvent], &AppendCondition) -> Option<SequencePosition>;

/// Position assignment for a batch, as a value. See [`sequence`].
pub type Sequencer = fn(&[Event], Option<SequencePosition>, Allocate) -> Vec<SequencedEvent>;

/// The whole correct append, returning the position of the **last** event
/// written.
///
/// # Errors
///
/// As [`commit_with`].
pub fn commit(
    stored: &mut Vec<SequencedEvent>,
    events: &[Event],
    condition: Option<&AppendCondition>,
    allocate: Allocate,
) -> Result<SequencePosition, AppendError<LogError>> {
    commit_with(stored, events, condition, allocate, violation, sequence)
}

/// [`commit`] with its two *replaceable* steps supplied.
///
/// The parameters are what keep a mutant a scalpel. A store that evaluates its
/// append condition wrongly, or that binds one position for every row of a
/// multi-row insert, differs from the correct store in exactly one of these two
/// functions — so it passes its own step in here and inherits the ordering, the
/// emptiness rule and the return value unchanged. Reimplementing the whole body
/// to change one line is how a mutant acquires a second defect nobody declared.
///
/// # Errors
///
/// [`AppendError::NoEvents`] for an empty batch, and
/// [`AppendError::ConditionViolated`] when `probe` finds a conflict — in that
/// order, which is ES-20's and not a detail. Never [`AppendError::Store`] —
/// that variant belongs to the caller's borrow discipline, not to this function.
pub fn commit_with(
    stored: &mut Vec<SequencedEvent>,
    events: &[Event],
    condition: Option<&AppendCondition>,
    allocate: Allocate,
    probe: Probe,
    sequence: Sequencer,
) -> Result<SequencePosition, AppendError<LogError>> {
    // Emptiness before the condition (ES-20). This order was the other way round
    // until phase 3, faithfully copying `MemoryEventStore`'s own D8 defect:
    // `append(&[], Some(&c))` answered `NoEvents` or `ConditionViolated`
    // depending on what the store held. `ConditionBeforeEmptinessStore` in
    // `mutants.rs` is the old order, kept as the mutant that fails the rule.
    if events.is_empty() {
        return Err(AppendError::NoEvents);
    }

    if let Some(condition) = condition
        && let Some(conflict) = probe(stored, condition)
    {
        return Err(AppendError::ConditionViolated(ConditionViolated::at(
            conflict,
        )));
    }

    let head = stored.last().map(|event| event.position);
    let sequenced = sequence(events, head, allocate);
    // Cannot be empty: `events` was checked above and `sequence` is
    // length-preserving. The `ok_or` rather than an index is what keeps this
    // function out of `harness::RUNTIME_PANICS` territory — a correct core that
    // can panic on a slice index would make every mutant built from it capable
    // of failing a rule for the wrong reason.
    let last = sequenced
        .last()
        .map(|event| event.position)
        .ok_or(AppendError::NoEvents)?;
    stored.extend(sequenced);
    Ok(last)
}

// =====================================================================
// The log, and a completely correct store over it
// =====================================================================

/// An event log plus the policy it allocates positions under.
///
/// Holding the allocator *in the log* rather than in the store is what lets one
/// correct core serve both a dense store and a gapped one without either of them
/// reimplementing `commit`.
#[derive(Debug)]
pub struct Log {
    events: Vec<SequencedEvent>,
    allocate: Allocate,
}

impl Log {
    /// An empty log allocating positions under `allocate`.
    pub fn new(allocate: Allocate) -> Self {
        Self {
            events: Vec::new(),
            allocate,
        }
    }

    /// A log seeded with what some durable medium already held.
    ///
    /// The **replay** half of a reopen, and the seam a fixture whose defect is
    /// what a reopen reconstructs needs. `new` plus [`append`](Self::append)
    /// cannot express it: `append` re-allocates positions and re-spends
    /// [`stamp`], so replaying through it would hand every event a *fresh*
    /// position and the same constant time — the opposite of both halves of what
    /// a replay is supposed to do.
    ///
    /// Nothing here validates `events`. A caller replaying a medium that lost
    /// something, or that reconstructed a store-assigned fact wrongly, is the
    /// point rather than the risk: `LosingFixture` is the first shape and
    /// `RestampingFixture` is the second.
    pub fn replayed(allocate: Allocate, events: Vec<SequencedEvent>) -> Self {
        Self { events, allocate }
    }

    /// The correct read path over this log.
    pub fn select(&self, query: &Query, options: ReadOptions) -> Vec<SequencedEvent> {
        select(&self.events, query, options)
    }

    /// The correct append path over this log.
    ///
    /// # Errors
    ///
    /// As [`commit`].
    pub fn append(
        &mut self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        commit(&mut self.events, events, condition, self.allocate)
    }
}

/// The stream a store built on [`Log`] returns from `read`.
///
/// Owns its events, so it holds no borrow of the `RefCell` — which is the
/// discipline `BorrowHoldingStore` in `tests/local_conformance.rs` gets wrong on
/// purpose. Nothing here objects to `Send`; it simply is not required to be.
#[derive(Debug)]
pub struct Snapshot {
    /// Surfaced as the stream's first item, so a read failure arrives lazily
    /// like every other one rather than before the first poll.
    error: Option<LogError>,
    events: std::vec::IntoIter<SequencedEvent>,
}

impl Snapshot {
    /// A stream over `selected`, or over the failure that prevented reading it.
    pub fn new(selected: Result<Vec<SequencedEvent>, LogError>) -> Self {
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
    type Item = Result<SequencedEvent, LogError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Some(err) = this.error.take() {
            return Poll::Ready(Some(Err(err)));
        }
        Poll::Ready(this.events.next().map(Ok))
    }
}

/// A completely correct [`EventStore`] over a shared [`Log`].
///
/// This is not a registered subject — it is the baseline instruments reach for
/// when their editorial content is somewhere other than the store. The
/// declining fixture in `variants.rs` uses it, because what that fixture is
/// wrong about is its *capabilities*, and a store with any defect of its own
/// would muddy the reading.
///
/// `Rc` rather than `Arc` throughout this binary: the harness drives everything
/// through `happenstance_testkit::block_on` on one thread, and `EventStore` is
/// the flavour with no `Send` bound, so paying for atomics would buy nothing and
/// would quietly stop exercising the flavour ADR-0001 exists for.
#[derive(Debug, Clone)]
pub struct LogStore(Rc<RefCell<Log>>);

impl LogStore {
    /// A store over a fresh log allocating positions under `allocate`.
    pub fn new(allocate: Allocate) -> Self {
        Self(Rc::new(RefCell::new(Log::new(allocate))))
    }

    /// A store over a log somebody else owns.
    ///
    /// This is how `SharedBackingFixture` in `mutants.rs` reproduces the
    /// file-backed fixture that points every instance at one temporary path:
    /// the *store* stays completely correct, and the fixture hands two
    /// "isolated" instances the same log.
    pub fn over(log: &Rc<RefCell<Log>>) -> Self {
        Self(Rc::clone(log))
    }
}

impl EventStore for LogStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // Note the absence of `+ Send`, and the absence of `async`: `read`
        // returns the stream at the top level (ADR-0008), and the borrow is
        // taken and released before the stream exists.
        Snapshot::new(
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
        // No `.await` in this body, which is what makes the exclusive borrow
        // acquired and released inside a single `poll`.
        let mut log = self
            .0
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        log.append(events, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let log = self.0.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(head_of(&log.events))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let log = self.0.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(contains(&log.events, id))
    }
}

/// The highest position in a log.
///
/// A free function so that a mutant of `head` is one step from correct, in the
/// same way `select` and `commit` make a mutant of the read and write paths one
/// step from correct.
///
/// # Precondition, and the one store that breaks it
///
/// Returns the **last** element, which is the highest only where the slice is in
/// ascending position order. That holds for every store here whose rows are
/// appended in the order they are allocated — which is all of them but one.
///
/// `PreCommitPositionStore` publishes rows in *commit* order, and rows arriving
/// out of position order is its whole declared defect, so it computes a `max`
/// instead and says so at its own `head`. Anything else would give it a second
/// defect — `head` lagging a row a reader can already see — and the meta-test
/// that checks mutants fail exactly their declared rules would then be reporting
/// this file rather than the store.
pub fn head_of(events: &[SequencedEvent]) -> Option<SequencePosition> {
    events.last().map(|event| event.position)
}

/// Whether a log holds an event with this identity.
pub fn contains(events: &[SequencedEvent], id: EventId) -> bool {
    events.iter().any(|event| event.id == id)
}
