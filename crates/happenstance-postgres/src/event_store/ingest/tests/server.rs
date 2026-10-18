//! The live server the ingest tests share, and a fresh schema per store.
//!
//! `tests/support/mod.rs` cannot be reached from here: each file under
//! `tests/` is a crate of its own, and this module's impls are `cfg(test)`,
//! which is invisible from there. So the bootstrap is restated, small, and its
//! constants must move with that file's: the same image and tag, the same
//! `max_connections` and the same container name, so a run attaches to the
//! container the conformance suite already reuses.
//!
//! One addition: `HAPPENSTANCE_POSTGRES_URL`, read first, so a machine with a
//! server and no Docker daemon can run these. It is read here, in test code,
//! and nowhere else.
//!
//! # Why the functions here panic
//!
//! A server that cannot be reached is a broken test environment, not an adapter
//! defect, and the message says so rather than letting a bare "connection
//! refused" stand in for a finding.

use std::env::VarError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, ReuseDirective};
use tokio::sync::OnceCell;

use crate::migration;

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

/// The one server, found or started on first use.
///
/// The container handle is kept alive in the cell, as `tests/support/mod.rs`
/// keeps it: dropping it would stop the container.
static SERVER: OnceCell<Server> = OnceCell::const_new();

/// A reachable server.
struct Server {
    /// Held, not used: `None` when the URL came from the environment.
    _container: Option<ContainerAsync<GenericImage>>,
    url: String,
}

/// The URL of the server every live test in this binary shares.
pub(super) async fn server_url() -> &'static str {
    &SERVER.get_or_init(find_or_start).await.url
}

/// The URL in [`URL_VARIABLE`] if it is set, else the shared container.
///
/// # Panics
///
/// If the variable is not Unicode, or the container cannot be started.
async fn find_or_start() -> Server {
    match std::env::var(URL_VARIABLE) {
        Ok(url) => Server {
            _container: None,
            url,
        },
        Err(VarError::NotPresent) => start_container().await,
        Err(VarError::NotUnicode(raw)) => {
            panic!("a broken test environment: {URL_VARIABLE} is not Unicode ({raw:?})")
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
                "a broken test environment, not a non-conformant adapter: could not start \
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

/// A pool onto a fresh schema, with `search_path` set on every connection and
/// migration 1 applied.
///
/// The schema is `hsi_<pid>_<n>`, a prefix neither of `tests/support`'s
/// fixtures uses. The pool is small and built on the calling test's runtime,
/// never in a `static`, which avoids the cross-runtime hazard
/// `PostgresEventStore::new` documents.
///
/// # Panics
///
/// On any failure to reach or prepare the server, naming the environment.
pub(super) async fn fresh_pool() -> PgPool {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let schema = format!(
        "hsi_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    );
    let url = server_url().await;

    // Owned by the hook, which runs once per connection the pool opens.
    let search_path = schema.clone();
    let pool = PgPoolOptions::new()
        // Six: the most any test holds at once is four, in the claim test
        // that holds a transaction and a lock beside a blocked ingest and
        // the connection that polls it.
        .max_connections(6)
        .min_connections(0)
        .idle_timeout(Duration::from_secs(5))
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
            panic!("a broken test environment: could not open a pool at {url} ({error})")
        });
    pool.execute(format!(r#"CREATE SCHEMA IF NOT EXISTS "{schema}""#).as_str())
        .await
        .unwrap_or_else(|error| {
            panic!("a broken test environment: could not create schema `{schema}` ({error})")
        });
    migration::apply(&pool).await.unwrap_or_else(|error| {
        panic!("a broken test environment: migration 1 failed in schema `{schema}` ({error})")
    });
    pool
}
