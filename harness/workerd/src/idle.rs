//! An idle object holding a store, with or without a partly consumed read
//! stream: what `workerd`'s inactivity timer is shown to evict (#262 item 2).
//!
//! The claim under test is that holding a `CloudflareEventStore`, or a read
//! stream from one, keeps nothing pending that would stop the platform from
//! evicting an idle object. So the hold must be **only** memory: no task, no
//! timer, no JavaScript promise. [`IdleHold`] is that. It owns one future,
//! never spawned and never woken, and it is polled by hand, once when it is
//! armed and once when it is released.
//!
//! The future is also how a stream is held at all. [`EventStore::read`]
//! borrows the store and the query, so a stream cannot sit in a struct beside
//! the store it borrows from without the struct referring to itself. Inside an
//! `async` block it can: the block owns the store, the query and the stream
//! together, and the box it is pinned in keeps their addresses fixed.

#[cfg(test)]
mod tests;

use core::cell::Cell;
use core::fmt;
use core::future::{Future, poll_fn};
use core::pin::{Pin, pin};
use core::str::FromStr;
use core::task::{Context, Poll, Waker};
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::bytes::Bytes;
use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};

/// How many events [`IdleHold::arm`] appends before it parks: more than one
/// page at the adapter's shipped page size (128 rows), so a stream released
/// after the wait still has a statement to issue against the database.
pub const EVENTS: usize = 200;

/// The type the appended events carry.
const EVENT_TYPE: &str = "IdleHeld";

/// What the hold keeps besides the store. A closed set, spelled as the
/// `stream=` query parameter spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamHold {
    /// The store alone; no stream is opened until the release.
    None,
    /// A read of the whole log, opened and advanced by one event.
    Held,
}

impl StreamHold {
    /// Both, in the order the runner reports them.
    pub const ALL: [Self; 2] = [Self::None, Self::Held];

    /// The name the route and the runner use.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Held => "held",
        }
    }
}

/// A `stream=` value that names neither hold, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownStreamHold(pub String);

impl fmt::Display for UnknownStreamHold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "stream= is none or held, not {:?}", self.0)
    }
}

impl core::error::Error for UnknownStreamHold {}

impl FromStr for StreamHold {
    type Err = UnknownStreamHold;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|hold| hold.name() == text)
            .ok_or_else(|| UnknownStreamHold(text.to_owned()))
    }
}

/// Why a hold could not be armed or released.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdleError {
    /// A step failed: an event could not be built, or the store answered an
    /// error. The message is the error's `Display`; it is only ever printed to
    /// the runner.
    Step {
        /// Which step.
        during: &'static str,
        /// The error, as text.
        message: String,
    },
    /// The held stream ended before yielding the one event it parks after.
    StreamEnded,
    /// The first poll did not reach the park point: something the hold
    /// awaited was not ready, so it is waiting on something other than its
    /// release.
    NotParked,
    /// Released, the hold did not finish on one poll: its stream hung.
    Hung,
}

impl fmt::Display for IdleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Step { during, message } => write!(f, "{during}: {message}"),
            Self::StreamEnded => f.write_str("the held stream ended before its first event"),
            Self::NotParked => f.write_str("the hold did not reach its park point on one poll"),
            Self::Hung => {
                f.write_str("the released hold did not finish on one poll: its stream hung")
            }
        }
    }
}

impl core::error::Error for IdleError {}

/// What a released hold read once it was let go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Released {
    /// What was held.
    pub hold: StreamHold,
    /// For [`StreamHold::None`], every event the store holds, read through
    /// the held store. For [`StreamHold::Held`], the events the parked stream
    /// still had to yield, read through that same stream.
    pub events: usize,
}

impl fmt::Display for Released {
    /// `held=<none|held> events=<n>`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "held={} events={}", self.hold.name(), self.events)
    }
}

/// Where the parked future is. Shared by the hold and the future it owns,
/// which is the only way the hold can tell the future to go on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Stage {
    Arming,
    Parked,
    Resumed,
}

/// The future a hold parks: it ends with how many events it read after the
/// release.
type Parked = Pin<Box<dyn Future<Output = Result<usize, IdleError>>>>;

/// A store, and optionally a partly read stream from it, held by an idle
/// object and nothing else.
pub struct IdleHold {
    hold: StreamHold,
    /// `Rc` because the parked future owns the other handle; `Cell` because
    /// the stage is `Copy`, and only ever set between two polls.
    stage: Rc<Cell<Stage>>,
    parked: Parked,
}

impl fmt::Debug for IdleHold {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdleHold")
            .field("hold", &self.hold)
            .field("stage", &self.stage.get())
            .finish_non_exhaustive()
    }
}

impl IdleHold {
    /// Appends [`EVENTS`] events to `store`, then takes ownership of it and,
    /// for [`StreamHold::Held`], opens a read of the whole log and takes one
    /// event from it. Then the hold parks.
    ///
    /// `S: 'static` because the store is moved into the parked future, which
    /// the object keeps after the request that armed it has answered.
    ///
    /// # Errors
    ///
    /// [`IdleError::Step`] if an append or the first read fails,
    /// [`IdleError::StreamEnded`] if the read yields nothing, and
    /// [`IdleError::NotParked`] if the hold could not reach its park point on
    /// one poll.
    pub async fn arm<S: EventStore + 'static>(
        store: S,
        hold: StreamHold,
    ) -> Result<Self, IdleError> {
        for _ in 0..EVENTS {
            let event = Event::new(EVENT_TYPE, Bytes::from_static(b"idle"))
                .map_err(step("building an event"))?;
            store
                .append(&[event], None)
                .await
                .map_err(step("appending an event"))?;
        }
        let stage = Rc::new(Cell::new(Stage::Arming));
        let parked: Parked = match hold {
            StreamHold::None => Box::pin(hold_store(store, Rc::clone(&stage))),
            StreamHold::Held => Box::pin(hold_stream(store, Rc::clone(&stage))),
        };
        let mut armed = Self {
            hold,
            stage,
            parked,
        };
        match armed.poll_once() {
            Poll::Pending if armed.stage.get() == Stage::Parked => Ok(armed),
            Poll::Ready(Err(err)) => Err(err),
            Poll::Pending | Poll::Ready(Ok(_)) => Err(IdleError::NotParked),
        }
    }

    /// Lets the parked future go on, polls it once, and drops it, the store
    /// and the stream with it.
    ///
    /// # Errors
    ///
    /// [`IdleError::Step`] if a read after the release fails, and
    /// [`IdleError::Hung`] if the future did not finish on that one poll.
    pub fn release(mut self) -> Result<Released, IdleError> {
        self.stage.set(Stage::Resumed);
        match self.poll_once() {
            Poll::Ready(Ok(events)) => Ok(Released {
                hold: self.hold,
                events,
            }),
            Poll::Ready(Err(err)) => Err(err),
            Poll::Pending => Err(IdleError::Hung),
        }
    }

    /// One poll with a waker that does nothing. The hold is only ever polled
    /// by hand, so nothing needs waking, and nothing is left registered with
    /// the runtime that could keep the object alive.
    fn poll_once(&mut self) -> Poll<Result<usize, IdleError>> {
        self.parked
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
    }
}

/// Holds `store`, then reads everything through it once released.
async fn hold_store<S: EventStore>(store: S, stage: Rc<Cell<Stage>>) -> Result<usize, IdleError> {
    park(&stage).await;
    let events = collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .map_err(step("reading through the released store"))?;
    Ok(events.len())
}

/// Opens a read of the whole log, takes one event, holds the stream, then
/// reads the rest of it once released.
async fn hold_stream<S: EventStore>(store: S, stage: Rc<Cell<Stage>>) -> Result<usize, IdleError> {
    let query = Query::all();
    let mut stream = pin!(store.read(&query, ReadOptions::new()));
    match next(stream.as_mut()).await {
        Some(Ok(_)) => {}
        Some(Err(err)) => return Err(step("the held stream's first event")(err)),
        None => return Err(IdleError::StreamEnded),
    }
    park(&stage).await;
    let mut events = 0_usize;
    while let Some(item) = next(stream.as_mut()).await {
        item.map_err(step("the released stream"))?;
        events = events.saturating_add(1);
    }
    Ok(events)
}

/// Marks the future parked, then answers `Pending` until the hold sets
/// [`Stage::Resumed`]. It registers no waker: the hold polls by hand.
async fn park(stage: &Cell<Stage>) {
    stage.set(Stage::Parked);
    poll_fn(|_| {
        if stage.get() == Stage::Resumed {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await;
}

/// The stream's next item. `futures-core` is the only stream crate the
/// harness carries, and it has no `next`.
fn next<St: Stream + ?Sized>(mut stream: Pin<&mut St>) -> impl Future<Output = Option<St::Item>> {
    poll_fn(move |cx| stream.as_mut().poll_next(cx))
}

/// Flattens an error into [`IdleError::Step`] for `during`.
fn step<E: fmt::Display>(during: &'static str) -> impl FnOnce(E) -> IdleError {
    move |err| IdleError::Step {
        during,
        message: err.to_string(),
    }
}
