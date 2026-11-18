//! SY-7 and SY-10's runner half, and SY-34's log assertion, each rule against
//! the shipped [`Edge`] and the wrong runners it rejects (T-450-4, T-450-7,
//! #487).
//!
//! A rule no runner can fail is decorative. Each rule here is written over an
//! [`EdgeRunner`], so it runs against the shipped edge **and** against
//! [`MiniEdge`], a runner over the public API alone that carries one
//! [`Defect`] and is otherwise correct for the rule's stores. Each defect
//! fails its own rule and no other, and [`Defect::Absent`] passes every rule,
//! so no rule passes for strict by failing every runner. The pattern is
//! `happenstance`'s `tests/fan_out/rules.rs`.
//!
//! Every rule takes one step of a fresh edge, between a local
//! [`MemoryIngestStore`] and a far [`StoreSyncPeer`] that judges nothing, and
//! reads the two logs afterwards.

use core::num::NonZeroUsize;

use happenstance_core::{Event, EventId, EventStore, SequencePosition, SequencedEvent};
use happenstance_sync::runner::{Adjudicate, Edge};
use happenstance_sync::{
    EventGroup, IngestGroup, IngestStore, Ingested, MemoryIngestStore, PeerLimits, PushBatch,
    StoreSyncPeer, SyncPeer, WriteGroupStore,
};

use super::adjudication::{
    HUB, ORIGIN_ONE, ORIGIN_TWO, SeatRule, claim, foreign_claim, ingest_unjudged, reversals,
    reversals_of,
};
use super::probe::Shared;
use super::the_log_is_kept;

/// The far side every rule steps against: a store peer that judges nothing.
type Far = StoreSyncPeer<Shared, Shared>;

/// A runner of one edge step, under `rule` or, given `None`, under
/// [`Accept`](happenstance_sync::runner::Accept).
trait EdgeRunner {
    async fn step(
        &self,
        rule: Option<SeatRule<'_>>,
        far: &Far,
        local: &MemoryIngestStore,
    ) -> Result<Ingested, String>;
}

/// The runner this crate ships.
struct Shipped;

impl EdgeRunner for Shipped {
    async fn step(
        &self,
        rule: Option<SeatRule<'_>>,
        far: &Far,
        local: &MemoryIngestStore,
    ) -> Result<Ingested, String> {
        let stepped = match rule {
            None => Edge::new()
                .step(far, local)
                .await
                .map_err(|error| error.to_string())?,
            Some(rule) => Edge::new()
                .with_adjudicator(rule)
                .step(far, local)
                .await
                .map_err(|error| error.to_string())?,
        };
        Ok(stepped.ingested)
    }
}

// ---------------------------------------------------------------------------
// The wrong runners
// ---------------------------------------------------------------------------

/// One defect per wrong runner, otherwise correct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Defect {
    /// No defect: the positive control.
    Absent,
    /// Adjudicates what it pulls whatever it was given: SY-7's forbidden
    /// configuration, every peer an adjudicator.
    AdjudicatesUnderAccept,
    /// Applies its rule to the groups it pushes too, writing the answer
    /// locally: the far side's rule, run on the near side.
    AdjudicatesWhatItPushes,
    /// Judges each pulled group against the store alone, never with the
    /// groups before it in the batch.
    IgnoresEarlier,
    /// Lands the pulled groups unjudged, then appends their compensation as a
    /// second write (SY-2).
    CompensatesApart,
    /// On a refused push, removes the own events it offered, rolling the log
    /// back to the last acknowledged position so the two logs agree: the tidy
    /// wrong implementation SY-34 names.
    RollsBackOnRefusal,
}

/// An edge step over the public API alone, carrying one [`Defect`].
struct MiniEdge(Defect);

impl MiniEdge {
    /// Each pulled group's compensation, under the rule this runner applies.
    async fn judge(
        &self,
        rule: Option<&SeatRule<'_>>,
        groups: &[EventGroup],
    ) -> Result<Vec<Vec<Event>>, String> {
        let Some(rule) = rule else {
            return Ok(vec![Vec::new(); groups.len()]);
        };
        let mut answers = Vec::with_capacity(groups.len());
        for (at, group) in groups.iter().enumerate() {
            let earlier = if self.0 == Defect::IgnoresEarlier {
                &[]
            } else {
                groups.get(..at).expect("a prefix of the batch")
            };
            let answer = rule
                .compensation(group, earlier)
                .await
                .map_err(|error| error.to_string())?;
            answers.push(answer);
        }
        Ok(answers)
    }

    /// Ingests the pulled groups with their compensation, together or apart.
    async fn land(
        &self,
        local: &MemoryIngestStore,
        groups: &[EventGroup],
        answers: &[Vec<Event>],
    ) -> Result<Ingested, String> {
        let apart = self.0 == Defect::CompensatesApart;
        let ingest: Vec<IngestGroup<'_>> = groups
            .iter()
            .zip(answers)
            .map(|(group, answer)| {
                let compensation: &[Event] = if apart { &[] } else { answer };
                IngestGroup::new(&group.events, compensation).with_guard(group.guard.as_ref())
            })
            .collect();
        let ingested = IngestStore::ingest(local, &ingest)
            .await
            .map_err(|error| error.to_string())?;
        if apart {
            for answer in answers.iter().filter(|answer| !answer.is_empty()) {
                EventStore::append(local, answer, None)
                    .await
                    .map_err(|error| error.to_string())?;
            }
        }
        Ok(ingested)
    }

    /// Pushes the whole local log, one group per local write.
    async fn push(
        &self,
        rule: Option<&SeatRule<'_>>,
        far: &Far,
        local: &MemoryIngestStore,
    ) -> Result<(), String> {
        let page = WriteGroupStore::read_groups(local, None, NonZeroUsize::MIN.saturating_add(63))
            .await
            .map_err(|error| error.to_string())?;
        let groups: Vec<EventGroup> = page.groups.into_iter().map(EventGroup::from).collect();
        if self.0 == Defect::AdjudicatesWhatItPushes {
            for answer in self.judge(rule, &groups).await? {
                if !answer.is_empty() {
                    EventStore::append(local, &answer, None)
                        .await
                        .map_err(|error| error.to_string())?;
                }
            }
        }
        let offered: Vec<SequencePosition> = groups
            .iter()
            .flat_map(|group| &group.events)
            .filter(|event| event.id.store() == local.store_id())
            .map(|event| event.id.position())
            .collect();
        match far.push(&PushBatch::new(groups)).await {
            Ok(_) => Ok(()),
            Err(error) if self.0 == Defect::RollsBackOnRefusal => {
                let removed = local.remove(&offered);
                Err(format!("{error}, and {removed} offered events rolled back"))
            }
            Err(error) => Err(error.to_string()),
        }
    }
}

impl EdgeRunner for MiniEdge {
    async fn step(
        &self,
        rule: Option<SeatRule<'_>>,
        far: &Far,
        local: &MemoryIngestStore,
    ) -> Result<Ingested, String> {
        // The fallback the defect reaches for when it was given no rule.
        let fallback = SeatRule::new(local);
        let pulling = match (&rule, self.0) {
            (Some(rule), _) => Some(rule),
            (None, Defect::AdjudicatesUnderAccept) => Some(&fallback),
            (None, _) => None,
        };
        let pulled = far.pull(None).await.map_err(|error| error.to_string())?;
        let answers = self.judge(pulling, &pulled.batch.groups).await?;
        let ingested = self.land(local, &pulled.batch.groups, &answers).await?;
        self.push(rule.as_ref(), far, local).await?;
        Ok(ingested)
    }
}

// ---------------------------------------------------------------------------
// The stores each rule steps over
// ---------------------------------------------------------------------------

/// A far store peer over a fresh store that took `claims` by ingest, one
/// group each, and the store.
async fn far_holding(claims: &[(happenstance_core::StoreId, &[u8])]) -> (Far, Shared) {
    let store = Shared::new(super::adjudication::SPOKE);
    for (origin, data) in claims {
        ingest_unjudged(&store, &foreign_claim(*origin, data)).await;
    }
    (StoreSyncPeer::new(store.clone(), store.clone()), store)
}

/// A far store peer over an empty store whose declared batch bytes are below
/// every event's payload, so it refuses every push before writing anything:
/// the refusal `main.rs`'s SY-34 test provokes from `MemorySyncPeer`.
fn far_refusing() -> Far {
    let mut limits = PeerLimits::minimum();
    limits.max_batch_bytes = 0;
    let store = Shared::new(super::adjudication::SPOKE);
    StoreSyncPeer::with_limits(store.clone(), store, limits)
}

/// The local store, holding `claims` by ingest, one group each.
async fn local_holding(claims: &[(happenstance_core::StoreId, &[u8])]) -> MemoryIngestStore {
    let local = MemoryIngestStore::with_store_id(HUB);
    for (origin, data) in claims {
        ingest_unjudged(&local, &foreign_claim(*origin, data)).await;
    }
    local
}

fn snapshot_ids(log: &[SequencedEvent]) -> Vec<EventId> {
    log.iter().map(|row| row.id).collect()
}

// ---------------------------------------------------------------------------
// The rules
// ---------------------------------------------------------------------------

/// SY-7: an edge nobody made an adjudicator lands a contested claim it pulls
/// with no reversal.
async fn accept_writes_no_compensation(runner: &impl EdgeRunner) -> Result<(), String> {
    let local = local_holding(&[(ORIGIN_ONE, b"first")]).await;
    let (far, _) = far_holding(&[(ORIGIN_TWO, b"second")]).await;

    runner.step(None, &far, &local).await?;

    let written = reversals(&local.snapshot()).len();
    (written == 0)
        .then_some(())
        .ok_or_else(|| format!("an edge under Accept wrote {written} reversals"))
}

/// SY-2 through the hook: the adjudicator's reversal of a pulled claim lands
/// in the claim's own write group, under the local identity.
async fn compensation_lands_with_its_group(runner: &impl EdgeRunner) -> Result<(), String> {
    let local = local_holding(&[(ORIGIN_ONE, b"first")]).await;
    let (far, _) = far_holding(&[(ORIGIN_TWO, b"second")]).await;
    let pulled = foreign_claim(ORIGIN_TWO, b"second").id;

    runner
        .step(Some(SeatRule::new(&local)), &far, &local)
        .await?;

    let page = WriteGroupStore::read_groups(&local, None, NonZeroUsize::MIN.saturating_add(63))
        .await
        .map_err(|error| error.to_string())?;
    let group = page
        .groups
        .iter()
        .find(|group| group.events().iter().any(|event| event.id == pulled))
        .ok_or("the pulled claim did not land")?;
    let named = pulled.to_string().into_bytes();
    let with_it: Vec<EventId> = group
        .events()
        .iter()
        .filter(|event| event.event.data() == named.as_slice())
        .map(|event| event.id)
        .collect();
    (with_it.len() == 1 && with_it.iter().all(|id| id.store() == HUB))
        .then_some(())
        .ok_or_else(|| {
            format!(
                "the pulled claim's write group holds the reversals {with_it:?}, not one of HUB's"
            )
        })
}

/// T-450-4: two contested claims in one pulled batch earn exactly one
/// reversal, which only the groups before the second can show.
async fn a_batch_is_judged_with_its_earlier_groups(runner: &impl EdgeRunner) -> Result<(), String> {
    let local = local_holding(&[]).await;
    let (far, _) = far_holding(&[(ORIGIN_ONE, b"first"), (ORIGIN_TWO, b"second")]).await;
    let second = foreign_claim(ORIGIN_TWO, b"second").id;

    runner
        .step(Some(SeatRule::new(&local)), &far, &local)
        .await?;

    let log = local.snapshot();
    let written = reversals(&log).len();
    let of_second = reversals_of(&log, second).len();
    (written == 1 && of_second == 1)
        .then_some(())
        .ok_or_else(|| {
            format!("one batch of two claims earned {written} reversals, {of_second} of the second")
        })
}

/// SY-10: what an edge pushes is the far side's to judge, so an adjudicating
/// edge writes nothing locally for it.
async fn an_edge_judges_nothing_it_pushes(runner: &impl EdgeRunner) -> Result<(), String> {
    let local = local_holding(&[(ORIGIN_ONE, b"first"), (ORIGIN_TWO, b"second")]).await;
    let before = snapshot_ids(&local.snapshot());
    let (far, far_store) = far_holding(&[]).await;

    runner
        .step(Some(SeatRule::new(&local)), &far, &local)
        .await?;

    let after = snapshot_ids(&local.snapshot());
    let reached = far_store.0.snapshot().len();
    (after == before && reached == 2)
        .then_some(())
        .ok_or_else(|| {
            format!(
                "the push changed the local log from {before:?} to {after:?}, and the far side \
                 holds {reached} events"
            )
        })
}

/// SY-34: a spoke whose push is refused keeps its own log, every row in place,
/// through `main.rs`'s assertion.
async fn a_refused_push_keeps_the_log(runner: &impl EdgeRunner) -> Result<(), String> {
    let local = local_holding(&[(ORIGIN_ONE, b"first")]).await;
    EventStore::append(&local, &[claim(b"own")], None)
        .await
        .map_err(|error| error.to_string())?;
    let before = local.snapshot();
    let far = far_refusing();

    if runner.step(None, &far, &local).await.is_ok() {
        return Err("the far side refused nothing".to_owned());
    }

    the_log_is_kept(&before, &local.snapshot())
}

/// Which rules fail `runner`, by name.
async fn failing(runner: &impl EdgeRunner) -> Vec<&'static str> {
    let mut failed = Vec::new();
    if accept_writes_no_compensation(runner).await.is_err() {
        failed.push("accept_writes_no_compensation");
    }
    if compensation_lands_with_its_group(runner).await.is_err() {
        failed.push("compensation_lands_with_its_group");
    }
    if a_batch_is_judged_with_its_earlier_groups(runner)
        .await
        .is_err()
    {
        failed.push("a_batch_is_judged_with_its_earlier_groups");
    }
    if an_edge_judges_nothing_it_pushes(runner).await.is_err() {
        failed.push("an_edge_judges_nothing_it_pushes");
    }
    if a_refused_push_keeps_the_log(runner).await.is_err() {
        failed.push("a_refused_push_keeps_the_log");
    }
    failed
}

#[tokio::test]
async fn the_shipped_edge_passes_every_rule() {
    assert_eq!(accept_writes_no_compensation(&Shipped).await, Ok(()));
    assert_eq!(compensation_lands_with_its_group(&Shipped).await, Ok(()));
    assert_eq!(
        a_batch_is_judged_with_its_earlier_groups(&Shipped).await,
        Ok(())
    );
    assert_eq!(an_edge_judges_nothing_it_pushes(&Shipped).await, Ok(()));
    assert_eq!(a_refused_push_keeps_the_log(&Shipped).await, Ok(()));
}

#[tokio::test]
async fn a_correct_mini_edge_passes_every_rule() {
    assert_eq!(failing(&MiniEdge(Defect::Absent)).await, Vec::<&str>::new());
}

#[tokio::test]
async fn accept_writes_no_compensation_rejects_a_runner_that_adjudicates_under_accept() {
    assert_eq!(
        failing(&MiniEdge(Defect::AdjudicatesUnderAccept)).await,
        vec!["accept_writes_no_compensation"]
    );
}

#[tokio::test]
async fn compensation_lands_with_its_group_rejects_a_runner_that_compensates_apart() {
    assert_eq!(
        failing(&MiniEdge(Defect::CompensatesApart)).await,
        vec!["compensation_lands_with_its_group"]
    );
}

#[tokio::test]
async fn a_batch_is_judged_with_its_earlier_groups_rejects_a_runner_that_ignores_them() {
    assert_eq!(
        failing(&MiniEdge(Defect::IgnoresEarlier)).await,
        vec!["a_batch_is_judged_with_its_earlier_groups"]
    );
}

#[tokio::test]
async fn an_edge_judges_nothing_it_pushes_rejects_a_runner_that_adjudicates_its_push() {
    assert_eq!(
        failing(&MiniEdge(Defect::AdjudicatesWhatItPushes)).await,
        vec!["an_edge_judges_nothing_it_pushes"]
    );
}

#[tokio::test]
async fn a_refused_push_keeps_the_log_rejects_a_runner_that_rolls_back_on_refusal() {
    assert_eq!(
        failing(&MiniEdge(Defect::RollsBackOnRefusal)).await,
        vec!["a_refused_push_keeps_the_log"]
    );
}
