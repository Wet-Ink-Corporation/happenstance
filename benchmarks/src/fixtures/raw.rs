//! The floor: what SQLite does on its own, so the published number is
//! happenstance's cost rather than SQLite's speed.
//!
//! # The question this answers, and why nothing else in the repository does
//!
//! `references/seeds/measured-not-claimed.md:100-104`:
//!
//! > The question an adopter actually asks is not how fast happenstance is; it
//! > is **what happenstance costs over the database they already run**. That
//! > number — the overhead above a raw insert on the same file or the same
//! > instance — is the one nobody can dispute and nobody can be sold, and it
//! > has never been taken.
//!
//! # Two floors, because "raw SQLite" is two different questions
//!
//! [`Floor::SameSchema`] runs hand-written SQL against **happenstance's own
//! schema**, on a file the adapter itself migrated. It isolates the cost of the
//! adapter's Rust — query planning, tag dedup, the connection mutex, the paged
//! read, the `spawn_blocking` hop — from the cost of the schema those
//! decisions were made for. It is the number that answers *"what does the
//! library layer cost me?"*
//!
//! [`Floor::HandRolled`] runs against the single-table event log an engineer
//! writes on day one: an autoincrement position, a type, a payload, and tags in
//! one column. No `event_tag` join table, no `tag_cardinality`, no `store_meta`,
//! no `origin_*` columns. It answers the wider and less flattering question:
//! *"what does the whole design — library and schema together — cost me over
//! what I would have written myself?"*
//!
//! Both are reported. Publishing only the first would price the adapter against
//! a schema chosen for capabilities the hand-rolled table does not have, and
//! publishing only the second would charge the library for a schema decision
//! ADR-0022 took deliberately.
//!
//! # The schema is never restated, and that is load-bearing
//!
//! `MIGRATION_1` is a private constant in
//! `crates/happenstance-sqlite/src/event_store.rs:202`. A copy of it here would
//! be a drift risk that no gate could see — and a floor measured against a
//! schema that has silently diverged is not a floor. So [`Floor::SameSchema`]
//! **opens a `SqliteEventStore` once, lets it migrate, and drops it**, then
//! works the file with a plain connection. The schema is the adapter's by
//! construction, and it cannot go stale.
//!
//! # This floor is deliberately cheaper than a correct store
//!
//! It writes no `EventId`, no `RecordedAt`, no `store_meta` incarnation, no
//! `tag_cardinality` upsert; it encodes the tags column as newline-joined UTF-8
//! rather than through the adapter's own encoder; and it evaluates no append
//! condition. Every one of those omissions makes it faster.
//!
//! That is the point. A floor is only useful if it is a genuine lower bound —
//! if there is no arrangement of raw SQL that could beat it — because then the
//! ratio above it is an *upper* bound on what happenstance costs. A "fair" floor
//! that re-implemented half the adapter would produce a smaller, more flattering
//! ratio and a weaker claim. Every table quoting a ratio carries this paragraph
//! by reference.
//!
//! # Neither side caches a prepared statement, and that is checked rather than
//! assumed
//!
//! The workspace pins `rusqlite` with `default-features = false`, which drops
//! its `cache` feature — so `Connection::prepare_cached` does not exist in this
//! graph at all, and `happenstance-sqlite` re-prepares on every call
//! (`crates/happenstance-sqlite/src/event_store.rs:698`, `:799`, `:1377`,
//! `query_sql.rs:101`). The floor therefore uses plain `prepare` too, and the
//! ratio between them says nothing about statement caching in either
//! direction. Had the floor cached where the adapter cannot, it would have won
//! a benchmark it was not running.
//!
//! # Pragmas come from the adapter, not from here
//!
//! [`open_configured`] is `happenstance-sqlite`'s own connection setup, and it
//! is what both floors use. Journal mode, `synchronous` and the busy timeout
//! are therefore identical to the arm being measured, and
//! [`RawStore::settings`] reads them back **off the live connection** rather
//! than trusting the `PRAGMA` that issued them — SQLite silently ignores a
//! `journal_mode` it cannot honour, and a floor that assumed WAL on a file
//! running in rollback mode would be measuring a different durability
//! guarantee.

use happenstance_core::Event;
use happenstance_sqlite::connection::{ConnectionSettings, open_configured};
use happenstance_sqlite::event_store::SqliteEventStore;
use rusqlite::Connection;

use crate::fixtures::sqlite::SqliteFixture;

/// Which floor a [`RawStore`] is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    /// Hand-written SQL against happenstance's own schema. Isolates the
    /// adapter's Rust from the schema it was written for.
    SameSchema,
    /// The minimal single-table event log an engineer writes on day one.
    /// Prices the library and its schema together.
    HandRolled,
}

impl Floor {
    /// Both floors, in the order a table should print them.
    pub const BOTH: [Self; 2] = [Self::SameSchema, Self::HandRolled];

    /// The short label a results column carries.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SameSchema => "raw/same-schema",
            Self::HandRolled => "raw/hand-rolled",
        }
    }
}

/// The hand-rolled table: what an engineer writes before they have met DCB.
///
/// One table, one index, tags in a column. Deliberately without the `event_tag`
/// join table `ADR-0022` decided on — a floor that had it would already be
/// paying for the capability that decision bought.
const HAND_ROLLED_DDL: &str = "\
CREATE TABLE IF NOT EXISTS event (
    position   INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT    NOT NULL,
    data       BLOB    NOT NULL,
    tags       TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS event_type_idx ON event(event_type, position);";

/// A raw connection onto a database, with the least code that could serve an
/// event log.
///
/// Not an `EventStore` and never will be: it has no condition evaluation, no
/// identity, no snapshot semantics and no error taxonomy, so it could not pass
/// one conformance rule. It is a control, and controls are allowed to be wrong
/// in stated ways.
#[derive(Debug)]
pub struct RawStore {
    connection: Connection,
    floor: Floor,
}

impl RawStore {
    /// Opens the floor over a fixture's own file.
    ///
    /// For [`Floor::SameSchema`] the file must already carry the adapter's
    /// schema; [`over`](Self::over) is the constructor that guarantees it.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the file cannot be opened or configured,
    /// or if the hand-rolled DDL is refused.
    pub fn open(path: &std::path::Path, floor: Floor) -> rusqlite::Result<Self> {
        let connection = open_configured(path)?;
        if floor == Floor::HandRolled {
            connection.execute_batch(HAND_ROLLED_DDL)?;
        }
        Ok(Self { connection, floor })
    }

    /// Builds a floor over a fresh file, migrating it with the **adapter** when
    /// the floor is [`Floor::SameSchema`].
    ///
    /// The `SqliteEventStore` is opened and immediately dropped: all it is here
    /// for is to run migration 1, so that the raw arm works the same schema the
    /// measured arm does without this crate ever restating it.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the migration or the subsequent open
    /// fails.
    ///
    /// # Panics
    ///
    /// Panics if the adapter refuses to open the fixture's file, which would
    /// mean the measurement environment is broken rather than that a floor is
    /// slow.
    pub fn over(fixture: &SqliteFixture, floor: Floor) -> rusqlite::Result<Self> {
        if floor == Floor::SameSchema {
            let migrator = SqliteEventStore::open(fixture.path())
                .expect("a broken measurement environment, not a finding");
            drop(migrator);
        }
        Self::open(fixture.path(), floor)
    }

    /// Which floor this is.
    pub const fn floor(&self) -> Floor {
        self.floor
    }

    /// The durability settings, **read back off the live connection**.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the pragmas cannot be read.
    pub fn settings(&self) -> rusqlite::Result<ConnectionSettings> {
        ConnectionSettings::read_back(&self.connection)
    }

    /// Appends a batch inside one `BEGIN IMMEDIATE` transaction.
    ///
    /// The transaction is `IMMEDIATE` rather than `DEFERRED` because that is
    /// what `SqliteEventStore::append_locked` takes
    /// (`crates/happenstance-sqlite/src/event_store.rs:505`). A floor that took
    /// a deferred lock would be measuring a different contention shape and
    /// would win on a benchmark it was not running.
    ///
    /// Returns the position of the last row written.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if any statement fails; the transaction rolls
    /// back.
    ///
    /// # Panics
    ///
    /// Panics if `events` is empty, which the contract refuses above the store
    /// and which here would return a position for a row nobody wrote.
    pub fn append(&mut self, events: &[Event]) -> rusqlite::Result<i64> {
        assert!(
            !events.is_empty(),
            "an empty batch is refused by the contract before the condition is \
             evaluated, so a floor that accepted one would be measuring \
             nothing and reporting it as throughput"
        );

        let transaction = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let mut last = 0_i64;

        {
            let mut insert_event = match self.floor {
                Floor::SameSchema => transaction.prepare(
                    "INSERT INTO event (event_type, data, metadata, tags, recorded_at) \
                     VALUES (?1, ?2, NULL, ?3, ?4)",
                )?,
                Floor::HandRolled => transaction
                    .prepare("INSERT INTO event (event_type, data, tags) VALUES (?1, ?2, ?3)")?,
            };
            let mut insert_tag = (self.floor == Floor::SameSchema)
                .then(|| {
                    transaction.prepare(
                        "INSERT OR IGNORE INTO event_tag (tag, position, event_type) \
                         VALUES (?1, ?2, ?3)",
                    )
                })
                .transpose()?;

            for event in events {
                let joined = joined_tags(event);
                match self.floor {
                    Floor::SameSchema => insert_event.execute(rusqlite::params![
                        event.event_type().as_str(),
                        event.data().as_ref(),
                        joined.as_bytes(),
                        0_i64,
                    ])?,
                    Floor::HandRolled => insert_event.execute(rusqlite::params![
                        event.event_type().as_str(),
                        event.data().as_ref(),
                        joined.as_str(),
                    ])?,
                };
                last = transaction.last_insert_rowid();

                if let Some(insert_tag) = insert_tag.as_mut() {
                    for tag in event.tags() {
                        insert_tag.execute(rusqlite::params![
                            tag.as_str(),
                            last,
                            event.event_type().as_str(),
                        ])?;
                    }
                }
            }
        }

        transaction.commit()?;
        Ok(last)
    }

    /// Reads every row, materialising the payloads.
    ///
    /// The payload is pulled out rather than counted so this is comparable with
    /// a `read` through the port, which yields whole `SequencedEvent`s. A floor
    /// that ran `SELECT count(*)` would beat the adapter by not doing the work.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the statement fails.
    pub fn read_all(&self) -> rusqlite::Result<usize> {
        let mut statement = self
            .connection
            .prepare("SELECT position, event_type, data FROM event ORDER BY position")?;
        let rows = statement.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;
        let mut seen = 0_usize;
        for row in rows {
            let _ = std::hint::black_box(row?);
            seen += 1;
        }
        Ok(seen)
    }

    /// Reads every row carrying `tag`.
    ///
    /// [`Floor::SameSchema`] joins `event_tag`, which is what the adapter does.
    /// [`Floor::HandRolled`] runs `LIKE` over the tags column, which is what an
    /// engineer writes before the join table exists — and it is a full scan, so
    /// this is the one arm where the hand-rolled floor is expected to *lose*.
    /// Reporting that is the point: the join table is a decision, and a floor
    /// that hid its benefit would make ADR-0022 look free.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the statement fails.
    pub fn read_tagged(&self, tag: &str) -> rusqlite::Result<usize> {
        let (sql, parameter) = match self.floor {
            Floor::SameSchema => (
                "SELECT e.position, e.event_type, e.data FROM event e \
                 JOIN event_tag t ON t.position = e.position \
                 WHERE t.tag = ?1 ORDER BY e.position",
                tag.to_owned(),
            ),
            Floor::HandRolled => (
                "SELECT position, event_type, data FROM event \
                 WHERE tags LIKE ?1 ORDER BY position",
                format!("%{tag}%"),
            ),
        };
        let mut statement = self.connection.prepare(sql)?;
        let rows = statement.query_map([parameter], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
            ))
        })?;
        let mut seen = 0_usize;
        for row in rows {
            let _ = std::hint::black_box(row?);
            seen += 1;
        }
        Ok(seen)
    }

    /// The highest position written, or `None` on an empty log.
    ///
    /// The floor's answer to `EventStore::head`.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if the statement fails.
    pub fn head(&self) -> rusqlite::Result<Option<i64>> {
        let mut statement = self.connection.prepare("SELECT max(position) FROM event")?;
        statement.query_row([], |row| row.get::<_, Option<i64>>(0))
    }
}

/// The tags column: newline-joined, and deliberately cheaper than the adapter's
/// own encoding.
///
/// See the module documentation — the floor is a lower bound on purpose, and
/// every way in which it is cheaper than a correct store makes the ratio above
/// it a stronger claim rather than a weaker one.
fn joined_tags(event: &Event) -> String {
    let mut joined = String::new();
    for (index, tag) in event.tags().iter().enumerate() {
        if index > 0 {
            joined.push('\n');
        }
        joined.push_str(tag.as_str());
    }
    joined
}
