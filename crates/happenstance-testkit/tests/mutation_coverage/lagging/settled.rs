//! CF-42: the named rules that read back their own writes settle the frontier
//! first, and the ones that do not, do not.
//!
//! Every measurement here runs the suite against
//! [`LaggingFixture`], whose reads hide each acknowledged append until the
//! fixture settles. None of it sleeps or reads a clock (CF-33, TST-10): the lag
//! is a function of the calls.

use happenstance_testkit::Fixture;

use super::LaggingFixture;
use crate::harness::{SubjectReport, Verdict, run_subject};
use crate::variants::GappedPositionFixture;

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
/// forgot to settle is usually one of several.
fn assert_passes_everything(report: &SubjectReport) {
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
        "{} rule(s) failed `{}` behind a settling lag. A conformant store whose \
         fixture settles its frontier must pass every rule, because a rule that \
         reads back its own append settles first (CF-42):\n{}",
        failed.len(),
        report.name,
        failed.join("\n")
    );
}

/// I4, the CF-5 analogue: a conformant store behind a lag whose fixture
/// settles fails nothing.
#[test]
fn a_conformant_store_behind_a_settling_lag_passes_every_rule() {
    let report = run_subject::<LaggingFixture<GappedPositionFixture>>();
    assert!(
        !<LaggingFixture<GappedPositionFixture> as Fixture>::READ_YOUR_OWN_WRITES.is_supported(),
        "the instrument must decline the capability, or nothing here settles"
    );
    assert_passes_everything(&report);
}
