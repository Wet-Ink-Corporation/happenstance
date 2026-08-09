//! The append condition: DCB's consistency mechanism.

use alloc::boxed::Box;
use alloc::vec::Vec;

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
/// A condition is a non-empty sequence of [`Guard`]s, each pairing a query with
/// its own boundary. The store **must** reject the append if **any** guard is
/// violated — that is, if the store holds an event matching that guard's query
/// at a position strictly greater than that guard's `after`. A guard whose
/// `after` is `None` is violated by any match at all.
///
/// Most conditions have exactly one guard, which is what
/// [`new`](Self::new) builds and what every example below shows. More than one
/// is for a decision model assembled from fragments read separately: four reads
/// produce four boundaries, and there is no single boundary that is correct for
/// all four.
///
/// # The usual shape
///
/// A command handler reads with some query, notes the last position it saw, and
/// appends with the *same* query and that position:
///
/// ```
/// # use happenstance_core::{AppendCondition, Query, QueryItem, Tags};
/// let query = Query::from_item(QueryItem::of_types(["StudentSubscribed"])?);
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
/// )?));
///
/// assert_eq!(condition.guards().len(), 1);
/// assert!(condition.guards()[0].after.is_none());
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
///
/// Two fragments read separately, each carrying its own boundary — the shape a
/// `min(p₁, p₂)` collapse gets wrong:
///
/// ```
/// use happenstance_core::{AppendCondition, Query, QueryItem, SequencePosition};
///
/// let busy = Query::from_item(QueryItem::of_types(["MeterReadingTaken"])?);
/// let quiet = Query::from_item(QueryItem::of_types(["TariffPublished"])?);
///
/// let condition = AppendCondition::new(busy)
///     .after_opt(SequencePosition::new(9_000))
///     .and_guard(quiet, SequencePosition::new(12));
///
/// assert_eq!(condition.guards().len(), 2);
/// // The quiet fragment's stale boundary does not govern the busy one.
/// assert_eq!(condition.guards()[0].after, SequencePosition::new(9_000));
/// assert_eq!(condition.guards()[1].after, SequencePosition::new(12));
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct AppendCondition {
    // Private, and this is the one place the shape is stricter than it looks
    // like it needs to be. `#[non_exhaustive]` blocks the struct-literal
    // *expression* and exhaustive matching; it does **not** block assignment to
    // a public field of a value the caller already owns. With a public field,
    // one line of safe downstream code —
    //
    //     let mut c = AppendCondition::new(q);
    //     c.guards = Box::new([]);
    //
    // — builds a condition with no guards, which nothing can ever violate. That
    // is a conditional append that is silently unconditional: a lost update,
    // with no diagnostic anywhere. The accessor below keeps the read that an
    // ingest policy needs and removes the expression that reaches zero guards.
    guards: Box<[Guard]>,
}

/// One clause of an [`AppendCondition`]: a query and the boundary it is checked
/// from.
///
/// `#[non_exhaustive]` with public fields, which is a deliberate asymmetry
/// rather than an oversight: a replication hub must be able to **read** `after`
/// on a peer-supplied condition in order to refuse it, and must not be able to
/// **fabricate** one. Downstream, `Guard { query, .. }` in a pattern and
/// `guard.after` as a read both compile; `Guard { query, after }` as an
/// expression is `error[E0639]`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Guard {
    /// The append is rejected if the store holds any event matching this.
    pub query: Query,
    /// Restricts this guard to events *after* this position, exclusive.
    ///
    /// `None` checks the entire log. Set it to the last position the caller
    /// observed *for this query*, so that events it already accounted for do
    /// not reject its own append.
    pub after: Option<SequencePosition>,
}

impl AppendCondition {
    /// A condition with one unbounded guard, checking the whole log.
    #[must_use]
    pub fn new(fail_if_events_match: Query) -> Self {
        Self {
            guards: Box::new([Guard {
                query: fail_if_events_match,
                after: None,
            }]),
        }
    }

    /// Adds a guard with its own boundary.
    ///
    /// This is what makes the specification's advice to "issue one read per
    /// fragment" sound. Four reads produce four boundaries, and collapsing them
    /// to `min(p₁…p₄)` — the obvious application-side workaround — means the
    /// quietest fragment's stale boundary governs the busiest one, so a busy
    /// consistency boundary rejects appends that never conflicted and the
    /// deployment reads the rejection rate as contention.
    ///
    /// Allocation note, since this is the trade a reader coming from a language
    /// with a growable list will not expect: `guards` is a boxed slice, which
    /// carries no spare capacity, so this unboxes, pushes and re-boxes. A chain
    /// of *n* calls is quadratic in allocation. For the counts this exists to
    /// serve — four in the worked case — that is the right side of the trade
    /// against carrying capacity in every cloned condition forever.
    #[must_use]
    pub fn and_guard(self, query: Query, after: Option<SequencePosition>) -> Self {
        let mut guards = Vec::from(self.guards);
        guards.push(Guard { query, after });
        Self {
            guards: guards.into_boxed_slice(),
        }
    }

    /// The guards, in the order they were added. Never empty.
    #[must_use]
    pub fn guards(&self) -> &[Guard] {
        &self.guards
    }

    /// Restricts **every** guard to events after `position`, exclusive.
    #[must_use]
    pub fn after(self, position: SequencePosition) -> Self {
        self.after_opt(Some(position))
    }

    /// Restricts **every** guard to events after `position`, exclusive, or
    /// checks the whole log when `position` is `None`.
    ///
    /// The `Option`-taking form exists because callers usually hold the last
    /// position they read, which is `None` when their query matched nothing.
    ///
    /// Applying to every guard is what keeps a single-guard condition — the
    /// shape every existing caller builds — behaving exactly as it always has.
    /// For per-guard boundaries, use [`and_guard`](Self::and_guard).
    #[must_use]
    pub fn after_opt(self, position: Option<SequencePosition>) -> Self {
        let guards = Vec::from(self.guards)
            .into_iter()
            .map(|guard| Guard {
                query: guard.query,
                after: position,
            })
            .collect::<Vec<_>>();
        Self {
            guards: guards.into_boxed_slice(),
        }
    }

    /// Whether an event at `position` with this type and these tags would
    /// violate the condition.
    ///
    /// Violated if **any** guard is violated — the guards are a conjunction of
    /// constraints, so one broken clause rejects the append.
    ///
    /// Adapters should push this down into storage; this is the reference
    /// definition the conformance suite holds them to. An adapter generating SQL
    /// must parenthesise each guard explicitly — `(item AND position > p) OR …`
    /// — because the precedence bug that already threatens the single-boundary
    /// form becomes *n* times more likely here.
    pub fn is_violated_by(
        &self,
        position: SequencePosition,
        event_type: &crate::EventType,
        tags: &crate::Tags,
    ) -> bool {
        self.guards
            .iter()
            .any(|guard| guard.is_violated_by(position, event_type, tags))
    }
}

impl Guard {
    /// Whether an event at `position` would violate this guard alone.
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
            _ => self.query.matches(event_type, tags),
        }
    }
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::{AppendCondition, Guard};
    use crate::event::SequencePosition;
    use crate::query::Query;
    use alloc::vec::Vec;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "Guard")]
    struct GuardWire {
        query: Query,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        after: Option<SequencePosition>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "AppendCondition")]
    struct Wire {
        guards: Vec<GuardWire>,
    }

    impl Serialize for AppendCondition {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            Wire {
                guards: self
                    .guards()
                    .iter()
                    .map(|guard| GuardWire {
                        query: guard.query.clone(),
                        after: guard.after,
                    })
                    .collect(),
            }
            .serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for AppendCondition {
        /// Rejects an empty guard sequence.
        ///
        /// Deserialisation is the other door into the private field, and a
        /// validity invariant that the constructor enforces and the decoder does
        /// not is not an invariant. An empty condition decoded from a peer would
        /// be a condition nothing can violate — the same silent lost update the
        /// private field exists to prevent, arriving over the wire instead of
        /// through an assignment.
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let wire = Wire::deserialize(deserializer)?;
            if wire.guards.is_empty() {
                return Err(serde::de::Error::custom(
                    "an append condition must carry at least one guard",
                ));
            }
            let guards = wire
                .guards
                .into_iter()
                .map(|guard| Guard {
                    query: guard.query,
                    after: guard.after,
                })
                .collect::<Vec<_>>();
            Ok(Self {
                guards: guards.into_boxed_slice(),
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
        let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()));

        assert!(condition.is_violated_by(pos(1), &ty("A"), &Tags::empty()));
        assert!(!condition.is_violated_by(pos(1), &ty("B"), &Tags::empty()));
    }

    #[test]
    fn after_is_exclusive() {
        let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()))
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
        let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()))
            .after(pos(5));

        assert!(!condition.is_violated_by(pos(99), &ty("B"), &Tags::empty()));
    }
}
