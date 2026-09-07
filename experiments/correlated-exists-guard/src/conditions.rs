//! The line every figure is printed beside, and the control that can refuse to
//! let a figure exist at all.
//!
//! # Read back, never asserted
//!
//! SQLite silently accepts a pragma it does not recognise — a misspelling is a
//! no-op, not an error — and `journal_mode` can *refuse* the value it was given
//! and report the mode it kept instead. So the settings come off the **live
//! connection**, through
//! [`ConnectionSettings::read_back`](happenstance_sqlite::connection::ConnectionSettings::read_back),
//! which is the adapter's own code rather than a second read-back that happens
//! to agree with it today.
//!
//! # The refusal
//!
//! `spec/SPECIFICATION.md:7481-7484` names `PRAGMA synchronous = OFF` **by name**
//! as a wrong implementation CF-14's reopen rule exists to reject. A figure taken
//! under it is not merely optimistic — it is a figure for a store that fails
//! conformance. `experiments/position-visibility/setup.sh` aborts under
//! `fsync=off` for the same reason, and this is the SQLite analogue: no table is
//! written and no partial row is emitted, because emitting one with a caveat
//! attached is how the caveat gets dropped and the number gets cited.
//!
//! `tests/conditions_are_enforced.rs` forces the refusal, because a control that
//! cannot fire is decorative.

use happenstance_sqlite::connection::ConnectionSettings;
use rusqlite::Connection;

/// `PRAGMA synchronous` as the integer SQLite answers, for `OFF`.
const SYNCHRONOUS_OFF: i64 = 0;

/// What a live connection reported about itself.
#[derive(Debug, Clone)]
pub struct Conditions {
    journal_mode: String,
    synchronous: i64,
    busy_timeout_ms: i64,
}

/// The refusal a measurement ends with rather than emitting a number.
#[derive(Debug, Clone, thiserror::Error)]
#[error(
    "refusing to measure: PRAGMA synchronous read back as {synchronous}, and the \
     specification names OFF by name as a wrong implementation — a figure taken \
     under it is a figure for a store that fails conformance"
)]
pub struct Refused {
    /// The value read off the live connection.
    pub synchronous: i64,
}

impl Conditions {
    /// Reads the settings off the live connection and refuses the one that
    /// cannot ship.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if a pragma cannot be read, and
    /// `Ok(Err(`[`Refused`]`))` under `synchronous = OFF`.
    pub fn read_back(connection: &Connection) -> rusqlite::Result<Result<Self, Refused>> {
        let settings = ConnectionSettings::read_back(connection)?;
        let synchronous = settings.synchronous();
        if synchronous == SYNCHRONOUS_OFF {
            return Ok(Err(Refused { synchronous }));
        }
        Ok(Ok(Self {
            journal_mode: settings.journal_mode().to_owned(),
            synchronous,
            busy_timeout_ms: settings.busy_timeout_ms(),
        }))
    }

    /// Reads the settings back, panicking on either failure.
    ///
    /// The spelling every timed target uses, because both failures are "this run
    /// must not produce a number" rather than "this arm is slow".
    ///
    /// # Panics
    ///
    /// Panics if the pragmas cannot be read, or if the connection is running
    /// under a setting the shipped adapter may not use.
    #[must_use]
    pub fn require(connection: &Connection) -> Self {
        match Self::read_back(connection).expect("the pragmas must read back") {
            Ok(conditions) => conditions,
            Err(refused) => panic!("{refused}"),
        }
    }

    /// The one line a results table carries beside every figure.
    #[must_use]
    pub fn line(&self) -> String {
        format!(
            "journal_mode={} synchronous={} busy_timeout_ms={} sqlite={}",
            self.journal_mode,
            self.synchronous,
            self.busy_timeout_ms,
            rusqlite::version(),
        )
    }
}
