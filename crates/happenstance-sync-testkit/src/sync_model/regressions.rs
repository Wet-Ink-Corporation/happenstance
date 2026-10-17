//! The three scenarios pinned by value, which every model rule runs before any
//! generated case.

use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::scenario::{Direction, Fault, Guard, Pick, Scenario, Step, WriteSize};

/// A scenario pinned by value rather than by seed: a seed's meaning changes
/// whenever the generator does, and these must not.
#[derive(Debug)]
pub(crate) struct Regression {
    pub(crate) name: &'static str,
    pub(crate) scenario: Scenario,
}

/// The chain every regression runs on: P(0) — H(1) — Q(2). Edge 0 is P–H and
/// edge 1 is H–Q. No regression writes at H, so H has no pick of its own.
const CHAIN: [(usize, usize); 2] = [(0, 1), (1, 2)];
const P: Pick = Pick::new(0);
const Q: Pick = Pick::new(2);
const PH: Pick = Pick::new(0);
const HQ: Pick = Pick::new(1);

const fn write(at: Pick, events: WriteSize) -> Step {
    Step::Write {
        at,
        events,
        guard: Guard::Unguarded,
    }
}

const fn exchange(edge: Pick, toward: Direction, fault: Fault) -> Step {
    Step::Exchange {
        edge,
        toward,
        fault,
    }
}

/// The three regressions, in the order every rule runs them.
///
/// # Errors
///
/// A message if one of them is not a topology the family accepts: the
/// testkit's own defect.
pub(crate) fn regressions() -> Result<Vec<Regression>, String> {
    use Direction::{Backward, Forward};

    let cases = [
        // SF-03: a page that half overlaps what its receiver holds. P's echo,
        // a lost ack that leaves P's token behind B, and H's page `[B],[C]`
        // arriving at P, which holds A and B. One appended, one skipped; and
        // P's re-push of `[B],[C]` is two skips at H.
        (
            "sf03_partial_overlap",
            vec![
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::None),
                exchange(PH, Backward, Fault::None),
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::DropAck),
                write(Q, WriteSize::ONE),
                exchange(HQ, Backward, Fault::None),
                exchange(PH, Backward, Fault::None),
                exchange(PH, Forward, Fault::None),
            ],
        ),
        // A two-event group whose ack is lost: re-pushed whole, then echoed
        // back to its origin. Both redeliveries are two skips.
        (
            "lost_ack_repush_and_echo",
            vec![
                write(P, WriteSize::TWO),
                exchange(PH, Forward, Fault::DropAck),
                exchange(PH, Forward, Fault::None),
                exchange(PH, Backward, Fault::None),
            ],
        ),
        // P's first event is held in flight and lands at H after P's second:
        // an origin's lower position arriving after its higher one. A peer
        // that dedupes by position prefix drops it.
        (
            "delayed_page_lands_after_a_later_one",
            vec![
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::Delay),
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::None),
            ],
        ),
    ];
    cases
        .into_iter()
        .map(|(name, steps)| {
            Scenario::fixed(CHAIN.len() + 1, &CHAIN, steps)
                .map(|scenario| Regression { name, scenario })
                .ok_or_else(|| format!("the regression scenario `{name}` is not a valid topology"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_regression_is_a_valid_scenario() {
        let all = super::regressions().expect("every regression is a valid topology");
        assert_eq!(all.len(), 3);
    }
}
