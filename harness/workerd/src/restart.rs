//! Event-store rules whose two halves sit on either side of a restart of the
//! Durable Object.
//!
//! A rule in the testkit runs inside one request, and an object cannot evict
//! itself mid-request, so a rule that spans a restart is **phased**: [`before`]
//! writes and returns [`Evidence`], the runner restarts the object (an
//! eviction it drives, or a hibernation through `workerd`'s own inactivity
//! timer), and [`after`] asserts against what the before-half saw. The object
//! stamps the evidence with its [`Incarnation`]
//! and checks it before the after-half runs, so a rule cannot pass across a
//! restart that did not happen.
//!
//! The rules are generic over [`EventStore`], so moving them into the testkit
//! later is a move rather than a rewrite. They never panic: a panic traps the
//! wasm instance, which would discard every other object's instance state and
//! read as a restart to the next cell. A rejection is a [`Failure`], and a
//! harness that could not run the rule is a [`Fault`].
//!
//! Each rule names the wrong store it rejects; [`Subject`] holds those stores
//! and the registry of exactly which (rule, check) pairs each one fails. Every
//! wrong store passes every rule when no restart happens, which is what makes a
//! rejection evidence that the rule ran across one.

mod evidence;
mod map_stream;
mod run;
mod subject;
#[cfg(test)]
mod tests;

use core::fmt;
use core::str::FromStr;

use happenstance_core::EventStore;

pub use evidence::{Evidence, EvidenceError};
use run::Run;
pub use subject::{FreshHighWater, InstanceLog, MintsPerOpen, Subject, UnknownSubject};

use crate::incarnation::Incarnation;

/// One cross-restart rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// Every event acknowledged before the restart is read back after it,
    /// with its position and identity.
    ///
    /// Rejects [`InstanceLog`], which keeps its log in the object instance
    /// (the events are lost), and [`MintsPerOpen`], which stamps reads with an
    /// identity minted per instance (the events come back under new ids).
    AcknowledgedWritesSurviveARestart,
    /// `head()` after the restart, before any write, is the last position
    /// acknowledged before it.
    ///
    /// Rejects [`InstanceLog`], and [`FreshHighWater`], which answers `head()`
    /// from a high-water mark cached in the instance.
    HeadSurvivesARestart,
    /// The first append after the restart is assigned a position above every
    /// position acknowledged before it, and the log still holds every
    /// acknowledged event, in ascending order, beside it.
    ///
    /// Rejects [`InstanceLog`], whose fresh log reissues the first positions.
    PositionsAfterARestartAreNeverReissued,
    /// An append condition captured before the restart, with a conflicting
    /// event already written before it, is refused after it.
    ///
    /// Rejects [`InstanceLog`] and [`FreshHighWater`], whose cache resumes
    /// below the condition's `after` and so skips the conflict check.
    AConditionFromBeforeARestartStillBinds,
    /// An append condition captured before the restart is refused after it
    /// when a conflicting event is written after it.
    ///
    /// Rejects [`InstanceLog`], whose fresh log places the conflict at or
    /// below the captured `after`.
    AConditionFromBeforeARestartSeesALaterConflict,
}

impl Rule {
    /// Every rule, in the order the runner reports them.
    pub const ALL: [Self; 5] = [
        Self::AcknowledgedWritesSurviveARestart,
        Self::HeadSurvivesARestart,
        Self::PositionsAfterARestartAreNeverReissued,
        Self::AConditionFromBeforeARestartStillBinds,
        Self::AConditionFromBeforeARestartSeesALaterConflict,
    ];

    /// The rule's name, as the runner and the routes spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AcknowledgedWritesSurviveARestart => "acknowledged_writes_survive_a_restart",
            Self::HeadSurvivesARestart => "head_survives_a_restart",
            Self::PositionsAfterARestartAreNeverReissued => {
                "positions_after_a_restart_are_never_reissued"
            }
            Self::AConditionFromBeforeARestartStillBinds => {
                "a_condition_from_before_a_restart_still_binds"
            }
            Self::AConditionFromBeforeARestartSeesALaterConflict => {
                "a_condition_from_before_a_restart_sees_a_later_conflict"
            }
        }
    }
}

/// A name that is no rule's, handed back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRule(pub String);

impl fmt::Display for UnknownRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no cross-restart rule is called {:?}", self.0)
    }
}

impl core::error::Error for UnknownRule {}

impl FromStr for Rule {
    type Err = UnknownRule;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|rule| rule.name() == text)
            .ok_or_else(|| UnknownRule(text.to_owned()))
    }
}

/// Which assertion rejected the store.
///
/// Each is a positive claim the runner matches exactly against the registry,
/// so a wrong store that fails a rule for a different reason than the one it
/// was built to exhibit is a red cell, not a pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Check {
    /// An event acknowledged before the restart is missing or changed after it.
    AcknowledgedWriteLost,
    /// `head()` after the restart is not the last acknowledged position.
    HeadRegressed,
    /// An append after the restart was assigned a position at or below one
    /// acknowledged before it.
    PositionReissued,
    /// An event acknowledged before the restart is read back after it under a
    /// different identity.
    IdentityChanged,
    /// A condition that a conflicting event violates was not enforced.
    ConditionNotEnforced,
    /// A condition that nothing violates was refused: the store refuses
    /// conditions wholesale, which would pass the enforcement checks vacuously.
    ConditionRefusedWithoutConflict,
}

impl Check {
    /// Every check.
    pub const ALL: [Self; 6] = [
        Self::AcknowledgedWriteLost,
        Self::HeadRegressed,
        Self::PositionReissued,
        Self::IdentityChanged,
        Self::ConditionNotEnforced,
        Self::ConditionRefusedWithoutConflict,
    ];

    /// The check's name, as the runner and the registry spell it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::AcknowledgedWriteLost => "acknowledged_write_lost",
            Self::HeadRegressed => "head_regressed",
            Self::PositionReissued => "position_reissued",
            Self::IdentityChanged => "identity_changed",
            Self::ConditionNotEnforced => "condition_not_enforced",
            Self::ConditionRefusedWithoutConflict => "condition_refused_without_conflict",
        }
    }
}

/// A rule rejected the store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    /// The rule.
    pub rule: Rule,
    /// The assertion that rejected the store.
    pub check: Check,
    /// What the store answered, for the log.
    pub detail: String,
}

impl fmt::Display for Failure {
    /// `REJECTED <rule> <check>`, then the detail on the next line: the
    /// runner matches the first line.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "REJECTED {} {}\n{}",
            self.rule.name(),
            self.check.name(),
            self.detail
        )
    }
}

impl core::error::Error for Failure {}

/// The harness, not the store under test, could not run the rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fault {
    /// A step the rule needed failed for a reason that is not the rule's
    /// verdict: a store call returned an error the rule did not expect, or an
    /// event could not be built. The message is the error's `Display`; it is
    /// only ever printed to the runner.
    Step {
        /// The rule.
        rule: Rule,
        /// Which step.
        during: &'static str,
        /// The error, as text.
        message: String,
    },
    /// The evidence lacks something this rule's after-half needs.
    Evidence {
        /// The rule.
        rule: Rule,
        /// What is missing or malformed.
        error: EvidenceError,
    },
    /// The evidence came from a different rule's before-half.
    ForeignEvidence {
        /// The rule asked to run.
        rule: Rule,
        /// The rule the evidence is from.
        carried: Rule,
    },
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Step {
                rule,
                during,
                message,
            } => write!(f, "HARNESS {}: {during}: {message}", rule.name()),
            Self::Evidence { rule, error } => write!(f, "HARNESS {}: {error}", rule.name()),
            Self::ForeignEvidence { rule, carried } => write!(
                f,
                "HARNESS {}: the evidence is from {}",
                rule.name(),
                carried.name()
            ),
        }
    }
}

impl core::error::Error for Fault {}

/// A rule's verdict on the store: `Err` is a rejection.
pub type Verdict = Result<(), Failure>;

/// Runs `rule`'s before-half against `store` in the instance `incarnation`.
///
/// # Errors
///
/// A [`Fault`] when a store call fails or an event cannot be built.
pub async fn before<S: EventStore>(
    rule: Rule,
    store: &S,
    incarnation: Incarnation,
) -> Result<Evidence, Fault> {
    let run = Run::new(rule, store);
    let observed = match rule {
        Rule::AcknowledgedWritesSurviveARestart => run.batch_then_read().await?,
        Rule::HeadSurvivesARestart | Rule::PositionsAfterARestartAreNeverReissued => {
            run.two_appends().await?
        }
        Rule::AConditionFromBeforeARestartStillBinds => {
            run.capture_condition_then_conflict().await?
        }
        Rule::AConditionFromBeforeARestartSeesALaterConflict => run.capture_condition().await?,
    };
    Ok(Evidence::new(rule, incarnation, observed))
}

/// Runs `rule`'s after-half against `store`, judged against `evidence`.
///
/// The caller has already checked that the restart `evidence` was carried
/// across did (or, for the control, did not) happen.
///
/// # Errors
///
/// A [`Fault`] when the evidence is another rule's or lacks what this rule
/// needs, or when a store call fails for a reason that is not the verdict.
pub async fn after<S: EventStore>(
    rule: Rule,
    store: &S,
    evidence: &Evidence,
) -> Result<Verdict, Fault> {
    if evidence.rule() != rule {
        return Err(Fault::ForeignEvidence {
            rule,
            carried: evidence.rule(),
        });
    }
    let run = Run::new(rule, store);
    let observed = evidence.observed();
    match rule {
        Rule::AcknowledgedWritesSurviveARestart => run.batch_read_back(observed).await,
        Rule::HeadSurvivesARestart => run.head_is_last_acknowledged(observed).await,
        Rule::PositionsAfterARestartAreNeverReissued => run.next_is_fresh(observed).await,
        Rule::AConditionFromBeforeARestartStillBinds => run.condition_binds(observed).await,
        Rule::AConditionFromBeforeARestartSeesALaterConflict => {
            run.conflict_then_condition_binds(observed).await
        }
    }
}
