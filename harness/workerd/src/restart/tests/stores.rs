//! Wrong stores that exist only on the host, for the checks no in-object
//! subject is built to reach.

use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, SequencePosition, SequencedEvent,
};

use super::super::map_stream::MapStream;
use crate::restart::InstanceLog;

/// WRONG STORE: refuses every conditional append. It passes the enforcement
/// half of both condition rules vacuously, and the anchor is what rejects it.
#[derive(Debug, Default)]
pub(super) struct RefusesEveryCondition(InstanceLog);

impl EventStore for RefusesEveryCondition {
    type Error = <InstanceLog as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl futures_core::Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if condition.is_some() {
            return Err(AppendError::ConditionViolated(
                ConditionViolated::unspecified(),
            ));
        }
        self.0.append(events, None).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
}

/// WRONG STORE: durable appends, so positions after a restart are fresh, and a
/// read path rebuilt on reopen with an off-by-one that skips the oldest row.
/// Within one instance it reads everything; after a restart one acknowledged
/// event is missing while every new append still lands above it, which is what
/// reaches the tail of `positions_after_a_restart_are_never_reissued`.
#[derive(Debug)]
pub(super) struct SkipsFirstRowOnReopen {
    inner: InstanceLog,
    reopened: bool,
}

impl SkipsFirstRowOnReopen {
    pub(super) fn new() -> Self {
        Self {
            inner: InstanceLog::default(),
            reopened: false,
        }
    }

    /// The same log under a new instance: the defect switches on.
    pub(super) fn reopen(self) -> Self {
        Self {
            inner: self.inner,
            reopened: true,
        }
    }
}

impl EventStore for SkipsFirstRowOnReopen {
    type Error = <InstanceLog as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl futures_core::Stream<Item = Result<SequencedEvent, Self::Error>> {
        let mut skip = self.reopened;
        MapStream::new(self.inner.read(query, options), move |event| {
            if skip {
                skip = false;
                return None;
            }
            Some(event)
        })
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.inner.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.inner.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.inner.contains_event_id(id).await
    }
}
