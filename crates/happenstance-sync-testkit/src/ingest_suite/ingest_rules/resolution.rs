//! SY-2's authoring half: a conflict adjudicated as the runner adjudicates
//! it, and a reader watching the store while the answer lands (#320).
//!
//! # What the transport's half leaves
//!
//! `compensation_is_atomic_with_the_losing_event` holds a peer to carrying a
//! group and its compensation together. SY-30's rule holds a store to landing
//! them together when its fixture can arm a fault between two rows. Neither
//! sees a store that commits the losing event and then the compensation in a
//! later write when nothing faults. No error surfaces, and the log is right
//! once the call returns. The only witness is a reader who reads between the
//! two writes, which is the reader SY-2 is written for.
//!
//! # How the rule reads between two writes
//!
//! [`watched`] polls the ingest and, each time it is pending, starts or
//! continues one read of the store on a second handle, in the same task. A
//! store that commits the losing event and then awaits anything before
//! committing its compensation is read inside that gap. A store that never
//! yields mid-call cannot be read inside the call at all, and it has no gap to
//! read: its one write is atomic with respect to this task. Joining futures
//! rather than spawning tasks keeps the rule free of a runtime and of a `Send`
//! bound, so it runs under every emitter, the `!Send` Durable Object's
//! included.
//!
//! # Why one read decides an observation
//!
//! The losing claim and its reversal are found in **one** read of the log, a
//! snapshot under ES-10. Two reads, one per event, would let a faithful store
//! commit both between them and be reported as having shown one without the
//! other.

use alloc::boxed::Box;
use alloc::format;
use alloc::vec::Vec;
use core::future::{Future, poll_fn};
use core::pin::{Pin, pin};
use core::task::Poll;

use happenstance_core::{
    Event, EventId, EventStore, EventType, Query, QueryItem, RecordedAt, SequencePosition, StoreId,
    Tag, Tags,
};
use happenstance_sync::runner::Adjudicate;
use happenstance_sync::{EventGroup, IngestGroup, IngestStore, ReplicatedEvent};

use super::seat_rule::SeatRule;
use super::{ensure_counts, ensure_foreign, ingest, opaque, read_all};
use crate::IngestFixture;
use crate::violation::{Violation, ensure};

/// SY-2, the clause the rule reports under.
pub(super) const SY_2: &str = "SY-2";

/// The claim already on the seat, and the claim that loses to it. SY-40's
/// rule holds `0x7E`, the batch-length rule `0x7F` and `sync_suite` `0x81`
/// and `0x82`; `0x80` is free.
const INCUMBENT: StoreId = StoreId::from_bytes([0x83; 16]);
const CHALLENGER: StoreId = StoreId::from_bytes([0x84; 16]);

const SEAT_CLAIMED: EventType = EventType::from_static("SeatClaimed");
const CLAIM_REVERSED: EventType = EventType::from_static("ClaimReversed");

/// The fact family the conflict belongs to: the tag key `seat` (SY-35).
const SEAT: &str = "seat:s2";

/// The bytes that identify the losing claim and its answer in a read of the
/// log, compared, never decoded (SY-35).
const LOSER_BYTES: &[u8] = b"sy2-the-losing-claim";
const REVERSAL_BYTES: &[u8] = b"sy2-the-reversal-that-answers-it";

/// What one read of the log showed of the conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seen {
    loser: bool,
    reversal: bool,
}

impl Seen {
    /// The state SY-2 forbids a reader to observe: the losing claim held, with
    /// nothing resolving it.
    const fn unresolved(self) -> bool {
        self.loser && !self.reversal
    }
}

/// The whole arrangement, on one fresh fixture.
///
/// # Errors
///
/// A [`Violation`] of SY-2 if the store cannot say who it is, refuses or
/// miscounts an ingest, does not show the incumbent claim the rule's merge
/// rule must judge against, or is read holding the losing claim without its
/// reversal, during the ingest or after it.
pub(super) async fn a_reader_never_sees_the_loser_alone<F: IngestFixture>(
    open: &impl AsyncFn() -> F,
) -> Result<(), Violation> {
    let fixture = open().await;
    let writer = fixture.connect().await;
    let reader = fixture.connect().await;
    ensure_foreign(&writer, &[INCUMBENT, CHALLENGER], SY_2).await?;

    let family = family()?;
    let reversal = opaque(CLAIM_REVERSED, REVERSAL_BYTES, SY_2)?.with_tags(seat());
    // The hub's rule, read from its own handle as a runner's rule is.
    let rule = SeatRule::new(&reader, &family, &reversal);

    let incumbent = claim(INCUMBENT, b"sy2-the-incumbent-claim")?;
    let (group, answer) = judged(&rule, incumbent).await?;
    ensure(answer.is_empty(), SY_2, || {
        format!(
            "the merge rule answered the first claim on an empty seat with {} events; the \
             rule's own arrangement is wrong",
            answer.len()
        )
    })?;
    let landed = ingest(&writer, &[IngestGroup::new(&group.events, &answer)], SY_2).await?;
    ensure_counts(landed, (1, 0, 0), "the incumbent claim, uncontested", SY_2)?;

    let challenger = claim(CHALLENGER, LOSER_BYTES)?;
    let (group, answer) = judged(&rule, challenger).await?;
    ensure(answer.len() == 1, SY_2, || {
        format!(
            "the merge rule read the store through a second handle after the incumbent claim \
             landed and answered the challenger with {} events, not one reversal: the store \
             did not show a reader the claim it had just taken",
            answer.len()
        )
    })?;

    // The runner's step, as `runner::adjudicate` lands it: the group and the
    // compensation its rule answered, in one `IngestGroup`.
    let groups = [IngestGroup::new(&group.events, &answer)];
    let landing = writer.ingest(&groups);
    let (outcome, during) = watched(landing, || observe(&reader)).await;
    let landed = outcome.map_err(|error| {
        Violation::new(
            SY_2,
            format!("the store refused the losing claim and its reversal: {error}"),
        )
    })?;
    for seen in during {
        if seen?.unresolved() {
            return Err(Violation::new(
                SY_2,
                "a reader read the losing claim held with no reversal, while the ingest that \
                 carries both was still in flight: the store committed the losing event apart \
                 from its compensation, so a slice cut at that moment says the seat is held \
                 twice",
            ));
        }
    }
    ensure_counts(
        landed,
        (1, 0, 1),
        "the losing claim with its reversal",
        SY_2,
    )?;
    let after = observe(&reader).await?;
    ensure(
        after
            == Seen {
                loser: true,
                reversal: true,
            },
        SY_2,
        || {
            format!(
                "once the ingest returned, a reader read {after:?}; the losing claim and its \
             reversal land in one write, so both are held"
            )
        },
    )
}

/// Runs `work` to completion, reading the store with `observe` each time it
/// is pending, and answers its output with every observation that finished.
///
/// At most one observation is in flight. One that finishes while `work` is
/// still pending is not followed by another until `work` is woken, so an
/// observation that is always ready does not spin. One still in flight when
/// `work` finishes is driven to its end and kept: it began inside the call,
/// so what it reads is a state the call let a reader see.
async fn watched<W: Future, O: Future>(
    work: W,
    mut observe: impl FnMut() -> O,
) -> (W::Output, Vec<O::Output>) {
    let mut work = pin!(work);
    let mut in_flight: Option<Pin<Box<O>>> = None;
    let mut seen = Vec::new();
    let output = poll_fn(|cx| {
        if let Poll::Ready(output) = work.as_mut().poll(cx) {
            return Poll::Ready(output);
        }
        let mut observation = in_flight.take().unwrap_or_else(|| Box::pin(observe()));
        match observation.as_mut().poll(cx) {
            Poll::Ready(observed) => seen.push(observed),
            Poll::Pending => in_flight = Some(observation),
        }
        Poll::Pending
    })
    .await;
    if let Some(observation) = in_flight {
        seen.push(observation.await);
    }
    (output, seen)
}

/// One read of the log, and what it showed of the conflict.
async fn observe<S: EventStore>(store: &S) -> Result<Seen, Violation> {
    let log = read_all(store, SY_2).await?;
    let holds = |event_type: &EventType, bytes: &[u8]| {
        log.iter()
            .any(|row| row.event_type() == event_type && row.event.data().as_ref() == bytes)
    };
    Ok(Seen {
        loser: holds(&SEAT_CLAIMED, LOSER_BYTES),
        reversal: holds(&CLAIM_REVERSED, REVERSAL_BYTES),
    })
}

/// One pulled claim, judged by `rule` as the runner judges a group: alone in
/// its batch, so `earlier` is empty.
async fn judged<A: Adjudicate>(
    rule: &A,
    claim: ReplicatedEvent,
) -> Result<(EventGroup, Vec<Event>), Violation> {
    let group = EventGroup::new(None, Vec::from([claim]));
    let answer = rule
        .compensation(&group, &[])
        .await
        .map_err(|error| Violation::new(SY_2, format!("{error}")))?;
    Ok((group, answer))
}

/// `origin`'s claim on the seat, the first event in its log.
fn claim(origin: StoreId, data: &[u8]) -> Result<ReplicatedEvent, Violation> {
    let event = opaque(SEAT_CLAIMED, data, SY_2)?.with_tags(seat());
    Ok(ReplicatedEvent::new(
        EventId::new(origin, SequencePosition::FIRST),
        RecordedAt::from_millis(1_800_000_000_000),
        event,
    ))
}

/// The seat family, which the merge rule judges by (SY-35).
fn family() -> Result<Query, Violation> {
    let item = QueryItem::new([SEAT_CLAIMED], seat()).map_err(|error| {
        Violation::new(SY_2, format!("the rule's own query is invalid: {error}"))
    })?;
    Ok(Query::from_item(item))
}

fn seat() -> Tags {
    [Tag::from_static(SEAT)].into_iter().collect()
}

#[cfg(test)]
mod tests {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    use happenstance_testkit::block_on;

    use super::watched;

    /// Pending `remaining` times, waking itself each time, then ready.
    struct PendingFor(usize);

    impl Future for PendingFor {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.0 == 0 {
                return Poll::Ready(());
            }
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    #[test]
    fn work_that_never_yields_is_never_watched() {
        let (output, seen) = block_on(watched(async { 7 }, || async {}));
        assert_eq!(output, 7);
        assert!(seen.is_empty());
    }

    #[test]
    fn each_pending_poll_of_the_work_starts_one_observation() {
        let ((), seen) = block_on(watched(PendingFor(3), || async {}));
        assert_eq!(seen.len(), 3);
    }

    #[test]
    fn an_observation_in_flight_when_the_work_ends_is_finished_and_kept() {
        // The observation takes five polls; the work ends after two.
        let ((), seen) = block_on(watched(PendingFor(2), || PendingFor(5)));
        assert_eq!(seen.len(), 1);
    }
}
