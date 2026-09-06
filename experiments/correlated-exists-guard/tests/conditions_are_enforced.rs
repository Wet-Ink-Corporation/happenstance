//! The control, forced to fire.
//!
//! A control that cannot fire is decorative. `spec/SPECIFICATION.md:7481-7484`
//! names `PRAGMA synchronous = OFF` **by name** as a wrong implementation CF-14's
//! reopen rule exists to reject, so a figure produced under it is a figure for a
//! store that fails conformance — and the runner must abort rather than emit one
//! with a caveat attached, because emitting one with a caveat is how the caveat
//! gets dropped and the number gets cited.
//!
//! Two halves are proved here, and the second is the one that is easy to skip:
//! that the refusal happens, and that the settings are read off the **live
//! connection** rather than out of the `PRAGMA` that was issued.

mod support;

use correlated_exists_guard::Conditions;
use rusqlite::Connection;

#[test]
fn the_settings_every_timed_run_uses_are_the_adapters_own() {
    let path = std::env::temp_dir().join(format!(
        "happenstance-shipped-guard-{}-conditions.sqlite3",
        std::process::id()
    ));
    let connection =
        happenstance_sqlite::connection::open_configured(&path).expect("opening must succeed");
    let conditions = Conditions::require(&connection);
    let line = conditions.line();

    // Named rather than merely printed: WAL and `synchronous = NORMAL` are what
    // ADR-0022 §11 fixes, and a run under anything else is a run about a
    // different store.
    assert!(line.contains("journal_mode=wal"), "{line}");
    assert!(line.contains("synchronous=1"), "{line}");
    println!("CONDITIONS\t{line}");

    drop(connection);
    for suffix in ["", "-wal", "-shm"] {
        let mut candidate = path.clone().into_os_string();
        candidate.push(suffix);
        let _ = std::fs::remove_file(std::path::PathBuf::from(candidate));
    }
}

#[test]
fn a_figure_is_refused_under_the_setting_the_specification_names() {
    let connection = Connection::open_in_memory().expect("an in-memory database");
    connection
        .execute_batch("PRAGMA synchronous = OFF;")
        .expect("the pragma is accepted");

    let refused = Conditions::read_back(&connection)
        .expect("the pragmas read back")
        .expect_err("OFF must be refused, or the control is decorative");
    assert_eq!(refused.synchronous, 0);
    println!("REFUSED\t{refused}");
}

#[test]
fn a_pragma_that_executed_is_not_a_pragma_that_is_in_effect() {
    // An in-memory database has no write-ahead log to switch to, so SQLite
    // *accepts* the statement and keeps the mode it had. A runner that trusted
    // its own `execute` would print `journal_mode=wal` for this connection; one
    // that reads the connection back cannot.
    let connection = Connection::open_in_memory().expect("an in-memory database");
    connection
        .query_row("PRAGMA journal_mode = WAL", [], |row| {
            row.get::<_, String>(0)
        })
        .expect("the statement is accepted");

    let conditions = Conditions::read_back(&connection)
        .expect("the pragmas read back")
        .expect("an in-memory database is not synchronous=OFF");
    let line = conditions.line();
    assert!(
        !line.contains("journal_mode=wal"),
        "an in-memory database cannot be in WAL; if this passes, the read-back is \
         reporting the statement rather than the connection: {line}"
    );
    println!("READBACK\t{line}");
}
