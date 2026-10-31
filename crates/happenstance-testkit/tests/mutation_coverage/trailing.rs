//! The trailing model's instruments: a store whose reads trail its commits,
//! three conformant ways to build one, four wrong ones and a stalled one, and
//! a wrapper that sends any registered store down the trailing path.
//!
//! [`TrailingFrontierStore`] over [`Settling`] is the conformant control. It
//! stands where `happenstance-postgres` and `happenstance-neon` stand behind
//! `pg_snapshot_xmin`: every committed row is held at once, a read shows a
//! prefix that may stop short of it, and the condition is judged over
//! everything held. [`GappedSettling`] and [`Draining`] are the same control
//! at the far end of the allocation axis and of the read-bracket axis. Each
//! wrong store differs from it in one step of [`Trailing`], after the
//! `FrontierStore`/`Membership` pattern in `variants.rs`.
//!
//! # Why these are in a file of their own, and not in `REGISTRY`
//!
//! A store that trails its own writes fails the named rules that read back
//! an append, and that is not the defect any of these is about. So they are
//! not in `REGISTRY` or `for_each_mutant!`, where those rules would be held
//! against them; `for_each_trailing_subject!` and `TRAILING_MODEL_COVERAGE`
//! are their list and their table. The whole file is behind the `proptest`
//! feature because the model family is, and a store nothing drives is dead
//! code under `-D warnings` in every other configuration.

use core::cell::RefCell;
use core::marker::PhantomData;
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, SequencePosition, SequencedEvent,
};
use happenstance_testkit::{Capability, Fixture};

use crate::correct::{self, Allocate, LogError, Snapshot, dense};
use crate::harness::Subject;

// =====================================================================
// The log and its schedule
// =====================================================================

/// A log whose reads trail its commits on a schedule.
///
/// The lag is a fixed function of the read calls, not of a clock, so a run is
/// reproducible and the schedule is exactly what a counterexample replays.
#[derive(Debug, Default)]
pub(crate) struct TrailingLog {
    events: Vec<SequencedEvent>,
    /// The row count at the end of each batch, in commit order.
    batches: Vec<usize>,
    /// The last position allocated, whether written or burned.
    ///
    /// Kept apart from the last row's position because a burned position
    /// leaves no row, and the next batch must still start above it.
    allocated: Option<SequencePosition>,
    /// How many rows were held at each read call, in call order.
    reads: Vec<usize>,
}

impl TrailingLog {
    /// Writes one batch above everything allocated so far, and returns the
    /// position of its last event.
    ///
    /// # Errors
    ///
    /// [`AppendError::NoEvents`] for an empty batch, which the caller has
    /// already refused; the `ok_or` keeps an index out of the instrument.
    fn write(
        &mut self,
        events: &[Event],
        allocate: Allocate,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let sequenced = correct::sequence(events, self.allocated, allocate);
        let last = sequenced
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        self.events.extend(sequenced);
        self.batches.push(self.events.len());
        self.allocated = Some(last);
        Ok(last)
    }

    /// Spends one position without writing a row, as `nextval()` does for a
    /// transaction that then rolls back.
    fn burn(&mut self, allocate: Allocate) {
        self.allocated = Some(allocate(self.allocated));
    }

    /// Every row the store holds, visible or not.
    pub(crate) fn held(&self) -> &[SequencedEvent] {
        &self.events
    }

    /// The rows held when read call `call` was made.
    pub(crate) fn as_of(&self, call: usize) -> &[SequencedEvent] {
        let held = self.reads.get(call).copied().unwrap_or(0);
        self.events.get(..held).unwrap_or(&self.events)
    }

    /// What read call `call` sees under the schedule: the rows held at the
    /// most recent call whose number is a multiple of three.
    ///
    /// Calls 0, 1 and 2 see what was held at call 0; calls 3, 4 and 5 what was
    /// held at call 3; and so on. It never moves backward.
    pub(crate) fn scheduled(&self, call: usize) -> &[SequencedEvent] {
        self.as_of(call.saturating_sub(call % 3))
    }

    /// What read call `call` sees when the frontier stands still while writes
    /// arrive and drains one batch per read once they stop.
    ///
    /// A read made after a write since the read before it reveals nothing
    /// new; any other read reveals one more batch, up to what is held. So lag
    /// builds across a run of appends, and a `Read` op's own read lands one
    /// batch above the full read before it and one below the full read after
    /// it. That is the case the model's read bracket exists for, and the
    /// three-call cycle of [`Self::scheduled`] never puts a read strictly
    /// inside it.
    pub(crate) fn revealed(&self, call: usize) -> &[SequencedEvent] {
        let mut visible = 0_usize;
        let mut previous = 0_usize;
        for held in self.reads.iter().take(call.saturating_add(1)) {
            if *held == previous {
                let batches = self.batches.iter().take_while(|end| *end <= held).count();
                visible = batches.min(visible.saturating_add(1));
            }
            previous = *held;
        }
        let rows = visible
            .checked_sub(1)
            .and_then(|last| self.batches.get(last))
            .copied()
            .unwrap_or(0);
        self.events.get(..rows).unwrap_or(&self.events)
    }

    /// What the most recent read call saw under the schedule.
    pub(crate) fn last_scheduled(&self) -> &[SequencedEvent] {
        self.scheduled(self.reads.len().saturating_sub(1))
    }

    /// Records a read call and returns its number.
    fn record(&mut self) -> usize {
        let call = self.reads.len();
        self.reads.push(self.events.len());
        call
    }
}

/// The two steps a [`TrailingFrontierStore`] may get wrong, what a read shows
/// and what the condition is judged over, and the allocation policy it may
/// legally choose.
pub(crate) trait Trailing {
    /// The coverage key for the store this behaviour is built into.
    const NAME: &'static str;

    /// How positions are allocated. Dense from 1 unless a behaviour says
    /// otherwise.
    const ALLOCATE: Allocate = dense;

    /// Whether a refused conditional append spends a position, as a
    /// `nextval()` taken inside a transaction that rolls back does.
    const BURNS_ON_REFUSAL: bool = false;

    /// What read call `call` returns, before the query and options apply.
    ///
    /// Owned, because the store hands the rows to a read stream; the copy is
    /// the read result itself.
    fn shown(log: &TrailingLog, call: usize) -> Vec<SequencedEvent> {
        log.scheduled(call).to_vec()
    }

    /// The first event violating `condition`, if any. Over every held row,
    /// which is ES-25's reading: a condition is judged over what is held.
    fn conflict(log: &TrailingLog, condition: &AppendCondition) -> Option<SequencePosition> {
        correct::violation(log.held(), condition)
    }
}

// =====================================================================
// The store, its fixture, and the conformant control
// =====================================================================

/// A store whose reads trail its commits, generic over the two steps.
#[derive(Debug)]
pub(crate) struct TrailingFrontierStore<T> {
    log: Rc<RefCell<TrailingLog>>,
    trailing: PhantomData<T>,
}

impl<T: Trailing> TrailingFrontierStore<T> {
    /// Records a read call and returns what it shows.
    fn show(&self) -> Result<Vec<SequencedEvent>, LogError> {
        let mut log = self
            .log
            .try_borrow_mut()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        let call = log.record();
        Ok(T::shown(&log, call))
    }
}

impl<T: Trailing> EventStore for TrailingFrontierStore<T> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.show()
                .map(|shown| correct::select(&shown, query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let mut log = self
            .log
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        // Emptiness before the condition (ES-20), in `correct::commit_with`'s
        // order, and the condition through the replaceable step.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        if let Some(condition) = condition
            && let Some(conflict) = T::conflict(&log, condition)
        {
            if T::BURNS_ON_REFUSAL {
                log.burn(T::ALLOCATE);
            }
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }
        log.write(events, T::ALLOCATE)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(correct::head_of(&self.show()?))
    }

    // Held, ADR-0095's reading, whatever a read shows.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let log = self
            .log
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::contains(log.held(), id))
    }
}

/// One trailing log, and any number of handles onto it.
#[derive(Debug)]
pub(crate) struct TrailingFrontierFixture<T> {
    log: Rc<RefCell<TrailingLog>>,
    trailing: PhantomData<T>,
}

impl<T: Trailing> Fixture for TrailingFrontierFixture<T> {
    type Store = TrailingFrontierStore<T>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(
        "this instrument exists for the trailing-read axis; its log is a Vec behind an \
         Rc with no durable medium",
    );
    const READ_YOUR_OWN_WRITES: Capability = Capability::declined(
        "this instrument's reads trail its commits on a fixed schedule of reads, which \
         is the trailing model's whole subject",
    );
    const HELD_ABOVE_FRONTIER: Capability = Capability::declined(
        "this instrument's lag follows a fixed schedule of reads and cannot be held on \
         demand",
    );

    async fn connect(&self) -> Self::Store {
        TrailingFrontierStore {
            log: Rc::clone(&self.log),
            trailing: PhantomData,
        }
    }
}

impl<T: Trailing> Subject for TrailingFrontierFixture<T> {
    const NAME: &'static str = T::NAME;

    fn open() -> Self {
        Self {
            log: Rc::new(RefCell::new(TrailingLog::default())),
            trailing: PhantomData,
        }
    }
}

/// The conformant behaviour: a monotone, lagging prefix, and a condition
/// judged over every held row.
#[derive(Debug)]
pub(crate) struct Settling;

impl Trailing for Settling {
    const NAME: &'static str = "TrailingFrontierStore";
}

/// The conformant behaviour over the positions a Postgres sequence hands out:
/// from 4096 in steps of seven, with a position burned by every refused
/// conditional append.
///
/// The control at the other end of the allocation axis from [`Settling`]. The
/// trailing model places an invisible event between positions it has seen,
/// and a model that quietly assumed `+1` would false-fail exactly this store.
#[derive(Debug)]
pub(crate) struct GappedSettling;

impl Trailing for GappedSettling {
    const NAME: &'static str = "GappedTrailingFrontierStore";
    const ALLOCATE: Allocate = super::gapped;
    const BURNS_ON_REFUSAL: bool = true;
}

/// The conformant behaviour with a frontier that drains one batch per read.
///
/// The control for the model's read bracket: under this schedule a `Read`
/// op's own read can land strictly between the full reads either side of it,
/// which [`TrailingLog::scheduled`]'s three-call cycle never produces.
#[derive(Debug)]
pub(crate) struct Draining;

impl Trailing for Draining {
    const NAME: &'static str = "DrainingFrontierStore";

    fn shown(log: &TrailingLog, call: usize) -> Vec<SequencedEvent> {
        log.revealed(call).to_vec()
    }
}

// =====================================================================
// The wrong stores, one step each
// =====================================================================

/// The condition is judged over the rows the last read could see.
///
/// Provenance: the condition SQL reuses the read path's
/// `xact_id < pg_snapshot_xmin(...)` predicate, which is the defect
/// `happenstance-postgres`'s `evaluate` warns of with "No frontier predicate
/// here". Such a store admits an append that a held, invisible row forbids.
#[derive(Debug)]
pub(crate) struct FrontierFilteredCondition;

impl Trailing for FrontierFilteredCondition {
    const NAME: &'static str = "FrontierFilteredConditionStore";

    fn conflict(log: &TrailingLog, condition: &AppendCondition) -> Option<SequencePosition> {
        // THE DEFECT: the frontier predicate is in the condition.
        correct::violation(log.last_scheduled(), condition)
    }
}

/// The oldest row above the frontier is hidden while newer ones show.
///
/// Provenance: `happenstance-postgres`'s naive arm (`Visibility::Naive`).
/// `nextval()` allocates outside the transaction, a later writer commits
/// first, and a read shows its row above a hole the earlier one will fill.
#[derive(Debug)]
pub(crate) struct HoleyFrontier;

impl Trailing for HoleyFrontier {
    const NAME: &'static str = "HoleyFrontierStore";

    fn shown(log: &TrailingLog, call: usize) -> Vec<SequencedEvent> {
        // THE DEFECT: every held row but the first one above the frontier.
        // Owned for `Trailing::shown`'s reason.
        let hole = log.scheduled(call).len();
        log.held()
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != hole)
            .map(|(_, event)| event.clone())
            .collect()
    }
}

/// The frontier moves backward on every other read.
///
/// Provenance: a frontier read off a replica, or through a pooler that hops
/// between servers, so consecutive reads come from snapshots that do not agree
/// on what has committed.
#[derive(Debug)]
pub(crate) struct RetreatingFrontier;

impl Trailing for RetreatingFrontier {
    const NAME: &'static str = "RetreatingFrontierStore";

    fn shown(log: &TrailingLog, call: usize) -> Vec<SequencedEvent> {
        // THE DEFECT: an odd call sees what was held two calls before the even
        // call ahead of it did.
        log.as_of(call.saturating_sub(call % 2 * 2)).to_vec()
    }
}

/// The condition's `after` is inclusive, behind a frontier.
///
/// Provenance: `AfterIsInclusiveStore`, on a store whose reads trail its
/// commits. `after` is exclusive and `from` is inclusive, and one comparison
/// served both.
#[derive(Debug)]
pub(crate) struct TrailingAfterIsInclusive;

impl Trailing for TrailingAfterIsInclusive {
    const NAME: &'static str = "TrailingAfterIsInclusiveStore";

    fn conflict(log: &TrailingLog, condition: &AppendCondition) -> Option<SequencePosition> {
        // THE DEFECT: `>=` where the clause says `>`.
        correct::first_violation(log.held(), condition, |guard, event| {
            guard.after.is_none_or(|after| event.position >= after)
                && guard.query.matches(event.event_type(), event.tags())
        })
    }
}

/// The frontier never moves past what the first read saw.
///
/// Provenance: a transaction left open somewhere on the cluster holds
/// `pg_snapshot_xmin` back for good, which ADR-0024 and ADR-0071 name as the
/// frontier's cluster-wide coupling. This one is a **measured miss**, not a
/// wrong store the model catches: every read is a legal prefix, every verdict
/// is judged over what is held, and only waiting could tell a frontier that
/// never moves from one that has not moved yet. CF-33 forbids the wait, so
/// liveness is the trailing model's stated non-goal, and this row is what
/// keeps that sentence true.
#[derive(Debug)]
pub(crate) struct StalledFrontier;

impl Trailing for StalledFrontier {
    const NAME: &'static str = "StalledFrontierStore";

    fn shown(log: &TrailingLog, _call: usize) -> Vec<SequencedEvent> {
        // THE DEFECT: the first read's frontier, forever.
        log.as_of(0).to_vec()
    }
}

// =====================================================================
// The weakening, measured
// =====================================================================

/// Any registered fixture, saying its reads may trail its commits.
///
/// Wrapped around a store that in fact reads its own writes, the rule runs the
/// trailing model at zero lag. Whatever the trailing model then misses that
/// the strict model catches is the price of the weakening, and
/// `WEAKENING_COSTS` pins it.
///
/// `REOPEN` is declined rather than delegated: the model family never reopens,
/// and delegating the claim without the `reopen` body behind it would be a
/// claim this wrapper cannot keep.
#[derive(Debug)]
pub(crate) struct Declined<S>(S);

impl<S: Fixture> Fixture for Declined<S> {
    type Store = S::Store;

    const SECOND_HANDLE: Capability = S::SECOND_HANDLE;
    const REOPEN: Capability =
        Capability::declined("this wrapper runs only the model family, which never reopens");
    const READ_YOUR_OWN_WRITES: Capability = Capability::declined(
        "this wrapper says the wrapped store's reads may trail its commits, so that the \
         trailing model runs against it",
    );
    const HELD_ABOVE_FRONTIER: Capability =
        Capability::declined("this wrapper adds no frontier, so it has none to hold a row above");

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

impl<S: Subject> Subject for Declined<S> {
    const NAME: &'static str = S::NAME;

    fn open() -> Self {
        Self(S::open())
    }
}

// =====================================================================
// The schedule and the allocator, pinned
// =====================================================================

/// The two properties the newer controls exist to supply, held directly so
/// that neither can quietly stop being true while its control still agrees.
mod pinned {
    use happenstance_core::Event;

    use super::TrailingLog;
    use crate::correct::dense;

    fn batch() -> [Event; 1] {
        [Event::new("A", &b"x"[..]).expect("a literal event type is valid")]
    }

    #[test]
    fn a_draining_frontier_puts_a_read_strictly_inside_its_bracket() {
        let mut log = TrailingLog::default();
        for expected in 1..=3 {
            let written = log.write(&batch(), dense);
            assert!(matches!(written, Ok(position) if position.get() == expected));
        }

        let shown: Vec<usize> = (0..4)
            .map(|_| {
                let call = log.record();
                log.revealed(call).len()
            })
            .collect();

        // The first read follows writes and stands still. Then the full read
        // before a `Read` op, the op's own read, and the full read after.
        assert_eq!(shown, [0, 1, 2, 3]);
    }

    #[test]
    fn a_refused_append_burns_a_gapped_position() {
        let mut log = TrailingLog::default();

        let first = log.write(&batch(), super::super::gapped);
        log.burn(super::super::gapped);
        let second = log.write(&batch(), super::super::gapped);

        assert!(matches!(first, Ok(position) if position.get() == 4096));
        assert!(
            matches!(second, Ok(position) if position.get() == 4110),
            "4103 was burned, so the next batch starts a step above it: {second:?}"
        );
    }
}
