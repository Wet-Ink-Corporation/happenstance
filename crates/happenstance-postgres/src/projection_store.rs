//! Postgres-backed [`SendProjectionStore`].
//!
//! # Status: not implemented
//!
//! # The `Batch` question, and why this adapter is the one that answers it
//!
//! The port declares `type Batch<'a> where Self: 'a`, and the module
//! documentation on it justifies the lifetime with *"a transaction cannot
//! outlive its connection"*. That is true of `rusqlite::Transaction<'a>`, which
//! borrows its `Connection`. It is **not** true here, and that is the finding.
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
//! So this adapter binds an **owned** type to today's GAT:
//!
//! ```text
//! type Batch<'a> = sqlx::Transaction<'static, Postgres> where Self: 'a;
//! ```
//!
//! The lifetime parameter is accepted and then ignored. That is the whole
//! working hypothesis of `PS-5` (*`Batch` must not carry a lifetime parameter*)
//! stated in the only way phase 2 is allowed to state it — as an adapter that
//! does not need the parameter, rather than as a change to the port, which is
//! phase 6's decision. What it costs to write it this way is one `where Self:
//! 'a` clause per impl for a lifetime nothing reads.
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

use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};
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

    // An owned type bound to a generic associated type. See the module
    // documentation: the lifetime is satisfiable and unused, which is the
    // evidence phase 6 needs and not a change to the port.
    type Batch<'a>
        = Transaction<'static, Postgres>
    where
        Self: 'a;

    async fn checkpoint(
        &self,
        _id: &ProjectionId,
    ) -> Result<Option<SequencePosition>, Self::Error> {
        todo!("postgres projection store: read a checkpoint")
    }

    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
        todo!("postgres projection store: begin")
    }

    async fn commit(
        &self,
        _batch: Self::Batch<'_>,
        _id: &ProjectionId,
        _position: SequencePosition,
    ) -> Result<(), Self::Error> {
        todo!("postgres projection store: upsert the checkpoint and commit")
    }

    async fn rollback(&self, _batch: Self::Batch<'_>) -> Result<(), Self::Error> {
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

    /// The substantive half of the owned-batch hypothesis, written so that it
    /// can actually fail.
    ///
    /// The obvious spelling — `assert_static::<Batch<'_>>()` — proves nothing:
    /// the elided lifetime is inferred, the compiler picks `'static`, and the
    /// assertion passes even when `type Batch<'a> = Transaction<'a, Postgres>`.
    /// It was written that way first and it certified the shape it was meant to
    /// reject.
    ///
    /// This form works because `'_` **in argument position** elides to a fresh
    /// universally-quantified lifetime parameter, so the assertion holds only if
    /// the batch type genuinely does not mention it. That is the opposite of
    /// `'_` in a turbofish, which is merely *inferred* — the same two characters
    /// mean different things in the two positions, and that difference is the
    /// whole reason the first version of this test was vacuous. Swapping the
    /// binding to
    /// `type Batch<'a> = Transaction<'a, Postgres>` makes it fail:
    ///
    /// ```text
    /// error: lifetime may not live long enough
    ///     |
    /// 155 |     fn the_batch_does_not_borrow_the_store<'a>(
    ///     |                                            -- lifetime `'a` defined here
    /// ...
    /// 158 |         batch
    ///     |         ^^^^^ returning this value requires that `'a` must outlive `'static`
    /// ```
    fn the_batch_does_not_borrow_the_store(
        batch: <PostgresProjectionStore as ProjectionStore>::Batch<'_>,
    ) -> Transaction<'static, Postgres> {
        batch
    }

    #[test]
    fn the_batch_is_owned() {
        let _ = the_batch_does_not_borrow_the_store;
    }
}
