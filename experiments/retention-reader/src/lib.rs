//! ES-39's reader experiment (#156): does a reader of a store that has
//! forgotten part of its own log need the port to say so?
//!
//! The question, the criteria fixed before any run, the predictions and the
//! verdict are in `README.md` beside this crate. This library is the support
//! the scenarios in `tests/it/` are written against; each scenario asserts
//! what was **observed**, so re-running the experiment on a later tree
//! re-checks the record rather than re-deriving it.
//!
//! - [`domain`]: the events the scenarios write and a projection that counts
//!   how many times it was handed each event.
//! - [`shared`]: a second handle onto one in-memory store that both appends
//!   and ingests.
//! - [`holed`]: CF-27's instrument with the two sync traits it cannot supply
//!   itself, so an edge and a store peer can run over it.
//! - [`prune`]: removal outside the port, a raw `DELETE` on a closed SQLite
//!   file.
//! - [`group_read`]: a SQLite store's `read` served from its write groups,
//!   because its own `read` needs a tokio runtime this crate does not have.
//! - [`seams`]: every answer the port gives about a store, reduced to what can
//!   be compared between a pruned store and a young one.

pub mod domain;
pub mod group_read;
pub mod holed;
pub mod prune;
pub mod seams;
pub mod shared;
