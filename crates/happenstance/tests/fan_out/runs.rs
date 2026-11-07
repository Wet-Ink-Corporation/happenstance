//! The fan-out runner against `run_projection`, view by view.
//!
//! The oracle is the runner this crate already ships: N views fanned out
//! over one read must leave the store, every checkpoint and every outcome
//! exactly as N sequential `run_projection` calls leave them.

use std::collections::BTreeMap;
use std::panic::AssertUnwindSafe;

use happenstance::{
    Checkpoint, Event, EventStore, Json, Progressed, ProjectionError, SequencePosition, ViewError,
    checkpoint_id, run_fan_out, run_projection,
};
use happenstance_testkit::block_on;

use crate::support::{
    Answers, Call, Instrumented, ReadFault, Shard, Watched, chunk, part, part_in, poisoned,
    poisoned_in,
};

/// A view's outcome as both runners can report it.
type Outcome = Result<Progressed, (Option<SequencePosition>, Progressed)>;

/// Everything a run leaves that a reader could observe.
#[derive(Debug, PartialEq, Eq)]
struct Observed {
    rows: BTreeMap<String, u64>,
    checkpoints: Vec<Checkpoint>,
    outcomes: Vec<Outcome>,
    /// Each view's own state after the run: how many it had applied.
    state: Vec<u64>,
}

async fn checkpoints(models: &Watched, views: &[Shard]) -> Vec<Checkpoint> {
    let mut out = Vec::with_capacity(views.len());
    for view in views {
        out.push(models.checkpoint_of(view).await);
    }
    out
}

/// The oracle: each view through `run_projection`, one after another.
///
/// Over the same event store as the subject, so the identities a skip
/// record is keyed on agree, and into a fresh projection store.
async fn sequentially(events: &Instrumented, views: &[Shard], size: usize) -> Observed {
    let models = Watched::new();
    let mut ran: Vec<Shard> = views.iter().map(Shard::fresh).collect();
    let mut outcomes = Vec::with_capacity(ran.len());
    for view in &mut ran {
        let outcome = run_projection(events, &models, view, &Json, chunk(size)).await;
        outcomes.push(outcome.map_err(|e: ProjectionError<_, _, _>| (e.position(), e.progress())));
    }
    Observed {
        rows: models.snapshot(),
        checkpoints: checkpoints(&models, &ran).await,
        outcomes,
        state: ran.iter().map(|view| view.applied).collect(),
    }
}

/// The subject: every view through one `run_fan_out`.
async fn fanned(events: &Instrumented, views: &[Shard], size: usize) -> (Observed, Watched) {
    let models = Watched::new();
    let mut ran: Vec<Shard> = views.iter().map(Shard::fresh).collect();
    let fan = run_fan_out(events, &models, &mut ran, &Json, chunk(size)).await;
    assert_eq!(fan.read, None, "the memory read failed");
    let outcomes = fan
        .views
        .into_iter()
        .map(|outcome| outcome.map_err(|e: ViewError<_, _>| (e.position(), e.progress())))
        .collect();
    let observed = Observed {
        rows: models.snapshot(),
        checkpoints: checkpoints(&models, &ran).await,
        outcomes,
        state: ran.iter().map(|view| view.applied).collect(),
    };
    (observed, models)
}

fn live(at: SequencePosition) -> Checkpoint {
    Checkpoint::Live { through: at }
}

// ---------------------------------------------------------------------------
// The degenerate cases
// ---------------------------------------------------------------------------

#[test]
fn an_empty_fan_out_reads_nothing() {
    block_on(async {
        let events = Instrumented::seeded(&[part("VIN1")]).await;
        let models = Watched::new();
        let mut views: Vec<Shard> = Vec::new();

        let ran = run_fan_out(&events, &models, &mut views, &Json, chunk(4)).await;

        assert_eq!(ran.views.len(), 0);
        assert_eq!(ran.read, None);
        assert_eq!(events.reads(), 0, "no view, and still a read");
        assert_eq!(models.calls(), []);
    });
}

#[test]
fn one_view_is_run_projection() {
    block_on(async {
        let events = Instrumented::seeded(&[part("VIN1"), part("VIN2"), part("VIN3")]).await;
        let views = [Shard::new("solo")];
        for size in 1..=4 {
            let (got, _) = fanned(&events, &views, size).await;
            assert_eq!(
                got,
                sequentially(&events, &views, size).await,
                "chunk {size}"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// One read, and each view's own share of it
// ---------------------------------------------------------------------------

#[test]
fn n_views_cost_one_read() {
    block_on(async {
        let events = Instrumented::seeded(&[part("VIN1"), part("VIN2"), part("VIN3")]).await;
        let views = [Shard::new("a"), Shard::new("b"), Shard::new("c")];

        let (got, _) = fanned(&events, &views, 2).await;

        assert_eq!(events.reads(), 1, "three views, and not one read");
        assert_eq!(events.delivered(), 3, "an event was delivered twice");
        assert_eq!(got.state, [3, 3, 3]);
    });
}

#[test]
fn each_view_sees_exactly_its_own_query() {
    block_on(async {
        // Shard 3 is nobody's: the union is pushed into the read, so the store
        // never hands it over.
        let log = [
            part_in("A1", 0),
            part_in("B1", 1),
            part_in("C1", 2),
            part_in("D1", 3),
            part_in("A2", 0),
            part_in("C2", 2),
        ];
        let views = [
            Shard::new("zero").in_shard(0),
            Shard::new("one").in_shard(1),
            Shard::new("two").in_shard(2),
        ];

        let events = Instrumented::seeded(&log).await;
        let (got, _) = fanned(&events, &views, 8).await;

        let rows: Vec<&str> = got.rows.keys().map(String::as_str).collect();
        assert_eq!(rows, ["one:B1", "two:C1", "two:C2", "zero:A1", "zero:A2"]);
        assert_eq!(events.reads(), 1);
        assert_eq!(events.delivered(), 5, "the read was not the views' union");
        assert_eq!(got, sequentially(&events, &views, 8).await);
    });
}

#[test]
fn fan_out_resumes_each_view_from_its_own_checkpoint() {
    block_on(async {
        // PS-38, both sentences, over N checkpoint ids on one handle: each
        // view resumes past its own checkpoint, and a commit under one id
        // moves no other.
        let events = Instrumented::seeded(&[part("VIN1"), part("VIN2")]).await;
        let models = Watched::new();
        let mut ahead = Shard::new("ahead");
        let pre = run_projection(&events, &models, &mut ahead, &Json, chunk(8))
            .await
            .expect("the pre-run reaches the end");
        assert_eq!(pre.applied, 2);
        EventStore::append(&events.inner, &[part("VIN3"), part("VIN4")], None)
            .await
            .expect("the tail appends");
        let log = events.snapshot();
        let reads = events.reads();

        let mut views = [ahead, Shard::new("fresh")];
        let ran = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;

        let applied: Vec<usize> = ran
            .views
            .iter()
            .map(|outcome| outcome.as_ref().expect("both views reach the end").applied)
            .collect();
        assert_eq!(applied, [2, 4], "a view re-applied what it had committed");
        assert_eq!(events.reads() - reads, 1);
        assert_eq!(models.get("ahead:VIN1"), Some(1), "the pre-run's row moved");
        assert_eq!(models.get("ahead:VIN4"), Some(4));
        for view in &views {
            assert_eq!(models.checkpoint_of(view).await, live(log[3].position));
        }

        // Both behind, at different places: the one read starts at the lower.
        EventStore::append(&events.inner, &[part("VIN5")], None)
            .await
            .expect("the tail appends");
        let [ahead, fresh] = views;
        let mut behind = Shard::new("behind");
        let pre = run_projection(&events, &models, &mut behind, &Json, chunk(8))
            .await
            .expect("the pre-run reaches the end");
        assert_eq!(pre.applied, 5);
        EventStore::append(&events.inner, &[part("VIN6")], None)
            .await
            .expect("the tail appends");
        let delivered = events.delivered();
        let mut views = [ahead, fresh, behind];
        let ran = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;
        let applied: Vec<usize> = ran
            .views
            .iter()
            .map(|outcome| {
                outcome
                    .as_ref()
                    .expect("every view reaches the end")
                    .applied
            })
            .collect();
        assert_eq!(applied, [2, 2, 1]);
        assert_eq!(
            events.delivered() - delivered,
            2,
            "the read began below every view"
        );
    });
}

#[test]
fn a_commit_for_one_view_moves_no_other_views_checkpoint() {
    block_on(async {
        let events = Instrumented::seeded(&[part("VIN1"), part("VIN2")]).await;
        let views = [Shard::new("busy"), Shard::new("idle").in_shard(9)];

        let (got, models) = fanned(&events, &views, 1).await;

        let read = events.snapshot();
        assert_eq!(
            got.checkpoints,
            [live(read[1].position), Checkpoint::NeverRun]
        );
        assert_eq!(got.outcomes[1], Ok(Progressed::default()));
        // The idle view nominated nothing, so it never opened a write set.
        assert_eq!(
            models.calls(),
            [
                Call::Begin(0),
                Call::Commit(0),
                Call::Begin(1),
                Call::Commit(1)
            ]
        );
    });
}

// ---------------------------------------------------------------------------
// Each view fails alone
// ---------------------------------------------------------------------------

#[test]
fn a_halting_view_stops_alone() {
    block_on(async {
        let events = Instrumented::seeded(&[part("VIN1"), poisoned(), part("VIN2")]).await;
        let views = [Shard::new("halts"), Shard::new("skips").skipping()];
        let read = events.snapshot();
        let models = Watched::new();
        let mut ran: Vec<Shard> = views.iter().map(Shard::fresh).collect();
        let fan = run_fan_out(&events, &models, &mut ran, &Json, chunk(1)).await;

        assert!(
            matches!(
                &fan.views[0],
                Err(ViewError::Stopped(ProjectionError::Decode { position, rollback: None, .. }))
                    if *position == read[1].position
            ),
            "the halting view's own decode failure: {:?}",
            fan.views[0]
        );
        let skipped = fan.views[1].as_ref().expect("the skipping view carries on");
        assert_eq!((skipped.applied, skipped.skipped), (2, 1));
        assert_eq!(models.checkpoint_of(&ran[0]).await, live(read[0].position));
        assert_eq!(models.checkpoint_of(&ran[1]).await, live(read[2].position));
        assert_eq!(models.get(&ran[1].skip_row(read[1].id)), Some(1));
        assert_eq!(models.get("halts:VIN2"), None);
    });
}

#[test]
fn a_failing_begin_or_commit_stops_only_its_view() {
    block_on(async {
        let log = [part("VIN1"), part("VIN2")];
        let events = Instrumented::seeded(&log).await;

        // The second `begin` of the run is the middle view's first.
        let models = Watched::failing_begin(1);
        let mut views = [Shard::new("a"), Shard::new("b"), Shard::new("c")];
        let fan = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;
        assert!(
            matches!(
                &fan.views[1],
                Err(ViewError::Stopped(ProjectionError::Begin { .. }))
            ),
            "{:?}",
            fan.views[1]
        );
        assert!(
            fan.views[0].is_ok() && fan.views[2].is_ok(),
            "{:?}",
            fan.views
        );
        assert_eq!(views[1].applied, 0, "a view with no write set applied");

        let mut views = [Shard::new("a"), Shard::new("b"), Shard::new("c")];
        let models = Watched::failing_commit(checkpoint_id(&views[1]).expect("a derivable id"));
        let fan = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;
        assert!(
            matches!(
                &fan.views[1],
                Err(ViewError::Stopped(ProjectionError::Commit { .. }))
            ),
            "{:?}",
            fan.views[1]
        );
        assert!(
            fan.views[0].is_ok() && fan.views[2].is_ok(),
            "{:?}",
            fan.views
        );
        assert_eq!(models.checkpoint_of(&views[1]).await, Checkpoint::NeverRun);
        assert_eq!(models.get("b:VIN1"), None, "a failed commit left rows");
        assert_eq!(models.get("c:VIN2"), Some(2));
    });
}

#[test]
fn a_read_failure_interrupts_every_running_view() {
    block_on(async {
        let log = [part("VIN1"), part("VIN2"), part("VIN3")];
        let mut events = Instrumented::seeded(&log).await;
        events.fail_after = Some(2);
        let models = Watched::new();
        let mut views = [
            Shard::new("a"),
            Shard::new("refuses").refusing("VIN1"),
            Shard::new("c"),
        ];

        let fan = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;

        assert_eq!(fan.read, Some(ReadFault::Injected));
        for at in [0, 2] {
            assert!(
                matches!(
                    &fan.views[at],
                    Err(ViewError::Interrupted { progress, rollback: None })
                        if *progress == Progressed::default()
                ),
                "view {at}: {:?}",
                fan.views[at]
            );
        }
        assert!(
            matches!(
                &fan.views[1],
                Err(ViewError::Stopped(ProjectionError::Apply { .. }))
            ),
            "a view that stopped first keeps its own failure: {:?}",
            fan.views[1]
        );
        assert!(
            fan.views.iter().all(Result::is_err),
            "a view is Ok after a read failure"
        );
        assert!(models.snapshot().is_empty(), "{:?}", models.snapshot());
        let calls = models.calls();
        for call in &calls {
            if let Call::Begin(serial) = call {
                assert!(
                    calls.contains(&Call::Rollback(*serial)),
                    "write set {serial} was not rolled back: {calls:?}"
                );
            }
        }
    });
}

#[test]
fn the_fan_out_stops_reading_once_every_view_has_stopped() {
    block_on(async {
        let log = [part("VIN1"), part("VIN2"), part("VIN3")];
        let events = Instrumented::seeded(&log).await;
        let models = Watched::new();
        let mut views = [
            Shard::new("a").refusing("VIN1"),
            Shard::new("b").refusing("VIN1"),
        ];

        let fan = run_fan_out(&events, &models, &mut views, &Json, chunk(8)).await;

        assert!(fan.views.iter().all(Result::is_err));
        assert_eq!(
            events.delivered(),
            1,
            "the read went on with nobody to serve"
        );
    });
}

// ---------------------------------------------------------------------------
// The oracle, exhaustively
// ---------------------------------------------------------------------------

/// Where the sweep puts its one failure.
#[derive(Debug, Clone, Copy)]
enum Fault {
    /// The event at this index is a payload no codec reads.
    Decode(usize),
    /// The first view refuses the event at this index in `apply`.
    Refuse(usize),
}

/// The five VINs the sweep's log carries, alternating between two shards.
const VINS: [&str; 5] = ["V0", "V1", "V2", "V3", "V4"];

/// Three views over the sweep's log: one over both shards, one per shard.
/// The first two answer as `answers` says; the third always skips.
fn sweep_case(fault: Fault, answers: [Answers; 2]) -> (Vec<Event>, Vec<Shard>) {
    let log = VINS
        .iter()
        .enumerate()
        .map(|(at, vin)| {
            let shard = u8::try_from(at % 2).expect("two shards");
            match fault {
                Fault::Decode(poison) if poison == at => poisoned_in(shard),
                _ => part_in(vin, shard),
            }
        })
        .collect();
    let answering = |mut view: Shard, answers: Answers| {
        view.answers = answers;
        view
    };
    let [first, second] = answers;
    let mut every = answering(Shard::new("every"), first);
    if let Fault::Refuse(at) = fault {
        every = every.refusing(VINS[at]);
    }
    let views = vec![
        every,
        answering(Shard::new("even").in_shard(0), second),
        Shard::new("odd").in_shard(1).skipping(),
    ];
    (log, views)
}

/// Every case of the sweep, handed to `check` with its fan-out run.
fn sweep(mut check: impl FnMut(&str, &Observed, &Observed, &Watched)) {
    use Answers::{Halt, Skip};

    let mut cases = 0;
    for size in 1..=5 {
        for at in 0..VINS.len() {
            for fault in [Fault::Decode(at), Fault::Refuse(at)] {
                for answers in [[Halt, Halt], [Halt, Skip], [Skip, Halt], [Skip, Skip]] {
                    let (log, views) = sweep_case(fault, answers);
                    let case = format!("chunk {size}, {fault:?}, {answers:?}");
                    block_on(async {
                        let events = Instrumented::seeded(&log).await;
                        let (got, models) = fanned(&events, &views, size).await;
                        let want = sequentially(&events, &views, size).await;
                        check(&case, &got, &want, &models);
                    });
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 200, "the sweep shrank");
}

#[test]
fn fan_out_equals_n_single_runs_at_every_chunk_and_failure_position() {
    sweep(|case, got, want, _| assert_eq!(got, want, "{case}"));
}

#[test]
fn every_write_set_is_committed_or_rolled_back() {
    sweep(|case, _, _, models| {
        let calls = models.calls();
        for call in &calls {
            let Call::Begin(serial) = *call else { continue };
            let ends: Vec<&Call> = calls
                .iter()
                .filter(|c| c.serial() == serial && c.ends())
                .collect();
            assert!(
                matches!(ends[..], [Call::Commit(_) | Call::Rollback(_)]),
                "{case}: write set {serial} ended as {ends:?}: {calls:?}"
            );
        }
    });
}

// ---------------------------------------------------------------------------
// A panic is not caught (O1, option B)
// ---------------------------------------------------------------------------

#[test]
fn a_panic_unwinds_out_and_nothing_commits_after_it() {
    for size in [1, 8] {
        let events = block_on(Instrumented::seeded(&[
            part("VIN1"),
            part("BOOM"),
            part("VIN2"),
        ]));
        let read = events.snapshot();
        let models = Watched::new();
        let mut views = [
            Shard::new("a"),
            Shard::new("boom").panicking_on("BOOM"),
            Shard::new("c"),
        ];

        // The observation, not control flow: the test asserts the panic
        // escapes, so something outside the runner must stop it. Unwind-safe
        // because only the stores are read afterwards, and each iteration
        // builds them fresh.
        let unwound = std::panic::catch_unwind(AssertUnwindSafe(|| {
            block_on(async {
                let _ran = run_fan_out(&events, &models, &mut views, &Json, chunk(size)).await;
            });
        }));
        assert!(
            unwound.is_err(),
            "chunk {size}: the fan-out runner caught the panic"
        );

        let calls = models.calls();
        let panicked = calls
            .iter()
            .position(|call| matches!(call, Call::Panicking(_)))
            .expect("the view panicked inside apply");
        assert!(
            !calls[panicked..]
                .iter()
                .any(|call| matches!(call, Call::Commit(_))),
            "chunk {size}: a commit after the panic: {calls:?}"
        );
        assert_eq!(models.get("boom:BOOM"), None, "chunk {size}");
        block_on(async {
            let boom = models.checkpoint_of(&views[1]).await;
            let want = if size == 1 {
                live(read[0].position)
            } else {
                Checkpoint::NeverRun
            };
            assert_eq!(
                boom, want,
                "chunk {size}: the panicking view's checkpoint moved"
            );
        });
    }
}
