//! What the convergence family is handed: a projection, its store, and a set
//! of events from two origins — and the two reference cases over one domain.

use alloc::collections::VecDeque;
use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use core::future::Future;

use happenstance::bytes::Bytes;
use happenstance::{
    Codec, CodecError, Delivered, DomainEvent, EventType, Postcard, Projection, ProjectionId, Tags,
};
use happenstance_core::{MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError};

use crate::Violation;

/// The store a fixture's projection commits into.
pub type StoreOf<F> = <<F as ConvergenceFixture>::Projection as Projection>::Store;

/// The domain event a fixture's projection folds.
pub type EventOf<F> = <<F as ConvergenceFixture>::Projection as Projection>::Event;

/// One projection, the events two origins authored, and a way to read what it
/// committed.
///
/// The convergence family's fixture, and a different shape from the peer and
/// ingest families' because its subject is different: not an adapter but a
/// typed fold. The rule replays the two origins' events in two interleavings
/// and runs a fresh [`projection`](Self::projection) over each into a fresh
/// [`store`](Self::store), so every method that builds something is called
/// once per interleaving and must build it anew.
///
/// # No convergence bound here, on purpose (SY-22)
///
/// `Projection` is bounded on [`Projection`] alone. The declaration is
/// required by the rule,
/// [`convergent_projection_is_interleaving_independent`](crate::convergence_rules::convergent_projection_is_interleaving_independent),
/// in its own where-clause, so a fixture over an undeclared projection can be
/// written and cannot be handed to it. That where-clause is the exclusion
/// SY-22 asks for, and [`CostLayersFixture`] is the honest case it excludes.
///
/// # The events are the check's strength
///
/// The rule reorders what it is given and nothing else, so a fold that
/// depends on arrival order is caught only by events whose order it depends
/// on. Hand it two origins that disagree about something the fold keeps — the
/// same key written from both sides, a quantity one consumes from the other —
/// or the rule passes a fold it should not.
pub trait ConvergenceFixture {
    /// The projection under test, in either flavour.
    type Projection: Projection;

    /// The codec its events are written with.
    type Codec: Codec;

    /// A fresh projection, in the state it starts a rebuild in.
    fn projection(&self) -> Self::Projection;

    /// The codec the rule encodes the events with and the runner decodes
    /// them with.
    fn codec(&self) -> &Self::Codec;

    /// The events each origin authored, in authorship order: origin one's,
    /// then origin two's.
    ///
    /// Neither may be empty. An origin with nothing to interleave makes the
    /// two interleavings the same log, and a check over one log is decorative,
    /// so the rule refuses that case rather than passing it.
    fn authored(&self) -> [Vec<EventOf<Self>>; 2];

    /// A fresh, empty store that shares nothing with any other it returned.
    ///
    /// # Errors
    ///
    /// A [`Violation`] if the store cannot be opened: the fixture's own
    /// failure, reported under the clause of the rule that asked.
    fn store(&self) -> impl Future<Output = Result<StoreOf<Self>, Violation>>;

    /// The read model `store` holds, as bytes the rule compares.
    ///
    /// Everything the projection wrote and nothing else: not the checkpoint,
    /// which is a position in the replica's own log.
    ///
    /// # Errors
    ///
    /// A [`Violation`] if the store cannot be read, as for
    /// [`store`](Self::store).
    fn read_model(&self, store: &StoreOf<Self>)
    -> impl Future<Output = Result<Vec<u8>, Violation>>;
}

// ---------------------------------------------------------------------------
// The reference cases: one stock domain, two folds over it
// ---------------------------------------------------------------------------

const RECEIVED: EventType = EventType::from_static("StockReceived");
const ISSUED: EventType = EventType::from_static("StockIssued");

/// A stock movement at one site: the reference cases' domain event.
///
/// Encoded as a `(units, unit_cost)` pair under the fixture's codec, which is
/// why this crate needs no `serde` of its own. An issue carries no cost; what
/// it cost is what the fold says it cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stock {
    /// `units` arrived, each bought at `unit_cost`.
    Received {
        /// How many units arrived.
        units: u64,
        /// What each one cost.
        unit_cost: u64,
    },
    /// `units` left.
    Issued {
        /// How many units left.
        units: u64,
    },
}

impl DomainEvent for Stock {
    const EVENT_TYPES: &'static [EventType] = &[RECEIVED, ISSUED];

    fn event_type(&self) -> EventType {
        match self {
            Self::Received { .. } => RECEIVED,
            Self::Issued { .. } => ISSUED,
        }
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        match *self {
            Self::Received { units, unit_cost } => codec.encode(&(units, unit_cost)),
            Self::Issued { units } => codec.encode(&(units, 0_u64)),
        }
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        let (units, unit_cost): (u64, u64) = codec.decode(data)?;
        if event_type == &RECEIVED {
            Ok(Self::Received { units, unit_cost })
        } else if event_type == &ISSUED {
            Ok(Self::Issued { units })
        } else {
            Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            })
        }
    }
}

/// The two sites both reference cases replay: each receives ten units at its
/// own price and issues four. The prices differ, so which layer an issue
/// consumes is visible in what it cost.
fn two_sites() -> [Vec<Stock>; 2] {
    [
        vec![
            Stock::Received {
                units: 10,
                unit_cost: 3,
            },
            Stock::Issued { units: 4 },
        ],
        vec![
            Stock::Received {
                units: 10,
                unit_cost: 7,
            },
            Stock::Issued { units: 4 },
        ],
    ]
}

/// The bytes a memory store's read model is compared as: every key and value,
/// in key order, one per line.
pub(crate) fn memory_read_model(store: &MemoryProjectionStore) -> Vec<u8> {
    store
        .snapshot()
        .iter()
        .flat_map(|(key, value)| format!("{key}={value}\n").into_bytes())
        .collect()
}

/// Units received and units issued, as two sums.
///
/// `stock-on-hand`: it converges because addition does not care about order,
/// and the declaration below is what makes that a checked property rather than
/// luck (SY-20). The sums saturate, and saturating addition of unsigned values
/// is still commutative and associative, so even an overflow cannot make the
/// answer depend on arrival.
#[derive(Debug)]
pub struct StockOnHand {
    id: ProjectionId,
    scope: Tags,
    received: u64,
    issued: u64,
}

impl Projection for StockOnHand {
    type Event = Stock;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Stock>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        match *event.event() {
            Stock::Received { units, .. } => self.received = self.received.saturating_add(units),
            Stock::Issued { units } => self.issued = self.issued.saturating_add(units),
        }
        batch.write("received", self.received);
        batch.write("issued", self.issued);
        Ok(())
    }
}

impl happenstance::ConvergentProjection for StockOnHand {}

/// The cost of every issue, consuming the oldest units received first.
///
/// `cost-layers`: FIFO consumption order **is** the fold, so what an issue
/// cost depends on which receipt arrived first, and no order exists that two
/// peers both hold. It is correct and it is not convergent, so it does not
/// implement [`ConvergentProjection`](happenstance::ConvergentProjection), and
/// that absence is its SY-22 declaration.
#[derive(Debug)]
pub struct CostLayers {
    id: ProjectionId,
    scope: Tags,
    /// Unconsumed receipts, oldest first: `(units, unit_cost)`.
    layers: VecDeque<(u64, u64)>,
    cost_of_goods_issued: u64,
}

impl CostLayers {
    /// Consumes `units` from the oldest layers, adding what they cost. Units
    /// issued beyond what is held cost nothing: the fold records what was
    /// consumed, and there was nothing to consume.
    fn issue(&mut self, mut units: u64) {
        while units > 0 {
            let Some(layer) = self.layers.front_mut() else {
                return;
            };
            let taken = units.min(layer.0);
            self.cost_of_goods_issued = self
                .cost_of_goods_issued
                .saturating_add(taken.saturating_mul(layer.1));
            // `taken` is the smaller of the two, so neither subtraction can
            // reach below zero; saturating says so without a second proof.
            layer.0 = layer.0.saturating_sub(taken);
            units = units.saturating_sub(taken);
            if layer.0 == 0 {
                self.layers.pop_front();
            }
        }
    }
}

impl Projection for CostLayers {
    type Event = Stock;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Stock>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        match *event.event() {
            Stock::Received { units, unit_cost } => self.layers.push_back((units, unit_cost)),
            Stock::Issued { units } => self.issue(units),
        }
        batch.write("cost_of_goods_issued", self.cost_of_goods_issued);
        Ok(())
    }
}

/// [`StockOnHand`] over two sites: the declared case SY-20's rule is proven
/// against before it may fail anyone's projection.
#[derive(Debug, Default)]
pub struct StockOnHandFixture;

impl StockOnHandFixture {
    /// The fixture. It holds nothing; every projection and store it builds
    /// is fresh.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ConvergenceFixture for StockOnHandFixture {
    type Projection = StockOnHand;
    type Codec = Postcard;

    fn projection(&self) -> StockOnHand {
        StockOnHand {
            id: ProjectionId::from_static("stock_on_hand"),
            scope: Tags::empty(),
            received: 0,
            issued: 0,
        }
    }

    fn codec(&self) -> &Postcard {
        &Postcard
    }

    fn authored(&self) -> [Vec<Stock>; 2] {
        two_sites()
    }

    async fn store(&self) -> Result<MemoryProjectionStore, Violation> {
        Ok(MemoryProjectionStore::new())
    }

    async fn read_model(&self, store: &MemoryProjectionStore) -> Result<Vec<u8>, Violation> {
        Ok(memory_read_model(store))
    }
}

/// [`CostLayers`] over the same two sites: the honest non-convergent case
/// SY-22 excludes.
///
/// A [`ConvergenceFixture`], so the machinery the rule uses can run it, and
/// not one the rule accepts: its projection makes no convergence declaration.
/// [`non_convergent_projections_are_excluded_not_failed`](crate::convergence_rules::non_convergent_projections_are_excluded_not_failed)
/// is where both facts are held.
#[derive(Debug, Default)]
pub struct CostLayersFixture;

impl CostLayersFixture {
    /// The fixture. It holds nothing; every projection and store it builds
    /// is fresh.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl ConvergenceFixture for CostLayersFixture {
    type Projection = CostLayers;
    type Codec = Postcard;

    fn projection(&self) -> CostLayers {
        CostLayers {
            id: ProjectionId::from_static("cost_layers"),
            scope: Tags::empty(),
            layers: VecDeque::new(),
            cost_of_goods_issued: 0,
        }
    }

    fn codec(&self) -> &Postcard {
        &Postcard
    }

    fn authored(&self) -> [Vec<Stock>; 2] {
        two_sites()
    }

    async fn store(&self) -> Result<MemoryProjectionStore, Violation> {
        Ok(MemoryProjectionStore::new())
    }

    async fn read_model(&self, store: &MemoryProjectionStore) -> Result<Vec<u8>, Violation> {
        Ok(memory_read_model(store))
    }
}
