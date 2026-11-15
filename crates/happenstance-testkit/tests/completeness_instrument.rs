//! CF-27: the completeness instrument, and the rule that holds it to account.
//!
//! `WithholdingStore` holds only part of its own log. CF-27 asks two things of
//! it, and `instrument_report_is_accurate_and_the_suite_cannot_tell` asserts
//! both:
//!
//! 1. **The suite cannot tell.** Every rule of every family passes against a
//!    store whose whole prior history is withheld, and the outcome of each is
//!    recorded in `completeness_instrument/pass_list.txt`. That list is the
//!    evidence that indistinguishability at the port is the specified outcome
//!    (ADR-0028's refusal).
//! 2. **The report is accurate.** What the instrument says it withholds is
//!    exactly what `read` no longer yields, what membership denies, and what it
//!    was asked to withhold, over a suffix, a scattered set, nothing, and
//!    everything.
//!
//! Three wrong instruments are rejected by the second half's own code path:
//! a report of a range, a report of the last call only, and a decorator that
//! forgot to answer membership itself. The suite cannot reach the third: no rule
//! asks `contains_event_id` of an event it did not itself append. That is why
//! the report half exists.
//!
//! The same fixture is also mounted through each family's macro, so a failing
//! rule names itself as a test of its own.
//!
//! Gated off `wasm32` because it uses threads and `catch_unwind`, and on
//! `memory` because the fixture wraps `MemoryFixture`'s store.

#![cfg(all(not(target_arch = "wasm32"), feature = "memory"))]

// An inline module, so that its children resolve under
// `tests/completeness_instrument/` without `#[path]`, and cargo does not
// compile them as test targets of their own.
mod completeness_instrument {
    pub(crate) mod accuracy;
    pub(crate) mod fixture;
    pub(crate) mod record;
    pub(crate) mod wrong;
}

use completeness_instrument::accuracy::{Correct, Inaccuracy, Rejection, Scenario};
use completeness_instrument::{accuracy, record, wrong};

// Named by the conformance mounts below, which `use super::*`.
use completeness_instrument::fixture::PrunedMemoryFixture;

/// The committed record.
const PASS_LIST: &str = include_str!("completeness_instrument/pass_list.txt");

/// CF-27's rule.
#[test]
fn instrument_report_is_accurate_and_the_suite_cannot_tell() {
    // The suite cannot tell.
    let entries = record::run_families();
    let failed: Vec<String> = entries
        .iter()
        .filter_map(|entry| match &entry.recorded {
            record::Recorded::Failed { message } => {
                Some(format!("{} {}: {message}", entry.family, entry.name))
            }
            _ => None,
        })
        .collect();
    assert!(
        failed.is_empty(),
        "CF-27: these rules told a pruned store from a young one:\n{}",
        failed.join("\n")
    );

    let rendered = record::render(&entries);
    assert!(
        rendered == record::expected(PASS_LIST),
        "CF-27: the record differs from completeness_instrument/pass_list.txt. \
         Review the change, then replace the file with:\n\n{}{}\n",
        record::header(PASS_LIST),
        rendered.join("\n")
    );

    // The report is accurate.
    assert_eq!(accuracy::report_is_accurate::<Correct>(), Ok(()));
}

/// The rejection a wrong instrument must produce, from the scenario that should
/// catch it.
fn rejected_in<D: accuracy::Defect>(scenario: Scenario) -> Rejection {
    let rejection = accuracy::report_is_accurate::<D>()
        .expect_err("a wrong instrument must not pass the report half");
    assert_eq!(rejection.scenario, scenario, "{rejection:?}");
    rejection
}

fn position(rejection: &Rejection, index: usize) -> happenstance_core::SequencePosition {
    *rejection
        .assigned
        .get(index)
        .expect("the scenario seeded this many")
}

#[test]
fn range_report_is_rejected() {
    // Exact on the suffix; on the scattered set the survivor `[2]` falls
    // between the holes `[1]` and `[3]`, inside the range, and is still read.
    let rejection = rejected_in::<wrong::RangeReport>(Scenario::Scattered);
    assert_eq!(
        rejection.inaccuracy,
        Inaccuracy::ReportedWithheldButReadable(position(&rejection, 2))
    );
}

#[test]
fn last_call_report_is_rejected() {
    // Exact while there is one call; the scattered set's second call drops
    // `[1]` and `[3]` from the report while they stay hidden.
    let rejection = rejected_in::<wrong::LastCallReport>(Scenario::Scattered);
    assert_eq!(
        rejection.inaccuracy,
        Inaccuracy::DroppedButReportedRetained(position(&rejection, 1))
    );
}

#[test]
fn leaky_membership_is_rejected() {
    // The inner store still holds the first withheld event, and says so.
    let rejection = rejected_in::<wrong::LeakyMembership>(Scenario::Suffix);
    assert_eq!(
        rejection.inaccuracy,
        Inaccuracy::MembershipDisagreesWithReport {
            position: position(&rejection, 0),
            answered: true,
        }
    );
}

happenstance_testkit::event_store_conformance!(PrunedMemoryFixture::pruned().await);

happenstance_testkit::event_store_concurrency_conformance!(PrunedMemoryFixture::pruned().await);

#[cfg(feature = "proptest")]
happenstance_testkit::event_store_model_conformance!(PrunedMemoryFixture::pruned().await);
