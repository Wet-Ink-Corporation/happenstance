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
//! # The rendezvous, and why it is not a sleep
//!
//! A wrong store that is *sometimes* caught is worse than useless in a proof
//! artefact: it turns the meta-test into a coin toss and teaches the next
//! reader to re-run. Every window below is therefore closed by a **rendezvous**
//! rather than by luck. A sleep would still be a race, run against a wall clock
//! on a loaded runner, and it is what CF-33 forbids the rules for that reason.
//!
//! # The second mechanism, and why the first was replaced
//!
//! The first rendezvous was an atomic counter and [`std::thread::yield_now`],
//! bounded at 8,000 yields and closed once the arrival count had stopped
//! growing for 512 of them. That is a timeout spelled in yields. A yield
//! returns at once when nothing else is runnable on its core, so 8,000 of them
//! can be over in a few milliseconds. A contender that the rule's starting
//! barrier has released but the scheduler has not yet run can take longer than
//! that. The window then closes without it, and the late contender probes
//! *after* the early ones committed.
//!
//! That turns a rejection into a correct result. In
//! `exactly_one_of_n_contenders_commits`, a `RacingProbeStore` contender that
//! waits out its bound alone commits, and every later contender sees its
//! commit and is told `ConditionViolated`. That is one winner, which is what a
//! correct store produces. In `k_disjoint_boundaries_never_conflict`, a
//! `GlobalVersionStore` contender that commits alone has a current version, so
//! it wins. Once each boundary has had one such contender, every boundary has a
//! winner and the rule passes. The meta-test passed when run on its own and
//! failed under `cargo xtask ci`. Measured on 2026-10-08 on a four-core host
//! that another build was also using: the whole test binary, run 200 times,
//! failed this way on 32 runs, and on 14 runs with two busy loops beside it.
//! The failures were `RacingProbeStore` on both of its rules and
//! `GlobalVersionStore` on its one. With the mechanism below,
//! the same two runs failed none of 400, and the meta-test alone beside eight
//! busy loops failed none of 200.
//!
//! The party is now known rather than guessed. Every handle is counted from
//! `connect` until it is dropped, in a [`Party`] that [`Shared`] keeps under one
//! mutex. Every concurrency rule connects all of its contenders on its own
//! thread before it starts any of them, so when the first contender reaches a
//! window the count already includes the last. A window waits for that count,
//! and for nothing that depends on the scheduler. It is an async barrier made
//! of a `Mutex` and the contenders' `Waker`s, so a waiting contender parks its
//! thread in [`happenstance_testkit::block_on`] rather than spinning.
//!
//! The one handle in the family that is live and never appends is
//! `a_concurrent_reader_never_sees_a_partial_batch`'s reader. It reads once on
//! the rule's thread before any writer is spawned, so a handle stops being a
//! contender at its first read, and that read happens before any writer thread
//! starts.
//!
//! **Nothing here is bounded, and that is deliberate.** Each wait ends when
//! the party changes in a way it is guaranteed to change: every contender
//! arrives, or drops its handle, and a contender that panics drops its handle
//! while it unwinds. A store that waits for company that is not coming is a
//! defect in this file, and CF-33's reasoning applies to it as it does to the
//! rules: the CI job timeout is what notices a hang, and a hang is better than
//! a verdict the scheduler decided.
//!
//! # How to add one
//!
//! 1. Write the store here, wrong in exactly one way, `Send + Sync`, and close
//!    its window with one of [`Shared`]'s rendezvous. Check that every rule
//!    connects the handles that will meet there before it starts them.
//! 2. Add its fixture to `for_each_racer!` in `tests/mutation_coverage.rs`.
//! 3. Add its row to `RACERS` in the same file, naming the exact set of
//!    concurrency rules it fails and the real adapter shape it comes from.

use core::future::{Future, poll_fn};
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

/// Who holds a handle onto one backing store, and who is waiting where.
///
/// Every rendezvous below is a predicate over this struct, checked and armed
/// under the one mutex that guards it. Every change to it wakes every waiter so
/// that each can check its predicate again. That is a thundering herd of at
/// most `CONTENDERS` threads, and it is what makes a lost wake-up impossible to
/// write here: no waiter can test a predicate between a change and its wake.
#[derive(Debug, Default)]
struct Party {
    /// Live handles that have never read. These are the contenders a
    /// [`Shared::cohort`] waits for.
    contenders: usize,
    /// Live handles that have read at least once.
    readers: usize,
    /// Contenders waiting in the current cohort.
    in_cohort: usize,
    /// Cohorts released so far. A waiter leaves when this moves past the value
    /// it arrived at, so a released waiter cannot be held back by the next
    /// cohort filling up behind it.
    cohorts: u64,
    /// Handles waiting in [`Shared::wait_for_growth`].
    awaiting_growth: usize,
    /// Completed reads.
    reads: u64,
    /// The waiters to wake on the next change. At most one per waiting task,
    /// because [`Party::listen`] does not add a waker twice.
    wakers: Vec<Waker>,
}

impl Party {
    /// Releases the current cohort if every contender is in it.
    ///
    /// `>=` rather than `==`, because a handle that has read is not a
    /// contender and may still append. No rule does that while contenders are
    /// waiting, and if one did, an early release is a verdict the table would
    /// report, where a missed release would be a hang.
    fn release_if_complete(&mut self) {
        if self.in_cohort > 0 && self.in_cohort >= self.contenders {
            self.in_cohort = 0;
            self.cohorts += 1;
        }
    }

    /// A handle has read for the first time, so it is a reader from now on.
    fn becomes_reader(&mut self) {
        self.contenders -= 1;
        self.readers += 1;
        self.release_if_complete();
    }

    /// A handle has been dropped.
    fn leave(&mut self, was_reader: bool) {
        if was_reader {
            self.readers -= 1;
        } else {
            self.contenders -= 1;
            self.release_if_complete();
        }
    }

    /// Registers `waker` for the next change, unless it is registered already.
    fn listen(&mut self, waker: &Waker) {
        if !self.wakers.iter().any(|known| known.will_wake(waker)) {
            // Cloned because it is stored: the `&Waker` a poll is handed
            // lives only as long as that poll.
            self.wakers.push(waker.clone());
        }
    }
}

/// One handle's membership of its store's [`Party`].
///
/// Every store below wraps one of these rather than a bare `Arc<Shared>`,
/// because a handle's arrival, first read and drop are the events the party
/// count is made of. The count is right only if no handle can be made without
/// [`Handle::join`] and none can be dropped without its `Drop`.
#[derive(Debug)]
struct Handle {
    /// The backing store every handle of one fixture shares.
    shared: Arc<Shared>,
    /// Whether this handle has read, and so is no longer a contender.
    has_read: AtomicBool,
}

impl Handle {
    /// Opens a handle, counting it as a contender.
    fn join(shared: Arc<Shared>) -> Self {
        shared.update(|party| party.contenders += 1);
        Self {
            shared,
            has_read: AtomicBool::new(false),
        }
    }

    /// The read every store here shares. The handle's first read makes it a
    /// reader.
    fn snapshot(&self, query: &Query, options: ReadOptions) -> Snapshot {
        // `Relaxed` is enough. A `swap` is one atomic read-modify-write, so
        // exactly one read sees `false` and makes the transition, and the
        // transition itself is ordered by the party mutex.
        if !self.has_read.swap(true, Ordering::Relaxed) {
            self.shared.update(Party::becomes_reader);
        }
        self.shared.select(query, options)
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        let was_reader = *self.has_read.get_mut();
        self.shared.update(|party| party.leave(was_reader));
    }
}

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
    /// Who holds a handle, and who is waiting where. A mutex rather than a set
    /// of atomics because every rendezvous reads several of these counts
    /// together, and a waiter that read them at different moments could miss
    /// the change that releases it.
    party: Mutex<Party>,
    /// How many appends have **written**, counted under the log's lock so the
    /// ordinal follows commit order. `BusyAfterWriteStore` lies on the odd
    /// ones, which makes which batches it lies about a property of the order
    /// they committed in rather than of the scheduler.
    writes: AtomicUsize,
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

    /// Locks the party, ignoring poisoning for [`Shared::log`]'s reason.
    ///
    /// Lock order: a caller may take [`Shared::log`] while holding this, and
    /// never the other way round. Nothing here holds the log while it changes
    /// the party.
    fn party(&self) -> MutexGuard<'_, Party> {
        self.party.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Applies `change` to the party, then wakes every waiter so that each
    /// checks its predicate again.
    ///
    /// The wakers are taken under the lock and woken after it is released. A
    /// woken waiter's first act is to take this lock, and it should not find
    /// the lock still held.
    fn update<R>(&self, change: impl FnOnce(&mut Party) -> R) -> R {
        let (result, waiting) = {
            let mut party = self.party();
            let result = change(&mut party);
            (result, core::mem::take(&mut party.wakers))
        };
        for waker in waiting {
            waker.wake();
        }
        result
    }

    /// Waits until **every** contender is waiting here, then releases them all
    /// together.
    ///
    /// # Four versions, and why the first three were wrong
    ///
    /// All three wrong versions failed the same way: the store went on being
    /// rejected *most* of the time. That is the worst failure mode for an
    /// instrument, because it looks like success.
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
    /// **Cumulative arrivals, settled over 512 yields and bounded at 8,000.**
    /// A counter that never goes down cannot shrink under a late arrival, so
    /// on an idle host the cohort did not close until the last contender was in
    /// it. Under load it did. A contender that is runnable but waiting for a
    /// core does not arrive within 512 yields, so "stopped growing" said nothing
    /// about whether it had finished growing. The module documentation has the
    /// measurement.
    ///
    /// **The party, counted.** The version below does not ask whether arrivals
    /// have stopped. It asks whether everyone has arrived, and [`Party`] knows
    /// who everyone is, because every rule connects its contenders before it
    /// starts them. A contender that is waiting for a core is still counted, so
    /// the cohort waits for it.
    ///
    /// # Why it cannot hang
    ///
    /// A contender that has not arrived is doing one of three things. It is on
    /// its way, and arrives. Its own probe rejected it, and it returns and drops
    /// its handle. Or it panicked, and drops its handle while it unwinds. Every
    /// one of those is a change to the party, and the change releases the cohort
    /// once the remaining contenders are all in it. No contender can be
    /// rejected by a commit *in* this cohort, because nobody commits until it is
    /// released.
    ///
    /// That argument assumes the wait is driven to completion, which `block_on`
    /// always does. A future dropped mid-wait would leave its count raised and
    /// release a later cohort early rather than hang; nothing here drops one.
    ///
    /// It also assumes every contender is one of those three, and that is a
    /// precondition on the rules rather than something this store can check.
    /// **No rule may hold open, across a race, a handle that has never read and
    /// does not append in that race.** A handle stops being a contender only at
    /// its first read or when it is dropped, and an earlier append does not
    /// release it. Such a handle counts as a contender that never arrives, so
    /// the cohort waits for it forever, and the binary hangs until the CI
    /// timeout without naming a rule. Every rule
    /// meets it today. `race` moves each handle into a contender that appends
    /// and drops it, and setup handles are dropped before the race starts.
    /// Observers are connected after the race, and `observe_while_writing`'s
    /// reader reads before any writer starts.
    async fn cohort(&self) {
        let cohort = self.update(|party| {
            let arrived_at = party.cohorts;
            party.in_cohort += 1;
            party.release_if_complete();
            arrived_at
        });
        poll_fn(|cx| {
            let mut party = self.party();
            if party.cohorts == cohort {
                party.listen(cx.waker());
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
    }

    /// Waits until the committed log holds more than `len` events, or until
    /// every live contender is waiting here too and none is left to grow it.
    ///
    /// The second condition is the one that keeps this from hanging, and it is
    /// exact. A contender that is not waiting here is about to append, which
    /// grows the log and then arrives here, or is about to drop its handle.
    /// Either way the party changes, so every waiter checks again.
    ///
    /// The log is read under the party lock, which is the lock order
    /// [`Shared::party`] documents. A commit made between this check and the
    /// next one cannot be missed, because the committer then arrives here or
    /// drops its handle, and both of those wake every waiter.
    async fn wait_for_growth(&self, len: usize) {
        self.update(|party| party.awaiting_growth += 1);
        poll_fn(|cx| {
            let mut party = self.party();
            let nobody_left = party.awaiting_growth >= party.contenders;
            if nobody_left || self.log().len() > len {
                Poll::Ready(())
            } else {
                party.listen(cx.waker());
                Poll::Pending
            }
        })
        .await;
        self.update(|party| party.awaiting_growth -= 1);
    }

    /// Waits until a read that *started after this call* has finished, unless
    /// no live handle has ever read.
    ///
    /// Two completions rather than one: a read already in flight when this is
    /// called may have sampled the store before the partial write, so only the
    /// second completion is guaranteed to have seen it.
    ///
    /// # Why the early-out is exact now
    ///
    /// The early-out used to be `reads == 0`, which is true both when a rule
    /// has no reader and when its reader exists but has not been scheduled yet.
    /// That was measured on an oversubscribed host: this store went unrejected
    /// on ten runs in twenty-four. `observe_while_writing` in the testkit now
    /// completes one read on the rule's own thread before any writer is spawned.
    /// That read makes its handle a reader in [`Party`] before any writer
    /// starts, so `readers == 0` means "this rule has no reader". The reader
    /// reads in a loop until every writer has joined, so a writer that waits
    /// here is always answered.
    async fn wait_for_a_reader(&self) {
        let start = self.party().reads;
        poll_fn(|cx| {
            let mut party = self.party();
            if party.readers == 0 || party.reads >= start + 2 {
                Poll::Ready(())
            } else {
                party.listen(cx.waker());
                Poll::Pending
            }
        })
        .await;
    }

    /// A snapshot of the log, plus the count that lets a writer wait for one.
    ///
    /// Reached only through [`Handle::snapshot`], which keeps the reader count.
    fn select(&self, query: &Query, options: ReadOptions) -> Snapshot {
        let selected = correct::select(&self.log(), query, options);
        // Counted after the log is released, so a writer waiting on it knows
        // the read is over rather than merely started.
        self.update(|party| party.reads += 1);
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
                // `Arc::clone` is the second connection: one more owner of
                // the one backing store, counted into its party by `join`.
                $store(Handle::join(Arc::clone(&self.0)))
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
        correct::commit(&mut self.0.shared.log(), events, condition, dense)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
            return correct::commit(&mut self.0.shared.log(), events, None, dense);
        };

        // The probe, answered correctly, against everything committed so far.
        // The guard is scoped so that none is held across the suspension below —
        // `clippy::await_holding_lock` is a workspace deny, and holding one here
        // would give the store a second defect nobody declared.
        if let Some(conflict) = correct::violation(&self.0.shared.log(), condition) {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        // THE DEFECT: the window between the answer and the act. Every
        // contender waits here until all of them have probed, so every probe
        // is answered before any insert.
        YieldOnce(false).await;
        self.0.shared.cohort().await;

        // The insert, with the probe's verdict already spent.
        let mut log = self.0.shared.log();
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
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
            return correct::commit(&mut self.0.shared.log(), events, None, dense);
        };

        let version = {
            let log = self.0.shared.log();
            if let Some(conflict) = correct::violation(&log, condition) {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict,
                )));
            }
            log.len()
        };

        // The transaction doing its work. The rendezvous is what makes the
        // rejection below happen on every run rather than on a lucky one:
        // every contender read the version before any of them commits.
        YieldOnce(false).await;
        self.0.shared.cohort().await;

        let mut log = self.0.shared.log();
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
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
        let stale = SequencePosition::new(self.0.shared.head.load(Ordering::Acquire));

        // Every contender has read the counter before any of them writes it.
        YieldOnce(false).await;
        self.0.shared.cohort().await;

        // Probe and insert under one lock, so the decision is atomic and this
        // store is wrong about exactly one thing.
        let _serialised = self
            .0
            .shared
            .appending
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let mut log = self.0.shared.log();
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
        self.0.shared.head.store(last.get(), Ordering::Release);
        Ok(last)
    }

    // Answered from the log rather than from `Shared::head`, which is the only
    // store here where the two can disagree. The atomic *is* this store's
    // declared defect — a counter read outside the serialising lock — and
    // reporting it as the head would be that defect leaking into a second
    // operation nobody declared it in.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
        let (mine, len) = {
            let mut log = self.0.shared.log();
            let mine = correct::commit(&mut log, events, condition, dense)?;
            (mine, log.len())
        };

        // The gap between the two statements. The rendezvous waits for somebody
        // else's rows to land, which is the only thing that makes the second
        // statement's answer differ from the first's.
        YieldOnce(false).await;
        self.0.shared.wait_for_growth(len).await;

        // THE DEFECT: `max(position)` over the whole table, which is whoever
        // committed last rather than whoever is asking.
        Ok(self.0.shared.committed_head().unwrap_or(mine))
    }

    // Correct, and identical to every other store's — which is the point. This
    // store's defect is that its *`append`* returns the store's head; `head`
    // itself returning the store's head is what `head` is for.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
            let mut log = self.0.shared.log();
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
            self.0.shared.wait_for_a_reader().await;
            YieldOnce(false).await;

            let mut log = self.0.shared.log();
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
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
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
            let mut log = self.0.shared.log();
            let last = correct::commit(&mut log, events, condition, dense)?;
            (last, self.0.shared.writes.fetch_add(1, Ordering::AcqRel))
        };

        // THE DEFECT: the batch is committed, and the answer says otherwise.
        if ordinal % 2 == 1 {
            return Err(AppendError::Busy(LogError::DeadlineElapsed));
        }
        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.0.shared.committed_head())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.0.shared.holds(id))
    }
}

racing_fixture!(
    BusyAfterWriteFixture,
    BusyAfterWriteStore,
    "BusyAfterWriteStore"
);
