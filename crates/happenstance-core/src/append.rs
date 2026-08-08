//! The append condition: DCB's consistency mechanism.

use crate::event::SequencePosition;
use crate::query::Query;

/// The consistency boundary an append is checked against.
///
/// This is the whole point of DCB. Rather than locking an aggregate, a command
/// handler declares *what it looked at* when it made its decision. The store
/// then refuses the append if anything matching that description has landed
/// since — which is precisely the set of writes that could have changed the
/// decision.
///
/// The store **must** reject the append if it holds at least one event matching
/// `fail_if_events_match` after `after`. With `after` set to `None`, any match
/// at all rejects it.
///
/// # The usual shape
///
/// A command handler reads with some query, notes the last position it saw, and
/// appends with the *same* query and that position:
///
/// ```
/// # use happenstance_core::{AppendCondition, Query, QueryItem, Tags};
/// let query = Query::from_item(QueryItem::of_types(["StudentSubscribed"])?)?;
///
/// // ... read with `query`, ending at `last_seen` ...
/// let last_seen = None; // nothing matched
///
/// let condition = AppendCondition::new(query).after_opt(last_seen);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// # Examples
///
/// Asserting that a course has never been defined — the classic uniqueness
/// check, expressed without an aggregate:
///
/// ```
/// use happenstance_core::{AppendCondition, Query, QueryItem, Tags};
///
/// let condition = AppendCondition::new(Query::from_item(QueryItem::new(
///     ["CourseDefined"],
///     Tags::from_pairs([("course", "c1")])?,
/// )?)?);
///
/// assert!(condition.after.is_none());
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct AppendCondition {
    /// The append is rejected if the store holds any event matching this.
    pub fail_if_events_match: Query,
    /// Restricts the check to events *after* this position, exclusive.
    ///
    /// `None` checks the entire log. Set this to the last position the caller
    /// observed, so that events it already accounted for do not reject its own
    /// append.
    pub after: Option<SequencePosition>,
}

impl AppendCondition {
    /// A condition checking the whole log.
    pub const fn new(fail_if_events_match: Query) -> Self {
        Self {
            fail_if_events_match,
            after: None,
        }
    }

    /// Restricts the check to events after `position`, exclusive.
    #[must_use]
    pub const fn after(mut self, position: SequencePosition) -> Self {
        self.after = Some(position);
        self
    }

    /// Restricts the check to events after `position`, exclusive, or checks the
    /// whole log when `position` is `None`.
    ///
    /// The `Option`-taking form exists because callers usually hold the last
    /// position they read, which is `None` when their query matched nothing.
    #[must_use]
    pub const fn after_opt(mut self, position: Option<SequencePosition>) -> Self {
        self.after = position;
        self
    }

    /// Whether an event at `position` with this type and these tags would
    /// violate the condition.
    ///
    /// Adapters should push this down into storage; this is the reference
    /// definition the conformance suite holds them to.
    pub fn is_violated_by(
        &self,
        position: SequencePosition,
        event_type: &crate::EventType,
        tags: &crate::Tags,
    ) -> bool {
        // Written as a match rather than a let-chain because the two arms read
        // as the two rules they are, not because the floor forbids one: ADR-0029
        // raised the MSRV to 1.97.1 and let-chains have been available since
        // 1.88.
        match self.after {
            Some(after) if position <= after => false,
            _ => self.fail_if_events_match.matches(event_type, tags),
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::AppendCondition;
    use crate::event::SequencePosition;
    use crate::query::Query;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "AppendCondition")]
    struct Wire {
        fail_if_events_match: Query,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<SequencePosition>,
    }

    impl Serialize for AppendCondition {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            Wire {
                fail_if_events_match: self.fail_if_events_match.clone(),
                after: self.after,
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for AppendCondition {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = Wire::deserialize(deserializer)?;
            Ok(Self {
                fail_if_events_match: wire.fail_if_events_match,
                after: wire.after,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::{EventType, QueryItem, Tags};

    fn pos(value: u64) -> SequencePosition {
        SequencePosition::new(value).unwrap()
    }

    fn ty(value: &str) -> EventType {
        EventType::new(value).unwrap()
    }

    #[test]
    fn without_after_any_match_violates() {
        let condition =
            AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()).unwrap());

        assert!(condition.is_violated_by(pos(1), &ty("A"), &Tags::empty()));
        assert!(!condition.is_violated_by(pos(1), &ty("B"), &Tags::empty()));
    }

    #[test]
    fn after_is_exclusive() {
        let condition =
            AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()).unwrap())
                .after(pos(5));

        // Strictly before the boundary: already accounted for.
        assert!(!condition.is_violated_by(pos(4), &ty("A"), &Tags::empty()));
        // Exactly at the boundary: also already accounted for — `after` is exclusive.
        assert!(!condition.is_violated_by(pos(5), &ty("A"), &Tags::empty()));
        // Strictly after: a write the caller never saw.
        assert!(condition.is_violated_by(pos(6), &ty("A"), &Tags::empty()));
    }

    #[test]
    fn non_matching_events_after_the_boundary_are_ignored() {
        let condition =
            AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()).unwrap())
                .after(pos(5));

        assert!(!condition.is_violated_by(pos(99), &ty("B"), &Tags::empty()));
    }
}
