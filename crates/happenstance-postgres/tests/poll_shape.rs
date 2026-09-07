//! How many polls this adapter's `append` needs, and what shape those polls are.
//!
//! # Why this measurement exists
//!
//! `spec/SPECIFICATION.md`, under ES-10: *"`nothing_below_an_observed_position_appears_later`
//! has a strength that varies with the adapter's poll shape. The rule polls two
//! `append` futures A, B, B, A and `Fixture` cannot express a poll budget, so
//! against a store whose `append` needs three polls the interleaving window never
//! opens where the rule looks and the rule cannot fail. The bounding instrument —
//! a poll-padding decorator over `PreCommitPositionStore` — is named and owed by
//! **phase 10**."*
//!
//! `.kb/open-questions/poll-count-bounds-the-visibility-rule.md` adds the part
//! that makes this a measurement rather than a guess. Its sub-question 3 asks
//! *"what value of `n` is the right first calibration target, and does it come
//! from `happenstance-postgres`'s actual `append` implementation once phase 10
//! builds it, or from a synthetic worst case chosen independently?"* — and
//! ADR-0013 refused to pick `n` by authorial choice, because *"an author choosing
//! n is the reference-store failure mode with one more step — so the calibration
//! waits for an adapter with real I/O."*
//!
//! This is that adapter. The number below is the one the decorator should be
//! calibrated against.
//!
//! # What the numbers mean
//!
//! Two are reported, because this adapter has a poll shape the open question does
//! not contemplate and reporting only the first would hide it.
//!
//! * **Polls to completion, tight loop.** The future is polled repeatedly with
//!   nothing in between. For an adapter that advances only when polled, this is
//!   *the* poll count and it is a constant.
//! * **Polls to completion, yielding.** The same, with a yield between polls so
//!   the runtime can make progress. For an adapter that advances only when
//!   polled, this is the same number.
//!
//! Where they differ, the adapter advances **off-poll** — its work is on a
//! runtime and proceeds whether or not anyone polls it — and "how many polls does
//! `append` need" stops being a property of the adapter and becomes a property of
//! how fast the caller polls. That distinction is the finding; see the ledger.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

mod support;

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

use happenstance_core::{Event, SendEventStore};
use happenstance_testkit::Fixture;
use support::PostgresFixture;

/// A poll that does nothing on wake, so polling is the only thing driving the
/// future — which is the property being measured.
fn noop_context() -> Context<'static> {
    Context::from_waker(Waker::noop())
}

/// Polls `future` until it is ready, counting, with nothing in between.
///
/// The bound is high because the answer is high. A first attempt capped it at
/// 10,000 and reported "did not complete", which is the correct observation and
/// the wrong number: ten thousand tight polls take microseconds and a database
/// round trip takes milliseconds, so the cap was measuring the cap.
fn polls_in_a_tight_loop<T>(future: impl Future<Output = T>) -> (usize, Option<T>) {
    let mut future = pin!(future);
    let mut context = noop_context();
    for count in 1..=TIGHT_LOOP_BOUND {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return (count, Some(value));
        }
    }
    (TIGHT_LOOP_BOUND, None)
}

/// Generous enough that the answer is the adapter's rather than this constant's.
const TIGHT_LOOP_BOUND: usize = 50_000_000;

/// Polls `future` until ready, sleeping a millisecond between polls.
///
/// A millisecond is the order of a loopback round trip, so this counts round
/// trips rather than CPU cycles — which is the number the open question is
/// actually asking for when it says "how many polls does this adapter's `append`
/// need".
async fn polls_at_one_millisecond<T>(future: impl Future<Output = T>) -> (usize, Option<T>) {
    let mut future = pin!(future);
    let mut context = noop_context();
    for count in 1..=10_000 {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return (count, Some(value));
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
    (10_000, None)
}

fn one_event() -> [Event; 1] {
    [Event::new("PollShape", &b"x"[..]).expect("a valid event type")]
}

/// The calibration ADR-0013 deferred until an adapter with real I/O existed.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn append_poll_count_is_measured_not_chosen() {
    let fixture = PostgresFixture::new();
    let store = fixture.connect().await;

    // Warm the store's cached identity first, so the number measures `append`
    // rather than the one-off `store_meta` read that precedes the first one.
    SendEventStore::append(&store, &one_event(), None)
        .await
        .expect("the warm-up append should succeed");

    let batch = one_event();
    let (tight, tight_outcome) =
        polls_in_a_tight_loop(SendEventStore::append(&store, &batch, None));

    let batch = one_event();
    let (paced, paced_outcome) =
        polls_at_one_millisecond(SendEventStore::append(&store, &batch, None)).await;

    println!(
        "POLL SHAPE | tight loop:        {tight} polls, completed={}",
        tight_outcome.is_some()
    );
    println!(
        "POLL SHAPE | 1ms between polls: {paced} polls, completed={}",
        paced_outcome.is_some()
    );

    assert!(
        tight_outcome.is_some() && paced_outcome.is_some(),
        "`append` did not complete under one of the two regimes, so no poll count          can be reported for it at all"
    );

    // The finding, asserted so it cannot rot into a comment.
    //
    // For an adapter that advances only when polled — every in-memory store, and
    // `rusqlite` under a mutex — these two numbers are THE SAME, because the poll
    // count is a property of the state machine. Here they differ by orders of
    // magnitude, because the work is on a runtime and the polls are only asking
    // whether it has finished yet. "How many polls does `append` need" is
    // therefore not a fact about this adapter at all; it is a fact about how fast
    // the caller polls.
    //
    // That is precisely what the poll-padding decorator cannot model. It pads a
    // state machine that advances on poll; it cannot make one that advances
    // off-poll.
    assert!(
        tight > paced * 10,
        "the two regimes agreed closely ({tight} vs {paced}), which would mean          `append` advances only when polled. If that ever becomes true, the          decorator's calibration applies to this adapter directly and the ledger's          finding needs revisiting."
    );
}
