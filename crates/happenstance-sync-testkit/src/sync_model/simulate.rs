//! The simulator: one isolated peer per node, a runner played by the harness,
//! and a [`Trace`] of everything pushed and everything held at the end.
//!
//! The harness is the runner here, and the smallest one that can express the
//! faults: per edge, two lanes, each a resume token into its source peer and at
//! most one page held in flight. It is not `happenstance-sync`'s runner, which
//! does not exist yet (#244), and nothing in it is offered as one.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{Ack, Classify, EventGroup, PushBatch, ReplicatedEvent, SyncPeer};

use super::scenario::{Direction, Fault, Guard, SEAT_CLAIMED, Scenario, Step, seat, seat_guard};
use crate::SyncFixture;

/// The most sweeps the heal-and-quiesce phase runs before it decides the peers
/// never settle. A sweep drains every lane, so a correct peer set settles in
/// about its diameter, five at most; the bound turns a peer that never says it
/// has caught up into a failure rather than a hang (H-13).
const MAX_SWEEPS: usize = 16;

/// The most pulls one lane makes in one drain, and the most one observation
/// makes. Far above what 96 events need under the minimum limits.
const MAX_PULLS: usize = 64;

/// One push the harness made, and what the peer said about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Delivery {
    /// The node pushed to.
    pub(crate) to: usize,
    /// The identities offered, in batch order.
    pub(crate) ids: Vec<EventId>,
    /// The peer's acknowledgement.
    pub(crate) ack: Ack,
}

/// What one scenario did, for the oracle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Trace {
    /// How many nodes there were.
    pub(crate) peers: usize,
    /// Every push, writes included, in the order the harness made them.
    pub(crate) deliveries: Vec<Delivery>,
    /// Each node's whole log at quiescence, pulled from the start.
    pub(crate) logs: Vec<Vec<ReplicatedEvent>>,
    /// Everything written anywhere, one copy each: the union every node must
    /// end up holding.
    pub(crate) written: BTreeMap<EventId, ReplicatedEvent>,
}

/// Whether an edge carries exchanges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Link {
    Up,
    Partitioned,
}

/// One direction of an edge: where its pulls resume, and the page it holds.
#[derive(Debug)]
struct Lane<R> {
    resume: Option<R>,
    in_flight: Option<PushBatch>,
}

impl<R> Lane<R> {
    const fn new() -> Self {
        Self {
            resume: None,
            in_flight: None,
        }
    }
}

#[derive(Debug)]
struct Edge<R> {
    a: usize,
    b: usize,
    link: Link,
    /// `a` to `b`.
    forward: Lane<R>,
    /// `b` to `a`.
    backward: Lane<R>,
}

/// A harness-minted origin identity, and the next position it hands out.
#[derive(Debug)]
struct Origin {
    store: StoreId,
    next: SequencePosition,
}

/// The peers and the record of every push made to them.
#[derive(Debug)]
struct Wire<P> {
    peers: Vec<P>,
    deliveries: Vec<Delivery>,
}

impl<P: SyncPeer> Wire<P> {
    fn peer(&self, node: usize) -> Result<&P, String> {
        self.peers
            .get(node)
            .ok_or_else(|| format!("the simulator named node {node}, which does not exist"))
    }

    async fn deliver(&mut self, to: usize, batch: &PushBatch) -> Result<(), String> {
        let ack = self.peer(to)?.push(batch).await.map_err(|error| {
            format!(
                "node {to} refused a push of {} events: {error} (class {:?})",
                batch.events().count(),
                error.class()
            )
        })?;
        self.deliveries.push(Delivery {
            to,
            ids: batch.events().map(|event| event.id).collect(),
            ack,
        });
        Ok(())
    }

    async fn pull(
        &self,
        from: usize,
        resume: Option<&P::Resume>,
    ) -> Result<(PushBatch, P::Resume), String> {
        let pulled = self
            .peer(from)?
            .pull(resume)
            .await
            .map_err(|error| format!("node {from} refused a pull: {error}"))?;
        Ok((pulled.batch, pulled.resume))
    }

    /// Everything `node` holds, from the start of its log.
    async fn observe(&self, node: usize) -> Result<Vec<ReplicatedEvent>, String> {
        let mut events = Vec::new();
        let mut resume = None;
        for _ in 0..MAX_PULLS {
            let (batch, next) = self.pull(node, resume.as_ref()).await?;
            if batch.groups.is_empty() {
                return Ok(events);
            }
            events.extend(batch.groups.into_iter().flat_map(|group| group.events));
            resume = Some(next);
        }
        Err(format!(
            "node {node} was still handing back groups after {MAX_PULLS} pulls"
        ))
    }
}

struct Network<P: SyncPeer> {
    wire: Wire<P>,
    origins: Vec<Origin>,
    edges: Vec<Edge<P::Resume>>,
    written: BTreeMap<EventId, ReplicatedEvent>,
}

impl<P: SyncPeer> Network<P> {
    async fn step(&mut self, step: Step) -> Result<(), String> {
        match step {
            Step::Write { at, events, guard } => {
                let node = at.of(self.node_count()?);
                self.write(node, events.get(), guard).await
            }
            Step::Exchange {
                edge,
                toward,
                fault,
            } => {
                let index = edge.of(self.edge_count()?);
                self.exchange(index, toward, fault).await.map(drop)
            }
            Step::Partition { edge } => self.link(edge.of(self.edge_count()?), Link::Partitioned),
            Step::Heal { edge } => self.link(edge.of(self.edge_count()?), Link::Up),
        }
    }

    fn node_count(&self) -> Result<NonZeroUsize, String> {
        NonZeroUsize::new(self.wire.peers.len()).ok_or_else(|| String::from("no nodes"))
    }

    fn edge_count(&self) -> Result<NonZeroUsize, String> {
        NonZeroUsize::new(self.edges.len()).ok_or_else(|| String::from("no edges"))
    }

    fn link(&mut self, index: usize, link: Link) -> Result<(), String> {
        let edge = self
            .edges
            .get_mut(index)
            .ok_or_else(|| format!("no edge {index}"))?;
        edge.link = link;
        Ok(())
    }

    /// One group of `count` fresh events, minted at `node` and pushed to it.
    /// The events are then moved into the union, not copied (H-08).
    async fn write(&mut self, node: usize, count: usize, guard: Guard) -> Result<(), String> {
        let origin = self
            .origins
            .get_mut(node)
            .ok_or_else(|| format!("no origin for node {node}"))?;
        let mut events = Vec::with_capacity(count);
        for _ in 0..count {
            let id = EventId::new(origin.store, origin.next);
            origin.next = origin
                .next
                .next()
                .ok_or_else(|| format!("node {node} ran out of positions"))?;
            // The payload is the identity's own text: distinct per event, so
            // "byte for byte" means something, and never parsed (SY-35).
            let event = Event::new(SEAT_CLAIMED, id.to_string().into_bytes())
                .map_err(|error| format!("the simulator's own event is invalid: {error}"))?
                .with_tags(seat());
            events.push(ReplicatedEvent::new(
                id,
                RecordedAt::from_millis(1_800_000_000_000),
                event,
            ));
        }
        let guard = match guard {
            Guard::Unguarded => None,
            Guard::SeatClaim => Some(seat_guard()?),
        };
        let batch = PushBatch::new(vec![EventGroup::new(guard, events)]);
        self.wire.deliver(node, &batch).await?;
        self.written.extend(
            batch
                .groups
                .into_iter()
                .flat_map(|group| group.events)
                .map(|event| (event.id, event)),
        );
        Ok(())
    }

    /// One pull along one lane, pushed on with `fault`. Answers whether the
    /// pull came back empty, which is what quiescence is made of.
    async fn exchange(
        &mut self,
        index: usize,
        toward: Direction,
        fault: Fault,
    ) -> Result<bool, String> {
        let edge = self
            .edges
            .get_mut(index)
            .ok_or_else(|| format!("no edge {index}"))?;
        if edge.link == Link::Partitioned {
            return Ok(true);
        }
        let (from, to, lane) = match toward {
            Direction::Forward => (edge.a, edge.b, &mut edge.forward),
            Direction::Backward => (edge.b, edge.a, &mut edge.backward),
        };
        let (page, resume) = self.wire.pull(from, lane.resume.as_ref()).await?;
        let empty = page.groups.is_empty();
        match fault {
            Fault::DropAck => {
                if !empty {
                    self.wire.deliver(to, &page).await?;
                }
            }
            Fault::Delay if lane.in_flight.is_none() && !empty => {
                lane.in_flight = Some(page);
                lane.resume = Some(resume);
            }
            Fault::None | Fault::Duplicate | Fault::Delay => {
                if !empty {
                    self.wire.deliver(to, &page).await?;
                    if fault == Fault::Duplicate {
                        self.wire.deliver(to, &page).await?;
                    }
                }
                lane.resume = Some(resume);
                if let Some(held) = lane.in_flight.take() {
                    self.wire.deliver(to, &held).await?;
                }
            }
        }
        Ok(empty)
    }

    /// Heals every link, lands every page in flight, then sweeps until a sweep
    /// in which every pull came back empty.
    async fn quiesce(&mut self) -> Result<(), String> {
        for edge in &mut self.edges {
            edge.link = Link::Up;
            for (to, lane) in [(edge.b, &mut edge.forward), (edge.a, &mut edge.backward)] {
                if let Some(held) = lane.in_flight.take() {
                    self.wire.deliver(to, &held).await?;
                }
            }
        }
        for _ in 0..MAX_SWEEPS {
            let mut settled = true;
            for index in 0..self.edges.len() {
                for toward in [Direction::Forward, Direction::Backward] {
                    settled &= self.drain(index, toward).await?;
                }
            }
            if settled {
                return Ok(());
            }
        }
        Err(format!("no quiescence after {MAX_SWEEPS} sweeps"))
    }

    /// Exchanges along one lane until it comes back empty. Answers whether the
    /// first pull already did.
    async fn drain(&mut self, index: usize, toward: Direction) -> Result<bool, String> {
        let first = self.exchange(index, toward, Fault::None).await?;
        if first {
            return Ok(true);
        }
        for _ in 0..MAX_PULLS {
            if self.exchange(index, toward, Fault::None).await? {
                return Ok(false);
            }
        }
        Err(format!(
            "edge {index} {toward:?} was still handing back groups after {MAX_PULLS} pulls"
        ))
    }
}

/// Runs `scenario` against one fresh fixture per node.
///
/// `open` is borrowed because the shrinker calls this many times; the trace is
/// returned owned.
///
/// # Errors
///
/// A message if a peer refuses a push or a pull, if the peers never settle, or
/// if the scenario names something the network does not have.
pub(crate) async fn simulate<F, O>(open: &O, scenario: &Scenario) -> Result<Trace, String>
where
    F: SyncFixture,
    O: AsyncFn() -> F,
{
    let topology = scenario.topology();
    let peers = topology.peers().get();
    // Kept alive to the end: a handle may depend on its fixture's backing.
    let mut fixtures = Vec::with_capacity(peers);
    let mut handles = Vec::with_capacity(peers);
    let mut origins = Vec::with_capacity(peers);
    for node in 0..peers {
        let fixture = open().await;
        handles.push(fixture.connect().await);
        fixtures.push(fixture);
        let tag = u8::try_from(node)
            .map_err(|_| format!("node {node} does not fit an origin byte"))?
            .checked_add(0x40)
            .ok_or_else(|| format!("node {node} has no origin byte"))?;
        origins.push(Origin {
            store: StoreId::from_bytes([tag; 16]),
            next: SequencePosition::FIRST,
        });
    }

    let mut network = Network {
        wire: Wire {
            peers: handles,
            deliveries: Vec::new(),
        },
        origins,
        edges: topology
            .edges()
            .iter()
            .map(|&(a, b)| Edge {
                a,
                b,
                link: Link::Up,
                forward: Lane::new(),
                backward: Lane::new(),
            })
            .collect(),
        written: BTreeMap::new(),
    };

    for (index, step) in scenario.steps().iter().enumerate() {
        network
            .step(*step)
            .await
            .map_err(|failure| format!("at step {index}, {step:?}: {failure}"))?;
    }
    network
        .quiesce()
        .await
        .map_err(|failure| format!("healing after the last step: {failure}"))?;

    let mut logs = Vec::with_capacity(peers);
    for node in 0..peers {
        logs.push(network.wire.observe(node).await?);
    }
    drop(fixtures);
    Ok(Trace {
        peers,
        deliveries: network.wire.deliveries,
        logs,
        written: network.written,
    })
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use happenstance_core::EventId;
    use happenstance_testkit::block_on;

    use super::{Trace, simulate};
    use crate::MemorySyncFixture;
    use crate::sync_model::scenario::{
        Direction, Fault, Guard, Pick, Scenario, Step, WriteSize, regressions,
    };

    fn run(name: &str) -> Trace {
        let regression = regressions()
            .expect("the regressions are valid")
            .into_iter()
            .find(|regression| regression.name == name)
            .expect("the regression exists");
        block_on(simulate(
            &async || MemorySyncFixture::new(),
            &regression.scenario,
        ))
        .expect("the reference peer runs every regression")
    }

    fn acks(trace: &Trace) -> Vec<(u64, u64)> {
        trace
            .deliveries
            .iter()
            .map(|delivery| (delivery.ack.appended, delivery.ack.skipped))
            .collect()
    }

    fn ids(trace: &Trace, node: usize) -> Vec<EventId> {
        trace.logs[node].iter().map(|event| event.id).collect()
    }

    #[test]
    fn lost_ack_repush_and_echo_counts_its_skips() {
        let trace = run("lost_ack_repush_and_echo");
        // The scenario's own four pushes; healing then carries the group on to Q.
        assert_eq!(acks(&trace)[..4], [(2, 0), (2, 0), (0, 2), (0, 2)]);
    }

    /// SF-03: a page half of which its receiver already holds.
    #[test]
    fn sf03_partial_overlap_applies_b_once() {
        let trace = run("sf03_partial_overlap");
        let steps = [
            (1, 0),
            (1, 0),
            (0, 1),
            (1, 0),
            (1, 0),
            (1, 0),
            (1, 0),
            (1, 1),
            (0, 2),
        ];
        assert_eq!(acks(&trace)[..steps.len()], steps);
        let at_p = ids(&trace, 0);
        assert_eq!(at_p.len(), 3, "P holds A, B and C once each: {at_p:?}");
        let written: Vec<EventId> = trace.written.keys().copied().collect();
        for id in written {
            assert!(at_p.contains(&id), "P is missing {id}");
        }
    }

    #[test]
    fn a_delayed_page_lands_after_a_later_one() {
        let trace = run("delayed_page_lands_after_a_later_one");
        assert_eq!(acks(&trace)[..4], [(1, 0), (1, 0), (1, 0), (1, 0)]);
        let (a, b) = (trace.deliveries[0].ids[0], trace.deliveries[1].ids[0]);
        assert_eq!(trace.deliveries[2].ids, [b], "the later page lands first");
        assert_eq!(trace.deliveries[3].ids, [a], "and the held one after it");
        assert_eq!(ids(&trace, 1), [b, a], "H appends in arrival order");
    }

    #[test]
    fn partitions_and_heal_still_quiesce() {
        let star = [(0, 1), (0, 2), (0, 3)];
        let mut steps = Vec::new();
        for edge in 0..3 {
            steps.push(Step::Partition {
                edge: Pick::new(edge),
            });
        }
        for node in 0..4 {
            steps.push(Step::Write {
                at: Pick::new(node),
                events: WriteSize::TWO,
                guard: Guard::SeatClaim,
            });
        }
        for edge in 0..3 {
            steps.push(Step::Exchange {
                edge: Pick::new(edge),
                toward: Direction::Backward,
                fault: Fault::Delay,
            });
        }
        let scenario = Scenario::fixed(4, &star, steps).expect("a star is a valid topology");
        let trace = block_on(simulate(&async || MemorySyncFixture::new(), &scenario))
            .expect("the partitions heal and the peers settle");
        assert_eq!(trace.written.len(), 8);
        for (node, log) in trace.logs.iter().enumerate() {
            assert_eq!(log.len(), 8, "node {node} holds the union once");
        }
        // While every edge was partitioned the only pushes were the four
        // writes, each to its own node, and the three exchanges pushed nothing.
        let first: Vec<usize> = trace.deliveries.iter().take(5).map(|d| d.to).collect();
        assert_eq!(first[..4], [0, 1, 2, 3]);
        assert_eq!(
            trace.deliveries[4].ids.len(),
            2,
            "the first push after healing is the hub's own write, carried to a spoke"
        );
    }
}
