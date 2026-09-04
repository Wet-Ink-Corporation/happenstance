//! The wrong stores.
//!
//! Every store here is a defect somebody would ship. That is CF-4's bar and it
//! is the whole difference between a mutant and a saboteur: `struct AlwaysWrong`
//! satisfies `every_rule_has_a_mutant` mechanically and proves nothing, because
//! no author would have written it. Each entry below names the adapter shape it
//! comes from in its `REGISTRY` row, and most of them come from SQL — a two-table
//! tag schema, a clause vector joined with the wrong separator, a `LIMIT` that
//! fetches one extra row to answer "is there more".
//!
//! # One defect per store, and how that is enforced mechanically
//!
//! [`Defect`] is a trait of *steps*, every one of which defaults to
//! `crate::correct`'s version. A mutant overrides exactly one and inherits the
//! rest, so a rule that goes red went red for the declared reason and not for a
//! second bug the author did not notice writing. Reimplementing `EventStore` per
//! mutant is how a mutant set decays into a pile of failures, which is what
//! `mutants_fail_exactly_their_declared_rules` exists to catch — and it would
//! then be catching it in the *instrument*.
//!
//! [`Defect::select`] is the seventh step, [`Defect::head_of`] the eighth and
//! [`Defect::contains`] the ninth. Read `select`'s documentation before adding a
//! tenth: it exists because two real defects couple a *filter* to a *read
//! option*, and `matching` is handed no options while `ordered` is handed no
//! query — its seam is the expensive one. The other two are the cheap kind, a
//! port operation with a body of its own that no other step can reach, and both
//! became steps only once a rule read the answer.
//!
//! Nine stores cannot be expressed that way and are written out longhand, each
//! for a stated reason: [`CachedHeadFixture`] and [`LastWrittenHeadFixture`]
//! because each carries its defect per *handle* rather than per store — the
//! first in what its condition probe can see, the second in what its `head`
//! reports — [`LosingFixture`] and [`RestampingFixture`] because each defect is
//! what a reopen finds, and reopen is a fixture operation with no `Defect` step
//! to override, [`SharedBackingFixture`] because its defect is that two fixture
//! instances are one, [`PreCommitPositionStore`] and [`AwaitAcrossBorrowStore`]
//! because each defect is a *window* — a suspension between two halves of an
//! append — and [`Defect::commit`] is a synchronous function with nowhere to put
//! one, [`BorrowHoldingStore`] because its defect is the *lifetime* of the
//! value `read` returns rather than anything a step computes, and
//! [`RefetchingPagedStore`] because its defect is the *poll schedule* of that
//! same value — the sign of `BorrowHoldingStore`'s reversed, a stream that holds
//! too little rather than too much.
//!
//! # Fixture-level mutants share the registry with store-level ones
//!
//! Three of the entries below carry their defect in the fixture — how handles are
//! handed out, what survives a reopen, whether two instances are isolated —
//! rather than in the `EventStore` impl. They are registered in the same table,
//! with one `Kind` and one `Declared` shape, and that is a decision rather than
//! an oversight. A `Subject` is already a *pair*: `harness::Subject` is
//! implemented on the fixture while its `NAME` names the store, precisely because
//! the registry is a map from a defect to the rules that catch it and a defect
//! can live in either half. Splitting the table in two would give the meta-tests
//! two things to assert the same property over, and a reviewer two places to
//! look for "which rules does this defect break". The `NAME` of a fixture-level
//! entry is the fixture's, so the table still reads as "this thing, these rules".
//!
//! # How to add one
//!
//! 1. Write the defect in this file as an `impl Defect` overriding **one** step,
//!    or — if it genuinely cannot be one step — as a store and fixture of its
//!    own, with a comment saying why.
//! 2. Add its fixture type to `for_each_mutant!` in `tests/mutation_coverage.rs`.
//! 3. Add its `Declared` row to `REGISTRY` in the same file: the exact set of
//!    rules it fails, its provenance, its `FailureMode`, and — wherever the rule
//!    carries more than one assertion — an `expect` pin per rule naming the one
//!    this defect is supposed to trip.
//!
//! Steps 2 and 3 are separate on purpose. The type enumeration and the claim
//! about it are the two lists that can drift, and
//! `mutant_registry_is_exhaustive` exists to hold them together.

use core::cell::{Cell, Ref, RefCell};
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::{Rc, Weak};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, EventType, Query,
    QueryItem, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId, Tag, Tags,
};
use happenstance_testkit::fixtures::{item_tagged, query_of_items, tagged_event};
use happenstance_testkit::{Capability, Fixture};

use crate::correct::{self, Allocate, Log, LogError, LogStore, Snapshot, dense};
use crate::harness::Subject;
use crate::variants::Fault;

// =====================================================================
// The shape a one-step defect takes
// =====================================================================

/// One step of the correct store, replaced.
///
/// Every method has a body, and every body is `crate::correct`'s. An
/// implementation that overrides nothing is the reference store; an
/// implementation that overrides one method is a mutant whose failure is
/// attributable to that method.
///
/// # Why associated functions rather than methods on a value
///
/// The store is `MutantStore<D>`, and `D` is never constructed — it is a type
/// carried in a `PhantomData` purely so that monomorphisation can pick the right
/// step. A trait of `&self` methods would need a value of `D` in every store, and
/// a `Box<dyn Defect>` inside the store would stop the store being
/// [`UnwindSafe`](std::panic::UnwindSafe), which `harness.rs` is built to avoid
/// needing to assert away.
pub(crate) trait Defect: 'static {
    /// The registry key, which names the *store*.
    const NAME: &'static str;

    /// How positions are handed out. Dense from 1, like `MemoryEventStore`.
    const ALLOCATE: Allocate = dense;

    /// CF-40's ceilings, defaulted to "this store has none".
    ///
    /// Four defects override one of these, and every other mutant inherits `None`
    /// and reports a skip for `append_reports_exceeded_store_limits`. They are on
    /// `Defect` rather than on [`MutantFixture`] because the ceiling belongs to
    /// the store — `PayloadCeilingStore::CEILING` is the number, and stating it
    /// twice is the drift `correct.rs` exists to avoid.
    const MAX_EVENT_DATA_LEN: Option<usize> = None;
    /// See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_TAGS_PER_EVENT: Option<usize> = None;
    /// See [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    const MAX_EVENTS_PER_BATCH: Option<usize> = None;

    /// Step 1 — the events matching `query`.
    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        correct::matching(events, query)
    }

    /// Step 2 — direction and the inclusive `from` bound.
    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        correct::ordered(matched, options)
    }

    /// Step 3 — truncation to `limit`.
    fn truncated(selected: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        correct::truncated(selected, options)
    }

    /// The three read steps composed — the only place a defect can see the
    /// query and the read options **at the same time**.
    ///
    /// It is a seventh primitive rather than a fourth read step, and the
    /// distinction is what keeps "one defect per store" true. Overriding it is
    /// still overriding exactly one method; what it buys is the two defects the
    /// three-step split cannot express, because each of them couples a *filter*
    /// to an *option*:
    ///
    /// * `WHERE a OR b AND position >= ?` without parentheses — the cursor binds
    ///   to one item of the query, so the predicate and the `from` bound have to
    ///   be built together (`UnparenthesisedPredicateStore`);
    /// * `SELECT … LIMIT n` pushed into the scan with the query's predicate
    ///   applied to the rows that come back (`LimitBeforeFilterStore`).
    ///
    /// `matching` is handed no options and `ordered` is handed no query, which is
    /// exactly right for the five one-step defects above and leaves nowhere for
    /// these two to live. A store that overrides `select` inherits none of the
    /// three steps unless it calls them, which is the cost of the seam and the
    /// reason it is not the default place to put a defect.
    ///
    /// # Why it returns a `Result` when the three steps do not
    ///
    /// Because a read can fail *before* it yields anything, and a mutant that
    /// models that failure as a panic is caught by the wrong thing.
    /// `NullHeadPagingStore` decodes `max(position)` into a column type that
    /// cannot hold `NULL`; the driver's answer is a decode error, and surfacing
    /// it here lets `read_ok` in `suite.rs` raise the rejection so the registry
    /// can hold the row to [`FailureMode::Assertion`]'s origin check. The error
    /// arrives as the stream's first item rather than up front, which is what
    /// the port specifies: `read` is not `async` and failures are `Err` items.
    ///
    /// # Errors
    ///
    /// Whatever the overriding defect models a failed read as. The provided body
    /// is infallible.
    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        Ok(Self::truncated(
            Self::ordered(Self::matching(events, query), options),
            options,
        )
        .into_iter()
        .cloned()
        .collect())
    }

    /// The append condition probe.
    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        correct::violation(events, condition)
    }

    /// Position assignment for a batch.
    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        correct::sequence(events, head, allocate)
    }

    /// The store's head: the highest position currently visible.
    ///
    /// The eighth step. Until phase 4 `head` was deliberately *not* one, and the
    /// reason it becomes one is that ES-30 acquired three rules: a defect in
    /// `head` is now a defect a rule can see, so it is a defect a mutant has to
    /// be able to express. Anything else would mean two more longhand stores for
    /// a difference of one function.
    ///
    /// `correct::head_of` was already a free function against exactly this
    /// possibility — its own doc comment says so — so a mutant of `head` is one
    /// step from correct in the same way `select` and `commit` make a mutant of
    /// the read and write paths one step from correct.
    fn head_of(events: &[SequencedEvent]) -> Option<SequencePosition> {
        correct::head_of(events)
    }

    /// The membership answer behind `contains_event_id`.
    ///
    /// The ninth step, and the only one on neither the read nor the write path:
    /// `contains_event_id` is a port operation of its own, so a store that
    /// answers it wrongly is wrong nowhere else and there is nothing else to
    /// override. It became a step for `head_of`'s reason, one clause over —
    /// ES-41 acquired `contains_event_id_reports_membership`, so a defect here is
    /// one a rule can see.
    fn contains(events: &[SequencedEvent], id: EventId) -> bool {
        correct::contains(events, id)
    }

    /// The write itself: probe, emptiness, allocate, extend.
    ///
    /// # Errors
    ///
    /// As [`correct::commit_with`].
    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// A store whose every step is `D`'s.
///
/// `PhantomData<D>` rather than a field: `D` is a marker, never a value.
pub(crate) struct MutantStore<D: Defect> {
    events: Rc<RefCell<Vec<SequencedEvent>>>,
    defect: PhantomData<D>,
}

impl<D: Defect> MutantStore<D> {
    /// A store over a fresh, empty log.
    fn new() -> Self {
        Self {
            events: Rc::new(RefCell::new(Vec::new())),
            defect: PhantomData,
        }
    }
}

// Written out rather than derived: `#[derive]` is syntactic and would add
// `D: Clone` / `D: Debug` bounds to the impls, obliging every marker type below
// to derive traits it has no values to exercise them on.
impl<D: Defect> Clone for MutantStore<D> {
    fn clone(&self) -> Self {
        Self {
            events: Rc::clone(&self.events),
            defect: PhantomData,
        }
    }
}

impl<D: Defect> core::fmt::Debug for MutantStore<D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MutantStore").field("of", &D::NAME).finish()
    }
}

impl<D: Defect> EventStore for MutantStore<D> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.events
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .and_then(|stored| D::select(&stored, query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let mut stored = self
            .events
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        D::commit(&mut stored, events, condition)
    }

    // Both of these were `crate::correct`'s for every `D` until phase 4, and the
    // comment here said so: no rule of the event-store family read either
    // answer, so a step would have been somewhere to put a defect nothing could
    // catch — a second, undeclared defect in every store that inherited it, and
    // `mutants_fail_exactly_their_declared_rules` would then be catching the
    // instrument rather than the implementation. Slice F gave `head` three rules
    // (ES-30) and `contains_event_id` one (ES-41), so both are now steps: a
    // defect in either is one a rule can see, and therefore one a mutant must be
    // able to declare.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let stored = self
            .events
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(D::head_of(&stored))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let stored = self
            .events
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(D::contains(&stored, id))
    }
}

/// One backing log for a [`MutantStore`], and any number of handles onto it.
#[derive(Debug)]
pub(crate) struct MutantFixture<D: Defect>(MutantStore<D>);

impl<D: Defect> Fixture for MutantFixture<D> {
    type Store = MutantStore<D>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The same honest answer `MemoryFixture` gives: a `Vec` behind an `Rc` has
    // no durable medium, so "reopen" could only mean doing nothing — which
    // passes `acknowledged_writes_survive_a_reopen` vacuously — or dropping the
    // log, which fails it for a reason none of these mutants is about.
    // `LosingFixture` below is the one entry whose defect *is* durability, and it
    // is the only one that declares this supported.
    const REOPEN: Capability = Capability::declined(
        "a mutant of the read or write path is a Vec behind an Rc, with no \
         durable medium to reopen over",
    );

    // Forwarded from the defect, because the ceiling is the store's fact and
    // `MutantFixture` is one type over forty of them.
    const MAX_EVENT_DATA_LEN: Option<usize> = D::MAX_EVENT_DATA_LEN;
    const MAX_TAGS_PER_EVENT: Option<usize> = D::MAX_TAGS_PER_EVENT;
    const MAX_EVENTS_PER_BATCH: Option<usize> = D::MAX_EVENTS_PER_BATCH;

    async fn connect(&self) -> Self::Store {
        self.0.clone()
    }
}

impl<D: Defect> Subject for MutantFixture<D> {
    const NAME: &'static str = D::NAME;

    fn open() -> Self {
        Self(MutantStore::new())
    }
}

// =====================================================================
// Shared helpers for the query-matching family
// =====================================================================

/// [`correct::matching`] with the *item* predicate replaced.
///
/// Keeps `Query::All` matching everything and keeps items joined with OR, so a
/// mutant of one item's internal logic cannot accidentally also be a mutant of
/// the item join. `ItemsAreAndStore` is the one that changes the join, and it
/// does not go through here.
fn matching_items<'a>(
    events: &'a [SequencedEvent],
    query: &Query,
    item_matches: fn(&QueryItem, &EventType, &Tags) -> bool,
) -> Vec<&'a SequencedEvent> {
    events
        .iter()
        .filter(|event| match query.items() {
            None => true,
            Some(items) => items
                .iter()
                .any(|item| item_matches(item, event.event_type(), event.tags())),
        })
        .collect()
}

/// The correct type half of an item: no types means any type.
fn type_matches(item: &QueryItem, event_type: &EventType) -> bool {
    item.types().is_empty() || item.types().contains(event_type)
}

// =====================================================================
// Query semantics — the read filter
// =====================================================================

/// Tags live in a side table, and the join that reaches them is an `INNER JOIN`.
///
/// An event carrying no tags therefore has no row on the other side and is
/// invisible to every query, `Query::all()` included — the join is in the `FROM`
/// clause, not the `WHERE`.
pub(crate) struct InnerJoinTagStore;

impl Defect for InnerJoinTagStore {
    const NAME: &'static str = "InnerJoinTagStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        correct::matching(events, query)
            .into_iter()
            .filter(|event| !event.tags().is_empty())
            .collect()
    }
}

/// Within one item, the type list is joined with `AND`.
pub(crate) struct TypesAreAndStore;

impl Defect for TypesAreAndStore {
    const NAME: &'static str = "TypesAreAndStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        matching_items(events, query, |item, event_type, tags| {
            let type_ok = item.types().is_empty() || item.types().iter().all(|t| t == event_type);
            type_ok && tags.contains_all(item.tags())
        })
    }
}

/// Within one item, the tag set is joined with `OR`.
pub(crate) struct TagsAreOrStore;

impl Defect for TagsAreOrStore {
    const NAME: &'static str = "TagsAreOrStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        matching_items(events, query, |item, event_type, tags| {
            let tags_ok = item.tags().is_empty() || item.tags().iter().any(|t| tags.contains(t));
            type_matches(item, event_type) && tags_ok
        })
    }
}

/// An item's tags must **equal** the event's rather than be a subset of them.
pub(crate) struct ExactTagMatchReadStore;

impl Defect for ExactTagMatchReadStore {
    const NAME: &'static str = "ExactTagMatchReadStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        matching_items(events, query, |item, event_type, tags| {
            // An item with no tags emits no tag clause at all, which is why the
            // empty case is still correct — and why this mutant survives every
            // type-only rule in the suite.
            let tags_ok = item.tags().is_empty() || tags.as_slice() == item.tags().as_slice();
            type_matches(item, event_type) && tags_ok
        })
    }
}

/// Within one item, the type clause and the tag clause are joined with OR.
pub(crate) struct ClauseJoinerStore;

impl Defect for ClauseJoinerStore {
    const NAME: &'static str = "ClauseJoinerStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        matching_items(events, query, |item, event_type, tags| {
            let type_ok = type_matches(item, event_type);
            let tags_ok = tags.contains_all(item.tags());
            if item.types().is_empty() || item.tags().is_empty() {
                // One clause in the vector, so the joiner never runs. This is
                // why the defect is invisible to every rule whose items carry a
                // single constraint.
                type_ok && tags_ok
            } else {
                type_ok || tags_ok
            }
        })
    }
}

/// Items are joined with `AND` across a query.
pub(crate) struct ItemsAreAndStore;

impl Defect for ItemsAreAndStore {
    const NAME: &'static str = "ItemsAreAndStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        events
            .iter()
            .filter(|event| match query.items() {
                None => true,
                Some(items) => items
                    .iter()
                    .all(|item| item.matches(event.event_type(), event.tags())),
            })
            .collect()
    }
}

/// Types are interned, and a type nobody has written yet has no id.
///
/// The generated clause is `type_id IN (<ids>)`; with no ids the parenthesised
/// list is empty, which is a syntax error, so the clause is dropped and the query
/// degenerates to its tag half. The same query builder serves `read` and the
/// append-condition probe — which is the norm, and the reason this one defect
/// shows up on both sides.
pub(crate) struct UninternedTypeStore;

impl UninternedTypeStore {
    /// Whether `item` matches, with the type clause dropped when none of its
    /// types has ever been written.
    fn item_matches(
        item: &QueryItem,
        known: &[EventType],
        event_type: &EventType,
        tags: &Tags,
    ) -> bool {
        let ids: Vec<&EventType> = item
            .types()
            .iter()
            .filter(|candidate| known.contains(candidate))
            .collect();
        let type_ok = item.types().is_empty() || ids.is_empty() || ids.contains(&event_type);
        type_ok && tags.contains_all(item.tags())
    }

    /// Whether `query` matches, under the interning defect.
    fn query_matches(query: &Query, known: &[EventType], event: &SequencedEvent) -> bool {
        match query.items() {
            None => true,
            Some(items) => items
                .iter()
                .any(|item| Self::item_matches(item, known, event.event_type(), event.tags())),
        }
    }

    /// Every type the intern table has an id for: the ones that have been
    /// written.
    fn interned(events: &[SequencedEvent]) -> Vec<EventType> {
        events
            .iter()
            .map(|event| event.event_type().clone())
            .collect()
    }
}

impl Defect for UninternedTypeStore {
    const NAME: &'static str = "UninternedTypeStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        let known = Self::interned(events);
        events
            .iter()
            .filter(|event| Self::query_matches(query, &known, event))
            .collect()
    }

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        let known = Self::interned(events);
        correct::first_violation(events, condition, |guard, event| {
            guard.after.is_none_or(|after| event.position > after)
                && Self::query_matches(&guard.query, &known, event)
        })
    }
}

/// Tags are rows in a side table, and the join that reaches them has no
/// `DISTINCT`.
///
/// `InnerJoinTagStore`'s sibling: the same two-table schema, the other way of
/// getting one join wrong. An event carrying three tags has three rows on the
/// other side, so it comes back three times.
///
/// The fan-out is modelled only where **no item constrains tags**, which is what
/// makes the defect survivable rather than obviously broken. The tag-AND path is
/// written as `GROUP BY e.id HAVING COUNT(*) = n` — it has to be, or "all of
/// these tags" is wrong — and the grouping collapses the duplicates for free. The
/// unconstrained path is a plain join with nothing to group by, which is
/// `Query::all()`: the query every projection runner starts from, and the one
/// every other rule in the suite reads over *untagged* events.
pub(crate) struct TagJoinFanOutStore;

impl Defect for TagJoinFanOutStore {
    const NAME: &'static str = "TagJoinFanOutStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        let matched = correct::matching(events, query);
        let grouped = query
            .items()
            .is_some_and(|items| items.iter().any(|item| !item.tags().is_empty()));
        if grouped {
            return matched;
        }
        matched
            .into_iter()
            .flat_map(|event| core::iter::repeat_n(event, event.tags().len().max(1)))
            .collect()
    }
}

/// A multi-item query is one statement per item, concatenated client-side.
///
/// The results are deduplicated — the author noticed that much — but never
/// merge-sorted, so the output is in **item** order rather than in position
/// order. This is the same one-statement-per-item shape ES-12 rejects for a
/// different reason, which is why it is worth having compiled: two clauses, one
/// adapter.
///
/// Note what it is *not*. ES-15's own `Rejects:` names an adapter that sorts and
/// deduplicates a query's items as an optimisation, and no order-invariance rule
/// can catch that — sorting the items is precisely what makes their order stop
/// mattering. What can be caught is a store whose *output* order is the item
/// order, and that is this one.
pub(crate) struct ItemOrderedUnionStore;

impl Defect for ItemOrderedUnionStore {
    const NAME: &'static str = "ItemOrderedUnionStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        let Some(items) = query.items() else {
            return correct::matching(events, query);
        };
        let mut out: Vec<&SequencedEvent> = Vec::new();
        for item in items {
            for event in events {
                if item.matches(event.event_type(), event.tags())
                    && !out.iter().any(|seen| seen.position == event.position)
                {
                    out.push(event);
                }
            }
        }
        out
    }
}

/// A query's items are interned by their **type list**, and a second item with
/// the same types is dropped.
///
/// The optimisation ES-15's `Rejects:` names and that no rule caught until
/// `query_union_is_item_concatenation` landed. `QueryItem::new` already sorts
/// and deduplicates *types* (`query.rs:62-67`), so extending the idea one level
/// up — group the items by their type set, emit one `type_id IN (…)` clause per
/// distinct set — looks like the same move. It is not: the tag half of every
/// swallowed item goes with it, so the query silently selects a **smaller** set
/// than the caller asked for.
///
/// `query_item_order_does_not_change_the_result_set` cannot see it, and that is
/// the point rather than an aside: deduplicating the items is precisely what
/// makes their order stop mattering, so this store passes an order-invariance
/// rule by construction. Only a match-*set* claim over an item whose presence
/// changes the set rejects it.
///
/// The defect is on the read path alone. An adapter would build both statements
/// from one query builder and get it wrong on the condition probe too, but
/// modelling that here would give the store a second declared failure across the
/// whole `condition_*` family for one bug — and `UninternedTypeStore` is already
/// the registered store that carries a query-builder defect onto both sides.
pub(crate) struct ItemDedupByTypeStore;

impl Defect for ItemDedupByTypeStore {
    const NAME: &'static str = "ItemDedupByTypeStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        let Some(items) = query.items() else {
            return correct::matching(events, query);
        };

        // THE DEFECT: the intern key is the type list, so two items differing
        // only in their tags collapse into whichever arrived first.
        let mut interned: Vec<&QueryItem> = Vec::new();
        for item in items {
            if !interned.iter().any(|seen| seen.types() == item.types()) {
                interned.push(item);
            }
        }

        events
            .iter()
            .filter(|event| {
                interned
                    .iter()
                    .any(|item| item.matches(event.event_type(), event.tags()))
            })
            .collect()
    }
}

// =====================================================================
// Read options — order, anchor, truncation
// =====================================================================

/// `ORDER BY type, position`, because the covering index says so.
pub(crate) struct SortByEventTypeStore;

impl Defect for SortByEventTypeStore {
    const NAME: &'static str = "SortByEventTypeStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        let mut sorted = matched;
        // An index scan over `(type, position)` with no outer sort. Position is
        // still the tiebreak, so the result is perfectly ordered — by the wrong
        // key.
        sorted.sort_by(|a, b| {
            a.event_type()
                .cmp(b.event_type())
                .then(a.position.cmp(&b.position))
        });

        if options.backwards {
            sorted.reverse();
            sorted.retain(|event| options.from.is_none_or(|from| event.position <= from));
            sorted.retain(|event| options.to.is_none_or(|to| event.position >= to));
        } else {
            sorted.retain(|event| options.from.is_none_or(|from| event.position >= from));
            sorted.retain(|event| options.to.is_none_or(|to| event.position <= to));
        }
        sorted
    }
}

/// `from` is a zero-based `OFFSET` into the result set rather than a threshold
/// on position.
pub(crate) struct FromIsAnOffsetStore;

impl Defect for FromIsAnOffsetStore {
    const NAME: &'static str = "FromIsAnOffsetStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        // The upper bound is honoured, and correctly — this store's declared
        // defect is the anchor alone. It is applied *before* the skip because
        // that is where a real adapter's `WHERE` clause sits relative to its
        // `OFFSET`.
        let bounded: Vec<&SequencedEvent> = matched
            .into_iter()
            .filter(|event| {
                options.to.is_none_or(|to| {
                    if options.backwards {
                        event.position >= to
                    } else {
                        event.position <= to
                    }
                })
            })
            .collect();
        let directed: Vec<&SequencedEvent> = if options.backwards {
            bounded.into_iter().rev().collect()
        } else {
            bounded
        };
        // THE DEFECT: the anchor is spent as a zero-based OFFSET.
        let offset = options
            .from
            .map_or(0, |from| usize::try_from(from.get()).unwrap_or(usize::MAX));
        directed.into_iter().skip(offset).collect()
    }
}

/// `backwards` reaches the query builder and never reaches the SQL.
pub(crate) struct BackwardsIgnoredStore;

impl Defect for BackwardsIgnoredStore {
    const NAME: &'static str = "BackwardsIgnoredStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        // `ReadOptions` is `Copy`, so this edits a local rather than the
        // caller's: the direction is a `bool` in one struct and a hard-coded
        // `ASC` in the string the other struct builds.
        let mut forwards = options;
        forwards.backwards = false;
        correct::ordered(matched, forwards)
    }
}

/// `ReadOptions` is destructured for the fields the adapter knows about, and
/// `to` is not one of them.
///
/// The shape every `#[non_exhaustive]` options struct invites, and the one
/// ES-16's `Rejects:` names first: the adapter was written before the field
/// existed, it compiles unchanged afterwards because the struct is passed by
/// value, and a bounded backfill silently becomes an unbounded one. There is no
/// error anywhere — the worker reads to the end of the log and the tail worker
/// beside it processes everything twice.
pub(crate) struct ToBoundIgnoredStore;

impl Defect for ToBoundIgnoredStore {
    const NAME: &'static str = "ToBoundIgnoredStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        // `ReadOptions` is `Copy`, so this edits a local rather than the
        // caller's — which is exactly what an adapter that copies the fields it
        // recognises into its own query-builder struct does.
        let mut known = options;
        known.to = None;
        correct::ordered(matched, known)
    }
}

/// `to` is honoured, as `WHERE position < ?`.
///
/// The other half of ES-16's `Rejects:`. Exclusive is the defensible reading of
/// an upper bound in half the APIs anyone has used, and it is wrong here: the
/// window comes back one event short at every chunk boundary, which is invisible
/// until the chunks are reassembled and then presents as a projection missing
/// one event per page for the whole backfill.
pub(crate) struct ToIsExclusiveStore;

impl Defect for ToIsExclusiveStore {
    const NAME: &'static str = "ToIsExclusiveStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        let mut open = options;
        open.to = None;
        correct::ordered(matched, open)
            .into_iter()
            // THE DEFECT: `<` where `<=` was meant, on whichever end `to` bounds.
            .filter(|event| {
                options.to.is_none_or(|to| {
                    if options.backwards {
                        event.position > to
                    } else {
                        event.position < to
                    }
                })
            })
            .collect()
    }
}

/// `WHERE position <= ?` for `to`, copied verbatim into the backwards branch.
///
/// ES-8's `Rejects:` describes this shape for `from` — "a lower bound
/// irrespective of direction, the natural reading of `WHERE position >= ?`
/// copied into the backwards branch" — and `to` is the same mistake one bound
/// over. It is **correct reading forwards**, which is what makes it survivable:
/// every forward `to` rule passes, and a backwards read comes back with the
/// oldest events instead of the newest.
///
/// It is the only store in this binary that fails
/// `read_to_under_backwards_bounds_the_older_end` and nothing else, which is
/// what earns that rule its place beside the other two.
pub(crate) struct BackwardsToIsAnUpperBoundStore;

impl Defect for BackwardsToIsAnUpperBoundStore {
    const NAME: &'static str = "BackwardsToIsAnUpperBoundStore";

    fn ordered(matched: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        let mut unswapped = options;
        unswapped.to = None;
        correct::ordered(matched, unswapped)
            .into_iter()
            // THE DEFECT: `to` never swaps ends. Under `backwards` it bounds the
            // newer end, so the read starts at the newest event and stops
            // nowhere.
            .filter(|event| options.to.is_none_or(|to| event.position <= to))
            .collect()
    }
}

/// `LIMIT n + 1` — the extra row that answers "is there more", never trimmed.
pub(crate) struct FetchOneExtraStore;

impl Defect for FetchOneExtraStore {
    const NAME: &'static str = "FetchOneExtraStore";

    fn truncated(selected: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        let mut selected = selected;
        if let Some(limit) = options.limit {
            selected.truncate(limit.saturating_add(1));
        }
        selected
    }
}

/// A limit of zero is read as no limit at all.
///
/// The DCB reference implementation's `if (limit)` guard, which is a coherent
/// reading of `0` in a language where `0` is falsy. Ported to a language where
/// it is not, it is `NonZeroUsize::new(limit)` — which is what
/// `happenstance-core` itself stored until phase 4, so this is the crate's own
/// former behaviour rather than an invented one.
///
/// The caller it breaks is the one who computed the zero: `.limit(budget -
/// fetched)` at parity does not read nothing, it reads the entire log, and the
/// paging loop that was protecting a memory ceiling stops protecting it with no
/// error and no failing test.
pub(crate) struct LimitZeroIsUnlimitedStore;

impl Defect for LimitZeroIsUnlimitedStore {
    const NAME: &'static str = "LimitZeroIsUnlimitedStore";

    fn truncated(selected: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        let mut selected = selected;
        // THE DEFECT: the zero is spent on the falsiness test rather than on the
        // truncation.
        if let Some(limit) = options.limit
            && limit > 0
        {
            selected.truncate(limit);
        }
        selected
    }
}

/// The forward resume branch never spends the caller's budget.
///
/// `from` arrives for the projection-resume path, which originally passed no
/// limit, and the branch written to serve it never threads `limit` through:
///
/// ```text
/// if let Some(from) = options.from {
///     self.read_resume(from)          // <- limit never reaches here
/// } else {
///     self.read_paged(options.limit)
/// }
/// ```
///
/// That is the order every SQL adapter in this workspace will be written in —
/// the paging query first, the cursor threaded in afterwards — and the
/// workspace's own runner cannot meet it: `run_projection` sets `from` and no
/// limit deliberately, one read for the whole run, so nothing in-tree issues the
/// composition that would notice.
///
/// The backwards branch is left correct, and that is what makes this a scalpel
/// rather than a broken store: the suite *does* compose backwards `from` with
/// `limit`, in `read_backwards_from_with_limit`, so a store that lost the budget
/// in both directions would go red for a reason that is not this defect's.
///
/// The wrong outcome is a silently over-large page. A projection runner that
/// asks for five hundred events from its checkpoint is handed the whole stream,
/// the read-model store behind it buffers a batch nobody sized, and the caller's
/// own paging arithmetic is arithmetic about a number the store ignored. Nothing
/// errors.
pub(crate) struct ForwardPagingBudgetStore;

impl Defect for ForwardPagingBudgetStore {
    const NAME: &'static str = "ForwardPagingBudgetStore";

    fn truncated(selected: Vec<&SequencedEvent>, options: ReadOptions) -> Vec<&SequencedEvent> {
        // THE DEFECT: the resume branch returns the whole ordered tail, and the
        // budget is applied only where `from` is absent. Backwards is left
        // alone, because the resume path this bug grows in reads forwards.
        if options.from.is_some() && !options.backwards {
            return selected;
        }
        correct::truncated(selected, options)
    }
}

/// `LIMIT` is pushed into the scan and the query's predicate is applied to the
/// rows that come back.
///
/// **A reviewer measured this passing the suite as it stood**, and it is the
/// classic event-store bug: `SELECT … FROM events WHERE position >= ? ORDER BY
/// position LIMIT n` is the statement, the tag join is the expensive half, and
/// filtering the *n* rows in the application looks like an optimisation rather
/// than a semantic change. It returns fewer than *n* matches, and sometimes
/// none, while a caller reading "no more events" stops.
///
/// It cannot be a one-step defect: the truncation has to happen before the
/// filter, and `matching` is handed no options while `ordered` and `truncated`
/// are handed no query. [`Defect::select`] is the seam that lets both be in
/// view.
pub(crate) struct LimitBeforeFilterStore;

impl Defect for LimitBeforeFilterStore {
    const NAME: &'static str = "LimitBeforeFilterStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        // The scan: direction, the cursor, and the row budget — everything the
        // index can serve without looking at the query.
        let scanned =
            correct::truncated(correct::ordered(events.iter().collect(), options), options);
        // THE DEFECT: the predicate is applied afterwards, in the application,
        // to whatever the budget happened to include.
        Ok(scanned
            .into_iter()
            .filter(|event| query.matches(event.event_type(), event.tags()))
            .cloned()
            .collect())
    }
}

/// One statement per `QueryItem`, each carrying `LIMIT n`, and the budget is
/// never applied to the union.
///
/// ES-14's `Rejects:` names it, and it is the same adapter shape ES-12 rejects
/// failing for an independent reason: a store that cannot express a disjunction
/// in one statement emits one per item, and the row budget goes onto each of
/// them because that is where the paging clause is written. The union is then
/// merge-sorted correctly — so every ordering rule passes — and a caller who
/// asked for four events gets `n × items`.
///
/// It cannot be a one-step defect: `truncated` is handed no query and `matching`
/// is handed no options, and this defect is a `LIMIT` applied per *item*.
/// [`Defect::select`] is the seam that lets both be in view, which is the same
/// reason `LimitBeforeFilterStore` lives there.
///
/// **It delegates whenever there is no budget**, and that is what keeps it a
/// single defect rather than a second ordering bug: with `limit` unset, per-item
/// evaluation and one-statement evaluation select the same set, and this store's
/// answer is the correct one.
pub(crate) struct LimitPerItemStore;

impl Defect for LimitPerItemStore {
    const NAME: &'static str = "LimitPerItemStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        let Some(items) = query.items() else {
            return Ok(correct::select(events, query, options));
        };
        if options.limit.is_none() {
            return Ok(correct::select(events, query, options));
        }

        // THE DEFECT: the row budget is a clause on each per-item statement, and
        // nothing re-applies it to the merged result.
        let mut merged: Vec<&SequencedEvent> = Vec::new();
        for item in items {
            let single = Query::from_item(item.clone());
            let page = correct::truncated(
                correct::ordered(correct::matching(events, &single), options),
                options,
            );
            for event in page {
                if !merged.iter().any(|seen| seen.position == event.position) {
                    merged.push(event);
                }
            }
        }

        // The merge itself is correct — this store sorts and deduplicates the
        // union properly, so `ItemOrderedUnionStore`'s defect is not also here.
        merged.sort_by_key(|event| event.position);
        if options.backwards {
            merged.reverse();
        }
        Ok(merged.into_iter().cloned().collect())
    }
}

/// `WHERE a OR b AND position >= ?`, without the parentheses.
///
/// The textbook operator-precedence bug, and the reason CF-12 exists: `AND`
/// binds tighter than `OR`, so the cursor applies to the *last* item of the
/// query alone and every event matching any earlier item is returned regardless
/// of where the caller resumed. A projection re-delivers already-checkpointed
/// events forever, with no error anywhere and no failing test.
///
/// It is invisible to a single-item query, which is every read-option rule the
/// suite had: with one item there is nothing for the `OR` to bind wrongly
/// across, and this store's answer is the correct one.
pub(crate) struct UnparenthesisedPredicateStore;

impl Defect for UnparenthesisedPredicateStore {
    const NAME: &'static str = "UnparenthesisedPredicateStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        let Some(items) = query.items() else {
            return Ok(correct::select(events, query, options));
        };
        let Some((last, rest)) = items.split_last() else {
            return Ok(correct::select(events, query, options));
        };

        let mut selected: Vec<&SequencedEvent> = events
            .iter()
            .filter(|event| {
                let bounded = options.from.is_none_or(|from| {
                    if options.backwards {
                        event.position <= from
                    } else {
                        event.position >= from
                    }
                }) && options.to.is_none_or(|to| {
                    if options.backwards {
                        event.position >= to
                    } else {
                        event.position <= to
                    }
                });
                let matched_early = rest
                    .iter()
                    .any(|item| item.matches(event.event_type(), event.tags()));
                let matched_last = last.matches(event.event_type(), event.tags());
                // THE DEFECT: the bound is conjoined with the last disjunct
                // instead of with the whole predicate.
                matched_early || (matched_last && bounded)
            })
            .collect();

        if options.backwards {
            selected.reverse();
        }
        Ok(correct::truncated(selected, options)
            .into_iter()
            .cloned()
            .collect())
    }
}

/// `WHERE a OR b AND position <= ?`, without the parentheses.
///
/// [`UnparenthesisedPredicateStore`]'s twin, one bound over. The lower bound is
/// conjoined with the whole predicate — correctly — and only the **upper** one is
/// left dangling off the last disjunct, which is what an adapter produces when
/// the `to` clause is appended to a `WHERE` string that already carries a cursor
/// and a disjunction someone else built. `AND` binds tighter than `OR`, so every
/// event matching an earlier item comes back regardless of the window's top.
///
/// The wrong outcome is a bounded backfill worker that reads past its own
/// window. It was given `[1, H]` while a tail worker owns everything above, and
/// it re-delivers events the tail worker has already processed — the failure
/// ES-16 exists to forbid, arriving through a query shape ES-16's own rules do
/// not exercise.
///
/// # Why it is `Kind::ModelOnlyMutant`
///
/// It fails no rule in the event-store family, and that is the finding rather
/// than an accident. All three `to` rules — `read_to_is_inclusive`,
/// `read_from_and_to_bound_a_closed_window` and
/// `read_to_under_backwards_bounds_the_older_end` — issue `Query::all()`, and
/// with no items there is nothing for the `OR` to bind wrongly across, so this
/// store's answer is the correct one. The rule that would see it is `to`
/// composed with a multi-item query, which does not exist; CF-12 closed that gap
/// for `from` alone. What catches it instead is the **model** family, which
/// generates multi-item queries and — since phase 12 — an upper bound to go with
/// them.
///
/// Registering it is therefore the honest way to hold that boundary in place. If
/// someone writes the missing rule, this row becomes an ordinary
/// [`Kind::Mutant`] with one entry in `fails` and the meta-tests say so; if the
/// generator ever stops reaching `to`, `MODEL_COVERAGE` goes red and names this
/// store. Either way the claim is checked rather than remembered.
///
/// That the store is defective *at all* is checked separately and without any
/// feature, by [`to_precedence_scenario`] below — the witness
/// `Kind::ModelOnlyMutant` requires. See that function, and
/// [`HidingPlaceStore`], for why the kind needed one.
///
/// It cannot be a one-step defect, for [`UnparenthesisedPredicateStore`]'s
/// reason: the bound and the predicate have to be built together, `matching` is
/// handed no options and `ordered` is handed no query, and [`Defect::select`] is
/// the only seam that sees both.
pub(crate) struct UnparenthesisedToPredicateStore;

impl Defect for UnparenthesisedToPredicateStore {
    const NAME: &'static str = "UnparenthesisedToPredicateStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        let Some(items) = query.items() else {
            return Ok(correct::select(events, query, options));
        };
        let Some((last, rest)) = items.split_last() else {
            return Ok(correct::select(events, query, options));
        };

        let mut selected: Vec<&SequencedEvent> = events
            .iter()
            .filter(|event| {
                // The cursor is conjoined with the whole predicate, which is
                // right, and is what keeps this store distinct from its twin.
                let resumed = options.from.is_none_or(|from| {
                    if options.backwards {
                        event.position <= from
                    } else {
                        event.position >= from
                    }
                });
                let bounded = options.to.is_none_or(|to| {
                    if options.backwards {
                        event.position >= to
                    } else {
                        event.position <= to
                    }
                });
                let matched_early = rest
                    .iter()
                    .any(|item| item.matches(event.event_type(), event.tags()));
                let matched_last = last.matches(event.event_type(), event.tags());
                // THE DEFECT: the UPPER bound is conjoined with the last
                // disjunct instead of with the whole predicate.
                resumed && (matched_early || (matched_last && bounded))
            })
            .collect();

        if options.backwards {
            selected.reverse();
        }
        Ok(correct::truncated(selected, options)
            .into_iter()
            .cloned()
            .collect())
    }
}

/// The two-event log and the bounded read on which
/// [`UnparenthesisedToPredicateStore`] disagrees with [`crate::correct`].
///
/// The witness `Kind::ModelOnlyMutant` requires, and it lives here rather than in
/// the registry for the reason the registry's own documentation gives about
/// distance: the *claim* belongs away from the store, and the *demonstration*
/// belongs beside it, because whoever writes the defect is the only person who
/// knows the smallest input that shows it.
///
/// Two events, one tagged `side:left` and one `side:right`, and a two-item query
/// listing `right` **first** so that `left` is the disjunct the dangling bound
/// attaches to. The read carries `to` at the first event's position. A correct
/// store returns the first event alone; this one also returns the second,
/// because the second matches an earlier disjunct and the upper bound never
/// reaches it. Nothing here needs a generator, a runtime or a feature.
pub(crate) fn to_precedence_scenario() -> (Vec<SequencedEvent>, Query, ReadOptions) {
    let events = correct::sequence(
        &[
            tagged_event("Ay", &[("side", "left")]),
            tagged_event("Bee", &[("side", "right")]),
        ],
        None,
        dense,
    );
    let bound = events[0].position;
    let query = query_of_items([
        item_tagged(&[("side", "right")]),
        item_tagged(&[("side", "left")]),
    ]);
    (events, query, ReadOptions::new().to(bound))
}

/// A store with **no defect at all**, filed as caught by the model family.
///
/// Not a mutant. It is the adversarial refutation of [`Kind::ModelOnlyMutant`],
/// transcribed from the review that found the hole: a bare `impl Defect`
/// carrying only `NAME`, so every step is `crate::correct`'s and the store is
/// byte-for-byte the reference implementation. Filed with an empty `fails` list
/// and a `MODEL_COVERAGE` row claiming `Rejected`, it satisfies both of the
/// obligations that kind was written with — and one of them is not compiled
/// without the `proptest` feature, which is a configuration `cargo hack`'s
/// feature powerset builds.
///
/// It exists so that "the kind cannot be used to hide a store nothing catches"
/// is a *checked* sentence rather than an argument, and it is **deliberately not
/// registered**: a row for it would now be rejected by
/// `every_model_only_mutant_demonstrates_its_defect`, which is the point.
/// `the_model_only_bar_rejects_a_store_with_no_defect` is where it earns its
/// place — it drives this store through every scenario the witness table holds
/// and asserts it agrees with `crate::correct` on all of them, so no witness for
/// it could be written.
pub(crate) struct HidingPlaceStore;

impl Defect for HidingPlaceStore {
    const NAME: &'static str = "HidingPlaceStore";
}

/// The read window is anchored on `max(position)`, which is `NULL` on an empty
/// store.
///
/// ES-11 prescribes exactly this shape for a paginating adapter — capture the
/// head at the first poll and bound every subsequent page by `position <= H` —
/// so it is not a strawman; it is the *recommended* implementation with one
/// column type wrong. `max()` over no rows is `NULL`, and decoding it into a
/// non-nullable integer is a decode error rather than a zero.
///
/// It fails on a store whose only fault is being **new**, which is the state
/// every adapter is in on its first run and the one state every rule in the
/// suite seeded away before it read.
///
/// # Why it errors rather than panics
///
/// It panicked, until a reviewer pointed out what that cost: a store that blows
/// up never reaches `reading_an_empty_store_yields_nothing`'s own assertions, so
/// the rule was CF-1-satisfied by a store falling over rather than by a store
/// answering wrongly, and the row was registered under
/// `FailureMode::StorePanic`, which is the one mode with no origin check to
/// stand behind it. A decode failure is also what the modelled driver actually
/// does. As an `Err` item the failure reaches `read_ok`, whose panic is raised
/// in `suite.rs` — so the row is now an `Assertion` and the origin check holds
/// it there.
pub(crate) struct NullHeadPagingStore;

impl Defect for NullHeadPagingStore {
    const NAME: &'static str = "NullHeadPagingStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        // THE DEFECT: `SELECT max(position) FROM events` decoded into a column
        // type that cannot hold NULL.
        let Some(head) = events.last().map(|event| event.position) else {
            return Err(LogError::NullHead);
        };

        Ok(correct::select(events, query, options)
            .into_iter()
            .filter(|event| event.position <= head)
            .collect())
    }
}

// =====================================================================
// Head (ES-30)
//
// Two one-step defects in `Defect::head_of`, the step ES-30's three rules made
// worth having. Both are correct on every other path, which is what makes them
// survivable: until phase 4 nothing in the suite called `head()` at all.
// =====================================================================

/// `IFNULL(MAX(position), 0)`, and a fallback that is a position.
///
/// `MAX(position)` over an empty table is `NULL`, and a driver's scalar decode
/// wants a column type that can hold what comes back. `IFNULL(…, 0)` is the
/// one-token fix, and then the adapter has a `u64` that has to become a
/// `SequencePosition` — which is a `NonZero` newtype, so `SequencePosition::new`
/// returns an `Option` and this workspace's own house rule forbids `unwrap` in
/// library code. `unwrap_or(SequencePosition::FIRST)` is the shortest spelling
/// that satisfies both, and it reports a position no event occupies on the one
/// state every adapter is in on its first run.
///
/// The cost is not cosmetic. ES-11 *prescribes* anchoring a paginating read on
/// `head()` at the first poll, and ES-31 makes "am I caught up?" a comparison
/// against it: a runner starting against a new store is told the log already
/// holds position 1, checkpoints there, and the first event ever appended is the
/// one it skips.
pub(crate) struct EmptyHeadIsFirstStore;

impl Defect for EmptyHeadIsFirstStore {
    const NAME: &'static str = "EmptyHeadIsFirstStore";

    fn head_of(events: &[SequencedEvent]) -> Option<SequencePosition> {
        // THE DEFECT: `NULL` is spent on a value of the wrong kind. Note that
        // the non-empty answer is untouched, which is what makes this store
        // correct everywhere except on the state nobody seeds.
        Some(correct::head_of(events).unwrap_or(SequencePosition::FIRST))
    }
}

/// `SELECT max(e.position) FROM event e JOIN tag t ON t.event_id = e.id`.
///
/// The head statement written against the same joined view the read path is
/// built around, because there is one view in the adapter and reusing it is the
/// obvious move — the same two-table schema `InnerJoinTagStore` and
/// `TagJoinFanOutStore` model from the read side, met a third way. An event
/// carrying no tags has no row on the other side of the join, so the store's
/// head stops at the highest *tagged* position.
///
/// It is ES-30's second rejected implementation — "a `head` that reports the
/// highest position matching some default query rather than the store's head" —
/// and it is invisible to any rule that appends one uniform batch, which is why
/// `head_is_the_highest_visible_position` has to seed an event the narrower
/// query cannot match. Reads are delegated in full and are correct: an adapter
/// whose reads were also tag-scoped would fail its rule's anchor rather than the
/// property, pinning the failure to the wrong half.
pub(crate) struct DefaultQueryHeadStore;

impl Defect for DefaultQueryHeadStore {
    const NAME: &'static str = "DefaultQueryHeadStore";

    fn head_of(events: &[SequencedEvent]) -> Option<SequencePosition> {
        // THE DEFECT: the `INNER JOIN` is in the `FROM` clause, so an untagged
        // row is not merely unmatched — it is not there to be aggregated over.
        events
            .iter()
            .filter(|event| !event.tags().is_empty())
            .map(|event| event.position)
            .max()
    }
}

// =====================================================================
// Append — allocation, return value, transaction
// =====================================================================

/// The next position is computed once and bound as a single parameter for every
/// row of a multi-row insert.
pub(crate) struct SharedBatchPositionStore;

impl Defect for SharedBatchPositionStore {
    const NAME: &'static str = "SharedBatchPositionStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let position = allocate(head);
        events
            .iter()
            .map(|event| correct::stamp(position, event.clone()))
            .collect()
    }
}

/// `INSERT … RETURNING position` followed by `fetch_one`, which takes the first
/// row.
pub(crate) struct ReturnsFirstOfBatchStore;

impl Defect for ReturnsFirstOfBatchStore {
    const NAME: &'static str = "ReturnsFirstOfBatchStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        let first_index = stored.len();
        let last = correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )?;
        Ok(stored.get(first_index).map_or(last, |event| event.position))
    }
}

/// The batch is inserted, the condition evaluated afterwards, and a violation
/// returns `Err` without rolling anything back — because there is no
/// transaction.
pub(crate) struct WriteThenCheckStore;

impl Defect for WriteThenCheckStore {
    const NAME: &'static str = "WriteThenCheckStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let head = stored.last().map(|event| event.position);
        let written = Self::sequence(events, head, Self::ALLOCATE);
        let last = written
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        stored.extend(written);

        // THE DEFECT: autocommit plus a separate probe. The rows are already
        // durable by the time the answer is known, and `Err` is the only thing
        // left to return.
        if let Some(condition) = condition
            && let Some(conflict) = Self::violation(stored, condition)
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        Ok(last)
    }
}

/// An insert of zero rows is a successful no-op, as it is in every driver.
pub(crate) struct EmptyBatchIsANoOpStore;

impl Defect for EmptyBatchIsANoOpStore {
    const NAME: &'static str = "EmptyBatchIsANoOpStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events.is_empty() {
            // Refusing an empty batch is a decision the adapter has to *make*.
            // This one did not make it, and answers with the head the sequence
            // is currently at.
            return Ok(stored
                .last()
                .map_or(SequencePosition::FIRST, |event| event.position));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The returned row set is read without checking that there is one.
///
/// The only mutant here whose failure is a **panic in the store** rather than a
/// rule's assertion, which is why `FailureMode::StorePanic` exists: a mutant
/// that fails the right rule for the wrong reason reads as proof, so the mode
/// has to be declared and matched against the message.
pub(crate) struct EmptyBatchPanicsStore;

impl Defect for EmptyBatchPanicsStore {
    const NAME: &'static str = "EmptyBatchPanicsStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: no emptiness guard, and the position comes from the last
        // returned row. `MemoryEventStore`'s own append is this code plus the
        // guard.
        //
        // The rows are built and read *before* the condition is probed, which is
        // the order a conditional `INSERT … SELECT … WHERE NOT EXISTS` forces:
        // the positions have to be bound into the statement that carries the
        // guard. It also keeps this store's two declared failures the same kind
        // of failure — a `FailureMode` is one field per mutant, and probing
        // first would make it panic on `append(&[], None)` and return
        // `ConditionViolated` on `append(&[], Some(&matching))`.
        let head = stored.last().map(|event| event.position);
        let written = Self::sequence(events, head, Self::ALLOCATE);
        let last = written
            .last()
            .map(|event| event.position)
            .expect("INSERT … RETURNING produced no rows");

        if let Some(condition) = condition
            && let Some(conflict) = Self::violation(stored, condition)
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        stored.extend(written);
        Ok(last)
    }
}

/// The condition is evaluated **before** the batch is checked for emptiness.
///
/// This is `correct::commit_with`'s own order until phase 3, and it is
/// `MemoryEventStore`'s until phase 3 — D8. The store is otherwise perfect: for
/// every non-empty batch the two orders are indistinguishable, which is why the
/// defect survived in the reference implementation, in the `!Send` reference
/// store that deliberately copied it, and in this file's correct core, all at
/// once.
///
/// What it costs is an interoperability hazard rather than a lost write:
/// `append(&[], Some(&c))` answers `NoEvents` or `ConditionViolated` depending
/// on what the store happens to hold, so two conformant adapters disagree about
/// one call — and `ConditionViolated` means *retry*, so a caller branching on
/// `is_condition_violated()` retries an empty batch forever.
pub(crate) struct ConditionBeforeEmptinessStore;

impl Defect for ConditionBeforeEmptinessStore {
    const NAME: &'static str = "ConditionBeforeEmptinessStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: the order of these two checks, and nothing else. The
        // second is `correct::commit_with`, which now checks emptiness first.
        if let Some(condition) = condition
            && let Some(conflict) = Self::violation(stored, condition)
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// A condition's `after` is validated against the store's own head, and a
/// position the store never assigned is refused.
///
/// Plausible and defensible: an anchor above the head cannot name anything this
/// store issued, so treating it as a client error reads like input validation.
/// ES-28 says otherwise, and the reason is replication — a peer resuming after a
/// gap, or a caller that read a store which has since been truncated, supplies
/// exactly this. On a store allocating in steps, *every* position between two
/// assigned ones is one it never assigned.
pub(crate) struct AfterValidatedAgainstHeadStore;

impl Defect for AfterValidatedAgainstHeadStore {
    const NAME: &'static str = "AfterValidatedAgainstHeadStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: input validation on an opaque ordering key.
        if let Some(condition) = condition
            && condition
                .guards()
                .iter()
                .filter_map(|guard| guard.after)
                .any(|after| stored.last().is_none_or(|event| event.position < after))
        {
            return Err(AppendError::Store(LogError::UnknownPosition));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// `metadata` is missing from the `INSERT` column list.
pub(crate) struct DropsMetadataStore;

impl Defect for DropsMetadataStore {
    const NAME: &'static str = "DropsMetadataStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let stripped: Vec<Event> = events.iter().map(without_metadata).collect();
        correct::sequence(&stripped, head, allocate)
    }
}

/// `event` with its metadata column dropped.
fn without_metadata(event: &Event) -> Event {
    let parts = event.clone().into_parts();
    let (event_type, data, tags) = (parts.event_type, parts.data, parts.tags);
    match Event::new(event_type.as_str(), data) {
        Ok(stripped) => stripped.with_tags(tags),
        // The type round-trips through the validator that produced it, so this
        // arm is unreachable. Returning the original rather than panicking keeps
        // a mutant of the *write* path from ever failing a rule with a runtime
        // panic, which `FailureMode::Assertion` would then reject as the wrong
        // reason.
        Err(_) => event.clone(),
    }
}

/// A condition violation surfaces as `AppendError::Store`.
pub(crate) struct ViolationAsStoreErrorStore;

impl Defect for ViolationAsStoreErrorStore {
    const NAME: &'static str = "ViolationAsStoreErrorStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        match correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        ) {
            // THE DEFECT: the driver's own error, passed straight through.
            Err(AppendError::ConditionViolated(_)) => {
                Err(AppendError::Store(LogError::UniqueViolation))
            }
            other => other,
        }
    }
}

// =====================================================================
// ES-19 — the batch written the wrong way round
// =====================================================================

/// The multi-row `INSERT` is built from the batch **reversed**, so the first
/// event of the slice is written last and takes the highest position.
///
/// Two real routes to it, and neither is a slip in the SQL. An adapter that
/// accumulates rows by pushing onto a stack and then drains it emits them
/// backwards; an adapter that groups a batch by event type — to bind one interned
/// type id per group rather than one per row, which is the first optimisation
/// anybody makes to a multi-row insert — reorders the batch and does not notice
/// that grouping is reordering.
///
/// It returns the **maximum** position, which is what makes it survivable: on a
/// quiescent store the highest position is the last row in the log whatever order
/// the rows went in, so `append_returns_last_written_position` is satisfied and
/// the store looks correct from the one place anybody checks.
pub(crate) struct ReverseOrderBatchStore;

impl Defect for ReverseOrderBatchStore {
    const NAME: &'static str = "ReverseOrderBatchStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        // THE DEFECT: one `rev()`. Positions are still dense, still unique and
        // still strictly ascending down the log — they are simply attached to the
        // wrong events, and `commit_with` then returns the last of them, which is
        // the *first* event of the caller's slice.
        let reversed: Vec<Event> = events.iter().rev().cloned().collect();
        correct::sequence(&reversed, head, allocate)
    }
}

// =====================================================================
// ES-21 — the guard carried per row
// =====================================================================

/// The append condition travels with **every row** of the batch, so the second
/// row is checked against a store that already holds the first.
///
/// The per-row conditional `INSERT … SELECT … WHERE NOT EXISTS`, which the
/// decision ledger carries as a live candidate for the append-condition SQL
/// strategy (`RUNBOOK.md:64`) and which ES-21 names. It is attractive precisely
/// because it needs no interactive transaction: the guard and the write are one
/// statement, which is the only shape `happenstance-neon` can express at all.
/// Carried per row it self-rejects on the canonical DCB uniqueness shape, where
/// the condition names the very type being written.
///
/// It **rolls back**, and that is the whole point of the shape rather than a
/// detail: a store that self-rejected and kept the rows it had written would fail
/// `append_is_atomic`, and the registry would then be unable to say which of the
/// two defects that rule catches. This one is atomic and wrong about *what the
/// condition is evaluated against*, which is the one thing ES-21 says.
pub(crate) struct PerRowConditionStore;

impl Defect for PerRowConditionStore {
    const NAME: &'static str = "PerRowConditionStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // The transaction. Cheap here and honest: a real adapter has `ROLLBACK`.
        let before = stored.clone();
        let mut last = None;

        for event in events {
            // THE DEFECT: `condition` is passed again for every row, so from the
            // second row onwards the probe reads a store that includes the rows
            // this same batch has just written.
            match correct::commit_with(
                stored,
                core::slice::from_ref(event),
                condition,
                Self::ALLOCATE,
                Self::violation,
                Self::sequence,
            ) {
                Ok(position) => last = Some(position),
                Err(err) => {
                    *stored = before;
                    return Err(err);
                }
            }
        }

        last.ok_or(AppendError::NoEvents)
    }
}

// =====================================================================
// ES-22 — a suspension point between two rows
// =====================================================================

/// One `INSERT` per row, with an `.await` between rows and no transaction around
/// them.
///
/// `NoTransactionStore`'s sibling on the other axis, and the two must not be
/// merged. That one needs a fault *armed* to show its defect and answers `Err`
/// over a partial log; this one needs no fault at all — the caller simply stops
/// polling, which at the edge is the *normal* termination path: a client
/// disconnect, a CPU limit, a Durable Object eviction, a pod eviction. The future
/// is destroyed at whatever suspension point it had reached and the rows already
/// written stay written, with no `Result` anywhere for anybody to read.
///
/// It also is not `RowAtATimeStore`, in `racers.rs`, which
/// `a_concurrent_reader_never_sees_a_partial_batch` owns: there every row lands
/// in the end and what is wrong is the window a *reader* can see through. Here a
/// row never lands.
///
/// # Why this cannot be a [`Defect`]
///
/// [`Defect::commit`] is a synchronous function and the defect **is** the
/// suspension point, so there is nowhere in that trait to put one — the same
/// reason `PreCommitPositionStore` and `AwaitAcrossBorrowStore` are written
/// longhand. Everything else is delegated to `crate::correct`, including the
/// condition probe, so the only difference from a correct store is where the
/// awaits are.
///
/// The condition is evaluated **once**, with the first row, against the store as
/// it stood before the batch — which is correct (ES-21) and deliberate. A store
/// that also got the condition wrong would fail two rules and the registry could
/// not say which defect either of them caught.
#[derive(Debug, Clone)]
pub(crate) struct YieldingRowAtATimeStore(Rc<RefCell<Log>>);

/// Suspends once: `Pending` on the first poll, `Ready` on the second.
///
/// Written out rather than reached for from a runtime, because this binary has
/// none: it drives everything through `happenstance_testkit::block_on`. The waker
/// is signalled *before* returning `Pending`, which is what makes this a store
/// that is slow rather than one that is hung — `PagedStream` in `variants.rs`
/// carries the same note and the same hazard.
async fn yield_once() {
    let mut yielded = false;
    core::future::poll_fn(move |cx| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    })
    .await;
}

impl EventStore for YieldingRowAtATimeStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
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
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let mut last = None;
        for (row, event) in events.iter().enumerate() {
            {
                let mut log = self
                    .0
                    .try_borrow_mut()
                    .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
                // The condition rides with the first row only, so it is evaluated
                // against the store as it stood before this batch — which is what
                // ES-21 requires and is not this store's defect.
                let carried = if row == 0 { condition } else { None };
                last = Some(log.append(core::slice::from_ref(event), carried)?);
            }
            // THE DEFECT: the borrow is released and the future suspends between
            // two rows of one batch, with nothing holding them together. A caller
            // that stops polling here has left `row + 1` rows in a log it was
            // never told about.
            yield_once().await;
        }

        last.ok_or(AppendError::NoEvents)
    }

    // Delegated to a correct store over the same log rather than answered here,
    // for `NoTransactionStore`'s reason: a `Log`'s events are private to
    // `crate::correct`, and a second implementation of either would be a second
    // difference on a store that is allowed exactly one.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        LogStore::over(&self.0).head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        LogStore::over(&self.0).contains_event_id(id).await
    }
}

/// One log, and any number of row-at-a-time handles onto it.
#[derive(Debug)]
pub(crate) struct YieldingRowAtATimeFixture(Rc<RefCell<Log>>);

impl Fixture for YieldingRowAtATimeFixture {
    type Store = YieldingRowAtATimeStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(
        "a Vec behind an Rc, with no durable medium to reopen over — this \
         instrument's axis is what a dropped future leaves behind, not \
         durability",
    );

    async fn connect(&self) -> Self::Store {
        YieldingRowAtATimeStore(Rc::clone(&self.0))
    }
}

impl Subject for YieldingRowAtATimeFixture {
    const NAME: &'static str = "YieldingRowAtATimeStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Log::new(dense))))
    }
}

// =====================================================================
// Append conditions — the probe
// =====================================================================

/// `guard.after.unwrap_or(FIRST)`, so an event at the first position never
/// violates.
pub(crate) struct AfterDefaultsToFirstStore;

impl Defect for AfterDefaultsToFirstStore {
    const NAME: &'static str = "AfterDefaultsToFirstStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        // The `Option` collapsed at the boundary because the SQL wanted a value.
        correct::first_violation(events, condition, |guard, event| {
            let after = guard.after.unwrap_or(SequencePosition::FIRST);
            event.position > after && guard.query.matches(event.event_type(), event.tags())
        })
    }
}

/// The probe asks whether *any* event exists after the anchor, and drops the
/// condition's query.
pub(crate) struct ExistenceProbeStore;

impl Defect for ExistenceProbeStore {
    const NAME: &'static str = "ExistenceProbeStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        // `SELECT EXISTS(SELECT 1 FROM events WHERE position > ?)` — the fast
        // path someone adds when the join is the expensive half.
        correct::first_violation(events, condition, |guard, event| {
            guard.after.is_none_or(|after| event.position > after)
        })
    }
}

/// `position >= after` in the condition probe.
pub(crate) struct AfterIsInclusiveStore;

impl Defect for AfterIsInclusiveStore {
    const NAME: &'static str = "AfterIsInclusiveStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        // `after` is exclusive and `from` is inclusive; one comparison served
        // both in the first draft.
        correct::first_violation(events, condition, |guard, event| {
            guard.after.is_none_or(|after| event.position >= after)
                && guard.query.matches(event.event_type(), event.tags())
        })
    }
}

/// `after` skips the first *N* matching events instead of anchoring on position.
pub(crate) struct AfterIsAnOffsetStore;

impl Defect for AfterIsAnOffsetStore {
    const NAME: &'static str = "AfterIsAnOffsetStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        // Applied per guard, like every other condition mutant here, so that
        // the offset reading is the only difference from `correct::violation`.
        condition.guards().iter().find_map(|guard| {
            let skip = guard.after.map_or(0, |after| {
                usize::try_from(after.get()).unwrap_or(usize::MAX)
            });
            events
                .iter()
                .filter(|event| guard.query.matches(event.event_type(), event.tags()))
                .nth(skip)
                .map(|event| event.position)
        })
    }
}

/// The condition probe compares serialised tags with `=`.
///
/// `ExactTagMatchReadStore`'s sibling one path over, and the pairing is the
/// whole argument of CF-9: **the read path and the probe are different code**.
/// An adapter that serialises tags to one canonical column to avoid a join will
/// often do it in the probe alone, because the probe is where the join costs
/// most — and then a stored event carrying the condition's tags *plus one more*
/// does not violate. The uniqueness guard silently stops guarding for exactly
/// the entities that have accumulated the most tags.
pub(crate) struct ExactTagMatchConditionStore;

impl Defect for ExactTagMatchConditionStore {
    const NAME: &'static str = "ExactTagMatchConditionStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        correct::first_violation(events, condition, |guard, event| {
            guard.after.is_none_or(|after| event.position > after)
                && exact_tag_match(&guard.query, event)
        })
    }
}

/// The condition probe drops the tag join and matches on **type alone**.
///
/// The canonical DCB uniqueness shape, wrong in the direction that looks safe.
/// The join is the expensive half of the probe and tags live in a second table
/// in the planned SQLite schema, so dropping it is the natural first cut — and
/// the result over-rejects rather than under-rejects, which reads as caution.
///
/// It is the worse of the two CF-7/CF-8 defects to ship: an adapter with it
/// rejects **every** command touching any entity of a type any other entity of
/// that type has an event for. Total availability failure, certified as
/// conformant, with no error in any log.
pub(crate) struct TagBlindConditionStore;

impl Defect for TagBlindConditionStore {
    const NAME: &'static str = "TagBlindConditionStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        correct::first_violation(events, condition, |guard, event| {
            guard.after.is_none_or(|after| event.position > after)
                && type_only_match(&guard.query, event)
        })
    }
}

/// The probe is an aggregate whose `NULL` on an empty table is read as a
/// rejection.
///
/// `SELECT max(position) FROM events WHERE <query>` returns `NULL` over no rows,
/// `NULL > ?` is *unknown* rather than false, and whether a `WHERE` discards the
/// unknown or a wrapping `NOT` turns it into a match depends on how the
/// predicate is nested. This store is the nesting that rejects.
///
/// It is invisible to every other condition rule in the suite, all of which seed
/// the store before they evaluate anything — so the case it breaks is every
/// adapter's very first conditional append.
pub(crate) struct NullAggregateProbeStore;

impl Defect for NullAggregateProbeStore {
    const NAME: &'static str = "NullAggregateProbeStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        if events.is_empty() {
            // THE DEFECT: three-valued logic collapsed the wrong way. There is
            // no conflicting event to name, which is itself the tell — the
            // position reported is invented because the aggregate had none.
            return Some(SequencePosition::FIRST);
        }
        correct::violation(events, condition)
    }
}

/// The probe ANDs two **uncorrelated** predicates: *does any event match the
/// query* and *is the head above `after`*.
///
/// What the check becomes when the existence test and the position test are
/// written as separate subqueries, which is the obvious decomposition — each
/// half is a statement someone can read. It is correct on every case the rest of
/// the suite exercises, because in all of them the matching event *is* the
/// latest one.
///
/// Where it is wrong is the steady state of a quiet entity in a busy store: the
/// caller read to a boundary above the last event touching its own entity, some
/// unrelated writer has since moved the head, and the append is refused. The
/// deployment reads it as contention and the retry loop never converges, because
/// nothing about the entity has changed.
pub(crate) struct UncorrelatedProbeStore;

impl Defect for UncorrelatedProbeStore {
    const NAME: &'static str = "UncorrelatedProbeStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        let matched = events.iter().find(|event| {
            condition
                .guards()
                .iter()
                .any(|guard| guard.query.matches(event.event_type(), event.tags()))
        })?;
        let head = events.last().map(|event| event.position)?;
        // THE DEFECT: the two questions are asked of the whole store rather than
        // of one event.
        let moved_on = condition
            .guards()
            .iter()
            .any(|guard| guard.after.is_none_or(|after| head > after));
        moved_on.then_some(matched.position)
    }
}

/// Whether `event`'s tags **equal** a condition query's, item by item.
///
/// Extracted so the defect is one comparison rather than a reimplementation of
/// query matching: everything else — the type half, the OR across items, the
/// `Query::All` case — is the correct behaviour.
fn exact_tag_match(query: &Query, event: &SequencedEvent) -> bool {
    match query.items() {
        None => true,
        Some(items) => items.iter().any(|item| {
            let tags_ok =
                item.tags().is_empty() || event.tags().as_slice() == item.tags().as_slice();
            type_matches(item, event.event_type()) && tags_ok
        }),
    }
}

/// Whether `event` matches a condition query with the tag clause dropped.
fn type_only_match(query: &Query, event: &SequencedEvent) -> bool {
    match query.items() {
        None => true,
        Some(items) => items
            .iter()
            .any(|item| type_matches(item, event.event_type())),
    }
}

// =====================================================================
// VT-30 — the two ways a guard fold goes wrong
// =====================================================================

/// Every guard is evaluated against `min(after)` across the guards.
///
/// The application-side workaround E2E-05 names, promoted into an adapter: four
/// reads produce four boundaries, one `WHERE position > ?` takes one number, and
/// the safe-looking choice is the smallest. It is sound — it never admits an
/// append it should have refused — and it is a liveness failure, which is the
/// harder defect to see: the quiet fragment's stale boundary governs the busy
/// one, so a consistency boundary that never conflicted starts refusing and the
/// deployment reads the rejection rate as contention rather than as a bug.
///
/// It passes the **entire** existing `condition_after_*` family, because every
/// rule in it carries a single guard and the minimum over one boundary is that
/// boundary. `condition_guards_carry_independent_boundaries` is the only thing
/// that can see it, which is what makes that rule non-decorative rather than a
/// restatement.
///
/// `None` is treated as the minimum, and that is the faithful reading rather than
/// a convenience: an unbounded guard checks the whole log, so a collapse that
/// took the smallest *stated* number would be strictly weaker than the condition
/// the caller wrote, and no adapter would ship that direction.
pub(crate) struct MinCollapseStore;

impl Defect for MinCollapseStore {
    const NAME: &'static str = "MinCollapseStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        let collapsed = if condition.guards().iter().any(|guard| guard.after.is_none()) {
            None
        } else {
            condition
                .guards()
                .iter()
                .filter_map(|guard| guard.after)
                .min()
        };

        // THE DEFECT: one boundary for every guard. The per-guard *query* is
        // still applied, so this differs from `correct::violation` in exactly one
        // comparison.
        correct::first_violation(events, condition, |guard, event| {
            collapsed.is_none_or(|after| event.position > after)
                && guard.query.matches(event.event_type(), event.tags())
        })
    }
}

/// The `guards.len() == 1` fast path is the statement that predates boundaries.
///
/// [`MinCollapseStore`]'s mirror image, and the regression VT-30's refactor
/// actually invites. An adapter generalising to N guards keeps a fast path for
/// one, because one guard is the overwhelmingly common case and a `UNION` per
/// guard is pure overhead there — and the fast path is the *old* statement, the
/// uniqueness probe written before `after` existed, kept because it was already
/// working. The general path is new and was reviewed; the fast path is old and
/// was not.
///
/// The result is a store with two answers to one question: a caller whose
/// decision model read one fragment gets a boundary that is ignored, and the same
/// caller with two fragments gets one that is honoured. Nothing in an adapter's
/// own tests distinguishes those two callers.
///
/// It fails three of the existing `condition_after_*` rules as well as
/// `condition_with_one_guard_behaves_as_today`, and that overlap is intrinsic
/// rather than sloppy: every rule in that family carries one guard, so a store
/// wrong on the one-guard path is wrong in all of them. What none of them can see
/// — and what its own rule is for — is that the *other* path is right, which is
/// the difference between this store and `ExistenceProbeStore`.
pub(crate) struct SingleGuardFastPathStore;

impl Defect for SingleGuardFastPathStore {
    const NAME: &'static str = "SingleGuardFastPathStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        if condition.guards().len() == 1 {
            // THE DEFECT: `SELECT EXISTS(SELECT 1 FROM events WHERE <query>)`,
            // with no position predicate at all — which is exactly right for a
            // uniqueness guard, where `after` is `None` anyway, and silently
            // wrong for every caller that supplies one.
            return correct::first_violation(events, condition, |guard, event| {
                guard.query.matches(event.event_type(), event.tags())
            });
        }
        correct::violation(events, condition)
    }
}

// =====================================================================
// ES-18 — the fault-injection axis, wrong end
// =====================================================================

/// The `BEGIN` is missing, so a fault part way through a batch leaves the rows
/// already written.
///
/// `variants::GappedPositionStore` with one line removed, and the two answer the
/// caller identically: `Err(AppendError::Store(..))`. What differs is the log
/// afterwards, which is the only place ES-18 can be observed at all.
///
/// This cannot be a [`Defect`] for [`CachedHeadStore`]'s reason turned inside
/// out: the defect is not a step, it is what the store does when a *fixture*
/// arms it, so the store and the fixture have to share a cell and
/// [`Defect::commit`] is handed neither.
///
/// It is registered as a mutant of exactly one rule and it is correct in every
/// other respect — including under `append_is_atomic`, whose rejection comes
/// through the condition path and never arms anything. That is the pair ES-18
/// insists on: two rules, two defects, neither reachable from the other.
#[derive(Debug, Clone)]
pub(crate) struct NoTransactionStore {
    log: Rc<RefCell<Log>>,
    fault: Fault,
}

impl EventStore for NoTransactionStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.log
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
        if let Some(after) = self.fault.get()
            && events.len() > after
        {
            self.fault.set(None);
            let mut log = self
                .log
                .try_borrow_mut()
                .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
            // THE DEFECT: `for event in batch { conn.execute(INSERT, …)? }` with
            // nothing around it. The rows that went in before the fault stay in,
            // and the `Err` below tells the caller none of them did.
            let _ = log.append(&events[..after], condition);
            return Err(AppendError::Store(LogError::WriteFailed));
        }

        let mut log = self
            .log
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        log.append(events, condition)
    }

    // Delegated to a correct store over the same log rather than answered here.
    // A `Log`'s events are private to `crate::correct`, and the alternative —
    // reaching them through `select` with a contrived `ReadOptions` — would be a
    // second implementation of `head` in the one file whose whole premise is that
    // there is only ever one. This store's declared defect is the missing
    // `BEGIN`, and nothing else may differ.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        LogStore::over(&self.log).head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        LogStore::over(&self.log).contains_event_id(id).await
    }
}

/// One log, one arming slot, and any number of transaction-less handles.
#[derive(Debug)]
pub(crate) struct NoTransactionFixture {
    log: Rc<RefCell<Log>>,
    fault: Fault,
}

impl Fixture for NoTransactionFixture {
    type Store = NoTransactionStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(
        "a Vec behind an Rc, with no durable medium to reopen over — this \
         instrument's axis is the fault, not durability",
    );
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        NoTransactionStore {
            log: Rc::clone(&self.log),
            fault: Rc::clone(&self.fault),
        }
    }

    async fn arm_mid_batch_fault(&self, after: usize) {
        self.fault.set(Some(after));
    }
}

impl Subject for NoTransactionFixture {
    const NAME: &'static str = "NoTransactionStore";

    fn open() -> Self {
        Self {
            log: Rc::new(RefCell::new(Log::new(dense))),
            fault: Rc::new(Cell::new(None)),
        }
    }
}

// =====================================================================
// Value edges — the columns that are wrong only at the boundary
//
// Every store in this section is correct for the values every other rule in the
// suite writes, and that is the point rather than a caveat: each one is a mapping
// that looks total because the *middle* of its range is all anybody tested. Four
// of them share one root cause between two of them — the helper that
// `Option`-ises a blob on the way in — which is why the pair land together.
// =====================================================================

/// Rebuilds an event with its identifiers rewritten by `map`.
///
/// Shared by the two identifier mutants, which differ only in what `map` does to
/// a string. Rebuilding rather than mutating is forced by the value types: an
/// `EventType` and a `Tag` are validated on construction and expose no setter,
/// which is the property that makes them worth having and the reason a store
/// mangling one has to go through the constructor to do it.
///
/// A rewritten value that fails validation leaves the event untouched. That arm
/// is unreachable for both callers — truncation stops on a character boundary and
/// `?` is a legal character — and it returns the original rather than panicking
/// for `without_metadata`'s reason: a mutant of the write path must never fail a
/// rule with a runtime panic, which `FailureMode::Assertion` would then reject as
/// the wrong reason.
fn with_identifiers_mapped(event: &Event, map: impl Fn(&str) -> String) -> Event {
    let happenstance_core::EventParts {
        event_type,
        data,
        tags,
        metadata,
        ..
    } = event.clone().into_parts();

    let mapped: Option<Vec<Tag>> = tags
        .iter()
        .map(|tag| Tag::new(map(tag.as_str())).ok())
        .collect();
    let (Some(mapped), Ok(rebuilt)) = (mapped, Event::new(map(event_type.as_str()), data)) else {
        return event.clone();
    };

    let rebuilt = rebuilt.with_tags(mapped.into_iter().collect::<Tags>());
    match metadata {
        Some(metadata) => rebuilt.with_metadata(metadata),
        None => rebuilt,
    }
}

/// The longest prefix of `value` that fits in `bytes`, cut on a character
/// boundary.
///
/// `MySQL` in non-strict mode truncates a `utf8mb4` column at a *character*
/// boundary and warns; it does not split a codepoint. Modelling it the other way
/// would produce a value that is not UTF-8, which `EventType` cannot hold — so
/// the faithful model is also the only representable one.
fn truncated_to(value: &str, bytes: usize) -> String {
    let end = value
        .char_indices()
        .map(|(at, ch)| at + ch.len_utf8())
        .take_while(|end| *end <= bytes)
        .last()
        .unwrap_or(0);
    value[..end].to_owned()
}

/// A zero-length payload is normalised to `NULL`, and the column is `NOT NULL`.
///
/// One helper — `(!blob.is_empty()).then_some(blob)` — binds both blob columns,
/// because `metadata` genuinely is optional and writing it once is the obvious
/// move. `data` is not optional, its column is `NOT NULL`, and the constraint
/// fires on an event that is perfectly legal.
///
/// [`MetadataConflatingStore`] is the same helper against the *nullable* column,
/// where nothing fires at all.
pub(crate) struct EmptyPayloadIsNullStore;

impl Defect for EmptyPayloadIsNullStore {
    const NAME: &'static str = "EmptyPayloadIsNullStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: the emptiness test is applied to a column that cannot hold
        // the answer.
        if events.iter().any(|event| event.data().is_empty()) {
            return Err(AppendError::Store(LogError::NotNullViolation));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// Zero-length metadata is normalised to absent metadata.
///
/// The nullable half of [`EmptyPayloadIsNullStore`]'s helper: `metadata` is an
/// `Option<Bytes>` already, so `filter(|m| !m.is_empty())` reads as tidying up
/// rather than as losing information — and a driver that maps a zero-length
/// `BLOB` to `NULL` does the same thing without being asked.
///
/// `DropsMetadataStore` is one column further along the same road, and fails the
/// same rule for the stronger reason.
pub(crate) struct MetadataConflatingStore;

impl Defect for MetadataConflatingStore {
    const NAME: &'static str = "MetadataConflatingStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let normalised: Vec<Event> = events
            .iter()
            .map(|event| {
                // THE DEFECT: `Some(<empty>)` and `None` are the same value on
                // the way in, so they are the same value for ever after.
                if matches!(event.metadata(), Some(metadata) if metadata.is_empty()) {
                    without_metadata(event)
                } else {
                    event.clone()
                }
            })
            .collect();
        correct::sequence(&normalised, head, allocate)
    }
}

/// The identifier columns are narrower than the constants say, and the engine
/// truncates rather than refusing.
///
/// `VARCHAR(64)` for an event type is what an adapter author picks after looking
/// at their own domain, where the longest type is thirty characters.
/// `MAX_EVENT_TYPE_LEN` and `MAX_TAG_LEN` are 255 **bytes** and VT-20 keeps them
/// there precisely because they are the width an adapter declares against. Under
/// `MySQL`'s non-strict `sql_mode` the over-length value is stored truncated with a
/// warning nobody reads.
pub(crate) struct NarrowIdentifierColumnStore;

impl NarrowIdentifierColumnStore {
    /// The column width this adapter actually declared.
    const WIDTH: usize = 64;
}

impl Defect for NarrowIdentifierColumnStore {
    const NAME: &'static str = "NarrowIdentifierColumnStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let truncated: Vec<Event> = events
            .iter()
            .map(|event| with_identifiers_mapped(event, |value| truncated_to(value, Self::WIDTH)))
            .collect();
        correct::sequence(&truncated, head, allocate)
    }
}

/// The identifier columns are ASCII-only, and the driver transcodes.
///
/// `VARCHAR(255) CHARACTER SET latin1`, SQL Server's non-`N` `VARCHAR`, or a
/// `CHECK` written against an `[[:ascii:]]` class. The first two do not refuse:
/// the driver converts, and every codepoint outside the target charset becomes
/// `?`. VT-14 permits Unicode category `Cf` deliberately — U+200C and U+200D are
/// load bearing in Persian, in Devanagari and in every emoji ZWJ sequence — so
/// this is a store that accepts values it then cannot represent.
pub(crate) struct Latin1IdentifierStore;

impl Defect for Latin1IdentifierStore {
    const NAME: &'static str = "Latin1IdentifierStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let transcoded: Vec<Event> = events
            .iter()
            .map(|event| {
                with_identifiers_mapped(event, |value| {
                    // THE DEFECT: the lossy half of a charset conversion, which
                    // is silent on every driver that does it at all.
                    value
                        .chars()
                        .map(|ch| if ch.is_ascii() { ch } else { '?' })
                        .collect()
                })
            })
            .collect();
        correct::sequence(&transcoded, head, allocate)
    }
}

/// The payload column has an undocumented ceiling, met at write time.
///
/// A payload held inline in a fixed-width column, or a KV backend with a
/// per-value cap the adapter never states. VT-21 makes 65,536 bytes a floor every
/// store must accept and requires a store that accepts less to *document* it and
/// refuse distinguishably; this one does neither, which is the shape rather than
/// a specific product.
pub(crate) struct PayloadCeilingStore;

impl PayloadCeilingStore {
    /// The ceiling this adapter never wrote down.
    const CEILING: usize = 4096;
}

impl Defect for PayloadCeilingStore {
    const NAME: &'static str = "PayloadCeilingStore";

    // CF-40: the ceiling this adapter never wrote down, written down. VT-21
    // requires a store to document its actual limit; stating it here is what
    // lets `append_reports_exceeded_store_limits` find out that the refusal
    // arrives as `AppendError::Store` rather than as `ExceedsStoreLimit`.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(Self::CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events
            .iter()
            .any(|event| event.data().len() > Self::CEILING)
        {
            return Err(AppendError::Store(LogError::ValueTooLarge));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The payload column has the same undocumented ceiling, and the engine
/// truncates rather than refusing.
///
/// The non-strict half of [`PayloadCeilingStore`], and it exists because that
/// store is caught by `append_ok` — the rule's *setup* — rather than by the
/// property the rule is named for. VT-21's MUST has two halves, "accept 65,536
/// bytes" and "refuse rather than truncating", and until this store compiled
/// nothing in the registry rejected the second: a store that accepted the payload
/// and stored a prefix passed every assertion the suite could raise.
///
/// It is the same pairing [`NarrowIdentifierColumnStore`] models one column over,
/// and the same engine setting: `MySQL`'s `sql_mode`. Strict raises
/// `ER_DATA_TOO_LONG` and the write fails loudly; non-strict stores what fits and
/// warns, and the warning goes to a log nobody reads. The ceiling matches
/// `PayloadCeilingStore`'s on purpose — one adapter, one column, two deployments.
pub(crate) struct TruncatingPayloadStore;

impl Defect for TruncatingPayloadStore {
    const NAME: &'static str = "TruncatingPayloadStore";

    // CF-40. The same column and the same number as `PayloadCeilingStore` —
    // one adapter, one column, two deployments — so that the two halves of
    // VT-25's MUST are checked at the same boundary: refuse through the wrong
    // variant, and do not refuse at all.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(PayloadCeilingStore::CEILING);

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let truncated: Vec<Event> = events
            .iter()
            .map(|event| {
                if event.data().len() <= PayloadCeilingStore::CEILING {
                    return event.clone();
                }
                // THE DEFECT: what fits is stored, and the caller is told the
                // whole write landed.
                let happenstance_core::EventParts {
                    event_type,
                    data,
                    tags,
                    metadata,
                    ..
                } = event.clone().into_parts();
                let clipped = data.slice(..PayloadCeilingStore::CEILING);
                // `event_type` came off a valid `Event`, so reconstruction cannot
                // fail; returning the original rather than panicking is the same
                // discipline `with_identifiers_mapped` follows, and for the same
                // reason — a mutant of the write path that panics fails a rule for
                // a reason `FailureMode::Assertion` would then reject.
                let Ok(rebuilt) = Event::new(event_type.as_str(), clipped) else {
                    return event.clone();
                };
                let rebuilt = rebuilt.with_tags(tags);
                match metadata {
                    Some(metadata) => rebuilt.with_metadata(metadata),
                    None => rebuilt,
                }
            })
            .collect();
        correct::sequence(&truncated, head, allocate)
    }
}

/// Tags are packed into one fixed-width column, and the overflow is dropped.
///
/// The schema that avoids a side table and a join: the canonical tag list joined
/// into a single `VARCHAR(255)`. It is correct for every event anybody looked at,
/// and VT-22 makes 64 tags a floor. What falls off the end is not reported, so
/// the event is in the store and a query on a dropped tag does not find it — VT-17's
/// map-shaped-index failure reached from a different door.
pub(crate) struct PackedTagColumnStore;

impl PackedTagColumnStore {
    /// The width of the packed column.
    const WIDTH: usize = 255;
}

impl Defect for PackedTagColumnStore {
    const NAME: &'static str = "PackedTagColumnStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let packed: Vec<Event> = events
            .iter()
            .map(|event| {
                // THE DEFECT: whatever does not fit in the column is not stored,
                // and nothing says so.
                let mut used = 0;
                let kept: Vec<Tag> = event
                    .tags()
                    .iter()
                    .take_while(|tag| {
                        used += tag.as_str().len() + usize::from(used > 0);
                        used <= Self::WIDTH
                    })
                    .cloned()
                    .collect();
                event.clone().with_tags(kept.into_iter().collect::<Tags>())
            })
            .collect();
        correct::sequence(&packed, head, allocate)
    }
}

/// A query's items are chunked to fit the driver's parameter limit, and only the
/// first chunk is sent.
///
/// An adapter that emits one bound parameter per item meets
/// `SQLITE_MAX_VARIABLE_NUMBER`, or Postgres' 65,535-parameter cap, and the fix
/// everybody reaches for is to chunk. Unioning the chunks is a second edit, and
/// this is the store where it was not made: the answer comes back *wrong* rather
/// than as an error, which is the part VT-23 is about.
pub(crate) struct ChunkedQueryStore;

impl ChunkedQueryStore {
    /// How many items fit in one round trip.
    ///
    /// **Chosen to sit below VT-23's floor of 128, and it is not a measurement.**
    /// At this suite's grain — one item, one tag — 64 items is nowhere near
    /// `SQLITE_MAX_VARIABLE_NUMBER`'s 32,766 or Postgres' 65,535, and reading the
    /// number as one is how a future reader concludes SQLite chokes on 64 query
    /// items. A real adapter's ceiling is a function of *parameters per item*,
    /// which depends on how it renders a tag conjunction; the shape being
    /// modelled — chunk, then forget to union the chunks — is what does not
    /// depend on that.
    const CHUNK: usize = 64;
}

impl Defect for ChunkedQueryStore {
    const NAME: &'static str = "ChunkedQueryStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        let Some(items) = query.items().filter(|items| items.len() > Self::CHUNK) else {
            return correct::matching(events, query);
        };
        // THE DEFECT: the first chunk is the whole answer.
        match Query::from_items(items[..Self::CHUNK].iter().cloned()) {
            Ok(first_chunk) => correct::matching(events, &first_chunk),
            Err(_) => correct::matching(events, query),
        }
    }
}

/// A multi-row `INSERT` binds one parameter set per event, and the driver has a
/// ceiling.
///
/// The refusal arrives at write time — after the caller has made its decision and
/// taken its side effects — which is exactly VT-24's complaint. A hundred events
/// at three columns and a tag apiece is inside every driver's limit, which is why
/// nobody meets this until a batch gets big.
pub(crate) struct BatchParameterCeilingStore;

impl BatchParameterCeilingStore {
    /// The largest batch this adapter's statement can carry.
    ///
    /// **Chosen to sit below VT-24's floor of 128, and it is not a measurement**
    /// — the same caveat [`ChunkedQueryStore::CHUNK`] carries. A hundred events
    /// at three columns and a tag apiece is inside every driver's limit; where a
    /// real adapter meets its ceiling is a function of parameters per *event*,
    /// which depends on its schema. What is faithful is the timing: the refusal
    /// arrives at write time, after the caller has decided.
    const CEILING: usize = 100;
}

impl Defect for BatchParameterCeilingStore {
    const NAME: &'static str = "BatchParameterCeilingStore";

    // CF-40: the driver's parameter ceiling, stated as the batch ceiling it
    // actually is.
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(Self::CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events.len() > Self::CEILING {
            return Err(AppendError::Store(LogError::TooManyParameters));
        }
        correct::commit_with(
            stored,
            events,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The batch is clamped to what one statement can carry, and the rest is
/// dropped.
///
/// The botched fix for [`BatchParameterCeilingStore`], and it stands in the same
/// relation to it that [`TruncatingPayloadStore`] does to
/// [`PayloadCeilingStore`]: the refusing store is caught by `append_ok` at the
/// rule's setup, so until this compiled nothing rejected the half of VT-24 the
/// rule is actually named for — that the batch comes back **whole and in order**.
///
/// The shape is the `truncate` reflex rather than a chunk loop. An author who has
/// just met "too many SQL variables" reaches for the limit that made it go away,
/// and `&events[..CEILING]` is one character from `events.chunks(CEILING)`; only
/// the second needs a loop, a transaction around the loop, and a decision about
/// which chunk's position to return. The store answers `Ok` with the position of
/// the last row it did write, which is a real position, so nothing downstream has
/// anything to notice.
pub(crate) struct ChunkLosingBatchStore;

impl Defect for ChunkLosingBatchStore {
    const NAME: &'static str = "ChunkLosingBatchStore";

    // CF-40. `BatchParameterCeilingStore`'s number, because this is that store
    // with the fix applied wrongly: the ceiling did not move, only what the
    // adapter does when it meets it.
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(BatchParameterCeilingStore::CEILING);

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        // THE DEFECT: the ceiling is applied to the caller's batch instead of to
        // the statement, so the tail is never written and never reported.
        let clamped = events
            .get(..BatchParameterCeilingStore::CEILING)
            .unwrap_or(events);
        correct::commit_with(
            stored,
            clamped,
            condition,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

/// The tag column carries a normalising collation, so two normal forms become
/// one value.
///
/// `CREATE COLLATION … (provider = icu, deterministic = false)` is one line and
/// is exactly what an author reaches for when a search stops matching an
/// accented word; a normaliser called in the row mapper "because tags should be
/// canonical" is the same defect written by hand. Either way a caller's
/// identifier is rewritten on the way in, so the tag read back is not the tag
/// written, ADR-0003's
/// byte-for-byte forwarding promise is broken, and the store's index disagrees
/// with every external system holding the original string.
///
/// A macOS client writes `"café"` in NFD and a Linux client writes it in NFC.
/// They render identically everywhere, and under this store they stop being two
/// consistency boundaries with no visible cue at all.
///
/// # It composes one sequence, and that is stated rather than hidden
///
/// A faithful NFC needs Unicode tables, which is the dependency VT-14 already
/// declines for the contract crate and which this test binary has no more claim
/// on. What is modelled is the composition of `e` + U+0301, which is the
/// sequence the rule writes and the only one this store is ever handed. A real
/// collation composes everything; the *observable* behaviour on this input is
/// the same, and the rule is what is being instrumented.
pub(crate) struct NormalisingTagStore;

impl NormalisingTagStore {
    // `&str` rather than `&'static str`: the lifetime is implied on a `const`
    // and `clippy::redundant_static_lifetimes` is a warn-by-default style lint
    // under a `-D warnings` gate. `Defect::NAME` spells it out because the trait
    // declaration does.

    /// `e` followed by U+0301 COMBINING ACUTE ACCENT.
    const DECOMPOSED: &str = "e\u{301}";
    /// U+00E9 LATIN SMALL LETTER E WITH ACUTE.
    const COMPOSED: &str = "\u{e9}";
}

impl Defect for NormalisingTagStore {
    const NAME: &'static str = "NormalisingTagStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let normalised: Vec<Event> = events
            .iter()
            .map(|event| {
                // THE DEFECT: the value stored is not the value written.
                with_identifiers_mapped(event, |value| {
                    value.replace(Self::DECOMPOSED, Self::COMPOSED)
                })
            })
            .collect();
        correct::sequence(&normalised, head, allocate)
    }
}

/// Tags are a map from key to value, so a repeated key keeps one value.
///
/// A `JSONB` object, a `HashMap<String, String>` column, or a side table under
/// `UNIQUE (event_id, key)` written with `ON CONFLICT (event_id, key) DO UPDATE`.
/// All three are natural schemas for something the contract itself invites you
/// to read as a pair — `Tag::key` and `Tag::value` are public — and all three
/// keep exactly one value per key.
///
/// `Tags::from_pairs([("tenant", "a"), ("tenant", "b")])` succeeds and yields a
/// **two**-element set, because deduplication is on the whole `key:value` string.
/// The event stays in the store and stops matching one of the two queries that
/// should select it. On Wattline's 4,200-tenant shared log that is a
/// cross-tenant correctness failure produced entirely by an indexing choice: the
/// tenant whose tag was dropped stops seeing its own events, and nothing
/// anywhere reports a fault.
///
/// A tag with no colon has no key and is kept, which is not a courtesy — it is
/// what the modelled schema does, since there is nothing to conflict on. It is
/// also what keeps this store invisible to the model family, whose generated
/// tags are single letters.
pub(crate) struct KeyedTagMapStore;

impl Defect for KeyedTagMapStore {
    const NAME: &'static str = "KeyedTagMapStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let mapped: Vec<Event> = events
            .iter()
            .map(|event| {
                let mut kept: Vec<Tag> = Vec::new();
                for tag in event.tags() {
                    // The search is `position` over an immutable borrow rather
                    // than `find` over a mutable one, so that the borrow is over
                    // before the `else` arm pushes. `find(..)` reads better and
                    // is `error[E0499]`: the scrutinee's `&mut kept` is still
                    // live in the arm that grows the vector.
                    let occupied = tag
                        .key()
                        .and_then(|key| kept.iter().position(|held| held.key() == Some(key)));
                    // THE DEFECT: `DO UPDATE`. The last value under a key wins,
                    // and `Tags` is sorted, so which one survives is stable and
                    // arbitrary. A tag with no key has nothing to conflict on
                    // and is kept, which is what the modelled schema does.
                    if let Some(at) = occupied {
                        kept[at] = tag.clone();
                    } else {
                        kept.push(tag.clone());
                    }
                }
                event.clone().with_tags(kept.into_iter().collect::<Tags>())
            })
            .collect();
        correct::sequence(&mapped, head, allocate)
    }
}

/// Identifiers are trimmed on the way in, because a trailing space "must be a
/// typo".
///
/// `TRIM()` in the insert statement, or `value.trim()` in the row mapper. It is
/// the most defensible-looking of the value-edge defects and the one with the
/// worst consequence, because it moves the decision about what an identifier
/// *is* from the application into the store: the value the caller wrote is no
/// longer in the log, so nothing downstream can detect what happened.
///
/// Kestrel Rotor replicated a `SerialisedUnitConsumed` carrying
/// `turbine:HW2-A14 `. `Tag::new` accepts it, `Tags` sorts it adjacent to the
/// unpadded tag, `contains_all` is a strict merge-scan on equality
/// (`tag.rs:231-245`) and does not match it, and a lot-recall query silently
/// missed a turbine. VT-15 makes that the application's bug to prevent and the
/// contract's job to state; an adapter that "fixes" it produces a different
/// wrong answer and hides the first one.
///
/// It is [`NarrowIdentifierColumnStore`]'s and [`Latin1IdentifierStore`]'s third
/// sibling — one column, three ways for it to rewrite what it was given — and
/// like both of them it is correct for every identifier the rest of this binary
/// writes.
pub(crate) struct TrimmingIdentifierStore;

impl Defect for TrimmingIdentifierStore {
    const NAME: &'static str = "TrimmingIdentifierStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let trimmed: Vec<Event> = events
            .iter()
            // THE DEFECT: whitespace is significant everywhere (VT-15), and this
            // store is the one place in the deployment that disagrees.
            .map(|event| with_identifiers_mapped(event, |value| value.trim().to_owned()))
            .collect();
        correct::sequence(&trimmed, head, allocate)
    }
}

// =====================================================================
// Identity, recorded time and membership
//
// Every store here is correct on the read path and on the write path. What each
// one is wrong about is a *store-assigned fact* — an identity, an incarnation,
// a time, or the answer to a membership question — which is why none of them
// fails a rule that landed before phase 4. That is the point rather than a
// coincidence: `SequencedEvent` grew two fields, and a field nothing can be
// wrong about is a field no rule needs.
// =====================================================================

/// The `SELECT` has no identity column, so the row mapper synthesises one from
/// the row's ordinal in the result set.
///
/// The shape is an `.enumerate()` inside `rows.map(…)`, and it is what an
/// adapter reaches for when `EventId` arrives on `SequencedEvent` after the
/// schema is written and nobody wants a migration this week. Under
/// `Query::all()` on a densely-allocated store the synthesised value is
/// **right**, which is exactly what makes it survivable: it is wrong only when
/// the result set is a different shape from the log, which is every filtered
/// read — and every rule in the suite that reads the whole log back agrees with
/// it.
pub(crate) struct RowOrdinalIdentityStore;

impl Defect for RowOrdinalIdentityStore {
    const NAME: &'static str = "RowOrdinalIdentityStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        Ok(correct::select(events, query, options)
            .into_iter()
            .enumerate()
            .map(|(ordinal, event)| {
                // THE DEFECT: the identity comes from where the row landed in
                // *this* answer, not from what the store assigned the event.
                let ordinal = u64::try_from(ordinal).unwrap_or(0).saturating_add(1);
                let synthesised = SequencePosition::new(ordinal).unwrap_or(SequencePosition::FIRST);
                SequencedEvent::new(
                    event.position,
                    EventId::new(correct::TEST_STORE, synthesised),
                    event.recorded_at,
                    event.event,
                )
            })
            .collect())
    }
}

/// There is no `recorded_at` column, so the row mapper fills the field from the
/// connection's clock.
///
/// `RecordedAt` arrives on `SequencedEvent` after the table exists; the field
/// has to be given *something*, and the connection's `now()` is the value
/// already in scope. The store is correct in every other respect, and the
/// fabrication is invisible to any rule that reads once — which, before phase 4,
/// was every rule.
///
/// The counter stands in for the clock, and the substitution is not a
/// simplification. A mutant that read a real clock would be non-deterministic,
/// and `correct.rs` fixes its time for the same reason; what is being modelled
/// is that the value moves between reads, not what it moves to.
pub(crate) struct ReadTimeClockStore;

thread_local! {
    /// Ticks once per read, standing in for a wall clock that has moved.
    ///
    /// Thread-local rather than a field, because a [`Defect`] is a type and
    /// never a value — the same reason every step is an associated function.
    static READ_TIME: Cell<i64> = const { Cell::new(0) };
}

impl Defect for ReadTimeClockStore {
    const NAME: &'static str = "ReadTimeClockStore";

    fn select(
        events: &[SequencedEvent],
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, LogError> {
        let read_at = READ_TIME.with(|tick| {
            let next = tick.get().saturating_add(1);
            tick.set(next);
            RecordedAt::from_millis(next)
        });
        Ok(correct::select(events, query, options)
            .into_iter()
            // THE DEFECT: the time is a property of the read, not of the append.
            .map(|event| event.with_recorded_at(read_at))
            .collect())
    }
}

/// The `EventId` is derived from the event's bytes rather than from the position
/// the store assigned.
///
/// Content-addressed identity is what anyone reaching for idempotent ingest
/// proposes first, and it is exactly what VT-8 forbids by making uniqueness the
/// store's obligation rather than the caller's. Its identities are unique across
/// *distinct* events, stable across a reopen and never reissued, so it survives
/// every rule about identity except the two that notice where the identity came
/// from: `appending_equal_events_yields_two_events`, which is VT-2's and is the
/// clause this store exists for, and `append_stamps_a_local_event_id`, whose
/// first assertion is that a local append's `id.position()` is the position the
/// store assigned.
///
/// **ADR-0014 §9 says it "fails this one alone"**, which was written before
/// VT-5's rule existed; `notes.md` records the disagreement rather than papering
/// over it.
pub(crate) struct ContentHashIdentityStore;

impl ContentHashIdentityStore {
    /// The FNV-1a offset basis.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    /// The FNV-1a prime.
    const PRIME: u64 = 0x0000_0100_0000_01b3;

    /// One FNV-1a round over `bytes`.
    ///
    /// A free-standing fold rather than a closure capturing the accumulator:
    /// the closure form borrows the accumulator mutably for its whole scope, and
    /// spelling the state as an argument keeps the borrow checker out of a
    /// function whose only job is to be boring.
    fn fold(hash: u64, bytes: &[u8]) -> u64 {
        let mut hash = hash;
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(Self::PRIME);
        }
        hash
    }

    /// FNV-1a over exactly the parts `Event`'s `PartialEq` compares, folded into
    /// a non-zero position.
    ///
    /// Hand-written because the testkit depends on no hasher and
    /// `DefaultHasher` is `std`-only *and* documented as not stable across
    /// releases — a mutant whose identity changed with the toolchain would be a
    /// flake in the one file that exists to be deterministic.
    fn digest(event: &Event) -> SequencePosition {
        let mut hash = Self::fold(Self::OFFSET, event.event_type().as_str().as_bytes());
        hash = Self::fold(hash, &event.data()[..]);
        for tag in event.tags() {
            hash = Self::fold(hash, tag.as_str().as_bytes());
        }
        if let Some(metadata) = event.metadata() {
            hash = Self::fold(hash, &metadata[..]);
        }

        // `| 1` rather than a fallback branch: `SequencePosition` is non-zero,
        // and forcing the low bit costs one collision class out of 2^64 while
        // removing the only failure mode of the conversion.
        SequencePosition::new(hash | 1).unwrap_or(SequencePosition::FIRST)
    }
}

impl Defect for ContentHashIdentityStore {
    const NAME: &'static str = "ContentHashIdentityStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        correct::sequence(events, head, allocate)
            .into_iter()
            .map(|event| {
                // THE DEFECT: identity is a function of the payload, so two
                // structurally equal events are one event.
                let digest = Self::digest(&event.event);
                SequencedEvent::new(
                    event.position,
                    EventId::new(correct::TEST_STORE, digest),
                    event.recorded_at,
                    event.event,
                )
            })
            .collect()
    }
}

/// A fresh incarnation minted per **event** rather than per store.
///
/// VT-6 permits an adapter to mint a fresh `StoreId` on every open, and an
/// adapter that reads "mint often, never reissue" as the whole of the clause
/// arrives here. Minting per append satisfies the literal MUST — no
/// `(StoreId, SequencePosition)` pair is ever issued twice — and destroys
/// everything the type is for: every event becomes its own origin, a peer's
/// `Watermark` grows one row per event rather than one per incarnation, and
/// VT-5's peer-independent sort degenerates to comparing 128 opaque bits.
///
/// This is the defect `store_id_is_stable_across_reopen` caught by accident, and
/// the reason `reopened_store_does_not_reissue_an_event_id` carries a first
/// assertion that looks like padding and is not. It is registered against
/// `append_stamps_a_local_event_id` rather than against that rule because
/// `MutantFixture` declines `REOPEN`, so the reopen-gated rule would report a
/// skip here — which is why VT-5's rule carries the incarnation assertion as
/// well.
///
/// The incarnation is derived from the position rather than sampled, because a
/// mutant that needed entropy would be a mutant whose output moved between runs.
pub(crate) struct PerEventStoreIdStore;

impl Defect for PerEventStoreIdStore {
    const NAME: &'static str = "PerEventStoreIdStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        correct::sequence(events, head, allocate)
            .into_iter()
            .map(|event| {
                // THE DEFECT: a new incarnation per row.
                let mut bytes = correct::TEST_STORE.to_bytes();
                bytes[..8].copy_from_slice(&event.position.get().to_be_bytes());
                SequencedEvent::new(
                    event.position,
                    EventId::new(StoreId::from_bytes(bytes), event.position),
                    event.recorded_at,
                    event.event,
                )
            })
            .collect()
    }
}

/// `INSERT … RETURNING event_id` read once and applied to every row of the
/// batch.
///
/// The off-by-a-loop `SharedBatchPositionStore` models on the position column,
/// arriving on the identity column where no rule about positions can see it: the
/// positions are all correct here, so `positions_are_unique` and
/// `positions_are_strictly_monotonic` both pass and the store holds two events
/// it cannot tell apart. It is the shape an adapter takes when identity is added
/// to a write path that already returns one value per statement rather than one
/// per row.
pub(crate) struct SharedBatchIdentityStore;

impl Defect for SharedBatchIdentityStore {
    const NAME: &'static str = "SharedBatchIdentityStore";

    fn sequence(
        events: &[Event],
        head: Option<SequencePosition>,
        allocate: Allocate,
    ) -> Vec<SequencedEvent> {
        let sequenced = correct::sequence(events, head, allocate);
        // An empty batch never reaches here — `commit_with` refuses it above —
        // but the `else` keeps this function total rather than indexing.
        let Some(shared) = sequenced.first().map(|event| event.id) else {
            return sequenced;
        };
        sequenced
            .into_iter()
            // THE DEFECT: one identity for the whole statement.
            .map(|event| {
                SequencedEvent::new(event.position, shared, event.recorded_at, event.event)
            })
            .collect()
    }
}

/// The `EventId` is materialised as a row in the tag side table, so that a
/// membership question can be answered by the index the adapter already has.
///
/// VT-7's named wrong implementation, and the tempting part is that the tag is
/// **not on the event**: the adapter writes the extra row itself, so
/// `Event::tags()` round-trips untouched and every payload-fidelity rule in the
/// suite still passes. What changes is that identity has entered the matching
/// algebra — a point lookup on a unique key grafted onto a set-superset
/// predicate — so `Items(a) ∪ Items(b) == Items(a ++ b)`, which VT-31 freezes
/// and E2E-32's fan-out runner depends on, stops being a statement about the
/// domain's vocabulary alone.
///
/// It is behaviourally identical to a correct store for every query whose items
/// do not name an `event_id` tag, which is every other rule in the suite: adding
/// a tag to the set an item is tested against can only ever make *more* things
/// match, and only for an item carrying that exact tag.
pub(crate) struct IdentityMatchableAsTagStore;

impl Defect for IdentityMatchableAsTagStore {
    const NAME: &'static str = "IdentityMatchableAsTagStore";

    fn matching<'a>(events: &'a [SequencedEvent], query: &Query) -> Vec<&'a SequencedEvent> {
        events
            .iter()
            .filter(|event| match query.items() {
                None => true,
                Some(items) => {
                    // THE DEFECT: the tag set the index answers from is the
                    // event's, plus a row the adapter wrote itself.
                    let mut indexed: Vec<Tag> = event.tags().iter().cloned().collect();
                    if let Ok(identity) = Tag::key_value("event_id", &event.id.to_string()) {
                        indexed.push(identity);
                    }
                    let indexed: Tags = indexed.into_iter().collect();
                    items.iter().any(|item| {
                        type_matches(item, event.event_type()) && indexed.contains_all(item.tags())
                    })
                }
            })
            .collect()
    }
}

/// `SELECT 1 FROM events WHERE position = ?`, with the `StoreId` half of the
/// identity dropped from the `WHERE` clause.
///
/// ES-41's named wrong implementation, and the natural query for an adapter
/// whose events table has one position column and no origin columns yet — which
/// is every adapter before it implements ingest. It passes every single-store
/// rule in the suite, because a store that has ingested nothing only ever holds
/// its own incarnation, and it fails the first time a peer asks: a foreign event
/// is reported as already present whenever the local log happens to be at least
/// that long, the ingest that trusted the answer skips it, and a real fact is
/// dropped with no error and no symptom. That is the same failure mode VT-6
/// spends a clause preventing, arriving by a different door.
pub(crate) struct PositionOnlyMembershipStore;

impl Defect for PositionOnlyMembershipStore {
    const NAME: &'static str = "PositionOnlyMembershipStore";

    fn contains(events: &[SequencedEvent], id: EventId) -> bool {
        // THE DEFECT: the incarnation is not in the predicate.
        events
            .iter()
            .any(|event| event.id.position() == id.position())
    }
}

// =====================================================================
// ES-24 — the store that deduplicates for you
// =====================================================================

/// Rows are content-addressed on `(event_type, tags, data)`, so a byte-identical
/// event is written once however many times it is appended.
///
/// `INSERT … ON CONFLICT DO NOTHING` against a unique index on a content hash,
/// which is what an adapter built to be safe under at-least-once ingest reaches
/// for and would ship as a feature. It is correct-looking from every angle a
/// reviewer checks: no data is lost, no error is hidden, and the *documented*
/// at-most-once guarantee (ES-24's first shape, a condition matching its own
/// events) still holds.
///
/// What it does is strengthen the two shapes the contract explicitly disclaims,
/// and that is worse than not providing the guarantee at all: callers write retry
/// loops against the adapter they happened to test on, and the same loop
/// double-charges against the next one. ES-24 therefore makes duplicate-landing a
/// MUST, and this store is what that MUST rejects.
///
/// The dedup runs **after** the condition probe, which is where a unique index
/// lives — the constraint is met at write time, not at plan time — and it matters
/// here: probing first is what keeps `racing_conditional_appends_elect_one_winner`
/// and `interleaved_appends_on_one_handle_elect_one_winner` green, so that this
/// store's declared failures are about reissue and not about concurrency.
pub(crate) struct PayloadDedupStore;

impl Defect for PayloadDedupStore {
    const NAME: &'static str = "PayloadDedupStore";

    fn commit(
        stored: &mut Vec<SequencedEvent>,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<LogError>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        if let Some(condition) = condition
            && let Some(conflict) = Self::violation(stored, condition)
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        // THE DEFECT: the unique index on the content hash. Whatever is already
        // there is not written again, and the caller is told the append
        // succeeded.
        let fresh: Vec<Event> = events
            .iter()
            .filter(|event| !stored.iter().any(|held| held.event == **event))
            .cloned()
            .collect();

        if fresh.is_empty() {
            // Every row was already present, so `ON CONFLICT DO NOTHING` wrote
            // nothing and the position reported is the one the existing copy of
            // the batch's last event already had.
            return stored
                .iter()
                .rev()
                .find(|held| events.last().is_some_and(|last| &held.event == last))
                .map(|held| held.position)
                .ok_or(AppendError::NoEvents);
        }

        correct::commit_with(
            stored,
            &fresh,
            None,
            Self::ALLOCATE,
            Self::violation,
            Self::sequence,
        )
    }
}

// =====================================================================
// CF-39 — the fixture whose fault does nothing
// =====================================================================

/// A fixture that declares `MID_BATCH_FAULT` supported and arms **nothing**.
///
/// The store under it is `LogStore`, which is completely correct, and that is the
/// point: the defect is not in the store at all. It is a fixture claiming a
/// capability it does not supply, and the cost is that
/// `append_is_atomic_under_a_mid_batch_fault` reports a green atomicity result
/// for a store nothing has ever faulted — a rule that ran, asserted, and observed
/// nothing.
///
/// # The empty body is load-bearing, and a forgotten override is not the same
/// mistake
///
/// `Fixture::arm_mid_batch_fault`'s provided body **panics**, and its message
/// names this exact hazard, so a fixture that declares the capability and simply
/// forgets to override it aborts loudly — which is the outcome that method was
/// written for. The vacuous pass needs an override that is present, honest-looking
/// and empty. That is what is written below, and writing it any other way would
/// make this store demonstrate the panic rather than the hole.
///
/// # Why it is plausible
///
/// Every fixture in this workspace whose store has no injectable fault declines
/// the capability, and the trait defaults to declining precisely so that
/// answering takes deliberate effort. The author who writes this one is the
/// author of a *real* adapter who intends to wire a trigger up later, declares
/// the capability while stubbing the method, and gets a green suite with the
/// atomicity rule apparently exercised. There is nothing in the build to tell
/// them otherwise until CF-39.
#[derive(Debug)]
pub(crate) struct NoopFaultFixture(LogStore);

impl Fixture for NoopFaultFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::declined(
        "a Vec behind an Rc, with no durable medium to reopen over — this \
         instrument's axis is what a declared fault capability actually does",
    );
    // THE DEFECT, first half: the claim.
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        self.0.clone()
    }

    // THE DEFECT, second half: an override with an empty body. Not a forgotten
    // override — that reaches the trait's panic, which is a different outcome and
    // a different mistake.
    async fn arm_mid_batch_fault(&self, after: usize) {
        // `let _` rather than an unused parameter: the gate denies warnings, and
        // an underscore-prefixed name would read as "this argument is not
        // interesting" when what is being modelled is a body that forgot to use
        // it.
        let _ = after;
    }
}

impl Subject for NoopFaultFixture {
    const NAME: &'static str = "NoopFaultFixture";

    fn open() -> Self {
        Self(LogStore::new(dense))
    }
}

// =====================================================================
// The eight that cannot be one step
// =====================================================================

/// A handle whose append-condition probe runs against a **cached** head.
///
/// This cannot be a [`Defect`] because the defect is not a property of the
/// store's steps: it is state acquired when a *handle* is opened and refreshed
/// only by that handle's own appends. Every single-handle rule keeps the cache
/// current and therefore cannot see it, which is what makes the shape plausible
/// rather than obviously broken.
///
/// Reads are delegated in full, deliberately: the fast path exists precisely
/// because the condition probe is the expensive half, so getting the read right
/// and the probe wrong is the realistic shape — and a mutant that also read
/// stalely would fail the rule's *first* assertion, pinning the failure to the
/// wrong half.
#[derive(Debug)]
pub(crate) struct CachedHeadStore {
    events: Rc<RefCell<Vec<SequencedEvent>>>,
    /// THE DEFECT: the highest position this handle believes exists.
    head: Cell<Option<SequencePosition>>,
}

impl EventStore for CachedHeadStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.events
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| correct::select(&stored, query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first, as ES-20 requires and as `correct::commit_with` does.
        // It is repeated here rather than left to the delegation below because
        // this store evaluates the condition itself, and doing that first would
        // give it a second, undeclared defect —
        // `ConditionBeforeEmptinessStore`'s.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let mut stored = self
            .events
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;

        if let Some(condition) = condition {
            let cached = self.head.get();
            let visible: Vec<SequencedEvent> = stored
                .iter()
                .filter(|event| cached.is_some_and(|head| event.position <= head))
                .cloned()
                .collect();
            if let Some(conflict) = correct::violation(&visible, condition) {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict,
                )));
            }
        }

        // The condition has already been decided, so it is not passed on.
        let position = correct::commit(&mut stored, events, None, dense)?;
        self.head.set(Some(position));
        Ok(position)
    }

    // Answered from the log, **not** from `self.head`, for the reason `read` is
    // delegated in full: this store's declared defect is the staleness of the
    // *condition probe*, and a handle that also reported its cache as the store's
    // head would fail rules it does not declare. The realistic shape is the one
    // modelled — the cache exists because the probe is the expensive half.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let stored = self
            .events
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::head_of(&stored))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let stored = self
            .events
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::contains(&stored, id))
    }
}

/// One log, and handles that each cache its head when they open.
#[derive(Debug)]
pub(crate) struct CachedHeadFixture(Rc<RefCell<Vec<SequencedEvent>>>);

impl Fixture for CachedHeadFixture {
    type Store = CachedHeadStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the handle-multiplicity axis only");

    async fn connect(&self) -> Self::Store {
        // Read once, here, and never again — the whole premise of the fast path
        // being modelled.
        let head = self.0.borrow().last().map(|event| event.position);
        CachedHeadStore {
            events: Rc::clone(&self.0),
            head: Cell::new(head),
        }
    }
}

impl Subject for CachedHeadFixture {
    const NAME: &'static str = "CachedHeadFixture";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }
}

/// A handle that answers `head()` from the position its own last `append`
/// returned.
///
/// ES-30's first rejected implementation: "a cached last-written position, which
/// is stale the moment a second handle writes". Whether the cache is a field, a
/// session variable or Postgres' own `currval()` is an implementation detail;
/// what makes it wrong is that a store is not a connection, and every deployment
/// with a pool reaches one store two ways.
///
/// # Why it is not a [`Defect`], and why it is not [`CachedHeadStore`] widened
///
/// The same reason as [`CachedHeadStore`]: the defect is not a property of the
/// store's steps but state acquired when a *handle* opens and refreshed only by
/// that handle's own writes. `Defect::head_of` is handed the log and nothing
/// else, which is exactly right for a head that is wrong about the log and
/// leaves nowhere for a head that is wrong about *which* log it last looked at.
///
/// And it is a second store rather than a second defect on the first.
/// `CachedHeadStore`'s declared defect is scoped to its **condition probe**, and
/// its `head` answers from the log on purpose — a handle that also reported its
/// cache as the store's head would fail rules it does not declare, which is the
/// failure `mutants_fail_exactly_their_declared_rules` exists to catch. Two
/// plausible adapters share one cache and spend it in two places; they are two
/// rows.
///
/// Reads, appends and the condition probe are delegated in full and are correct.
/// That is the realistic shape *and* the one that pins the failure: a handle
/// whose reads were also stale would fail
/// `head_advances_across_two_handles` at its anchor rather than at its head
/// assertion, and the row would evidence ES-34 instead of ES-30.
#[derive(Debug)]
pub(crate) struct LastWrittenHeadStore {
    events: Rc<RefCell<Vec<SequencedEvent>>>,
    /// THE DEFECT: the highest position this handle has *written*, sampled when
    /// the handle opened and advanced by nothing else.
    written: Cell<Option<SequencePosition>>,
}

impl EventStore for LastWrittenHeadStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.events
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| correct::select(&stored, query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let mut stored = self
            .events
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        let position = correct::commit(&mut stored, events, condition, dense)?;
        self.written.set(Some(position));
        Ok(position)
    }

    // THE DEFECT, spent: the log is right there and is not consulted.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.written.get())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let stored = self
            .events
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::contains(&stored, id))
    }
}

/// One log, and handles that each sample its head when they open.
#[derive(Debug)]
pub(crate) struct LastWrittenHeadFixture(Rc<RefCell<Vec<SequencedEvent>>>);

impl Fixture for LastWrittenHeadFixture {
    type Store = LastWrittenHeadStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the handle-multiplicity axis only");

    async fn connect(&self) -> Self::Store {
        // Sampled once, here — a session that reads `max(position)` when it is
        // checked out of the pool. Without this a handle that has written
        // nothing would report `None` even on a store it *did* connect to
        // full, which is a cruder defect than the one being modelled and would
        // fail `head_of_an_empty_store_is_none`'s anchor for a different reason.
        let written = self.0.borrow().last().map(|event| event.position);
        LastWrittenHeadStore {
            events: Rc::clone(&self.0),
            written: Cell::new(written),
        }
    }
}

impl Subject for LastWrittenHeadFixture {
    const NAME: &'static str = "LastWrittenHeadStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }
}

/// A fixture that acknowledges an append before its `COMMIT` returns.
///
/// The store is completely correct; what is wrong is that there is nothing
/// behind it. A reopen discards the process state and finds an empty medium,
/// which is what a pooled store answering on a `spawn_blocking` join, a Durable
/// Object trusting output-gate semantics it does not have, or a connection
/// carrying `PRAGMA synchronous = OFF` looks like from the outside.
///
/// It is the one entry here that declares `REOPEN` supported, because declining
/// it would turn its declared failure into a skip — which
/// `mutants_fail_exactly_their_declared_rules` reports as a hole in the map
/// rather than as a pass, and rightly.
#[derive(Debug)]
pub(crate) struct LosingFixture {
    /// Replaced wholesale by `reopen`, so a handle taken beforehand keeps the
    /// old log exactly as a real connection keeps talking to a closed file.
    live: RefCell<Rc<RefCell<Log>>>,
}

impl Fixture for LosingFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.live.borrow())
    }

    async fn reopen(&self) {
        // THE DEFECT: nothing was ever written to a durable medium, so there is
        // nothing to replay.
        *self.live.borrow_mut() = Rc::new(RefCell::new(Log::new(dense)));
    }
}

impl Subject for LosingFixture {
    const NAME: &'static str = "LosingFixture";

    fn open() -> Self {
        Self {
            live: RefCell::new(Rc::new(RefCell::new(Log::new(dense)))),
        }
    }
}

/// A store whose schema has no `recorded_at` column, so a reopen re-stamps.
///
/// [`LosingFixture`]'s opposite, and reading the two together is what makes
/// either legible: that one loses **everything** across a reopen, this one loses
/// **exactly one field**. Every event still reads back, at the position the store
/// assigned it, under the identity it was minted with; only the one clock reading
/// whose provenance the log itself attested is silently replaced.
///
/// # The defect somebody would ship
///
/// A migration that stores payload, type and tags and nothing else, so `open`
/// reconstructs `SequencedEvent`s by replaying rows and stamping them at open
/// time. It is a natural first schema — `recorded_at` reads like metadata until
/// somebody has to answer *when did this happen* from the log rather than from a
/// backup — and it is the exact mirror of what `happenstance-sqlite`'s migration
/// 1 does instead: persist the column and **read it back**, never re-derive it.
///
/// # Why it is written longhand rather than as a [`Defect`] step
///
/// Same reason as [`LosingFixture`]: `Defect` is a trait of *store* steps, and
/// reopen is a *fixture* operation. There is no step to override.
///
/// # Why the new stamp is a generation counter and not a clock
///
/// A naive re-stamp is **invisible in this binary**. [`correct::stamp`] spends
/// the constant [`correct::TEST_RECORDED_AT`] — fixed rather than read from a
/// clock, because CF-33 forbids one — so a replay that re-stamps through the
/// correct path lands on the same value it replaced and nothing can see it.
/// `GappedPositionStore`'s doc comment already records exactly this: it restamps
/// on replay and passes `recorded_time_survives_a_reopen` anyway.
///
/// So the new value is derived from a per-fixture **reopen generation**:
/// deterministic, so the harness stays reproducible run to run; strictly
/// monotone, so the value is *never* equal to the one it replaced; and not a
/// clock, so CF-33 is untouched and the row cannot go flaky on a fast machine.
/// A wall-clock stamp would fail on both counts — millisecond resolution makes
/// "the two stamps differ" a race this test would lose intermittently.
#[derive(Debug)]
pub(crate) struct RestampingFixture {
    /// Replaced wholesale by `reopen` with a log replayed out of the same
    /// events, exactly as a real reopen replaces a connection.
    live: RefCell<Rc<RefCell<Log>>>,
    /// How many times this fixture has been reopened. The stamp is a function of
    /// it, which is what makes the new value differ from the old one *by
    /// construction* rather than by luck.
    generation: Cell<i64>,
}

impl Fixture for RestampingFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.live.borrow())
    }

    async fn reopen(&self) {
        let generation = self.generation.get() + 1;
        self.generation.set(generation);

        // The durable medium, read back the way a replay reads it. Everything
        // is here: payloads, types, tags, positions and identities.
        let held = {
            let live = self.live.borrow();
            let log = live.borrow();
            log.select(&Query::all(), ReadOptions::new())
        };

        // THE DEFECT: every store-assigned fact is restored except the stamp,
        // which is recomputed at open time because no column held it.
        //
        // The generation is what makes "recomputed" *observable* in a binary
        // whose correct clock is a constant. Written the naive way —
        // `with_recorded_at(correct::TEST_RECORDED_AT)` — this fixture passes
        // `recorded_time_survives_a_reopen` perfectly, and
        // `mutants_fail_exactly_their_declared_rules` reports it as *"declares
        // that it fails … but the rule passed"*. That run happened; this line is
        // its answer.
        let restamped = RecordedAt::from_millis(correct::TEST_RECORDED_AT.as_millis() + generation);
        let replayed = held
            .into_iter()
            .map(|event| event.with_recorded_at(restamped))
            .collect();

        *self.live.borrow_mut() = Rc::new(RefCell::new(Log::replayed(dense, replayed)));
    }
}

impl Subject for RestampingFixture {
    const NAME: &'static str = "RestampingFixture";

    fn open() -> Self {
        Self {
            live: RefCell::new(Rc::new(RefCell::new(Log::new(dense)))),
            generation: Cell::new(0),
        }
    }
}

thread_local! {
    /// The one backing log every live [`SharedBackingFixture`] instance shares.
    ///
    /// A `Weak`, and that is the mechanism rather than a refinement. A strong
    /// static would leak state from one rule into the next and make this mutant
    /// fail rules it does not declare; a `Weak` is upgraded only while some
    /// fixture still holds the log, so two instances alive *at the same time*
    /// share — which is the defect — and the next rule, opening its first
    /// instance after the previous rule's have dropped, gets a fresh one. That
    /// is also exactly how the real mistake behaves: a temporary directory keyed
    /// by process rather than by instance.
    static SHARED: RefCell<Weak<RefCell<Log>>> = const { RefCell::new(Weak::new()) };
}

/// Two fixture instances that are one backing store.
///
/// The store is completely correct. The mistake is an adapter's, and it is the
/// one CF-15 names: a file-backed fixture that points every instance at one
/// temporary path.
#[derive(Debug)]
pub(crate) struct SharedBackingFixture(Rc<RefCell<Log>>);

impl Fixture for SharedBackingFixture {
    type Store = LogStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the fixture-isolation axis only");

    async fn connect(&self) -> Self::Store {
        LogStore::over(&self.0)
    }
}

impl Subject for SharedBackingFixture {
    const NAME: &'static str = "SharedBackingFixture";

    fn open() -> Self {
        Self(SHARED.with(|shared| {
            let mut shared = shared.borrow_mut();
            if let Some(existing) = shared.upgrade() {
                return existing;
            }
            let fresh = Rc::new(RefCell::new(Log::new(dense)));
            *shared = Rc::downgrade(&fresh);
            fresh
        }))
    }
}

/// Suspends exactly once: `Pending` on the first poll, `Ready` on the second.
///
/// Stands in for the transaction's own work — `SqlStorage::exec(..).await` in a
/// Durable Object, a round trip to Postgres — and it is the only future in this
/// binary that is not ready on its first poll.
///
/// The `wake_by_ref` before `Pending` is `variants.rs`'s `PagedStream`
/// discipline, and it is load-bearing for the same reason: a future that
/// returns `Pending` without waking hangs the whole binary with no message, and
/// `harness.rs` explains why there is deliberately no watchdog to rescue it.
struct YieldOnce(bool);

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            return Poll::Ready(());
        }
        self.0 = true;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

/// Positions allocated **before** the transaction commits.
///
/// A writer takes its number from a sequence, does its work, and publishes its
/// row at commit. A writer that took a *low* number and commits *late*
/// therefore becomes visible underneath a position a reader has already
/// observed, and `AppendCondition::after` — which compares position values —
/// stops enforcing the boundary it exists to enforce, with no error anywhere.
///
/// # Why it is not a [`Defect`]
///
/// A sharper reason than the other three longhand entries. [`Defect::commit`]
/// is a **synchronous** function, so the shape of this defect — a *window*
/// between allocating and publishing — has nowhere to live in it. Making
/// `commit` async to accommodate one store would put a suspension point into
/// all twenty-two of the one-step mutants, every one of which is correct
/// precisely because it has none.
///
/// # This is a faithful model of a real database, not an implementation slip
///
/// It is what Postgres does by default: `nextval()` is non-transactional, which
/// is also why a rollback does not give the number back and why gaps are legal.
/// Every other mutant in this binary is somebody's mistake; this one is
/// somebody's *database*, and `happenstance-postgres` sits in the workspace to
/// occupy this end of the position-allocation axis.
///
/// Everything except the window is `crate::correct`'s, in `commit_with`'s own
/// order — emptiness first, then the condition (ES-20) — so a rule that goes red
/// here went red for the visibility defect and nothing else.
#[derive(Debug)]
pub(crate) struct PreCommitPositionStore {
    /// The rows a reader can see. A row lands here at commit, and not before.
    committed: Rc<RefCell<Vec<SequencedEvent>>>,
    /// THE DEFECT: the sequence, shared by every handle and advanced *outside*
    /// the transaction that will publish the row.
    sequence: Rc<Cell<Option<SequencePosition>>>,
}

impl EventStore for PreCommitPositionStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Snapshot::new(
            self.committed
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| correct::select(&stored, query, options)),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first (ES-20), then the condition — `commit_with`'s own
        // order, so that nothing here is a second, undeclared defect.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // The probe runs against what is *committed*, which is correct: this
        // store's defect is when a row becomes visible, not what its condition
        // can see. The borrow is scoped to this block so that none is held
        // across the suspension below — `clippy::await_holding_refcell_ref` is
        // a workspace deny and would otherwise fire, and holding one here would
        // give the store a second defect nobody declared.
        {
            let stored = self
                .committed
                .try_borrow()
                .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
            if let Some(condition) = condition
                && let Some(conflict) = correct::violation(&stored, condition)
            {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict,
                )));
            }
        }

        // THE DEFECT, first half: the position is taken here, from a sequence
        // that knows nothing about the transaction. Note what it is *not* taken
        // from — the committed rows — which is what makes the number stable
        // across the window.
        let sequenced = correct::sequence(events, self.sequence.get(), dense);
        let last = sequenced
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        self.sequence.set(Some(last));

        // The transaction doing its work, and committing. One suspension, and
        // it is the whole window: driven to completion in one go — which is
        // what every other rule in the suite does — this store is
        // indistinguishable from a correct one.
        YieldOnce(false).await;

        // THE DEFECT, second half: the rows become visible now, which may be
        // after a later-positioned transaction has already published its own.
        self.committed
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?
            .extend(sequenced);

        Ok(last)
    }

    // Over the committed rows, not over `self.sequence`. The sequence holds a
    // number that has been *allocated*, which is precisely the value no reader is
    // entitled to see yet; answering with it would make the visibility window
    // observable through a second door and give the store a defect it does not
    // declare. Postgres' own `currval()` is the same distinction.
    //
    // And `max`, not `correct::head_of`, which is this store's one departure from
    // the shared core and needs its reason on the page. `head_of` returns the
    // *last* element, which is the highest only where positions ascend — and
    // rows arriving out of position order is this store's entire declared defect.
    // Every committed row here is visible (there is no frontier predicate, so
    // committed and visible are the same set), so "the highest position currently
    // visible" is the maximum. Answering `last()` would make `head` lag a row a
    // reader can already see, which is a *second* defect on top of the declared
    // one — and `mutants_fail_exactly_their_declared_rules` would then be
    // reporting the instrument rather than the implementation.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let stored = self
            .committed
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(stored.iter().map(|event| event.position).max())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let stored = self
            .committed
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::contains(&stored, id))
    }
}

/// One sequence and one set of committed rows, shared by every handle.
#[derive(Debug)]
pub(crate) struct PreCommitPositionFixture {
    committed: Rc<RefCell<Vec<SequencedEvent>>>,
    sequence: Rc<Cell<Option<SequencePosition>>>,
}

impl Fixture for PreCommitPositionFixture {
    type Store = PreCommitPositionStore;

    // Genuinely supported, and it has to be: the sequence is shared, so two
    // handles allocate from one counter exactly as two Postgres sessions do.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the position-visibility axis only");

    async fn connect(&self) -> Self::Store {
        PreCommitPositionStore {
            committed: Rc::clone(&self.committed),
            sequence: Rc::clone(&self.sequence),
        }
    }
}

impl Subject for PreCommitPositionFixture {
    const NAME: &'static str = "PreCommitPositionStore";

    fn open() -> Self {
        Self {
            committed: Rc::new(RefCell::new(Vec::new())),
            sequence: Rc::new(Cell::new(None)),
        }
    }
}

/// A `read` stream that keeps a borrow of the store alive.
///
/// The stream yields rows from a live cursor rather than from a snapshot, so it
/// holds the store for as long as the caller holds it — a rusqlite adapter
/// streaming from an open statement, an `Rc`-shared cursor, a pooled adapter
/// that checked a connection out in `read` and returns it in `Drop`. This
/// type-checks, and that is the finding rather than an aside: RPITIT lets an
/// implementer return a stream that borrows from `&self`, so the **port cannot
/// express** "your stream must not hold a borrow" and only a rule will catch it.
///
/// # Why it is not a [`Defect`]
///
/// Every one of the seven steps is correct here. What is wrong is the *lifetime*
/// of the value `read` returns, and `Defect` composes functions over slices —
/// there is nowhere in it to say "and the result borrows the log".
///
/// # It panics here and **deadlocks** in production
///
/// The asymmetry is worth carrying, because it is the one place in this
/// catalogue where a real adapter behaves worse than the mutant. A `RefCell`
/// answers a conflicting borrow immediately, with a message. A pooled SQL
/// adapter holding its only connection answers by waiting, and the conformance
/// run reports a CI timeout that names no rule. That is why the workspace's
/// instrument for the shape is a `RefCell` and not a pool.
#[derive(Debug)]
pub(crate) struct BorrowHoldingStore(Rc<RefCell<Vec<SequencedEvent>>>);

/// The stream [`BorrowHoldingStore`] returns: a live borrow plus a cursor.
#[derive(Debug)]
pub(crate) struct BorrowingStream<'a> {
    borrowed: Ref<'a, Vec<SequencedEvent>>,
    positions: std::vec::IntoIter<SequencePosition>,
}

impl Stream for BorrowingStream<'_> {
    type Item = Result<SequencedEvent, LogError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let next = this.positions.next();
        Poll::Ready(next.and_then(|position| {
            this.borrowed
                .iter()
                .find(|event| event.position == position)
                .cloned()
                .map(Ok)
        }))
    }
}

impl EventStore for BorrowHoldingStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // `borrow`, not `try_borrow`: an adapter that models its failure mode in
        // `Self::Error` would have to have noticed it had one.
        let borrowed = self.0.borrow();
        // Positions rather than indices, so the selection is `correct::select`'s
        // verbatim and the only difference from a conformant store is what the
        // stream holds.
        let positions: Vec<SequencePosition> = correct::select(&borrowed, query, options)
            .iter()
            .map(|event| event.position)
            .collect();
        BorrowingStream {
            borrowed,
            positions: positions.into_iter(),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        correct::commit(&mut self.0.borrow_mut(), events, condition, dense)
    }

    // `borrow`, like the two methods above, and the borrow is released before
    // either returns — this store's declared defect is the lifetime of what
    // `read` hands back, so a shared borrow taken and dropped here is
    // indistinguishable from a conformant store's.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(correct::head_of(&self.0.borrow()))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(correct::contains(&self.0.borrow(), id))
    }
}

/// One log, and handles that hand out borrow-holding streams.
#[derive(Debug)]
pub(crate) struct BorrowHoldingFixture(Rc<RefCell<Vec<SequencedEvent>>>);

impl Fixture for BorrowHoldingFixture {
    type Store = BorrowHoldingStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the re-entrancy axis only");

    async fn connect(&self) -> Self::Store {
        BorrowHoldingStore(Rc::clone(&self.0))
    }
}

impl Subject for BorrowHoldingFixture {
    const NAME: &'static str = "BorrowHoldingStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }
}

/// An `append` that holds the exclusive borrow **across an `.await`**.
///
/// The Durable Object shape: take the store, `SqlStorage::exec(..).await`, write.
/// It is correct for as long as nothing re-enters the store, which is every
/// sequential test anyone writes.
///
/// # Why it is not a [`Defect`]
///
/// The same reason as [`PreCommitPositionStore`]: [`Defect::commit`] is
/// synchronous, and this defect *is* a suspension point. Making `commit` async
/// to accommodate it would put one into every one-step mutant, all of which are
/// correct precisely because they have none.
///
/// # Two findings ride on the `allow`, and both are worth keeping
///
/// `clippy::await_holding_refcell_ref` catches this **statically**, and it fires
/// under this workspace's own `-D warnings` — so for any adapter that adopts the
/// same lint policy the defect never reaches a test. But it is `warn`-by-default,
/// so a downstream adapter on stock settings gets a warning it can ignore, which
/// is why the runtime rule still earns its place. And the lint does not catch
/// [`BorrowHoldingStore`] at all, which is the sibling defect and needs the other
/// rule.
#[derive(Debug)]
pub(crate) struct AwaitAcrossBorrowStore(Rc<RefCell<Vec<SequencedEvent>>>);

impl EventStore for AwaitAcrossBorrowStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // `borrow`, matching `append` below: this store's whole subject is what
        // happens when two operations overlap, and reporting one of them through
        // `Self::Error` while the other panics would split its single defect
        // across two failure modes.
        Snapshot::new(Ok(correct::select(&self.0.borrow(), query, options)))
    }

    #[allow(clippy::await_holding_refcell_ref)]
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // THE DEFECT: the borrow is taken, and then the future suspends.
        let mut stored = self.0.borrow_mut();
        YieldOnce(false).await;
        correct::commit(&mut stored, events, condition, dense)
    }

    // No suspension point, so no borrow is held across one. The defect this store
    // exists for is `append`'s alone; a second `.await` under a live borrow here
    // would make it fail the re-entrancy rules twice over and pin neither failure
    // to the declared half.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(correct::head_of(&self.0.borrow()))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(correct::contains(&self.0.borrow(), id))
    }
}

/// One log, and handles whose appends suspend while holding it.
#[derive(Debug)]
pub(crate) struct AwaitAcrossBorrowFixture(Rc<RefCell<Vec<SequencedEvent>>>);

impl Fixture for AwaitAcrossBorrowFixture {
    type Store = AwaitAcrossBorrowStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the re-entrancy axis only");

    async fn connect(&self) -> Self::Store {
        AwaitAcrossBorrowStore(Rc::clone(&self.0))
    }
}

impl Subject for AwaitAcrossBorrowFixture {
    const NAME: &'static str = "AwaitAcrossBorrowStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }
}
/// A `read` that takes a **new sample per page**.
///
/// The self-paginating adapter ES-11's `Rejects:` names, and the natural shape
/// for a transport with no cursor: `happenstance-neon` reaches Postgres over
/// one-shot HTTP, gets one buffered JSON document per round trip under a 64 MiB
/// cap, and has no way to hold a statement open across polls. Above the cap the
/// only way to answer at all is an independent
/// `WHERE position > $last ORDER BY position LIMIT n` per chunk — and each of
/// those is a fresh snapshot. The result grows under the caller's feet.
///
/// ES-11's prescribed fix is a **position ceiling**: capture *H* no later than
/// the first poll and bound every later statement by `position <= H`. This store
/// is that adapter with the ceiling missing, which is one line rather than a
/// redesign, and is why the clause makes the ceiling a MUST rather than advice.
///
/// # Why the page is one event
///
/// A page size is a deployment constant; the defect is independent of it, and
/// only the *number of events* needed to observe it depends on it. Modelling a
/// 5,000-row page would mean seeding 5,001 events into every rule that had to
/// catch this store — which would make the rules slow and would make the
/// arrangement, rather than the defect, the thing a reader has to understand. A
/// page of one puts the tear at every item.
///
/// # Why it is not a [`Defect`]
///
/// Every step it uses is `crate::correct`'s, and it uses them correctly. What is
/// wrong is *when* it uses them: [`Defect`] composes functions over a slice that
/// `MutantStore::read` samples once, and there is nowhere in it to say "and this
/// is evaluated again on the next poll". It is `BorrowHoldingStore`'s reason
/// with the sign reversed — that store's stream holds too much, this one holds
/// too little.
///
/// # What it is *not*
///
/// It is not a re-entrancy defect. The stream owns a refcount rather than a
/// borrow, so an append while it is alive is answered rather than refused, and
/// `a_live_read_stream_does_not_block_an_append` passes. The only thing it gets
/// wrong is the sample.
#[derive(Debug)]
pub(crate) struct RefetchingPagedStore(Rc<RefCell<Vec<SequencedEvent>>>);

/// The stream [`RefetchingPagedStore`] returns: a query, a cursor, and no
/// snapshot.
///
/// It borrows the query and not the store — `read`'s RPITIT captures the query's
/// lifetime, which is ES-13's whole subject, and this is what an adapter that
/// keeps the caller's `&Query` to rebuild its next statement from looks like.
#[derive(Debug)]
pub(crate) struct RefetchingStream<'a> {
    log: Rc<RefCell<Vec<SequencedEvent>>>,
    query: &'a Query,
    options: ReadOptions,
    /// How many events this stream has already yielded — the cursor the next
    /// statement resumes from.
    yielded: usize,
}

impl Stream for RefetchingStream<'_> {
    type Item = Result<SequencedEvent, LogError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // THE DEFECT: an independent statement per page, against whatever the
        // store holds *now*. Every step of the selection is `correct`'s; what is
        // wrong is that it is taken again.
        let page = {
            let Ok(stored) = this.log.try_borrow() else {
                return Poll::Ready(Some(Err(LogError::AlreadyBorrowed)));
            };
            correct::select(&stored, this.query, this.options)
        };

        match page.get(this.yielded) {
            Some(event) => {
                let event = event.clone();
                this.yielded += 1;
                Poll::Ready(Some(Ok(event)))
            }
            None => Poll::Ready(None),
        }
    }
}

impl EventStore for RefetchingPagedStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // A refcount, not a borrow: this store's declared defect is the sample,
        // and holding the log open would give it `BorrowHoldingStore`'s defect
        // as well.
        RefetchingStream {
            log: Rc::clone(&self.0),
            query,
            options,
            yielded: 0,
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let mut stored = self
            .0
            .try_borrow_mut()
            .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
        correct::commit(&mut stored, events, condition, dense)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let stored = self.0.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::head_of(&stored))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let stored = self.0.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
        Ok(correct::contains(&stored, id))
    }
}

/// One log, and handles whose read streams re-sample it on every poll.
#[derive(Debug)]
pub(crate) struct RefetchingPagedFixture(Rc<RefCell<Vec<SequencedEvent>>>);

impl Fixture for RefetchingPagedFixture {
    type Store = RefetchingPagedStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability =
        Capability::declined("this instrument exists for the read-isolation axis only");

    async fn connect(&self) -> Self::Store {
        RefetchingPagedStore(Rc::clone(&self.0))
    }
}

impl Subject for RefetchingPagedFixture {
    const NAME: &'static str = "RefetchingPagedStore";

    fn open() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }
}
