//! Postgres-backed [`SendProjectionStore`].
//!
//! # Status: not implemented
//!
//! # The `Batch` question, and why this adapter is the one that answers it
//!
//! The port used to declare `type Batch<'a> where Self: 'a`, and the module
//! documentation on it justified the lifetime with *"a transaction cannot
//! outlive its connection"*. That is true of `rusqlite::Transaction<'a>`, which
//! borrows its `Connection`. It was **not** true here, and that was the finding
//! — PS-5 has since removed the parameter, and the disposition is at the end of
//! this section.
//!
//! [`PgPool::begin`](sqlx::PgPool::begin) is declared, in sqlx 0.8.6's own
//! source:
//!
//! ```text
//! pub async fn begin(&self) -> Result<Transaction<'static, DB>, Error> {
//!     Transaction::begin(
//!         MaybePoolConnection::PoolConnection(self.acquire().await?),
//!         None,
//!     )
//!     .await
//! }
//! ```
//!
//! The `'static` is not a convenience: the transaction takes the
//! `PoolConnection` by value and returns it to the pool on drop. A driver with
//! every opportunity to hand back a handle borrowed from `&self` chose to hand
//! back an owned one — and it is a *pooled, networked* driver, which is the case
//! the borrowed shape was supposed to serve best.
//!
//! This adapter bound an **owned** type to that GAT — `type Batch<'a> =
//! sqlx::Transaction<'static, Postgres> where Self: 'a` — accepting the
//! lifetime parameter and then ignoring it. `PS-5` has since removed the
//! parameter from the port, so the binding is now simply:
//!
//! ```text
//! type Batch = sqlx::Transaction<'static, Postgres>;
//! ```
//!
//! The batch type, the error type and the storage strategy are unchanged: what
//! the port's shape change cost this adapter is the deletion of a `where Self:
//! 'a` clause for a lifetime nothing read.
//!
//! # The invariant
//!
//! A read-model write and its checkpoint write must be one transaction, or a
//! restart either replays applied events or skips unapplied ones. Postgres makes
//! that easy — [`commit`](happenstance_core::ProjectionStore::commit) does the
//! checkpoint `UPSERT` on the same [`Transaction`] the caller
//! has been writing through, and then commits once.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE projection_checkpoint (
//!     projection_id text   PRIMARY KEY,
//!     position      bigint NOT NULL
//! );
//! ```
//!
//! Read-model tables are the application's business; this adapter owns the
//! checkpoint and the transaction that carries it.

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::error::PostgresProjectionStoreError;

/// A Postgres-backed projection store.
///
/// # Status: not implemented
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PostgresProjectionStore {
    pool: PgPool,
}

impl PostgresProjectionStore {
    /// Wraps an existing pool.
    ///
    /// Sharing one pool with
    /// [`PostgresEventStore`](crate::event_store::PostgresEventStore) is the
    /// expected deployment, which is why neither type builds its own.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// The pool this store writes through.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl SendProjectionStore for PostgresProjectionStore {
    type Error = PostgresProjectionStoreError;

    // The same owned type as before, with the port's lifetime parameter — and
    // the `where Self: 'a` clause it forced — deleted rather than rebound. See
    // the module documentation.
    type Batch = Transaction<'static, Postgres>;

    // Neither `async` nor fallible now. `PgPool::begin` is both, so this
    // adapter's real body will acquire the pooled connection at the first
    // statement rather than here — which is PS-6's stated arrangement, not a
    // problem it creates.
    fn begin(&self) -> Self::Batch {
        todo!("postgres projection store: begin")
    }

    async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        todo!("postgres projection store: read a checkpoint")
    }

    async fn commit(
        &self,
        _batch: Self::Batch,
        _id: &ProjectionId,
        _position: SequencePosition,
        _authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        todo!("postgres projection store: upsert the checkpoint and commit")
    }

    async fn reset(
        &self,
        _batch: Self::Batch,
        _id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        todo!("postgres projection store: apply the caller's deletes and clear the checkpoint")
    }

    async fn rollback(&self, _batch: Self::Batch) -> Result<(), Self::Error> {
        todo!("postgres projection store: rollback")
    }
}

#[cfg(test)]
mod tests {
    use super::{Postgres, PostgresProjectionStore, Transaction};
    use happenstance_core::ProjectionStore;

    /// The bare flavour must come free from the `Send` one, at this concrete
    /// store rather than in the abstract.
    #[test]
    fn send_flavour_satisfies_the_bare_bound() {
        fn assert_projection_store<P: ProjectionStore>() {}
        assert_projection_store::<PostgresProjectionStore>();
    }

    /// The substantive half of the owned-batch hypothesis, restated for
    /// `type Batch;` and still able to fail.
    ///
    /// # What the old spelling was about, and why it is gone
    ///
    /// Against the GAT this test had to be written in argument position:
    /// `assert_static::<Batch<'_>>()` proved nothing, because `'_` in a
    /// turbofish is merely *inferred* and the compiler simply picked
    /// `'static` — so the assertion passed even against
    /// `type Batch<'a> = Transaction<'a, Postgres>`, the exact shape it existed
    /// to reject. It was written that way first. Argument position was the fix,
    /// because `'_` there elides to a fresh universally-quantified lifetime.
    ///
    /// The port no longer has a lifetime to elide, so that hazard cannot
    /// recur — but the test is not therefore vacuous, and it is not deleted.
    /// Two wrong shapes remain for it to reject, and it rejects both:
    ///
    /// 1. **A batch that is not this adapter's own owned type.** Rebinding
    ///    `type Batch` to anything but `Transaction<'static, Postgres>` makes
    ///    this function `error[E0308]`. That is the AC-013 claim — the adapter
    ///    keeps the batch type its author chose — checked rather than asserted.
    /// 2. **A batch that borrows.** `type Batch;` is still bindable to a
    ///    borrowing type by a store that carries a lifetime of its own
    ///    (`impl ProjectionStore for Store<'db> { type Batch = Handle<'db>; }`),
    ///    which is precisely what the workspace's live-handle instrument did.
    ///    [`the_batch_is_owned`] asserts `'static` on the *normalised* type,
    ///    with no lifetime anywhere for inference to paper over.
    fn the_batch_does_not_borrow_the_store(
        batch: <PostgresProjectionStore as ProjectionStore>::Batch,
    ) -> Transaction<'static, Postgres> {
        batch
    }

    #[test]
    fn the_batch_is_owned() {
        fn assert_static<T: 'static>() {}

        assert_static::<<PostgresProjectionStore as ProjectionStore>::Batch>();
        let _ = the_batch_does_not_borrow_the_store;
    }
}
