//! AC-004 — staleness is observed, and `head() - checkpoint` appears nowhere.
//!
//! The mutant this file exists to reject is the obvious one: a backlog computed
//! as the difference between the head position and the checkpoint position.
//! Against a dense store it is right, which is what lets it survive review.
//! Against a store that leaves holes it reports a backlog that does not exist —
//! and every store in this specification is permitted to leave holes.
//!
//! So the observer is fed a **gapped** position sequence and a dense one
//! carrying the same delivery pattern, and must report the same figures. The
//! subtraction reports a larger number for the gapped one and fails.

use std::num::NonZeroU64;

use happenstance::{Checkpoint, Event, EventStore, EventType, MemoryEventStore, SequencePosition};
use happenstance_testkit::GappyMemoryStore;
use polling_cost::{Observer, block_on, harness_sources};

const MEASURED: EventType = EventType::from_static("Measured");

fn event(ordinal: u64) -> Event {
    Event::new(MEASURED, ordinal.to_string().into_bytes()).expect("a valid event type")
}

/// The mutant: a backlog derived by subtracting one position from another.
///
/// It is written here rather than in the harness so it can be failed. A rule no
/// implementation can fail is decorative.
fn subtracting_pending(head: Option<SequencePosition>, checkpoint: Checkpoint) -> u64 {
    let head = head.map_or(0, SequencePosition::get);
    let through = match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => through.get(),
        _ => 0,
    };
    head.saturating_sub(through)
}

/// Appends three events and returns their positions, from whichever store.
fn dense() -> Vec<SequencePosition> {
    let store = MemoryEventStore::new();
    block_on(async {
        let mut out = Vec::new();
        for ordinal in 0..3u64 {
            out.push(
                EventStore::append(&store, &[event(ordinal)], None)
                    .await
                    .expect("the dense fixture appends"),
            );
        }
        out
    })
}

fn gapped() -> Vec<SequencePosition> {
    let store = GappyMemoryStore::with_stride(NonZeroU64::new(7).expect("seven is not zero"));
    block_on(async {
        let mut out = Vec::new();
        for ordinal in 0..3u64 {
            out.push(
                EventStore::append(&store, &[event(ordinal)], None)
                    .await
                    .expect("the gappy fixture appends"),
            );
        }
        out
    })
}

/// Feeds one position sequence through the observer with one event observed.
fn pending_after_observing_the_first(positions: &[SequencePosition]) -> usize {
    let mut observer = Observer::new();
    for (index, position) in positions.iter().enumerate() {
        observer.appended(*position, index as u64 * 1_000);
    }
    let checkpoint = Checkpoint::Live {
        through: positions[0],
    };
    observer.observe(checkpoint, 5_000);
    observer.pending(checkpoint)
}

#[test]
fn the_observer_reports_the_same_backlog_on_a_gapped_store() {
    let dense = dense();
    let gapped = gapped();

    // The same delivery pattern: three appended, the first observed.
    assert_eq!(pending_after_observing_the_first(&dense), 2);
    assert_eq!(pending_after_observing_the_first(&gapped), 2);
    assert_eq!(
        pending_after_observing_the_first(&dense),
        pending_after_observing_the_first(&gapped),
        "the observer's answer moved with the store's position stride, which \
         means it is subtracting"
    );
}

#[test]
fn the_subtracting_mutant_fails_the_same_comparison() {
    let dense = dense();
    let gapped = gapped();

    let dense_backlog = subtracting_pending(
        dense.last().copied(),
        Checkpoint::Live { through: dense[0] },
    );
    let gapped_backlog = subtracting_pending(
        gapped.last().copied(),
        Checkpoint::Live { through: gapped[0] },
    );

    // Both should be 2. The subtraction says 2 and 14, and only one of those is
    // a count of anything.
    assert_eq!(dense_backlog, 2);
    assert_eq!(gapped_backlog, 14);
    assert_ne!(
        dense_backlog, gapped_backlog,
        "the mutant agreed with the observer, so this test proves nothing and \
         the discriminator has to be rebuilt"
    );
}

#[test]
fn staleness_is_a_duration_between_two_observed_instants() {
    let positions = dense();
    let mut observer = Observer::new();
    observer.appended(positions[0], 1_000);
    observer.appended(positions[1], 2_000);

    observer.observe(
        Checkpoint::Live {
            through: positions[1],
        },
        9_000,
    );

    // Two samples, each the gap between the append instant and the observation
    // instant. Nothing here is derived from a position.
    assert_eq!(observer.staleness_ns(), &[8_000, 7_000]);
    assert_eq!(
        observer.pending(Checkpoint::Live {
            through: positions[1]
        }),
        0
    );
}

#[test]
fn no_head_value_participates_in_arithmetic_anywhere_in_the_harness() {
    for (path, body) in harness_sources() {
        let code: String = body
            .lines()
            .map(|line| match line.find("//") {
                Some(at) => &line[..at],
                None => line,
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !code.contains("head()"),
            "{}: the harness reads `head()`, and the only thing it could be for \
             is the subtraction this file rejects",
            path.display()
        );
        for forbidden in ["saturating_sub", "checked_sub", "wrapping_sub"] {
            assert!(
                !code.contains(&format!("position.{forbidden}"))
                    && !code.contains(&format!("through.{forbidden}")),
                "{}: a position took part in `{forbidden}`",
                path.display()
            );
        }
    }
}
