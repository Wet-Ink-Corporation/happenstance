//! E11-R: every Unicode scalar value, in one position, against the oracle.

use crate::blocks::RTL_BLOCKS;
use crate::verdict::{Verdict, expected};

/// How many Unicode scalar values there are: U+0000–U+10FFFF less the 2,048
/// surrogates, which are not `char`s. Written as arithmetic so the number in
/// ADR-0015 is derived here rather than copied.
pub const SCALAR_VALUE_COUNT: usize = 0x11_0000 - 0x800;

/// The most disagreements and refusals a report keeps by value. The counts are
/// exact regardless; this only bounds what a badly wrong validator can make the
/// report hold (H-13).
pub const MAX_RECORDED: usize = 256;

/// U+0628 ARABIC LETTER BEH: two bytes, strongly right-to-left, and joining on
/// both sides, so a codepoint between two of them sits inside shaped RTL text.
const BEH: char = '\u{0628}';

/// Where the scalar value under test sits in the candidate identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// The value is the whole identifier: its lead byte is at index 0.
    Alone,
    /// Between `a` and `b`: its lead byte follows a one-byte character. E11's
    /// second position.
    BetweenAscii,
    /// Between two U+0628: its lead byte follows a two-byte RTL sequence. New
    /// here, because the walk then enters the value from inside multi-byte text.
    BetweenArabic,
}

impl Position {
    /// All three positions, in the order the README states them.
    pub const ALL: [Self; 3] = [Self::Alone, Self::BetweenAscii, Self::BetweenArabic];

    /// Writes `c` in this position into `buf`, replacing what was there.
    fn place(self, c: char, buf: &mut String) {
        buf.clear();
        match self {
            Self::Alone => buf.push(c),
            Self::BetweenAscii => {
                buf.push('a');
                buf.push(c);
                buf.push('b');
            }
            Self::BetweenArabic => {
                buf.push(BEH);
                buf.push(c);
                buf.push(BEH);
            }
        }
    }
}

/// One scalar value on which the validator and the oracle differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Disagreement {
    /// The scalar value.
    pub value: char,
    /// What VT-14 says.
    pub expected: Verdict,
    /// What the validator said.
    pub actual: Verdict,
}

/// The outcome of one sweep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SweepReport {
    /// The position every value was tested in.
    pub position: Position,
    /// Scalar values tested.
    pub checked: usize,
    /// Scalar values on which the validator and the oracle differ.
    pub disagreements: usize,
    /// The first [`MAX_RECORDED`] of them, in codepoint order.
    pub first_disagreements: Vec<Disagreement>,
    /// Scalar values the validator refused, whether or not the oracle agrees.
    pub refused_count: usize,
    /// The first [`MAX_RECORDED`] of them with the validator's verdict, in
    /// codepoint order.
    pub refused: Vec<(char, Verdict)>,
    /// Refused scalar values lying in one of [`RTL_BLOCKS`], counted over every
    /// refusal rather than over the capped [`refused`](Self::refused) list.
    /// Criterion 2: zero.
    pub refused_in_rtl_block: usize,
}

/// Tests every Unicode scalar value in `position` against `judge`, and compares
/// each verdict with [`expected`].
pub fn sweep(position: Position, mut judge: impl FnMut(&str) -> Verdict) -> SweepReport {
    let mut report = SweepReport {
        position,
        checked: 0,
        disagreements: 0,
        first_disagreements: Vec::new(),
        refused_count: 0,
        refused: Vec::new(),
        refused_in_rtl_block: 0,
    };
    let mut buf = String::with_capacity(8);

    for value in '\0'..=char::MAX {
        position.place(value, &mut buf);
        let actual = judge(&buf);
        let expected = expected(value);
        report.checked += 1;

        if actual != expected {
            report.disagreements += 1;
            if report.first_disagreements.len() < MAX_RECORDED {
                report.first_disagreements.push(Disagreement {
                    value,
                    expected,
                    actual,
                });
            }
        }
        if actual != Verdict::Accepted {
            report.refused_count += 1;
            if RTL_BLOCKS.iter().any(|block| block.range.contains(&value)) {
                report.refused_in_rtl_block += 1;
            }
            if report.refused.len() < MAX_RECORDED {
                report.refused.push((value, actual));
            }
        }
    }

    report
}
