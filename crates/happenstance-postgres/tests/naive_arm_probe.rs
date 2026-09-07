//! Does the naive arm actually break, and does CF-13's schedule reach it?
//!
//! `tests/rule_controls.rs` found that
//! `nothing_below_an_observed_position_appears_later` **passes** against the
//! naive arm. Two things could explain that and they have opposite consequences:
//!
//! 1. **The naive arm is not actually broken** — removing the frontier predicate
//!    changes nothing observable, and the mechanism this adapter ships is
//!    unnecessary.
//! 2. **The rule's schedule cannot reach the defect** — the arm is broken and the
//!    rule cannot create the window in which it shows.
//!
//! The first would be a finding about the adapter and the second a finding about
//! the rule, and guessing between them is exactly the mistake this file exists to
//! avoid. So this constructs the inversion **by hand**, with the commit ordering
//! under the test's control rather than the runtime's, and asks the arm directly.
//!
//! # Why the ordering cannot be left to `poll_once`
//!
//! CF-13 interleaves two appends by polling each future once per step: the slow
//! writer starts, the fast writer starts, the fast writer commits, then the slow
//! one. That works for an adapter whose `append` advances only while it is being
//! polled.
//!
//! This adapter's does not. `append` hands its work to the runtime
//! (`PostgresEventStore::on_runtime`), because `sqlx` needs a runtime in
//! thread-local scope and the suite's concurrency contenders have none. So one
//! poll returns `Pending` having *committed to* the whole append, and the runtime
//! finishes it whenever it finishes — the schedule is the runtime's, not the
//! rule's.
//!
//! Below, the two transactions are driven through raw `sqlx` on the pool, so
//! "take a position", "commit", and "read" happen in exactly the order written.

#![cfg(all(feature = "naive-arm", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_core::{EventStore, Query, ReadOptions, SendEventStore};
use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::sqlx;
use support::PostgresFixture;

/// Every position a read yields right now, through the store's own read path.
async fn visible(store: &PostgresEventStore) -> Vec<u64> {
    let query = Query::all();
    let stream = SendEventStore::read(store, &query, ReadOptions::new());
    happenstance_core::collect(stream)
        .await
        .expect("a read against a live store should succeed")
        .into_iter()
        .map(|event| event.position.get())
        .collect()
}

/// The inversion, built by hand, against whichever arm is passed.
///
/// Returns the positions observed after the *fast* writer commits, and again
/// after the *slow* one does. A store that satisfies ES-10 never shows a
/// position in the second list that is lower than one already in the first.
async fn observe_inversion(
    fixture: &PostgresFixture,
    store_is_naive: bool,
) -> (Vec<u64>, Vec<u64>) {
    let store = if store_is_naive {
        PostgresEventStore::new_naive(fixture.pool_for_test().await)
    } else {
        PostgresEventStore::new(fixture.pool_for_test().await)
    };
    let pool = store.pool().clone();

    // The slow writer opens first and takes the lower position. Its transaction
    // stays open across everything below, which is the whole point: a position
    // allocated and not yet published.
    let mut slow = pool.begin().await.expect("slow writer should begin");
    let slow_position: i64 = sqlx::query_scalar("SELECT nextval('event_position_seq')")
        .fetch_one(&mut *slow)
        .await
        .expect("slow writer should take a position");

    // The fast writer opens second, takes the higher position, and commits
    // first.
    let mut fast = pool.begin().await.expect("fast writer should begin");
    let fast_position: i64 = sqlx::query_scalar("SELECT nextval('event_position_seq')")
        .fetch_one(&mut *fast)
        .await
        .expect("fast writer should take a position");
    assert!(
        fast_position > slow_position,
        "the sequence must hand out increasing values, or this probe proves nothing"
    );

    insert_at(&mut fast, fast_position, "FastWriter").await;
    fast.commit().await.expect("fast writer should commit");

    let after_fast = visible(&store).await;

    // Only now does the lower position land.
    insert_at(&mut slow, slow_position, "SlowWriter").await;
    slow.commit().await.expect("slow writer should commit");

    let after_slow = visible(&store).await;
    (after_fast, after_slow)
}

/// One row at an explicitly chosen position.
async fn insert_at(
    transaction: &mut sqlx::Transaction<'static, sqlx::Postgres>,
    position: i64,
    event_type: &str,
) {
    sqlx::query(
        "INSERT INTO event \
         (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) \
         VALUES ($1, $2, '\\x00'::bytea, NULL, ARRAY['writer:probe']::text[], \
                 uuid_send(gen_random_uuid()), $1, 0)",
    )
    .bind(position)
    .bind(event_type)
    .execute(&mut **transaction)
    .await
    .expect("the probe's insert should succeed");
}

/// The naive arm must show the inversion. If it does not, the mechanism this
/// adapter ships is unnecessary and that is the finding.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_naive_arm_admits_a_position_beneath_one_already_observed() {
    let fixture = PostgresFixture::new();
    let (after_fast, after_slow) = observe_inversion(&fixture, true).await;

    println!("naive   | after fast commit: {after_fast:?}");
    println!("naive   | after slow commit: {after_slow:?}");

    let highest_seen = after_fast.iter().copied().max();
    let appeared_below = highest_seen.is_some_and(|high| {
        after_slow
            .iter()
            .any(|&position| position < high && !after_fast.contains(&position))
    });

    assert!(
        appeared_below,
        "the naive arm did NOT invert: {after_fast:?} then {after_slow:?}. \
         If this holds, removing the frontier predicate changes nothing a reader \
         can see, and the mechanism this adapter ships is not buying what it claims."
    );
}

/// The shipped arm must not.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_shipped_arm_never_admits_a_position_beneath_one_already_observed() {
    let fixture = PostgresFixture::new();
    let (after_fast, after_slow) = observe_inversion(&fixture, false).await;

    println!("shipped | after fast commit: {after_fast:?}");
    println!("shipped | after slow commit: {after_slow:?}");

    let highest_seen = after_fast.iter().copied().max();
    let appeared_below = highest_seen.is_some_and(|high| {
        after_slow
            .iter()
            .any(|&position| position < high && !after_fast.contains(&position))
    });

    assert!(
        !appeared_below,
        "the SHIPPED arm inverted: {after_fast:?} then {after_slow:?}. \
         The frontier predicate is not doing what this adapter claims for it."
    );
}

/// Proof that `EventStore` is still what generic code binds. Unrelated to the
/// probe; here because this target is the only one that names both flavours.
#[allow(dead_code)]
fn generic_code_binds_the_bare_flavour<S: EventStore>() {}
