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
/// One seat is landed *before* the read, so `upto` is `Some(SequencePosition(1))`
/// rather than `None` and the guard carries a boundary this program actually
/// observed. That is not decoration: with an empty store the read yields `None`,
/// `AppendCondition::new` already carries `after: None`, and `.after_opt(upto)`
/// is inert — the scenario would refuse for the whole-log reason rather than for
/// the reason the page claims. See `boundary-refusal-encounter/_conditions.md`
/// § BC-004.
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
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?; // already held

    let (_taken, upto) = read_decision_model(&store, &seats).await?;
    store.append(&[seat.clone()], None).await?; // another writer

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

/// What "the `after` is load-bearing" can mean, and what it cannot, executed.
///
/// The page's criterion asks for a second falsification alongside the tag join:
/// remove `after_opt(upto)` and the scenario stops refusing. That one cannot be
/// had, and this is where it is *checked* rather than asserted in prose.
/// `AppendCondition::new` carries `after: None`, and `None` checks the **whole
/// log** — strictly stronger than any position — so deleting the call tightens
/// the guard and the same append is refused anyway. The first half below is
/// that fact as a test.
///
/// What is falsifiable, and what a reader must actually get right, is the
/// position's *value*: build the guard from a read taken after the race and the
/// store admits the append. That acceptance is the lost update the whole
/// encounter exists to make visible, and the second half below is it.
///
/// Recorded and routed as **BC-004** in
/// `.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/_conditions.md`.
#[allow(
    clippy::cloned_ref_to_slice_refs,
    reason = "the same scenario as `raced`, spelled the same way on purpose"
)]
#[tokio::test]
async fn the_after_is_load_bearing_in_its_value_not_in_its_presence()
-> Result<(), Box<dyn core::error::Error>> {
    let store = MemoryEventStore::new();
    let held = Tags::from_pairs([("course", "c1")])?;
    let item = QueryItem::new(["SeatHeld"], held.clone())?;
    let seats = Query::from_items([item])?;
    let seat = Event::new("SeatHeld", &b"{}"[..])?.with_tags(held);
    store.append(&[seat.clone()], None).await?; // already held

    let (_taken, _upto) = read_decision_model(&store, &seats).await?;
    store.append(&[seat.clone()], None).await?; // another writer

    // Presence: the guard a reader is left with if they delete `after_opt`.
    let whole_log = AppendCondition::new(seats.clone());
    let refused = store.append(&[seat.clone()], Some(&whole_log)).await;
    assert!(
        matches!(refused, Err(AppendError::ConditionViolated(_))),
        "the whole-log guard admitted the append, so `after: None` is not the \
         stronger condition this test and BC-004 both rest on: {refused:?}"
    );

    // Value: a guard built from a read taken *after* the race, which is the
    // mistake the page's `upto` exists to prevent.
    let (_again, fresh) = read_decision_model(&store, &seats).await?;
    let stale = AppendCondition::new(seats).after_opt(fresh);
    let accepted = store.append(&[seat], Some(&stale)).await;
    assert!(
        accepted.is_ok(),
        "a guard built from a read taken after the race refused, so this test \
         no longer demonstrates the lost update it exists for: {accepted:?}"
    );
    Ok(())
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
