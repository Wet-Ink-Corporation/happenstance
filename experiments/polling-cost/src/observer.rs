//! Staleness, observed rather than subtracted.
//!
//! `EventStore::head`'s own documentation forbids `head() - checkpoint`:
//! positions are an opaque ordering key, the specification permits gaps, and the
//! difference between two of them counts nothing. Against a dense store the
//! subtraction happens to be right, which is exactly what lets it survive
//! review; against a store that leaves holes it reports a backlog that does not
//! exist.
//!
//! So this observer keeps the positions it was told about and **compares**. Feed
//! it a gapped sequence and a dense one carrying the same delivery pattern and
//! it reports the same figures; a subtraction reports a larger number for the
//! gapped one and is wrong about it.

use happenstance::{Checkpoint, SequencePosition};

/// One view's staleness ledger.
#[derive(Debug, Default)]
pub struct Observer {
    appended: Vec<Appended>,
    staleness_ns: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
struct Appended {
    position: SequencePosition,
    at_ns: u64,
    observed: bool,
}

impl Observer {
    /// An observer that has seen nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Records that an event landed at `position` at `at_ns`.
    pub fn appended(&mut self, position: SequencePosition, at_ns: u64) {
        self.appended.push(Appended {
            position,
            at_ns,
            observed: false,
        });
    }

    /// Records what a checkpoint reading at `now_ns` reveals.
    ///
    /// Every appended event at or below the checkpoint's `through` that has not
    /// been seen yet is closed out, with its staleness taken as the difference
    /// between the two **instants** — a duration, which is a quantity — rather
    /// than between two positions, which is not.
    pub fn observe(&mut self, checkpoint: Checkpoint, now_ns: u64) {
        let Some(through) = considered_through(checkpoint) else {
            return;
        };
        for entry in &mut self.appended {
            if !entry.observed && entry.position <= through {
                entry.observed = true;
                self.staleness_ns.push(now_ns.saturating_sub(entry.at_ns));
            }
        }
    }

    /// How many appended events this view has not yet observed.
    ///
    /// A **comparison** over positions, never a difference: the answer is the
    /// same whether the store assigns 1, 2, 3 or 7, 14, 21.
    #[must_use]
    pub fn pending(&self, checkpoint: Checkpoint) -> usize {
        let through = considered_through(checkpoint);
        self.appended
            .iter()
            .filter(|entry| match through {
                None => true,
                Some(through) => entry.position > through,
            })
            .count()
    }

    /// Every staleness sample recorded so far, in nanoseconds.
    #[must_use]
    pub fn staleness_ns(&self) -> &[u64] {
        &self.staleness_ns
    }
}

/// Where a checkpoint says a view has got to, if anywhere.
fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
        _ => None,
    }
}
