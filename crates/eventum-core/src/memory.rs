//! An in-memory reference event store.

use alloc::vec::Vec;
use std::sync::{PoisonError, RwLock};

use futures_core::Stream;

use crate::append::AppendCondition;
use crate::error::{AppendError, ConditionViolated};
use crate::event::{Event, SequencePosition, SequencedEvent};
use crate::query::{Query, ReadOptions};
use crate::store::SendEventStore;

/// The reference implementation of [`EventStore`](crate::EventStore).
///
/// It exists for three reasons, in order of importance:
///
/// 1. it is the oracle the conformance suite is validated against, so that a
///    failing adapter is known to be the adapter's fault and not the suite's;
/// 2. it makes this crate's examples runnable, so the documentation cannot
///    drift from the API;
/// 3. it lets application code be written and tested before any real adapter
///    exists.
///
/// It is **not** built for scale. [`read`](crate::EventStore::read) snapshots
/// the matching events under the lock and streams from that snapshot, so a read
/// never holds the lock across a poll — correct, and deliberately simple.
/// Cloning is cheap regardless: payloads are [`Bytes`](bytes::Bytes), so a
/// snapshot bumps refcounts rather than copying data.
///
/// Positions are dense and start at 1. The specification permits gaps, so
/// nothing may depend on that.
///
/// # Examples
///
/// The full DCB loop — read a decision model, then append conditioned on it:
///
/// ```
/// use eventum_core::{
///     AppendCondition, Event, EventStore, MemoryEventStore, Query, QueryItem, Tags,
///     read_decision_model,
/// };
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = MemoryEventStore::new();
///
/// // "Has course c1 already been defined?"
/// let query = Query::from_item(QueryItem::new(
///     ["CourseDefined"],
///     Tags::from_pairs([("course", "c1")])?,
/// )?)?;
///
/// let (events, last_seen) = read_decision_model(&store, &query).await?;
/// assert!(events.is_empty());
///
/// let defined = Event::new("CourseDefined", &b"{\"capacity\":2}"[..])?
///     .with_tags(Tags::from_pairs([("course", "c1")])?);
///
/// // Append only if nothing matching has appeared since we looked.
/// let condition = AppendCondition::new(query.clone()).after_opt(last_seen);
/// store.append(&[defined.clone()], Some(&condition)).await?;
///
/// // Replaying the same command now fails — the course exists.
/// let err = store.append(&[defined], Some(&condition)).await.unwrap_err();
/// assert!(err.is_condition_violated());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct MemoryEventStore {
    events: RwLock<Vec<SequencedEvent>>,
}

impl MemoryEventStore {
    /// Creates an empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a store pre-loaded with `events`, assigned dense positions from
    /// 1.
    ///
    /// Useful for arranging test fixtures without going through
    /// [`append`](crate::EventStore::append).
    pub fn with_events(events: impl IntoIterator<Item = Event>) -> Self {
        let sequenced = events
            .into_iter()
            .enumerate()
            .map(|(index, event)| SequencedEvent::new(position_at(index), event))
            .collect();

        Self {
            events: RwLock::new(sequenced),
        }
    }

    /// The number of events held.
    pub fn len(&self) -> usize {
        self.read_guard().len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.read_guard().is_empty()
    }

    /// The position most recently assigned, or `None` if the store is empty.
    pub fn last_position(&self) -> Option<SequencePosition> {
        self.read_guard().last().map(|event| event.position)
    }

    /// A snapshot of every event held, in position order.
    pub fn snapshot(&self) -> Vec<SequencedEvent> {
        self.read_guard().clone()
    }

    /// Reads through the lock, recovering from poisoning.
    ///
    /// A panic in another thread while holding the lock cannot have left this
    /// store inconsistent: every mutation happens in one `extend` under the
    /// write lock, and the validation that precedes it does not mutate. So
    /// poisoning carries no information here and is ignored rather than
    /// propagated as a spurious failure.
    fn read_guard(&self) -> std::sync::RwLockReadGuard<'_, Vec<SequencedEvent>> {
        self.events.read().unwrap_or_else(PoisonError::into_inner)
    }
}

/// The dense position for a zero-based index.
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index)
        .unwrap_or(u64::MAX - 1)
        .saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// [`MemoryEventStore`]'s error type, which is uninhabited.
///
/// An uninhabited error is worth having: it proves the contract does not
/// *require* a fallible read path, and it documents at the type level that this
/// store has no failure modes of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unreachable: the in-memory event store cannot fail")]
pub enum MemoryStoreError {}

impl SendEventStore for MemoryEventStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        let guard = self.read_guard();

        // Filter, order and truncate under the lock, then release it. The
        // stream that leaves this function borrows nothing.
        let matched = guard
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));

        let mut selected: Vec<SequencedEvent> = if options.backwards {
            matched
                .rev()
                .filter(|event| options.from.is_none_or(|from| event.position <= from))
                .cloned()
                .collect()
        } else {
            matched
                .filter(|event| options.from.is_none_or(|from| event.position >= from))
                .cloned()
                .collect()
        };

        if let Some(limit) = options.limit {
            selected.truncate(limit.get());
        }

        drop(guard);
        Snapshot(selected.into_iter())
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // The whole operation runs under one write lock, which is what makes it
        // atomic: no reader can observe a partially-applied batch, and no
        // concurrent appender can slip between the condition check and the
        // write. That race is precisely what an append condition exists to
        // prevent, so getting it wrong here would make the reference
        // implementation useless as an oracle.
        let mut stored = self.events.write().unwrap_or_else(PoisonError::into_inner);

        if let Some(condition) = condition {
            let conflict = stored.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });

            if let Some(conflict) = conflict {
                // Returning without touching `stored` is what "a rejected
                // append leaves the store byte-identical" means.
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }

        if events.is_empty() {
            // The specification defines `Events` as a non-empty collection, so
            // there is no position to return. Adapters must reject this rather
            // than invent one.
            return Err(AppendError::NoEvents);
        }

        let first_index = stored.len();
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            SequencedEvent::new(position_at(first_index + offset), event.clone())
        }));

        Ok(position_at(first_index + events.len() - 1))
    }
}

/// The stream returned by [`MemoryEventStore::read`].
///
/// Owns its events, so it borrows nothing from the store and is `Send`.
#[derive(Debug)]
struct Snapshot(alloc::vec::IntoIter<SequencedEvent>);

impl Stream for Snapshot {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(self.get_mut().0.next().map(Ok))
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    // Only ONE flavour is imported for method-call syntax. Importing both
    // `EventStore` and `SendEventStore` makes `store.read(..)` ambiguous,
    // because `MemoryEventStore` satisfies both; see the `store` module
    // documentation. The `SendEventStore` path is exercised below through
    // fully-qualified calls.
    use super::{AppendCondition, Event, MemoryEventStore, Query, ReadOptions};
    use crate::store::EventStore;
    use crate::{QueryItem, Tags, collect};

    fn event(event_type: &str) -> Event {
        Event::new(event_type, &b"{}"[..]).unwrap()
    }

    #[tokio::test]
    async fn append_assigns_dense_positions_from_one() {
        let store = MemoryEventStore::new();

        let last = store.append(&[event("A"), event("B")], None).await.unwrap();
        assert_eq!(last.get(), 2);

        let all = collect(store.read(&Query::all(), ReadOptions::new()))
            .await
            .unwrap();
        assert_eq!(
            all.iter().map(|e| e.position.get()).collect::<Vec<_>>(),
            [1, 2]
        );
    }

    #[tokio::test]
    async fn rejected_append_leaves_the_store_untouched() {
        let store = MemoryEventStore::new();
        store.append(&[event("A")], None).await.unwrap();
        let before = store.snapshot();

        let condition =
            AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()).unwrap());
        let err = store
            .append(&[event("B"), event("C")], Some(&condition))
            .await
            .unwrap_err();

        assert!(err.is_condition_violated());
        assert_eq!(store.snapshot(), before);
    }

    #[tokio::test]
    async fn empty_append_is_rejected() {
        let store = MemoryEventStore::new();
        assert!(store.append(&[], None).await.is_err());
    }

    #[tokio::test]
    async fn with_events_matches_append() {
        let seeded = MemoryEventStore::with_events([event("A"), event("B")]);
        let appended = MemoryEventStore::new();
        appended
            .append(&[event("A"), event("B")], None)
            .await
            .unwrap();

        assert_eq!(seeded.snapshot(), appended.snapshot());
    }

    #[tokio::test]
    async fn implements_the_non_send_flavour_too() {
        // Compiles only because `SendEventStore` implies `EventStore`. This is
        // the property that lets every consumer bind the weaker trait.
        async fn count<S: EventStore>(store: &S) -> usize {
            collect(store.read(&Query::all(), ReadOptions::new()))
                .await
                .map(|events| events.len())
                .unwrap_or_default()
        }

        let store = MemoryEventStore::new();
        store.append(&[event("A")], None).await.unwrap();
        assert_eq!(count(&store).await, 1);
    }

    #[test]
    fn read_stream_is_send() {
        // Guards the property the whole two-trait design exists for: on the
        // `Send` flavour it is the *stream* that is `Send`, not merely the
        // future producing it. Without that, a caller could not hold a read
        // across an await inside `tokio::spawn`.
        fn assert_send<T: Send>(_: &T) {}

        let store = MemoryEventStore::new();
        let query = Query::all();
        let stream = crate::SendEventStore::read(&store, &query, ReadOptions::new());
        assert_send(&stream);
    }

    #[tokio::test]
    async fn tags_filter_with_and_semantics() {
        let store = MemoryEventStore::new();
        store
            .append(
                &[
                    event("A").with_tags(Tags::from_pairs([("course", "c1")]).unwrap()),
                    event("A").with_tags(
                        Tags::from_pairs([("course", "c1"), ("student", "s1")]).unwrap(),
                    ),
                ],
                None,
            )
            .await
            .unwrap();

        let query = Query::from_item(
            QueryItem::tagged(Tags::from_pairs([("course", "c1"), ("student", "s1")]).unwrap())
                .unwrap(),
        )
        .unwrap();

        let matched = collect(store.read(&query, ReadOptions::new()))
            .await
            .unwrap();
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].position.get(), 2);
    }
}
