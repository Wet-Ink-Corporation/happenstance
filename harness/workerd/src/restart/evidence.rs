//! What crosses a restart: the before-half's observations, as text.
//!
//! The evidence travels through the test runner, never through the storage
//! under test, so a store cannot corrupt the witness it is judged against. The
//! format is `key=value` lines, written by [`Evidence`]'s `Display` and read by
//! its `FromStr`, and nothing else.

use core::fmt;
use core::str::FromStr;

use happenstance_core::SequencePosition;

use super::{Rule, UnknownRule};
use crate::incarnation::Incarnation;

/// What one rule's before-half saw, stamped with the instance it ran in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    rule: Rule,
    incarnation: Incarnation,
    observed: Observed,
}

/// The rule-specific half of the evidence. Every field is something the store
/// answered; nothing here is a literal the rule chose.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Observed {
    /// Every position `append` returned before the restart, in order.
    pub(crate) acknowledged: Vec<SequencePosition>,
    /// Every event identity read back before the restart, as its `Display`
    /// text. The rules only compare identities for equality, so parsing them
    /// back would buy nothing.
    pub(crate) event_ids: Vec<String>,
    /// The `after` of the append condition the before-half captured.
    pub(crate) captured_after: Option<SequencePosition>,
}

impl Evidence {
    /// `observed`, from `rule`'s before-half, which ran in `incarnation`.
    pub(crate) const fn new(rule: Rule, incarnation: Incarnation, observed: Observed) -> Self {
        Self {
            rule,
            incarnation,
            observed,
        }
    }

    /// The rule whose before-half produced this.
    #[must_use]
    pub const fn rule(&self) -> Rule {
        self.rule
    }

    /// The object incarnation the before-half ran in.
    #[must_use]
    pub const fn incarnation(&self) -> Incarnation {
        self.incarnation
    }

    pub(crate) const fn observed(&self) -> &Observed {
        &self.observed
    }
}

impl fmt::Display for Evidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "rule={}", self.rule.name())?;
        writeln!(f, "incarnation={}", self.incarnation)?;
        for position in &self.observed.acknowledged {
            writeln!(f, "position={position}")?;
        }
        for id in &self.observed.event_ids {
            writeln!(f, "id={id}")?;
        }
        if let Some(after) = self.observed.captured_after {
            writeln!(f, "after={after}")?;
        }
        Ok(())
    }
}

/// Evidence text that `Evidence`'s `Display` could not have written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceError {
    /// A required key is absent.
    MissingField {
        /// The key.
        field: &'static str,
    },
    /// A key that appears at most once appeared twice.
    DuplicateField {
        /// The key.
        field: &'static str,
    },
    /// A line whose key is not one of the five, or that has no `=`.
    UnknownKey {
        /// The line, handed back.
        line: String,
    },
    /// A `position=` or `after=` value that is zero or not a `u64`.
    BadPosition {
        /// The line, handed back.
        line: String,
    },
    /// An `incarnation=` value that is not sixteen hex digits.
    BadIncarnation {
        /// The line, handed back.
        line: String,
    },
    /// A `rule=` value that names no rule.
    UnknownRule(UnknownRule),
}

impl fmt::Display for EvidenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingField { field } => write!(f, "evidence has no {field}="),
            Self::DuplicateField { field } => write!(f, "evidence has {field}= twice"),
            Self::UnknownKey { line } => write!(f, "evidence line has no known key: {line:?}"),
            Self::BadPosition { line } => write!(f, "evidence position is not one: {line:?}"),
            Self::BadIncarnation { line } => {
                write!(f, "evidence incarnation is not one: {line:?}")
            }
            Self::UnknownRule(unknown) => write!(f, "evidence {unknown}"),
        }
    }
}

impl core::error::Error for EvidenceError {}

impl FromStr for Evidence {
    type Err = EvidenceError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut rule = None;
        let mut incarnation = None;
        let mut observed = Observed::default();
        for line in text.lines().filter(|line| !line.is_empty()) {
            let unknown = || EvidenceError::UnknownKey {
                line: line.to_owned(),
            };
            let (key, value) = line.split_once('=').ok_or_else(unknown)?;
            match key {
                "rule" => {
                    let parsed = value.parse().map_err(EvidenceError::UnknownRule)?;
                    set_once(&mut rule, "rule", parsed)?;
                }
                "incarnation" => {
                    let parsed = value.parse().map_err(|_| EvidenceError::BadIncarnation {
                        line: line.to_owned(),
                    })?;
                    set_once(&mut incarnation, "incarnation", parsed)?;
                }
                "position" => observed.acknowledged.push(position(line, value)?),
                "id" => observed.event_ids.push(value.to_owned()),
                "after" => set_once(
                    &mut observed.captured_after,
                    "after",
                    position(line, value)?,
                )?,
                _ => return Err(unknown()),
            }
        }
        Ok(Self {
            rule: rule.ok_or(EvidenceError::MissingField { field: "rule" })?,
            incarnation: incarnation.ok_or(EvidenceError::MissingField {
                field: "incarnation",
            })?,
            observed,
        })
    }
}

fn set_once<T>(slot: &mut Option<T>, field: &'static str, value: T) -> Result<(), EvidenceError> {
    if slot.is_some() {
        return Err(EvidenceError::DuplicateField { field });
    }
    *slot = Some(value);
    Ok(())
}

fn position(line: &str, value: &str) -> Result<SequencePosition, EvidenceError> {
    value
        .parse::<u64>()
        .ok()
        .and_then(SequencePosition::new)
        .ok_or_else(|| EvidenceError::BadPosition {
            line: line.to_owned(),
        })
}

#[cfg(test)]
mod tests {
    use happenstance_core::SequencePosition;

    use super::{Evidence, EvidenceError, Observed};
    use crate::incarnation::Incarnation;
    use crate::restart::{Rule, UnknownRule};

    fn at(raw: u64) -> SequencePosition {
        SequencePosition::new(raw).expect("the examples use non-zero positions")
    }

    const ONE: Incarnation = Incarnation::from_raw(0x00ab_cdef);

    #[test]
    fn evidence_of_every_shape_round_trips_through_its_text() {
        let shapes = [
            Observed::default(),
            Observed {
                acknowledged: vec![at(3)],
                event_ids: vec!["0123:3".to_owned(), "0123:2".to_owned()],
                captured_after: None,
            },
            Observed {
                acknowledged: vec![at(1), at(9)],
                event_ids: Vec::new(),
                captured_after: Some(at(1)),
            },
        ];
        for rule in Rule::ALL {
            for observed in shapes.clone() {
                let evidence = Evidence::new(rule, ONE, observed);
                assert_eq!(evidence.to_string().parse(), Ok(evidence.clone()));
            }
        }
    }

    #[test]
    fn evidence_without_a_rule_is_refused() {
        assert_eq!(
            "incarnation=0000000000abcdef\n".parse::<Evidence>(),
            Err(EvidenceError::MissingField { field: "rule" })
        );
    }

    #[test]
    fn evidence_without_an_incarnation_is_refused() {
        assert_eq!(
            "rule=head_survives_a_restart\n".parse::<Evidence>(),
            Err(EvidenceError::MissingField {
                field: "incarnation"
            })
        );
    }

    #[test]
    fn a_zero_or_non_numeric_position_is_refused() {
        for line in ["position=0", "position=-1", "after=x", "position="] {
            let text = format!("rule=head_survives_a_restart\n{line}\n");
            assert_eq!(
                text.parse::<Evidence>(),
                Err(EvidenceError::BadPosition {
                    line: line.to_owned()
                })
            );
        }
    }

    #[test]
    fn a_single_valued_key_given_twice_is_refused() {
        let text = "incarnation=0000000000abcdef\nincarnation=0000000000abcdef\n";
        assert_eq!(
            text.parse::<Evidence>(),
            Err(EvidenceError::DuplicateField {
                field: "incarnation"
            })
        );
        let text = "after=1\nafter=2\n";
        assert_eq!(
            text.parse::<Evidence>(),
            Err(EvidenceError::DuplicateField { field: "after" })
        );
    }

    #[test]
    fn an_unknown_key_a_bare_line_and_an_unknown_rule_are_refused() {
        assert_eq!(
            "frob=1".parse::<Evidence>(),
            Err(EvidenceError::UnknownKey {
                line: "frob=1".to_owned()
            })
        );
        assert_eq!(
            "rule".parse::<Evidence>(),
            Err(EvidenceError::UnknownKey {
                line: "rule".to_owned()
            })
        );
        assert_eq!(
            "rule=no_such_rule".parse::<Evidence>(),
            Err(EvidenceError::UnknownRule(UnknownRule(
                "no_such_rule".to_owned()
            )))
        );
        assert_eq!(
            "incarnation=7".parse::<Evidence>(),
            Err(EvidenceError::BadIncarnation {
                line: "incarnation=7".to_owned()
            })
        );
    }
}
