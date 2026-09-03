//! Arms 5 & 6: `MemoryEventStore::read` with `ReadOptions::new().limit(1)`
//! against a fully-matching query, at store sizes 10 / 10,000 / 1,000,000,
//! before and after moving `.take(limit)` above `.cloned()`.
//!
//! Run after `arms_are_equivalent`, which is what establishes that
//! `readpath::read_before` is `memory.rs:302-333` and that `readpath::read_after`
//! returns the same answer. Without that, this file is a benchmark of two
//! functions with no established relationship to the crate.
//!
//! Two rows are printed per case that the finding does not ask for and that
//! change how it reads:
//!
//! * `limit=None` through the **real** store, so the `limit(1)` figure can be
//!   compared against reading the entire matched set. If they are equal, the
//!   `limit` argument bought the caller nothing at all on this path, and that is
//!   a stronger statement than "limit is inefficient".
//! * a 64-tag case at 10,000 events, because the per-event clone cost is `t + 2`
//!   (`measure_clone.rs`) and the read path pays it once per *matched* event —
//!   so the two findings multiply rather than add, and the product is the number
//!   an adapter author would actually meet.

use std::time::{Duration, Instant};

use event_clone_allocations::arms::{Payload, Regime};
use event_clone_allocations::counting::{Counts, measure};
use event_clone_allocations::readpath::{self, filled, matching_query, read_real};
use happenstance_core::{MemoryEventStore, Query, ReadOptions, SequencedEvent};

/// How many timed repetitions each arm gets. The median is reported; the spread
/// is printed beside it so a reader can see whether the median means anything.
const REPETITIONS: usize = 5;

fn median(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn time<T>(repetitions: usize, mut f: impl FnMut() -> T) -> (Duration, Duration, Duration) {
    // One warm-up outside the timer: the first pass faults in pages the
    // allocator has not touched yet, and at a million events that is the
    // difference between a median and a first-run artefact.
    drop(std::hint::black_box(f()));
    let mut samples = Vec::with_capacity(repetitions);
    for _ in 0..repetitions {
        let start = Instant::now();
        let value = std::hint::black_box(f());
        samples.push(start.elapsed());
        drop(value);
    }
    let lo = *samples.iter().min().expect("at least one sample");
    let hi = *samples.iter().max().expect("at least one sample");
    (median(samples), lo, hi)
}

struct Case {
    events: usize,
    tags_per_event: usize,
}

const CASES: [Case; 4] = [
    Case {
        events: 10,
        tags_per_event: 2,
    },
    Case {
        events: 10_000,
        tags_per_event: 2,
    },
    Case {
        events: 1_000_000,
        tags_per_event: 2,
    },
    // VT-22's floor, at a size that still fits comfortably beside its own mirror.
    Case {
        events: 10_000,
        tags_per_event: 64,
    },
];

fn report(
    label: &str,
    counts: Counts,
    returned: usize,
    timing: Option<(Duration, Duration, Duration)>,
) {
    let timing = timing.map_or_else(
        || "                                  ".to_owned(),
        |(mid, lo, hi)| format!("median={mid:>12.3?} [{lo:.3?}..{hi:.3?}]"),
    );
    println!("  {label:<34} returned={returned:<8} {counts}  {timing}");
}

#[test]
fn read_limit_one() {
    println!("\n== arms 5 & 6: read(limit=1) against a fully-matching query ==");
    println!(
        "regime = from_pairs (Cow::Owned) throughout: a store filled with\n\
         `Tag::from_static` tags would make every clone below cost one allocation\n\
         regardless of size, which is the measurement this experiment exists to\n\
         refuse. Payload = Bytes::from_static, so no promotion allocation lands\n\
         in a read.\n"
    );

    for case in CASES {
        let Case {
            events,
            tags_per_event,
        } = case;
        println!(
            "-- {events} events x {tags_per_event} owned tags \
             (clone cost per event = {} allocations) --",
            tags_per_event + 2
        );

        let (store, mirror): (MemoryEventStore, Vec<SequencedEvent>) =
            filled(events, Regime::Owned, tags_per_event, Payload::Static);
        let query = matching_query(Regime::Owned, tags_per_event);

        // The query must match everything, or the sweep is measuring the filter
        // rather than the clone.
        let all = read_real(&store, &query, ReadOptions::new());
        assert_eq!(all.len(), events, "the query is not fully matching");
        drop(all);

        let limited = ReadOptions::new().limit(1);
        let unlimited = ReadOptions::new();

        let (result, counts) = measure(|| read_real(&store, &query, limited));
        let returned = result.len();
        drop(result);
        let timing = time(REPETITIONS, || read_real(&store, &query, limited));
        report("real store, limit=1", counts, returned, Some(timing));
        let real_limited = counts;

        let (result, counts) = measure(|| read_real(&store, &query, unlimited));
        let returned = result.len();
        drop(result);
        report("real store, limit=None", counts, returned, None);
        let real_unlimited = counts;

        let (result, counts) = measure(|| readpath::read_before(&mirror, &query, limited));
        let returned = result.len();
        drop(result);
        let timing = time(REPETITIONS, || {
            readpath::read_before(&mirror, &query, limited)
        });
        report("replica BEFORE, limit=1", counts, returned, Some(timing));
        let before = counts;

        let (result, counts) = measure(|| readpath::read_after(&mirror, &query, limited));
        let returned = result.len();
        drop(result);
        let timing = time(REPETITIONS, || {
            readpath::read_after(&mirror, &query, limited)
        });
        report("replica AFTER,  limit=1", counts, returned, Some(timing));
        let after = counts;

        println!(
            "  => limit=1 costs {:.4}x what limit=None costs on the real store; \
             the fix costs {:.6}x the before arm ({} vs {} heap ops)",
            real_limited.heap_ops() as f64 / real_unlimited.heap_ops() as f64,
            after.heap_ops() as f64 / before.heap_ops() as f64,
            after.heap_ops(),
            before.heap_ops()
        );
        println!(
            "  => bytes requested: before={} after={} ({:.6}x)",
            before.bytes,
            after.bytes,
            after.bytes as f64 / before.bytes as f64
        );

        // The claim, checked. The before arm's cost is proportional to the
        // matched set; the after arm's is proportional to the limit and is the
        // same small number at ten events and at a million.
        assert!(
            before.heap_ops() >= (events as u64) * (tags_per_event as u64 + 2),
            "the before arm did not clone the whole matched set"
        );
        // One returned event's clone (`t + 2`) plus a handful for the output
        // `Vec`. Written against the *limit* and the *tag count*, never against
        // the store size, because independence from the store size is the whole
        // claim and an assertion that mentioned `events` could not state it.
        let after_ceiling = (tags_per_event as u64 + 2) + 4;
        assert!(
            after.heap_ops() <= after_ceiling,
            "the after arm's cost should not scale with the store; got {} against a \
             ceiling of {after_ceiling} derived from limit=1 and {tags_per_event} tags",
            after.heap_ops()
        );
        // And the real store is the before arm, within the handful of
        // reallocations `drain`'s output `Vec` costs.
        assert!(
            real_limited.heap_ops().abs_diff(before.heap_ops()) < 64,
            "the real store ({}) and the replica ({}) disagree",
            real_limited.heap_ops(),
            before.heap_ops()
        );

        drop(mirror);
        drop(store);
        println!();
    }
}

/// The control that makes the sweep mean something: at `limit = None` the two
/// shapes must cost the *same*, because there is nothing to take. A fix that
/// were cheaper everywhere would be a fix that had changed the semantics.
#[test]
fn the_fix_is_free_and_only_free_when_unlimited() {
    let events = 10_000;
    let (_store, mirror) = filled(events, Regime::Owned, 2, Payload::Static);
    let query = matching_query(Regime::Owned, 2);
    let unlimited = ReadOptions::new();

    let (before_result, before) = measure(|| readpath::read_before(&mirror, &query, unlimited));
    let (after_result, after) = measure(|| readpath::read_after(&mirror, &query, unlimited));

    println!("\n== control: limit=None, {events} events ==");
    println!("  before {before}");
    println!("  after  {after}");
    assert_eq!(before_result.len(), events);
    assert_eq!(after_result.len(), events);
    assert_eq!(
        before.allocs, after.allocs,
        "unlimited reads must cost the same in both shapes"
    );
}

/// A second control, on the query rather than on the limit: `Query::All`
/// short-circuits `matches`, so a sweep run against it would be measuring a read
/// path with the tag comparison compiled out. The figures above use a tag query
/// on purpose; this prints both so the difference is visible rather than
/// asserted.
#[test]
fn all_versus_a_matching_tag_query() {
    let events = 10_000;
    let (store, _mirror) = filled(events, Regime::Owned, 2, Payload::Static);
    let limited = ReadOptions::new().limit(1);

    let all = Query::all();
    let tagged = matching_query(Regime::Owned, 2);

    let (result, all_counts) = measure(|| read_real(&store, &all, limited));
    drop(result);
    let (result, tagged_counts) = measure(|| read_real(&store, &tagged, limited));
    drop(result);

    println!("\n== control: which query, {events} events, limit=1 ==");
    println!("  Query::all()      {all_counts}");
    println!("  tag query         {tagged_counts}");
    assert_eq!(
        all_counts.allocs, tagged_counts.allocs,
        "the clone cost does not depend on which fully-matching query asked"
    );
}
