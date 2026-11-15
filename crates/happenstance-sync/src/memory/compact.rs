//! [`MemorySyncPeer`] forgetting a prefix of its log, as a peer with a
//! retention window does (SY-32). Its own file because `memory.rs` is the
//! peer's port and this is the stand-in for an adapter's operations side.

use core::num::NonZeroUsize;

use happenstance_core::{EventId, RecordedAt};

use super::{MemoryPeerError, MemorySyncPeer, PeerState};

/// What compaction has dropped from the front of a [`MemorySyncPeer`]'s log.
///
/// One value rather than a count beside an `Option<EventId>`, so a peer that
/// has dropped groups always knows its floor, and one that knows a floor has
/// always dropped a group (API-02).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Compaction {
    /// How many groups have been dropped from the front, counted from the
    /// start of the log as it was ever written: a `MemoryResume` minted before
    /// a compaction still means what it meant.
    pub(super) groups: NonZeroUsize,
    /// The newest event dropped: the event a resume token must have reached.
    pub(super) floor: EventId,
}

impl Compaction {
    /// Where the absolute group index `next` sits in what is retained, or the
    /// floor it fell below.
    pub(super) fn locate(compaction: Option<Self>, next: usize) -> Result<usize, EventId> {
        match compaction {
            None => Ok(next),
            Some(compaction) => next
                .checked_sub(compaction.groups.get())
                .ok_or(compaction.floor),
        }
    }

    /// How many groups have been dropped: zero before any compaction.
    pub(super) fn dropped(compaction: Option<Self>) -> usize {
        compaction.map_or(0, |compaction| compaction.groups.get())
    }
}

impl MemorySyncPeer {
    /// Drops the longest prefix of this peer's log in which every event was
    /// recorded before `cutoff`, as a peer compacting on a retention window
    /// does, and answers the floor afterwards.
    ///
    /// **Only a prefix.** A group recorded before `cutoff` that sits after a
    /// newer one stays, because the floor describes resumability, not
    /// completeness (SY-32, ADR-0028). `cutoff` is read as a window predicate
    /// on each event's [`RecordedAt`], which never orders events: the prefix
    /// is in this peer's forwarding order.
    ///
    /// The floor is the identity of the newest event dropped, and it only
    /// moves forward: a cutoff that drops nothing more answers the floor as it
    /// was. A pull from a token that has not consumed the dropped groups is
    /// then refused with [`MemoryPeerError::BelowRetentionFloor`], class
    /// [`Reseed`](crate::FailureClass::Reseed), and
    /// [`limits`](crate::SyncPeer::limits) reports the floor. A prefix of
    /// groups with no events forgets no history, and is kept.
    ///
    /// Dropped identities are forgotten, as a physical prune forgets them, so
    /// a later push of one appends it again; [`Ack::confirmed`](crate::Ack::confirmed)
    /// does not move back.
    ///
    /// # Errors
    ///
    /// [`MemoryPeerError::Poisoned`] if the log lock was poisoned.
    pub fn compact_recorded_before(
        &self,
        cutoff: RecordedAt,
    ) -> Result<Option<EventId>, MemoryPeerError> {
        let mut state = self.state.lock().map_err(|_| MemoryPeerError::Poisoned)?;
        let PeerState {
            groups,
            held,
            compaction,
            ..
        } = &mut *state;
        let prior = *compaction;
        let expired = groups
            .iter()
            .take_while(|group| {
                group
                    .events
                    .iter()
                    .all(|event| event.recorded_at.as_millis() < cutoff.as_millis())
            })
            .count();
        let Some(count) = NonZeroUsize::new(expired) else {
            return Ok(prior.map(|compaction| compaction.floor));
        };
        let newest = groups
            .iter()
            .take(expired)
            .rev()
            .find_map(|group| group.events.last())
            .map(|event| event.id);
        let Some(floor) = newest.or(prior.map(|compaction| compaction.floor)) else {
            return Ok(None);
        };

        // The floor is set before anything is dropped, so the peer can only
        // ever over-report what it has forgotten, never under-report it.
        *compaction = Some(Compaction {
            groups: count.saturating_add(Compaction::dropped(prior)),
            floor,
        });
        // `expired` counted leading elements of `groups`, so the range is in
        // bounds by construction.
        for group in groups.drain(..expired) {
            for event in &group.events {
                held.remove(&event.id);
            }
        }
        Ok(Some(floor))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};

    use super::MemorySyncPeer;
    use crate::{
        Classify, EventGroup, FailureClass, MemoryPeerError, MemoryResume, PushBatch,
        ReplicatedEvent, SendSyncPeer, Waiting,
    };

    const ORIGIN: StoreId = StoreId::from_bytes([0xC0; 16]);
    const HUB: StoreId = StoreId::from_bytes([0xC1; 16]);
    const T0: i64 = 1_800_000_000_000;
    const DAY_MS: i64 = 86_400_000;

    fn day(day: i64) -> RecordedAt {
        RecordedAt::from_millis(T0 + day * DAY_MS)
    }

    fn id(position: u64) -> EventId {
        EventId::new(
            ORIGIN,
            SequencePosition::new(position).expect("a test position is non-zero"),
        )
    }

    fn recorded(position: u64, on: i64) -> ReplicatedEvent {
        let event = Event::new("Logged", vec![u8::try_from(position).unwrap_or(0)])
            .expect("a static event type is valid");
        ReplicatedEvent::new(id(position), day(on), event)
    }

    /// A hub seeded with one group per entry, each event recorded on the day
    /// given, positions numbered 1, 2, … in log order.
    fn hub(groups: &[&[i64]]) -> MemorySyncPeer {
        let peer = MemorySyncPeer::new(HUB);
        let mut position = 0;
        for days in groups {
            let events = days
                .iter()
                .map(|&on| {
                    position += 1;
                    recorded(position, on)
                })
                .collect();
            peer.seed(EventGroup::new(None, events))
                .expect("an unpoisoned peer");
        }
        peer
    }

    fn ids_of(pulled: &crate::Pulled<MemoryResume>) -> Vec<EventId> {
        pulled.batch.events().map(|event| event.id).collect()
    }

    /// Before any compaction, and after one that drops nothing, there is no
    /// floor and nothing is gone.
    #[test]
    fn a_peer_that_has_forgotten_nothing_reports_no_floor() {
        let peer = hub(&[&[10], &[20]]);

        let floor = peer.compact_recorded_before(day(5)).expect("compacts");

        assert_eq!(floor, None);
        assert_eq!(SendSyncPeer::limits(&peer).retention_floor, None);
        assert_eq!(peer.group_count().expect("unpoisoned"), 2);
    }

    /// The floor is the newest event forgotten. Rejects a floor that names
    /// the oldest retained event, which a total compaction cannot state.
    #[test]
    fn compaction_drops_the_expired_prefix_and_reports_its_newest_event() {
        // Positions: 1 | 2 3 | 4 | 5
        let peer = hub(&[&[0], &[10, 20], &[50], &[70]]);

        let floor = peer.compact_recorded_before(day(40)).expect("compacts");

        assert_eq!(floor, Some(id(3)));
        assert_eq!(SendSyncPeer::limits(&peer).retention_floor, Some(id(3)));
        assert_eq!(peer.group_count().expect("unpoisoned"), 2);
    }

    /// Rejects a compaction that filters rather than cutting a prefix: the
    /// day-10 group sits behind the day-50 one, so it stays, and the floor
    /// still describes where a token can resume.
    #[tokio::test]
    async fn an_old_group_behind_a_newer_one_survives() {
        // Positions: 1 | 2 | 3
        let peer = hub(&[&[0], &[50], &[10]]);

        let floor = peer.compact_recorded_before(day(40)).expect("compacts");
        let pulled = peer
            .pull(Some(&MemoryResume { next_group: 1 }))
            .await
            .expect("a token at the floor resumes");

        assert_eq!(floor, Some(id(1)));
        assert_eq!(ids_of(&pulled), vec![id(2), id(3)]);
    }

    /// Rejects the peer that drains without an offset: its stale index would
    /// silently land on a later group.
    #[tokio::test]
    async fn a_token_below_the_floor_is_refused_as_reseed() {
        let peer = hub(&[&[0], &[10], &[50], &[70]]);
        let stale = MemoryResume { next_group: 1 };
        peer.compact_recorded_before(day(40)).expect("compacts");

        let refused = peer.pull(Some(&stale)).await;

        let Err(error) = refused else {
            panic!("a token below the floor must not resume: {refused:?}");
        };
        assert_eq!(error.class(), FailureClass::Reseed);
        assert!(
            matches!(
                error,
                MemoryPeerError::BelowRetentionFloor { floor, from: Some(from) }
                    if floor == id(2) && from == stale
            ),
            "{error:?}"
        );
    }

    /// Rejects a floor that refuses resumable tokens: a token at or past the
    /// floor gets exactly the retained groups past it, counts what waits from
    /// the retained log, and the next token is still absolute.
    #[tokio::test]
    async fn a_token_at_or_above_the_floor_resumes_with_absolute_tokens() {
        let mut limits = crate::PeerLimits::minimum();
        limits.max_batch_events = 1;
        let peer = MemorySyncPeer::with_limits(HUB, limits);
        for (position, on) in [(1, 0), (2, 10), (3, 50), (4, 70), (5, 100)] {
            peer.seed(EventGroup::new(None, vec![recorded(position, on)]))
                .expect("an unpoisoned peer");
        }
        peer.compact_recorded_before(day(40)).expect("compacts");

        let at_floor = peer
            .pull(Some(&MemoryResume { next_group: 2 }))
            .await
            .expect("a token at the floor resumes");
        let above = peer
            .pull(Some(&MemoryResume { next_group: 3 }))
            .await
            .expect("a token above the floor resumes");

        assert_eq!(ids_of(&at_floor), vec![id(3)]);
        assert_eq!(at_floor.resume, MemoryResume { next_group: 3 });
        assert_eq!(at_floor.waiting, Waiting::counted(2));
        assert_eq!(ids_of(&above), vec![id(4)]);
        assert_eq!(above.resume, MemoryResume { next_group: 4 });
        assert_eq!(above.waiting, Waiting::counted(1));
    }

    /// The floor only moves forward: a later cutoff moves it, an earlier one
    /// changes nothing, and a compaction of everything still reports one.
    #[test]
    fn the_floor_only_moves_forward() {
        let peer = hub(&[&[0], &[10], &[50], &[70]]);

        let first = peer.compact_recorded_before(day(20)).expect("compacts");
        let earlier = peer.compact_recorded_before(day(5)).expect("compacts");
        let later = peer.compact_recorded_before(day(60)).expect("compacts");
        let everything = peer.compact_recorded_before(day(500)).expect("compacts");

        assert_eq!(first, Some(id(2)));
        assert_eq!(earlier, Some(id(2)), "an earlier cutoff is a no-op");
        assert_eq!(later, Some(id(3)));
        assert_eq!(everything, Some(id(4)));
        assert_eq!(peer.group_count().expect("unpoisoned"), 0);
        assert_eq!(SendSyncPeer::limits(&peer).retention_floor, Some(id(4)));
    }

    /// Rejects a peer that reports a floor it was handed and does not
    /// enforce: that is the silent failure SY-32 forbids.
    #[test]
    fn a_declared_floor_is_replaced_by_the_compacted_one() {
        let mut limits = crate::PeerLimits::minimum();
        limits.retention_floor = Some(id(9));
        let peer = MemorySyncPeer::with_limits(HUB, limits);
        peer.seed(EventGroup::new(None, vec![recorded(1, 0)]))
            .expect("an unpoisoned peer");

        let before = SendSyncPeer::limits(&peer).retention_floor;
        peer.compact_recorded_before(day(40)).expect("compacts");
        let after = SendSyncPeer::limits(&peer).retention_floor;

        assert_eq!(before, None, "nothing has been forgotten yet");
        assert_eq!(after, Some(id(1)));
    }

    /// A forgotten identity is forgotten: pushed again, it is appended, and
    /// the high-water mark the peer confirms does not move back.
    #[tokio::test]
    async fn a_forgotten_identity_pushed_again_is_appended() {
        let peer = hub(&[&[0], &[50]]);
        peer.compact_recorded_before(day(40)).expect("compacts");

        let ack = peer
            .push(&PushBatch::new(vec![EventGroup::new(
                None,
                vec![recorded(1, 0)],
            )]))
            .await
            .expect("the push fits");

        assert_eq!((ack.appended, ack.skipped), (1, 0));
        assert_eq!(ack.confirmed.get(ORIGIN), Some(id(2).position()));
    }
}
