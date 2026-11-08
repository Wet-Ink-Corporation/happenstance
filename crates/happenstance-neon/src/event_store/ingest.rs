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
//! * the identity guard (ADR-0086): the compensation is gated on the meta row
//!   it stamps from, and [`refusal_statement`] raises
//!   [`UNIDENTIFIED_SQLSTATE`] when that row is gone, so a store with no
//!   identity ingests nothing rather than committing rows with a NULL origin;
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
//! the one exception the port names on its `Store` arm: a round trip that
//! got no answer is [`NeonError::Transport`], and may have committed.
//! Re-delivery is a skip, so a runner's retry converges (SY-11).
//!
//! A dropped `ingest` future may already have committed too, for the reason
//! `event_store`'s `# Cancellation` gives for `append` (ES-23). Reissuing the
//! identical batch is safe for the same reason.
//!
//! # The identity
//!
//! The port is implemented on `NeonEventStore` itself, with no wrapper.
//! `NeonEventStore` holds no identity in Rust at all: every `INSERT` reads it
//! from the meta row (ADR-0086), and so does the port's
//! [`store_id`](IngestStore::store_id), in one read-only round trip on every
//! call, so it can never answer a copy a re-mint has made stale. `ingest`
//! never asks for it: the statement judges a claim against the meta row it
//! reads, so an ingest is still one round trip, and the scripted tests answer
//! only the statements under test.
//!
//! # The bare flavour
//!
//! `NeonEventStore` implements the bare `EventStore` and cannot satisfy
//! `SendEventStore` (`event_store`'s `# Why the bare flavour`), so this
//! implements the bare `IngestStore`, and only that name is in scope
//! (constraint 4). The one place Neon departs from the other two ingest runs,
//! which implement the `Send` flavours.
//!
//! # Why here, and why behind `sync`
//!
//! **Here**, as a child of `event_store`, because a module's private items are
//! visible to its descendants: this reaches the builder, the decoders and the
//! ceilings without any of them being widened. Coherence allows the impls: a
//! foreign trait on a local type.
//!
//! **Behind `sync`** because `happenstance-sync` is an optional dependency, so
//! a consumer who never replicates compiles none of this. It was `cfg(test)`
//! until #108 part E published that crate. The tests stay unit tests under
//! this module, where they reach the builder; the gate checks the feature's
//! `wasm32` build, so the impl is held to the target this crate claims.

use happenstance_core::{EventId, StoreId};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, Watermark,
};

use super::remint::stored_store_id;
use super::{NeonEventStore, decode_body, first_result_set, position_from_text};
use crate::error::{NeonError, NeonSqlError};
use crate::transport::{HttpResponse, SqlRequest, SqlStatement, SqlTransport};
use crate::wire::{ResponseBody, ResultSet};

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

/// The transaction-local setting the ingest statement leaves `absent` in when
/// the meta row it stamps compensations from is gone, or the empty string.
///
/// Transaction-local for [`UNHELD_GUC`]'s reason.
pub(super) const UNIDENTIFIED_GUC: &str = "happenstance.unidentified";

/// The SQLSTATE this adapter raises an ingest into a store with no identity
/// row with, which [`refused`] maps to [`NeonError::MissingIdentity`].
///
/// Chosen by [`UNHELD_SQLSTATE`]'s rules, numbered for ADR-0086.
pub(super) const UNIDENTIFIED_SQLSTATE: &str = "HS086";

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
/// the first statement wrote, when the first found no identity row
/// ([`UNIDENTIFIED_GUC`]) or left a claim in [`UNHELD_GUC`].
///
/// The identity is checked first: a store with no identity row holds no claim
/// to name, and its refusal must not depend on what the batch carried.
///
/// A `DO` block, which takes no bind parameters and needs none: it reads only
/// the two settings. It names no relation, so it has no schema to qualify.
/// `DETAIL` carries the claim's index; the messages are for a human reading a
/// log.
pub(super) fn refusal_statement() -> SqlStatement {
    SqlStatement::new(format!(
        "DO $sy38$ BEGIN \
         IF current_setting('{UNIDENTIFIED_GUC}', true) <> '' THEN \
         RAISE EXCEPTION USING ERRCODE = '{UNIDENTIFIED_SQLSTATE}', \
         MESSAGE = 'this store''s meta table holds no store_id row; was migration 1 \
         applied? (ADR-0086)'; \
         END IF; \
         IF current_setting('{UNHELD_GUC}', true) <> '' THEN \
         RAISE EXCEPTION USING ERRCODE = '{UNHELD_SQLSTATE}', \
         MESSAGE = 'an ingested event claims this store''s own identity at a position it \
         did not hold (SY-38)', \
         DETAIL = current_setting('{UNHELD_GUC}'); \
         END IF; END $sy38$"
    ))
}

impl<T: SqlTransport> NeonEventStore<T> {
    /// Every refusal before the wire, then one round trip.
    async fn ingest_groups(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<NeonError<T::Error>>> {
        let Some(request) = self.ingest_write(groups)? else {
            return Ok(Ingested::default());
        };
        // ES-11, as `append` waits: an ingest is a write.
        self.transport().reads_settled().await;
        let response = self
            .transport()
            .round_trip(request)
            .await
            .map_err(|error| IngestError::Store(NeonError::Transport(error)))?;
        decode_ingest_answer(&response, self.config().max_response_bytes, groups)
    }

    /// The ingest's one request, after every refusal before the wire, or
    /// `None` when the call has nothing foreign to write.
    ///
    /// `NeonSyncPeer`'s push sends this request with the watermark and the
    /// identity read appended, so its ack is read inside the write's own
    /// transaction (#131).
    ///
    /// # Errors
    ///
    /// [`IngestError::ExceedsStoreLimit`] for a call over a ceiling, and
    /// `Store` for a guard or an origin position the request cannot carry.
    pub(super) fn ingest_write(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Option<SqlRequest>, IngestError<NeonError<T::Error>>> {
        // An empty pull writes nothing, and compensation needs a landed event
        // (SY-11), so there is nothing to send.
        if groups.iter().all(|group| group.events.is_empty()) {
            return Ok(None);
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
        if let Some((limit, len)) = Self::exceeded(len, events) {
            return Err(IngestError::exceeds_store_limit(limit, len));
        }

        self.ingest_request::<T::Error>(groups)
            .map(Some)
            .map_err(IngestError::Store)
    }

    /// The highest origin position held from each origin store, this store's
    /// own included. One `GROUP BY` over the origin index, nothing cached.
    async fn origin_watermark(&self) -> Result<Watermark, NeonError<T::Error>> {
        let response = self
            .transport()
            .round_trip(SqlRequest::single(self.watermark_statement()).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config().max_response_bytes)?;
        watermark_from(first_result_set::<T::Error>(&body)?)
    }
}

impl<T> NeonEventStore<T> {
    /// [`WATERMARK_SQL`] over this store's event table.
    pub(super) fn watermark_statement(&self) -> SqlStatement {
        SqlStatement::new(WATERMARK_SQL.replace("{event}", &self.config().qualified_event()))
    }

    /// The meta row's identity, base64, as column `v`.
    pub(super) fn identity_statement(&self) -> SqlStatement {
        SqlStatement::new(format!(
            "SELECT encode(v, 'base64') AS v FROM {} WHERE k = 'store_id'",
            self.config().qualified_meta()
        ))
    }
}

// The bare flavour, the only one `NeonEventStore` has, and only that name in
// scope here (constraint 4).
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
impl<T: SqlTransport> IngestStore for NeonEventStore<T> {
    // The bare adapter error, as the other two runs: the port's `IngestError`
    // carries capacity, contention and the unheld identity in arms of its own.
    type Error = NeonError<T::Error>;

    /// The identity the meta row holds, read on every call in one read-only
    /// round trip. Nothing is cached, as nothing is for `append` (ADR-0086),
    /// so a re-mint is answered by the next call on any handle.
    ///
    /// # Errors
    ///
    /// `Transport`, `Http` or `Sql` as any read reports them;
    /// `MissingIdentity` when the meta row is absent; `MalformedIdentity` when
    /// it is not sixteen bytes; `MalformedPayload` when it is not base64.
    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        let response = self
            .transport()
            .round_trip(SqlRequest::single(self.identity_statement()).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config().max_response_bytes)?;
        identity_from(first_result_set::<T::Error>(&body)?)
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
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
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
            | Self::MissingIdentity
            // A guard the codec refuses either way, or stored write groups the
            // page builder refuses: the same read or write fails the same way.
            | Self::StoredGuard { .. }
            | Self::UnstorableGuard { .. }
            | Self::GroupOrder(_)
            // An answer shaped unlike its request: the same request is
            // answered the same way again.
            | Self::ResultSets { .. } => FailureClass::Stop,
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
    let body = ingest_body(response, max_response_bytes, groups)?;
    ingested_from(first_result_set::<E>(&body).map_err(IngestError::Store)?)
}

/// An ingest answer's body, or the refusal it carries mapped onto the
/// port's error, as [`refused`] maps it.
///
/// # Errors
///
/// [`refused`]'s mapping of a SQL error, and `Store` for every other way
/// [`decode_body`] fails.
pub(super) fn ingest_body<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
    groups: &[IngestGroup<'_>],
) -> Result<ResponseBody, IngestError<NeonError<E>>> {
    match decode_body::<E>(response, max_response_bytes) {
        Ok(body) => Ok(body),
        Err(NeonError::Sql(sql)) => Err(refused(sql, groups)),
        Err(other) => Err(IngestError::Store(other)),
    }
}

/// The counts row an ingest statement answers with.
///
/// # Errors
///
/// `Store(MissingColumn)` for a count that is absent or not a number.
pub(super) fn ingested_from<E>(counts: &ResultSet) -> Result<Ingested, IngestError<NeonError<E>>> {
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
/// reported, never one rebuilt from the store's identity. A refusal whose index
/// is missing or names no event stays the SQL error it is: still `Stop`, and
/// never a guess. A store with no identity row is `MissingIdentity`, the error
/// an append and [`store_id`](IngestStore::store_id) give for it. A
/// serialisation failure or a deadlock is `Busy`: the endpoint answered, so it
/// ran the batch and aborted it whole.
fn refused<E>(sql: NeonSqlError, groups: &[IngestGroup<'_>]) -> IngestError<NeonError<E>> {
    match sql.code.as_deref() {
        Some(UNIDENTIFIED_SQLSTATE) => IngestError::Store(NeonError::MissingIdentity),
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

/// The identity an [`identity_statement`](NeonEventStore::identity_statement)
/// answered.
///
/// # Errors
///
/// `MissingIdentity` when the meta row is absent, and what
/// `stored_store_id` refuses.
pub(super) fn identity_from<E>(set: &ResultSet) -> Result<StoreId, NeonError<E>> {
    let stored = set.text(0, "v").ok_or(NeonError::MissingIdentity)?;
    stored_store_id(stored)
}

/// The watermark a [`watermark_statement`](NeonEventStore::watermark_statement)
/// answered.
///
/// # Errors
///
/// `MissingColumn` for a row without its two columns, and what
/// `stored_store_id` and `position_from_text` refuse.
pub(super) fn watermark_from<E>(set: &ResultSet) -> Result<Watermark, NeonError<E>> {
    let mut watermark = Watermark::new();
    for row in &set.rows {
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

// The whole file is behind `sync`, through its declaration in
// `event_store.rs`, so these tests run in a build with the feature on, which
// the gate's `--all-features` is.
#[cfg(test)]
mod tests;
