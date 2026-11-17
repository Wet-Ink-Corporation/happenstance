//! Central's rule, asked about one push at a time.

use happenstance::DecisionModel;

use super::{adjudicate, schedule, sell_seat};
use crate::domain::{BoxOffice, Place, ShowId, ShowLedger};

/// A show of `capacity` seats, as a ledger that holds its scheduling.
fn scheduled(capacity: u32) -> ShowLedger {
    let show = ShowId::new("premiere").expect("a valid show");
    let mut ledger = ShowLedger::new(&show);
    for event in schedule(&ledger, capacity).expect("scheduling is infallible") {
        ledger.apply(event);
    }
    ledger
}

/// The sale `at` would decide against `ledger`.
fn sale(ledger: &ShowLedger, at: Place) -> BoxOffice {
    let mut decided = sell_seat(ledger, at, at.as_str()).expect("a free seat");
    decided.pop().expect("one sale")
}

fn refunded(answer: &[BoxOffice]) -> Vec<&str> {
    answer
        .iter()
        .filter_map(|event| match event {
            BoxOffice::SaleRefunded { sale, .. } => Some(sale.as_str()),
            BoxOffice::ShowScheduled { .. } | BoxOffice::SeatSold { .. } => None,
        })
        .collect()
}

/// The sale that lands past capacity is refunded, and the one before it in
/// the same push keeps its seat. Rejects a rule that judges a group against
/// the store alone, which sees no contest in a push of two.
#[test]
fn the_later_sale_in_one_push_is_refunded() {
    let ledger = scheduled(1);
    let tablet = sale(&ledger, Place::Tablet);
    let phone = sale(&ledger, Place::Phone);

    let first = adjudicate(ledger.clone(), Vec::new(), [tablet.clone()]);
    let second = adjudicate(ledger, [vec![tablet]], [phone]);

    assert!(first.is_empty(), "the first sale keeps the seat: {first:?}");
    assert_eq!(refunded(&second), ["phone-1"]);
}

/// Two groups in one push, each past capacity: each earns its refund, and
/// the second's reason counts the sales holding a seat after the first's
/// refund. Rejects a rule that folds an earlier group without the refund it
/// earned, which would count that sale as still holding (3 rather than 2).
#[test]
fn a_second_oversold_group_counts_the_first_ones_refund() {
    let mut ledger = scheduled(1);
    let tablet = sale(&ledger, Place::Tablet);
    let phone = sale(&ledger, Place::Phone);
    ledger.apply(sale(&ledger, Place::Central));

    // The tablet's sale is judged alone, then as the earlier group of the
    // phone's push: the same event in two calls, so it is cloned once.
    let first = adjudicate(ledger.clone(), Vec::new(), [tablet.clone()]);
    let second = adjudicate(ledger, [vec![tablet]], [phone]);

    assert_eq!(refunded(&first), ["tablet-1"]);
    assert_eq!(refunded(&second), ["phone-1"]);
    let seen: Vec<u32> = second
        .iter()
        .filter_map(|event| match event {
            BoxOffice::SaleRefunded { reason, .. } => Some(reason.sales_seen),
            BoxOffice::ShowScheduled { .. } | BoxOffice::SeatSold { .. } => None,
        })
        .collect();
    assert_eq!(
        seen,
        [2],
        "central-1 and phone-1 hold seats; tablet-1 was refunded"
    );
}

/// A sale central already holds earns nothing when it arrives again, refunded
/// or not. Rejects a rule that counts a re-delivered sale as a new arrival,
/// which refunds a held sale a second time.
#[test]
fn a_sale_delivered_again_earns_nothing() {
    let mut ledger = scheduled(1);
    let tablet = sale(&ledger, Place::Tablet);
    let phone = sale(&ledger, Place::Phone);
    ledger.apply(tablet.clone());
    // What central holds once the phone's push lands: the sale, then its
    // refund, in one write.
    let refund = adjudicate(ledger.clone(), Vec::new(), [phone.clone()]);
    assert_eq!(refunded(&refund), ["phone-1"]);
    for event in [phone.clone()].into_iter().chain(refund) {
        ledger.apply(event);
    }

    for again in [tablet, phone] {
        let answer = adjudicate(ledger.clone(), Vec::new(), [again]);
        assert!(answer.is_empty(), "a judged sale, judged again: {answer:?}");
    }
}
