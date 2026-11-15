//! The adjudication hook: the merge rule a runner applies to what it ingests
//! (SY-7, SY-10, #450).
//!
//! # Where a rule is mounted is the direction it judges
//!
//! An edge has two ingest sites, one per direction. What the local side pulls
//! lands through [`Edge::step`](super::Edge::step), so a rule given to
//! [`Edge::with_adjudicator`](super::Edge::with_adjudicator) judges the pull.
//! What a far side is pushed lands through
//! [`StoreSyncPeer`](crate::StoreSyncPeer)'s push, so a rule given to
//! [`StoreSyncPeer::with_adjudicator`](crate::StoreSyncPeer::with_adjudicator)
//! judges the push. Nothing else judges anything: an edge never judges what it
//! pushes, and a store peer never judges what it is pulled from. So the two
//! directions of one edge take two rules, and no role parameter appears
//! anywhere (SY-9). [`Accept`], the default at both sites, judges nothing,
//! which is every peer SY-7 has not made a family's adjudicator.
//!
//! # Decide, never write
//!
//! A rule returns the compensation a group lands with and writes nothing
//! itself. The runner lands the group and its compensation in **one**
//! [`IngestGroup`](crate::IngestGroup), so they commit together or not at all
//! (SY-2), and a compensation committed apart from the claim it answers cannot
//! be written through this trait at all. The alternative that lost was a hook
//! handed the store, which makes that forbidden state one forgotten
//! transaction away.
//!
//! # Why `earlier` is there
//!
//! A pulled batch lands in one ingest call, after every group in it has been
//! judged. A rule that reads the local store therefore sees none of the
//! batch: two claims on one seat in one batch both look uncontested against
//! the log. `earlier` is the groups of the same batch that land before the one
//! being judged, so the rule sees the contest the store cannot show it yet.
//!
//! # Flavours
//!
//! Defined without a `Send` bound, as the ports are, and `trait_variant`
//! derives [`SendAdjudicate`], whose future is `Send` (ADR-0001). A rule that
//! may cross threads implements [`SendAdjudicate`] and gets this trait from
//! the blanket impl. Bind `Adjudicate` in generic code: it is the weaker
//! requirement and accepts both (CLAUDE.md rule 4).

use alloc::vec::Vec;
use core::convert::Infallible;

use happenstance_core::Event;

use crate::classify::Classify;
use crate::ingest::IngestGroup;
use crate::peer::EventGroup;

/// A merge rule: what one direction of an edge writes in answer to a group it
/// ingests (SY-7).
///
/// [`compensation`](Self::compensation) is asked once per incoming group, in
/// batch order, before anything in the batch is written. What it returns is
/// minted by the receiving store as its own events and lands in the same
/// atomic write as the group (SY-2); an empty answer lands the group unjudged.
/// A group the store already holds every event of is a skip, and its
/// compensation is not written again (SY-11), so a rule may answer a
/// re-delivered group as it answered the first delivery.
///
/// A rule holds whatever it reads its judgment from, typically its own read
/// handle onto the receiving store, so the runner's bounds on the store do not
/// grow. It judges by fact family (SY-35); the group's
/// [`guard`](EventGroup::guard) is evidence it may read, never an instruction
/// (SY-6).
///
/// # Examples
///
/// A rule that reverses the second claim on a seat in one batch, judging from
/// `earlier` alone:
///
/// ```
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use core::convert::Infallible;
///
/// use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
/// use happenstance_sync::runner::Adjudicate;
/// use happenstance_sync::{EventGroup, ReplicatedEvent};
///
/// /// Holds the reversal it answers with, built once.
/// struct FirstClaimWins {
///     reversal: Event,
/// }
///
/// fn claims(group: &EventGroup) -> bool {
///     group
///         .events
///         .iter()
///         .any(|event| event.event.event_type().as_str() == "SeatClaimed")
/// }
///
/// impl Adjudicate for FirstClaimWins {
///     type Error = Infallible;
///
///     async fn compensation(
///         &self,
///         group: &EventGroup,
///         earlier: &[EventGroup],
///     ) -> Result<Vec<Event>, Infallible> {
///         if claims(group) && earlier.iter().any(claims) {
///             return Ok(vec![self.reversal.clone()]);
///         }
///         Ok(Vec::new())
///     }
/// }
///
/// let claim = |origin: u8| -> Result<EventGroup, Box<dyn core::error::Error>> {
///     let event = Event::new("SeatClaimed", &b"{}"[..])?;
///     let id = EventId::new(StoreId::from_bytes([origin; 16]), SequencePosition::FIRST);
///     let at = RecordedAt::from_millis(1_700_000_000_000);
///     Ok(EventGroup::new(None, vec![ReplicatedEvent::new(id, at, event)]))
/// };
/// let first = claim(1)?;
/// let second = claim(2)?;
/// let rule = FirstClaimWins {
///     reversal: Event::new("ClaimReversed", &b"{}"[..])?,
/// };
///
/// assert!(rule.compensation(&first, &[]).await?.is_empty());
/// let earlier = [first];
/// assert_eq!(rule.compensation(&second, &earlier).await?.len(), 1);
/// # Ok(())
/// # }
/// ```
#[trait_variant::make(SendAdjudicate: Send)]
pub trait Adjudicate {
    /// How judging fails, stating a class so the runner reacts to it as to any
    /// other failure (SY-36).
    type Error: core::error::Error + Classify + 'static;

    /// The compensation `group` lands with: empty to land it unjudged.
    ///
    /// `earlier` is the groups of the same batch that land before `group`, in
    /// the same ingest call, and so are not yet in the receiving store.
    ///
    /// # Errors
    ///
    /// The rule's own error, when it cannot decide: say, its read of the
    /// receiving store failed. Nothing in the batch is written, and its class
    /// says what the runner does next.
    async fn compensation(
        &self,
        group: &EventGroup,
        earlier: &[EventGroup],
    ) -> Result<Vec<Event>, Self::Error>;
}

/// The rule that judges nothing: every group lands unjudged.
///
/// The default on every [`Edge`](super::Edge) and every
/// [`StoreSyncPeer`](crate::StoreSyncPeer), and the rule of every peer SY-7
/// has not made a family's adjudicator. It never fails, so its error is
/// [`Infallible`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Accept;

impl SendAdjudicate for Accept {
    type Error = Infallible;

    /// No compensation, whatever the group.
    ///
    /// # Errors
    ///
    /// Never.
    async fn compensation(
        &self,
        _group: &EventGroup,
        _earlier: &[EventGroup],
    ) -> Result<Vec<Event>, Infallible> {
        Ok(Vec::new())
    }
}

/// One incoming group, paired with the compensation its rule answered.
///
/// A pair rather than two vectors zipped later, so a group and its answer
/// cannot come apart, and a short list of answers cannot drop a group from the
/// ingest while the pull token advances past it.
#[derive(Debug)]
pub(crate) struct Judged<'a> {
    pub(crate) group: &'a EventGroup,
    pub(crate) compensation: Vec<Event>,
}

impl Judged<'_> {
    /// The group as one atomic unit of the ingest: its events, its
    /// compensation in the same write (SY-2), and its guard (SY-30).
    pub(crate) fn ingest_group(&self) -> IngestGroup<'_> {
        IngestGroup::new(&self.group.events, &self.compensation)
            .with_guard(self.group.guard.as_ref())
    }
}

/// Asks `rule` for each group's compensation, in batch order, each with the
/// groups before it, and hands every group back paired with its answer.
///
/// One function for both ingest sites, so the edge and the store peer cannot
/// disagree about what `earlier` holds. The group and its prefix come from the
/// same split of the batch, and the loop ends exactly when no group is left,
/// so every group is judged once.
pub(crate) async fn judge<'a, A: Adjudicate>(
    rule: &A,
    groups: &'a [EventGroup],
) -> Result<Vec<Judged<'a>>, A::Error> {
    let mut judged = Vec::with_capacity(groups.len());
    while let Some((earlier, [group, ..])) = groups.split_at_checked(judged.len()) {
        let compensation = rule.compensation(group, earlier).await?;
        judged.push(Judged {
            group,
            compensation,
        });
    }
    Ok(judged)
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::cell::RefCell;
    use core::convert::Infallible;

    use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};

    use super::{Adjudicate, judge};
    use crate::identity::ReplicatedEvent;
    use crate::peer::EventGroup;

    /// Records the length of `earlier` it was handed for each group, and
    /// answers each group with one event per group before it.
    #[derive(Default)]
    struct Recording(RefCell<Vec<usize>>);

    impl Adjudicate for Recording {
        type Error = Infallible;

        async fn compensation(
            &self,
            _group: &EventGroup,
            earlier: &[EventGroup],
        ) -> Result<Vec<Event>, Infallible> {
            self.0.borrow_mut().push(earlier.len());
            Ok(earlier
                .iter()
                .map(|_| Event::new("Answered", &b"{}"[..]).expect("a static type is valid"))
                .collect())
        }
    }

    fn batch(len: u8) -> Vec<EventGroup> {
        (1..=len)
            .map(|origin| {
                let id = EventId::new(StoreId::from_bytes([origin; 16]), SequencePosition::FIRST);
                let event = Event::new("Claimed", &b"{}"[..]).expect("a static type is valid");
                let at = RecordedAt::from_millis(1_800_000_000_000);
                EventGroup::new(None, Vec::from([ReplicatedEvent::new(id, at, event)]))
            })
            .collect()
    }

    /// Every group comes back paired with its own answer, in batch order,
    /// each judged with exactly the groups before it: none, one, or many.
    /// Rejects a judgement that drops a group, pairs an answer with the wrong
    /// group, or hands a group a prefix other than its own.
    #[tokio::test]
    async fn every_group_is_judged_once_with_its_own_prefix_and_paired_with_its_answer() {
        for len in [0_u8, 1, 5] {
            let groups = batch(len);
            let rule = Recording::default();

            let judged = judge(&rule, &groups).await.expect("an infallible rule");

            assert_eq!(judged.len(), groups.len(), "{len} groups");
            for (at, (pair, group)) in judged.iter().zip(&groups).enumerate() {
                assert!(core::ptr::eq(pair.group, group), "group {at} of {len}");
                assert_eq!(pair.compensation.len(), at, "group {at}'s own answer");
            }
            let expected: Vec<usize> = (0..usize::from(len)).collect();
            assert_eq!(*rule.0.borrow(), expected, "the prefixes handed out");
        }
    }
}
