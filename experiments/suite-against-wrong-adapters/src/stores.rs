//! The four wrong stores.
//!
//! Every one of them is a defect somebody would ship — that is the testkit's own
//! CF-4 bar, and it is what separates a mutant from a saboteur. Each names the
//! adapter shape it comes from and the review finding that named it.
//!
//! | Store | Finding | The shape it comes from |
//! |---|---|---|
//! | [`ForwardPagingBudgetStore`] | L1-1 | a forward read that branches on `from` and never threads `limit` into the resume branch |
//! | [`PlainStore`] under a no-op `reopen` | L1-2 | a fixture that declares `REOPEN` supported because "the pool handles reconnection" |
//! | [`SwallowedReadFaultStore`] | L3-01 | `let Ok(page) = fetch().await else { return Poll::Ready(None) };` |
//! | [`StagedCommitStore`] | F2-5 | an interactive transaction: one round trip per row, `COMMIT` at the end |
//!
//! # One defect per store
//!
//! Everything not named above is delegated to [`crate::correct`], which is the
//! testkit's own correct core. A rule that goes red against one of these went red
//! for the declared reason and not for a second bug nobody noticed writing — the
//! property the testkit's `mutants_fail_exactly_their_declared_rules` exists to
//! hold, borrowed here because the same hazard applies.
//!
//! # `const INJECTED: bool`
//!
//! The one deliberate exception to "one defect per store". Every store here takes
//! it, and `true` adds the testkit's `InnerJoinTagStore` defect on top: the read
//! filter drops events carrying no tags, as an `INNER JOIN` onto a tag side table
//! does. It exists so that "this store fails nothing" can be distinguished from
//! "this store was never driven" — see [`crate`] for the argument.

use core::cell::{Cell, RefCell};
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent,
};

use crate::correct::{self, LogError, dense};

/// The backing log every store here shares with its handles.
///
/// A bare `Vec` behind an `Rc<RefCell<..>>`, which is what the testkit's own
/// `MutantStore` holds, and for its reason: `Rc` rather than `Arc` because the
/// harness drives everything on one thread through `happenstance_testkit::block_on`
/// and `EventStore` is the flavour with no `Send` bound, so atomics would buy
/// nothing and would quietly stop exercising the flavour ADR-0001 exists for.
pub type Log = Rc<RefCell<Vec<SequencedEvent>>>;

/// A fresh, empty log.
#[must_use]
pub fn empty_log() -> Log {
    Rc::new(RefCell::new(Vec::new()))
}

/// [`correct::matching`], with the injected second defect optionally applied.
///
/// The injected arm is `InnerJoinTagStore`'s defect, transcribed from
/// `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:427-437`: tags
/// live in a side table and the join that reaches them is in the `FROM` clause
/// rather than the `WHERE`, so an event with no tags has no row on the other side
/// and is invisible to every query — `Query::all()` included.
fn filtered<'a>(
    events: &'a [SequencedEvent],
    query: &Query,
    injected: bool,
) -> Vec<&'a SequencedEvent> {
    let matched = correct::matching(events, query);
    if injected {
        matched
            .into_iter()
            .filter(|event| !event.tags().is_empty())
            .collect()
    } else {
        matched
    }
}

/// The correct append path, shared by three of the four stores.
fn commit_correctly(
    log: &Log,
    events: &[Event],
    condition: Option<&AppendCondition>,
) -> Result<SequencePosition, AppendError<LogError>> {
    let mut stored = log
        .try_borrow_mut()
        .map_err(|_| AppendError::Store(LogError::AlreadyBorrowed))?;
    correct::commit(&mut stored, events, condition, dense)
}

/// The correct `head`, shared by all four stores.
fn head_correctly(log: &Log) -> Result<Option<SequencePosition>, LogError> {
    let stored = log.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
    Ok(correct::head_of(&stored))
}

/// The correct `contains_event_id`, shared by all four stores.
fn contains_correctly(log: &Log, id: EventId) -> Result<bool, LogError> {
    let stored = log.try_borrow().map_err(|_| LogError::AlreadyBorrowed)?;
    Ok(correct::contains(&stored, id))
}

// =====================================================================
// 0. The baseline these four are one step away from
// =====================================================================

/// A completely correct store over [`Log`], except for `INJECTED`.
///
/// Not itself one of the four. It is what the **L1-2** fixture is wrapped around
/// — that finding's defect is in the fixture's `reopen`, not in any store — and
/// it is the carrier for that entry's injected arm.
#[derive(Debug)]
pub struct PlainStore<const INJECTED: bool> {
    log: Log,
}

impl<const INJECTED: bool> PlainStore<INJECTED> {
    /// A handle onto `log`.
    #[must_use]
    pub fn over(log: &Log) -> Self {
        Self {
            log: Rc::clone(log),
        }
    }
}

impl<const INJECTED: bool> EventStore for PlainStore<INJECTED> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        correct::Snapshot::new(
            self.log
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| {
                    correct::truncated(
                        correct::ordered(filtered(&stored, query, INJECTED), options),
                        options,
                    )
                    .into_iter()
                    .cloned()
                    .collect()
                }),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        commit_correctly(&self.log, events, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        head_correctly(&self.log)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        contains_correctly(&self.log, id)
    }
}

// =====================================================================
// 1. L1-1 — the forward paging budget that is never spent
// =====================================================================

/// A forward read that branches on `from` and forgets `limit` in that branch.
///
/// The shape the finding names: `from` arrives for the projection-resume path,
/// which originally passed no limit, and the resume branch is written without
/// threading `limit` into it —
///
/// ```text
/// if let Some(from) = options.from {
///     self.read_resume(from)          // <- limit never reaches here
/// } else {
///     self.read_paged(options.limit)
/// }
/// ```
///
/// The backwards branch is left correct, which is what makes this a scalpel: the
/// suite *does* compose backwards `from` with `limit`
/// (`read_backwards_from_with_limit`, `suite.rs:979`), so a store that broke both
/// directions would be caught by a rule that is not this finding's subject.
///
/// The wrong outcome is a silently over-large page: a projection runner that
/// asks for 500 events from its checkpoint is handed the whole stream, and the
/// store it is projecting into buffers a batch nobody sized.
#[derive(Debug)]
pub struct ForwardPagingBudgetStore<const INJECTED: bool> {
    log: Log,
}

impl<const INJECTED: bool> ForwardPagingBudgetStore<INJECTED> {
    /// A handle onto `log`.
    #[must_use]
    pub fn over(log: &Log) -> Self {
        Self {
            log: Rc::clone(log),
        }
    }
}

impl<const INJECTED: bool> EventStore for ForwardPagingBudgetStore<INJECTED> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        correct::Snapshot::new(
            self.log
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| {
                    let ordered = correct::ordered(filtered(&stored, query, INJECTED), options);
                    // THE DEFECT. The resume branch returns the whole filtered,
                    // ordered set; `limit` is applied only where `from` is absent.
                    // Backwards is left alone.
                    let selected = if options.from.is_some() && !options.backwards {
                        ordered
                    } else {
                        correct::truncated(ordered, options)
                    };
                    selected.into_iter().cloned().collect()
                }),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        commit_correctly(&self.log, events, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        head_correctly(&self.log)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        contains_correctly(&self.log, id)
    }
}

// =====================================================================
// 3. L3-01 — a fetch failure reported as end-of-stream
// =====================================================================

/// How many events one "page" of [`SwallowedReadFaultStore`] carries.
///
/// Small, so that a read of a handful of events is genuinely several fetches and
/// the fault has somewhere to land other than the first one.
pub const PAGE: usize = 2;

/// A paged read whose fetch failure is reported as the end of the stream.
///
/// The finding's exact line, in a `poll_next` that must return a value:
///
/// ```text
/// let Ok(page) = fetch().await else { return Poll::Ready(None) };
/// ```
///
/// It is the most natural way to get a fallible fetch past a `poll_next`, and it
/// is what an adapter over `SqlStorage` (Cloudflare) or one-shot HTTP (Neon)
/// reaches for. The port expresses the failure — `read` yields
/// `Result<SequencedEvent, Self::Error>` **per item** — and this store declines
/// to use it. A consumer sees a short, successful read: a projection checkpoints
/// past events it never saw.
///
/// # There is no `Fixture` seam to arm this, and that is the finding
///
/// [`SwallowedReadFaultFixture`](crate::stores::SwallowedReadFaultStore) — see
/// `tests/support/` — carries an `arm_read_fault` of its own, which is **not** a
/// `Fixture` method and which no conformance rule can therefore call. That is not
/// this experiment being coy: `Fixture` has `MID_BATCH_FAULT` / `arm_mid_batch_fault`
/// for the *write* path and no read-path analogue, so a fault seam a rule could
/// reach does not exist to be used. `tests/defect_is_real.rs` arms it by hand and
/// records what the store then does, which is the other half of the evidence.
#[derive(Debug)]
pub struct SwallowedReadFaultStore<const INJECTED: bool> {
    log: Log,
    /// The number of successful page fetches before the next one fails, or
    /// `None` for "no fault armed".
    fault_after: Rc<Cell<Option<usize>>>,
}

impl<const INJECTED: bool> SwallowedReadFaultStore<INJECTED> {
    /// A handle onto `log`, sharing `fault_after` with every other handle.
    #[must_use]
    pub fn over(log: &Log, fault_after: &Rc<Cell<Option<usize>>>) -> Self {
        Self {
            log: Rc::clone(log),
            fault_after: Rc::clone(fault_after),
        }
    }
}

impl<const INJECTED: bool> EventStore for SwallowedReadFaultStore<INJECTED> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        let selected = self
            .log
            .try_borrow()
            .map_err(|_| LogError::AlreadyBorrowed)
            .map(|stored| {
                correct::truncated(
                    correct::ordered(filtered(&stored, query, INJECTED), options),
                    options,
                )
                .into_iter()
                .cloned()
                .collect::<Vec<_>>()
            });

        // THE DEFECT, part one: a read that could not even start is reported as
        // an empty stream rather than as the `Err` item the port provides for.
        let pages = selected
            .unwrap_or_default()
            .chunks(PAGE)
            .map(<[SequencedEvent]>::to_vec)
            .collect::<Vec<_>>();

        SwallowingPagedStream {
            pages: pages.into_iter(),
            page: Vec::new().into_iter(),
            fetched: 0,
            fault_after: self.fault_after.get(),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        commit_correctly(&self.log, events, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        head_correctly(&self.log)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        contains_correctly(&self.log, id)
    }
}

/// The stream [`SwallowedReadFaultStore`] returns.
///
/// Every field is `Unpin`, so `poll_next` reaches its state through
/// [`Pin::get_mut`] and no pin projection is hand-written — which the workspace's
/// `unsafe_code = "forbid"` would forbid anyway, and this crate repeats the
/// `#![forbid(unsafe_code)]` so the constraint is not merely inherited.
#[derive(Debug)]
pub struct SwallowingPagedStream {
    pages: std::vec::IntoIter<Vec<SequencedEvent>>,
    page: std::vec::IntoIter<SequencedEvent>,
    fetched: usize,
    fault_after: Option<usize>,
}

impl Stream for SwallowingPagedStream {
    type Item = Result<SequencedEvent, LogError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if let Some(event) = this.page.next() {
                return Poll::Ready(Some(Ok(event)));
            }
            let Some(next) = this.pages.next() else {
                return Poll::Ready(None);
            };
            if this.fault_after == Some(this.fetched) {
                // THE DEFECT, part two, and it is the finding's own line:
                //
                //     let Ok(page) = fetch().await else { return Poll::Ready(None) };
                //
                // The fetch failed. The caller is told the stream ended.
                return Poll::Ready(None);
            }
            this.fetched += 1;
            this.page = next.into_iter();
        }
    }
}

// =====================================================================
// 4. F2-5 — a store that genuinely suspends inside `append`
// =====================================================================

/// Suspends once: `Pending` on the first poll, `Ready` on the second.
///
/// Written out rather than reached for from a runtime, because this crate's
/// census binary has none — it drives everything through
/// `happenstance_testkit::block_on`. The waker is signalled *before* returning
/// `Pending`, which is what makes this a store that is slow rather than one that
/// is hung. Copied from the testkit's `YieldingRowAtATimeStore`, which carries
/// the same note.
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

/// One round trip per row, staged, then one atomic `COMMIT`.
///
/// The interactive-transaction shape — `BEGIN`, an `INSERT` per row over the
/// network, `COMMIT` — done **correctly**: nothing is visible to anybody until
/// the commit, and the commit is a single synchronous section under the exclusive
/// borrow, so a dropped future leaves the store byte-identical.
///
/// # What it is an instrument for
///
/// F2-5 says `dropped_append_future_leaves_no_partial_batch` "cannot fail against
/// either shipping adapter, because neither `append` body has a suspension point"
/// — and, more sharply, that the rule's `landed == 0` arm *has never executed
/// against any store in the workspace*. Every store that mounts the suite today
/// completes `append` on its first poll, so the rule always takes its
/// "fully applied" branch. This store takes the other one. It is not a wrong
/// implementation; it is the *right* implementation of the shape the workspace
/// does not have, and what it measures is whether the rule works when it finally
/// meets one.
///
/// The testkit already ships `YieldingRowAtATimeStore`, which suspends and is
/// **not** atomic, and it fails the rule. The pair is what makes the answer
/// two-sided.
#[derive(Debug)]
pub struct StagedCommitStore<const INJECTED: bool> {
    log: Log,
}

impl<const INJECTED: bool> StagedCommitStore<INJECTED> {
    /// A handle onto `log`.
    #[must_use]
    pub fn over(log: &Log) -> Self {
        Self {
            log: Rc::clone(log),
        }
    }
}

impl<const INJECTED: bool> EventStore for StagedCommitStore<INJECTED> {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        correct::Snapshot::new(
            self.log
                .try_borrow()
                .map_err(|_| LogError::AlreadyBorrowed)
                .map(|stored| {
                    correct::truncated(
                        correct::ordered(filtered(&stored, query, INJECTED), options),
                        options,
                    )
                    .into_iter()
                    .cloned()
                    .collect()
                }),
        )
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // The staging round trips. One statement per row, a suspension point
        // between two of them, and nothing anyone can observe: the rows exist
        // only in this future's own frame until the commit below.
        let mut staged = Vec::with_capacity(events.len());
        for event in events {
            staged.push(event.clone());
            yield_once().await;
        }

        // COMMIT. One synchronous section, no `.await` inside it, so the
        // exclusive borrow is acquired and released within a single `poll` — and
        // the condition is evaluated here rather than before the round trips,
        // which is what keeps this store correct under
        // `interleaved_appends_on_one_handle_elect_one_winner`. Evaluating it up
        // front would be a second defect (two winners), and this store is allowed
        // exactly one difference from correct.
        commit_correctly(&self.log, &staged, condition)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        head_correctly(&self.log)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        contains_correctly(&self.log, id)
    }
}
