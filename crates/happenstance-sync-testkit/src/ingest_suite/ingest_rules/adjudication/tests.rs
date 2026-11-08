//! SY-7's falsification test, run on the reference store under both
//! assignments, with the divergence the second one produces recorded as the
//! evidence SY-7's marker asks for.
//!
//! The marker: *two adjudicators for one family, disconnected from each
//! other, each ingesting the same conflicting pair; assert the two logs
//! converge after they meet.* Read as "the two logs hold the same events",
//! that assertion holds under whole-log replication (SY-4) whatever the
//! assignment, so it cannot be what falsifies the clause. What would is the
//! escape the marker names: the two compensations collapsing into one
//! resolution. They do not. Each carries the identity of the store that wrote
//! it (VT-5, SY-12), so they never dedupe, and because each adjudicator saw a
//! different claim arrive second, they reverse opposite claims: the merged log
//! answers one conflict twice and leaves the seat held by nobody. ADR-0027
//! predicted exactly this; the second test below observes it.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use happenstance_core::{EventId, EventStore, SequencedEvent};
use happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};
use happenstance_testkit::block_on;

use super::{Assignment, Conflict, SY_7, Side, judge, reversals, take_the_conflicting_pair};
use crate::MemoryIngestFixture;
use crate::ingest_suite::ingest_rules::{ingest, read_all};
use crate::violation::Violation;

/// The two sides exchange their whole logs, each way once: what a runner
/// does when two partitioned peers meet. Nothing new conflicts, so the runner
/// writes no compensation here.
async fn meet<S: EventStore + IngestStore>(conflict: &Conflict<S>) -> Result<(), Violation> {
    let [first, second] = &conflict.sides;
    ship(first, second).await?;
    ship(second, first).await
}

async fn ship<S: EventStore + IngestStore>(from: &Side<S>, to: &Side<S>) -> Result<(), Violation> {
    let shipped: Vec<ReplicatedEvent> = read_all(&from.store, SY_7)
        .await?
        .into_iter()
        .map(|row| ReplicatedEvent::new(row.id, row.recorded_at, row.event))
        .collect();
    ingest(&to.store, &[IngestGroup::new(&shipped, &[])], SY_7).await?;
    Ok(())
}

/// Each side's log after the meeting, adjudicator first.
async fn logs<S: EventStore + IngestStore>(
    conflict: &Conflict<S>,
) -> Result<[Vec<SequencedEvent>; 2], Violation> {
    let [first, second] = &conflict.sides;
    Ok([
        read_all(&first.store, SY_7).await?,
        read_all(&second.store, SY_7).await?,
    ])
}

fn identities(log: &[SequencedEvent]) -> BTreeSet<EventId> {
    log.iter().map(|row| row.id).collect()
}

/// The claim a reversal answers: the event just below it in the log of the
/// store that wrote it, because a compensation lands with the claim it
/// answers, in one group (SY-2). Read from the store, never decoded.
fn answered(own_log: &[SequencedEvent], reversal: EventId) -> Option<EventId> {
    let at = own_log.iter().position(|row| row.id == reversal)?;
    own_log.get(at.checked_sub(1)?).map(|row| row.id)
}

#[test]
fn one_adjudicator_leaves_one_reversal_on_every_side_after_they_meet() -> Result<(), Violation> {
    block_on(async {
        let conflict = take_the_conflicting_pair(
            &async || MemoryIngestFixture::new(),
            Assignment::Adjudicator,
        )
        .await?;
        judge(&conflict).await?;

        meet(&conflict).await?;

        let [adjudicator, other] = logs(&conflict).await?;
        assert_eq!(
            identities(&adjudicator),
            identities(&other),
            "the logs converge"
        );
        for log in [&adjudicator, &other] {
            let authors: Vec<_> = reversals(log).iter().map(|row| row.id.store()).collect();
            assert_eq!(
                authors,
                [conflict.sides[0].id],
                "one reversal, the adjudicator's"
            );
        }
        Ok(())
    })
}

#[test]
fn two_adjudicators_leave_two_reversals_of_opposite_claims_after_they_meet() -> Result<(), Violation>
{
    block_on(async {
        let conflict =
            take_the_conflicting_pair(&async || MemoryIngestFixture::new(), Assignment::EverySide)
                .await?;
        // Each side did what its own configuration told it to.
        judge(&conflict).await?;
        let [own_first, own_second] = logs(&conflict).await?;

        meet(&conflict).await?;

        let [first, second] = logs(&conflict).await?;
        // The marker's assertion, read literally: it holds.
        assert_eq!(identities(&first), identities(&second), "the logs converge");
        // The escape it names does not: two compensations, two identities.
        let held: Vec<EventId> = reversals(&first).iter().map(|row| row.id).collect();
        let authors: BTreeSet<_> = held.iter().map(|id| id.store()).collect();
        let [one, two] = conflict.sides.each_ref().map(|side| side.id);
        assert_eq!(
            authors,
            BTreeSet::from([one, two]),
            "every side's reversal is held"
        );
        assert_eq!(held.len(), 2, "one conflict, answered twice");
        // And they reverse opposite claims, so no claim to the seat stands.
        let answers: BTreeSet<Option<EventId>> = held
            .iter()
            .map(|id| {
                let own = if id.store() == one {
                    &own_first
                } else {
                    &own_second
                };
                answered(own, *id)
            })
            .collect();
        // The claims: everything the first side held that it did not write.
        let claims: BTreeSet<Option<EventId>> = own_first
            .iter()
            .filter(|row| row.id.store() != one)
            .map(|row| Some(row.id))
            .collect();
        assert_eq!(answers, claims, "each claim reversed by one adjudicator");
        Ok(())
    })
}
