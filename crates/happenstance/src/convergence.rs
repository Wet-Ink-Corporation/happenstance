//! The convergence declaration a projection makes, and a check bounds on.
//!
//! Its own module, as `Delivered` is, and a top-level one, because
//! `tests/doc_budget.rs` reads `src/*.rs` and nothing below it. The trait and
//! its fences live here alone, so a different spelling is a contained swap.

/// A projection whose read model is the same in any ingest order.
///
/// # What the declaration promises (SY-20)
///
/// Two stores that hold the same events, ingested in two interleavings that
/// agree on each origin's order, commit byte-identical read models. A fold
/// that keeps the promise reads only the event and
/// [`Delivered::id`](crate::Delivered::id): the origin identity, including
/// `id().position()`, which every replica shares. Where the fold needs an
/// order, `EventId` order is the one every peer agrees on (SY-23).
///
/// # What it cannot reach (SY-21)
///
/// The arrival position, and the type already says so: `Delivered` has no
/// accessor for it, for any projection. The declaration adds no restriction
/// the argument has not already made. What stays the implementer's is state
/// outside the events, such as a clock, a count of arrivals kept as an
/// order, or a field of `self` read as if it were history. Nothing here
/// checks the claim; `happenstance-sync-testkit`'s
/// `convergent_projection_is_interleaving_independent` will (phase 13).
///
/// # Not declaring is the non-convergent declaration (SY-22)
///
/// A projection that does not implement this trait is left out of a
/// convergence check, not failed by it: a check bound on this trait cannot
/// be handed one. A fold whose answer is its arrival order, such as FIFO
/// cost layers, stays undeclared, and every other bound still takes it.
///
/// # Either flavour, one name
///
/// Implement it beside [`Projection`](crate::Projection) or
/// [`SendProjection`](crate::SendProjection). Generic code writes
/// `P: Projection + ConvergentProjection`, or, where the run is spawned,
/// the spawner bounds on `Projection`'s page plus `+ ConvergentProjection`.
///
/// # Why it is not `ConvergentProjection: Projection`
///
/// That supertrait puts `P: Projection` in scope as a where-clause, which
/// the compiler prefers to the blanket impl `SendProjection` gets. Beside a
/// `SendProjection` bound it loses `Send` on `apply`'s future (`E0277`),
/// stops `P::Store` normalising (`E0271`) and makes `p.id()` ambiguous
/// (`E0034`), so generic code could not require convergence and spawn the
/// run in one body. Return-type notation would repair it and is unstable at
/// 1.97.1. The cost of this spelling is that the trait does not itself
/// require a projection: a declaration on any other type compiles, and
/// means nothing.
///
/// # Declaring it
///
/// The first fence compiles, and is the control: it goes red if the trait
/// is renamed or gains a supertrait `Sold` does not meet. It needs `memory`
/// and `json`. A `: Projection` supertrait is refused elsewhere, by the
/// generic spawn test in `tests/convergence.rs`.
///
/// ```
/// use happenstance::{ConvergentProjection, Projection};
/// # use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// # use happenstance::{Delivered, EventType, ProjectionId, Tags};
/// # use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
/// # use happenstance::MemoryProjectionStoreError;
/// # #[derive(serde::Serialize, serde::Deserialize)]
/// # enum Seat { Taken }
/// # const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// # impl DomainEvent for Seat {
/// #     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
/// #     fn event_type(&self) -> EventType { SEAT_TAKEN }
/// #     fn tags(&self) -> Tags { Tags::empty() }
/// #     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
/// #         c.encode(self)
/// #     }
/// #     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
/// #         -> Result<Self, CodecError> { c.decode(d) }
/// # }
/// # struct Sold { id: ProjectionId, scope: Tags, sold: u64 }
/// # impl happenstance::SendProjection for Sold {
/// #     type Event = Seat;
/// #     type Store = MemoryProjectionStore;
/// #     type Error = MemoryProjectionStoreError;
/// #     fn id(&self) -> &ProjectionId { &self.id }
/// #     fn scope(&self) -> &Tags { &self.scope }
/// #     async fn apply(
/// #         &mut self,
/// #         _: Delivered<Seat>,
/// #         batch: &mut MemoryProjectionBatch,
/// #     ) -> Result<(), MemoryProjectionStoreError> {
/// #         self.sold += 1;
/// #         batch.write("sold", self.sold);
/// #         Ok(())
/// #     }
/// # }
/// // A count is a sum, and a sum does not care about order.
/// impl ConvergentProjection for Sold {}
///
/// fn check<P: Projection + ConvergentProjection>(_: &P) {}
///
/// let sold = Sold {
///     id: ProjectionId::from_static("seats_sold"),
///     scope: Tags::empty(),
///     sold: 0,
/// };
/// check(&sold);
/// ```
///
/// The second is the same projection, undeclared, and does not compile: the
/// check cannot be handed it. Rustdoc 1.97.1 does not check the error code
/// on a `compile_fail` fence (RS-62-1), so the `E0277` below is a statement
/// of intent and the control above is what keeps this fence honest.
///
/// ```compile_fail,E0277
/// use happenstance::{ConvergentProjection, Projection};
/// # use happenstance::{bytes::Bytes, Codec, CodecError, DomainEvent};
/// # use happenstance::{Delivered, EventType, ProjectionId, Tags};
/// # use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
/// # use happenstance::MemoryProjectionStoreError;
/// # #[derive(serde::Serialize, serde::Deserialize)]
/// # enum Seat { Taken }
/// # const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
/// # impl DomainEvent for Seat {
/// #     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
/// #     fn event_type(&self) -> EventType { SEAT_TAKEN }
/// #     fn tags(&self) -> Tags { Tags::empty() }
/// #     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
/// #         c.encode(self)
/// #     }
/// #     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
/// #         -> Result<Self, CodecError> { c.decode(d) }
/// # }
/// # struct Sold { id: ProjectionId, scope: Tags, sold: u64 }
/// # impl happenstance::SendProjection for Sold {
/// #     type Event = Seat;
/// #     type Store = MemoryProjectionStore;
/// #     type Error = MemoryProjectionStoreError;
/// #     fn id(&self) -> &ProjectionId { &self.id }
/// #     fn scope(&self) -> &Tags { &self.scope }
/// #     async fn apply(
/// #         &mut self,
/// #         _: Delivered<Seat>,
/// #         batch: &mut MemoryProjectionBatch,
/// #     ) -> Result<(), MemoryProjectionStoreError> {
/// #         self.sold += 1;
/// #         batch.write("sold", self.sold);
/// #         Ok(())
/// #     }
/// # }
/// fn check<P: Projection + ConvergentProjection>(_: &P) {}
///
/// let sold = Sold {
///     id: ProjectionId::from_static("seats_sold"),
///     scope: Tags::empty(),
///     sold: 0,
/// };
/// check(&sold);
/// ```
#[diagnostic::on_unimplemented(
    message = "`{Self}` has not declared itself convergent",
    label = "not declared convergent",
    note = "implement `ConvergentProjection` only for a projection whose \
            read model is identical under any ingest interleaving (SY-20); \
            one that is not stays out of the convergence check (SY-22)"
)]
pub trait ConvergentProjection {}
