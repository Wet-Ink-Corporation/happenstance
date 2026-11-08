//! The local write groups one step offers, and what it read past them.

use alloc::vec::Vec;
use core::num::NonZeroUsize;

use happenstance_core::SequencePosition;

use super::Pending;
use crate::peer::{EventGroup, PeerLimits, PullBatchLimit, Waiting};
use crate::write_group::GroupPage;

/// The groups a push carries, in local order, and the count of those it left.
pub(super) struct Run {
    /// One wire group per local write group, each whole (I-7).
    pub(super) groups: Vec<EventGroup>,
    /// How many events `groups` carries.
    pub(super) events: usize,
    /// How many events the page held past `groups`.
    pub(super) left: usize,
    /// The [`first`](crate::WriteGroup::first) position of the last group in
    /// `groups`: the watermark once the push is acknowledged.
    pub(super) last_first: Option<SequencePosition>,
    /// Whether anything may lie past the page: the store said so, or the page
    /// filled the read.
    bounded: bool,
}

impl Run {
    /// Takes the page's groups while the peer's limits admit them, stopping at
    /// the first that does not fit rather than skipping it: the watermark is a
    /// prefix, and a group left behind with a later one pushed could never be
    /// confirmed (I-6). A first group that does not fit at all is taken
    /// alone, so the peer refuses it and says why (SY-36).
    ///
    /// The page is never re-cut: a group goes whole or not at all.
    pub(super) fn gather(page: GroupPage, limits: &PeerLimits, read_limit: NonZeroUsize) -> Self {
        let read: usize = page.groups.iter().map(|group| group.events().len()).sum();
        // The store's `Waiting::Nothing` is believed only when the page did not
        // fill the read: a page cut by its own budget is reported as a lower
        // bound, as the per-event read before it was, so a reader never takes a
        // bound for an exact figure.
        let bounded = page.waiting != Waiting::Nothing || read >= read_limit.get();
        let mut run = Self {
            groups: Vec::with_capacity(page.groups.len()),
            events: 0,
            left: 0,
            last_first: None,
            bounded,
        };
        let mut budget = PullBatchLimit::from(limits);
        let mut open = true;
        for group in page.groups {
            let first = group.first();
            let wire = EventGroup::from(group);
            let fits = budget.admits(&wire);
            if open && (fits || run.groups.is_empty()) {
                budget.charge(&wire);
                run.events = run.events.saturating_add(wire.events.len());
                run.last_first = Some(first);
                run.groups.push(wire);
                open = fits;
            } else {
                open = false;
                run.left = run.left.saturating_add(wire.events.len());
            }
        }
        run
    }

    /// `count` as exact, or as a lower bound when the page may not have
    /// reached the end of the local log.
    pub(super) const fn pending(&self, count: usize) -> Pending {
        if self.bounded {
            Pending::AtLeast(count)
        } else {
            Pending::Exactly(count)
        }
    }
}
