//! SY-32's log, its staged pulls and its checks, kept out of `sync_suite.rs`
//! so the rule there reads as what it checks.
//!
//! The case is the vessel away 120 days against a hub that keeps 90. Every
//! event is recorded on a test clock, `T0` plus whole days, and the rule calls
//! the fixture's compaction explicitly: nothing waits (TST-10).

use alloc::format;
use alloc::vec::Vec;

use happenstance_core::{Event, EventId, EventType, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{Classify, EventGroup, FailureClass, PushBatch, ReplicatedEvent, SyncPeer};

use super::MAX_PULLS;
use crate::SyncFixture;
use crate::violation::{Violation, ensure};

const SY_32: &str = "SY-32";

/// The shore station whose log the far side holds.
const SHORE: StoreId = StoreId::from_bytes([0x5A; 16]);
const LOGGED: EventType = EventType::from_static("PositionLogged");

const T0: i64 = 1_800_000_000_000;
const DAY_MS: i64 = 86_400_000;
/// "Now", when the far side compacts.
const TODAY: i64 = 130;
/// The far side's retention window.
const WINDOW_DAYS: i64 = 90;

/// The days of stage A, which the stale token has seen: the vessel that went
/// offline on day 10. The day-10 group carries two events (SY-30).
const STAGE_A: &[&[i64]] = &[&[0], &[10, 10]];
/// The days of stage B, which the recent token has seen too: the vessel that
/// last synced on day 70.
const STAGE_B: &[&[i64]] = &[&[20], &[50], &[70]];
/// The days written after both tokens.
const STAGE_C: &[&[i64]] = &[&[100], &[125]];

fn day(day: i64) -> RecordedAt {
    RecordedAt::from_millis(T0 + day * DAY_MS)
}

/// One stage's groups, the shore's positions numbered on from `next`.
fn stage(days: &[&[i64]], next: &mut u64) -> Result<Vec<EventGroup>, Violation> {
    days.iter()
        .map(|group| {
            let events = group
                .iter()
                .map(|&on| {
                    let position = SequencePosition::new(*next)
                        .ok_or_else(|| Violation::new(SY_32, "the rule's position is zero"))?;
                    *next += 1;
                    let data = format!("sy32 {position} day {on}").into_bytes();
                    let event = Event::new(LOGGED, data).map_err(|error| {
                        Violation::new(SY_32, format!("the rule's own event is invalid: {error}"))
                    })?;
                    Ok(ReplicatedEvent::new(
                        EventId::new(SHORE, position),
                        day(on),
                        event,
                    ))
                })
                .collect::<Result<Vec<_>, Violation>>()?;
            Ok(EventGroup::new(None, events))
        })
        .collect()
}

/// Pushes `groups`, one push each, as the shore wrote them.
async fn push_each<P: SyncPeer>(peer: &P, groups: &[EventGroup]) -> Result<(), Violation> {
    for group in groups {
        peer.push(&PushBatch::new(alloc::vec![group.clone()]))
            .await
            .map_err(|error| {
                Violation::new(
                    SY_32,
                    format!("the peer refused a one-group push of the rule's log: {error}"),
                )
            })?;
    }
    Ok(())
}

/// Pulls from `from` until an empty batch, and answers what came back and
/// the token to resume from next.
async fn pull_to_the_end<P: SyncPeer>(
    peer: &P,
    from: Option<P::Resume>,
    what: &str,
) -> Result<(Vec<ReplicatedEvent>, P::Resume), Violation> {
    let mut events = Vec::new();
    let mut resume = from;
    for _ in 0..MAX_PULLS {
        let pulled = peer.pull(resume.as_ref()).await.map_err(|error| {
            Violation::new(
                SY_32,
                format!(
                    "the peer refused {what} (class {:?}): {error}; a token at or above the \
                     floor must resume",
                    error.class()
                ),
            )
        })?;
        let done = pulled.batch.groups.is_empty();
        events.extend(
            pulled
                .batch
                .groups
                .into_iter()
                .flat_map(|group| group.events),
        );
        if done {
            return Ok((events, pulled.resume));
        }
        resume = Some(pulled.resume);
    }
    Err(Violation::new(
        SY_32,
        format!("the peer was still handing back groups after {MAX_PULLS} pulls"),
    ))
}

/// What the rule checks, in order, against one handle of a fixture that
/// compacts. The caller has checked the capability.
pub(super) async fn reported_not_silent<F: SyncFixture>(
    fixture: &F,
    peer: &F::Peer,
) -> Result<(), Violation> {
    let mut next = 1;
    let stage_a = stage(STAGE_A, &mut next)?;
    let stage_b = stage(STAGE_B, &mut next)?;
    let stage_c = stage(STAGE_C, &mut next)?;

    push_each(peer, &stage_a).await?;
    let (_, stale) = pull_to_the_end(peer, None, "a pull from the start").await?;
    push_each(peer, &stage_b).await?;
    let (_, recent) =
        pull_to_the_end(peer, Some(stale.clone()), "a pull before compaction").await?;
    push_each(peer, &stage_c).await?;

    let cutoff = day(TODAY - WINDOW_DAYS);
    fixture.compact_recorded_before(peer, cutoff).await?;

    reported(peer, [&stage_a, &stage_b], cutoff)?;
    not_silent(peer, Some(&stale), "the stale token").await?;
    // c. The start is not a back door: an edge that lost its token, or a
    // runner that drops it on a stop, would converge on the retained subset.
    not_silent(peer, None, "a pull from the start").await?;
    resumable(peer, recent, &stage_c).await
}

/// a. The floor is reported, and names an event recorded before the cutoff.
fn reported<P: SyncPeer>(
    peer: &P,
    stages: [&[EventGroup]; 2],
    cutoff: RecordedAt,
) -> Result<(), Violation> {
    let forgotten: Vec<EventId> = stages
        .into_iter()
        .flatten()
        .flat_map(|group| &group.events)
        .filter(|event| event.recorded_at.as_millis() < cutoff.as_millis())
        .map(|event| event.id)
        .collect();
    let floor = peer.limits().retention_floor.ok_or_else(|| {
        Violation::new(
            SY_32,
            "the peer compacted and declares no retention floor: a runner reading its limits \
             cannot report what is gone",
        )
    })?;
    ensure(forgotten.contains(&floor), SY_32, || {
        format!(
            "the peer declares the floor {floor}, which is not an event it was asked to \
             forget ({forgotten:?})"
        )
    })
}

/// b and c. A pull from below the floor, `from` a stale token or from the
/// start, is refused with class `Reseed`.
async fn not_silent<P: SyncPeer>(
    peer: &P,
    from: Option<&P::Resume>,
    what: &str,
) -> Result<(), Violation> {
    match peer.pull(from).await {
        Ok(pulled) => Err(Violation::new(
            SY_32,
            format!(
                "the peer answered {what}, below its floor, with {} events: a strict subset, \
                 in silence",
                pulled.batch.events().count()
            ),
        )),
        Err(error) => ensure(error.class() == FailureClass::Reseed, SY_32, || {
            format!(
                "the peer refused {what}, below its floor, as {:?}: a runner retries Transient \
                 for ever and treats Stop as configuration; a gap is Reseed",
                error.class()
            )
        }),
    }
}

/// d. A token above the floor resumes, with exactly what was written after
/// it, each event whole and once.
async fn resumable<P: SyncPeer>(
    peer: &P,
    recent: P::Resume,
    written_after: &[EventGroup],
) -> Result<(), Violation> {
    let (mut pulled, _) = pull_to_the_end(
        peer,
        Some(recent),
        "a token above the floor, minted after day 70",
    )
    .await?;
    let mut expected: Vec<ReplicatedEvent> = written_after
        .iter()
        .flat_map(|group| group.events.iter().cloned())
        .collect();
    pulled.sort_by_key(|event| event.id);
    expected.sort_by_key(|event| event.id);
    ensure(pulled == expected, SY_32, || {
        format!(
            "a token above the floor resumed with {pulled:?}; it must hand back exactly what was \
             written after it, {expected:?}"
        )
    })
}
