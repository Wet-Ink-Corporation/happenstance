//! CF-42's two parity claims: a settle is checked, and settling costs the suite
//! no rejection it had.
//!
//! `a_settle_that_reveals_nothing_is_caught_at_the_anchors` runs the read-back
//! rules against [`EmptySettle`], whose `settle_frontier` answers `Visible` and
//! reveals nothing. A rule that read an empty log after such a settle and passed
//! would be a rule a lying fixture could satisfy, so every read-back rule must
//! fail it from its own body, bar the rows [`EMPTY_SETTLE_UNCAUGHT`] measures.
//!
//! `settling_costs_no_mutant_coverage` runs every racer, and every registered
//! store a lag can reach, behind a [`Settles`](super::Settles) lag and holds each
//! to its own registry row, bar the rows [`LAG_MASKED`] measures. Which stores a
//! lag can reach is measured too, by
//! `the_lag_exposed_stores_are_every_store_a_lag_can_reach`: only a rule that
//! reads while the lag runs can see a lagging store differ from the store it
//! wraps, and [`Tripwire`] finds those rules.

use std::collections::BTreeSet;

use super::inventory::READS_BACK_ITS_OWN_WRITES;
use super::{EmptySettle, LaggingFixture, TRIPPED, Tripwire};
use crate::harness::{
    RUNTIME_PANICS, SubjectReport, Verdict, run_concurrency_subject, run_subject,
};
use crate::mutation_coverage::{assert_declared_failure, assert_undeclared_outcome};
use crate::racers::LockedFixture;
use crate::variants::{FrontierFixture, GappedPositionFixture, Held};
use crate::{
    Kind, RACERS, REGISTRY, RacerOutcome, declared, lag_exposed_names, lagging_racer_reports,
    lagging_reports,
};

/// The read-back rules an empty settle does not make fail, each with why.
///
/// Every row is measured: `a_settle_that_reveals_nothing_is_caught_at_the_anchors`
/// fails on a row whose rule the empty settle does catch, so the table cannot
/// keep a rule that has since gained an anchor.
///
/// Each row's read-back is absence, or a property the empty log has, so a read
/// that shows nothing satisfies it. A lying settle is still caught on the same
/// run by every other read-back rule, because a fixture runs the whole suite.
const EMPTY_SETTLE_UNCAUGHT: &[(&str, &str)] = &[
    (
        "query_item_rejects_partial_tag_overlap",
        "its only read asserts that nothing matches",
    ),
    (
        "positions_are_unique",
        "it asserts the positions read are distinct, which no positions are",
    ),
    (
        "append_reports_exceeded_store_limits",
        "its reads look for the leftovers of refused appends, and find none",
    ),
];

/// The `(store, rule)` pairs a lag hides from a rule that rejects the store
/// without one, each with why the lag rather than the settle does the hiding.
///
/// Every row is measured: `settling_costs_no_mutant_coverage` fails on a row
/// the lagged rule still rejects.
const LAG_MASKED: &[(&str, &str, &str)] = &[
    (
        "EmptyArmFrontierFixture",
        "contains_event_id_reports_a_held_row_above_the_frontier",
        "its arming holds nothing, and the lag holds the rule's append above the \
         frontier in its place: the instrument is itself a store with a held row, \
         so the pair is the conformant shape the rule asks for",
    ),
    (
        "PreCommitPositionStore",
        "nothing_below_an_observed_position_appears_later",
        "the defect shows only at step 2, where the fast writer's row is visible \
         with the slow writer's lower one in flight. The fast writer's acknowledged \
         append starts the lag, so that read shows nothing, and the one settle \
         cannot move earlier because the slow writer is suspended there (I6). A \
         frontier is the remedy for this store's defect, and the lag is one",
    ),
    (
        "PollPaddedPositionStore",
        "nothing_below_an_observed_position_appears_later",
        "`PreCommitPositionStore`'s defect with one more poll, hidden at the same \
         step for the same reason",
    ),
];

/// Whether a verdict is a rule rejecting the store from its own body.
fn caught_by_a_rule(verdict: &Verdict) -> bool {
    match verdict {
        Verdict::Panicked { message, origin } => {
            origin
                .as_ref()
                .is_some_and(|at| at.is_a_rule_body() || at.is_a_concurrency_rule_body())
                && !RUNTIME_PANICS.iter().any(|needle| message.contains(needle))
        }
        Verdict::Passed | Verdict::Skipped { .. } => false,
    }
}

/// I5: a settle that answers `Visible` and reveals nothing fails every
/// read-back rule at an anchor in the rule itself.
#[test]
fn a_settle_that_reveals_nothing_is_caught_at_the_anchors() {
    let reports: [SubjectReport; 3] = [
        run_subject::<LaggingFixture<GappedPositionFixture, EmptySettle>>(),
        run_subject::<LaggingFixture<FrontierFixture<Held>, EmptySettle>>(),
        run_concurrency_subject::<LaggingFixture<LockedFixture, EmptySettle>>(),
    ];
    let caught: BTreeSet<&str> = reports
        .iter()
        .flat_map(|report| report.outcomes.iter())
        .filter(|(_, verdict)| caught_by_a_rule(verdict))
        .map(|(rule, _)| *rule)
        .collect();
    let excused: BTreeSet<&str> = EMPTY_SETTLE_UNCAUGHT
        .iter()
        .map(|(rule, _)| *rule)
        .collect();

    let uncaught: Vec<String> = READS_BACK_ITS_OWN_WRITES
        .iter()
        .filter(|rule| !caught.contains(*rule) && !excused.contains(*rule))
        .map(|rule| {
            let seen: Vec<String> = reports
                .iter()
                .filter_map(|report| report.verdict(rule).map(|v| (report.name, v)))
                .map(|(name, verdict)| format!("{name}: {}", verdict.describe()))
                .collect();
            format!("`{rule}` ({})", seen.join("; "))
        })
        .collect();
    assert!(
        uncaught.is_empty(),
        "{} read-back rule(s) passed against a fixture whose settle reveals \
         nothing, so a lying `settle_frontier` satisfies them (CF-42):\n{}",
        uncaught.len(),
        uncaught.join("\n")
    );

    let stale: Vec<&str> = EMPTY_SETTLE_UNCAUGHT
        .iter()
        .map(|(rule, _)| *rule)
        .filter(|rule| caught.contains(rule))
        .collect();
    assert!(
        stale.is_empty(),
        "these `EMPTY_SETTLE_UNCAUGHT` rows are caught now; remove them: {stale:?}"
    );
}

/// The rules that read while a lag runs, measured on conformant stores behind
/// [`Tripwire`].
///
/// Three subjects for `an_unsettling_lag_skips_exactly_the_read_back_rules`'s
/// reason: between them they reach every rule past its capability checks.
fn rules_that_read_during_a_lag() -> BTreeSet<&'static str> {
    let reports: [SubjectReport; 3] = [
        run_subject::<LaggingFixture<GappedPositionFixture, Tripwire>>(),
        run_subject::<LaggingFixture<FrontierFixture<Held>, Tripwire>>(),
        run_concurrency_subject::<LaggingFixture<LockedFixture, Tripwire>>(),
    ];
    let mut tripped = BTreeSet::new();
    for report in &reports {
        for (rule, verdict) in &report.outcomes {
            match verdict {
                Verdict::Panicked { message, .. } if message.contains(TRIPPED) => {
                    tripped.insert(*rule);
                }
                Verdict::Passed | Verdict::Skipped { .. } => {}
                Verdict::Panicked { .. } => panic!(
                    "`{rule}` failed `{}` behind the tripwire for another reason than \
                     a read during a lag: {}",
                    report.name,
                    verdict.describe()
                ),
            }
        }
    }
    tripped
}

/// `for_each_lag_exposed!` is exactly the registered stores that fail a rule
/// which reads while a lag runs. Every other store answers every rule behind a
/// settling lag as it does without one, so running it there measures nothing.
#[test]
fn the_lag_exposed_stores_are_every_store_a_lag_can_reach() {
    let exposed_rules = rules_that_read_during_a_lag();
    let required: BTreeSet<&str> = REGISTRY
        .iter()
        .filter(|entry| entry.fails.iter().any(|rule| exposed_rules.contains(rule)))
        .map(|entry| entry.name)
        .collect();
    let listed: BTreeSet<&str> = lag_exposed_names().into_iter().collect();

    let missing: Vec<&&str> = required.difference(&listed).collect();
    assert!(
        missing.is_empty(),
        "these stores fail a rule that reads during a lag ({exposed_rules:?}), so a \
         lag could hide their defect; add them to `for_each_lag_exposed!`: {missing:?}"
    );
    let idle: Vec<&&str> = listed.difference(&required).collect();
    assert!(
        idle.is_empty(),
        "these stores in `for_each_lag_exposed!` fail no rule that reads during a \
         lag, so the lag cannot reach them; remove them: {idle:?}"
    );
}

/// Whether `LAG_MASKED` excuses `rule` for `store`.
fn masked(store: &str, rule: &str) -> bool {
    LAG_MASKED
        .iter()
        .any(|(name, masked_rule, _)| *name == store && *masked_rule == rule)
}

/// I6: every registered store behind a settling lag is rejected by exactly the
/// rules that reject it without one, and in the same way.
#[test]
fn settling_costs_no_mutant_coverage() {
    let mut measured = BTreeSet::new();

    for report in lagging_reports() {
        let Some(entry) = declared(report.name) else {
            continue; // `mutant_registry_is_exhaustive` owns this failure.
        };
        for (rule, verdict) in &report.outcomes {
            if masked(report.name, rule) {
                assert!(
                    !matches!(verdict, Verdict::Panicked { .. }),
                    "`LAG_MASKED` says the lag hides `{rule}` from `{}`, and the rule \
                     still rejects it; remove the row. Saw: {}",
                    report.name,
                    verdict.describe()
                );
                measured.insert((report.name, *rule));
            } else if entry.kind != Kind::ConformantVariant && entry.fails.contains(rule) {
                assert_declared_failure(entry, rule, verdict);
            } else {
                assert_undeclared_outcome(entry, &report.declines, rule, verdict);
            }
        }
    }

    for (store, rule, outcome, description) in lagging_racer_reports() {
        let Some(row) = RACERS.iter().find(|row| row.name == store) else {
            continue; // `the_concurrency_rules_reject_exactly_what_they_claim` owns this.
        };
        assert_ne!(
            outcome,
            RacerOutcome::FellOver,
            "`{store}` behind a settling lag fell over in `{rule}`: {description}"
        );
        let rejected = outcome == RacerOutcome::Rejected;
        if masked(store, rule) {
            assert!(
                !rejected,
                "`LAG_MASKED` says the lag hides `{rule}` from `{store}`, and the rule \
                 still rejects it; remove the row. Saw: {description}"
            );
            measured.insert((store, rule));
            continue;
        }
        let claimed = row.fails.contains(&rule);
        assert_eq!(
            rejected,
            claimed,
            "`{store}` behind a settling lag against `{rule}`: `RACERS` claims it {} \
             and it {}. Saw: {description}",
            if claimed { "fails" } else { "passes" },
            if rejected { "was rejected" } else { "passed" },
        );
        if let Some((_, needle)) = row
            .expect
            .iter()
            .find(|(pinned, _)| *pinned == rule)
            .filter(|_| rejected)
        {
            assert!(
                description.contains(needle),
                "`{store}` behind a settling lag was rejected by `{rule}` at another \
                 assertion than the pinned {needle:?}. Saw: {description}"
            );
        }
    }

    let unmeasured: Vec<(&str, &str)> = LAG_MASKED
        .iter()
        .map(|(store, rule, _)| (*store, *rule))
        .filter(|pair| !measured.contains(pair))
        .collect();
    assert!(
        unmeasured.is_empty(),
        "`LAG_MASKED` rows naming a store or rule nothing ran: {unmeasured:?}"
    );
}
