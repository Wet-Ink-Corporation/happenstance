//! The projection store port.
//!
//! # Status: provisional
//!
//! Unlike [`EventStore`](crate::EventStore), this port is **not yet frozen**.
//! It is defined here so the seam is visible and so adapter crates have
//! something to compile against, but the conformance suite does not cover it
//! yet — and a port without a conformance suite is a guess. It will be settled
//! in the pass that lands the first real projection adapter, where the design
//! can be checked against an actual transaction API rather than an imagined
//! one. Treat its shape as subject to change.
//!
//! # The invariant that drives the design
//!
//! A read model and its checkpoint must move together. If the read-model write
//! commits and the checkpoint write does not, a restart replays events that
//! were already applied; if the checkpoint commits first, a crash silently
//! skips events. Neither is acceptable, and no amount of ordering or retrying
//! fixes it — the two writes must be **one** transaction.
//!
//! So the port cannot offer `apply()` and `set_checkpoint()` as independent
//! calls. It hands out an adapter-owned batch — a SQL transaction, a Ladybug
//! write handle — and takes it back at commit time together with the position.
//! The batch is a generic associated type because only the adapter knows what
//! it is, and it borrows from the store because a transaction cannot outlive
//! its connection.
//!
//! Projections that cannot be made transactional with their checkpoint must
//! instead be made **idempotent**, so that replaying an event is harmless. That
//! is a property of the projection, not of this port.

use alloc::boxed::Box;
use alloc::string::String;

use crate::event::SequencePosition;

/// Names a read model within a projection store.
///
/// Distinct projections advance independently, so each needs its own
/// checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionId(Box<str>);

impl ProjectionId {
    /// Creates a projection identifier.
    ///
    /// **Infallible, and that is an open question rather than a decision.**
    /// Both sibling identifiers — [`EventType`](crate::EventType) and
    /// [`Tag`](crate::Tag) — validate and return a `Result`; this one accepts
    /// anything, including the empty string, and the value becomes the primary
    /// key of a checkpoint row.
    ///
    /// Do not read the inconsistency as a deliberate "opaque operator-chosen
    /// key" design. There is no decision behind it. It is left standing because
    /// `ProjectionId` belongs to [`ProjectionStore`], which is provisional, has
    /// no conformance suite, and is frozen at a later phase — and a validating
    /// constructor with nothing able to check it would be exactly the decorative
    /// rule this project's conformance discipline exists to prevent. Adding a
    /// fallible `parse` beside this constructor would be worse than either
    /// choice: two constructors enforcing different rules is the defect that
    /// makes an invalid value reachable through the weaker one.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into().into_boxed_str())
    }

    /// The identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ProjectionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// A store that holds read models and their replay checkpoints.
///
/// See the [module documentation](self) for the transactional invariant this
/// shape exists to enforce, and for its provisional status.
///
/// As with [`EventStore`](crate::EventStore), this is the `!Send` flavour and
/// the one to use in bounds; adapters that can be `Send` should implement
/// [`SendProjectionStore`] and get this for free.
#[trait_variant::make(SendProjectionStore: Send)]
pub trait ProjectionStore {
    /// How this adapter fails.
    type Error: core::error::Error + 'static;

    /// An in-flight write against this store.
    ///
    /// Borrows from `Self` because a transaction cannot outlive the connection
    /// that opened it. Applying an event mutates this; it becomes durable only
    /// when handed to [`commit`](ProjectionStore::commit).
    type Batch<'a>
    where
        Self: 'a;

    /// The position this projection has been brought up to, or `None` if it has
    /// never run.
    ///
    /// Feed this to [`ReadOptions::from`](crate::ReadOptions::from) — after
    /// advancing past it — to resume a replay.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the checkpoint cannot be read.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Option<SequencePosition>, Self::Error>;

    /// Opens a write batch.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if a transaction cannot be started.
    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error>;

    /// Commits the batch and advances `id`'s checkpoint to `position`, as one
    /// atomic unit.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the commit fails. The batch is consumed
    /// either way; a failed commit must leave the store unchanged.
    async fn commit(
        &self,
        batch: Self::Batch<'_>,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), Self::Error>;

    /// Discards the batch without committing.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the rollback fails.
    async fn rollback(&self, batch: Self::Batch<'_>) -> Result<(), Self::Error>;
}
