//! The simulator against the reference peer: the pinned regressions' traces,
//! and a partitioned star that heals.

use alloc::vec::Vec;

use happenstance_core::EventId;
use happenstance_testkit::block_on;

use super::{Trace, simulate};
use crate::MemorySyncFixture;
use crate::sync_model::regressions::regressions;
use crate::sync_model::scenario::{Direction, Fault, Guard, Pick, Scenario, Step, WriteSize};

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
