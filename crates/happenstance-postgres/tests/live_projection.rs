//! The projection conformance suite against the **far end** of PS-2's axis: a
//! Postgres projection store whose batch is a live transaction.
//!
//! Gated exactly as `projection.rs` is, and for the reasons that file carries in
//! full: `#[ignore]` composes with `--all-features`, keeps the target compiled
//! and linted by the default gate, and makes the gated tests appear as `ignored`
//! rather than vanishing from the count.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features --test live_projection -- --ignored --list
//! cargo test -p happenstance-postgres --all-features --test live_projection -- --ignored --show-output --test-threads=1
//! ```
//!
//! `--test-threads=1` on the run and not on the listing: each fixture instance
//! opens a pool sized for the concurrency family, the container's
//! `max_connections` is finite, and twenty fixtures opening pools at once is
//! how a rule fails on a pool timeout that says nothing about the adapter.
//! `--list` opens no connection and needs no such flag.
//!
//! # What this adapter reports rather than passes
//!
//! **One** rule is a reported skip — `refused_reset_changes_nothing`, because
//! `RESET_REFUSAL` is declined for the buffered store's reason. Everything else
//! runs, and that is the point of the file: `batch_reads_reflect_pending_writes`
//! and `rebuild_is_chunk_size_invariant`, reported skips on every adapter over a
//! real database until now, execute here for the first time against something
//! that is not an in-process map. The adapter-private test below asserts that
//! on the `RuleOutcome` value rather than on stdout.

#![cfg(all(
    feature = "projection-store",
    feature = "conformance",
    not(target_arch = "wasm32")
))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_core::{
    Authority, Checkpoint, ProjectionId, ProjectionProbe, ProjectionStore, SequencePosition,
};
use happenstance_postgres::sqlx;
use happenstance_testkit::{ProjectionFixture, RuleOutcome};
use support::LivePostgresProjectionFixture;

/// The projection family's emitter, plus `#[ignore]`. See `projection.rs`.
macro_rules! emit_ignored_projection_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
            async fn $name() {
                happenstance_testkit::projection::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_projection_tokio;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance,
    emit = crate::emit_ignored_projection_tokio,
    fixture = LivePostgresProjectionFixture::new()
);

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// PS-12's rule **ran** here, and did not report a skip.
///
/// This is the observation ADR-0060 found the suite unable to make: a
/// live-transaction adapter that can declare `READS_THROUGH_BATCH = true`
/// truthfully, and a read-through rule that executes against it rather than
/// being taken as a reported skip. Asserted on the [`RuleOutcome`] rather than
/// on stdout, because a skip line is a string and `Ran` is a value.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_read_through_rule_runs_rather_than_skipping() {
    const {
        assert!(
            <LivePostgresProjectionFixture as ProjectionFixture>::Store::READS_THROUGH_BATCH,
            "the live store declares the capability its transaction has"
        );
    }

    let outcome =
        happenstance_testkit::projection::rules::batch_reads_reflect_pending_writes(async || {
            LivePostgresProjectionFixture::new()
        })
        .await;
    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "PS-12's rule must execute against a live transaction, not report a skip: {outcome:?}"
    );

    let outcome =
        happenstance_testkit::projection::rules::rebuild_is_chunk_size_invariant(async || {
            LivePostgresProjectionFixture::new()
        })
        .await;
    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "the rebuild rule must execute against a live transaction, not report a skip: {outcome:?}"
    );
}

/// The write is on the server before commit, and it is the server that layers
/// it over committed state.
///
/// A buffering store can pass `batch_reads_reflect_pending_writes` by keeping a
/// shadow map — which its documentation rejects for good reason. This test is
/// the control that the live store does no such thing: the row is observable
/// through the batch's transaction, invisible through a **second** backend
/// session, and gone from both after a rollback.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_pending_write_lives_on_the_server_and_only_in_this_session() {
    let fixture = LivePostgresProjectionFixture::new();
    let writer = fixture.connect().await;
    let observer = fixture.connect().await;

    let mut batch = writer.begin().await.unwrap();
    writer.probe_write(&mut batch, "depot-7", 12).await.unwrap();

    assert_eq!(
        writer
            .probe_read_through(&mut batch, "depot-7")
            .await
            .unwrap(),
        Some(12),
        "the open transaction sees its own statement"
    );
    assert_eq!(
        observer.probe_read("depot-7").await.unwrap(),
        None,
        "a second session sees nothing until commit"
    );

    writer.rollback(batch).await.unwrap();
    assert_eq!(
        writer.probe_read("depot-7").await.unwrap(),
        None,
        "and a rollback undid a statement the server had already received"
    );
}

/// A statement refused mid-batch poisons the transaction, and the seam says so.
///
/// This is what the probe seam's `Result` was added for. The write is refused
/// by the server — a `CHECK` constraint the driver cannot pre-check — and the
/// batch is then unusable: Postgres answers every further statement on it with
/// *"current transaction is aborted"*, which `commit` surfaces as
/// `CommitError::Store` rather than reporting success over a write that never
/// happened.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_refused_statement_poisons_the_batch_and_commit_says_so() {
    let fixture = LivePostgresProjectionFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    let id = ProjectionId::new("poisoned");

    // A constraint the probe write will violate: `v` must be small.
    sqlx::query("ALTER TABLE projection_probe ADD CONSTRAINT v_is_small CHECK (v < 100)")
        .execute(&pool)
        .await
        .unwrap();

    let mut batch = store.begin().await.unwrap();
    store.probe_write(&mut batch, "ok", 1).await.unwrap();
    store
        .probe_write(&mut batch, "too-big", 1_000)
        .await
        .expect_err("the server refuses the statement, and the seam reports it");

    let refused = store
        .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect_err("a poisoned transaction cannot be committed");
    assert!(
        matches!(refused, happenstance_core::CommitError::Store(_)),
        "an aborted transaction is the store's failure, not a regression or a foreign batch: {refused:?}"
    );

    assert_eq!(
        store.probe_read("ok").await.unwrap(),
        None,
        "the statement that succeeded before the poison was rolled back with it"
    );
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::NeverRun,
        "and the checkpoint did not move"
    );

    // The store is still usable afterwards: the poisoned connection went back
    // to the pool rolled back, which is PS-7 for a batch that held a resource.
    let mut fresh = store.begin().await.unwrap();
    store.probe_write(&mut fresh, "after", 2).await.unwrap();
    store
        .commit(fresh, &id, SequencePosition::FIRST, Authority::Live)
        .await
        .unwrap();
    assert_eq!(store.probe_read("after").await.unwrap(), Some(2));
}
