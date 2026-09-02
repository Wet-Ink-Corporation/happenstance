//! AC-002 — the grid contains every declared axis value and both arms.
//!
//! Ranges may be trimmed for runtime if the trim is recorded with its reason.
//! The *shape* may not: dropping the 32-view cell, or the overlapping arm, would
//! remove exactly the case the tail-seam argument turns on and leave a sweep
//! that looks complete.

use std::collections::BTreeSet;

use polling_cost::{ARMS, Arm, FAN_OUTS, LOG_SIZES, POLL_INTERVALS_MS, Phase, sweep};

#[test]
fn every_declared_axis_value_appears() {
    let cells = sweep();
    assert!(!cells.is_empty(), "the sweep enumerates nothing");

    let fan_outs: BTreeSet<usize> = cells.iter().map(|c| c.fan_out).collect();
    for declared in FAN_OUTS {
        assert!(
            fan_outs.contains(&declared),
            "fan-out {declared} is missing"
        );
    }

    let logs: BTreeSet<usize> = cells
        .iter()
        .filter(|c| c.phase == Phase::Amplification)
        .map(|c| c.log_size)
        .collect();
    for declared in LOG_SIZES {
        assert!(logs.contains(&declared), "log size {declared} is missing");
    }

    let intervals: BTreeSet<u64> = cells
        .iter()
        .filter(|c| c.phase == Phase::Staleness)
        .map(|c| c.poll_interval_ms)
        .collect();
    for declared in POLL_INTERVALS_MS {
        assert!(
            intervals.contains(&declared),
            "poll interval {declared}ms is missing"
        );
    }
}

#[test]
fn both_selectivity_arms_are_present_in_both_phases() {
    let cells = sweep();
    for phase in [Phase::Amplification, Phase::Staleness] {
        for arm in ARMS {
            assert!(
                cells.iter().any(|c| c.phase == phase && c.arm == arm),
                "the {} phase has no {} arm",
                phase.as_str(),
                arm.as_str()
            );
        }
    }
    // Named rather than counted, so a reader of the failure knows which one
    // went: the overlapping arm is ES-32's worst case.
    assert!(
        cells
            .iter()
            .any(|c| c.arm == Arm::Overlapping && c.fan_out == 32),
        "the 32-view overlapping cell — the one the argument turns on — is gone"
    );
}

#[test]
fn every_cell_carries_its_seed_and_its_repeat() {
    for cell in sweep() {
        assert_eq!(cell.seed, polling_cost::SEED);
        assert!(cell.repeat < polling_cost::REPEATS);
        assert_eq!(cell.chunk, polling_cost::CHUNK);
    }
}
