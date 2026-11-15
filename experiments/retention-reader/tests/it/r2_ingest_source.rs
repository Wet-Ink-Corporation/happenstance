//! R2-src, the ingest path pulling *from* a holed hub: an edge over a
//! `StoreSyncPeer` whose store is CF-27's instrument.
//!
//! Faithful for a source: a source only reads, and every read seam the store
//! peer uses (`read_groups`, membership) goes through the instrument.

use std::collections::BTreeSet;

use happenstance::{Event, EventId, EventStore, SequencePosition};
use happenstance_sync::runner::{Edge, Pending};
use happenstance_sync::{IngestStore, StoreSyncPeer};
use happenstance_testkit::block_on;
use retention_reader::holed::Holed;
use retention_reader::seams::{seams, young_twin};
use retention_reader::shared::Shared;

use crate::support::{Error, Outcome, append_each, interleaved, sync};

/// A hub of ten events, one per append, and the positions they took.
async fn hub() -> Result<(Holed, Vec<SequencePosition>), Error> {
    let hub = Holed::new(Shared::new());
    let events = interleaved()?;
    let landed = append_each(hub.inner(), events.get(..10).ok_or("twelve")?).await?;
    Ok((hub, landed))
}

/// The ids `store` holds, through the port.
async fn ids<S: EventStore>(store: &S) -> Result<BTreeSet<EventId>, Error>
where
    S::Error: 'static,
{
    Ok(seams(store, &[]).await?.held.iter().map(|e| e.id).collect())
}

/// R2-src-1. A spoke pulling a scattered hub holds exactly the retained set,
/// reports synced, and keeps a watermark with no trace of the holes below it.
/// Under ES-38 and SY-27 the spoke holds what the hub holds: no harm.
#[test]
fn a_spoke_pulling_a_scattered_hub_holds_the_retained_set_and_reports_synced() -> Outcome {
    block_on(async {
        let (hub, landed) = hub().await?;
        let scattered: Vec<_> = [1, 4, 7]
            .iter()
            .filter_map(|&i| landed.get(i).copied())
            .collect();
        hub.view().withhold(&scattered).await?;
        // Two handles onto one hub: the peer stores both halves.
        let peer = StoreSyncPeer::new(hub.clone(), hub.clone());
        let spoke = Shared::new();
        let mut edge = Edge::new();

        sync(&mut edge, &peer, &spoke).await?;

        let retained = seams(hub.view(), &[]).await?.held;
        assert_eq!(ids(&spoke).await?, ids(hub.view()).await?);
        assert!(edge.status().is_synced());
        assert_eq!(edge.status().pending, Pending::Exactly(0));
        let watermark = spoke.watermark().await?;
        assert_eq!(
            watermark.get(hub.inner().id()),
            retained.last().map(|e| e.id.position())
        );
        let probe: Vec<EventId> = ids(hub.inner()).await?.into_iter().collect();
        let spoke_seams = seams(&spoke, &probe).await?;
        let young = seams(&young_twin(&retained).await?, &probe).await?;
        assert_eq!(
            spoke_seams.shape(),
            young.shape(),
            "the port cannot tell them apart"
        );
        Ok(())
    })
}

/// R2-src-2. Withholding on the hub after the spoke pulled leaves the spoke
/// holding more than the hub, and nothing at the port reconciles it. Expected
/// under SY-1 and SY-34, and recorded.
#[test]
fn withholding_after_a_pull_leaves_the_spoke_with_more_than_the_hub() -> Outcome {
    block_on(async {
        let (hub, landed) = hub().await?;
        // Two handles onto one hub: the peer stores both halves.
        let peer = StoreSyncPeer::new(hub.clone(), hub.clone());
        let spoke = Shared::new();
        let mut edge = Edge::new();
        sync(&mut edge, &peer, &spoke).await?;

        let late: Vec<_> = [2, 5, 8]
            .iter()
            .filter_map(|&i| landed.get(i).copied())
            .collect();
        hub.view().withhold(&late).await?;
        sync(&mut edge, &peer, &spoke).await?;

        assert_eq!(ids(&spoke).await?.len(), landed.len());
        assert_eq!(ids(hub.view()).await?.len(), landed.len() - late.len());
        assert!(edge.status().is_synced());
        Ok(())
    })
}

/// R2-src-3. A purge that cuts a multi-event write group ships the remainder
/// as a group of one. SY-30's "group whole" binds groups as the store holds
/// them, which is this experiment's reading; recorded as a question for #158.
#[test]
fn a_purge_that_cuts_a_write_group_ships_the_remainder() -> Outcome {
    block_on(async {
        let hub = Holed::new(Shared::new());
        let pair = [
            Event::new("Written", &b"first"[..])?,
            Event::new("Written", &b"second"[..])?,
        ];
        let second = hub.inner().append(&pair, None).await?;
        let both = seams(hub.inner(), &[]).await?.held;
        let first = both.first().ok_or("two were appended")?;
        hub.view().withhold(&[first.position]).await?;
        // Two handles onto one hub: the peer stores both halves.
        let peer = StoreSyncPeer::new(hub.clone(), hub.clone());
        let spoke = Shared::new();
        let mut edge = Edge::new();

        let stepped = sync(&mut edge, &peer, &spoke).await?;

        let held = seams(&spoke, &[]).await?.held;
        assert_eq!(stepped.ingested.appended, 1);
        assert_eq!(
            held.iter().map(|e| e.id.position()).collect::<Vec<_>>(),
            [second]
        );
        assert_eq!(
            held.first().map(|e| e.event.data().as_ref()),
            Some(&b"second"[..])
        );
        Ok(())
    })
}
