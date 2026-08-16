//! AC-008 — a number nobody likes is still a number, and the run still passes.
//!
//! The harness has no pass/fail path. It exits non-zero when *it* fails and
//! never because a value was large, and it holds no threshold anywhere. That is
//! CF-34 as code rather than as an intention.

use polling_cost::{Arm, Cell, Phase, harness_sources, run_cell};

#[test]
fn an_absurd_amplification_still_returns_ok_and_writes_a_record() {
    // Thirty-two views over one log, every one selecting all of it: the worst
    // case the tail-seam argument is about, and a figure somebody will dislike.
    let record = run_cell(&Cell {
        phase: Phase::Amplification,
        fan_out: 32,
        log_size: 512,
        poll_interval_ms: 10,
        chunk: polling_cost::CHUNK,
        arm: Arm::Overlapping,
        repeat: 0,
        seed: polling_cost::SEED,
    })
    .expect("a large number is not a failure");

    assert!(record.delivery_amplification > 30.0);
    // And it serialises into a record like any other, rather than being
    // suppressed or flagged.
    let json = serde_json::to_string(&record).expect("it serialises");
    assert!(json.contains("\"delivery_amplification\""));
}

#[test]
fn the_harness_holds_no_threshold() {
    for (path, body) in harness_sources() {
        // `sources.rs` is test support: it reads the harness so a test can
        // assert about it, and it never runs during a measurement. Excluding it
        // here is narrower than carving an exception into the scan itself.
        if path.ends_with("sources.rs") {
            continue;
        }
        let code: String = body
            .lines()
            .map(|line| match line.find("//") {
                Some(at) => &line[..at],
                None => line,
            })
            .collect::<Vec<_>>()
            .join("\n");
        for banned in [
            "THRESHOLD",
            "BUDGET_",
            "MAX_AMPLIFICATION",
            "assert!(",
            "assert_eq!(",
            "panic!(",
        ] {
            assert!(
                !code.contains(banned),
                "{}: `{banned}` on the measured path. A harness that can fail on \
                 a value is a gate step wearing a different name",
                path.display()
            );
        }
    }
}

#[test]
fn the_error_type_has_no_arm_for_a_number_being_large() {
    let sources = harness_sources();
    let lib = sources
        .iter()
        .find(|(path, _)| path.ends_with("lib.rs"))
        .map(|(_, body)| body.as_str())
        .expect("the harness has a lib.rs");
    let at = lib
        .find("pub enum HarnessError")
        .expect("the error is declared");
    let body = &lib[at..lib[at..].find("\n}\n").map_or(lib.len(), |e| at + e)];

    for arm in ["TooSlow", "Exceeded", "OverBudget", "Regression"] {
        assert!(
            !body.contains(arm),
            "`HarnessError` grew a `{arm}` arm, which is a verdict in a type"
        );
    }
}
