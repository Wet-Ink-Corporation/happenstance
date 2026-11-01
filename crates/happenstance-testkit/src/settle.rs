//! The rule-side half of CF-42: a rule that reads back its own append settles
//! the frontier first, or reports why it cannot.
//!
//! Nothing here waits. The branch is on
//! [`Fixture::READ_YOUR_OWN_WRITES`], a `const`, so a fixture that supports it
//! is never called and does no I/O, and any waiting belongs to the fixture's
//! own override, outside this crate (CF-33).

use crate::{Fixture, RuleOutcome, Settled};

/// `Ok` when `fixture` reads its own writes or has settled, and the skip to
/// report when it cannot.
///
/// The skip names the capability the fixture already declines, with its own
/// stated reason: the same pair a `require!` gate on that capability would
/// produce, so no new constant is needed to report it.
///
/// # Errors
///
/// [`RuleOutcome::Skipped`] under `READ_YOUR_OWN_WRITES` when the fixture
/// declines that capability and its
/// [`settle_frontier`](Fixture::settle_frontier) answers [`Settled::Nothing`].
pub(crate) async fn settled<F: Fixture>(fixture: &F) -> Result<(), RuleOutcome> {
    let Some(reason) = F::READ_YOUR_OWN_WRITES.reason() else {
        return Ok(());
    };
    match fixture.settle_frontier().await {
        Settled::Visible => Ok(()),
        Settled::Nothing => Err(RuleOutcome::Skipped {
            capability: "READ_YOUR_OWN_WRITES",
            reason,
        }),
    }
}

/// Settles `$fixture`'s frontier, or returns the skip from the enclosing rule.
///
/// A macro rather than a function for `require!`'s reason (ARC-15): the early
/// return from the caller is the whole point of it.
macro_rules! settle {
    ($fixture:expr) => {
        if let Err(skip) = $crate::settle::settled(&$fixture).await {
            return skip;
        }
    };
}
pub(crate) use settle;
