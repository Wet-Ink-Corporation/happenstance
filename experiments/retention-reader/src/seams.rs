//! What the port says about a store, for the "could anything at the port have
//! told it?" column.
//!
//! A holed store is compared with a **young twin**: a fresh store that was
//! handed the same retained events, by ingest so each keeps its identity, and
//! never held anything else. Positions differ (holes against dense), so the
//! comparison is of [`Shape`], which leaves the position values out. ES-38
//! permits gaps everywhere, so no reader may read meaning into them.

use happenstance_core::{
    EventId, EventStore, EventType, MemoryStoreError, Query, ReadOptions, SequencePosition,
    SequencedEvent, Tags, collect,
};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, ReplicatedEvent};

use crate::shared::Shared;

/// Every answer the port gives about one store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seams {
    /// `head()`.
    pub head: Option<SequencePosition>,
    /// `read(Query::all())`, forward.
    pub held: Vec<SequencedEvent>,
    /// `contains_event_id` for every probed id, in probe order.
    pub member: Vec<(EventId, bool)>,
}

/// [`Seams`] without the position values: what a pruned store and a young
/// one could differ in, if the port told them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    /// Each held event's identity, type and tags, in read order.
    pub held: Vec<(EventId, EventType, Tags)>,
    /// Membership of each probed id.
    pub member: Vec<(EventId, bool)>,
    /// Whether `head()` is the last position `read` yielded.
    pub head_is_last_held: bool,
}

impl Seams {
    /// The shape of these answers.
    #[must_use]
    pub fn shape(&self) -> Shape {
        Shape {
            // A new owned value to compare: the type and tags are copied once.
            held: self
                .held
                .iter()
                .map(|e| (e.id, e.event_type().clone(), e.tags().clone()))
                .collect(),
            // The shape owns its answers, so two can be compared side by side.
            member: self.member.clone(),
            head_is_last_held: self.head == self.held.last().map(|e| e.position),
        }
    }

    /// The lowest position `read` yielded: what an `earliest_position()`
    /// floor would answer.
    #[must_use]
    pub fn floor(&self) -> Option<SequencePosition> {
        self.held.first().map(|e| e.position)
    }
}

/// Asks `store` every question the port has, probing membership of `probe`.
///
/// # Errors
///
/// The store's own error.
pub async fn seams<S: EventStore>(store: &S, probe: &[EventId]) -> Result<Seams, S::Error> {
    let top = store.head().await?;
    let held = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    let mut member = Vec::with_capacity(probe.len());
    for &id in probe {
        member.push((id, store.contains_event_id(id).await?));
    }
    Ok(Seams {
        head: top,
        held,
        member,
    })
}

/// A fresh store holding exactly `held`, each event under its own identity.
///
/// # Errors
///
/// The ingest's refusal; the in-memory store refuses only a claim on its own
/// identity, which a fresh incarnation never meets.
pub async fn young_twin(held: &[SequencedEvent]) -> Result<Shared, IngestError<MemoryStoreError>> {
    let twin = Shared::new();
    // The twin stores its own copy of each event: genuine duplication.
    let shipped: Vec<ReplicatedEvent> = held
        .iter()
        .map(|e| ReplicatedEvent::new(e.id, e.recorded_at, e.event.clone()))
        .collect();
    let groups: Vec<IngestGroup<'_>> = shipped
        .chunks(1)
        .map(|one| IngestGroup::new(one, &[]))
        .collect();
    twin.ingest(&groups).await?;
    Ok(twin)
}

#[cfg(test)]
mod tests {
    use happenstance_core::{Event, EventStore, Tags};
    use happenstance_testkit::block_on;

    use super::{seams, young_twin};
    use crate::holed::Holed;
    use crate::shared::Shared;

    #[test]
    fn seams_of_a_holed_store_match_a_young_store_in_shape()
    -> Result<(), Box<dyn core::error::Error>> {
        let holed = Holed::new(Shared::new());
        block_on(async {
            let mut landed = Vec::new();
            for claim in ["a", "b", "c", "d", "e"] {
                let tags = Tags::from_pairs([("claim", claim)])?;
                let event = Event::new("Written", &b"{}"[..])?.with_tags(tags);
                landed.push(holed.inner().append(&[event], None).await?);
            }
            let all = seams(holed.inner(), &[]).await?;
            let probe: Vec<_> = all.held.iter().map(|e| e.id).collect();
            let b = *landed.get(1).ok_or("five were appended")?;
            let d = *landed.get(3).ok_or("five were appended")?;
            holed.view().withhold(&[b, d]).await?;

            let pruned = seams(holed.view(), &probe).await?;
            let twin = young_twin(&pruned.held).await?;
            let young = seams(&twin, &probe).await?;

            assert_eq!(pruned.shape(), young.shape());
            assert_ne!(
                pruned.held.iter().map(|e| e.position).collect::<Vec<_>>(),
                young.held.iter().map(|e| e.position).collect::<Vec<_>>(),
                "only the position values differ"
            );
            assert_eq!(
                young
                    .shape()
                    .member
                    .iter()
                    .filter(|(_, held)| !held)
                    .count(),
                2
            );
            assert!(twin.head().await? < pruned.head);
            Ok(())
        })
    }
}
