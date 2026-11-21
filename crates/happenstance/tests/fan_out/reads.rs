//! A refused checkpoint read, of the pin or of the main key, through both
//! runners: `ProjectionError::Checkpoint`, and no write set opened after it.
//!
//! The fault is the testkit's own: `failing_checkpoint_reads_of` arms a
//! second handle onto a recording store, so a test can set state up through
//! the unarmed handle and meet it through the armed one, with one call log.

use happenstance::{
    EventStore, Json, MemoryEventStore, ProjectionError, ViewError, checkpoint_id, run_fan_out,
    run_projection, source_checkpoint_id,
};
use happenstance_testkit::{BatchCall, RecordingProjectionStore, RecordingStoreError, block_on};

use crate::support::{Shard, chunk, part};

/// A store holding one part.
async fn one_part() -> MemoryEventStore {
    let events = MemoryEventStore::new();
    EventStore::append(&events, &[part("VIN1")], None)
        .await
        .expect("the fixture seeds");
    events
}

/// How many write sets the log says were opened.
fn begins(models: &RecordingProjectionStore) -> usize {
    models
        .calls()
        .iter()
        .filter(|call| matches!(call, BatchCall::Begin(_)))
        .count()
}

/// The pin's read is refused on a fresh view: `Checkpoint`, and not one
/// `begin`, so nothing is pinned and nothing applied.
#[test]
fn a_refused_pin_read_is_checkpoint_and_opens_nothing() {
    block_on(async {
        let events = one_part().await;
        let mut view = Shard::new("solo");
        let pin = source_checkpoint_id(&view).expect("a derivable id");
        let models = RecordingProjectionStore::new().failing_checkpoint_reads_of(pin);

        let error = run_projection((&events).into(), &models, &mut view, &Json, chunk(8))
            .await
            .expect_err("the pin cannot be read");

        assert!(
            matches!(
                error,
                ProjectionError::Checkpoint(RecordingStoreError::Injected)
            ),
            "{error:?}"
        );
        assert_eq!(models.calls(), [], "no write set was opened");
        assert_eq!(view.applied, 0);
    });
}

/// The main checkpoint's read is refused on a view already pinned to this
/// store: `Checkpoint`, and no chunk's write set opened after it.
#[test]
fn a_refused_main_read_is_checkpoint_and_opens_no_chunk() {
    block_on(async {
        let events = one_part().await;
        let models = RecordingProjectionStore::new();
        let mut first = Shard::new("solo");
        let ran = run_projection((&events).into(), &models, &mut first, &Json, chunk(8))
            .await
            .expect("the first run pins the store and applies");
        assert_eq!(ran.applied, 1);
        let before = models.calls();
        let mut view = first.fresh();
        let main = checkpoint_id(&view).expect("a derivable id");
        let armed = models.failing_checkpoint_reads_of(main);

        let error = run_projection((&events).into(), &armed, &mut view, &Json, chunk(8))
            .await
            .expect_err("the main checkpoint cannot be read");

        assert!(
            matches!(
                error,
                ProjectionError::Checkpoint(RecordingStoreError::Injected)
            ),
            "{error:?}"
        );
        assert_eq!(models.calls(), before, "nothing opened after the pin");
        assert_eq!(view.applied, 0);
    });
}

/// In a fan-out, one view's refused pin read stops that view alone, having
/// opened nothing for it, and the view beside it runs.
#[test]
fn a_refused_read_stops_only_its_view_in_a_fan_out() {
    block_on(async {
        let events = one_part().await;
        let mut views = [Shard::new("a"), Shard::new("b")];
        let [refused, _] = &views;
        let pin = source_checkpoint_id(refused).expect("a derivable id");
        let models = RecordingProjectionStore::new().failing_checkpoint_reads_of(pin);

        let fan = run_fan_out((&events).into(), &models, &mut views, &Json, chunk(8)).await;

        let [stopped, other] = fan.views.as_slice() else {
            panic!("one outcome per view: {:?}", fan.views);
        };
        assert!(
            matches!(
                stopped,
                Err(ViewError::Stopped(ProjectionError::Checkpoint(
                    RecordingStoreError::Injected
                )))
            ),
            "{stopped:?}"
        );
        let ran = other.as_ref().expect("the other view runs");
        assert_eq!(ran.applied, 1);
        let [a, _] = &views;
        assert_eq!(a.applied, 0);
        assert_eq!(models.get("a:VIN1"), None);
        assert_eq!(models.get("b:VIN1"), Some(1));
        assert_eq!(
            begins(&models),
            2,
            "b's pin and b's chunk, and nothing for a"
        );
    });
}
