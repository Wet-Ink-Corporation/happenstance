//! The Neon-backed [`ProjectionStore`], and the batch that is not a transaction.
//!
//! # What this is here to falsify
//!
//! [`ProjectionStore`]'s `Batch` is a generic associated type,
//! `type Batch<'a> where Self: 'a`, because "a transaction cannot outlive the
//! connection that opened it". Neon has no connection and no transaction, so the
//! lifetime has nothing to borrow from. Binding an **owned** type to the GAT —
//! `type Batch<'a> = NeonWriteBatch;` — is what this crate does, and it compiles:
//! a GAT is free to ignore its parameter.
//!
//! That makes this adapter evidence for the owned `type Batch;` hypothesis from
//! the far end of the transport axis. It also surfaces two places where today's
//! port asks for something this adapter has no way to mean, both recorded as
//! findings rather than fixed here:
//!
//! * [`ProjectionStore::begin`] is `async` and fallible. There is nothing to
//!   open and nothing to fail; `begin` is a `Vec::new()`. The `Result` is
//!   uninhabitable in practice and the `async` is a future that is ready on
//!   first poll.
//! * [`ProjectionStore::rollback`] is `async` and fallible. Dropping the
//!   statement list is the rollback, and it cannot fail either — because
//!   nothing was ever sent.
//!
//! # Where the atomicity actually comes from
//!
//! The port's invariant is that the read-model write and the checkpoint write
//! land together. This adapter gets that not from a transaction handle but from
//! the **non-interactive batch**: `commit` appends the checkpoint `UPSERT` to
//! the accumulated statements and sends all of them as one array in one round
//! trip, which the endpoint runs server-side inside one `BEGIN`/`COMMIT`. The
//! invariant is honoured. What is lost is the ability to *decide* anything
//! between two statements of the batch — which the projection port, unlike the
//! event store port, never asks for.

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError, SequencePosition,
};

use crate::config::NeonConfig;
use crate::error::NeonError;
use crate::transport::{HttpResponse, SqlRequest, SqlStatement, SqlTransport};

/// An accumulated list of statements, owned outright.
///
/// Not a transaction. Nothing has been sent when one of these exists, and
/// nothing is holding a lock, a snapshot or a connection — because there is no
/// connection to hold. It is a `Vec<SqlStatement>` that becomes a single
/// non-interactive transaction at
/// [`commit`](ProjectionStore::commit) time and is otherwise inert.
///
/// This is what `type Batch<'a>`'s lifetime is bound to in the impl below: an
/// owned type that ignores the parameter entirely.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct NeonWriteBatch {
    /// The statements to run, in order, inside one server-side transaction.
    pub statements: Vec<SqlStatement>,
}

impl NeonWriteBatch {
    /// An empty batch.
    pub const fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    /// Queues a statement.
    pub fn push(&mut self, statement: SqlStatement) {
        self.statements.push(statement);
    }

    /// How many statements are queued.
    ///
    /// Worth watching: the whole batch travels in one request body, so a
    /// projection that applies ten thousand events before committing builds a
    /// ten-thousand-statement JSON document and sends it in one round trip.
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether anything is queued.
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// A projection store over Neon's serverless `/sql` HTTP endpoint.
///
/// # Status: not implemented
///
/// ```
/// use happenstance_core::ProjectionStore;
/// use happenstance_neon::{NeonConfig, NeonProjectionStore, NullTransport};
///
/// fn takes_a_projection_store<P: ProjectionStore>(_store: &P) {}
///
/// let store = NeonProjectionStore::new(NullTransport::new(), NeonConfig::default());
/// takes_a_projection_store(&store);
/// ```
#[derive(Debug, Clone)]
pub struct NeonProjectionStore<T> {
    transport: T,
    config: NeonConfig,
}

impl<T> NeonProjectionStore<T> {
    /// Builds a projection store over `transport`.
    pub const fn new(transport: T, config: NeonConfig) -> Self {
        Self { transport, config }
    }

    /// The transport this store sends round trips through.
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// The table names and limits this store was built with.
    pub const fn config(&self) -> &NeonConfig {
        &self.config
    }
}

impl<T: SqlTransport> NeonProjectionStore<T> {
    /// The `SELECT` that reads one projection's checkpoint.
    fn checkpoint_request(&self, _id: &ProjectionId) -> SqlRequest {
        todo!("neon: compile a checkpoint read")
    }

    /// The batch's statements plus the checkpoint `UPSERT`, as one array.
    fn commit_request(
        &self,
        _batch: NeonWriteBatch,
        _id: &ProjectionId,
        _position: SequencePosition,
        _authority: Authority,
    ) -> SqlRequest {
        todo!("neon: append the checkpoint upsert and build the batch request")
    }

    /// The batch's statements plus the checkpoint `DELETE`, as one array.
    fn reset_request(&self, _batch: NeonWriteBatch, _id: &ProjectionId) -> SqlRequest {
        todo!("neon: append the checkpoint delete and build the batch request")
    }
}

// The `+ 'static` that used to sit on this `impl` header is gone, and its
// removal is a consequence rather than a choice.
//
// `ProjectionStore` declared `type Batch<'a> where Self: 'a` and then took
// `Self::Batch<'_>` by value in `commit` and `rollback`. That anonymous lifetime
// was late-bound and universally quantified, so the compiler had to discharge
// `NeonProjectionStore<T>: 'a` for *every* `'a` — which is `T: 'static` and
// nothing weaker. Without it, `cargo check` reported four `error[E0311]`, two of
// them pointing into `happenstance-core/src/projection.rs` itself.
//
// The consequence was a port constraint nobody had written down: **any
// projection-store adapter generic over a type parameter was forced to
// `'static` by the GAT**, whether or not its batch borrowed anything. This
// one's batch borrows nothing at all. That finding went to phase 6, ADR-0017
// removed the lifetime, and with no `'a` left to quantify over the obligation
// has nothing to discharge — so the bound goes with it.
impl<T: SqlTransport> ProjectionStore for NeonProjectionStore<T> {
    type Error = NeonError<T::Error>;

    // The same owned batch as before, now bound to an associated type that no
    // longer carries a lifetime this adapter had no use for.
    type Batch = NeonWriteBatch;

    // No round trip, and now no `async` and no `Result` either. There is
    // nothing to open and nothing that can fail: this adapter is the one PS-6
    // is written for, and the shape finally says so.
    fn begin(&self) -> Self::Batch {
        NeonWriteBatch::new()
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let request = self.checkpoint_request(id);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)?;
        decode_checkpoint::<T::Error>(&response)
    }

    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        // The only round trip in the whole port, and the only moment at which
        // any of this became durable.
        let request = self.commit_request(batch, id, position, authority);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)
            .map_err(CommitError::Store)?;
        decode_commit::<T::Error>(&response).map_err(CommitError::Store)
    }

    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        // Same one round trip, with a `DELETE` of the checkpoint row in place of
        // the `UPSERT`: the caller's own statements carry the read-model
        // deletes, and the whole array is one request.
        let request = self.reset_request(batch, id);
        let response = self
            .transport
            .round_trip(request)
            .await
            .map_err(NeonError::Transport)
            .map_err(ResetError::Store)?;
        decode_reset::<T::Error>(&response).map_err(ResetError::Store)
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        // Dropping the statements *is* the rollback. There is no server-side
        // state to undo, because nothing was sent.
        drop(batch);
        Ok(())
    }
}

/// The checkpoint row, or [`Checkpoint::NeverRun`] when there is none.
///
/// An absent row is `NeverRun` rather than `None`: the port's return type is a
/// three-variant enum, so the row must also carry which authority the last
/// commit claimed.
fn decode_checkpoint<E>(_response: &HttpResponse) -> Result<Checkpoint, NeonError<E>> {
    todo!("neon: decode a checkpoint row")
}

/// Confirms every statement in the batch reported success.
fn decode_commit<E>(_response: &HttpResponse) -> Result<(), NeonError<E>> {
    todo!("neon: check every result set in the batch response")
}

/// Confirms the deletes and the checkpoint removal both reported success.
fn decode_reset<E>(_response: &HttpResponse) -> Result<(), NeonError<E>> {
    todo!("neon: check every result set in the reset response")
}

#[cfg(test)]
mod tests {
    use super::{NeonProjectionStore, NeonWriteBatch};
    use crate::config::NeonConfig;
    use crate::transport::NullTransport;
    use happenstance_core::ProjectionStore;

    /// The bare projection-store flavour, at a concrete transport.
    fn assert_bare_flavour<P: ProjectionStore>() {}

    #[test]
    fn the_store_is_a_bare_projection_store() {
        assert_bare_flavour::<NeonProjectionStore<NullTransport>>();
    }

    /// The owned batch really is owned: it outlives the borrow it was made
    /// from, which a `rusqlite::Transaction<'a>`-shaped batch could not.
    #[test]
    fn the_batch_owns_itself() {
        let batch = {
            let store = NeonProjectionStore::new(NullTransport::new(), NeonConfig::default());
            let _ = &store;
            NeonWriteBatch::new()
        };
        assert!(batch.is_empty());
    }
}
