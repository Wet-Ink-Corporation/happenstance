//! SQLite-backed [`SendEventStore`].
//!
//! # Status: every event-store path is real
//!
//! Migration 1, the persisted identity, the connection settings, `append`,
//! `read`, `head` and `contains_event_id` all have real bodies, and four test
//! targets read them back out of SQLite — `tests/migration.rs`,
//! `tests/append.rs`, `tests/read.rs` and `tests/wide_query.rs`. The *types*
//! were never stubbed: the connection is a real [`rusqlite::Connection`], the
//! error enum wraps [`rusqlite::Error`], and [`SqliteReadStream`] is the state
//! machine the real read path uses. A skeleton that stubs its associated types
//! has stubbed the only part of it a type checker can disagree with.
//!
//! # Why the stream is a hand-written state machine
//!
//! `rusqlite` is synchronous. The only correct way to call it from an async
//! context is [`tokio::task::spawn_blocking`], and `spawn_blocking` **panics**
//! when there is no runtime in thread-local scope. Meanwhile
//! [`EventStore::read`](happenstance_core::EventStore::read) is deliberately
//! *not* `async` — it returns the stream at the top level so that the `Send`
//! flavour can mark the *stream* `Send` rather than merely the future that
//! produces it (ADR-0001, ADR-0008).
//!
//! Put those together and the consequence is forced: `read` runs on whatever
//! thread called it, possibly outside any runtime, so it must not spawn. The
//! spawn has to be deferred to the first `poll_next`, which by definition runs
//! under an executor. **Laziness stops being a nicety and becomes load-bearing**
//! — it is what makes the two constraints compatible at all.
//!
//! One residual risk survives that, and it is why
//! [`SqliteEventStoreError::NoRuntime`] exists: a `poll` under a *non-tokio*
//! executor (`futures::executor::block_on`, say) is still runtime-less.
//! [`tokio::runtime::Handle::try_current`] turns that from a panic into an
//! ordinary stream error, which is what a lazy stream's contract already
//! promises — failures surface as `Err` items rather than up front.
//!
//! # Intended schema
//!
//! Migration 1, exactly as [`SqliteEventStore::migrate`] applies it. The block
//! below is compared object-by-object against `sqlite_master` by
//! `tests/migration.rs::module_doc_schema_matches_sqlite_master`, because a
//! reviewer reading SQL prose against SQL code is the check that passes by
//! fatigue.
//!
//! ```sql
//! CREATE TABLE event (
//!     position        INTEGER PRIMARY KEY AUTOINCREMENT, -- monotonic, gaps allowed
//!     event_type      TEXT    NOT NULL,
//!     data            BLOB    NOT NULL,
//!     metadata        BLOB,                              -- nullable on purpose
//!     tags            BLOB    NOT NULL,                  -- canonical sorted encoding
//!     origin_store    BLOB,
//!     origin_position INTEGER,
//!     recorded_at     INTEGER NOT NULL,
//!     UNIQUE (origin_store, origin_position)
//! );
//!
//! CREATE INDEX event_type_idx ON event(event_type, position);
//!
//! CREATE TABLE event_tag (
//!     tag        TEXT    NOT NULL,
//!     position   INTEGER NOT NULL REFERENCES event(position),
//!     event_type TEXT    NOT NULL,  -- covering column, deliberately not in the key
//!     PRIMARY KEY (tag, position)
//! ) WITHOUT ROWID;
//!
//! CREATE TABLE tag_cardinality (
//!     tag    TEXT    PRIMARY KEY,
//!     events INTEGER NOT NULL
//! ) WITHOUT ROWID;
//!
//! CREATE TABLE store_meta (
//!     k TEXT PRIMARY KEY,
//!     v BLOB NOT NULL
//! ) WITHOUT ROWID;
//! ```
//!
//! Every line of it is load-bearing, and the two amendments in the middle are
//! the reason this block was rewritten rather than merely filled in:
//!
//! * **`event_type` on `event_tag` is a covering column and not part of the
//!   key.** In the key it would break the position ordering that makes a tag's
//!   range already sorted; *absent* — which is what this block published while
//!   the bodies were `todo!()` — it forces a query item constraining both type
//!   and tags into a join back to `event`, walked while the `BEGIN IMMEDIATE`
//!   write lock is held, with every other writer queued behind it. That is a
//!   conformant adapter that serialises every writer, and it passes every rule
//!   (ADR-0022 §7).
//! * **`tag_cardinality` is a requirement rather than a convenience.**
//!   Multi-tag items must be probed most-selective-tag-first and SQLite cannot
//!   supply per-value cardinality: `ANALYZE` stores an *average*, which is
//!   exactly wrong for a tag set where one value matches a third of the log and
//!   another matches one percent. Measured, a two-tag boundary costs roughly
//!   200x a single-tag one (ADR-0022 §8).
//!
//! And three that predate them:
//!
//! * **`AUTOINCREMENT` is deliberate.** It guarantees positions are never
//!   reused after a delete, which plain `rowid` does not, and the specification
//!   requires uniqueness across the store's whole lifetime. It also *permits
//!   gaps*, which is why nothing here and no rule may assume `+ 1`.
//! * **`metadata` is nullable.** `None` and `Some(<empty>)` are two values the
//!   contract keeps apart, and a store that folds them has lost one.
//! * **The `EventId` origin pair is `UNIQUE` together**, not separately: one
//!   constraint serving two jobs, the index
//!   [`contains_event_id`](SendEventStore::contains_event_id) probes and the
//!   guard that stops ingest storing one event twice.
//!
//! `store_meta` carries this store's own [`StoreId`], minted once at schema
//! creation and read back on every open, beside the schema-version marker that
//! gives migration 2 something to test against. See
//! [`SqliteEventStore::remint_identity`] for what that mint-once choice costs.

use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::vec;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, InvalidEventType, InvalidTag,
    Query, ReadOptions, RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use rusqlite::Connection;
use rusqlite::types::Value;
use tokio::runtime::{Handle, TryCurrentError};
use tokio::task::{JoinError, JoinHandle};

use crate::connection::ConnectionSettings;
use crate::query_sql::{Selectivity, match_sql};

/// How many rows one `spawn_blocking` hop fetches.
///
/// The point of paging at all is that a replay of a million events must not be
/// buffered, which is the promise [`EventStore::read`](happenstance_core::EventStore::read)
/// makes. The value is a placeholder until it is measured.
const PAGE_SIZE: usize = 512;

/// A SQLite-backed event store.
///
/// The connection lives behind a [`Mutex`] because [`rusqlite::Connection`] is
/// [`Send`] but **not** [`Sync`]: without the mutex, `&SqliteEventStore` would
/// not be `Send`, and every future in the [`SendEventStore`] flavour captures
/// `&self`. The mutex is what buys `Self: Sync`, and `Self: Sync` is what makes
/// the `Send` flavour implementable at all. It also means this adapter
/// **serialises its writers** by construction — that is the shape it is here to
/// represent, not an accident.
///
/// The [`Arc`] is not for sharing the store; it is so that a
/// [`SqliteReadStream`] can outlive the `&self` borrow that produced it, which
/// it must, because `read` is not `async` and hands the stream back to the
/// caller.
#[derive(Debug, Clone)]
pub struct SqliteEventStore {
    connection: Arc<Mutex<Connection>>,
    /// This database's incarnation, minted once at schema creation and read
    /// back on every open.
    ///
    /// Sixteen plain bytes, so it costs the `Send`/`Sync` assertions nothing —
    /// unlike a cached `rusqlite::Statement` or `Transaction<'_>`, both of which
    /// borrow the connection and are `!Send`. See [`SqliteReadStream`]'s docs.
    store_id: StoreId,
    /// The runtime a read's `spawn_blocking` hops onto, captured here rather
    /// than looked up at every poll.
    ///
    /// ADR-0022 §9's decision, and the reason is the concurrency family: its
    /// contenders are **bare OS threads** driving futures under the testkit's
    /// own park-loop `block_on`, with no tokio context anywhere at poll time. A
    /// store constructed inside the harness's runtime carries a handle out to
    /// them; `Handle::try_current()` at poll would find nothing and every read
    /// would fail as `NoRuntime` — a red family that is not about this
    /// adapter's logic.
    ///
    /// A [`Handle`] is `Clone`, `Send`, `Sync` and `Unpin`, so carrying one in
    /// the store, the cursor and the stream costs the shape assertions nothing.
    ///
    /// The rejected alternative was to run the statement inline on the calling
    /// thread when no runtime is found. It is not unsound — it is what the
    /// experiment's candidates did, and sixty-four bare threads completed
    /// correctly — but it keeps blocking work on an executor's thread whenever
    /// there *is* one, and it makes [`SqliteEventStoreError::NoRuntime`]
    /// unreachable. Under the option that won, the variant keeps a real meaning:
    /// a store both constructed *and* driven with no runtime anywhere.
    runtime: Option<Handle>,
}

/// The migration this build knows how to operate.
///
/// Persisted in `store_meta` so that migration 2 has something to test against:
/// a schema with no version marker cannot be migrated later without guessing.
pub const SCHEMA_VERSION: u32 = 1;

/// Migration 1, verbatim. Mirrored in the [module documentation](self), which
/// `tests/migration.rs` compares against `sqlite_master`.
///
/// `IF NOT EXISTS` throughout, because `open` runs `migrate` on **every**
/// connect and a fixture connects more than once onto one file.
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

/// The `store_meta` key holding this database's incarnation.
const STORE_ID_KEY: &str = "store_id";

/// The `store_meta` key holding the migration the file is at.
const SCHEMA_VERSION_KEY: &str = "schema_version";

impl SqliteEventStore {
    /// The largest `data` payload this store accepts, in **bytes of
    /// [`Event::data`]** — not of an encoded row, and not of `data` and
    /// `metadata` together.
    ///
    /// A **fact about this adapter**, not a trade: SQLite's own ceiling is near
    /// a gigabyte, and this is the number below it that keeps the conformance
    /// suite runnable, because the rule that checks a ceiling allocates it plus
    /// one byte twice per run. It is sixteen times VT-21's 65,536-byte floor.
    pub const MAX_EVENT_DATA_LEN: usize = 1_048_576;

    /// The largest number of tags on one event this store accepts.
    ///
    /// Twice VT-22's floor of 64. Every tag costs a row in `event_tag` and an
    /// upsert in `tag_cardinality`, both inside the write transaction, so the
    /// number is bounded by lock hold time rather than by storage.
    pub const MAX_TAGS_PER_EVENT: usize = 128;

    /// The largest number of events this store accepts in one append.
    ///
    /// Twice VT-24's floor of 128. At this ceiling with
    /// [`MAX_TAGS_PER_EVENT`](Self::MAX_TAGS_PER_EVENT) tags on every event, a
    /// single multi-row tag insert would bind 98,304 of SQLite's 32,766 bound
    /// parameters — which is why the insert is chunked to the parameter budget
    /// and the transaction is not.
    pub const MAX_EVENTS_PER_BATCH: usize = 256;

    /// How many index arms one prepared statement carries before the query is
    /// split across several.
    ///
    /// SQLite compiles a `UNION` of *n* arms as one compound `SELECT`, and
    /// `SQLITE_MAX_COMPOUND_SELECT` defaults to **500** terms. A `Query` bounds
    /// nothing by design — the specification requires every store to evaluate at
    /// least 128 items and puts no ceiling above that — so a wide query is
    /// **chunked and merged, never refused**: there is no `MAX_QUERY_ITEMS`
    /// anywhere in this crate and no fourth `StoreLimit` variant to report one
    /// through, because a query-item refusal is not an append outcome.
    ///
    /// The number is public so that a test can compute the boundary rather than
    /// guess at it: a merge that never executes is dead code behind a green
    /// suite, which is the failure mode this whole project exists to retire.
    pub const MAX_QUERY_ARMS_PER_STATEMENT: usize = 400;

    /// How many prepared statements one page of `query` will take.
    ///
    /// `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT)`, and never zero.
    /// [`Query::all`] is one arm, because it goes straight to the `event` table
    /// rather than through the tag index.
    ///
    /// This is the seam a test uses to observe that a wide query genuinely
    /// crossed the chunk boundary. It is the same function the read path itself
    /// plans with, so it cannot drift from the behaviour it reports.
    #[must_use]
    pub fn planned_statement_count(query: &Query) -> usize {
        crate::query_sql::statement_count(query, Self::MAX_QUERY_ARMS_PER_STATEMENT)
    }

    /// Wraps an already-open connection onto an **already-migrated** database.
    ///
    /// The caller is responsible for having applied the schema — use
    /// [`open`](Self::open) to have that done — and this constructor is
    /// fallible because it is not merely taking the caller's word for it: the
    /// store carries the database's incarnation, so it reads it back here. A
    /// `StoreId`-less store would mint every [`EventId`] under a zero origin,
    /// which has no error path and no observable symptom.
    ///
    /// The connection is used as it is given. It is **not** reconfigured, so a
    /// caller reaching this constructor directly should have opened it through
    /// [`crate::connection::open_configured`].
    ///
    /// # Errors
    ///
    /// * [`SqliteEventStoreError::Sqlite`] if the database cannot be queried —
    ///   which for an unmigrated database is the missing `store_meta` table.
    /// * [`SqliteEventStoreError::MissingIdentity`] if the schema is present and
    ///   the incarnation row is not.
    /// * [`SqliteEventStoreError::MalformedIdentity`] if that row is not the
    ///   sixteen bytes a [`StoreId`] is.
    /// * [`SqliteEventStoreError::UnsupportedSchemaVersion`] if the file was
    ///   written by a newer build.
    pub fn new(connection: Connection) -> Result<Self, SqliteEventStoreError> {
        let store_id = read_identity(&connection)?;
        Ok(Self::with_store_id(connection, store_id))
    }

    /// Pairs a connection with an incarnation already read off it.
    ///
    /// The runtime handle is captured **here**, at construction, because this is
    /// the point at which a caller is most likely to be inside one — and the
    /// concurrency family's contenders, which are bare OS threads, never are.
    fn with_store_id(connection: Connection, store_id: StoreId) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            store_id,
            runtime: Handle::try_current().ok(),
        }
    }

    /// Opens (creating if absent) a store at `path` and applies the schema.
    ///
    /// The connection is configured before anything is written to it — WAL, a
    /// stated `synchronous`, and a finite busy timeout — through the one path
    /// [`crate::connection::open_configured`] both stores in this crate share.
    /// Migration is idempotent and safe under a concurrent open: re-opening an
    /// already-migrated file adds no schema object and mints no second
    /// incarnation.
    ///
    /// # Errors
    ///
    /// * [`SqliteEventStoreError::Sqlite`] if the file cannot be opened or
    ///   created — a missing directory, a permission refusal, a corrupt header
    ///   — or if the schema cannot be applied. Nothing partially migrated is
    ///   left behind: every statement runs inside one transaction.
    /// * [`SqliteEventStoreError::UnsupportedSchemaVersion`] if the file was
    ///   written by a build that knows a later migration. Operating on an
    ///   unknown schema is how a later migration loses data.
    /// * [`SqliteEventStoreError::MissingIdentity`] or
    ///   [`SqliteEventStoreError::MalformedIdentity`] if the schema is present
    ///   and its incarnation row is not readable. Silently minting a
    ///   replacement is the worst available behaviour — it re-issues
    ///   [`EventId`]s under a new origin for events that already exist.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteEventStoreError> {
        let mut connection = crate::connection::open_configured(path)?;
        let store_id = Self::migrate(&mut connection)?;
        Ok(Self::with_store_id(connection, store_id))
    }

    /// Opens a private in-memory store and applies the schema.
    ///
    /// A convenience for a single-handle caller, and **not** what a
    /// [`Fixture`](https://docs.rs/happenstance-testkit) should reach for: a
    /// private in-memory database is per-*connection*, so a second connect would
    /// open a second, empty database rather than a second handle onto this one.
    /// WAL has nothing to switch to here, so this connection runs under the
    /// journal mode SQLite keeps for memory databases.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteEventStoreError::Sqlite`] if SQLite refuses the
    /// connection or the schema cannot be applied, and the identity errors
    /// [`open`](Self::open) documents.
    pub fn open_in_memory() -> Result<Self, SqliteEventStoreError> {
        let mut connection = Connection::open_in_memory()?;
        crate::connection::configure(&connection)?;
        let store_id = Self::migrate(&mut connection)?;
        Ok(Self::with_store_id(connection, store_id))
    }

    /// The incarnation this store mints identities under.
    ///
    /// The same value before and after a close-and-reopen of the file, which is
    /// what makes `reopened_store_does_not_reissue_an_event_id` askable: with
    /// the incarnation held fixed, the *position* is what must never repeat, and
    /// `AUTOINCREMENT` is the mechanism.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// What this handle's connection is actually running under.
    ///
    /// The values are read off the live connection rather than restated from
    /// the constants that set them: SQLite silently accepts a pragma it does not
    /// recognise, so a statement that executed is not a setting that took.
    ///
    /// # Errors
    ///
    /// * [`SqliteEventStoreError::ConnectionPoisoned`] if another thread
    ///   panicked while holding the connection.
    /// * [`SqliteEventStoreError::Sqlite`] if the connection cannot answer.
    pub fn settings(&self) -> Result<ConnectionSettings, SqliteEventStoreError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        Ok(ConnectionSettings::read_back(&connection)?)
    }

    /// Replaces the database's persisted incarnation with a freshly minted one.
    ///
    /// # When a deployment invokes this
    ///
    /// **After restoring this file from a backup, or after copying it.** This
    /// adapter mints its incarnation once, at schema creation, and reads it back
    /// on every open — the mechanism VT-6 permits *only* for an adapter that can
    /// detect its state was restored or cloned, **or** whose deployment is
    /// documented to invoke a re-mint. A SQLite file copied from Friday's backup
    /// is undetectable from inside SQLite, so this operation is the second half
    /// of that permission rather than a convenience: without it, mint-once here
    /// would not be legitimate and this adapter would owe a fresh incarnation on
    /// every open.
    ///
    /// Two copies of one file that keep one incarnation are two stores minting
    /// [`EventId`]s that collide — a failure with no error path and no
    /// observable symptom until a replication peer sees the same identity twice.
    ///
    /// Run it with nothing else holding the database open, and *before* the
    /// restored copy accepts its first append: events already in the file keep
    /// the origin they were stamped with, which is correct — they were written
    /// by the incarnation that is being retired.
    ///
    /// # Errors
    ///
    /// * [`SqliteEventStoreError::Sqlite`] if the file cannot be opened, or if
    ///   another connection holds the write lock for longer than the configured
    ///   busy timeout.
    /// * [`SqliteEventStoreError::UnsupportedSchemaVersion`] if the file was
    ///   written by a newer build — a re-mint is refused rather than applied
    ///   blind to a schema this build does not know.
    /// * [`SqliteEventStoreError::MissingIdentity`] or
    ///   [`SqliteEventStoreError::MalformedIdentity`] if the freshly written row
    ///   cannot be read back, in which case nothing is committed.
    pub fn remint_identity(path: impl AsRef<Path>) -> Result<StoreId, SqliteEventStoreError> {
        let mut connection = crate::connection::open_configured(path)?;
        // Migrating first is what makes this operation total rather than
        // conditional on the caller having opened the store already.
        Self::migrate(&mut connection)?;

        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        transaction.execute(
            "UPDATE store_meta SET v = randomblob(16) WHERE k = ?",
            [STORE_ID_KEY],
        )?;
        let minted = read_identity(&transaction)?;
        transaction.commit()?;
        Ok(minted)
    }

    /// The three ceilings, checked before any transaction opens.
    ///
    /// # Errors
    ///
    /// Returns `AppendError::ExceedsStoreLimit` naming which ceiling was
    /// exceeded and by what magnitude. Never `AppendError::Store`: a caller that
    /// cannot tell "this will never fit here, park it and tell a human" from
    /// "the disk is full, retry" has to guess, and a sync runner that guesses
    /// wrong drops an event permanently.
    fn check_ceilings(events: &[Event]) -> Result<(), AppendError<SqliteEventStoreError>> {
        if events.len() > Self::MAX_EVENTS_PER_BATCH {
            return Err(AppendError::ExceedsStoreLimit {
                limit: happenstance_core::StoreLimit::EventsPerBatch,
                len: events.len(),
            });
        }
        for event in events {
            if event.data().len() > Self::MAX_EVENT_DATA_LEN {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: happenstance_core::StoreLimit::EventDataLen,
                    len: event.data().len(),
                });
            }
            if event.tags().len() > Self::MAX_TAGS_PER_EVENT {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: happenstance_core::StoreLimit::TagsPerEvent,
                    len: event.tags().len(),
                });
            }
        }
        Ok(())
    }

    /// Everything that happens inside the one `BEGIN IMMEDIATE`.
    ///
    /// In order: every guard probed against the state the store already held —
    /// which is free, because no row of this batch exists yet — then the rows
    /// inserted, then the identity stamped, then commit. A guard violation drops
    /// the transaction without committing, which is what "a rejected append
    /// leaves the file byte-identical" means.
    fn append_locked(
        connection: &mut Connection,
        store_id: StoreId,
        events: &[Event],
        condition: Option<&AppendCondition>,
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, AppendError<SqliteEventStoreError>> {
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

    /// Applies the schema in the module documentation, and returns the
    /// incarnation the file carries afterwards.
    ///
    /// Called on **every** connect, so it is a bounded number of
    /// `IF NOT EXISTS` statements and one read-back — no `ANALYZE`, no table
    /// scan, no schema introspection loop.
    ///
    /// # Why one `BEGIN IMMEDIATE`, and why the read-back is the load-bearing
    /// half
    ///
    /// Two connects onto a *fresh* file can race, and both would otherwise mint
    /// an incarnation. `INSERT OR IGNORE` makes only one of them land; reading
    /// the row back **inside the same transaction** is what makes the loser
    /// adopt the winner's value rather than keep the one it generated. A store
    /// that skipped the read-back would hold two different `StoreId`s for one
    /// file and every reopen rule downstream would go non-deterministic in a way
    /// that looks like flakiness.
    fn migrate(connection: &mut Connection) -> Result<StoreId, SqliteEventStoreError> {
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        transaction.execute_batch(MIGRATION_1)?;
        transaction.execute(
            "INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, ?)",
            rusqlite::params![SCHEMA_VERSION_KEY, SCHEMA_VERSION],
        )?;
        // `randomblob` is SQLite's own CSPRNG. Sixteen bytes of entropy from the
        // driver already in the tree, weighed against adding a crate for them
        // and chosen deliberately — `happenstance-core` mints nothing because it
        // is `no_std`-capable and has no entropy source, so the adapter must.
        transaction.execute(
            "INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, randomblob(16))",
            [STORE_ID_KEY],
        )?;

        let store_id = read_identity(&transaction)?;
        transaction.commit()?;
        Ok(store_id)
    }
}

/// How many bound parameters one statement may carry.
///
/// SQLite's `SQLITE_MAX_VARIABLE_NUMBER` is 32,766. This sits below it with
/// headroom rather than at it, because the arithmetic that matters is done once
/// in `happenstance-core`'s `limits.rs` and an adapter that binds one extra
/// parameter per row should not be within rounding distance of the wall.
const PARAMETER_BUDGET: usize = 30_000;

/// Bound parameters one `event_tag` row costs.
const TAG_ROW_PARAMETERS: usize = 3;

/// Milliseconds since the Unix epoch, stamped **once**, at append.
///
/// ADR-0014's whole point is that the stamp is a property of the append rather
/// than of the read: a store that re-stamps on reopen hands every auditor the
/// time of the last restart.
fn now() -> RecordedAt {
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
fn store_error(error: impl Into<SqliteEventStoreError>) -> AppendError<SqliteEventStoreError> {
    AppendError::Store(error.into())
}

/// The position violating a guard of `condition`, if one exists.
///
/// **One `SELECT max(position)` per guard**, which is ADR-0022's decision and
/// the one that measured fastest on the *rejection* path — the path a DCB
/// command loop takes every time it loses a race. The insight it trades on: a
/// guard asks *"is there anything matching after this boundary"*, which reads
/// like an existence question and is an inequality on the **highest** matching
/// position. One `max()` answers both halves at once — whether the condition is
/// violated, and by which event — so the rejection path needs no second query,
/// where an `EXISTS` probe would need a follow-up `min(position)` to name the
/// conflict.
///
/// `after: None` is a boundary of zero, because positions start at one. Guards
/// are checked in order and the first violation ends the evaluation.
fn evaluate(
    connection: &Connection,
    condition: &AppendCondition,
) -> rusqlite::Result<Option<SequencePosition>> {
    for guard in condition.guards() {
        let selectivity = Selectivity::read_for(connection, &guard.query)?;
        let mut params: Vec<Value> = Vec::new();
        let matched = match_sql(&guard.query, &selectivity, &mut params);

        let highest: Option<i64> = connection.query_row(
            &format!("SELECT max(position) FROM ({matched})"),
            rusqlite::params_from_iter(params.iter()),
            |row| row.get(0),
        )?;

        let boundary = guard.after.map_or(0, as_i64);
        if let Some(highest) = highest
            && highest > boundary
        {
            return Ok(SequencePosition::new(highest.unsigned_abs()));
        }
    }
    Ok(None)
}

/// Writes every row of the batch, and returns the position of its last event.
///
/// The `event` rows go in one at a time so that each one's assigned position is
/// read from `last_insert_rowid()` rather than inferred: `AUTOINCREMENT` permits
/// gaps and nothing may assume `+ 1`. The `event_tag` rows are where the
/// parameter pressure actually is — at the declared ceilings a single multi-row
/// statement would bind 98,304 of SQLite's 32,766 — so they are batched into
/// statements sized from the budget, and the buffer is bounded by the chunk
/// rather than by the batch.
///
/// **Chunking the statements is not chunking the transaction.** Committing
/// between chunks would produce a partially applied batch, which is the one way
/// to fail atomicity that a single-threaded read-back would happily confirm.
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

    // One statement at the end of the batch rather than a value bound per row:
    // a locally appended event's identity is *this store's incarnation paired
    // with the position it was just given*, and that position is not known until
    // the row exists. `origin_position IS NULL` is the marker, and the
    // `UNIQUE (origin_store, origin_position)` constraint tolerates it because
    // SQLite treats NULLs as distinct — which is what lets a multi-row batch
    // stamp itself without tripping it.
    connection.execute(
        "UPDATE event SET origin_store = ?, origin_position = position \
         WHERE origin_position IS NULL",
        [&store_id.to_bytes()[..]],
    )?;

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

/// Maintains `tag_cardinality`, which is what orders a multi-tag probe.
///
/// Migration 1 creates the table; this is what writes to it. A table created and
/// never updated silently restores the plan the schema amendment was made to
/// avoid, and nothing in the conformance suite would notice.
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

/// Reads the schema version and the incarnation back out of `store_meta`.
///
/// Refuses a file a newer build wrote, and refuses to invent an incarnation for
/// a migrated file that has lost one — silently minting a replacement re-issues
/// [`EventId`]s under a new origin for events that already exist, which is the
/// failure mode with no error path and no observable symptom.
fn read_identity(connection: &Connection) -> Result<StoreId, SqliteEventStoreError> {
    let version: u32 = connection.query_row(
        "SELECT CAST(v AS INTEGER) FROM store_meta WHERE k = ?",
        [SCHEMA_VERSION_KEY],
        |row| row.get(0),
    )?;
    if version > SCHEMA_VERSION {
        return Err(SqliteEventStoreError::UnsupportedSchemaVersion {
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
            rusqlite::Error::QueryReturnedNoRows => SqliteEventStoreError::MissingIdentity,
            other => SqliteEventStoreError::Sqlite(other),
        })?;

    let bytes: [u8; 16] = raw
        .as_slice()
        .try_into()
        .map_err(|_| SqliteEventStoreError::MalformedIdentity { len: raw.len() })?;
    Ok(StoreId::from_bytes(bytes))
}

/// How [`SqliteEventStore`] fails.
///
/// Every variant names something `rusqlite` or the surrounding runtime can
/// actually produce. Append-condition violations are **not** here: they travel
/// through [`AppendError::ConditionViolated`], so a caller can tell "rebuild the
/// decision model and retry" from "something broke" without knowing which
/// adapter it holds.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqliteEventStoreError {
    /// The driver failed: I/O, `SQLITE_BUSY`, a constraint, a bad statement.
    #[error("SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A thread panicked while holding the connection mutex.
    ///
    /// Carried as a unit variant rather than wrapping
    /// [`std::sync::PoisonError`], because that type is generic over the guard
    /// and the guard borrows the connection — it is neither `'static` nor
    /// `Send`, and [`EventStore::Error`](happenstance_core::EventStore::Error)
    /// requires `'static`.
    #[error("the SQLite connection mutex was poisoned by a panicking thread")]
    ConnectionPoisoned,

    /// The blocking task carrying a query panicked or was cancelled.
    #[error("the blocking SQLite task did not complete: {0}")]
    Worker(#[from] JoinError),

    /// A stream was polled outside a tokio runtime, so no blocking task could
    /// be spawned.
    ///
    /// See the [module documentation](self) for why this is an error rather
    /// than the panic `spawn_blocking` would otherwise raise.
    #[error("no tokio runtime is available to run the blocking SQLite query: {0}")]
    NoRuntime(#[from] TryCurrentError),

    /// A stored row carried a position SQLite accepted and the contract does
    /// not: [`SequencePosition`] wraps a `NonZeroU64`, so zero and negatives
    /// are unrepresentable.
    #[error("stored position {0} is not a valid sequence position")]
    InvalidPosition(i64),

    /// A stored row carried an event type that no longer validates.
    #[error("stored event type is invalid: {0}")]
    StoredEventType(#[from] InvalidEventType),

    /// A stored row carried a tag that no longer validates.
    #[error("stored tag is invalid: {0}")]
    StoredTag(#[from] InvalidTag),

    /// The file was written by a build that knows a later migration.
    ///
    /// Refusing is the point: operating on an unknown schema is how a later
    /// migration loses data, and the version marker exists precisely so that
    /// migration 2 has something to test against.
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
    ///
    /// A **distinct** variant rather than a driver error, because the honest
    /// alternative — minting a replacement — re-issues [`EventId`]s under a new
    /// origin for events that already exist, and that failure has no error path
    /// and no observable symptom until a replication peer sees one identity
    /// twice.
    #[error(
        "the database has been migrated but carries no store identity; minting \
         a replacement would re-issue event identities for events that already \
         exist"
    )]
    MissingIdentity,

    /// The incarnation row is present and is not sixteen bytes.
    #[error("the stored store identity is {len} bytes, and a StoreId is sixteen")]
    MalformedIdentity {
        /// How many bytes the row actually held.
        len: usize,
    },

    /// A stored row carries no event identity.
    ///
    /// Unreachable if the write path is correct — `append` stamps the origin
    /// pair onto every row of the batch before it commits — and reported rather
    /// than panicked so that a schema mistake surfaces as a failing rule naming
    /// this store rather than as a crash naming the suite.
    #[error("the event at position {position} carries no origin identity")]
    UnstampedEvent {
        /// Where the unstamped row sits.
        position: SequencePosition,
    },

    /// The canonical tag column of a stored row is not UTF-8.
    #[error("a stored tag column is not valid UTF-8")]
    CorruptTags,
}

impl SendEventStore for SqliteEventStore {
    type Error = SqliteEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        // Nothing is executed here on purpose. See the module documentation:
        // `read` may legally be called with no runtime in scope, and
        // `spawn_blocking` panics there. No lock is taken either — a guard
        // parked in the cursor would deadlock the next `append` through this
        // same handle.
        SqliteReadStream {
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

    /// Appends `events`, refusing the write if `condition` is violated.
    ///
    /// The body is mostly *order*, and the order is the specification rather
    /// than a style: two of its four steps decide what happens **before** any
    /// SQL runs, and the fourth decides that everything remaining happens inside
    /// exactly one `BEGIN IMMEDIATE` transaction — the write lock taken before
    /// the condition is read and held to commit, so the snapshot the guard sees
    /// is the snapshot the insert writes into and no second writer can fit
    /// between the two halves.
    ///
    /// A probe followed by an unrelated insert is the wrong implementation this
    /// whole port exists to reject: it passes every single-threaded rule
    /// forever, and fails only when two writers decide from the same state.
    ///
    /// Returns the position assigned to the **last event of this batch**, in
    /// slice order — never the store head, which is a different number the
    /// moment another connection commits.
    ///
    /// # Errors
    ///
    /// Named by condition rather than by type:
    ///
    /// * the batch was empty — decided first, before the condition is read,
    ///   because *rebuild the decision model and retry* is not advice a caller
    ///   can act on for an empty batch;
    /// * one event's `data` was larger than
    ///   [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN), one event carried
    ///   more tags than [`MAX_TAGS_PER_EVENT`](Self::MAX_TAGS_PER_EVENT), or the
    ///   batch held more events than
    ///   [`MAX_EVENTS_PER_BATCH`](Self::MAX_EVENTS_PER_BATCH) — each reported as
    ///   a capacity refusal carrying the magnitude that exceeded it, never as a
    ///   store failure and never by truncating, so that a caller can tell *this
    ///   will never fit here* from *the disk is full, retry*;
    /// * a guard of the condition matched an event strictly after its boundary,
    ///   in which case the transaction rolls back and the file is unchanged;
    /// * the driver failed — including `SQLITE_BUSY` after the configured busy
    ///   timeout has genuinely elapsed, which is contention reported honestly
    ///   rather than a condition violation;
    /// * another thread panicked while holding this store's connection.
    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Step 1. An empty batch is the caller's own bug, and it is decided
        // above everything else. ES-20 is not a stylistic ordering: a caller
        // whose retry loop branches on `is_condition_violated` and receives
        // `ConditionViolated` for an empty batch never terminates, because an
        // empty batch will still be empty next time.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // Step 2. The declared ceilings, still before any transaction exists.
        // A caller must be able to learn a value will never fit *here* without
        // the store having to try, which is what keeps a quarantine path open
        // for a sync runner.
        Self::check_ceilings(events)?;

        let recorded_at = now();
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| AppendError::Store(SqliteEventStoreError::ConnectionPoisoned))?;

        // Steps 3 and 4 are one transaction, and that is the whole of the
        // atomicity claim.
        Self::append_locked(
            &mut connection,
            self.store_id,
            events,
            condition,
            recorded_at,
        )
    }

    /// The highest position any reader can currently observe, or `None` on an
    /// empty store.
    ///
    /// Asked of the database **every time**, and that is the whole of it. What
    /// this must never become is a field the store caches and `append` updates:
    /// one file backs several handles, so a second connection would then report
    /// a head that predates the first connection's commit — the stale head ES-30
    /// exists to reject, and the defect a single-handle test cannot see.
    ///
    /// # Errors
    ///
    /// * another thread panicked while holding this store's connection;
    /// * the driver failed, or a stored position is not one the contract can
    ///   represent.
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        // `NULL` on an empty table is the `None` arm, not an error.
        let highest: Option<i64> =
            connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
        Ok(highest.and_then(|value| SequencePosition::new(value.unsigned_abs())))
    }

    /// Whether this store already holds the event `id` names.
    ///
    /// The probe seeks the `UNIQUE (origin_store, origin_position)` index
    /// migration 1 created — one constraint serving two jobs: this lookup, and
    /// the guard that keeps ingest from storing one event twice.
    ///
    /// # Errors
    ///
    /// * another thread panicked while holding this store's connection;
    /// * the driver failed.
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
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

/// The stream [`SqliteEventStore::read`](SendEventStore::read) returns.
///
/// # Why this type is `Send`
///
/// Nothing here is `Send` by accident, and each piece earns it separately:
///
/// * the cursor holds `Arc<Mutex<Connection>>`, and `Mutex<T>: Sync` whenever
///   `T: Send` — so wrapping the `!Sync` [`Connection`] is what makes the whole
///   cursor `Send`;
/// * [`JoinHandle<T>`] is `Send` when `T` is, and `T` here is the cursor plus a
///   `Result<Page, SqliteEventStoreError>`, whose error wraps `rusqlite::Error`
///   ([`Send`] + [`Sync`]) and [`JoinError`];
/// * the drain state holds `vec::IntoIter<SequencedEvent>`, `Send` because
///   [`SequencedEvent`] is.
///
/// No `rusqlite` handle that borrows the connection — `Statement`, `Rows`,
/// `Transaction` — ever appears in a field, and that is the load-bearing part:
/// all three are `!Send`, so holding one across the `poll_next` boundary would
/// cost the stream its `Send`-ness and with it the `SendEventStore` impl.
/// Confining them to the inside of the blocking closure is not a style choice.
///
/// The type is also [`Unpin`] — every field is — so `poll_next` needs no pin
/// projection and no `unsafe`, which matters because `unsafe_code` is
/// `forbid`den workspace-wide.
#[derive(Debug)]
pub struct SqliteReadStream {
    state: ReadState,
}

/// Where a [`SqliteReadStream`] is in its life.
#[derive(Debug)]
enum ReadState {
    /// No query in flight; the next poll spawns one.
    Idle(Box<ReadCursor>),
    /// A blocking fetch is running on a `spawn_blocking` thread.
    Fetching(JoinHandle<FetchOutcome>),
    /// Yielding rows already fetched, with the cursor parked for the next page.
    Draining {
        cursor: Box<ReadCursor>,
        rows: vec::IntoIter<SequencedEvent>,
    },
    /// Terminal: exhausted, or errored and not resumable.
    Done,
}

/// What one blocking hop hands back: the cursor it borrowed, and its result.
///
/// The cursor makes the round trip because `spawn_blocking` demands a `'static`
/// closure, so the only way to mutate it on the blocking thread is to move it
/// there and back.
type FetchOutcome = (Box<ReadCursor>, Result<Page, SqliteEventStoreError>);

/// One page of rows, plus whether the query is spent.
#[derive(Debug)]
struct Page {
    rows: Vec<SequencedEvent>,
    exhausted: bool,
}

/// ADR-0011's position ceiling, and whether it has been taken yet.
///
/// Three states rather than an `Option<Option<SequencePosition>>`, because the
/// two `None`s mean genuinely different things and conflating them is how the
/// registered defect gets written: *not sampled yet* is a thing to do, and
/// *sampled, and the store was empty* is an answer.
#[derive(Debug, Clone, Copy)]
enum Ceiling {
    /// No statement has run yet. The next `fetch_page` takes the sample.
    Unsampled,
    /// Nothing was in the store when the sample was taken, so this read is
    /// spent. **Not an error**: it is the state every adapter is in before it
    /// works, and computing arithmetic on it is the defect
    /// `reading_an_empty_store_yields_nothing` exists to reject.
    Empty,
    /// Every statement of this read is bounded at or below this position.
    At(SequencePosition),
}

/// Everything the blocking thread needs to fetch the next page.
#[derive(Debug)]
struct ReadCursor {
    connection: Arc<Mutex<Connection>>,
    /// The runtime the store captured, carried so that every hop of this read
    /// uses the same one. See [`SqliteEventStore`]'s field of the same name.
    runtime: Option<Handle>,
    query: Query,
    options: ReadOptions,
    /// Where the next page resumes, **inclusive** — the same sense as
    /// [`ReadOptions::from`], which is what seeds it.
    ///
    /// The name matters. It was `resume_after` and it was seeded from an
    /// *inclusive* `from` and then advanced to `last.position`, which is two
    /// different senses in one field: page two would have re-read the last row
    /// of page one, once per page boundary. `AppendCondition::after` is the
    /// exclusive one in this contract and `ReadOptions::from` is the inclusive
    /// one, and they sit two types apart — mixing them is the easiest mistake
    /// in the port and this field made it. The multi-page criteria in
    /// `tests/read.rs` seed past `2 × PAGE_SIZE` rows precisely so that a return
    /// of that bug is visible.
    resume_from: Option<SequencePosition>,
    /// What is left of [`ReadOptions::limit`], or `None` for unlimited.
    remaining: Option<usize>,
    /// ADR-0011's ceiling: sampled no later than the first poll, never
    /// re-sampled.
    ceiling: Ceiling,
    finished: bool,
}

impl ReadCursor {
    /// Takes ADR-0011's position ceiling, once, and does nothing thereafter.
    ///
    /// # Why this runs on the polling thread rather than on the blocking one
    ///
    /// ES-11 says the sample is taken **no later than the first poll**, and that
    /// is stricter than it looks: a sample taken inside the `spawn_blocking`
    /// hop is taken *after* the first poll returned, so a caller that polls once
    /// — which is legal, and may legitimately answer `Pending` — and then
    /// appends can have its append land **before** the sample. The event is then
    /// below the ceiling and the read observes it.
    ///
    /// That is not theoretical and it is not flakiness. It is exactly what
    /// `read_result_is_stable_under_concurrent_append` and
    /// `query_items_share_one_snapshot` were written to catch, and this adapter
    /// failed both intermittently — roughly one run in two — until the sample
    /// moved here.
    ///
    /// The cost is one `SELECT max(position)` on the executor's thread. It is
    /// an O(1) seek to the end of an integer primary key, and it is the same
    /// lock `append` and `head` already take synchronously, so it adds no shape
    /// this crate did not already have.
    fn sample_ceiling(&mut self) -> Result<(), SqliteEventStoreError> {
        if !matches!(self.ceiling, Ceiling::Unsampled) {
            return Ok(());
        }
        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        let highest: Option<i64> =
            connection.query_row("SELECT max(position) FROM event", [], |row| row.get(0))?;
        self.ceiling = highest
            .and_then(|value| SequencePosition::new(value.unsigned_abs()))
            .map_or(Ceiling::Empty, Ceiling::At);
        Ok(())
    }

    /// Runs one page's worth of SQL. Called only on a blocking thread.
    ///
    /// # The ceiling, and why one `max(position)` is a snapshot
    ///
    /// ADR-0011 requires an adapter issuing more than one statement per `read`
    /// to capture a position ceiling no later than the first poll and bound
    /// every later statement by it. At `PAGE_SIZE = 512` that is every log over
    /// 512 events, so it is not a corner.
    ///
    /// It is sound because position order **is** visibility order (ADR-0013):
    /// nothing that commits after *H* is captured can ever land at or below *H*,
    /// which is what makes one cheap `max(position)` equivalent to a snapshot
    /// and why ES-11 and ES-12 reduce to ES-10 plus a ceiling. A store that
    /// re-samples per page instead grows under the caller's feet, and the
    /// consequence is not a wrong read — it is an **accepted append that should
    /// have been rejected**, because the caller derives its condition's boundary
    /// from the maximum position the read observed, and that maximum sits above
    /// an event the read silently missed.
    ///
    /// Every item of one query shares this one predicate, which is how ES-12 is
    /// discharged. There is deliberately no second mechanism for it.
    fn fetch_page(&mut self) -> Result<Page, SqliteEventStoreError> {
        // A no-op if `poll_next` already took it, which it always has — the
        // sample is not allowed to wait for this thread. See `sample_ceiling`.
        self.sample_ceiling()?;

        let connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;

        let Ceiling::At(ceiling) = self.ceiling else {
            return Ok(Page {
                rows: Vec::new(),
                exhausted: true,
            });
        };

        let budget = self.remaining.map_or(PAGE_SIZE, |left| left.min(PAGE_SIZE));
        if budget == 0 {
            return Ok(Page {
                rows: Vec::new(),
                exhausted: true,
            });
        }

        let selectivity = Selectivity::read_for(&connection, &self.query)?;

        // **Chunk and merge, never refuse.** A `Query` bounds nothing by design;
        // SQLite compiles a `UNION` of n arms as one compound `SELECT` and stops
        // at `SQLITE_MAX_COMPOUND_SELECT`, which is 500 by default. An adapter
        // that returned an error at its own pushdown limit would be inventing a
        // refusal the contract has no way to report, so a wide query becomes
        // `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT)` statements merged here.
        //
        // Every chunk statement is **identical in shape** — same ceiling, same
        // `resume_from`, same `to`, same direction, same page budget — which is
        // what makes merging them sound rather than approximate.
        let plan = crate::query_sql::chunks(
            &self.query,
            &selectivity,
            SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT,
        );
        let mut merged: Vec<SequencedEvent> = Vec::with_capacity(budget);

        for (matched, arm_params) in plan {
            let mut params = arm_params;
            let columns = crate::row::COLUMNS;
            let mut sql = format!("SELECT {columns} FROM event WHERE position IN ({matched})");

            // `resume_from` is inclusive in both directions; which side of the
            // position order it sits on is what `backwards` decides. Under
            // `backwards`, `from` stays the *starting* (higher) bound and `to`
            // the stopping (lower) one — they swap roles in position order, not
            // in meaning. Copying `position <= ?` into the descending branch is
            // correct forwards, passes two of the three read-bound rules, and
            // returns the oldest events where the newest were asked for.
            //
            // It is applied **per chunk and per hop**, never carried as
            // per-chunk state across a hop: `advance()` folds only the *merged*
            // page back into `resume_from`, which is what stops the historical
            // `resume_after` bug returning in a new disguise.
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

            // The ceiling **composes** with `to` rather than replacing it, and
            // it is the same clause in both directions because *H* is an upper
            // bound either way: forwards it tightens the stopping end,
            // backwards it tightens the starting one. Every chunk carries it,
            // which is how ES-12 survives a read becoming multi-statement.
            sql.push_str(" AND position <= ?");
            params.push(Value::Integer(as_i64(ceiling)));

            sql.push_str(if self.options.backwards {
                " ORDER BY position DESC"
            } else {
                " ORDER BY position ASC"
            });

            // The page budget, which is `min(remaining, PAGE_SIZE)` and is an
            // implementation detail *beneath* `ReadOptions::limit` — never a
            // limit applied per page, and never one applied per query item. A
            // chunk may return at most this many rows, and the merged top
            // `budget` is a subset of the union of the per-chunk tops, so
            // bounding each one loses nothing.
            sql.push_str(" LIMIT ?");
            params.push(Value::Integer(i64::try_from(budget).unwrap_or(i64::MAX)));

            let mut statement = connection.prepare(&sql)?;
            let mut rows = statement.query(rusqlite::params_from_iter(params.iter()))?;
            while let Some(row) = rows.next()? {
                merged.push(crate::row::to_event(row)?);
            }
        }

        // The merge, and the two things it has to get right: one order for the
        // whole page regardless of how the items were partitioned, and each
        // event exactly once however many arms matched it.
        if self.options.backwards {
            merged.sort_by_key(|event| core::cmp::Reverse(event.position));
        } else {
            merged.sort_by_key(|event| event.position);
        }
        merged.dedup_by_key(|event| event.position);

        // Computed **before** truncation. Every chunk returned fewer rows than
        // the budget exactly when the merged set is short of it: a chunk's rows
        // are already distinct, so a chunk that filled its budget puts that many
        // distinct positions into the merge.
        let exhausted = merged.len() < budget;
        merged.truncate(budget);

        Ok(Page {
            rows: merged,
            exhausted,
        })
    }

    /// Folds a fetched page back into the cursor's position and budget.
    ///
    /// `resume_from` stays inclusive, so it must step *strictly past* the last
    /// row — and "past" is direction-dependent, which is why this is not a
    /// `+ 1`. Running out of positions in either direction means the log has no
    /// more rows that way, so the cursor is spent rather than wrapped.
    fn advance(&mut self, page: &Page) {
        let mut exhausted_by_position = false;
        if let Some(last) = page.rows.last() {
            let next = if self.options.backwards {
                // No `SequencePosition::prev`: positions are `NonZeroU64`, so
                // stepping below `FIRST` is the same fact as being spent.
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

impl Stream for SqliteReadStream {
    type Item = Result<SequencedEvent, SqliteEventStoreError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Safe without projection because `Self: Unpin`; see the type's docs.
        let this = Pin::into_inner(self);

        loop {
            // Taking the state by value is what lets the cursor be *moved* into
            // the `'static` closure `spawn_blocking` demands. `Done` is the
            // right placeholder: every arm either restores a live state or is
            // genuinely terminal.
            match std::mem::replace(&mut this.state, ReadState::Done) {
                ReadState::Idle(mut cursor) => {
                    if cursor.finished {
                        return Poll::Ready(None);
                    }
                    // ES-11's sample, taken **on this thread, before the spawn**.
                    // Deferring it into the blocking hop would put it after this
                    // poll returned, and a caller that polls once and then
                    // appends would see its own later event. See
                    // `ReadCursor::sample_ceiling` for the failing runs that
                    // moved this line.
                    if let Err(err) = cursor.sample_ceiling() {
                        return Poll::Ready(Some(Err(err)));
                    }
                    // The deferred spawn. This is the line that could not have
                    // been written inside `read`.
                    //
                    // The handle captured at construction is preferred, and
                    // `Handle::try_current()` is the fallback — ADR-0022 §9's
                    // decision, and the reason is that the concurrency family's
                    // contenders are bare OS threads with no tokio context at
                    // poll time. `NoRuntime` keeps a real meaning under it: it
                    // is reachable only for a store both constructed *and*
                    // driven with no runtime anywhere.
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
