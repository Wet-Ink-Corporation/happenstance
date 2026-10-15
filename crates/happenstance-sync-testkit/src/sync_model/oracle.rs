//! The model a [`Trace`] is judged against, and the three checks.
//!
//! # Why the model is not agreeing with the peer
//!
//! The model replays the trace's pushes into sets of its own: per node, the
//! identities it has been offered and the order they first arrived in. It
//! shares no subroutine with any peer (RS-60-4) — no `Watermark`, no
//! `PeerLimits::admits`, nothing from `MemorySyncPeer` — so a peer that agrees
//! with it is not agreeing with itself.
//!
//! Each check asserts one clause's proposition and nothing else, so one defect
//! fails one rule: a missing event does not trip the order check, and a
//! duplicated one does not trip the union check.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use happenstance_core::EventId;
use happenstance_sync::ReplicatedEvent;

use super::simulate::Trace;

/// Which proposition a rule asserts. A closed set (API-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Check {
    /// Every node holds the union of what was written, byte for byte.
    Union,
    /// Every push is acknowledged exactly, and no node holds anything twice.
    ExactlyOnce,
    /// Every node forwards in the order things arrived.
    ArrivalOrder,
}

impl Check {
    /// The clause a failure that is not a judgement — a peer refusing, the
    /// peers never settling — is reported under.
    pub(crate) const fn clause(self) -> &'static str {
        match self {
            Self::Union => "SY-24",
            Self::ExactlyOnce => "SY-11",
            Self::ArrivalOrder => "SY-25",
        }
    }
}

/// A check that did not hold: the clause, and what the peers did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Failure {
    pub(crate) clause: &'static str,
    pub(crate) message: String,
}

impl Failure {
    pub(crate) fn new(clause: &'static str, message: impl Into<String>) -> Self {
        Self {
            clause,
            message: message.into(),
        }
    }
}

/// What the model believes one node was offered.
#[derive(Debug, Default)]
struct Node {
    held: BTreeSet<EventId>,
    /// First arrivals, in order.
    arrival: Vec<EventId>,
    /// Whether any push to this node was acknowledged with fewer appended
    /// events than it had never been offered before.
    under_acked: bool,
}

struct Model {
    nodes: Vec<Node>,
    /// The first push whose acknowledgement disagreed with the model.
    first_miscount: Option<String>,
}

fn model(trace: &Trace) -> Result<Model, Failure> {
    let mut nodes: Vec<Node> = (0..trace.peers).map(|_| Node::default()).collect();
    let mut first_miscount = None;
    for (index, delivery) in trace.deliveries.iter().enumerate() {
        let node = nodes.get_mut(delivery.to).ok_or_else(|| {
            Failure::new(
                "SY-24",
                format!(
                    "the trace pushed to node {}, which does not exist",
                    delivery.to
                ),
            )
        })?;
        let (mut fresh, mut stale) = (0_u64, 0_u64);
        for id in &delivery.ids {
            if node.held.insert(*id) {
                node.arrival.push(*id);
                fresh += 1;
            } else {
                stale += 1;
            }
        }
        let ack = &delivery.ack;
        node.under_acked |= ack.appended < fresh;
        if first_miscount.is_none() && (ack.appended, ack.skipped) != (fresh, stale) {
            first_miscount = Some(format!(
                "push {index} to node {}: acknowledged ({} appended, {} skipped) for {} events \
                 offered, of which it already held {stale}",
                delivery.to,
                ack.appended,
                ack.skipped,
                delivery.ids.len(),
            ));
        }
    }
    Ok(Model {
        nodes,
        first_miscount,
    })
}

/// Judges `trace` by `check`.
///
/// # Errors
///
/// The [`Failure`] the check found, under the clause it breaks.
pub(crate) fn judge(check: Check, trace: &Trace) -> Result<(), Failure> {
    let model = model(trace)?;
    match check {
        Check::Union => union(trace, &model),
        Check::ExactlyOnce => exactly_once(trace, model),
        Check::ArrivalOrder => arrival_order(trace, &model),
    }
}

/// How a missing event is classified, most specific first. The derived order
/// is the reporting priority, so the clause named does not depend on which node
/// was scanned first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Miss {
    /// Accepted, acknowledged honestly, and never handed back: a holding area.
    Quarantined,
    /// Offered, and acknowledged as already held when it never was.
    CalledHeld,
    /// Never offered to this node at all, or handed back with other bytes.
    NotForwarded,
}

impl Miss {
    const fn clause(self) -> &'static str {
        match self {
            Self::Quarantined => "SY-4",
            Self::CalledHeld => "SY-1",
            Self::NotForwarded => "SY-24",
        }
    }
}

fn union(trace: &Trace, model: &Model) -> Result<(), Failure> {
    let mut worst: Option<(Miss, String)> = None;
    for (index, node) in model.nodes.iter().enumerate() {
        let log: BTreeMap<EventId, &ReplicatedEvent> = trace
            .logs
            .get(index)
            .into_iter()
            .flatten()
            .rev()
            .map(|event| (event.id, event))
            .collect();
        for (id, written) in &trace.written {
            let miss = match log.get(id) {
                Some(held) if *held == written => continue,
                Some(_) => (
                    Miss::NotForwarded,
                    format!("node {index} hands back {id} with bytes other than those written"),
                ),
                None if !node.held.contains(id) => (
                    Miss::NotForwarded,
                    format!("node {index} never received {id}: nothing forwarded it there"),
                ),
                None if node.under_acked => (
                    Miss::CalledHeld,
                    format!(
                        "node {index} was offered {id} and does not hold it; it acknowledged \
                         a push as holding events it never held"
                    ),
                ),
                None => (
                    Miss::Quarantined,
                    format!("node {index} acknowledged {id} as appended and never hands it back"),
                ),
            };
            if worst.as_ref().is_none_or(|(seen, _)| miss.0 < *seen) {
                worst = Some(miss);
            }
        }
    }
    worst.map_or(Ok(()), |(miss, message)| {
        Err(Failure::new(miss.clause(), message))
    })
}

/// Takes the model by value: the miscount message moves into the failure.
fn exactly_once(trace: &Trace, model: Model) -> Result<(), Failure> {
    if let Some(message) = model.first_miscount {
        return Err(Failure::new("SY-11", message));
    }
    for (index, log) in trace.logs.iter().enumerate() {
        let mut seen = BTreeSet::new();
        if let Some(twice) = log.iter().find(|event| !seen.insert(event.id)) {
            return Err(Failure::new(
                "SY-11",
                format!("node {index} holds {} twice", twice.id),
            ));
        }
    }
    Ok(())
}

fn arrival_order(trace: &Trace, model: &Model) -> Result<(), Failure> {
    for (index, node) in model.nodes.iter().enumerate() {
        let log = trace.logs.get(index).map_or(&[][..], Vec::as_slice);
        // Both sides cut to what both hold, and the log to first occurrences:
        // what is missing or doubled is another check's finding, not this one's.
        let mut first = BTreeSet::new();
        let forwarded: Vec<EventId> = log
            .iter()
            .map(|event| event.id)
            .filter(|id| node.held.contains(id) && first.insert(*id))
            .collect();
        let arrived: Vec<EventId> = node
            .arrival
            .iter()
            .copied()
            .filter(|id| first.contains(id))
            .collect();
        if let Some((at, (got, want))) = forwarded
            .iter()
            .zip(&arrived)
            .enumerate()
            .find(|(_, (got, want))| got != want)
        {
            return Err(Failure::new(
                "SY-25",
                format!(
                    "node {index} hands back {got} at place {at}, where {want} arrived; it \
                     arrived in the order {arrived:?} and is forwarded as {forwarded:?}"
                ),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeMap;
    use alloc::vec;
    use alloc::vec::Vec;

    use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
    use happenstance_sync::{Ack, ReplicatedEvent};

    use super::{Check, judge};
    use crate::sync_model::simulate::{Delivery, Trace};

    fn event(origin: u8, nth: u64) -> ReplicatedEvent {
        let position = SequencePosition::new(nth).expect("positions start at one");
        let id = EventId::new(StoreId::from_bytes([origin; 16]), position);
        ReplicatedEvent::new(
            id,
            RecordedAt::from_millis(1_800_000_000_000),
            Event::new("Written", id.to_string().into_bytes()).expect("a valid type"),
        )
    }

    fn ack(appended: u64, skipped: u64) -> Ack {
        let mut ack = Ack::default();
        ack.appended = appended;
        ack.skipped = skipped;
        ack
    }

    fn push(to: usize, events: &[&ReplicatedEvent], acked: (u64, u64)) -> Delivery {
        Delivery {
            to,
            ids: events.iter().map(|event| event.id).collect(),
            ack: ack(acked.0, acked.1),
        }
    }

    /// Two nodes: A written at 0 and B at 1, each forwarded to the other, and
    /// each node's log in the order things arrived.
    fn consistent() -> (Trace, ReplicatedEvent, ReplicatedEvent) {
        let (a, b) = (event(1, 1), event(2, 1));
        let trace = Trace {
            peers: 2,
            deliveries: vec![
                push(0, &[&a], (1, 0)),
                push(1, &[&b], (1, 0)),
                push(1, &[&a], (1, 0)),
                push(0, &[&b, &a], (1, 1)),
            ],
            logs: vec![vec![a.clone(), b.clone()], vec![b.clone(), a.clone()]],
            written: BTreeMap::from([(a.id, a.clone()), (b.id, b.clone())]),
        };
        (trace, a, b)
    }

    fn clause(check: Check, trace: &Trace) -> Option<&'static str> {
        judge(check, trace).err().map(|failure| failure.clause)
    }

    #[test]
    fn a_consistent_trace_passes_every_check() {
        let (trace, ..) = consistent();
        for check in [Check::Union, Check::ExactlyOnce, Check::ArrivalOrder] {
            assert_eq!(judge(check, &trace), Ok(()), "{check:?}");
        }
    }

    #[test]
    fn a_missing_event_never_delivered_is_sy_24() {
        let (mut trace, a, _) = consistent();
        let forwarded = trace.deliveries.remove(2);
        assert_eq!((forwarded.to, forwarded.ids), (1, vec![a.id]));
        trace.logs[1].retain(|event| event.id != a.id);
        assert_eq!(clause(Check::Union, &trace), Some("SY-24"));
    }

    #[test]
    fn a_missing_event_acked_as_appended_is_sy_4() {
        let (mut trace, a, _) = consistent();
        trace.logs[1].retain(|event| event.id != a.id);
        assert_eq!(clause(Check::Union, &trace), Some("SY-4"));
    }

    #[test]
    fn a_missing_event_under_an_undercounted_ack_is_sy_1() {
        let (mut trace, a, _) = consistent();
        trace.logs[1].retain(|event| event.id != a.id);
        trace.deliveries[2].ack = ack(0, 1);
        assert_eq!(clause(Check::Union, &trace), Some("SY-1"));
    }

    #[test]
    fn an_ack_miscount_is_sy_11() {
        let (mut trace, ..) = consistent();
        trace.deliveries[3].ack = ack(2, 0);
        assert_eq!(clause(Check::ExactlyOnce, &trace), Some("SY-11"));
    }

    #[test]
    fn a_duplicate_in_a_log_is_sy_11_and_nothing_else() {
        let (mut trace, a, _) = consistent();
        trace.logs[0].push(a);
        assert_eq!(clause(Check::ExactlyOnce, &trace), Some("SY-11"));
        assert_eq!(clause(Check::ArrivalOrder, &trace), None, "orthogonal");
        assert_eq!(clause(Check::Union, &trace), None, "orthogonal");
    }

    #[test]
    fn a_swapped_pair_is_sy_25() {
        let (mut trace, ..) = consistent();
        trace.logs[1].reverse();
        assert_eq!(clause(Check::ArrivalOrder, &trace), Some("SY-25"));
        assert_eq!(clause(Check::Union, &trace), None, "orthogonal");
        assert_eq!(clause(Check::ExactlyOnce, &trace), None, "orthogonal");
    }

    #[test]
    fn a_dropped_event_does_not_trip_arrival_order() {
        let (mut trace, a, _) = consistent();
        trace.logs[1].retain(|event| event.id != a.id);
        assert_eq!(clause(Check::ArrivalOrder, &trace), None);
        assert_eq!(clause(Check::ExactlyOnce, &trace), None);
    }

    #[test]
    fn other_bytes_under_the_same_identity_are_sy_24() {
        let (mut trace, a, _) = consistent();
        let mut forged = a.clone();
        forged.recorded_at = RecordedAt::from_millis(1);
        let logs: Vec<ReplicatedEvent> = trace.logs[1]
            .iter()
            .map(|event| {
                if event.id == a.id {
                    forged.clone()
                } else {
                    event.clone()
                }
            })
            .collect();
        trace.logs[1] = logs;
        assert_eq!(clause(Check::Union, &trace), Some("SY-24"));
    }
}
