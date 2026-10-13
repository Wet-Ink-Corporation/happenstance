//! The rules on the host, with a restart simulated by handing the after-half a
//! new instance: a fresh [`InstanceLog`] (the instance held the log), or a new
//! wrapper over the same log (storage survived, the instance's state did not).
//! The runner repeats this inside `workerd` with a real restart.

mod stores;

use std::collections::BTreeSet;

use happenstance_core::{EventStore, StoreId};
use happenstance_testkit::__private::block_on;

use super::{
    Check, EvidenceError, Failure, Fault, FreshHighWater, InstanceLog, MintsPerOpen, Rule, Subject,
    Verdict, after, before,
};
use crate::incarnation::Incarnation;
use stores::{RefusesEveryCondition, SkipsFirstRowOnReopen};

const BEFORE: Incarnation = Incarnation::from_raw(1);
const FIRST_MINT: StoreId = StoreId::from_bytes([1; 16]);
const SECOND_MINT: StoreId = StoreId::from_bytes([2; 16]);

/// Runs `rule`'s before-half on `first`, then its after-half on `second`, and
/// answers the check that rejected the store, if one did.
fn verdict<A: EventStore, B: EventStore>(rule: Rule, first: &A, second: &B) -> Option<Check> {
    let evidence = block_on(before(rule, first, BEFORE)).expect("the before-half runs");
    let verdict: Verdict = block_on(after(rule, second, &evidence)).expect("the after-half runs");
    verdict.err().map(|Failure { check, .. }| check)
}

/// Runs `rule` on `instance`, then on `restart(instance)`.
fn across<S: EventStore, T: EventStore>(
    rule: Rule,
    instance: S,
    restart: impl FnOnce(S) -> T,
) -> Option<Check> {
    let evidence = block_on(before(rule, &instance, BEFORE)).expect("the before-half runs");
    let restarted = restart(instance);
    let verdict = block_on(after(rule, &restarted, &evidence)).expect("the after-half runs");
    verdict.err().map(|failure| failure.check)
}

/// What each in-object wrong store fails `rule` on across a simulated restart.
fn simulated(subject: Subject, rule: Rule) -> Option<Check> {
    match subject {
        // The adapter's stand-in: a log that survives, under one instance.
        Subject::Cloudflare => {
            let log = InstanceLog::default();
            verdict(rule, &log, &log)
        }
        Subject::InstanceLog => verdict(rule, &InstanceLog::default(), &InstanceLog::default()),
        Subject::FreshHighWater => across(
            rule,
            FreshHighWater::new(InstanceLog::default()),
            |instance| FreshHighWater::new(instance.into_inner()),
        ),
        Subject::MintsPerOpen => across(
            rule,
            MintsPerOpen::new(InstanceLog::default(), FIRST_MINT),
            |instance| MintsPerOpen::new(instance.into_inner(), SECOND_MINT),
        ),
    }
}

#[test]
fn every_rule_subject_and_check_name_is_distinct_and_parses_back() {
    let rules: BTreeSet<_> = Rule::ALL.iter().map(|rule| rule.name()).collect();
    assert_eq!(rules.len(), Rule::ALL.len());
    for rule in Rule::ALL {
        assert_eq!(rule.name().parse(), Ok(rule));
    }
    let subjects: BTreeSet<_> = Subject::ALL.iter().map(|subject| subject.name()).collect();
    assert_eq!(subjects.len(), Subject::ALL.len());
    for subject in Subject::ALL {
        assert_eq!(subject.name().parse(), Ok(subject));
    }
    let names: BTreeSet<_> = Check::ALL.iter().map(|check| check.name()).collect();
    assert_eq!(names.len(), Check::ALL.len());
}

#[test]
fn the_adapter_fails_nothing_and_every_rule_has_a_wrong_store_that_fails_it() {
    assert_eq!(Subject::Cloudflare.rejects(), &[]);
    for rule in Rule::ALL {
        let rejecting: Vec<&str> = Subject::ALL
            .iter()
            .filter(|subject| subject.rejection(rule).is_some())
            .map(|subject| subject.name())
            .collect();
        assert_ne!(
            rejecting,
            Vec::<&str>::new(),
            "{} is rejected by no wrong store: it would be decorative",
            rule.name()
        );
    }
    for subject in Subject::ALL {
        let rules: BTreeSet<_> = subject
            .rejects()
            .iter()
            .map(|(rule, _)| rule.name())
            .collect();
        assert_eq!(
            rules.len(),
            subject.rejects().len(),
            "{} lists a rule twice",
            subject.name()
        );
    }
}

#[test]
fn every_subject_fails_exactly_its_registry_row_across_a_simulated_restart() {
    for subject in Subject::ALL {
        for rule in Rule::ALL {
            assert_eq!(
                simulated(subject, rule),
                subject.rejection(rule),
                "{} {}",
                subject.name(),
                rule.name()
            );
        }
    }
}

#[test]
fn every_wrong_store_passes_every_rule_without_a_restart() {
    for rule in Rule::ALL {
        let log = InstanceLog::default();
        assert_eq!(
            verdict(rule, &log, &log),
            None,
            "instance-log {}",
            rule.name()
        );
        let cached = FreshHighWater::new(InstanceLog::default());
        assert_eq!(
            verdict(rule, &cached, &cached),
            None,
            "fresh-high-water {}",
            rule.name()
        );
        let minted = MintsPerOpen::new(InstanceLog::default(), FIRST_MINT);
        assert_eq!(
            verdict(rule, &minted, &minted),
            None,
            "mints-per-open {}",
            rule.name()
        );
        let skips = SkipsFirstRowOnReopen::new();
        assert_eq!(
            verdict(rule, &skips, &skips),
            None,
            "skips-first-row {}",
            rule.name()
        );
    }
}

/// The only store that loses an acknowledged event while still assigning fresh
/// positions, so it alone reaches the last check of the positions rule.
#[test]
fn a_store_that_loses_an_old_event_but_issues_fresh_positions_fails_the_tail_check() {
    let tail = across(
        Rule::PositionsAfterARestartAreNeverReissued,
        SkipsFirstRowOnReopen::new(),
        SkipsFirstRowOnReopen::reopen,
    );
    assert_eq!(tail, Some(Check::AcknowledgedWriteLost));
}

#[test]
fn a_store_that_refuses_every_condition_fails_the_anchor() {
    for rule in [
        Rule::AConditionFromBeforeARestartStillBinds,
        Rule::AConditionFromBeforeARestartSeesALaterConflict,
    ] {
        let store = RefusesEveryCondition::default();
        assert_eq!(
            verdict(rule, &store, &store),
            Some(Check::ConditionRefusedWithoutConflict),
            "{}",
            rule.name()
        );
    }
}

/// No check is decorative: each one is the verdict of some wrong store, in an
/// object (the registry) or here on the host.
#[test]
fn every_check_is_reached_by_some_wrong_store() {
    let mut reached: BTreeSet<&str> = Subject::ALL
        .iter()
        .flat_map(|subject| subject.rejects().iter().map(|(_, check)| check.name()))
        .collect();
    for rule in Rule::ALL {
        let skips = across(
            rule,
            SkipsFirstRowOnReopen::new(),
            SkipsFirstRowOnReopen::reopen,
        );
        let refuses = RefusesEveryCondition::default();
        reached.extend(skips.map(Check::name));
        reached.extend(verdict(rule, &refuses, &refuses).map(Check::name));
    }
    let all: BTreeSet<&str> = Check::ALL.iter().map(|check| check.name()).collect();
    assert_eq!(reached, all);
}

#[test]
fn a_rejection_names_its_rule_and_check_on_the_first_line() {
    let failure = Failure {
        rule: Rule::HeadSurvivesARestart,
        check: Check::HeadRegressed,
        detail: "head() answered None".to_owned(),
    };
    assert_eq!(
        failure.to_string(),
        "REJECTED head_survives_a_restart head_regressed\nhead() answered None"
    );
}

#[test]
fn evidence_from_another_rule_is_a_harness_fault() {
    let log = InstanceLog::default();
    let evidence = block_on(before(Rule::HeadSurvivesARestart, &log, BEFORE)).expect("runs");
    assert_eq!(
        block_on(after(
            Rule::AcknowledgedWritesSurviveARestart,
            &log,
            &evidence
        )),
        Err(Fault::ForeignEvidence {
            rule: Rule::AcknowledgedWritesSurviveARestart,
            carried: Rule::HeadSurvivesARestart,
        })
    );
}

#[test]
fn evidence_missing_what_the_after_half_needs_is_a_harness_fault() {
    let log = InstanceLog::default();
    let evidence =
        "rule=a_condition_from_before_a_restart_still_binds\nincarnation=0000000000000001\n"
            .parse()
            .expect("well-formed evidence");
    assert_eq!(
        block_on(after(
            Rule::AConditionFromBeforeARestartStillBinds,
            &log,
            &evidence
        )),
        Err(Fault::Evidence {
            rule: Rule::AConditionFromBeforeARestartStillBinds,
            error: EvidenceError::MissingField { field: "after" },
        })
    );
}
