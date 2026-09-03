//! The one schema, the one read path, and the one identity story every arm
//! shares.
//!
//! Everything that is *not* the axis under test lives here, so that a
//! difference between two figures can only be the strategy or the tag storage.
//! `crates/happenstance-testkit/src/fixtures.rs`'s `MemoryFixture` and
//! `happenstance-core`'s `MemoryEventStore` are the reference shapes this was
//! written against.
//!
//! # Migration 1, as ADR-0022 specifies it
//!
//! * `event(position INTEGER PRIMARY KEY AUTOINCREMENT, …)`. `AUTOINCREMENT` is
//!   load-bearing rather than stylistic: positions must never be reused after a
//!   delete, and plain `rowid` does not guarantee that. It also *permits gaps*,
//!   which is why nothing here or in any rule may assume `+1`.
//! * `metadata BLOB` **nullable**, because `None` and `Some(<empty>)` are two
//!   values the contract keeps apart and a store that folds them has lost one.
//! * The `EventId` origin pair — `origin_store` and `origin_position`, `UNIQUE`
//!   **together**. It is both the index `contains_event_id` seeks and the
//!   constraint that stops ingest storing one event twice.
//! * `recorded_at`, read back on reopen rather than re-stamped. A store whose
//!   reopen restamps hands every auditor the time of the last restart.
//! * `tag_cardinality`, because multi-tag items are probed most-selective-tag-
//!   first and SQLite cannot supply per-value cardinality — `ANALYZE` stores
//!   only an average. It is maintained on **every** arm, so the write cost it
//!   adds is common and cannot flatter one of them.
//! * The store's own `StoreId`, minted once at schema creation and persisted.
//!   Mint-per-open is permitted by VT-6 in general and is the wrong choice for
//!   a file-backed store.
//!
//! Migration is idempotent and safe under a concurrent open: `IF NOT EXISTS`
//! inside `BEGIN IMMEDIATE`, and `INSERT OR IGNORE` then read-back for the
//! `StoreId` row. A fixture connects more than once onto one file, so this is a
//! correctness requirement rather than tidiness.
//!
//! # What is deliberately simpler than an adapter
//!
//! `read` runs its SQL on the calling thread and streams from the snapshot,
//! exactly as `MemoryEventStore` does. The real adapter defers a
//! `spawn_blocking` into the first `poll_next`, and that machinery is
//! `read-path-and-query-translation`'s to build — reproducing it here would be
//! this crate implementing the thing the record exists to decide before.

use std::sync::Mutex;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId, Tag, Tags,
};
use rusqlite::Connection;
use rusqlite::types::Value;

use crate::durability::Durability;
use crate::strategy::{AppendFailure, AppendStrategy};
use crate::tags::{TagStorage, encoded_columns, match_sql};

/// How a candidate store fails for its own reasons.
///
/// `ConditionViolated` is deliberately absent: it is the DCB retry signal and
/// reaches the caller through [`AppendError::ConditionViolated`], never through
/// an adapter's own error type.
#[derive(Debug, thiserror::Error)]
pub enum SqliteProbeError {
    /// The driver failed.
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// A row this store wrote could not be read back as the contract's types.
    ///
    /// Unreachable if the write path is correct, and reported rather than
    /// panicked so that a schema mistake surfaces as a failing rule naming this
    /// store rather than as a crash naming the suite.
    #[error("a stored row did not round-trip: {0}")]
    Corrupt(String),
}

/// One handle onto one candidate store: one `rusqlite::Connection` on one file.
///
/// No pool, deliberately. `crates/happenstance-sqlite/src/lib.rs:47-53` settles
/// the driver as `rusqlite` *without* one, because a single `Mutex`-guarded
/// connection is the serialising instrument the portfolio needs, and an
/// experiment that quietly added a pool would be measuring a different adapter
/// from the one the record is written for.
#[derive(Debug)]
pub struct CandidateStore<A: AppendStrategy, T: TagStorage> {
    connection: Mutex<Connection>,
    store_id: StoreId,
    durability: Durability,
    strategy: core::marker::PhantomData<fn() -> (A, T)>,
}

impl<A: AppendStrategy, T: TagStorage> CandidateStore<A, T> {
    /// Opens a handle onto the database at `path`, migrating it if needed.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the file cannot be opened or migrated.
    pub fn open(path: &std::path::Path, busy_timeout_ms: u64) -> rusqlite::Result<Self> {
        let connection = Connection::open(path)?;
        configure(&connection, busy_timeout_ms)?;
        migrate::<T>(&connection)?;
        let store_id = read_store_id(&connection)?;
        let durability = Durability::read_back(&connection)?;

        Ok(Self {
            connection: Mutex::new(connection),
            store_id,
            durability,
            strategy: core::marker::PhantomData,
        })
    }

    /// The settings this handle is actually running under.
    #[must_use]
    pub const fn durability(&self) -> Durability {
        self.durability
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// Locks the connection, recovering from poisoning.
    ///
    /// A panic elsewhere while holding the lock cannot have left the *database*
    /// inconsistent — every mutation is inside a transaction that either
    /// committed or rolled back — so poisoning carries no information here.
    fn locked(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Runs the read query and returns the matching events.
    fn select(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> Result<Vec<SequencedEvent>, SqliteProbeError> {
        let mut params: Vec<Value> = Vec::new();
        let matched = match_sql::<T>(query, &mut params);

        let mut sql = format!(
            "SELECT position, event_type, data, metadata, tags_text, \
             origin_store, origin_position, recorded_at \
             FROM event WHERE position IN ({matched})"
        );

        // `from` is the starting bound and `to` the stopping one, so reading
        // backwards swaps which side of the position order each sits on. Both
        // are inclusive in both directions — the same rule `MemoryEventStore`
        // states, pushed into SQL.
        if let Some(from) = options.from {
            let comparison = if options.backwards { "<=" } else { ">=" };
            sql.push_str(&format!(" AND position {comparison} ?"));
            params.push(Value::Integer(as_i64(from)));
        }
        if let Some(to) = options.to {
            let comparison = if options.backwards { ">=" } else { "<=" };
            sql.push_str(&format!(" AND position {comparison} ?"));
            params.push(Value::Integer(as_i64(to)));
        }

        sql.push_str(if options.backwards {
            " ORDER BY position DESC"
        } else {
            " ORDER BY position ASC"
        });

        // After filtering and ordering, never before: `limit` truncates the
        // result set the caller would otherwise have seen, and a limit of zero
        // truncates to nothing.
        if let Some(limit) = options.limit {
            sql.push_str(" LIMIT ?");
            params.push(Value::Integer(i64::try_from(limit).unwrap_or(i64::MAX)));
        }

        let connection = self.locked();
        let mut statement = connection.prepare(&sql)?;
        let mut rows = statement.query(rusqlite::params_from_iter(params.iter()))?;

        let mut out = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row_to_event(row)?);
        }
        Ok(out)
    }
}

/// A `SequencePosition` as the integer SQLite stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// The connection settings every arm runs under.
///
/// WAL and `synchronous = NORMAL` are the two values ADR-0022 fixes, and the
/// busy timeout is the third. It is **finite and generous**: an unbounded busy
/// handler converts a livelock into a hung run naming no rule, and there is no
/// watchdog anywhere in the suite to notice (CF-33).
///
/// # The one line this crate changes
///
/// This function is the **whole** of the difference between this file and
/// `experiments/append-condition/src/candidate.rs`, which it is otherwise a
/// verbatim copy of. There, the busy handler is
/// `Connection::busy_timeout(5000)` — SQLite's own `sqliteDefaultBusyCallback`,
/// which waits and reports nothing. Here it is
/// [`crate::busy::counting_busy_handler`], which runs the identical back-off
/// schedule under the identical cap and additionally records how long each
/// contender spent inside it.
///
/// `busy_timeout_ms` is therefore *asserted* rather than applied: this crate
/// exists to ask whether 5,000 ms is enough, so it may not quietly measure under
/// some other number, and a caller passing a different one is a bug rather than
/// a configuration.
///
/// # Panics
///
/// Panics if `busy_timeout_ms` is not [`crate::busy::BUSY_TIMEOUT_MS`].
fn configure(connection: &Connection, busy_timeout_ms: u64) -> rusqlite::Result<()> {
    assert_eq!(
        busy_timeout_ms,
        crate::busy::BUSY_TIMEOUT_MS,
        "the counting handler enforces its own cap, so a caller asking for a \
         different timeout would get a figure for a timeout it did not ask for",
    );
    connection.execute_batch(
        "PRAGMA journal_mode = WAL;
         PRAGMA synchronous = NORMAL;
         PRAGMA foreign_keys = ON;",
    )?;
    match crate::busy::handler() {
        // `busy_handler` and `busy_timeout` are the same slot: installing this
        // one clears SQLite's default *and* zeroes `db->busyTimeout`, so
        // `PRAGMA busy_timeout` reads back 0 from here on. That reading is true
        // and misleading, so `Durability::conditions()` is never printed alone
        // in this crate — every row carries `busy_handler=<kind> cap_ms=5000`
        // beside it.
        crate::busy::Handler::Counting => {
            connection.busy_handler(Some(crate::busy::counting_busy_handler))?;
        }
        // The control for the instrument: SQLite's own handler, which reports
        // nothing and is exactly what `experiments/append-condition` measured
        // under. `PRAGMA busy_timeout` reads back 5000 on this arm, which is how
        // a reader tells the two rows apart even without the label.
        crate::busy::Handler::SqliteDefault => {
            connection.busy_timeout(core::time::Duration::from_millis(busy_timeout_ms))?;
        }
    }
    Ok(())
}

/// Creates the schema if it is not there, and mints the store's identity once.
fn migrate<T: TagStorage>(connection: &Connection) -> rusqlite::Result<()> {
    connection.execute_batch(
        "BEGIN IMMEDIATE;
         CREATE TABLE IF NOT EXISTS event (
             position        INTEGER PRIMARY KEY AUTOINCREMENT,
             event_type      TEXT    NOT NULL,
             data            BLOB    NOT NULL,
             metadata        BLOB,
             tags_text       TEXT    NOT NULL,
             tags_json       TEXT    NOT NULL,
             origin_store    BLOB,
             origin_position INTEGER,
             recorded_at     INTEGER NOT NULL,
             UNIQUE (origin_store, origin_position)
         );
         CREATE INDEX IF NOT EXISTS event_type_idx ON event(event_type, position);
         CREATE TABLE IF NOT EXISTS store_meta (
             k TEXT PRIMARY KEY,
             v BLOB NOT NULL
         ) WITHOUT ROWID;
         CREATE TABLE IF NOT EXISTS tag_cardinality (
             tag    TEXT    PRIMARY KEY,
             events INTEGER NOT NULL
         ) WITHOUT ROWID;
         COMMIT;",
    )?;
    T::migrate(connection)?;
    connection.execute(
        "INSERT OR IGNORE INTO store_meta (k, v) VALUES ('store_id', ?)",
        [&mint_store_id().to_bytes()[..]],
    )?;
    Ok(())
}

/// Reads the persisted incarnation back.
fn read_store_id(connection: &Connection) -> rusqlite::Result<StoreId> {
    let raw: Vec<u8> =
        connection.query_row("SELECT v FROM store_meta WHERE k = 'store_id'", [], |row| {
            row.get(0)
        })?;
    let mut bytes = [0u8; 16];
    let take = raw.len().min(16);
    bytes[..take].copy_from_slice(&raw[..take]);
    Ok(StoreId::from_bytes(bytes))
}

/// A fresh incarnation identifier, minted once per database file.
///
/// Salt plus counter, the shape `MemoryEventStore::next_store_id` uses: the
/// salt separates two runs, the counter separates two files within one run, and
/// the value is written to `store_meta` exactly once and read back on every
/// open thereafter.
fn mint_store_id() -> StoreId {
    use core::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(1);

    let salt = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos() as u64);
    let ordinal = COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&salt.to_be_bytes());
    bytes[8..].copy_from_slice(&ordinal.to_be_bytes());
    StoreId::from_bytes(bytes)
}

/// Inserts one event row and everything that hangs off it.
///
/// Shared by all three strategies, so an arm can differ only in how it decides
/// whether to reach here.
pub(crate) fn insert_event<T: TagStorage>(
    connection: &Connection,
    event: &Event,
    recorded_at: RecordedAt,
) -> rusqlite::Result<i64> {
    let (tags_text, tags_json) = encoded_columns(event.tags());
    connection.execute(
        "INSERT INTO event \
         (event_type, data, metadata, tags_text, tags_json, recorded_at) \
         VALUES (?, ?, ?, ?, ?, ?)",
        rusqlite::params![
            event.event_type().as_str(),
            &event.data()[..],
            event.metadata().map(|meta| &meta[..]),
            tags_text,
            tags_json,
            recorded_at.as_millis(),
        ],
    )?;
    let position = connection.last_insert_rowid();
    T::write_tags(
        connection,
        position,
        event.event_type().as_str(),
        event.tags(),
    )?;
    bump_cardinality(connection, event.tags())?;
    Ok(position)
}

/// Maintains `tag_cardinality`, on every arm.
///
/// `pub(crate)` because [`ConditionalInsert`](crate::ConditionalInsert) writes
/// its first row through a conditional `INSERT … SELECT` rather than through
/// [`insert_event`], and must not silently skip the bookkeeping every other
/// path does — an arm whose write is cheaper because it forgot something is an
/// arm that wins the measurement dishonestly.
pub(crate) fn bump_cardinality(connection: &Connection, tags: &Tags) -> rusqlite::Result<()> {
    if tags.is_empty() {
        return Ok(());
    }
    let mut statement = connection.prepare(
        "INSERT INTO tag_cardinality (tag, events) VALUES (?, 1) \
         ON CONFLICT(tag) DO UPDATE SET events = events + 1",
    )?;
    for tag in tags.iter() {
        statement.execute([tag.as_str()])?;
    }
    Ok(())
}

/// Stamps identity onto every row this transaction inserted.
///
/// One statement at the end of the batch rather than a value bound per row,
/// because a locally appended event's identity is *this store's incarnation
/// paired with the position just assigned* — and the position is not known
/// until the row exists. `origin_position IS NULL` is the marker, which the
/// `UNIQUE (origin_store, origin_position)` constraint tolerates because SQLite
/// treats NULLs as distinct.
pub(crate) fn stamp_identity(connection: &Connection, store_id: StoreId) -> rusqlite::Result<()> {
    connection.execute(
        "UPDATE event SET origin_store = ?, origin_position = position \
         WHERE origin_position IS NULL",
        [&store_id.to_bytes()[..]],
    )?;
    Ok(())
}

/// The lowest position violating any guard of `condition`, if one exists.
///
/// The reference definition is `AppendCondition::is_violated_by`: violated if
/// **any** guard is violated, and a guard is violated by an event that matches
/// its query and sits strictly after its boundary. Each guard is parenthesised
/// into its own subquery rather than folded into one `WHERE`, which is what
/// `append.rs` warns adapters generating SQL to do.
pub(crate) fn conflicting_position<T: TagStorage>(
    connection: &Connection,
    condition: &AppendCondition,
) -> rusqlite::Result<Option<i64>> {
    for guard in condition.guards() {
        let mut params: Vec<Value> = Vec::new();
        let matched = match_sql::<T>(&guard.query, &mut params);
        params.push(Value::Integer(guard.after.map_or(0, as_i64)));

        let found: Option<i64> = connection.query_row(
            &format!("SELECT min(position) FROM ({matched}) WHERE position > ?"),
            rusqlite::params_from_iter(params.iter()),
            |row| row.get(0),
        )?;
        if found.is_some() {
            return Ok(found);
        }
    }
    Ok(None)
}

/// Rebuilds a stored row as the contract's own type.
fn row_to_event(row: &rusqlite::Row<'_>) -> Result<SequencedEvent, SqliteProbeError> {
    let position: i64 = row.get(0)?;
    let event_type: String = row.get(1)?;
    let data: Vec<u8> = row.get(2)?;
    let metadata: Option<Vec<u8>> = row.get(3)?;
    let tags_text: String = row.get(4)?;
    let origin_store: Vec<u8> = row.get(5)?;
    let origin_position: i64 = row.get(6)?;
    let recorded_at: i64 = row.get(7)?;

    let position = SequencePosition::new(position.unsigned_abs()).ok_or_else(|| {
        SqliteProbeError::Corrupt("position 0 is not a valid position".to_owned())
    })?;
    let origin = SequencePosition::new(origin_position.unsigned_abs()).ok_or_else(|| {
        SqliteProbeError::Corrupt("origin position 0 is not a valid position".to_owned())
    })?;

    let mut store_bytes = [0u8; 16];
    let take = origin_store.len().min(16);
    store_bytes[..take].copy_from_slice(&origin_store[..take]);

    let mut event = Event::new(event_type, data)
        .map_err(|err| SqliteProbeError::Corrupt(format!("stored event type: {err}")))?
        .with_tags(decode_tags(&tags_text)?);
    if let Some(metadata) = metadata {
        event = event.with_metadata(metadata);
    }

    Ok(SequencedEvent::new(
        position,
        EventId::new(StoreId::from_bytes(store_bytes), origin),
        RecordedAt::from_millis(recorded_at),
        event,
    ))
}

/// Reads the canonical tag column back as a `Tags`.
///
/// The column is written by every arm whichever one is in force, so this is the
/// one decoder rather than three.
fn decode_tags(encoded: &str) -> Result<Tags, SqliteProbeError> {
    encoded
        .split('\u{1f}')
        .filter(|part| !part.is_empty())
        .map(|part| {
            Tag::new(part).map_err(|err| SqliteProbeError::Corrupt(format!("stored tag: {err}")))
        })
        .collect::<Result<Vec<Tag>, _>>()
        .map(|tags| tags.into_iter().collect())
}

impl<A: AppendStrategy, T: TagStorage> SendEventStore for CandidateStore<A, T> {
    type Error = SqliteProbeError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        // Snapshot under the lock, then release it. The stream that leaves this
        // function borrows nothing and holds no guard, which is what lets a
        // live read stream sit open while another handle appends.
        match self.select(query, options) {
            Ok(events) => Rows::Ready(events.into_iter()),
            Err(err) => Rows::Failed(Some(err)),
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first, and above the lock — ES-20. The specification defines
        // a batch as non-empty, so there is no position to return, and the
        // precedence matters: `ConditionViolated` means "rebuild and retry", so
        // reporting it for an empty batch puts a correct client into a loop that
        // never terminates.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let recorded_at = now();
        let mut connection = self.locked();

        match A::append::<T>(
            &mut connection,
            self.store_id,
            events,
            condition,
            recorded_at,
        ) {
            Ok(position) => Ok(position),
            Err(AppendFailure::Violated(at)) => Err(AppendError::ConditionViolated(
                at.map_or_else(ConditionViolated::unspecified, ConditionViolated::at),
            )),
            Err(AppendFailure::Sqlite(err)) => Err(AppendError::Store(err.into())),
        }
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let connection = self.locked();
        let highest: Option<i64> =
            connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
        Ok(highest.and_then(|value| SequencePosition::new(value.unsigned_abs())))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let connection = self.locked();
        let found: Option<i64> = connection
            .query_row(
                "SELECT 1 FROM event WHERE origin_store = ? AND origin_position = ?",
                rusqlite::params![&id.store().to_bytes()[..], as_i64(id.position())],
                |row| row.get(0),
            )
            .or_else(|err| match err {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(other),
            })?;
        Ok(found.is_some())
    }
}

/// The stream `read` returns: an owned snapshot, or the one error that stopped
/// it being made.
///
/// A failure surfaces as an `Err` *item* rather than up front, which is what the
/// port's laziness contract already promises.
#[derive(Debug)]
enum Rows {
    Ready(std::vec::IntoIter<SequencedEvent>),
    Failed(Option<SqliteProbeError>),
}

impl Stream for Rows {
    type Item = Result<SequencedEvent, SqliteProbeError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _context: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(match self.get_mut() {
            Rows::Ready(events) => events.next().map(Ok),
            Rows::Failed(err) => err.take().map(Err),
        })
    }
}

/// The host clock, as milliseconds since the Unix epoch.
fn now() -> RecordedAt {
    use std::time::{SystemTime, UNIX_EPOCH};

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|since| i64::try_from(since.as_millis()).ok())
        .unwrap_or(0);
    RecordedAt::from_millis(millis)
}
