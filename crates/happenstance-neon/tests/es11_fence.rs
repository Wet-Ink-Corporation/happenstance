//! ES-11's fence, offline: an `append` waits for the reads the same transport
//! dispatched before it, and for nothing else.
//!
//! Written as a downstream transport author would meet the crate — public API
//! only — over [`OrderingTransport`], a fake that answers nothing until the test
//! says so. Every future is polled by hand with a counting waker. No executor,
//! no timer and no runtime is involved, which is the offline half of the claim
//! that the fence needs none of them on `wasm32`.
//!
//! The live half — whether the endpoint honours "answered before dispatched" —
//! is not testable here. It was measured against the live endpoint, with the
//! fence and without it, and ADR-0087 records the sweep and its result; the
//! live conformance suite's two racing rules are its standing check.

use core::cell::RefCell;
use core::future::Future;
use core::pin::{Pin, pin};
use core::task::{Context, Poll, Waker};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Wake;

use futures_core::Stream;
use happenstance_core::{
    Event, EventId, EventStore, Query, ReadOptions, SequencePosition, StoreId,
};
use happenstance_neon::SqlTransport;
use happenstance_neon::transport::{ReadLedger, ReadTicket};
use happenstance_neon::{HttpResponse, NeonConfig, NeonError, NeonEventStore, SqlRequest};

/// A row recorded from the live endpoint, through the adapter's own `SELECT`:
/// what every read answers, standing for the log before the append.
const READ_ROWS: &[u8] = br#"{"fields":[],"command":"SELECT","rowCount":1,"rows":[{"position":"2","event_type":"Enrolled","data":"3q2+7w==","metadata":null,"tags":["course:c1","student:s1"],"origin_store":"HYfjIWOxSmyhl/na/L7bag==","origin_position":"2","recorded_at":"1757203200000"}]}"#;

/// The append's answer, in the recorded two-result-set shape: no conflict, one
/// row inserted at position 3.
const APPENDED: &[u8] = br#"{"results":[{"fields":[],"rows":[{"conflict":null}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[{"position":"3"}],"command":"INSERT","rowCount":1,"rowAsArray":false}]}"#;

/// What the transport saw, in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Logged {
    /// `round_trip` was called with this request.
    Dispatched { index: usize, read_only: bool },
    /// The request at `index` was answered.
    Answered { index: usize },
}

/// Who holds a read's ticket while it is in flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Settlement {
    /// The transport, until the test answers the read. The shape every
    /// conforming transport has: settlement is driven by the transport.
    ByTransport,
    /// The returned future, so dropping it settles the read: a transport whose
    /// drop cancels the request.
    OnDrop,
}

/// What a read is answered with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reads {
    Rows,
    Fail,
}

/// Whether a write is answered at once or waits for the test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Writes {
    AnsweredAtOnce,
    Parked,
}

/// One answer slot, shared between the transport and the future it returned.
#[derive(Debug, Default)]
struct Slot {
    answer: Option<Result<HttpResponse, std::io::Error>>,
    waker: Option<Waker>,
}

/// A request the transport has not answered yet.
#[derive(Debug)]
struct Parked {
    index: usize,
    read_only: bool,
    ticket: Option<ReadTicket>,
    slot: Rc<RefCell<Slot>>,
}

#[derive(Debug, Default)]
struct State {
    log: Vec<Logged>,
    next_index: usize,
    parked: Vec<Parked>,
}

/// A fake transport that dispatches eagerly, as the trait requires, and
/// answers only when the test releases a request.
#[derive(Debug)]
struct OrderingTransport {
    ledger: ReadLedger,
    settlement: Settlement,
    reads: Reads,
    writes: Writes,
    state: RefCell<State>,
}

impl OrderingTransport {
    fn new() -> Self {
        Self {
            ledger: ReadLedger::new(),
            settlement: Settlement::ByTransport,
            reads: Reads::Rows,
            writes: Writes::AnsweredAtOnce,
            state: RefCell::new(State::default()),
        }
    }

    fn cancel_on_drop() -> Self {
        Self {
            settlement: Settlement::OnDrop,
            ..Self::new()
        }
    }

    fn failing_reads() -> Self {
        Self {
            reads: Reads::Fail,
            ..Self::new()
        }
    }

    fn parking_writes() -> Self {
        Self {
            writes: Writes::Parked,
            ..Self::new()
        }
    }

    fn log(&self) -> Vec<Logged> {
        self.state.borrow().log.clone()
    }

    fn writes_dispatched(&self) -> usize {
        self.log()
            .iter()
            .filter(|entry| {
                matches!(
                    entry,
                    Logged::Dispatched {
                        read_only: false,
                        ..
                    }
                )
            })
            .count()
    }

    /// Answers every parked read.
    fn release_reads(&self) {
        self.release_where(|parked| parked.read_only);
    }

    /// Answers every parked write.
    fn release_writes(&self) {
        self.release_where(|parked| !parked.read_only);
    }

    /// Answers the one request dispatched at `index`.
    fn release(&self, index: usize) {
        self.release_where(|parked| parked.index == index);
    }

    fn release_where(&self, chosen: impl Fn(&Parked) -> bool) {
        let released: Vec<Parked> = {
            let mut state = self.state.borrow_mut();
            let (released, kept) = core::mem::take(&mut state.parked)
                .into_iter()
                .partition(|parked| chosen(parked));
            state.parked = kept;
            for parked in &released {
                state.log.push(Logged::Answered {
                    index: parked.index,
                });
            }
            released
        };
        for parked in released {
            let waker = {
                let mut slot = parked.slot.borrow_mut();
                slot.answer = Some(self.answer(parked.read_only));
                slot.waker.take()
            };
            // The answer exists before the read settles, as on the wire.
            drop(parked.ticket);
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }

    fn answer(&self, read_only: bool) -> Result<HttpResponse, std::io::Error> {
        match (read_only, self.reads) {
            (true, Reads::Rows) => Ok(HttpResponse::new(200, READ_ROWS)),
            (true, Reads::Fail) => Err(std::io::Error::other("the fetch was refused")),
            (false, _) => Ok(HttpResponse::new(200, APPENDED)),
        }
    }
}

/// The future `round_trip` returns: it waits on its slot.
#[derive(Debug)]
struct Answer {
    slot: Rc<RefCell<Slot>>,
    /// Held only under [`Settlement::OnDrop`].
    ticket: Option<ReadTicket>,
}

impl Future for Answer {
    type Output = Result<HttpResponse, std::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let mut slot = this.slot.borrow_mut();
        if let Some(answer) = slot.answer.take() {
            this.ticket = None;
            return Poll::Ready(answer);
        }
        // The slot stores the waker and the context only lends one.
        slot.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl SqlTransport for &OrderingTransport {
    type Error = std::io::Error;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        let read_only = request.read_only;
        let mut state = self.state.borrow_mut();
        let index = state.next_index;
        state.next_index += 1;
        state.log.push(Logged::Dispatched { index, read_only });
        let slot = Rc::new(RefCell::new(Slot::default()));

        if !read_only && self.writes == Writes::AnsweredAtOnce {
            slot.borrow_mut().answer = Some(self.answer(false));
            state.log.push(Logged::Answered { index });
            return Answer { slot, ticket: None };
        }

        // Eager: registered before this method returns.
        let ticket = read_only.then(|| self.ledger.dispatch());
        let (parked_ticket, future_ticket) = match self.settlement {
            Settlement::ByTransport => (ticket, None),
            Settlement::OnDrop => (None, ticket),
        };
        state.parked.push(Parked {
            index,
            read_only,
            ticket: parked_ticket,
            slot: Rc::clone(&slot),
        });
        Answer {
            slot,
            ticket: future_ticket,
        }
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        self.ledger.settled()
    }
}

/// A waker that counts its wakes.
#[derive(Debug, Default)]
struct Counter(AtomicUsize);

impl Counter {
    fn wakes(&self) -> usize {
        // `Relaxed`: the count orders nothing else, and it is read on the
        // thread that woke it.
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

fn poll<F: Future>(future: Pin<&mut F>, waker: &Waker) -> Poll<F::Output> {
    future.poll(&mut Context::from_waker(waker))
}

fn poll_next<S: Stream>(stream: Pin<&mut S>, waker: &Waker) -> Poll<Option<S::Item>> {
    stream.poll_next(&mut Context::from_waker(waker))
}

fn store(transport: &OrderingTransport) -> NeonEventStore<&OrderingTransport> {
    NeonEventStore::new(transport, NeonConfig::default())
}

fn later() -> [Event; 1] {
    [Event::new("Later", b"\x01".to_vec()).expect("a valid event")]
}

/// The position [`APPENDED`] reports.
const APPENDED_AT: u64 = 3;

/// A1. The rule's shape exactly: poll the read once, then append. The write
/// must not leave until the read is answered.
///
/// Rejects: no fence; and a fence keyed on the stream's first poll or on its
/// dispatch rather than on its answer.
#[test]
fn an_append_after_a_first_poll_waits_for_the_read_to_be_answered() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    let mut read = pin!(store.read(&query, ReadOptions::default()));
    assert!(poll_next(read.as_mut(), &waker).is_pending());
    let mut append = pin!(store.append(&events, None));

    let first = poll(append.as_mut(), &waker);

    assert!(first.is_pending(), "the append finished: {first:?}");
    assert_eq!(
        transport.log(),
        [Logged::Dispatched {
            index: 0,
            read_only: true
        }],
        "no write may be dispatched while the read is unanswered"
    );

    transport.release_reads();

    let appended = poll(append, &waker);
    assert!(
        matches!(&appended, Poll::Ready(Ok(position)) if position.get() == APPENDED_AT),
        "got {appended:?}"
    );
    assert_eq!(
        transport.log(),
        [
            Logged::Dispatched {
                index: 0,
                read_only: true
            },
            Logged::Answered { index: 0 },
            Logged::Dispatched {
                index: 1,
                read_only: false
            },
            Logged::Answered { index: 1 },
        ]
    );
}

/// A2 (I2). The fence is released by the transport answering the read, not
/// by anyone polling the stream. F0 — a release that lives in the stream —
/// never wakes the append here, because the only task that could poll the
/// stream is the one parked inside `append`.
#[test]
fn the_fence_releases_without_the_stream_being_polled() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let (_, stream_waker) = counting();
    let (woken, append_waker) = counting();
    let events = later();
    let query = Query::all();
    let mut read = pin!(store.read(&query, ReadOptions::default()));
    assert!(poll_next(read.as_mut(), &stream_waker).is_pending());
    let mut append = pin!(store.append(&events, None));
    assert!(poll(append.as_mut(), &append_waker).is_pending());

    transport.release_reads();

    assert!(
        woken.wakes() >= 1,
        "the append must be woken before the stream is polled again"
    );
    let appended = poll(append, &append_waker);
    assert!(
        matches!(&appended, Poll::Ready(Ok(position)) if position.get() == APPENDED_AT),
        "got {appended:?}"
    );

    let mut drained = Vec::new();
    while let Poll::Ready(Some(item)) = poll_next(read.as_mut(), &stream_waker) {
        drained.push(item.expect("the canned rows decode").position.get());
    }
    assert_eq!(drained, [2], "exactly the rows the read was answered with");
}

/// A3 (I5). A stream dropped mid-flight leaves a read the transport still
/// settles when the request is answered.
#[test]
fn a_dropped_in_flight_stream_still_lets_an_append_through() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    {
        let mut read = Box::pin(store.read(&query, ReadOptions::default()));
        assert!(poll_next(read.as_mut(), &waker).is_pending());
    }
    let mut append = pin!(store.append(&events, None));
    assert!(poll(append.as_mut(), &waker).is_pending());

    transport.release_reads();

    assert!(matches!(poll(append, &waker), Poll::Ready(Ok(_))));
}

/// A3 (I5), the other settlement shape: a transport whose dropped future
/// cancels the request settles the read at the drop, with no answer at all.
///
/// Rejects: a registration that leaks when its future is dropped.
#[test]
fn a_dropped_stream_over_a_cancelling_transport_settles_at_the_drop() {
    let transport = OrderingTransport::cancel_on_drop();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    {
        let mut read = Box::pin(store.read(&query, ReadOptions::default()));
        assert!(poll_next(read.as_mut(), &waker).is_pending());
    }
    let mut append = pin!(store.append(&events, None));

    let appended = poll(append.as_mut(), &waker);

    assert!(matches!(appended, Poll::Ready(Ok(_))), "got {appended:?}");
    assert_eq!(transport.writes_dispatched(), 1);
}

/// A4 (I4). Appends never wait on appends: with no read outstanding, two
/// appends both reach the wire before either is answered.
///
/// Rejects: a fence that waits on every request, which would turn the
/// concurrency family's race into a queue.
#[test]
fn concurrent_appends_are_not_queued_behind_each_other() {
    let transport = OrderingTransport::parking_writes();
    let store = store(&transport);
    let (_, waker) = counting();
    let (first_events, second_events) = (later(), later());
    let mut first = pin!(store.append(&first_events, None));
    let mut second = pin!(store.append(&second_events, None));

    assert!(poll(first.as_mut(), &waker).is_pending());
    assert!(poll(second.as_mut(), &waker).is_pending());

    assert_eq!(
        transport.writes_dispatched(),
        2,
        "both writes are in flight at once"
    );
    transport.release_writes();
    assert!(matches!(poll(first, &waker), Poll::Ready(Ok(_))));
    assert!(matches!(poll(second, &waker), Poll::Ready(Ok(_))));
}

/// A5 (I6). A stream that is built and never polled has sent nothing, and
/// costs an append nothing.
///
/// Rejects: registering a read when `read` is called rather than when its
/// request is dispatched, which also breaks the laziness `wasm32` needs.
#[test]
fn an_unpolled_stream_costs_an_append_nothing() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    let read = store.read(&query, ReadOptions::default());
    let mut append = pin!(store.append(&events, None));

    let appended = poll(append.as_mut(), &waker);

    assert!(matches!(appended, Poll::Ready(Ok(_))), "got {appended:?}");
    assert_eq!(
        transport.log(),
        [
            Logged::Dispatched {
                index: 0,
                read_only: false
            },
            Logged::Answered { index: 0 },
        ]
    );
    drop(read);
}

/// A6. A read that fails still settles: the stream reports the failure and
/// the append proceeds.
#[test]
fn a_failed_read_settles_and_the_append_proceeds() {
    let transport = OrderingTransport::failing_reads();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    let mut read = pin!(store.read(&query, ReadOptions::default()));
    assert!(poll_next(read.as_mut(), &waker).is_pending());
    let mut append = pin!(store.append(&events, None));
    assert!(poll(append.as_mut(), &waker).is_pending());

    transport.release_reads();

    assert!(matches!(poll(append, &waker), Poll::Ready(Ok(_))));
    assert!(matches!(
        poll_next(read, &waker),
        Poll::Ready(Some(Err(NeonError::Transport(_))))
    ));
}

/// A7 (I3), at the adapter: a read dispatched while an append waits does not
/// extend the wait.
///
/// Rejects: a fence that waits for the ledger to empty rather than for the
/// reads that preceded it.
#[test]
fn a_read_dispatched_while_an_append_waits_does_not_extend_the_wait() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let (_, waker) = counting();
    let events = later();
    let query = Query::all();
    let mut earlier = pin!(store.read(&query, ReadOptions::default()));
    assert!(poll_next(earlier.as_mut(), &waker).is_pending());
    let mut append = pin!(store.append(&events, None));
    assert!(poll(append.as_mut(), &waker).is_pending());
    let mut overtaking = pin!(store.read(&query, ReadOptions::default()));
    assert!(poll_next(overtaking.as_mut(), &waker).is_pending());

    transport.release(0);

    assert!(matches!(poll(append, &waker), Poll::Ready(Ok(_))));
    assert!(
        !transport.log().contains(&Logged::Answered { index: 1 }),
        "the later read is still in flight"
    );
    transport.release_reads();
    assert!(poll_next(overtaking, &waker).is_ready());
}

/// Polls `in_flight` once, so its read is dispatched, then checks that an
/// append waits for that read's answer and goes out after it.
fn assert_an_append_waits_behind<F: Future>(
    transport: &OrderingTransport,
    store: &NeonEventStore<&OrderingTransport>,
    mut in_flight: Pin<&mut F>,
) -> Poll<F::Output> {
    let (_, waker) = counting();
    let events = later();
    assert!(poll(in_flight.as_mut(), &waker).is_pending());
    let mut append = pin!(store.append(&events, None));

    assert!(poll(append.as_mut(), &waker).is_pending());
    assert_eq!(
        transport.log(),
        [Logged::Dispatched {
            index: 0,
            read_only: true
        }],
        "no write may be dispatched while the read is unanswered"
    );

    transport.release_reads();

    assert!(matches!(poll(append, &waker), Poll::Ready(Ok(_))));
    assert_eq!(
        transport.log().get(1..3),
        Some(
            &[
                Logged::Answered { index: 0 },
                Logged::Dispatched {
                    index: 1,
                    read_only: false
                },
            ][..]
        ),
        "the write leaves only after the read is answered"
    );
    poll(in_flight, &waker)
}

/// D13: `head` is a read-only request, so an append waits behind one in
/// flight exactly as it waits behind a stream.
///
/// Rejects: an adapter that sends `head` without marking it read-only, so no
/// transport ever registers it.
#[test]
fn an_append_waits_behind_an_in_flight_head() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let head = pin!(store.head());

    let answered = assert_an_append_waits_behind(&transport, &store, head);

    assert!(matches!(answered, Poll::Ready(Ok(_))), "got {answered:?}");
}

/// D13, for `contains_event_id`: the other read-only request outside `read`.
///
/// Rejects: as for `head`.
#[test]
fn an_append_waits_behind_an_in_flight_contains_event_id() {
    let transport = OrderingTransport::new();
    let store = store(&transport);
    let id = EventId::new(
        StoreId::from_bytes([7; 16]),
        SequencePosition::new(APPENDED_AT).expect("a non-zero position"),
    );
    let contains = pin!(store.contains_event_id(id));

    let answered = assert_an_append_waits_behind(&transport, &store, contains);

    assert!(matches!(answered, Poll::Ready(Ok(_))), "got {answered:?}");
}
