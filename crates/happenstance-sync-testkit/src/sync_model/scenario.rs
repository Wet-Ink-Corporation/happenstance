//! What the model family generates: a peer topology, and the steps driven over
//! it. Also the three scenarios pinned as regressions, and the seed expansion.
//!
//! Everything a step names is a [`Pick`], resolved against a length only when
//! the step runs. So shrinking the topology — fewer peers, fewer edges — never
//! leaves a step naming a peer or an edge that is gone, and no generated value
//! needs a filter. This is `happenstance-testkit`'s `Anchor` idea, one family
//! over.

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::num::{NonZeroU8, NonZeroUsize};

use happenstance_core::{AppendCondition, EventType, Query, QueryItem, Tag, Tags};
use proptest::prelude::{Just, Strategy, any, prop_oneof};

/// The longest step list generated. At three events a write, that is at most
/// 96 events, under the 128 one pull carries at `PeerLimits::minimum()`, so a
/// [`Fault::Delay`] is the only thing that reorders a page: reordering is a
/// scheduled fault, not an accident of paging.
pub(crate) const MAX_STEPS: usize = 32;

/// The most edges a tree topology gains beyond its spanning tree. Two, so a
/// partial mesh — a cycle, a node with two routes to another — is in the space
/// without every topology becoming a full mesh, where every peer meets every
/// other and forwarding is never tested.
const MAX_EXTRA_EDGES: usize = 2;

/// The event type every written event carries: a claim on one seat, so every
/// guarded write conflicts with every other (SY-1's premise).
pub(crate) const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");

/// A symbolic index, resolved against a length that is non-zero by its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Pick(u16);

impl Pick {
    /// The pick that resolves to `raw` against any longer length.
    pub(crate) const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// The index this pick names among `len` things.
    ///
    /// `usize % NonZeroUsize` is the std impl that cannot panic: the divisor is
    /// non-zero by its type (H-03).
    pub(crate) fn of(self, len: NonZeroUsize) -> usize {
        usize::from(self.0) % len
    }
}

/// How the spanning tree of a topology is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    /// `0 — 1 — … — n`, and nothing else: the ends never meet. SY-24's own
    /// A—B—C, stretched to six.
    Chain,
    /// Each node joins one earlier node, then up to [`MAX_EXTRA_EDGES`] more
    /// edges are added.
    Tree,
}

/// Which way an exchange runs along an edge `(a, b)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    /// `a` to `b`.
    Forward,
    /// `b` to `a`.
    Backward,
}

/// What goes wrong with one exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Fault {
    /// Nothing.
    None,
    /// The push is applied and the runner never learns it, so the lane's
    /// resume token stays where it was and the next exchange re-pushes the
    /// same page.
    DropAck,
    /// The same page is pushed twice in a row.
    Duplicate,
    /// The page is pulled and the token advanced, and the page is held in
    /// flight. It lands after the next page this lane pushes.
    Delay,
}

/// Whether a write travels under the seat guard. A closed set rather than a
/// bool (API-10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Guard {
    /// No guard.
    Unguarded,
    /// The position-free seat guard, so two writes on two peers conflict.
    SeatClaim,
}

/// How many events one write puts in its one group: one to three, and never
/// none, by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WriteSize(NonZeroU8);

impl WriteSize {
    pub(crate) const ONE: Self = Self(NonZeroU8::MIN);
    pub(crate) const TWO: Self = Self(NonZeroU8::MIN.saturating_add(1));
    pub(crate) const THREE: Self = Self(NonZeroU8::MIN.saturating_add(2));

    /// The number of events.
    pub(crate) fn get(self) -> usize {
        usize::from(self.0.get())
    }
}

/// One thing the simulator does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Step {
    /// A peer's own application writes one group, and it is pushed to that
    /// peer.
    Write {
        /// The peer.
        at: Pick,
        /// The events in the group.
        events: WriteSize,
        /// Whether the group carries the seat guard.
        guard: Guard,
    },
    /// One pull from one side of an edge, pushed to the other.
    Exchange {
        /// The edge.
        edge: Pick,
        /// Which side is pulled from.
        toward: Direction,
        /// What goes wrong with it.
        fault: Fault,
    },
    /// The edge stops carrying exchanges.
    Partition {
        /// The edge.
        edge: Pick,
    },
    /// The edge carries exchanges again.
    Heal {
        /// The edge.
        edge: Pick,
    },
}

/// A connected, simple graph of three to six peers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Topology {
    peers: NonZeroUsize,
    /// `(a, b)` with `a < b`, no duplicates. Edge `i` is the `i`th.
    edges: Vec<(usize, usize)>,
}

impl Topology {
    const MIN_PEERS: usize = 3;
    const MAX_PEERS: usize = 6;

    /// A topology of `1 + extra` peers. Connected by construction: node
    /// `i + 1` always joins a node in `0..=i`.
    fn drawn(extra: u8, shape: Shape, parents: &[Pick], extras: &[(Pick, Pick)]) -> Self {
        let peers = NonZeroUsize::MIN.saturating_add(usize::from(extra));
        let mut edges = BTreeSet::new();
        for (index, pick) in parents.iter().enumerate() {
            let child = NonZeroUsize::MIN.saturating_add(index);
            let parent = match shape {
                Shape::Chain => index,
                Shape::Tree => pick.of(child),
            };
            edges.insert((parent, child.get()));
        }
        if shape == Shape::Tree {
            for (left, right) in extras {
                let (a, b) = (left.of(peers), right.of(peers));
                if a != b {
                    edges.insert((a.min(b), a.max(b)));
                }
            }
        }
        Self {
            peers,
            edges: edges.into_iter().collect(),
        }
    }

    /// `edges` over `peers` nodes, if that is a topology this family runs:
    /// three to six peers, every edge `(a, b)` with `a < b < peers`, no edge
    /// twice, and connected.
    pub(crate) fn checked(peers: usize, edges: &[(usize, usize)]) -> Option<Self> {
        let count = NonZeroUsize::new(peers)
            .filter(|n| (Self::MIN_PEERS..=Self::MAX_PEERS).contains(&n.get()))?;
        let distinct: BTreeSet<(usize, usize)> = edges.iter().copied().collect();
        let simple =
            distinct.len() == edges.len() && edges.iter().all(|&(a, b)| a < b && b < peers);
        (simple && connected(peers, edges)).then(|| Self {
            peers: count,
            edges: edges.to_vec(),
        })
    }

    pub(crate) const fn peers(&self) -> NonZeroUsize {
        self.peers
    }

    pub(crate) fn edges(&self) -> &[(usize, usize)] {
        &self.edges
    }
}

/// Whether every node in `0..peers` is reachable from node 0 over `edges`.
/// A plain flood fill, written here rather than shared with the generator, so
/// it is an independent check of [`Topology::drawn`].
pub(crate) fn connected(peers: usize, edges: &[(usize, usize)]) -> bool {
    let mut reached = BTreeSet::from([0]);
    let mut frontier = vec![0];
    while let Some(node) = frontier.pop() {
        for &(a, b) in edges {
            let next = if a == node {
                b
            } else if b == node {
                a
            } else {
                continue;
            };
            if reached.insert(next) {
                frontier.push(next);
            }
        }
    }
    (0..peers).all(|node| reached.contains(&node))
}

/// One run of the simulator: a topology and the steps driven over it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Scenario {
    topology: Topology,
    steps: Vec<Step>,
}

impl Scenario {
    /// A scenario written out by hand, for a regression, if its topology is
    /// one [`Topology::checked`] accepts.
    pub(crate) fn fixed(peers: usize, edges: &[(usize, usize)], steps: Vec<Step>) -> Option<Self> {
        Topology::checked(peers, edges).map(|topology| Self { topology, steps })
    }

    pub(crate) const fn topology(&self) -> &Topology {
        &self.topology
    }

    pub(crate) fn steps(&self) -> &[Step] {
        &self.steps
    }
}

fn any_pick() -> impl Strategy<Value = Pick> {
    any::<u16>().prop_map(Pick)
}

/// A topology of three to six peers, connected and simple by construction.
pub(crate) fn any_topology() -> impl Strategy<Value = Topology> {
    // `extra` is the peer count less one, so the parent list is exactly as
    // long as the number of nodes that need a parent.
    (2_u8..=5).prop_flat_map(|extra| {
        (
            prop_oneof![Just(Shape::Chain), Just(Shape::Tree)],
            proptest::collection::vec(any_pick(), usize::from(extra)),
            proptest::collection::vec((any_pick(), any_pick()), 0..=MAX_EXTRA_EDGES),
        )
            .prop_map(move |(shape, parents, extras)| {
                Topology::drawn(extra, shape, &parents, &extras)
            })
    })
}

fn any_step() -> impl Strategy<Value = Step> {
    let size = prop_oneof![
        Just(WriteSize::ONE),
        Just(WriteSize::TWO),
        Just(WriteSize::THREE)
    ];
    let guard = prop_oneof![Just(Guard::Unguarded), Just(Guard::SeatClaim)];
    let toward = prop_oneof![Just(Direction::Forward), Just(Direction::Backward)];
    let fault = prop_oneof![
        5 => Just(Fault::None),
        1 => Just(Fault::DropAck),
        1 => Just(Fault::Duplicate),
        2 => Just(Fault::Delay),
    ];
    prop_oneof![
        3 => (any_pick(), size, guard).prop_map(|(at, events, guard)| Step::Write { at, events, guard }),
        6 => (any_pick(), toward, fault).prop_map(|(edge, toward, fault)| Step::Exchange { edge, toward, fault }),
        1 => any_pick().prop_map(|edge| Step::Partition { edge }),
        1 => any_pick().prop_map(|edge| Step::Heal { edge }),
    ]
}

/// Any scenario: a topology and one to [`MAX_STEPS`] steps. No filter
/// anywhere, so generation never rejects.
pub(crate) fn any_scenario() -> impl Strategy<Value = Scenario> {
    (
        any_topology(),
        proptest::collection::vec(any_step(), 1..=MAX_STEPS),
    )
        .prop_map(|(topology, steps)| Scenario { topology, steps })
}

/// The 32 bytes `TestRng::from_seed` asserts for `ChaCha`: `seed`'s
/// little-endian bytes, four times. Filled by zipping with a cycle rather than
/// by `copy_from_slice`, so there is no length for a copy to disagree with.
pub(crate) fn seed_bytes(seed: u64) -> [u8; 32] {
    let mut out = [0_u8; 32];
    for (slot, byte) in out.iter_mut().zip(seed.to_le_bytes().into_iter().cycle()) {
        *slot = byte;
    }
    out
}

/// The guard a [`Guard::SeatClaim`] write travels under: "nobody has claimed
/// seat s1", with no position, so it is byte-identical wherever it was written.
///
/// A copy of `sync_suite`'s `seat_guard`, not a move of it: that file's lines
/// are cited by number, and a shared helper would be one edit away from two
/// families agreeing with each other.
pub(crate) fn seat_guard() -> Result<AppendCondition, String> {
    let item = QueryItem::new([SEAT_CLAIMED], seat())
        .map_err(|error| format!("the simulator's own query is invalid: {error}"))?;
    Ok(AppendCondition::new(Query::from_item(item)))
}

/// The one tag every written event carries.
pub(crate) fn seat() -> Tags {
    [Tag::from_static("seat:s1")].into_iter().collect()
}

/// A scenario pinned by value rather than by seed: a seed's meaning changes
/// whenever the generator does, and these must not.
#[derive(Debug)]
pub(crate) struct Regression {
    pub(crate) name: &'static str,
    pub(crate) scenario: Scenario,
}

/// The chain every regression runs on: P(0) — H(1) — Q(2). Edge 0 is P–H and
/// edge 1 is H–Q. No regression writes at H, so H has no pick of its own.
const CHAIN: [(usize, usize); 2] = [(0, 1), (1, 2)];
const P: Pick = Pick::new(0);
const Q: Pick = Pick::new(2);
const PH: Pick = Pick::new(0);
const HQ: Pick = Pick::new(1);

const fn write(at: Pick, events: WriteSize) -> Step {
    Step::Write {
        at,
        events,
        guard: Guard::Unguarded,
    }
}

const fn exchange(edge: Pick, toward: Direction, fault: Fault) -> Step {
    Step::Exchange {
        edge,
        toward,
        fault,
    }
}

/// The three regressions, in the order every rule runs them.
///
/// # Errors
///
/// A message if one of them is not a topology the family accepts: the
/// testkit's own defect.
pub(crate) fn regressions() -> Result<Vec<Regression>, String> {
    use Direction::{Backward, Forward};

    let cases = [
        // SF-03: a page that half overlaps what its receiver holds. P's echo,
        // a lost ack that leaves P's token behind B, and H's page `[B],[C]`
        // arriving at P, which holds A and B. One appended, one skipped; and
        // P's re-push of `[B],[C]` is two skips at H.
        (
            "sf03_partial_overlap",
            vec![
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::None),
                exchange(PH, Backward, Fault::None),
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::DropAck),
                write(Q, WriteSize::ONE),
                exchange(HQ, Backward, Fault::None),
                exchange(PH, Backward, Fault::None),
                exchange(PH, Forward, Fault::None),
            ],
        ),
        // A two-event group whose ack is lost: re-pushed whole, then echoed
        // back to its origin. Both redeliveries are two skips.
        (
            "lost_ack_repush_and_echo",
            vec![
                write(P, WriteSize::TWO),
                exchange(PH, Forward, Fault::DropAck),
                exchange(PH, Forward, Fault::None),
                exchange(PH, Backward, Fault::None),
            ],
        ),
        // P's first event is held in flight and lands at H after P's second:
        // an origin's lower position arriving after its higher one. A peer
        // that dedupes by position prefix drops it.
        (
            "delayed_page_lands_after_a_later_one",
            vec![
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::Delay),
                write(P, WriteSize::ONE),
                exchange(PH, Forward, Fault::None),
            ],
        ),
    ];
    cases
        .into_iter()
        .map(|(name, steps)| {
            Scenario::fixed(CHAIN.len() + 1, &CHAIN, steps)
                .map(|scenario| Regression { name, scenario })
                .ok_or_else(|| format!("the regression scenario `{name}` is not a valid topology"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use alloc::collections::BTreeSet;
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use proptest::prelude::{Strategy, any, prop_assert, prop_assert_eq, proptest};
    use proptest::strategy::ValueTree;
    use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

    use super::{Pick, Shape, Topology, any_scenario, any_topology, connected, seed_bytes};

    proptest! {
        #[test]
        fn pick_always_lands_in_range(raw in any::<u16>(), len in 1_usize..=64) {
            let len = NonZeroUsize::new(len).expect("the range starts at one");
            prop_assert!(Pick::new(raw).of(len) < len.get());
        }

        /// Connectivity is checked by [`connected`], a flood fill that shares
        /// nothing with [`Topology::drawn`].
        #[test]
        fn every_topology_is_connected_and_simple(topology in any_topology()) {
            let peers = topology.peers().get();
            let edges = topology.edges();
            prop_assert!((3..=6).contains(&peers), "{peers} peers");
            prop_assert!(connected(peers, edges), "disconnected: {edges:?}");
            let distinct: BTreeSet<_> = edges.iter().collect();
            prop_assert_eq!(distinct.len(), edges.len(), "a duplicate edge: {:?}", edges);
            for &(a, b) in edges {
                prop_assert!(a < b && b < peers, "edge ({}, {}) over {} peers", a, b, peers);
            }
        }
    }

    #[test]
    fn a_chain_never_joins_its_ends() {
        let parents = [Pick::new(7); 5];
        let extras = [(Pick::new(0), Pick::new(5)), (Pick::new(1), Pick::new(4))];
        let chain = Topology::drawn(5, Shape::Chain, &parents, &extras);
        let expected: Vec<(usize, usize)> = (0..5).map(|a| (a, a + 1)).collect();
        assert_eq!(chain.edges(), expected.as_slice());
        assert_eq!(chain.peers().get(), 6);
    }

    #[test]
    fn one_seed_is_one_scenario() {
        let bytes = seed_bytes(0x0123_4567_89ab_cdef);
        assert_eq!(bytes.len(), 32);
        let draw = || {
            let mut runner = TestRunner::new_with_rng(
                Config::default(),
                TestRng::from_seed(RngAlgorithm::ChaCha, &bytes),
            );
            any_scenario()
                .new_tree(&mut runner)
                .expect("the scenario strategy has no filter")
                .current()
        };
        assert_eq!(draw(), draw());
        assert_ne!(
            seed_bytes(1),
            seed_bytes(2),
            "two seeds must not collapse to one stream"
        );
    }

    #[test]
    fn every_regression_is_a_valid_scenario() {
        let all = super::regressions().expect("every regression is a valid topology");
        assert_eq!(all.len(), 3);
    }
}
