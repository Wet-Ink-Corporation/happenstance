//! `PostgresEventStore::store_id`: this store's identity, with no feature
//! (#485, ADR-0109).
//!
//! A child of `event_store` so it reaches `IDENTITY_SQL`, the row decoder and
//! the runtime hop without widening them, and a file of its own so the lines
//! of `event_store.rs` that the specification and the decisions cite stay
//! where they are. The `sync` feature's `IngestStore::store_id` delegates here.

use happenstance_core::StoreId;

use super::{IDENTITY_SQL, PostgresEventStore, identity_from_row};
use crate::error::PostgresEventStoreError;

impl PostgresEventStore {
    /// This store's identity, as `store_meta` holds it.
    ///
    /// Read on every call and never cached (ADR-0086), so after a
    /// [`remint_identity`](Self::remint_identity) the next call on any handle,
    /// clones included, answers the new one.
    ///
    /// It is what the typed layer's projection runner pins a view's checkpoint
    /// to: build its `Source` from **this** store's answer, at the call site,
    /// never from configuration, so a view resumed against another database
    /// is refused rather than resumed from positions that mean something else
    /// there (PS-40).
    ///
    /// # Errors
    ///
    /// * [`PostgresEventStoreError::MissingIdentity`] if `store_meta` has no
    ///   `store_id` row, which means migration 1 has not been applied.
    /// * [`PostgresEventStoreError::MalformedIdentity`] if the row does not
    ///   read back as sixteen bytes.
    /// * [`PostgresEventStoreError::Driver`] if the server refuses the
    ///   statement, and [`PostgresEventStoreError::NoRuntime`] or
    ///   [`PostgresEventStoreError::Worker`] as for every other operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use happenstance_postgres::error::PostgresEventStoreError;
    /// use happenstance_postgres::event_store::PostgresEventStore;
    /// use happenstance_postgres::sqlx::PgPool;
    ///
    /// # async fn identity(pool: PgPool) -> Result<(), PostgresEventStoreError> {
    /// let events = PostgresEventStore::new(pool);
    /// let id = events.store_id().await?;
    /// // `happenstance::Source::new(&events, id)` pairs the store with it.
    /// assert_eq!(id, events.store_id().await?, "the same row, read twice");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn store_id(&self) -> Result<StoreId, PostgresEventStoreError> {
        // A cheap handle onto the pool, moved into the runtime hop.
        let pool = self.pool.clone();
        let stored: Option<Vec<u8>> = self
            .on_runtime(async move {
                sqlx::query_scalar(IDENTITY_SQL)
                    .fetch_optional(&pool)
                    .await
                    .map_err(PostgresEventStoreError::from)
            })
            .await?;
        identity_from_row(stored)
    }
}
