//! What `RecordingProjectionStore` tells apart, and the failures it injects.
//!
//! The instrument exists for the distinction the memory store cannot draw: a
//! write set rolled back and one dropped bare leave the same rows behind.
//! Each test pins one entry of the log against the call that produced it.

#![cfg(feature = "memory")]

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, SequencePosition,
};
use happenstance_testkit::{BatchCall, RecordingProjectionStore, RecordingStoreError, block_on};

fn at(n: u64) -> SequencePosition {
    SequencePosition::new(n).expect("a non-zero position")
}

#[test]
fn a_rollback_and_a_bare_drop_are_logged_apart() {
    let store = RecordingProjectionStore::new();
    block_on(async {
        let mut rolled = store.begin().await.expect("the memory store opens");
        rolled.write("rolled", 1);
        store.rollback(rolled).await.expect("a buffer drops");
        let mut dropped = store.begin().await.expect("the memory store opens");
        dropped.write("dropped", 1);
        drop(dropped);
    });
    assert!(store.is_empty(), "neither write set committed");
    assert_eq!(
        store.calls(),
        [
            BatchCall::Begin(0),
            BatchCall::Rollback(0),
            BatchCall::Begin(1),
            BatchCall::Dropped(1),
        ]
    );
}

#[test]
fn a_mark_lands_between_the_calls_around_it() {
    let store = RecordingProjectionStore::new();
    let id = ProjectionId::from_static("marked");
    block_on(async {
        let batch = store.begin().await.expect("the memory store opens");
        batch.mark();
        store
            .commit(batch, &id, at(1), Authority::Live)
            .await
            .expect("the memory store commits");
    });
    let calls = store.calls();
    assert_eq!(
        calls,
        [
            BatchCall::Begin(0),
            BatchCall::Marked(0),
            BatchCall::Commit(0)
        ]
    );
    assert!(calls.iter().all(|call| call.serial() == 0));
    assert_eq!(
        calls.iter().map(|call| call.ends()).collect::<Vec<_>>(),
        [false, false, true]
    );
}

#[test]
fn the_nth_begin_fails_and_opens_nothing() {
    let store = RecordingProjectionStore::failing_begin(1);
    block_on(async {
        let first = store.begin().await.expect("the zeroth begin succeeds");
        let second = store.begin().await;
        assert!(matches!(second, Err(RecordingStoreError::Injected)));
        let third = store.begin().await.expect("only the first begin fails");
        store.rollback(first).await.expect("a buffer drops");
        store.rollback(third).await.expect("a buffer drops");
    });
    assert_eq!(
        store.calls(),
        [
            BatchCall::Begin(0),
            BatchCall::Begin(1),
            BatchCall::Rollback(0),
            BatchCall::Rollback(1),
        ],
        "a failed begin takes no serial and logs nothing"
    );
}

#[test]
fn commits_under_one_id_fail_and_others_land() {
    let refused = ProjectionId::from_static("refused");
    let kept = ProjectionId::from_static("kept");
    let store = RecordingProjectionStore::failing_commits_under(refused.clone());
    block_on(async {
        let mut batch = store.begin().await.expect("the memory store opens");
        batch.write("refused", 1);
        let failed = store.commit(batch, &refused, at(1), Authority::Live).await;
        assert!(matches!(
            failed,
            Err(CommitError::Store(RecordingStoreError::Injected))
        ));
        let mut batch = store.begin().await.expect("the memory store opens");
        batch.write("kept", 1);
        store
            .commit(batch, &kept, at(1), Authority::Live)
            .await
            .expect("another id commits");
        assert_eq!(
            store.checkpoint(&refused).await.expect("a memory read"),
            Checkpoint::NeverRun
        );
    });
    assert_eq!(store.get("refused"), None, "the failed commit kept nothing");
    assert_eq!(store.get("kept"), Some(1));
    assert_eq!(
        store.calls(),
        [
            BatchCall::Begin(0),
            BatchCall::Commit(0),
            BatchCall::Begin(1),
            BatchCall::Commit(1),
        ]
    );
}
