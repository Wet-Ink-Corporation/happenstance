//! VT-10's second run: `happenstance-sync`'s ingest port, implemented for this
//! adapter over the row writer `append` already uses.
//!
//! # What is being tested
//!
//! VT-10's falsifier reads *"a store adapter cannot implement `IngestStore`
//! without duplicating append's write path"*. `happenstance-sqlite` was the
//! first run; this is the first against a store whose writers do not
//! serialise. Append and ingest send **one** `INSERT` text, built by
//! `event_store::insert_statement`, and the rows differ only in the origin
//! each binds: a local row binds this store and its own position, a foreign
//! row the origin the peer sent. What is ingest-only is small enough to name:
//!
//! * the `Origin::Foreign` arm of the writer and `StoredOrigin`, which refuses
//!   an origin position past `i64::MAX` rather than saturating it, in
//!   `event_store.rs` under `#[cfg(test)]`;
//! * [`CONFLICT_CLAUSE`], appended to the foreign rows' statement, so a
//!   re-delivery is a skip decided inside the `INSERT` on the origin pair's
//!   partial unique index, never by a lookup ahead of it (VT-8, SY-11);
//! * the own-identity check (SY-38), which needs no lookup ahead of the write
//!   either. A claim on this store's identity that **lands** was not held. One
//!   that is **skipped** met a row, and [`HIDDEN_CLAIMS_SQL`] asks whether that
//!   row was visible in the snapshot the transaction took first: a row this
//!   call wrote, or one another transaction committed during it, was not held
//!   when the call began, and refuses the whole call;
//! * [`WATERMARK_SQL`], one `GROUP BY` over the origin index.
//!
//! No append condition is evaluated (SY-1), and the transaction runs at the
//! pool's default `READ COMMITTED`, as an unconditional append does.
//!
//! # The identity
//!
//! The port is implemented on `PostgresEventStore` itself, with no wrapper,
//! because the port's [`store_id`](SendIngestStore::store_id) is `async` and
//! fallible: it is the store's own read of `store_meta`, made on a handle's
//! first use and cached on it, the one `append` stamps from. An ingest asks
//! for it after every refusal that needs none, so a call over a ceiling, or
//! with an origin position past a `bigint`, still never reaches the wire; on a
//! handle that has not read it yet, it is one round trip more, and a failed
//! read is [`IngestError::Store`].
//!
//! The tests that need no server reach the cache directly: it is a private
//! field, and they are descendants of `event_store`. `unreachable_store`
//! seeds a fixed `StoreId` into a store whose pool points nowhere, so a
//! refusal that must come before the wire is shown not to reach it, and
//! nothing is added to `event_store.rs` for them.
//!
//! # Positions
//!
//! Every position an ingest takes comes from `nextval('event_position_seq')`
//! inside its transaction, the sequence `append` reads. So everything an
//! ingest writes is above every position allocated before the call began
//! (SY-5), and ingested rows get their `xact_id` from the same column default
//! as appended ones, so ES-10's frontier treats them alike. A skipped row has
//! consumed its position, and so has every row of a refused call: gaps, which
//! the specification permits.
//!
//! # Failure
//!
//! One transaction per call, so every arm of [`IngestError`] means nothing in
//! the call was written, with one exception the port's wording does not yet
//! allow for: a `COMMIT` whose reply is lost surfaces as a transient driver
//! error and may have committed. Re-delivery is a skip, so a runner's retry
//! converges (SY-11). [`IngestError::Busy`] is never produced: ingest takes no
//! write lock and has no serialisation retry to exhaust.
//!
//! A dropped `ingest` future detaches the spawned task, which may still commit,
//! as `append`'s does (ES-23). Reissuing the identical batch is safe for the
//! same reason.
//!
//! Two concurrent ingests carrying overlapping foreign events in different
//! orders can deadlock on `ON CONFLICT`'s speculative insertion. The server
//! aborts one whole with `40P01`, which classifies as transient. Batch order is
//! kept rather than sorted, so a group's local positions follow its origin's.
//!
//! # Why here, and why `cfg(test)`
//!
//! **Here**, as a child of `event_store`, because privacy is per module and a
//! module's private items are visible to its descendants: this reaches the
//! writer, the pool, the cached identity and `on_runtime` without any of them
//! being widened. Coherence allows the impl: a foreign trait on a local type.
//!
//! **`cfg(test)`** because `happenstance-sync` is `publish = false` and its name
//! is unclaimed, so it can only be a path-only dev-dependency, which cargo
//! strips on publish. Every test is therefore a unit test under this module: a
//! `cfg(test)` impl is invisible to `tests/`. Part E turns the gate into a
//! feature.

use std::collections::HashSet;

use happenstance_core::{Event, EventId, RecordedAt, StoreId};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, Ingested, SendIngestStore, Watermark,
};
use sqlx::{PgPool, Postgres, Transaction};

use super::{
    NewRow, Origin, PostgresEventStore, StoredOrigin, allocate_positions, exceeded,
    insert_statement, now, position_from_row,
};
use crate::error::PostgresEventStoreError;

/// Appended to `insert_statement` for foreign rows.
///
/// The `WHERE` repeats the partial index's predicate, without which Postgres
/// cannot infer `event_origin_idx` and refuses the statement. A conflict on any
/// other constraint still raises. `RETURNING` names the rows that landed, so
/// the rows that did not are the skips.
const CONFLICT_CLAUSE: &str = " ON CONFLICT (origin_store, origin_position) \
     WHERE origin_store IS NOT NULL AND origin_position IS NOT NULL DO NOTHING \
     RETURNING origin_store, origin_position";

/// The transaction's begin snapshot, taken as its first statement when the
/// call carries a claim on this store's identity.
const BEGIN_SNAPSHOT_SQL: &str = "SELECT pg_current_snapshot()::text";

/// Of the claimed own positions the insert skipped, those whose row the begin
/// snapshot could not see: written by this call, or committed by another
/// transaction during it.
///
/// `$3::text::pg_snapshot` because the snapshot travels as `text`.
const HIDDEN_CLAIMS_SQL: &str = "SELECT origin_position FROM event \
     WHERE origin_store = $1 AND origin_position = ANY($2) \
     AND NOT pg_visible_in_snapshot(xact_id, $3::text::pg_snapshot)";

/// The highest origin position held per origin store.
///
/// Both `IS NOT NULL`s are load-bearing: without the second the planner cannot
/// prove the partial index's predicate, and falls back to a sequential scan
/// and a sort. No frontier predicate: the watermark reports what the store
/// holds, which is ES-41's reading of membership and VT-8's dedupe set.
const WATERMARK_SQL: &str = "SELECT origin_store, max(origin_position) FROM event \
     WHERE origin_store IS NOT NULL AND origin_position IS NOT NULL GROUP BY origin_store";

/// Binds per row in `insert_statement`.
const ROW_PARAMETERS: usize = 8;

/// The extended query protocol's parameter count is an `int16`.
const MAX_BIND_PARAMETERS: usize = 65_535;

// No statement here ever needs chunking: the ceilings refuse a call over the
// batch limit before the wire, and a full batch binds well under the protocol's
// limit. Checked at compile time, so it adds no runtime panic.
const _: () =
    assert!(PostgresEventStore::MAX_EVENTS_PER_BATCH * ROW_PARAMETERS <= MAX_BIND_PARAMETERS);

impl PostgresEventStore {
    /// The ceilings, the origin check, this store's identity, then one
    /// transaction on the store's runtime. Every refusal comes before the
    /// wire, and the identity is a round trip only on a handle that has not
    /// read it yet.
    async fn ingest_groups(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<PostgresEventStoreError>> {
        // An empty pull writes nothing, and compensation needs a landed event
        // (SY-11), so there is nothing to open a transaction for.
        if groups.iter().all(|group| group.events.is_empty()) {
            return Ok(Ingested::default());
        }

        // Against the worst case, the whole call: every foreign event and
        // every compensation, whichever of them turn out to be skips.
        let len = groups
            .iter()
            .map(|group| group.events.len() + group.compensation.len())
            .sum();
        let events = groups.iter().flat_map(|group| {
            group
                .events
                .iter()
                .map(|replicated| &replicated.event)
                .chain(group.compensation)
        });
        if let Some((limit, len)) = exceeded(len, events) {
            return Err(IngestError::exceeds_store_limit(limit, len));
        }

        // Owned, because the work crosses onto the runtime and must be
        // `'static`, as `append`'s `events.to_vec()` is. Not free: the
        // payload is `Bytes`, a refcount bump, but the event type and each
        // tag are `Cow`s, so cloning one built from owned strings allocates
        // for the type, the tag slice and every tag: `t + 2` for `t` tags
        // (`experiments/event-clone-allocations/README.md`).
        let owned = groups
            .iter()
            .map(OwnedGroup::try_from_group)
            .collect::<Result<Vec<_>, _>>()
            .map_err(IngestError::Store)?;
        // The inherent read, cached per handle, which `append` stamps from.
        // `Self::store_id` names it rather than the port's method of the same
        // name, because an inherent associated function wins the lookup.
        let own = Self::store_id(self).await.map_err(IngestError::Store)?;
        let recorded_at = now();
        let pool = self.pool.clone();
        let outcome = self
            .on_runtime(ingest_in_transaction(pool, own, owned, recorded_at))
            .await
            .map_err(IngestError::Store)?;
        match outcome {
            Outcome::Ingested(ingested) => Ok(ingested),
            Outcome::UnheldOwnIdentity(id) => Err(IngestError::unheld_own_identity(id)),
        }
    }

    /// The highest origin position held from each origin store, this store's
    /// own included. One `GROUP BY` over the origin index, nothing cached.
    async fn origin_watermark(&self) -> Result<Watermark, PostgresEventStoreError> {
        let pool = self.pool.clone();
        let rows: Vec<(Vec<u8>, i64)> = self
            .on_runtime(async move {
                sqlx::query_as(WATERMARK_SQL)
                    .fetch_all(&pool)
                    .await
                    .map_err(PostgresEventStoreError::from)
            })
            .await?;
        let mut watermark = Watermark::new();
        for (store, highest) in rows {
            watermark.advance(stored_store_id(&store)?, position_from_row(highest)?);
        }
        Ok(watermark)
    }
}

// The `Send` flavour, as `PostgresEventStore`'s `SendEventStore` is, and only
// that name in scope here (constraint 4): `trait_variant` derives the bare
// one, which the tests bind on.
impl SendIngestStore for PostgresEventStore {
    // The bare adapter error, as `happenstance-sqlite`'s spike: the port's
    // `IngestError` carries capacity and the unheld identity in arms of its own.
    type Error = PostgresEventStoreError;

    /// The identity `store_meta` holds, read on this handle's first use and
    /// cached, by the same read `append` stamps from. A re-mint is seen by a
    /// new handle; a clone shares the cache.
    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        // The inherent method, not this one: inherent wins path resolution.
        Self::store_id(self).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        self.ingest_groups(groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.origin_watermark().await
    }
}

/// SY-36's class for each way this adapter fails.
///
/// Exhaustive over this crate's own enum, with no wildcard, so a new variant
/// does not compile until somebody decides its class. A driver failure is
/// transient when the server aborted or never started the work and time or an
/// operator cures it: the network, the pool, a serialisation failure, a
/// deadlock, an administrator's shutdown, or exhausted resources. Every other
/// driver failure, and every decoding variant, fails the same way again.
impl Classify for PostgresEventStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Driver(error) if transient_driver_failure(error) => FailureClass::Transient,
            // A cancelled task did not run; a panicked one would again.
            Self::Worker(error) if error.is_cancelled() => FailureClass::Transient,
            // Past a `bigint`, so it can never land here, and the rest of the
            // batch can: VT-25's "park it".
            Self::OriginPositionOutOfRange { .. } => FailureClass::Park,
            Self::Driver(_)
            | Self::Worker(_)
            | Self::PositionOutOfRange { .. }
            | Self::EventType(_)
            | Self::Tag(_)
            | Self::MissingIdentity
            | Self::MalformedIdentity { .. }
            | Self::UnstampedEvent { .. }
            | Self::NoRuntime
            | Self::UnreadableTransactionId => FailureClass::Stop,
        }
    }
}

/// Whether waiting can cure a driver failure.
fn transient_driver_failure(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Database(database) => {
            database.code().as_deref().is_some_and(transient_sqlstate)
        }
        sqlx::Error::Io(_) | sqlx::Error::PoolTimedOut => true,
        // `sqlx::Error` is foreign and `#[non_exhaustive]`, so this is a
        // wildcard over its variants only: a closed pool, a protocol or TLS
        // failure, a decode error.
        _ => false,
    }
}

/// Connection exceptions (`08`), insufficient resources (`53`), a
/// serialisation failure, a deadlock, and the three operator interventions
/// (`57P01`–`57P03`). Matched on the code, as `is_serialisation_failure` is,
/// rather than by a variant per code.
fn transient_sqlstate(code: &str) -> bool {
    code.starts_with("08")
        || code.starts_with("53")
        || matches!(code, "40001" | "40P01" | "57P01" | "57P02" | "57P03")
}

/// One group, owned so it can cross onto the runtime.
struct OwnedGroup {
    events: Vec<ForeignEvent>,
    compensation: Vec<Event>,
}

impl OwnedGroup {
    /// Copies `group`, checking each origin once.
    ///
    /// # Errors
    ///
    /// `OriginPositionOutOfRange` for the first origin position past
    /// `i64::MAX`.
    fn try_from_group(group: &IngestGroup<'_>) -> Result<Self, PostgresEventStoreError> {
        let events = group
            .events
            .iter()
            .map(|replicated| {
                Ok(ForeignEvent {
                    id: replicated.id,
                    origin: StoredOrigin::try_from(replicated.id)?,
                    recorded_at: replicated.recorded_at,
                    event: replicated.event.clone(),
                })
            })
            .collect::<Result<_, PostgresEventStoreError>>()?;
        Ok(Self {
            events,
            compensation: group.compensation.to_vec(),
        })
    }
}

/// A foreign event, its origin already in the schema's shape.
struct ForeignEvent {
    id: EventId,
    origin: StoredOrigin,
    recorded_at: RecordedAt,
    event: Event,
}

impl ForeignEvent {
    /// The row the shared writer inserts for this event.
    fn row(&self) -> NewRow<'_> {
        NewRow {
            event: &self.event,
            origin: Origin::Foreign(self.origin),
            recorded_at: self.recorded_at,
        }
    }
}

/// What the transaction decided, before it is mapped onto the port's error.
enum Outcome {
    /// Committed.
    Ingested(Ingested),
    /// Rolled back: the first claim on this store's identity it did not hold.
    UnheldOwnIdentity(EventId),
}

/// One ingest: every group, in order, in one transaction.
async fn ingest_in_transaction(
    pool: PgPool,
    own: StoreId,
    groups: Vec<OwnedGroup>,
    recorded_at: RecordedAt,
) -> Result<Outcome, PostgresEventStoreError> {
    let mut transaction = pool.begin().await?;

    // Only a call that carries a claim takes a snapshot, and the claim check
    // runs only with one, so no claim is ever judged without it.
    let claims = groups
        .iter()
        .flat_map(|group| &group.events)
        .any(|foreign| foreign.id.store() == own);
    let began: Option<String> = if claims {
        Some(
            sqlx::query_scalar(BEGIN_SNAPSHOT_SQL)
                .fetch_one(&mut *transaction)
                .await?,
        )
    } else {
        None
    };

    let mut ingested = Ingested::default();
    for group in &groups {
        let landed = insert_foreign(&mut transaction, own, &group.events).await?;
        if let Some(began) = &began
            && let Some(id) =
                unheld_claim(&mut transaction, own, &group.events, &landed, began).await?
        {
            // Explicitly, not by drop. Dropping only queues the `ROLLBACK`,
            // and until the pool flushes it this transaction's id holds the
            // frontier back, so the caller's next read could miss rows.
            transaction.rollback().await?;
            return Ok(Outcome::UnheldOwnIdentity(id));
        }
        ingested.appended += landed.len();
        // A duplicate inside one call lands once and counts as a skip.
        ingested.skipped += group.events.len().saturating_sub(landed.len());

        // Only for a group that landed something (SY-11): the first delivery
        // of a re-delivered group already wrote its compensation.
        if !landed.is_empty() {
            insert_compensation(&mut transaction, own, &group.compensation, recorded_at).await?;
            ingested.compensated += group.compensation.len();
        }
    }

    transaction.commit().await?;
    Ok(Outcome::Ingested(ingested))
}

/// Inserts one group's foreign events under the conflict clause, returning the
/// identities that landed.
async fn insert_foreign(
    transaction: &mut Transaction<'static, Postgres>,
    own: StoreId,
    events: &[ForeignEvent],
) -> Result<HashSet<EventId>, PostgresEventStoreError> {
    // A `VALUES` list cannot be empty.
    if events.is_empty() {
        return Ok(HashSet::new());
    }
    let positions = allocate_positions(transaction, events.len()).await?;
    let returned: Vec<(Vec<u8>, i64)> = foreign_statement(own, events, &positions)
        .build_query_as()
        .fetch_all(&mut **transaction)
        .await?;
    returned
        .into_iter()
        .map(|(store, position)| {
            Ok(EventId::new(
                stored_store_id(&store)?,
                position_from_row(position)?,
            ))
        })
        .collect()
}

/// Inserts one group's compensation as local rows, minted here and stamped
/// `recorded_at`, after the group's foreign rows (SY-2).
///
/// No conflict clause: a collision here would mean an own identity already
/// sits at a fresh sequence value, which must raise rather than skip.
async fn insert_compensation(
    transaction: &mut Transaction<'static, Postgres>,
    own: StoreId,
    compensation: &[Event],
    recorded_at: RecordedAt,
) -> Result<(), PostgresEventStoreError> {
    // A `VALUES` list cannot be empty.
    if compensation.is_empty() {
        return Ok(());
    }
    let positions = allocate_positions(transaction, compensation.len()).await?;
    let rows = compensation.iter().map(|event| NewRow {
        event,
        origin: Origin::Local,
        recorded_at,
    });
    insert_statement(own, rows, &positions)
        .build()
        .execute(&mut **transaction)
        .await?;
    Ok(())
}

/// The statement [`insert_foreign`] sends: append's `INSERT`, over the foreign
/// rows, with [`CONFLICT_CLAUSE`] appended and nothing else changed.
fn foreign_statement(
    own: StoreId,
    events: &[ForeignEvent],
    positions: &[i64],
) -> sqlx::QueryBuilder<'static, Postgres> {
    let mut statement = insert_statement(own, events.iter().map(ForeignEvent::row), positions);
    statement.push(CONFLICT_CLAUSE);
    statement
}

/// The first claim on `own` in `events` that this store did not hold when the
/// transaction began, in the group's order.
///
/// A claim that landed was not held. A claim that was skipped met a row, and
/// that row was held only if the begin snapshot `began` could see it.
async fn unheld_claim(
    transaction: &mut Transaction<'static, Postgres>,
    own: StoreId,
    events: &[ForeignEvent],
    landed: &HashSet<EventId>,
    began: &str,
) -> Result<Option<EventId>, PostgresEventStoreError> {
    let skipped: Vec<i64> = events
        .iter()
        .filter(|foreign| foreign.id.store() == own && !landed.contains(&foreign.id))
        .map(|foreign| foreign.origin.position)
        .collect();
    let hidden: HashSet<i64> = if skipped.is_empty() {
        HashSet::new()
    } else {
        sqlx::query_scalar::<Postgres, i64>(HIDDEN_CLAIMS_SQL)
            .bind(own.to_bytes().to_vec())
            .bind(skipped)
            .bind(began)
            .fetch_all(&mut **transaction)
            .await?
            .into_iter()
            .collect()
    };
    Ok(events
        .iter()
        .filter(|foreign| foreign.id.store() == own)
        .find(|foreign| landed.contains(&foreign.id) || hidden.contains(&foreign.origin.position))
        .map(|foreign| foreign.id))
}

/// Decodes a stored `origin_store`.
///
/// # Errors
///
/// `MalformedIdentity` when it is not sixteen bytes.
fn stored_store_id(stored: &[u8]) -> Result<StoreId, PostgresEventStoreError> {
    let bytes: [u8; 16] = stored
        .try_into()
        .map_err(|_| PostgresEventStoreError::MalformedIdentity { len: stored.len() })?;
    Ok(StoreId::from_bytes(bytes))
}

// The whole file is `cfg(test)` already, through its declaration in
// `event_store.rs`. Said again here so the module reads as test code where it
// stands.
#[cfg(test)]
mod tests;
