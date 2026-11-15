//! A store that holds only part of its own log: CF-27's completeness instrument.
//!
//! # Why a decorator, and not a log of its own
//!
//! CF-27 asks for an instrument over *any* `EventStore`, so that a real adapter,
//! or a [`FaultyStore`](crate::FaultyStore), can be the store that has forgotten
//! something. A store holding its own log (the shape `GappyMemoryStore` takes)
//! is simpler and could never wrap anything. Removing events physically from the
//! inner store was the other alternative, and it lost harder: the reference
//! store allocates from its own length, so removal would make it reuse
//! positions, which is the wrong implementation ES-38 names, built into the
//! instrument. Real adapters remove outside the port through a fixture
//! capability instead (ADR-0028).
//!
//! So the inner store keeps everything, and this wrapper hides what it
//! withholds at every seam the port has: `read`, `head`, `contains_event_id`,
//! and the condition `append` evaluates.
//!
//! # Why the condition rewrite is sound
//!
//! Filtering `read` alone would leave the inner store evaluating conditions over
//! withheld events, refusing appends a pruned store admits (ES-40). Let `W` be
//! the highest withheld position. A conditional append is split in two:
//!
//! 1. **At or below `W`**, the retained events are checked here, against the
//!    caller's condition as written, by reading the inner store up to `W`.
//! 2. **Above `W`**, the inner store evaluates the condition with every guard's
//!    `after` raised to at least `W`, atomically with the write, under whatever
//!    lock or transaction it uses.
//!
//! Every event above `W` is retained, by the definition of `W`, so step 2 covers
//! exactly the retained events above it. Step 1 is not atomic with the write,
//! and it does not need to be: `W` is a position this wrapper has observed, no
//! append lands below an observed position (ES-10), and withholding only shrinks
//! the retained set. The region step 1 reads is frozen. Racing conditional
//! appends still elect one winner, because the contended region is the inner
//! store's.

use core::pin::Pin;
use core::task::{Context, Poll};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, PoisonError, RwLock};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, EventStore, Query,
    ReadOptions, SequencePosition, SequencedEvent, collect,
};

/// The withheld set: position to the id of the event the inner store holds
/// there.
///
/// The id is captured when the event is withheld, because membership (ES-41) is
/// asked by id, and an ingested event's id is its origin's, so it cannot be
/// derived from the position.
#[derive(Debug, Default, Clone)]
struct Withheld {
    by_position: BTreeMap<SequencePosition, EventId>,
}

impl Withheld {
    fn holds_position(&self, position: SequencePosition) -> bool {
        self.by_position.contains_key(&position)
    }

    /// A linear scan. This is an instrument, and an index by id would be a
    /// second map to keep in step for no caller that needs the speed.
    fn holds_id(&self, id: EventId) -> bool {
        self.by_position.values().any(|held| *held == id)
    }

    fn highest(&self) -> Option<SequencePosition> {
        self.by_position
            .last_key_value()
            .map(|(position, _)| *position)
    }

    fn len(&self) -> usize {
        self.by_position.len()
    }
}

/// Why a [`WithholdingStore::withhold`] or
/// [`WithholdingStore::withhold_where`] call withheld nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum WithholdError<E> {
    /// The inner store yields no event at `position`: never assigned, a gap,
    /// above the head, or not yet visible.
    ///
    /// Refused rather than recorded, because a report naming it would claim the
    /// instrument hid an event that never existed.
    #[non_exhaustive]
    NotHeld {
        /// The lowest requested position the inner store does not hold.
        position: SequencePosition,
    },
    /// Reading the inner store failed.
    ///
    /// The inner error is carried, not rendered: it is reachable through
    /// [`source`](core::error::Error::source).
    Store(E),
}

impl<E> core::fmt::Display for WithholdError<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotHeld { position } => write!(
                f,
                "the wrapped store holds no event at position {position}, \
                 so there is nothing there to withhold"
            ),
            Self::Store(_) => f.write_str("reading the wrapped store failed"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for WithholdError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::NotHeld { .. } => None,
            Self::Store(err) => Some(err),
        }
    }
}

/// Wraps a store so that it holds only part of its own log: a suffix, or a
/// scattered subset with survivors between the holes.
///
/// **Rejects: a reader that infers completeness from a port value.** A pruned
/// store and a young store are the same value at every seam the port exposes
/// (CF-27, ES-39), so a projection runner or an ingest path that reads "the log
/// starts here" from `head`, from the lowest position it read, or from a
/// condition that passed, is wrong, and this is the store that shows it.
///
/// Every event it withholds behaves as removed: `read` never yields it, `head`
/// skips it, `contains_event_id` answers `false` for it, and an append condition
/// that only it would violate passes. What it withholds is reported through
/// [`withheld`](Self::withheld), its own inherent method, and never through an
/// `EventStore` method (ADR-0028).
///
/// With nothing withheld it is transparent: every call is delegated with the
/// same arguments. Cloning it produces a second handle onto the **same**
/// withheld set.
///
/// **Cost.** Each conditional append made while anything is withheld first reads
/// the inner log up to the highest withheld position, which is what an
/// instrument can afford and an adapter could not.
///
/// # What it cannot model
///
/// - **Ingest deduplication.** The inner store still holds what is withheld, so
///   an ingest path over the inner store would deduplicate a re-delivered
///   withheld event where a physically pruned store would admit it again.
/// - **Position reuse.** The inner store forgets nothing, so the instrument
///   never reuses a position (ES-38) by delegation. That rule's teeth are the
///   real adapters, removing outside the port.
///
/// It implements the bare [`EventStore`] flavour only, as [`FaultyStore`]
/// does, and for the same coherence reason a `Send` sibling would be a second
/// type.
///
/// [`FaultyStore`]: crate::FaultyStore
///
/// # Examples
///
/// ```
/// use happenstance_core::{AppendCondition, Event, EventStore, MemoryEventStore};
/// use happenstance_core::{Query, QueryItem, ReadOptions, Tags, collect};
/// use happenstance_testkit::{WithholdingStore, block_on};
///
/// let claimed = |claim: &str| -> Result<Event, Box<dyn core::error::Error>> {
///     let tags = Tags::from_pairs([("claim", claim)])?;
///     Ok(Event::new("Claimed", &b"{}"[..])?.with_tags(tags))
/// };
/// let store = WithholdingStore::new(MemoryEventStore::new());
///
/// block_on(async {
///     for claim in ["first", "middle", "last"] {
///         store.append(&[claimed(claim)?], None).await?;
///     }
///     let all = collect(store.read(&Query::all(), ReadOptions::new())).await?;
///     let middle = all.get(1).ok_or("three were appended")?.position;
///
///     store.withhold(&[middle]).await?;
///
///     // `read` skips it, and the report names it.
///     let now = collect(store.read(&Query::all(), ReadOptions::new())).await?;
///     assert_eq!(now.len(), all.len() - 1);
///     assert!(now.iter().all(|event| event.position != middle));
///     assert!(store.withheld().contains(&middle));
///
///     // A condition only the withheld event would violate admits the append.
///     let only_middle =
///         Query::from_item(QueryItem::tagged(Tags::from_pairs([("claim", "middle")])?)?);
///     let landed = store
///         .append(&[claimed("middle")?], Some(&AppendCondition::new(only_middle)))
///         .await?;
///     assert!(landed > middle);
///     Ok::<(), Box<dyn core::error::Error>>(())
/// })?;
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct WithholdingStore<S> {
    inner: S,
    // Interior mutability: `withhold` takes `&self` because the port does and
    // handles are shared. `RwLock` rather than `RefCell` so the wrapper stays
    // `Send + Sync` over a store that is, which the concurrency family needs.
    // The inner `Arc` is a snapshot: a reader clones it under a momentary guard
    // and drops the guard before any await, and a writer copies on write, so a
    // snapshot never changes under its reader. The outer `Arc` is what makes a
    // clone a second handle onto the same set.
    withheld: Arc<RwLock<Arc<Withheld>>>,
}

impl<S> WithholdingStore<S> {
    /// Wraps `inner`, withholding nothing.
    #[must_use]
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            withheld: Arc::new(RwLock::new(Arc::new(Withheld::default()))),
        }
    }

    /// Every position this instrument withholds, ascending: the report.
    ///
    /// An owned snapshot rather than a borrowing iterator, because the set
    /// lives behind a lock that cannot be held while a caller iterates, and a
    /// concurrent `withhold` may change it meanwhile.
    #[must_use]
    pub fn withheld(&self) -> BTreeSet<SequencePosition> {
        self.snapshot().by_position.keys().copied().collect()
    }

    /// The current withheld set. The guard is dropped before this returns, so
    /// no caller can hold it across an await.
    fn snapshot(&self) -> Arc<Withheld> {
        Arc::clone(&self.withheld.read().unwrap_or_else(PoisonError::into_inner))
    }

    /// Adds `found` to the withheld set and returns the positions it added.
    ///
    /// Merged under the write guard into the *current* set, not the one read
    /// before the caller's await, so two concurrent withholds cannot lose each
    /// other's entries. Poisoning is recovered from: this is the only writer,
    /// and nothing in it panics part way, so a poisoned lock carries no
    /// information.
    fn merge(&self, found: BTreeMap<SequencePosition, EventId>) -> Vec<SequencePosition> {
        let mut guard = self
            .withheld
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        // Copy-on-write: the set is cloned only if a reader still holds the
        // snapshot this replaces.
        let next = Arc::make_mut(&mut guard);
        found
            .into_iter()
            .filter_map(|(position, id)| {
                next.by_position
                    .insert(position, id)
                    .is_none()
                    .then_some(position)
            })
            .collect()
    }

    /// The inner stream with `withheld` applied.
    ///
    /// `limit` has to count retained events only, so the inner read is given
    /// room for every withheld event it might yield besides: at most
    /// `withheld.len()` are skipped. With nothing withheld that is the caller's
    /// own limit, unchanged.
    fn filtered<'s, 'q>(
        &'s self,
        withheld: Arc<Withheld>,
        query: &'q Query,
        options: ReadOptions,
    ) -> Withholding<impl Stream<Item = Result<SequencedEvent, S::Error>> + use<'s, 'q, S>>
    where
        S: EventStore,
    {
        let mut inner_options = options;
        inner_options.limit = options
            .limit
            .map(|limit| limit.saturating_add(withheld.len()));
        Withholding {
            inner: Box::pin(EventStore::read(&self.inner, query, inner_options)),
            withheld,
            remaining: options.limit,
        }
    }
}

impl<S: EventStore> WithholdingStore<S> {
    /// Withholds the events the inner store holds at `positions`, all or
    /// nothing.
    ///
    /// Withholding an already-withheld position again is a no-op, and an empty
    /// slice withholds nothing and succeeds.
    ///
    /// # Errors
    ///
    /// [`WithholdError::NotHeld`], naming the lowest such position, if the inner
    /// store yields no event at one of `positions`; and
    /// [`WithholdError::Store`] if reading the inner store fails. Either way
    /// nothing is withheld.
    pub async fn withhold(
        &self,
        positions: &[SequencePosition],
    ) -> Result<(), WithholdError<S::Error>> {
        let requested: BTreeSet<SequencePosition> = positions.iter().copied().collect();
        let (Some(&lowest), Some(&highest)) = (requested.first(), requested.last()) else {
            return Ok(());
        };

        let window = ReadOptions::new().from(lowest).to(highest);
        let held = collect(EventStore::read(&self.inner, &Query::all(), window))
            .await
            .map_err(WithholdError::Store)?;
        let found: BTreeMap<SequencePosition, EventId> = held
            .into_iter()
            .filter(|event| requested.contains(&event.position))
            .map(|event| (event.position, event.id))
            .collect();

        if let Some(&position) = requested.iter().find(|wanted| !found.contains_key(wanted)) {
            return Err(WithholdError::NotHeld { position });
        }
        self.merge(found);
        Ok(())
    }

    /// Withholds every held, not yet withheld event that `select` picks, and
    /// returns the positions it withheld, ascending.
    ///
    /// The suffix is `|event| event.position <= cut`, a purge is a predicate on
    /// tags, and `|_| true` withholds everything held.
    ///
    /// # Errors
    ///
    /// [`WithholdError::Store`] if reading the inner store fails, in which case
    /// nothing is withheld.
    ///
    /// # Examples
    ///
    /// ```
    /// use happenstance_core::{Event, EventStore, MemoryEventStore, Query, ReadOptions, collect};
    /// use happenstance_testkit::{WithholdingStore, block_on};
    ///
    /// let store = WithholdingStore::new(MemoryEventStore::new());
    /// block_on(async {
    ///     let mut assigned = Vec::new();
    ///     for _ in 0..4 {
    ///         assigned.push(store.append(&[Event::new("Ticked", &b"{}"[..])?], None).await?);
    ///     }
    ///     let cut = *assigned.get(1).ok_or("four were appended")?;
    ///
    ///     // Withhold a prefix of history: the store now holds only a suffix.
    ///     let withheld = store.withhold_where(|event| event.position <= cut).await?;
    ///     assert_eq!(withheld, assigned.get(..2).ok_or("four were appended")?);
    ///
    ///     let retained = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    ///     assert!(retained.iter().all(|event| event.position > cut));
    ///     Ok::<(), Box<dyn core::error::Error>>(())
    /// })?;
    /// # Ok::<(), Box<dyn core::error::Error>>(())
    /// ```
    pub async fn withhold_where(
        &self,
        mut select: impl FnMut(&SequencedEvent) -> bool,
    ) -> Result<Vec<SequencePosition>, WithholdError<S::Error>> {
        let already = self.snapshot();
        let held = collect(EventStore::read(
            &self.inner,
            &Query::all(),
            ReadOptions::new(),
        ))
        .await
        .map_err(WithholdError::Store)?;
        let chosen: BTreeMap<SequencePosition, EventId> = held
            .iter()
            .filter(|event| !already.holds_position(event.position))
            .filter(|event| select(event))
            .map(|event| (event.position, event.id))
            .collect();
        Ok(self.merge(chosen))
    }
}

impl<S: EventStore> EventStore for WithholdingStore<S> {
    // The instrument adds no failure mode at the port, so the inner store's
    // errors pass through unchanged.
    type Error = S::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.filtered(self.snapshot(), query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let withheld = self.snapshot();
        // An empty batch is delegated before any check, so the inner store
        // refuses it in its own words (ES-20). No condition, or nothing
        // withheld, is a pure delegation.
        let (Some(condition), Some(floor), false) =
            (condition, withheld.highest(), events.is_empty())
        else {
            return EventStore::append(&self.inner, events, condition).await;
        };

        // At or below the floor: the frozen region, checked here.
        let frozen = collect(EventStore::read(
            &self.inner,
            &Query::all(),
            ReadOptions::new().to(floor),
        ))
        .await
        .map_err(AppendError::Store)?;
        let conflict = frozen.iter().find(|event| {
            !withheld.holds_position(event.position)
                && condition.is_violated_by(event.position, event.event_type(), event.tags())
        });
        if let Some(conflict) = conflict {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict.position,
            )));
        }

        // Above the floor: the inner store's own atomic evaluation.
        EventStore::append(&self.inner, events, Some(&raised(condition, floor))).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        let withheld = self.snapshot();
        let Some(top) = EventStore::head(&self.inner).await? else {
            return Ok(None);
        };
        if !withheld.holds_position(top) {
            return Ok(Some(top));
        }
        // Bounded by the inner head, so the answer never passes the inner
        // store's own visibility frontier.
        let below = ReadOptions::new().backwards().from(top).limit(1);
        let highest_retained = collect(self.filtered(withheld, &Query::all(), below)).await?;
        Ok(highest_retained.first().map(|event| event.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        if self.snapshot().holds_id(id) {
            return Ok(false);
        }
        EventStore::contains_event_id(&self.inner, id).await
    }
}

/// `condition` with every guard's `after` raised to at least `floor`, so the
/// inner store evaluates it over events above `floor` only.
///
/// The first guard is rebuilt before any other is added, because
/// `after_every_guard_opt` erases the boundary of every guard already present.
/// The queries are cloned into the new, owned condition.
fn raised(condition: &AppendCondition, floor: SequencePosition) -> AppendCondition {
    let lift =
        |after: Option<SequencePosition>| Some(after.map_or(floor, |after| after.max(floor)));
    // `guards()` is documented never empty. Were it empty anyway, the condition
    // could not be violated, and passing it on unchanged keeps that.
    let Some((first, rest)) = condition.guards().split_first() else {
        return condition.clone();
    };
    let base = AppendCondition::new(first.query.clone()).after_every_guard_opt(lift(first.after));
    rest.iter().fold(base, |raised, guard| {
        raised.and_guard(guard.query.clone(), lift(guard.after))
    })
}

/// The stream a [`WithholdingStore`] hands back.
///
/// Every field is `Unpin` (`Pin<Box<St>>` is whatever `St` is), so `poll_next`
/// needs no projection.
#[derive(Debug)]
struct Withholding<St> {
    inner: Pin<Box<St>>,
    withheld: Arc<Withheld>,
    /// How many more retained events may be yielded, or `None` for no limit.
    remaining: Option<usize>,
}

impl<St, E> Stream for Withholding<St>
where
    St: Stream<Item = Result<SequencedEvent, E>>,
{
    type Item = Result<SequencedEvent, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        loop {
            if this.remaining == Some(0) {
                return Poll::Ready(None);
            }
            match this.inner.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(event))) if this.withheld.holds_position(event.position) => {}
                Poll::Ready(Some(Ok(event))) => {
                    this.remaining = this.remaining.map(|left| left.saturating_sub(1));
                    return Poll::Ready(Some(Ok(event)));
                }
                // An error is passed on and not counted against `limit`.
                other => return other,
            }
        }
    }
}
