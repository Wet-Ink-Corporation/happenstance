//! The contended schedule: k = 8 writers against one boundary, deterministic.
//!
//! ADR-0012 item 3 asks for a stated commit/reject mix, because a rejected
//! append clones nothing on the borrowed shape and a benchmark that never
//! rejects measures the uncontended path only. Threads would make the mix a
//! property of the scheduler; this makes it a property of the code.
//!
//! **The schedule.** Each round, every pending contender decides at the
//! current boundary — the position of the last commit, or none — and then they
//! attempt in order. The first lands and moves the boundary; every later one
//! in that round decided before it moved, so its condition refuses it, and it
//! decides again next round. With k contenders that is `k(k+1)/2` attempts, k
//! commits and `k(k-1)/2` refusals: **36, 8 and 28** for k = 8.
//!
//! What a contender does with its batch between attempts is the caller-side
//! axis, and it is the closure's business, not the schedule's.

use happenstance_core::SequencePosition;

/// How many writers contend.
pub const CONTENDERS: usize = 8;

/// One attempt's outcome, as the schedule needs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attempt<E> {
    /// The batch landed; its last event is at this position.
    Committed(SequencePosition),
    /// The condition refused it.
    Refused,
    /// Anything else, which ends the run.
    Failed(E),
}

/// What a run did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tally {
    /// Appends issued.
    pub attempts: usize,
    /// Appends that landed.
    pub commits: usize,
    /// Appends the condition refused.
    pub rejections: usize,
}

/// Runs the schedule. `attempt(contender, decided_at)` makes one append for
/// `contender`, conditioned on nothing past `decided_at`.
///
/// # Errors
///
/// The first [`Attempt::Failed`], unchanged.
pub fn run<E>(
    mut attempt: impl FnMut(usize, Option<SequencePosition>) -> Attempt<E>,
) -> Result<Tally, E> {
    let mut tally = Tally {
        attempts: 0,
        commits: 0,
        rejections: 0,
    };
    let mut boundary: Option<SequencePosition> = None;
    let mut pending: Vec<usize> = (0..CONTENDERS).collect();
    while !pending.is_empty() {
        let decided_at = boundary;
        let mut refused = Vec::with_capacity(pending.len());
        for contender in pending {
            tally.attempts += 1;
            match attempt(contender, decided_at) {
                Attempt::Committed(position) => {
                    tally.commits += 1;
                    boundary = Some(position);
                }
                Attempt::Refused => {
                    tally.rejections += 1;
                    refused.push(contender);
                }
                Attempt::Failed(err) => return Err(err),
            }
        }
        pending = refused;
    }
    Ok(tally)
}
