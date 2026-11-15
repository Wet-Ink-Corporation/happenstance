//! `WithholdingStore`, CF-27's completeness instrument, held to its own
//! invariants.
//!
//! Every withheld event must behave as removed at every seam the port has, and
//! every retained one must behave as though nothing happened. No assertion here
//! names a literal position: each one compares against positions the inner
//! store assigned, or indexes into that list.
//!
//! `memory` gates `MemoryFixture`, and `not(wasm32)` is the `#[tokio::test]`
//! harness.

#![cfg(all(not(target_arch = "wasm32"), feature = "memory"))]

use std::collections::BTreeSet;

use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventStore, MemoryEventStore,
    MemoryStoreError, Query, ReadOptions, SequencePosition, SequencedEvent, collect,
};
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle, query_tagged, tagged_event};
use happenstance_testkit::{
    FaultyStore, FaultyStoreError, Fixture, WithholdError, WithholdingStore,
};

type Store = WithholdingStore<MemoryHandle>;

/// A fresh instrument over a fresh memory store, and a second handle onto the
/// inner store so a test can compare against what the inner store holds.
async fn instrument() -> (Store, MemoryHandle) {
    let fixture = MemoryFixture::new();
    let inner = fixture.connect().await;
    (WithholdingStore::new(fixture.connect().await), inner)
}

/// Appends one event per entry, each tagged `claim:<entry>`, one batch each.
async fn seed(store: &Store, claims: &[&str]) -> Vec<SequencedEvent> {
    for claim in claims {
        store
            .append(&[tagged_event("Claimed", &[("claim", claim)])], None)
            .await
            .expect("an unconditional append to a memory store lands");
    }
    read_all(store).await
}

async fn read_with<S: EventStore>(store: &S, options: ReadOptions) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    collect(store.read(&Query::all(), options))
        .await
        .expect("this store's reads do not fail")
}

async fn read_all<S: EventStore>(store: &S) -> Vec<SequencedEvent>
where
    S::Error: core::fmt::Debug,
{
    read_with(store, ReadOptions::new()).await
}

fn positions(events: &[SequencedEvent]) -> Vec<SequencePosition> {
    events.iter().map(|event| event.position).collect()
}

fn at(events: &[SequencedEvent], index: usize) -> SequencePosition {
    events
        .get(index)
        .expect("the test appended at least this many")
        .position
}

fn claimed(claim: &str) -> Event {
    tagged_event("Claimed", &[("claim", claim)])
}

fn over_claim(claim: &str) -> AppendCondition {
    AppendCondition::new(query_tagged(&[("claim", claim)]))
}

// ---------------------------------------------------------------------------
// Nothing withheld: the inner store, unchanged (invariant 9)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_fresh_instrument_withholds_nothing_and_delegates() {
    let (store, _inner) = instrument().await;

    assert!(store.withheld().is_empty());
    assert!(read_all(&store).await.is_empty());
    assert_eq!(store.head().await.expect("memory head"), None);
}

#[tokio::test]
async fn withholding_nothing_is_the_inner_store() {
    let (store, inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c", "d"]).await;

    let options = [
        ReadOptions::new(),
        ReadOptions::new().limit(2),
        ReadOptions::new().limit(0),
        ReadOptions::new().backwards(),
        ReadOptions::new().backwards().limit(1),
        ReadOptions::new().from(at(&assigned, 1)),
        ReadOptions::new().to(at(&assigned, 2)),
        ReadOptions::new()
            .backwards()
            .from(at(&assigned, 2))
            .to(at(&assigned, 1)),
    ];
    for option in options {
        assert_eq!(
            read_with(&store, option).await,
            read_with(&inner, option).await,
            "with nothing withheld, {option:?} must read exactly what the inner store reads"
        );
    }
    assert_eq!(
        store.head().await.expect("head"),
        inner.head().await.expect("head")
    );
}

// ---------------------------------------------------------------------------
// Withholding by position (invariants 1, 2, 8 and 10)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_withheld_position_is_not_read() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c", "d"]).await;
    let hidden = at(&assigned, 1);

    store
        .withhold(&[hidden])
        .await
        .expect("the position is held");

    let expected: Vec<SequencePosition> = positions(&assigned)
        .into_iter()
        .filter(|p| *p != hidden)
        .collect();
    assert_eq!(positions(&read_all(&store).await), expected);
    assert_eq!(store.withheld(), BTreeSet::from([hidden]));
}

#[tokio::test]
async fn withholding_is_cumulative_and_the_report_is_the_union() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c", "d", "e"]).await;
    let (first, second) = (at(&assigned, 1), at(&assigned, 3));

    store.withhold(&[first]).await.expect("held");
    store.withhold(&[second]).await.expect("held");

    assert_eq!(store.withheld(), BTreeSet::from([first, second]));
    let read = positions(&read_all(&store).await);
    assert!(
        !read.contains(&first) && !read.contains(&second),
        "{read:?}"
    );
    assert_eq!(read.len(), assigned.len() - 2);
}

#[tokio::test]
async fn withholding_an_unheld_position_is_refused_and_withholds_nothing() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b"]).await;
    let head = store
        .head()
        .await
        .expect("head")
        .expect("two were appended");
    let beyond = head.next().expect("the position space is not exhausted");

    let refused = store.withhold(&[at(&assigned, 0), beyond]).await;

    assert!(
        matches!(refused, Err(WithholdError::NotHeld { position, .. }) if position == beyond),
        "{refused:?}"
    );
    assert!(
        store.withheld().is_empty(),
        "all or nothing: {:?}",
        store.withheld()
    );
    assert_eq!(read_all(&store).await, assigned);
}

#[tokio::test]
async fn rewithholding_a_withheld_position_is_a_no_op() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c"]).await;
    let hidden = at(&assigned, 1);

    store.withhold(&[hidden]).await.expect("held");
    store
        .withhold(&[hidden, hidden])
        .await
        .expect("still held by the inner store");
    store
        .withhold(&[])
        .await
        .expect("an empty request withholds nothing");

    assert_eq!(store.withheld(), BTreeSet::from([hidden]));
    assert_eq!(read_all(&store).await.len(), assigned.len() - 1);
}

// ---------------------------------------------------------------------------
// Read options compose with withholding (invariant 3)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn limit_counts_only_retained_events() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c", "d"]).await;
    store.withhold(&[at(&assigned, 0)]).await.expect("held");

    let limited = read_with(&store, ReadOptions::new().limit(2)).await;
    assert_eq!(positions(&limited), positions(&assigned[1..3]));

    assert!(
        read_with(&store, ReadOptions::new().limit(0))
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn backwards_from_and_to_compose_with_withholding() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c", "d", "e"]).await;
    store
        .withhold(&[at(&assigned, 1), at(&assigned, 3)])
        .await
        .expect("held");

    let backwards = read_with(&store, ReadOptions::new().backwards()).await;
    assert_eq!(
        positions(&backwards),
        vec![at(&assigned, 4), at(&assigned, 2), at(&assigned, 0)]
    );

    let window = ReadOptions::new()
        .from(at(&assigned, 1))
        .to(at(&assigned, 3));
    assert_eq!(
        positions(&read_with(&store, window).await),
        vec![at(&assigned, 2)]
    );

    let back_window = ReadOptions::new()
        .backwards()
        .from(at(&assigned, 4))
        .to(at(&assigned, 1))
        .limit(1);
    assert_eq!(
        positions(&read_with(&store, back_window).await),
        vec![at(&assigned, 4)]
    );
}

// ---------------------------------------------------------------------------
// head (invariant 4)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn head_skips_a_withheld_top() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c"]).await;
    store.withhold(&[at(&assigned, 2)]).await.expect("held");

    assert_eq!(store.head().await.expect("head"), Some(at(&assigned, 1)));
}

#[tokio::test]
async fn head_is_none_when_everything_is_withheld() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c"]).await;
    store.withhold(&positions(&assigned)).await.expect("held");

    assert_eq!(store.head().await.expect("head"), None);
    assert!(read_all(&store).await.is_empty());
}

// ---------------------------------------------------------------------------
// Membership (invariant 5, ES-41)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn membership_is_false_for_a_withheld_event_and_true_for_a_retained_one() {
    let (store, inner) = instrument().await;
    let assigned = seed(&store, &["a", "b"]).await;
    let (hidden, kept) = (&assigned[0], &assigned[1]);
    store.withhold(&[hidden.position]).await.expect("held");

    assert!(
        !store
            .contains_event_id(hidden.id)
            .await
            .expect("membership")
    );
    assert!(store.contains_event_id(kept.id).await.expect("membership"));
    assert!(
        inner
            .contains_event_id(hidden.id)
            .await
            .expect("membership"),
        "the inner store still holds it; only the instrument hides it"
    );
}

// ---------------------------------------------------------------------------
// Positions (invariant 7, ES-38 by delegation)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn positions_after_a_withheld_top_are_never_reused() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "c"]).await;
    store
        .withhold(&[at(&assigned, 1), at(&assigned, 2)])
        .await
        .expect("held");

    let landed = store.append(&[claimed("d")], None).await.expect("lands");

    let highest_withheld = store.withheld().last().copied().expect("two withheld");
    assert!(
        landed > highest_withheld,
        "{landed} reused or undercut {highest_withheld}"
    );
}

// ---------------------------------------------------------------------------
// Conditions see only retained events (invariant 6)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn a_condition_matching_only_withheld_history_passes_vacuously() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["k", "other"]).await;
    store.withhold(&[at(&assigned, 0)]).await.expect("held");

    let landed = store
        .append(&[claimed("k")], Some(&over_claim("k")))
        .await
        .expect("ES-40: only withheld history matches, so nothing retained violates");

    assert!(landed > at(&assigned, 1));
}

#[tokio::test]
async fn a_retained_match_below_the_highest_withheld_still_rejects() {
    let (store, _inner) = instrument().await;
    // The retained match sits below the withheld event, so only the pre-check
    // over the frozen region can see it.
    let assigned = seed(&store, &["k", "withheld"]).await;
    store.withhold(&[at(&assigned, 1)]).await.expect("held");
    let head_before = store.head().await.expect("head");

    let refused = store.append(&[claimed("k")], Some(&over_claim("k"))).await;

    assert_eq!(
        refused,
        Err(AppendError::ConditionViolated(ConditionViolated::at(at(
            &assigned, 0
        ))))
    );
    assert_eq!(
        store.head().await.expect("head"),
        head_before,
        "nothing was written"
    );
}

#[tokio::test]
async fn a_retained_match_above_the_highest_withheld_still_rejects() {
    let (store, _inner) = instrument().await;
    // The retained match sits above the withheld event: the inner store's own
    // evaluation of the raised condition is what must see it.
    let assigned = seed(&store, &["withheld", "k"]).await;
    store.withhold(&[at(&assigned, 0)]).await.expect("held");

    let refused = store.append(&[claimed("k")], Some(&over_claim("k"))).await;

    assert_eq!(
        refused,
        Err(AppendError::ConditionViolated(ConditionViolated::at(at(
            &assigned, 1
        ))))
    );
}

#[tokio::test]
async fn guards_keep_independent_boundaries_over_a_holed_log() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["quiet", "busy", "withheld", "busy"]).await;
    store
        .withhold(&[at(&assigned, 0), at(&assigned, 2)])
        .await
        .expect("held");

    // The first guard matches only withheld history; the second matches a
    // retained event beyond its own boundary, so the append is refused there.
    let rejected = AppendCondition::new(query_tagged(&[("claim", "quiet")]))
        .and_guard(query_tagged(&[("claim", "busy")]), Some(at(&assigned, 1)));
    let refused = store.append(&[claimed("busy")], Some(&rejected)).await;
    assert_eq!(
        refused,
        Err(AppendError::ConditionViolated(ConditionViolated::at(at(
            &assigned, 3
        ))))
    );

    // Moving only the second guard's boundary past that match admits it.
    let admitted = AppendCondition::new(query_tagged(&[("claim", "quiet")]))
        .and_guard(query_tagged(&[("claim", "busy")]), Some(at(&assigned, 3)));
    store
        .append(&[claimed("busy")], Some(&admitted))
        .await
        .expect("nothing retained violates either guard");
}

#[tokio::test]
async fn an_empty_batch_is_refused_as_no_events_before_any_precheck() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["k", "withheld"]).await;
    store.withhold(&[at(&assigned, 1)]).await.expect("held");

    let refused = store.append(&[], Some(&over_claim("k"))).await;

    assert_eq!(refused, Err(AppendError::NoEvents));
}

// ---------------------------------------------------------------------------
// Withholding by predicate, and shared handles
// ---------------------------------------------------------------------------

#[tokio::test]
async fn withhold_where_selects_a_scattered_set_and_returns_it_ascending() {
    let (store, _inner) = instrument().await;
    let assigned = seed(&store, &["a", "b", "a", "b", "a", "b"]).await;
    let is_a = |event: &SequencedEvent| event.tags().values_of("claim").any(|v| v == "a");
    let expected: Vec<SequencePosition> = assigned
        .iter()
        .filter(|event| is_a(event))
        .map(|event| event.position)
        .collect();

    let withheld = store.withhold_where(is_a).await.expect("memory reads");

    assert_eq!(withheld, expected);
    assert!(withheld.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(store.withheld(), withheld.iter().copied().collect());
    let survivors = positions(&read_all(&store).await);
    assert!(
        survivors.iter().any(|s| withheld.iter().any(|w| w < s))
            && survivors.iter().any(|s| withheld.iter().any(|w| w > s)),
        "survivors sit between the holes: {survivors:?} against {withheld:?}"
    );

    let again = store.withhold_where(|_| true).await.expect("memory reads");
    assert_eq!(
        again, survivors,
        "a second call returns only what it newly withheld"
    );
}

#[tokio::test]
async fn clones_share_one_withheld_set() {
    let (store, _inner) = instrument().await;
    let twin = store.clone();
    let assigned = seed(&store, &["a", "b"]).await;

    store.withhold(&[at(&assigned, 0)]).await.expect("held");

    assert_eq!(twin.withheld(), store.withheld());
    assert_eq!(positions(&read_all(&twin).await), vec![at(&assigned, 1)]);
}

// ---------------------------------------------------------------------------
// Failures of the inner store
// ---------------------------------------------------------------------------

#[tokio::test]
async fn inner_read_errors_pass_through() {
    let store = WithholdingStore::new(FaultyStore::new(MemoryEventStore::new()).fail_next_read(1));

    let read = collect(store.read(&Query::all(), ReadOptions::new())).await;

    assert_eq!(read, Err(FaultyStoreError::Injected));
}

#[tokio::test]
async fn a_failed_inner_read_fails_withhold() {
    let fixture = MemoryFixture::new();
    // A clone of a faulty wrapper shares its arming, so the test arms the
    // inner store after seeding it through the instrument.
    let faulty = FaultyStore::new(fixture.connect().await);
    let store = WithholdingStore::new(faulty.clone());
    let landed = store.append(&[claimed("a")], None).await.expect("lands");

    let _armed = faulty.fail_next_read(2);
    let refused = store.withhold(&[landed]).await;
    let refused_where = store.withhold_where(|_| true).await;

    assert_eq!(
        refused,
        Err(WithholdError::Store(FaultyStoreError::Injected))
    );
    assert_eq!(
        refused_where,
        Err(WithholdError::Store(FaultyStoreError::Injected))
    );
    assert!(
        store.withheld().is_empty(),
        "a failed withhold withholds nothing"
    );
}

#[tokio::test]
async fn a_withhold_error_renders_its_position_and_carries_its_source() {
    use core::error::Error as _;

    let (store, _inner) = instrument().await;
    let position = SequencePosition::FIRST;
    let not_held = store
        .withhold(&[position])
        .await
        .expect_err("an empty store holds nothing to withhold");
    assert!(
        not_held.to_string().contains(&position.to_string()),
        "{not_held}"
    );
    assert_eq!(
        not_held.source().map(ToString::to_string),
        None,
        "a refusal of this instrument's own has no underlying cause"
    );

    let failed: WithholdError<FaultyStoreError<MemoryStoreError>> =
        WithholdError::Store(FaultyStoreError::Injected);
    let source = failed.source().expect("the inner error is the source");
    assert_eq!(
        source.to_string(),
        FaultyStoreError::<MemoryStoreError>::Injected.to_string()
    );
    assert!(
        !failed.to_string().contains(&source.to_string()),
        "carried, not rendered: {failed}"
    );
}

// ---------------------------------------------------------------------------
// Auto traits
// ---------------------------------------------------------------------------

/// The concurrency family moves each contender's handle onto its own thread,
/// so the instrument must stay `Send + Sync` over a store that is.
const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WithholdingStore<MemoryHandle>>();
};
