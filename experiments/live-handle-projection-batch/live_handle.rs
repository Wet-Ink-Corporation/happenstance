//! The second batch shape: a **live** borrowed write handle bound to the GAT.
//!
//! # Why this module exists
//!
//! This is not a second adapter anybody should use. It is the instrument for
//! PS-4 and PS-5, and it is here because it **compiles** — which is the
//! opposite of what the specification's reasoning predicts.
//!
//! §4.2 argues the GAT lifetime off the port like this: *"`type Batch<'a> =
//! rusqlite::Transaction<'a>` compiles on the bare flavour and fails on
//! `SendProjectionStore` twice over — `Connection` is `Send` and not `Sync`, so
//! `&Self` is not `Send`; and `Transaction<'_>` is not `Send`, so neither
//! future can be. The flavour every native adapter implements is the one the
//! lifetime does not serve."*
//!
//! Both halves of that are properties of **rusqlite**, not of live handles.
//! LadybugDB's `Connection` is `Send` *and* `Sync`
//! (`docs.rs/lbug/0.16.1/lbug/struct.Connection.html`), so:
//!
//! * `&Self` is `Send`, because `Self` is `Sync`; and
//! * [`GraphWriteHandle`] is `Send`, because a `Connection` is.
//!
//! Neither objection survives, and [`LiveHandleProjectionStore`] below binds a
//! genuinely borrowed, genuinely live transaction handle to `type Batch<'a>` on
//! the **`Send`** flavour — with real bodies, not `todo!()`, so `!` cannot be
//! doing the work. The evidence for dropping the GAT is therefore
//! one-driver-wide. That does not make dropping it wrong; PS-5's *other*
//! argument — that an owned batch removes `error[E0195]` and the `where Self:
//! 'a` bound from every impl — is untouched by this, and this crate reproduces
//! both of those failures too (see [What did not compile](#what-did-not-compile)).
//! It means the clause should rest on that argument rather than on this one.
//!
//! # What did not compile
//!
//! Two failures, both worth carrying forward, both reproduced on
//! rustc 1.97.1.
//!
//! **1. A non-`'static` store crashes the compiler.** The obvious layout for a
//! driver whose connection borrows its database is to let the caller own the
//! `Database` and give the store a lifetime:
//!
//! ```text
//! pub struct LadybugProjectionStore<'db> { connection: Connection<'db> }
//! impl SendProjectionStore for LadybugProjectionStore<'_> { … }
//! ```
//!
//! ```text
//! thread 'rustc' panicked at compiler\rustc_trait_selection\src\errors\note_and_explain.rs:27:22:
//! DefId::expect_local: `DefId(14:546 ~ happenstance_core[4d31]::projection::SendProjectionStore::commit)` isn't local
//! …
//! error: the compiler unexpectedly panicked. This is a bug
//! query stack during panic:
//! #0 [compare_impl_item] checking assoc item `…::commit::{anon_assoc#0}` is compatible with trait definition
//! ```
//!
//! The panic is in the **error-reporting** path, which is the diagnostic worth
//! keeping: rustc had already found a region error on `commit` and crashed
//! while explaining it, because the trait is in another crate. The region error
//! is the GAT's own `where Self: 'a` — for `Self = Store<'db>` it demands
//! `'db: 'a` for the anonymous `'a` in `batch: Self::Batch<'_>`, which nothing
//! proves. Replacing `Self` with a `'static` type makes the same code compile.
//!
//! So today's port is implementable **only by stores that outlive every batch
//! lifetime**, in practice only by `'static` ones, and the failure mode is an
//! ICE rather than a diagnostic. That is a much stronger argument for PS-5 than
//! the `Send` one, and it belongs to phase 6.
//!
//! **2. `error[E0195]`, exactly where PS-5 says it is.** Spelling the concrete
//! batch type in the impl instead of `Self::Batch<'_>` — the natural thing to
//! write when the batch is owned and the lifetime is noise:
//!
//! ```text
//! error[E0195]: lifetime parameters or bounds on method `commit` do not match the trait declaration
//!    --> crates\happenstance-ladybug\src\experiments.rs:37:20
//!     |
//!  37 |       async fn commit(
//!     |                      ^ lifetimes do not match method in trait
//! ```
//!
//! PS-5 claims an owned `type Batch;` "removes `error[E0195]` entirely". This
//! is that error, met from the other side, and it is the trap: the GAT forces
//! every implementer to write `Self::Batch<'_>` in a position where the
//! lifetime means nothing to them.
//!
//! Both transcripts were produced in a scratch module in this crate, deleted
//! once captured — the first one cannot be kept in tree, because a file that
//! ICEs the compiler is a file that fails the gate.

use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};

use crate::projection_store::{GraphStatement, LadybugProjectionStoreError};
use crate::stand_in::{Connection, Database};

/// A live LadybugDB write transaction, borrowed from the store's database.
///
/// Holds a `Connection` on which `BEGIN TRANSACTION` has already run, so the
/// transaction is open for the handle's whole lifetime. `Send`, because
/// `lbug`'s `Connection` is — which is the fact that makes this shape
/// expressible on the `Send` flavour where `rusqlite::Transaction<'a>` is not.
#[derive(Debug)]
pub struct GraphWriteHandle<'db> {
    connection: Connection<'db>,
}

impl<'db> GraphWriteHandle<'db> {
    /// Opens a connection on `database` and enters a write transaction.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if the connection cannot
    /// be opened or the transaction cannot be begun.
    ///
    /// It does **not** return
    /// [`LadybugProjectionStoreError::WriteTransactionInUse`], despite that
    /// variant existing for this call: the stand-in models Ladybug's
    /// single-writer rule in its *types* and not in its behaviour, so nothing
    /// here contends for the write slot and no code path constructs the variant.
    /// It becomes reachable at phase 11, against the real `lbug`. Recorded
    /// rather than deleted because an `# Errors` section promising a variant
    /// nothing can produce is a smaller version of the decorative-rule failure.
    pub fn open(database: &'db Database) -> Result<Self, LadybugProjectionStoreError> {
        let connection = Connection::new(database)?;
        connection.query("BEGIN TRANSACTION")?;
        Ok(Self { connection })
    }

    /// Runs a statement inside the open transaction.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if the statement fails.
    pub fn apply(&self, statement: &GraphStatement) -> Result<(), LadybugProjectionStoreError> {
        self.connection.query(statement.cypher())?;
        Ok(())
    }

    /// Ends the transaction with `terminator`, which is `COMMIT` or `ROLLBACK`.
    ///
    /// Consumes the handle, because the connection's transaction state is not
    /// in the type system and a handle that survived its own `COMMIT` would
    /// silently join whatever transaction opened next.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Commit`] if the terminator fails.
    pub fn finish(self, terminator: &str) -> Result<(), LadybugProjectionStoreError> {
        self.connection
            .query(terminator)
            .map_err(LadybugProjectionStoreError::Commit)?;
        Ok(())
    }
}

/// The same store, with a live borrowed handle as its `Batch`.
///
/// # Status: instrument
///
/// Not a shipping adapter. It exists so that phase 6 has two *unlike* batch
/// shapes in one workspace to freeze against, and so that the claim "the
/// borrowed GAT batch does not work on the `Send` flavour" has a
/// counter-example that compiles. See the [module documentation](self).
#[derive(Debug)]
pub struct LiveHandleProjectionStore {
    database: Database,
}

impl LiveHandleProjectionStore {
    /// Wraps an open database.
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

impl SendProjectionStore for LiveHandleProjectionStore {
    type Error = LadybugProjectionStoreError;

    type Batch<'a>
        = GraphWriteHandle<'a>
    where
        Self: 'a;

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Option<SequencePosition>, Self::Error> {
        let _ = id;
        todo!("phase 11 implements this")
    }

    // Real bodies from here down, deliberately. A `todo!()` has type `!` and
    // coerces to anything, so a skeleton of `todo!()`s proves a signature is
    // *nameable*, not that it can be *satisfied*. These three are the claim
    // this module makes, so they are written out.
    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
        GraphWriteHandle::open(&self.database)
    }

    async fn commit(
        &self,
        batch: Self::Batch<'_>,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), Self::Error> {
        // The checkpoint write goes inside the same transaction, which is the
        // whole point of the port (PS-1).
        batch.apply(&GraphStatement::new(
            "MERGE (c:ProjectionCheckpoint {id: $id}) SET c.position = $position",
            vec![
                (
                    "id".to_owned(),
                    crate::stand_in::Value::String(id.as_str().to_owned()),
                ),
                (
                    "position".to_owned(),
                    crate::stand_in::Value::Int64(i64::try_from(position.get()).map_err(|_| {
                        LadybugProjectionStoreError::PositionOutOfRange { position }
                    })?),
                ),
            ],
        ))?;
        batch.finish("COMMIT")
    }

    async fn rollback(&self, batch: Self::Batch<'_>) -> Result<(), Self::Error> {
        batch.finish("ROLLBACK")
    }
}
