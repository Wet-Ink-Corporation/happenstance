//! The wrong stores that are only wrong in parallel.
//!
//! Every store in `mutants.rs` is `Rc`-backed and driven on one thread, which
//! that file's own closing note calls out as the set's largest uncovered axis:
//! *a store whose defect appears only under genuine parallelism, a lost update
//! between two OS threads, still cannot be expressed.* These five can be. They
//! are `Arc`/`Mutex` stores implementing [`SendEventStore`], and each is wrong in
//! exactly one way that no sequential rule in the suite can see.
//!
//! # Why they are not in `REGISTRY`
//!
//! `mutant_registry_is_exhaustive` rejects a row whose `fails` list is empty, so
//! a mutant cannot land before the rule that catches it. Every store here fails
//! **no** rule of the event-store family — that is the point of them — so a row
//! in `REGISTRY` would be a contradiction rather than an oversight. They carry
//! their own enumeration and their own table, [`RACERS`](super::RACERS), checked
//! by the same two directions CF-3 asks for: every store fails every rule it
//! declares, and passes every rule it does not.
//!
//! # The rendezvous, and why it is not a sleep
//!
//! A wrong store that is *sometimes* caught is worse than useless in a proof
//! artefact: it turns the meta-test into a coin toss and teaches the next
//! reader to re-run. Every window below is therefore closed by a **rendezvous**
//! rather than by luck — an atomic counter and [`std::thread::yield_now`], with
//! the wait bounded by a number of yields so that a store which is never joined
//! proceeds instead of hanging.
//!
//! That bound is a count of *yields*, not of milliseconds, and nothing asserts
//! on it. CF-33 constrains conformance **rules** — no clock, no elapsed time, no
//! assertion on an operation count — and these are instruments rather than
//! rules; but the reason CF-33 exists applies here too, which is why the
//! mechanism is a rendezvous with an escape hatch and not `thread::sleep(10ms)`.
//! A sleep would be both slower and less reliable: it would still be a race, run
//! against a wall clock on a loaded runner.
//!
//! # How to add one
//!
//! 1. Write the store here, wrong in exactly one way, `Send + Sync`, and close
//!    its window with a rendezvous.
//! 2. Add its fixture to `for_each_racer!` in `tests/mutation_coverage.rs`.
//! 3. Add its row to `RACERS` in the same file, naming the exact set of
//!    concurrency rules it fails and the real adapter shape it comes from.

use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use core::task::{Context, Poll};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent,
};
use happenstance_testkit::{Capability, Fixture};

use crate::correct::{self, LogError, Snapshot, dense};
use crate::harness::Subject;

// =====================================================================
// The shared mechanism
// =====================================================================

/// How many times a rendezvous yields before giving up and proceeding.
///
/// Bounded so that a store nobody joined finishes rather than hanging — a hung
/// proof artefact names no rule, which is the failure `harness.rs` explains
/// there is deliberately no watchdog for. Large enough that two threads which
/// are genuinely running concurrently will always meet; small enough that a
/// store which waits alone costs milliseconds rather than seconds.
const RENDEZVOUS_YIELDS: usize = 8_000;

/// How many yields a rendezvous waits for its company to *stop growing*.
///
/// See [`Shared::wait_for_company`] for why waiting for a count rather than for
/// a settled count is not enough, and why the count has to be cumulative.
const SETTLE_YIELDS: usize = 512;

/// One backing store, shared by every handle of one fixture.
///
/// All five stores share one state type even though none uses every field. The
/// alternative — a state struct per store — buys nothing but five more
/// constructors, and the fields are named for what they are so that a store
/// which ignores one is obviously ignoring it.
#[derive(Debug, Default)]
pub(crate) struct Shared {
    /// The committed log. The one thing every store here has.
    events: Mutex<Vec<SequencedEvent>>,
    /// A second lock, held across probe-and-insert by the store whose defect is
    /// *not* about that pair being atomic.
    appending: Mutex<()>,
    /// A position counter maintained outside `events`, as `SELECT
    /// max(position)` before `BEGIN` is. Zero means "nothing yet".
    head: AtomicU64,
    /// How many appends have **ever** entered a store's window.
    ///
    /// Cumulative rather than an occupancy count, which is the third version of
    /// this field and the first that works. See [`Shared::wait_for_company`].
    arrivals: AtomicUsize,
    /// Completed reads, so a writer can wait for a reader rather than for a
    /// clock.
    reads: AtomicU64,
}

impl Shared {
    /// Locks the log, ignoring poisoning.
    ///
    /// A contender that panics poisons every mutex it held, and a store that
    /// then refused to answer would convert one rule's failure into four. The
    /// data behind the lock is a `Vec` with no invariant a panic could break.
    fn log(&self) -> MutexGuard<'_, Vec<SequencedEvent>> {
        self.events.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The highest position currently committed.
    ///
    /// Every store below answers `EventStore::head` from here, and it delegates
    /// to the correct core for the reason `correct.rs` exists: each store in
    /// this file is wrong in exactly one *concurrent* way, and a second,
    /// undeclared defect in `head` would leave
    /// `the_concurrency_rules_reject_exactly_what_they_claim` catching the
    /// instrument rather than the implementation. `GlobalHeadStore`'s defect is
    /// not that this answer is wrong — it is that this is not the answer its
    /// `append` was asked for.
    fn committed_head(&self) -> Option<SequencePosition> {
        correct::head_of(&self.log())
    }

    /// Whether the committed log holds an event with this identity.
    fn holds(&self, id: EventId) -> bool {
        correct::contains(&self.log(), id)
    }

    /// Announces this append's arrival at a window, then waits — bounded — until
    /// at least `want` have arrived **and no more are still arriving**.
    ///
    /// # Three versions, and why the first two were wrong
    ///
    /// This is the only genuinely subtle thing in the file, and both wrong
    /// versions failed the same way: the store went on being rejected *most* of
    /// the time, which is the worst possible failure mode for an instrument
    /// because it looks like success.
    ///
    /// **Wait for two.** Contenders then meet in *pairs*: the first two rendezvous,
    /// one commits, and everybody who arrives afterwards is told — correctly — that
    /// the log has moved. A store that rejects everyone it overlapped is
    /// indistinguishable from a correct one when it only ever overlapped one
    /// person, so `GlobalVersionStore` looked conformant about half the time.
    ///
    /// **Wait for two, then for the count to stop climbing, with the count being
    /// current occupancy.** Better, and still wrong about one run in thirty: a
    /// thread that has finished decrements, so a late arrival sees a small and
    /// stable crowd, settles, and starts a *second* wave — and four waves of
    /// three is four boundaries with one winner each, which is exactly what a
    /// conformant store produces.
    ///
    /// **Cumulative arrivals.** A counter that never goes down cannot shrink
    /// under a late arrival: every straggler bumps it and re-arms everybody still
    /// waiting, so the cohort cannot close until the last contender is in it.
    /// That is the version below.
    ///
    /// The settle window is a heuristic and is allowed to be one: this is an
    /// instrument rather than a rule, nothing asserts on the number, and the
    /// whole loop is bounded so a store nobody joins proceeds rather than
    /// hanging.
    fn wait_for_company(&self, want: usize) {
        let mut seen = self.arrivals.fetch_add(1, Ordering::AcqRel) + 1;
        let mut still = 0;
        for _ in 0..RENDEZVOUS_YIELDS {
            let now = self.arrivals.load(Ordering::Acquire);
            if now > seen {
                seen = now;
                still = 0;
            } else {
                still += 1;
            }
            if seen >= want && still >= SETTLE_YIELDS {
                return;
            }
            std::thread::yield_now();
        }
    }

    /// Waits, bounded, until the committed log holds more than `len` events.
    fn wait_for_growth(&self, len: usize) {
        for _ in 0..RENDEZVOUS_YIELDS {
            if self.log().len() > len {
                return;
            }
            std::thread::yield_now();
        }
    }

    /// Waits, bounded, until a read that *started after this call* has finished.
    ///
    /// Two completions rather than one: a read already in flight when this is
    /// called may have sampled the store before the partial write, so only the
    /// second completion is guaranteed to have seen it.
    ///
    /// Returns immediately when nothing has ever read, which is every rule but
    /// one — there is no reader to rendezvous with, and waiting would cost the
    /// whole budget per batch for nothing.
    ///
    /// # What that early-out cannot tell apart, and who fixed it
    ///
    /// `reads == 0` means *no read has completed*, which covers both "this rule
    /// has no reader" and "the reader exists but has not been scheduled yet".
    /// The second was measured on an oversubscribed host: the writer phase of
    /// `a_concurrent_reader_never_sees_a_partial_batch` is a few dozen appends
    /// of in-memory work, so every writer could finish before the reader thread
    /// ran once, every rendezvous was skipped, and this store went unrejected on
    /// ten runs in twenty-four. An instrument that stops demonstrating anything
    /// while still passing is the worst failure mode a proof artefact has.
    ///
    /// The fix is not here, and deliberately not: dropping the early-out would
    /// make every other rule pay the whole yield budget per row for a reader
    /// that does not exist, and no counter this store can see distinguishes the
    /// two cases — a reader announces itself only by reading.
    /// `observe_while_writing` in the testkit now completes one read on the
    /// rule's own thread before any writer is spawned, which is a happens-before
    /// no scheduler can take away. That belongs in the rule for the rule's own
    /// sake: without it the live half of that rule samples an already-quiescent
    /// store and silently degenerates into the post-hoc half.
    fn wait_for_a_reader(&self) {
        let start = self.reads.load(Ordering::Acquire);
        if start == 0 {
            return;
        }
        for _ in 0..RENDEZVOUS_YIELDS {
            if self.reads.load(Ordering::Acquire) >= start + 2 {
                return;
            }
            std::thread::yield_now();
        }
    }

    /// The read every store here shares: a snapshot, plus the counter that lets
    /// a writer wait for one.
    fn snapshot(&self, query: &Query, options: ReadOptions) -> Snapshot {
        let selected = correct::select(&self.log(), query, options);
        // Incremented after the log is released, so a writer waiting on it knows
        // the read is over rather than merely started.
        self.reads.fetch_add(1, Ordering::Release);
        Snapshot::new(Ok(selected))
    }
}

/// A future that suspends exactly once.
///
/// The window every store below opens is spelled as a real suspension rather
/// than as a bare `yield_now`, because the defect being modelled is *an append
/// that awaits between two of its own steps* — and a store whose `append` body
/// contains no `.await` at all has no window to be wrong in, which is why
/// `MemoryEventStore` passes these rules structurally.
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

/// Declares a fixture and its `Subject` row for a store built on [`Shared`].
macro_rules! racing_fixture {
    ($fixture:ident, $store:ident, $name:literal) => {
        #[doc = concat!("The fixture that opens handles onto one `", $name, "`.")]
        #[derive(Debug, Default)]
        pub(crate) struct $fixture(Arc<Shared>);

        impl Fixture for $fixture {
            type Store = $store;

            // Genuinely supported, and it has to be: contention between two
            // handles onto one backing store is the whole subject.
            const SECOND_HANDLE: Capability = Capability::SUPPORTED;
            const REOPEN: Capability =
                Capability::declined("this instrument exists for the concurrency axis only");

            async fn connect(&self) -> Self::Store {
                $store(Arc::clone(&self.0))
            }
        }

        impl Subject for $fixture {
            const NAME: &'static str = $name;

            fn open() -> Self {
                Self::default()
            }
        }
    };
}

// =====================================================================
// The conformant control
// =====================================================================

/// One mutex, held across the whole append. Correct, and correct for a
/// structural reason.
///
/// The control this file needs for the same reason `variants.rs` holds two: a
/// table in which nothing passes is a table that proves the harness rejects
/// everything. It is also the shape almost every adapter in this workspace will
/// have — rusqlite behind a connection, a Durable Object, `MemoryEventStore`
/// itself — which is why `happenstance-postgres`, whose writers are *not*
/// serialised, is the instrument the runbook still owes.
#[derive(Debug)]
pub(crate) struct LockedStore(Arc<Shared>);

impl SendEventStore for LockedStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        correct::commit(&mut self.0.log(), events, condition, dense)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(LockedFixture, LockedStore, "LockedStore");

// =====================================================================
// The five defects
// =====================================================================

/// The condition is probed, the transaction is not held, and the rows are
/// inserted afterwards.
///
/// Autocommit plus a separate probe — `WriteThenCheckStore`'s sibling, and the
/// order is the other way round, which is what makes it invisible to every
/// sequential rule. `WriteThenCheckStore` writes and *then* discovers it should
/// not have, so `append_is_atomic` catches it with one caller. This one asks
/// first and answers correctly; the answer is simply stale by the time it acts
/// on it, and with one caller at a time nothing can make it stale.
///
/// The provenance in SQL is **`BEGIN DEFERRED`**, then
/// `SELECT 1 FROM events WHERE …`, then `INSERT` — and the transaction verb is
/// the load-bearing word. `BEGIN DEFERRED` is SQLite's default and is what
/// `rusqlite::Connection::transaction` opens: the read lock is taken at the
/// probe and only promoted to a write lock at the insert, so between those two
/// statements another connection may commit and the probe's answer goes stale
/// while the transaction is still open. It looks atomic, it is inside a
/// transaction, and it is wrong — which is why `happenstance-sqlite` opens
/// `BEGIN IMMEDIATE` instead and takes the write lock *before* the condition is
/// read.
///
/// The same defect also arrives with no `BEGIN` at all and no `SERIALIZABLE`
/// under it, which is what an adapter writes when its driver's convenience API
/// is one statement per call.
///
/// There is deliberately **no `REGISTRY` row** for this store: `fails` would be
/// empty, because it fails no sequential rule by construction, and
/// `mutant_registry_is_exhaustive` rejects that. See the comment on its `RACERS`
/// row.
#[derive(Debug)]
pub(crate) struct RacingProbeStore(Arc<Shared>);

impl SendEventStore for RacingProbeStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let Some(condition) = condition else {
            // No condition, no probe, no window: an unconditional append has no
            // decision to make stale. Keeping the window shut here is what makes
            // this a scalpel rather than a store that is wrong four ways.
            return correct::commit(&mut self.0.log(), events, None, dense);
        };

        // The probe, answered correctly, against everything committed so far.
        // The guard is scoped so that none is held across the suspension below —
        // `clippy::await_holding_lock` is a workspace deny, and holding one here
        // would give the store a second defect nobody declared.
        if let Some(conflict) = correct::violation(&self.0.log(), condition) {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        // THE DEFECT: the window between the answer and the act.
        YieldOnce(false).await;
        self.0.wait_for_company(2);

        // The insert, with the probe's verdict already spent.
        let mut log = self.0.log();
        let head = log.last().map(|event| event.position);
        let written = correct::sequence(events, head, dense);
        let last = written
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        log.extend(written);
        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(RacingProbeFixture, RacingProbeStore, "RacingProbeStore");

/// The condition is correct, and then a global version check rejects everything
/// that ran beside it.
///
/// Optimistic concurrency control on a single version number: read the head,
/// do the work, and commit only if the head has not moved. It is what a Durable
/// Object with one `version` key does, what `UPDATE … WHERE version = ?` does,
/// and what a `SERIALIZABLE` adapter that maps `40001 serialization_failure`
/// onto `AppendError::ConditionViolated` does.
///
/// Every sequential rule passes, because a sequential writer never overlaps
/// anybody: the version has never moved when it looks. Under contention it
/// reports conflicts between commands that share no query at all — the steady
/// state of a busy store with many independent entities, reported to every one
/// of them as contention.
///
/// This is the store that makes `k_disjoint_boundaries_admit_exactly_k_commits`
/// worth having rather than being a restatement of
/// `exactly_one_of_n_contenders_commits`: it passes the second, because one
/// winner out of eight on **one** boundary is exactly what it produces.
#[derive(Debug)]
pub(crate) struct GlobalVersionStore(Arc<Shared>);

impl SendEventStore for GlobalVersionStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let Some(condition) = condition else {
            // An unconditional append carries no version to compare, so there is
            // nothing here to be wrong about.
            return correct::commit(&mut self.0.log(), events, None, dense);
        };

        let version = {
            let log = self.0.log();
            if let Some(conflict) = correct::violation(&log, condition) {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict,
                )));
            }
            log.len()
        };

        // The transaction doing its work. The rendezvous is what makes the
        // rejection below happen on every run rather than on a lucky one.
        YieldOnce(false).await;
        self.0.wait_for_company(2);

        let mut log = self.0.log();
        // THE DEFECT: the commit-time check is *did anything at all land*, not
        // *does anything matching my query land above my `after`*. The
        // conflicting position it reports is real, which is what makes the error
        // indistinguishable from a genuine violation.
        if log.len() != version {
            let conflict = log
                .last()
                .map_or(SequencePosition::FIRST, |event| event.position);
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        let head = log.last().map(|event| event.position);
        let written = correct::sequence(events, head, dense);
        let last = written
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        log.extend(written);
        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(
    GlobalVersionFixture,
    GlobalVersionStore,
    "GlobalVersionStore"
);

/// Positions come from a counter read **before** the transaction that uses it.
///
/// `SELECT max(position) FROM events` and then `BEGIN`. The probe and the insert
/// are properly serialised — the second mutex below is held across both — so
/// nothing about the *decision* is wrong; only the numbers are, and only when
/// two writers read the counter before either writes it back.
///
/// The sequential form of this defect does not exist: with one writer the read
/// and the write-back are adjacent and the counter is never stale.
/// `positions_are_unique` reads a quiescent store after a single writer, which
/// is why it cannot see it.
#[derive(Debug)]
pub(crate) struct RacingSequenceStore(Arc<Shared>);

impl SendEventStore for RacingSequenceStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // THE DEFECT, first half: the counter is read here, outside everything
        // that will serialise this append against another.
        let stale = SequencePosition::new(self.0.head.load(Ordering::Acquire));

        YieldOnce(false).await;
        self.0.wait_for_company(2);

        // Probe and insert under one lock, so the decision is atomic and this
        // store is wrong about exactly one thing.
        let _serialised = self
            .0
            .appending
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let mut log = self.0.log();
        if let Some(condition) = condition
            && let Some(conflict) = correct::violation(&log, condition)
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        // THE DEFECT, second half: allocated from the number read before the
        // window, so two writers that read it together assign the same rows.
        let written = correct::sequence(events, stale, dense);
        let last = written
            .last()
            .map(|event| event.position)
            .ok_or(AppendError::NoEvents)?;
        log.extend(written);
        self.0.head.store(last.get(), Ordering::Release);
        Ok(last)
    }

    // Answered from the log rather than from `Shared::head`, which is the only
    // store here where the two can disagree. The atomic *is* this store's
    // declared defect — a counter read outside the serialising lock — and
    // reporting it as the head would be that defect leaking into a second
    // operation nobody declared it in.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(
    RacingSequenceFixture,
    RacingSequenceStore,
    "RacingSequenceStore"
);

/// The batch is written correctly, and then the store's **head** is returned
/// instead of the caller's own last position.
///
/// `INSERT …;` followed by `SELECT max(position) FROM events`, two statements
/// with no transaction around them — which is what an adapter writes when its
/// driver cannot give it `RETURNING` on a multi-row insert. With one caller the
/// two values are the same number, which is why every sequential rule passes it
/// and why the defect has to be looked for here.
///
/// What the caller does with the wrong number is the reason it matters: it
/// checkpoints a projection at it, or builds the next `AppendCondition::after`
/// from it, and either way silently skips every event another writer landed in
/// between.
#[derive(Debug)]
pub(crate) struct GlobalHeadStore(Arc<Shared>);

impl SendEventStore for GlobalHeadStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let (mine, len) = {
            let mut log = self.0.log();
            let mine = correct::commit(&mut log, events, condition, dense)?;
            (mine, log.len())
        };

        // The gap between the two statements. The rendezvous waits for somebody
        // else's rows to land, which is the only thing that makes the second
        // statement's answer differ from the first's.
        YieldOnce(false).await;
        self.0.wait_for_growth(len);

        // THE DEFECT: `max(position)` over the whole table, which is whoever
        // committed last rather than whoever is asking.
        Ok(self.0.committed_head().unwrap_or(mine))
    }

    // Correct, and identical to every other store's — which is the point. This
    // store's defect is that its *`append`* returns the store's head; `head`
    // itself returning the store's head is what `head` is for.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(GlobalHeadFixture, GlobalHeadStore, "GlobalHeadStore");

/// The batch is inserted a row at a time, with no transaction around the loop.
///
/// The driver has no multi-row insert, so the adapter writes
/// `for event in batch { conn.execute(INSERT, …)? }` and forgets the `BEGIN`.
/// Every row lands, so the store is correct at rest and every sequential rule
/// passes; a reader that looks *while* the loop is running sees a command that
/// half-happened.
///
/// The condition probe and the first row go under one lock, so a conditional
/// append still makes its decision atomically. This store is wrong about
/// visibility during a batch and about nothing else.
#[derive(Debug)]
pub(crate) struct RowAtATimeStore(Arc<Shared>);

impl SendEventStore for RowAtATimeStore {
    type Error = LogError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.snapshot(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let Some((first, rest)) = events.split_first() else {
            return Err(AppendError::NoEvents);
        };

        let mut last = {
            let mut log = self.0.log();
            if let Some(condition) = condition
                && let Some(conflict) = correct::violation(&log, condition)
            {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict,
                )));
            }
            let head = log.last().map(|event| event.position);
            let written = correct::sequence(core::slice::from_ref(first), head, dense);
            let position = written
                .last()
                .map(|event| event.position)
                .ok_or(AppendError::NoEvents)?;
            log.extend(written);
            position
        };

        for event in rest {
            // THE DEFECT: the batch is visible part-written here. The wait is
            // for a *reader*, not for a duration, so the sighting happens on
            // every run rather than on a lucky one.
            self.0.wait_for_a_reader();
            YieldOnce(false).await;

            let mut log = self.0.log();
            let head = log.last().map(|stored| stored.position);
            let written = correct::sequence(core::slice::from_ref(event), head, dense);
            last = written
                .last()
                .map(|stored| stored.position)
                .ok_or(AppendError::NoEvents)?;
            log.extend(written);
        }

        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.holds(id))
    }
}

racing_fixture!(RowAtATimeFixture, RowAtATimeStore, "RowAtATimeStore");
