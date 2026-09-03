//! **The second half of the evidence: each defect is real.**
//!
//! `tests/census.rs` reports how many of the four the suite rejects. That number
//! is worthless if the four are strawmen — a store whose "defect" cannot be
//! observed through the port at all would pass the suite for the same reason a
//! correct store does, and counting it would flatter nothing and prove nothing.
//!
//! So this file drives each defect **directly**, through the port, with no rule
//! in the path, and asserts the wrong observable behaviour a consumer would meet.
//! Every assertion here is red against a correct store and green against the
//! mutant, which is the opposite polarity from the census and is why it is a
//! separate binary.
//!
//! It is also where the ES-22 arm witness lives: F2-5's claim is not that the
//! rule is missing but that its `landed == 0` branch has never executed, and the
//! only way to show which branch a rule took, without editing the rule, is to
//! reproduce its drop-after-one-poll sequence outside it and read the counts.

mod support;

use core::future::{Future, poll_fn};
use core::pin::pin;
use core::task::Poll;

use happenstance_core::{Event, EventStore, Query, ReadOptions, SequencePosition, Tags, collect};
use happenstance_testkit::{Fixture, block_on};
use support::wrong_fixtures::{
    CorrectFixture, ForwardPagingBudgetFixture, NoopReopenFixture, StagedCommitFixture,
    SwallowedReadFaultFixture,
};
use support::harness::Subject;

/// A tagged event, so that nothing here is accidentally sensitive to the
/// injected `InnerJoin` arm (which drops *untagged* events).
fn ev(kind: &str) -> Event {
    Event::new(kind, happenstance_core::bytes::Bytes::from_static(b"x"))
        .expect("a valid event type")
        .with_tags(Tags::from_pairs([("k", "v")]).expect("a valid tag"))
}

/// Polls `future` exactly once, then drops it.
///
/// That *is* cancellation in Rust: there is no cancel token, the caller simply
/// stops polling and the state machine is destroyed at whatever suspension point
/// it had reached. Returns `Some` if the future completed on that one poll and
/// `None` if it suspended — the same two answers `suite.rs`'s own `poll_once`
/// gives, and both are permitted by ES-22.
fn poll_once_then_drop<F: Future>(future: F) -> Option<F::Output> {
    block_on(async move {
        let mut future = pin!(future);
        poll_fn(move |cx| {
            Poll::Ready(match future.as_mut().poll(cx) {
                Poll::Ready(output) => Some(output),
                Poll::Pending => None,
            })
        })
        .await
    })
}

/// The position `n` events into a dense store.
fn at(n: u64) -> SequencePosition {
    SequencePosition::new(n).expect("a non-zero position")
}

// =====================================================================
// 1 — L1-1: the forward paging budget that is never spent
// =====================================================================

/// A forward `from` + `limit` read is answered with the whole tail.
///
/// The read a projection runner issues on every tick: resume at the checkpoint,
/// take a page of two. The correct store hands back two events; this one hands
/// back four, and says nothing about it.
#[test]
fn forward_paging_budget_is_ignored() {
    block_on(async {
        let seed = [ev("A"), ev("B"), ev("C"), ev("D"), ev("E")];
        let options = ReadOptions::new().from(at(2)).limit(2);

        let correct = CorrectFixture::open();
        let correct = correct.connect().await;
        correct.append(&seed, None).await.expect("the seed appends");
        let correct_page = collect(correct.read(&Query::all(), options))
            .await
            .expect("the read succeeds");

        let mutant = ForwardPagingBudgetFixture::<false>::open();
        let mutant = mutant.connect().await;
        mutant.append(&seed, None).await.expect("the seed appends");
        let mutant_page = collect(mutant.read(&Query::all(), options))
            .await
            .expect("the read succeeds");

        assert_eq!(
            correct_page.len(),
            2,
            "the control: a page of two from position 2 is two events"
        );
        assert_eq!(
            mutant_page.len(),
            4,
            "the defect: `limit` never reaches the resume branch, so the caller \
             that asked for two is handed the whole tail — and is told nothing"
        );

        // And the backwards branch really is untouched, which is what keeps this
        // a scalpel rather than a store that broke paging outright.
        let backwards = ReadOptions::new().from(at(4)).backwards().limit(2);
        let back = collect(mutant.read(&Query::all(), backwards))
            .await
            .expect("the read succeeds");
        assert_eq!(
            back.len(),
            2,
            "the backwards branch is left correct on purpose: the suite DOES \
             compose backwards `from` with `limit`, and a store that broke both \
             directions would be caught by a rule that is not this finding's"
        );
    });
}

// =====================================================================
// 2 — L1-2: REOPEN declared supported, honoured by an empty body
// =====================================================================

/// `reopen()` returns without closing anything, and the pre-existing handle
/// still works.
///
/// `Fixture::reopen`'s contract is that a fresh `connect` afterwards observes
/// exactly what was durably committed and nothing else, and that handles taken
/// before the call "may stop working". Here there was never a medium: the events
/// the three durability rules read back after the reopen are the same `Vec`, in
/// the same process, reached through the same `Rc` — as this asserts by using the
/// *old* handle, which a store that had genuinely closed could not answer from.
#[test]
fn a_noop_reopen_closes_nothing() {
    block_on(async {
        let fixture = NoopReopenFixture::<false>::open();
        let before = fixture.connect().await;
        before
            .append(&[ev("Acknowledged")], None)
            .await
            .expect("the append is acknowledged");

        fixture.reopen().await;

        let through_the_old_handle = collect(before.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the read succeeds");
        assert_eq!(
            through_the_old_handle.len(),
            1,
            "the defect: the handle taken before the reopen still answers, from \
             the same in-process `Vec`. Nothing was closed, nothing was replayed, \
             and no durable medium was consulted — yet the three rules that are \
             the whole of the suite's durability certification read this back and \
             pass"
        );
    });
}

// =====================================================================
// 3 — L3-01: a fetch failure reported as end-of-stream
// =====================================================================

/// An armed read fault produces a short, *successful* read.
///
/// `collect` is correct — it returns the first `Err` the stream yields — so the
/// `Ok` here is the store's answer and not the helper's. Five events are in the
/// store; the caller is handed two and told the stream ended.
#[test]
fn a_swallowed_read_fault_looks_like_the_end_of_the_stream() {
    block_on(async {
        let fixture = SwallowedReadFaultFixture::<false>::open();
        let store = fixture.connect().await;
        store
            .append(&[ev("A"), ev("B"), ev("C"), ev("D"), ev("E")], None)
            .await
            .expect("the seed appends");

        let whole = collect(store.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the read succeeds");
        assert_eq!(
            whole.len(),
            5,
            "the control: with no fault armed this store is correct, which is \
             what makes it a mutant rather than a saboteur"
        );

        // No conformance rule can execute this line: `Fixture` has
        // `arm_mid_batch_fault` for the write path and no read-path analogue.
        fixture.arm_read_fault(1);

        let truncated = collect(store.read(&Query::all(), ReadOptions::new())).await;
        assert_eq!(
            truncated.map(|events| events.len()),
            Ok(2),
            "the defect: the second page fetch failed and the caller was told the \
             stream ended. The read reports SUCCESS over two of five events — a \
             projection checkpoints past three events it never saw, and the \
             port's own per-item `Err` arm, which exists exactly for this, is \
             never used"
        );
    });
}

// =====================================================================
// 4 — F2-5: the ES-22 arm witness
// =====================================================================

/// Which branch of `dropped_append_future_leaves_no_partial_batch` each store
/// takes.
///
/// The rule asserts `landed == 0 || landed == batch.len()` and then does one of
/// two very different things. F2-5's sharpened claim is that the `landed == 0`
/// arm — the byte-identical snapshot comparison — has never executed against any
/// store in the workspace, because every store that mounts the suite completes
/// `append` on its first poll. This reproduces the rule's own sequence outside
/// the rule and reads the counts, which is the only way to see the branch without
/// editing `suite.rs`.
#[test]
fn the_es22_arm_a_store_takes_depends_on_whether_it_suspends() {
    let batch = [ev("Ex"), ev("Why"), ev("Zed")];

    // The correct core: no `.await` in `append`, so the first poll runs it to
    // completion and the rule takes its "fully applied" arm.
    let correct = block_on(async { CorrectFixture::open().connect().await });
    let finished_immediately = poll_once_then_drop(correct.append(&batch, None));
    let landed_in_correct = block_on(async {
        collect(correct.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the read succeeds")
            .len()
    });

    // The staging store: a suspension between two statements, and a commit that
    // never ran.
    let staged = block_on(async { StagedCommitFixture::<false>::open().connect().await });
    let suspended = poll_once_then_drop(staged.append(&batch, None));
    let landed_in_staged = block_on(async {
        collect(staged.read(&Query::all(), ReadOptions::new()))
            .await
            .expect("the read succeeds")
            .len()
    });

    assert!(
        finished_immediately.is_some(),
        "the correct core finishes on the first poll, which is why it cannot \
         answer ES-22's question"
    );
    assert_eq!(
        landed_in_correct, 3,
        "so the rule takes its `landed == batch.len()` arm — the membership \
         loop — and the byte-identical snapshot comparison is never reached"
    );

    assert!(
        suspended.is_none(),
        "the staging store suspends inside `append`, which is the shape ES-22 \
         was written for and which no store in the workspace has"
    );
    assert_eq!(
        landed_in_staged, 0,
        "so the rule takes its `landed == 0` arm — `snapshot_of(&after) == \
         snapshot_of(&before)` — for the first time. That branch is not broken; \
         it had simply never run"
    );
}
