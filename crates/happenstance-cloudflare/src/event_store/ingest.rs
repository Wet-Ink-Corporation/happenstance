//! VT-10's fourth run: `happenstance-sync`'s ingest port, implemented for the
//! Durable Object over the row writer `append` already uses.
//!
//! # What is being tested
//!
//! VT-10's falsifier reads *"a store adapter cannot implement `IngestStore`
//! without duplicating append's write path"*. SQLite ran it over a store that
//! serialises its writers, Postgres over one that does not, and Neon over one
//! with no connection; this is the run on the workspace's only `!Send` store.
//! A foreign row goes in through `append`'s own `INSERT`, byte for byte
//! (`CloudflareEventStore::insert_row`), and gets its foreign identity the way
//! an appended row gets its local one: through the `origin_position IS NULL`
//! marker. A group's compensation goes through `write_rows`, append's writer,
//! whole. What is ingest-only is small enough to name:
//!
//! * the foreign stamp, [`stamp_foreign_sql`], an `UPDATE OR IGNORE` whose
//!   `RETURNING` row says whether the stamp landed: `UNIQUE (origin_store,
//!   origin_position)` refuses a held identity, and SQLite skips the row;
//! * the discard of a skipped event's staged row, [`discard_staged_sql`];
//! * the own-identity check (SY-38), decided by the stamp's outcome and the
//!   head read once as the call's first read, with no lookup ahead of it;
//! * one `transactionSync` per call (below);
//! * the watermark, [`watermark_sql`], one `GROUP BY` over the origin index.
//!
//! No append condition is evaluated (SY-1).
//!
//! # Why a transaction here, and not in `append`
//!
//! `append`'s all-or-none rests on one statement, the identity `UPDATE` at the
//! end of the batch (`STAMPED`). An ingest cannot be carried that way: whether
//! each foreign row lands, whether a group's compensation is written (SY-11:
//! only if one of its foreign rows landed) and whether the call is refused
//! (SY-38: only if a claim landed) are all decided by the outcomes of the
//! stamps, and no single statement can depend on its own rows' outcomes. A
//! lookup ahead of the stamp is what ADR-0098 item 2 and VT-8 rule out, and two
//! stamps with a compensating undo is the `PartialBatch` shape ES-18 made this
//! crate repent of. So each non-empty call runs inside the platform's own
//! synchronous transaction, `ctx.storage.transactionSync`, which rolls back
//! every statement, the `AUTOINCREMENT` counter included, when the callback
//! throws: so every arm of [`IngestError`] means nothing in the call was
//! written, and a refused call leaves the next positions dense.
//!
//! The one exception is a commit that throws after the callback returned
//! (`Settled::Primitive`): the outcome is then uncertain, the call fails
//! `Stop`, and a re-delivery converges, because it is a skip (SY-11).
//!
//! # Why a wrapper, and not an impl on `CloudflareEventStore`
//!
//! For the transaction seam, not for the identity: since part E0 `store_id`
//! reads `store_meta` on every call, as `append` does, so no identity is cached
//! here and a re-mint on any handle reaches this one. The seam is the storage
//! object `transactionSync` lives on, which `CloudflareEventStore` is not built
//! from (it holds only `SqlStorage`). An `Option` field on the published struct
//! would make "an ingest store that cannot transact" representable; a wrapper
//! whose only constructors take the object's `DurableObjectState` does not
//! (API-02). `worker` binds no `transactionSync`, so the constructors read it
//! off the state's `storage` (the owner's D5 **B**, #108 part E).
//!
//! # Cancellation, and `Busy`
//!
//! **A dropped `ingest` future has already committed or rolled back.** The
//! body awaits nothing (`ingest_groups` is a plain `fn`), so a future polled
//! once has run to completion, which is `event_store`'s `# Cancellation` answer
//! for `append` (ES-23). Re-delivery is a skip, so reissuing the identical batch
//! is precisely safe (ES-24's shape).
//!
//! **`IngestError::Busy` is never produced.** A Durable Object is a
//! single-threaded actor with no other writer, for the reason
//! `CloudflareEventStoreError`'s `# Why this store never reports
//! AppendError::Busy` gives.
//!
//! # What has run where
//!
//! On `wasm32-unknown-unknown` under the gate's Node shim, whose
//! `transactionSync` is this module's `model` over `SAVEPOINT`, and under
//! `workerd`, local and deployed: `harness/workerd` runs the ingest family
//! through `namespaced_from_object_state` over a real object (#108 part E3).
//! The primitive itself is probed there by `/transaction-probe` routes. The
//! 1,024-event ceiling is the declared policy; below it, local `workerd`
//! commits every call at 1,024 events of 1 or of 1,024 tags (about two
//! statements and the tags per foreign row, in one transaction), measured in
//! `experiments/durable-object-limits/results/run-workerd-local-2026-10-31-ingest.txt`.
//!
//! # Why here, and why behind `sync`
//!
//! **Here**, as a child of `event_store`, because a module's private items are
//! visible to its descendants: this reaches the writer, the identity read, the
//! decoders and the ceilings without any of them being widened. **`sync`**
//! because the port is an optional dependency: a consumer who never replicates
//! compiles none of this. It was `cfg(test)` until part E published
//! `happenstance-sync`.

use core::slice;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, RecordedAt,
    SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
    Watermark,
};

use super::{
    CloudflareEventStore, CloudflareEventStoreError, MAX_SAFE_POSITION, decode_position,
    insert_statement, max_position, now, position_as_i64,
};
use worker::wasm_bindgen::JsValue;
use worker::worker_sys::DurableObjectState;

use crate::js::{JsHandle, JsThrow};
use crate::namespace::{TableNamespace, Tables};
use crate::sql_storage::{SqlError, SqlStorage, SqlValue, storage_from_durable_object_state};
use transaction::{Settled, Transactions};

/// The ingest-only stamp.
///
/// `OR IGNORE` is the dedupe: `UNIQUE (origin_store, origin_position)` refuses
/// a held identity and SQLite skips the row, so `RETURNING` yields a row only
/// for a stamp that landed (`harness/workerd`'s `update-or-ignore-returning`
/// probe measures it on the runtime). The `IS NULL` guard makes it
/// impossible to restamp an event.
fn stamp_foreign_sql(tables: &Tables) -> String {
    format!(
        "UPDATE OR IGNORE {} SET origin_store = ?, origin_position = ? \
         WHERE position = ? AND origin_position IS NULL RETURNING position",
        tables.event
    )
}

/// Removes the staged row of a skipped event.
///
/// Guarded on `IS NULL`, so it can only ever delete a row that is not an
/// event. It has no tag rows: tags are written only after a stamp lands.
fn discard_staged_sql(tables: &Tables) -> String {
    format!(
        "DELETE FROM {} WHERE position = ? AND origin_position IS NULL",
        tables.event
    )
}

/// The highest origin position held per origin store, this store's own
/// included. The `IS NOT NULL` is `STAMPED`: a staged row is not an event and
/// holds no origin. No frontier predicate: the watermark reports what the store
/// holds, which is VT-8's dedupe set.
fn watermark_sql(tables: &Tables) -> String {
    format!(
        "SELECT origin_store, max(origin_position) AS position FROM {} \
         WHERE origin_position IS NOT NULL GROUP BY origin_store",
        tables.event
    )
}

/// A Durable Object event store that can also ingest: the store, and the
/// object's storage, whose `transactionSync` makes one call one transaction.
///
/// It implements [`EventStore`] by delegating to the store it wraps, and
/// [`IngestStore`], `happenstance-sync`'s port, in the bare flavour only, as the
/// store does. Built only from the object's [`DurableObjectState`], because
/// that is where `transactionSync` lives; there is no ingest store that cannot
/// transact. The schema is the store's: run
/// [`CloudflareEventStore::migrate`] over the same object before the first
/// ingest, as before the first append.
///
/// `Clone` aliases the same object, as the store's own does.
///
/// # Examples
///
/// A `#[durable_object]` class receives a [`worker::State`]. `State::_inner`
/// hands back the raw state, and `State::from` restores it.
///
/// ```no_run
/// use happenstance_cloudflare::worker::State;
/// use happenstance_cloudflare::{CloudflareEventStore, CloudflareIngestStore, SqlStorage};
///
/// # fn build(state: State) -> Result<(State, CloudflareIngestStore), Box<dyn std::error::Error>> {
/// CloudflareEventStore::new(SqlStorage::from_state(&state)).migrate()?;
/// let raw = state._inner();
/// let store = CloudflareIngestStore::from_object_state(&raw)?;
/// let state = State::from(raw);
/// # Ok((state, store))
/// # }
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub struct CloudflareIngestStore {
    /// The store this ingests into. Owned, because the wrapper keeps it.
    store: CloudflareEventStore,
    /// The object's synchronous transaction.
    transactions: Transactions,
}

impl CloudflareIngestStore {
    /// An ingest store over the object `state` belongs to, keeping its log in
    /// the unprefixed tables [`CloudflareEventStore::new`] uses.
    ///
    /// Borrows `state`: it keeps a handle onto the object's storage, and binds
    /// its SQL storage as [`SqlStorage::from_state`] does.
    ///
    /// # Errors
    ///
    /// [`CloudflareEventStoreError::Sql`] if reading `state.storage` or
    /// `storage.transactionSync` throws, or if `transactionSync` is absent or
    /// not a function.
    ///
    /// # Panics
    ///
    /// A known limitation (#108): a `storage` getter that succeeds once and then
    /// throws reaches an `unwrap` inside `worker` 0.8.5's `State::storage`, which
    /// has no fallible public path.
    pub fn from_object_state(
        state: &DurableObjectState,
    ) -> Result<Self, CloudflareEventStoreError> {
        Self::over_object_state(state, CloudflareEventStore::new)
    }

    /// [`from_object_state`](Self::from_object_state), keeping this log in
    /// `namespace`'s own tables, as [`CloudflareEventStore::namespaced`] does.
    ///
    /// # Errors
    ///
    /// [`from_object_state`](Self::from_object_state)'s.
    ///
    /// # Panics
    ///
    /// [`from_object_state`](Self::from_object_state)'s.
    pub fn namespaced_from_object_state(
        state: &DurableObjectState,
        namespace: &TableNamespace,
    ) -> Result<Self, CloudflareEventStoreError> {
        Self::over_object_state(state, |sql| {
            CloudflareEventStore::namespaced(sql, namespace)
        })
    }

    /// The one path both public constructors take: the storage object first,
    /// read through the throwing getter so a throw is an error, and only then
    /// the SQL binding over it. That binding reads the same getter again
    /// through `worker::State::storage`, which unwraps; it is reached only
    /// after the first read has answered.
    fn over_object_state(
        state: &DurableObjectState,
        store: impl FnOnce(SqlStorage) -> CloudflareEventStore,
    ) -> Result<Self, CloudflareEventStoreError> {
        let storage = state
            .storage()
            .map_err(|thrown| SqlError::Thrown(JsThrow::new(&JsHandle::new(thrown))))?;
        // `worker::State::from` consumes, so it gets a clone of the value: a
        // second reference to the same JavaScript object, not a copy (H-08).
        // The path `SqlStorage::from_state` takes, so both bind one object.
        let sql = storage_from_durable_object_state(AsRef::<JsValue>::as_ref(state).clone());
        Self::open(store(sql), JsHandle::new(storage.into()))
    }

    /// An ingest store over `store`, transacting through `storage`'s
    /// `transactionSync`.
    ///
    /// # Errors
    ///
    /// [`Transactions::of`]'s: `storage` offers no `transactionSync`.
    fn open(
        store: CloudflareEventStore,
        storage: JsHandle,
    ) -> Result<Self, CloudflareEventStoreError> {
        Ok(Self {
            store,
            transactions: Transactions::of(storage)?,
        })
    }

    /// The ceilings, the origin checks, then one transaction, so every refusal
    /// but a failed write comes before the first statement.
    ///
    /// Synchronous, so it awaits nothing.
    fn ingest_groups(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<CloudflareEventStoreError>> {
        // An empty pull writes nothing, and compensation needs a landed event
        // (SY-11), so there is nothing to transact.
        if groups.iter().all(|group| group.events.is_empty()) {
            return Ok(Ingested::default());
        }

        // Against the worst case, the whole call: every foreign event and every
        // compensation, whichever of them turn out to be skips. Both totals,
        // the events and their tags, count compensation alike.
        let len = groups.iter().fold(0_usize, |len, group| {
            len.saturating_add(group.events.len())
                .saturating_add(group.compensation.len())
        });
        let events = groups.iter().flat_map(|group| {
            group
                .events
                .iter()
                .map(|replicated| &replicated.event)
                .chain(group.compensation)
        });
        if let Some((limit, len)) = self.store.ceilings.exceeded(len, events) {
            return Err(IngestError::exceeds_store_limit(limit, len));
        }

        let plan = groups
            .iter()
            .map(GroupPlan::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(IngestError::Store)?;
        // One stamp per call for the compensation, as `append` stamps one per
        // batch.
        let recorded_at = now();

        match self
            .transactions
            .run(|| self.write_call(&plan, recorded_at))
        {
            Settled::Committed(ingested) => Ok(ingested),
            Settled::RolledBack(Refusal::Unheld(id)) => Err(IngestError::unheld_own_identity(id)),
            Settled::RolledBack(Refusal::Failed(error)) | Settled::Primitive(error) => {
                Err(IngestError::Store(error))
            }
        }
    }

    /// Writes the planned call. Runs inside the transaction, and an `Err` rolls
    /// the whole call back.
    fn write_call(
        &self,
        plan: &[GroupPlan<'_>],
        recorded_at: RecordedAt,
    ) -> Result<Ingested, Refusal> {
        // Read inside the transaction, so the claim check and the compensation
        // stamp use the identity current when the call runs.
        let own = self.store.store_id()?;
        // The head, read as the call's first read, and only when the call
        // carries a claim. A claim is judged only inside `if let Some(head)`,
        // so none is ever judged without it; the inner `None` is an empty
        // store, which holds no own identity.
        let held_through = if plan.iter().any(|group| group.claims(own)) {
            Some(max_position(&self.store.sql, &self.store.tables)?)
        } else {
            None
        };

        let insert = insert_statement(&self.store.tables);
        let mut ingested = Ingested::default();
        for group in plan {
            let mut landed = false;
            for (event, origin) in &group.events {
                let position = self
                    .store
                    .insert_row(&insert, &event.event, event.recorded_at)?;
                if self.stamp_foreign(position, *origin)? {
                    self.store
                        .write_tag_rows(slice::from_ref(&event.event), &[position])?;
                    ingested.appended = ingested.appended.saturating_add(1);
                    landed = true;
                    // It landed, so this store did not hold it.
                    if origin.store == own {
                        return Err(Refusal::Unheld(event.id));
                    }
                } else {
                    self.discard_staged(position)?;
                    ingested.skipped = ingested.skipped.saturating_add(1);
                    // Skipped above the head: the row it met is this call's own
                    // compensation, so this store did not hold it either. Every
                    // own-identity row has `origin_position = position`, and an
                    // unheld claim never lands, so at or below the head it is a
                    // held echo.
                    if let Some(head) = held_through
                        && origin.store == own
                        && head.is_none_or(|head| origin.position > position_as_i64(head))
                    {
                        return Err(Refusal::Unheld(event.id));
                    }
                }
            }
            if landed && !group.compensation.is_empty() {
                // Append's writer, which stamps `(own, position)` through its own
                // `IS NULL` marker. A collision there would mean an own identity
                // already sits at a fresh position, which this adapter cannot
                // produce, so it raises rather than skips.
                let mut positions = Vec::with_capacity(group.compensation.len());
                self.store
                    .write_rows(own, group.compensation, recorded_at, &mut positions)?;
                ingested.compensated = ingested
                    .compensated
                    .saturating_add(group.compensation.len());
            }
        }
        Ok(ingested)
    }

    /// Stamps the staged row at `position` with `origin`, and says whether the
    /// stamp landed.
    fn stamp_foreign(
        &self,
        position: SequencePosition,
        origin: StoredOrigin,
    ) -> Result<bool, CloudflareEventStoreError> {
        let mut cursor = self.store.sql.exec(
            &stamp_foreign_sql(&self.store.tables),
            &[
                SqlValue::Blob(origin.store.to_bytes().to_vec()),
                SqlValue::Integer(origin.position),
                SqlValue::Integer(position_as_i64(position)),
            ],
        )?;
        Ok(cursor.next_row().transpose()?.is_some())
    }

    /// Deletes the staged row at `position`, which a skipped event left.
    fn discard_staged(&self, position: SequencePosition) -> Result<(), CloudflareEventStoreError> {
        self.store.sql.exec(
            &discard_staged_sql(&self.store.tables),
            &[SqlValue::Integer(position_as_i64(position))],
        )?;
        Ok(())
    }

    /// The highest origin position held from each origin store. One `GROUP BY`
    /// over the origin index, nothing cached.
    fn origin_watermark(&self) -> Result<Watermark, CloudflareEventStoreError> {
        let mut cursor = self
            .store
            .sql
            .exec(&watermark_sql(&self.store.tables), &[])?;
        let mut watermark = Watermark::new();
        while let Some(row) = cursor.next_row().transpose()? {
            match row.values() {
                [SqlValue::Blob(store), position] => {
                    let store = <[u8; 16]>::try_from(store.as_slice()).map_err(|_| {
                        CloudflareEventStoreError::ColumnType {
                            column: "event.origin_store",
                        }
                    })?;
                    watermark.advance(
                        StoreId::from_bytes(store),
                        decode_position(position, "event.origin_position")?,
                    );
                }
                [_, _] => {
                    return Err(CloudflareEventStoreError::ColumnType {
                        column: "event.origin_store",
                    });
                }
                values => {
                    return Err(CloudflareEventStoreError::RowShape {
                        expected: 2,
                        actual: values.len(),
                    });
                }
            }
        }
        Ok(watermark)
    }
}

// The bare flavours, the only ones a Durable Object store has, and only those
// names in scope here (constraint 4).
impl EventStore for CloudflareIngestStore {
    type Error = CloudflareEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.store.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.store.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.store.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.store.contains_event_id(id).await
    }
}

/// `Busy` is never produced: a Durable Object has no other writer (the module
/// documentation). A dropped `ingest` future has already committed or rolled
/// back, because the body awaits nothing.
impl IngestStore for CloudflareIngestStore {
    // The bare adapter error, as the other runs: the port's `IngestError`
    // carries capacity and the unheld identity in arms of its own.
    type Error = CloudflareEventStoreError;

    /// The `store_meta` row, read on every call.
    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        self.store.store_id()
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        self.ingest_groups(groups)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.origin_watermark()
    }
}

/// An ingested event's origin, in the shape this runtime binds exactly: parse,
/// don't validate (API-03).
///
/// `worker` binds an integer as `i as f64` (`worker-0.8.5/src/sql.rs:112-123`),
/// so an origin position or a recorded time past 2^53 − 1 would be stored as a
/// neighbour of itself. Building one of these is the check, so no statement is
/// ever issued for a value that cannot land exactly.
#[derive(Debug, Clone, Copy)]
struct StoredOrigin {
    /// The store that minted the event.
    store: StoreId,
    /// The position its origin gave it, as the integer column holds it.
    position: i64,
}

impl TryFrom<&ReplicatedEvent> for StoredOrigin {
    type Error = CloudflareEventStoreError;

    /// Borrows, because it reads two `Copy` fields and the recorded time.
    fn try_from(event: &ReplicatedEvent) -> Result<Self, Self::Error> {
        let origin = event.id.position();
        let position = i64::try_from(origin.get())
            .ok()
            .filter(|position| *position <= MAX_SAFE_POSITION)
            .ok_or(CloudflareEventStoreError::OriginPositionOutOfRange { position: origin })?;
        let millis = event.recorded_at.as_millis();
        if !(-MAX_SAFE_POSITION..=MAX_SAFE_POSITION).contains(&millis) {
            return Err(CloudflareEventStoreError::RecordedAtOutOfRange {
                recorded_at: event.recorded_at,
            });
        }
        Ok(Self {
            store: event.id.store(),
            position,
        })
    }
}

/// One group, checked, borrowing the caller's batch: no payload is cloned
/// (H-08).
struct GroupPlan<'a> {
    /// Each foreign event beside its checked origin, in batch order.
    events: Vec<(&'a ReplicatedEvent, StoredOrigin)>,
    /// The group's compensation.
    compensation: &'a [Event],
}

impl GroupPlan<'_> {
    /// Whether any foreign event of this group claims `own`.
    fn claims(&self, own: StoreId) -> bool {
        self.events.iter().any(|(_, origin)| origin.store == own)
    }
}

impl<'a> TryFrom<&IngestGroup<'a>> for GroupPlan<'a> {
    type Error = CloudflareEventStoreError;

    /// The first origin this runtime cannot bind, in batch order, refuses it.
    fn try_from(group: &IngestGroup<'a>) -> Result<Self, Self::Error> {
        let events = group
            .events
            .iter()
            .map(|event| StoredOrigin::try_from(event).map(|origin| (event, origin)))
            .collect::<Result<_, _>>()?;
        Ok(Self {
            events,
            compensation: group.compensation,
        })
    }
}

/// Why a call rolled back.
enum Refusal {
    /// A foreign event claims this store's own identity at a position it did
    /// not hold (SY-38).
    Unheld(EventId),
    /// A statement failed.
    Failed(CloudflareEventStoreError),
}

/// So `?` reads cleanly in `write_call`.
impl From<CloudflareEventStoreError> for Refusal {
    fn from(error: CloudflareEventStoreError) -> Self {
        Self::Failed(error)
    }
}

/// SY-36's class for each way this adapter fails.
///
/// Exhaustive over this crate's own enums, with no wildcard, so a new variant
/// does not compile until somebody decides its class. `SqlError` is
/// `#[non_exhaustive]`, but it is this crate's, so an in-crate match needs no
/// wildcard either. Coherence allows the impl: a foreign trait on a local type.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
impl Classify for CloudflareEventStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Sql(sql) => match sql {
                // A re-entrant statement was in flight, or a cursor outlived the
                // statement after it: the next attempt finds the storage free
                // and takes a fresh cursor.
                SqlError::AlreadyBorrowed | SqlError::CursorInvalidated => FailureClass::Transient,
                // An object's storage cap does not empty itself, and a thrown
                // storage error fails the same way again: under workerd a
                // trigger's RAISE and a missing table each recurred on the
                // identical retry (#108 part E3's census, in
                // experiments/durable-object-limits). No message is parsed; a
                // full object could not be staged there (the PRAGMA is refused).
                SqlError::StorageLimitExceeded | SqlError::Thrown(_) => FailureClass::Stop,
            },
            // Past what a binding carries exactly, so it can never land here,
            // and the rest of the batch can: VT-25's "park it".
            Self::OriginPositionOutOfRange { .. } | Self::RecordedAtOutOfRange { .. } => {
                FailureClass::Park
            }
            // Append's: its rows are not events (ES-18), so the failure that
            // stopped it is the one that matters.
            Self::PartialBatch { cause, .. } => cause.class(),
            // A corrupt store or a missing schema, which fails the same way
            // again.
            Self::RowShape { .. }
            | Self::ColumnType { .. }
            | Self::StoredEventType(_)
            | Self::StoredTag(_)
            | Self::CorruptTags
            | Self::UnstampedEvent { .. }
            | Self::StoredPosition { .. } => FailureClass::Stop,
        }
    }
}

// Below the `Classify` impl, so that no line the decision records cite above
// it moves.
#[cfg(all(test, target_arch = "wasm32"))]
impl CloudflareIngestStore {
    /// The same store, refusing calls against `ceilings`: the event store's
    /// seam, one wrapper up.
    #[must_use]
    pub(crate) fn with_ceilings(mut self, ceilings: super::Ceilings) -> Self {
        self.store = self.store.with_ceilings(ceilings);
        self
    }
}

mod transaction;

#[cfg(test)]
mod tests;

// The Node shim's `transactionSync`: test support, and only on
// the target with a JavaScript heap.
#[cfg(all(test, target_arch = "wasm32"))]
mod model;
