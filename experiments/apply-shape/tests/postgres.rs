//! Case 3: `LivePostgresProjectionStore` — the batch **is** a live `sqlx`
//! transaction, and `apply` awaits a statement through it.
//!
//! This is the case a synchronous `apply` cannot express at all
//! (`crates/happenstance-postgres/src/live_projection_store.rs:38-47` says so),
//! and the one phase 18's exit criterion names: *"a projection can write into a
//! live batch, demonstrated against a real database rather than a buffer."*
//!
//! # Running it
//!
//! `#[ignore]`d, because it needs a server. Start one, point the test at it,
//! and stop it afterwards — started by hand, so it is stopped by hand and
//! nothing leaks (the reason the workspace's own fixture needs
//! `reusable-containers` does not arise):
//!
//! ```console
//! docker run -d --rm --name apply-shape-pg -p 55432:5432 \
//!     -e POSTGRES_PASSWORD=postgres postgres:17.10
//! APPLY_SHAPE_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres \
//!     cargo test --test postgres -- --ignored --test-threads=1
//! docker stop apply-shape-pg
//! ```
//!
//! `postgres:17.10` is the server `experiments/position-visibility/` and the
//! adapter's own conformance fixture pin. A URL rather than `testcontainers`
//! because this crate's graph is its own and `testcontainers` is the heaviest
//! thing the workspace would lend it; the fixture's `static`-held container
//! is also the leak the workspace manifest documents.

#![cfg(feature = "native")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use core::num::NonZeroUsize;
use std::sync::Arc;

use apply_shape::domain::{POISON, Ticked, scope, tick, undecodable};
use apply_shape::{ApplyFailure, Delivered, Policy, SendProjection};
use happenstance::{MemoryEventStore, ProjectionId, Tags};
use happenstance_postgres::error::PostgresProjectionStoreError;
use happenstance_postgres::live_projection_store::{
    LivePostgresBatch, LivePostgresProjectionStore,
};
use happenstance_postgres::migration;
use happenstance_postgres::projection_store::PgParam;
use happenstance_postgres::sqlx::postgres::PgPoolOptions;
use happenstance_postgres::sqlx::{self, Executor, PgPool, Row};

/// A key whose `apply` issues a statement the **server** refuses, so the live
/// transaction is aborted before `on_error` runs.
const BAD_SQL: &str = "bad-sql";

#[derive(Debug, thiserror::Error)]
enum TallyError {
    #[error(transparent)]
    Store(#[from] PostgresProjectionStoreError),
    #[error("refused a tick against `{0}`")]
    Refused(String),
}

struct LiveTally {
    id: ProjectionId,
    scope: Tags,
}

impl SendProjection for LiveTally {
    type Event = Ticked;
    type Store = LivePostgresProjectionStore;
    type Error = TallyError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Ticked>,
        batch: &mut LivePostgresBatch,
    ) -> Result<(), TallyError> {
        let Ticked { key } = event.into_event();
        if key == POISON {
            // Refused in application code, before any statement: the
            // transaction is still healthy when `on_error` writes into it.
            return Err(TallyError::Refused(key));
        }
        if key == BAD_SQL {
            // Refused by the server: the transaction is now aborted.
            batch
                .execute(
                    "INSERT INTO no_such_table (key) VALUES ($1)",
                    [PgParam::text(key)],
                )
                .await?;
            return Ok(());
        }
        // The await a synchronous `apply` cannot write. `?` converts the
        // adapter's error through `type Error: From<..>`.
        batch
            .execute(
                "INSERT INTO tally (key, n) VALUES ($1, 1) \
                 ON CONFLICT (key) DO UPDATE SET n = tally.n + 1",
                [PgParam::text(key)],
            )
            .await?;
        Ok(())
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, TallyError>,
        batch: &mut LivePostgresBatch,
    ) -> Result<Policy, TallyError> {
        let reason = match failure {
            ApplyFailure::Decode { .. } => "decode",
            _ => "apply",
        };
        batch
            .execute(
                "INSERT INTO skipped (event_id, reason) VALUES ($1, $2)",
                [
                    PgParam::text(failure.id().to_string()),
                    PgParam::text(reason),
                ],
            )
            .await?;
        Ok(Policy::Skip)
    }
}

/// A pool onto a fresh schema holding the checkpoint table and the read model.
async fn pool(name: &str) -> PgPool {
    let url = std::env::var("APPLY_SHAPE_DATABASE_URL").unwrap_or_else(|_| {
        panic!(
            "a broken test environment: set APPLY_SHAPE_DATABASE_URL to a Postgres \
             17.10 server (see this file's module documentation for the docker line)"
        )
    });
    let schema = format!("apply_shape_{}_{name}", std::process::id());
    let search_path = schema.clone();
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .after_connect(move |connection, _meta| {
            let schema = search_path.clone();
            Box::pin(async move {
                connection
                    .execute(format!(r#"SET search_path TO "{schema}""#).as_str())
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .unwrap();
    pool.execute(format!(r#"CREATE SCHEMA IF NOT EXISTS "{schema}""#).as_str())
        .await
        .unwrap();
    migration::apply_projection(&pool).await.unwrap();
    sqlx::raw_sql(
        "CREATE TABLE tally   (key TEXT PRIMARY KEY, n BIGINT NOT NULL);
         CREATE TABLE skipped (event_id TEXT PRIMARY KEY, reason TEXT NOT NULL);",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "needs a live Postgres: set APPLY_SHAPE_DATABASE_URL and run with `-- --ignored`"]
async fn apply_awaits_statements_through_a_live_transaction() {
    let events = MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick(POISON).unwrap(),
        tick("b").unwrap(),
        undecodable().unwrap(),
        tick("a").unwrap(),
    ]);
    let head = events.last_position();
    let pool = pool("live").await;
    let store = LivePostgresProjectionStore::new(pool.clone());
    let tally = LiveTally {
        id: ProjectionId::new("live-tally"),
        scope: scope(),
    };

    // Spawned, from code generic over `P` — the third store type through the
    // one signature.
    let (_tally, ran) = common::spawn_run(
        Arc::new(events),
        Arc::new(store),
        tally,
        NonZeroUsize::new(2).unwrap(),
    )
    .await
    .unwrap();
    let ran = ran.unwrap();

    assert_eq!((ran.applied, ran.skipped), (3, 2));
    assert_eq!(ran.through, head);

    let tally: Vec<(String, i64)> = sqlx::query("SELECT key, n FROM tally ORDER BY key")
        .fetch_all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|row| (row.get(0), row.get(1)))
        .collect();
    assert_eq!(tally, [("a".to_owned(), 2), ("b".to_owned(), 1)]);
    let skipped: i64 = sqlx::query("SELECT count(*) FROM skipped")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    assert_eq!(skipped, 2, "both skip records committed");

    // And the checkpoint is committed at the head, read back through a fresh
    // store over the same server rather than taken from the runner's report.
    let reread = LivePostgresProjectionStore::new(pool.clone());
    let checkpoint =
        happenstance::SendProjectionStore::checkpoint(&reread, &ProjectionId::new("live-tally"))
            .await
            .unwrap();
    assert_eq!(
        checkpoint,
        happenstance::Checkpoint::Live {
            through: head.unwrap()
        },
        "the skip records committed with the checkpoint"
    );
}

/// The limit of `Policy::Skip` over a live batch, stated before it was run.
///
/// A statement the server refuses aborts the transaction, so `on_error`'s skip
/// write is refused too, and the run stops with `RunError::Policy` carrying the
/// adapter's error — the chunk rolled back, the checkpoint unmoved. Skipping a
/// **server-side** failure on a live batch needs a `SAVEPOINT` around each
/// `apply`, which is the adapter's to offer or the runner's to issue; the
/// record should name it as open rather than let `Skip` read as universal.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "needs a live Postgres: set APPLY_SHAPE_DATABASE_URL and run with `-- --ignored`"]
async fn a_server_side_failure_leaves_nothing_for_on_error_to_write_into() {
    let events = MemoryEventStore::with_events([tick("a").unwrap(), tick(BAD_SQL).unwrap()]);
    let pool = pool("poisoned").await;
    let store = LivePostgresProjectionStore::new(pool.clone());
    let mut tally = LiveTally {
        id: ProjectionId::new("poisoned"),
        scope: scope(),
    };

    let error = apply_shape::run(
        &events,
        &store,
        &mut tally,
        &happenstance::Json,
        NonZeroUsize::new(8).unwrap(),
    )
    .await
    .unwrap_err();

    assert!(
        matches!(
            error,
            apply_shape::RunError::Policy {
                source: TallyError::Store(_),
                ..
            }
        ),
        "expected on_error's own write to be refused by the aborted transaction: {error:?}"
    );
    let rows: i64 = sqlx::query("SELECT count(*) FROM tally")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    assert_eq!(rows, 0, "the chunk rolled back whole");

    // And the checkpoint did not move: nothing was ever committed for it.
    let reread = LivePostgresProjectionStore::new(pool.clone());
    let checkpoint =
        happenstance::SendProjectionStore::checkpoint(&reread, &ProjectionId::new("poisoned"))
            .await
            .unwrap();
    assert_eq!(
        checkpoint,
        happenstance::Checkpoint::NeverRun,
        "the checkpoint unmoved"
    );
}

/// Records which server the two cases above ran against, so the transcript
/// carries the version rather than the README asserting it. Printed, and run
/// with `--nocapture` by `run.sh`; it asserts only that the server answered.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "needs a live Postgres: set APPLY_SHAPE_DATABASE_URL and run with `-- --ignored`"]
async fn records_the_server_version() {
    let pool = pool("version").await;
    let version: String = sqlx::query("SELECT version()")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
    println!("server: {version}");
    assert!(version.starts_with("PostgreSQL "), "{version}");
}
