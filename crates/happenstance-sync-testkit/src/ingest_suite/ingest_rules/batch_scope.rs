//! VT-36's ingest arrangements: where one call's tags go, and the three calls
//! the rule makes with them.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use happenstance_core::{
    Event, EventId, EventStore, EventType, RecordedAt, SequencePosition, StoreId, StoreLimit, Tag,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent,
};

use super::{
    COMPENSATED, LOCAL, append, ensure_counts, ensure_foreign, ensure_unchanged, held_once, holds,
    ingest, observe, opaque, read_all,
};
use crate::violation::{Violation, ensure};
use crate::{IngestFixture, MAX_PROVOKED_TAGS};

/// VT-36, the clause every arrangement here reports under.
const VT_36: &str = "VT-36";

/// VT-36's origin, the next free one after SY-18's.
const BATCH_ORIGIN: StoreId = StoreId::from_bytes([0x79; 16]);

// The event types that tell the rule's tagged events apart.
const BATCH_FOREIGN: EventType = EventType::from_static("BatchForeign");
const AT_COMPENSATION: EventType = EventType::from_static("BatchAtCompensation");
const OVER_COMPENSATION: EventType = EventType::from_static("BatchOverCompensation");

/// How many tags each event of the rule's calls carries, after checking that
/// the fixture's `ceiling` is coherent with its per-event one (VT-36).
///
/// One under a declared per-event ceiling, so no event is at its own ceiling
/// and a store off by one there fails SY-18's rule rather than this one; at
/// most [`MAX_PROVOKED_TAGS`]; and at most half the ceiling, so the
/// at-ceiling call, not only the over one, spans two events or more.
///
/// # Errors
///
/// A [`Violation`] of VT-36 for a ceiling of `usize::MAX`, which has no total
/// past it, or one below the fixture's own per-event ceiling.
pub(super) fn per_event<F: IngestFixture>(ceiling: usize) -> Result<usize, Violation> {
    ensure(ceiling < usize::MAX, VT_36, || {
        String::from(
            "this fixture states a tags-per-batch ceiling of usize::MAX, which has no total past \
             it to refuse; a ceiling that cannot be crossed is not a ceiling, and `None` is the \
             honest declaration",
        )
    })?;
    if let Some(stated) = F::MAX_TAGS_PER_EVENT {
        ensure(ceiling >= stated, VT_36, || {
            format!(
                "this fixture states a tags-per-batch ceiling of {ceiling}, below its own ceiling \
                 of {stated} tags per event; an event within its own ceiling would then be \
                 refused as a batch of one, as `TagsPerBatch`, which a sync runner splits, and a \
                 batch of one has nothing left to split"
            )
        })?;
    }
    Ok(width(ceiling, F::MAX_TAGS_PER_EVENT))
}

/// [`per_event`]'s number, once the ceiling is known to be coherent.
fn width(ceiling: usize, stated: Option<usize>) -> usize {
    stated
        .map_or(MAX_PROVOKED_TAGS, |stated| stated.saturating_sub(1))
        .min(MAX_PROVOKED_TAGS)
        .min(ceiling.div_ceil(2))
        .max(1)
}

/// The tag counts of one ingest group: each foreign event's, then, when the
/// group carries compensation, its tagged compensation event's. The
/// compensation also carries an untagged event ahead of that one, so the
/// tagged one sits behind its head, as SY-18's rule puts its sized one.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape {
    foreign: Vec<usize>,
    compensation: Option<usize>,
}

impl Shape {
    /// `foreign` tags laid out `per_event` to an event, and `compensation`.
    fn new(foreign: usize, compensation: Option<usize>, per_event: usize) -> Self {
        Self {
            foreign: lay_out(foreign, per_event),
            compensation,
        }
    }

    /// Every tag the group carries, compensation included. The rule checks
    /// the events it built instead, which `Tags`' deduplication could shrink.
    #[cfg(test)]
    fn total(&self) -> usize {
        self.foreign.iter().chain(&self.compensation).sum()
    }
}

/// `total` tags laid out `per_event` to an event, the remainder last: each
/// event's count. One untagged event when `total` is 0, so a group is never
/// empty.
fn lay_out(total: usize, per_event: usize) -> Vec<usize> {
    let width = per_event.max(1);
    let mut counts = Vec::with_capacity(total.div_ceil(width).max(1));
    let mut left = total;
    while left > 0 {
        let here = left.min(width);
        counts.push(here);
        left -= here;
    }
    if counts.is_empty() {
        counts.push(0);
    }
    counts
}

/// The control: two groups whose tags, the second's compensation included,
/// sum to exactly `ceiling`.
fn control(ceiling: usize, per_event: usize) -> [Shape; 2] {
    let compensation = per_event.min(ceiling / 2);
    let foreign = ceiling.saturating_sub(compensation);
    let first = foreign / 2;
    [
        Shape::new(first, None, per_event),
        Shape::new(foreign.saturating_sub(first), Some(compensation), per_event),
    ]
}

/// Arrangement A: two groups without compensation, each at or under
/// `ceiling`, one tag over it together.
fn spread_over(ceiling: usize, per_event: usize) -> [Shape; 2] {
    let over = ceiling.saturating_add(1);
    let first = over.div_ceil(2);
    [
        Shape::new(first, None, per_event),
        Shape::new(over.saturating_sub(first), None, per_event),
    ]
}

/// Arrangement B: one group whose foreign events are at or under `ceiling`
/// and whose compensation takes the call one tag over it.
fn over_in_compensation(ceiling: usize, per_event: usize) -> Shape {
    let compensation = per_event.min(ceiling).max(1);
    Shape::new(
        ceiling.saturating_add(1).saturating_sub(compensation),
        Some(compensation),
        per_event,
    )
}

/// One group's events, built from a [`Shape`]: its foreign events and its
/// compensation.
type Built = (Vec<ReplicatedEvent>, Vec<Event>);

/// Builds the rule's events: foreign ones at origin positions that run upward
/// across every call, so no call is a re-delivery of another's, each with
/// bytes of its own so it is found without its identity.
struct Calls<'n> {
    /// `per_event` distinct tag names, each event cloning the first of them it
    /// needs: the store owns each event's tags, so each clone is data the
    /// store holds.
    names: &'n [Tag],
    /// The next origin position, and the suffix that makes bytes distinct.
    next: u64,
}

impl Calls<'_> {
    /// An event of `event_type` carrying `count` tags and bytes no other
    /// event of the rule carries.
    fn tagged(&mut self, event_type: EventType, count: usize) -> Result<Event, Violation> {
        let tags = self.names.get(..count).ok_or_else(|| {
            Violation::new(
                VT_36,
                format!("the rule's own event needs {count} tags and built fewer"),
            )
        })?;
        let data = format!("vt36-{}", self.next);
        self.next = self.next.saturating_add(1);
        let event = opaque(event_type, data.as_bytes(), VT_36)?;
        Ok(event.with_tags(tags.iter().cloned().collect()))
    }

    /// A foreign event from [`BATCH_ORIGIN`] carrying `count` tags.
    fn shipped(&mut self, count: usize) -> Result<ReplicatedEvent, Violation> {
        let position = SequencePosition::new(self.next)
            .ok_or_else(|| Violation::new(VT_36, "the rule's origin position is zero"))?;
        Ok(ReplicatedEvent::new(
            EventId::new(BATCH_ORIGIN, position),
            RecordedAt::from_millis(1_800_000_000_000),
            self.tagged(BATCH_FOREIGN, count)?,
        ))
    }

    /// Every group of `shapes`, a tagged compensation event of
    /// `compensation_type`, which must carry `total` tags in all.
    ///
    /// The total is counted on the built events rather than the shapes,
    /// because `Tags` deduplicates: a tag generator that collided would leave
    /// the rule testing a smaller call than it claims.
    fn build(
        &mut self,
        shapes: &[Shape],
        compensation_type: &EventType,
        total: usize,
    ) -> Result<Vec<Built>, Violation> {
        let built = self.build_unchecked(shapes, compensation_type)?;
        let carried: usize = built
            .iter()
            .flat_map(|(events, compensation)| {
                events.iter().map(|event| &event.event).chain(compensation)
            })
            .map(|event| event.tags().len())
            .sum();
        ensure(carried == total, VT_36, || {
            format!("the rule's own call carries {carried} tags, not the {total} it claims")
        })?;
        Ok(built)
    }

    /// [`build`](Self::build)'s events, before their total is counted.
    fn build_unchecked(
        &mut self,
        shapes: &[Shape],
        compensation_type: &EventType,
    ) -> Result<Vec<Built>, Violation> {
        let mut built = Vec::with_capacity(shapes.len());
        for shape in shapes {
            let events = shape
                .foreign
                .iter()
                .map(|count| self.shipped(*count))
                .collect::<Result<Vec<_>, _>>()?;
            let compensation = match shape.compensation {
                // The type is one of this module's `from_static` constants, so
                // the clone copies a borrowed `&'static str`, nothing more.
                Some(count) => Vec::from([
                    self.tagged(COMPENSATED, 0)?,
                    self.tagged(compensation_type.clone(), count)?,
                ]),
                None => Vec::new(),
            };
            built.push((events, compensation));
        }
        Ok(built)
    }
}

/// The call `built` makes, as the port takes it.
fn groups(built: &[Built]) -> Vec<IngestGroup<'_>> {
    built
        .iter()
        .map(|(events, compensation)| IngestGroup::new(events, compensation))
        .collect()
}

/// The `per_event` distinct tag names every event of the rule draws on.
fn names(per_event: usize) -> Result<Vec<Tag>, Violation> {
    (0..per_event)
        .map(|i| Tag::new(format!("t:{i}")))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| Violation::new(VT_36, format!("the rule's own tag is invalid: {error}")))
}

/// VT-36's three arrangements against one fresh `fixture`, its ceiling
/// `ceiling`, its events carrying at most `per_event` tags.
///
/// # Errors
///
/// A [`Violation`] of VT-36: see the rule.
pub(super) async fn the_call_is_held_to<F: IngestFixture>(
    fixture: &F,
    ceiling: usize,
    per_event: usize,
) -> Result<(), Violation> {
    let store = fixture.connect().await;
    ensure_foreign(&store, &[BATCH_ORIGIN], VT_36).await?;
    append(&store, &[opaque(LOCAL, b"vt36-local", VT_36)?], VT_36).await?;
    let names = names(per_event)?;
    let mut calls = Calls {
        names: &names,
        next: 1,
    };
    let over = ceiling.saturating_add(1);

    // The control, dropped before either over call exists: at half a million
    // tags, two calls alive at once is most of an isolate.
    let built = calls.build(&control(ceiling, per_event), &AT_COMPENSATION, ceiling)?;
    let landed = ingest(&store, &groups(&built), VT_36).await?;
    let foreign = built.iter().map(|(events, _)| events.len()).sum();
    ensure_counts(
        landed,
        (foreign, 0, 2),
        "a call exactly at its tags-per-batch ceiling",
        VT_36,
    )?;
    let log = read_all(&store, VT_36).await?;
    for (events, compensation) in &built {
        for written in events
            .last()
            .map(|event| &event.event)
            .into_iter()
            .chain(compensation.last())
        {
            held_once(&log, written, VT_36)?;
        }
    }
    drop(log);
    drop(built);
    let before = observe(&store, VT_36).await?;

    // A: no group over the ceiling on its own, so a store that holds each
    // group to it apart writes the call.
    let built = calls.build(&spread_over(ceiling, per_event), &OVER_COMPENSATION, over)?;
    let refused = store.ingest(&groups(&built)).await;
    let what = "a call of two groups, each at or under the ceiling, one tag over it together";
    ensure_split(refused, over, what)?;
    ensure_unchanged(&store, &before, what, VT_36).await?;
    ensure_none_held(&store, &built, what).await?;
    drop(built);

    // B: the foreign events at or under the ceiling, so a store that never
    // counts the compensation it writes itself writes the call.
    let built = calls.build(
        &[over_in_compensation(ceiling, per_event)],
        &OVER_COMPENSATION,
        over,
    )?;
    let refused = store.ingest(&groups(&built)).await;
    let what = "a call whose compensation alone takes it one tag over the ceiling";
    ensure_split(refused, over, what)?;
    ensure_unchanged(&store, &before, what, VT_36).await?;
    ensure_none_held(&store, &built, what).await
}

/// No group of the refused call `built` left its first or last foreign event
/// held. The whole log is `ensure_unchanged`'s; this is the identity half,
/// asked of the ends only, because each ask is one store round trip.
async fn ensure_none_held<S: EventStore>(
    store: &S,
    built: &[Built],
    what: &str,
) -> Result<(), Violation> {
    for (events, _) in built {
        for event in events.first().into_iter().chain(events.last()) {
            ensure(!holds(store, event.id, VT_36).await?, VT_36, || {
                format!("{what} was refused and left {} held", event.id)
            })?;
        }
    }
    Ok(())
}

/// `outcome` is the refusal of a call of `len` tags as
/// [`StoreLimit::TagsPerBatch`], and states `Split`.
///
/// `len` is pinned: every call the rule refuses carries exactly one tag over
/// the ceiling, so a running sum past it is the whole sum, whether the store
/// counts every event or stops at the first total past its ceiling.
fn ensure_split<E: core::error::Error + Classify>(
    outcome: Result<Ingested, IngestError<E>>,
    len: usize,
    arrangement: &str,
) -> Result<(), Violation> {
    let error = match outcome {
        Ok(ingested) => {
            return Err(Violation::new(
                VT_36,
                format!(
                    "the store accepted {arrangement} as {ingested:?}; it declares that ceiling, \
                     and on the ingest path the batch is the call: every group and every \
                     compensation count together"
                ),
            ));
        }
        Err(error) => error,
    };
    let named = match &error {
        IngestError::ExceedsStoreLimit {
            limit: refused,
            len: measured,
            ..
        } => Some((*refused, *measured)),
        _ => None,
    };
    let limit = StoreLimit::TagsPerBatch;
    ensure(named == Some((limit, len)), VT_36, || {
        format!(
            "{arrangement} was refused as `{error}`, not as the {limit} limit exceeded by {len}; \
             VT-25 makes a capacity refusal one a runner can tell from a retry, and a store \
             whose call-length ceiling is under {len} tags at the rule's spread refuses this \
             call under that limit first"
        )
    })?;
    ensure(error.class() == FailureClass::Split, VT_36, || {
        format!(
            "the refusal of {arrangement} states {:?}; a call over its tags-per-batch ceiling is \
             Split (ADR-0100, decision 3), so a runner halves it",
            error.class()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{Shape, control, lay_out, over_in_compensation, spread_over};

    /// Every layout sums to what it was asked for, no event over the width,
    /// and nothing at all is one untagged event.
    #[test]
    fn lay_out_spreads_exactly() {
        assert_eq!(lay_out(0, 3), [0]);
        for total in 0..=300 {
            for per_event in 1..=20 {
                let counts = lay_out(total, per_event);
                assert_eq!(
                    counts.iter().sum::<usize>(),
                    total,
                    "{total} by {per_event}"
                );
                assert!(
                    counts.iter().all(|count| *count <= per_event),
                    "{total} by {per_event}: {counts:?}"
                );
                assert!(!counts.is_empty(), "{total} by {per_event}");
            }
        }
    }

    /// Each arrangement's total is the one it claims, and nothing in it is
    /// over a limit other than the call's.
    #[test]
    fn every_arrangement_is_where_it_claims() {
        let total = |shapes: &[Shape]| shapes.iter().map(Shape::total).sum::<usize>();
        let within = |shape: &Shape, per_event: usize| {
            shape.foreign.iter().all(|count| *count <= per_event)
                && shape.compensation.is_none_or(|count| count <= per_event)
        };
        for ceiling in 1..=200 {
            for per_event in 1..=ceiling.min(20) {
                let at = control(ceiling, per_event);
                assert_eq!(total(&at), ceiling, "control at {ceiling} by {per_event}");
                assert!(at.iter().all(|shape| within(shape, per_event)));
                assert!(
                    at.iter().any(|shape| shape.compensation.is_some()),
                    "the control carries compensation"
                );

                let over = spread_over(ceiling, per_event);
                assert_eq!(total(&over), ceiling + 1, "A at {ceiling} by {per_event}");
                assert!(over.iter().all(|shape| shape.total() <= ceiling));
                assert!(over.iter().all(|shape| within(shape, per_event)));

                let lone = over_in_compensation(ceiling, per_event);
                assert_eq!(lone.total(), ceiling + 1, "B at {ceiling} by {per_event}");
                assert!(lone.foreign.iter().sum::<usize>() <= ceiling);
                assert!(lone.compensation.is_some_and(|count| count >= 1));
                assert!(within(&lone, per_event));
            }
        }
    }
}
