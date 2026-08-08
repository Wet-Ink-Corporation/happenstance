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
//! [`Defect::select`] is the seventh step and the newest, and it is the one to
//! read before adding an eighth: it exists because two real defects couple a
//! *filter* to a *read option*, and `matching` is handed no options while
//! `ordered` is handed no query. Its documentation says which two and why the
//! seam is not the default place to put a defect.
//!
//! Six stores cannot be expressed that way and are written out longhand, each
//! for a stated reason: [`CachedHeadFixture`] because its defect is per *handle*
//! rather than per store, [`LosingFixture`] because its defect is what a reopen
//! finds, [`SharedBackingFixture`] because its defect is that two fixture
//! instances are one, [`PreCommitPositionStore`] and [`AwaitAcrossBorrowStore`]
//! because each defect is a *window* — a suspension between two halves of an
//! append — and [`Defect::commit`] is a synchronous function with nowhere to put
//! one, and [`BorrowHoldingStore`] because its defect is the *lifetime* of the
//! value `read` returns rather than anything a step computes.
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
    AppendCondition, AppendError, ConditionViolated, Event, EventStore, EventType, Query,
    QueryItem, ReadOptions, SequencePosition, SequencedEvent, Tag, Tags,
};
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
        events
            .iter()
            .find(|event| {
                condition.after.is_none_or(|after| event.position > after)
                    && Self::query_matches(&condition.fail_if_events_match, &known, event)
            })
            .map(|event| event.position)
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
        } else {
            sorted.retain(|event| options.from.is_none_or(|from| event.position >= from));
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
        let directed: Vec<&SequencedEvent> = if options.backwards {
            matched.into_iter().rev().collect()
        } else {
            matched
        };
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
            && let Some(after) = condition.after
            && stored.last().is_none_or(|event| event.position < after)
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
// Append conditions — the probe
// =====================================================================

/// `condition.after.unwrap_or(FIRST)`, so an event at the first position never
/// violates.
pub(crate) struct AfterDefaultsToFirstStore;

impl Defect for AfterDefaultsToFirstStore {
    const NAME: &'static str = "AfterDefaultsToFirstStore";

    fn violation(
        events: &[SequencedEvent],
        condition: &AppendCondition,
    ) -> Option<SequencePosition> {
        // The `Option` collapsed at the boundary because the SQL wanted a value.
        let after = condition.after.unwrap_or(SequencePosition::FIRST);
        events
            .iter()
            .find(|event| {
                event.position > after
                    && condition
                        .fail_if_events_match
                        .matches(event.event_type(), event.tags())
            })
            .map(|event| event.position)
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
        events
            .iter()
            .find(|event| condition.after.is_none_or(|after| event.position > after))
            .map(|event| event.position)
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
        events
            .iter()
            .find(|event| {
                condition.after.is_none_or(|after| event.position >= after)
                    && condition
                        .fail_if_events_match
                        .matches(event.event_type(), event.tags())
            })
            .map(|event| event.position)
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
        let skip = condition.after.map_or(0, |after| {
            usize::try_from(after.get()).unwrap_or(usize::MAX)
        });
        events
            .iter()
            .filter(|event| {
                condition
                    .fail_if_events_match
                    .matches(event.event_type(), event.tags())
            })
            .nth(skip)
            .map(|event| event.position)
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
        events
            .iter()
            .find(|event| {
                condition.after.is_none_or(|after| event.position > after)
                    && exact_tag_match(&condition.fail_if_events_match, event)
            })
            .map(|event| event.position)
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
        events
            .iter()
            .find(|event| {
                condition.after.is_none_or(|after| event.position > after)
                    && type_only_match(&condition.fail_if_events_match, event)
            })
            .map(|event| event.position)
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
                .fail_if_events_match
                .matches(event.event_type(), event.tags())
        })?;
        let head = events.last().map(|event| event.position)?;
        // THE DEFECT: the two questions are asked of the whole store rather than
        // of one event.
        let moved_on = condition.after.is_none_or(|after| head > after);
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

// =====================================================================
// The six that cannot be one step
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
