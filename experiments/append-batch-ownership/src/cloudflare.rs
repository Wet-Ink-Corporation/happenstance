//! The three Cloudflare arms, over one Durable Object host.
//!
//! All three write the adapter's schema — created by the real
//! `CloudflareEventStore::migrate` — and stamp their rows with the object's real
//! incarnation, so the real `CloudflareEventStore::read` reads any arm's rows
//! back. That is what `tests/cloudflare.rs` compares, before anything is timed.
//!
//! **What is copied verbatim and what is not.** `write_rows`, `write_tag_rows`
//! and the identity stamp (`crates/happenstance-cloudflare/src/event_store.rs`,
//! `write_batch` onwards) are copied statement for statement and value for
//! value. Three things are not, and none is on the path the arms differ on:
//!
//! * the compensating discard on a failed batch — no measured batch fails;
//! * `now()` — a fixed stamp instead, because `Date.now()` is one JS call per
//!   append in every arm and the `f64` narrowing would need an `as` cast;
//! * the append condition — a [`Fence`] on the boundary tag, evaluated by one
//!   `SELECT max(position)` statement, the same in every arm. The adapter's
//!   `evaluate` chunks an arbitrary `Query`; the contention scenario needs one
//!   tag, and the B0 calibration runs with no condition at all.

mod binding;

use std::fmt;

use happenstance_cloudflare::host::DurableObjectHost;
use happenstance_cloudflare::worker::SqlStorageValue;
use happenstance_cloudflare::worker::wasm_bindgen::JsValue;
use happenstance_cloudflare::{
    CloudflareEventStore, CloudflareEventStoreError, SqlError, SqlRow, SqlStorage, SqlValue,
};
use happenstance_core::{Event, EventParts, InvalidTag, SequencePosition, StoreId, Tag, Tags};

use self::binding::Raw;
use crate::batch::BOUNDARY;

/// The adapter's `event` table, unprefixed.
const EVENT: &str = "event";
/// The adapter's `event_tag` table, unprefixed.
const EVENT_TAG: &str = "event_tag";
/// The adapter's tag delimiter (`event_store.rs`, `UNIT`).
const UNIT: u8 = 0x1f;
/// The stamp every arm writes; see the module documentation.
const RECORDED_AT_MILLIS: i64 = 1_767_225_600_000;
/// The adapter's declared ceilings (`Ceilings::DECLARED`), checked by every arm
/// before any SQL as the adapter does.
const MAX_EVENTS_PER_BATCH: usize = 1024;
const MAX_EVENT_DATA_LEN: usize = 1024 * 1024;
const MAX_TAGS_PER_EVENT: usize = 1024;

/// Why an arm failed. Every variant but [`ArmError::Conflict`] is a defect in
/// the experiment, and is reported rather than panicked on.
#[derive(Debug)]
pub enum ArmError {
    /// The published binding failed.
    Sql(SqlError),
    /// The real adapter failed.
    Store(CloudflareEventStoreError),
    /// The `worker` binding failed.
    Worker(happenstance_cloudflare::worker::Error),
    /// A JS call outside `worker` threw.
    Js(JsValue),
    /// A statement returned no row, or a row of the wrong shape.
    Row(&'static str),
    /// A column held something these statements never write.
    Column(&'static str),
    /// The batch was empty.
    NoEvents,
    /// The batch crossed a declared ceiling.
    Ceiling,
    /// The fence matched the event at this position. The one expected refusal.
    Conflict(SequencePosition),
}

impl fmt::Display for ArmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<SqlError> for ArmError {
    fn from(err: SqlError) -> Self {
        Self::Sql(err)
    }
}

/// One fresh, migrated Durable Object, and the real adapter over it.
#[derive(Debug)]
pub struct Subject {
    /// Held so the object outlives every handle onto it.
    _host: DurableObjectHost,
    real: CloudflareEventStore,
    sql: SqlStorage,
}

impl Subject {
    /// A fresh object, migrated by the real adapter.
    ///
    /// # Errors
    ///
    /// [`ArmError::Store`] if the migration fails.
    pub fn open() -> Result<Self, ArmError> {
        let host = DurableObjectHost::new();
        let real = CloudflareEventStore::new(host.storage());
        real.migrate().map_err(ArmError::Store)?;
        let sql = host.storage();
        Ok(Self {
            _host: host,
            real,
            sql,
        })
    }

    /// The real adapter over this object.
    #[must_use]
    pub fn real(&self) -> &CloudflareEventStore {
        &self.real
    }

    /// Removes every event and tag row, leaving the schema and the incarnation.
    /// Run between repetitions, outside the timed region.
    ///
    /// # Errors
    ///
    /// [`ArmError::Sql`] if a `DELETE` fails.
    pub fn clear(&self) -> Result<(), ArmError> {
        self.sql.exec(&format!("DELETE FROM {EVENT_TAG}"), &[])?;
        self.sql.exec(&format!("DELETE FROM {EVENT}"), &[])?;
        Ok(())
    }

    /// The object's incarnation, read back as the adapter reads it.
    fn store_id(&self) -> Result<StoreId, ArmError> {
        let mut cursor = self.sql.exec(
            "SELECT v AS store_id FROM store_meta WHERE k = ?",
            &[SqlValue::Text("store_id".to_owned())],
        )?;
        let row = cursor
            .next_row()
            .transpose()?
            .ok_or(ArmError::Row("store_meta holds no incarnation"))?;
        match row.values() {
            [SqlValue::Blob(bytes)] => <[u8; 16]>::try_from(bytes.as_slice())
                .map(StoreId::from_bytes)
                .map_err(|_| ArmError::Column("store_meta.v")),
            _ => Err(ArmError::Row("store_meta.v")),
        }
    }
}

/// A one-tag append condition: refuse if an event carrying the boundary tag
/// sits above `after`.
#[derive(Debug, Clone)]
pub struct Fence {
    tag: Tag,
    after: Option<SequencePosition>,
}

impl Fence {
    /// The fence on [`BOUNDARY`] a contender decided at `after`.
    ///
    /// # Errors
    ///
    /// [`InvalidTag`] if [`BOUNDARY`] is not a valid tag, which it is.
    pub fn on_boundary(after: Option<SequencePosition>) -> Result<Self, InvalidTag> {
        let (key, value) = BOUNDARY;
        Ok(Self {
            tag: Tag::key_value(key, value)?,
            after,
        })
    }

    fn statement() -> String {
        format!("SELECT max(position) AS position FROM {EVENT_TAG} WHERE tag = ? AND position > ?")
    }

    fn after(&self) -> i64 {
        self.after.map_or(0, position_as_i64)
    }
}

fn position_as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

fn decode_position(value: &SqlValue) -> Result<SequencePosition, ArmError> {
    match value {
        SqlValue::Integer(raw) => u64::try_from(*raw)
            .ok()
            .and_then(SequencePosition::new)
            .ok_or(ArmError::Column("position")),
        _ => Err(ArmError::Column("position")),
    }
}

/// The single position a `RETURNING position` or `max(position)` row holds;
/// `None` for SQL `NULL`.
fn position_in(row: Option<SqlRow>) -> Result<Option<SequencePosition>, ArmError> {
    let row = row.ok_or(ArmError::Row("a single-row statement yielded nothing"))?;
    match row.values() {
        [SqlValue::Null] => Ok(None),
        [value] => decode_position(value).map(Some),
        _ => Err(ArmError::Row("expected one column")),
    }
}

/// `encode_tags`, verbatim (`event_store.rs`).
fn encode_tags(tags: &Tags) -> Vec<u8> {
    let mut out = Vec::with_capacity(tags.len() * 16 + 1);
    out.push(UNIT);
    for tag in tags {
        out.extend_from_slice(tag.as_str().as_bytes());
        out.push(UNIT);
    }
    out
}

/// `check_ceilings`, over `(payload length, tag count)` per event, so the owned
/// arm checks before it consumes. Alloc-free, as the adapter's is; every arm
/// runs it so that none is faster for skipping it.
fn check_ceilings(
    batch: usize,
    shapes: impl IntoIterator<Item = (usize, usize)>,
) -> Result<(), ArmError> {
    if batch == 0 {
        return Err(ArmError::NoEvents);
    }
    if batch > MAX_EVENTS_PER_BATCH {
        return Err(ArmError::Ceiling);
    }
    for (data, tags) in shapes {
        if data > MAX_EVENT_DATA_LEN || tags > MAX_TAGS_PER_EVENT {
            return Err(ArmError::Ceiling);
        }
    }
    Ok(())
}

fn insert_statement() -> String {
    format!(
        "INSERT INTO {EVENT} (event_type, data, metadata, tags, recorded_at) \
         VALUES (?, ?, ?, ?, ?) RETURNING position"
    )
}

fn tag_statement() -> String {
    format!("INSERT INTO {EVENT_TAG} (tag, position, event_type) VALUES (?, ?, ?)")
}

fn stamp_statement() -> String {
    format!(
        "UPDATE {EVENT} SET origin_store = ?, origin_position = position \
         WHERE position >= ? AND origin_position IS NULL"
    )
}

/// **B0** — the adapter's write path as shipped, through the published
/// `SqlStorage::exec`, `to_binding` clone and all.
#[derive(Debug)]
pub struct B0 {
    sql: SqlStorage,
    store_id: StoreId,
}

impl B0 {
    /// B0 over `subject`'s object.
    ///
    /// # Errors
    ///
    /// [`ArmError`] if the incarnation cannot be read.
    pub fn over(subject: &Subject) -> Result<Self, ArmError> {
        Ok(Self {
            sql: subject.sql.clone(),
            store_id: subject.store_id()?,
        })
    }

    /// Appends `events`, borrowed, as `CloudflareEventStore::append` does.
    ///
    /// # Errors
    ///
    /// [`ArmError::Conflict`] when `fence` matches; any other variant is a
    /// defect.
    pub fn append(
        &self,
        events: &[Event],
        fence: Option<&Fence>,
    ) -> Result<SequencePosition, ArmError> {
        check_ceilings(
            events.len(),
            events
                .iter()
                .map(|event| (event.data().len(), event.tags().len())),
        )?;
        if let Some(fence) = fence {
            let mut cursor = self.sql.exec(
                &Fence::statement(),
                &[
                    SqlValue::Text(fence.tag.as_str().to_owned()),
                    SqlValue::Integer(fence.after()),
                ],
            )?;
            if let Some(conflict) = position_in(cursor.next_row().transpose()?)? {
                return Err(ArmError::Conflict(conflict));
            }
        }
        // `write_batch`: owned out here so a failed batch's positions survive.
        let mut positions = Vec::with_capacity(events.len());
        self.write_rows(events, &mut positions)
    }

    fn write_rows(
        &self,
        events: &[Event],
        positions: &mut Vec<SequencePosition>,
    ) -> Result<SequencePosition, ArmError> {
        let insert = insert_statement();
        for event in events {
            let metadata = event
                .metadata()
                .map_or(SqlValue::Null, |bytes| SqlValue::Blob(bytes.to_vec()));
            let mut cursor = self.sql.exec(
                &insert,
                &[
                    SqlValue::Text(event.event_type().as_str().to_owned()),
                    SqlValue::Blob(event.data().to_vec()),
                    metadata,
                    SqlValue::Blob(encode_tags(event.tags())),
                    SqlValue::Integer(RECORDED_AT_MILLIS),
                ],
            )?;
            let position = position_in(cursor.next_row().transpose()?)?
                .ok_or(ArmError::Row("RETURNING position was NULL"))?;
            positions.push(position);
        }

        let statement = tag_statement();
        for (event, position) in events.iter().zip(positions.iter()) {
            for tag in event.tags() {
                self.sql.exec(
                    &statement,
                    &[
                        SqlValue::Text(tag.as_str().to_owned()),
                        SqlValue::Integer(position_as_i64(*position)),
                        SqlValue::Text(event.event_type().as_str().to_owned()),
                    ],
                )?;
            }
        }

        if let Some(first) = positions.first() {
            self.sql.exec(
                &stamp_statement(),
                &[
                    SqlValue::Blob(self.store_id.to_bytes().to_vec()),
                    SqlValue::Integer(position_as_i64(*first)),
                ],
            )?;
        }
        positions.last().copied().ok_or(ArmError::NoEvents)
    }
}

/// What B1 and O1 share: the consuming binding and the stamp.
#[derive(Debug)]
struct Moving {
    raw: Raw,
    store_id: StoreId,
}

impl Moving {
    fn over(subject: &Subject) -> Result<Self, ArmError> {
        Ok(Self {
            raw: Raw::under(&subject.sql)?,
            store_id: subject.store_id()?,
        })
    }

    fn check(&self, fence: Option<&Fence>) -> Result<(), ArmError> {
        let Some(fence) = fence else {
            return Ok(());
        };
        let cursor = self.raw.exec(
            &Fence::statement(),
            vec![
                SqlStorageValue::String(fence.tag.as_str().to_owned()),
                SqlStorageValue::Integer(fence.after()),
            ],
        )?;
        match position_in(cursor.next_row().transpose()?)? {
            Some(conflict) => Err(ArmError::Conflict(conflict)),
            None => Ok(()),
        }
    }

    /// One `event` row; returns the position it was given.
    fn insert(
        &self,
        insert: &str,
        bound: Vec<SqlStorageValue>,
    ) -> Result<SequencePosition, ArmError> {
        let cursor = self.raw.exec(insert, bound)?;
        position_in(cursor.next_row().transpose()?)?
            .ok_or(ArmError::Row("RETURNING position was NULL"))
    }

    /// The tag rows and the stamp, from `(event_type, tags)` per event.
    fn finish<'a>(
        &self,
        rows: impl Iterator<Item = (&'a str, &'a Tags)>,
        positions: &[SequencePosition],
    ) -> Result<SequencePosition, ArmError> {
        let statement = tag_statement();
        for ((event_type, tags), position) in rows.zip(positions) {
            for tag in tags {
                self.raw.exec(
                    &statement,
                    vec![
                        SqlStorageValue::String(tag.as_str().to_owned()),
                        SqlStorageValue::Integer(position_as_i64(*position)),
                        SqlStorageValue::String(event_type.to_owned()),
                    ],
                )?;
            }
        }
        if let Some(first) = positions.first() {
            self.raw.exec(
                &stamp_statement(),
                vec![
                    SqlStorageValue::Blob(self.store_id.to_bytes().to_vec()),
                    SqlStorageValue::Integer(position_as_i64(*first)),
                ],
            )?;
        }
        positions.last().copied().ok_or(ArmError::NoEvents)
    }
}

/// **B1** — the batch borrowed, each payload copied once (`to_vec`) and moved
/// into its binding: borrowed, one Rust copy per payload, with no signature
/// change. Not the best `&[Event]` can do: `SqlStorage::exec_raw`
/// (`worker-0.8.5/src/sql.rs:220`) would bind from the borrow with no Rust copy.
#[derive(Debug)]
pub struct B1(Moving);

impl B1 {
    /// B1 over `subject`'s object.
    ///
    /// # Errors
    ///
    /// [`ArmError`] if the binding or the incarnation cannot be reached.
    pub fn over(subject: &Subject) -> Result<Self, ArmError> {
        Moving::over(subject).map(Self)
    }

    /// Appends `events`, borrowed.
    ///
    /// # Errors
    ///
    /// As [`B0::append`].
    pub fn append(
        &self,
        events: &[Event],
        fence: Option<&Fence>,
    ) -> Result<SequencePosition, ArmError> {
        check_ceilings(
            events.len(),
            events
                .iter()
                .map(|event| (event.data().len(), event.tags().len())),
        )?;
        self.0.check(fence)?;
        let mut positions = Vec::with_capacity(events.len());
        let insert = insert_statement();
        for event in events {
            let metadata = event.metadata().map_or(SqlStorageValue::Null, |bytes| {
                SqlStorageValue::Blob(bytes.to_vec())
            });
            positions.push(self.0.insert(
                &insert,
                vec![
                    SqlStorageValue::String(event.event_type().as_str().to_owned()),
                    SqlStorageValue::Blob(event.data().to_vec()),
                    metadata,
                    SqlStorageValue::Blob(encode_tags(event.tags())),
                    SqlStorageValue::Integer(RECORDED_AT_MILLIS),
                ],
            )?);
        }
        self.0.finish(
            events
                .iter()
                .map(|event| (event.event_type().as_str(), event.tags())),
            &positions,
        )
    }
}

/// **O1** — the batch owned; each payload and metadata buffer moved into its
/// binding, copied only where `Vec::from(Bytes)` must copy (a shared buffer).
#[derive(Debug)]
pub struct O1(Moving);

impl O1 {
    /// O1 over `subject`'s object.
    ///
    /// # Errors
    ///
    /// [`ArmError`] if the binding or the incarnation cannot be reached.
    pub fn over(subject: &Subject) -> Result<Self, ArmError> {
        Moving::over(subject).map(Self)
    }

    /// Appends `events`, owned. The batch is consumed whatever the outcome: a
    /// refused batch is gone, which is ADR-0012 item 5's problem and what the
    /// caller-side scenario prices.
    ///
    /// # Errors
    ///
    /// As [`B0::append`].
    pub fn append_owned(
        &self,
        events: Vec<Event>,
        fence: Option<&Fence>,
    ) -> Result<SequencePosition, ArmError> {
        check_ceilings(
            events.len(),
            events
                .iter()
                .map(|event| (event.data().len(), event.tags().len())),
        )?;
        self.0.check(fence)?;
        // `Event` and `EventParts` have the same fields, so this collect reuses
        // the batch's buffer in place; the payloads are then *taken* out of the
        // parts, leaving the type and tags for the tag rows that follow.
        let mut parts: Vec<EventParts> = events.into_iter().map(Event::into_parts).collect();
        let mut positions = Vec::with_capacity(parts.len());
        let insert = insert_statement();
        for part in &mut parts {
            let data = Vec::<u8>::from(std::mem::take(&mut part.data));
            let metadata = part.metadata.take().map_or(SqlStorageValue::Null, |bytes| {
                SqlStorageValue::Blob(Vec::from(bytes))
            });
            positions.push(self.0.insert(
                &insert,
                vec![
                    SqlStorageValue::String(part.event_type.as_str().to_owned()),
                    SqlStorageValue::Blob(data),
                    metadata,
                    SqlStorageValue::Blob(encode_tags(&part.tags)),
                    SqlStorageValue::Integer(RECORDED_AT_MILLIS),
                ],
            )?);
        }
        self.0.finish(
            parts
                .iter()
                .map(|part| (part.event_type.as_str(), &part.tags)),
            &positions,
        )
    }
}
