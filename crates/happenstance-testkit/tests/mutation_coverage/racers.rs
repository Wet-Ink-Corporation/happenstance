//! The wrong stores that are only wrong in parallel.
//!
//! Every store in `mutants.rs` is `Rc`-backed and driven on one thread, which
//! that file's own closing note calls out as the set's largest uncovered axis:
//! *a store whose defect appears only under genuine parallelism, a lost update
//! between two OS threads, still cannot be expressed.* These six can be. They
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
//! # The rendezvous, and why it cannot time out
//!
//! A wrong store that is *sometimes* caught is worse than useless in a proof
//! artefact: it turns the meta-test into a coin toss and teaches the next
//! reader to re-run. Every window below is therefore closed by a **rendezvous**
//! rather than by luck, and the rendezvous waits on an exact condition, with no
//! budget of any kind.
//!
//! ## What it replaced, and how that failed
//!
//! Until 2026-10-07 the wait was an atomic counter and
//! [`std::thread::yield_now`], bounded by a number of yields (8,000 to meet,
//! 512 to settle) so that a store nobody joined would proceed rather than hang.
//! On an oversubscribed host the bound was the bug. The waiting thread kept
//! being scheduled while its cohort was not, so the budget ran out first. The
//! mutant then proceeded alone, never raced, and *passed* the rule it exists to
//! fail. On 2026-10-07, on a 4-core host with `CARGO_BUILD_JOBS=2` and a
//! second cargo build running, it was:
//!
//! * red 3 runs in 3 under the workspace tests;
//! * red 1 in 4 for the whole binary run alone (1 in 2 in the run the phase-17
//!   runbook records);
//! * red 0 in 4 for the filtered test.
//!
//! The two rows seen were `RacingProbeStore` passing
//! `exactly_one_of_n_contenders_commits`, and `GlobalVersionStore` passing
//! `k_disjoint_boundaries_never_conflict`.
//!
//! A host's load is not reproducible on demand, so the decisive measurement
//! takes the host out of it. A 0–35 ms sleep was added at the top of
//! `RacingProbeStore`'s and `GlobalVersionStore`'s `append`, standing in for a
//! contender the scheduler has not run yet, and both were measured with it:
//!
//! * **The yield-bounded version** was red 3 runs in 3, on exactly those rows.
//! * **The census below** was green 10 runs in 10, and 3 in 3 at ten times the
//!   delay.
//!
//! The sleep was an experiment and was never committed.
//!
//! Neither obvious repair is one. A larger budget moves the load at which the
//! pass happens and does not remove it. A bounded wait whose expiry panics turns
//! a silent pass into a loud one, which is better, but it is still a flake: the
//! verdict still depends on whether the cohort beats a count. A sleep is the same
//! race run against a wall clock. CF-33 forbids clocks and operation counts in
//! **rules**, not in instruments, but the reason CF-33 exists applies here too.
//!
//! ## What it waits for instead
//!
//! The cohort is now known exactly, and no rule had to change for that to be
//! true. **A rule declares its cohort by the handles it opens.** Each handle
//! joins a census on `connect` and leaves it on `Drop`. A handle that has never
//! read counts as a *contender*. One that has read counts as an *observer*,
//! which is the only role a reader in this family plays. A rendezvous releases
//! its cohort when every open contender has arrived, and the count it compares
//! against falls as contenders close. Nothing counts yields and nothing reads a
//! clock, so load can make the meeting slower and cannot make it not happen.
//!
//! ## Why it cannot hang either
//!
//! It still has to keep the old bound's promise: a hung proof artefact names no
//! rule, and `harness.rs` explains why there is deliberately no watchdog.
//!
//! * **A store nobody joins** is a cohort of one. Its only open contender is
//!   the one arriving, so it is released on arrival, with no wait at all. Every
//!   setup append is this case: the setup handle is dropped before the
//!   contenders are connected.
//! * **A contender that never arrives** has lost: the probe rejected it before
//!   the window, or it fell over. Either way its thread ends, its handle drops,
//!   and the cohort it was holding open shrinks to the ones already waiting.
//! * **A reader** marks its handle an observer on its first read.
//!   `observe_while_writing` makes that read on the rule's own thread before any
//!   writer is spawned, which is a happens-before, so no writer ever counts the
//!   reader into its cohort.
//!
//! What remains is a precondition, and it is written here so that the author of
//! the next rule meets it on purpose. **Every open handle that has never read
//! must eventually append or be dropped while a cohort is waiting.** All six
//! rules meet it. `race` moves each handle into a contender that appends once
//! and drops it. `observe_while_writing`'s writers append once per round, all
//! for the same number of rounds, so each round is one cohort and the last
//! round is followed by the drop. Setup handles go out of scope before the
//! race, and post-hoc observers are connected after it. A rule that held an
//! idle, never-read handle open across a race would hang this binary: the CI
//! timeout would catch it, and it would name no rule. That is loud, which the
//! old failure mode never was.
//!
//! A second, narrower rule follows from the counting: **an observer must not
//! append into a window.** It would count as an arrival without counting as a
//! contender, so a cohort could release one contender short. No rule here
//! appends through a handle it has read from.
//!
//! The wait is a [`Future`] that registers its waker, not a spin loop. The
//! testkit's `block_on` parks the contender's thread until a census change
//! wakes it, so a waiting contender gives its core to the ones it is waiting
//! for, which is the point on a host with no core to spare.
//!
//! # How to add one
//!
//! 1. Write the store here, wrong in exactly one way, `Send + Sync`, wrapping a
//!    [`Handle`], and close its window with [`Shared::rendezvous`] or another
//!    [`Until`] over the census. Never with a count or a clock.
//! 2. Add its fixture to `for_each_racer!` in `tests/mutation_coverage.rs`.
//! 3. Add its row to `RACERS` in the same file, naming the exact set of
//!    concurrency rules it fails and the real adapter shape it comes from.

use core::future::Future;
use core::ops::Deref;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use core::task::{Context, Poll, Waker};
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

/// One backing store, shared by every handle of one fixture.
///
/// All six stores share one state type even though none uses every field. The
/// alternative — a state struct per store — buys nothing but six more
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
    /// Who is open onto this store, and who is waiting for whom. Every
    /// rendezvous below is a predicate over it. See the module documentation.
    census: Mutex<Census>,
    /// How many appends have **written**, counted under the log's lock so the
    /// ordinal follows commit order. `BusyAfterWriteStore` lies on the odd
    /// ones, which makes which batches it lies about a property of the order
    /// they committed in rather than of the scheduler.
    writes: AtomicUsize,
}

/// The handles open onto one [`Shared`], and the waits suspended on them.
///
/// One mutex over all of it rather than an atomic per field. That is the fix,
/// not a style choice. A release reads the arrivals and the contender count
/// together, and a contender that closes between those two reads is the kind
/// of interleaving the yield-bounded version kept losing to. Under one lock the
/// question "is everybody in?" has one answer at a time.
#[derive(Debug, Default)]
struct Census {
    /// Open handles that have never read: the cohort a rendezvous waits for.
    contenders: usize,
    /// Open handles that have read. A reader is not a contender.
    observers: usize,
    /// Contenders that have arrived at the rendezvous now filling.
    arrived: usize,
    /// Turns each time a rendezvous releases its cohort, so a waiter can tell
    /// "my cohort has gone" from "the next one is filling".
    generation: u64,
    /// Completed reads, so a writer can wait for a reader rather than for a
    /// clock.
    reads: u64,
    /// Every wait suspended on a change to the fields above.
    ///
    /// Bounded by the number of waits, because [`Census::park`] does not push a
    /// waker that is already here. At most one per open handle.
    waiting: Vec<Waker>,
}

impl Census {
    /// Releases the cohort now filling, if every open contender is in it.
    ///
    /// Called wherever either side of the comparison moves: on an arrival, and
    /// on a contender closing or becoming an observer. The second is what lets
    /// a cohort with a lost contender release without that contender.
    fn settle(&mut self) {
        if self.arrived > 0 && self.arrived >= self.contenders {
            self.arrived = 0;
            self.generation = self.generation.wrapping_add(1);
        }
    }

    /// Takes every suspended wait, for the caller to wake once it has dropped
    /// the census lock.
    ///
    /// Every wait is woken, not only the ones whose predicate now holds. That
    /// keeps the predicates in one place, their futures, and a wait woken early
    /// parks again. At most sixty-three are ever suspended at once (sixty-four
    /// contenders, the last of whom releases the rest), so this costs nothing
    /// measurable.
    ///
    /// They are woken *after* the lock is released, never under it. Each one is
    /// a contender that will take this lock as soon as it runs, so waking it
    /// while still holding the lock sends it straight into contention. A waker
    /// that polled inline would also deadlock.
    fn take_waiting(&mut self) -> Vec<Waker> {
        core::mem::take(&mut self.waiting)
    }

    /// Registers a suspended wait, once.
    fn park(&mut self, waker: &Waker) {
        if !self.waiting.iter().any(|parked| parked.will_wake(waker)) {
            // Stored, so cloned: the census outlives this poll.
            self.waiting.push(waker.clone());
        }
    }
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

    /// Locks the census, ignoring poisoning for [`Shared::log`]'s reason.
    ///
    /// Nothing panics while holding this lock. Every update under it is a
    /// saturating count, a `Vec` push, or a `mem::take` of the waiters.
    fn census(&self) -> MutexGuard<'_, Census> {
        self.census.lock().unwrap_or_else(PoisonError::into_inner)
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

    /// Announces this append's arrival at a window. The future it returns is
    /// ready once **every open contender** has arrived too, or has closed.
    ///
    /// # Four versions, and why the first three were wrong
    ///
    /// This is the only genuinely subtle thing in the file. All three wrong
    /// versions failed the same way: the store went on being rejected *most*
    /// of the time. That is the worst possible failure mode for an instrument,
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
    /// **Cumulative arrivals, settled over a yield budget.** A counter that
    /// never goes down cannot shrink under a late arrival, so the cohort could
    /// not close early on an idle host. But "settled" meant "512 yields with no
    /// new arrival", and the whole wait was bounded at 8,000 yields. On a loaded
    /// host both ran out while the rest of the cohort was still unscheduled,
    /// and the store proceeded alone. The module documentation has the numbers.
    ///
    /// **Every open contender.** The version below. It needs no guess at the
    /// cohort's size and no guess at how long it takes to gather, because the
    /// census counts the cohort exactly and releases it on that count and
    /// nothing else. Why that cannot hang is in the module documentation.
    fn rendezvous(&self) -> Until<'_, impl Fn(&Census) -> bool> {
        let (mine, woken) = {
            let mut census = self.census();
            census.arrived = census.arrived.saturating_add(1);
            let mine = census.generation;
            census.settle();
            (mine, census.take_waiting())
        };
        woken.into_iter().for_each(Waker::wake);
        Until {
            shared: self,
            ready: move |census: &Census| census.generation != mine,
        }
    }

    /// Ready once a read that *started after this call* has finished, or once
    /// no reader is open.
    ///
    /// Two completions rather than one: a read already in flight when this is
    /// called may have sampled the store before the partial write, so only the
    /// second completion is guaranteed to have seen it. That holds because
    /// there is exactly one reader. With two, both completions could belong to
    /// reads that were already in flight.
    ///
    /// Ready immediately when no observer is open, which is every rule but one.
    /// There is no reader to meet, and no reader can arrive later: a reader
    /// becomes an observer by reading, and `observe_while_writing` makes its
    /// reader read once before any writer is spawned. The same count ends the
    /// wait when the reader closes, so a writer can never outlive the reader it
    /// is waiting for.
    ///
    /// # The early-out it replaced
    ///
    /// The early-out used to be "no read has completed", with the wait bounded
    /// by yields. Two things went wrong with that. First, the check could not
    /// tell "this rule has no reader" from "the reader has not been scheduled
    /// yet". On an oversubscribed host every writer finished before the reader
    /// thread ran once, and this store went unrejected on ten runs in
    /// twenty-four. `observe_while_writing`'s first read on the rule's own
    /// thread fixed that, and it belongs in the rule for the rule's own sake.
    /// Second, the yield budget had the same flaw as
    /// [`Shared::rendezvous`]'s. Counting observers closes both, and it leaves
    /// nothing to time out.
    fn wait_for_a_reader(&self) -> Until<'_, impl Fn(&Census) -> bool> {
        let start = self.census().reads;
        Until {
            shared: self,
            ready: move |census: &Census| {
                census.observers == 0 || census.reads >= start.saturating_add(2)
            },
        }
    }
}

/// A wait for a predicate over the [`Census`] to hold.
///
/// Every rendezvous in this file is one of these, so there is one place where a
/// wait parks and one place where it is woken. A `Mutex` guard is taken and
/// dropped inside each `poll` and never held across a suspension.
#[must_use = "a rendezvous that is not awaited waits for nothing"]
struct Until<'shared, P> {
    shared: &'shared Shared,
    ready: P,
}

impl<P: Fn(&Census) -> bool> Future for Until<'_, P> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut census = self.shared.census();
        if (self.ready)(&census) {
            return Poll::Ready(());
        }
        census.park(cx.waker());
        Poll::Pending
    }
}

/// One handle's membership of the [`Census`]: a contender from `connect` until
/// its first read, an observer after it, and gone on `Drop`.
///
/// This is the field every store below wraps, in place of a bare
/// `Arc<Shared>`. Joining on construction and leaving on `Drop` is the only way
/// to make the census exact. A handle can be dropped from any thread at any
/// point, including during an unwind, and `Drop` runs in all of those cases.
///
/// It dereferences to [`Shared`] so the store bodies read `self.0.log()` as
/// they always have. That is `Deref` on a handle type whose only job is to be a
/// counted `Arc`, which is the case `Deref` exists for.
#[derive(Debug)]
struct Handle {
    shared: Arc<Shared>,
    /// Whether this handle has read. Swapped only under the census lock, which
    /// orders it, so `Relaxed` is enough: nothing else synchronises through it.
    /// `Drop` has `&mut self` and reads it without an atomic operation at all.
    observer: AtomicBool,
}

impl Handle {
    /// Opens a handle onto `shared`, counted as a contender.
    fn join(shared: &Arc<Shared>) -> Self {
        let mut census = shared.census();
        census.contenders = census.contenders.saturating_add(1);
        drop(census);
        Self {
            shared: Arc::clone(shared),
            observer: AtomicBool::new(false),
        }
    }

    /// The read every store here shares: a snapshot, plus the census update
    /// that lets a writer wait for one.
    ///
    /// The first read moves this handle from contender to observer, which can
    /// complete a cohort that was waiting only for it. The read is counted
    /// after the log is released, so a writer waiting on the count knows the
    /// read is over rather than merely started.
    fn snapshot(&self, query: &Query, options: ReadOptions) -> Snapshot {
        let selected = correct::select(&self.shared.log(), query, options);
        let woken = {
            let mut census = self.shared.census();
            if !self.observer.swap(true, Ordering::Relaxed) {
                census.contenders = census.contenders.saturating_sub(1);
                census.observers = census.observers.saturating_add(1);
                census.settle();
            }
            census.reads = census.reads.saturating_add(1);
            census.take_waiting()
        };
        woken.into_iter().for_each(Waker::wake);
        Snapshot::new(Ok(selected))
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let observer = *self.observer.get_mut();
        let woken = {
            let mut census = self.shared.census();
            if observer {
                census.observers = census.observers.saturating_sub(1);
            } else {
                census.contenders = census.contenders.saturating_sub(1);
                census.settle();
            }
            census.take_waiting()
        };
        woken.into_iter().for_each(Waker::wake);
    }
}

impl Deref for Handle {
    type Target = Shared;

    fn deref(&self) -> &Shared {
        &self.shared
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
                $store(Handle::join(&self.0))
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
/// everything. Its `RACERS` row says so in the data now
/// (`RacerKind::ConformantControl`) rather than by an empty list and this
/// paragraph — deleting this store used to leave every meta-test green. It is also the shape almost every adapter in this workspace will
/// have — rusqlite behind a connection, a Durable Object, `MemoryEventStore`
/// itself — which is why `happenstance-postgres`, whose writers are *not*
/// serialised, is the instrument the runbook still owes.
#[derive(Debug)]
pub(crate) struct LockedStore(Handle);

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
// The six defects
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
/// row, and `crates/happenstance-testkit/README.md:194-197`, which is where that
/// rule is stated in prose — a concurrency rule's wrong store goes in
/// `racers.rs` and `RACERS` precisely because it can have no `REGISTRY` row.
#[derive(Debug)]
pub(crate) struct RacingProbeStore(Handle);

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

        // THE DEFECT: the window between the answer and the act. Every
        // contender's probe has been answered before any of them acts on it.
        YieldOnce(false).await;
        self.0.rendezvous().await;

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
/// This is the store that makes `k_disjoint_boundaries_never_conflict`
/// worth having rather than being a restatement of
/// `exactly_one_of_n_contenders_commits`: it passes the second, because one
/// winner out of eight on **one** boundary is exactly what it produces.
#[derive(Debug)]
pub(crate) struct GlobalVersionStore(Handle);

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
        // rejection below happen on every run rather than on a lucky one: every
        // contender has read its version before the first of them commits.
        YieldOnce(false).await;
        self.0.rendezvous().await;

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
pub(crate) struct RacingSequenceStore(Handle);

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
        self.0.rendezvous().await;

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
pub(crate) struct GlobalHeadStore(Handle);

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
        let mine = correct::commit(&mut self.0.log(), events, condition, dense)?;

        // The gap between the two statements. The rendezvous waits until every
        // other contender has committed or lost, which is the only thing that
        // makes the second statement's answer differ from the first's. Every
        // committer but the last is then handed somebody else's position, on
        // every run.
        //
        // This used to wait, bounded by yields, for the log to grow past this
        // append. That had the module documentation's flaw on a loaded host,
        // and it left the store's verdict resting on one contender out of
        // sixty-four still landing in time.
        YieldOnce(false).await;
        self.0.rendezvous().await;

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
pub(crate) struct RowAtATimeStore(Handle);

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
            self.0.wait_for_a_reader().await;
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

/// The batch is written, and then the caller is told it was **busy**.
///
/// ES-43 lets a store refuse an append as `AppendError::Busy` only when the
/// batch took no effect. This store breaks exactly that: every second append
/// that writes is answered `Busy(DeadlineElapsed)` after its rows are committed.
/// The rows are correct, the positions are correct, and the log is correct at
/// rest. What is wrong is the answer, and the caller that believes it runs the
/// command again, so an unconditional append lands twice.
///
/// The adapter shape is a deadline around the whole append, such as
/// `tokio::time::timeout(append)` or a driver's statement timeout, that fires
/// after `COMMIT` was sent and is classified as a transient refusal. The same
/// defect appears when an adapter reclassifies every `SQLITE_BUSY` as `Busy`
/// without asking whether the rows were already written. ES-43 requires an
/// outcome the store cannot vouch for to stay `Store`.
///
/// Which appends lie is fixed by commit order, not by the scheduler. The
/// ordinal is taken under the log's lock, and only appends that wrote are
/// counted. So in every rule the first contender to commit after the setup
/// append is the one it lies to, and the verdict is the same on every run.
#[derive(Debug)]
pub(crate) struct BusyAfterWriteStore(Handle);

impl SendEventStore for BusyAfterWriteStore {
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
        let (last, ordinal) = {
            let mut log = self.0.log();
            let last = correct::commit(&mut log, events, condition, dense)?;
            (last, self.0.writes.fetch_add(1, Ordering::AcqRel))
        };

        // THE DEFECT: the batch is committed, and the answer says otherwise.
        if ordinal % 2 == 1 {
            return Err(AppendError::Busy(LogError::DeadlineElapsed));
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

racing_fixture!(
    BusyAfterWriteFixture,
    BusyAfterWriteStore,
    "BusyAfterWriteStore"
);
