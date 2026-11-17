//! `NeonEventStore::remint_identity` and `NeonEventStore::store_id`, and the
//! one decoder of a stored identity every path here shares (#104, ADR-0086,
//! #485).
//!
//! A child of `event_store` so it reaches the private response decoders
//! without widening them, and a file of its own so the lines of `event_store.rs`
//! that the specification and the decisions cite stay where they are.

use happenstance_core::StoreId;

use super::{NeonEventStore, decode_base64, decode_body, first_result_set};
use crate::error::NeonError;
use crate::migration::remint_statement;
use crate::transport::{HttpResponse, SqlRequest, SqlStatement, SqlTransport};
use crate::wire::ResultSet;

impl<T> NeonEventStore<T> {
    /// The meta row's identity, base64, as column `v`.
    pub(super) fn identity_statement(&self) -> SqlStatement {
        SqlStatement::new(format!(
            "SELECT encode(v, 'base64') AS v FROM {} WHERE k = 'store_id'",
            self.config().qualified_meta()
        ))
    }
}

impl<T: SqlTransport> NeonEventStore<T> {
    /// This store's identity, as the meta row holds it, in one read-only
    /// round trip.
    ///
    /// Read on every call and never cached (ADR-0086), so after a
    /// [`remint_identity`](Self::remint_identity) the next call on any handle
    /// answers the new one. The `sync` feature's `IngestStore::store_id` is
    /// this read.
    ///
    /// It is what the typed layer's projection runner pins a view's checkpoint
    /// to: build its `Source` from **this** store's answer, at the call site,
    /// never from configuration, so a view resumed against another branch or
    /// database is refused rather than resumed from positions that mean
    /// something else there (PS-40).
    ///
    /// # Errors
    ///
    /// * [`NeonError::MissingIdentity`] if the meta table has no `store_id`
    ///   row, which means migration 1 has not been applied.
    /// * [`NeonError::MalformedIdentity`] or [`NeonError::MalformedPayload`] if
    ///   the value is not sixteen bytes of base64.
    /// * [`NeonError::Transport`], [`NeonError::Http`], [`NeonError::Sql`] or a
    ///   decoding variant as any read reports them.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use happenstance_neon::{NeonConfig, NeonError, NeonEventStore, SqlTransport};
    ///
    /// # async fn identity<T: SqlTransport>(transport: T) -> Result<(), NeonError<T::Error>> {
    /// let events = NeonEventStore::new(transport, NeonConfig::default());
    /// let id = events.store_id().await?;
    /// // `happenstance::Source::new(&events, id)` pairs the store with it.
    /// assert_eq!(id, events.store_id().await?, "the same row, read twice");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn store_id(&self) -> Result<StoreId, NeonError<T::Error>> {
        let response = self
            .transport()
            .round_trip(SqlRequest::single(self.identity_statement()).read_only())
            .await
            .map_err(NeonError::Transport)?;
        let body = decode_body::<T::Error>(&response, self.config().max_response_bytes)?;
        identity_from(first_result_set::<T::Error>(&body)?)
    }

    /// Replaces this store's persisted identity with a freshly minted one, and
    /// returns it.
    ///
    /// # When a deployment invokes this
    ///
    /// **On every Neon branch that is created, reset or restored, and after a
    /// `pg_restore` or a point-in-time recovery**, before the copy accepts its
    /// first append. A branch is a copy-on-write clone of its parent, so it
    /// carries the parent's identity row together with an older position
    /// sequence, and would then re-issue [`EventId`](happenstance_core::EventId)s
    /// the parent already issued, which a replication peer drops as already
    /// seen, with no error anywhere. VT-6 permits this store's mint-once only
    /// because the deployment is documented to invoke a re-mint, and this is
    /// that re-mint (ADR-0086). Minting afresh on every open was considered and
    /// declined there: a Workers deployment builds a store per request.
    ///
    /// Stop every writer to the copy, call this, then resume appends. Tooling
    /// that makes branches and holds no store runs the same statement,
    /// [`migration::remint_statement`](crate::migration::remint_statement).
    ///
    /// # Handles
    ///
    /// This store holds no identity in Rust: every append reads the meta row
    /// inside its own statement. So every handle and clone, in this isolate or
    /// another, stamps the new identity on its next append, and nothing has to
    /// be rebuilt.
    ///
    /// # What it does not change
    ///
    /// Events already in the store keep the identity they were stamped with,
    /// which is correct: they were written by the incarnation being retired. A
    /// re-mint when no copy was made breaks nothing; it only starts a new
    /// origin for later events. It applies no migration, and refuses rather
    /// than minting into a schema migration 1 never ran on.
    ///
    /// # Errors
    ///
    /// * [`NeonError::MissingIdentity`] if the meta table has no `store_id`
    ///   row, which means migration 1 has not been applied. No row is created.
    /// * [`NeonError::MalformedIdentity`] or [`NeonError::MalformedPayload`] if
    ///   the value read back is not sixteen bytes of base64.
    /// * [`NeonError::Transport`], [`NeonError::Http`], [`NeonError::Sql`] or a
    ///   decoding variant as any round trip reports them. A `Transport` failure
    ///   may have committed. Calling again is the remedy, because a re-mint
    ///   never makes an identity collide.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use happenstance_neon::{NeonConfig, NeonError, NeonEventStore, SqlTransport};
    ///
    /// # async fn after_branch<T: SqlTransport>(transport: T) -> Result<(), NeonError<T::Error>> {
    /// // Every writer to the new branch is stopped.
    /// let store = NeonEventStore::new(transport, NeonConfig::default());
    /// let _new_identity = store.remint_identity().await?;
    /// // Appends may resume, through this handle or any other.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn remint_identity(&self) -> Result<StoreId, NeonError<T::Error>> {
        let request = SqlRequest::single(remint_statement(self.config()));
        let response = self
            .transport()
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)?;
        decode_reminted(&response, self.config().max_response_bytes)
    }
}

/// The identity the re-mint statement's `RETURNING` carried.
///
/// # Errors
///
/// `MissingIdentity` when the update matched no row; `MissingColumn` when it
/// matched one but the answer does not carry it as text; the decoders' errors
/// otherwise.
fn decode_reminted<E>(
    response: &HttpResponse,
    max_response_bytes: usize,
) -> Result<StoreId, NeonError<E>> {
    let body = decode_body::<E>(response, max_response_bytes)?;
    let [row, ..] = first_result_set::<E>(&body)?.rows.as_slice() else {
        return Err(NeonError::MissingIdentity);
    };
    let column = "store_id";
    let stored = row
        .get(column)
        .and_then(serde_json::Value::as_str)
        .ok_or(NeonError::MissingColumn { column })?;
    stored_store_id(stored)
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

/// A stored `bytea` identity, which every `SELECT` here asks for as base64.
///
/// # Errors
///
/// `MalformedPayload` when it is not base64; `MalformedIdentity` when it is
/// not sixteen bytes.
pub(super) fn stored_store_id<E>(stored: &str) -> Result<StoreId, NeonError<E>> {
    let bytes = decode_base64(stored)?;
    let identity: [u8; 16] = bytes
        .as_slice()
        .try_into()
        .map_err(|_| NeonError::MalformedIdentity { len: bytes.len() })?;
    Ok(StoreId::from_bytes(identity))
}
