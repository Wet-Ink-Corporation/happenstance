//! The rules on the host, with a restart simulated by handing the after-half a
//! new instance: a fresh [`InstanceLog`] (the instance held the log), or a
//! fresh [`FreshHighWater`] over the same log (storage survived, the cache did
//! not). The runner repeats this inside `workerd` with a real restart.

use std::collections::BTreeSet;

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent,
};
use happenstance_testkit::__private::block_on;

use super::{
    Check, EvidenceError, Failure, Fault, FreshHighWater, InstanceLog, Rule, Subject, Verdict,
    after, before,
};
use crate::incarnation::Incarnation;

const BEFORE: Incarnation = Incarnation::from_raw(1);

/// Runs `rule`'s before-half on `first`, then its after-half on `second`, and
/// answers the check that rejected the store, if one did.
fn verdict<A: EventStore, B: EventStore>(rule: Rule, first: &A, second: &B) -> Option<Check> {
    let evidence = block_on(before(rule, first, BEFORE)).expect("the before-half runs");
    let verdict: Verdict = block_on(after(rule, second, &evidence)).expect("the after-half runs");
    verdict.err().map(|Failure { check, .. }| check)
}

/// What the registry says `subject` fails `rule` on.
fn registered(subject: Subject, rule: Rule) -> Option<Check> {
    subject.rejection(rule)
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
    let checks = [
        Check::AcknowledgedWriteLost,
        Check::HeadRegressed,
        Check::PositionReissued,
        Check::IdentityReissued,
        Check::ConditionNotEnforced,
        Check::ConditionRefusedWithoutConflict,
    ];
    let names: BTreeSet<_> = checks.iter().map(|check| check.name()).collect();
    assert_eq!(names.len(), checks.len());
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
fn every_rule_passes_when_both_halves_run_on_one_instance() {
    for rule in Rule::ALL {
        let log = InstanceLog::default();
        assert_eq!(verdict(rule, &log, &log), None, "{}", rule.name());
    }
}

#[test]
fn the_instance_log_fails_exactly_its_registry_row_across_a_restart() {
    for rule in Rule::ALL {
        let observed = verdict(rule, &InstanceLog::default(), &InstanceLog::default());
        assert_eq!(
            observed,
            registered(Subject::InstanceLog, rule),
            "{}",
            rule.name()
        );
    }
}

#[test]
fn the_fresh_high_water_store_fails_exactly_its_registry_row_across_a_restart() {
    for rule in Rule::ALL {
        let instance = FreshHighWater::new(InstanceLog::default());
        let evidence = block_on(before(rule, &instance, BEFORE)).expect("the before-half runs");
        let restarted = FreshHighWater::new(instance.into_inner());
        let verdict = block_on(after(rule, &restarted, &evidence)).expect("the after-half runs");
        let observed = verdict.err().map(|failure| failure.check);
        assert_eq!(
            observed,
            registered(Subject::FreshHighWater, rule),
            "{}",
            rule.name()
        );
    }
}

#[test]
fn the_fresh_high_water_store_passes_every_rule_without_a_restart() {
    for rule in Rule::ALL {
        let instance = FreshHighWater::new(InstanceLog::default());
        assert_eq!(verdict(rule, &instance, &instance), None, "{}", rule.name());
    }
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

/// WRONG STORE, here only: refuses every conditional append. It passes the
/// enforcement half of both condition rules vacuously, and the anchor is what
/// rejects it.
#[derive(Debug, Default)]
struct RefusesEveryCondition(InstanceLog);

impl EventStore for RefusesEveryCondition {
    type Error = <InstanceLog as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl futures_core::Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if condition.is_some() {
            return Err(AppendError::ConditionViolated(
                happenstance_core::ConditionViolated::unspecified(),
            ));
        }
        self.0.append(events, None).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
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
