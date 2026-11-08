//! `PostgresEventStore::remint_identity`, and the append path that has to see
//! it (#104, ADR-0086, VT-6).
//!
//! A `pg_restore` of an older dump, a point-in-time recovery, or any other copy
//! brings back the `store_meta` row together with an older position sequence.
//! Without a re-mint the copy hands out `(StoreId, SequencePosition)` pairs it
//! already gave to events that are gone from it but not from its peers, which
//! is what VT-6 forbids. The restore is simulated in place: the tail row is
//! deleted and `event_position_seq` is set back to the value it held before
//! that row was written, which is what restoring a dump taken at that moment
//! does to both.
//!
//! The oracles read raw SQL, never the adapter's own identity read or `read`,
//! so no check shares a subroutine with the code under test, and no check waits
//! on the visibility frontier.
//!
//! # How this target is gated
//!
//! As `runtime_seam.rs` is: every case needs the server and is `#[ignore]`d
//! with a reason, so the default gate compiles and lints it, and CI's
//! live-postgres job runs it with `-- --ignored`.
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features --test remint -- --ignored --list
//! cargo test -p happenstance-postgres --all-features --test remint -- --ignored --show-output --test-threads=1
//! ```

#![cfg(not(target_arch = "wasm32"))]

mod support;

use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, Query, SequencePosition, StoreId,
};
use happenstance_postgres::error::PostgresEventStoreError;
use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::sqlx::{self, PgPool};
use happenstance_testkit::Fixture;
use support::PostgresFixture;

fn event(event_type: &str) -> [Event; 1] {
    [Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")]
}

/// One unconditional append, answering the position the store assigned.
async fn append(store: &PostgresEventStore, event_type: &str) -> SequencePosition {
    EventStore::append(store, &event(event_type), None)
        .await
        .expect("an unconditional append to a migrated store lands")
}

/// The `(origin_store, origin_position)` the row at `position` carries, read
/// raw, so the oracle is the table and not the adapter's decoder.
async fn stamped(pool: &PgPool, position: SequencePosition) -> (StoreId, i64) {
    let position = i64::try_from(position.get()).expect("test positions fit a bigint");
    let (store, origin): (Vec<u8>, i64) =
        sqlx::query_as("SELECT origin_store, origin_position FROM event WHERE position = $1")
            .bind(position)
            .fetch_one(pool)
            .await
            .expect("the appended row is there");
    let bytes: [u8; 16] = store.try_into().expect("origin_store is sixteen bytes");
    (StoreId::from_bytes(bytes), origin)
}

/// The identity row, read raw: every `v` under `store_id`.
async fn persisted(pool: &PgPool) -> Vec<Vec<u8>> {
    sqlx::query_scalar("SELECT v FROM store_meta WHERE k = 'store_id'")
        .fetch_all(pool)
        .await
        .expect("store_meta reads")
}

async fn rows(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM event")
        .fetch_one(pool)
        .await
        .expect("the event table counts")
}

async fn delete_identity(pool: &PgPool) {
    sqlx::query("DELETE FROM store_meta WHERE k = 'store_id'")
        .execute(pool)
        .await
        .expect("the identity row deletes");
}

/// Restores the store to the moment before `tail` was written: the row goes,
/// and the sequence goes back to `saved`, as a dump taken then would leave both.
async fn rewind(pool: &PgPool, tail: SequencePosition, saved: i64) {
    let tail = i64::try_from(tail.get()).expect("test positions fit a bigint");
    sqlx::query("DELETE FROM event WHERE position = $1")
        .bind(tail)
        .execute(pool)
        .await
        .expect("the tail row deletes");
    sqlx::query("SELECT setval('event_position_seq', $1)")
        .bind(saved)
        .execute(pool)
        .await
        .expect("the sequence rewinds");
}

async fn sequence_value(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT last_value FROM event_position_seq")
        .fetch_one(pool)
        .await
        .expect("the sequence reads")
}

/// A re-mint through one handle changes what an **already open** handle, and a
/// clone of it, stamp on their next append.
///
/// Rejects the per-handle `Arc<OnceLock<StoreId>>` this store shipped until
/// #104: that cache passes any test that opens a fresh handle after the
/// re-mint, which is why this one reuses the handle that appended first.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn remint_changes_the_id_stamped_by_an_already_open_handle() {
    let fixture = PostgresFixture::new();
    let pool = fixture.pool_for_test().await;
    let open = fixture.connect().await;
    // A clone shares whatever the handle holds, which is the point: the
    // retired cache was shared by every clone of one handle.
    let clone = open.clone();
    let first = append(&open, "BeforeRemint").await;
    let (old, _) = stamped(&pool, first).await;

    let other = fixture.connect().await;
    let minted = other
        .remint_identity()
        .await
        .expect("a migrated store re-mints");

    assert_ne!(minted, old, "a re-mint mints a different identity");
    assert_eq!(
        persisted(&pool).await,
        vec![minted.to_bytes().to_vec()],
        "the row now holds the minted identity, once"
    );
    for handle in [&open, &clone] {
        let after = append(handle, "AfterRemint").await;
        let (store, origin) = stamped(&pool, after).await;
        assert_eq!(
            store, minted,
            "an open handle stamps the re-minted identity"
        );
        assert_eq!(
            origin,
            i64::try_from(after.get()).expect("fits"),
            "the origin half is the row's own position"
        );
    }
    let (kept, _) = stamped(&pool, first).await;
    assert_eq!(
        kept, old,
        "an event written before the re-mint keeps its origin"
    );
}

/// An append refuses with `MissingIdentity`, and writes nothing, when the
/// identity row is gone, on both write paths, even on a handle that has
/// appended before.
///
/// Rejects the cache, which would go on stamping the identity it read first,
/// and the naive stamp, a scalar subquery that writes a NULL origin.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn append_refuses_without_an_identity_row() {
    let fixture = PostgresFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    append(&store, "Warm").await;
    delete_identity(&pool).await;

    let condition = AppendCondition::new(Query::all());
    for condition in [None, Some(&condition)] {
        let refused = EventStore::append(&store, &event("Refused"), condition)
            .await
            .expect_err("an append with no identity row cannot stamp an EventId");
        assert!(
            matches!(
                refused,
                AppendError::Store(PostgresEventStoreError::MissingIdentity)
            ),
            "{refused:?}"
        );
    }
    assert_eq!(rows(&pool).await, 1, "a refused append writes nothing");
}

/// A re-mint of a schema with no identity row refuses, and creates none.
///
/// Rejects an upsert, which would quietly mint an identity into a schema that
/// migration 1 never ran on.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn remint_identity_refuses_without_an_identity_row() {
    let fixture = PostgresFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    delete_identity(&pool).await;

    let refused = store
        .remint_identity()
        .await
        .expect_err("there is no row to re-mint");
    assert!(
        matches!(refused, PostgresEventStoreError::MissingIdentity),
        "{refused:?}"
    );
    assert!(persisted(&pool).await.is_empty(), "no row was created");
}

/// The negative control: a restored store that is **not** re-minted re-issues
/// the identity of an event it lost, to a different event.
///
/// Without this the positive case below is decorative: it would pass on a
/// rewind that never brought the position back.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn without_a_remint_a_restored_store_reissues_an_event_id() {
    let fixture = PostgresFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    append(&store, "Kept").await;
    let saved = sequence_value(&pool).await;
    let lost = append(&store, "Lost").await;
    let lost_id = stamped(&pool, lost).await;

    rewind(&pool, lost, saved).await;
    let reissued = append(&store, "Different").await;

    assert_eq!(
        stamped(&pool, reissued).await,
        lost_id,
        "an un-reminted restore re-issues the lost event's identity to a different event"
    );
}

/// After a restore, `remint_identity` on the same handle that appended before
/// the restore means its next event's identity is not one already issued.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_reminted_restored_store_does_not_reissue_an_event_id() {
    let fixture = PostgresFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = fixture.connect().await;
    let kept = append(&store, "Kept").await;
    let saved = sequence_value(&pool).await;
    let lost = append(&store, "Lost").await;
    let issued = [stamped(&pool, kept).await, stamped(&pool, lost).await];

    rewind(&pool, lost, saved).await;
    let minted = store
        .remint_identity()
        .await
        .expect("the restored store re-mints");
    let next = append(&store, "Different").await;
    let next_id = stamped(&pool, next).await;

    assert!(
        !issued.contains(&next_id),
        "{next_id:?} was already issued: {issued:?}"
    );
    assert_eq!(next_id.0, minted, "the next event carries the new identity");
}
