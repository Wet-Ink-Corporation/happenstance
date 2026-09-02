//! A conformant store whose positions are not dense.
//!
//! Modelled on `happenstance-core`'s reference store, and standalone rather
//! than built on it for one concrete reason: that store allocates from its own
//! **length**, so seeding it with strided positions through `restore` makes its
//! *next* append land below its own head — a VT-11 violation reached by a
//! shortcut that looks obviously correct. What is reused is the frozen crate's
//! public matching surface (`Query::matches`, `ReadOptions`, and
//! `AppendCondition::is_violated_by`), which is where the behaviour lives.

use core::num::NonZeroU64;
use core::sync::atomic::{AtomicU64, Ordering};
use std::sync::{PoisonError, RwLock};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, MemoryStoreError, Query,
    ReadOptions, RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId,
};

/// The time every event this store accepts is stamped with.
///
/// A constant rather than a clock, and the constraint is the repository's own:
/// CF-33 forbids anything under this crate's `src/` from reading one, because a
/// timing assertion passes on the author's machine and fails on a loaded
/// runner. VT-9 asks that a recorded time be *stable*, never that it be
/// recent, so a constant satisfies it exactly.
const RECORDED_AT: RecordedAt = RecordedAt::from_millis(1_700_000_000_000);

/// A conformant store whose positions advance by a stride, never by one.
///
/// **Rejects: a read-model handler that computes its next position by adding
/// one.** Positions are an opaque ordering key and the specification permits
/// gaps, but every store an application author has met assigns them densely,
/// so the input separating a correct handler from an incorrect one never
/// occurs in-process. This is that input, and it is conformant: it runs the
/// full conformance suite, because gaps are a freedom VT-11 grants rather than
/// a defect.
///
/// A real adapter reaches the same shape by allocating from a sequence with
/// `CACHE 7`, by encoding a shard id in the low bits, or by using a transaction
/// id as the position.
///
/// # Panics
///
/// `append` panics if the position space is exhausted — if `previous + stride`
/// would overflow `u64`. Wrapping instead would break the strict monotonicity
/// this store exists to demonstrate it *keeps*, and there is no error to report
/// it through: [`Error`](SendEventStore::Error) is
/// `MemoryStoreError`, which is uninhabited because this store has no failure
/// modes of its own. Reaching it takes about `2^64 / stride` appends.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroU64;
/// use happenstance_core::{AppendError, Event, EventStore};
/// use happenstance_core::MemoryStoreError;
/// use happenstance_testkit::{GappyMemoryStore, block_on};
///
/// let stride = NonZeroU64::new(7).ok_or("seven is not zero")?;
/// let store = GappyMemoryStore::with_stride(stride);
/// let seat = Event::new("Seated", &b"{}"[..])?;
///
/// let (first, second) = block_on(async {
///     let first = store.append(&[seat.clone()], None).await?;
///     let second = store.append(&[seat], None).await?;
///     Ok::<_, AppendError<MemoryStoreError>>((first, second))
/// })?;
///
/// // Strictly increasing, and never the next integer.
/// assert!(second > first);
/// assert_ne!(second.get(), first.get() + 1);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct GappyMemoryStore {
    events: RwLock<Vec<SequencedEvent>>,
    store_id: StoreId,
    stride: NonZeroU64,
}

impl GappyMemoryStore {
    /// Creates an empty store whose positions advance by `stride`.
    ///
    /// The first position assigned is `stride` itself, and each one after it is
    /// `previous + stride`. There is deliberately no `Default` and no implicit
    /// stride: a default here would be a number nobody chose, and the gap it
    /// left would be an accident rather than an instrument.
    ///
    /// A stride of one is legal and makes the store dense, which is a caller
    /// asking for a plain in-memory store; anything above one leaves a hole
    /// between every pair of consecutive events.
    #[must_use]
    pub fn with_stride(stride: NonZeroU64) -> Self {
        Self {
            events: RwLock::new(Vec::new()),
            store_id: next_store_id(),
            stride,
        }
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// The next position after `previous`, or the first if there is none.
    fn next_position(&self, previous: Option<SequencePosition>) -> SequencePosition {
        let raw = match previous {
            None => self.stride.get(),
            Some(previous) => match previous.get().checked_add(self.stride.get()) {
                Some(next) => next,
                None => panic!(
                    "GappyMemoryStore has exhausted its position space: the next \
                     position after {previous} would overflow. Wrapping would \
                     break the strict monotonicity this store exists to keep"
                ),
            },
        };

        // Unreachable: `raw` is at least `stride`, which is non-zero, so
        // `SequencePosition::new` cannot refuse it. Absorbed rather than
        // unwrapped, for the reason the reference store absorbs the same call.
        SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
    }

    /// Reads through the lock, recovering from poisoning.
    ///
    /// Every mutation happens under one write lock and the validation that
    /// precedes it does not mutate, so a poisoned lock carries no information
    /// here and is not worth propagating as a spurious failure.
    fn read_guard(&self) -> std::sync::RwLockReadGuard<'_, Vec<SequencedEvent>> {
        self.events.read().unwrap_or_else(PoisonError::into_inner)
    }
}

/// A fresh incarnation identifier, without a clock.
///
/// Process-local and monotonic, which is all a store holding nothing across a
/// process boundary needs: there is no persistent state a second incarnation
/// could be confused with. The leading tag keeps a gappy store's identifiers
/// away from any other allocator's in the same process.
fn next_store_id() -> StoreId {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    const TAG: u64 = 0x_6761_7070_795F_6D65;

    let ordinal = COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&TAG.to_be_bytes());
    bytes[8..].copy_from_slice(&ordinal.to_be_bytes());
    StoreId::from_bytes(bytes)
}

impl SendEventStore for GappyMemoryStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        let guard = self.read_guard();

        // Filter, order and truncate under the lock, then release it. The stream
        // that leaves this function borrows nothing.
        let matched = guard
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));

        // `from` is the starting bound and `to` the stopping one, so reading
        // backwards swaps which side of the position order each sits on. Both
        // are inclusive in both directions.
        let mut selected: Vec<SequencedEvent> = if options.backwards {
            matched
                .rev()
                .filter(|event| options.from.is_none_or(|from| event.position <= from))
                .filter(|event| options.to.is_none_or(|to| event.position >= to))
                .cloned()
                .collect()
        } else {
            matched
                .filter(|event| options.from.is_none_or(|from| event.position >= from))
                .filter(|event| options.to.is_none_or(|to| event.position <= to))
                .cloned()
                .collect()
        };

        // After filtering and ordering, never before: `limit` truncates the
        // result set the caller would otherwise have seen.
        if let Some(limit) = options.limit {
            selected.truncate(limit);
        }

        drop(guard);
        Snapshot(selected.into_iter())
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first, and above the lock — ES-20. A batch is defined as
        // non-empty, so there is no position to return, and reporting the
        // violation instead would put a correct client into a retry loop that
        // never terminates.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        let mut stored = self.events.write().unwrap_or_else(PoisonError::into_inner);

        if let Some(condition) = condition {
            let conflict = stored.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });

            if let Some(conflict) = conflict {
                // Returning without touching `stored` is what "a rejected append
                // leaves the store byte-identical" means.
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }

        // The allocator is the whole instrument: each position comes from the
        // previous **position**, never from the log's length. Deriving it from a
        // count is the `COUNT(*) + 1` defect, and it is unwritable here.
        let mut previous = stored.last().map(|event| event.position);
        let mut last = None;

        for event in events {
            let position = self.next_position(previous);
            stored.push(SequencedEvent::new(
                position,
                EventId::new(self.store_id, position),
                RECORDED_AT,
                event.clone(),
            ));
            previous = Some(position);
            last = Some(position);
        }

        match last {
            Some(position) => Ok(position),
            // Unreachable: the emptiness check above already returned.
            None => Err(AppendError::NoEvents),
        }
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.read_guard().last().map(|event| event.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.read_guard().iter().any(|event| event.id == id))
    }
}

/// The stream returned by [`GappyMemoryStore::read`].
///
/// Owns its events, so it borrows nothing from the store and is `Send`.
#[derive(Debug)]
struct Snapshot(std::vec::IntoIter<SequencedEvent>);

impl Stream for Snapshot {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(self.get_mut().0.next().map(Ok))
    }
}
