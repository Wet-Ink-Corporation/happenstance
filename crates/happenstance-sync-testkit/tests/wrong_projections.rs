//! SY-20's rule, failing the projections that declare convergence and do not
//! converge, and the cases too weak to tell; and SY-21's, failing the one that
//! lets its checkpoint, a local position, reach its read model.
//!
//! Every mutant is one projection type with one mistake, declared convergent:
//! the declaration is the lie the rule exists to catch. The honest fold, which
//! keeps the value with the greatest `EventId` (SY-23's order), is the control
//! that shows the plumbing is not what fails the others. Each mutant's verdict
//! is asserted whole: the clause, and the sentence that names the mistake.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

use happenstance::bytes::Bytes;
use happenstance::{
    Authority, Checkpoint, Codec, CodecError, CommitError, ConvergentProjection, Delivered,
    DomainEvent, EventId, EventType, InvalidCheckpointId, Postcard, Projection, ProjectionId,
    ProjectionStore, ResetError, SequencePosition, Tag, Tags, checkpoint_id,
};
use happenstance_core::{MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError};
use happenstance_sync_testkit::{
    ConvergenceFixture, CostLayers, CostLayersFixture, Stock, Violation, convergence_rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

const WROTE: EventType = EventType::from_static("Wrote");

/// A value written under a key: a serial's location, say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Wrote {
    key: u64,
    value: u64,
}

impl DomainEvent for Wrote {
    const EVENT_TYPES: &'static [EventType] = &[WROTE];

    fn event_type(&self) -> EventType {
        WROTE
    }

    fn tags(&self) -> Tags {
        Tags::empty()
    }

    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(&(self.key, self.value))
    }

    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError> {
        if event_type != &WROTE {
            return Err(CodecError::UnknownEventType {
                event_type: event_type.clone(),
            });
        }
        let (key, value) = codec.decode(data)?;
        Ok(Self { key, value })
    }
}

/// The one mistake a [`Keyed`] fold makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: per key, the value with the greatest identity. The control.
    None,
    /// `unit-ledger`, the specification's own: per serial, whatever arrived
    /// last. The fold that produced Kestrel Rotor's divergence.
    LastArrivalWins,
    /// Per key, whatever arrived first.
    FirstArrivalWins,
    /// A count of arrivals kept as an order: each value's arrival rank.
    ArrivalRank,
}

/// A fold over keyed writes, wrong in exactly one way.
struct Keyed {
    id: ProjectionId,
    scope: Tags,
    mistake: Mistake,
    held: BTreeMap<u64, (EventId, u64)>,
    arrived: u64,
}

impl Projection for Keyed {
    type Event = Wrote;
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
        event: Delivered<Wrote>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        let id = event.id();
        let Wrote { key, value } = *event.event();
        self.arrived += 1;
        let replaces = match (self.mistake, self.held.get(&key)) {
            (_, None) | (Mistake::LastArrivalWins, Some(_)) => true,
            (Mistake::None | Mistake::ArrivalRank, Some(&(held, _))) => id > held,
            (Mistake::FirstArrivalWins, Some(_)) => false,
        };
        if replaces {
            self.held.insert(key, (id, value));
            batch.write(format!("key_{key}"), value);
        }
        if self.mistake == Mistake::ArrivalRank {
            batch.write(format!("rank_of_{value}"), self.arrived);
        }
        Ok(())
    }
}

// The lie, for every mistake at once; the control is the one it is true of.
impl ConvergentProjection for Keyed {}

/// The two origins write key 1 from both sides, so arrival decides it for
/// every wrong fold, and origin one also writes key 2, which nobody contests.
fn contested() -> [Vec<Wrote>; 2] {
    [
        vec![Wrote { key: 1, value: 10 }, Wrote { key: 2, value: 11 }],
        vec![Wrote { key: 1, value: 20 }],
    ]
}

/// [`Keyed`] over a case, in a scope.
struct KeyedFixture {
    mistake: Mistake,
    authored: fn() -> [Vec<Wrote>; 2],
    scope: fn() -> Tags,
}

impl KeyedFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            mistake,
            authored: contested,
            scope: Tags::empty,
        }
    }
}

impl ConvergenceFixture for KeyedFixture {
    type Projection = Keyed;
    type Codec = Postcard;

    fn projection(&self) -> Keyed {
        Keyed {
            id: ProjectionId::from_static("keyed"),
            scope: (self.scope)(),
            mistake: self.mistake,
            held: BTreeMap::new(),
            arrived: 0,
        }
    }

    fn codec(&self) -> &Postcard {
        &Postcard
    }

    fn authored(&self) -> [Vec<Wrote>; 2] {
        (self.authored)()
    }

    async fn store(&self) -> Result<MemoryProjectionStore, Violation> {
        Ok(MemoryProjectionStore::new())
    }

    async fn read_model(&self, store: &MemoryProjectionStore) -> Result<Vec<u8>, Violation> {
        Ok(format!("{:?}", store.snapshot()).into_bytes())
    }
}

fn verdict(fixture: impl Fn() -> KeyedFixture) -> Result<RuleOutcome, Violation> {
    block_on(
        convergence_rules::convergent_projection_is_interleaving_independent(async || fixture()),
    )
}

/// Asserts the rule failed under SY-20 with a message containing `because`.
fn fails_because(verdict: Result<RuleOutcome, Violation>, because: &str) {
    fails_under("SY-20", verdict, because);
}

/// Asserts the rule failed under `clause` with a message containing `because`.
fn fails_under(clause: &str, verdict: Result<RuleOutcome, Violation>, because: &str) {
    let violation = verdict.expect_err("the rule passed a projection it must fail");
    assert_eq!(violation.clause(), clause, "{violation}");
    assert!(
        violation.message().contains(because),
        "the rule failed for another reason: {violation}"
    );
}

const DIVERGED: &str = "its answer depends on arrival order";

#[test]
fn the_control_converges() {
    assert_eq!(
        verdict(|| KeyedFixture::new(Mistake::None)),
        Ok(RuleOutcome::Ran)
    );
}

#[test]
fn unit_ledger_is_failed() {
    fails_because(
        verdict(|| KeyedFixture::new(Mistake::LastArrivalWins)),
        DIVERGED,
    );
}

#[test]
fn first_arrival_wins_is_failed() {
    fails_because(
        verdict(|| KeyedFixture::new(Mistake::FirstArrivalWins)),
        DIVERGED,
    );
}

#[test]
fn an_arrival_rank_is_failed() {
    fails_because(
        verdict(|| KeyedFixture::new(Mistake::ArrivalRank)),
        DIVERGED,
    );
}

/// One origin with nothing to interleave: both logs are one log, and a fold
/// that keeps the last arrival would pass. Refused, not passed.
#[test]
fn a_case_with_one_origin_is_refused() {
    let one_origin = || KeyedFixture {
        authored: || {
            [
                vec![Wrote { key: 1, value: 10 }, Wrote { key: 1, value: 20 }],
                vec![],
            ]
        },
        ..KeyedFixture::new(Mistake::LastArrivalWins)
    };
    fails_because(verdict(one_origin), "authored no events at origin");
}

/// A scope no event carries: the run applies nothing, both read models are
/// empty, and they agree. Refused, not passed.
#[test]
fn a_case_outside_the_projections_scope_is_refused() {
    let out_of_scope = || KeyedFixture {
        scope: || [Tag::from_static("elsewhere")].into_iter().collect(),
        ..KeyedFixture::new(Mistake::LastArrivalWins)
    };
    fails_because(verdict(out_of_scope), "applied 0 of the 3 events");
}

/// FIFO cost layers, declared convergent: the honest non-convergent fold,
/// lying. The declaration is the only difference from the case SY-22 excludes.
struct DeclaredCostLayers(CostLayers);

impl Projection for DeclaredCostLayers {
    type Event = Stock;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        self.0.id()
    }

    fn scope(&self) -> &Tags {
        self.0.scope()
    }

    async fn apply(
        &mut self,
        event: Delivered<Stock>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.0.apply(event, batch).await
    }
}

impl ConvergentProjection for DeclaredCostLayers {}

struct DeclaredCostLayersFixture(CostLayersFixture);

impl ConvergenceFixture for DeclaredCostLayersFixture {
    type Projection = DeclaredCostLayers;
    type Codec = Postcard;

    fn projection(&self) -> DeclaredCostLayers {
        DeclaredCostLayers(self.0.projection())
    }

    fn codec(&self) -> &Postcard {
        self.0.codec()
    }

    fn authored(&self) -> [Vec<Stock>; 2] {
        self.0.authored()
    }

    async fn store(&self) -> Result<MemoryProjectionStore, Violation> {
        self.0.store().await
    }

    async fn read_model(&self, store: &MemoryProjectionStore) -> Result<Vec<u8>, Violation> {
        self.0.read_model(store).await
    }
}

#[test]
fn cost_layers_declared_convergent_is_failed() {
    let verdict = block_on(
        convergence_rules::convergent_projection_is_interleaving_independent(async || {
            DeclaredCostLayersFixture(CostLayersFixture::new())
        }),
    );
    fails_because(verdict, DIVERGED);
}

// ---------------------------------------------------------------------------
// SY-21: the local position, reached through the one channel left open
// ---------------------------------------------------------------------------

fn sy_21_verdict<F: ConvergenceFixture>(fixture: impl Fn() -> F) -> Result<RuleOutcome, Violation>
where
    F::Projection: ConvergentProjection,
{
    block_on(
        convergence_rules::convergent_projection_cannot_observe_local_position(async || fixture()),
    )
}

const OBSERVED: &str = "its answer depends on where this replica numbered the events";

/// A memory store behind a shared handle. The runner commits through this
/// one, and [`RecordsItsCheckpoint`] reads the same store's checkpoint
/// through its own.
struct Shared(Rc<MemoryProjectionStore>);

impl ProjectionStore for Shared {
    type Error = MemoryProjectionStoreError;
    type Batch = MemoryProjectionBatch;

    async fn begin(&self) -> Result<MemoryProjectionBatch, MemoryProjectionStoreError> {
        self.0.begin().await
    }

    async fn checkpoint(
        &self,
        id: &ProjectionId,
    ) -> Result<Checkpoint, MemoryProjectionStoreError> {
        self.0.checkpoint(id).await
    }

    async fn commit(
        &self,
        batch: MemoryProjectionBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<MemoryProjectionStoreError>> {
        self.0.commit(batch, id, position, authority).await
    }

    async fn reset(
        &self,
        batch: MemoryProjectionBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<MemoryProjectionStoreError>> {
        self.0.reset(batch, id).await
    }

    async fn rollback(
        &self,
        batch: MemoryProjectionBatch,
    ) -> Result<(), MemoryProjectionStoreError> {
        self.0.rollback(batch).await
    }
}

/// Why [`RecordsItsCheckpoint`] could not apply an event.
#[derive(Debug)]
enum Unrecorded {
    /// The store did not answer.
    Store(MemoryProjectionStoreError),
    /// The projection's checkpoint key could not be derived.
    Key(InvalidCheckpointId),
}

impl fmt::Display for Unrecorded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store(error) => write!(f, "the store did not answer: {error}"),
            Self::Key(error) => write!(f, "no checkpoint key: {error}"),
        }
    }
}

impl std::error::Error for Unrecorded {}

impl From<MemoryProjectionStoreError> for Unrecorded {
    fn from(error: MemoryProjectionStoreError) -> Self {
        Self::Store(error)
    }
}

/// A count of writes, which commutes, beside the one mistake SY-21 names: it
/// reads its own checkpoint, as SY-21's falsifier says a projection may, and
/// lets it reach the read model.
///
/// `Delivered` carries no position, so this is the only channel to the local
/// one. Under SY-20's two interleavings the checkpoint sequence is the same,
/// because both logs number the same count of events densely from the first
/// position, so SY-20's rule passes it. Only a second numbering of the same
/// arrivals can tell.
struct RecordsItsCheckpoint {
    id: ProjectionId,
    scope: Tags,
    models: Rc<MemoryProjectionStore>,
    writes: u64,
}

impl Projection for RecordsItsCheckpoint {
    type Event = Wrote;
    type Store = Shared;
    type Error = Unrecorded;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        _event: Delivered<Wrote>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Unrecorded> {
        let key = checkpoint_id(&*self).map_err(Unrecorded::Key)?;
        let through = match self.models.checkpoint(&key).await? {
            Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => through.get(),
            // `NeverRun`, and whatever a later `Checkpoint` adds: nothing
            // committed yet, as far as this fold can say.
            _ => 0,
        };
        self.writes += 1;
        batch.write("writes", self.writes);
        batch.write("checkpoint", through);
        Ok(())
    }
}

// The lie: a fold whose read model holds a local position is not convergent.
impl ConvergentProjection for RecordsItsCheckpoint {}

/// [`RecordsItsCheckpoint`] over the contested case.
struct RecordsItsCheckpointFixture {
    // The rule opens a store and then builds the projection it runs into it,
    // through two `&self` methods, and this projection must hold the store
    // the runner commits into. So the fixture keeps the store it opened last:
    // `Rc` because the test is single-threaded, `RefCell` because each run
    // opens a fresh one.
    opened: RefCell<Rc<MemoryProjectionStore>>,
}

impl RecordsItsCheckpointFixture {
    fn new() -> Self {
        Self {
            opened: RefCell::new(Rc::new(MemoryProjectionStore::new())),
        }
    }
}

impl ConvergenceFixture for RecordsItsCheckpointFixture {
    type Projection = RecordsItsCheckpoint;
    type Codec = Postcard;

    fn projection(&self) -> RecordsItsCheckpoint {
        RecordsItsCheckpoint {
            id: ProjectionId::from_static("records_its_checkpoint"),
            scope: Tags::empty(),
            models: Rc::clone(&self.opened.borrow()),
            writes: 0,
        }
    }

    fn codec(&self) -> &Postcard {
        &Postcard
    }

    fn authored(&self) -> [Vec<Wrote>; 2] {
        contested()
    }

    async fn store(&self) -> Result<Shared, Violation> {
        let models = Rc::new(MemoryProjectionStore::new());
        *self.opened.borrow_mut() = Rc::clone(&models);
        Ok(Shared(models))
    }

    async fn read_model(&self, store: &Shared) -> Result<Vec<u8>, Violation> {
        Ok(format!("{:?}", store.0.snapshot()).into_bytes())
    }
}

/// The checkpoint is the same under both of SY-20's interleavings, so SY-20's
/// rule cannot see this mistake.
#[test]
fn a_recorded_checkpoint_passes_sy_20() {
    let verdict = block_on(
        convergence_rules::convergent_projection_is_interleaving_independent(async || {
            RecordsItsCheckpointFixture::new()
        }),
    );
    assert_eq!(verdict, Ok(RuleOutcome::Ran));
}

/// And SY-21's rule fails it: the same arrivals, numbered twice, commit two
/// read models.
#[test]
fn a_recorded_checkpoint_is_failed_by_sy_21() {
    fails_under(
        "SY-21",
        sy_21_verdict(RecordsItsCheckpointFixture::new),
        OBSERVED,
    );
}

/// SY-21's rule fails that projection and no other here: every fold SY-20's
/// rule fails depends on arrival order, and SY-21's rule holds the order
/// still.
#[test]
fn every_arrival_order_mistake_passes_sy_21() {
    for mistake in [
        Mistake::None,
        Mistake::LastArrivalWins,
        Mistake::FirstArrivalWins,
        Mistake::ArrivalRank,
    ] {
        assert_eq!(
            sy_21_verdict(|| KeyedFixture::new(mistake)),
            Ok(RuleOutcome::Ran),
            "{mistake:?}"
        );
    }
}

/// Cost layers declared convergent, too: FIFO over one arrival order is one
/// answer however the replica numbered it.
#[test]
fn cost_layers_declared_convergent_passes_sy_21() {
    assert_eq!(
        sy_21_verdict(|| DeclaredCostLayersFixture(CostLayersFixture::new())),
        Ok(RuleOutcome::Ran)
    );
}

/// A run that applied nothing commits two empty read models, which agree.
/// Refused under SY-21 as under SY-20, not passed.
#[test]
fn sy_21_refuses_a_case_outside_the_projections_scope() {
    let out_of_scope = || KeyedFixture {
        scope: || [Tag::from_static("elsewhere")].into_iter().collect(),
        ..KeyedFixture::new(Mistake::None)
    };
    fails_under(
        "SY-21",
        sy_21_verdict(out_of_scope),
        "applied 0 of the 3 events",
    );
}
