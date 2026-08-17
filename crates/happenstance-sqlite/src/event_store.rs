//! SQLite-backed [`SendEventStore`].
//!
//! # Status: the schema is real; the port's bodies are landing
//!
//! Migration 1, the persisted identity and the connection settings have real
//! bodies and a test target that reads them back out of SQLite. The port's four
//! methods are still `todo!()`. The *types* were never stubbed: the connection
//! is a real [`rusqlite::Connection`], the error enum wraps
//! [`rusqlite::Error`], and [`SqliteReadStream`] is the state machine the real
//! read path uses. A skeleton that stubs its associated types has stubbed the
//! only part of it a type checker can disagree with.
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
    AppendCondition, AppendError, Event, EventId, InvalidEventType, InvalidTag, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use rusqlite::Connection;
use tokio::runtime::{Handle, TryCurrentError};
use tokio::task::{JoinError, JoinHandle};

use crate::connection::ConnectionSettings;

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
    fn with_store_id(connection: Connection, store_id: StoreId) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            store_id,
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
        // `spawn_blocking` panics there.
        SqliteReadStream {
            state: ReadState::Idle(Box::new(ReadCursor {
                connection: Arc::clone(&self.connection),
                query: query.clone(),
                options,
                resume_from: options.from,
                remaining: options.limit,
                finished: false,
            })),
        }
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        todo!("SQLite event store: append")
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // `SELECT max(position) FROM event`, on a blocking thread like every
        // other statement here, and asked of the database *every time*. What it
        // must not become is a field this store caches and `append` updates:
        // one file backs several handles, so a second connection would then
        // report a head that predates the first connection's commit — the stale
        // head ES-30 exists to reject. The row is `NULL` on an empty table,
        // which is the `None` arm rather than an error.
        todo!("SQLite event store: head")
    }

    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
        // `SELECT 1 FROM event WHERE origin_store = ? AND origin_position = ?
        // LIMIT 1`. Migration 1 creates those two columns and constrains them
        // `UNIQUE` **together**, which is one constraint serving two jobs: the
        // index this probe seeks, and the guard that keeps ingest from storing
        // one event twice.
        todo!("SQLite event store: contains_event_id")
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

/// Everything the blocking thread needs to fetch the next page.
#[derive(Debug)]
struct ReadCursor {
    connection: Arc<Mutex<Connection>>,
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
    /// in the port and this field made it. Invisible today only because
    /// [`ReadCursor::fetch_page`] is `todo!()`.
    resume_from: Option<SequencePosition>,
    /// What is left of [`ReadOptions::limit`], or `None` for unlimited.
    remaining: Option<usize>,
    finished: bool,
}

impl ReadCursor {
    /// Runs one page's worth of SQL. Called only on a blocking thread.
    fn fetch_page(&mut self) -> Result<Page, SqliteEventStoreError> {
        let _connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        let _budget = self.remaining.map_or(PAGE_SIZE, |left| left.min(PAGE_SIZE));
        todo!(
            "SQLite event store: page query for {:?} {:?}",
            self.query,
            self.options
        )
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
                ReadState::Idle(cursor) => {
                    if cursor.finished {
                        return Poll::Ready(None);
                    }
                    // The deferred spawn. This is the line that could not have
                    // been written inside `read`.
                    let runtime = match Handle::try_current() {
                        Ok(runtime) => runtime,
                        Err(err) => return Poll::Ready(Some(Err(err.into()))),
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
