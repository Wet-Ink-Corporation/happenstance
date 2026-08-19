//! A Durable Object-backed [`EventStore`], in the bare `!Send` flavour.
//!
//! # Status
//!
//! Complete. `migrate`, `append`, `head`, `contains_event_id` and `read` all
//! execute SQL against a Durable Object's own storage, and no body in this
//! crate is unimplemented any more.
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
//! part way through. That is a *stronger* isolation guarantee than any other
//! adapter in the workspace gets, and it is why this adapter is a poor
//! instrument for the position-allocation axis and a good one for the flavour
//! axis.
//!
//! **Isolation is not atomicity, and this crate once conflated them.** The turn's
//! implicit transaction commits when the handler returns *normally*, and an
//! adapter that catches a thrown statement and reports it as `Err(…)` returns
//! normally — so the rows written before the throw would commit with the rest of
//! the turn. Nothing rolls them back, because from the runtime's point of view
//! nothing went wrong. All-or-none is therefore something
//! [`CloudflareEventStore`]'s write path does explicitly, by discarding the
//! positions the failed batch was assigned, and
//! `a_batch_that_throws_after_its_first_row_leaves_nothing_behind` is what fails
//! if that is removed or believed rather than done.
//!
//! There is still no `BEGIN`/`COMMIT` here, and none is available: a Durable
//! Object rejects transaction-control statements — `SAVEPOINT` among them —
//! through `sql.exec()`, and offers a callback form instead, which would put a
//! second seam in the constructor. The compensating discard is exact rather than
//! a best effort for the reason above: with nothing awaited mid-batch, no row
//! outside the batch can be in the range it deletes.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, InvalidEventType,
    InvalidTag, Query, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId,
    StoreLimit, Tag, Tags,
};

use crate::query_sql;
use crate::sql_storage::{SqlError, SqlRow, SqlStorage, SqlValue};

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
    /// How many rows one page of a read asks for.
    page_size: usize,
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
            page_size: PAGE_SIZE,
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

    /// The same store, paging its reads `page_size` rows at a time.
    ///
    /// Crate-private, and it exists so a test can make paging *happen* over a
    /// handful of events instead of over [`PAGE_SIZE`] of them. The mechanism
    /// under test is identical at any size — which is the property
    /// `read_pages_at_the_shipped_page_size` checks by not using this seam.
    ///
    /// # Panics
    ///
    /// If `page_size` is zero: a page that asks for no rows never terminates.
    #[cfg(all(test, target_arch = "wasm32"))]
    #[must_use]
    pub(crate) fn with_page_size(mut self, page_size: usize) -> Self {
        assert!(page_size > 0, "a page must ask for at least one row");
        self.page_size = page_size;
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

    /// Writes every row of the batch, or none of them, and returns the position
    /// of its last event.
    ///
    /// # The all-or-none half is this function, not the runtime
    ///
    /// A batch is several statements — one `INSERT` per event, one per tag row,
    /// and the identity `UPDATE` at the end — and a throw at any of them leaves
    /// the ones before it written. The runtime does not undo them: a Durable
    /// Object commits the turn's writes when the handler **returns normally**,
    /// and this adapter converts every throw into `Err(…)` and returns normally,
    /// which is exactly what a caller branching on `AppendError` needs it to do.
    /// So the implicit transaction that covers the turn is not a rollback
    /// mechanism for a failure the adapter caught and reported, and reading it
    /// as one was this crate's own error — see
    /// `a_batch_that_throws_after_its_first_row_leaves_nothing_behind`, which
    /// fails against the version that believed it.
    ///
    /// `SAVEPOINT` is not the fix either: a Durable Object rejects transaction
    /// control through `sql.exec()` outright, which is the same reason there is
    /// no `BEGIN`/`COMMIT` here. What is left is **explicit compensation** —
    /// discard the rows this batch assigned — and it is exact rather than
    /// approximate because nothing is awaited between the first `INSERT` and the
    /// last statement, so no other writer can have interleaved a row into the
    /// range being discarded.
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
        // Owned out here rather than inside `write_rows`, because the discard
        // needs the positions of a batch that did **not** finish — which is the
        // one case a return value cannot carry.
        let mut positions = Vec::with_capacity(events.len());
        let outcome = self.write_rows(store_id, events, recorded_at, &mut positions);

        let Err(cause) = outcome else {
            return outcome;
        };
        let Some(from) = positions.first().copied() else {
            // The very first `INSERT` threw, so nothing was assigned and there
            // is nothing to discard. Distinguished rather than folded in: a
            // `DELETE` over an empty range is a statement issued for no reason,
            // and on a metered runtime that is billed rows read.
            return Err(cause);
        };

        match self.discard_from(from) {
            Ok(()) => Err(cause),
            Err(while_discarding) => Err(CloudflareEventStoreError::PartialBatch {
                from,
                cause: Box::new(cause),
                while_discarding: Box::new(while_discarding),
            }),
        }
    }

    /// Writes every row of the batch, leaving whatever it managed in place.
    ///
    /// Split out of `write_batch` so that the positions of a *failed* batch
    /// survive the failure: they are what the compensating discard is aimed at,
    /// and a function that returned them only on success could not report them.
    fn write_rows(
        &self,
        store_id: StoreId,
        events: &[Event],
        recorded_at: RecordedAt,
        positions: &mut Vec<SequencePosition>,
    ) -> Result<SequencePosition, CloudflareEventStoreError> {
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

        self.write_tag_rows(events, positions)?;

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

    /// Discards every row at or above `from`, undoing a batch that failed part
    /// way through.
    ///
    /// **The range is exact, not a guess.** `from` is the position the batch's
    /// first `INSERT … RETURNING` was given, every later row of the batch was
    /// given a higher one, and nothing is awaited between them — so on a
    /// single-threaded object no row outside this batch can sit in the range.
    ///
    /// Tag rows first, because `event_tag.position` references `event.position`
    /// and the reverse order asks SQLite to orphan them.
    ///
    /// What this deliberately does not do is reset the position counter. The
    /// schema's `AUTOINCREMENT` keeps its high-water mark across the delete, so
    /// a discarded batch leaves a **gap** rather than positions to be handed out
    /// twice — and it has to, because an `EventId` is `(store, position)` and a
    /// reused position is two different events wearing one identity. Gaps are
    /// permitted by the specification; reuse is not.
    fn discard_from(&self, from: SequencePosition) -> Result<(), CloudflareEventStoreError> {
        let from = position_as_i64(from);
        self.sql.exec(
            "DELETE FROM event_tag WHERE position >= ?",
            &[SqlValue::Integer(from)],
        )?;
        self.sql.exec(
            "DELETE FROM event WHERE position >= ?",
            &[SqlValue::Integer(from)],
        )?;
        Ok(())
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

    /// An append failed part way through **and** the rows it had already written
    /// could not be discarded, so this object still holds part of a batch that
    /// never succeeded.
    ///
    /// Both failures travel because either alone misleads. `cause` says why the
    /// append stopped; `while_discarding` says why the compensation could not
    /// clean up after it — and it is the second that changes what the caller
    /// must do. Every other failure of `append` leaves the log as it found it,
    /// so a DCB command loop may re-read and retry. This one does not: the log
    /// now contains events the caller's own failed append put there, and a
    /// retry would decide against them.
    ///
    /// It is reachable, not defensive — a storage ceiling reached mid-batch
    /// fails the `INSERT` and then fails the `DELETE` that would undo it. See
    /// `a_batch_whose_discard_also_fails_reports_both_failures`.
    #[error(
        "an append failed and its rows from position {from} could not be discarded: {cause} (while discarding: {while_discarding})"
    )]
    PartialBatch {
        /// The first position this batch was given; every row at or above it was
        /// this batch's, and is what the discard was aimed at.
        from: SequencePosition,
        /// Why the append stopped.
        cause: Box<CloudflareEventStoreError>,
        /// Why the rows it had written could not be discarded.
        while_discarding: Box<CloudflareEventStoreError>,
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
        // there is nothing to await before a cursor exists, and the object is
        // single-threaded, so there is nothing to send it to.
        //
        // Nothing is executed here. ES-11's promise is that the events a read
        // yields are the events that existed at **one moment no later than the
        // first poll** — laziness is permitted and never required — so the
        // sample point is where the ceiling is captured, which is the first
        // poll. A caller may not depend on whether an event appended between
        // this call and that poll appears.
        SqlRowStream {
            state: StreamState::Deferred {
                sql: self.sql.clone(),
                query: query.clone(),
                options,
                page_size: self.page_size,
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
        //
        // The same statement the read path captures its ceiling with, and the
        // same function: "the highest position this store holds" is one
        // question, and two spellings of it would be two answers.
        max_position(&self.sql)
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

/// The columns a stored row is rebuilt from, in the order [`decode_row`] reads
/// them.
///
/// Spelled once so the `SELECT` list and the decode cannot drift apart, which is
/// the same one-codec discipline [`encode_tags`] and [`decode_tags`] follow one
/// level down. A disagreement between this list and what `migrate` created is
/// reported as [`CloudflareEventStoreError::RowShape`] rather than mis-decoded.
const READ_COLUMNS: &str =
    "position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at";

/// How many rows one page of a read asks for.
///
/// The one number the ceiling-and-page mechanism costs, and it is a trade rather
/// than a tuning knob: a Durable Object bills by rows read, so a small page
/// multiplies round trips over a long replay and a large one holds more memory
/// than a constrained runtime wants. 128 is the same order as the contract's own
/// `MIN_SUPPORTED_EVENTS_PER_BATCH`, so a replay of a batch-sized consistency
/// boundary is one page. `measured-store-limits` is where a number measured
/// against the real runtime would replace it.
const PAGE_SIZE: usize = 128;

/// Reads the canonical tag column back.
///
/// The decode half lives beside its encode rather than in the read path,
/// because a decode that disagrees with its encode by one byte produces a store
/// that passes `append_preserves_event_payload` and fails
/// `append_preserves_event_type_and_tags_byte_for_byte`, and the diagnosis costs
/// a day. `Tags` is canonically sorted, so the round trip is order-preserving
/// rather than merely set-preserving.
fn decode_tags(raw: &[u8]) -> Result<Tags, CloudflareEventStoreError> {
    let text = core::str::from_utf8(raw).map_err(|_| CloudflareEventStoreError::CorruptTags)?;
    text.split(UNIT as char)
        .filter(|part| !part.is_empty())
        .map(|part| Tag::new(part).map_err(CloudflareEventStoreError::StoredTag))
        .collect::<Result<Vec<Tag>, _>>()
        .map(|tags| tags.into_iter().collect())
}

/// The stream [`CloudflareEventStore::read`] returns.
///
/// **ADR-0011's ceiling-and-page, and the state machine is hand-written on
/// purpose.** A generator (`async_stream::stream!`) is the natural spelling of a
/// chunked-cursor read, and ADR-0011 deliberately declined an `Unpin` bound so
/// one would stay legal — but a coroutine's auto traits are *inferred*, and this
/// crate's whole value is that its `Send`-ness is decided by its **fields**.
/// The probe at the crate root asserts `SqlRowStream: !Send`, and it means
/// something only while that remains a property of what this type holds.
///
/// # Why no cursor is held between polls
///
/// Cloudflare documents that a `SqlStorageCursor` held across an `await` "does
/// not provide a stable snapshot of query results". The caller's suspension
/// points are *between polls*, which is exactly where a cursor would have to
/// span. So each page is `exec`-ed, drained into memory immediately, and yielded
/// from there; the next page is a fresh statement bounded by the ceiling and by
/// the last position yielded. Nothing is held across a poll boundary, which is
/// also what stops a live replay from wedging an object that is single-threaded
/// but re-entrant.
///
/// The predecessor of this design opened one cursor at the first poll and
/// checked afterwards whether the storage had moved under it. That detected a
/// torn read rather than preventing one, and it is gone.
#[derive(Debug)]
#[non_exhaustive]
pub struct SqlRowStream {
    state: StreamState,
}

/// Where the stream has got to.
#[derive(Debug)]
enum StreamState {
    /// Nothing has been executed. ES-11 permits laziness and never requires it;
    /// deferring here is what makes "no later than the first poll" the sample
    /// point rather than the call.
    Deferred {
        /// The storage to execute against.
        sql: SqlStorage,
        /// Cloned rather than borrowed, so the stream outlives the caller's
        /// query without a lifetime on the stream type.
        query: Query,
        /// `Copy`, so nothing is cloned.
        options: ReadOptions,
        /// How many rows one page asks for.
        page_size: usize,
    },
    /// The sample is fixed and a page is in hand.
    Paging {
        /// The storage to execute against.
        sql: SqlStorage,
        /// The caller's query.
        query: Query,
        /// The caller's options.
        options: ReadOptions,
        /// The position ceiling captured at the first poll. **Every** statement
        /// after the first is bounded by it, which is what makes one `read` one
        /// sample — and, because it is one ceiling rather than one per item,
        /// what makes every item of one query share that sample.
        ceiling: SequencePosition,
        /// The last position yielded, which is where the next page resumes.
        cursor: Option<SequencePosition>,
        /// The rows of the current page, oldest first in the caller's own
        /// direction. Bounded by `page_size`, so the working set is one page
        /// however large the result is.
        page: VecDeque<SqlRow>,
        /// Whether the last page came back full. A short page is the end of the
        /// result set and saves a statement that would return nothing.
        page_was_full: bool,
        /// How many rows one page asks for.
        page_size: usize,
        /// What is left of [`ReadOptions::limit`], or `None` when unlimited.
        /// Spent on **matched** rows only: the page statement carries the
        /// caller's predicate, so a row that reaches this counter is a row the
        /// caller asked for.
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
                    page_size,
                } => {
                    // `limit(0)` reads nothing, deliberately — the contract
                    // diverges from the DCB reference implementation's falsy
                    // zero here — so there is nothing to sample either.
                    if options.limit == Some(0) {
                        return Poll::Ready(None);
                    }
                    // The ceiling. An empty store has none, and that is not an
                    // error: it is the state every adapter is in on its first
                    // run, and arithmetic on it is the registered failure mode
                    // of this very mechanism.
                    match max_position(&sql) {
                        Err(err) => return Poll::Ready(Some(Err(err))),
                        Ok(None) => return Poll::Ready(None),
                        Ok(Some(ceiling)) => {
                            this.state = StreamState::Paging {
                                sql,
                                query,
                                options,
                                ceiling,
                                cursor: None,
                                page: VecDeque::new(),
                                page_was_full: true,
                                page_size,
                                remaining: options.limit,
                            };
                        }
                    }
                }

                StreamState::Paging {
                    sql,
                    query,
                    options,
                    ceiling,
                    cursor,
                    mut page,
                    page_was_full,
                    page_size,
                    remaining,
                } => {
                    if let Some(row) = page.pop_front() {
                        let decoded = match decode_row(&row) {
                            // A row this store cannot represent ends the
                            // replay rather than being skipped: the position is
                            // what the next page resumes from, so a row whose
                            // position did not decode leaves nowhere to resume.
                            // The error is an item on the stream, never a panic
                            // and never a narrowed value.
                            Err(err) => return Poll::Ready(Some(Err(err))),
                            Ok(decoded) => decoded,
                        };
                        this.state = StreamState::Paging {
                            sql,
                            query,
                            options,
                            ceiling,
                            cursor: Some(decoded.position),
                            page,
                            page_was_full,
                            page_size,
                            remaining: remaining.map(|left| left.saturating_sub(1)),
                        };
                        return Poll::Ready(Some(Ok(decoded)));
                    }

                    if !page_was_full || remaining == Some(0) {
                        return Poll::Ready(None);
                    }

                    let want = remaining.map_or(page_size, |left| left.min(page_size));
                    let (statement, bindings) = render_read(&query, options, ceiling, cursor, want);
                    let rows = match drain_page(&sql, &statement, &bindings) {
                        Err(err) => return Poll::Ready(Some(Err(err))),
                        Ok(rows) => rows,
                    };
                    let filled = rows.len() >= want;
                    if rows.is_empty() {
                        return Poll::Ready(None);
                    }
                    this.state = StreamState::Paging {
                        sql,
                        query,
                        options,
                        ceiling,
                        cursor,
                        page: rows,
                        page_was_full: filled,
                        page_size,
                        remaining,
                    };
                }

                StreamState::Done => return Poll::Ready(None),
            }
        }
    }
}

/// The highest position the store holds, or `None` on an empty store.
///
/// The ceiling capture and `head`'s body are the same question, so they are the
/// same statement. A failure here fails the read **loudly** rather than falling
/// back to unbounded paging: a read that quietly drops its ceiling is the wrong
/// implementation ES-11 and ES-12 exist to reject, and it would pass every other
/// assertion this adapter carries.
fn max_position(sql: &SqlStorage) -> Result<Option<SequencePosition>, CloudflareEventStoreError> {
    let mut cursor = sql.exec("SELECT max(position) AS position FROM event", &[])?;
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

/// Runs one page statement and drains its cursor completely.
///
/// Draining before returning is the whole point: the cursor is dead before this
/// function returns, so nothing of it can be held across the caller's next
/// suspension point.
fn drain_page(
    sql: &SqlStorage,
    statement: &str,
    bindings: &[SqlValue],
) -> Result<VecDeque<SqlRow>, CloudflareEventStoreError> {
    let mut cursor = sql.exec(statement, bindings)?;
    let mut rows = VecDeque::new();
    while let Some(row) = cursor.next_row() {
        rows.push_back(row?);
    }
    Ok(rows)
}

/// Renders one page of a read: the caller's query and options, the sample
/// ceiling, and where the previous page stopped.
///
/// Four bounds compose into one `WHERE`, and every one of them is `AND`-ed
/// rather than chosen between:
///
/// * **the ceiling** — `position <= H` in *both* directions. The events a
///   ceiling excludes are the ones appended after the sample, and those are
///   above `H` whichever way the read walks; a `>= H` bound under `backwards`
///   would exclude the log instead of the future.
/// * **`from`** — a threshold, not a seek. Forwards it is a lower bound,
///   backwards an upper one; a position nothing occupies simply yields the next
///   match past it.
/// * **`to`** — inclusive, and the caller-side spelling of a window. It
///   composes with the ceiling rather than replacing it, so the effective bound
///   is the tighter of the two by construction.
/// * **the page cursor** — strict, so the row already yielded is not yielded
///   twice.
///
/// `position IN (…)` rather than a join is what makes "no event is yielded
/// twice across items" true by construction: the query's arms are `UNION`-ed and
/// `IN` is a membership test, so an event matching three items is one row.
fn render_read(
    query: &Query,
    options: ReadOptions,
    ceiling: SequencePosition,
    cursor: Option<SequencePosition>,
    limit: usize,
) -> (String, Vec<SqlValue>) {
    let mut bindings = Vec::new();
    let matched = query_sql::positions_matching(query, &mut bindings);

    let mut sql = format!("SELECT {READ_COLUMNS} FROM event WHERE position IN ({matched})");

    sql.push_str(" AND position <= ?");
    bindings.push(SqlValue::Integer(position_as_i64(ceiling)));

    if let Some(from) = options.from {
        sql.push_str(if options.backwards {
            " AND position <= ?"
        } else {
            " AND position >= ?"
        });
        bindings.push(SqlValue::Integer(position_as_i64(from)));
    }
    if let Some(to) = options.to {
        sql.push_str(if options.backwards {
            " AND position >= ?"
        } else {
            " AND position <= ?"
        });
        bindings.push(SqlValue::Integer(position_as_i64(to)));
    }
    if let Some(cursor) = cursor {
        sql.push_str(if options.backwards {
            " AND position < ?"
        } else {
            " AND position > ?"
        });
        bindings.push(SqlValue::Integer(position_as_i64(cursor)));
    }

    sql.push_str(if options.backwards {
        " ORDER BY position DESC LIMIT ?"
    } else {
        " ORDER BY position ASC LIMIT ?"
    });
    bindings.push(SqlValue::Integer(i64::try_from(limit).unwrap_or(i64::MAX)));

    (sql, bindings)
}

/// Decodes one raw row into a [`SequencedEvent`].
///
/// Three things here are decisions rather than plumbing:
///
/// * **`metadata` keeps `NULL` and an empty blob apart.** They are two values
///   the contract keeps apart, and coercing one into the other loses a state a
///   caller can observe.
/// * **The [`EventId`] is reconstructed from the *stored* origin pair**, never
///   from this object's own incarnation plus the row's own position. Those agree
///   for a locally appended event and disagree for every ingested one, so the
///   shortcut is correct exactly until replication exists.
/// * **`recorded_at` is returned as stored.** A read that stamps `now()` is a
///   store that loses the time it accepted the event at.
fn decode_row(row: &SqlRow) -> Result<SequencedEvent, CloudflareEventStoreError> {
    let values = row.values();
    let [
        position,
        event_type,
        data,
        metadata,
        tags,
        origin_store,
        origin_position,
        recorded_at,
    ] = values
    else {
        return Err(CloudflareEventStoreError::RowShape {
            expected: 8,
            actual: values.len(),
        });
    };

    let position = decode_position(position, "position")?;

    let SqlValue::Text(event_type) = event_type else {
        return Err(CloudflareEventStoreError::ColumnType {
            column: "event_type",
        });
    };
    let SqlValue::Blob(data) = data else {
        return Err(CloudflareEventStoreError::ColumnType { column: "data" });
    };
    let SqlValue::Blob(tags) = tags else {
        return Err(CloudflareEventStoreError::ColumnType { column: "tags" });
    };
    let SqlValue::Integer(recorded_at) = recorded_at else {
        return Err(CloudflareEventStoreError::ColumnType {
            column: "recorded_at",
        });
    };

    let mut event = Event::new(event_type.clone(), data.clone())?.with_tags(decode_tags(tags)?);
    match metadata {
        SqlValue::Null => {}
        SqlValue::Blob(metadata) => event = event.with_metadata(metadata.clone()),
        _ => {
            return Err(CloudflareEventStoreError::ColumnType { column: "metadata" });
        }
    }

    let SqlValue::Blob(origin_store) = origin_store else {
        return Err(CloudflareEventStoreError::UnstampedEvent { position });
    };
    let origin_store = <[u8; 16]>::try_from(origin_store.as_slice())
        .map(StoreId::from_bytes)
        .map_err(|_| CloudflareEventStoreError::ColumnType {
            column: "origin_store",
        })?;
    if matches!(origin_position, SqlValue::Null) {
        return Err(CloudflareEventStoreError::UnstampedEvent { position });
    }
    let origin_position = decode_position(origin_position, "origin_position")?;

    Ok(SequencedEvent::new(
        position,
        EventId::new(origin_store, origin_position),
        RecordedAt::from_millis(*recorded_at),
        event,
    ))
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
    use crate::test_object::{arm_throw, arm_throws, durable_object};

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

    /// Every `(tag, position)` row the store holds, in order.
    ///
    /// Read separately from [`stored_positions`] because a batch discarded
    /// half-way can leave either table populated without the other, and a
    /// compensation that cleaned up one of them would pass an assertion written
    /// only over the other.
    fn stored_tag_rows(sql: &SqlStorage) -> Vec<(String, i64)> {
        let mut cursor = sql
            .exec(
                "SELECT tag, position FROM event_tag ORDER BY position, tag",
                &[],
            )
            .expect("the select runs");
        let mut out = Vec::new();
        while let Some(row) = cursor.next_row() {
            let row = row.expect("the row decodes");
            match row.values() {
                [SqlValue::Text(tag), SqlValue::Integer(position)] => {
                    out.push((tag.clone(), *position));
                }
                other => panic!("unexpected tag row: {other:?}"),
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
                CloudflareEventStoreError::PartialBatch { .. } => "partial batch",
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

    /// AC-007, the half `a_failed_batch_leaves_no_partial_rows` cannot reach.
    ///
    /// That test refuses on the pre-flight ceiling check, where **zero**
    /// statements have run — so it observes a batch that never started, not a
    /// batch that started and stopped. This one arms a throw on the *tag* insert,
    /// which is issued after every `event` row already exists, and is therefore
    /// the only test in this module that can observe all-or-none at all.
    ///
    /// The named wrong implementation it rejects is the one this adapter was:
    /// issue N inserts, convert the throw into `Err(…)`, return normally. A
    /// Durable Object commits its turn's writes when the handler returns
    /// normally, so the rows written before the throw survive — an append that
    /// failed and half-landed, which is the one outcome a DCB command loop
    /// cannot recover from, because its retry re-reads a log containing events
    /// its own failed append put there.
    #[wasm_bindgen_test]
    async fn a_batch_that_throws_after_its_first_row_leaves_nothing_behind() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event_tag", "no space left on device");

        let failure = store
            .append(
                &[tagged("First", &["a:1"]), tagged("Second", &["b:2"])],
                None,
            )
            .await
            .expect_err("the armed throw refuses the write");

        assert!(
            matches!(failure, AppendError::Store(_)),
            "a storage fault is a transport failure, not a conflict: {failure:?}"
        );
        assert!(
            stored_positions(&sql).is_empty(),
            "and the `event` rows written before the throw must not survive it"
        );
        assert!(
            stored_tag_rows(&sql).is_empty(),
            "nor may the tag rows of the events that did insert"
        );
    }

    /// AC-007. The same property at the last statement of the batch — the
    /// `UPDATE … SET origin_store` that stamps identity, which runs after every
    /// `event` **and** every `event_tag` row exists.
    ///
    /// Separate from the test above rather than a second `arm_throw` inside it,
    /// because the two failure points compensate through different amounts of
    /// written state and a single test would only ever prove the first.
    #[wasm_bindgen_test]
    async fn a_batch_that_throws_while_stamping_identity_leaves_nothing_behind() {
        let (sql, store) = open();
        arm_throw(
            &sql,
            "UPDATE event SET origin_store",
            "no space left on device",
        );

        let failure = store
            .append(&[tagged("First", &["a:1"]), event("Second")], None)
            .await
            .expect_err("the armed throw refuses the write");

        assert!(
            matches!(failure, AppendError::Store(_)),
            "a storage fault is a transport failure, not a conflict: {failure:?}"
        );
        assert!(
            stored_positions(&sql).is_empty(),
            "every row of an unstamped batch is discarded"
        );
        assert!(
            stored_tag_rows(&sql).is_empty(),
            "tag rows included, or the object keeps rows pointing at nothing"
        );
    }

    /// AC-007. The store a discarded batch leaves behind is still a usable
    /// store: the next append lands, and it lands *above* the discarded
    /// positions rather than reusing them.
    ///
    /// The second half is why `AUTOINCREMENT` is in the schema. A position this
    /// object once handed out — even to a batch that was then discarded — must
    /// never be handed out again, because `EventId` is `(store, position)` and a
    /// reused position is two different events with one identity.
    #[wasm_bindgen_test]
    async fn a_discarded_batch_does_not_wedge_or_rewind_the_store() {
        let (sql, store) = open();
        arm_throw(&sql, "INSERT INTO event_tag", "no space left on device");

        let discarded = store
            .append(&[tagged("First", &["a:1"])], None)
            .await
            .expect_err("the armed throw refuses the first write");
        assert!(matches!(discarded, AppendError::Store(_)), "{discarded:?}");

        let landed = store
            .append(&[event("Second")], None)
            .await
            .expect("the arming disarmed as it fired, so the next batch lands");

        assert_eq!(
            stored_positions(&sql),
            vec![i64::try_from(landed.get()).expect("a position fits an i64")],
            "the surviving log is exactly the batch that succeeded"
        );
        assert!(
            landed.get() > 1,
            "and the discarded batch's position was not reused: {landed:?}"
        );
    }

    /// AC-007. When the compensation *itself* fails, the caller is told — with
    /// both failures and the position the surviving rows start at.
    ///
    /// The reachable shape, not a defensive one: the failure that stops a batch
    /// is most often the object running out of room, and an object with no room
    /// fails the `DELETE` that would undo the `INSERT` just as readily. Arming
    /// two throws on `event_tag` fires the first on the batch's tag insert and
    /// the second on the discard that answers it.
    ///
    /// Without this test [`CloudflareEventStoreError::PartialBatch`] would be a
    /// variant no execution reaches — a claim about a state, rather than the
    /// report of one.
    #[wasm_bindgen_test]
    async fn a_batch_whose_discard_also_fails_reports_both_failures() {
        let (sql, store) = open();
        arm_throws(&sql, "event_tag", "no space left on device", 2);

        let failure = store
            .append(&[tagged("First", &["a:1"])], None)
            .await
            .expect_err("the armed throw refuses the write");

        let AppendError::Store(CloudflareEventStoreError::PartialBatch {
            from,
            cause,
            while_discarding,
        }) = &failure
        else {
            panic!("a batch whose discard failed reports PartialBatch: {failure:?}");
        };

        assert_eq!(
            stored_positions(&sql),
            vec![i64::try_from(from.get()).expect("a position fits an i64")],
            "`from` names where the rows the object still holds begin"
        );
        assert!(
            cause.to_string().contains("no space left on device"),
            "the original failure is not discarded: {cause}"
        );
        assert!(
            while_discarding
                .to_string()
                .contains("no space left on device"),
            "and neither is the reason the cleanup could not run: {while_discarding}"
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

/// ADR-0011's ceiling-and-page, observed the way a caller observes it.
///
/// Every test here reaches the store through the one construction root
/// `CloudflareEventStore::new(sql)` and reads through `EventStore::read`, and
/// every assertion about position compares against positions the store actually
/// assigned — never a literal, because the specification permits gaps and a
/// conformant adapter may leave them.
#[cfg(all(test, target_arch = "wasm32"))]
mod read_path_tests {
    use core::future::poll_fn;
    use core::pin::{Pin, pin};
    use core::task::{Context, Poll};
    use std::collections::VecDeque;

    use futures_core::Stream;
    use happenstance_core::{
        Event, EventId, EventStore, Query, QueryItem, ReadOptions, RecordedAt, SequencePosition,
        SequencedEvent, StoreId, Tag, Tags,
    };
    use wasm_bindgen_test::wasm_bindgen_test;
    use worker::wasm_bindgen::JsValue;

    use super::{CloudflareEventStore, CloudflareEventStoreError, PAGE_SIZE};
    use crate::js::JsHandle;
    use crate::sql_storage::{SqlCursor, SqlRow, SqlStorage, SqlValue};
    use crate::test_object::{durable_object, statements};

    /// One fresh Durable Object, migrated, reached through the one constructor —
    /// the same shape the write path's tests use, because a read reachable only
    /// through a second construction path would be unmounted.
    fn open() -> (SqlStorage, CloudflareEventStore) {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone());
        store.migrate().expect("the schema applies");
        (sql, store)
    }

    /// The same object, paging a handful of rows at a time so that paging
    /// *happens* over a test-sized log. The mechanism is identical at any page
    /// size, which is what `read_pages_at_the_shipped_page_size` checks by not
    /// using this seam.
    fn open_paged(page_size: usize) -> (SqlStorage, CloudflareEventStore) {
        let sql = durable_object();
        let store = CloudflareEventStore::new(sql.clone()).with_page_size(page_size);
        store.migrate().expect("the schema applies");
        (sql, store)
    }

    fn event(event_type: &str) -> Event {
        Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
    }

    fn tagged(event_type: &str, tags: &[&str]) -> Event {
        event(event_type).with_tags(tag_set(tags))
    }

    fn tag_set(tags: &[&str]) -> Tags {
        tags.iter()
            .map(|tag| Tag::new((*tag).to_owned()).expect("a valid tag"))
            .collect()
    }

    fn of_types(types: &[&str]) -> QueryItem {
        QueryItem::of_types(types.iter().map(|value| (*value).to_owned()))
            .expect("a valid query item")
    }

    /// Advances a pinned stream by one item.
    ///
    /// Hand-rolled because this crate carries no `futures-util`, and useful for
    /// that reason: `read` returns `impl Stream` at the top level, so a caller
    /// reaches the next item through `poll_next` and nothing else. It is also
    /// the shape the interleaving tests need — they suspend *between* polls,
    /// which is exactly where a held cursor would have to span.
    async fn step<S: Stream + ?Sized>(stream: &mut Pin<&mut S>) -> Option<S::Item> {
        poll_fn(|cx| stream.as_mut().poll_next(cx)).await
    }

    /// Drains a stream to exhaustion.
    async fn drain<S: Stream>(stream: S) -> Vec<S::Item> {
        let mut stream = pin!(stream);
        let mut out = Vec::new();
        while let Some(item) = step(&mut stream).await {
            out.push(item);
        }
        out
    }

    /// The positions a read yields, in the order it yielded them.
    async fn read_positions(
        store: &CloudflareEventStore,
        query: &Query,
        options: ReadOptions,
    ) -> Vec<SequencePosition> {
        drain(store.read(query, options))
            .await
            .into_iter()
            .map(|item| item.expect("every row decodes").position)
            .collect()
    }

    /// The same replay, reached through **generic code binding the bare
    /// [`EventStore`]** — the weaker of the two flavours, which accepts both.
    ///
    /// Its value is entirely at compile time: `read` is called *without*
    /// `.await` and the stream it returns is pinned at the top level. An
    /// `async fn read` refactor, or a `+ Send` appearing on the stream, stops
    /// this function compiling.
    async fn replay_through_the_port<S: EventStore>(
        store: &S,
        query: &Query,
        options: ReadOptions,
    ) -> Vec<SequencePosition> {
        let stream = store.read(query, options);
        let mut stream = pin!(stream);
        let mut out = Vec::new();
        while let Some(item) = step(&mut stream).await {
            let Ok(event) = item else {
                panic!("every row decodes");
            };
            out.push(event.position);
        }
        out
    }

    /// Appends `types` one at a time, returning the position each landed at.
    async fn append_each(store: &CloudflareEventStore, types: &[&str]) -> Vec<SequencePosition> {
        let mut out = Vec::new();
        for event_type in types {
            out.push(
                store
                    .append(&[event(event_type)], None)
                    .await
                    .expect("the append lands"),
            );
        }
        out
    }

    fn event_types(items: &[Result<SequencedEvent, CloudflareEventStoreError>]) -> Vec<String> {
        items
            .iter()
            .map(|item| {
                item.as_ref()
                    .expect("every row decodes")
                    .event_type()
                    .as_str()
                    .to_owned()
            })
            .collect()
    }

    fn positions_of(
        items: &[Result<SequencedEvent, CloudflareEventStoreError>],
    ) -> Vec<SequencePosition> {
        items
            .iter()
            .map(|item| item.as_ref().expect("every row decodes").position)
            .collect()
    }

    /// The positions of the items that decoded, ignoring any error item.
    ///
    /// [`positions_of`] panics on an error, which is exactly right for the tests
    /// that must not produce one — and useless for the negative controls, whose
    /// whole purpose is to produce one.
    fn ok_positions_of(items: &[Item]) -> Vec<SequencePosition> {
        items
            .iter()
            .filter_map(|item| item.as_ref().ok().map(|event| event.position))
            .collect()
    }

    /// A row built by hand, for the decoder's own boundary cases.
    fn row(values: Vec<SqlValue>) -> SqlRow {
        SqlRow::new(JsHandle::new(JsValue::NULL), values)
    }

    /// What both the real read and every wrong shape below yield.
    type Item = Result<SequencedEvent, CloudflareEventStoreError>;

    /// Which decision the wrong paging shape gets wrong.
    ///
    /// One type with two policies rather than two types, because everything else
    /// about them — the statement, the decode, the page loop — must stay
    /// *identical to the shipped read*, or the control proves that two
    /// implementations differ rather than that one decision matters.
    #[derive(Debug, Clone, Copy)]
    enum WrongCeiling {
        /// AC-003's named wrong implementation: no stable sample. The ceiling is
        /// re-captured before every page, which is what "pages without a
        /// ceiling" amounts to against a store whose maximum only grows — and it
        /// is the shape a careful implementer reaches honestly, by treating the
        /// bound as a detail of rendering one page rather than as the sample
        /// point of the whole read.
        RecapturedPerPage,
        /// AC-006's `NullHeadPagingStore`: the ceiling is arithmetic over
        /// `head()`, and an empty store's head is `None`. Spelled as an error
        /// item rather than a panic because that is the milder of the two
        /// outcomes ES-9 records, and the milder one still has to be rejected.
        ArithmeticOnHead,
    }

    /// A paging read that gets its ceiling wrong, and nothing else.
    ///
    /// **A committed negative control, not a fixture.** It reuses the shipped
    /// [`render_read`](super::render_read), [`drain_page`](super::drain_page)
    /// and [`decode_row`](super::decode_row) verbatim, so the only difference
    /// between it and [`SqlRowStream`](super::SqlRowStream) is the one line each
    /// [`WrongCeiling`] names. That is what makes a test that rejects it a test
    /// about the ceiling.
    ///
    /// It is deliberately reachable only from this module: it is evidence that
    /// the read-path assertions can fail, never an alternative read.
    struct WrongPagingStream {
        sql: SqlStorage,
        query: Query,
        options: ReadOptions,
        page_size: usize,
        ceiling: WrongCeiling,
        cursor: Option<SequencePosition>,
        page: VecDeque<SqlRow>,
        page_was_full: bool,
        done: bool,
    }

    impl WrongPagingStream {
        fn new(
            sql: &SqlStorage,
            query: Query,
            options: ReadOptions,
            page_size: usize,
            ceiling: WrongCeiling,
        ) -> Self {
            Self {
                sql: sql.clone(),
                query,
                options,
                page_size,
                ceiling,
                cursor: None,
                page: VecDeque::new(),
                page_was_full: true,
                done: false,
            }
        }
    }

    impl Stream for WrongPagingStream {
        type Item = Item;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            loop {
                if let Some(row) = this.page.pop_front() {
                    return match super::decode_row(&row) {
                        Ok(decoded) => {
                            this.cursor = Some(decoded.position);
                            Poll::Ready(Some(Ok(decoded)))
                        }
                        Err(err) => {
                            this.done = true;
                            Poll::Ready(Some(Err(err)))
                        }
                    };
                }
                if this.done || !this.page_was_full {
                    return Poll::Ready(None);
                }

                let ceiling = match super::max_position(&this.sql) {
                    Err(err) => {
                        this.done = true;
                        return Poll::Ready(Some(Err(err)));
                    }
                    Ok(Some(ceiling)) => ceiling,
                    Ok(None) => match this.ceiling {
                        WrongCeiling::RecapturedPerPage => return Poll::Ready(None),
                        WrongCeiling::ArithmeticOnHead => {
                            // The registered failure mode, and the whole of it:
                            // arithmetic on an absent head. The shipped read
                            // returns an empty stream here.
                            this.done = true;
                            return Poll::Ready(Some(Err(
                                CloudflareEventStoreError::StoredPosition { raw: 0 },
                            )));
                        }
                    },
                };

                let (statement, bindings) = super::render_read(
                    &this.query,
                    this.options,
                    ceiling,
                    this.cursor,
                    this.page_size,
                );
                match super::drain_page(&this.sql, &statement, &bindings) {
                    Err(err) => {
                        this.done = true;
                        return Poll::Ready(Some(Err(err)));
                    }
                    Ok(rows) => {
                        if rows.is_empty() {
                            return Poll::Ready(None);
                        }
                        this.page_was_full = rows.len() >= this.page_size;
                        this.page = rows;
                    }
                }
            }
        }
    }

    /// A read that opens one cursor at the first poll and advances it one row
    /// per poll — the predecessor design [`SqlRowStream`](super::SqlRowStream)'s
    /// own documentation describes and rejects.
    ///
    /// **A committed negative control.** The statement it runs is the shipped
    /// one, rendered by [`render_read`](super::render_read) under a real ceiling
    /// captured up front, so the single difference from the shipped read is that
    /// the cursor is *held across the caller's suspension point* rather than
    /// drained before the poll returns. Cloudflare documents that a cursor held
    /// across an `await` is not a stable snapshot; this is what that costs.
    struct CursorHoldingStream {
        sql: SqlStorage,
        statement: Option<(String, Vec<SqlValue>)>,
        cursor: Option<SqlCursor>,
        done: bool,
    }

    impl CursorHoldingStream {
        fn new(sql: &SqlStorage, query: &Query, options: ReadOptions) -> Self {
            let ceiling = super::max_position(sql)
                .expect("the ceiling capture runs")
                .expect("this control is only ever pointed at a non-empty store");
            Self {
                sql: sql.clone(),
                statement: Some(super::render_read(
                    query,
                    options,
                    ceiling,
                    None,
                    usize::MAX,
                )),
                cursor: None,
                done: false,
            }
        }
    }

    impl Stream for CursorHoldingStream {
        type Item = Item;

        fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();
            if this.done {
                return Poll::Ready(None);
            }
            if this.cursor.is_none() {
                let Some((statement, bindings)) = this.statement.take() else {
                    this.done = true;
                    return Poll::Ready(None);
                };
                match this.sql.exec(&statement, &bindings) {
                    Err(err) => {
                        this.done = true;
                        return Poll::Ready(Some(Err(err.into())));
                    }
                    Ok(cursor) => this.cursor = Some(cursor),
                }
            }

            let cursor = this
                .cursor
                .as_mut()
                .expect("the cursor was just opened or already held");
            match cursor.next_row() {
                None => {
                    this.done = true;
                    Poll::Ready(None)
                }
                Some(Err(err)) => {
                    this.done = true;
                    Poll::Ready(Some(Err(err.into())))
                }
                Some(Ok(row)) => Poll::Ready(Some(super::decode_row(&row))),
            }
        }
    }

    /// A well-formed stored row, which each decode test breaks in exactly one
    /// place — so a failure names the break rather than the scaffolding.
    fn well_formed_row() -> Vec<SqlValue> {
        vec![
            SqlValue::Integer(1),
            SqlValue::Text("SeatMapPublished".to_owned()),
            SqlValue::Blob(b"payload".to_vec()),
            SqlValue::Null,
            SqlValue::Blob(vec![super::UNIT]),
            SqlValue::Blob(vec![0_u8; 16]),
            SqlValue::Integer(1),
            SqlValue::Integer(0),
        ]
    }

    /// AC-001. The backbone activity's second half: store **and replay** events
    /// inside a Durable Object. Every appended event comes back, in ascending
    /// position order, at the positions the store itself assigned.
    #[wasm_bindgen_test]
    async fn read_all_replays_every_appended_event() {
        let (_sql, store) = open();
        let assigned = append_each(&store, &["One", "Two", "Three", "Four"]).await;

        let replayed = drain(store.read(&Query::all(), ReadOptions::new())).await;

        assert_eq!(
            positions_of(&replayed),
            assigned,
            "a replay yields exactly the positions the store assigned, in order"
        );
        assert_eq!(event_types(&replayed), ["One", "Two", "Three", "Four"]);
    }

    /// AC-001. Everything a replayed event carries survives the round trip:
    /// type, opaque payload, tags in canonical order, and the identity the store
    /// stamped.
    #[wasm_bindgen_test]
    async fn a_replayed_event_carries_everything_it_was_appended_with() {
        let (_sql, store) = open();
        let written =
            tagged("SeatReserved", &["seat:A1", "order:7"]).with_metadata(&b"trace-id"[..]);
        let position = store
            .append(core::slice::from_ref(&written), None)
            .await
            .expect("the append lands");
        let store_id = store.store_id().expect("the incarnation is readable");

        let replayed = drain(store.read(&Query::all(), ReadOptions::new())).await;
        let [Ok(only)] = replayed.as_slice() else {
            panic!("exactly one event was appended: {replayed:?}");
        };

        assert_eq!(only.position, position);
        assert_eq!(only.id, EventId::new(store_id, position));
        assert_eq!(only.event.event_type(), written.event_type());
        assert_eq!(only.event.data(), written.data());
        assert_eq!(only.event.tags(), written.tags());
        assert_eq!(only.event.metadata(), written.metadata());
    }

    /// AC-001. A sibling handle built the same way over the same object replays
    /// the same log — "one instance, many handles", from the read side.
    #[wasm_bindgen_test]
    async fn read_reaches_the_store_only_through_new() {
        let (sql, store) = open();
        let assigned = append_each(&store, &["One", "Two"]).await;

        let second = CloudflareEventStore::new(sql.clone());
        assert_eq!(
            read_positions(&second, &Query::all(), ReadOptions::new()).await,
            assigned,
            "a sibling handle over the same object sees the same log"
        );
    }

    /// AC-001. Paging at the shipped [`PAGE_SIZE`] rather than through the test
    /// seam, so the mechanism is exercised at the size that actually ships.
    #[wasm_bindgen_test]
    async fn read_pages_at_the_shipped_page_size() {
        let (_sql, store) = open();
        let mut assigned = Vec::new();
        for index in 0..=PAGE_SIZE {
            assigned.push(
                store
                    .append(&[event(&format!("Event{index}"))], None)
                    .await
                    .expect("the append lands"),
            );
        }

        assert_eq!(
            read_positions(&store, &Query::all(), ReadOptions::new()).await,
            assigned,
            "a log longer than one page replays completely"
        );
    }

    /// AC-002. `read` is reachable from generic code binding the **bare**
    /// `EventStore`, and the stream comes back at the top level rather than
    /// inside a future. The proof is that [`replay_through_the_port`] compiles.
    #[wasm_bindgen_test]
    async fn read_is_usable_from_generic_code_binding_the_bare_port() {
        let (_sql, store) = open();
        let assigned = append_each(&store, &["One", "Two", "Three"]).await;

        assert_eq!(
            replay_through_the_port(&store, &Query::all(), ReadOptions::new()).await,
            assigned
        );
    }

    /// AC-003's assertion body, written **once** so that the real read and the
    /// named wrong shape face the same predicate rather than two predicates that
    /// happen to be spelled alike.
    ///
    /// Drains two items, appends mid-drain on the same handle, drains the rest,
    /// and hands back everything it saw plus the position the late append
    /// landed at. The caller decides what that means.
    async fn drain_across_an_interleaved_append<S>(
        stream: S,
        store: &CloudflareEventStore,
    ) -> (Vec<Item>, SequencePosition)
    where
        S: Stream<Item = Item>,
    {
        let mut stream = pin!(stream);
        let mut seen = vec![
            step(&mut stream).await.expect("a first event"),
            step(&mut stream).await.expect("a second event"),
        ];

        let late = store
            .append(&[event("AppendedMidDrain")], None)
            .await
            .expect("an append during a live read lands");

        while let Some(item) = step(&mut stream).await {
            seen.push(item);
        }
        (seen, late)
    }

    /// AC-003. The ceiling as a caller observes it: drain part of a paged read,
    /// append, drain the rest — and the late event is absent. A paging read that
    /// re-`exec`s *without* a ceiling picks it up on the next page, which is the
    /// wrong implementation this rejects — committed as
    /// [`WrongCeiling::RecapturedPerPage`] and driven through the same helper by
    /// `a_ceilingless_paging_read_is_rejected` below.
    #[wasm_bindgen_test]
    async fn read_is_stable_under_an_interleaved_append() {
        let (_sql, store) = open_paged(2);
        append_each(&store, &["One", "Two", "Three", "Four"]).await;

        // Bound to a local rather than written inline: `read` returns
        // `impl Stream` capturing the query's lifetime, so a temporary would be
        // dropped while the stream still holds it.
        let all = Query::all();
        let (seen, late) =
            drain_across_an_interleaved_append(store.read(&all, ReadOptions::new()), &store).await;

        assert_eq!(
            event_types(&seen),
            ["One", "Two", "Three", "Four"],
            "a read is one sample: nothing appended mid-drain appears part-way through"
        );
        assert!(
            !positions_of(&seen).contains(&late),
            "and the late event is absent by position too"
        );
    }

    /// AC-003's **negative control**, committed rather than swept.
    ///
    /// The named wrong implementation, driven through
    /// [`drain_across_an_interleaved_append`] — the same helper, the same
    /// assertions — and shown to fail both of them. Without this the test above
    /// is a rule no adapter can fail, which is the decorative shape `CLAUDE.md`
    /// names; a mutation sweep proves the same thing on the afternoon it is run
    /// and nothing re-runs it afterwards.
    #[wasm_bindgen_test]
    async fn a_ceilingless_paging_read_is_rejected() {
        let (sql, store) = open_paged(2);
        append_each(&store, &["One", "Two", "Three", "Four"]).await;

        let wrong = WrongPagingStream::new(
            &sql,
            Query::all(),
            ReadOptions::new(),
            2,
            WrongCeiling::RecapturedPerPage,
        );
        let (seen, late) = drain_across_an_interleaved_append(wrong, &store).await;

        assert!(
            event_types(&seen) != ["One", "Two", "Three", "Four"],
            "the ceiling-less shape must fail the assertion the real read passes: {:?}",
            event_types(&seen)
        );
        assert!(
            positions_of(&seen).contains(&late),
            "and it fails it by picking the mid-drain append up on the next page"
        );
    }

    /// AC-003. The sample point is *the first poll*, not the call. An event
    /// appended between the two may appear; everything appended after the first
    /// poll may not.
    #[wasm_bindgen_test]
    async fn the_ceiling_is_captured_no_later_than_the_first_poll() {
        let (_sql, store) = open_paged(1);
        append_each(&store, &["Before"]).await;

        let all = Query::all();
        let stream = store.read(&all, ReadOptions::new());
        let mut stream = pin!(stream);

        // Between the call and the first poll. A caller may not depend on
        // whether this one appears, so nothing is asserted about it.
        store
            .append(&[event("BetweenCallAndPoll")], None)
            .await
            .expect("the append lands");

        let first = step(&mut stream).await.expect("a first event");
        assert!(first.is_ok(), "the first poll yields an event: {first:?}");

        // After the sample. This one may not appear.
        let after = store
            .append(&[event("AfterTheFirstPoll")], None)
            .await
            .expect("the append lands");

        let mut seen = vec![first];
        while let Some(item) = step(&mut stream).await {
            seen.push(item);
        }

        assert!(
            !positions_of(&seen).contains(&after),
            "nothing appended after the first poll may appear: {:?}",
            event_types(&seen)
        );
    }

    /// AC-003. The ceiling is not decoration: **every** page statement carries
    /// it, so a page issued after the caller suspended cannot reach past the
    /// sample. This is the committed detector for a paging read that drops it.
    #[wasm_bindgen_test]
    async fn every_page_statement_carries_the_ceiling_bound() {
        let (sql, store) = open_paged(1);
        append_each(&store, &["One", "Two", "Three"]).await;

        drain(store.read(&Query::all(), ReadOptions::new())).await;

        let pages: Vec<String> = statements(&sql)
            .into_iter()
            .filter(|statement| statement.contains("FROM event WHERE position IN ("))
            .collect();
        assert!(
            pages.len() > 1,
            "the read must have paged for this assertion to mean anything: {pages:?}"
        );
        for page in &pages {
            assert!(
                page.contains("AND position <= ?"),
                "every page statement is bounded by the ceiling: {page}"
            );
        }
    }

    /// AC-004. A multi-item query is served by one ceiling, so an event matching
    /// an early item that lands between two statements is excluded by the same
    /// predicate that bounds every other statement.
    #[wasm_bindgen_test]
    async fn all_items_of_one_query_share_one_ceiling() {
        let (_sql, store) = open_paged(1);
        append_each(&store, &["Alpha", "Beta", "Alpha", "Beta"]).await;
        let query =
            Query::from_items([of_types(&["Alpha"]), of_types(&["Beta"])]).expect("a valid query");

        let stream = store.read(&query, ReadOptions::new());
        let mut stream = pin!(stream);
        let mut seen = vec![step(&mut stream).await.expect("a first event")];

        let late_alpha = store
            .append(&[event("Alpha")], None)
            .await
            .expect("the append lands");
        let late_beta = store
            .append(&[event("Beta")], None)
            .await
            .expect("the append lands");

        while let Some(item) = step(&mut stream).await {
            seen.push(item);
        }

        let positions = positions_of(&seen);
        assert_eq!(
            positions.len(),
            4,
            "the four events that existed at the sample, and only those: {:?}",
            event_types(&seen)
        );
        assert!(
            !positions.contains(&late_alpha) && !positions.contains(&late_beta),
            "neither item picks up an event that landed after the sample"
        );
    }

    /// AC-004. One ceiling capture per `read`, not one per `QueryItem` — counted
    /// at the storage seam, because on a store nobody is writing to the rows
    /// that come back look identical either way.
    #[wasm_bindgen_test]
    async fn one_ceiling_is_captured_per_read_not_one_per_item() {
        let (sql, store) = open_paged(1);
        append_each(&store, &["Alpha", "Beta", "Gamma"]).await;
        let query = Query::from_items([
            of_types(&["Alpha"]),
            of_types(&["Beta"]),
            of_types(&["Gamma"]),
        ])
        .expect("a valid query");

        let before = statements(&sql).len();
        drain(store.read(&query, ReadOptions::new())).await;

        let captures = statements(&sql)
            .into_iter()
            .skip(before)
            .filter(|statement| statement.contains("max(position)"))
            .count();
        assert_eq!(
            captures, 1,
            "a three-item query samples the store once, not three times"
        );
    }

    /// AC-004. `UNION` and not `UNION ALL`: an event matching two items of one
    /// query is one event on the stream.
    #[wasm_bindgen_test]
    async fn query_union_is_item_concatenation() {
        let (_sql, store) = open();
        let both = store
            .append(&[tagged("Alpha", &["shared"])], None)
            .await
            .expect("the append lands");
        let query = Query::from_items([
            of_types(&["Alpha"]),
            QueryItem::tagged(tag_set(&["shared"])).expect("a valid item"),
        ])
        .expect("a valid query");

        assert_eq!(
            read_positions(&store, &query, ReadOptions::new()).await,
            [both],
            "an event matching two items is yielded once"
        );
    }

    /// AC-005's assertion body, written **once** so the real read and the named
    /// cursor-holding shape face the same predicate.
    ///
    /// Steps one item, appends on the same handle while the read is suspended,
    /// then drains what is left and hands all of it back.
    async fn replay_across_an_append_on_one_handle<S>(
        stream: S,
        store: &CloudflareEventStore,
    ) -> Vec<Item>
    where
        S: Stream<Item = Item>,
    {
        let mut stream = pin!(stream);
        let mut seen = vec![step(&mut stream).await.expect("a first event")];

        store
            .append(&[event("WhileTheReadIsLive")], None)
            .await
            .expect("an append must not be blocked by a suspended read");

        while let Some(item) = step(&mut stream).await {
            seen.push(item);
        }
        seen
    }

    /// AC-005. A suspended read holds no borrow of the object and no live
    /// cursor, so an `append` on the same handle completes mid-drain **and the
    /// replay survives it**.
    ///
    /// Both halves are asserted, and the second is the one with teeth. A stream
    /// that held its cursor across the poll would let the append through — the
    /// cursor holds an `Rc`, not a `Ref` — and then break on its next advance
    /// with [`SqlError::CursorInvalidated`]. An assertion that only counted
    /// items would pass against it, so it counts *outcomes*. See
    /// `a_cursor_held_across_a_poll_is_rejected`.
    #[wasm_bindgen_test]
    async fn a_live_read_stream_does_not_block_an_append_on_one_handle() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two"]).await;

        let all = Query::all();
        let seen =
            replay_across_an_append_on_one_handle(store.read(&all, ReadOptions::new()), &store)
                .await;

        assert!(
            seen.iter().all(Result::is_ok),
            "the replay survives an append on the same handle: {seen:?}"
        );
        assert_eq!(
            positions_of(&seen),
            assigned,
            "and it completes, rather than stopping where the append interrupted it"
        );
    }

    /// AC-005's **negative control**, committed rather than swept.
    ///
    /// [`CursorHoldingStream`] is the predecessor design `SqlRowStream`'s own
    /// documentation describes and rejects: one cursor opened at the first poll
    /// and advanced one row per poll. It differs from the shipped read in
    /// exactly one decision — the cursor is held across the caller's suspension
    /// point instead of being drained before the poll returns — and it is
    /// driven through the same helper and rejected by the same predicate.
    #[wasm_bindgen_test]
    async fn a_cursor_held_across_a_poll_is_rejected() {
        let (sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two"]).await;

        let wrong = CursorHoldingStream::new(&sql, &Query::all(), ReadOptions::new());
        let seen = replay_across_an_append_on_one_handle(wrong, &store).await;

        assert!(
            seen.iter().any(Result::is_err),
            "a held cursor cannot survive another statement, so this must fail: {seen:?}"
        );
        assert!(
            ok_positions_of(&seen) != assigned,
            "and the replay it hands back is short of the log it was reading"
        );
    }

    /// AC-005. The other direction, and the obligation no clause states: a read
    /// issued while an `append` future on the same handle is suspended
    /// completes.
    #[wasm_bindgen_test]
    async fn a_read_issued_during_a_suspended_append_completes() {
        let (_sql, store) = open();
        let assigned = append_each(&store, &["One"]).await;

        // Created and deliberately not polled: the append is suspended at its
        // very first suspension point, holding nothing.
        let batch = [event("Suspended")];
        let suspended = store.append(&batch, None);

        assert_eq!(
            read_positions(&store, &Query::all(), ReadOptions::new()).await,
            assigned,
            "a read issued during a suspended append completes"
        );

        suspended.await.expect("and the append still lands");
    }

    /// AC-005. Two live reads over one object interleave. If either held the
    /// object's SQL state across a poll boundary the other would be refused with
    /// `SqlError::AlreadyBorrowed`; if either held a cursor, the other's
    /// statement would invalidate it.
    #[wasm_bindgen_test]
    async fn two_live_reads_interleave_without_borrowing_the_object() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three"]).await;

        let all = Query::all();
        let first = store.read(&all, ReadOptions::new());
        let second = store.read(&all, ReadOptions::new());
        let mut first = pin!(first);
        let mut second = pin!(second);

        let mut left = Vec::new();
        let mut right = Vec::new();
        loop {
            let a = step(&mut first).await;
            let b = step(&mut second).await;
            if a.is_none() && b.is_none() {
                break;
            }
            left.extend(a);
            right.extend(b);
        }

        for side in [&left, &right] {
            assert_eq!(
                positions_of(side),
                assigned,
                "both interleaved reads complete in full"
            );
        }
    }

    /// The option sets AC-006's empty-store case is exercised under.
    ///
    /// A function rather than a literal repeated in two tests, because the
    /// negative control is only a control while it faces the *same* inputs.
    fn empty_store_option_sets() -> [ReadOptions; 5] {
        [
            ReadOptions::new(),
            ReadOptions::new().backwards(),
            ReadOptions::new().limit(3),
            ReadOptions::new().from(SequencePosition::new(1).expect("one is a position")),
            ReadOptions::new().to(SequencePosition::new(9).expect("nine is a position")),
        ]
    }

    /// AC-006. An empty store reads as empty, with no error — the state every
    /// adapter is in on its first run, and the registered failure mode of
    /// ceiling arithmetic on `head() == None`. The options are exercised against
    /// it rather than short-circuited.
    #[wasm_bindgen_test]
    async fn reading_an_empty_store_yields_nothing() {
        let (_sql, store) = open();

        for options in empty_store_option_sets() {
            let items = drain(store.read(&Query::all(), options)).await;
            assert!(
                items.is_empty(),
                "an empty store yields nothing and no error under {options:?}: {items:?}"
            );
        }
    }

    /// AC-006's **negative control**, committed rather than swept.
    ///
    /// `NullHeadPagingStore` is the registered failing adapter ES-9 already
    /// names (`references/adr/0011-read-laziness-and-isolation.md:374-384`): its
    /// ceiling is arithmetic over `head()`, so the one store state every adapter
    /// starts in — `head() == None` — becomes an error rather than an empty
    /// replay. [`WrongCeiling::ArithmeticOnHead`] is that shape, held to the
    /// same option sets and the same predicate as the test above.
    #[wasm_bindgen_test]
    async fn a_null_head_ceiling_is_rejected_on_the_empty_store() {
        let (sql, _store) = open();

        for options in empty_store_option_sets() {
            let items = drain(WrongPagingStream::new(
                &sql,
                Query::all(),
                options,
                2,
                WrongCeiling::ArithmeticOnHead,
            ))
            .await;
            assert!(
                !items.is_empty(),
                "the null-head shape must fail the empty-store case under {options:?}"
            );
            assert!(
                items.iter().any(Result::is_err),
                "and it fails it by erroring on a store whose only fault is being new: {items:?}"
            );
        }
    }

    /// AC-006. `from` is inclusive.
    #[wasm_bindgen_test]
    async fn read_from_is_inclusive() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three"]).await;

        assert_eq!(
            read_positions(&store, &Query::all(), ReadOptions::new().from(assigned[1])).await,
            assigned[1..],
            "the event at `from` is included"
        );
    }

    /// AC-006. `from` is a threshold, not a seek: a position nothing occupies
    /// yields the next match above it rather than erroring or coming back empty.
    #[wasm_bindgen_test]
    async fn read_from_a_gap_position() {
        let (sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two"]).await;

        // A gap this store is allowed to leave, arranged store-side rather than
        // asserted as a literal.
        sql.exec(
            "DELETE FROM event WHERE position = ?",
            &[SqlValue::Integer(super::position_as_i64(assigned[0]))],
        )
        .expect("the row is removed");

        assert_eq!(
            read_positions(&store, &Query::all(), ReadOptions::new().from(assigned[0])).await,
            assigned[1..],
            "a read from a gap yields the next match above it"
        );
    }

    /// AC-006. `to` is inclusive, and composes with the internal ceiling as the
    /// tighter of the two.
    #[wasm_bindgen_test]
    async fn read_to_is_inclusive() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three"]).await;

        assert_eq!(
            read_positions(&store, &Query::all(), ReadOptions::new().to(assigned[1])).await,
            assigned[..2],
            "the event at `to` is included and nothing above it is"
        );
    }

    /// AC-006. `from` and `to` bound a closed window.
    #[wasm_bindgen_test]
    async fn read_from_and_to_bound_a_closed_window() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three", "Four"]).await;

        assert_eq!(
            read_positions(
                &store,
                &Query::all(),
                ReadOptions::new().from(assigned[1]).to(assigned[2]),
            )
            .await,
            assigned[1..3]
        );
    }

    /// AC-006. Backwards: `from` stays the higher bound, and the order reverses.
    #[wasm_bindgen_test]
    async fn read_backwards_from_with_limit() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three", "Four"]).await;

        assert_eq!(
            read_positions(
                &store,
                &Query::all(),
                ReadOptions::new().backwards().from(assigned[2]).limit(2),
            )
            .await,
            [assigned[2], assigned[1]],
            "backwards from the third event, two of them, newest first"
        );
    }

    /// AC-006. Backwards, `to` is the *older* end.
    #[wasm_bindgen_test]
    async fn read_to_under_backwards_bounds_the_older_end() {
        let (_sql, store) = open_paged(1);
        let assigned = append_each(&store, &["One", "Two", "Three"]).await;

        assert_eq!(
            read_positions(
                &store,
                &Query::all(),
                ReadOptions::new().backwards().to(assigned[1]),
            )
            .await,
            [assigned[2], assigned[1]],
            "`to` stops the walk at the older end"
        );
    }

    /// AC-006. The caller's budget is spent on **matched** events only: a page
    /// that scanned rows the query item excluded must not consume it.
    #[wasm_bindgen_test]
    async fn read_limit_applies_after_filtering() {
        let (_sql, store) = open_paged(1);
        let mut wanted = Vec::new();
        for event_type in ["Skip", "Want", "Skip", "Want", "Skip", "Want"] {
            let position = store
                .append(&[event(event_type)], None)
                .await
                .expect("the append lands");
            if event_type == "Want" {
                wanted.push(position);
            }
        }
        let query = Query::from_item(of_types(&["Want"]));

        assert_eq!(
            read_positions(&store, &query, ReadOptions::new().limit(2)).await,
            wanted[..2],
            "two matched events, not two scanned rows"
        );
    }

    /// AC-006. The same, backwards.
    #[wasm_bindgen_test]
    async fn read_backwards_limit_applies_after_filtering() {
        let (_sql, store) = open_paged(1);
        let mut wanted = Vec::new();
        for event_type in ["Want", "Skip", "Want", "Skip", "Want"] {
            let position = store
                .append(&[event(event_type)], None)
                .await
                .expect("the append lands");
            if event_type == "Want" {
                wanted.push(position);
            }
        }
        let query = Query::from_item(of_types(&["Want"]));

        assert_eq!(
            read_positions(&store, &query, ReadOptions::new().backwards().limit(2)).await,
            [wanted[2], wanted[1]]
        );
    }

    /// AC-006. `limit(0)` yields nothing — the deliberate divergence from the
    /// reference implementation's falsy zero, and the one that stops a paging
    /// loop turning into a full scan at parity.
    #[wasm_bindgen_test]
    async fn read_limit_zero_yields_nothing() {
        let (sql, store) = open();
        append_each(&store, &["One", "Two"]).await;

        let before = statements(&sql).len();
        let items = drain(store.read(&Query::all(), ReadOptions::new().limit(0))).await;

        assert!(items.is_empty(), "limit(0) reads nothing: {items:?}");
        assert_eq!(
            statements(&sql).len(),
            before,
            "and it reads nothing by not asking the store at all"
        );
    }

    /// AC-006. The budget is the query's, not each item's.
    #[wasm_bindgen_test]
    async fn limit_applies_across_items_not_per_item() {
        let (_sql, store) = open_paged(1);
        append_each(&store, &["Alpha", "Beta", "Alpha", "Beta"]).await;
        let query =
            Query::from_items([of_types(&["Alpha"]), of_types(&["Beta"])]).expect("a valid query");

        assert_eq!(
            read_positions(&store, &query, ReadOptions::new().limit(3))
                .await
                .len(),
            3,
            "three events across the whole query, not three per item"
        );
    }

    /// AC-006. Tags AND within an item with **superset** matching, and types OR
    /// within an item.
    #[wasm_bindgen_test]
    async fn tags_and_within_an_item_types_or_within_an_item() {
        let (_sql, store) = open_paged(1);
        let superset = store
            .append(&[tagged("Alpha", &["seat:A1", "order:7", "extra:1"])], None)
            .await
            .expect("the append lands");
        store
            .append(&[tagged("Alpha", &["seat:A1"])], None)
            .await
            .expect("the append lands");
        let other_type = store
            .append(&[tagged("Beta", &["seat:A1", "order:7"])], None)
            .await
            .expect("the append lands");

        let query = Query::from_item(
            QueryItem::new(
                ["Alpha".to_owned(), "Beta".to_owned()],
                tag_set(&["seat:A1", "order:7"]),
            )
            .expect("a valid item"),
        );

        assert_eq!(
            read_positions(&store, &query, ReadOptions::new()).await,
            [superset, other_type],
            "a superset matches, a partial overlap does not, and both types match"
        );
    }

    /// AC-006. An item naming no tags still matches an untagged event — which is
    /// why `Query::all` does not go through the tag index at all.
    #[wasm_bindgen_test]
    async fn an_item_naming_no_tags_matches_an_untagged_event() {
        let (_sql, store) = open_paged(1);
        let untagged = store
            .append(&[event("Alpha")], None)
            .await
            .expect("the append lands");

        assert_eq!(
            read_positions(
                &store,
                &Query::from_item(of_types(&["Alpha"])),
                ReadOptions::new(),
            )
            .await,
            [untagged]
        );
    }

    /// AC-007. A column count that disagrees with the `SELECT` is reported, not
    /// mis-decoded: the schema on disk is not the schema this build expects.
    #[wasm_bindgen_test]
    fn decode_row_reports_row_shape() {
        let mut values = well_formed_row();
        values.pop();

        assert!(
            matches!(
                super::decode_row(&row(values)),
                Err(CloudflareEventStoreError::RowShape { .. })
            ),
            "a short row is reported"
        );
    }

    /// AC-007. An undecodable `SqlValue` names its column.
    #[wasm_bindgen_test]
    fn decode_row_reports_column_type() {
        let mut values = well_formed_row();
        values[2] = SqlValue::Text("not a blob".to_owned());

        assert!(
            matches!(
                super::decode_row(&row(values)),
                Err(CloudflareEventStoreError::ColumnType { column: "data" })
            ),
            "the offending column is named"
        );
    }

    /// AC-007. A stored event type that no longer validates is reported rather
    /// than handed back.
    #[wasm_bindgen_test]
    fn decode_row_reports_stored_event_type() {
        let mut values = well_formed_row();
        values[1] = SqlValue::Text(String::new());

        assert!(
            matches!(
                super::decode_row(&row(values)),
                Err(CloudflareEventStoreError::StoredEventType(_))
            ),
            "a stored type that fails validation is reported"
        );
    }

    /// AC-007. The 2^53 boundary on the way *out*, constructed rather than
    /// asserted as a number: a stored position Workers SQL cannot round-trip
    /// arrives back as [`SqlValue::Real`], and the decoder reports it.
    #[wasm_bindgen_test]
    fn decode_row_reports_stored_position_at_the_boundary() {
        let mut values = well_formed_row();
        #[allow(clippy::cast_precision_loss)]
        let above = (super::MAX_SAFE_POSITION as f64) + 2.0;
        values[0] = SqlValue::Real(above);

        assert!(
            matches!(
                super::decode_row(&row(values)),
                Err(CloudflareEventStoreError::StoredPosition { .. })
            ),
            "a position this runtime cannot round-trip is reported, never narrowed"
        );
    }

    /// AC-007. The same fact reached the way a caller reaches it — as an **error
    /// item on the stream**, never a panic and never a narrowed position.
    #[wasm_bindgen_test]
    async fn an_unrepresentable_position_is_an_error_item_on_the_stream() {
        let (sql, store) = open();
        // A literal rather than a binding: binding it would widen it through a
        // JS number on the way *in*, and the fact under test is the way out.
        sql.exec(
            "INSERT INTO event (position, event_type, data, tags, recorded_at) \
             VALUES (9007199254740993, 'Unreachable', x'00', x'1f', 0)",
            &[],
        )
        .expect("the seeded row lands");

        let items = drain(store.read(&Query::all(), ReadOptions::new())).await;
        assert!(
            items
                .iter()
                .any(|item| matches!(item, Err(CloudflareEventStoreError::StoredPosition { .. }))),
            "the caller receives the error rather than a narrowed position: {items:?}"
        );
    }

    /// AC-007. Payloads stay opaque, and `metadata: None` and `Some(<empty>)`
    /// stay two distinguishable values.
    #[wasm_bindgen_test]
    async fn metadata_none_and_some_empty_stay_distinguishable() {
        let (_sql, store) = open_paged(1);
        store
            .append(&[event("NoMetadata")], None)
            .await
            .expect("the append lands");
        store
            .append(&[event("EmptyMetadata").with_metadata(&b""[..])], None)
            .await
            .expect("the append lands");

        let items = drain(store.read(&Query::all(), ReadOptions::new())).await;
        let [Ok(none), Ok(empty)] = items.as_slice() else {
            panic!("two events were appended: {items:?}");
        };

        assert!(none.event.metadata().is_none(), "absent stays absent");
        assert_eq!(
            empty
                .event
                .metadata()
                .map(happenstance_core::bytes::Bytes::len),
            Some(0),
            "and an empty payload stays present and empty"
        );
    }

    /// AC-007. `recorded_at` is returned **as stored**. A read that stamped
    /// `now()` would lose the time the store accepted the event at.
    #[wasm_bindgen_test]
    fn decode_row_returns_recorded_at_as_stored() {
        let mut values = well_formed_row();
        values[7] = SqlValue::Integer(1_700_000_000_000);

        let decoded = super::decode_row(&row(values)).expect("the row decodes");
        assert_eq!(
            decoded.recorded_at,
            RecordedAt::from_millis(1_700_000_000_000)
        );
    }

    /// AC-007. The identity comes from the *stored* origin pair, not from this
    /// object's incarnation plus the row's own position — the two agree for a
    /// locally appended event and disagree for every ingested one.
    #[wasm_bindgen_test]
    fn decode_row_rebuilds_the_identity_from_the_stored_origin() {
        let mut values = well_formed_row();
        values[0] = SqlValue::Integer(9);
        values[5] = SqlValue::Blob(vec![7_u8; 16]);
        values[6] = SqlValue::Integer(4);

        let decoded = super::decode_row(&row(values)).expect("the row decodes");
        assert_eq!(
            decoded.id,
            EventId::new(
                StoreId::from_bytes([7_u8; 16]),
                SequencePosition::new(4).expect("four is a position"),
            ),
        );
        assert_ne!(
            decoded.id.position(),
            decoded.position,
            "an ingested row's identity is not its local coordinate"
        );
    }

    /// The control on the whole decode-error family: the well-formed row the
    /// tests above break one field of at a time really does decode, so those
    /// assertions are not a decoder that refuses everything.
    #[wasm_bindgen_test]
    fn the_well_formed_row_decodes() {
        let decoded = super::decode_row(&row(well_formed_row())).expect("the control row decodes");
        assert_eq!(decoded.event_type().as_str(), "SeatMapPublished");
        assert!(decoded.event.tags().is_empty());
    }
}
