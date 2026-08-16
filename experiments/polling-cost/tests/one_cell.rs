//! AC-001 — the harness drives the **real** runner, through the sweep's own
//! entry point.
//!
//! A harness that re-implemented the poll loop would measure the harness. The
//! assertion below is over `run_cell`, which is the function the recorded pass
//! calls, so a test that passes here is a statement about the pass rather than
//! about a code path only tests take.

use polling_cost::{Arm, Cell, Phase, harness_sources, run_cell};

fn one(phase: Phase, fan_out: usize, log_size: usize, arm: Arm) -> Cell {
    Cell {
        phase,
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
fn every_seeded_event_reaches_every_view_through_the_runner() {
    let cell = one(Phase::Amplification, 4, 256, Arm::Overlapping);
    let record = run_cell(&cell).expect("the harness produces a number");

    // Overlapping: every view's query selects the whole log, so every view
    // applies every event and the deliveries are the fan-out times the log.
    assert_eq!(record.events_applied_total, 4 * 256);
    assert_eq!(record.events_delivered, 4 * 256);
    assert_eq!(record.events_applied_distinct, 256);

    // Disjoint: the union of the views' selections is still the whole log, and
    // that is the point — the same events, delivered once instead of N times.
    let disjoint = run_cell(&one(Phase::Amplification, 4, 256, Arm::Disjoint))
        .expect("the harness produces a number");
    assert_eq!(disjoint.events_delivered, 256);
    assert_eq!(disjoint.events_applied_distinct, 256);
}

#[test]
fn the_harness_calls_the_published_runner_and_binds_the_bare_flavour() {
    let sources = harness_sources();
    // Comments stripped first. The harness's own pages say *"binds
    // `EventStore`, never `SendEventStore`"*, and a `contains` over raw text
    // would read the sentence that states the rule as a violation of it.
    let all = sources
        .iter()
        .map(|(_, body)| {
            body.lines()
                .map(|line| match line.find("//") {
                    Some(at) => &line[..at],
                    None => line,
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert!(
        all.contains("happenstance::run_projection("),
        "the harness does not call the published runner, so it is measuring \
         itself"
    );
    // Binding constraint 4: bind `EventStore`, never `SendEventStore`. It is the
    // weaker requirement and accepts both flavours.
    assert!(
        !all.contains("SendEventStore") && !all.contains("SendProjectionStore"),
        "the harness bound the `Send` flavour, which is the stronger requirement"
    );
    // And no local poll loop: the runner owns the read, the decode and the
    // chunking, and a second one here would be the thing under measurement.
    for (path, body) in &sources {
        assert!(
            !body.contains("EventStore::read(&self.inner, query, options)")
                || path.ends_with("counting.rs"),
            "{}: a read outside the counting instrument is a second poll loop",
            path.display()
        );
    }
}
