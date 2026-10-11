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

/// 65 `Cc` and the nine codepoints of the two bidi ranges: 74. VT-14's text
/// says "the seven", and its ranges hold nine; the ranges are the criterion.
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

        let inside: Vec<(char, &str)> = report
            .refused
            .iter()
            .flat_map(|&(c, _)| {
                RTL_BLOCKS
                    .iter()
                    .filter(move |block| block.range.contains(&c))
                    .map(move |block| (c, block.name))
            })
            .collect();

        assert_eq!(inside, Vec::new(), "{constructor:?}");
    }
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
