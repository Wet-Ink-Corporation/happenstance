//! **The measurement.** Every registered event-store rule, against every subject.
//!
//! The question, and it is the only one this file answers:
//!
//! > How many of the four wrong implementations the pre-publication review named
//! > pass the whole conformance suite today?
//!
//! The number printed as `ANSWER:` below is the count of the four that fail
//! nothing. Everything else in the output is what makes that number readable.
//!
//! # Two controls, in two directions
//!
//! The first is the house rule made concrete. A store that fails nothing is only
//! evidence if a store that *should* fail something does — otherwise "fails
//! nothing" is equally consistent with a fixture the harness silently declined,
//! a `connect` that handed back an empty log, or a `const` that turned the rules
//! off. So each of the four is registered twice: once as itself, and once with a
//! second, unrelated defect injected (the testkit's own `InnerJoinTagStore`
//! shape). The injected arm **must fail at least one rule**, and this file
//! asserts it.
//!
//! The second is the baseline. `MemoryEventStore` — the testkit's published
//! reference — and `LogStore`, the correct core the four are one step away from,
//! must fail **nothing** in the same run. This file asserts that too.
//!
//! Both assertions are real: if either fails, `cargo test` goes red and the
//! number is discarded rather than reported.
//!
//! # What this file does not do
//!
//! It does not assert the answer. The count is the finding; a test that pinned it
//! would be a test that has to be edited whenever the suite improves, which is
//! the wrong direction of dependency for an instrument.

mod support;

use happenstance_testkit::fixtures::MemoryFixture;
use support::wrong_fixtures::{
    ArmedReadFaultFixture, CorrectFixture, ForwardPagingBudgetFixture, NoopReopenFixture,
    StagedCommitFixture, SwallowedReadFaultFixture,
};
use support::harness::{Kind, SubjectReport, run_subject};

/// Runs every subject and prints the census.
///
/// One `#[test]`, not ten, because the ten readings are one measurement: a
/// per-subject test that failed would take its siblings' output down with it, and
/// the controls are only meaningful *beside* the mutants they qualify.
#[test]
fn the_suite_against_four_wrong_adapters() {
    let reports = vec![
        // --- controls, direction two: these must fail nothing ---------------
        run_subject::<MemoryFixture>(),
        run_subject::<CorrectFixture>(),
        // --- the four under measurement -------------------------------------
        run_subject::<ForwardPagingBudgetFixture<false>>(),
        run_subject::<NoopReopenFixture<false>>(),
        run_subject::<SwallowedReadFaultFixture<false>>(),
        run_subject::<StagedCommitFixture<false>>(),
        // --- controls, direction one: these must each fail something --------
        run_subject::<ForwardPagingBudgetFixture<true>>(),
        run_subject::<NoopReopenFixture<true>>(),
        run_subject::<SwallowedReadFaultFixture<true>>(),
        run_subject::<StagedCommitFixture<true>>(),
        // --- a diagnostic, asserted in neither direction ---------------------
        run_subject::<ArmedReadFaultFixture>(),
    ];

    let rules = reports[0].outcomes.len();
    println!("\n=== rules in the event-store family: {rules} ===\n");

    for report in &reports {
        print_report(report, rules);
    }

    // -----------------------------------------------------------------
    // Control, direction two: the conformant baseline
    // -----------------------------------------------------------------
    for report in reports.iter().filter(|r| r.kind == Kind::Control) {
        assert!(
            report.failed().is_empty(),
            "CONTROL FAILED: `{}` is registered as a conformant control and must \
             fail nothing, or every \"fails nothing\" below it is a comparison \
             against a broken baseline. It failed {:?}",
            report.name,
            report.failed()
        );
    }

    // -----------------------------------------------------------------
    // Control, direction one: each mutant is otherwise a working store
    // -----------------------------------------------------------------
    for report in reports.iter().filter(|r| r.kind == Kind::InjectedControl) {
        assert!(
            !report.failed().is_empty(),
            "CONTROL FAILED: `{}` carries a second, unrelated defect (untagged \
             events dropped by the read filter) on top of the store under \
             measurement, and must therefore fail at least one rule. It failed \
             none — which means the rules were never driven against this store at \
             all, and its unqualified twin's result says nothing about the suite.",
            report.name
        );
    }

    // -----------------------------------------------------------------
    // The answer
    // -----------------------------------------------------------------
    let mutants: Vec<&SubjectReport> = reports.iter().filter(|r| r.kind == Kind::Mutant).collect();
    let clean: Vec<&str> = mutants
        .iter()
        .filter(|r| r.failed().is_empty())
        .map(|r| r.name)
        .collect();

    println!("=================================================================");
    println!(
        "ANSWER: {} of {} wrong implementations fail NOTHING",
        clean.len(),
        mutants.len()
    );
    println!("=================================================================");
    for report in &mutants {
        let failed = report.failed();
        if failed.is_empty() {
            println!("  PASSES THE WHOLE SUITE  {}", report.name);
        } else {
            println!(
                "  rejected by {:>2} rule(s)  {}  {:?}",
                failed.len(),
                report.name,
                failed
            );
        }
    }
    println!();
}

/// One subject's rows: the failures in full, then the counts.
fn print_report(report: &SubjectReport, rules: usize) {
    println!("-----------------------------------------------------------------");
    println!("SUBJECT {} [{:?}]", report.name, report.kind);
    for (capability, reason) in &report.declines {
        println!("  declines {capability}: {reason}");
    }
    let failed = report.failed();
    let skipped = report.skipped();
    println!(
        "  {} rules: {} passed, {} skipped, {} FAILED",
        rules,
        report.passed(),
        skipped.len(),
        failed.len()
    );
    if !skipped.is_empty() {
        println!("  skipped: {skipped:?}");
    }
    for (name, verdict) in &report.outcomes {
        if verdict.is_failure() {
            println!(
                "  FAIL {name} [{}]: {}",
                if verdict.is_a_rule_rejection() {
                    "the rule rejected the store"
                } else {
                    "the store fell over — NOT a rule rejection"
                },
                verdict.describe()
            );
        }
    }
    println!();
}
