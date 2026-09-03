//! The reproduced event store: `crates/happenstance-sqlite/src/event_store.rs`
//! with `PAGE_SIZE` lifted into a const generic and three counters added.
//!
//! # What this is a copy of, and why a copy exists at all
//!
//! The question this crate answers — *how long does a read page hold the
//! connection mutex* — cannot be asked of the shipped adapter from outside it.
//! `PAGE_SIZE` is a private `const` (`event_store.rs:141`), `fetch_page` is a
//! private method on a private type (`event_store.rs:1276`), and the mutex is a
//! private field. Nothing public observes any of the three. So the page fetch is
//! reproduced here, and the review's constraint that no file under `crates/`
//! may be touched is what makes that the only available shape.
//!
//! **A copy that has drifted from the original measures nothing.** Three things
//! hold this one honest, and they are listed here because a reader is owed the
//! means to check rather than the assertion:
//!
//! 1. `src/query_sql.rs` is a **byte-for-byte** copy of
//!    `crates/happenstance-sqlite/src/query_sql.rs`, and `src/row.rs` is a copy
//!    of `crates/happenstance-sqlite/src/row.rs` under one documented rename.
//!    `tests/the_copy_has_not_drifted.rs` re-derives both from the originals at
//!    the live tree and fails on any difference.
//! 2. Every constant the original exposes is **used from the original** rather
//!    than restated: [`SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT`],
//!    `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH` are
//!    read off the real type below. The one constant that is private —
//!    `PAGE_SIZE` — is checked against the original's source text by the same
//!    drift test.
//! 3. CONTROL 2: `tests/replica_is_conformant.rs` runs
//!    `happenstance_testkit::event_store_conformance!` against this type at
//!    **every** page size that is later timed. A page size that drops rows at a
//!    page boundary is fast and wrong, and a reproduction that has drifted from
//!    the original in any way the suite can see is caught before a figure exists.
//!
//! # What is deliberately different, and where
//!
//! Four differences, each of them the measurement itself:
//!
//! * `const PAGE_SIZE: usize = 512` (`event_store.rs:141`) becomes the const
//!   generic parameter `PAGE`. The default arm the harness calls `Replica<512>`
//!   is the shipped value.
//! * `fetch_page` times the `connection.lock()` call and the life of the guard,
//!   and reports both to [`crate::probe`].
//! * `fetch_page` optionally opens a residency region around the guard's life
//!   (see [`crate::counting`]), and reports the merge buffer's length **before**
//!   `truncate(budget)` — which is where the `ceil(arms / 400) × PAGE_SIZE`
//!   multiplier is either visible or invented.
//! * The error enum drops the variants no path here can reach
//!   (`UnsupportedSchemaVersion`'s siblings are kept; the projection store's are
//!   not) and is otherwise the same set.
//!
//! Everything else — migration 1, the ceilings check, `BEGIN IMMEDIATE`, the
//! `SELECT max(position)` guard, the chunked plan, the ceiling sample on the
//! polling thread, the deferred `spawn_blocking`, the merge, the dedup, the
//! `exhausted` computation before truncation — is the original's, transcribed.

use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Instant;
use std::vec;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, InvalidEventType, InvalidTag,
    Query, ReadOptions, RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;
use rusqlite::types::Value;
use tokio::runtime::{Handle, TryCurrentError};
use tokio::task::{JoinError, JoinHandle};

use crate::query_sql::Selectivity;

/// The page size the shipped adapter uses, restated here **only** so that
/// `tests/the_copy_has_not_drifted.rs` has something to compare the original's
/// source text against.
///
/// Nothing in this crate reads it as a page size; the harness passes `512`
/// explicitly, beside `64`, `128` and `2048`.
pub const SHIPPED_PAGE_SIZE: usize = 512;

/// Migration 1, transcribed from `crates/happenstance-sqlite/src/event_store.rs:202-228`.
const MIGRATION_1: &str = "\
CREATE TABLE IF NOT EXISTS event (
    position        INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type      TEXT    NOT NULL,
    data            BLOB    NOT NULL,
    metadata        BLOB,
    tags            BLOB    NOT NULL,
    origin_store    BLOB,
    origin_position INTEGER,
    recorded_at     INTEGER NOT NULL,
    UNIQUE (origin_store, origin_position)
);
CREATE INDEX IF NOT EXISTS event_type_idx ON event(event_type, position);
CREATE TABLE IF NOT EXISTS event_tag (
    tag        TEXT    NOT NULL,
    position   INTEGER NOT NULL REFERENCES event(position),
    event_type TEXT    NOT NULL,
    PRIMARY KEY (tag, position)
) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS tag_cardinality (
    tag    TEXT    PRIMARY KEY,
    events INTEGER NOT NULL
) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS store_meta (
    k TEXT PRIMARY KEY,
    v BLOB NOT NULL
) WITHOUT ROWID;";

/// The migration this build knows how to operate.
pub const SCHEMA_VERSION: u32 = 1;

/// The `store_meta` key holding this database's incarnation.
const STORE_ID_KEY: &str = "store_id";

/// The `store_meta` key holding the migration the file is at.
const SCHEMA_VERSION_KEY: &str = "schema_version";

/// How many bound parameters one statement may carry
/// (`event_store.rs:577`).
const PARAMETER_BUDGET: usize = 30_000;

/// Bound parameters one `event_tag` row costs (`event_store.rs:580`).
const TAG_ROW_PARAMETERS: usize = 3;

/// A SQLite-backed event store whose page size is a type parameter.
///
/// One connection behind one [`Mutex`], exactly as
/// [`SqliteEventStore`] has it, and for the same reason: `rusqlite::Connection`
/// is [`Send`] and not [`Sync`], and the mutex is what buys `Self: Sync` and
/// therefore the `Send` flavour.
#[derive(Debug, Clone)]
pub struct Replica<const PAGE: usize> {
    connection: Arc<Mutex<Connection>>,
    store_id: StoreId,
    runtime: Option<Handle>,
}

impl<const PAGE: usize> Replica<PAGE> {
    /// Opens (creating if absent) a store at `path` and applies the schema.
    ///
    /// Through `happenstance_sqlite::connection::open_configured` — the shipped
    /// adapter's own door, called rather than reproduced, so WAL, `synchronous`
    /// and the 5,000 ms busy timeout are the settings the adapter ships with and
    /// not a second opinion about them.
    ///
    /// # Errors
    ///
    /// As [`SqliteEventStore::open`]: the driver's error if the file cannot be
    /// opened or migrated, and the identity errors below.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ReplicaError> {
        let mut connection = happenstance_sqlite::connection::open_configured(path)?;
        let store_id = migrate(&mut connection)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            store_id,
            runtime: Handle::try_current().ok(),
        })
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// The page size this instance was monomorphised for.
    #[must_use]
    pub const fn page_size(&self) -> usize {
        PAGE
    }

    /// The connection this handle shares, so that [`crate::seam::SeamStore`] can
    /// be built onto the **same** mutex rather than onto a second one.
    ///
    /// Sharing it is the whole point: instrument (c) is only a comparison if the
    /// seam arm and the inline arm contend for the same lock and the same file.
    #[must_use]
    pub fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.connection)
    }
}

/// The three ceilings, checked before any transaction opens
/// (`event_store.rs:474-496`).
///
/// The magnitudes are read off [`SqliteEventStore`] rather than restated, so a
/// change to one of them in the shipped crate reaches this reproduction through
/// the type system rather than through someone remembering.
pub(crate) fn check_ceilings(events: &[Event]) -> Result<(), AppendError<ReplicaError>> {
    if events.len() > SqliteEventStore::MAX_EVENTS_PER_BATCH {
        return Err(AppendError::ExceedsStoreLimit {
            limit: happenstance_core::StoreLimit::EventsPerBatch,
            len: events.len(),
        });
    }
    for event in events {
        if event.data().len() > SqliteEventStore::MAX_EVENT_DATA_LEN {
            return Err(AppendError::ExceedsStoreLimit {
                limit: happenstance_core::StoreLimit::EventDataLen,
                len: event.data().len(),
            });
        }
        if event.tags().len() > SqliteEventStore::MAX_TAGS_PER_EVENT {
            return Err(AppendError::ExceedsStoreLimit {
                limit: happenstance_core::StoreLimit::TagsPerEvent,
                len: event.tags().len(),
            });
        }
    }
    Ok(())
}

/// Everything that happens inside the one `BEGIN IMMEDIATE`
/// (`event_store.rs:505-528`).
pub(crate) fn append_locked(
    connection: &mut Connection,
    store_id: StoreId,
    events: &[Event],
    condition: Option<&AppendCondition>,
    recorded_at: RecordedAt,
) -> Result<SequencePosition, AppendError<ReplicaError>> {
    let transaction = connection
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .map_err(store_error)?;

    if let Some(condition) = condition
        && let Some(conflict) = evaluate(&transaction, condition).map_err(store_error)?
    {
        drop(transaction);
        return Err(AppendError::ConditionViolated(ConditionViolated::at(
            conflict,
        )));
    }

    let last = write_batch(&transaction, store_id, events, recorded_at).map_err(store_error)?;
    transaction.commit().map_err(store_error)?;
    Ok(last)
}

/// Applies the schema and returns the incarnation the file carries afterwards
/// (`event_store.rs:547-568`).
fn migrate(connection: &mut Connection) -> Result<StoreId, ReplicaError> {
    let transaction =
        connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

    transaction.execute_batch(MIGRATION_1)?;
    transaction.execute(
        "INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, ?)",
        rusqlite::params![SCHEMA_VERSION_KEY, SCHEMA_VERSION],
    )?;
    transaction.execute(
        "INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, randomblob(16))",
        [STORE_ID_KEY],
    )?;

    let store_id = read_identity(&transaction)?;
    transaction.commit()?;
    Ok(store_id)
}

/// Milliseconds since the Unix epoch, stamped once, at append
/// (`event_store.rs:587-596`).
pub(crate) fn now() -> RecordedAt {
    use std::time::{SystemTime, UNIX_EPOCH};

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|since| i64::try_from(since.as_millis()).ok())
        .unwrap_or(0);
    RecordedAt::from_millis(millis)
}

/// A [`SequencePosition`] as the integer SQLite stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// Wraps an adapter failure as the port's store-error arm.
fn store_error(error: impl Into<ReplicaError>) -> AppendError<ReplicaError> {
    AppendError::Store(error.into())
}

/// The position violating a guard of `condition`, if one exists
/// (`event_store.rs:642-675`).
fn evaluate(
    connection: &Connection,
    condition: &AppendCondition,
) -> rusqlite::Result<Option<SequencePosition>> {
    for guard in condition.guards() {
        let selectivity = Selectivity::read_for(connection, &guard.query)?;
        let plan = crate::query_sql::chunks(
            &guard.query,
            &selectivity,
            SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
        );

        let mut highest: Option<i64> = None;
        for (matched, params) in plan {
            let chunk: Option<i64> = connection.query_row(
                &format!("SELECT max(position) FROM ({matched})"),
                rusqlite::params_from_iter(params.iter()),
                |row| row.get(0),
            )?;
            highest = highest.max(chunk);
        }

        let boundary = guard.after.map_or(0, as_i64);
        if let Some(highest) = highest
            && highest > boundary
        {
            return Ok(SequencePosition::new(highest.unsigned_abs()));
        }
    }
    Ok(None)
}

/// Writes every row of the batch, and returns the position of its last event
/// (`event_store.rs:690-751`).
fn write_batch(
    connection: &Connection,
    store_id: StoreId,
    events: &[Event],
    recorded_at: RecordedAt,
) -> rusqlite::Result<SequencePosition> {
    let mut positions = Vec::with_capacity(events.len());
    {
        let mut insert = connection.prepare(
            "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
             VALUES (?, ?, ?, ?, ?)",
        )?;
        for event in events {
            insert.execute(rusqlite::params![
                event.event_type().as_str(),
                &event.data()[..],
                event.metadata().map(|metadata| &metadata[..]),
                crate::row::encode_tags(event.tags()),
                recorded_at.as_millis(),
            ])?;
            positions.push(connection.last_insert_rowid());
        }
    }

    write_tag_rows(connection, events, &positions)?;
    bump_cardinality(connection, events)?;

    if let Some(&first) = positions.first() {
        connection.execute(
            "UPDATE event SET origin_store = ?, origin_position = position \
             WHERE position >= ? AND origin_position IS NULL",
            rusqlite::params![&store_id.to_bytes()[..], first],
        )?;
    }

    let last = positions.last().copied().unwrap_or_default();
    SequencePosition::new(last.unsigned_abs())
        .ok_or(rusqlite::Error::IntegralValueOutOfRange(0, last))
}

/// Inserts every `(tag, position, event_type)` row, chunked to the parameter
/// budget (`event_store.rs:755-781`).
fn write_tag_rows(
    connection: &Connection,
    events: &[Event],
    positions: &[i64],
) -> rusqlite::Result<()> {
    let rows_per_statement = (PARAMETER_BUDGET / TAG_ROW_PARAMETERS).max(1);
    let mut buffer: Vec<Value> = Vec::with_capacity(rows_per_statement * TAG_ROW_PARAMETERS);
    let mut buffered = 0usize;

    for (event, position) in events.iter().zip(positions) {
        for tag in event.tags() {
            buffer.push(Value::Text(tag.as_str().to_owned()));
            buffer.push(Value::Integer(*position));
            buffer.push(Value::Text(event.event_type().as_str().to_owned()));
            buffered += 1;
            if buffered == rows_per_statement {
                flush_tag_rows(connection, &buffer, buffered)?;
                buffer.clear();
                buffered = 0;
            }
        }
    }
    if buffered > 0 {
        flush_tag_rows(connection, &buffer, buffered)?;
    }
    Ok(())
}

/// One multi-row `event_tag` insert (`event_store.rs:784-791`).
fn flush_tag_rows(connection: &Connection, buffer: &[Value], rows: usize) -> rusqlite::Result<()> {
    let tuples = (0..rows).map(|_| "(?,?,?)").collect::<Vec<_>>().join(",");
    connection.execute(
        &format!("INSERT INTO event_tag (tag, position, event_type) VALUES {tuples}"),
        rusqlite::params_from_iter(buffer.iter()),
    )?;
    Ok(())
}

/// Maintains `tag_cardinality` (`event_store.rs:798-809`).
fn bump_cardinality(connection: &Connection, events: &[Event]) -> rusqlite::Result<()> {
    let mut statement = connection.prepare(
        "INSERT INTO tag_cardinality (tag, events) VALUES (?, 1) \
         ON CONFLICT(tag) DO UPDATE SET events = events + 1",
    )?;
    for event in events {
        for tag in event.tags() {
            statement.execute([tag.as_str()])?;
        }
    }
    Ok(())
}

/// Reads the schema version and the incarnation back out of `store_meta`
/// (`event_store.rs:817-846`).
fn read_identity(connection: &Connection) -> Result<StoreId, ReplicaError> {
    let version: u32 = connection.query_row(
        "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = ?",
        [SCHEMA_VERSION_KEY],
        |row| row.get(0),
    )?;
    if version > SCHEMA_VERSION {
        return Err(ReplicaError::UnsupportedSchemaVersion {
            found: version,
            supported: SCHEMA_VERSION,
        });
    }

    let raw: Vec<u8> = connection
        .query_row(
            "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = ?",
            [STORE_ID_KEY],
            |row| row.get(0),
        )
        .map_err(|err| match err {
            rusqlite::Error::QueryReturnedNoRows => ReplicaError::MissingIdentity,
            other => ReplicaError::Sqlite(other),
        })?;

    let bytes: [u8; 16] = raw
        .as_slice()
        .try_into()
        .map_err(|_| ReplicaError::MalformedIdentity { len: raw.len() })?;
    Ok(StoreId::from_bytes(bytes))
}

/// How [`Replica`] fails — `SqliteEventStoreError`'s variants, transcribed.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReplicaError {
    /// The driver failed: I/O, `SQLITE_BUSY`, a constraint, a bad statement.
    #[error("SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A thread panicked while holding the connection mutex.
    #[error("the SQLite connection mutex was poisoned by a panicking thread")]
    ConnectionPoisoned,

    /// The blocking task carrying a query panicked or was cancelled.
    #[error("the blocking SQLite task did not complete: {0}")]
    Worker(#[from] JoinError),

    /// A stream was polled outside a tokio runtime.
    #[error("no tokio runtime is available to run the blocking SQLite query: {0}")]
    NoRuntime(#[from] TryCurrentError),

    /// A stored row carried a position the contract cannot represent.
    #[error("stored position {0} is not a valid sequence position")]
    InvalidPosition(i64),

    /// A stored row carried an event type that no longer validates.
    #[error("stored event type is invalid: {0}")]
    StoredEventType(#[from] InvalidEventType),

    /// A stored row carried a tag that no longer validates.
    #[error("stored tag is invalid: {0}")]
    StoredTag(#[from] InvalidTag),

    /// The file was written by a build that knows a later migration.
    #[error(
        "this build understands schema version {supported}, and the database is \
         at version {found}"
    )]
    UnsupportedSchemaVersion {
        /// The version the file carries.
        found: u32,
        /// The version this build knows.
        supported: u32,
    },

    /// The schema is present and its incarnation row is not.
    #[error("the database has been migrated but carries no store identity")]
    MissingIdentity,

    /// The incarnation row is present and is not sixteen bytes.
    #[error("the stored store identity is {len} bytes, and a StoreId is sixteen")]
    MalformedIdentity {
        /// How many bytes the row actually held.
        len: usize,
    },

    /// A stored row carries no event identity.
    #[error("the event at position {position} carries no origin identity")]
    UnstampedEvent {
        /// Where the unstamped row sits.
        position: SequencePosition,
    },

    /// The canonical tag column of a stored row is not UTF-8.
    #[error("a stored tag column is not valid UTF-8")]
    CorruptTags,
}

impl<const PAGE: usize> SendEventStore for Replica<PAGE> {
    type Error = ReplicaError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        ReplicaReadStream::<PAGE> {
            state: ReadState::Idle(Box::new(ReadCursor {
                connection: Arc::clone(&self.connection),
                runtime: self.runtime.clone(),
                query: query.clone(),
                options,
                resume_from: options.from,
                remaining: options.limit,
                ceiling: Ceiling::Unsampled,
                finished: false,
            })),
        }
    }

    /// `event_store.rs:1018-1053`, transcribed. The mutex is taken on the
    /// calling task and the whole transaction runs under it.
    ///
    /// # Errors
    ///
    /// As [`SqliteEventStore`]'s `append`.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        check_ceilings(events)?;

        let recorded_at = now();
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| AppendError::Store(ReplicaError::ConnectionPoisoned))?;

        append_locked(
            &mut connection,
            self.store_id,
            events,
            condition,
            recorded_at,
        )
    }

    /// `event_store.rs:1069-1078`, transcribed.
    ///
    /// # Errors
    ///
    /// As [`SqliteEventStore`]'s `head`.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| ReplicaError::ConnectionPoisoned)?;
        let highest: Option<i64> =
            connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
        Ok(highest.and_then(|value| SequencePosition::new(value.unsigned_abs())))
    }

    /// `event_store.rs:1090-1106`, transcribed.
    ///
    /// # Errors
    ///
    /// As [`SqliteEventStore`]'s `contains_event_id`.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| ReplicaError::ConnectionPoisoned)?;
        let found: Option<i64> = connection
            .query_row(
                "SELECT 1 FROM event WHERE origin_store = ? AND origin_position = ? LIMIT 1",
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

/// The stream [`Replica::read`](SendEventStore::read) returns
/// (`event_store.rs:1134-1136`).
#[derive(Debug)]
pub struct ReplicaReadStream<const PAGE: usize> {
    state: ReadState<PAGE>,
}

/// Where a [`ReplicaReadStream`] is in its life (`event_store.rs:1140-1152`).
#[derive(Debug)]
enum ReadState<const PAGE: usize> {
    Idle(Box<ReadCursor<PAGE>>),
    Fetching(JoinHandle<FetchOutcome<PAGE>>),
    Draining {
        cursor: Box<ReadCursor<PAGE>>,
        rows: vec::IntoIter<SequencedEvent>,
    },
    Done,
}

/// What one blocking hop hands back (`event_store.rs:1159`).
type FetchOutcome<const PAGE: usize> = (Box<ReadCursor<PAGE>>, Result<Page, ReplicaError>);

/// One page of rows, plus whether the query is spent (`event_store.rs:1163-1166`).
#[derive(Debug)]
struct Page {
    rows: Vec<SequencedEvent>,
    exhausted: bool,
}

/// ADR-0011's position ceiling, and whether it has been taken yet
/// (`event_store.rs:1175-1185`).
#[derive(Debug, Clone, Copy)]
enum Ceiling {
    Unsampled,
    Empty,
    At(SequencePosition),
}

/// Everything the blocking thread needs to fetch the next page
/// (`event_store.rs:1189-1215`).
#[derive(Debug)]
struct ReadCursor<const PAGE: usize> {
    connection: Arc<Mutex<Connection>>,
    runtime: Option<Handle>,
    query: Query,
    options: ReadOptions,
    resume_from: Option<SequencePosition>,
    remaining: Option<usize>,
    ceiling: Ceiling,
    finished: bool,
}

impl<const PAGE: usize> ReadCursor<PAGE> {
    /// Takes ADR-0011's position ceiling, once (`event_store.rs:1239-1253`).
    ///
    /// **This runs on the polling thread**, which is the original's deliberate
    /// choice and the residual instrument (c) cannot remove: it takes the same
    /// mutex `append` holds, from inside `poll_next`.
    fn sample_ceiling(&mut self) -> Result<(), ReplicaError> {
        if !matches!(self.ceiling, Ceiling::Unsampled) {
            return Ok(());
        }
        let connection = self
            .connection
            .lock()
            .map_err(|_| ReplicaError::ConnectionPoisoned)?;
        let highest: Option<i64> =
            connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
        self.ceiling = highest
            .and_then(|value| SequencePosition::new(value.unsigned_abs()))
            .map_or(Ceiling::Empty, Ceiling::At);
        Ok(())
    }

    /// Runs one page's worth of SQL (`event_store.rs:1276-1405`), timed.
    ///
    /// The three instrument lines are the `Instant`s around the lock and the
    /// residency region; everything between them is the original.
    fn fetch_page(&mut self) -> Result<Page, ReplicaError> {
        self.sample_ceiling()?;

        // The `Arc` is cloned before the lock rather than locked in place, and
        // that is a borrow-checker consequence of splitting the body out rather
        // than a change of behaviour: a guard taken from `self.connection`
        // borrows `self` immutably, which `fetch_page_locked(&mut self)` cannot
        // then take. The original keeps everything in one function and needs no
        // clone. One relaxed atomic increment, once per page, against a page
        // whose median hold is measured in hundreds of microseconds.
        let shared = Arc::clone(&self.connection);

        // --- instrument: the wait for the lock -------------------------------
        let queued = Instant::now();
        let connection = shared.lock().map_err(|_| ReplicaError::ConnectionPoisoned)?;
        let wait = queued.elapsed();
        let held_from = Instant::now();
        let residency = crate::probe::want_peak().then(crate::counting::begin_region);
        // --- the original, from `event_store.rs:1286` ------------------------

        let outcome = self.fetch_page_locked(&connection);

        // --- instrument: the life of the guard -------------------------------
        let hold = held_from.elapsed();
        if let Some(baseline) = residency {
            crate::probe::record_peak_bytes(crate::counting::end_region(baseline));
        }
        drop(connection);
        crate::probe::record_page(
            u64::try_from(wait.as_nanos()).unwrap_or(u64::MAX),
            u64::try_from(hold.as_nanos()).unwrap_or(u64::MAX),
        );

        outcome
    }

    /// The body that runs with the guard live — `event_store.rs:1286-1404`
    /// verbatim in behaviour, split out only so that the timer above can bracket
    /// it without an early `return` escaping the measurement.
    ///
    /// That split is the one structural liberty taken with the original, and it
    /// is behaviour-preserving: the original's three early returns all leave the
    /// function, so they release the guard at exactly the point this one returns
    /// to the caller above.
    fn fetch_page_locked(&mut self, connection: &Connection) -> Result<Page, ReplicaError> {
        let Ceiling::At(ceiling) = self.ceiling else {
            return Ok(Page {
                rows: Vec::new(),
                exhausted: true,
            });
        };

        let budget = self.remaining.map_or(PAGE, |left| left.min(PAGE));
        if budget == 0 {
            return Ok(Page {
                rows: Vec::new(),
                exhausted: true,
            });
        }

        let selectivity = Selectivity::read_for(connection, &self.query)?;

        let plan = crate::query_sql::chunks(
            &self.query,
            &selectivity,
            SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
        );
        let statements = plan.len();
        let mut merged: Vec<SequencedEvent> = Vec::with_capacity(budget);

        for (matched, arm_params) in plan {
            let mut params = arm_params;
            let columns = crate::row::COLUMNS;
            let mut sql = format!("SELECT {columns} FROM event WHERE position IN ({matched})");

            if let Some(from) = self.resume_from {
                sql.push_str(if self.options.backwards {
                    " AND position <= ?"
                } else {
                    " AND position >= ?"
                });
                params.push(Value::Integer(as_i64(from)));
            }
            if let Some(to) = self.options.to {
                sql.push_str(if self.options.backwards {
                    " AND position >= ?"
                } else {
                    " AND position <= ?"
                });
                params.push(Value::Integer(as_i64(to)));
            }

            sql.push_str(" AND position <= ?");
            params.push(Value::Integer(as_i64(ceiling)));

            sql.push_str(if self.options.backwards {
                " ORDER BY position DESC"
            } else {
                " ORDER BY position ASC"
            });

            sql.push_str(" LIMIT ?");
            params.push(Value::Integer(i64::try_from(budget).unwrap_or(i64::MAX)));

            let mut statement = connection.prepare(&sql)?;
            let mut rows = statement.query(rusqlite::params_from_iter(params.iter()))?;
            while let Some(row) = rows.next()? {
                merged.push(crate::row::to_event(row)?);
            }
        }

        // --- instrument: the multiplier, before it is truncated away ---------
        crate::probe::record_shape(merged.len(), statements);

        if self.options.backwards {
            merged.sort_by_key(|event| core::cmp::Reverse(event.position));
        } else {
            merged.sort_by_key(|event| event.position);
        }
        merged.dedup_by_key(|event| event.position);

        let exhausted = merged.len() < budget;
        merged.truncate(budget);

        Ok(Page {
            rows: merged,
            exhausted,
        })
    }

    /// Folds a fetched page back into the cursor (`event_store.rs:1413-1430`).
    fn advance(&mut self, page: &Page) {
        let mut exhausted_by_position = false;
        if let Some(last) = page.rows.last() {
            let next = if self.options.backwards {
                SequencePosition::new(last.position.get().saturating_sub(1))
            } else {
                last.position.next()
            };
            exhausted_by_position = next.is_none();
            self.resume_from = next;
        }
        if let Some(remaining) = self.remaining.as_mut() {
            *remaining = remaining.saturating_sub(page.rows.len());
        }
        self.finished = page.exhausted || exhausted_by_position || self.remaining == Some(0);
    }
}

impl<const PAGE: usize> Stream for ReplicaReadStream<PAGE> {
    type Item = Result<SequencedEvent, ReplicaError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = Pin::into_inner(self);

        loop {
            match std::mem::replace(&mut this.state, ReadState::Done) {
                ReadState::Idle(mut cursor) => {
                    if cursor.finished {
                        return Poll::Ready(None);
                    }
                    if let Err(err) = cursor.sample_ceiling() {
                        return Poll::Ready(Some(Err(err)));
                    }
                    let runtime = match cursor.runtime.clone() {
                        Some(runtime) => runtime,
                        None => match Handle::try_current() {
                            Ok(runtime) => runtime,
                            Err(err) => return Poll::Ready(Some(Err(err.into()))),
                        },
                    };
                    this.state = ReadState::Fetching(runtime.spawn_blocking(move || {
                        let mut cursor = cursor;
                        let page = cursor.fetch_page();
                        (cursor, page)
                    }));
                }
                ReadState::Fetching(mut handle) => match Pin::new(&mut handle).poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::Fetching(handle);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(join)) => return Poll::Ready(Some(Err(join.into()))),
                    Poll::Ready(Ok((_cursor, Err(err)))) => return Poll::Ready(Some(Err(err))),
                    Poll::Ready(Ok((mut cursor, Ok(page)))) => {
                        cursor.advance(&page);
                        this.state = ReadState::Draining {
                            cursor,
                            rows: page.rows.into_iter(),
                        };
                    }
                },
                ReadState::Draining { cursor, mut rows } => match rows.next() {
                    Some(event) => {
                        this.state = ReadState::Draining { cursor, rows };
                        return Poll::Ready(Some(Ok(event)));
                    }
                    None => this.state = ReadState::Idle(cursor),
                },
                ReadState::Done => return Poll::Ready(None),
            }
        }
    }
}
