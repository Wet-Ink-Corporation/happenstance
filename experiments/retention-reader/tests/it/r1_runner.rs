//! R1, the typed projection runner (`happenstance::run_projection`), over
//! CF-27's instrument.

use std::collections::BTreeSet;

use happenstance::{EventId, MemoryProjectionStore, SequencePosition};
use happenstance_sync::{IngestGroup, IngestStore, ReplicatedEvent};
use happenstance_testkit::block_on;
use retention_reader::domain::Counter;
use retention_reader::holed::Holed;
use retention_reader::seams::{seams, young_twin};
use retention_reader::shared::Shared;

use crate::support::{Error, Outcome, append_each, interleaved, project};

/// The positions at `indexes` into `landed`.
fn at(landed: &[SequencePosition], indexes: &[usize]) -> Result<Vec<SequencePosition>, Error> {
    indexes
        .iter()
        .map(|&i| {
            landed
                .get(i)
                .copied()
                .ok_or_else(|| "index past the log".into())
        })
        .collect()
}

/// R1-1. A rebuild over a scattered purge applies exactly the retained events,
/// checkpoints at the highest retained one, and the port answers exactly as a
/// young store holding the same events would. Smaller, and correct relative to
/// its store (ES-38): no harm.
#[test]
fn rebuild_over_a_scattered_purge_is_smaller_and_says_nothing() -> Outcome {
    block_on(async {
        let events = interleaved()?;
        let full = Shared::new();
        append_each(&full, &events).await?;
        let mut full_counter = Counter::new("claims")?;
        let full_run = project(&full, &MemoryProjectionStore::new(), &mut full_counter).await?;

        let holed = Holed::new(Shared::new());
        let landed = append_each(holed.inner(), &events).await?;
        // Claim b (indexes 1, 4, 7, 10) and claim a's second note (index 6):
        // survivors sit between the holes, and the last event survives.
        holed
            .view()
            .withhold(&at(&landed, &[1, 4, 6, 7, 10])?)
            .await?;
        let mut counter = Counter::new("claims")?;
        let run = project(holed.view(), &MemoryProjectionStore::new(), &mut counter).await?;

        let probe: Vec<EventId> = seams(holed.inner(), &[])
            .await?
            .held
            .iter()
            .map(|e| e.id)
            .collect();
        let pruned = seams(holed.view(), &probe).await?;
        let retained: BTreeSet<EventId> = pruned.held.iter().map(|e| e.id).collect();

        assert_eq!(full_run.applied, events.len());
        assert_eq!(run.applied, events.len() - 5);
        assert_eq!(run.through, pruned.held.last().map(|e| e.position));
        assert_eq!(
            counter.applied().keys().copied().collect::<BTreeSet<_>>(),
            retained
        );
        assert_eq!(counter.applied_twice().count(), 0);
        let young = seams(&young_twin(&pruned.held).await?, &probe).await?;
        assert_eq!(
            pruned.shape(),
            young.shape(),
            "the port cannot tell them apart"
        );
        Ok(())
    })
}

/// R1-2. Withholding events the runner has already applied changes nothing:
/// the next run applies only what was appended since, and nothing twice.
#[test]
fn pruning_behind_the_checkpoint_changes_nothing() -> Outcome {
    block_on(async {
        let events = interleaved()?;
        let holed = Holed::new(Shared::new());
        let models = MemoryProjectionStore::new();
        let mut counter = Counter::new("claims")?;
        let landed = append_each(holed.inner(), &events).await?;
        let before = project(holed.view(), &models, &mut counter).await?;
        assert_eq!(before.applied, events.len());

        holed
            .view()
            .withhold(&at(&landed, &[0, 1, 2, 3, 4, 5])?)
            .await?;
        let more = append_each(holed.inner(), events.get(..2).ok_or("twelve")?).await?;
        let run = project(holed.view(), &models, &mut counter).await?;

        let all = seams(holed.inner(), &[]).await?;
        assert_eq!(run.applied, 2);
        assert_eq!(run.through, more.last().copied());
        assert_eq!(
            counter.applied().len(),
            all.held.len(),
            "the full log's view"
        );
        assert_eq!(counter.applied_twice().count(), 0);
        Ok(())
    })
}

/// R1-3. Events withheld above the checkpoint are never delivered, and the
/// checkpoint passes them. Under ES-38 the store never held them: no harm.
#[test]
fn pruning_ahead_of_the_checkpoint_is_never_delivered() -> Outcome {
    block_on(async {
        let events = interleaved()?;
        let (first, rest) = events.split_at(6);
        let holed = Holed::new(Shared::new());
        let models = MemoryProjectionStore::new();
        let mut counter = Counter::new("claims")?;
        let early = append_each(holed.inner(), first).await?;
        let before = project(holed.view(), &models, &mut counter).await?;
        assert_eq!(before.through, early.last().copied());

        let later = append_each(holed.inner(), rest).await?;
        let gone = at(&later, &[1, 3])?;
        holed.view().withhold(&gone).await?;
        let run = project(holed.view(), &models, &mut counter).await?;

        let all = seams(holed.inner(), &[]).await?;
        let gone_ids: BTreeSet<EventId> = all
            .held
            .iter()
            .filter(|e| gone.contains(&e.position))
            .map(|e| e.id)
            .collect();
        assert_eq!(run.applied, rest.len() - 2);
        assert_eq!(run.through, later.last().copied());
        assert!(run.through > gone.last().copied());
        assert!(counter.applied().keys().all(|id| !gone_ids.contains(id)));
        assert_eq!(counter.applied_twice().count(), 0);
        Ok(())
    })
}

/// R1-4. **A recorded hazard, not a desired behaviour.** One projection, one
/// projection store, two event stores: the checkpoint store A left is resumed
/// against store B, whose positions mean something else (ES-29: a position is
/// meaningful only within the store that assigned it). B's own events at or
/// below A's checkpoint are skipped silently, and an event already applied
/// from A, which sits above it in B, is applied again.
///
/// Nothing in the runner stops it, and no completeness report would: both
/// stores are complete. The two stores' identities differ, and each is
/// reachable today (`IngestStore::store_id`, the origin in every `EventId`);
/// the checkpoint records no source to compare them with.
#[test]
fn a_source_switch_resumes_against_the_wrong_store() -> Outcome {
    block_on(async {
        let events = interleaved()?;
        let a = Shared::new();
        append_each(&a, events.get(..8).ok_or("twelve")?).await?;
        let models = MemoryProjectionStore::new();
        let mut counter = Counter::new("claims")?;
        let on_a = project(&a, &models, &mut counter).await?;

        // B: four events of its own, then the last five of A's, ingested as a
        // rebuild-from-the-hub source would hold them.
        let b = Shared::new();
        let local = append_each(&b, events.get(8..).ok_or("twelve")?).await?;
        let from_a = seams(&a, &[]).await?.held;
        let shipped: Vec<ReplicatedEvent> = from_a
            .iter()
            .skip(3)
            // B stores its own copy of each: genuine duplication.
            .map(|e| ReplicatedEvent::new(e.id, e.recorded_at, e.event.clone()))
            .collect();
        let groups: Vec<IngestGroup<'_>> = shipped
            .chunks(1)
            .map(|one| IngestGroup::new(one, &[]))
            .collect();
        b.ingest(&groups).await?;
        let on_b = project(&b, &models, &mut counter).await?;

        let b_held = seams(&b, &[]).await?.held;
        let local_ids: Vec<EventId> = b_held
            .iter()
            .filter(|e| local.contains(&e.position))
            .map(|e| e.id)
            .collect();
        let replayed: Vec<EventId> = b_held
            .iter()
            .filter(|e| Some(e.position) > on_a.through)
            .map(|e| e.id)
            .collect();
        assert_eq!(on_a.applied, 8);
        assert_eq!(local_ids.len(), 4);
        assert!(
            local_ids
                .iter()
                .all(|id| !counter.applied().contains_key(id)),
            "B's own events were skipped"
        );
        assert_eq!(on_b.applied, replayed.len());
        assert_eq!(
            counter.applied_twice().collect::<Vec<_>>(),
            replayed,
            "applied twice"
        );
        assert!(!replayed.is_empty());
        assert_ne!(
            IngestStore::store_id(&a).await?,
            IngestStore::store_id(&b).await?,
            "the identity that would have told it is reachable today"
        );
        Ok(())
    })
}
