//! The model a store is held to when its reads trail its commits.
//!
//! A fixture that declines `READ_YOUR_OWN_WRITES` has said that `append`
//! returning `Ok(P)` does not promise a read straight afterwards shows `P`.
//! The strict `Model` assumes the opposite at every step, so for such a
//! store the rule runs this model instead. It reads no clock and never waits
//! (CF-33): every check is decided from what the store has already answered.
//!
//! # What a trailing store still promises
//!
//! The model drives one handle, strictly sequentially, so these follow from
//! existing clauses:
//!
//! - **Held is accepted.** `Ok(P)` is a durability promise, so every accepted
//!   batch is held from the moment `append` returns.
//! - **Visible is a position prefix of held.** Position order is visibility
//!   order (ES-10), and one sequential writer assigns positions in append
//!   order.
//! - **Nothing is reused.** Positions strictly increase (VT-11), the batch's
//!   last position is `Ok(P)` (ES-19), and an event keeps the position it was
//!   first shown at.
//! - **A condition is judged over what is held, not what is visible** (ES-25,
//!   ADR-0095's reading). So a verdict can turn on rows no read has shown yet.
//!   Their content and order are known; their positions may not be.
//! - **A read is one snapshot** (ES-11), so its frontier lies between the
//!   frontiers of the full reads before and after it.
//!
//! And one thing **no clause states**: the frontier never moves backward. It
//! holds for `pg_snapshot_xmin` on one primary, both frontier adapters meet it
//! by construction, and the strict model implies it. The model asserts it as a
//! stated assumption, with `RetreatingFrontierStore` in the testkit's
//! `tests/mutation_coverage/trailing.rs` as the store that breaks it. Making it
//! normative is an ADR's question, not this module's.
//!
//! # The three checks
//!
//! - **T1, the log.** Every full read is a prefix of what is held, by content
//!   and order; no shorter than the last one; strictly increasing; agrees on
//!   every position already shown; shows each visible batch-last where `append`
//!   said; and shows nothing at or above a returned position whose batch is
//!   still invisible.
//! - **T2, reads.** A read's result is the selection over **some** prefix
//!   between the full read before it and the full read after it.
//! - **T3, verdicts.** Each held event gets an exact position, or an open
//!   interval between its nearest exact neighbours. A violation is monotone in
//!   position, so probing the interval's two ends gives a three-valued
//!   prediction: refuse, accept, or either. A verdict that could go either way
//!   is deferred and re-judged at every later full read, and a contradiction
//!   then fails naming the op that received it.
//!
//! At zero lag every interval collapses and every bracket is one prefix, so
//! the three checks are the strict model's, less one: an accepted write that
//! has not appeared reads as lag rather than as a lost write.
//!
//! # What it no longer catches
//!
//! Each of these the strict model catches and this one, by design, does not:
//!
//! 1. **A lost write.** An accepted batch that never becomes visible is
//!    indistinguishable from lag without waiting. That includes a frontier
//!    that stalls for good (`StalledFrontierStore`, a measured miss) and a
//!    tail write lost at zero lag.
//! 2. **A non-atomic reveal.** Part of a batch may be visible without the
//!    rest; the strict model's whole-log length check is what refuses that.
//! 3. **A read defect above the bracket's floor.** When the frontier moves
//!    during a read, a result that drops the newest matching events is
//!    explained by a lower frontier, so it passes.
//! 4. **A verdict nothing decides.** A deferred verdict whose deciding event
//!    is never shown, and is not the last of its batch, stays unjudged. So
//!    does any verdict still open when the sequence ends.
//! 5. **The exact position of a non-last batch member** until a read shows
//!    it. Gaps are legal, so only its bounds are known.
//!
//! Run at zero lag against every registered store, the measured cost is
//! nothing: `WEAKENING_COSTS` in the testkit's `tests/mutation_coverage.rs`
//! pins that set, and it is empty.

use core::mem;

use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, collect,
};

use super::{Op, read_options, render, resolve, select};

/// The highest representable position: the upper end of a placement with no
/// known neighbour above it.
///
/// Written as a `match` so the constant needs no panic. The `None` arm is
/// never taken, because `u64::MAX` is non-zero, and it is total anyway.
const CEILING: SequencePosition = match SequencePosition::new(u64::MAX) {
    Some(position) => position,
    None => SequencePosition::FIRST,
};

/// What the log should be, when a read may show only a prefix of it.
#[derive(Debug, Default)]
pub(super) struct TrailingModel {
    /// Every event the store accepted, in append order: what it **holds**.
    expected: Vec<Event>,
    /// The visible prefix of `expected`, as the last full read showed it.
    known: Vec<SequencedEvent>,
    /// `(index in expected of a batch's last event, the position append
    /// returned for that batch)`. Both components strictly ascend.
    returned: Vec<(usize, SequencePosition)>,
    /// Verdicts the store gave that were legal when given, and are re-judged
    /// as positions become known.
    deferred: Vec<Deferred>,
}

/// A conditional append whose verdict the model could not decide in advance.
#[derive(Debug)]
struct Deferred {
    /// The op's index in the sequence, so a later mismatch names it.
    op: usize,
    /// The condition the store judged.
    condition: AppendCondition,
    /// How many accepted events the store held when it judged it.
    held: usize,
    /// Whether the store accepted the append.
    accepted: bool,
}

/// What the model can say about a condition before the store judges it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Prediction {
    /// A held event certainly violates it: the store must refuse.
    Rejected,
    /// No held event can violate it: the store must accept.
    Accepted,
    /// Either verdict is legal on what the model knows now.
    Undecided,
}

/// Where a held event sits, as far as the model knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Placed {
    /// Shown by a read, or the last of a batch `append` returned.
    At(SequencePosition),
    /// Strictly between its nearest exactly-placed neighbours.
    Between {
        /// The exact position of its nearest earlier neighbour, which it sits
        /// above, or `None` when no earlier event is placed.
        above: Option<SequencePosition>,
        /// The exact position of its nearest later neighbour, which it sits
        /// below, or `None` when no later event is placed.
        below: Option<SequencePosition>,
    },
}

impl TrailingModel {
    /// The exact position of the held event at `index`, if the model has one.
    fn exact(&self, index: usize) -> Option<SequencePosition> {
        self.known
            .get(index)
            .map(|event| event.position)
            .or_else(|| {
                self.returned
                    .iter()
                    .find(|(at, _)| *at == index)
                    .map(|(_, position)| *position)
            })
    }

    /// Where the held event at `index` sits.
    fn placed(&self, index: usize) -> Placed {
        if let Some(position) = self.exact(index) {
            return Placed::At(position);
        }
        Placed::Between {
            above: (0..index).rev().find_map(|at| self.exact(at)),
            below: (index.saturating_add(1)..self.expected.len()).find_map(|at| self.exact(at)),
        }
    }

    /// What the store must answer to `condition`, judged over the first
    /// `held` accepted events.
    ///
    /// A violation is monotone in position: each guard is clear at or below
    /// its `after` and constant above it, and a condition is the disjunction
    /// of its guards. So an event placed in a range is probed at the range's
    /// two ends. Violated at the low end means violated wherever it is. Clear
    /// at the high end means clear wherever it is. Anything else is
    /// undecided until a later read or a returned position narrows the range.
    fn predict(&self, condition: &AppendCondition, held: usize) -> Prediction {
        let mut undecided = false;
        for (index, event) in self.expected.iter().take(held).enumerate() {
            // An empty range would mean the store's positions contradict
            // themselves. Undecided is the answer that can never false-fail.
            let Some((low, high)) = range(self.placed(index)) else {
                undecided = true;
                continue;
            };
            if condition.is_violated_by(low, event.event_type(), event.tags()) {
                return Prediction::Rejected;
            }
            if condition.is_violated_by(high, event.event_type(), event.tags()) {
                undecided = true;
            }
        }
        if undecided {
            Prediction::Undecided
        } else {
            Prediction::Accepted
        }
    }

    /// "visible N, held M", for a failure message a human reads once.
    fn counts(&self) -> String {
        format!("visible {}, held {}", self.known.len(), self.expected.len())
    }

    /// Holds a full read to everything a trailing store still promises (T1).
    ///
    /// It is a prefix of what is held, by content and order. It is no shorter
    /// than the last full read. Its positions strictly increase and none has
    /// moved. Every visible batch-last sits where `append` said, and nothing
    /// visible sits at or above a position `append` returned for a batch that
    /// is not visible yet.
    fn check_log(&self, observed: &[SequencedEvent]) -> Result<(), String> {
        if observed.len() > self.expected.len() {
            return Err(format!(
                "the log shows {} event(s), and only {} were accepted ({})",
                observed.len(),
                self.expected.len(),
                self.counts()
            ));
        }
        for (index, (seen, wanted)) in observed.iter().zip(&self.expected).enumerate() {
            if &seen.event != wanted {
                return Err(format!(
                    "the event at index {index} is not the one accepted there, so the \
                     read is not a prefix of the accepted log: read back {:?}, \
                     appended {wanted:?} ({})",
                    seen.event,
                    self.counts()
                ));
            }
        }
        if observed.len() < self.known.len() {
            return Err(format!(
                "the frontier went backward: a full read showed {} event(s) and a \
                 later one shows {}. The trailing model assumes visibility never \
                 moves backward, which no clause states yet ({})",
                self.known.len(),
                observed.len(),
                self.counts()
            ));
        }
        for (earlier, later) in observed.iter().zip(observed.iter().skip(1)) {
            if later.position <= earlier.position {
                return Err(format!(
                    "positions must be unique and strictly increasing in assignment \
                     order, but {} follows {} ({})",
                    later.position,
                    earlier.position,
                    self.counts()
                ));
            }
        }
        for (index, (before, now)) in self.known.iter().zip(observed).enumerate() {
            if before.position != now.position {
                return Err(format!(
                    "the event at index {index} was shown at {}, and is now shown at \
                     {} ({})",
                    before.position,
                    now.position,
                    self.counts()
                ));
            }
        }
        self.check_returned(observed)
    }

    /// T1's last two checks: the positions `append` returned, against a read.
    fn check_returned(&self, observed: &[SequencedEvent]) -> Result<(), String> {
        let highest = observed.last().map(|event| event.position);
        for &(index, returned) in &self.returned {
            match observed.get(index) {
                Some(seen) if seen.position != returned => {
                    return Err(format!(
                        "`append` returned {returned} for the batch ending at index \
                         {index}, and the log shows that event at {} ({})",
                        seen.position,
                        self.counts()
                    ));
                }
                Some(_) => {}
                None => {
                    if let Some(highest) = highest
                        && highest >= returned
                    {
                        return Err(format!(
                            "{highest} is visible while the batch `append` returned \
                             {returned} for is not, and position order is visibility \
                             order (ES-10) ({})",
                            self.counts()
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// Holds a read to the frontier bracket (T2).
    ///
    /// `before` is how many events were visible to the full read before it.
    /// `self.known` is already the full read after it. A read is one snapshot
    /// (ES-11), and its frontier lies between those two, so its result must
    /// be the selection over **some** prefix in that range.
    fn check_read(
        &self,
        before: usize,
        query: &Query,
        options: ReadOptions,
        observed: &[SequencedEvent],
    ) -> Result<(), String> {
        let explained = (before..=self.known.len()).any(|visible| {
            self.known
                .get(..visible)
                .is_some_and(|prefix| select(prefix, query, options) == observed)
        });
        if explained {
            Ok(())
        } else {
            Err(format!(
                "read({query:?}, {options:?}) returned {}, which no frontier between \
                 the {before} event(s) visible before it and the {} visible after it \
                 explains; over all of them the model expected {} ({})",
                render(observed),
                self.known.len(),
                render(&select(&self.known, query, options)),
                self.counts()
            ))
        }
    }

    /// Re-judges every deferred verdict against what is known now (T3).
    ///
    /// A verdict that is now decided either agrees, and is dropped, or
    /// disagrees, and fails the sequence naming the op that received it.
    fn rejudge(&mut self) -> Result<(), String> {
        let pending = mem::take(&mut self.deferred);
        for deferred in pending {
            match (
                self.predict(&deferred.condition, deferred.held),
                deferred.accepted,
            ) {
                (Prediction::Undecided, _) => self.deferred.push(deferred),
                (Prediction::Rejected, false) | (Prediction::Accepted, true) => {}
                (Prediction::Rejected, true) => {
                    return Err(format!(
                        "op {} was accepted under {:?}, and a held event is now known \
                         to violate it ({})",
                        deferred.op,
                        deferred.condition,
                        self.counts()
                    ));
                }
                (Prediction::Accepted, false) => {
                    return Err(format!(
                        "op {} was rejected under {:?}, and no held event is now able \
                         to violate it ({})",
                        deferred.op,
                        deferred.condition,
                        self.counts()
                    ));
                }
            }
        }
        Ok(())
    }

    /// Adopts an accepted batch, and the position `append` returned for it.
    ///
    /// That position must lie above every position already known, because
    /// positions strictly increase in assignment order (VT-11) and it is the
    /// batch's last (ES-19).
    fn accept(&mut self, events: &[Event], written: SequencePosition) -> Result<(), String> {
        let floor = self
            .known
            .last()
            .map(|event| event.position)
            .max(self.returned.last().map(|(_, position)| *position));
        if let Some(floor) = floor
            && written <= floor
        {
            return Err(format!(
                "`append` returned {written}, which is not above {floor}, a position \
                 an earlier batch already holds ({})",
                self.counts()
            ));
        }
        self.expected.extend(events.iter().cloned());
        if let Some(last) = self.expected.len().checked_sub(1) {
            self.returned.push((last, written));
        }
        Ok(())
    }

    /// Reads the whole log, checks it (T1), adopts it, and re-judges (T3).
    async fn reconcile<S: EventStore>(&mut self, store: &S) -> Result<(), String> {
        let observed = match collect(store.read(&Query::all(), ReadOptions::new())).await {
            Ok(events) => events,
            Err(err) => return Err(format!("reading the whole log failed: {err:?}")),
        };
        self.check_log(&observed)?;
        self.known = observed;
        self.rejudge()
    }

    /// Runs op number `index` and checks everything a trailing store promises.
    pub(super) async fn apply<S: EventStore>(
        &mut self,
        store: &S,
        index: usize,
        op: &Op,
    ) -> Result<(), String> {
        match op {
            Op::Append { events } => {
                let written = match store.append(events, None).await {
                    Ok(position) => position,
                    Err(err) => {
                        return Err(format!("an unconditional append was refused: {err:?}"));
                    }
                };
                self.accept(events, written)?;
                self.reconcile(store).await
            }

            Op::AppendConditional {
                events,
                query,
                anchor,
            } => {
                let condition = AppendCondition::new(query.clone())
                    .after_every_guard_opt(resolve(&self.known, *anchor));
                self.append_conditional(store, index, events, condition)
                    .await?;
                self.reconcile(store).await
            }

            Op::Read {
                query,
                from,
                to,
                backwards,
                limit,
            } => {
                let before = self.known.len();
                let options = read_options(&self.known, *from, *to, *backwards, *limit);
                let observed = match collect(store.read(query, options)).await {
                    Ok(events) => events,
                    Err(err) => return Err(format!("the read failed: {err:?}")),
                };
                self.reconcile(store).await?;
                self.check_read(before, query, options, &observed)
            }
        }
    }

    /// The conditional append, judged against the three-valued prediction.
    ///
    /// The reported `conflicting_position` is ignored, for the reason the
    /// strict model's `violation` gives (ES-25).
    async fn append_conditional<S: EventStore>(
        &mut self,
        store: &S,
        index: usize,
        events: &[Event],
        condition: AppendCondition,
    ) -> Result<(), String> {
        let held = self.expected.len();
        let predicted = self.predict(&condition, held);
        let result = store.append(events, Some(&condition)).await;

        let accepted = match (predicted, result) {
            (
                Prediction::Rejected | Prediction::Undecided,
                Err(AppendError::ConditionViolated(_)),
            ) => false,
            (Prediction::Accepted | Prediction::Undecided, Ok(written)) => {
                self.accept(events, written)?;
                true
            }
            (Prediction::Rejected, Ok(written)) => {
                return Err(format!(
                    "the append was accepted at {written}, but a held event violates \
                     {condition:?} ({})",
                    self.counts()
                ));
            }
            (Prediction::Accepted, Err(AppendError::ConditionViolated(violated))) => {
                return Err(format!(
                    "the append was rejected as {violated:?}, but no held event can \
                     violate {condition:?} ({})",
                    self.counts()
                ));
            }
            (_, Err(other)) => {
                return Err(format!("a conditional append failed outright: {other:?}"));
            }
        };

        if predicted == Prediction::Undecided {
            self.deferred.push(Deferred {
                op: index,
                condition,
                held,
                accepted,
            });
        }
        Ok(())
    }
}

/// The positions a placed event can hold, as inclusive `(low, high)`, or
/// `None` when no position fits.
fn range(placed: Placed) -> Option<(SequencePosition, SequencePosition)> {
    match placed {
        Placed::At(position) => Some((position, position)),
        Placed::Between { above, below } => {
            let low = match above {
                Some(above) => above.next()?,
                None => SequencePosition::FIRST,
            };
            let high = match below {
                Some(below) => SequencePosition::new(below.get().checked_sub(1)?)?,
                None => CEILING,
            };
            (low <= high).then_some((low, high))
        }
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{
        AppendCondition, Event, EventId, Query, QueryItem, ReadOptions, RecordedAt,
        SequencePosition, SequencedEvent, StoreId,
    };

    use super::{Deferred, Prediction, TrailingModel};

    fn event(event_type: &str) -> Event {
        Event::new(event_type, &b"x"[..]).expect("a literal event type is valid")
    }

    fn at(position: u64) -> SequencePosition {
        SequencePosition::new(position).expect("a test position is non-zero")
    }

    fn shown(position: u64, event_type: &str) -> SequencedEvent {
        SequencedEvent::new(
            at(position),
            EventId::new(StoreId::from_bytes([7; 16]), at(position)),
            RecordedAt::from_millis(0),
            event(event_type),
        )
    }

    fn fails_on(event_type: &str, after: Option<u64>) -> AppendCondition {
        let item = QueryItem::of_types([event_type]).expect("a literal type is a valid item");
        AppendCondition::new(Query::from_item(item)).after_every_guard_opt(after.map(at))
    }

    /// Two single-event batches held, `A` at 10 visible and `B` invisible. The
    /// second batch's returned position is known only if `returned` says so.
    fn lagging(returned: Vec<(usize, SequencePosition)>) -> TrailingModel {
        TrailingModel {
            expected: vec![event("A"), event("B")],
            known: vec![shown(10, "A")],
            returned,
            ..TrailingModel::default()
        }
    }

    #[test]
    fn a_visible_violator_predicts_rejection() {
        let model = lagging(vec![(0, at(10))]);

        assert_eq!(model.predict(&fails_on("A", None), 2), Prediction::Rejected);
    }

    #[test]
    fn an_invisible_violator_at_or_below_the_visible_head_predicts_rejection() {
        let model = lagging(vec![(0, at(10))]);

        assert_eq!(
            model.predict(&fails_on("B", Some(10)), 2),
            Prediction::Rejected
        );
    }

    #[test]
    fn an_invisible_candidate_beyond_the_visible_head_is_undecided() {
        let model = lagging(vec![(0, at(10))]);

        assert_eq!(
            model.predict(&fails_on("B", Some(11)), 2),
            Prediction::Undecided
        );
    }

    #[test]
    fn a_query_no_held_event_matches_predicts_acceptance() {
        let model = lagging(vec![(0, at(10))]);

        assert_eq!(model.predict(&fails_on("C", None), 2), Prediction::Accepted);
    }

    #[test]
    fn a_returned_batch_position_decides_the_beyond_head_case() {
        let violated = lagging(vec![(0, at(10)), (1, at(12))]);
        let clear = lagging(vec![(0, at(10)), (1, at(11))]);

        assert_eq!(
            violated.predict(&fails_on("B", Some(11)), 2),
            Prediction::Rejected
        );
        assert_eq!(
            clear.predict(&fails_on("B", Some(11)), 2),
            Prediction::Accepted
        );
    }

    /// `A` visible at 10, `B` and `C` held in one batch `append` returned 20
    /// for, neither visible yet.
    fn behind_a_batch() -> TrailingModel {
        TrailingModel {
            expected: vec![event("A"), event("B"), event("C")],
            known: vec![shown(10, "A")],
            returned: vec![(0, at(10)), (2, at(20))],
            ..TrailingModel::default()
        }
    }

    #[test]
    fn reconcile_accepts_a_strict_prefix_of_what_is_held() {
        let model = behind_a_batch();

        assert_eq!(model.check_log(&[shown(10, "A")]), Ok(()));
        assert_eq!(
            model.check_log(&[shown(10, "A"), shown(15, "B"), shown(20, "C")]),
            Ok(())
        );
    }

    #[test]
    fn reconcile_rejects_a_read_that_is_not_a_prefix() {
        let model = behind_a_batch();

        let skipped = model.check_log(&[shown(10, "A"), shown(20, "C")]);

        assert!(
            skipped.is_err_and(|failure| failure.contains("index 1")),
            "a read that skips a held event is not a prefix"
        );
    }

    #[test]
    fn reconcile_rejects_a_frontier_that_moved_backward() {
        let model = behind_a_batch();

        let shrunk = model.check_log(&[]);

        assert!(shrunk.is_err_and(|failure| failure.contains("went backward")));
    }

    #[test]
    fn reconcile_rejects_a_position_that_moved() {
        let model = behind_a_batch();

        let moved = model.check_log(&[shown(11, "A")]);

        assert!(moved.is_err_and(|failure| failure.contains("was shown at 10")));
    }

    #[test]
    fn reconcile_rejects_positions_that_do_not_increase() {
        let model = behind_a_batch();

        let repeated = model.check_log(&[shown(10, "A"), shown(10, "B")]);

        assert!(repeated.is_err_and(|failure| failure.contains("strictly increasing")));
    }

    #[test]
    fn reconcile_rejects_a_visible_batch_last_not_where_append_said() {
        let model = behind_a_batch();

        let elsewhere = model.check_log(&[shown(10, "A"), shown(15, "B"), shown(21, "C")]);

        assert!(elsewhere.is_err_and(|failure| failure.contains("returned 20")));
    }

    #[test]
    fn reconcile_rejects_a_visible_position_above_an_invisible_returned_one() {
        // One batch of two, `append` returned 12 for it, and nothing shown yet.
        let model = TrailingModel {
            expected: vec![event("A"), event("B")],
            returned: vec![(1, at(12))],
            ..TrailingModel::default()
        };

        let below = model.check_log(&[shown(11, "A")]);
        let above = model.check_log(&[shown(13, "A")]);

        assert_eq!(below, Ok(()));
        assert!(above.is_err_and(|failure| failure.contains("position order is visibility order")));
    }

    /// The model after a full read showed `A`, `B` and `C`, of which only `A`
    /// was visible to the read before it.
    fn caught_up() -> TrailingModel {
        TrailingModel {
            expected: vec![event("A"), event("B"), event("C")],
            known: vec![shown(10, "A"), shown(11, "B"), shown(12, "C")],
            returned: vec![(0, at(10)), (1, at(11)), (2, at(12))],
            ..TrailingModel::default()
        }
    }

    #[test]
    fn a_read_explained_by_an_intermediate_frontier_is_accepted() {
        let model = caught_up();

        let between = model.check_read(
            1,
            &Query::all(),
            ReadOptions::new(),
            &[shown(10, "A"), shown(11, "B")],
        );

        assert_eq!(between, Ok(()));
    }

    #[test]
    fn a_read_explained_by_no_frontier_is_rejected() {
        let model = caught_up();

        let holey = model.check_read(
            1,
            &Query::all(),
            ReadOptions::new(),
            &[shown(10, "A"), shown(12, "C")],
        );
        let behind = model.check_read(1, &Query::all(), ReadOptions::new(), &[]);

        assert!(holey.is_err_and(|failure| failure.contains("no frontier")));
        assert!(behind.is_err_and(|failure| failure.contains("no frontier")));
    }

    /// `B` is held above `A`'s 10 with no position known, and op 3 was
    /// accepted under a condition only an event above 11 violates.
    fn deferred_on_b() -> TrailingModel {
        TrailingModel {
            expected: vec![event("A"), event("B")],
            known: vec![shown(10, "A")],
            returned: vec![(0, at(10))],
            deferred: vec![Deferred {
                op: 3,
                condition: fails_on("B", Some(11)),
                held: 2,
                accepted: true,
            }],
        }
    }

    #[test]
    fn a_deferred_verdict_stays_open_until_positions_decide_it() {
        let mut model = deferred_on_b();

        assert_eq!(model.rejudge(), Ok(()));
        assert_eq!(model.deferred.len(), 1);

        model.known.push(shown(11, "B"));
        assert_eq!(model.rejudge(), Ok(()));
        assert!(model.deferred.is_empty(), "B at 11 decides the verdict");
    }

    #[test]
    fn a_deferred_verdict_contradicted_later_names_its_op() {
        let mut model = deferred_on_b();

        model.known.push(shown(12, "B"));
        let contradicted = model.rejudge();

        assert!(contradicted.is_err_and(|failure| failure.contains("op 3")));
    }
}
