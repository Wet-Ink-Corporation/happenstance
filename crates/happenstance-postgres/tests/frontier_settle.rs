//! CF-42 against a live server: a read-back rule passes while another
//! transaction holds the visibility frontier back, because the fixture settles.
//!
//! #272 was this shape on a loaded CI runner. A transaction open anywhere on the
//! server keeps `pg_snapshot_xmin` below the rule's own appends, so a rule that
//! read straight after appending read `[]` and failed with *"`Query::all()` must
//! match every event"*. Here the holder is opened on purpose, in another
//! fixture's schema: the frontier is cluster-wide, so a schema is as unrelated
//! as a database and needs no second one. `PostgresFixture::settle_frontier`
//! waits for it, and the rule passes once it is released.
//!
//! `#[ignore]`d like its siblings: it needs a live server, and reaches one with
//! `-- --ignored`. The holder's release is the one timed step in it, justified
//! as `runtime_seam.rs`'s `HANG_LIMIT` is: an ignored live-server instrument,
//! not a conformance rule, so CF-33 does not reach it.

#![cfg(not(target_arch = "wasm32"))]

mod support;

use std::time::Duration;

use happenstance_postgres::sqlx;
use happenstance_testkit::RuleOutcome;
use support::PostgresFixture;

/// How long the holder keeps its transaction open after the rule starts.
///
/// Long enough that the rule's appends commit while it is held, which they do
/// in milliseconds on loopback, and well inside the settle's thirty-second
/// deadline.
const HOLD: Duration = Duration::from_secs(2);

/// The opener the rule is handed.
async fn fixture() -> PostgresFixture {
    PostgresFixture::new()
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_frontier_held_back_by_another_database_still_passes_after_settling() {
    let holder_fixture = PostgresFixture::new();
    let pool = holder_fixture.pool_for_test().await;
    let mut holder = pool
        .begin()
        .await
        .expect("a broken test environment: the holder could not begin");
    // Taking an xid is what holds the frontier. A transaction that has only
    // read has none, and `pg_snapshot_xmin` passes it by.
    let xid: String = sqlx::query_scalar("SELECT pg_current_xact_id()::text")
        .fetch_one(&mut *holder)
        .await
        .expect("a broken test environment: the holder could not take an xid");

    // Released from a task the test joins, so the hold ends while the rule's
    // settle is waiting for it rather than after the rule has given up.
    let release = tokio::spawn(async move {
        tokio::time::sleep(HOLD).await;
        holder.rollback().await
    });

    let outcome = happenstance_testkit::rules::query_all_matches_every_event(fixture).await;
    release
        .await
        .expect("the holder's task panicked")
        .expect("a broken test environment: the holder could not roll back");

    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "the rule must run and pass behind a frontier held by transaction {xid}, \
         because the fixture settles before the rule reads; got {outcome:?}"
    );
}
