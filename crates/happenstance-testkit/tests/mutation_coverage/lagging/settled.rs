//! CF-42: the named rules that read back their own writes settle the frontier
//! first, and the ones that do not, do not.
//!
//! Every measurement here runs the suite against
//! [`LaggingFixture`], whose reads hide each acknowledged append until the
//! fixture settles. None of it sleeps or reads a clock (CF-33, TST-10): the lag
//! is a function of the calls.

use std::collections::BTreeSet;

use happenstance_testkit::Fixture;

use super::inventory::{READS_BACK_ITS_OWN_WRITES, READS_NOTHING_IT_WROTE};
use super::never_asked::NeverAsked;
use super::{LaggingFixture, Unsettling};
use crate::harness::{SubjectReport, Verdict, run_concurrency_subject, run_subject};
use crate::racers::LockedFixture;
use crate::variants::{FrontierFixture, GappedPositionFixture, Held};

/// Whether `verdict` is a skip for a capability the inner fixture declines,
/// rather than the read-your-own-writes skip this file is about.
fn skipped_for_the_inner_fixture(report: &SubjectReport, verdict: &Verdict) -> bool {
    match verdict {
        Verdict::Skipped { capability, reason } => {
            *capability != "READ_YOUR_OWN_WRITES"
                && report.declines.contains(&(*capability, *reason))
        }
        Verdict::Passed | Verdict::Panicked { .. } => false,
    }
}

/// Every rule passes, or skips for a capability the wrapped store declines.
///
/// Every offender in one message rather than the first, because a rule that
/// forgot to settle is usually one of several. `why` says what the subject is
/// and why it must pass.
fn assert_passes_everything(report: &SubjectReport, why: &str) {
    let failed: Vec<String> = report
        .outcomes
        .iter()
        .filter(|(_, verdict)| {
            !matches!(verdict, Verdict::Passed) && !skipped_for_the_inner_fixture(report, verdict)
        })
        .map(|(rule, verdict)| format!("`{rule}`: {}", verdict.describe()))
        .collect();
    assert!(
        failed.is_empty(),
        "{} rule(s) failed `{}` {why}:\n{}",
        failed.len(),
        report.name,
        failed.join("\n")
    );
}

/// [`assert_passes_everything`]'s `why` for a conformant store behind a
/// settling lag.
const BEHIND_A_SETTLING_LAG: &str = "behind a settling lag. A conformant store \
     whose fixture settles its frontier must pass every rule, because a rule that \
     reads back its own append settles first (CF-42)";

/// [`assert_passes_everything`]'s `why` for [`NeverAsked`].
const NEVER_ASKED: &str = "behind a `settle_frontier` that panics. A fixture \
     that supports READ_YOUR_OWN_WRITES is never asked to settle, so its run is the \
     run it had before the hook existed (CF-42)";

/// I4, the CF-5 analogue: a conformant store behind a lag whose fixture
/// settles fails nothing.
#[test]
fn a_conformant_store_behind_a_settling_lag_passes_every_rule() {
    let report = run_subject::<LaggingFixture<GappedPositionFixture>>();
    assert!(
        !<LaggingFixture<GappedPositionFixture> as Fixture>::READ_YOUR_OWN_WRITES.is_supported(),
        "the instrument must decline the capability, or nothing here settles"
    );
    assert_passes_everything(&report, BEHIND_A_SETTLING_LAG);
}

/// The same claim for the one rule that arms a held row, which `Gapped`
/// declines and so never reaches.
#[test]
fn a_store_holding_a_row_above_its_frontier_behind_a_settling_lag_passes_every_rule() {
    assert_passes_everything(
        &run_subject::<LaggingFixture<FrontierFixture<Held>>>(),
        BEHIND_A_SETTLING_LAG,
    );
}

/// The same claim for the concurrency family: each rule settles after its
/// contenders join and before it reads what they wrote.
#[test]
fn the_concurrency_family_behind_a_settling_lag_passes_every_rule() {
    assert_passes_everything(
        &run_concurrency_subject::<LaggingFixture<LockedFixture>>(),
        BEHIND_A_SETTLING_LAG,
    );
}

/// The capability's own reason, which every read-back rule must report when
/// the fixture keeps the provided `settle_frontier`.
fn the_lag_reason() -> &'static str {
    <LaggingFixture<GappedPositionFixture, Unsettling> as Fixture>::READ_YOUR_OWN_WRITES
        .reason()
        .unwrap_or_default()
}

/// I1 and I2: a fixture that declines read-your-own-writes and keeps the
/// provided `settle_frontier` skips exactly the read-back rules, citing the
/// capability and the fixture's own reason, and runs every other rule to a
/// pass.
///
/// Three subjects, because no one of them reaches every rule: `Gapped` declines
/// a held row, `FrontierFixture<Held>` supplies one, and `LockedFixture` is a
/// concurrency subject. The union must see every read-back rule skip, or a rule
/// on the list could stop settling unnoticed.
#[test]
fn an_unsettling_lag_skips_exactly_the_read_back_rules() {
    let reports = [
        run_subject::<LaggingFixture<GappedPositionFixture, Unsettling>>(),
        run_subject::<LaggingFixture<FrontierFixture<Held>, Unsettling>>(),
        run_concurrency_subject::<LaggingFixture<LockedFixture, Unsettling>>(),
    ];
    let lag = the_lag_reason();
    let mut wrong = Vec::new();
    let mut skipped = BTreeSet::new();

    for report in &reports {
        for (rule, verdict) in &report.outcomes {
            let reads_back = READS_BACK_ITS_OWN_WRITES.contains(rule);
            let lag_skip = matches!(
                verdict,
                Verdict::Skipped { capability: "READ_YOUR_OWN_WRITES", reason } if *reason == lag
            );
            if lag_skip {
                skipped.insert(*rule);
            }
            let inner_skip = skipped_for_the_inner_fixture(report, verdict);
            let expected = if reads_back {
                lag_skip || inner_skip
            } else {
                matches!(verdict, Verdict::Passed) || inner_skip
            };
            if !expected {
                wrong.push(format!(
                    "`{rule}` against `{}` ({}): {}",
                    report.name,
                    if reads_back {
                        "reads back"
                    } else {
                        "reads nothing it wrote"
                    },
                    verdict.describe()
                ));
            }
        }
    }

    assert!(
        wrong.is_empty(),
        "{} rule(s) broke CF-42's skip contract. A read-back rule must skip under \
         READ_YOUR_OWN_WRITES when the fixture cannot settle, and every other rule \
         must run as before:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
    let never: Vec<&str> = READS_BACK_ITS_OWN_WRITES
        .iter()
        .copied()
        .filter(|rule| !skipped.contains(rule))
        .collect();
    assert!(
        never.is_empty(),
        "these rules are listed as reading back their own writes, and no subject \
         saw them skip, so none of them is shown to settle: {never:?}"
    );
}

/// The two inventory lists are a partition of the named rules: every rule of
/// both families on exactly one list, and nothing on either that is not a rule.
#[test]
fn the_read_back_inventory_partitions_the_named_rules() {
    let named: BTreeSet<&str> = crate::all_rules()
        .into_iter()
        .chain(crate::all_concurrency_rules())
        .collect();
    let reads_back: BTreeSet<&str> = READS_BACK_ITS_OWN_WRITES.iter().copied().collect();
    let reads_nothing: BTreeSet<&str> = READS_NOTHING_IT_WROTE.iter().copied().collect();

    assert_eq!(
        reads_back.len(),
        READS_BACK_ITS_OWN_WRITES.len(),
        "a rule is listed twice as reading back"
    );
    assert_eq!(
        reads_nothing.len(),
        READS_NOTHING_IT_WROTE.len(),
        "a rule is listed twice as reading nothing"
    );
    let both: Vec<&&str> = reads_back.intersection(&reads_nothing).collect();
    assert!(both.is_empty(), "rules on both lists: {both:?}");
    let listed: BTreeSet<&str> = reads_back.union(&reads_nothing).copied().collect();
    let unlisted: Vec<&&str> = named.difference(&listed).collect();
    assert!(
        unlisted.is_empty(),
        "named rules on neither list; say whether each reads back its own writes: \
         {unlisted:?}"
    );
    let unknown: Vec<&&str> = listed.difference(&named).collect();
    assert!(
        unknown.is_empty(),
        "listed names that are not rules: {unknown:?}"
    );
}

/// I3: a fixture that supports read-your-own-writes is never asked to settle,
/// in either family. Its `settle_frontier` panics, and nothing fails.
#[test]
fn a_fixture_that_reads_its_own_writes_is_never_asked_to_settle() {
    assert!(
        <NeverAsked<GappedPositionFixture> as Fixture>::READ_YOUR_OWN_WRITES.is_supported(),
        "the instrument must support the capability, or it proves nothing"
    );
    assert_passes_everything(
        &run_subject::<NeverAsked<GappedPositionFixture>>(),
        NEVER_ASKED,
    );
    assert_passes_everything(
        &run_subject::<NeverAsked<FrontierFixture<Held>>>(),
        NEVER_ASKED,
    );
    assert_passes_everything(
        &run_concurrency_subject::<NeverAsked<LockedFixture>>(),
        NEVER_ASKED,
    );
}
