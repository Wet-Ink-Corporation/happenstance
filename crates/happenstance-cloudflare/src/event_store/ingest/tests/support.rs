//! What the `wasm32` ingest tests share: a store over a fresh object with the
//! model installed, and raw-SQL oracles that read the tables directly and
//! share no subroutine with the code under test.

use happenstance_core::{
    Event, EventId, EventStore, Query, ReadOptions, RecordedAt, SequencePosition, SequencedEvent,
    StoreId, Tag, Tags, collect,
};
use happenstance_sync::ReplicatedEvent;

use crate::event_store::CloudflareEventStore;
use crate::event_store::ingest::{CloudflareIngestStore, model};
use crate::host::{DurableObjectHost, statements};
use crate::sql_storage::{SqlStorage, SqlValue};

/// One fresh object, migrated, with the model's `transactionSync` installed.
pub(super) struct Opened {
    /// The object, for a second handle or the model's call count.
    pub(super) host: DurableObjectHost,
    /// A handle onto its storage, for the raw-SQL oracles.
    pub(super) sql: SqlStorage,
    /// The store under test.
    pub(super) store: CloudflareIngestStore,
}

/// A fresh object and an ingest store over it.
pub(super) fn open() -> Opened {
    let host = DurableObjectHost::new();
    model::install(&host).expect("the model installs on the shim");
    let sql = host.storage();
    let event_store = CloudflareEventStore::new(sql.clone());
    event_store.migrate().expect("the schema applies");
    let store = connect(&host, event_store);
    Opened { host, sql, store }
}

/// An ingest store over `store`, transacting through `host`'s storage.
pub(super) fn connect(
    host: &DurableObjectHost,
    store: CloudflareEventStore,
) -> CloudflareIngestStore {
    CloudflareIngestStore::open(
        store,
        model::storage(host).expect("the shim's state carries a storage"),
    )
    .expect("the model's transactionSync is installed")
}

/// A local event of type `event_type` carrying `data`, and `tags`.
pub(super) fn local(event_type: &str, data: &[u8], tags: &[&str]) -> Event {
    let tags: Tags = tags
        .iter()
        .map(|tag| Tag::new((*tag).to_owned()).expect("a valid tag"))
        .collect();
    Event::new(event_type.to_owned(), data.to_vec())
        .expect("a valid event type")
        .with_tags(tags)
}

/// A position, from a literal the test chose.
pub(super) fn at(position: u64) -> SequencePosition {
    SequencePosition::new(position).expect("a nonzero position")
}

/// An event `origin` minted at `position`, recorded at `millis`, carrying
/// `data` and `tags`.
pub(super) fn minted(
    origin: StoreId,
    position: u64,
    millis: i64,
    data: &[u8],
    tags: &[&str],
) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(origin, at(position)),
        RecordedAt::from_millis(millis),
        local("Shipped", data, tags),
    )
}

/// Every event the store holds, through the port.
pub(super) async fn log<S: EventStore>(store: &S) -> Vec<SequencedEvent> {
    collect(store.read(&Query::all(), ReadOptions::new()))
        .await
        .expect("the log reads")
}

/// The one integer a single-row `SELECT` answers, or `None` for `NULL`.
pub(super) fn scalar(sql: &SqlStorage, statement: &str) -> Option<i64> {
    let mut cursor = sql.exec(statement, &[]).expect("the oracle reads");
    let row = cursor
        .next_row()
        .expect("the oracle answers one row")
        .expect("the oracle's row decodes");
    match row.values() {
        [SqlValue::Integer(value)] => Some(*value),
        [SqlValue::Null] => None,
        other => panic!("the oracle answered {other:?}"),
    }
}

/// What the tables hold, read raw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Raw {
    /// Rows in `event`, stamped or not.
    pub(super) events: Option<i64>,
    /// Rows in `event_tag`.
    pub(super) tags: Option<i64>,
    /// Rows in `event` with no origin, which are not events.
    pub(super) unstamped: Option<i64>,
    /// `sqlite_sequence`'s counter for `event`, the next positions' source.
    pub(super) sequence: Option<i64>,
}

/// The raw state of `sql`'s tables.
pub(super) fn raw(sql: &SqlStorage) -> Raw {
    Raw {
        events: scalar(sql, "SELECT count(*) FROM event"),
        tags: scalar(sql, "SELECT count(*) FROM event_tag"),
        unstamped: scalar(
            sql,
            "SELECT count(*) FROM event WHERE origin_position IS NULL",
        ),
        sequence: scalar(
            sql,
            "SELECT max(seq) FROM sqlite_sequence WHERE name = 'event'",
        ),
    }
}

/// The statements issued on `sql` since the `from`-th, without the model's
/// own transaction control: what the adapter issued.
pub(super) fn issued_since(sql: &SqlStorage, from: usize) -> Vec<String> {
    let mut issued = statements(sql);
    issued
        .split_off(from.min(issued.len()))
        .into_iter()
        .filter(|statement| !model::STATEMENTS.contains(&statement.as_str()))
        .collect()
}

/// How many statements `sql` has been asked to run so far.
pub(super) fn issued(sql: &SqlStorage) -> usize {
    statements(sql).len()
}
