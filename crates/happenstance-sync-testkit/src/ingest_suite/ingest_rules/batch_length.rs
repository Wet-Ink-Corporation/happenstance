//! VT-24's ingest arrangements: how long each of the rule's three calls is,
//! and the calls themselves.

use alloc::format;
use alloc::vec::Vec;

use happenstance_core::{Event, EventStore, StoreId, StoreLimit};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent};

use super::{
    COMPENSATED, LOCAL, append, ensure_counts, ensure_foreign, ensure_unchanged, held_once, holds,
    ingest, observe, opaque, read_all, shipped,
};
use crate::IngestFixture;
use crate::violation::{Violation, ensure};

/// VT-24, the clause every arrangement here reports under: on the ingest path
/// the batch is the call, and a capacity refusal is `ExceedsStoreLimit`.
const VT_24: &str = "VT-24";

/// VT-24's origin, the next free one after SY-40's.
const LENGTH_ORIGIN: StoreId = StoreId::from_bytes([0x7F; 16]);

/// How many compensation events a group that carries compensation carries:
/// two, so the second sits behind the compensation's head, as SY-18's rule
/// puts its sized one.
const COMPENSATION: usize = 2;

/// The shortest ceiling the control can sit exactly at: a foreign event in
/// each of its two groups, and the second group's compensation.
pub(super) const SHORTEST_LOCATABLE: usize = 2 + COMPENSATION;

/// The length of one ingest group: its foreign events, and whether it
/// carries [`COMPENSATION`] compensation events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Length {
    foreign: usize,
    compensated: bool,
}

impl Length {
    /// Every event the group carries, compensation included.
    const fn total(self) -> usize {
        self.foreign + if self.compensated { COMPENSATION } else { 0 }
    }
}

/// The control: two groups, the second compensated, `ceiling` events in all.
/// `ceiling` is at least [`SHORTEST_LOCATABLE`], so each group has a foreign
/// event.
fn control(ceiling: usize) -> [Length; 2] {
    let foreign = ceiling.saturating_sub(COMPENSATION);
    let first = foreign / 2;
    [
        Length {
            foreign: first,
            compensated: false,
        },
        Length {
            foreign: foreign.saturating_sub(first),
            compensated: true,
        },
    ]
}

/// Arrangement A: two groups without compensation, each at or under
/// `ceiling`, one event over it together.
fn spread_over(ceiling: usize) -> [Length; 2] {
    let over = ceiling.saturating_add(1);
    let first = over.div_ceil(2);
    [
        Length {
            foreign: first,
            compensated: false,
        },
        Length {
            foreign: over.saturating_sub(first),
            compensated: false,
        },
    ]
}

/// Arrangement B: one group whose foreign events are under `ceiling` and
/// whose compensation takes the call one event over it.
fn over_in_compensation(ceiling: usize) -> Length {
    Length {
        foreign: ceiling.saturating_add(1).saturating_sub(COMPENSATION),
        compensated: true,
    }
}

/// One group's events: its foreign events and its compensation.
type Built = (Vec<ReplicatedEvent>, Vec<Event>);

/// Builds the rule's events: foreign ones at origin positions that run upward
/// across every call, so no call is a re-delivery of another's, and every
/// event with bytes of its own, so it is found without its identity. None is
/// tagged, so no tag ceiling is approached.
struct Calls {
    next: u64,
}

impl Calls {
    /// Bytes no other event of the rule carries, and the step past them.
    fn fresh(&mut self) -> Vec<u8> {
        let data = format!("vt24-{}", self.next);
        self.next = self.next.saturating_add(1);
        data.into_bytes()
    }

    /// Every group of `lengths`, which must carry `total` events in all.
    fn build(&mut self, lengths: &[Length], total: usize) -> Result<Vec<Built>, Violation> {
        let carried: usize = lengths.iter().map(|length| length.total()).sum();
        ensure(carried == total, VT_24, || {
            format!("the rule's own call carries {carried} events, not the {total} it claims")
        })?;
        let mut built = Vec::with_capacity(lengths.len());
        for length in lengths {
            let mut events = Vec::with_capacity(length.foreign);
            for _ in 0..length.foreign {
                let position = self.next;
                let millis = 1_800_000_000_000_i64.saturating_add_unsigned(position);
                events.push(shipped(
                    LENGTH_ORIGIN,
                    position,
                    millis,
                    &self.fresh(),
                    VT_24,
                )?);
            }
            let compensation = if length.compensated {
                (0..COMPENSATION)
                    .map(|_| opaque(COMPENSATED, &self.fresh(), VT_24))
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                Vec::new()
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

/// A declared `ceiling` the control can sit exactly at.
///
/// # Errors
///
/// A [`Violation`] of VT-24 for a ceiling under [`SHORTEST_LOCATABLE`].
pub(super) fn ensure_locatable(ceiling: usize) -> Result<(), Violation> {
    ensure(ceiling >= SHORTEST_LOCATABLE, VT_24, || {
        format!(
            "this fixture states an events-per-call ceiling of {ceiling} events, under the \
             {SHORTEST_LOCATABLE} the rule needs to land a call of two groups and a compensation \
             exactly at it, and far under VT-24's floor of 128 events a call"
        )
    })
}

/// VT-24's three arrangements against one fresh `fixture` and its declared
/// `ceiling`.
///
/// # Errors
///
/// A [`Violation`] of VT-24: see the rule.
pub(super) async fn the_call_is_held_to<F: IngestFixture>(
    fixture: &F,
    ceiling: usize,
) -> Result<(), Violation> {
    let store = fixture.connect().await;
    ensure_foreign(&store, &[LENGTH_ORIGIN], VT_24).await?;
    append(&store, &[opaque(LOCAL, b"vt24-local", VT_24)?], VT_24).await?;
    let mut calls = Calls { next: 1 };
    let over = ceiling.saturating_add(1);

    // The control: exactly at the ceiling, so a store that refused everything
    // would not pass, and one off by one is seen.
    let built = calls.build(&control(ceiling), ceiling)?;
    let landed = ingest(&store, &groups(&built), VT_24).await?;
    let foreign = built.iter().map(|(events, _)| events.len()).sum();
    ensure_counts(
        landed,
        (foreign, 0, COMPENSATION),
        "a call exactly at its events-per-call ceiling",
        VT_24,
    )?;
    let log = read_all(&store, VT_24).await?;
    for (events, compensation) in &built {
        for written in events
            .last()
            .map(|event| &event.event)
            .into_iter()
            .chain(compensation.last())
        {
            held_once(&log, written, VT_24)?;
        }
    }
    let before = observe(&store, VT_24).await?;

    // A: no group over the ceiling on its own, so a store that holds each
    // group to it apart writes the call.
    let built = calls.build(&spread_over(ceiling), over)?;
    let refused = store.ingest(&groups(&built)).await;
    let what = "a call of two groups, each at or under the ceiling, one event over it together";
    ensure_too_long(refused, over, what)?;
    ensure_unchanged(&store, &before, what, VT_24).await?;
    ensure_none_held(&store, &built, what).await?;

    // B: the foreign events under the ceiling, so a store that never counts
    // the compensation it writes itself writes the call.
    let built = calls.build(&[over_in_compensation(ceiling)], over)?;
    let refused = store.ingest(&groups(&built)).await;
    let what = "a call whose compensation alone takes it one event over the ceiling";
    ensure_too_long(refused, over, what)?;
    ensure_unchanged(&store, &before, what, VT_24).await?;
    ensure_none_held(&store, &built, what).await
}

/// No group of the refused call `built` left its first or last foreign event
/// held. The whole log is `ensure_unchanged`'s; this is the identity half.
async fn ensure_none_held<S: EventStore>(
    store: &S,
    built: &[Built],
    what: &str,
) -> Result<(), Violation> {
    for (events, _) in built {
        for event in events.first().into_iter().chain(events.last()) {
            ensure(!holds(store, event.id, VT_24).await?, VT_24, || {
                format!("{what} was refused and left {} held", event.id)
            })?;
        }
    }
    Ok(())
}

/// `outcome` is the refusal of a call of `len` events as
/// [`StoreLimit::EventsPerBatch`].
///
/// `len` is pinned, as VT-36's ingest rule pins its total: every call the rule
/// refuses carries exactly one event over the ceiling. The class needs no
/// check of its own, because `IngestError::class` derives `Split` from the
/// limit.
fn ensure_too_long<E: core::error::Error>(
    outcome: Result<Ingested, IngestError<E>>,
    len: usize,
    arrangement: &str,
) -> Result<(), Violation> {
    let error = match outcome {
        Ok(ingested) => {
            return Err(Violation::new(
                VT_24,
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
    let limit = StoreLimit::EventsPerBatch;
    ensure(named == Some((limit, len)), VT_24, || {
        format!(
            "{arrangement} was refused as `{error}`, not as the {limit} limit exceeded by {len}; \
             VT-25 makes a capacity refusal one a runner can tell from a retry, and only \
             `EventsPerBatch` tells it to halve the call"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{
        COMPENSATION, Length, SHORTEST_LOCATABLE, control, over_in_compensation, spread_over,
    };

    fn total(lengths: &[Length]) -> usize {
        lengths.iter().map(|length| length.total()).sum()
    }

    /// Each arrangement carries what the rule claims, at every ceiling from
    /// the shortest it locates, and no group of an over call is over alone.
    #[test]
    fn the_arrangements_carry_what_they_claim() {
        for ceiling in SHORTEST_LOCATABLE..300 {
            let at = control(ceiling);
            assert_eq!(total(&at), ceiling, "control at {ceiling}");
            let [first, second] = at;
            assert!(first.foreign > 0 && second.foreign > 0, "{at:?}");
            assert!(second.compensated && !first.compensated, "{at:?}");

            let a = spread_over(ceiling);
            assert_eq!(total(&a), ceiling + 1, "A at {ceiling}");
            assert!(a.iter().all(|length| length.total() <= ceiling), "{a:?}");

            let b = over_in_compensation(ceiling);
            assert_eq!(b.total(), ceiling + 1, "B at {ceiling}");
            assert_eq!(b.foreign, ceiling + 1 - COMPENSATION, "B at {ceiling}");
        }
    }
}
