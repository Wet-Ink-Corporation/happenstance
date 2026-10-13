//! `IdleHold` on the host, over a scripted in-memory store: the hold parks
//! where it says it does, reads nothing more until it is released, and names
//! a stream that stalls rather than hanging the request that armed it.

use core::cell::Cell;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, Query, ReadOptions,
    SequencePosition, SequencedEvent,
};
use happenstance_testkit::__private::block_on;

use super::{EVENTS, IdleError, IdleHold, Released, StreamHold, UnknownStreamHold};

/// A conformant in-memory store whose read streams count what they yield and,
/// when told to, stall (answer `Pending` for ever) after that many events.
#[derive(Debug, Default)]
struct Scripted {
    log: MemoryEventStore,
    yielded: Rc<Cell<usize>>,
    stall_after: Option<usize>,
}

impl Scripted {
    fn stalling_after(events: usize) -> Self {
        Self {
            stall_after: Some(events),
            ..Self::default()
        }
    }
}

struct Counted<St> {
    inner: Pin<Box<St>>,
    yielded: Rc<Cell<usize>>,
    stall_after: Option<usize>,
}

impl<St: Stream> Stream for Counted<St> {
    type Item = St::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.stall_after == Some(this.yielded.get()) {
            return Poll::Pending;
        }
        let polled = this.inner.as_mut().poll_next(cx);
        if let Poll::Ready(Some(_)) = &polled {
            this.yielded.set(this.yielded.get() + 1);
        }
        polled
    }
}

impl EventStore for Scripted {
    type Error = <MemoryEventStore as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        Counted {
            inner: Box::pin(self.log.read(query, options)),
            yielded: Rc::clone(&self.yielded),
            stall_after: self.stall_after,
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.log.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.log.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.log.contains_event_id(id).await
    }
}

#[test]
fn stream_hold_parses_its_names_and_hands_back_any_other() {
    for hold in StreamHold::ALL {
        assert_eq!(hold.name().parse(), Ok(hold));
    }
    assert_eq!(
        "open".parse::<StreamHold>(),
        Err(UnknownStreamHold("open".to_owned()))
    );
}

#[test]
fn a_held_store_opens_no_stream_and_reads_every_event_on_release() {
    let store = Scripted::default();
    let yielded = Rc::clone(&store.yielded);

    let hold = block_on(IdleHold::arm(store, StreamHold::None)).expect("the hold parks");
    assert_eq!(yielded.get(), 0, "nothing is read while the store is held");

    let released = hold.release().expect("the release completes");
    assert_eq!(
        released,
        Released {
            hold: StreamHold::None,
            events: EVENTS
        }
    );
}

#[test]
fn a_held_stream_parks_after_one_event_and_yields_the_rest_on_release() {
    let store = Scripted::default();
    let yielded = Rc::clone(&store.yielded);

    let hold = block_on(IdleHold::arm(store, StreamHold::Held)).expect("the hold parks");
    assert_eq!(yielded.get(), 1, "parked after exactly one event");

    let released = hold.release().expect("the release completes");
    assert_eq!(
        released,
        Released {
            hold: StreamHold::Held,
            events: EVENTS - 1
        }
    );
    assert_eq!(yielded.get(), EVENTS, "the same stream read the rest");
}

#[test]
fn a_stream_that_stalls_before_its_first_event_is_not_parked() {
    let armed = block_on(IdleHold::arm(Scripted::stalling_after(0), StreamHold::Held));
    assert_eq!(armed.err(), Some(IdleError::NotParked));
}

#[test]
fn a_held_stream_that_stalls_after_the_release_is_reported_hung() {
    let hold = block_on(IdleHold::arm(Scripted::stalling_after(1), StreamHold::Held))
        .expect("the hold parks after its first event");
    assert_eq!(hold.release(), Err(IdleError::Hung));
}

#[test]
fn released_and_its_error_say_what_the_runner_reads() {
    let released = Released {
        hold: StreamHold::Held,
        events: 3,
    };
    assert_eq!(released.to_string(), "held=held events=3");
    assert_eq!(
        IdleError::Hung.to_string(),
        "the released hold did not finish on one poll: its stream hung"
    );
}
