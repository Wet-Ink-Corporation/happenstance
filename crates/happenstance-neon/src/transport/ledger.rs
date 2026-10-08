//! The bookkeeping behind [`SqlTransport::reads_settled`](super::SqlTransport::reads_settled).
//!
//! A `std::sync::Mutex` and a table of `Waker`s, and nothing else: no executor,
//! no timer, no atomics, no runtime. That is what lets it work unchanged on
//! `wasm32-unknown-unknown`, where std's mutex is the single-threaded one and
//! the agent that settles a read is a JavaScript promise callback rather than a
//! Rust task.

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

/// The read-only round trips a transport has dispatched and the endpoint has not
/// yet answered.
///
/// A cheap handle: clones share one ledger, and one ledger is one **ordering
/// domain**. A transport holds one, calls [`dispatch`](Self::dispatch) for every
/// read-only request *before* `round_trip` returns, and moves the
/// [`ReadTicket`] to whatever observes the answer — a spawned task on the host,
/// a promise's settlement callback on `wasm32`. Its
/// [`reads_settled`](super::SqlTransport::reads_settled) is then
/// [`settled`](Self::settled).
///
/// Reads are ordered by **generation**, not counted. A waiter waits only for the
/// reads dispatched before it was created, so a steady stream of later reads
/// cannot hold an append back indefinitely.
///
/// ```
/// use core::pin::pin;
/// use core::task::{Context, Poll, Waker};
/// use happenstance_neon::transport::ReadLedger;
///
/// let ledger = ReadLedger::new();
/// let in_flight = ledger.dispatch();
/// let mut settled = pin!(ledger.settled());
/// let mut cx = Context::from_waker(Waker::noop());
///
/// assert_eq!(settled.as_mut().poll(&mut cx), Poll::Pending);
/// in_flight.settle(); // the endpoint answered
/// assert_eq!(settled.as_mut().poll(&mut cx), Poll::Ready(()));
/// ```
#[derive(Debug, Clone, Default)]
pub struct ReadLedger {
    // Shared and mutable by design: the transport dispatches through it, and
    // every task or callback holding a ticket, and every waiting append,
    // mutates it. A lock rather than a channel, because each update is one map
    // operation from synchronous code (`Drop` among it), and nothing here has
    // a task to run an actor on.
    state: Arc<Mutex<LedgerState>>,
}

impl ReadLedger {
    /// An empty ledger: nothing dispatched, nothing waiting.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records one read as dispatched and returns the proof that it has not
    /// settled.
    ///
    /// Call it before `round_trip` returns, and move the ticket to whatever
    /// observes the answer. Dropping the ticket — or calling
    /// [`ReadTicket::settle`] — settles the read.
    #[must_use = "dropping the ticket settles the read immediately"]
    pub fn dispatch(&self) -> ReadTicket {
        let mut state = lock(&self.state);
        let generation = state.next_generation;
        // Saturating rather than wrapping: past 2^64 reads every later one
        // shares the last generation, and `LedgerState::settled_before` treats
        // that generation as preceding every waiter. Possibly a longer wait,
        // never an early one.
        state.next_generation = generation.saturating_add(1);
        let count = state.outstanding.entry(generation).or_insert(0);
        *count = count.saturating_add(1);
        drop(state);
        ReadTicket {
            state: Arc::clone(&self.state),
            generation,
        }
    }

    /// A future that resolves once every read dispatched **before this call**
    /// has settled.
    ///
    /// The horizon is fixed here, at the call, not at the first poll: a read
    /// dispatched between the two does not extend the wait.
    pub fn settled(&self) -> ReadsSettled {
        let horizon = lock(&self.state).next_generation;
        ReadsSettled {
            state: Arc::clone(&self.state),
            horizon,
            slot: None,
        }
    }

    /// How many waiters hold a waker slot.
    #[cfg(test)]
    fn waiter_slots(&self) -> usize {
        lock(&self.state).waiters.len()
    }

    /// Moves the generation counter to its ceiling.
    #[cfg(test)]
    fn saturate_for_test(&self) {
        lock(&self.state).next_generation = u64::MAX;
    }
}

/// Proof that one dispatched read has not settled. Dropping it settles the read.
///
/// `Drop` takes one uncontended lock for one map operation, does no I/O, and
/// wakes the waiters it released only after the lock is released, so it never
/// blocks and never runs a waker under the ledger's lock.
#[derive(Debug)]
pub struct ReadTicket {
    // The ledger's own state, shared for the reason `ReadLedger` gives: the
    // ticket's drop is one of the mutations.
    state: Arc<Mutex<LedgerState>>,
    generation: u64,
}

impl ReadTicket {
    /// Settles the read: the endpoint answered, or the request failed.
    ///
    /// The explicit spelling of dropping the ticket.
    pub fn settle(self) {
        drop(self);
    }
}

impl Drop for ReadTicket {
    fn drop(&mut self) {
        let released = {
            let mut state = lock(&self.state);
            state.release(self.generation);
            state.take_released_waiters()
        };
        for waker in released {
            waker.wake();
        }
    }
}

/// Resolves once every read its [`ReadLedger`] had dispatched when it was
/// created has settled.
///
/// Holds at most one waker slot in the ledger, reused across polls and given
/// back when the future resolves or is dropped.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct ReadsSettled {
    /// The ledger's own state, shared for the reason [`ReadLedger`] gives: a
    /// waiter registers and removes its waker through it.
    state: Arc<Mutex<LedgerState>>,
    /// Generations below this one were dispatched before this future existed.
    horizon: u64,
    /// This future's key in the ledger's waker table, once it has waited.
    slot: Option<u64>,
}

impl Future for ReadsSettled {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // Every field is `Unpin`, so the projection is the trivial one.
        let this = self.get_mut();
        // The check and the registration happen under one lock, so a ticket
        // dropped between them cannot be missed. A waiter this replaces is
        // dropped after the lock is released: a waker's drop is executor code.
        let mut state = lock(&this.state);
        let (poll, replaced) = if state.settled_before(this.horizon) {
            let removed = this
                .slot
                .take()
                .and_then(|slot| state.waiters.remove(&slot));
            (Poll::Ready(()), removed)
        } else {
            let slot = *this.slot.get_or_insert_with(|| state.mint_slot());
            let replaced = match state.waiters.get(&slot) {
                Some(held) if held.waker.will_wake(cx.waker()) => None,
                // The table stores the waker and the context only lends one:
                // this clone is the ownership the ledger needs.
                _ => state.waiters.insert(
                    slot,
                    Waiter {
                        horizon: this.horizon,
                        waker: cx.waker().clone(),
                    },
                ),
            };
            (Poll::Pending, replaced)
        };
        drop(state);
        drop(replaced);
        poll
    }
}

impl Drop for ReadsSettled {
    fn drop(&mut self) {
        if let Some(slot) = self.slot.take() {
            // The guard is a temporary of this statement, so the removed
            // waiter outlives it and is dropped with the lock released.
            let removed = lock(&self.state).waiters.remove(&slot);
            drop(removed);
        }
    }
}

/// One parked waiter: what it waits for, and how to wake it.
#[derive(Debug)]
struct Waiter {
    horizon: u64,
    waker: Waker,
}

#[derive(Debug, Default)]
struct LedgerState {
    /// The generation the next dispatched read receives.
    next_generation: u64,
    /// Generation to the number of unsettled reads carrying it. A count rather
    /// than a set because generations stop being unique at saturation.
    outstanding: BTreeMap<u64, usize>,
    /// The next waker-table key.
    next_slot: u64,
    /// One entry per pending [`ReadsSettled`]; bounded by the live futures.
    waiters: BTreeMap<u64, Waiter>,
}

impl LedgerState {
    /// Whether every read dispatched before `horizon` was fixed has settled.
    ///
    /// The saturated generation is treated as preceding every waiter, because
    /// once the counter stops moving it can no longer say which side of a
    /// horizon a read fell on.
    fn settled_before(&self, horizon: u64) -> bool {
        let oldest_is_later = self
            .outstanding
            .first_key_value()
            .is_none_or(|(oldest, _)| *oldest >= horizon);
        oldest_is_later && !self.outstanding.contains_key(&u64::MAX)
    }

    /// Settles one read of `generation`.
    fn release(&mut self, generation: u64) {
        if let Entry::Occupied(mut count) = self.outstanding.entry(generation) {
            if *count.get() > 1 {
                *count.get_mut() -= 1;
            } else {
                count.remove();
            }
        }
    }

    /// Removes and returns the wakers of every waiter that may now proceed.
    fn take_released_waiters(&mut self) -> Vec<Waker> {
        let mut released = Vec::new();
        let ready: Vec<u64> = self
            .waiters
            .iter()
            .filter(|(_, waiter)| self.settled_before(waiter.horizon))
            .map(|(slot, _)| *slot)
            .collect();
        for slot in ready {
            if let Some(waiter) = self.waiters.remove(&slot) {
                released.push(waiter.waker);
            }
        }
        released
    }

    /// A fresh waker-table key.
    ///
    /// Wrapping: a collision needs one waiter to stay pending across 2^64
    /// others being created.
    fn mint_slot(&mut self) -> u64 {
        let slot = self.next_slot;
        self.next_slot = slot.wrapping_add(1);
        slot
    }
}

/// Locks the ledger, recovering from poison.
///
/// The only foreign code that runs under this lock is a waker's `clone` and
/// `will_wake`, both before the map is touched. Every mutation is a single map
/// operation, so a panic cannot leave the state half-written. No waker is
/// woken or dropped under it.
fn lock(state: &Mutex<LedgerState>) -> MutexGuard<'_, LedgerState> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(test)]
mod tests {
    use core::future::Future;
    use core::pin::pin;
    use core::task::{Context, Poll, Waker};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::Wake;

    use super::{ReadLedger, ReadTicket, ReadsSettled};

    /// A waker that counts its wakes and does nothing else.
    #[derive(Debug, Default)]
    struct Counter(AtomicUsize);

    impl Counter {
        fn wakes(&self) -> usize {
            // `Relaxed`: the count orders nothing else, and every read here is
            // on the thread that did the waking.
            self.0.load(Ordering::Relaxed)
        }
    }

    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            // `Relaxed`, for the reason `wakes` gives.
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn counting() -> (Arc<Counter>, Waker) {
        let counter = Arc::new(Counter::default());
        let waker = Waker::from(Arc::clone(&counter));
        (counter, waker)
    }

    fn poll_once(future: core::pin::Pin<&mut ReadsSettled>, waker: &Waker) -> Poll<()> {
        future.poll(&mut Context::from_waker(waker))
    }

    #[test]
    fn settled_is_ready_when_nothing_is_outstanding() {
        let ledger = ReadLedger::new();
        let (_, waker) = counting();

        let settled = pin!(ledger.settled());

        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
    }

    #[test]
    fn settled_waits_for_a_read_dispatched_before_it() {
        let ledger = ReadLedger::new();
        let (counter, waker) = counting();
        let ticket = ledger.dispatch();
        let mut settled = pin!(ledger.settled());

        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);
        assert_eq!(counter.wakes(), 0, "nothing has settled yet");

        drop(ticket);

        assert_eq!(
            counter.wakes(),
            1,
            "the ticket's drop wakes the waiter once"
        );
        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
    }

    /// I3. A count-to-zero ledger fails this: the later read keeps it non-zero.
    #[test]
    fn settled_ignores_a_read_dispatched_after_it() {
        let ledger = ReadLedger::new();
        let (_, waker) = counting();
        let earlier = ledger.dispatch();
        let mut settled = pin!(ledger.settled());
        let later = ledger.dispatch();

        drop(earlier);

        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Ready(()));
        drop(later);
    }

    #[test]
    fn settling_out_of_order_waits_for_the_oldest() {
        let ledger = ReadLedger::new();
        let (counter, waker) = counting();
        let first = ledger.dispatch();
        let second = ledger.dispatch();
        let mut settled = pin!(ledger.settled());
        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);

        drop(second);

        assert_eq!(counter.wakes(), 0, "the oldest read is still outstanding");
        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);

        drop(first);

        assert_eq!(counter.wakes(), 1);
        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
    }

    /// I5: a ticket nobody settles explicitly still settles when it goes away.
    #[test]
    fn a_dropped_ticket_settles() {
        let ledger = ReadLedger::new();
        let (_, waker) = counting();
        let mut settled = {
            let _in_flight: ReadTicket = ledger.dispatch();
            Box::pin(ledger.settled())
        };

        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Ready(()));
    }

    #[test]
    fn settle_is_drop() {
        let ledger = ReadLedger::new();
        let (counter, waker) = counting();
        let ticket = ledger.dispatch();
        let mut settled = pin!(ledger.settled());
        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);

        ticket.settle();

        assert_eq!(counter.wakes(), 1);
        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
    }

    /// H-13: a waiter polled again and again holds one slot, not one per poll,
    /// and gives it back when it is dropped.
    #[test]
    fn a_repolled_waiter_holds_one_slot() {
        let ledger = ReadLedger::new();
        let (_, waker) = counting();
        let ticket = ledger.dispatch();
        let mut settled = Box::pin(ledger.settled());

        for _ in 0..100 {
            assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);
        }

        assert_eq!(ledger.waiter_slots(), 1);
        drop(settled);
        assert_eq!(ledger.waiter_slots(), 0, "a dropped waiter leaves no slot");
        drop(ticket);
    }

    /// I7, on the host: every type a transport moves into a task or a JS
    /// callback can be moved there.
    #[test]
    fn the_ledger_types_are_send_and_sync() {
        const fn send_and_sync<T: Send + Sync>() {}
        const fn send<T: Send>() {}
        const {
            send_and_sync::<ReadLedger>();
            send_and_sync::<ReadTicket>();
            send::<ReadsSettled>();
        }
    }

    /// A waker that, when woken, records whether the ledger's lock was free.
    struct LockProbe {
        ledger: ReadLedger,
        free_at_wake: Mutex<Vec<bool>>,
    }

    impl Wake for LockProbe {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            let free = self.ledger.state.try_lock().is_ok();
            self.free_at_wake
                .lock()
                .expect("the probe's own lock is never poisoned")
                .push(free);
        }
    }

    /// A waker may run executor code, including code that touches this
    /// ledger again. Waking under the lock would deadlock it.
    #[test]
    fn a_waker_runs_outside_the_lock() {
        let ledger = ReadLedger::new();
        let probe = Arc::new(LockProbe {
            ledger: ledger.clone(),
            free_at_wake: Mutex::new(Vec::new()),
        });
        let waker = Waker::from(Arc::clone(&probe));
        let ticket = ledger.dispatch();
        let mut settled = pin!(ledger.settled());
        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);

        drop(ticket);

        assert_eq!(
            *probe.free_at_wake.lock().expect("never poisoned"),
            vec![true],
            "woken once, with the ledger's lock released"
        );
    }

    /// Two waiters at different horizons: settling the one read only the
    /// earlier waiter was behind releases that waiter and leaves the later one
    /// parked behind the read it was created after.
    ///
    /// Rejects: a release that wakes every waiter on any settlement, and one
    /// that measures every waiter against the newest horizon.
    #[test]
    fn settling_a_read_releases_only_the_waiters_it_preceded() {
        let ledger = ReadLedger::new();
        let (early_counter, early_waker) = counting();
        let (late_counter, late_waker) = counting();
        let first = ledger.dispatch();
        let mut early = pin!(ledger.settled());
        let second = ledger.dispatch();
        let mut late = pin!(ledger.settled());
        assert_eq!(poll_once(early.as_mut(), &early_waker), Poll::Pending);
        assert_eq!(poll_once(late.as_mut(), &late_waker), Poll::Pending);

        drop(first);

        assert_eq!(early_counter.wakes(), 1, "the earlier waiter is released");
        assert_eq!(late_counter.wakes(), 0, "the later one still waits");
        assert_eq!(poll_once(early, &early_waker), Poll::Ready(()));
        assert_eq!(poll_once(late.as_mut(), &late_waker), Poll::Pending);
        assert_eq!(ledger.waiter_slots(), 1, "only the later waiter is parked");

        drop(second);

        assert_eq!(late_counter.wakes(), 1);
        assert_eq!(poll_once(late, &late_waker), Poll::Ready(()));
    }

    /// A thread that panics while holding the ledger's lock poisons it. The
    /// ledger recovers the state rather than failing: later dispatches,
    /// settlements and waits behave exactly as before the panic.
    ///
    /// Rejects: a `lock` that propagates poison, which would turn one
    /// panicking task anywhere in a transport into every later append failing.
    #[test]
    fn a_poisoned_ledger_keeps_working() {
        let ledger = ReadLedger::new();
        let (counter, waker) = counting();
        let in_flight = ledger.dispatch();
        std::thread::scope(|scope| {
            let poisoner = scope.spawn(|| {
                let _held = ledger.state.lock();
                panic!("a deliberate panic while the ledger's lock is held");
            });
            assert!(poisoner.join().is_err(), "the thread panicked");
        });
        assert!(ledger.state.is_poisoned(), "the panic poisoned the lock");

        let mut settled = pin!(ledger.settled());
        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);
        drop(in_flight);

        assert_eq!(counter.wakes(), 1, "the settlement still wakes the waiter");
        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
        let after = ledger.dispatch();
        let mut next = pin!(ledger.settled());
        assert_eq!(poll_once(next.as_mut(), &waker), Poll::Pending);
        drop(after);
        assert_eq!(poll_once(next, &waker), Poll::Ready(()));
    }

    /// Past 2^64 dispatches every read shares the last generation. A waiter
    /// then waits for all of them: possibly longer, never early.
    #[test]
    fn a_saturated_ledger_waits_rather_than_releasing_early() {
        let ledger = ReadLedger::new();
        let (_, waker) = counting();
        ledger.saturate_for_test();
        let before = ledger.dispatch();
        let mut settled = pin!(ledger.settled());

        assert_eq!(poll_once(settled.as_mut(), &waker), Poll::Pending);
        drop(before);
        assert_eq!(poll_once(settled, &waker), Poll::Ready(()));
    }
}
