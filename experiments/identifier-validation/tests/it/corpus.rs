//! The RTL corpus. Criterion 3 of the README, and the conditions that void the
//! run: the negative controls and the corpus's shape.

use identifier_validation::{
    CORPUS, Constructor, NEGATIVE_INSERTS, PAIRS, Verdict, check_corpus, check_pairs,
    shape_problems,
};

#[test]
fn every_corpus_entry_is_accepted_by_both_constructors() {
    for constructor in Constructor::ALL {
        let report = check_corpus(|value| constructor.verdict(value));

        assert_eq!(
            (report.entries_checked, report.refused),
            (CORPUS.len(), Vec::new()),
            "{constructor:?}"
        );
    }
}

#[test]
fn every_key_value_pair_is_accepted() {
    let refused = check_pairs();

    assert_eq!(refused, Vec::new(), "of {} pairs", PAIRS.len());
}

#[test]
fn every_negative_control_is_refused_with_its_variant() {
    for constructor in Constructor::ALL {
        let report = check_corpus(|value| constructor.verdict(value));

        assert_eq!(
            (
                report.controls_checked,
                report.control_escape_count,
                report.control_escapes
            ),
            (CORPUS.len() * NEGATIVE_INSERTS.len() * 3, 0, Vec::new()),
            "{constructor:?}"
        );
    }
}

#[test]
fn corpus_meets_its_preregistered_shape() {
    assert_eq!(shape_problems(), Vec::new());
}

/// The wrong validator VT-14's `Rejects:` line names first: one that bans all of
/// `Cf`. ZWNJ and ZWJ stand in for the category here — they are the two `Cf`
/// characters the clause says a blanket ban would take — and the corpus must
/// notice the Persian it then refuses.
#[test]
fn corpus_catches_a_validator_that_bans_the_joiners() {
    let bans_joiners = |value: &str| {
        if value.contains(['\u{200C}', '\u{200D}']) {
            Verdict::OtherRefusal
        } else {
            Constructor::Tag.verdict(value)
        }
    };

    let report = check_corpus(bans_joiners);

    let refused_without_a_joiner = report
        .refused
        .iter()
        .filter(|refusal| !refusal.text.contains(['\u{200C}', '\u{200D}']))
        .count();
    assert!(
        report.refused.len() >= 3,
        "only {} refused",
        report.refused.len()
    );
    assert_eq!(refused_without_a_joiner, 0);
}

/// A validator that refuses nothing must let every negative control escape, or
/// the controls are not reaching the verdict.
#[test]
fn negative_controls_catch_a_validator_that_refuses_nothing() {
    let report = check_corpus(|_| Verdict::Accepted);

    assert_eq!(report.control_escape_count, report.controls_checked);
}
