//! SY-14's arrangement: VT-24's floor of foreign events in one call, with a
//! scattered part of it delivered first.

use alloc::format;
use alloc::vec::Vec;

use happenstance_core::{EventStore, MIN_SUPPORTED_EVENTS_PER_BATCH, SequencedEvent, StoreId};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent};

use super::{
    ensure_counts, ensure_foreign, ensure_unchanged, held_once, observe, read_all, shipped,
    watermark,
};
use crate::violation::{Violation, ensure};

/// SY-14, the clause every check here reports under but one.
const SY_14: &str = "SY-14";

/// VT-24, the clause a capacity refusal of a call at or under its floor
/// breaks: the call is the batch on the ingest path, as VT-36 says of tags.
const VT_24: &str = "VT-24";

/// SY-14's origin, the next free one after SY-10's.
const BULK_ORIGIN: StoreId = StoreId::from_bytes([0x7D; 16]);

/// The lengths of the call's groups, summing to
/// [`MIN_SUPPORTED_EVENTS_PER_BATCH`]: a group of one, which the first
/// delivery holds whole, and six more, each doubling, so the last group alone
/// is half the call and a held event sits past its first half.
const GROUP_LENGTHS: [usize; 7] = [1, 3, 4, 8, 16, 32, 64];

/// When the first event was recorded at its origin; each later one a
/// millisecond on, so no two share a time.
const FIRST_RECORDED: i64 = 1_800_000_000_000;

/// The call's events, in call order: origin positions 1 to the floor, each
/// with bytes of its own, so the rule finds every one without its identity.
fn the_floor() -> Result<Vec<ReplicatedEvent>, Violation> {
    (1..=MIN_SUPPORTED_EVENTS_PER_BATCH)
        .map(|index| {
            let position = u64::try_from(index)
                .map_err(|_| Violation::new(SY_14, "the floor does not fit an origin position"))?;
            let millis = FIRST_RECORDED.saturating_add_unsigned(position);
            shipped(
                BULK_ORIGIN,
                position,
                millis,
                format!("sy14-{index:03}").as_bytes(),
                SY_14,
            )
        })
        .collect()
}

/// `events` cut into [`GROUP_LENGTHS`], in order. A length past the end
/// yields what is left, and nothing past that.
fn grouped(events: &[ReplicatedEvent]) -> Vec<&[ReplicatedEvent]> {
    let mut rest = events;
    let mut groups = Vec::with_capacity(GROUP_LENGTHS.len());
    for length in GROUP_LENGTHS {
        let (group, after) = rest.split_at_checked(length).unwrap_or((rest, &[]));
        if !group.is_empty() {
            groups.push(group);
        }
        rest = after;
    }
    groups
}

/// Whether the event at `index` in the call is delivered first: every third,
/// from the first, and the last. So the first group is held whole, every
/// other group in part, and held events sit past any prefix a store might
/// look up alone.
const fn delivered_first(index: usize) -> bool {
    index.is_multiple_of(3) || index + 1 == MIN_SUPPORTED_EVENTS_PER_BATCH
}

/// Each group's events [`delivered_first`] picks, keeping their groups.
///
/// Owned, because the first delivery is a second batch of the same events: a
/// clone of each is a reference count on its bytes.
fn first_delivery(groups: &[&[ReplicatedEvent]]) -> Vec<Vec<ReplicatedEvent>> {
    let mut index = 0_usize;
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .filter(|_| {
                    let picked = delivered_first(index);
                    index += 1;
                    picked
                })
                .cloned()
                .collect()
        })
        .collect()
}

/// Ingests `groups`, which the store must accept in this one call. A capacity
/// refusal of a call no longer than VT-24's floor is reported under VT-24.
async fn ingest_once<S: IngestStore>(
    store: &S,
    groups: &[IngestGroup<'_>],
    what: &str,
) -> Result<Ingested, Violation> {
    store.ingest(groups).await.map_err(|error| match error {
        IngestError::ExceedsStoreLimit { limit, len, .. } => Violation::new(
            VT_24,
            format!(
                "the store refused {what} as over its {limit} limit at {len}; every store takes \
                 {MIN_SUPPORTED_EVENTS_PER_BATCH} events in one ingest call, which is the push \
                 the minimum peer declares it takes"
            ),
        ),
        other => Violation::new(
            SY_14,
            format!(
                "the store refused {what}: {other}; a pulled batch lands in one ingest, however \
                 many of its events the store already holds"
            ),
        ),
    })
}

/// SY-14's whole arrangement on one store: the first delivery, the bulk call
/// and its re-delivery.
///
/// # Errors
///
/// A [`Violation`] of SY-14 if the store cannot say who it is, if a call is
/// refused or miscounted, or if the log holds a sent event other than once,
/// changes a row the first delivery wrote, or is changed by the re-delivery; of
/// VT-24 if a call is refused as over a capacity limit.
pub(super) async fn the_floor_lands_once<S: EventStore + IngestStore>(
    store: &S,
) -> Result<(), Violation> {
    ensure_foreign(store, &[BULK_ORIGIN], SY_14).await?;
    let events = the_floor()?;
    let groups = grouped(&events);
    let early = first_delivery(&groups);
    let held = early.iter().map(Vec::len).sum::<usize>();

    let first: Vec<IngestGroup<'_>> = early
        .iter()
        .filter(|group| !group.is_empty())
        .map(|group| IngestGroup::new(group, &[]))
        .collect();
    let landed = ingest_once(store, &first, "the first delivery").await?;
    ensure_counts(landed, (held, 0, 0), "the first delivery", SY_14)?;
    let before = read_all(store, SY_14).await?;

    let bulk: Vec<IngestGroup<'_>> = groups
        .iter()
        .map(|group| IngestGroup::new(group, &[]))
        .collect();
    let what = format!(
        "a call of {} events in {} groups, {held} of them already held",
        events.len(),
        bulk.len()
    );
    let landed = ingest_once(store, &bulk, &what).await?;
    let new = events.len().saturating_sub(held);
    ensure_counts(landed, (new, held, 0), &what, SY_14)?;
    holds_each_once(store, &events, &before).await?;

    let settled = observe(store, SY_14).await?;
    let again = ingest_once(store, &bulk, "the same call again").await?;
    ensure_counts(again, (0, events.len(), 0), "the same call again", SY_14)?;
    ensure_unchanged(store, &settled, "the same call again", SY_14).await
}

/// The log holds each of `events` once and nothing else, keeps every row
/// `before` held with its identity, time and bytes, and its watermark for the
/// origin is the highest origin position sent.
async fn holds_each_once<S: EventStore + IngestStore>(
    store: &S,
    events: &[ReplicatedEvent],
    before: &[SequencedEvent],
) -> Result<(), Violation> {
    let log = read_all(store, SY_14).await?;
    for sent in events {
        held_once(&log, &sent.event, SY_14)?;
    }
    ensure(log.len() == events.len(), SY_14, || {
        format!(
            "the store holds {} events after one call of {} new and held ones; it should hold \
             each once",
            log.len(),
            events.len()
        )
    })?;
    // Identity, time and bytes, and not position: whether anything moved for
    // the new events is SY-5's to judge.
    let kept = |earlier: &SequencedEvent| {
        log.iter().any(|row| {
            row.id == earlier.id
                && row.recorded_at == earlier.recorded_at
                && row.event == earlier.event
        })
    };
    let changed = before.iter().filter(|row| !kept(row)).count();
    ensure(changed == 0, SY_14, || {
        format!(
            "{changed} of the {} events the first delivery wrote are not held as they were after \
             the call that re-delivered them; a held event is a skip",
            before.len()
        )
    })?;
    let highest = events.last().map(|event| event.id.position());
    let mark = watermark(store, SY_14).await?.get(BULK_ORIGIN);
    ensure(mark == highest, SY_14, || {
        format!("the watermark for {BULK_ORIGIN} is {mark:?}; the highest sent was {highest:?}")
    })
}

#[cfg(test)]
mod tests {
    use happenstance_core::{EventId, MIN_SUPPORTED_EVENTS_PER_BATCH, SequencePosition};

    use super::{BULK_ORIGIN, GROUP_LENGTHS, delivered_first, first_delivery, grouped, the_floor};

    /// The identity the rule's event at call `index` carries.
    fn identity_at(index: usize) -> Option<EventId> {
        u64::try_from(index + 1)
            .ok()
            .and_then(SequencePosition::new)
            .map(|position| EventId::new(BULK_ORIGIN, position))
    }

    /// The groups are VT-24's floor exactly, cut where the lengths say.
    #[test]
    fn the_call_is_the_floor_in_seven_groups() {
        assert_eq!(
            GROUP_LENGTHS.iter().sum::<usize>(),
            MIN_SUPPORTED_EVENTS_PER_BATCH
        );
        let events = the_floor().expect("the rule's own events are valid");
        let groups = grouped(&events);
        let lengths: Vec<usize> = groups.iter().map(|group| group.len()).collect();
        assert_eq!(lengths, GROUP_LENGTHS);
        assert_eq!(events.first().map(|event| event.id), identity_at(0));
        assert_eq!(
            events.last().map(|event| event.id),
            identity_at(MIN_SUPPORTED_EVENTS_PER_BATCH - 1)
        );
    }

    /// The first delivery holds the first group whole, part of every other,
    /// the last event, and events past the first half of the call: what the
    /// one-chunk mistake needs to be seen.
    #[test]
    fn the_first_delivery_is_scattered_across_every_group() {
        let events = the_floor().expect("the rule's own events are valid");
        let groups = grouped(&events);
        let early = first_delivery(&groups);

        assert_eq!(early.iter().map(Vec::len).sum::<usize>(), 44);
        assert_eq!(
            early.first().map(Vec::len),
            Some(1),
            "the first group whole"
        );
        for (group, picked) in groups.iter().zip(&early).skip(1) {
            assert!(!picked.is_empty() && picked.len() < group.len());
        }
        assert!(delivered_first(MIN_SUPPORTED_EVENTS_PER_BATCH - 1));
        let past_half = (MIN_SUPPORTED_EVENTS_PER_BATCH / 2..MIN_SUPPORTED_EVENTS_PER_BATCH)
            .filter(|index| delivered_first(*index))
            .count();
        assert!(past_half > 1, "{past_half}");
    }
}
