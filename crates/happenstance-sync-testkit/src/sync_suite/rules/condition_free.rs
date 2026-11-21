//! SY-33's probes, seed and comparison, kept out of `sync_suite.rs` so the
//! rule there reads as what it checks.
//!
//! The rule asks every refusal one question: does its answer change when the
//! receiver's log does? It answers by asking twice, on one receiver, with the
//! rule's own seed pushed in between. The seed is chosen to be the state a
//! condition-level refusal keys on: a claim on the slot the probe's guard says
//! nobody holds, beside events nothing refers to.

use alloc::format;
use alloc::vec;
use alloc::vec::Vec;

use happenstance_core::{
    AppendCondition, Event, EventId, EventType, Query, QueryItem, RecordedAt, SequencePosition,
    StoreId, Tag, Tags,
};
use happenstance_sync::{Classify, EventGroup, FailureClass, PushBatch, ReplicatedEvent, SyncPeer};

use crate::violation::{Violation, ensure};
use crate::{Refusal, SyncFixture};

const SY_33: &str = "SY-33";

/// The origin whose guarded claim is the probe, pushed first and re-sent.
const PROBE_ORIGIN: StoreId = StoreId::from_bytes([0xC1; 16]);
/// The origin of the seed: a rival claim on the slot, and unrelated events.
const SEED_ORIGIN: StoreId = StoreId::from_bytes([0xC2; 16]);
/// The origin of the late probe: the probe's shape, under a fresh identity,
/// arriving after the seed.
const LATE_ORIGIN: StoreId = StoreId::from_bytes([0xC3; 16]);

const SLOT_CLAIMED: EventType = EventType::from_static("SlotClaimed");
const NOTE_TAKEN: EventType = EventType::from_static("NoteTaken");

/// How many unrelated events the seed carries beside its rival claim: enough
/// that "holds something" and "holds one thing" are different states.
const UNRELATED: u64 = 3;

/// How a peer answered one push: SY-33's whole subject, with the ack left out
/// because SY-1's and SY-31's rules judge what an ack says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Answer {
    Accepted,
    Refused(FailureClass),
}

impl Answer {
    /// The answer `outcome` is. The class is read once: whether it is a
    /// function of the error is SY-36's rule's question, not this one's.
    fn of<T, E: Classify>(outcome: &Result<T, E>) -> Self {
        match outcome {
            Ok(_) => Self::Accepted,
            Err(error) => Self::Refused(error.class()),
        }
    }
}

/// Each refusal the fixture provoked, by what provoked it, with its answer.
fn answers<E: Classify>(refusals: &[Refusal<E>]) -> Vec<(&'static str, Answer)> {
    refusals
        .iter()
        .map(|refusal| (refusal.provoked_by, Answer::of(&refusal.outcome)))
        .collect()
}

/// SY-33's check on `peer`, the fixture's own handle: provoke, probe, seed,
/// probe again, provoke again, and require every answer to have held.
pub(super) async fn never_condition_derived<F: SyncFixture>(
    fixture: &F,
    peer: &F::Peer,
) -> Result<(), Violation> {
    let before = answers(&fixture.refusals(peer).await?);

    let probe = probe_batch(PROBE_ORIGIN)?;
    let first = Answer::of(&peer.push(&probe).await);

    peer.push(&seed()?).await.map_err(|error| {
        Violation::new(
            SY_33,
            format!(
                "the peer refused the rule's seed, {} foreign events in single-event groups \
                 with no guard ({error}, class {:?}); the rule cannot grow the receiver's log \
                 to ask whether a refusal varies with it",
                UNRELATED + 1,
                error.class()
            ),
        )
    })?;

    let resent = Answer::of(&peer.push(&probe).await);
    ensure(resent == first, SY_33, || {
        format!(
            "the probe, a guarded claim from {PROBE_ORIGIN}, was answered {first:?} when first \
             sent and {resent:?} when re-sent after the seed; a re-delivery is a skip, and a \
             refusal keyed on what the receiver holds can never succeed on a retry"
        )
    })?;
    let late = Answer::of(&peer.push(&probe_batch(LATE_ORIGIN)?).await);
    ensure(late == first, SY_33, || {
        format!(
            "a claim under the probe's guard was answered {first:?} while the receiver held \
             nothing the guard names and {late:?} from {LATE_ORIGIN} once it held a rival \
             claim; the guard is evidence, and a refusal derived from it is a condition-level \
             refusal"
        )
    })?;

    let after = answers(&fixture.refusals(peer).await?);
    compare(&before, &after)
}

/// The fixture's provocations, answered the same before the seed and after it.
fn compare(
    before: &[(&'static str, Answer)],
    after: &[(&'static str, Answer)],
) -> Result<(), Violation> {
    ensure(before.len() == after.len(), SY_33, || {
        format!(
            "the fixture provoked {} refusals on the fresh receiver and {} once it held the \
             rule's seed; the provocations must not depend on the receiver's log",
            before.len(),
            after.len()
        )
    })?;
    for ((provoked, was), (again, now)) in before.iter().zip(after) {
        ensure(provoked == again, SY_33, || {
            format!(
                "the fixture's provocations came back in another order: `{provoked}` on the \
                 fresh receiver, `{again}` once it held the rule's seed"
            )
        })?;
        ensure(was == now, SY_33, || {
            format!(
                "{provoked} was answered {was:?} by the fresh receiver and {now:?} once it held \
                 the rule's seed; a refusal whose truth value depends on which events the \
                 receiver holds is a condition-level refusal"
            )
        })?;
    }
    Ok(())
}

/// A claim on the slot from `origin`, under the guard "nobody has claimed the
/// slot": position-free, so byte-identical wherever it was written, and never
/// carrying `after`, which SY-6 lets a peer refuse whatever its log holds.
fn probe_batch(origin: StoreId) -> Result<PushBatch, Violation> {
    let item = QueryItem::new([SLOT_CLAIMED], slot()).map_err(|error| {
        Violation::new(SY_33, format!("the rule's own query is invalid: {error}"))
    })?;
    let guard = AppendCondition::new(Query::from_item(item));
    let claim = event(origin, 1, SLOT_CLAIMED, slot())?;
    Ok(PushBatch::new(vec![EventGroup::new(
        Some(guard),
        vec![claim],
    )]))
}

/// The seed: a rival claim on the slot, which violates the probe's guard, and
/// [`UNRELATED`] notes that nothing names.
fn seed() -> Result<PushBatch, Violation> {
    let rival = event(SEED_ORIGIN, 1, SLOT_CLAIMED, slot())?;
    let mut groups = vec![EventGroup::new(None, vec![rival])];
    for position in 2..=UNRELATED + 1 {
        let note = event(SEED_ORIGIN, position, NOTE_TAKEN, Tags::empty())?;
        groups.push(EventGroup::new(None, vec![note]));
    }
    Ok(PushBatch::new(groups))
}

fn slot() -> Tags {
    [Tag::from_static("slot:s1")].into_iter().collect()
}

/// `origin`'s event at its own `position`, its payload naming both so "the
/// same event" means byte for byte (SY-35).
fn event(
    origin: StoreId,
    position: u64,
    event_type: EventType,
    tags: Tags,
) -> Result<ReplicatedEvent, Violation> {
    let at = SequencePosition::new(position)
        .ok_or_else(|| Violation::new(SY_33, "the rule's origin position is zero"))?;
    let event = Event::new(event_type, format!("sy33 {origin} {position}").into_bytes())
        .map_err(|error| {
            Violation::new(SY_33, format!("the rule's own event is invalid: {error}"))
        })?
        .with_tags(tags);
    Ok(ReplicatedEvent::new(
        EventId::new(origin, at),
        RecordedAt::from_millis(1_800_000_000_000),
        event,
    ))
}

#[cfg(test)]
mod tests {
    use happenstance_sync::FailureClass;

    use super::{Answer, compare};

    const OVERSIZED: &str = "a push one byte over max_batch_bytes";

    #[test]
    fn the_same_answers_in_the_same_order_pass() {
        let answers = [(OVERSIZED, Answer::Refused(FailureClass::Split))];
        assert_eq!(compare(&answers, &answers), Ok(()));
    }

    #[test]
    fn a_class_that_changed_with_the_log_is_sy_33s() {
        let before = [(OVERSIZED, Answer::Refused(FailureClass::Split))];
        let after = [(OVERSIZED, Answer::Refused(FailureClass::Stop))];
        assert_eq!(
            compare(&before, &after).map_err(|v| v.clause()),
            Err("SY-33")
        );
    }

    #[test]
    fn a_refusal_that_appeared_with_the_log_is_sy_33s() {
        let before = [(OVERSIZED, Answer::Accepted)];
        let after = [(OVERSIZED, Answer::Refused(FailureClass::Stop))];
        assert_eq!(
            compare(&before, &after).map_err(|v| v.clause()),
            Err("SY-33")
        );
    }

    #[test]
    fn a_provocation_that_vanished_with_the_log_is_sy_33s() {
        let before = [(OVERSIZED, Answer::Refused(FailureClass::Split))];
        assert_eq!(compare(&before, &[]).map_err(|v| v.clause()), Err("SY-33"));
    }
}
