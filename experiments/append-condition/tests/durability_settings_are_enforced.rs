//! The positive control: the runner refuses to emit a number under a setting
//! the shipped adapter may not use.
//!
//! `experiments/position-visibility/README.md:47-52` records that its `setup.sh`
//! **aborts** under `fsync=off` rather than produce a figure, because what these
//! mechanisms charge for is the length of an interval held across a durable
//! commit. The SQLite analogue is `PRAGMA synchronous = OFF`, and here it is a
//! *correctness* constraint as well as an honesty one:
//! `spec/SPECIFICATION.md:7481-7484` names that pragma **by name** as a wrong
//! implementation CF-14's reopen rule exists to reject. A figure produced under
//! it is a figure for a store that cannot ship.
//!
//! A control that cannot fire is decorative, so this file forces the refusal
//! rather than describing it.

mod support;

use append_condition_probes::{
    BeginImmediateProbe, Durability, JoinTable, JournalMode, Synchronous,
};
use support::{BUSY_TIMEOUT_MS, CandidateFixture};

/// The control fires: a connection under `synchronous = OFF` is refused, and
/// the refusal names the setting that caused it.
#[test]
fn a_connection_under_synchronous_off_is_refused() {
    let fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new();
    let connection = rusqlite::Connection::open(fixture.path()).expect("opening the probe file");
    connection
        .execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = OFF;")
        .expect("forcing the wrong setting");

    let settings = Durability::read_back(&connection).expect("reading the pragmas back");
    assert_eq!(
        settings.synchronous,
        Synchronous::Off,
        "the control must actually be running under the setting it exists to refuse"
    );

    let refused = settings
        .require_shippable()
        .expect_err("a measurement under `synchronous = OFF` must be refused, not caveated");
    assert_eq!(refused.settings.synchronous, Synchronous::Off);
    assert!(
        refused.to_string().contains("synchronous = off"),
        "the refusal must name the setting a reader has to change: {refused}"
    );
}

/// The control does not fire on the settings every arm actually runs under, so
/// a green measurement is evidence rather than an artefact of a check that
/// never says no.
#[test]
fn the_shipped_settings_are_accepted_and_reported() {
    let fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new();
    let store = fixture.open();
    let settings = store
        .durability()
        .require_shippable()
        .expect("WAL + synchronous = NORMAL is a configuration the adapter may ship");

    assert_eq!(settings.journal_mode, JournalMode::Wal);
    assert_eq!(settings.synchronous, Synchronous::Normal);
    assert_eq!(
        settings.busy_timeout_ms,
        i64::try_from(BUSY_TIMEOUT_MS).expect("the timeout fits an i64"),
        "the busy timeout is one of the three pragma values ADR-0022 owes a \
         number for, and it must be finite: an unbounded handler turns a \
         livelock into a hung run naming no rule (CF-33)"
    );

    let conditions = settings.conditions();
    for expected in [
        "journal_mode=wal",
        "synchronous=normal",
        "busy_timeout_ms=5000",
    ] {
        assert!(
            conditions.contains(expected),
            "every figure is printed beside the settings it was produced on, \
             and `{expected}` is missing from: {conditions}"
        );
    }
}

/// The settings are read **back** off the live connection rather than trusted
/// from the `PRAGMA` that was issued.
///
/// SQLite silently ignores a `journal_mode` it cannot honour, so a runner that
/// believed its own `execute` would report a WAL number for a rollback-journal
/// database. This is the assertion that the two are not the same thing.
#[test]
fn the_journal_mode_is_read_back_not_assumed() {
    let fixture = CandidateFixture::<BeginImmediateProbe, JoinTable>::new();
    let connection = rusqlite::Connection::open(fixture.path()).expect("opening the probe file");
    connection
        .execute_batch("PRAGMA journal_mode = DELETE;")
        .expect("setting the rollback journal");

    let settings = Durability::read_back(&connection).expect("reading the pragmas back");
    assert_eq!(
        settings.journal_mode,
        JournalMode::Delete,
        "the reader must report the mode in force, not the last one requested"
    );
}
