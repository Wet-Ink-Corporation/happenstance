//! The judge, on the host, against the outcomes a real restore can produce,
//! and against the wrong stores each verdict must reject.

use super::{InvalidLedger, Ledger, Mode, Sighting, Verdict, judge, restore_observed};
use crate::incarnation::Incarnation;

/// The incarnation the identities in these tests were minted under before
/// the restore, and the one a re-mint moves to.
const OLD: &str = "0000000000000000000000000000000a";
const NEW: &str = "0000000000000000000000000000000b";

fn seen(event_type: &str, store: &str, position: u64) -> Sighting {
    Sighting {
        event_type: event_type.to_owned(),
        id: format!("{store}:{position}"),
    }
}

/// Two events kept, two lost, all four under the old incarnation.
fn ledger() -> Ledger {
    Ledger {
        incarnation: Incarnation::from_raw(7),
        events: vec![
            seen("A1", OLD, 1),
            seen("A2", OLD, 2),
            seen("B1", OLD, 3),
            seen("B2", OLD, 4),
        ],
    }
}

/// The kept events, then the three appended after the restore at the
/// positions the rolled-back counter hands out, under `store`.
fn after_restore(store: &str) -> Vec<Sighting> {
    vec![
        seen("A1", OLD, 1),
        seen("A2", OLD, 2),
        seen("C1", store, 3),
        seen("C2", store, 4),
        seen("C3", store, 5),
    ]
}

#[test]
fn a_remint_after_a_restore_reissues_nothing_and_passes() {
    let verdict = judge(Mode::Remint, &ledger(), &after_restore(NEW));

    assert!(verdict.reissued.is_empty(), "{verdict}");
    assert_eq!(verdict.reused_positions, 2, "{verdict}");
    assert!(verdict.passed(), "{verdict}");
}

/// The wrong store the re-mint cell exists to reject: one whose re-mint does
/// nothing, so the counter's rolled-back positions go out under the old
/// incarnation.
#[test]
fn a_remint_that_changed_nothing_fails_the_remint_cell() {
    let verdict = judge(Mode::Remint, &ledger(), &after_restore(OLD));

    assert_eq!(verdict.reissued.len(), 2, "{verdict}");
    assert!(!verdict.passed(), "{verdict}");
    assert!(
        verdict.to_string().contains(
            "reissued 0000000000000000000000000000000a:3 named B1 before the restore and C1 after"
        ),
        "{verdict}"
    );
}

/// The negative control: with no re-mint, an identity is reissued, and that
/// is the control passing.
#[test]
fn the_control_passes_when_an_identity_is_reissued() {
    assert!(judge(Mode::Control, &ledger(), &after_restore(OLD)).passed());
    assert!(!judge(Mode::Control, &ledger(), &after_restore(NEW)).passed());
}

/// A restore that kept the position counter hands out fresh positions, so
/// nothing is reissued whatever the identity. The re-mint cell must not read
/// that as the re-mint working.
#[test]
fn a_restore_that_kept_the_position_counter_proves_nothing() {
    let after = vec![
        seen("A1", OLD, 1),
        seen("A2", OLD, 2),
        seen("C1", NEW, 5),
        seen("C2", NEW, 6),
        seen("C3", NEW, 7),
    ];
    let verdict = judge(Mode::Remint, &ledger(), &after);

    assert_eq!(verdict.reused_positions, 0, "{verdict}");
    assert!(!verdict.passed(), "{verdict}");
}

/// A re-mint that restamped the events already in the object breaks the
/// promise that they keep the origin they were written with.
#[test]
fn kept_events_under_a_new_identity_fail_the_cell() {
    let mut after = after_restore(NEW);
    for kept in after.iter_mut().take(2) {
        kept.id = kept.id.replace(OLD, NEW);
    }
    let verdict = judge(Mode::Remint, &ledger(), &after);

    assert!(!verdict.kept_intact, "{verdict}");
    assert!(!verdict.passed(), "{verdict}");
}

#[test]
fn an_append_that_did_not_land_fails_either_cell() {
    let mut after = after_restore(NEW);
    after.pop();

    assert!(!judge(Mode::Remint, &ledger(), &after).passed());
}

#[test]
fn the_restore_is_observed_only_as_exactly_the_kept_events() {
    let ledger = ledger();
    let kept = vec![seen("A1", OLD, 1), seen("A2", OLD, 2)];
    assert!(restore_observed(&ledger, &kept));

    assert!(
        !restore_observed(&ledger, &ledger.events),
        "the lost events are still there: no restore happened"
    );
    assert!(
        !restore_observed(&ledger, &kept[..1]),
        "a kept event is missing: restored to the wrong point"
    );
    let restamped = vec![seen("A1", NEW, 1), seen("A2", NEW, 2)];
    assert!(!restore_observed(&ledger, &restamped));

    let empty = Ledger {
        incarnation: ledger.incarnation,
        events: Vec::new(),
    };
    assert!(
        !restore_observed(&empty, &[]),
        "a ledger with nothing kept cannot witness a restore"
    );
}

#[test]
fn a_ledger_round_trips_through_its_text() {
    let ledger = ledger();
    let parsed: Ledger = ledger.to_string().parse().expect("the ledger parses");
    assert_eq!(parsed, ledger);
}

#[test]
fn a_ledger_without_exactly_one_incarnation_is_refused() {
    assert_eq!(
        "event=A1 x:1\n".parse::<Ledger>(),
        Err(InvalidLedger::Incarnation)
    );
    let twice = "incarnation=0000000000000007\nincarnation=0000000000000007\n";
    assert_eq!(twice.parse::<Ledger>(), Err(InvalidLedger::Incarnation));
    assert!(matches!(
        "incarnation=7\n".parse::<Ledger>(),
        Err(InvalidLedger::BadIncarnation(_))
    ));
    assert_eq!(
        "incarnation=0000000000000007\nposition=3\n".parse::<Ledger>(),
        Err(InvalidLedger::Line("position=3".to_owned()))
    );
}

#[test]
fn a_mode_parses_by_name_and_refuses_anything_else() {
    for mode in [Mode::Remint, Mode::Control] {
        assert_eq!(mode.name().parse::<Mode>(), Ok(mode));
    }
    assert_eq!(
        "both".parse::<Mode>(),
        Err(super::UnknownMode("both".to_owned()))
    );
}

/// The first line is what the runner prints; it must say pass or fail.
#[test]
fn a_verdict_says_its_outcome_on_its_first_line() {
    let verdict: Verdict = judge(Mode::Remint, &ledger(), &after_restore(NEW));
    assert_eq!(
        verdict.to_string(),
        "mode=remint verdict=pass reissued=0 reused-positions=2 appended=3 kept=intact"
    );
}
