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

use std::{path::Path, time::Duration, time::Instant};

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
/// # Fifteen seconds, and why it is not five
///
/// ADR-0022 §11 set this to five seconds on a measurement that reported
/// `busy = 0` in every row of its 64-contender table, and rested the value on
/// that zero: *"the timeout did real work and never ran out"*. **That premise
/// is falsified.** `experiments/busy-timeout-margin` — built to make §11's own
/// re-open condition fireable, because nothing in the tree had ever reported a
/// busy count — recorded `busy > 0` at the shipped `CONTENDERS = 64` in one
/// launch of seven, with a margin of only 1.31x–1.38x on the plateau and every
/// wait figure a *lower* bound.
///
/// The consequence is not an unsound store. `committed` stays correct in every
/// row, so what fails is **liveness**, in the one place CF-33 guarantees the
/// suite cannot diagnose it: `SQLITE_BUSY` → `AppendError::Store` →
/// `Attempt::Failed` → a red rule telling an adapter author their store is
/// wrong when nothing about it is. That chain was true when this value was
/// chosen; ADR-0077 has since cut its second link, and the section below says
/// what that changed.
///
/// Fifteen was then measured rather than reasoned, on **this** adapter rather
/// than on the experiment's candidate — which is what closes that experiment's
/// own first caveat. Serialised so that all 64 contenders get the whole
/// machine, which is the worst case this host can produce:
///
/// | `BUSY_TIMEOUT_MS` | red launches |
/// | ---: | ---: |
/// | 5,000 | 7 of 8 |
/// | **15,000** | **0 of 16** |
/// | 30,000 | 0 of 8 |
///
/// **It costs nothing in the healthy path**, which is the argument that
/// decided it over staying put: the handler returns the instant the lock is
/// acquired, so the cap bounds only the pathological tail. Measured at the
/// gate's own parallelism, passing runs took 4.91–5.65 s at five seconds and
/// 4.99–5.40 s at fifteen — indistinguishable. What the raise does buy is paid
/// by a genuinely stuck writer, which now takes 15 s rather than 5 s to
/// report; that path is rare and ends in a red rule rather than a hang.
///
/// **Thirty seconds was rejected**: also clean here, but with no measured gain
/// over fifteen on this host, and it triples the pathological wait. The core
/// sweep is what makes fifteen defensible off this host rather than lucky —
/// fewer cores measured *better*, by about 450x from twenty cores to one, so a
/// smaller CI runner sits in the safer regime rather than the riskier one.
///
/// # What this does **not** fix, and what did
///
/// Raising the cap lowered the *rate* at which a momentarily contended store
/// and a wrong one arrived as the same failed `Attempt`; it could not remove
/// the conflation, because CF-33 is `[FROZEN]` and denies a rule the clock that
/// would tell them apart. The channel did: ADR-0077 added
/// `AppendError::Busy`, and a `BEGIN IMMEDIATE` that runs out this timeout now
/// reports through it — see `begin_error` in the event store — so the
/// concurrency rules accept the refusal as legitimate, under structural floors,
/// rather than failing the adapter for it.
///
/// **The value still matters, and for the same reason it was raised.** A busy
/// contender is not a commit: the rules still require at least one commit and a
/// store where every contender is busy has made no progress, and an application
/// retrying `Busy` pays for every one it receives. Fifteen seconds keeps
/// `Busy` for a writer genuinely stuck behind another rather than for one
/// that merely queued behind 63 others.
pub const BUSY_TIMEOUT_MS: u64 = 15_000;

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
    connection.busy_timeout(Duration::from_millis(BUSY_TIMEOUT_MS))?;
    connection.execute_batch(
        "PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;",
    )?;
    ensure_wal(connection)
}

/// How long the WAL conversion keeps retrying a refused promotion: the busy
/// timeout, ADR-0065's fifteen seconds.
///
/// The same budget, because it answers the same question — how long a
/// connection waits for a peer's write before calling it stuck — and a second
/// number for it would be a second thing to re-measure. The handler cannot
/// spend it here (see [`ensure_wal`]), so this loop spends it instead.
const WAL_CONVERSION_BUDGET: Duration = Duration::from_millis(BUSY_TIMEOUT_MS);

/// The pause after the first refused conversion, in milliseconds. Each refusal
/// doubles it, up to [`WAL_BACKOFF_CAP_MS`].
const WAL_FIRST_BACKOFF_MS: u64 = 1;

/// The longest single pause between conversion attempts, in milliseconds.
///
/// Short, because the usual wait is a peer's journal write and header fsync —
/// milliseconds — and a waiter should notice the peer finishing soon after it
/// does. Not zero, because the wait is for the peer's I/O, and a pause that
/// spends no time gives that I/O none.
const WAL_BACKOFF_CAP_MS: u64 = 25;

/// How many times the WAL conversion is attempted at most, whatever the clock
/// says.
///
/// The budget is what ends the loop. This ceiling is the backstop that keeps it
/// finite if the clock does not advance: there is no watchdog anywhere in the
/// conformance suite and there must not be one, so an unbounded retry would
/// turn a livelock into a hung job that names no rule. It is set so that it
/// cannot bind first on a clock that works: the pauses before it, less the
/// ramp, already outlast the budget at the cap alone (the assertion below), so
/// a refusal at the ceiling means the clock stopped, not that the peer was slow.
const WAL_CONVERSION_ATTEMPTS: u64 = 1_024;

/// How many pauses fall below the cap while the backoff doubles up to it: five
/// for 1 ms doubling to 25 ms (1, 2, 4, 8 and 16).
const WAL_BACKOFF_RAMP: u64 = {
    let mut pause = WAL_FIRST_BACKOFF_MS;
    let mut below_cap = 0;
    while pause < WAL_BACKOFF_CAP_MS {
        pause *= 2;
        below_cap += 1;
    }
    below_cap
};

// The ceiling allows `WAL_CONVERSION_ATTEMPTS - 1` pauses. Those past the ramp
// are each a full cap, and they alone outlast the budget, so a working clock
// always ends the loop before the ceiling does.
const _: () = assert!(
    (WAL_CONVERSION_ATTEMPTS - 1 - WAL_BACKOFF_RAMP) * WAL_BACKOFF_CAP_MS > BUSY_TIMEOUT_MS
);

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
/// process to release its read lock and allow the second to proceed". Waiting
/// *while holding the read lock* is exactly the wrong response; releasing and
/// retrying is the documented one. A refused attempt holds nothing once it
/// returns, so the pause between attempts is that release.
///
/// Measured here: eight threads opening one fresh file behind a barrier, with
/// the busy timeout installed first, still saw seven of eight fail instantly.
/// This loop is what makes all eight succeed.
///
/// # Why the bound is elapsed time
///
/// What a refused connection waits for is a peer's **I/O**: creating and
/// writing its rollback journal while it holds the reserved lock, then the
/// header write and fsync. That takes milliseconds, more on some platforms and
/// file systems than others, and nothing this connection does makes it finish
/// sooner. A bound counted in retries that take no time measures this
/// thread's speed, not the peer's progress, so the bound here is measured on a
/// clock, and every pause spends real time.
///
/// So the loop is bounded by elapsed time, within [`WAL_CONVERSION_BUDGET`],
/// with a doubling backoff from [`WAL_FIRST_BACKOFF_MS`] capped at
/// [`WAL_BACKOFF_CAP_MS`], and [`WAL_CONVERSION_ATTEMPTS`] as a ceiling that
/// only a stopped clock reaches. A peer that is genuinely stuck is reported
/// after about the budget, overshooting by at most one capped pause and one
/// attempt. In the healthy path nothing changes: the first attempt on a file
/// already in WAL is a read that succeeds, and nothing pauses.
///
/// A database with no WAL to switch to — `:memory:`, a temporary file — reports
/// the mode it kept instead of failing, and that is not an error.
fn ensure_wal(connection: &Connection) -> rusqlite::Result<()> {
    ensure_wal_within(connection, &mut WallClock::start())
}

/// The time source the conversion loop retries against: injected so that a
/// test can drive it without sleeping.
trait RetryClock {
    /// Time since the loop started.
    fn elapsed(&self) -> Duration;
    /// Waits `pause` before the next attempt.
    fn pause(&mut self, pause: Duration);
}

/// The production clock: a monotonic start instant and a real sleep.
///
/// Sleeping blocks the calling thread, which `open` already does for up to the
/// busy timeout in SQLite's own handler; this adds no new kind of blocking.
struct WallClock {
    start: Instant,
}

impl WallClock {
    fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl RetryClock for WallClock {
    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    fn pause(&mut self, pause: Duration) {
        std::thread::sleep(pause);
    }
}

/// [`ensure_wal`]'s loop against `clock`.
fn ensure_wal_within(connection: &Connection, clock: &mut impl RetryClock) -> rusqlite::Result<()> {
    let cap = Duration::from_millis(WAL_BACKOFF_CAP_MS);
    let mut backoff = Duration::from_millis(WAL_FIRST_BACKOFF_MS);
    for _ in 1..WAL_CONVERSION_ATTEMPTS {
        match attempt_wal(connection) {
            Err(err) if is_busy(&err) && clock.elapsed() < WAL_CONVERSION_BUDGET => {
                clock.pause(backoff);
                backoff = backoff.saturating_mul(2).min(cap);
            }
            // Success, an error that is not contention, or contention past the
            // budget: each is this attempt's own result, reported as itself.
            settled => return settled,
        }
    }
    // The ceiling's attempt reports its own error rather than a summary of the
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
///
/// Shared with the event store, which asks it of `BEGIN IMMEDIATE` and of
/// nothing later: that is where an append learns it did not get the write lock,
/// and the only place this crate can say the batch took no effect.
pub(crate) fn is_busy(error: &rusqlite::Error) -> bool {
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

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;
    use std::path::PathBuf;

    use super::*;

    /// A fresh file of its own, removed with its siblings on drop — after
    /// every connection onto it, because Windows will not unlink an open file.
    struct FreshFile(PathBuf);

    impl FreshFile {
        /// `test` names the file, so two tests running in parallel in this
        /// process never share one.
        fn new(test: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!(
                "happenstance-sqlite-wal-{}-{test}.db",
                std::process::id()
            ));
            let file = Self(path);
            file.remove();
            file
        }

        fn remove(&self) {
            for suffix in ["", "-wal", "-shm", "-journal"] {
                // A new path per sibling: the base name with a suffix appended.
                let mut path = self.0.as_os_str().to_os_string();
                path.push(suffix);
                match std::fs::remove_file(PathBuf::from(path)) {
                    Err(err) if err.kind() != ErrorKind::NotFound => {
                        // Leaking a temp file is not this test's finding, and
                        // panicking here would abort an unwinding test.
                        eprintln!("could not remove a scratch file: {err}");
                    }
                    // Absent is the expected case for most of the four.
                    _ => {}
                }
            }
        }
    }

    impl Drop for FreshFile {
        fn drop(&mut self) {
            self.remove();
        }
    }

    /// A clock the test drives: `pause` advances virtual time instead of
    /// sleeping, and releases `holder`'s write lock once `release_at` has
    /// passed — standing in for a peer whose conversion takes that long.
    struct Scripted {
        now: Duration,
        /// Every pause asked for, in order.
        pauses: Vec<Duration>,
        release_at: Option<Duration>,
        holder: Option<Connection>,
    }

    impl Scripted {
        fn holding(holder: Connection, release_at: Option<Duration>) -> Self {
            Self {
                now: Duration::ZERO,
                pauses: Vec::new(),
                release_at,
                holder: Some(holder),
            }
        }
    }

    impl RetryClock for Scripted {
        fn elapsed(&self) -> Duration {
            self.now
        }

        fn pause(&mut self, pause: Duration) {
            self.pauses.push(pause);
            self.now += pause;
            if self.release_at.is_some_and(|at| self.now >= at)
                && let Some(holder) = self.holder.take()
            {
                holder
                    .execute_batch("ROLLBACK")
                    .expect("the holder's transaction is still open");
            }
        }
    }

    /// A rollback-mode connection holding RESERVED on `file`: the lock a peer
    /// holds while it writes the journal for its own conversion, and the one
    /// the connection under test is refused its promotion on.
    fn reserved(file: &FreshFile) -> Connection {
        let holder = Connection::open(&file.0).expect("a temp file opens");
        holder
            .execute_batch("BEGIN IMMEDIATE")
            .expect("nothing else holds the fresh file");
        holder
    }

    /// The connection under test, with the busy timeout `configure` installs,
    /// so every refusal it sees is one the handler did not absorb.
    fn opener(file: &FreshFile) -> Connection {
        let connection = Connection::open(&file.0).expect("a temp file opens");
        connection
            .busy_timeout(Duration::from_millis(BUSY_TIMEOUT_MS))
            .expect("the busy timeout installs");
        connection
    }

    /// #238: a peer still converting after more retries than the old bound of
    /// 64 allowed — a quarter of a second, far inside the budget — must be
    /// outlasted, not reported. The wrong implementation this rejects is the
    /// one that shipped: 64 attempts separated by `yield_now`, which spend no
    /// time at all and so gave up on Windows, where creating the peer's
    /// journal takes milliseconds.
    #[test]
    fn a_conversion_held_past_the_old_attempt_bound_is_outlasted() {
        let file = FreshFile::new("outlasted");
        let mut clock = Scripted::holding(reserved(&file), Some(Duration::from_millis(250)));
        let connection = opener(&file);

        let outcome = ensure_wal_within(&connection, &mut clock);

        assert!(
            outcome.is_ok(),
            "refused after {} pauses totalling {:?}: {outcome:?}",
            clock.pauses.len(),
            clock.now
        );
        assert!(clock.holder.is_none(), "the lock was held until released");
        let mode = ConnectionSettings::read_back(&connection).expect("pragmas read");
        assert_eq!(mode.journal_mode(), JOURNAL_MODE);
    }

    /// A peer that never lets go is reported once the budget is spent — not
    /// before, and not much after — and the report is the driver's own busy
    /// error from the last attempt. The wrong implementations this rejects
    /// are an unbounded retry (the test would not end) and one that gives up
    /// early or summarises the refusals into an error of its own.
    #[test]
    fn a_lock_never_released_ends_at_the_budget_as_busy() {
        let file = FreshFile::new("never-released");
        let mut clock = Scripted::holding(reserved(&file), None);
        let connection = opener(&file);

        let outcome = ensure_wal_within(&connection, &mut clock);

        let err = outcome.expect_err("a lock never released must be reported");
        assert!(is_busy(&err), "the driver's own busy error: {err:?}");
        assert!(
            clock.now >= WAL_CONVERSION_BUDGET,
            "gave up at {:?}, inside the budget",
            clock.now
        );
        assert!(
            clock.now < WAL_CONVERSION_BUDGET + Duration::from_millis(WAL_BACKOFF_CAP_MS),
            "overshot the budget by more than one pause: {:?}",
            clock.now
        );
        assert!(
            u64::try_from(clock.pauses.len()).expect("a pause count fits u64")
                < WAL_CONVERSION_ATTEMPTS - 1,
            "the clock, not the ceiling, ended it"
        );
    }

    /// The pauses double from one millisecond and then hold at the cap: short
    /// first, so a peer that finishes in a millisecond or two is noticed at
    /// once, and bounded, so a waiter checks again every 25 ms however long it
    /// has waited. The wrong implementations this rejects are a constant pause
    /// (slow to notice a fast peer) and an uncapped doubling (slow to notice a
    /// slow one).
    #[test]
    fn the_backoff_doubles_from_one_millisecond_and_holds_at_the_cap() {
        let file = FreshFile::new("backoff-shape");
        let mut clock = Scripted::holding(reserved(&file), None);
        let connection = opener(&file);

        ensure_wal_within(&connection, &mut clock).expect_err("the lock is never released");

        let millis: Vec<u128> = clock.pauses.iter().map(Duration::as_millis).collect();
        assert_eq!(millis.get(..7), Some(&[1, 2, 4, 8, 16, 25, 25][..]));
        assert!(
            millis.iter().all(|&pause| pause <= 25),
            "no pause exceeds the cap: {millis:?}"
        );
    }

    /// A clock that never advances counts every pause and never moves.
    struct Frozen {
        pauses: u64,
    }

    impl RetryClock for Frozen {
        fn elapsed(&self) -> Duration {
            Duration::ZERO
        }

        fn pause(&mut self, _pause: Duration) {
            self.pauses += 1;
        }
    }

    /// The ceiling is what keeps the loop finite when the clock gives no
    /// answer: with time standing still, it still ends, after exactly the
    /// ceiling's attempts, with the driver's busy error. The wrong
    /// implementation this rejects is a loop bounded by the clock alone.
    #[test]
    fn a_clock_that_never_advances_still_ends_at_the_ceiling() {
        let file = FreshFile::new("frozen-clock");
        let holder = reserved(&file);
        let connection = opener(&file);
        let mut clock = Frozen { pauses: 0 };

        let outcome = ensure_wal_within(&connection, &mut clock);

        let err = outcome.expect_err("a lock never released must be reported");
        assert!(is_busy(&err), "the driver's own busy error: {err:?}");
        assert_eq!(clock.pauses, WAL_CONVERSION_ATTEMPTS - 1);
        drop(holder);
    }
}
