//! `happenstance-sync-testkit`'s ingest family, against this adapter.
//!
//! Mounted here rather than in `tests/` because the impl is `cfg(test)`, and a
//! `cfg(test)` impl is invisible to an integration test. The tests beside this
//! pin the port's own behaviour; this is the bar every adapter that implements
//! the port clears.
//!
//! Expected outcome: VT-5, SY-5 and SY-12 run; SY-38 passes its refusals and
//! reports its hidden-claim half as skipped, because a refused call here has
//! consumed sequence values, so the compensation the rule then writes lands
//! past the identity it claimed. `claims.rs` verifies that half directly.

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

impl happenstance_sync_testkit::IngestFixture for PostgresIngestFixture {
    type Store = PostgresIngestStore;

    async fn connect(&self) -> Self::Store {
        // A refcounted handle onto the one pool, which every handle shares.
        let pool = self.pool.get_or_init(server::fresh_pool).await.clone();
        PostgresIngestStore::open(PostgresEventStore::new(pool))
            .await
            .expect("a broken test environment: a freshly migrated schema has an identity")
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
