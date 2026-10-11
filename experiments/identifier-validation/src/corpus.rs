//! The RTL corpus check: criterion 3 of the README, the negative controls that
//! void a blind run, and the shape the corpus was pre-registered to have.

mod data;

pub use data::{CORPUS, PAIRS};

use crate::sweep::MAX_RECORDED;
use crate::verdict::{Verdict, key_value_verdict};

/// The pre-registered minimum entries per script.
pub const MIN_ENTRIES_PER_SCRIPT: usize = 10;

/// The pre-registered minimum mixed-LTR entries per script.
pub const MIN_MIXED_PER_SCRIPT: usize = 3;

/// The three scripts the ledger names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    /// Arabic.
    Arabic,
    /// Hebrew.
    Hebrew,
    /// Persian, written in the Arabic script.
    Persian,
}

impl Script {
    /// All three, in the ledger's order.
    pub const ALL: [Self; 3] = [Self::Arabic, Self::Hebrew, Self::Persian];

    /// Whether `c` is in one of this script's Unicode blocks.
    pub fn contains(self, c: char) -> bool {
        match self {
            Self::Arabic | Self::Persian => matches!(
                c,
                '\u{0600}'..='\u{06FF}'
                    | '\u{0750}'..='\u{077F}'
                    | '\u{0870}'..='\u{08FF}'
                    | '\u{FB50}'..='\u{FDFF}'
                    | '\u{FE70}'..='\u{FEFF}'
            ),
            Self::Hebrew => matches!(c, '\u{0590}'..='\u{05FF}' | '\u{FB1D}'..='\u{FB4F}'),
        }
    }
}

/// One corpus identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// The script it is written in.
    pub script: Script,
    /// The identifier, exactly as tested.
    pub text: &'static str,
    /// What in it the corpus means to exercise.
    pub exercises: &'static str,
}

/// One `key:value` pair for `Tag::key_value`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair {
    /// The script the RTL half is written in.
    pub script: Script,
    /// The key.
    pub key: &'static str,
    /// The value.
    pub value: &'static str,
}

/// A corpus entry the validator did not accept as written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refusal {
    /// The entry.
    pub text: &'static str,
    /// Its script.
    pub script: Script,
    /// What the validator said.
    pub verdict: Verdict,
}

/// A pair `Tag::key_value` did not accept as written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairRefusal {
    /// The pair.
    pub pair: Pair,
    /// What `Tag::key_value` said.
    pub verdict: Verdict,
}

/// Where a negative control's offending character was inserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Insertion {
    /// Before the first character.
    Start,
    /// Before the character at half the entry's character count.
    Middle,
    /// After the last character.
    End,
}

impl Insertion {
    /// All three.
    pub const ALL: [Self; 3] = [Self::Start, Self::Middle, Self::End];

    /// `text` with `c` inserted here, always on a character boundary.
    ///
    /// Walking `chars` rather than slicing at a byte offset is what keeps the
    /// insertion on a boundary without a fallible split.
    fn apply(self, text: &str, c: char) -> String {
        let at = match self {
            Self::Start => 0,
            Self::Middle => text.chars().count() / 2,
            Self::End => text.chars().count(),
        };
        let mut out = String::with_capacity(text.len() + c.len_utf8());
        for (i, existing) in text.chars().enumerate() {
            if i == at {
                out.push(c);
            }
            out.push(existing);
        }
        if at == text.chars().count() {
            out.push(c);
        }
        out
    }
}

/// The characters inserted as negative controls, and the verdict each must draw:
/// every codepoint of VT-14's two bidirectional ranges — nine, though the
/// clause calls them seven — and U+0085, a C1 control.
pub const NEGATIVE_INSERTS: [(char, Verdict); 10] = [
    ('\u{202A}', Verdict::BidirectionalControl),
    ('\u{202B}', Verdict::BidirectionalControl),
    ('\u{202C}', Verdict::BidirectionalControl),
    ('\u{202D}', Verdict::BidirectionalControl),
    ('\u{202E}', Verdict::BidirectionalControl),
    ('\u{2066}', Verdict::BidirectionalControl),
    ('\u{2067}', Verdict::BidirectionalControl),
    ('\u{2068}', Verdict::BidirectionalControl),
    ('\u{2069}', Verdict::BidirectionalControl),
    ('\u{0085}', Verdict::ControlCharacter),
];

/// A negative control the validator did not refuse as it must.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Escape {
    /// The corpus entry it was built from.
    pub entry: &'static str,
    /// The character inserted.
    pub inserted: char,
    /// Where.
    pub at: Insertion,
    /// What the validator said.
    pub actual: Verdict,
}

/// The outcome of one corpus check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusReport {
    /// Corpus entries tested.
    pub entries_checked: usize,
    /// Entries not accepted as written. Criterion 3: empty.
    pub refused: Vec<Refusal>,
    /// Negative controls tested.
    pub controls_checked: usize,
    /// Negative controls that drew the wrong verdict. Must be zero, or the run
    /// is void.
    pub control_escape_count: usize,
    /// The first [`MAX_RECORDED`] of them.
    pub control_escapes: Vec<Escape>,
}

/// Runs every corpus entry, and every negative control built from it, past
/// `judge`.
pub fn check_corpus(judge: impl Fn(&str) -> Verdict) -> CorpusReport {
    let mut report = CorpusReport {
        entries_checked: 0,
        refused: Vec::new(),
        controls_checked: 0,
        control_escape_count: 0,
        control_escapes: Vec::new(),
    };

    for entry in CORPUS {
        report.entries_checked += 1;
        let verdict = judge(entry.text);
        if verdict != Verdict::Accepted {
            report.refused.push(Refusal {
                text: entry.text,
                script: entry.script,
                verdict,
            });
        }
        check_controls(entry.text, &judge, &mut report);
    }

    report
}

fn check_controls(entry: &'static str, judge: impl Fn(&str) -> Verdict, report: &mut CorpusReport) {
    for (inserted, expected) in NEGATIVE_INSERTS {
        for at in Insertion::ALL {
            report.controls_checked += 1;
            let actual = judge(&at.apply(entry, inserted));
            if actual == expected {
                continue;
            }
            report.control_escape_count += 1;
            if report.control_escapes.len() < MAX_RECORDED {
                report.control_escapes.push(Escape {
                    entry,
                    inserted,
                    at,
                    actual,
                });
            }
        }
    }
}

/// Runs every pair through `Tag::key_value` and returns those not accepted as
/// written.
pub fn check_pairs() -> Vec<PairRefusal> {
    PAIRS
        .iter()
        .map(|&pair| PairRefusal {
            pair,
            verdict: key_value_verdict(pair.key, pair.value),
        })
        .filter(|refusal| refusal.verdict != Verdict::Accepted)
        .collect()
}

/// A way the corpus falls short of the shape the README pre-registered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeProblem {
    /// Fewer than [`MIN_ENTRIES_PER_SCRIPT`] entries.
    TooFewEntries {
        /// The script.
        script: Script,
        /// How many it has.
        count: usize,
    },
    /// Fewer than [`MIN_MIXED_PER_SCRIPT`] entries carrying an ASCII letter or
    /// digit.
    TooFewMixed {
        /// The script.
        script: Script,
        /// How many it has.
        count: usize,
    },
    /// An entry with no character from its own script's blocks.
    NoScriptCharacter {
        /// The entry.
        text: &'static str,
    },
}

/// Checks the corpus against its pre-registered shape. Empty means it holds.
pub fn shape_problems() -> Vec<ShapeProblem> {
    let mut problems: Vec<ShapeProblem> = CORPUS
        .iter()
        .filter(|entry| !entry.text.chars().any(|c| entry.script.contains(c)))
        .map(|entry| ShapeProblem::NoScriptCharacter { text: entry.text })
        .collect();

    for script in Script::ALL {
        let entries = || CORPUS.iter().filter(move |entry| entry.script == script);
        let count = entries().count();
        if count < MIN_ENTRIES_PER_SCRIPT {
            problems.push(ShapeProblem::TooFewEntries { script, count });
        }
        let mixed = entries()
            .filter(|entry| entry.text.chars().any(|c| c.is_ascii_alphanumeric()))
            .count();
        if mixed < MIN_MIXED_PER_SCRIPT {
            problems.push(ShapeProblem::TooFewMixed {
                script,
                count: mixed,
            });
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::Insertion;

    #[test]
    fn insertions_land_on_character_boundaries_of_rtl_text() {
        let placed = Insertion::ALL.map(|at| at.apply("אבגד", '|'));

        assert_eq!(placed, ["|אבגד", "אב|גד", "אבגד|"].map(String::from));
    }
}
