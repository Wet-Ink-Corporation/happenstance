//! `happenstance-sync-testkit`'s ingest family, against this adapter.
//!
//! Mounted here rather than in `tests/` because the impl is `cfg(test)`, and a
//! `cfg(test)` impl is invisible to an integration test. The tests beside this
//! pin the port's own behaviour; this is the bar every adapter that implements
//! the port clears.
//!
//! Expected outcome: VT-5, SY-5 and SY-12 run; SY-30 runs, faulted by this
//! fixture's trigger at every cut of its group; SY-18 runs against the two
//! per-event ceilings this store imposes; SY-38 passes its refusals and
//! reports its hidden-claim half as skipped, because a refused call here has
//! consumed sequence values, so the compensation the rule then writes lands
//! past the identity it claimed. `claims.rs` verifies that half directly.

use happenstance_sync_testkit::{IngestFixture, Violation};
use happenstance_testkit::Capability;
use sqlx::PgPool;
use tokio::sync::OnceCell;

use super::super::PostgresIngestStore;
use super::server;
use crate::event_store::PostgresEventStore;

/// One fresh schema per fixture, and a handle onto it per `connect`.
///
/// The pool is built on first `connect`, on the rule's own runtime. A handle
/// is a new `PostgresEventStore` over a clone of it, so two handles a rule
/// holds are two pooled connections onto one store.
#[derive(Debug)]
struct PostgresIngestFixture {
    pool: OnceCell<PgPool>,
}

impl PostgresIngestFixture {
    fn new() -> Self {
        Self {
            pool: OnceCell::new(),
        }
    }
}

impl PostgresIngestFixture {
    /// This fixture's one pool, built on first use.
    async fn pool(&self) -> &PgPool {
        self.pool.get_or_init(server::fresh_pool).await
    }
}

impl IngestFixture for PostgresIngestFixture {
    type Store = PostgresIngestStore;

    /// Supported, by an `AFTER INSERT` row trigger in this fixture's schema
    /// that counts the rows the armed transaction inserts and raises on the
    /// `after + 1`-th. Row-level `AFTER` triggers fire only for rows that
    /// were inserted, so a row `ON CONFLICT` skipped is not counted, and they
    /// fire in insertion order across both of a group's statements, its
    /// foreign rows and its compensation. The raise aborts the transaction,
    /// which is the unit an ingest commits.
    const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the constants `exceeded` holds every foreign and
    /// compensation event to before the transaction opens. No metadata
    /// ceiling: this store states none, and `exceeded` does not bound it.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(PostgresEventStore::MAX_EVENT_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(PostgresEventStore::MAX_TAGS_PER_EVENT);

    async fn connect(&self) -> Self::Store {
        // A refcounted handle onto the one pool, which every handle shares.
        let pool = self.pool().await.clone();
        PostgresIngestStore::open(PostgresEventStore::new(pool))
            .await
            .expect("a broken test environment: a freshly migrated schema has an identity")
    }

    /// Installs the trigger `MID_GROUP_FAULT` describes. The counter is a
    /// transaction-local setting, so each ingest counts from zero; the
    /// trigger stays for the fixture's life, which the arm's contract allows.
    async fn arm_mid_group_fault(&self, after: usize) -> Result<(), Violation> {
        // Only an integer is formatted into the statement, so there is
        // nothing to inject.
        let threshold = after
            .checked_add(1)
            .and_then(|row| i64::try_from(row).ok())
            .ok_or_else(|| {
                Violation::new("SY-30", format!("a cut at row {after} overflows an i64"))
            })?;
        let sql = format!(
            "CREATE OR REPLACE FUNCTION mid_group_fault() RETURNS trigger AS $fn$ \
             DECLARE seen bigint; \
             BEGIN \
                 seen := coalesce(nullif(current_setting('hs.mid_group_seen', true), ''), \
                     '0')::bigint + 1; \
                 PERFORM set_config('hs.mid_group_seen', seen::text, true); \
                 IF seen = {threshold} THEN \
                     RAISE EXCEPTION 'the fixture armed a mid-group fault'; \
                 END IF; \
                 RETURN NEW; \
             END $fn$ LANGUAGE plpgsql; \
             DROP TRIGGER IF EXISTS mid_group_fault ON event; \
             CREATE TRIGGER mid_group_fault AFTER INSERT ON event \
                 FOR EACH ROW EXECUTE FUNCTION mid_group_fault();"
        );
        sqlx::raw_sql(&sql)
            .execute(self.pool().await)
            .await
            .map(|_| ())
            .map_err(|error| {
                Violation::new(
                    "SY-30",
                    format!("the Postgres fixture could not arm its trigger: {error}"),
                )
            })
    }
}

/// `happenstance_sync_testkit::emit_ingest_tokio`, with this crate's
/// `#[ignore]` reason added, as every live test here carries.
macro_rules! emit_ignored_ingest_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
            async fn $name() -> ::core::result::Result<(), ::happenstance_sync_testkit::Violation> {
                ::happenstance_sync_testkit::ingest_rules::$name(__ingest_fixture)
                    .await?
                    .report(::core::stringify!($name));
                Ok(())
            }
        )*
    };
}
pub(crate) use emit_ignored_ingest_tokio;

happenstance_sync_testkit::ingest_store_conformance!(
    mod_name = ingest_conformance,
    emit = super::emit_ignored_ingest_tokio,
    fixture = super::PostgresIngestFixture::new()
);
