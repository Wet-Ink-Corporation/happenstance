//! PS-40 through both runners: the source pin is recorded, kept, adopted,
//! reset and refused (E2E-59).
//!
//! The rule itself, `a_checkpoint_is_not_resumed_against_another_store`,
//! and the wrong runners it rejects are in `tests/projection_runner.rs`.
//! What is here is the pin's life: when it is written, what a failure to
//! write it reports, how a checkpoint an earlier release left is adopted,
//! and how a refusal stays inside one view of a fan-out.

use happenstance::{
    Checkpoint, Event, EventStore, Json, MemoryEventStore, Progressed, ProjectionError,
    ProjectionStore, SequencePosition, SourceMark, ViewError, checkpoint_id, run_fan_out,
    run_projection, source_checkpoint_id,
};
use happenstance_testkit::{BatchCall, RecordingProjectionStore, block_on};

use crate::support::{Shard, checkpoint_of, chunk, part};

/// A memory store holding `log`.
async fn store_with(log: &[Event]) -> MemoryEventStore {
    let events = MemoryEventStore::new();
    if !log.is_empty() {
        EventStore::append(&events, log, None)
            .await
            .expect("the fixture seeds");
    }
    events
}

/// What `view`'s pin reads.
async fn pin_of(models: &RecordingProjectionStore, view: &Shard) -> Checkpoint {
    let pin = source_checkpoint_id(view).expect("a derivable id");
    models.checkpoint(&pin).await.expect("readable")
}

/// What a pin reads once it records `events`.
fn pinned_to(events: &MemoryEventStore) -> Checkpoint {
    let mark = SourceMark::of(events.store_id());
    Checkpoint::Live {
        through: SequencePosition::new(mark.get()).expect("a mark is a position"),
    }
}

#[test]
fn a_first_run_records_its_source() {
    block_on(async {
        let events = store_with(&[part("VIN1"), part("VIN2")]).await;
        let models = RecordingProjectionStore::new();
        let mut view = Shard::new("solo");

        let done = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the run reaches the end");

        assert_eq!(done.applied, 2);
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
    });
}

/// Invariant 2: the same store, resumed, behaves as `0.7` did.
#[test]
fn the_same_source_resumes_unchanged() {
    block_on(async {
        let events = store_with(&[part("VIN1"), part("VIN2")]).await;
        let models = RecordingProjectionStore::new();
        let mut view = Shard::new("solo");
        let _first = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the first run reaches the end");
        let third = EventStore::append(&events, &[part("VIN3")], None)
            .await
            .expect("the tail appends");

        let resumed = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the resumed run reaches the end");

        assert_eq!((resumed.applied, resumed.through), (1, Some(third)));
        assert_eq!(
            checkpoint_of(&models, &view).await,
            Checkpoint::Live { through: third }
        );
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
        assert_eq!(models.get("solo:VIN3"), Some(3));
    });
}

/// W5: the pin is durable before the first chunk commits, so a run whose
/// first chunk fails has still recorded its store.
#[test]
fn the_pin_is_durable_before_the_first_chunk() {
    block_on(async {
        let events = store_with(&[part("VIN1")]).await;
        let mut view = Shard::new("solo");
        let key = checkpoint_id(&view).expect("a derivable id");
        let models = RecordingProjectionStore::failing_commits_under(key);

        let error = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect_err("the chunk's commit is refused");

        assert!(matches!(error, ProjectionError::Commit { .. }), "{error:?}");
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
        assert_eq!(checkpoint_of(&models, &view).await, Checkpoint::NeverRun);
    });
}

/// The eager pin, recorded: a fresh run with nothing to apply still pins,
/// and touches the store for nothing else.
#[test]
fn an_idle_fresh_run_still_pins() {
    block_on(async {
        let events = store_with(&[]).await;
        let models = RecordingProjectionStore::new();
        let mut view = Shard::new("solo");

        let done = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("an idle run completes");

        assert_eq!(done, Progressed::default());
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
        assert_eq!(models.calls(), [BatchCall::Begin(0), BatchCall::Commit(0)]);
        assert!(models.is_empty(), "a pin wrote a row");
    });
}

/// 485-Q3: a checkpoint written by `0.7`, with no pin, is adopted for the
/// store its first `0.8` run reads, and resumed rather than rebuilt.
#[test]
fn a_0_7_checkpoint_is_adopted_on_first_run() {
    block_on(async {
        let events = store_with(&[part("VIN1"), part("VIN2")]).await;
        let models = RecordingProjectionStore::new();
        let mut view = Shard::new("solo");
        // What `0.7` left: the rows and the main checkpoint, and no pin.
        let last = events.last_position().expect("two events");
        let mut batch = models.begin().await.expect("a buffered begin");
        batch.write("solo:VIN2", 2);
        let key = checkpoint_id(&view).expect("a derivable id");
        models
            .commit(batch, &key, last, happenstance::Authority::Live)
            .await
            .expect("the 0.7 shape commits");
        let third = EventStore::append(&events, &[part("VIN3")], None)
            .await
            .expect("the tail appends");

        let resumed = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the adopting run reaches the end");

        assert_eq!((resumed.applied, resumed.through), (1, Some(third)));
        assert_eq!(models.get("solo:VIN1"), None, "an adopted view replayed");
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
    });
}

/// The documented remedy, executed: a refusal, the pin reset, and the next
/// run adopting the store it is handed.
#[test]
fn resetting_the_pin_re_adopts() {
    block_on(async {
        let a = store_with(&[part("VIN1")]).await;
        let b = store_with(&[part("VIN1"), part("VIN2")]).await;
        let models = RecordingProjectionStore::new();
        let mut view = Shard::new("solo");
        let _built = run_projection((&a).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the run on A reaches the end");
        let refused = run_projection((&b).into(), &models, &mut view, &Json, chunk(8)).await;
        assert!(
            matches!(refused, Err(ProjectionError::SourceChanged { .. })),
            "{refused:?}"
        );

        let pin = source_checkpoint_id(&view).expect("a derivable id");
        let empty = models.begin().await.expect("a buffered begin");
        models.reset(empty, &pin).await.expect("the pin resets");
        let adopted = run_projection((&b).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect("the run on B adopts it");

        assert_eq!(adopted.applied, 1, "resumed past A's checkpoint in B");
        assert_eq!(pin_of(&models, &view).await, pinned_to(&b));
    });
}

/// The pin's empty write set is refused: `Begin`, with nothing read.
#[test]
fn a_refused_pin_begin_is_begin_with_no_progress() {
    block_on(async {
        let events = store_with(&[part("VIN1")]).await;
        let models = RecordingProjectionStore::failing_begin(0);
        let mut view = Shard::new("solo");

        let error = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect_err("the pin's begin is refused");

        assert!(matches!(error, ProjectionError::Begin { .. }), "{error:?}");
        assert_eq!(error.progress(), Progressed::default());
        assert_eq!(view.applied, 0);
        assert_eq!(pin_of(&models, &view).await, Checkpoint::NeverRun);
    });
}

/// A chunk's `begin` refused after the pin and one committed chunk is still
/// `Begin`, but carries the chunk the run committed, which is how it differs
/// from a refused pin begin.
///
/// Begin 0 is the pin's, 1 the first chunk's and 2 the second's.
#[test]
fn a_refused_chunk_begin_is_begin_with_the_committed_progress() {
    block_on(async {
        let events = store_with(&[]).await;
        let first = EventStore::append(&events, &[part("VIN1")], None)
            .await
            .expect("the first event appends");
        EventStore::append(&events, &[part("VIN2")], None)
            .await
            .expect("the second event appends");
        let models = RecordingProjectionStore::failing_begin(2);
        let mut view = Shard::new("solo");

        let error = run_projection((&events).into(), &models, &mut view, &Json, chunk(1))
            .await
            .expect_err("the second chunk's begin is refused");

        let ProjectionError::Begin { progress, .. } = error else {
            panic!("a chunk begin: {error:?}");
        };
        assert_eq!(
            (progress.through, progress.applied, progress.skipped),
            (Some(first), 1, 0)
        );
        assert_eq!(pin_of(&models, &view).await, pinned_to(&events));
        assert_eq!(models.get("solo:VIN2"), None, "no write set, no row");
    });
}

/// The pin's commit is refused: `RecordSource`, with nothing read.
#[test]
fn a_refused_pin_commit_is_record_source() {
    block_on(async {
        let events = store_with(&[part("VIN1")]).await;
        let mut view = Shard::new("solo");
        let pin = source_checkpoint_id(&view).expect("a derivable id");
        let models = RecordingProjectionStore::failing_commits_under(pin);

        let error = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect_err("the pin's commit is refused");

        assert!(
            matches!(error, ProjectionError::RecordSource(_)),
            "{error:?}"
        );
        assert_eq!(error.progress(), Progressed::default());
        assert_eq!(error.position(), None);
        assert_eq!(view.applied, 0);
        assert_eq!(checkpoint_of(&models, &view).await, Checkpoint::NeverRun);
    });
}

/// Invariant 6: one view's refusal in a fan-out stops that view alone, and
/// the view beside it runs and pins the store it was handed.
#[test]
fn a_source_switch_stops_only_that_view() {
    block_on(async {
        let a = store_with(&[part("VIN1")]).await;
        let b = store_with(&[part("VIN1"), part("VIN2")]).await;
        let models = RecordingProjectionStore::new();
        let mut built = Shard::new("built");
        let _on_a = run_projection((&a).into(), &models, &mut built, &Json, chunk(8))
            .await
            .expect("the run on A reaches the end");

        let mut views = [built, Shard::new("fresh")];
        let fan = run_fan_out((&b).into(), &models, &mut views, &Json, chunk(8)).await;

        assert!(fan.read.is_none(), "the memory read failed");
        assert!(
            matches!(
                &fan.views[0],
                Err(ViewError::Stopped(ProjectionError::SourceChanged { found, .. }))
                    if *found == b.store_id()
            ),
            "{:?}",
            fan.views[0]
        );
        let fresh = fan.views[1].as_ref().expect("the fresh view runs");
        assert_eq!(fresh.applied, 2);
        assert_eq!(pin_of(&models, &views[0]).await, pinned_to(&a));
        assert_eq!(pin_of(&models, &views[1]).await, pinned_to(&b));
        assert_eq!(models.get("built:VIN2"), None, "the refused view applied");
    });
}

/// Every view a fan-out runs is pinned, including one that nominates
/// nothing.
#[test]
fn fan_out_pins_every_view_it_runs() {
    block_on(async {
        let events = store_with(&[part("VIN1")]).await;
        let models = RecordingProjectionStore::new();
        let mut views = [Shard::new("busy"), Shard::new("idle").in_shard(9)];

        let fan = run_fan_out((&events).into(), &models, &mut views, &Json, chunk(8)).await;

        assert!(fan.views.iter().all(Result::is_ok), "{:?}", fan.views);
        for view in &views {
            assert_eq!(pin_of(&models, view).await, pinned_to(&events));
        }
    });
}
