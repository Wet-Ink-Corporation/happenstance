//! The control that decides whether a number is allowed to exist.
//!
//! `experiments/position-visibility/README.md` records that its whole result
//! stands on `fsync=on`, and that its `setup.sh` **aborts** rather than produce
//! a figure under `fsync=off`, because what these mechanisms charge for is the
//! length of an interval held across a durable commit. The SQLite analogue is
//! exact — and here it is a *correctness* constraint as well as an honesty one:
//! `spec/SPECIFICATION.md:7481-7484` names `PRAGMA synchronous = OFF` by name as
//! a wrong implementation CF-14's reopen rule rejects, so a figure produced
//! under it is a figure for a store that fails conformance.
//!
//! So the settings are read back off the live connection rather than assumed
//! from the `PRAGMA` statements that were issued. The two are not the same
//! thing: SQLite silently ignores a `journal_mode` it cannot honour, and a
//! runner that trusted its own `execute` would report a WAL number for a
//! rollback-journal database.

use rusqlite::Connection;

/// A journal mode a shipped adapter may run under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalMode {
    /// Write-ahead logging: readers do not block the writer.
    Wal,
    /// The rollback journal, which serialises readers against the writer.
    Delete,
    /// Anything else the connection reported.
    Other,
}

impl JournalMode {
    /// Parses what `PRAGMA journal_mode` answered.
    fn parse(reported: &str) -> Self {
        match reported.to_ascii_lowercase().as_str() {
            "wal" => Self::Wal,
            "delete" => Self::Delete,
            _ => Self::Other,
        }
    }

    /// How the mode names itself in a results table.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Wal => "wal",
            Self::Delete => "delete",
            Self::Other => "other",
        }
    }
}

/// A `synchronous` setting a shipped adapter may run under.
///
/// `Off` is here so that the control has something to refuse. It is not a
/// configuration this crate offers as a choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Synchronous {
    /// `PRAGMA synchronous = OFF` — the setting the specification names as a
    /// wrong implementation.
    Off,
    /// `NORMAL`: durable across a process crash, and under WAL durable across a
    /// power loss only up to the last checkpoint.
    Normal,
    /// `FULL`: an fsync at every commit.
    Full,
    /// `EXTRA`, or anything else the connection reported.
    Other(i64),
}

impl Synchronous {
    /// Parses what `PRAGMA synchronous` answered.
    const fn parse(reported: i64) -> Self {
        match reported {
            0 => Self::Off,
            1 => Self::Normal,
            2 => Self::Full,
            other => Self::Other(other),
        }
    }

    /// How the setting names itself in a results table.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Normal => "normal",
            Self::Full => "full",
            Self::Other(_) => "other",
        }
    }

    /// Whether a shipped adapter may run under this setting.
    ///
    /// `OFF` is the only refusal, and it is the specification's refusal rather
    /// than this crate's taste.
    #[must_use]
    pub const fn is_shippable(self) -> bool {
        !matches!(self, Self::Off)
    }
}

/// The durability settings a connection is actually running under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Durability {
    /// What `PRAGMA journal_mode` reported.
    pub journal_mode: JournalMode,
    /// What `PRAGMA synchronous` reported.
    pub synchronous: Synchronous,
    /// What `PRAGMA busy_timeout` reported, in milliseconds.
    ///
    /// Reported rather than merely set, because a busy timeout is one of the
    /// three numbers ADR-0022 owes a value for, and an *unbounded* handler is
    /// forbidden: there is no watchdog anywhere in the suite (CF-33), so an
    /// unbounded one converts a livelock into a hung run naming no rule.
    pub busy_timeout_ms: i64,
}

/// Why a measurement was refused before it produced a figure.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error(
    "refusing to measure under `synchronous = {}` with `journal_mode = {}`: \
     spec/SPECIFICATION.md:7481-7484 names `PRAGMA synchronous = OFF` as a wrong \
     implementation CF-14's reopen rule rejects, so a number produced here is a \
     number for a store that cannot ship — and a caveat attached to a table is \
     how a caveat becomes a citation",
    .settings.synchronous.as_str(),
    .settings.journal_mode.as_str()
)]
pub struct DurabilityRefused {
    /// What the connection reported when it was refused.
    pub settings: Durability,
}

impl Durability {
    /// Reads the three settings back off a live connection.
    ///
    /// Read back, never assumed: SQLite silently ignores a `journal_mode` it
    /// cannot honour, so the `PRAGMA` that was issued is not evidence of the
    /// mode that is in force.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if any of the three pragmas cannot be read.
    pub fn read_back(connection: &Connection) -> rusqlite::Result<Self> {
        let journal: String = connection.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
        let synchronous: i64 = connection.query_row("PRAGMA synchronous", [], |row| row.get(0))?;
        let busy_timeout: i64 =
            connection.query_row("PRAGMA busy_timeout", [], |row| row.get(0))?;

        Ok(Self {
            journal_mode: JournalMode::parse(&journal),
            synchronous: Synchronous::parse(synchronous),
            busy_timeout_ms: busy_timeout,
        })
    }

    /// Refuses to proceed under a setting the shipped adapter may not use.
    ///
    /// The abort is the whole of AC-002: no results file is written and no
    /// partial table is emitted, because emitting one with a caveat is how the
    /// caveat gets dropped and the number gets cited.
    ///
    /// # Errors
    ///
    /// Returns [`DurabilityRefused`] when `synchronous` is `OFF`.
    pub const fn require_shippable(self) -> Result<Self, DurabilityRefused> {
        if self.synchronous.is_shippable() {
            Ok(self)
        } else {
            Err(DurabilityRefused { settings: self })
        }
    }

    /// The one line a results table carries beside every figure.
    #[must_use]
    pub fn conditions(&self) -> String {
        format!(
            "journal_mode={} synchronous={} busy_timeout_ms={} sqlite={}",
            self.journal_mode.as_str(),
            self.synchronous.as_str(),
            self.busy_timeout_ms,
            rusqlite::version()
        )
    }
}
