//! Every write-group rule against the wrong stores it names.
//!
//! A rule that no store can fail is decorative. Each wrong store here is the
//! in-memory oracle with one plausible mistake made in how it reads its groups
//! back, and each test runs the **whole** enumeration against it and asserts
//! the exact set of rules that report it. A mistake in grouping shows on most
//! arrangements, so most sets hold more than one rule; the set is pinned
//! whole so a rule that stops seeing its mistake turns a test red.
//!
//! The faithful store is the control: the same plumbing with no mistake passes
//! every rule, so a red result below is the mistake's and not the plumbing's.
//!
//! The mistakes all live in `read_groups`. Writes go to the oracle untouched;
//! a read takes the oracle's log and its groups whole, as rows that each know
//! their group's first position and its guard, bends the rows or the paging,
//! and pages the result. That is what a real store's `read_groups` does over
//! its own columns, which is where each of these mistakes would be made.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeMap;
use std::future::Future;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryStoreError, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent, StoreId, collect,
};
use happenstance_sync::write_group::{GroupOrderError, GroupPageBuilder};
use happenstance_sync::{
    EventGroup, GroupPage, IngestError, IngestGroup, Ingested, ReplicatedEvent, SendIngestStore,
    SendWriteGroupStore, Waiting, Watermark,
};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, MemoryIngestHandle, write_group_rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a [`WrongHandle`] makes when it reads its groups back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// Records no groups: every row reads back as a group of its own, as a
    /// legacy row does, with no guard.
    EveryRowItsOwnGroup,
    /// Records one group for the whole log, so separate writes run together.
    MergesAdjacentGroups,
    /// Records the groups but not the guard an ingested group arrived under.
    DropsGuard,
    /// Records a compensation as a group of its own, apart from the losing
    /// event it answers (SY-2).
    CompensationApart,
    /// Fills a page by event, cutting the group that crosses `max_events`.
    CutsAtMaxEvents,
    /// Skips a first group larger than the page rather than returning it
    /// whole, so a pull past it would never see it.
    SkipsAnOversizedGroup,
    /// Resumes above a row position rather than a group's first, which hands a
    /// group's tail back as if it were a group.
    ResumesByRowPosition,
    /// Says nothing is waiting on every page, including one its budget cut.
    SaysNothingWhenCut,
    /// Names an ingested group by the first position it **allocated** rather
    /// than the first row it wrote: a sequence that burns one value per
    /// skipped re-delivery, as Postgres's does, so a group whose head was
    /// already held is named that many positions below its first row.
    NamesByFirstAllocated,
}

/// For [`Mistake::NamesByFirstAllocated`]: each group's first new event, and
/// how many re-delivered events ahead of it burnt a position.
///
/// Shared by every handle onto one fixture, as the sequence would be, and
/// mutated by ingests through `&self`: `Arc<Mutex<_>>` because the handles are
/// `Send` and cloned, and the lock is never held across an await (OWN-11).
type Burnt = Arc<Mutex<BTreeMap<EventId, u64>>>;

/// A fixture over a fresh oracle, whose handles make one mistake.
#[derive(Debug)]
struct WrongFixture {
    inner: MemoryIngestFixture,
    mistake: Mistake,
    burnt: Burnt,
}

impl WrongFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            inner: MemoryIngestFixture::new(),
            mistake,
            burnt: Burnt::default(),
        }
    }
}

impl IngestFixture for WrongFixture {
    type Store = WrongHandle;

    async fn connect(&self) -> Self::Store {
        WrongHandle {
            inner: self.inner.connect().await,
            mistake: self.mistake,
            burnt: Arc::clone(&self.burnt),
        }
    }
}

/// The oracle's handle, with a mistaken `read_groups`.
#[derive(Debug, Clone)]
struct WrongHandle {
    inner: MemoryIngestHandle,
    mistake: Mistake,
    burnt: Burnt,
}

impl SendEventStore for WrongHandle {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        SendEventStore::read(&self.inner, query, options)
    }

    fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> impl Future<Output = Result<SequencePosition, AppendError<Self::Error>>> + Send {
        SendEventStore::append(&self.inner, events, condition)
    }

    fn head(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>> + Send {
        SendEventStore::head(&self.inner)
    }

    fn contains_event_id(
        &self,
        id: EventId,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        SendEventStore::contains_event_id(&self.inner, id)
    }
}

impl SendIngestStore for WrongHandle {
    type Error = MemoryStoreError;

    fn store_id(&self) -> impl Future<Output = Result<StoreId, Self::Error>> + Send {
        SendIngestStore::store_id(&self.inner)
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        if self.mistake == Mistake::NamesByFirstAllocated {
            for group in groups {
                // The re-delivered events ahead of the first new one: each
                // would have drawn, and burnt, a value from the sequence.
                for (skipped, event) in (0_u64..).zip(group.events) {
                    let held = SendEventStore::contains_event_id(&self.inner, event.id)
                        .await
                        .map_err(IngestError::Store)?;
                    if !held {
                        if skipped > 0 {
                            self.burnt
                                .lock()
                                .expect("no ingest panics holding the lock")
                                .insert(event.id, skipped);
                        }
                        break;
                    }
                }
            }
        }
        SendIngestStore::ingest(&self.inner, groups).await
    }

    fn watermark(&self) -> impl Future<Output = Result<Watermark, Self::Error>> + Send {
        SendIngestStore::watermark(&self.inner)
    }
}

/// One row of the log, as a store with a group column holds it.
struct Row {
    position: SequencePosition,
    first: SequencePosition,
    guard: Option<AppendCondition>,
    event: ReplicatedEvent,
}

impl SendWriteGroupStore for WrongHandle {
    type Error = GroupOrderError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        let rows = self.rows().await?;
        let rows = {
            let burnt = self
                .burnt
                .lock()
                .expect("no ingest panics holding the lock");
            self.mistake.bend(rows, &burnt)
        };
        self.mistake.page(rows, after, max_events)
    }
}

impl WrongHandle {
    /// The oracle's whole log, each row knowing its group's first position,
    /// and the group's head carrying its guard.
    async fn rows(&self) -> Result<Vec<Row>, GroupOrderError> {
        let log = collect(SendEventStore::read(
            &self.inner,
            &Query::all(),
            ReadOptions::new(),
        ))
        .await
        .map_err(|never: MemoryStoreError| match never {})?;
        let truth = SendWriteGroupStore::read_groups(&self.inner, None, NonZeroUsize::MAX).await?;
        // The guards are moved out of the oracle's page, not copied.
        let mut guards: BTreeMap<SequencePosition, AppendCondition> = BTreeMap::new();
        let mut member: Vec<SequencePosition> = Vec::with_capacity(log.len());
        for group in truth.groups {
            let first = group.first();
            let len = group.events().len();
            if let Some(guard) = EventGroup::from(group).guard {
                guards.insert(first, guard);
            }
            member.extend(std::iter::repeat_n(first, len));
        }
        Ok(log
            .into_iter()
            .zip(member)
            .map(|(row, first)| {
                let guard = if row.position == first {
                    guards.remove(&first)
                } else {
                    None
                };
                Row {
                    position: row.position,
                    first,
                    guard,
                    event: ReplicatedEvent::new(row.id, row.recorded_at, row.event),
                }
            })
            .collect())
    }
}

impl Mistake {
    /// The rows as a store with this mistake recorded them. `burnt` is what
    /// [`Mistake::NamesByFirstAllocated`]'s ingests recorded.
    fn bend(self, mut rows: Vec<Row>, burnt: &BTreeMap<EventId, u64>) -> Vec<Row> {
        match self {
            Self::NamesByFirstAllocated => {
                // (the group's true first, the position it is named by)
                let mut renamed: Option<(SequencePosition, SequencePosition)> = None;
                for row in &mut rows {
                    if row.position == row.first {
                        renamed = burnt
                            .get(&row.event.id)
                            .and_then(|&skipped| row.position.get().checked_sub(skipped))
                            .and_then(SequencePosition::new)
                            .map(|named| (row.first, named));
                    }
                    if let Some((truth, named)) = renamed
                        && truth == row.first
                    {
                        row.first = named;
                    }
                }
            }
            Self::EveryRowItsOwnGroup => {
                for row in &mut rows {
                    row.first = row.position;
                    row.guard = None;
                }
            }
            Self::MergesAdjacentGroups => {
                if let Some(head) = rows.first().map(|row| row.position) {
                    for row in &mut rows {
                        row.first = head;
                    }
                }
            }
            Self::DropsGuard => {
                for row in &mut rows {
                    row.guard = None;
                }
            }
            Self::CompensationApart => {
                // A row minted by another store than its group's head is the
                // compensation the head's group was answered with.
                let mut head: Option<(SequencePosition, StoreId)> = None;
                let mut apart: Option<SequencePosition> = None;
                for row in &mut rows {
                    let origin = row.event.id.store();
                    match head {
                        Some((first, head_origin)) if first == row.first => {
                            if origin != head_origin {
                                let start = *apart.get_or_insert(row.position);
                                row.first = start;
                            }
                        }
                        _ => {
                            head = Some((row.first, origin));
                            apart = None;
                        }
                    }
                }
            }
            Self::Faithful
            | Self::CutsAtMaxEvents
            | Self::SkipsAnOversizedGroup
            | Self::ResumesByRowPosition
            | Self::SaysNothingWhenCut => {}
        }
        rows
    }

    /// One page of `rows` above `after`, as a store with this mistake pages.
    fn page(
        self,
        rows: Vec<Row>,
        after: Option<SequencePosition>,
        max: NonZeroUsize,
    ) -> Result<GroupPage, GroupOrderError> {
        let mut above: Vec<Row> = rows
            .into_iter()
            .filter(|row| {
                let key = if self == Self::ResumesByRowPosition {
                    row.position
                } else {
                    row.first
                };
                after.is_none_or(|after| key > after)
            })
            .collect();
        if self == Self::CutsAtMaxEvents {
            let waiting = if above.len() > max.get() {
                Waiting::Unknown
            } else {
                Waiting::Nothing
            };
            above.truncate(max.get());
            return offer_all(above, NonZeroUsize::MAX, waiting);
        }
        if self == Self::SkipsAnOversizedGroup
            && let Some(head) = above.first().map(|row| row.first)
        {
            let size = above.iter().filter(|row| row.first == head).count();
            if size > max.get() {
                above.retain(|row| row.first != head);
            }
        }
        let page = offer_all(above, max, Waiting::Nothing)?;
        if self == Self::SaysNothingWhenCut {
            // The builder says `Unknown` for a page it cut; this store
            // overrules it.
            return Ok(GroupPage::new(page.groups, Waiting::Nothing));
        }
        Ok(page)
    }
}

/// Every row in one page, finished with `waiting`.
fn offer_all(
    rows: Vec<Row>,
    max: NonZeroUsize,
    waiting: Waiting,
) -> Result<GroupPage, GroupOrderError> {
    let mut page = GroupPageBuilder::new(max);
    for row in rows {
        if page.offer(row.first, row.guard, row.event).is_break() {
            return page.finish(Waiting::Unknown);
        }
    }
    page.finish(waiting)
}

/// Every rule in the enumeration against one mistake, by name, each verdict
/// reduced to the clause it reports.
fn verdicts(mistake: Mistake) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)> {
    let open = async || WrongFixture::new(mistake);
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$((
                stringify!($name),
                block_on(write_group_rules::$name(open)).map_err(|violation| violation.clause()),
            )),*]
        };
    }
    happenstance_sync_testkit::for_each_write_group_rule!(run_each)
}

/// The rules in `failing` report SY-39, and every other rule passes.
fn fails_exactly(mistake: Mistake, failing: &[&str]) {
    let all = verdicts(mistake);
    for rule in failing {
        assert!(
            all.iter().any(|(name, _)| name == rule),
            "`{rule}` is not in the enumeration: {all:?}"
        );
    }
    for (name, verdict) in all {
        if failing.contains(&name) {
            assert_eq!(verdict, Err("SY-39"), "`{name}` must reject {mistake:?}");
        } else {
            assert_eq!(
                verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` failed {mistake:?}, which is pinned to {failing:?}"
            );
        }
    }
}

const APPEND: &str = "an_append_reads_back_as_one_group";
const INGEST: &str = "an_ingested_group_keeps_its_boundary_and_guard";
const COMPENSATION: &str = "compensation_joins_its_group";
const REDELIVERY: &str = "a_group_after_a_redelivery_is_named_by_its_first_landed_row";
const NEVER_CUTS: &str = "a_read_never_cuts_a_group";
const CHAIN: &str = "pages_chain_on_first_and_end_with_nothing";

#[test]
fn the_faithful_store_passes_every_rule() {
    fails_exactly(Mistake::Faithful, &[]);
}

#[test]
fn a_store_that_records_no_groups_fails_every_rule() {
    fails_exactly(
        Mistake::EveryRowItsOwnGroup,
        &[APPEND, INGEST, COMPENSATION, REDELIVERY, NEVER_CUTS, CHAIN],
    );
}

#[test]
fn a_store_that_merges_its_writes_fails_the_boundary_and_paging_rules() {
    fails_exactly(
        Mistake::MergesAdjacentGroups,
        &[APPEND, INGEST, REDELIVERY, NEVER_CUTS, CHAIN],
    );
}

#[test]
fn a_store_that_drops_the_guard_fails_both_guard_rules() {
    fails_exactly(Mistake::DropsGuard, &[INGEST, REDELIVERY]);
}

#[test]
fn a_store_that_records_a_compensation_apart_fails_the_compensation_rule() {
    fails_exactly(Mistake::CompensationApart, &[COMPENSATION]);
}

#[test]
fn a_store_that_fills_a_page_by_event_fails_both_paging_rules() {
    fails_exactly(Mistake::CutsAtMaxEvents, &[NEVER_CUTS, CHAIN]);
}

#[test]
fn a_store_that_skips_an_oversized_group_fails_the_cut_rule() {
    fails_exactly(Mistake::SkipsAnOversizedGroup, &[NEVER_CUTS]);
}

#[test]
fn a_store_that_resumes_by_row_position_fails_both_paging_rules() {
    fails_exactly(Mistake::ResumesByRowPosition, &[NEVER_CUTS, CHAIN]);
}

#[test]
fn a_store_that_says_nothing_on_a_cut_page_fails_the_chain_rule() {
    fails_exactly(Mistake::SaysNothingWhenCut, &[CHAIN]);
}

#[test]
fn a_store_that_names_a_group_by_its_first_allocated_position_fails_the_redelivery_rule() {
    fails_exactly(Mistake::NamesByFirstAllocated, &[REDELIVERY]);
}
