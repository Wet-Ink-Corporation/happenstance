//! The halves of the rules, and the steps they share.
//!
//! Every position compared here is one the store returned: the specification
//! permits gaps, so a rule that expected a literal position would reject a
//! conformant store.

use happenstance_core::bytes::Bytes;
use happenstance_core::{
    AppendCondition, Event, EventStore, Query, QueryItem, ReadOptions, SequencePosition,
    SequencedEvent, Tags, collect,
};

use super::evidence::Observed;
use super::{Check, EvidenceError, Failure, Fault, Rule, Verdict};

/// The types the rules write.
const FIRST: &str = "RestartFirst";
const SECOND: &str = "RestartSecond";
const LATER: &str = "RestartLater";
/// The tag every condition rule's events carry and its query selects.
const TAG: (&str, &str) = ("restart", "scope");

/// One rule run against one store.
pub(super) struct Run<'s, S> {
    rule: Rule,
    store: &'s S,
}

/// What a conditional append came to, when it did not fail for another reason.
enum Conditional {
    Appended(SequencePosition),
    Refused,
}

/// The before-halves.
impl<'s, S: EventStore> Run<'s, S> {
    pub(super) const fn new(rule: Rule, store: &'s S) -> Self {
        Self { rule, store }
    }

    /// One batch of two, then everything read back for its identities.
    pub(super) async fn batch_then_read(&self) -> Result<Observed, Fault> {
        let batch = [self.event(FIRST)?, self.event(SECOND)?];
        let position = self.append("appending the batch", &batch).await?;
        Ok(Observed {
            acknowledged: vec![position],
            event_ids: self.identities().await?,
            captured_after: None,
        })
    }

    /// Two appends of one.
    pub(super) async fn two_appends(&self) -> Result<Observed, Fault> {
        let first = self
            .append("the first append", &[self.event(FIRST)?])
            .await?;
        let second = self
            .append("the second append", &[self.event(SECOND)?])
            .await?;
        Ok(Observed {
            acknowledged: vec![first, second],
            event_ids: Vec::new(),
            captured_after: None,
        })
    }

    /// One scoped event, then a condition captured from a read of the scope.
    pub(super) async fn capture_condition(&self) -> Result<Observed, Fault> {
        let scope = self.scope()?;
        let first = self
            .append("the first scoped append", &[self.scoped(FIRST)?])
            .await?;
        let captured = self
            .observed_head("reading the scope", &scope)
            .await?
            .ok_or_else(|| {
                self.fault("reading the scope")(
                    "an acknowledged event was not read back".to_owned(),
                )
            })?;
        Ok(Observed {
            acknowledged: vec![first],
            event_ids: Vec::new(),
            captured_after: Some(captured),
        })
    }

    /// [`Run::capture_condition`], then a second scoped event that violates
    /// the captured condition.
    pub(super) async fn capture_condition_then_conflict(&self) -> Result<Observed, Fault> {
        let mut observed = self.capture_condition().await?;
        let event = self.scoped(SECOND)?;
        observed
            .acknowledged
            .push(self.append("the conflicting append", &[event]).await?);
        Ok(observed)
    }
}

/// The after-halves. The check that names a rule's wrong store runs first.
impl<S: EventStore> Run<'_, S> {
    /// Both events of the batch, in order, ending at its acknowledged position,
    /// under the identities read before the restart.
    pub(super) async fn batch_read_back(&self, before: &Observed) -> Result<Verdict, Fault> {
        let acknowledged = self.last_acknowledged(before)?;
        let events = self.read("reading everything back", &Query::all()).await?;
        let types: Vec<&str> = events
            .iter()
            .map(|event| event.event_type().as_str())
            .collect();
        let last = events.last().map(|event| event.position);
        if types != [FIRST, SECOND] || last != Some(acknowledged) {
            return Ok(self.reject(
                Check::AcknowledgedWriteLost,
                format!(
                    "read back types {types:?} ending at {last:?}; \
                     acknowledged {FIRST}, {SECOND} ending at {acknowledged}"
                ),
            ));
        }
        let ids: Vec<String> = events.iter().map(|event| event.id.to_string()).collect();
        if ids != before.event_ids {
            return Ok(self.reject(
                Check::IdentityChanged,
                format!(
                    "read back ids {ids:?}; read before the restart {:?}",
                    before.event_ids
                ),
            ));
        }
        Ok(Ok(()))
    }

    /// `head()`, before anything is written, is the last acknowledged position.
    pub(super) async fn head_is_last_acknowledged(
        &self,
        before: &Observed,
    ) -> Result<Verdict, Fault> {
        let acknowledged = self.last_acknowledged(before)?;
        let head = self
            .store
            .head()
            .await
            .map_err(|err| self.fault("asking for the head")(err.to_string()))?;
        if head != Some(acknowledged) {
            return Ok(self.reject(
                Check::HeadRegressed,
                format!(
                    "head() answered {head:?}; the last acknowledged position is {acknowledged}"
                ),
            ));
        }
        Ok(Ok(()))
    }

    /// A new append lands above every acknowledged position, and the log holds
    /// it and everything acknowledged before the restart, in ascending order.
    pub(super) async fn next_is_fresh(&self, before: &Observed) -> Result<Verdict, Fault> {
        let highest = self.last_acknowledged(before)?;
        let position = self
            .append("the append after the restart", &[self.event(LATER)?])
            .await?;
        if position <= highest {
            return Ok(self.reject(
                Check::PositionReissued,
                format!("the append after the restart was assigned {position}; {highest} was acknowledged before it"),
            ));
        }
        let events = self.read("reading everything back", &Query::all()).await?;
        let positions: Vec<SequencePosition> = events.iter().map(|event| event.position).collect();
        let ascending = positions
            .windows(2)
            .all(|pair| matches!(pair, [a, b] if a < b));
        // The new event must be there too, beside everything acknowledged before.
        let held = before
            .acknowledged
            .iter()
            .chain(core::iter::once(&position))
            .all(|p| positions.contains(p));
        if !ascending || !held {
            return Ok(self.reject(
                Check::AcknowledgedWriteLost,
                format!(
                    "read back {positions:?}; acknowledged before the restart {:?}",
                    before.acknowledged
                ),
            ));
        }
        Ok(Ok(()))
    }

    /// A conflicting scoped event now, then [`Run::condition_binds`].
    pub(super) async fn conflict_then_condition_binds(
        &self,
        before: &Observed,
    ) -> Result<Verdict, Fault> {
        let event = self.scoped(SECOND)?;
        self.append("the conflicting append", &[event]).await?;
        self.condition_binds(before).await
    }

    /// The captured condition refuses an append, a conflicting event sitting
    /// above its `after`, and the same condition re-captured at the scope's
    /// current head does not.
    pub(super) async fn condition_binds(&self, before: &Observed) -> Result<Verdict, Fault> {
        let captured = before.captured_after.ok_or_else(|| self.missing("after"))?;
        let scope = self.scope()?;
        // Built from its own query rather than a copy of `scope`, which the
        // fresh condition below takes.
        let stale = AppendCondition::new(self.scope()?).after_every_guard(captured);
        let appended = self
            .append_if("the stale conditional append", &stale)
            .await?;
        if let Conditional::Appended(position) = appended {
            return Ok(self.reject(
                Check::ConditionNotEnforced,
                format!("a condition captured after {captured} was accepted at {position} over a conflict above it"),
            ));
        }
        let head = self.observed_head("reading the scope", &scope).await?;
        let fresh = AppendCondition::new(scope).after_every_guard_opt(head);
        if let Conditional::Refused = self
            .append_if("the fresh conditional append", &fresh)
            .await?
        {
            return Ok(self.reject(
                Check::ConditionRefusedWithoutConflict,
                format!("a condition captured after {head:?}, the scope's head, was refused"),
            ));
        }
        Ok(Ok(()))
    }
}

/// The steps both halves share.
impl<S: EventStore> Run<'_, S> {
    fn fault(&self, during: &'static str) -> impl FnOnce(String) -> Fault {
        let rule = self.rule;
        move |message| Fault::Step {
            rule,
            during,
            message,
        }
    }

    fn missing(&self, field: &'static str) -> Fault {
        Fault::Evidence {
            rule: self.rule,
            error: EvidenceError::MissingField { field },
        }
    }

    fn reject(&self, check: Check, detail: String) -> Verdict {
        Err(Failure {
            rule: self.rule,
            check,
            detail,
        })
    }

    fn last_acknowledged(&self, before: &Observed) -> Result<SequencePosition, Fault> {
        before
            .acknowledged
            .last()
            .copied()
            .ok_or_else(|| self.missing("position"))
    }

    fn event(&self, event_type: &'static str) -> Result<Event, Fault> {
        Event::new(event_type, Bytes::from_static(b"restart"))
            .map_err(|err| self.fault("building an event")(err.to_string()))
    }

    /// An event carrying the scope's tag, so the scope query selects it.
    fn scoped(&self, event_type: &'static str) -> Result<Event, Fault> {
        Ok(self.event(event_type)?.with_tags(self.tags()?))
    }

    fn tags(&self) -> Result<Tags, Fault> {
        Tags::from_pairs([TAG])
            .map_err(|err| self.fault("building the scope's tag")(err.to_string()))
    }

    fn scope(&self) -> Result<Query, Fault> {
        QueryItem::tagged(self.tags()?)
            .map(Query::from_item)
            .map_err(|err| self.fault("building the scope query")(err.to_string()))
    }

    async fn append(
        &self,
        during: &'static str,
        events: &[Event],
    ) -> Result<SequencePosition, Fault> {
        self.store
            .append(events, None)
            .await
            .map_err(|err| self.fault(during)(err.to_string()))
    }

    /// A scoped `LATER` under `condition`.
    async fn append_if(
        &self,
        during: &'static str,
        condition: &AppendCondition,
    ) -> Result<Conditional, Fault> {
        let event = self.scoped(LATER)?;
        match self.store.append(&[event], Some(condition)).await {
            Ok(position) => Ok(Conditional::Appended(position)),
            Err(err) if err.is_condition_violated() => Ok(Conditional::Refused),
            Err(err) => Err(self.fault(during)(err.to_string())),
        }
    }

    async fn read(
        &self,
        during: &'static str,
        query: &Query,
    ) -> Result<Vec<SequencedEvent>, Fault> {
        collect(self.store.read(query, ReadOptions::new()))
            .await
            .map_err(|err| self.fault(during)(err.to_string()))
    }

    /// Every identity in the log, in order.
    async fn identities(&self) -> Result<Vec<String>, Fault> {
        let events = self.read("reading identities", &Query::all()).await?;
        Ok(events.iter().map(|event| event.id.to_string()).collect())
    }

    /// The last position a read of `query` observed: the sound `after` for a
    /// condition on it, as `EventStore::append`'s documentation requires.
    async fn observed_head(
        &self,
        during: &'static str,
        query: &Query,
    ) -> Result<Option<SequencePosition>, Fault> {
        Ok(self
            .read(during, query)
            .await?
            .last()
            .map(|event| event.position))
    }
}
