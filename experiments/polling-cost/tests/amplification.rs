//! AC-002 — the headline is a **count**, and it is the right count.
//!
//! Discovery's first mutant is a harness reporting `N=10 views: 3.2ms total`,
//! which is the cost of scanning a `Vec` ten times in one process and can be
//! waved at from either side of the argument. The guard is that the headline
//! field is dimensionless: a ratio of two observed counts, with the timings
//! recorded beside it and named as secondary.

use polling_cost::{Arm, Cell, Phase, run_cell};

fn cell(fan_out: usize, log_size: usize, arm: Arm) -> Cell {
    Cell {
        phase: Phase::Amplification,
        fan_out,
        log_size,
        poll_interval_ms: 10,
        chunk: polling_cost::CHUNK,
        arm,
        repeat: 0,
        seed: polling_cost::SEED,
    }
}

#[test]
fn the_headline_is_delivery_amplification_and_not_a_duration() {
    // A hand-built cell with known counts: 8 views, 128 events, every view
    // selecting the whole log. Deliveries are 8 × 128; the distinct events those
    // deliveries carried are 128.
    let record = run_cell(&cell(8, 128, Arm::Overlapping)).expect("a number");

    assert_eq!(record.events_delivered, 8 * 128);
    assert_eq!(record.events_applied_distinct, 128);
    assert!(
        (record.delivery_amplification - 8.0).abs() < f64::EPSILON,
        "the headline is {}, not the fan-out it should equal in this arm",
        record.delivery_amplification
    );

    // The headline is the ratio of two counts and carries no unit. The timing
    // is a separate field, named for what it is, and is not the headline.
    let json = serde_json::to_value(&record).expect("a record serialises");
    assert!(
        json["delivery_amplification"].is_f64(),
        "the headline is not a number at all"
    );
    assert!(
        json["elapsed_ns"].is_u64(),
        "the timing is not recorded beside it"
    );
    // No field name in the record claims the headline is a duration.
    assert!(
        json.get("elapsed_amplification").is_none() && json.get("amplification_ms").is_none(),
        "a duration is wearing the headline's name"
    );
}

#[test]
fn the_two_arms_differ_and_that_is_what_makes_the_ratio_mean_anything() {
    let overlapping = run_cell(&cell(8, 128, Arm::Overlapping)).expect("a number");
    let disjoint = run_cell(&cell(8, 128, Arm::Disjoint)).expect("a number");

    // ES-32's worst case: the same events, delivered once per view.
    assert!(overlapping.delivery_amplification > 7.5);
    // And the case where fan-out costs nothing extra, because the views do not
    // overlap. A ratio that came out the same in both arms would be measuring
    // the instrument.
    assert!(disjoint.delivery_amplification < 1.5);
    assert!(overlapping.delivery_amplification > disjoint.delivery_amplification);
}

#[test]
fn amplification_grows_with_fan_out_in_the_overlapping_arm() {
    let mut previous = 0.0;
    for fan_out in [1usize, 2, 4, 8] {
        let record = run_cell(&cell(fan_out, 64, Arm::Overlapping)).expect("a number");
        assert!(
            record.delivery_amplification > previous,
            "amplification did not grow from {previous} at fan-out {fan_out}"
        );
        previous = record.delivery_amplification;
    }
}
