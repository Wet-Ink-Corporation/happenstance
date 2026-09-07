//! One store, four guard shapes: everything that is *not* the SQL under test.
//!
//! The schema is `crates/happenstance-sqlite/src/event_store.rs:202-227`'s
//! `MIGRATION_1` **verbatim**, and the two `store_meta` rows are written with the
//! same keys and the same `randomblob(16)` identity, so a database this store
//! writes is a database the real
//! [`SqliteEventStore`](happenstance_sqlite::event_store::SqliteEventStore)
//! opens. That is not a nicety: `tests/guard_cost.rs` runs the real adapter as a
//! fifth arm on the same file, and `tests/query_plan.rs` and
//! `tests/seed_ordering.rs` seed through [`crate::seed`] and then read the log
//! back through the adapter itself. A number measured against a different
//! `event_tag` is a number about a different adapter.
//!
//! The connection is opened by
//! [`happenstance_sqlite::connection::open_configured`] — the adapter's own
//! door — so the pragmas are the adapter's rather than a second set that happens
//! to agree today.
//!
//! # What is faithful, and what is deliberately not
//!
//! Faithful: the schema, `event_tag`'s covering `event_type` column, the
//! `tag_cardinality` maintenance, the [`Selectivity`](crate::chain::Selectivity)
//! lookup *inside* the transaction, `SELECT max(position) FROM (…)` per chunk
//! with `highest > boundary` compared in Rust, the chunk width read off
//! [`SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT`], the batch-bounded identity
//! stamp, and the parameter-budgeted multi-row `event_tag` insert.
//!
//! Not faithful, and stated because it changes how the figures read:
//!
//! 1. **No `spawn_blocking`, no paging.** [`read`](ProbeStore::read) snapshots
//!    under the lock and yields from a `Vec`, as
//!    `experiments/append-condition`'s candidate does. The shipped read path's
//!    512-row `fetch_page` is finding I-3's subject, and a plan question is
//!    answered against the **real** adapter — which `tests/query_plan.rs` does,
//!    capturing the statement off a live replay rather than writing it out.
//! 2. **No declared ceilings.** The shipped store refuses a batch above
//!    `MAX_EVENTS_PER_BATCH = 256`. This one does not, and the conformance suite
//!    has no rule that requires a ceiling — which is why all 89 still pass.
//! 3. **The guard shape is a field**, not a type parameter. Four shapes is four
//!    values of [`Shape`], one store type, and one results table that does not
//!    have to re-derive which arm produced a row.

use std::sync::Mutex;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId, Tag, Tags,
};
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;
use rusqlite::types::Value;

use crate::chain::{Selectivity, Shape, chunks, guard_sql};

/// Migration 1, copied verbatim from
/// `crates/happenstance-sqlite/src/event_store.rs:202-227`.
///
/// Copied rather than referenced because the constant is private. The gap is
/// closed the only way an outsider can: every timed target hands its seeded file
/// to `SqliteEventStore::open` and reads the log back through it before quoting
/// a figure.
pub const MIGRATION_1: &str = "\
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

/// The `store_meta` key holding this database's incarnation.
const STORE_ID_KEY: &str = "store_id";

/// The `store_meta` key holding the migration the file is at.
const SCHEMA_VERSION_KEY: &str = "schema_version";

/// The migration this store writes, which is the one the adapter reads.
const SCHEMA_VERSION: u32 = 1;

/// How many bound parameters one statement may carry, as shipped.
const PARAMETER_BUDGET: usize = 30_000;

/// Bound parameters one `event_tag` row costs.
const TAG_ROW_PARAMETERS: usize = 3;

/// The byte that separates one encoded tag from the next.
const UNIT: u8 = 0x1f;

/// How a probe store fails for its own reasons.
///
/// `ConditionViolated` is deliberately absent: it is the DCB retry signal and
/// reaches the caller through [`AppendError::ConditionViolated`], never through
/// an adapter's own error type.
#[derive(Debug, thiserror::Error)]
pub enum ProbeStoreError {
    /// The driver failed.
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
    /// A row this store wrote could not be read back as the contract's types.
    #[error("a stored row did not round-trip: {0}")]
    Corrupt(String),
}

/// One handle onto one probe store: one `rusqlite::Connection` on one file,
/// evaluating guards in one [`Shape`].
#[derive(Debug)]
pub struct ProbeStore {
    connection: Mutex<Connection>,
    store_id: StoreId,
    shape: Shape,
}

impl ProbeStore {
    /// Opens a handle onto the database at `path`, migrating it if needed.
    ///
    /// The connection comes from the adapter's own
    /// [`open_configured`](happenstance_sqlite::connection::open_configured), so
    /// WAL, `synchronous = NORMAL` and the 5,000 ms busy timeout are the
    /// adapter's settings rather than a second set that happens to agree.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the file cannot be opened or migrated.
    pub fn open(path: &std::path::Path, shape: Shape) -> rusqlite::Result<Self> {
        let mut connection = happenstance_sqlite::connection::open_configured(path)?;
        let store_id = migrate(&mut connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
            store_id,
            shape,
        })
    }

    /// Which SQL shape this handle evaluates guards with.
    #[must_use]
    pub const fn shape(&self) -> Shape {
        self.shape
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// Runs `body` with this handle's connection locked.
    pub fn with_connection<R>(&self, body: impl FnOnce(&Connection) -> R) -> R {
        body(&self.locked())
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
    ) -> Result<Vec<SequencedEvent>, ProbeStoreError> {
        let connection = self.locked();
        let selectivity = Selectivity::read_for(&connection, query)?;
        // **Zero, and it is load-bearing.** A read has no boundary, and the
        // bounded shapes emit `position > 0`, which every position satisfies
        // because a `SequencePosition` is a `NonZeroU64`. That is what makes the
        // bounded shapes' emitted SQL exercised by all 89 conformance rules
        // rather than by only the rules that happen to carry a boundary.
        let plan = chunks(
            self.shape,
            query,
            &selectivity,
            SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
            0,
        );

        let mut merged: Vec<SequencedEvent> = Vec::new();
        for (matched, arm_params) in plan {
            let mut params = arm_params;
            let mut sql = format!(
                "SELECT position, event_type, data, metadata, tags, \
                 origin_store, origin_position, recorded_at \
                 FROM event WHERE position IN ({matched})"
            );

            if let Some(from) = options.from {
                sql.push_str(if options.backwards {
                    " AND position <= ?"
                } else {
                    " AND position >= ?"
                });
                params.push(Value::Integer(as_i64(from)));
            }
            if let Some(to) = options.to {
                sql.push_str(if options.backwards {
                    " AND position >= ?"
                } else {
                    " AND position <= ?"
                });
                params.push(Value::Integer(as_i64(to)));
            }

            sql.push_str(if options.backwards {
                " ORDER BY position DESC"
            } else {
                " ORDER BY position ASC"
            });

            // Applied per chunk and again after the merge: a chunk's rows are
            // already distinct, so the merged top `limit` is a subset of the
            // union of the per-chunk tops and bounding each one loses nothing.
            if let Some(limit) = options.limit {
                sql.push_str(" LIMIT ?");
                params.push(Value::Integer(i64::try_from(limit).unwrap_or(i64::MAX)));
            }

            let mut statement = connection.prepare(&sql)?;
            let mut rows = statement.query(rusqlite::params_from_iter(params.iter()))?;
            while let Some(row) = rows.next()? {
                merged.push(row_to_event(row)?);
            }
        }

        if options.backwards {
            merged.sort_by_key(|event| core::cmp::Reverse(event.position));
        } else {
            merged.sort_by_key(|event| event.position);
        }
        merged.dedup_by_key(|event| event.position);
        if let Some(limit) = options.limit {
            merged.truncate(limit);
        }
        Ok(merged)
    }
}

/// A `SequencePosition` as the integer SQLite stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// Applies the schema if it is not there and mints the identity once.
fn migrate(connection: &mut Connection) -> rusqlite::Result<StoreId> {
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
    let raw: Vec<u8> = transaction.query_row(
        "SELECT CAST(v AS BLOB) FROM store_meta WHERE k = ?",
        [STORE_ID_KEY],
        |row| row.get(0),
    )?;
    transaction.commit()?;

    let mut bytes = [0u8; 16];
    let take = raw.len().min(16);
    bytes[..take].copy_from_slice(&raw[..take]);
    Ok(StoreId::from_bytes(bytes))
}

/// The position violating a guard of `condition`, if one exists.
///
/// A transcription of `crates/happenstance-sqlite/src/event_store.rs:640-673`,
/// with exactly one thing added: the guard's boundary is handed down to
/// [`chunks`], so a [`Shape`] that wants it in SQL can have it.
///
/// **The Rust-side comparison stays where it is for every shape.** A bounded
/// shape returns only positions already above the boundary, so `highest >
/// boundary` is then trivially true for any row it returns and false for the
/// `NULL` it returns otherwise — the two spellings agree by construction, and no
/// shape can win by deciding less. The 89 rules are what actually check that.
fn evaluate(
    connection: &Connection,
    shape: Shape,
    condition: &AppendCondition,
) -> rusqlite::Result<Option<SequencePosition>> {
    for guard in condition.guards() {
        let selectivity = Selectivity::read_for(connection, &guard.query)?;
        let boundary = guard.after.map_or(0, as_i64);
        let plan = chunks(
            shape,
            &guard.query,
            &selectivity,
            SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
            boundary,
        );

        let mut highest: Option<i64> = None;
        for (matched, params) in plan {
            let chunk: Option<i64> = connection.query_row(
                &guard_sql(&matched),
                rusqlite::params_from_iter(params.iter()),
                |row| row.get(0),
            )?;
            highest = highest.max(chunk);
        }

        if let Some(highest) = highest
            && highest > boundary
        {
            return Ok(SequencePosition::new(highest.unsigned_abs()));
        }
    }
    Ok(None)
}

/// The canonical `tags` column value for a tag set — `row.rs:44-52`.
///
/// An empty set encodes to the single delimiter rather than to nothing, so that
/// "no tags" and "a tag that is the empty string" could never collide.
pub(crate) fn encode_tags(tags: &Tags) -> Vec<u8> {
    let mut out = Vec::with_capacity(tags.len() * 16 + 1);
    out.push(UNIT);
    for tag in tags {
        out.extend_from_slice(tag.as_str().as_bytes());
        out.push(UNIT);
    }
    out
}

/// Reads the canonical tag column back.
fn decode_tags(encoded: &[u8]) -> Result<Tags, ProbeStoreError> {
    encoded
        .split(|byte| *byte == UNIT)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let text = core::str::from_utf8(part)
                .map_err(|err| ProbeStoreError::Corrupt(format!("stored tag: {err}")))?;
            Tag::new(text).map_err(|err| ProbeStoreError::Corrupt(format!("stored tag: {err}")))
        })
        .collect::<Result<Vec<Tag>, _>>()
        .map(|tags| tags.into_iter().collect())
}

/// Writes every row of the batch, and returns the position of its last event.
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
                encode_tags(event.tags()),
                recorded_at.as_millis(),
            ])?;
            positions.push(connection.last_insert_rowid());
        }
    }

    write_tag_rows(connection, events, &positions)?;
    bump_cardinality(connection, events)?;

    // Bounded by the batch's own first assigned position, which is the shipped
    // form: the `origin_position IS NULL` marker alone is not a predicate a
    // planner can seek, and the unbounded spelling scans the whole `event` table
    // under the write lock.
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
/// budget.
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

/// One multi-row `event_tag` insert.
fn flush_tag_rows(connection: &Connection, buffer: &[Value], rows: usize) -> rusqlite::Result<()> {
    let tuples = (0..rows).map(|_| "(?,?,?)").collect::<Vec<_>>().join(",");
    connection.execute(
        &format!("INSERT INTO event_tag (tag, position, event_type) VALUES {tuples}"),
        rusqlite::params_from_iter(buffer.iter()),
    )?;
    Ok(())
}

/// Maintains `tag_cardinality`, which is what the selectivity lookup reads.
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

/// Rebuilds a stored row as the contract's own type.
fn row_to_event(row: &rusqlite::Row<'_>) -> Result<SequencedEvent, ProbeStoreError> {
    let position: i64 = row.get(0)?;
    let event_type: String = row.get(1)?;
    let data: Vec<u8> = row.get(2)?;
    let metadata: Option<Vec<u8>> = row.get(3)?;
    let tags: Vec<u8> = row.get(4)?;
    let origin_store: Vec<u8> = row.get(5)?;
    let origin_position: i64 = row.get(6)?;
    let recorded_at: i64 = row.get(7)?;

    let position = SequencePosition::new(position.unsigned_abs())
        .ok_or_else(|| ProbeStoreError::Corrupt("position 0 is not a valid position".to_owned()))?;
    let origin = SequencePosition::new(origin_position.unsigned_abs()).ok_or_else(|| {
        ProbeStoreError::Corrupt("origin position 0 is not a valid position".to_owned())
    })?;

    let mut store_bytes = [0u8; 16];
    let take = origin_store.len().min(16);
    store_bytes[..take].copy_from_slice(&origin_store[..take]);

    let mut event = Event::new(event_type, data)
        .map_err(|err| ProbeStoreError::Corrupt(format!("stored event type: {err}")))?
        .with_tags(decode_tags(&tags)?);
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

impl SendEventStore for ProbeStore {
    type Error = ProbeStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
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
        // Emptiness first, and above the lock — ES-20. `ConditionViolated` means
        // "rebuild and retry", so reporting it for an empty batch puts a correct
        // client into a loop that never terminates.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let recorded_at = now();
        let mut connection = self.locked();

        let transaction = connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|err| AppendError::Store(ProbeStoreError::Sqlite(err)))?;

        if let Some(condition) = condition {
            match evaluate(&transaction, self.shape, condition) {
                Ok(Some(at)) => {
                    // Rolling back rather than committing is what "a rejected
                    // append leaves the store byte-identical" means.
                    drop(transaction);
                    return Err(AppendError::ConditionViolated(ConditionViolated::at(at)));
                }
                Ok(None) => {}
                Err(err) => {
                    drop(transaction);
                    return Err(AppendError::Store(ProbeStoreError::Sqlite(err)));
                }
            }
        }

        let last = write_batch(&transaction, self.store_id, events, recorded_at)
            .map_err(|err| AppendError::Store(ProbeStoreError::Sqlite(err)))?;
        transaction
            .commit()
            .map_err(|err| AppendError::Store(ProbeStoreError::Sqlite(err)))?;
        Ok(last)
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
#[derive(Debug)]
enum Rows {
    Ready(std::vec::IntoIter<SequencedEvent>),
    Failed(Option<ProbeStoreError>),
}

impl Stream for Rows {
    type Item = Result<SequencedEvent, ProbeStoreError>;

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
