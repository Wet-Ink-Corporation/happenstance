//! A Durable Object-backed [`EventStore`], in the bare `!Send` flavour.
//!
//! # Status
//!
//! The write path is real: `migrate`, `append`, `head` and `contains_event_id`
//! execute SQL against a Durable Object's own storage. The read path — the
//! private `render_read` and `decode_row` that [`SqlRowStream`] drives — is
//! still `todo!()`.
//!
//! # Schema
//!
//! Applied by [`CloudflareEventStore::migrate`], which is `CREATE … IF NOT
//! EXISTS` throughout and safe to call on every open.
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
//! CREATE TABLE store_meta (
//!     k TEXT PRIMARY KEY,
//!     v BLOB NOT NULL
//! ) WITHOUT ROWID;
//! ```
//!
//! The storage engine *is* SQLite, so the shape follows `happenstance-sqlite`'s
//! rather than inventing a second one — copied, never depended on
//! (`CLAUDE.md`, dependency rule). Five lines are decisions:
//!
//! * **`AUTOINCREMENT` is deliberate.** It guarantees positions are never reused
//!   after a delete, which a plain `rowid` does not, and the specification
//!   requires uniqueness across the store's whole lifetime. It also *permits
//!   gaps*, which is why nothing here and no rule may assume `+ 1`.
//! * **`metadata` is nullable.** `None` and `Some(<empty>)` are two values the
//!   contract keeps apart, and a store that folds them has lost one.
//! * **The [`EventId`] origin pair is `UNIQUE` together**, not separately: one
//!   constraint serving two jobs — the index [`contains_event_id`] probes, and
//!   the guard that stops a future ingest storing one event twice.
//! * **`event_type` on `event_tag` is a covering column and not part of the
//!   key.** In the key it would break the position ordering that makes a tag's
//!   range already sorted; absent, it forces every item constraining both type
//!   and tags to join back to `event`, which on a metered runtime is rows read
//!   and billed for nothing.
//! * **`store_meta` carries this object's own [`StoreId`]**, minted once at the
//!   first `migrate` and read back on every later open. Through `exec` like
//!   everything else, so the store's only injected seam stays
//!   [`SqlStorage`] — reaching the Durable Object's KV would be a second handle
//!   every fixture and every host then has to supply.
//!
//! [`contains_event_id`]: EventStore::contains_event_id
//!
//! # Why the whole object is one consistency boundary
//!
//! A Durable Object is single-threaded and has exclusive ownership of its
//! storage, so there is no second writer to lose a race to, and
//! [`SqlStorage::exec`] is synchronous. `append` therefore evaluates its
//! condition and inserts its rows with **nothing awaited in between** — the body
//! contains no `.await` at all — so the object cannot yield to its event loop
//! part way through and the runtime's implicit transaction covers the batch.
//! That is a *stronger* guarantee than any other adapter in the workspace gets,
//! and it is why this adapter is a poor instrument for the position-allocation
//! axis and a good one for the flavour axis.
//!
//! It is also why there is no `BEGIN`/`COMMIT` here: a Durable Object rejects
//! transaction-control statements through `sql.exec()` and offers a callback
//! form instead, which would put a second seam in the constructor for a
//! guarantee the single-threaded turn already gives.

use std::cell::RefCell;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, InvalidEventType,
    InvalidTag, Query, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId,
    StoreLimit, Tags,
};

use crate::query_sql;
use crate::sql_storage::{SqlCursor, SqlError, SqlRow, SqlStorage, SqlValue};

/// The schema, one statement per [`SqlStorage::exec`] call.
///
/// Separate statements rather than one blob: a Durable Object's `exec` does
/// accept several at once, but a prepared statement does not, and issuing them
/// one at a time is the spelling that works on every host this crate is
/// exercised against.
const MIGRATION: &[&str] = &[
    "CREATE TABLE IF NOT EXISTS event (\
        position        INTEGER PRIMARY KEY AUTOINCREMENT, \
        event_type      TEXT    NOT NULL, \
        data            BLOB    NOT NULL, \
        metadata        BLOB, \
        tags            BLOB    NOT NULL, \
        origin_store    BLOB, \
        origin_position INTEGER, \
        recorded_at     INTEGER NOT NULL, \
        UNIQUE (origin_store, origin_position))",
    "CREATE INDEX IF NOT EXISTS event_type_idx ON event(event_type, position)",
    "CREATE TABLE IF NOT EXISTS event_tag (\
        tag        TEXT    NOT NULL, \
        position   INTEGER NOT NULL REFERENCES event(position), \
        event_type TEXT    NOT NULL, \
        PRIMARY KEY (tag, position)) WITHOUT ROWID",
    "CREATE TABLE IF NOT EXISTS store_meta (\
        k TEXT PRIMARY KEY, \
        v BLOB NOT NULL) WITHOUT ROWID",
];

/// The `store_meta` key holding this object's incarnation.
const STORE_ID_KEY: &str = "store_id";

/// The highest integer Workers SQL can hand back through a JS number.
///
/// `Number.MAX_SAFE_INTEGER`. A `SequencePosition` is a `NonZeroU64` and this is
/// 2^53 − 1, so the contract permits positions this runtime cannot round-trip.
/// That is a **declared store limit**, reported through
/// [`CloudflareEventStoreError::StoredPosition`], never narrowed away.
const MAX_SAFE_POSITION: i64 = 9_007_199_254_740_991;

/// The byte that separates one encoded tag from the next.
///
/// `0x1F`, the ASCII unit separator, and it is safe rather than merely
/// convenient: the contract's tag validator rejects every character in Unicode
/// general category `Cc`, and `0x1F` is one of them — so no tag can contain the
/// byte that separates tags. A delimiter a value can contain is how a canonical
/// encoding becomes a matching bug.
const UNIT: u8 = 0x1f;

/// The capacity ceilings this store refuses a batch against.
///
/// A seam rather than three literals, because the *numbers* are
/// `measured-store-limits`' and the *channel* is this story's: a ceiling that
/// is `usize::MAX` refuses nothing, and one that is small refuses through the
/// same code path a measured one will. Nothing public states a numeric limit
/// yet, and a fixture that declared one before it was measured would promise
/// that exactly that many bytes are accepted and one more refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Ceilings {
    /// The largest `data` payload one event may carry.
    pub(crate) event_data_len: usize,
    /// The most tags one event may carry.
    pub(crate) tags_per_event: usize,
    /// The most events one `append` may carry.
    pub(crate) events_per_batch: usize,
}

impl Ceilings {
    /// What this store declares before anything has been measured: nothing.
    pub(crate) const UNMEASURED: Self = Self {
        event_data_len: usize::MAX,
        tags_per_event: usize::MAX,
        events_per_batch: usize::MAX,
    };
}

/// An event store backed by a Durable Object's SQL storage.
///
/// `!Send` and `!Sync`, because a Durable Object is a single-threaded actor and
/// its storage is reached through a JS handle. It therefore implements
/// [`EventStore`] directly rather than `SendEventStore`, and it is the only
/// adapter in the workspace that does.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CloudflareEventStore {
    /// The object's SQL storage. Cloning aliases it rather than copying it.
    sql: SqlStorage,
    /// This object's incarnation, read back from `store_meta` on first use.
    ///
    /// A cache, never a source: it is filled from the row `migrate` wrote, so a
    /// second handle onto the same object reads the same value rather than
    /// minting its own. The `RefCell` is always *tried* — a Durable Object is
    /// re-entrant, and a `borrow_mut` that panicked would take the object down
    /// where [`SqlError::AlreadyBorrowed`] is recoverable.
    identity: RefCell<Option<StoreId>>,
    /// What this store refuses a batch against.
    ceilings: Ceilings,
}

// There is deliberately no `impl Default`. It used to exist, and it could only
// exist while `SqlStorage::new()` minted empty storage out of nothing. A real
// Durable Object's storage comes off `State::storage().sql()` and cannot be
// conjured, so a `Default` that returned one would be a second construction
// path the fixture's "one instance, one object" invariant does not cover.

impl CloudflareEventStore {
    /// Wraps a Durable Object's SQL storage.
    ///
    /// Injection, never construction: in production the handle comes off
    /// `State::storage().sql()` inside a `#[durable_object]` class, and in a
    /// test it comes off whatever the harness has. One constructor means one
    /// path for both.
    #[must_use]
    pub fn new(sql: SqlStorage) -> Self {
        Self {
            sql,
            identity: RefCell::new(None),
            ceilings: Ceilings::UNMEASURED,
        }
    }

    /// The same store, refusing batches against `ceilings`.
    ///
    /// Crate-private, and it is the seam that lets the refusal path be proven in
    /// *both* directions before the real numbers exist. What a test drives
    /// through it is still observed through `append`'s public return type.
    #[cfg(all(test, target_arch = "wasm32"))]
    #[must_use]
    pub(crate) fn with_ceilings(mut self, ceilings: Ceilings) -> Self {
        self.ceilings = ceilings;
        self
    }

    /// Creates the schema if it is absent, and mints this object's incarnation
    /// if it has none.
    ///
    /// Idempotent, and called once per instance by a fixture and once at startup
    /// by a host. There is no schema-version table and none is proposed: no
    /// store has ever written a row with this crate, so there is no deployed
    /// shape to be compatible with, and inventing a migration story for a store
    /// with no users is a decision for a later phase.
    ///
    /// Stays a non-`async` inherent method, because [`SqlStorage::exec`] is
    /// synchronous and there is nothing to await.
    ///
    /// # Errors
    ///
    /// Returns [`CloudflareEventStoreError::Sql`] if any statement fails, and
    /// reports rather than mints if the incarnation row cannot be read back —
    /// a store whose identity is guessed answers `contains_event_id` wrongly
    /// about its own past.
    pub fn migrate(&self) -> Result<(), CloudflareEventStoreError> {
        for statement in MIGRATION {
            self.sql.exec(statement, &[])?;
        }
        // `randomblob` is SQLite's own CSPRNG. Sixteen bytes of entropy from the
        // engine already under the object, weighed against adding a crate for
        // them: `happenstance-core` mints nothing because it is `no_std`-capable
        // and has no entropy source, so the adapter must.
        self.sql.exec(
            "INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, randomblob(16))",
            &[SqlValue::Text(STORE_ID_KEY.to_owned())],
        )?;

        // The read-back is the load-bearing half. `INSERT OR IGNORE` makes only
        // one mint land; reading the row back is what makes every later opener
        // adopt it rather than believe the value it would have generated.
        let store_id = self.read_identity()?;
        *self
            .identity
            .try_borrow_mut()
            .map_err(|_| SqlError::AlreadyBorrowed)? = Some(store_id);
        Ok(())
    }

    /// This object's incarnation, read back from storage the first time it is
    /// asked for.
    fn store_id(&self) -> Result<StoreId, CloudflareEventStoreError> {
        if let Some(store_id) = *self
            .identity
            .try_borrow()
            .map_err(|_| SqlError::AlreadyBorrowed)?
        {
            return Ok(store_id);
        }
        let store_id = self.read_identity()?;
        *self
            .identity
            .try_borrow_mut()
            .map_err(|_| SqlError::AlreadyBorrowed)? = Some(store_id);
        Ok(store_id)
    }

    /// Reads the incarnation row, without consulting or filling the cache.
    fn read_identity(&self) -> Result<StoreId, CloudflareEventStoreError> {
        let mut cursor = self.sql.exec(
            "SELECT v AS store_id FROM store_meta WHERE k = ?",
            &[SqlValue::Text(STORE_ID_KEY.to_owned())],
        )?;
        let row = cursor.next_row().transpose()?.ok_or_else(|| {
            SqlError::internal("the object's store_meta table carries no incarnation row")
        })?;
        match row.values() {
            [SqlValue::Blob(bytes)] => <[u8; 16]>::try_from(bytes.as_slice())
                .map(StoreId::from_bytes)
                .map_err(|_| CloudflareEventStoreError::ColumnType {
                    column: "store_meta.store_id",
                }),
            [_] => Err(CloudflareEventStoreError::ColumnType {
                column: "store_meta.store_id",
            }),
            values => Err(CloudflareEventStoreError::RowShape {
                expected: 1,
                actual: values.len(),
            }),
        }
    }

    /// Refuses a batch that crosses a ceiling this store declares.
    ///
    /// Before any SQL, deliberately: a capacity refusal that arrives after some
    /// rows have landed is a partial batch, and a refusal classified from a
    /// thrown storage error cannot say *which* ceiling was crossed, which CF-40
    /// requires it to name.
    fn check_ceilings(
        &self,
        events: &[Event],
    ) -> Result<(), AppendError<CloudflareEventStoreError>> {
        if events.len() > self.ceilings.events_per_batch {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: events.len(),
            });
        }
        for event in events {
            if event.data().len() > self.ceilings.event_data_len {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    len: event.data().len(),
                });
            }
            if event.tags().len() > self.ceilings.tags_per_event {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::TagsPerEvent,
                    len: event.tags().len(),
                });
            }
        }
        Ok(())
    }

    /// The position violating a guard of `condition`, if one exists.
    ///
    /// One statement per guard, and it asks for the **highest** matching
    /// position above the guard's boundary rather than for existence: a guard is
    /// an inequality on the highest match, so one `max()` answers both halves at
    /// once — whether the condition is violated, and by which event. An
    /// `EXISTS` probe would need a second query on the rejection path, which is
    /// the path a DCB command loop takes every time it loses a race.
    ///
    /// `after: None` is a boundary of zero, because positions start at one.
    fn evaluate(
        &self,
        condition: &AppendCondition,
    ) -> Result<Option<SequencePosition>, CloudflareEventStoreError> {
        for guard in condition.guards() {
            let mut bindings = Vec::new();
            let matched = query_sql::positions_matching(&guard.query, &mut bindings);
            let boundary = guard.after.map_or(0, position_as_i64);
            bindings.push(SqlValue::Integer(boundary));

            let statement =
                format!("SELECT max(position) AS position FROM ({matched}) WHERE position > ?");
            let mut cursor = self.sql.exec(&statement, &bindings)?;
            let Some(row) = cursor.next_row().transpose()? else {
                continue;
            };
            match row.values() {
                [SqlValue::Null] => {}
                [value] => return Ok(Some(decode_position(value, "position")?)),
                values => {
                    return Err(CloudflareEventStoreError::RowShape {
                        expected: 1,
                        actual: values.len(),
                    });
                }
            }
        }
        Ok(None)
    }

    /// Writes every row of the batch and returns the position of its last event.
    ///
    /// The `event` rows go in one at a time, each with `RETURNING position`,
    /// because `AUTOINCREMENT` permits gaps and nothing may assume `+ 1`; SQLite
    /// leaves the row order of a multi-row `RETURNING` undefined, so the
    /// one-statement-per-event shape is what makes "positions follow slice
    /// order" a fact rather than a hope.
    fn write_batch(
        &self,
        store_id: StoreId,
        events: &[Event],
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, CloudflareEventStoreError> {
        let mut positions = Vec::with_capacity(events.len());
        for event in events {
            let metadata = event
                .metadata()
                .map_or(SqlValue::Null, |bytes| SqlValue::Blob(bytes.to_vec()));
            let mut cursor = self.sql.exec(
                "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
                 VALUES (?, ?, ?, ?, ?) RETURNING position",
                &[
                    SqlValue::Text(event.event_type().as_str().to_owned()),
                    SqlValue::Blob(event.data().to_vec()),
                    metadata,
                    SqlValue::Blob(encode_tags(event.tags())),
                    SqlValue::Integer(recorded_at.as_millis()),
                ],
            )?;
            let row = cursor
                .next_row()
                .transpose()?
                .ok_or_else(|| SqlError::internal("INSERT … RETURNING position yielded no row"))?;
            let position = match row.values() {
                [value] => decode_position(value, "position")?,
                values => {
                    return Err(CloudflareEventStoreError::RowShape {
                        expected: 1,
                        actual: values.len(),
                    });
                }
            };
            positions.push(position);
        }

        self.write_tag_rows(events, &positions)?;

        // One statement at the end of the batch rather than a value bound per
        // row: a locally appended event's identity is *this object's incarnation
        // paired with the position it was just given*, and that position is not
        // known until the row exists. `origin_position IS NULL` is the marker,
        // and `UNIQUE (origin_store, origin_position)` tolerates it because
        // SQLite treats NULLs as distinct — which is what lets a multi-row batch
        // stamp itself without tripping the constraint.
        //
        // The marker stays beside the positional bound rather than being
        // replaced by it: a future replication ingest writes rows carrying
        // *another* store's origin, and one landing inside this range must not
        // be restamped under this incarnation.
        if let Some(first) = positions.first() {
            self.sql.exec(
                "UPDATE event SET origin_store = ?, origin_position = position \
                 WHERE position >= ? AND origin_position IS NULL",
                &[
                    SqlValue::Blob(store_id.to_bytes().to_vec()),
                    SqlValue::Integer(position_as_i64(*first)),
                ],
            )?;
        }

        positions
            .last()
            .copied()
            .ok_or_else(|| SqlError::internal("an append of no events reached write_batch").into())
    }

    /// Inserts every `(tag, position, event_type)` row.
    fn write_tag_rows(
        &self,
        events: &[Event],
        positions: &[SequencePosition],
    ) -> Result<(), CloudflareEventStoreError> {
        for (event, position) in events.iter().zip(positions) {
            for tag in event.tags() {
                self.sql.exec(
                    "INSERT INTO event_tag (tag, position, event_type) VALUES (?, ?, ?)",
                    &[
                        SqlValue::Text(tag.as_str().to_owned()),
                        SqlValue::Integer(position_as_i64(*position)),
                        SqlValue::Text(event.event_type().as_str().to_owned()),
                    ],
                )?;
            }
        }
        Ok(())
    }
}

/// How [`CloudflareEventStore`] fails for its own reasons.
///
/// # This type is the ES-6 instrument
///
/// It is `!Send` and `!Sync`, transitively, because [`SqlError::Thrown`] carries
/// a live `worker::Error` behind an `Rc`. It is the only error type in the
/// workspace that is: `MemoryStoreError` is uninhabited and
/// `SqliteEventStoreError`'s variants are all thread-safe, so neither could fail
/// a `Send + Sync` bound if that bound were wrong. See the crate documentation
/// for the compiled result of adding one.
///
/// Note what is *not* here: there is no `ConditionViolated` variant. The DCB
/// concurrency signal travels in
/// [`AppendError::ConditionViolated`], lifted out of the adapter's error type by
/// the contract itself, so this adapter classifies a constraint violation
/// *before* `Self::Error` is constructed. That is load bearing for ES-6 — see
/// the crate documentation's third finding — and adding a variant here would
/// look like the fix and be the defect.
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum CloudflareEventStoreError {
    /// The Workers SQL API failed.
    #[error(transparent)]
    Sql(#[from] SqlError),

    /// A row came back with the wrong number of columns, which means the schema
    /// on disk is not the schema this build expects.
    #[error("a stored row had {actual} columns, expected {expected}")]
    RowShape {
        /// Columns the adapter's `SELECT` asked for.
        expected: usize,
        /// Columns the row actually carried.
        actual: usize,
    },

    /// A column held a [`SqlValue`] variant the adapter cannot decode. Workers
    /// SQL has no boolean and no date, so every mismatch is a real one.
    #[error("column `{column}` held an unusable SQL type")]
    ColumnType {
        /// The column that could not be decoded.
        column: &'static str,
    },

    /// A stored `event_type` no longer satisfies the contract's validation.
    /// Reachable when a row predates a tightening of the rules.
    #[error("a stored event type failed validation on read-back")]
    StoredEventType(#[from] InvalidEventType),

    /// A stored tag no longer satisfies the contract's validation.
    #[error("a stored tag failed validation on read-back")]
    StoredTag(#[from] InvalidTag),

    /// A stored tag column is not UTF-8, so the canonical encoding cannot be
    /// read back at all.
    #[error("a stored tag column is not valid UTF-8")]
    CorruptTags,

    /// A stored row carries no origin identity, which means it was written by
    /// something other than this adapter's `append`.
    #[error("the event at position {position} carries no origin identity")]
    UnstampedEvent {
        /// Where the unstamped row sits.
        position: SequencePosition,
    },

    /// A stored position is not a valid [`SequencePosition`], which is anything
    /// below one or above 2^53 — the latter because Workers SQL widens integers
    /// through a JS number on the way out.
    #[error("the stored position {raw} is not a usable sequence position")]
    StoredPosition {
        /// The rejected value.
        raw: i64,
    },
}

impl EventStore for CloudflareEventStore {
    type Error = CloudflareEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // No `+ Send`, and no `async`. Both matter, and for once they are cheap
        // rather than merely required: `SqlStorage::exec` is synchronous, so
        // there is nothing to await before the cursor exists, and the object is
        // single-threaded, so there is nothing to send it to.
        //
        // `exec` is still deferred to the first poll. ES-9 requires the stream
        // to be lazy, and here that is not a nicety either: a cursor opened at
        // `read` time and never polled would be a live cursor the object had to
        // keep valid across every subsequent await.
        SqlRowStream {
            state: StreamState::Deferred {
                sql: self.sql.clone(),
                query: query.clone(),
                options,
            },
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // The order of refusals is a conformance property, not an
        // implementation detail. An empty batch is refused **before** the
        // condition is evaluated, so a caller whose retry loop branches on
        // `is_condition_violated` terminates instead of retrying a batch that
        // will still be empty next time.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        self.check_ceilings(events)?;

        let store_id = self.store_id().map_err(AppendError::Store)?;

        // Nothing is awaited from here to the end of the function — this whole
        // body contains no `.await` — so the object cannot yield to its event
        // loop between the probe and the insert. That is what makes the pair
        // atomic on this runtime.
        if let Some(condition) = condition
            && let Some(conflict) = self.evaluate(condition).map_err(AppendError::Store)?
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict,
            )));
        }

        self.write_batch(store_id, events, now())
            .map_err(classify_write)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // One row out of `SqlStorage::exec`, and no `await` on the way: the
        // Workers SQL API is synchronous, so this adapter never holds the
        // storage handle across a suspension point and never needs the `Sync`
        // bound the provided body would have wanted.
        let mut cursor = self
            .sql
            .exec("SELECT max(position) AS position FROM event", &[])?;
        let Some(row) = cursor.next_row().transpose()? else {
            return Ok(None);
        };
        match row.values() {
            [SqlValue::Null] => Ok(None),
            [value] => decode_position(value, "position").map(Some),
            values => Err(CloudflareEventStoreError::RowShape {
                expected: 1,
                actual: values.len(),
            }),
        }
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        // Over `origin_store` **and** `origin_position`, never over `position`
        // alone. An `EventId` is an identity, not a coordinate: a foreign
        // `StoreId` at a position this store did assign must read `false`, and
        // `WHERE position = ?` gets the easy half right and the hard half
        // exactly wrong.
        let mut cursor = self.sql.exec(
            "SELECT 1 AS present FROM event \
             WHERE origin_store = ? AND origin_position = ? LIMIT 1",
            &[
                SqlValue::Blob(id.store().to_bytes().to_vec()),
                SqlValue::Integer(position_as_i64(id.position())),
            ],
        )?;
        Ok(cursor.next_row().transpose()?.is_some())
    }
}

/// Which caller-visible channel a write failure travels in.
///
/// **One classifier, in one place, called from one site.** Two call sites is how
/// the arms drift apart, and the thrown value's `message` is the only signal
/// available — Workers exposes no numeric code — so the text match belongs
/// somewhere a future change to the Workers wording can be re-pointed at once.
///
/// Three channels, and the difference between them is what a caller branches on:
///
/// * **[`AppendError::ConditionViolated`]** for a constraint violation. The
///   ordinary rejection path never reaches here — the condition is probed and
///   decided before any row is written — but the `UNIQUE (origin_store,
///   origin_position)` index is a *second* expression of the same fact: an
///   event whose identity this object already holds is a conflict, not a
///   transport fault. `ConditionViolated::unspecified` rather than `at`,
///   because a thrown constraint names the index and not the position.
/// * **[`AppendError::ExceedsStoreLimit`]** is *not* produced here, and the
///   absence is the decision: a ceiling this store declares is refused before
///   any SQL runs, by [`CloudflareEventStore::check_ceilings`], because a
///   refusal classified after the fact cannot say which ceiling was crossed and
///   CF-40 requires it to name one.
/// * **[`AppendError::Store`]** for everything else, object-wide storage
///   exhaustion included. A full object is "the disk is full, retry"; it is a
///   different fact from "this will never fit here", and flattening the two is
///   what the `ExceedsStoreLimit`/`Store` split exists to prevent.
fn classify_write(error: CloudflareEventStoreError) -> AppendError<CloudflareEventStoreError> {
    if let CloudflareEventStoreError::Sql(SqlError::Thrown(throw)) = &error
        && throw.is_constraint_violation().unwrap_or(false)
    {
        return AppendError::ConditionViolated(ConditionViolated::unspecified());
    }
    AppendError::Store(error)
}

/// The time this store stamps an accepted event with.
///
/// `Date.now()` is the Durable Object's own clock, which the runtime pins to the
/// last I/O — so two events accepted in one turn carry the same millisecond,
/// which is exactly why [`RecordedAt`] is not an ordering key.
fn now() -> RecordedAt {
    RecordedAt::from_millis(truncate_millis(worker::js_sys::Date::now()))
}

/// `Date.now()` is integral milliseconds in an `f64`; this is the narrowing.
#[allow(clippy::cast_possible_truncation)]
fn truncate_millis(millis: f64) -> i64 {
    millis as i64
}

/// A position as the integer column holds it.
///
/// Total, and lossless in the direction that matters: `SequencePosition` is a
/// `NonZeroU64`, so a value above `i64::MAX` saturates — and such a value could
/// never have come out of this store, whose ceiling is 2^53 − 1.
fn position_as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// Decodes a stored integer column into a [`SequencePosition`].
///
/// The one decoder, shared by `head`, `contains_event_id`'s neighbours and the
/// read path, so the 2^53 rule is stated once. Two rejections, and neither
/// narrows:
///
/// * below one — not a `SequencePosition` at all;
/// * arriving as [`SqlValue::Real`] — which is what a stored integer above
///   `Number.MAX_SAFE_INTEGER` becomes on the way out of Workers SQL, because
///   the value is widened through a JS number. Reporting it is the difference
///   between a declared store limit and a silently wrong position.
fn decode_position(
    value: &SqlValue,
    column: &'static str,
) -> Result<SequencePosition, CloudflareEventStoreError> {
    match value {
        SqlValue::Integer(raw) if *raw >= 1 && *raw <= MAX_SAFE_POSITION => {
            SequencePosition::new(raw.unsigned_abs())
                .ok_or(CloudflareEventStoreError::StoredPosition { raw: *raw })
        }
        SqlValue::Integer(raw) => Err(CloudflareEventStoreError::StoredPosition { raw: *raw }),
        SqlValue::Real(raw) => Err(CloudflareEventStoreError::StoredPosition {
            raw: truncate_millis(*raw),
        }),
        _ => Err(CloudflareEventStoreError::ColumnType { column }),
    }
}

/// The canonical column value for a tag set.
///
/// The form is `UNIT tag UNIT tag UNIT`, with a leading *and* trailing
/// delimiter, so that a substring match on the column cannot find `course:c1`
/// inside `course:c10`. An empty set encodes to the single delimiter rather than
/// to nothing, so "no tags" and "a tag that is the empty string" could never
/// collide — the second is unconstructible, and the encoding keeps it that way
/// by shape.
fn encode_tags(tags: &Tags) -> Vec<u8> {
    let mut out = Vec::with_capacity(tags.len() * 16 + 1);
    out.push(UNIT);
    for tag in tags {
        out.extend_from_slice(tag.as_str().as_bytes());
        out.push(UNIT);
    }
    out
}

/// The stream [`CloudflareEventStore::read`] returns.
///
/// A real cursor-backed stream, not a placeholder: it holds the storage handle
/// until the first poll, then the live [`SqlCursor`], and it is `!Send` because
/// both of those are. Nothing about it is `Pin`-sensitive — the state machine is
/// written by hand precisely so that it is not a coroutine and its `Send`-ness
/// is decided by its fields rather than by inference.
#[derive(Debug)]
#[non_exhaustive]
pub struct SqlRowStream {
    state: StreamState,
}

/// Where the stream has got to.
#[derive(Debug)]
enum StreamState {
    /// `exec` has not run yet. ES-9's laziness lives here.
    Deferred {
        /// The storage to execute against.
        sql: SqlStorage,
        /// Cloned rather than borrowed, so the stream outlives the caller's
        /// query without a lifetime on the stream type.
        query: Query,
        /// `Copy`, so nothing is cloned.
        options: ReadOptions,
    },
    /// A cursor is open.
    Draining {
        /// The live cursor.
        cursor: SqlCursor,
        /// What is left of [`ReadOptions::limit`], or `None` when unlimited.
        remaining: Option<usize>,
    },
    /// Terminal.
    Done,
}

impl Stream for SqlRowStream {
    type Item = Result<SequencedEvent, CloudflareEventStoreError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            match core::mem::replace(&mut this.state, StreamState::Done) {
                StreamState::Deferred {
                    sql,
                    query,
                    options,
                } => {
                    let (statement, bindings) = render_read(&query, options);
                    match sql.exec(&statement, &bindings) {
                        Ok(cursor) => {
                            this.state = StreamState::Draining {
                                cursor,
                                remaining: options.limit,
                            };
                        }
                        Err(err) => return Poll::Ready(Some(Err(err.into()))),
                    }
                }

                StreamState::Draining {
                    mut cursor,
                    remaining,
                } => {
                    if remaining == Some(0) {
                        return Poll::Ready(None);
                    }

                    if let Err(err) = check_cursor_still_valid(&cursor) {
                        return Poll::Ready(Some(Err(err)));
                    }

                    match cursor.next_row() {
                        None => return Poll::Ready(None),
                        Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),
                        Some(Ok(row)) => {
                            let decoded = decode_row(&row);
                            this.state = StreamState::Draining {
                                cursor,
                                remaining: remaining.map(|left| left.saturating_sub(1)),
                            };
                            return Poll::Ready(Some(decoded));
                        }
                    }
                }

                StreamState::Done => return Poll::Ready(None),
            }
        }
    }
}

/// Rejects a cursor whose storage moved under it.
///
/// Cloudflare documents that a `SqlStorageCursor` held across an `await` "does
/// not provide a stable snapshot of query results". A lazy stream is exactly
/// that, so the check belongs on every poll rather than once at open. It is the
/// clearest example in this crate of a **capability** limit rather than a type
/// error: the port's signature is satisfied either way, and only a conformance
/// rule can tell the difference.
fn check_cursor_still_valid(cursor: &SqlCursor) -> Result<(), CloudflareEventStoreError> {
    cursor
        .still_valid()
        .map_err(CloudflareEventStoreError::from)
}

/// Renders a [`Query`] and [`ReadOptions`] into one statement and its bindings.
fn render_read(_query: &Query, _options: ReadOptions) -> (String, Vec<SqlValue>) {
    todo!("durable-object-read-path: ADR-0011's ceiling-and-page render")
}

/// Decodes one raw row into a [`SequencedEvent`].
fn decode_row(_row: &SqlRow) -> Result<SequencedEvent, CloudflareEventStoreError> {
    todo!("durable-object-read-path: decode position, event_type, data, metadata, tags")
}

#[cfg(all(test, target_arch = "wasm32"))]
mod write_path_tests {
    use happenstance_core::{
        AppendCondition, AppendError, Event, EventId, EventStore, Query, QueryItem,
        SequencePosition, StoreId, StoreLimit, Tag, Tags,
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::{Ceilings, CloudflareEventStore, CloudflareEventStoreError};
    use crate::sql_storage::{SqlStorage, SqlValue};
    use crate::test_object::durable_object;

    /// One fresh Durable Object, migrated, reached through the one constructor.
    fn open() -> (SqlStorage, CloudflareEventStore) {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone());
        store.migrate().expect("the schema applies");
        (sql, store)
    }

    fn event(event_type: &str) -> Event {
        Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
    }

    fn sized_event(event_type: &str, bytes: usize) -> Event {
        Event::new(event_type.to_owned(), vec![0_u8; bytes]).expect("a valid event type")
    }

    fn tagged(event_type: &str, tags: &[&str]) -> Event {
        let tags: Tags = tags
            .iter()
            .map(|tag| Tag::new((*tag).to_owned()).expect("a valid tag"))
            .collect();
        event(event_type).with_tags(tags)
    }

    fn condition_on(event_type: &str) -> AppendCondition {
        AppendCondition::new(Query::from_item(
            QueryItem::of_types([event_type.to_owned()]).expect("a valid query item"),
        ))
    }

    /// Every position the store holds, in order, read through raw SQL rather
    /// than through `read` — which is `durable-object-read-path`'s.
    fn stored_positions(sql: &SqlStorage) -> Vec<i64> {
        let mut cursor = sql
            .exec("SELECT position FROM event ORDER BY position", &[])
            .expect("the select runs");
        let mut out = Vec::new();
        while let Some(row) = cursor.next_row() {
            let row = row.expect("the row decodes");
            match row.values() {
                [SqlValue::Integer(position)] => out.push(*position),
                other => panic!("unexpected position row: {other:?}"),
            }
        }
        out
    }

    fn stored_types(sql: &SqlStorage) -> Vec<String> {
        let mut cursor = sql
            .exec("SELECT event_type FROM event ORDER BY position", &[])
            .expect("the select runs");
        let mut out = Vec::new();
        while let Some(row) = cursor.next_row() {
            let row = row.expect("the row decodes");
            match row.values() {
                [SqlValue::Text(event_type)] => out.push(event_type.clone()),
                other => panic!("unexpected type row: {other:?}"),
            }
        }
        out
    }

    /// AC-001. The capability a constrained-runtime developer came for: build the
    /// store over a Durable Object's storage, migrate, append, and get a real
    /// position back that `head` agrees with.
    #[wasm_bindgen_test]
    async fn append_then_head_round_trips() {
        let (_sql, store) = open();

        let last = store
            .append(&[event("SeatMapPublished")], None)
            .await
            .expect("the append lands");

        assert_eq!(
            store.head().await.expect("head reads"),
            Some(last),
            "head must agree with the position append returned"
        );
    }

    /// AC-002. `migrate` runs on every open, so the second call must be a no-op
    /// rather than an error or a second schema.
    #[wasm_bindgen_test]
    async fn migrate_is_idempotent() {
        let (sql, store) = open();
        let first = store.store_id().expect("the incarnation is readable");

        store.migrate().expect("a second migrate is a no-op");
        store.migrate().expect("and a third");

        assert_eq!(
            store.store_id().expect("the incarnation is still readable"),
            first,
            "re-running migrate must not re-mint the object's identity"
        );
        store
            .append(&[event("StillWorks")], None)
            .await
            .expect("and the schema still accepts writes");
        assert_eq!(stored_types(&sql), ["StillWorks"]);
    }

    /// AC-002. The two identity columns are real — writable and readable —
    /// rather than a comment. `contains_event_id` cannot distinguish a foreign
    /// identity from a local one without them.
    #[wasm_bindgen_test]
    async fn schema_carries_the_identity_columns() {
        let (sql, store) = open();
        let position = store
            .append(&[event("Stamped")], None)
            .await
            .expect("the append lands");
        let store_id = store.store_id().expect("the incarnation is readable");

        let mut cursor = sql
            .exec("SELECT origin_store, origin_position FROM event", &[])
            .expect("the select runs");
        let row = cursor
            .next_row()
            .expect("one row")
            .expect("the row decodes");

        match row.values() {
            [
                SqlValue::Blob(origin_store),
                SqlValue::Integer(origin_position),
            ] => {
                assert_eq!(
                    origin_store.as_slice(),
                    &store_id.to_bytes()[..],
                    "origin_store must be this object's own incarnation"
                );
                assert_eq!(
                    u64::try_from(*origin_position).ok(),
                    Some(position.get()),
                    "origin_position must be the position the row was given"
                );
            }
            other => panic!("the identity columns did not come back: {other:?}"),
        }
    }

    /// AC-003. A second handle onto the same object adopts the incarnation the
    /// first minted, rather than minting its own — which is what makes
    /// `contains_event_id` answer `true` about an event a sibling handle wrote
    /// seconds earlier.
    #[wasm_bindgen_test]
    async fn store_id_is_read_back_on_a_second_handle() {
        let sql = durable_object();
        let first = CloudflareEventStore::new(sql.clone());
        first.migrate().expect("the schema applies");

        let position = first
            .append(&[event("WrittenByTheFirstHandle")], None)
            .await
            .expect("the append lands");
        let id = EventId::new(
            first.store_id().expect("the incarnation is readable"),
            position,
        );

        let second = CloudflareEventStore::new(sql.clone());
        second.migrate().expect("a second open is a no-op");

        assert!(
            second
                .contains_event_id(id)
                .await
                .expect("the membership probe runs"),
            "a second handle must recognise the first handle's event"
        );
    }

    /// AC-003, the other half: the incarnation is a fact of the object, not of
    /// the handle. A store that re-minted per handle would answer the test above
    /// correctly only by accident.
    #[wasm_bindgen_test]
    async fn store_id_is_not_reminted_per_handle() {
        let sql = durable_object();
        let first = CloudflareEventStore::new(sql.clone());
        first.migrate().expect("the schema applies");
        let second = CloudflareEventStore::new(sql.clone());
        second.migrate().expect("a second open is a no-op");
        let third = CloudflareEventStore::new(sql.clone());

        let minted = first.store_id().expect("the incarnation is readable");
        assert_eq!(second.store_id().expect("readable"), minted);
        assert_eq!(
            third.store_id().expect("readable without migrating"),
            minted,
            "a handle that never migrated must still read the object's identity"
        );

        let other_object = CloudflareEventStore::new(durable_object());
        other_object.migrate().expect("the schema applies");
        assert_ne!(
            other_object.store_id().expect("readable"),
            minted,
            "and two different objects must not share an incarnation"
        );
    }

    /// AC-004. An empty batch is refused **before** the condition is read from,
    /// so an application author's retry loop terminates instead of retrying a
    /// batch that will still be empty next time.
    #[wasm_bindgen_test]
    async fn an_empty_batch_is_refused_before_the_condition() {
        let (_sql, store) = open();
        store
            .append(&[event("Existing")], None)
            .await
            .expect("the anchor lands");

        // The condition matches, so a store that evaluated it first would answer
        // `ConditionViolated` and send the caller round the loop again.
        let refused = store.append(&[], Some(&condition_on("Existing"))).await;

        assert!(
            matches!(refused, Err(AppendError::NoEvents)),
            "an empty batch is a caller bug and must be reported as one: {refused:?}"
        );
    }

    /// AC-005. The classifier routes a real constraint violation into the
    /// caller-visible conflict channel rather than into a transport fault.
    ///
    /// Reached through `append` rather than by calling the classifier: the
    /// object already holds a row whose identity the next append's own stamping
    /// statement would duplicate, so `UNIQUE (origin_store, origin_position)`
    /// throws SQLite's own `UNIQUE constraint failed:` text out of
    /// `worker::SqlStorage::exec`. That is the shape a replication ingest
    /// produces, and it is a conflict rather than a fault.
    #[wasm_bindgen_test]
    async fn a_unique_constraint_message_classifies_as_condition_violated() {
        let (sql, store) = open();
        let store_id = store.store_id().expect("the incarnation is readable");

        // Occupy the identity the *next* append will stamp itself with.
        sql.exec(
            "INSERT INTO event \
             (position, event_type, data, tags, recorded_at, origin_store, origin_position) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            &[
                SqlValue::Integer(500),
                SqlValue::Text("Squatter".to_owned()),
                SqlValue::Blob(Vec::new()),
                SqlValue::Blob(vec![0x1f]),
                SqlValue::Integer(0),
                SqlValue::Blob(store_id.to_bytes().to_vec()),
                SqlValue::Integer(501),
            ],
        )
        .expect("the squatting row lands");

        let refused = store.append(&[event("Colliding")], None).await;

        assert!(
            matches!(refused, Err(AppendError::ConditionViolated(_))),
            "a constraint violation must reach the caller as a conflict, not as \
             AppendError::Store: {refused:?}"
        );
    }

    /// AC-005. A rejected condition leaves the object byte-for-byte as it was:
    /// the probe runs before any row is written, and nothing is awaited in
    /// between.
    #[wasm_bindgen_test]
    async fn a_rejected_condition_writes_nothing() {
        let (sql, store) = open();
        store
            .append(&[event("Existing")], None)
            .await
            .expect("the anchor lands");

        let refused = store
            .append(
                &[event("X"), event("Y"), event("Z")],
                Some(&condition_on("Existing")),
            )
            .await;

        assert!(
            matches!(refused, Err(AppendError::ConditionViolated(_))),
            "the condition already matches: {refused:?}"
        );
        assert_eq!(
            stored_types(&sql),
            ["Existing"],
            "a rejected batch must leave NO partial writes behind"
        );
    }

    /// AC-005. The absence is structural, and this is what makes it stay that
    /// way: an exhaustive match over the adapter's error. Adding a
    /// `ConditionViolated` variant — which looks like the fix and is the defect —
    /// stops this compiling.
    #[wasm_bindgen_test]
    fn the_error_type_has_no_condition_violated_variant() {
        fn exhaustive(error: &CloudflareEventStoreError) -> &'static str {
            match error {
                CloudflareEventStoreError::Sql(_) => "sql",
                CloudflareEventStoreError::RowShape { .. } => "row shape",
                CloudflareEventStoreError::ColumnType { .. } => "column type",
                CloudflareEventStoreError::StoredEventType(_) => "stored event type",
                CloudflareEventStoreError::StoredTag(_) => "stored tag",
                CloudflareEventStoreError::CorruptTags => "corrupt tags",
                CloudflareEventStoreError::UnstampedEvent { .. } => "unstamped event",
                CloudflareEventStoreError::StoredPosition { .. } => "stored position",
            }
        }

        assert_eq!(
            exhaustive(&CloudflareEventStoreError::StoredPosition { raw: 0 }),
            "stored position"
        );
    }

    /// AC-006. A capacity refusal names the limit it crossed and the count that
    /// crossed it, and travels in `ExceedsStoreLimit` rather than `Store`.
    #[wasm_bindgen_test]
    async fn a_capacity_refusal_names_a_store_limit() {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql).with_ceilings(Ceilings {
            event_data_len: 8,
            tags_per_event: 2,
            events_per_batch: 3,
        });
        store.migrate().expect("the schema applies");

        let too_big = store.append(&[sized_event("Big", 9)], None).await;
        assert!(
            matches!(
                too_big,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    len: 9
                })
            ),
            "an oversized payload names EventDataLen and its byte count: {too_big:?}"
        );

        let too_many_tags = store
            .append(&[tagged("Tagged", &["a:1", "b:2", "c:3"])], None)
            .await;
        assert!(
            matches!(
                too_many_tags,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::TagsPerEvent,
                    len: 3
                })
            ),
            "too many tags names TagsPerEvent and its tag count: {too_many_tags:?}"
        );

        let too_long = store
            .append(&[event("A"), event("B"), event("C"), event("D")], None)
            .await;
        assert!(
            matches!(
                too_long,
                Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventsPerBatch,
                    len: 4
                })
            ),
            "an over-long batch names EventsPerBatch and its event count: {too_long:?}"
        );
    }

    /// AC-006, the other direction. A ceiling promises that exactly that many
    /// are accepted and one more refused, so a store that can only fail is not
    /// a store that passes.
    #[wasm_bindgen_test]
    async fn exactly_at_the_ceiling_is_accepted() {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone()).with_ceilings(Ceilings {
            event_data_len: 8,
            tags_per_event: 2,
            events_per_batch: 3,
        });
        store.migrate().expect("the schema applies");

        store
            .append(
                &[
                    sized_event("Exact", 8),
                    tagged("AlsoExact", &["a:1", "b:2"]),
                    event("Third"),
                ],
                None,
            )
            .await
            .expect("a batch at every ceiling is accepted");

        assert_eq!(stored_positions(&sql).len(), 3);
    }

    /// AC-007. `append` returns the position of the **last** event in slice
    /// order, and the batch's positions follow slice order. Compared against the
    /// positions the store actually assigned, never against literals — the
    /// specification permits gaps.
    #[wasm_bindgen_test]
    async fn append_returns_the_last_written_position() {
        let (sql, store) = open();

        let returned = store
            .append(&[event("First"), event("Second"), event("Third")], None)
            .await
            .expect("the batch lands");

        let assigned = stored_positions(&sql);
        assert_eq!(assigned.len(), 3, "every event of the batch is visible");
        assert_eq!(
            assigned.last().and_then(|last| u64::try_from(*last).ok()),
            Some(returned.get()),
            "append returns the position of the last event, not the store's head"
        );
        assert_eq!(
            stored_types(&sql),
            ["First", "Second", "Third"],
            "and positions follow slice order"
        );
    }

    /// AC-007. A batch refused part way through its own validation leaves
    /// nothing behind — the ceilings are checked before any statement runs, so
    /// a store that validated as it wrote is rejected here.
    #[wasm_bindgen_test]
    async fn a_failed_batch_leaves_no_partial_rows() {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone()).with_ceilings(Ceilings {
            event_data_len: 8,
            ..Ceilings::UNMEASURED
        });
        store.migrate().expect("the schema applies");

        let refused = store
            .append(
                &[
                    sized_event("One", 4),
                    sized_event("Two", 4),
                    sized_event("ThreeIsTooBig", 9),
                    sized_event("Four", 4),
                ],
                None,
            )
            .await;

        assert!(
            matches!(refused, Err(AppendError::ExceedsStoreLimit { .. })),
            "the third event crosses the ceiling: {refused:?}"
        );
        assert!(
            stored_positions(&sql).is_empty(),
            "and none of the first two may have landed"
        );
    }

    /// AC-008. An empty store has no head.
    #[wasm_bindgen_test]
    async fn head_of_an_empty_store_is_none() {
        let (_sql, store) = open();
        assert_eq!(store.head().await.expect("head reads"), None);
    }

    /// AC-008. `head` is the highest position this store assigned, decoded
    /// through the one shared decoder and compared against what `append`
    /// returned rather than against a number.
    #[wasm_bindgen_test]
    async fn head_is_the_highest_position_this_store_assigned() {
        let (_sql, store) = open();

        let first = store
            .append(&[event("First")], None)
            .await
            .expect("the first append lands");
        assert_eq!(store.head().await.expect("head reads"), Some(first));

        let second = store
            .append(&[event("Second"), event("Third")], None)
            .await
            .expect("the second append lands");
        assert_eq!(
            store.head().await.expect("head reads"),
            Some(second),
            "head advances to the last position of the newest batch"
        );
        assert!(second > first, "and positions are monotonic");
    }

    /// AC-009. A foreign `StoreId` at a position this store *did* assign must
    /// read `false`. `WHERE position = ?` is the named wrong implementation and
    /// it answers this `true`.
    #[wasm_bindgen_test]
    async fn a_foreign_origin_is_not_a_member() {
        let (_sql, store) = open();
        let position = store
            .append(&[event("Local")], None)
            .await
            .expect("the append lands");
        let local = store.store_id().expect("the incarnation is readable");

        let mut flipped = local.to_bytes();
        for byte in &mut flipped {
            *byte = !*byte;
        }
        let foreign = EventId::new(StoreId::from_bytes(flipped), position);

        assert!(
            !store
                .contains_event_id(foreign)
                .await
                .expect("the membership probe runs"),
            "a foreign origin at a position this store assigned is not a member"
        );
    }

    /// AC-009, the positive half — without it the rule above passes against a
    /// store that answers `false` to everything.
    #[wasm_bindgen_test]
    async fn a_local_identity_is_a_member() {
        let (_sql, store) = open();
        let position = store
            .append(&[event("Local")], None)
            .await
            .expect("the append lands");
        let local = EventId::new(store.store_id().expect("readable"), position);

        assert!(
            store
                .contains_event_id(local)
                .await
                .expect("the membership probe runs"),
            "the store must recognise an identity it minted"
        );

        let never_assigned = EventId::new(
            store.store_id().expect("readable"),
            position.next().expect("a next position exists"),
        );
        assert!(
            !store
                .contains_event_id(never_assigned)
                .await
                .expect("the membership probe runs"),
            "and must not recognise one it never assigned"
        );
    }

    /// AC-010. A stored position above `Number.MAX_SAFE_INTEGER` is reported,
    /// never narrowed. Seeded through `exec` — a store-side fact this adapter is
    /// allowed to arrange — and asserted on the **variant**, never on a number.
    #[wasm_bindgen_test]
    async fn a_position_above_two_pow_53_is_reported_not_narrowed() {
        let (sql, store) = open();
        // Written as a literal rather than a binding: binding it would already
        // have widened it through a JS number on the way *in*, and the fact
        // under test is what happens on the way out.
        sql.exec(
            "INSERT INTO event (position, event_type, data, tags, recorded_at) \
             VALUES (9007199254740993, 'Unreachable', x'00', x'1f', 0)",
            &[],
        )
        .expect("the seeded row lands");

        let head = store.head().await;
        assert!(
            matches!(head, Err(CloudflareEventStoreError::StoredPosition { .. })),
            "a position this runtime cannot round-trip must be reported: {head:?}"
        );
    }

    /// AC-010. The other end of the same decoder: below one is not a
    /// `SequencePosition` at all.
    #[wasm_bindgen_test]
    async fn a_position_below_one_is_reported() {
        let (sql, store) = open();
        sql.exec(
            "INSERT INTO event (position, event_type, data, tags, recorded_at) \
             VALUES (0, 'Unreachable', x'00', x'1f', 0)",
            &[],
        )
        .expect("the seeded row lands");

        let head = store.head().await;
        assert!(
            matches!(head, Err(CloudflareEventStoreError::StoredPosition { .. })),
            "a position below one must be reported rather than coerced: {head:?}"
        );
    }

    /// A guard on the shared decoder itself: a position at the ceiling is
    /// accepted, so the two rejections above are not a decoder that refuses
    /// everything.
    #[wasm_bindgen_test]
    fn the_ceiling_itself_is_a_usable_position() {
        let at_ceiling =
            super::decode_position(&SqlValue::Integer(super::MAX_SAFE_POSITION), "position");
        assert_eq!(
            at_ceiling.ok().map(SequencePosition::get),
            u64::try_from(super::MAX_SAFE_POSITION).ok()
        );
    }
}
