//! A run's event store, paired with its identity (PS-40).
//!
//! The identity is what the run's checkpoint is pinned to.

#[cfg(feature = "memory")]
use happenstance_core::MemoryEventStore;
use happenstance_core::StoreId;

/// The event store a projection run reads, and its identity.
///
/// The identity is what the run's checkpoint is pinned to.
/// [`run_projection`](crate::run_projection) and
/// [`run_fan_out`](crate::run_fan_out) take one in place of a bare `&S`.
/// The runner records the identity beside the checkpoint on the first run,
/// and refuses a later run whose identity differs with
/// [`ProjectionError::SourceChanged`](crate::ProjectionError::SourceChanged),
/// because a position means something only in the store that assigned it.
///
/// # Where the identity comes from
///
/// **From the store being passed, never from configuration.** The pairing is
/// the caller's to get right: `Source::new(&b, a_id)` compiles, and it
/// defeats the check. Read it from the store at the call site:
///
/// - `MemoryEventStore`: `(&events).into()`, the one door where the identity
///   cannot be wrong.
/// - `SqliteEventStore`: `Source::new(&events, events.store_id())`.
/// - `CloudflareEventStore`: `Source::new(&events, events.store_id()?)`.
/// - `PostgresEventStore` and `NeonEventStore`:
///   `Source::new(&events, events.store_id().await?)`, one read of the
///   store's identity row, with no feature to enable.
///
/// `EventStore` itself carries no identity, which is why this type exists:
/// adding one to the contract would change every adapter, and the hazard is
/// the typed runner's (ADR-0109).
///
/// ```
/// use happenstance::{MemoryEventStore, Source};
///
/// let events = MemoryEventStore::new();
/// let source: Source<'_, _> = (&events).into();
/// assert_eq!(source.id(), events.store_id());
///
/// let spelled = Source::new(&events, events.store_id());
/// assert_eq!(spelled.id(), source.id());
/// ```
///
/// # Why it is `Copy` without `S: Copy`
///
/// It holds a shared reference and a sixteen-byte identity, both `Copy`, so
/// it is passed by value. `Clone` and `Copy` are written by hand: the
/// derives would add `S: Clone` and `S: Copy` bounds, and no event store is
/// `Copy`.
// No bounds on the struct (API-21): the runners bound `S: EventStore`.
#[derive(Debug)]
pub struct Source<'a, S> {
    events: &'a S,
    id: StoreId,
}

impl<'a, S> Source<'a, S> {
    /// Pairs `events` with the identity it mints under.
    ///
    /// Read `id` from `events` at this call site; see the type's page.
    #[must_use]
    pub const fn new(events: &'a S, id: StoreId) -> Self {
        Self { events, id }
    }

    /// The event store the run reads.
    #[must_use]
    pub const fn events(&self) -> &'a S {
        self.events
    }

    /// The identity the run's checkpoint is pinned to.
    #[must_use]
    pub const fn id(&self) -> StoreId {
        self.id
    }
}

impl<S> Clone for Source<'_, S> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<S> Copy for Source<'_, S> {}

#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
impl<'a> From<&'a MemoryEventStore> for Source<'a, MemoryEventStore> {
    /// The store, with the identity it reports itself.
    fn from(events: &'a MemoryEventStore) -> Self {
        Self::new(events, events.store_id())
    }
}
