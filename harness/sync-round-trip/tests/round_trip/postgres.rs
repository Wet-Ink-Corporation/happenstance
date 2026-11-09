//! SQLite through `StoreSyncPeer<PostgresEventStore, PostgresEventStore>`,
//! the Postgres sync peer, against a live server.
//!
//! The bootstrap is `happenstance-postgres`'s own live tests' restated,
//! small, because theirs is test code no other crate can reach, and its
//! constants must move with `crates/happenstance-postgres/tests/support/mod.rs`:
//! the same image and tag, the same `max_connections` and the same container
//! name, so a run attaches to the container the conformance suite already
//! reuses. `HAPPENSTANCE_POSTGRES_URL`, read first, names a server instead.
//! Each test gets its own schema, `hsr_<pid>_<n>`.
//!
//! # Why the functions here panic
//!
//! A server that cannot be reached is a broken test environment, not a peer
//! defect, and the message says so.

use std::env::VarError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::migration;
use happenstance_sync::StoreSyncPeer;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, ReuseDirective};

/// A server to use instead of the container, as a connection URL.
const URL_VARIABLE: &str = "HAPPENSTANCE_POSTGRES_URL";
/// `tests/support/mod.rs`'s, which this must match.
const POSTGRES_IMAGE: &str = "postgres";
/// `tests/support/mod.rs`'s, which this must match.
const POSTGRES_TAG: &str = "17.10";
/// `tests/support/mod.rs`'s, which this must match.
const CONTAINER_NAME: &str = "happenstance-postgres-conformance";
/// The port inside the container. The host port is mapped and read back.
const POSTGRES_PORT: u16 = 5432;
/// The container's throwaway superuser, database and password.
const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

/// A reachable server: the URL, and the container if one was started, held
/// so it outlives the test that started it.
struct Server {
    _container: Option<ContainerAsync<GenericImage>>,
    url: String,
}

/// The URL in [`URL_VARIABLE`] if it is set, else the shared container.
///
/// # Panics
///
/// If the variable is not Unicode, or the container cannot be started.
async fn server() -> Server {
    match std::env::var(URL_VARIABLE) {
        Ok(url) => Server {
            _container: None,
            url,
        },
        Err(VarError::NotPresent) => start_container().await,
        Err(VarError::NotUnicode(raw)) => {
            panic!(
                "a broken test environment: {URL_VARIABLE} is not Unicode ({})",
                raw.display()
            )
        }
    }
}

/// Starts the shared container, or attaches to the one already running.
///
/// # Panics
///
/// If Docker is unreachable, or the container maps no host or port.
async fn start_container() -> Server {
    let container = GenericImage::new(POSTGRES_IMAGE, POSTGRES_TAG)
        .with_exposed_port(POSTGRES_PORT.tcp())
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
        .with_env_var("POSTGRES_USER", POSTGRES_USER)
        .with_env_var("POSTGRES_DB", POSTGRES_DB)
        .with_container_name(CONTAINER_NAME)
        .with_reuse(ReuseDirective::Always)
        .with_cmd(["postgres", "-c", "max_connections=200"])
        .start()
        .await
        .unwrap_or_else(|error| {
            panic!(
                "a broken test environment, not a failing peer: could not start \
                 `{POSTGRES_IMAGE}:{POSTGRES_TAG}` ({error}). Is the Docker daemon running? \
                 Or set {URL_VARIABLE} to a server's URL."
            )
        });
    let host = container
        .get_host()
        .await
        .expect("a broken test environment: the container has no reachable host");
    let port = container
        .get_host_port_ipv4(POSTGRES_PORT.tcp())
        .await
        .expect("a broken test environment: the container mapped no host port");
    Server {
        _container: Some(container),
        url: format!("postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{host}:{port}/{POSTGRES_DB}"),
    }
}

/// A pool onto a fresh schema, `search_path` set on every connection, the
/// event log's migrations applied.
///
/// # Panics
///
/// On any failure to reach or prepare the server.
async fn fresh_pool(url: &str) -> PgPool {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    // Relaxed: the counter publishes no other memory; all it must give is a
    // distinct value per call.
    let schema = format!(
        "hsr_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    );
    // Owned by the hook, which runs once per connection the pool opens.
    let search_path = schema.clone();
    let pool = PgPoolOptions::new()
        .max_connections(4)
        .min_connections(0)
        .acquire_timeout(Duration::from_secs(5))
        .after_connect(move |connection, _meta| {
            let schema = search_path.clone();
            Box::pin(async move {
                connection
                    .execute(format!(r#"SET search_path TO "{schema}""#).as_str())
                    .await?;
                Ok(())
            })
        })
        .connect(url)
        .await
        .unwrap_or_else(|error| {
            panic!("a broken test environment: could not open a pool ({error})")
        });
    pool.execute(format!(r#"CREATE SCHEMA IF NOT EXISTS "{schema}""#).as_str())
        .await
        .unwrap_or_else(|error| {
            panic!("a broken test environment: could not create schema `{schema}` ({error})")
        });
    migration::apply(&pool).await.unwrap_or_else(|error| {
        panic!("a broken test environment: the event log's migrations failed ({error})")
    });
    pool
}

/// The Postgres sync peer over a fresh schema, and the store it serves.
async fn peer_and_store(
    url: &str,
) -> (
    StoreSyncPeer<PostgresEventStore, PostgresEventStore>,
    PostgresEventStore,
) {
    let pool = fresh_pool(url).await;
    // Refcounted handles onto the one pool.
    let peer = StoreSyncPeer::new(
        PostgresEventStore::new(pool.clone()),
        PostgresEventStore::new(pool.clone()),
    );
    (peer, PostgresEventStore::new(pool))
}

/// SQLite to Postgres and back, and Postgres's own append to SQLite, byte for
/// byte.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn sqlite_through_the_postgres_peer_is_byte_identical_both_ways() {
    let server = server().await;
    let (peer, far) = peer_and_store(&server.url).await;

    super::sqlite_round_trip(&peer, &far).await;
}

/// A SQLite batch pushed to Postgres twice changes nothing.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_sqlite_batch_replayed_to_the_postgres_peer_changes_nothing() {
    let server = server().await;
    let (peer, far) = peer_and_store(&server.url).await;

    super::replay_changes_nothing(&peer, &far).await;
}

/// The runner pushes a SQLite log to the Postgres peer in its write groups,
/// over several pushes, and each arrives whole with its guard (SY-30, #143).
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn the_runner_keeps_write_groups_through_the_postgres_peer() {
    let server = server().await;
    let pool = fresh_pool(&server.url).await;
    // Refcounted handles onto the one pool.
    let peer = StoreSyncPeer::with_limits(
        PostgresEventStore::new(pool.clone()),
        PostgresEventStore::new(pool),
        super::groups::small_batches(),
    );

    super::groups::runner_keeps_write_groups(&peer).await;
}
