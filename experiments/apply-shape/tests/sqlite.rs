//! Case 2: `SqliteProjectionStore` — a buffered batch of SQL statements.
//!
//! `apply` pushes a statement into `SqliteBatch` and never awaits; `on_error`
//! pushes a skip record into the same batch, through the store's inherent API
//! and with no bound on `Batch` anywhere in the library (PS-9, PS-11). Both are
//! read back through a **fresh connection** after the store is dropped, so what
//! is asserted is what the file holds.

#![cfg(feature = "native")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

mod common;

use core::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use apply_shape::domain::{POISON, Ticked, scope, tick, undecodable};
use apply_shape::{ApplyFailure, Delivered, Policy, SendProjection};
use happenstance::{MemoryEventStore, ProjectionId, Tags};
use happenstance_sqlite::projection_store::{
    SqliteBatch, SqliteProjectionStore, SqliteProjectionStoreError,
};
use happenstance_sqlite::rusqlite::{Connection, types::Value};

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS tally   (key TEXT PRIMARY KEY, n INTEGER NOT NULL);
    CREATE TABLE IF NOT EXISTS skipped (event_id TEXT PRIMARY KEY, reason TEXT NOT NULL);
";

#[derive(Debug, thiserror::Error)]
enum TallyError {
    #[error(transparent)]
    Store(#[from] SqliteProjectionStoreError),
    #[error("refused a tick against `{0}`")]
    Refused(String),
}

struct SqlTally {
    id: ProjectionId,
    scope: Tags,
}

impl SendProjection for SqlTally {
    type Event = Ticked;
    type Store = SqliteProjectionStore;
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
        batch: &mut SqliteBatch,
    ) -> Result<(), TallyError> {
        let Ticked { key } = event.into_event();
        if key == POISON {
            return Err(TallyError::Refused(key));
        }
        batch.push(
            "INSERT INTO tally (key, n) VALUES (?1, 1) \
             ON CONFLICT (key) DO UPDATE SET n = n + 1",
            [Value::Text(key)],
        );
        Ok(())
    }

    async fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, TallyError>,
        batch: &mut SqliteBatch,
    ) -> Result<Policy, TallyError> {
        let reason = match failure {
            ApplyFailure::Decode { .. } => "decode",
            _ => "apply",
        };
        batch.push(
            "INSERT INTO skipped (event_id, reason) VALUES (?1, ?2)",
            [
                Value::Text(failure.id().to_string()),
                Value::Text(reason.to_owned()),
            ],
        );
        Ok(Policy::Skip)
    }
}

/// A fresh database file with the read-model tables in it.
fn database(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("apply-shape-{}-{name}.db", std::process::id()));
    for suffix in ["", "-wal", "-shm"] {
        let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
    }
    Connection::open(&path)
        .unwrap()
        .execute_batch(SCHEMA)
        .unwrap();
    path
}

/// The read model's two tables, as a fresh connection sees them.
type Rows = (Vec<(String, i64)>, Vec<(String, String)>);

fn rows(path: &Path) -> Rows {
    let connection = Connection::open(path).unwrap();
    let tally = connection
        .prepare("SELECT key, n FROM tally ORDER BY key")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let skipped = connection
        .prepare("SELECT event_id, reason FROM skipped ORDER BY reason")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    (tally, skipped)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_buffered_sql_projection_applies_and_skips_through_the_spawned_runner() {
    let events = MemoryEventStore::with_events([
        tick("a").unwrap(),
        tick(POISON).unwrap(),
        tick("b").unwrap(),
        undecodable().unwrap(),
        tick("a").unwrap(),
    ]);
    let snapshot = events.snapshot();
    let poison_id = snapshot[1].id.to_string();
    let shredded_id = snapshot[3].id.to_string();
    let head = events.last_position();

    let path = database("buffered");
    let store = SqliteProjectionStore::open(&path).unwrap();
    let tally = SqlTally {
        id: ProjectionId::from_static("sql-tally"),
        scope: scope(),
    };

    // The same generic spawner the memory case uses: a second store type
    // through one `P: SendProjection` signature.
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

    let (tally, skipped) = rows(&path);
    assert_eq!(tally, [("a".to_owned(), 2), ("b".to_owned(), 1)]);
    assert_eq!(
        skipped,
        [
            (poison_id, "apply".to_owned()),
            (shredded_id, "decode".to_owned())
        ],
        "each skip record committed, keyed by the origin identity"
    );

    // And the checkpoint moved past both, in the same transaction.
    let reopened = SqliteProjectionStore::open(&path).unwrap();
    let checkpoint = happenstance::SendProjectionStore::checkpoint(
        &reopened,
        &ProjectionId::from_static("sql-tally"),
    )
    .await
    .unwrap();
    assert_eq!(
        checkpoint,
        happenstance::Checkpoint::Live {
            through: head.unwrap()
        }
    );
}
