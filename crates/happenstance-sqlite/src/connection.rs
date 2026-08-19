//! The one connection configuration both stores in this crate use.
//!
//! An event store and a projection store on one file are **two connections, not
//! one**, and they have to agree: a projection connection with no busy timeout
//! fails instantly against the event store's `BEGIN IMMEDIATE` write lock, and
//! the symptom then appears three stories away from its cause. So the settings
//! live here, in one function, rather than once per role.
//!
//! # A pragma that executed is not a pragma that is in effect
//!
//! SQLite silently accepts a pragma it does not recognise — a misspelling is not
//! an error, it is a no-op — and `journal_mode` in particular can *refuse* the
//! value it was given (an in-memory database has no WAL to switch to) and report
//! the mode it kept instead. [`ConnectionSettings::read_back`] is what turns
//! that from an assumption into an observation: the value asserted on comes off
//! the live connection rather than out of the constants below.

use std::path::Path;

use rusqlite::Connection;

/// The journal mode every connection this crate opens runs under, as SQLite
/// reports it back.
///
/// WAL, because readers do not block the writer — the property a store whose
/// read path is a long replay wants most, and the one that makes two connections
/// onto one file workable at all. The rollback journal (`DELETE`) is the
/// alternative that lost: it serialises readers against the writer
/// (ADR-0022 §11).
///
/// It is spelled lowercase because that is what `PRAGMA journal_mode` answers,
/// and the point of the constant is to be comparable against the answer.
pub const JOURNAL_MODE: &str = "wal";

/// The `synchronous` setting every connection this crate opens runs under, as
/// the integer `PRAGMA synchronous` answers.
///
/// `1` is `NORMAL`. Under WAL that is durable across a process crash and loses
/// at most the tail since the last checkpoint on power loss, which is the trade
/// a local-first store should be making rather than paying an fsync per event.
///
/// **`OFF` is not a performance option here at all**: `spec/SPECIFICATION.md`
/// names `PRAGMA synchronous = OFF` by name as the wrong implementation CF-14's
/// reopen rule exists to reject. `FULL` is the other alternative that lost, on
/// cost (ADR-0022 §11).
pub const SYNCHRONOUS: i64 = 1;

/// How long a connection waits for a contended write lock before giving up, in
/// milliseconds.
///
/// **Finite and generous**, and both halves are load-bearing. With many
/// connections on one file, `BEGIN IMMEDIATE` on a busy database returns
/// `SQLITE_BUSY` *immediately* unless a handler is configured, and that error
/// becomes an adapter failure rather than the contention it actually is. An
/// *unbounded* handler is the opposite mistake: there is no watchdog anywhere in
/// the conformance suite and there must not be one, so an unbounded wait
/// converts a livelock into a hung job that names no rule.
///
/// Five seconds was measured to absorb 64-way contention with zero `SQLITE_BUSY`
/// (ADR-0022 §11), which is what makes that zero mean something rather than
/// being the zero an unbounded handler would also have produced.
pub const BUSY_TIMEOUT_MS: u64 = 5_000;

/// Opens (creating if absent) the database at `path` and configures it.
///
/// This is the single door both [`SqliteEventStore::open`] and
/// [`SqliteProjectionStore::open`] go through, so that neither role can drift
/// from the other's settings.
///
/// [`SqliteEventStore::open`]: crate::event_store::SqliteEventStore::open
/// [`SqliteProjectionStore::open`]: crate::projection_store::SqliteProjectionStore::open
///
/// # Errors
///
/// Returns the driver's error if the file cannot be opened or created — a
/// missing directory, a permission refusal, a corrupt header — or if a pragma
/// statement is rejected.
pub fn open_configured(path: impl AsRef<Path>) -> rusqlite::Result<Connection> {
    let connection = Connection::open(path)?;
    configure(&connection)?;
    Ok(connection)
}

/// Applies this crate's settings to an already-open connection.
///
/// # Errors
///
/// Returns the driver's error if a pragma statement is rejected.
pub fn configure(connection: &Connection) -> rusqlite::Result<()> {
    // The busy timeout goes on **first**, before any statement runs. It is what
    // every later lock acquisition — `BEGIN IMMEDIATE` above all — waits on
    // instead of failing instantly.
    connection.busy_timeout(core::time::Duration::from_millis(BUSY_TIMEOUT_MS))?;
    connection.execute_batch(
        "PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;",
    )?;
    ensure_wal(connection)
}

/// How many times the WAL conversion is attempted before its error is reported.
///
/// Bounded, and that is the whole of the safety argument: there is no watchdog
/// anywhere in the conformance suite and there must not be one, so an unbounded
/// retry would convert a livelock into a hung job that names no rule. Sixty-four
/// is far above what contention needs — the *first* connection to convert makes
/// every other attempt a no-op — and far below anything that could look like a
/// hang.
const WAL_CONVERSION_ATTEMPTS: usize = 64;

/// Puts the database into WAL, tolerating the one `SQLITE_BUSY` a busy handler
/// deliberately will not absorb.
///
/// # Why this is a loop and the busy timeout is not enough
///
/// Converting a *fresh* file to WAL is a write, and reading `journal_mode` to
/// discover it needs converting takes a read lock first. That makes the
/// conversion a **lock promotion**, and SQLite documents that it returns
/// `SQLITE_BUSY` for a promotion *without* invoking the busy handler, on purpose:
/// two connections each holding a read lock and each waiting to promote would
/// deadlock, so it fails one of them "hoping that this will induce the first
/// process to release its read lock and allow the second to proceed". Waiting is
/// exactly the wrong response; releasing and retrying is the documented one.
///
/// Measured here: eight threads opening one fresh file behind a barrier, with
/// the busy timeout installed first, still saw seven of eight fail instantly.
/// This loop is what makes all eight succeed.
///
/// A database with no WAL to switch to — `:memory:`, a temporary file — reports
/// the mode it kept instead of failing, and that is not an error.
fn ensure_wal(connection: &Connection) -> rusqlite::Result<()> {
    for _ in 1..WAL_CONVERSION_ATTEMPTS {
        match attempt_wal(connection) {
            // Yield rather than sleep: no clock is read, nothing is timed, and
            // the connection that *is* converting gets the core.
            Err(err) if is_busy(&err) => std::thread::yield_now(),
            settled => return settled,
        }
    }
    // The last attempt reports its own error rather than a summary of the
    // others, so a genuine failure arrives as itself.
    attempt_wal(connection)
}

/// One conversion attempt: a no-op if the file is already in WAL.
fn attempt_wal(connection: &Connection) -> rusqlite::Result<()> {
    let current: String = connection.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
    if current.eq_ignore_ascii_case(JOURNAL_MODE) {
        return Ok(());
    }
    connection.query_row("PRAGMA journal_mode = WAL", [], |row| {
        row.get::<_, String>(0)
    })?;
    Ok(())
}

/// Whether `error` is the driver saying the database was locked.
fn is_busy(error: &rusqlite::Error) -> bool {
    matches!(
        error,
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.code == rusqlite::ErrorCode::DatabaseBusy
                || failure.code == rusqlite::ErrorCode::DatabaseLocked
    )
}

/// What one connection is **actually** running under, read off that connection.
///
/// Not the constants above: the values SQLite answered when asked. The
/// difference is the whole reason this type exists — see the
/// [module documentation](self).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionSettings {
    journal_mode: String,
    synchronous: i64,
    busy_timeout_ms: i64,
}

impl ConnectionSettings {
    /// Asks `connection` what it is running under.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if one of the three pragmas cannot be read —
    /// which on a live connection means the connection itself has failed.
    pub fn read_back(connection: &Connection) -> rusqlite::Result<Self> {
        let journal_mode: String =
            connection.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
        let synchronous: i64 = connection.query_row("PRAGMA synchronous", [], |row| row.get(0))?;
        let busy_timeout_ms: i64 =
            connection.query_row("PRAGMA busy_timeout", [], |row| row.get(0))?;

        Ok(Self {
            journal_mode: journal_mode.to_lowercase(),
            synchronous,
            busy_timeout_ms,
        })
    }

    /// The journal mode in force, lowercased as SQLite reports it.
    #[must_use]
    pub fn journal_mode(&self) -> &str {
        &self.journal_mode
    }

    /// The `synchronous` level in force, as SQLite's integer code.
    #[must_use]
    pub const fn synchronous(&self) -> i64 {
        self.synchronous
    }

    /// The busy timeout in force, in milliseconds.
    #[must_use]
    pub const fn busy_timeout_ms(&self) -> i64 {
        self.busy_timeout_ms
    }
}
