//! Where the watermark lives (SY-31, #143): through the projection port,
//! under the reserved id the far side's identity names, in the caller's batch.
//!
//! The projection store is `happenstance-core`'s `MemoryProjectionStore`, whose
//! batch takes rows by an inherent `write`, standing in for a read model the
//! caller writes beside the watermark.

use happenstance_core::{
    Authority, Checkpoint, CommitError, MemoryProjectionStore, ProjectionId, ProjectionStore,
};
use happenstance_sync::runner::{Edge, LoadWatermarkError, Watermark};

use super::{FAR, LOCAL, position};

/// Rejects a watermark minted before the far side has named itself, or before
/// anything was confirmed: neither can be kept under SY-31's id.
#[test]
fn an_edge_has_a_watermark_once_it_knows_its_peer_and_a_confirmed_position() {
    let fresh: Edge<()> = Edge::new();
    let unnamed: Edge<()> = Edge::resuming(Some(position(7)), None);
    let unconfirmed: Edge<()> = Edge::new().with_responder(FAR);
    let both: Edge<()> = Edge::resuming(Some(position(7)), None).with_responder(FAR);

    assert_eq!(fresh.watermark(), None);
    assert_eq!(unnamed.watermark(), None);
    assert_eq!(unconfirmed.watermark(), None);
    assert_eq!(both.watermark(), Some(Watermark::new(FAR, position(7))));
}

/// Rejects a watermark kept under an application id, or one that two peers
/// would share.
#[test]
fn a_watermark_is_kept_under_its_peers_reserved_id() {
    let watermark = Watermark::new(FAR, position(3));

    assert_eq!(watermark.id(), ProjectionId::sync_watermark(FAR));
    assert_ne!(
        watermark.id(),
        Watermark::new(LOCAL, position(3)).id(),
        "two peers, two ids"
    );
    assert_eq!(
        (watermark.peer(), watermark.confirmed()),
        (FAR, position(3))
    );
}

/// Rejects a commit that writes the watermark apart from the caller's rows:
/// both land in one commit, and a fresh read sees both.
#[tokio::test]
async fn a_watermark_commits_in_the_callers_batch() {
    let store = MemoryProjectionStore::new();
    let watermark = Watermark::new(FAR, position(5));
    let mut batch = store.open();
    batch.write("schedule:today", 2);

    watermark
        .commit(&store, batch)
        .await
        .expect("an in-memory commit lands");

    assert_eq!(store.get("schedule:today"), Some(2));
    assert_eq!(
        store
            .checkpoint(&ProjectionId::sync_watermark(FAR))
            .await
            .expect("an in-memory read"),
        Checkpoint::Live {
            through: position(5)
        }
    );
    assert_eq!(
        Watermark::load(&store, FAR)
            .await
            .expect("an in-memory read"),
        Some(watermark)
    );
}

/// Rejects a watermark that moves backwards: the port's regression check
/// holds for it as for any projection, and the caller's rows stay out.
#[tokio::test]
async fn a_stale_watermark_is_refused_and_its_rows_with_it() {
    let store = MemoryProjectionStore::new();
    Watermark::new(FAR, position(9))
        .commit(&store, store.open())
        .await
        .expect("the first commit lands");
    let mut stale = store.open();
    stale.write("schedule:stale", 1);

    let refused = Watermark::new(FAR, position(4)).commit(&store, stale).await;

    assert!(
        matches!(
            refused,
            Err(CommitError::CheckpointRegression { current, attempted })
                if current == position(9) && attempted == position(4)
        ),
        "{refused:?}"
    );
    assert_eq!(store.get("schedule:stale"), None);
}

/// A peer never synced has no watermark, and that is not an error.
#[tokio::test]
async fn an_unwritten_watermark_loads_as_none() {
    let store = MemoryProjectionStore::new();

    let loaded = Watermark::load(&store, FAR).await;

    assert_eq!(loaded, Ok(None));
}

/// Rejects a load that reads a rebuilding checkpoint as a confirmation: the
/// runner never writes one, so it is refused rather than resumed from.
#[tokio::test]
async fn a_rebuilding_checkpoint_under_the_id_is_refused() {
    let store = MemoryProjectionStore::new();
    store
        .commit(
            store.open(),
            &ProjectionId::sync_watermark(FAR),
            position(2),
            Authority::Rebuilding,
        )
        .await
        .expect("an in-memory commit lands");

    let loaded = Watermark::load(&store, FAR).await;

    assert_eq!(
        loaded,
        Err(LoadWatermarkError::NotLive(Checkpoint::Rebuilding {
            through: position(2)
        }))
    );
}

/// A restored edge resumes from the watermark and knows its peer, so its
/// first answer from another store stops it.
#[test]
fn a_restored_edge_resumes_from_the_watermark_and_pins_its_peer() {
    let watermark = Watermark::new(FAR, position(11));

    let edge: Edge<()> = Edge::restored(watermark);

    assert_eq!(edge.status().confirmed, Some(position(11)));
    assert_eq!(edge.status().responder, Some(FAR));
    assert_eq!(edge.resume_token(), None);
    assert_eq!(edge.watermark(), Some(watermark));
}
