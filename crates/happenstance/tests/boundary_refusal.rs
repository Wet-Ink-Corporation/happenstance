//! The opening encounter's step-3 scenario, as an executed test of this crate.
//!
//! # Why this exists, when the page already runs
//!
//! `docs/first-encounter.md` step 3 is compiled *and executed* as a doctest of
//! `xtask` through `xtask/src/narrative.rs`, so the boundary is already
//! load-bearing to the repository at one mount. This is the second, and the two
//! are not redundant:
//!
//! * **It is a mount the page's own failure mode cannot reach.** A doctest
//!   reports against a temporary bundle file under the OS temp directory, which
//!   is not a location anybody can open — `xtask/src/narrative.rs` measured that
//!   and records it as a standing limit. This one reports against a repo path
//!   and a real line.
//! * **It asserts the half a single scenario cannot.** The page shows the
//!   guarded append being refused. Only running the *same* append with the
//!   condition removed shows that the condition is what refused it: without that
//!   second assertion, a query that happens to match nothing, an `after` doing
//!   all the work, or a store that rejects everything would all look identical.
//!   That is the wrong implementation the project's risk table ranks first — a
//!   boundary that looks real in the source and is not load-bearing.
//!
//! The two copies are the same scenario written twice, deliberately:
//! `include_str!` cannot cross the package boundary of a published crate
//! (`crates/happenstance/src/lib.rs`), so the page cannot include this file and
//! this file cannot include the page. Drift between them can be silent about
//! *identity*; it cannot be silent about *correctness*, because both assert the
//! same `ConditionViolated`. Consolidating them through a shared
//! `examples/` file is a substrate question for HS-P0020.
//!
//! No mock, no stub, no `happenstance-testkit` fixture: the store is a real
//! in-process `MemoryEventStore`, constructed and dropped per test. A stubbed
//! store returning a canned `ConditionViolated` regardless of what was appended
//! would pass every assertion below while destroying the one thing they exist
//! for.

use happenstance::{AppendCondition, AppendError, Event, EventStore};
use happenstance::{MemoryEventStore, Query, QueryItem, Tags, read_decision_model};

/// The scenario, set up exactly as step 3 sets it up.
///
/// Returns the store, the seat event another writer already landed, and the
/// condition built from the query and the position the read observed — so the
/// two tests below differ in **one argument** and nothing else.
///
/// The `clone` clippy asks to replace with `std::slice::from_ref` is kept
/// deliberately: this body is the page's fence written a second time, and the
/// only defence against the two drifting apart is that they read the same. A
/// `std::slice::from_ref` here and a `&[seat.clone()]` on the page would be a
/// difference a reviewer has to think about, in the one file whose whole job is
/// to be the same program twice.
#[allow(
    clippy::cloned_ref_to_slice_refs,
    reason = "the twin is the page's fence, spelled identically on purpose"
)]
async fn raced() -> Result<(MemoryEventStore, Event, AppendCondition), Box<dyn core::error::Error>>
{
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let item = QueryItem::new(["SeatHeld"], held.clone())?;
    let seats = Query::from_items([item])?;

    let (_taken, upto) = read_decision_model(&store, &seats).await?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?;

    let condition = AppendCondition::new(seats).after_opt(upto);
    Ok((store, seat, condition))
}

/// The claim the page makes: with the guard in place, the append is refused.
#[tokio::test]
async fn the_guarded_append_is_refused() -> Result<(), Box<dyn core::error::Error>> {
    let (store, seat, condition) = raced().await?;

    match store.append(&[seat], Some(&condition)).await {
        Err(AppendError::ConditionViolated(_)) => Ok(()),
        ok => panic!("the boundary did not hold: {ok:?}"),
    }
}

/// The half that makes the claim falsifiable: remove the condition — the
/// drill's own edit — and the same append is accepted.
///
/// This is the assertion that fails if the scenario refuses for a reason other
/// than the boundary. A store that rejected everything, or a guard broad enough
/// that anything violates it, would pass the test above and fail this one.
#[tokio::test]
async fn without_the_condition_the_same_append_is_accepted()
-> Result<(), Box<dyn core::error::Error>> {
    let (store, seat, _condition) = raced().await?;

    let accepted = store.append(&[seat], None).await;
    assert!(
        accepted.is_ok(),
        "the unconditional append was refused, so the scenario's refusal is not \
         the condition's doing: {accepted:?}"
    );
    Ok(())
}
