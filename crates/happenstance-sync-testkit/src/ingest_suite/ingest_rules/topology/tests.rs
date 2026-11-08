//! SY-10's two topologies on the reference store, and its falsification test.
//!
//! The marker: *falsified if a hub and a spoke can be shown to need the same
//! merge rule in a deployment where the hub holds every log and the spoke
//! holds one. Falsification test: implement the hub's rule on a spoke and show
//! the spoke's projections stay correct.* The projection is the one the seat
//! family exists for, who holds the seat, folded from the log without decoding
//! a payload: a claim stands unless a reversal answers it, and a reversal
//! answers the claim just below it in the log of the store that wrote it,
//! because a compensation lands with the claim it answers (SY-2).
//!
//! Under the scenarios' merge rule, *first to the hub, not first to decide*
//! (`references/scenarios/README.md:436`), the seat is the other spoke's: its
//! claim reached the hub first. The third test runs the hub's rule on the
//! spoke too. The spoke then reverses the other spoke's claim, the hub reverses
//! the spoke's, and once the two logs meet no claim to the seat stands. So the
//! spoke's projection is wrong and the clause is not falsified.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use happenstance_core::{EventId, EventStore, SequencePosition, SequencedEvent};
use happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};
use happenstance_testkit::block_on;

use super::{
    Directions, Edge, End, HUB_AND_SPOKE, MergeRule, OTHER_SPOKE, PEER_TO_PEER, SY_10, claims,
    hub_and_spoke, hub_and_spoke_facts, judge, peer_to_peer, peer_to_peer_facts, reversals,
};
use crate::MemoryIngestFixture;
use crate::ingest_suite::ingest_rules::{ingest, read_all};
use crate::violation::Violation;

/// The hub's rule in both directions: the spoke judges what the hub sends.
const HUB_RULE_ON_BOTH: Directions = Directions {
    push: MergeRule::Adjudicate,
    pull: MergeRule::Adjudicate,
};

async fn open() -> MemoryIngestFixture {
    MemoryIngestFixture::new()
}

/// The two ends exchange their whole logs, each way once, judging nothing.
async fn meet<S: EventStore + IngestStore>(edge: &Edge<S>) -> Result<(), Violation> {
    ship(&edge.near, &edge.far).await?;
    ship(&edge.far, &edge.near).await
}

async fn ship<S: EventStore + IngestStore>(from: &End<S>, to: &End<S>) -> Result<(), Violation> {
    let shipped: Vec<ReplicatedEvent> = read_all(&from.store, SY_10)
        .await?
        .into_iter()
        .map(|row| ReplicatedEvent::new(row.id, row.recorded_at, row.event))
        .collect();
    ingest(&to.store, &[IngestGroup::new(&shipped, &[])], SY_10).await?;
    Ok(())
}

/// The claim a reversal answers: the event just below it in its writer's log.
fn answered(own_log: &[SequencedEvent], reversal: EventId) -> Option<EventId> {
    let at = own_log.iter().position(|row| row.id == reversal)?;
    own_log.get(at.checked_sub(1)?).map(|row| row.id)
}

/// Who holds the seat once the two ends have met: every claim no reversal
/// answers. `own` holds each end's log as it was before the meeting, where
/// each reversal still sits above the claim its writer answered.
async fn standing<S: EventStore + IngestStore>(
    edge: &Edge<S>,
) -> Result<BTreeSet<EventId>, Violation> {
    let own = [
        (edge.near.id, read_all(&edge.near.store, SY_10).await?),
        (edge.far.id, read_all(&edge.far.store, SY_10).await?),
    ];
    meet(edge).await?;
    let merged = read_all(&edge.near.store, SY_10).await?;
    let answered: BTreeSet<Option<EventId>> = reversals(&merged)
        .iter()
        .map(|reversal| {
            own.iter()
                .find(|(writer, _)| *writer == reversal.id.store())
                .and_then(|(_, log)| answered(log, reversal.id))
        })
        .collect();
    Ok(claims(&merged)
        .iter()
        .map(|claim| claim.id)
        .filter(|id| !answered.contains(&Some(*id)))
        .collect())
}

#[test]
fn hub_and_spoke_leaves_the_seat_with_the_first_claim_to_the_hub() -> Result<(), Violation> {
    block_on(async {
        let edge = hub_and_spoke(&open, HUB_AND_SPOKE).await?;
        judge(&edge, &hub_and_spoke_facts()).await?;
        let first = EventId::new(OTHER_SPOKE, SequencePosition::FIRST);
        assert_eq!(standing(&edge).await?, BTreeSet::from([first]));
        Ok(())
    })
}

#[test]
fn peer_to_peer_leaves_both_claims_standing_for_the_adjudicator() -> Result<(), Violation> {
    block_on(async {
        let edge = peer_to_peer(&open, PEER_TO_PEER).await?;
        judge(&edge, &peer_to_peer_facts()).await?;
        let own: BTreeSet<EventId> = [edge.near.claim, edge.far.claim]
            .into_iter()
            .flatten()
            .collect();
        assert_eq!(own.len(), 2, "each peer appended its own claim");
        assert_eq!(standing(&edge).await?, own, "nobody judged the conflict");
        Ok(())
    })
}

#[test]
fn the_hubs_rule_on_the_spoke_leaves_no_claim_standing() -> Result<(), Violation> {
    block_on(async {
        // Each direction did what its configuration told it to, so the walk's
        // own counts hold; what SY-10 rejects is the outcome.
        let edge = hub_and_spoke(&open, HUB_RULE_ON_BOTH).await?;
        let verdict = judge(&edge, &hub_and_spoke_facts()).await;
        assert_eq!(
            verdict.map_err(|violation| violation.clause()),
            Err(SY_10),
            "the spoke wrote a reversal of its own"
        );
        let spoke = read_all(&edge.near.store, SY_10).await?;
        let authors: Vec<_> = reversals(&spoke).iter().map(|row| row.id.store()).collect();
        assert_eq!(
            authors,
            [edge.near.id, edge.far.id],
            "the spoke reversed the claim the hub sent, then took the hub's reversal of its own"
        );
        assert_eq!(
            standing(&edge).await?,
            BTreeSet::new(),
            "first to the hub is lost: the spoke's projection says nobody holds the seat"
        );
        Ok(())
    })
}
