//! `CloudflareEventStore::store_id`: this log's incarnation, with no feature
//! (#485, ADR-0109).
//!
//! A child of `event_store` so it reaches the private read without widening
//! it, and a file of its own so the lines of `event_store.rs` that the
//! specification and the decisions cite stay where they are. The `sync`
//! feature's `IngestStore::store_id` is this read.

use happenstance_core::StoreId;

use super::{CloudflareEventStore, CloudflareEventStoreError};

impl CloudflareEventStore {
    /// This log's incarnation, as `store_meta` holds it.
    ///
    /// Read on every call and never cached, so after a
    /// [`remint_identity`](Self::remint_identity) the next call on any handle
    /// answers the new one. [`migrate`](Self::migrate) mints it; this reads
    /// and never mints.
    ///
    /// It is what the typed layer's projection runner pins a view's checkpoint
    /// to: build its `Source` from **this** store's answer, at the call site,
    /// never from configuration, so a view resumed against another object's
    /// log is refused rather than resumed from positions that mean something
    /// else there (PS-40).
    ///
    /// # Errors
    ///
    /// Returns [`CloudflareEventStoreError::Sql`] if the identity row cannot
    /// be read, which includes an object [`migrate`](Self::migrate) has never
    /// run on, and [`CloudflareEventStoreError::ColumnType`] or
    /// [`CloudflareEventStoreError::RowShape`] if what is read is not a 16-byte
    /// blob.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use happenstance_cloudflare::{CloudflareEventStore, CloudflareEventStoreError, SqlStorage};
    ///
    /// # fn identity(sql: SqlStorage) -> Result<(), CloudflareEventStoreError> {
    /// let events = CloudflareEventStore::new(sql);
    /// events.migrate()?;
    /// let id = events.store_id()?;
    /// // `happenstance::Source::new(&events, id)` pairs the store with it.
    /// assert_eq!(id, events.store_id()?, "the same row, read twice");
    /// # Ok(())
    /// # }
    /// ```
    pub fn store_id(&self) -> Result<StoreId, CloudflareEventStoreError> {
        self.incarnation()
    }
}
