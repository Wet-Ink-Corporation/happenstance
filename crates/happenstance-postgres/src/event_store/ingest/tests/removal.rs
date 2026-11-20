//! SY-40 on Postgres (#483, #484, ADR-0110): ingest over a store whose rows
//! another session deleted with a raw `DELETE`.
//!
//! The first five tests mirror PR 1's oracle behaviours in
//! `happenstance-sync/src/memory_ingest/removal.rs`. The rest are this
//! adapter's own: the trigger writing the record whatever the deleting
//! session's `search_path`, a skip-only call drawing no sequence value, the
//! record reaching every handle and surviving reopen (I-4), a version-2 schema
//! upgraded in place, and the `SHARE` lock excluding a removal and an ingest
//! from each other in both directions (I-6), each shown without a sleep by a
//! `lock_timeout` that fails the waiter with `55P03`. I-5, the restore, is
//! `restore_family.rs`'s, whose fixture rolls the record back (D-7).
//!
//! The literal positions are **origin** positions, which a peer supplies;
//! every local position is one the store assigned.

use std::str::FromStr;
use std::time::Duration;

use happenstance_core::{EventId, SequencePosition};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, Ingested};
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use super::super::FIRST_ROUND_TRIP_SQL;
use super::{append, carrying, event, holds, id, ingest, log, own, peer, rows, server, shipped};
use crate::error::PostgresEventStoreError;
use crate::event_store::PostgresEventStore;
use crate::migration;

/// A store on a fresh, migrated schema, and that schema's name.
async fn named_store() -> (PostgresEventStore, String) {
    let schema = server::fresh_schema_name();
    let pool = server::pool_onto(&schema).await;
    migration::apply(&pool)
        .await
        .expect("the event log's migrations apply to a fresh schema");
    (PostgresEventStore::new(pool), schema)
}

/// The SQL position of a store-assigned position.
fn bigint(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).expect("a Postgres position fits a bigint")
}

/// Deletes the events at `positions` in one statement on `pool`, as an
/// operator's retention script would, and checks each was held. The shape the
/// event-store fixture's `remove_outside_port` uses and the ingest fixture's
/// calls.
pub(super) async fn remove(pool: &PgPool, positions: &[SequencePosition]) {
    let positions: Vec<i64> = positions.iter().copied().map(bigint).collect();
    let deleted = sqlx::query("DELETE FROM event WHERE position = ANY($1)")
        .bind(&positions)
        .execute(pool)
        .await
        .expect("the removal commits")
        .rows_affected();
    assert_eq!(
        usize::try_from(deleted).ok(),
        Some(positions.len()),
        "every position named was held: {positions:?}"
    );
}

/// The local position of the one event carrying `id`.
async fn position_of(store: &PostgresEventStore, id: EventId) -> SequencePosition {
    carrying(&log(store).await, id).position
}

/// `(appended, skipped, compensated)`, the three counts every test compares.
fn counts(ingested: Ingested) -> (usize, usize, usize) {
    (ingested.appended, ingested.skipped, ingested.compensated)
}

/// The record, read raw from `pool`'s schema, in key order.
async fn record(pool: &PgPool) -> Vec<(Vec<u8>, i64)> {
    sqlx::query_as(
        "SELECT origin_store, origin_position FROM event_removed \
         ORDER BY origin_store, origin_position",
    )
    .fetch_all(pool)
    .await
    .expect("the record reads")
}

/// The position sequence's state: its value and whether it was handed out.
async fn sequence(pool: &PgPool) -> (i64, bool) {
    sqlx::query_as("SELECT last_value, is_called FROM event_position_seq")
        .fetch_one(pool)
        .await
        .expect("the sequence reads")
}

/// A second pool onto `schema` whose every connection gives up on a lock
/// after 50 milliseconds, with `55P03`, so a test can show a wait without
/// sleeping through one.
async fn impatient_pool(schema: &str) -> PgPool {
    let options = PgConnectOptions::from_str(server::server_url().await)
        .expect("the server's URL parses")
        .options([("search_path", schema), ("lock_timeout", "50ms")]);
    PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("a second pool onto the schema opens")
}

/// Whether `error` is the server giving up on a lock (`55P03`).
fn lock_not_available(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(sqlx::error::DatabaseError::code)
        .as_deref()
        == Some("55P03")
}

/// #483. A removed foreign event offered again is skipped and counted as
/// skipped: no row comes back, the log is the pruned log, and
/// `contains_event_id` keeps answering `false` (ES-38).
///
/// **Red before the record:** `(1, 2, 0)`, the removed event re-admitted at
/// the tail.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_removed_foreign_event_is_skipped_on_redelivery() {
    let (store, _) = named_store().await;
    let from = peer(0x61);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
        shipped(id(from, 3), 3, event("Shipped", "c")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("three foreign events land");
    remove(store.pool(), &[position_of(&store, batch[1].id).await]).await;
    let pruned = log(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery over a removal is not an error");

    assert_eq!(counts(again), (0, 3, 0), "the removed event came back");
    assert_eq!(log(&store).await, pruned, "the log is the pruned log");
    assert!(
        !holds(&store, batch[1].id).await,
        "the removed identity reads as held again"
    );
}

/// #484. A removed **own** event offered again is a skip, not
/// `UnheldOwnIdentity`, and the new foreign event beside it in the same call
/// lands.
///
/// **Red before the record:** `UnheldOwnIdentity` naming the removed event,
/// class `Stop`, and nothing landed.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_removed_own_event_is_skipped_not_refused() {
    let (store, _) = named_store().await;
    let first = append(&store, &[event("Local", "a")]).await;
    append(&store, &[event("Local", "b")]).await;
    remove(store.pool(), &[first]).await;
    let echo = EventId::new(own(&store).await, first);
    let arrived = id(peer(0x62), 1);
    let batch = [
        shipped(echo, 0, event("Local", "a")),
        shipped(arrived, 1, event("Shipped", "y")),
    ];

    let landed = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a removed own event is a skip, not SY-38's refusal");

    assert_eq!(counts(landed), (1, 1, 0));
    assert!(holds(&store, arrived).await);
    assert!(
        !holds(&store, echo).await,
        "the removed own event came back"
    );
}

/// I-3: the record is a set of identities, not "anything unheld is fine". An
/// own claim the store **never** held, past its allocator, is still refused
/// whole as SY-38 says, in a store that has recorded removals.
///
/// Green from the start when the record is probed by identity; it guards
/// against a store that, once anything is removed, skips every unheld own
/// claim (the sync-testkit's `SkipsEveryUnheldOwnIdentity`).
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_own_claim_never_held_is_refused_after_a_removal() {
    let (store, _) = named_store().await;
    let first = append(&store, &[event("Local", "a")]).await;
    let head = append(&store, &[event("Local", "b")]).await;
    remove(store.pool(), &[first]).await;
    let never = head
        .next()
        .and_then(SequencePosition::next)
        .and_then(SequencePosition::next)
        .expect("three past a fresh head exist");
    let claim = EventId::new(own(&store).await, never);
    let batch = [
        shipped(id(peer(0x63), 1), 1, event("Shipped", "z")),
        shipped(claim, 2, event("Local", "forged")),
    ];
    let before = log(&store).await;

    let refused = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("a claim never held is refused after a removal too");

    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == claim),
        "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
    assert_eq!(
        log(&store).await,
        before,
        "a refused ingest wrote something"
    );
}

/// SY-11 over SY-40: a group made only of removed and held events lands
/// nothing, so its compensation is not written either.
///
/// **Red before the record:** `(1, 1, 1)`, the removed event re-admitted and
/// answered a second time.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_group_of_removed_and_held_events_writes_no_compensation() {
    let (store, _) = named_store().await;
    let from = peer(0x64);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let answer = [event("ReservationCompensated", "seat")];
    ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("the group lands with its compensation");
    remove(store.pool(), &[position_of(&store, group[0].id).await]).await;
    let pruned = log(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(
        counts(again),
        (0, 2, 0),
        "compensated a group that landed nothing"
    );
    assert_eq!(log(&store).await, pruned);
}

/// SY-11 over SY-40: a group with one new event beside a removed one lands
/// the new event and its compensation once, and skips the removed one.
///
/// **Red before the record:** `(2, 0, 1)`, the removed event landing with the
/// new one.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_group_with_one_new_event_lands_it_and_compensates_once() {
    let (store, _) = named_store().await;
    let from = peer(0x65);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let (removed, fresh) = (&group[0], &group[1]);
    ingest(&store, &[IngestGroup::new(&group[..1], &[])])
        .await
        .expect("the first event lands");
    remove(store.pool(), &[position_of(&store, removed.id).await]).await;
    let answer = [event("ReservationCompensated", "seat")];

    let landed = ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("a group with one new event lands");

    assert_eq!(counts(landed), (1, 1, 1));
    assert!(holds(&store, fresh.id).await);
    assert!(!holds(&store, removed.id).await);
    assert_eq!(rows(&store).await, 2, "the new event and its answer");
}

/// A call that only re-offers removed events lands nothing and draws no
/// sequence value: a recorded event is dropped before positions are
/// allocated, so the sequence and the table are as the removal left them.
///
/// **Red before the record:** the removed event lands, and the sequence moves.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_skip_against_a_removal_draws_no_position() {
    let (store, _) = named_store().await;
    let from = peer(0x66);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two foreign events land");
    remove(store.pool(), &[position_of(&store, batch[0].id).await]).await;
    let before = (sequence(store.pool()).await, log(&store).await);

    let again = ingest(&store, &[IngestGroup::new(&batch[..1], &[])])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(counts(again), (0, 1, 0));
    assert_eq!(
        (sequence(store.pool()).await, log(&store).await),
        before,
        "a skip-only call drew a position or wrote a row"
    );
}

/// I-4: the record is a table in the store's schema, so it reaches a handle
/// connected before the removal, one connected after it, and one on a pool
/// opened after the first was closed.
///
/// **Named wrong store:** a record kept per handle, as
/// `RecordsRemovalsPerHandle` in the sync-testkit's wrong stores keeps it;
/// the handles opened after the removal re-admit the event.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn the_record_reaches_every_handle_and_survives_reopen() {
    let (earlier, schema) = named_store().await;
    let batch = [shipped(id(peer(0x67), 1), 1, event("Shipped", "a"))];
    ingest(&earlier, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a foreign event lands");
    remove(earlier.pool(), &[position_of(&earlier, batch[0].id).await]).await;
    let later = PostgresEventStore::new(earlier.pool().clone());

    for (which, store) in [("earlier", &earlier), ("later", &later)] {
        let again = ingest(store, &[IngestGroup::new(&batch, &[])])
            .await
            .expect("a re-delivery is not an error");
        assert_eq!(counts(again), (0, 1, 0), "through the {which} handle");
    }
    earlier.pool().close().await;
    drop((earlier, later));

    let reopened = PostgresEventStore::new(server::pool_onto(&schema).await);
    let again = ingest(&reopened, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!(counts(again), (0, 1, 0), "after reopen");
    assert!(!holds(&reopened, batch[0].id).await);
}

/// The trigger records the origin pair of every row deleted from `event`,
/// own and foreign, and writes it to the record in the **table's** schema,
/// whatever the deleting session's `search_path` says.
///
/// The deleter's `search_path` names a second, fully migrated schema, so a
/// trigger that resolved `event_removed` through the path would write the
/// identities there, silently; this asserts the decoy's record stays empty.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_raw_delete_records_the_identity_whatever_the_search_path() {
    let (store, schema) = named_store().await;
    let local = append(&store, &[event("Local", "a")]).await;
    let foreign = [shipped(id(peer(0xF0), 7), 7, event("Shipped", "b"))];
    ingest(&store, &[IngestGroup::new(&foreign, &[])])
        .await
        .expect("a foreign event lands");
    let foreign_at = position_of(&store, foreign[0].id).await;
    let decoy = server::fresh_pool().await;

    let deleted = sqlx::query(&format!(
        r#"DELETE FROM "{schema}".event WHERE position = ANY($1)"#
    ))
    .bind([bigint(local), bigint(foreign_at)])
    .execute(&decoy)
    .await
    .expect("a delete through another schema's path commits")
    .rows_affected();
    assert_eq!(deleted, 2);

    let mut expected = vec![
        (own(&store).await.to_bytes().to_vec(), bigint(local)),
        (peer(0xF0).to_bytes().to_vec(), 7),
    ];
    expected.sort();
    assert_eq!(record(store.pool()).await, expected);
    assert!(
        record(&decoy).await.is_empty(),
        "the trigger wrote to the deleting session's search_path"
    );
}

/// A row with no origin pair records nothing when it is deleted: the trigger's
/// `WHEN` mirrors the origin index's predicate. Such a row was never in the
/// dedupe set, and no peer can offer its identity back.
///
/// Rejects an unguarded trigger, whose insert of a NULL key fails the
/// operator's `DELETE` outright.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_row_with_no_origin_records_nothing() {
    let (store, _) = named_store().await;
    let pool = store.pool();
    sqlx::query(
        "INSERT INTO event (position, event_type, data, metadata, tags, origin_store, \
         origin_position, recorded_at) VALUES (nextval('event_position_seq'), 'Legacy', \
         '\\x00', NULL, '{}', NULL, NULL, 0)",
    )
    .execute(pool)
    .await
    .expect("a row from before stamping inserts");

    sqlx::query("DELETE FROM event WHERE event_type = 'Legacy'")
        .execute(pool)
        .await
        .expect("deleting a row with no origin is not refused");

    assert!(record(pool).await.is_empty());
}

/// A rolled-back `DELETE` records nothing: the record is written in the
/// deleting transaction, and goes with it.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_rolled_back_delete_records_nothing() {
    let (store, _) = named_store().await;
    let local = append(&store, &[event("Local", "a")]).await;
    let mut operator = store.pool().begin().await.expect("a transaction opens");
    sqlx::query("DELETE FROM event WHERE position = $1")
        .bind(bigint(local))
        .execute(&mut *operator)
        .await
        .expect("the delete runs");
    operator.rollback().await.expect("the delete rolls back");

    assert!(record(store.pool()).await.is_empty());
    assert!(holds(&store, EventId::new(own(&store).await, local)).await);
}

/// A schema an older build migrated to version 2 is brought to version 3 by
/// this build's `apply`, keeping its rows, and applying it again changes
/// nothing: one trigger, one record, the same rows. Removals made before the
/// upgrade are not recorded; one made after it is.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_version_two_schema_upgrades_in_place_and_reapplies_as_a_no_op() {
    let pool = server::empty_pool().await;
    sqlx::raw_sql(migration::WRITE_GROUP_LOG)
        .execute(&pool)
        .await
        .expect("a version-2 build's migrations apply");
    let store = PostgresEventStore::new(pool.clone());
    let before_upgrade = append(&store, &[event("Local", "early")]).await;
    let kept = append(&store, &[event("Local", "kept")]).await;
    remove(&pool, &[before_upgrade]).await;

    for _ in 0..2 {
        migration::apply(&pool)
            .await
            .expect("this build's migrations apply over version 2");
    }

    let triggers: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM pg_trigger \
         WHERE tgrelid = 'event'::regclass AND tgname = 'event_record_removal'",
    )
    .fetch_one(&pool)
    .await
    .expect("the catalogue reads");
    assert_eq!(triggers, 1, "applying twice made a second trigger");
    assert!(
        record(&pool).await.is_empty(),
        "a removal from before the upgrade was recorded"
    );
    assert!(holds(&store, EventId::new(own(&store).await, kept)).await);

    remove(&pool, &[kept]).await;
    assert_eq!(
        record(&pool).await,
        [(own(&store).await.to_bytes().to_vec(), bigint(kept))]
    );
}

/// I-6, one way: a removal waits for an ingest in flight. Transaction A runs
/// the ingest's own first round trip, which takes the record's `SHARE` lock,
/// and stays open; a `DELETE` on a session with a 50 ms `lock_timeout` then
/// fails with `55P03`, classed transient, and deletes nothing. Once A
/// commits, the same `DELETE` lands and is recorded.
///
/// Rejects an ingest that reads the record without the lock, or under a mode
/// a `ROW EXCLUSIVE` writer does not wait for: the `DELETE` would commit
/// between the ingest's read of the record and its insert.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_removal_waits_for_an_ingest_in_flight() {
    let (store, schema) = named_store().await;
    let local = append(&store, &[event("Local", "a")]).await;
    let mut ingesting = store.pool().begin().await.expect("A opens");
    sqlx::raw_sql(FIRST_ROUND_TRIP_SQL)
        .fetch_one(&mut *ingesting)
        .await
        .expect("A takes the record's lock and reads the identity");
    let impatient = impatient_pool(&schema).await;

    let refused = sqlx::query("DELETE FROM event WHERE position = $1")
        .bind(bigint(local))
        .execute(&impatient)
        .await
        .expect_err("a removal cannot record while an ingest holds the lock");
    assert!(lock_not_available(&refused), "{refused:?}");
    assert_eq!(
        PostgresEventStoreError::from(refused).class(),
        FailureClass::Transient
    );
    assert!(holds(&store, EventId::new(own(&store).await, local)).await);

    ingesting.commit().await.expect("A commits");
    remove(&impatient, &[local]).await;
    assert_eq!(record(store.pool()).await.len(), 1);
}

/// I-6, the other way: an ingest waits for a removal in flight, and then
/// reads its record. Transaction B deletes a foreign event and stays open.
///
/// First, an ingest re-delivering it through a handle whose sessions give up
/// on a lock after 50 ms fails with `55P03`, as the store's transient error,
/// and writes nothing. Then the same ingest on a patient handle is started,
/// seen blocked behind B, and B commits: the ingest proceeds and skips the
/// event as removed.
///
/// Rejects an ingest that reads the record without waiting for an
/// uncommitted removal. Its insert would still wait, on the deleted row's
/// in-progress `DELETE`, which the timeout half alone cannot tell apart; but
/// once B commits the row is dead and the insert lands it, `(1, 0, 0)`: the
/// race D-8 closes, and the half this test exists for.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_ingest_waits_for_a_removal_in_flight() {
    let (store, schema) = named_store().await;
    let batch = [shipped(id(peer(0x68), 1), 1, event("Shipped", "a"))];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a foreign event lands");
    let at = position_of(&store, batch[0].id).await;
    let mut removing = store.pool().begin().await.expect("B opens");
    let removing_pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
        .fetch_one(&mut *removing)
        .await
        .expect("B names its backend");
    sqlx::query("DELETE FROM event WHERE position = $1")
        .bind(bigint(at))
        .execute(&mut *removing)
        .await
        .expect("B deletes, and its trigger takes the record's lock");
    let impatient = PostgresEventStore::new(impatient_pool(&schema).await);

    let refused = ingest(&impatient, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("an ingest cannot read the record while a removal holds it");
    assert!(
        matches!(&refused, IngestError::Store(PostgresEventStoreError::Driver(error)) if lock_not_available(error)),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Transient);

    let removed = batch[0].id;
    // The task must own what it uses: a refcounted handle onto the pool, and
    // the batch, which nothing reads after this.
    let ingesting = tokio::spawn({
        let store = store.clone();
        async move {
            happenstance_sync::SendIngestStore::ingest(&store, &[IngestGroup::new(&batch, &[])])
                .await
        }
    });
    wait_until_blocked_behind(store.pool(), removing_pid).await;
    removing.commit().await.expect("B commits");

    let again = ingesting
        .await
        .expect("the ingest task finishes")
        .expect("after the removal commits, the re-delivery is not an error");
    assert_eq!(counts(again), (0, 1, 0), "the removed event came back");
    assert!(!holds(&store, removed).await);
}

/// The backend blocked by `pid`: any session in this database waiting on a
/// lock `pid` holds, whichever lock that is.
const BLOCKED_BEHIND_SQL: &str = "SELECT count(*) FROM pg_stat_activity \
     WHERE datname = current_database() AND $1 = ANY(pg_blocking_pids(pid))";

/// How long the ingest may take to block before the arrangement is declared
/// wrong. Far above the round trip it needs.
const BLOCK_DEADLINE: Duration = Duration::from_secs(30);

/// Polls, a round trip at a time and with no pause beyond it, until some
/// session is waiting on a lock the backend `pid` holds.
///
/// # Panics
///
/// After [`BLOCK_DEADLINE`], because then the arrangement is wrong.
async fn wait_until_blocked_behind(pool: &PgPool, pid: i32) {
    let blocked = async {
        // Paced, so the poll yields between reads rather than spinning on
        // `pg_stat_activity` until the deadline (#517).
        let mut tick = tokio::time::interval(Duration::from_millis(10));
        loop {
            tick.tick().await;
            let waiting: i64 = sqlx::query_scalar(BLOCKED_BEHIND_SQL)
                .bind(pid)
                .fetch_one(pool)
                .await
                .expect("pg_stat_activity reads");
            if waiting > 0 {
                return;
            }
        }
    };
    tokio::time::timeout(BLOCK_DEADLINE, blocked)
        .await
        .unwrap_or_else(|_| {
            panic!("a broken arrangement: nothing blocked behind backend {pid} within {BLOCK_DEADLINE:?}")
        });
}
