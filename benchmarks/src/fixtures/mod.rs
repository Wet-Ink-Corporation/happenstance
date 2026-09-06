//! The arms every figure is taken against, and the floor they are measured
//! above.
//!
//! # Three arms, and why the third is not an adapter
//!
//! * [`memory`] — `MemoryEventStore`, a `Vec` behind an `RwLock`. No I/O at
//!   all, so it is the arm that isolates what the *contract* costs from what
//!   storage costs.
//! * [`sqlite`] — the shipped `SqliteEventStore` on a real file under WAL. The
//!   arm every published figure about this library is actually about.
//! * [`raw`] — **not an implementation of `EventStore`.** Hand-written SQL
//!   against a `rusqlite::Connection`, doing the least a correct event log
//!   could do. It exists to answer the one question
//!   `references/seeds/measured-not-claimed.md:100-104` says nobody can
//!   currently answer: *"what happenstance costs over the database they already
//!   run"*.
//!
//! # Why the fixtures are rewritten here rather than imported
//!
//! `happenstance-sqlite`'s own `SqliteFixture` is `pub(crate)` inside
//! `crates/happenstance-sqlite/tests/support/mod.rs`, which is a test target —
//! a different crate, with no exported surface. There is nothing to import.
//! [`sqlite::SqliteFixture`] is therefore a copy, and it is kept honest the way
//! the house does it: `tests/conformance_first.rs` runs the full
//! `event_store_conformance!` suite against it before any figure taken through
//! it is kept. A fixture that had drifted into being wrong would be *fast*, and
//! the suite is what stops that reading as a finding.

pub mod memory;
pub mod raw;
pub mod sqlite;
