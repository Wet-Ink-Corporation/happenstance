//! E11-R: every Unicode scalar value, in three positions, through both shipped
//! constructors, against the oracle. Criteria 1 and 2 of the README.

use identifier_validation::{
    Constructor, Position, RTL_BLOCKS, SCALAR_VALUE_COUNT, SweepReport, Verdict, sweep,
};

fn assert_agrees_everywhere(position: Position) {
    for constructor in Constructor::ALL {
        let report = sweep(position, |value| constructor.verdict(value));

        assert_eq!(
            (
                report.checked,
                report.disagreements,
                report.first_disagreements.len()
            ),
            (SCALAR_VALUE_COUNT, 0, 0),
            "{constructor:?} at {position:?}: {:?}",
            report.first_disagreements,
        );
    }
}

#[test]
fn every_scalar_value_alone_agrees_with_the_oracle() {
    assert_agrees_everywhere(Position::Alone);
}

#[test]
fn every_scalar_value_between_ascii_letters_agrees_with_the_oracle() {
    assert_agrees_everywhere(Position::BetweenAscii);
}

#[test]
fn every_scalar_value_between_arabic_letters_agrees_with_the_oracle() {
    assert_agrees_everywhere(Position::BetweenArabic);
}

fn refused_alone(constructor: Constructor) -> SweepReport {
    sweep(Position::Alone, |value| constructor.verdict(value))
}

/// 65 `Cc` and the nine codepoints of the two bidi ranges: 74. VT-14's text,
/// at `4d18e771`, says "the seven" (corrected by ADR-0089), and its ranges
/// hold nine; the ranges are the criterion.
#[test]
fn the_refused_set_alone_is_exactly_the_clauses_ranges() {
    let c0 = ('\u{0}'..='\u{1F}').map(|c| (c, Verdict::ControlCharacter));
    let del = std::iter::once(('\u{7F}', Verdict::ControlCharacter));
    let c1 = ('\u{80}'..='\u{9F}').map(|c| (c, Verdict::ControlCharacter));
    let bidi = ('\u{202A}'..='\u{202E}')
        .chain('\u{2066}'..='\u{2069}')
        .map(|c| (c, Verdict::BidirectionalControl));
    let clause: Vec<(char, Verdict)> = c0.chain(del).chain(c1).chain(bidi).collect();

    for constructor in Constructor::ALL {
        let report = refused_alone(constructor);

        assert_eq!(
            (report.refused_count, report.refused.as_slice()),
            (74, clause.as_slice()),
            "{constructor:?}"
        );
    }
}

#[test]
fn no_refused_codepoint_lies_in_an_rtl_block() {
    for constructor in Constructor::ALL {
        let report = refused_alone(constructor);

        assert_eq!(report.refused_in_rtl_block, 0, "{constructor:?}");
    }
}

/// The RTL-block count is taken over every refusal, not over the capped list:
/// a validator that refuses everything refuses every codepoint of every block,
/// far more than [`MAX_RECORDED`](identifier_validation::MAX_RECORDED).
#[test]
fn rtl_block_count_covers_refusals_past_the_recorded_cap() {
    let every_block_codepoint = ('\0'..=char::MAX)
        .filter(|c| RTL_BLOCKS.iter().any(|block| block.range.contains(c)))
        .count();

    let report = sweep(Position::Alone, |_| Verdict::OtherRefusal);

    assert_eq!(report.refused_in_rtl_block, every_block_codepoint);
}

/// The wrong validator VT-14's `Rejects:` line names second: one that bans only
/// ASCII C0 and DEL. The sweep must see its 32 missed C1 controls and its nine
/// missed bidi controls, and nothing else.
#[test]
fn sweep_catches_a_validator_that_bans_only_ascii_controls() {
    let ascii_only = |value: &str| {
        if value.bytes().any(|b| b < 0x20 || b == 0x7F) {
            Verdict::ControlCharacter
        } else {
            Verdict::Accepted
        }
    };

    let report = sweep(Position::BetweenAscii, ascii_only);

    assert_eq!(report.disagreements, 32 + 9);
}
