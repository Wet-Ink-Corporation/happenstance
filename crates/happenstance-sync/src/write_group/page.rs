//! Filling one [`GroupPage`] from rows read in group order: the one spelling
//! of "fill to `max_events`, never cut a group, always return the first".
//!
//! Every adapter reads rows and has to fold them into whole groups under the
//! same budget rule, and every adapter would otherwise write that loop slightly
//! differently, which is [`PullBatchLimit`](crate::PullBatchLimit)'s argument
//! one layer down. Module-qualified, as the guard codec is: only adapters call
//! it.

use alloc::vec::Vec;
use core::num::NonZeroUsize;
use core::ops::ControlFlow;

use happenstance_core::{AppendCondition, SequencePosition};

use super::{GroupPage, WriteGroup};
use crate::classify::{Classify, FailureClass};
use crate::identity::ReplicatedEvent;
use crate::peer::Waiting;

/// Folds rows, offered in group order, into a page of whole groups.
///
/// Offer each row with the group it belongs to. Rows of one group must be
/// offered together, in local position order, and groups in ascending
/// [`first`](WriteGroup::first) order: a store that serialises its writers gets
/// that from a scan by position, and one that does not orders by group.
///
/// The order is checked, not trusted. A row that names a group at or below
/// one already passed is refused: [`offer`](Self::offer) breaks, the builder
/// takes nothing more, and [`finish`](Self::finish) returns a
/// [`GroupOrderError`]. That is the mistake a store that does not serialise its
/// writers makes by scanning in position order, where G = {5, 7} and H = {6}
/// arrive as 5, 6, 7: without the refusal the page would carry G torn in two,
/// under one name twice.
///
/// # Examples
///
/// ```
/// use core::num::NonZeroUsize;
/// use core::ops::ControlFlow;
///
/// use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
/// use happenstance_sync::write_group::GroupPageBuilder;
/// use happenstance_sync::{ReplicatedEvent, Waiting};
///
/// let row = |position: u64| -> Result<(SequencePosition, ReplicatedEvent), Box<dyn core::error::Error>> {
///     let position = SequencePosition::new(position).ok_or("position zero")?;
///     let id = EventId::new(StoreId::from_bytes([1; 16]), position);
///     let event = Event::new("Written", &b""[..])?;
///     Ok((position, ReplicatedEvent::new(id, RecordedAt::from_millis(0), event)))
/// };
///
/// // Two groups, of two events and one, under a budget of two.
/// let mut page = GroupPageBuilder::new(NonZeroUsize::MIN.saturating_add(1));
/// let (a, a1) = row(1)?;
/// let (_, a2) = row(2)?;
/// let (b, b1) = row(3)?;
/// assert!(page.offer(a, None, a1).is_continue());
/// assert!(page.offer(a, None, a2).is_continue());
/// assert!(page.offer(b, None, b1).is_break(), "the second group would overflow");
///
/// let page = page.finish(Waiting::Nothing)?;
/// assert_eq!(page.groups.len(), 1);
/// assert_eq!(page.waiting, Waiting::Unknown);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug)]
pub struct GroupPageBuilder {
    max_events: usize,
    taken: usize,
    groups: Vec<WriteGroup>,
    open: Option<WriteGroup>,
    full: bool,
    misordered: Option<GroupOrderError>,
}

impl GroupPageBuilder {
    /// An empty page that holds at most `max_events` events, past which only
    /// a first group may go.
    #[must_use]
    pub const fn new(max_events: NonZeroUsize) -> Self {
        Self {
            max_events: max_events.get(),
            taken: 0,
            groups: Vec::new(),
            open: None,
            full: false,
            misordered: None,
        }
    }

    /// Offers the next row: `event`, written in the group named `first`.
    ///
    /// `guard` is read only from the row that opens a group, which is the row
    /// at `first`; pass `None` for the rest. Returns
    /// [`ControlFlow::Break`] once the page is full, and the caller stops
    /// reading: the row that broke it is not on the page.
    ///
    /// Also breaks, for good, on a row whose `first` is at or below the
    /// `first` of a group already offered and left: the builder is then
    /// refused, takes nothing more, and [`finish`](Self::finish) says why.
    pub fn offer(
        &mut self,
        first: SequencePosition,
        guard: Option<AppendCondition>,
        event: ReplicatedEvent,
    ) -> ControlFlow<()> {
        if self.full || self.misordered.is_some() {
            return ControlFlow::Break(());
        }
        match &mut self.open {
            Some(open) if open.first() == first => open.push(event),
            _ => {
                // The group being left is the open one, or, once the page has
                // closed it, the last on the page: a new group must name a
                // position above it.
                let passed = self
                    .open
                    .as_ref()
                    .or(self.groups.last())
                    .map(WriteGroup::first);
                if let Some(after) = passed.filter(|&after| first <= after) {
                    self.misordered = Some(GroupOrderError { first, after });
                    return ControlFlow::Break(());
                }
                if self.close().is_break() {
                    return ControlFlow::Break(());
                }
                self.open = Some(WriteGroup::new(first, guard, event));
            }
        }
        // A group already past what is left can never be taken unless it is
        // the first; stop reading it rather than scan to its end.
        match &self.open {
            Some(open) if !self.groups.is_empty() && !self.fits(open) => {
                self.full = true;
                ControlFlow::Break(())
            }
            _ => ControlFlow::Continue(()),
        }
    }

    /// The page, given what lay past the last row offered: `past_rows` is
    /// [`Waiting::Nothing`] when the read reached the end of the log, and
    /// [`Waiting::Unknown`] when it stopped short of it.
    ///
    /// A page that stopped before a group always says
    /// [`Waiting::Unknown`].
    ///
    /// # Errors
    ///
    /// [`GroupOrderError`] if a row was offered out of group order, in which
    /// case no page is returned: what the builder held is not known to be
    /// whole groups.
    pub fn finish(mut self, past_rows: Waiting) -> Result<GroupPage, GroupOrderError> {
        if let Some(misordered) = self.misordered {
            return Err(misordered);
        }
        let waiting = if self.close().is_break() {
            Waiting::Unknown
        } else {
            past_rows
        };
        Ok(GroupPage::new(self.groups, waiting))
    }

    /// Whether `group` fits in what the budget has left.
    fn fits(&self, group: &WriteGroup) -> bool {
        self.taken.saturating_add(group.events().len()) <= self.max_events
    }

    /// Takes the open group onto the page if it fits, or if it is the first;
    /// otherwise marks the page full.
    fn close(&mut self) -> ControlFlow<()> {
        if self.full {
            return ControlFlow::Break(());
        }
        let Some(open) = self.open.take() else {
            return ControlFlow::Continue(());
        };
        if self.groups.is_empty() || self.fits(&open) {
            self.taken = self.taken.saturating_add(open.events().len());
            self.groups.push(open);
            ControlFlow::Continue(())
        } else {
            self.full = true;
            ControlFlow::Break(())
        }
    }
}

/// A [`GroupPageBuilder`] was offered a row out of group order: one naming the
/// group `first`, after the group `after` had been offered and left, where
/// `first` is not above `after`.
///
/// It is the store's read that is wrong, not the log: a store that does not
/// serialise its writers has to read in group order, not position order. The
/// same read fails the same way again, so it is
/// [`Stop`](FailureClass::Stop).
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error(
    "write group {first} was offered after write group {after}; groups must be offered in \
     ascending order of their first position"
)]
#[non_exhaustive]
pub struct GroupOrderError {
    /// The group the refused row named.
    pub first: SequencePosition,
    /// The group already offered and left, at or above `first`.
    pub after: SequencePosition,
}

impl Classify for GroupOrderError {
    fn class(&self) -> FailureClass {
        FailureClass::Stop
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};

    use super::{GroupOrderError, GroupPageBuilder};
    use crate::identity::ReplicatedEvent;
    use crate::peer::Waiting;
    use crate::write_group::{GroupPage, WriteGroup};

    fn position(raw: u64) -> SequencePosition {
        SequencePosition::new(raw).expect("test positions are non-zero")
    }

    fn row(raw: u64) -> ReplicatedEvent {
        ReplicatedEvent::new(
            EventId::new(StoreId::from_bytes([5; 16]), position(raw)),
            RecordedAt::from_millis(0),
            Event::new("Written", &b""[..]).expect("a valid event type"),
        )
    }

    /// Offers `rows` as `(group, position)` pairs, in group order, until the
    /// builder breaks.
    fn fill(max: usize, rows: &[(u64, u64)], past: Waiting) -> GroupPage {
        let mut builder = GroupPageBuilder::new(NonZeroUsize::new(max).expect("a non-zero budget"));
        for &(group, at) in rows {
            if builder.offer(position(group), None, row(at)).is_break() {
                break;
            }
        }
        builder
            .finish(past)
            .expect("every arrangement here is offered in group order")
    }

    fn firsts(page: &GroupPage) -> Vec<u64> {
        page.groups
            .iter()
            .map(|group: &WriteGroup| group.first().get())
            .collect()
    }

    #[test]
    fn every_group_that_fits_is_taken_and_the_end_says_nothing() {
        let page = fill(4, &[(1, 1), (1, 2), (3, 3), (4, 4)], Waiting::Nothing);

        assert_eq!(firsts(&page), [1, 3, 4]);
        assert_eq!(page.waiting, Waiting::Nothing);
    }

    #[test]
    fn a_first_group_over_the_budget_is_taken_whole_and_alone() {
        let page = fill(1, &[(1, 1), (1, 2), (1, 3), (4, 4)], Waiting::Nothing);

        assert_eq!(firsts(&page), [1]);
        assert_eq!(page.groups.first().map(|g| g.events().len()), Some(3));
        assert_eq!(page.waiting, Waiting::Unknown);
    }

    #[test]
    fn a_group_that_would_overflow_is_left_whole_for_the_next_page() {
        let page = fill(3, &[(1, 1), (1, 2), (3, 3), (3, 4)], Waiting::Nothing);

        assert_eq!(firsts(&page), [1]);
        assert_eq!(page.waiting, Waiting::Unknown);
    }

    #[test]
    fn a_last_group_that_overflows_at_the_end_of_the_rows_still_says_unknown() {
        let mut builder = GroupPageBuilder::new(NonZeroUsize::new(2).expect("a non-zero budget"));
        assert!(builder.offer(position(1), None, row(1)).is_continue());
        assert!(builder.offer(position(2), None, row(2)).is_continue());
        assert!(builder.offer(position(2), None, row(3)).is_break());

        let page = builder
            .finish(Waiting::Nothing)
            .expect("offered in group order");

        assert_eq!(firsts(&page), [1]);
        assert_eq!(page.waiting, Waiting::Unknown);
    }

    #[test]
    fn a_read_cut_short_by_the_caller_passes_its_waiting_through() {
        let page = fill(8, &[(1, 1), (2, 2)], Waiting::Unknown);

        assert_eq!(firsts(&page), [1, 2]);
        assert_eq!(page.waiting, Waiting::Unknown);
    }

    #[test]
    fn interleaved_positions_offered_in_group_order_keep_both_groups_whole() {
        // G = {5, 7} and H = {6, 8}, as two non-serialised writers leave them.
        let page = fill(4, &[(5, 5), (5, 7), (6, 6), (6, 8)], Waiting::Nothing);

        assert_eq!(firsts(&page), [5, 6]);
        let lens: Vec<usize> = page.groups.iter().map(|g| g.events().len()).collect();
        assert_eq!(lens, [2, 2]);
    }

    /// The same two groups offered by position, as a store that does not
    /// serialise its writers would meet them in a scan by position: G5 opens,
    /// H6 closes it, and G's second row then names a group H already passed.
    /// Without the refusal the page reads `[G5 = {5}, H6 = {6}, G5 = {7}]`,
    /// tearing G in two and naming two groups 5.
    #[test]
    fn a_group_offered_after_a_later_one_closed_is_refused() {
        let mut builder = GroupPageBuilder::new(NonZeroUsize::new(8).expect("a non-zero budget"));
        assert!(builder.offer(position(5), None, row(5)).is_continue());
        assert!(builder.offer(position(6), None, row(6)).is_continue());
        assert!(
            builder.offer(position(5), None, row(7)).is_break(),
            "an out-of-order offer stops the read"
        );
        assert!(
            builder.offer(position(8), None, row(8)).is_break(),
            "a refused builder takes nothing more"
        );

        let refused = builder.finish(Waiting::Nothing);

        assert_eq!(
            refused,
            Err(GroupOrderError {
                first: position(5),
                after: position(6),
            })
        );
    }
}
