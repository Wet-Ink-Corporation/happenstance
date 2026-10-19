//! VT-10's third run: `happenstance-sync`'s ingest port, implemented for this
//! adapter over the insert builder `append` already uses.
//!
//! # What is being tested
//!
//! VT-10's falsifier reads *"a store adapter cannot implement `IngestStore`
//! without duplicating append's write path"*. `happenstance-sqlite` ran it over
//! a store that serialises its writers and `happenstance-postgres` over one
//! that does not; this is the run on a store with no connection, no
//! interactive transaction and no cursor. Append and ingest write through one
//! builder, `NeonEventStore::insert_rows`, and the rows differ only in the
//! origin each carries. What is ingest-only is small enough to name:
//!
//! * the foreign origin triple, the conflict clause on the origin index and the
//!   group gating, in `NeonEventStore::ingest_statement`;
//! * the own-identity check (SY-38), decided by the write and needing no
//!   lookup ahead of it: the statement reads which claims **landed** out of its
//!   own `RETURNING`, and [`refusal_statement`] raises [`UNHELD_SQLSTATE`] when
//!   one did, which aborts the request's one transaction;
//! * [`WATERMARK_SQL`], one `GROUP BY` over the origin index.
//!
//! No append condition is evaluated (SY-1).
//!
//! # One request, one transaction
//!
//! An ingest is **one round trip** of two statements at `REPEATABLE READ`.
//! `REPEATABLE READ` makes the server refuse, with `40001`, an `ON CONFLICT`
//! that meets a row committed by a transaction the call's snapshot cannot
//! see, so a claim on a row committed during the call is never taken for a
//! held one. That arrives here as [`IngestError::Busy`]: the endpoint
//! answered, so it ran the batch and aborted it whole. It is not retried here;
//! the runner owns backoff, and a re-pull sees the row as held.
//!
//! So every arm of [`IngestError`] means nothing in the call was written, with
//! one exception the port's wording does not yet allow for: a round trip that
//! got no answer is [`NeonError::Transport`], and may have committed.
//! Re-delivery is a skip, so a runner's retry converges (SY-11).
//!
//! A dropped `ingest` future may already have committed too, for the reason
//! `event_store`'s `# Cancellation` gives for `append` (ES-23). Reissuing the
//! identical batch is safe for the same reason.
//!
//! # Why a wrapper rather than an impl on `NeonEventStore`
//!
//! The port's [`store_id`](IngestStore::store_id) is synchronous and
//! infallible, and `NeonEventStore` holds no identity in Rust at all: every
//! `INSERT` reads it from the meta row (ADR-0086). [`NeonIngestStore`] is
//! built only by an `async` constructor that reads it once, which makes "an
//! ingest store whose identity is unknown" unrepresentable. The cached copy
//! can go stale only through a manual `UPDATE` of the meta row, and the
//! refusal does not depend on it: the statement judges a claim against the
//! meta row it reads. That is a finding about the port's shape, recorded for
//! #108's part E, as Postgres's is.
//!
//! # The bare flavour
//!
//! `NeonEventStore` implements the bare `EventStore` and cannot satisfy
//! `SendEventStore` (`event_store`'s `# Why the bare flavour`), so this
//! implements the bare `EventStore` and `IngestStore`, and only those names
//! are in scope (constraint 4). The one place Neon departs from the other two
//! ingest runs, which implement the `Send` flavours.
//!
//! # Why here, and why `cfg(test)`
//!
//! **Here**, as a child of `event_store`, because a module's private items are
//! visible to its descendants: this reaches the builder, the decoders and the
//! ceilings without any of them being widened. Coherence allows the impls: a
//! foreign trait on a local type.
//!
//! **`cfg(test)`** because `happenstance-sync` is `publish = false` and its name
//! is unclaimed, so it can only be a path-only dev-dependency, which cargo
//! strips on publish. Every test is therefore a unit test under this module,
//! and nothing here compiles for `wasm32`, where the gate builds the library
//! alone. Part E turns the gate into a feature.

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, StoreId,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, Watermark,
};

use super::{NeonEventStore, decode_base64, decode_body, first_result_set, position_from_text};
use crate::error::{NeonError, NeonSqlError};
use crate::transport::{HttpResponse, SqlRequest, SqlStatement, SqlTransport};

/// The transaction-local setting the ingest statement leaves the first landed
/// claim's 1-based index in, or the empty string.
///
/// Transaction-local because a session setting can outlive its request behind
/// the `/sql` proxy, onto a backend a later, unrelated request reuses.
pub(super) const UNHELD_GUC: &str = "happenstance.unheld_own_identity";

/// The SQLSTATE this adapter raises an unheld own identity with.
///
/// Class `HS` is not one PostgreSQL assigns, and the code does not end in
/// `000`, which PL/pgSQL's guidance reserves. The refusal travels as a code
/// and a `DETAIL`, so nothing here parses a message.
pub(super) const UNHELD_SQLSTATE: &str = "HS038";

/// The highest origin position held per origin store, with the event table's
/// qualified name where the placeholder stands.
///
/// Both `IS NOT NULL`s are load-bearing: without the second the planner cannot
/// prove the partial index's predicate, and falls back to a sequential scan
/// and a sort. No frontier predicate: the watermark reports what the store
/// holds, which is ES-41's reading of membership and VT-8's dedupe set.
const WATERMARK_SQL: &str = "SELECT encode(origin_store, 'base64') AS s, \
     max(origin_position)::text AS p FROM {event} \
     WHERE origin_store IS NOT NULL AND origin_position IS NOT NULL GROUP BY origin_store";

/// The request's second statement: aborts the batch, and with it everything
/// the first statement wrote, when the first left a claim in [`UNHELD_GUC`].
///
/// A `DO` block, which takes no bind parameters and needs none: it reads only
/// the setting. It names no relation, so it has no schema to qualify. `DETAIL`
/// carries the claim's index; the message is for a human reading a log.
pub(super) fn refusal_statement() -> SqlStatement {
    SqlStatement::new(format!(
        "DO $sy38$ BEGIN \
         IF current_setting('{UNHELD_GUC}', true) <> '' THEN \
         RAISE EXCEPTION USING ERRCODE = '{UNHELD_SQLSTATE}', \
         MESSAGE = 'an ingested event claims this store''s own identity at a position it \
         did not hold (SY-38)', \
         DETAIL = current_setting('{UNHELD_GUC}'); \
         END IF; END $sy38$"
    ))
}

/// A [`NeonEventStore`] whose identity has been read, which is what the
/// ingest port's synchronous [`store_id`](IngestStore::store_id) answers
/// from.
#[derive(Debug, Clone)]
struct NeonIngestStore<T> {
    /// The store this ingests into. Owned, because the wrapper keeps it.
    store: NeonEventStore<T>,
    /// `store`'s identity, read once by [`open`](Self::open).
    store_id: StoreId,
}

impl<T: SqlTransport> NeonIngestStore<T> {
    /// Reads `store`'s identity once, in one read-only round trip, so every
    /// ingest and `store_id` answer from it.
    ///
    /// # Errors
    ///
    /// `Transport`, `Http` or `Sql` as any read reports them;
    /// `MissingIdentity` when the meta row is absent; `MalformedIdentity` when
    /// it is not sixteen bytes; `MalformedPayload` when it is not base64.
    async fn open(store: NeonEventStore<T>) -> Result<Self, NeonError<T::Error>> {
        let sql = format!(
            "SELECT encode(v, 'base64') AS v FROM {} WHERE k = 'store_id'",
            store.config().qualified_meta()
        );
        let response = store
            .transport()
            .round_trip(SqlRequest::single(SqlStatement::new(sql)).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let store_id = decode_identity(&response, store.config().max_response_bytes)?;
        Ok(Self { store, store_id })
    }

    /// The store this wraps, for tests that send raw SQL beside an ingest.
    const fn event_store(&self) -> &NeonEventStore<T> {
        &self.store
    }

    /// Every refusal before the wire, then one round trip.
    async fn ingest_groups(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<NeonError<T::Error>>> {
        // An empty pull writes nothing, and compensation needs a landed event
        // (SY-11), so there is nothing to send.
        if groups.iter().all(|group| group.events.is_empty()) {
            return Ok(Ingested::default());
        }

        // Against the worst case, the whole call: every foreign event and
        // every compensation, whichever of them turn out to be skips. The
        // batch ceiling applies because the call is one JSON document held in
        // memory at both ends, which is that ceiling's own reason.
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
        if let Some((limit, len)) = NeonEventStore::<T>::exceeded(len, events) {
            return Err(IngestError::exceeds_store_limit(limit, len));
        }

        let request = self
            .store
            .ingest_request::<T::Error>(groups)
            .map_err(IngestError::Store)?;
        // ES-11, as `append` waits: an ingest is a write.
        self.store.transport().reads_settled().await;
        let response = self
            .store
            .transport()
            .round_trip(request)
            .await
            .map_err(|error| IngestError::Store(NeonError::Transport(error)))?;
        decode_ingest_answer(&response, self.store.config().max_response_bytes, groups)
    }

    /// The highest origin position held from each origin store, this store's
    /// own included. One `GROUP BY` over the origin index, nothing cached.
    async fn origin_watermark(&self) -> Result<Watermark, NeonError<T::Error>> {
        let sql = WATERMARK_SQL.replace("{event}", &self.store.config().qualified_event());
        let response = self
            .store
            .transport()
            .round_trip(SqlRequest::single(SqlStatement::new(sql)).read_only())
            .await
            .map_err(NeonError::Transport)?;
        decode_watermark(&response, self.store.config().max_response_bytes)
    }
}

// The bare flavours, the only ones `NeonEventStore` has, and only those names
// in scope here (constraint 4).
impl<T: SqlTransport> EventStore for NeonIngestStore<T> {
    type Error = NeonError<T::Error>;

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

impl<T: SqlTransport> IngestStore for NeonIngestStore<T> {
    // The bare adapter error, as the other two runs: the port's `IngestError`
    // carries capacity, contention and the unheld identity in arms of its own.
    type Error = NeonError<T::Error>;

    fn store_id(&self) -> StoreId {
        self.store_id
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
/// does not compile until somebody decides its class. Every decoding variant
/// is a corrupt store, a missing schema, or this crate and the endpoint
/// disagreeing, and fails the same way again.
impl<E> Classify for NeonError<E> {
    fn class(&self) -> FailureClass {
        match self {
            // The body was too big; the events may not be.
            Self::Http { status: 413, .. } => FailureClass::Split,
            // No answer, which may have committed, and a re-delivery is a skip,
            // so the retry converges (SY-11); or the gateway, a timeout or a
            // rate limit.
            Self::Transport(_)
            | Self::Http {
                status: 408 | 429 | 500..=599,
                ..
            } => FailureClass::Transient,
            Self::Sql(sql) if sql.code.as_deref().is_some_and(transient_sqlstate) => {
                FailureClass::Transient
            }
            // Past a `bigint`, so it can never land here, and the rest of the
            // batch can: VT-25's "park it".
            Self::OriginPositionOutOfRange { .. } => FailureClass::Park,
            // Credentials or configuration, a statement that fails the same
            // way again (this adapter's own `HS038` among them), or a store
            // that is not the shape this crate wrote.
            Self::Http { .. }
            | Self::Sql(_)
            | Self::ResponseTooLarge { .. }
            | Self::MalformedResponse(_)
            | Self::InvalidPosition { .. }
            | Self::MissingColumn { .. }
            | Self::StoredEventType(_)
            | Self::StoredTag(_)
            | Self::UnstampedEvent { .. }
            | Self::MalformedIdentity { .. }
            | Self::MalformedPayload
            | Self::ForeignBatch
            | Self::MissingIdentity => FailureClass::Stop,
        }
    }
}

/// Connection exceptions (`08`), insufficient resources (`53`), a
/// serialisation failure, a deadlock, and the three operator interventions
/// (`57P01`–`57P03`): the work was aborted whole or never started, and time or
/// an operator cures it. Matched on the code, as Postgres's is, rather than by
/// a variant per code.
fn transient_sqlstate(code: &str) -> bool {
    code.starts_with("08")
        || code.starts_with("53")
        || matches!(code, "40001" | "40P01" | "57P01" | "57P02" | "57P03")
}

/// What one ingest answer says, mapped onto the port's error.
///
/// The second result set, the `DO` block's, carries nothing and is not read.
fn decode_ingest_answer<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
    groups: &[IngestGroup<'_>],
) -> Result<Ingested, IngestError<NeonError<E>>> {
    let body = match decode_body::<E>(response, max_response_bytes) {
        Ok(body) => body,
        Err(NeonError::Sql(sql)) => return Err(refused(sql, groups)),
        Err(other) => return Err(IngestError::Store(other)),
    };
    let counts = first_result_set::<E>(&body).map_err(IngestError::Store)?;
    let count = |column: &'static str| {
        counts
            .text(0, column)
            .and_then(|text| text.parse::<usize>().ok())
            .ok_or(IngestError::Store(NeonError::MissingColumn { column }))
    };
    // `Ingested` is `#[non_exhaustive]`, so it is filled rather than written
    // as a literal.
    let mut ingested = Ingested::default();
    ingested.appended = count("appended")?;
    ingested.skipped = count("skipped")?;
    ingested.compensated = count("compensated")?;
    Ok(ingested)
}

/// The port's error for an answer the endpoint gave as a SQL error.
///
/// The refusal names the event the peer **sent** at the index the server
/// reported, never one rebuilt from the cached identity. A refusal whose index
/// is missing or names no event stays the SQL error it is: still `Stop`, and
/// never a guess. A serialisation failure or a deadlock is `Busy`: the
/// endpoint answered, so it ran the batch and aborted it whole.
fn refused<E>(sql: NeonSqlError, groups: &[IngestGroup<'_>]) -> IngestError<NeonError<E>> {
    match sql.code.as_deref() {
        Some(UNHELD_SQLSTATE) => match claimed(sql.detail.as_deref(), groups) {
            Some(id) => IngestError::unheld_own_identity(id),
            None => IngestError::Store(NeonError::Sql(sql)),
        },
        Some("40001" | "40P01") => IngestError::Busy(NeonError::Sql(sql)),
        _ => IngestError::Store(NeonError::Sql(sql)),
    }
}

/// The foreign event at the 1-based `detail` in the call's batch order, which
/// is the order of the statement's `$1`.
fn claimed(detail: Option<&str>, groups: &[IngestGroup<'_>]) -> Option<EventId> {
    let index = detail?.parse::<usize>().ok()?.checked_sub(1)?;
    groups
        .iter()
        .flat_map(|group| group.events)
        .nth(index)
        .map(|replicated| replicated.id)
}

/// The identity [`NeonIngestStore::open`]'s read answered.
fn decode_identity<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<StoreId, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    let stored = first_result_set::<E>(&body)?
        .text(0, "v")
        .ok_or(NeonError::MissingIdentity)?;
    stored_store_id(stored)
}

/// The watermark [`NeonIngestStore::origin_watermark`]'s read answered.
fn decode_watermark<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<Watermark, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    let mut watermark = Watermark::new();
    for row in &first_result_set::<E>(&body)?.rows {
        let text = |column: &'static str| {
            row.get(column)
                .and_then(serde_json::Value::as_str)
                .ok_or(NeonError::MissingColumn { column })
        };
        watermark.advance(
            stored_store_id(text("s")?)?,
            position_from_text(text("p")?)?,
        );
    }
    Ok(watermark)
}

/// A stored `bytea` identity, which every `SELECT` here asks for as base64.
///
/// # Errors
///
/// `MalformedPayload` when it is not base64; `MalformedIdentity` when it is
/// not sixteen bytes.
fn stored_store_id<E>(stored: &str) -> Result<StoreId, NeonError<E>> {
    let bytes = decode_base64(stored)?;
    let identity: [u8; 16] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| NeonError::MalformedIdentity { len: bytes.len() })?;
    Ok(StoreId::from_bytes(identity))
}

// The whole file is `cfg(test)` already, through its declaration in
// `event_store.rs`. Said again here so the module reads as test code where it
// stands.
#[cfg(test)]
mod tests;
