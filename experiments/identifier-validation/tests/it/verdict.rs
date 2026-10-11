//! The oracle and the two shipped constructors, on four codepoints whose
//! verdicts are known without running anything.

use identifier_validation::{Constructor, Verdict, expected};

/// NUL (C0), NEL (C1), RLO (a bidi override) and ZWNJ (a `Cf` Persian needs).
const PROBES: [(char, Verdict); 4] = [
    ('\u{0}', Verdict::ControlCharacter),
    ('\u{85}', Verdict::ControlCharacter),
    ('\u{202E}', Verdict::BidirectionalControl),
    ('\u{200C}', Verdict::Accepted),
];

#[test]
fn oracle_classifies_the_four_probes() {
    let verdicts = PROBES.map(|(c, _)| expected(c));

    assert_eq!(verdicts, PROBES.map(|(_, v)| v));
}

#[test]
fn both_constructors_agree_with_the_oracle_on_the_four_probes() {
    for constructor in Constructor::ALL {
        let verdicts = PROBES.map(|(c, _)| constructor.verdict(&format!("a{c}b")));

        assert_eq!(verdicts, PROBES.map(|(_, v)| v), "{constructor:?}");
    }
}

#[test]
fn constructors_report_empty_and_too_long_as_their_own_verdicts() {
    for constructor in Constructor::ALL {
        let verdicts = [
            constructor.verdict(""),
            constructor.verdict(&"ب".repeat(128)),
        ];

        assert_eq!(
            verdicts,
            [Verdict::Empty, Verdict::TooLong],
            "{constructor:?}"
        );
    }
}
