//! The stores a cross-restart rule runs against: the adapter, and two wrong
//! stores that are each conformant within one instance and wrong across a
//! restart.

use core::cell::Cell;
use core::fmt;
use core::str::FromStr;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, Query, ReadOptions,
    SequencePosition, SequencedEvent,
};

use super::{Check, Rule};

/// The store a cross-restart rule runs against. A closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    /// `CloudflareEventStore`, the store under test. It fails nothing.
    Cloudflare,
    /// [`InstanceLog`], a wrong store.
    InstanceLog,
    /// [`FreshHighWater`] over `CloudflareEventStore`, a wrong store.
    FreshHighWater,
}

impl Subject {
    /// Every subject, in the order the runner reports them.
    pub const ALL: [Self; 3] = [Self::Cloudflare, Self::InstanceLog, Self::FreshHighWater];

    /// The subject's name, as the runner and the routes spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Cloudflare => "cloudflare",
            Self::InstanceLog => "instance-log",
            Self::FreshHighWater => "fresh-high-water",
        }
    }

    /// The registry: exactly the (rule, check) pairs this store fails when the
    /// object restarts between the halves. Every other pair passes, and with no
    /// restart every pair passes.
    #[must_use]
    pub const fn rejects(self) -> &'static [(Rule, Check)] {
        match self {
            Self::Cloudflare => &[],
            Self::InstanceLog => &[
                (
                    Rule::AcknowledgedWritesSurviveARestart,
                    Check::AcknowledgedWriteLost,
                ),
                (Rule::HeadSurvivesARestart, Check::HeadRegressed),
                (
                    Rule::PositionsAfterARestartAreNeverReissued,
                    Check::PositionReissued,
                ),
                (
                    Rule::AConditionFromBeforeARestartStillBinds,
                    Check::ConditionNotEnforced,
                ),
                (
                    Rule::AConditionFromBeforeARestartSeesALaterConflict,
                    Check::ConditionNotEnforced,
                ),
            ],
            Self::FreshHighWater => &[
                (Rule::HeadSurvivesARestart, Check::HeadRegressed),
                (
                    Rule::AConditionFromBeforeARestartStillBinds,
                    Check::ConditionNotEnforced,
                ),
            ],
        }
    }

    /// The check this store fails `rule` on across a restart, if it fails it.
    #[must_use]
    pub fn rejection(self, rule: Rule) -> Option<Check> {
        self.rejects()
            .iter()
            .find(|(rejected, _)| *rejected == rule)
            .map(|(_, check)| *check)
    }
}

/// A name that is no subject's, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownSubject(pub String);

impl fmt::Display for UnknownSubject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no restart subject is called {:?}", self.0)
    }
}

impl core::error::Error for UnknownSubject {}

impl FromStr for Subject {
    type Err = UnknownSubject;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|subject| subject.name() == text)
            .ok_or_else(|| UnknownSubject(text.to_owned()))
    }
}

/// **WRONG STORE.** Keeps its whole log in the object instance.
///
/// The textbook Durable Object mistake: a conformant in-memory store, so it
/// passes every rule that runs inside one request, and it loses everything at
/// the first eviction. Every cross-restart rule rejects it.
#[derive(Debug, Default)]
pub struct InstanceLog(MemoryEventStore);

impl EventStore for InstanceLog {
    type Error = <MemoryEventStore as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.0.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
}

/// **WRONG STORE**, the one issue #262 names: durable events, and a high-water
/// position cached in the object.
///
/// A new instance starts from the cache's constructed default instead of
/// reading storage. `head()` answers from the cache, and a conditional append
/// whose every guard's `after` is at or above the cache skips the conflict
/// check, on the reasoning that nothing newer than the cache has been written.
/// Within one instance both are true, so it passes every rule there; after a
/// restart the cache is empty while the log is not.
#[derive(Debug)]
pub struct FreshHighWater<S> {
    inner: S,
    // A `Cell` rather than a `RefCell`: the value is `Copy`, so it is only ever
    // read or replaced whole and there is no borrow to hold.
    high_water: Cell<Option<SequencePosition>>,
}

impl<S> FreshHighWater<S> {
    /// A new instance over `inner`, with nothing cached: the defect.
    pub const fn new(inner: S) -> Self {
        Self {
            inner,
            high_water: Cell::new(None),
        }
    }

    /// The durable store underneath, without the cache. Handing it to
    /// [`FreshHighWater::new`] is what a restart does to this store.
    pub fn into_inner(self) -> S {
        self.inner
    }
}

impl<S: EventStore> EventStore for FreshHighWater<S> {
    type Error = S::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.inner.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let cached = self.high_water.get();
        // `None` orders below every position, so an empty cache covers every
        // guard: exactly what a fresh instance believes.
        let skip = |condition: &&AppendCondition| {
            condition.guards().iter().all(|guard| guard.after >= cached)
        };
        let checked = condition.filter(|condition| !skip(condition));
        let position = self.inner.append(events, checked).await?;
        self.high_water
            .set(Some(cached.map_or(position, |c| c.max(position))));
        Ok(position)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.high_water.get())
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.inner.contains_event_id(id).await
    }
}
