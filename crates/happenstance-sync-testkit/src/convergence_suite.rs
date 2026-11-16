//! The convergence family: SY-20's check over a declared projection, SY-21's
//! check that it cannot see the local position, and SY-22's proof that the
//! check excludes what it must not fail.
//!
//! # How a check of convergence reorders without a second peer
//!
//! Convergence is a property of a fold, not of a peer, so no rule here holds
//! a peer at all (SY-8). Two peers that ingested the same events differ only
//! in the order they appended them, and that difference is reproduced exactly
//! by two replica logs: each event keeps its origin's [`EventId`], and the
//! local position is the arrival order. The rule writes both logs with
//! [`MemoryEventStore::restore`], which keeps every store-assigned fact it is
//! handed, and runs the fixture's projection over each through the shipped
//! [`run_projection`]. Whether a real ingest store assigns those positions is
//! the ingest family's question (SY-5), not this one's.
//!
//! The two interleavings are origin one's events then origin two's, and the
//! reverse. They agree on each origin's order and reverse every pair across
//! origins, and they are the only such pair: an interleaving that alternates
//! has no partner that reverses all of its cross-origin pairs while keeping
//! each origin's order. So the two are exactly what SY-20 quantifies over,
//! "agree on per-origin order and disagree everywhere else", and a third,
//! alternating interleaving would hold a fold to more than the clause says.
//!
//! SY-21's check holds the order still and moves the numbering instead. The
//! same arrivals, origin one's first, are written to two replicas: one
//! numbered densely from the first position, one from far past it with gaps
//! between, which the event-store contract permits. `Delivered` hands a fold
//! no position; the channel to the local position that the port a fold is
//! given offers is the projection store's checkpoint. The rule compares read
//! models, not channels, so a fold that lets the local numbering reach its
//! read model by any route commits two, and the checkpoint is the route the
//! wrong projection in `tests/` takes. SY-20's interleavings cannot see that
//! fold: both are numbered densely over the same count of events, so the
//! checkpoint runs through the same values in each.
//!
//! Every event carries one [`RecordedAt`]. `Delivered` does not hand a fold
//! the time, and a fold that found it another way could order by it only
//! with ties broken by arrival, which is arrival order again.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::{NonZeroU64, NonZeroUsize};

use happenstance::{Chunk, DomainEvent, run_projection};
use happenstance_core::{
    Event, EventId, MemoryEventStore, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};

use crate::Violation;
use crate::convergence_fixture::{ConvergenceFixture, EventOf};
use crate::violation::ensure;

/// The first origin's identity. Distinct from the second's and the replica's,
/// so no identity coincides with a local position by construction.
const ORIGIN_ONE: StoreId = StoreId::from_bytes([0x0a; 16]);
/// The second origin's identity.
const ORIGIN_TWO: StoreId = StoreId::from_bytes([0x0b; 16]);
/// The replica both interleavings are ingested into.
const REPLICA: StoreId = StoreId::from_bytes([0x0c; 16]);
/// One instant for every event.
const RECORDED: RecordedAt = RecordedAt::from_millis(0);
/// Two events per commit, so a fold's state crosses a commit boundary in
/// every case with more than two events.
const CHUNK: NonZeroUsize = NonZeroUsize::MIN.saturating_add(1);
/// One event per commit, for SY-21's runs: every event after the first is
/// applied over a committed checkpoint, so a fold that reads it sees a local
/// position in every case the fixture contract admits, which is two events
/// at least.
const ONE_AT_A_TIME: NonZeroUsize = NonZeroUsize::MIN;
/// Where SY-21's second replica starts numbering: far from the first
/// position, so every event's local position differs between the two
/// replicas.
const OFFSET_FROM: NonZeroU64 = NonZeroU64::MIN.saturating_add(999);
/// The gap between its consecutive positions, so the numbering is not only
/// shifted but gapped.
const OFFSET_STRIDE: NonZeroU64 = NonZeroU64::MIN.saturating_add(2);

/// One event as its origin wrote it: the identity every replica shares, and
/// the encoded event.
type Authored = (EventId, Event);

/// Each origin's events, identified and encoded once, in authorship order.
fn author<F: ConvergenceFixture>(
    fixture: &F,
    clause: &'static str,
) -> Result<[Vec<Authored>; 2], Violation> {
    let [one, two] = fixture.authored();
    Ok([
        identify(ORIGIN_ONE, one, fixture, clause)?,
        identify(ORIGIN_TWO, two, fixture, clause)?,
    ])
}

fn identify<F: ConvergenceFixture>(
    origin: StoreId,
    events: Vec<EventOf<F>>,
    fixture: &F,
    clause: &'static str,
) -> Result<Vec<Authored>, Violation> {
    ensure(!events.is_empty(), clause, || {
        format!(
            "the fixture authored no events at origin {origin}, so both interleavings are one \
             log and the check would compare it with itself; hand the rule events from both \
             origins"
        )
    })?;
    let mut authored = Vec::with_capacity(events.len());
    for (there, event) in (1..).zip(events) {
        let position = SequencePosition::new(there).ok_or_else(|| {
            Violation::new(clause, "an origin position counted from one was zero")
        })?;
        let data = event.encode(fixture.codec()).map_err(|error| {
            Violation::new(
                clause,
                format!("the fixture's event did not encode: {error}"),
            )
        })?;
        let encoded = Event::new(event.event_type(), data)
            .map_err(|error| {
                Violation::new(
                    clause,
                    format!("the fixture's event type is invalid: {error}"),
                )
            })?
            .with_tags(event.tags());
        authored.push((EventId::new(origin, position), encoded));
    }
    Ok(authored)
}

/// The replica that appended `arrivals` in that order, numbered densely from
/// the first position.
fn replica<'a>(
    arrivals: impl Iterator<Item = &'a Authored>,
    clause: &'static str,
) -> Result<MemoryEventStore, Violation> {
    replica_at(arrivals, NonZeroU64::MIN, NonZeroU64::MIN, clause)
}

/// The replica that appended `arrivals` in that order, numbering them from
/// `first` with `stride` between consecutive positions.
fn replica_at<'a>(
    arrivals: impl Iterator<Item = &'a Authored>,
    first: NonZeroU64,
    stride: NonZeroU64,
    clause: &'static str,
) -> Result<MemoryEventStore, Violation> {
    let mut log = Vec::new();
    let mut here = Some(first.get());
    for (id, event) in arrivals {
        let position = here.and_then(SequencePosition::new).ok_or_else(|| {
            Violation::new(clause, "a local position ran past the end of the key space")
        })?;
        here = position.get().checked_add(stride.get());
        // Genuinely duplicated: the same event is appended to both replicas'
        // logs, and each store keeps its own. The payload is a `Bytes`, so
        // the copy is of the envelope, not the data.
        log.push(SequencedEvent::new(position, *id, RECORDED, event.clone()));
    }
    Ok(MemoryEventStore::restore(REPLICA, log))
}

/// What a fresh projection commits over `log`, read back as bytes.
async fn committed<F: ConvergenceFixture>(
    fixture: &F,
    log: &MemoryEventStore,
    expected: usize,
    order: &str,
    chunk: NonZeroUsize,
    clause: &'static str,
) -> Result<Vec<u8>, Violation> {
    let models = fixture.store().await?;
    let mut projection = fixture.projection();
    let progressed = run_projection(
        log,
        &models,
        &mut projection,
        fixture.codec(),
        Chunk::of(chunk),
    )
    .await
    .map_err(|error| {
        Violation::new(
            clause,
            format!("the projection's run over {order} failed: {error}"),
        )
    })?;
    ensure(progressed.applied == expected, clause, || {
        format!(
            "the projection applied {} of the {expected} events over {order}, so the check saw \
             less than the case; every event the fixture authors must be in the projection's \
             scope",
            progressed.applied
        )
    })?;
    fixture.read_model(&models).await
}

/// The read models a fresh projection commits under each interleaving:
/// origin one first, then origin two first.
///
/// Not bounded on the declaration, so SY-22's rule can run the case the
/// declaration excludes through exactly the machinery SY-20's rule uses.
pub(crate) async fn read_models<F: ConvergenceFixture>(
    fixture: &F,
    clause: &'static str,
) -> Result<[Vec<u8>; 2], Violation> {
    let [one, two] = author(fixture, clause)?;
    let expected = one.len().saturating_add(two.len());
    let one_first = replica(one.iter().chain(&two), clause)?;
    let two_first = replica(two.iter().chain(&one), clause)?;
    Ok([
        committed(
            fixture,
            &one_first,
            expected,
            "origin one first",
            CHUNK,
            clause,
        )
        .await?,
        committed(
            fixture,
            &two_first,
            expected,
            "origin two first",
            CHUNK,
            clause,
        )
        .await?,
    ])
}

/// The read models a fresh projection commits over one arrival order, origin
/// one's events first, under two numberings of it: dense from the first
/// position, then offset and gapped.
async fn read_models_numbered_twice<F: ConvergenceFixture>(
    fixture: &F,
    clause: &'static str,
) -> Result<[Vec<u8>; 2], Violation> {
    let [one, two] = author(fixture, clause)?;
    let expected = one.len().saturating_add(two.len());
    let dense = replica(one.iter().chain(&two), clause)?;
    let offset = replica_at(one.iter().chain(&two), OFFSET_FROM, OFFSET_STRIDE, clause)?;
    Ok([
        committed(
            fixture,
            &dense,
            expected,
            "dense local positions",
            ONE_AT_A_TIME,
            clause,
        )
        .await?,
        committed(
            fixture,
            &offset,
            expected,
            "offset, gapped local positions",
            ONE_AT_A_TIME,
            clause,
        )
        .await?,
    ])
}

/// A read model as a reader of a failure message wants it.
pub(crate) fn shown(model: &[u8]) -> String {
    format!("{:?}", String::from_utf8_lossy(model))
}

/// Hands the complete convergence rule set to `$callback`.
///
/// This family's single enumeration (CF-22), beside the rules it names. No
/// mounting macro reads it, because the family has no adapter to mount it
/// beside: a projection author calls the SY-20 and SY-21 rules over a fixture
/// of their own, and the SY-22 rule takes nothing.
///
/// # Examples
///
/// ```
/// macro_rules! rule_names {
///     ($($name:ident),* $(,)?) => { [ $( stringify!($name) ),* ] };
/// }
///
/// let names = happenstance_sync_testkit::for_each_convergence_rule!(rule_names);
/// assert_eq!(
///     names,
///     [
///         "convergent_projection_is_interleaving_independent",
///         "convergent_projection_cannot_observe_local_position",
///         "non_convergent_projections_are_excluded_not_failed",
///     ]
/// );
/// ```
#[macro_export]
macro_rules! for_each_convergence_rule {
    ($($callback:tt)+) => {
        $($callback)+! {
            // --- A declared projection converges (SY-20) ---------------------
            convergent_projection_is_interleaving_independent,

            // --- ... and cannot see the local position (SY-21) ---------------
            convergent_projection_cannot_observe_local_position,

            // --- An undeclared one is excluded, not failed (SY-22) -----------
            non_convergent_projections_are_excluded_not_failed,
        }
    };
}

/// The convergence rules (SY-20, SY-21, SY-22).
///
/// Each returns `Result<RuleOutcome, Violation>`, as every rule in this crate
/// does. None has a capability to decline, so none ever reports a skip.
pub mod convergence_rules {
    use happenstance::ConvergentProjection;
    use happenstance_testkit::RuleOutcome;

    use super::{read_models, read_models_numbered_twice, shown};
    use crate::Violation;
    use crate::convergence_fixture::{ConvergenceFixture, CostLayersFixture, StockOnHandFixture};
    use crate::violation::ensure;

    /// **SY-20.** A declared projection commits byte-identical read models
    /// under two interleavings that agree on each origin's order and disagree
    /// everywhere else.
    ///
    /// The fixture's projection is run twice, fresh each time, over the two
    /// origins' events: origin one's first, then origin two's first. Its read
    /// models are compared byte for byte. A fixture whose projection has not
    /// declared [`ConvergentProjection`] cannot be handed to this rule; that
    /// is SY-22's exclusion, and
    /// [`non_convergent_projections_are_excluded_not_failed`] holds it.
    ///
    /// Rejects `unit-ledger`, a per-serial state machine that keeps whatever
    /// arrived last, and with it the class: any fold whose answer depends on
    /// arrival — a count of arrivals kept as an order, a first or last
    /// writer by arrival, a queue consumed in arrival order.
    /// `tests/wrong_projections.rs` holds each against this rule.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-20 if the two read models differ, if either
    /// origin authored nothing, if a run applied fewer events than it was
    /// handed or failed, or if the fixture could not open or read a store.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_sync_testkit::{StockOnHandFixture, convergence_rules};
    /// use happenstance_testkit::{RuleOutcome, block_on};
    ///
    /// let outcome = block_on(
    ///     convergence_rules::convergent_projection_is_interleaving_independent(async || {
    ///         StockOnHandFixture::new()
    ///     }),
    /// )?;
    /// assert_eq!(outcome, RuleOutcome::Ran);
    /// # Ok::<(), happenstance_sync_testkit::Violation>(())
    /// ```
    pub async fn convergent_projection_is_interleaving_independent<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: ConvergenceFixture,
        F::Projection: ConvergentProjection,
    {
        const CLAUSE: &str = "SY-20";
        let fixture = open().await;
        let [one_first, two_first] = read_models(&fixture, CLAUSE).await?;
        ensure(one_first == two_first, CLAUSE, || {
            format!(
                "a projection declared convergent committed {} with origin one's events first \
                 and {} with origin two's first; its answer depends on arrival order",
                shown(&one_first),
                shown(&two_first)
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// **SY-21.** A declared projection cannot observe the local position:
    /// the same arrivals, numbered twice, commit byte-identical read models.
    ///
    /// The fixture's projection is run twice, fresh each time and one event
    /// per commit, over one arrival order, origin one's events first. The
    /// first replica numbers the events densely from the first position; the
    /// second numbers them from far past it, with gaps between. `Delivered`
    /// already withholds the position from the fold (SY-21's compile-fail
    /// fences). The rule compares read models, so any channel that lets the
    /// local numbering reach them fails it; the one the port a fold is given
    /// offers is its store's checkpoint.
    ///
    /// That is a reading of SY-21, not its letter. The clause forbids handing
    /// the arrival position "in the value it folds over", and its reopening
    /// condition grants a fold the store's own checkpoint. This rule takes
    /// the clause to mean that a fold may read its checkpoint but must not
    /// let it reach the read model: SY-20's property, applied to positions.
    ///
    /// Rejects a projection that writes its checkpoint into its read model,
    /// which SY-20's rule passes, since both of its interleavings number the
    /// same count of events densely. `tests/wrong_projections.rs` holds it
    /// against both rules.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-21 if the two read models differ, if either
    /// origin authored nothing, if a run applied fewer events than it was
    /// handed or failed, or if the fixture could not open or read a store.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_sync_testkit::{StockOnHandFixture, convergence_rules};
    /// use happenstance_testkit::{RuleOutcome, block_on};
    ///
    /// let outcome = block_on(
    ///     convergence_rules::convergent_projection_cannot_observe_local_position(async || {
    ///         StockOnHandFixture::new()
    ///     }),
    /// )?;
    /// assert_eq!(outcome, RuleOutcome::Ran);
    /// # Ok::<(), happenstance_sync_testkit::Violation>(())
    /// ```
    pub async fn convergent_projection_cannot_observe_local_position<F>(
        open: impl AsyncFn() -> F,
    ) -> Result<RuleOutcome, Violation>
    where
        F: ConvergenceFixture,
        F::Projection: ConvergentProjection,
    {
        const CLAUSE: &str = "SY-21";
        let fixture = open().await;
        let [dense, offset] = read_models_numbered_twice(&fixture, CLAUSE).await?;
        ensure(dense == offset, CLAUSE, || {
            format!(
                "a projection declared convergent committed {} over the arrivals numbered \
                 densely and {} over the same arrivals numbered from elsewhere with gaps; its \
                 answer depends on where this replica numbered the events",
                shown(&dense),
                shown(&offset)
            )
        })?;
        Ok(RuleOutcome::Ran)
    }

    /// **SY-22.** A non-convergent projection is left out of the convergence
    /// check, not failed by it.
    ///
    /// Two halves. The exclusion is
    /// [`convergent_projection_is_interleaving_independent`]'s where-clause:
    /// a fixture over [`CostLayers`](crate::CostLayers), which makes no
    /// declaration, does not compile into it. And the exclusion is needed:
    /// this rule runs cost layers through the same machinery the check uses,
    /// and requires the two read models to **differ**, beside
    /// [`StockOnHand`](crate::StockOnHand) over the same events, which must
    /// agree. Without the exclusion, an honest projection is failed.
    ///
    /// Rejects a convergence check with no exclusion, which fails cost layers;
    /// and a check that does not really reorder — one interleaving replayed
    /// twice — under which cost layers would agree with itself and every
    /// projection would pass. If cost layers is ever expressed convergently,
    /// this rule fails too, and that is SY-22's own falsifier firing.
    ///
    /// # Errors
    ///
    /// A [`Violation`] of SY-22 if stock on hand diverges, if cost layers
    /// does not, or if either run fails.
    ///
    /// # The exclusion, compiled
    ///
    /// A fixture over an undeclared projection is a [`ConvergenceFixture`],
    /// and every bound but the check's still takes it. This is the control:
    ///
    /// ```
    /// use happenstance_sync_testkit::{ConvergenceFixture, CostLayersFixture};
    ///
    /// fn any_case<F: ConvergenceFixture>(_: &F) {}
    /// any_case(&CostLayersFixture::new());
    /// ```
    ///
    /// The check is not one of them. Rustdoc 1.97.1 does not check the error
    /// code on a `compile_fail` fence (RS-62-1), so the `E0277` is a statement
    /// of intent. Two controls keep the fence from passing on a typo: the
    /// `any_case` control above, which names the same fixture, and the SY-20
    /// rule's own example, which makes the same call in the same shape:
    ///
    /// ```compile_fail,E0277
    /// use happenstance_sync_testkit::{CostLayersFixture, convergence_rules};
    ///
    /// let check = convergence_rules::convergent_projection_is_interleaving_independent(
    ///     async || CostLayersFixture::new(),
    /// );
    /// drop(check);
    /// ```
    pub async fn non_convergent_projections_are_excluded_not_failed()
    -> Result<RuleOutcome, Violation> {
        const CLAUSE: &str = "SY-22";
        let [one_first, two_first] = read_models(&StockOnHandFixture::new(), CLAUSE).await?;
        ensure(one_first == two_first, CLAUSE, || {
            format!(
                "stock on hand, two sums, committed {} and {} under the two interleavings; the \
                 check differs on something other than arrival order, so it cannot tell a \
                 convergent fold from one that is not",
                shown(&one_first),
                shown(&two_first)
            )
        })?;
        let [one_first, two_first] = read_models(&CostLayersFixture::new(), CLAUSE).await?;
        ensure(one_first != two_first, CLAUSE, || {
            format!(
                "cost layers committed {} under both interleavings. Either the check no longer \
                 reorders the origins, and SY-20's rule passes any fold, or FIFO cost layers \
                 has been expressed convergently, which is SY-22's falsifier",
                shown(&one_first)
            )
        })?;
        Ok(RuleOutcome::Ran)
    }
}
