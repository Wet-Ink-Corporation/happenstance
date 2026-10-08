//! The k = 8 schedule both legs run, checked against a fake store.

use append_batch_ownership::contention::{self, Attempt, CONTENDERS, Tally};
use happenstance_core::SequencePosition;

/// Rejects: a schedule with no rejections, which measures the uncontended path
/// only (ADR-0012 item 3). Every contender decides at the round's boundary, the
/// first to attempt commits and the rest are refused, so k contenders make
/// `k(k+1)/2` attempts: 36 for 8, of which 28 are refusals.
#[test]
fn eight_contenders_make_thirty_six_attempts_of_which_twenty_eight_are_refused() {
    let mut head: Option<SequencePosition> = None;
    let mut next = 1_u64;
    let tally = contention::run(|_contender, decided_at| {
        if decided_at != head {
            return Attempt::<()>::Refused;
        }
        let position = SequencePosition::new(next).expect("non-zero");
        next += 1;
        head = Some(position);
        Attempt::Committed(position)
    });

    assert_eq!(CONTENDERS, 8);
    assert_eq!(
        tally,
        Ok(Tally {
            attempts: 36,
            commits: 8,
            rejections: 28
        })
    );
}

/// A store failure ends the run and is handed back, rather than spinning.
#[test]
fn a_failed_attempt_ends_the_run() {
    let tally = contention::run(|_, _| Attempt::Failed("disk"));
    assert_eq!(tally, Err("disk"));
}
