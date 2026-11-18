//! Migration 4 under least-privileged roles (#483, #484, review F1).
//!
//! The record adds two privilege questions a deployment answers with grants,
//! and these tests pin the answers the README documents:
//!
//! * a role that deletes from `event` needs nothing on `event_removed`: the
//!   trigger's function is `SECURITY DEFINER`, so its insert runs as the
//!   function's owner and not as the deleting role;
//! * a role that ingests needs `SELECT` and `UPDATE` on `event_removed`, on top
//!   of what a `0.7` ingest needed: `SELECT` for the anti-join, and `UPDATE`
//!   because `LOCK TABLE ... IN SHARE MODE` is stronger than `ROW EXCLUSIVE`,
//!   which Postgres grants only to a role holding `UPDATE`, `DELETE`,
//!   `TRUNCATE` or (from 17) `MAINTAIN`.
//!
//! Each test creates its roles with `LOGIN` and connects as them, so the server
//! checks privileges exactly as it would for a retention job or a replication
//! process, and drops them once it has passed. Role names derive from the
//! test's schema, which is unique to this process, because roles are global to
//! the cluster and the container is reused between runs.

use std::str::FromStr;
use std::time::Duration;

use happenstance_sync::{IngestError, IngestGroup};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Executor, PgPool};

use super::{carrying, event, id, ingest, log, peer, server, shipped};
use crate::error::PostgresEventStoreError;
use crate::event_store::PostgresEventStore;
use crate::migration;

/// The throwaway password every role here logs in with.
const PASSWORD: &str = "least-privilege";

/// A migrated schema, a superuser pool onto it, and the schema's name.
async fn migrated() -> (PgPool, String) {
    let schema = server::fresh_schema_name();
    let pool = server::pool_onto(&schema).await;
    migration::apply(&pool)
        .await
        .expect("the event log's migrations apply to a fresh schema");
    (pool, schema)
}

/// Creates `role` with `LOGIN` and runs `grants` for it, through `admin`.
/// Each grant names the role as `{role}`, the schema as `{schema}` and its
/// tables unqualified, as `admin`'s `search_path` resolves them.
async fn create_role(admin: &PgPool, role: &str, schema: &str, grants: &[&str]) {
    admin
        .execute(format!("CREATE ROLE \"{role}\" LOGIN PASSWORD '{PASSWORD}'").as_str())
        .await
        .expect("the test's superuser creates a role");
    for grant in grants {
        admin
            .execute(
                grant
                    .replace("{role}", &format!("\"{role}\""))
                    .replace("{schema}", &format!("\"{schema}\""))
                    .as_str(),
            )
            .await
            .unwrap_or_else(|error| panic!("`{grant}` applies: {error}"));
    }
}

/// Revokes what `role` was granted and drops it: reached only by a test that
/// passed, as the other live tests leave their schemas.
async fn drop_role(admin: &PgPool, role: &str) {
    admin
        .execute(format!("DROP OWNED BY \"{role}\"; DROP ROLE \"{role}\"").as_str())
        .await
        .expect("the role is dropped once its grants are revoked");
}

/// A pool onto `schema` that logs in as `role`.
async fn pool_as(role: &str, schema: &str) -> PgPool {
    let options = PgConnectOptions::from_str(server::server_url().await)
        .expect("the server's URL parses")
        .username(role)
        .password(PASSWORD)
        .options([("search_path", schema)]);
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await
        .expect("a least-privileged role logs in")
}

/// Whether `error` is the server refusing a privilege (`42501`).
fn insufficient_privilege(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(sqlx::error::DatabaseError::code)
        .as_deref()
        == Some("42501")
}

/// The grants a retention role holds: it reads and deletes rows of `event`,
/// and holds nothing on the record.
const DELETER_GRANTS: &[&str] = &[
    "GRANT USAGE ON SCHEMA {schema} TO {role}",
    "GRANT SELECT, DELETE ON event TO {role}",
];

/// A role holding only `SELECT, DELETE` on `event` deletes a row, and the
/// trigger records its identity.
///
/// Rejects a trigger function that runs as the deleting role, as a plain
/// `plpgsql` function does: that role holds no `INSERT` on `event_removed`, so
/// its `DELETE` fails with `permission denied for table event_removed`, and
/// migration 4 would have broken every retention job granted the least it
/// needed under `0.7`.
///
/// **Red before `SECURITY DEFINER`:** `the deleter's DELETE commits:
/// permission denied for table event_removed`.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_deleter_holding_only_select_and_delete_is_recorded() {
    let (admin, schema) = migrated().await;
    // A pool is a cheap handle onto shared connections: the store and the
    // checks below go through the same superuser pool.
    let store = PostgresEventStore::new(admin.clone());
    let from = peer(0x71);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two foreign events land");
    let removed = carrying(&log(&store).await, batch[0].id).position;
    let role = format!("{schema}_deleter");
    create_role(&admin, &role, &schema, DELETER_GRANTS).await;

    let deleter = pool_as(&role, &schema).await;
    let rows_gone = sqlx::query("DELETE FROM event WHERE position = $1")
        .bind(i64::try_from(removed.get()).expect("a Postgres position fits a bigint"))
        .execute(&deleter)
        .await
        .unwrap_or_else(|error| panic!("the deleter's DELETE commits: {error}"))
        .rows_affected();
    assert_eq!(rows_gone, 1, "the deleter removed the row it named");

    let record: Vec<(Vec<u8>, i64)> =
        sqlx::query_as("SELECT origin_store, origin_position FROM event_removed")
            .fetch_all(&admin)
            .await
            .expect("the record reads");
    assert_eq!(
        record,
        vec![(from.to_bytes().to_vec(), 1)],
        "the trigger recorded the deleted row's origin pair, and only it"
    );
    deleter.close().await;
    drop_role(&admin, &role).await;
}

/// What a role that ingests holds before it is granted anything on the
/// record: what a `0.7` ingest touched. `UPDATE` on `event` is the write-group
/// stamp (SY-39).
const INGESTER_GRANTS: &[&str] = &[
    "GRANT USAGE ON SCHEMA {schema} TO {role}",
    "GRANT SELECT, INSERT, UPDATE ON event TO {role}",
    "GRANT SELECT ON store_meta TO {role}",
    "GRANT USAGE ON SEQUENCE event_position_seq TO {role}",
];

/// A role that ingests needs `SELECT` and `UPDATE` on `event_removed`, the set
/// the README documents, and `SELECT` alone is not enough.
///
/// Rejects documentation that names `SELECT, INSERT` or `SELECT` alone, the
/// grants the anti-join suggests: with them, the ingest's first statement,
/// `LOCK TABLE event_removed IN SHARE MODE`, is refused with `42501` and no
/// ingest lands. And rejects a set larger than needed: with `SELECT, UPDATE`
/// added to [`INGESTER_GRANTS`], the batch lands.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn an_ingest_role_needs_select_and_update_on_the_record() {
    let (admin, schema) = migrated().await;
    let role = format!("{schema}_ingester");
    create_role(&admin, &role, &schema, INGESTER_GRANTS).await;
    admin
        .execute(format!("GRANT SELECT ON event_removed TO \"{role}\"").as_str())
        .await
        .expect("a grant of SELECT alone applies");
    let ingester = PostgresEventStore::new(pool_as(&role, &schema).await);
    let from = peer(0x72);
    let batch = [shipped(id(from, 1), 1, event("Shipped", "a"))];

    let refused = ingest(&ingester, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("SELECT alone on the record does not admit a SHARE lock");
    assert!(
        matches!(
            &refused,
            IngestError::Store(PostgresEventStoreError::Driver(error))
                if insufficient_privilege(error) && error.to_string().contains("event_removed")
        ),
        "{refused:?}"
    );

    admin
        .execute(format!("GRANT UPDATE ON event_removed TO \"{role}\"").as_str())
        .await
        .expect("the documented grant applies");
    let landed = ingest(&ingester, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("with SELECT, UPDATE on the record the ingest lands");
    assert_eq!(landed.appended, 1, "{landed:?}");

    ingester.pool().close().await;
    drop_role(&admin, &role).await;
}
