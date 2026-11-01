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
//! # It cannot pass by the holder holding nothing
//!
//! The holder is released only after the settle probe, asked of the rule's own
//! schema, has answered `false`: the rule's rows were there, and the frontier
//! had not passed them. A holder that held nothing, or a rule that appended
//! nothing, never produces that answer, and the test fails saying so rather
//! than passing on a quiet server.
//!
//! `#[ignore]`d like its siblings: it needs a live server, and reaches one with
//! `-- --ignored`. The watch for that answer is the one timed step in it,
//! justified as `runtime_seam.rs`'s `HANG_LIMIT` is: an ignored live-server
//! instrument, not a conformance rule, so CF-33 does not reach it.

#![cfg(not(target_arch = "wasm32"))]

mod support;

use std::time::Duration;

use happenstance_postgres::sqlx::{self, PgPool};
use happenstance_testkit::RuleOutcome;
use support::{PostgresFixture, frontier_passes_every_row};
use tokio::sync::watch;

/// How long the watcher waits to see the rule's rows held above the frontier.
///
/// Inside the settle's thirty-second deadline, so a watcher that gives up
/// releases the holder while the rule can still finish, and the test reports
/// the watcher's failure rather than a settle timeout.
const WATCH_LIMIT: Duration = Duration::from_secs(20);

/// The pause between two probes while the watcher waits.
const WATCH_PAUSE: Duration = Duration::from_millis(10);

/// Waits until the probe over `pool`'s schema answers `false`: rows are held
/// there that the frontier has not passed.
async fn held_above_the_frontier(pool: &PgPool) {
    while frontier_passes_every_row(pool).await {
        tokio::time::sleep(WATCH_PAUSE).await;
    }
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_frontier_held_back_by_another_schema_still_passes_after_settling() {
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

    // The rule's fixture hands its migrated pool to the watcher before the rule
    // appends anything, so the watcher probes the schema the rule writes to.
    let (rule_pool_tx, mut rule_pool_rx) = watch::channel(None::<PgPool>);
    let open = async || {
        let fixture = PostgresFixture::new();
        rule_pool_tx.send_replace(Some(fixture.pool_for_test().await));
        fixture
    };

    // Released from a task the test joins, and only once the hold is shown to
    // bind the rule's rows, so the release lands while the settle is waiting.
    let release = tokio::spawn(async move {
        let watched = tokio::time::timeout(WATCH_LIMIT, async {
            // Cloned out of the `watch::Ref` so no borrow of the channel is held
            // across the probes' awaits; a `PgPool` is a cheap shared handle.
            let rule_pool = rule_pool_rx
                .wait_for(Option::is_some)
                .await
                .expect("the opener keeps the sender alive until it has sent")
                .clone()
                .expect("waited for `Some`");
            held_above_the_frontier(&rule_pool).await;
        })
        .await;
        let rolled_back = holder.rollback().await;
        (watched.is_ok(), rolled_back)
    });

    let outcome = happenstance_testkit::rules::query_all_matches_every_event(open).await;
    let (held, rolled_back) = release.await.expect("the holder's task panicked");
    rolled_back.expect("a broken test environment: the holder could not roll back");

    assert!(
        held,
        "transaction {xid} never held the rule's rows above the frontier within \
         {WATCH_LIMIT:?}, so the rule passing proves nothing about the settle"
    );
    assert!(
        matches!(outcome, RuleOutcome::Ran),
        "the rule must run and pass behind a frontier held by transaction {xid}, \
         because the fixture settles before the rule reads; got {outcome:?}"
    );
}
