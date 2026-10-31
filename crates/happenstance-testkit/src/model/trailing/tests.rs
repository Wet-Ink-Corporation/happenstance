//! Unit tests for the trailing model's pure steps: prediction, the log and
//! read checks, re-judging, and adopting an accepted batch.

use happenstance_core::{
    AppendCondition, Event, EventId, Query, QueryItem, ReadOptions, RecordedAt, SequencePosition,
    SequencedEvent, StoreId,
};

use super::{Deferred, Placed, Prediction, TrailingModel, range};

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

#[test]
fn a_range_no_position_fits_is_none() {
    let squeezed = Placed::Between {
        above: Some(at(5)),
        below: Some(at(6)),
    };
    let below_first = Placed::Between {
        above: None,
        below: Some(SequencePosition::FIRST),
    };

    assert_eq!(range(squeezed), None);
    assert_eq!(range(below_first), None);
}

#[test]
fn an_event_with_no_position_left_is_undecided_rather_than_a_failure() {
    // `B` sits between 10 and 11, where no position exists. The store
    // contradicts itself, and the only safe prediction is "either".
    let model = TrailingModel {
        expected: vec![event("A"), event("B"), event("C")],
        known: vec![shown(10, "A")],
        returned: vec![(0, at(10)), (2, at(11))],
        ..TrailingModel::default()
    };

    assert_eq!(
        model.predict(&fails_on("B", Some(10)), 3),
        Prediction::Undecided
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

/// `B` is held above `A`'s 10 with no position known, and op 3 got the
/// verdict `accepted` under a condition only an event above 11 violates.
fn deferred_on_b(accepted: bool) -> TrailingModel {
    TrailingModel {
        expected: vec![event("A"), event("B")],
        known: vec![shown(10, "A")],
        returned: vec![(0, at(10))],
        deferred: vec![Deferred {
            op: 3,
            condition: fails_on("B", Some(11)),
            held: 2,
            accepted,
        }],
    }
}

#[test]
fn a_deferred_verdict_stays_open_until_positions_decide_it() {
    let mut model = deferred_on_b(true);

    assert_eq!(model.rejudge(), Ok(()));
    assert_eq!(model.deferred.len(), 1);

    model.known.push(shown(11, "B"));
    assert_eq!(model.rejudge(), Ok(()));
    assert!(model.deferred.is_empty(), "B at 11 decides the verdict");
}

#[test]
fn a_deferred_acceptance_contradicted_later_names_its_op() {
    let mut model = deferred_on_b(true);

    model.known.push(shown(12, "B"));
    let contradicted = model.rejudge();

    assert!(contradicted.is_err_and(|failure| failure.contains("op 3 was accepted")));
}

#[test]
fn a_deferred_refusal_contradicted_later_names_its_op() {
    let mut model = deferred_on_b(false);

    // `B` at 11 is not above 11, so nothing held violates the condition and
    // the refusal was wrong.
    model.known.push(shown(11, "B"));
    let contradicted = model.rejudge();

    assert!(contradicted.is_err_and(|failure| failure.contains("op 3 was rejected")));
}

#[test]
fn an_append_returning_a_position_already_held_is_refused() {
    let mut model = caught_up();

    let at_head = model.accept(&[event("D")], at(12));
    let below_head = model.accept(&[event("D")], at(11));

    assert!(at_head.is_err_and(|failure| failure.contains("not above 12")));
    assert!(below_head.is_err_and(|failure| failure.contains("not above 12")));
    assert_eq!(model.expected.len(), 3, "a refused position adopts nothing");
}

#[test]
fn an_append_above_every_known_position_is_adopted() {
    let mut model = caught_up();

    let adopted = model.accept(&[event("D"), event("E")], at(30));

    assert_eq!(adopted, Ok(()));
    assert_eq!(model.expected.len(), 5);
    assert_eq!(model.returned.last(), Some(&(4, at(30))));
}
