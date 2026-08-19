//! A dependency-free stand-in for the `lbug` crate.
//!
//! # Why this exists
//!
//! `lbug` builds LadybugDB's C++ through `cxx` and `cmake`, which would put a
//! multi-minute native build on every `cargo check` of this workspace. The
//! adapter still has to be *type-checked* against something, and the only part
//! of `lbug` a type checker can disagree with is its shape: what borrows what,
//! and which auto traits each handle carries. So this module reproduces the
//! shape and nothing else. Bodies are `todo!()`; the real dependency arrives at
//! phase 11 and this module goes with it.
//!
//! # Calibration
//!
//! Every declaration below was checked against the published documentation for
//! `lbug` 0.16.1 — the last version docs.rs built, because 0.19.1's build
//! fails, which is itself evidence for keeping the dependency out of the gate.
//! The facts that the adapter's design actually rests on:
//!
//! | Fact | Source |
//! |---|---|
//! | `pub struct Connection<'a>`, built by `Connection::new(database: &'a Database)` | `docs.rs/lbug/0.16.1/lbug/struct.Connection.html` |
//! | `impl Send for Connection<'_>` and `impl Sync for Connection<'_>` — listed under *Trait Implementations*, not *Auto Trait Implementations*, so they are hand-written | same |
//! | `impl Send for Database` and `impl Sync for Database`, likewise hand-written | `…/struct.Database.html` |
//! | `Connection::query(&self, …)` takes `&self`, not `&mut self` | `…/struct.Connection.html` |
//! | `QueryResult<'a>` is `Send` and `!Sync` | `…/struct.QueryResult.html` |
//! | There is **no** `Transaction` type in the crate | the crate index, `…/lbug/index.html` |
//! | `Error` has five variants: `CxxException`, `FailedQuery`, `FailedPreparedStatement`, `ReadOnlyType`, `ArrowError` | `…/enum.Error.html` |
//! | Transactions are Cypher statements — `BEGIN TRANSACTION`, `COMMIT`, `ROLLBACK` — and there can be many readers but **one** writer at a time | `docs.ladybugdb.com/cypher/transaction/` |
//! | Every method is blocking; the crate exposes no futures | the whole API surface, above |
//!
//! # What a stand-in cannot reproduce
//!
//! `Database` and `Connection` are `Send + Sync` in `lbug` because the crate
//! writes `unsafe impl` over a pointer into C++. `unsafe_code` is `forbid` in
//! this workspace, so the stand-in reaches the same *conclusion* by being built
//! out of `Send + Sync` parts. The type checker cannot tell the difference,
//! which is the only thing that matters here — but a reader should know the
//! mechanism differs, because it means this module cannot be wrong in the
//! direction `lbug` could be wrong.

use core::cell::Cell;
use core::fmt;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU64, Ordering};

/// Stand-in for `cxx::Exception`, the error `cxx` raises when a C++ call
/// throws.
///
/// `cxx::Exception` is `Send + Sync`
/// (`docs.rs/cxx/latest/cxx/struct.Exception.html`), which is what lets the
/// adapter's error type be `Send + Sync` without erasing anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exception {
    what: Box<str>,
}

impl Exception {
    /// Wraps the message the C++ side threw.
    pub fn new(what: impl Into<String>) -> Self {
        Self {
            what: what.into().into_boxed_str(),
        }
    }

    /// The message the C++ side threw.
    pub fn what(&self) -> &str {
        &self.what
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.what)
    }
}

impl core::error::Error for Exception {}

/// Stand-in for `lbug::LogicalType`.
///
/// Only the variants the adapter can actually meet on a checkpoint round trip
/// are modelled; the real enum is much wider.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LogicalType {
    /// A boolean.
    Bool,
    /// A 64-bit signed integer. The widest integer LadybugDB stores, and the
    /// reason [`SequencePosition`](happenstance_core::SequencePosition) cannot
    /// round-trip through a property without a checked conversion.
    Int64,
    /// A UTF-8 string.
    String,
    /// A node. Read-only across the FFI boundary, hence
    /// [`Error::ReadOnlyType`].
    Node,
    /// A relationship. Read-only across the FFI boundary.
    Rel,
}

/// Stand-in for `lbug::Value`, the parameter and result type.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Value {
    /// A typed null.
    Null(LogicalType),
    /// A boolean.
    Bool(bool),
    /// A 64-bit signed integer.
    Int64(i64),
    /// A UTF-8 string.
    String(String),
}

/// Stand-in for `lbug::Error`.
///
/// Carries the same five failure modes, minus `ArrowError`, which the real
/// enum gates behind an `arrow` feature this adapter would not enable.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// An exception raised by the LadybugDB C++ library.
    #[error("the LadybugDB C++ library raised an exception: {0}")]
    CxxException(#[from] Exception),
    /// A query that parsed and then failed.
    #[error("query failed: {0}")]
    FailedQuery(String),
    /// A query that failed to prepare.
    #[error("prepared statement failed to prepare: {0}")]
    FailedPreparedStatement(String),
    /// A value whose type cannot cross the FFI boundary.
    #[error("{0:?} is read-only and cannot be passed over the FFI boundary")]
    ReadOnlyType(LogicalType),
}

/// Stand-in for `lbug::SystemConfig`.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct SystemConfig {
    /// Whether the database is opened read-only.
    pub read_only: bool,
}

/// Stand-in for `lbug::Database`, the owner of every database component.
///
/// `Send + Sync`, matching the real type. Note what that does *not* buy: a
/// [`Connection`] borrows it, so a struct holding both a `Database` and a
/// `Connection` into it is self-referential and does not compile.
///
/// That is why [`LadybugProjectionStore`](crate::LadybugProjectionStore) owns
/// its database **by value** and opens a connection per batch, rather than
/// holding one open. An earlier draft gave the store a lifetime instead and this
/// paragraph described it; the lifetime is what produced the rustc ICE recorded
/// in `experiments/live-handle-projection-batch/` (spelled plainly, because that
/// module is no longer in this crate), and it went. The sentence survived
/// the code it described by one revision, which is the reason it is spelled out
/// here rather than quietly corrected.
#[derive(Debug)]
pub struct Database {
    path: Box<str>,
    config: SystemConfig,
}

impl Database {
    /// Opens a database at `path`. An empty path or `":memory:"` opens an
    /// in-memory database.
    ///
    /// This stand-in performs no I/O — it exists so call sites type-check — so
    /// it cannot fail. The signature stays fallible because the real one is,
    /// and a stand-in that is easier to call than the thing it stands in for
    /// has stopped being an instrument.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the real driver cannot open the database.
    pub fn new(path: &str, config: SystemConfig) -> Result<Self, Error> {
        Ok(Self {
            path: path.into(),
            config,
        })
    }

    /// Opens an in-memory database.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the real driver cannot open the database.
    pub fn in_memory(config: SystemConfig) -> Result<Self, Error> {
        Self::new(":memory:", config)
    }

    /// The path this database was opened at.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The configuration this database was opened with.
    pub fn config(&self) -> &SystemConfig {
        &self.config
    }
}

/// Stand-in for `lbug::Connection<'db>`.
///
/// Borrows the [`Database`], is `Send + Sync`, and takes `&self` — not
/// `&mut self` — on every query method. That combination is the whole reason
/// this crate is an interesting instrument: transaction state in LadybugDB
/// lives on the connection and is entered by *executing a statement*
/// (`BEGIN TRANSACTION`), so the type system holds no evidence that a
/// transaction is open. Two write batches sharing one connection would
/// interleave into one transaction and neither the borrow checker nor the
/// driver would object.
#[derive(Debug)]
pub struct Connection<'db> {
    database: &'db Database,
    query_timeout_ms: AtomicU64,
}

impl<'db> Connection<'db> {
    /// Opens a connection against `database`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the real driver cannot open a connection.
    pub fn new(database: &'db Database) -> Result<Self, Error> {
        Ok(Self {
            database,
            query_timeout_ms: AtomicU64::new(0),
        })
    }

    /// The database this connection was opened against.
    pub fn database(&self) -> &'db Database {
        self.database
    }

    /// Runs a Cypher statement.
    ///
    /// Note the receiver: `&self`. This is the real signature, and it is what
    /// makes `BEGIN TRANSACTION` expressible without a `&mut` handle — and
    /// equally what makes an interleaved transaction expressible.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedQuery`] when the statement fails.
    pub fn query(&self, cypher: &str) -> Result<QueryResult<'db>, Error> {
        let _ = cypher;
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }

    /// Prepares a parameterised Cypher statement.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedPreparedStatement`] when the statement will not
    /// prepare.
    pub fn prepare(&self, cypher: &str) -> Result<PreparedStatement, Error> {
        let _ = cypher;
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }

    /// Executes a prepared statement with `parameters`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedQuery`] when execution fails, or
    /// [`Error::ReadOnlyType`] when a parameter's type cannot cross the FFI
    /// boundary.
    pub fn execute(
        &self,
        statement: &mut PreparedStatement,
        parameters: Vec<(&str, Value)>,
    ) -> Result<QueryResult<'db>, Error> {
        let _ = (statement, parameters);
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }

    /// Sets the per-query timeout in milliseconds.
    pub fn set_query_timeout(&self, timeout_ms: u64) {
        self.query_timeout_ms.store(timeout_ms, Ordering::Relaxed);
    }

    /// The per-query timeout in milliseconds.
    pub fn query_timeout(&self) -> u64 {
        self.query_timeout_ms.load(Ordering::Relaxed)
    }
}

/// Stand-in for `lbug::PreparedStatement`.
#[derive(Debug)]
pub struct PreparedStatement {
    cypher: Box<str>,
}

impl PreparedStatement {
    /// The Cypher this statement was prepared from.
    pub fn cypher(&self) -> &str {
        &self.cypher
    }
}

/// Stand-in for `lbug::QueryResult<'db>`.
///
/// `Send` and `!Sync`, matching the real type. The `Cell` is what carries the
/// `!Sync`: the real result holds an interior-mutable cursor into C++, and an
/// adapter that tried to share one across threads must fail to compile here for
/// the same reason it would there.
#[derive(Debug)]
pub struct QueryResult<'db> {
    cursor: Cell<u64>,
    database: PhantomData<&'db Database>,
}

impl QueryResult<'_> {
    /// The number of tuples the query produced.
    pub fn num_tuples(&self) -> u64 {
        let _ = (&self.cursor, &self.database);
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }

    /// The names of the columns the query produced.
    pub fn column_names(&self) -> Vec<String> {
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }

    /// The next row, or `None` when the result is drained.
    pub fn next_row(&mut self) -> Option<Vec<Value>> {
        todo!("phase 11 replaces this module with the real `lbug` crate")
    }
}

#[cfg(test)]
mod tests {
    use super::{Connection, Database, Error, QueryResult};

    const fn assert_send<T: Send>() {}
    const fn assert_sync<T: Sync>() {}

    // These are the calibration facts, restated where the compiler can check
    // that the stand-in did not drift from them. If `lbug` is later found to
    // differ, the fix is here and the adapter's flavour choice is reopened.
    #[test]
    fn database_and_connection_are_send_and_sync() {
        assert_send::<Database>();
        assert_sync::<Database>();
        assert_send::<Connection<'_>>();
        assert_sync::<Connection<'_>>();
    }

    #[test]
    fn query_result_is_send() {
        assert_send::<QueryResult<'_>>();
    }

    #[test]
    fn the_driver_error_is_send_and_sync() {
        assert_send::<Error>();
        assert_sync::<Error>();
    }
}
