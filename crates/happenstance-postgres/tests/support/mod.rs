//! The live-Postgres fixture, and the container it talks to.
//!
//! # Why this is `support/mod.rs` and not `support.rs`
//!
//! Cargo compiles every file directly under `tests/` as its own test binary. A
//! `tests/support.rs` would therefore be built, linked and run as a target of
//! its own — one that contains no `#[test]` and so reports success having
//! executed nothing. `happenstance-sqlite` has a test asserting this out of the
//! filesystem for the same reason.
//!
//! # Three things are won or lost in this file and no conformance rule can see
//! any of them
//!
//! 1. **`connect()` must hand out a genuine second handle.** It returns a
//!    `PgPool` clone, which is an `Arc` internally and shares the connection
//!    pool — so two handles are two pooled connections onto one schema. Handing
//!    back a clone of one `PostgresEventStore` would pass
//!    `two_handles_observe_each_others_appends` perfectly while filling none of
//!    the handle-multiplicity axis.
//! 2. **`new()` must mint a genuinely isolated backing store.** It creates a
//!    schema per instance. Pointing every instance at `public` is exactly what
//!    `two_fixture_instances_observe_none_of_each_others_appends` exists to
//!    catch, and `MemoryFixture::sharing` is the in-tree implementation that
//!    deliberately fails it.
//! 3. **The ceilings must be stated.** Leaving them `None` produces a *green*
//!    suite in which `append_reports_exceeded_store_limits` reports a skip that
//!    is indistinguishable from a pass to anyone reading an exit code.
//!
//! # Why one container and a schema per instance
//!
//! The conformance suite constructs a fixture instance per rule — around ninety
//! of them for the event-store family alone. A container per instance would be
//! correct and would turn the live job into a timeout, so the container is
//! started **once** per test binary and each instance gets `CREATE SCHEMA`, a
//! pool whose `search_path` points at it, and migration 1 applied inside it.
//!
//! That interacts with the migration mechanism, and the interaction is why
//! `sqlx`'s `migrate` feature stayed off: `_sqlx_migrations` is per-schema, so a
//! migrator would write one row of bookkeeping per instance about a history that
//! is one migration long. The manifest records the decision.
//!
//! # Why `connect` panics
//!
//! Because [`Fixture::connect`] has no `Result`, and the trait's own docs say
//! why: a fixture that cannot connect is a broken **test environment**, not a
//! non-conformant adapter, and a `Result` would put "the database is down" into
//! the same channel as "the adapter is wrong". This is the first fixture in the
//! workspace where those two are genuinely different events, so every panic
//! below names the container and the schema rather than letting a bare
//! "connection refused" reach a CI log.

#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use happenstance_postgres::event_store::PostgresEventStore;
use happenstance_postgres::migration;
#[cfg(all(feature = "projection-store", feature = "conformance"))]
use happenstance_postgres::projection_store::PostgresProjectionStore;
use happenstance_postgres::sqlx::postgres::PgPoolOptions;
use happenstance_postgres::sqlx::{Executor, PgPool};
use happenstance_testkit::concurrency::CONTENDERS;
use happenstance_testkit::{Capability, Fixture};
use testcontainers::ReuseDirective;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use tokio::sync::OnceCell;

/// The server every fixture instance in this process shares.
///
/// Pinned to a **specific minor**, never a floating tag: an unpinned image makes
/// a red job unattributable to a change in this repository. `17.10` is the exact
/// server `experiments/position-visibility/` took its numbers on, so the adapter
/// and the measurement that chose its mechanism are checked against the same
/// build.
const POSTGRES_IMAGE: &str = "postgres";
const POSTGRES_TAG: &str = "17.10";

/// The port Postgres listens on inside the container. The host port is mapped
/// and is never this one — reading it back is what stops the suite colliding
/// with a developer's own server on 5432.
const POSTGRES_PORT: u16 = 5432;

/// The superuser the image creates, and the password we hand it.
///
/// Not a secret in any meaningful sense: it authenticates to a throwaway
/// container on a mapped ephemeral port that is destroyed with the test binary.
/// Written as constants rather than inline so the two places that need them
/// cannot drift.
/// The reused container's name. Fixed, so a second run finds the first's server
/// rather than starting another one.
const CONTAINER_NAME: &str = "happenstance-postgres-conformance";

const POSTGRES_USER: &str = "postgres";
const POSTGRES_PASSWORD: &str = "postgres";
const POSTGRES_DB: &str = "postgres";

/// The one container, started on first use and dropped when the process ends.
///
/// `OnceCell` rather than `OnceLock` because starting it is async. The
/// `ContainerAsync` handle is deliberately kept alive inside the cell: dropping
/// it stops the container, and a `static` that is never dropped is exactly the
/// lifetime this needs.
static SERVER: OnceCell<Server> = OnceCell::const_new();

/// A running Postgres, and the host port it answers on.
struct Server {
    /// Held, not used. Dropping this stops the container.
    _container: ContainerAsync<GenericImage>,
    host: String,
    port: u16,
}

impl Server {
    /// The connection URL for the container's default database.
    fn admin_url(&self) -> String {
        let Self { host, port, .. } = self;
        format!("postgres://{POSTGRES_USER}:{POSTGRES_PASSWORD}@{host}:{port}/{POSTGRES_DB}")
    }
}

/// Starts the shared container, or returns the one already running.
///
/// # Panics
///
/// If Docker is unreachable or the image cannot be pulled or started. That is an
/// environment failure and the message says so, because the alternative is a
/// conformance rule failing for a reason that has nothing to do with the
/// adapter.
async fn server() -> &'static Server {
    SERVER
        .get_or_init(|| async {
            let container = GenericImage::new(POSTGRES_IMAGE, POSTGRES_TAG)
                .with_exposed_port(POSTGRES_PORT.tcp())
                // The image logs this line on stderr once it is genuinely
                // accepting connections. Waiting on the port alone races: the
                // first `postgres` process binds, runs initdb, and restarts.
                .with_wait_for(WaitFor::message_on_stderr(
                    "database system is ready to accept connections",
                ))
                .with_env_var("POSTGRES_PASSWORD", POSTGRES_PASSWORD)
                .with_env_var("POSTGRES_USER", POSTGRES_USER)
                .with_env_var("POSTGRES_DB", POSTGRES_DB)
                // Reused across runs, under a fixed name, and this is a bug fix
                // rather than an optimisation.
                //
                // The `ContainerAsync` handle lives in the `static` above, and
                // Rust does not drop statics at process exit -- so nothing ever
                // stopped the container, and every invocation of this test
                // binary left another Postgres running. Twenty-seven had
                // accumulated before anything noticed, and what noticed was the
                // machine running out of memory in the middle of a suite run,
                // not the gate. A leak whose only symptom is that the NEXT
                // measurement is unreliable is the worst shape a test-harness
                // bug can take.
                //
                // The crate's `watchdog` feature is the intended fix and does
                // not build on Windows (`SIGQUIT` is Unix-only), which this
                // workspace treats as first-class. Reuse bounds the leak at one
                // container instead: the first run starts it, every later run
                // attaches to it. Safe because isolation here is per-SCHEMA and
                // schema names carry the process id, so a reused server holds
                // one schema per fixture instance per run and no two collide.
                .with_container_name(CONTAINER_NAME)
                .with_reuse(ReuseDirective::Always)
                // The experiment that chose this adapter's mechanism ran
                // with `max_connections=200` and nothing else off-default
                // (`experiments/position-visibility/README.md` section 1).
                // Matching it is not cosmetic: the event-store family builds
                // around ninety fixture instances, each with a pool of
                // `CONTENDERS + 4`, and the stock 100 is comfortably inside
                // what a run can ask for while pools are still draining.
                .with_cmd(["postgres", "-c", "max_connections=200"])
                .start()
                .await
                .unwrap_or_else(|error| {
                    panic!(
                        "a broken test environment, not a non-conformant adapter: could not \
                         start `{POSTGRES_IMAGE}:{POSTGRES_TAG}` ({error}). Is the Docker \
                         daemon running? These tests are `#[ignore]`d precisely so that a \
                         machine without one still has a green `cargo xtask ci`; they are \
                         reached with `-- --ignored`."
                    )
                });

            let host = container
                .get_host()
                .await
                .expect("a broken test environment: the container has no reachable host")
                .to_string();
            let port = container
                .get_host_port_ipv4(POSTGRES_PORT.tcp())
                .await
                .expect("a broken test environment: the container mapped no host port");

            Server {
                _container: container,
                host,
                port,
            }
        })
        .await
}

/// One isolated Postgres schema, and handles onto it.
///
/// One instance is one backing store, which is CLAUDE.md's fixture rule and
/// `two_fixture_instances_observe_none_of_each_others_appends`'s subject.
#[derive(Debug)]
pub(crate) struct PostgresFixture {
    /// The schema this instance owns. Unique per instance per process.
    schema: String,
    /// Whether `connect` hands out the naive arm rather than the shipped one.
    ///
    /// Always `false` except in the negative control, and the field exists at
    /// all only under that feature.
    #[cfg(feature = "naive-arm")]
    naive: bool,
    /// This instance's one pool, built on first use.
    ///
    /// **One pool per instance, not one per `connect()`**, and the first
    /// implementation got this wrong in a way worth recording: it built a fresh
    /// pool on every `connect()`, so `CONTENDERS` handles meant `CONTENDERS`
    /// pools of `CONTENDERS + 4` connections each — up to 96 sessions against a
    /// server whose default `max_connections` is 100. The symptom was not a
    /// clear refusal but `pool timed out while waiting for an open connection`
    /// after four minutes, which is EC-002's failure mode arriving from the
    /// opposite direction: not a pool too small for its handles, but too many
    /// pools for one server.
    ///
    /// Sharing one pool is also the *right* answer rather than merely the
    /// affordable one. A `PgPool` hands out a different pooled connection per
    /// statement, so two stores sharing a pool still execute on different
    /// backend sessions — which is exactly the per-session hazard
    /// `SECOND_HANDLE` exists to expose (a cached `max(position)`, a
    /// per-connection snapshot, an advisory lock scoped to one pool member).
    pool: OnceCell<PgPool>,
}

impl PostgresFixture {
    /// Mints a fresh, empty schema name for this instance.
    ///
    /// Nothing reaches the server here: `new()` is synchronous because the macro
    /// calls it in an expression position, and every fixture instance would
    /// otherwise pay a round trip whether or not its rule ever connects. The
    /// schema is created on the first [`connect`](Fixture::connect).
    ///
    /// The name carries the process id as well as an ordinal because two test
    /// binaries — `postgres_conformance` and any sibling — share one container
    /// only if they share a process, which they do not; but they *may* share a
    /// developer's manually started server, and a collision there would make one
    /// binary's rows visible to the other's isolation rule.
    pub(crate) fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        Self {
            schema: format!("hs_{}_{ordinal}", std::process::id()),
            pool: OnceCell::new(),
            #[cfg(feature = "naive-arm")]
            naive: false,
        }
    }

    /// A fixture whose handles have the visibility mechanism removed.
    ///
    /// Everything else is identical -- same container, same schema-per-instance
    /// isolation, same migration, same append path. Only `head` and `read` lose
    /// the frontier predicate, which is the single difference the control is
    /// measuring. See `PostgresEventStore::new_naive`.
    #[cfg(feature = "naive-arm")]
    pub(crate) fn naive() -> Self {
        Self {
            naive: true,
            ..Self::new()
        }
    }

    /// This instance's pool, for a test that needs to drive raw SQL against the
    /// same schema its handles use.
    ///
    /// Exposed only to the in-crate test targets. The conformance rules never
    /// need it; the negative-control probe does, because it has to control
    /// transaction ordering that no port method exposes.
    pub(crate) async fn pool_for_test(&self) -> PgPool {
        self.pool().await
    }

    /// The schema this instance owns, for tests that need to address it directly.
    pub(crate) fn schema(&self) -> &str {
        &self.schema
    }

    /// A pool whose every connection resolves unqualified names in this
    /// instance's schema.
    ///
    /// # Panics
    ///
    /// On any failure to reach or prepare the server. See the module docs.
    async fn pool(&self) -> PgPool {
        self.pool.get_or_init(|| self.build_pool()).await.clone()
    }

    /// Builds this instance's pool and prepares its schema. Called once.
    async fn build_pool(&self) -> PgPool {
        let pool = open_pool(&self.schema).await;
        self.prepare(&pool).await;
        pool
    }

    /// Applies migration 1 inside this instance's schema.
    ///
    /// The schema itself is created by [`open_pool`]; what is left here is the
    /// one thing the two fixtures in this module do differently.
    ///
    /// Idempotent in both halves, because `connect()` may be called many times
    /// on one instance and the second call must not fail. EC-004's requirement
    /// is that this fails at fixture construction with a message naming the
    /// environment rather than part way through a conformance rule.
    async fn prepare(&self, pool: &PgPool) {
        migration::apply(pool).await.unwrap_or_else(|error| {
            panic!(
                "a broken test environment: migration 1 failed in schema `{}` ({error}). \
                 A conformance rule that fails because the table is missing is an \
                 environment failure wearing an adapter defect's clothes.",
                self.schema
            )
        });
    }
}

/// Opens a pool whose every connection resolves unqualified names in `schema`,
/// creating the schema first.
///
/// Free rather than a method, because two fixtures in this module need the same
/// pool and the same isolation and differ only in **which migration** they then
/// apply. Extracting it is what stops the second fixture from being a copy of
/// the first with two lines changed — and a copy is how one of them would later
/// acquire a pool size the other does not have.
///
/// # Panics
///
/// On any failure to reach or prepare the server. See the module docs: an
/// environment failure must say so rather than surface as a conformance rule
/// failing for a reason that has nothing to do with the adapter.
async fn open_pool(schema_name: &str) -> PgPool {
    let server = server().await;
    let schema = schema_name.to_owned();

    // Sized from `CONTENDERS` rather than from the literal 8. The concurrency
    // family starts that many contenders, and a pool smaller than the number
    // of simultaneous handles **deadlocks rather than failing** — CF-33 says
    // there is no watchdog to tell the two apart, and the symptom is a CI
    // timeout, which is the least diagnosable failure this fixture could
    // ship. The headroom covers `SECOND_HANDLE`'s extra handle and any
    // bookkeeping connection a rule opens beside its contenders.
    let size = u32::try_from(CONTENDERS).expect("CONTENDERS fits in a u32") + 4;

    let pool = PgPoolOptions::new()
        .max_connections(size)
        // Nothing is held open just in case. Around ninety fixture
        // instances are constructed over one run of the event-store family,
        // and a pool that keeps a floor of idle connections turns that into
        // a server-side connection leak that only shows up near the end.
        .min_connections(0)
        .idle_timeout(Duration::from_secs(5))
        // Finite and short. The default is thirty seconds, which turns the
        // undersized-pool failure into four minutes of nothing and reads as
        // a hang — the least diagnosable failure this fixture could ship
        // (CF-33: there is no watchdog). Five seconds is far longer than a
        // healthy checkout on loopback and short enough that the message
        // below arrives while anyone is still watching.
        .acquire_timeout(Duration::from_secs(5))
        // Every connection in this pool, including ones created later to
        // meet demand, must land in this instance's schema — so the
        // `search_path` is set per connection rather than once after
        // construction. A pool that sets it on the first connection only is
        // a pool whose isolation quietly depends on how many rules ran.
        .after_connect(move |connection, _meta| {
            let schema = schema.clone();
            Box::pin(async move {
                connection
                    .execute(format!(r#"SET search_path TO "{schema}""#).as_str())
                    .await?;
                Ok(())
            })
        })
        .connect(&server.admin_url())
        .await
        .unwrap_or_else(|error| {
            panic!(
                "a broken test environment, not a non-conformant adapter: could not open \
                     a pool onto schema `{}` at {} ({error})",
                schema_name,
                server.admin_url(),
            )
        });

    // `CREATE SCHEMA` is not run through the pool's own `after_connect` path —
    // that sets a `search_path` pointing at a schema that does not exist yet,
    // which Postgres tolerates — so the statement below names the schema
    // explicitly rather than relying on it.
    pool.execute(format!(r#"CREATE SCHEMA IF NOT EXISTS "{schema_name}""#).as_str())
        .await
        .unwrap_or_else(|error| {
            panic!("a broken test environment: could not create schema `{schema_name}` ({error})")
        });

    pool
}

impl Fixture for PostgresFixture {
    type Store = PostgresEventStore;

    /// Postgres answers this with a second pooled connection onto the same
    /// schema. It is the only MUST among the capabilities, and declining it
    /// makes `two_handles_observe_each_others_appends` **panic** quoting the
    /// fixture's own words rather than skip.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, and answered on the merits rather than conveniently.
    ///
    /// The schema outlives every pool opened onto it — it is dropped when the
    /// container stops, which is when the process ends — so closing all handles
    /// and opening a new one observes exactly what was committed. That is
    /// precisely what `REOPEN` means here: it is the weaker of the two things
    /// "restart" could mean, and the trait says so.
    const REOPEN: Capability = Capability::SUPPORTED;

    /// **Declined**, and this is the one capability this adapter exists to be
    /// unable to offer.
    ///
    /// The crate root says it in its own words: *"Read-your-own-writes does not
    /// hold, and is not claimed."* ES-10 is bought with `xid8` +
    /// `pg_snapshot_xmin`, so a row becomes visible when its writing
    /// transaction id falls below the frontier beneath which nothing can still
    /// be in flight — and that frontier belongs to the **server**, not to this
    /// store. An unrelated open transaction in an unrelated database holds it
    /// back, measured at 0.688 ms unloaded and 4010.719 ms behind a five-second
    /// write elsewhere.
    ///
    /// Measured consequence, which is why this override exists: the model
    /// family passes on a quiet machine and fails on a loaded one, on the
    /// **same** deterministic op sequence. All 256 cases pass against a local
    /// container; the same commit fails on a CI runner at op 0 of case 1 with
    /// *"the log holds 0 event(s); 3 were accepted"*. That is a flake, and
    /// `[FROZEN]` CF-33 names a flaky conformance rule as worse than none.
    const READ_YOUR_OWN_WRITES: Capability = Capability::declined(
        "reads and `head` admit only rows whose `xact_id` is below \
         `pg_snapshot_xmin(pg_current_snapshot())`, so an append this caller \
         was just told succeeded is not visible to it until every transaction \
         open anywhere on the server when it committed has ended. `append` \
         returning `Ok(P)` is a durability promise, not a visibility one.",
    );

    /// Supported, and armed for real.
    ///
    /// It was declined while `append` was `todo!()` — a fault armed against a
    /// body that did not exist would have made
    /// `append_is_atomic_under_a_mid_batch_fault` report on nothing. `append`
    /// exists now, and taking the trait's silent default on a store that can
    /// genuinely co-operate is the one outcome the architecture brief singles
    /// out as wrong.
    ///
    /// The injection is an `AFTER INSERT` trigger that counts rows within the
    /// statement and raises on the `after + 1`-th. It works precisely because
    /// this adapter inserts a batch as **one** multi-row statement: the raise
    /// aborts that statement, which aborts the transaction, which is the atomic
    /// unit `append` promises.
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    /// **Supported, and armed for real.** This fixture is the one that owed it.
    ///
    /// The capability's default declension says the injection "has to come from
    /// the adapter and this one has none to offer", and that was false here:
    /// `PgReadStream` opens a `REPEATABLE READ READ ONLY` transaction, `DECLARE`s
    /// a server-side cursor and issues a `FETCH` per chunk, so it is exactly the
    /// paged adapter the capability was invented for. `happenstance-sqlite`
    /// declines saying *"the paged adapters are where this capability has
    /// something to inject"*; this is a paged adapter, and CF-18's
    /// declension-by-inheritance check is what surfaced the gap.
    ///
    /// See [`PostgresFixture::arm_read_fault`] for what is injected and why the
    /// two obvious injections are not.
    const READ_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the adapter's own constants, never restated as literals.
    ///
    /// A number restated here is a number that drifts from the one `append`
    /// enforces, and the rule checks the fixture's copy — so the drift would
    /// show up as a conformance failure blaming the adapter.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(PostgresEventStore::MAX_EVENT_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(PostgresEventStore::MAX_TAGS_PER_EVENT);
    const MAX_EVENTS_PER_BATCH: Option<usize> = Some(PostgresEventStore::MAX_EVENTS_PER_BATCH);

    async fn connect(&self) -> Self::Store {
        // `prepare` runs inside the pool's one-time initialisation rather
        // than here: it is idempotent, but running it per `connect()` spends
        // two round trips on every handle to re-establish a schema that
        // cannot have gone away.
        let pool = self.pool().await;
        #[cfg(feature = "naive-arm")]
        if self.naive {
            return PostgresEventStore::new_naive(pool);
        }
        PostgresEventStore::new(pool)
    }

    async fn arm_mid_batch_fault(&self, after: usize) {
        let pool = self.pool().await;
        let threshold = i64::try_from(after).unwrap_or(i64::MAX) + 1;
        // A per-statement counter, reset by the trigger's own creation, so the
        // fault fires once for the next batch rather than for every batch.
        let sql = format!(
            r"
            CREATE OR REPLACE FUNCTION mid_batch_fault() RETURNS trigger AS $fn$
            DECLARE seen bigint;
            BEGIN
                seen := coalesce(nullif(current_setting('hs.fault_seen', true), ''), '0')::bigint + 1;
                PERFORM set_config('hs.fault_seen', seen::text, false);
                IF seen = {threshold} THEN
                    RAISE EXCEPTION 'the fixture armed a mid-batch fault';
                END IF;
                RETURN NEW;
            END $fn$ LANGUAGE plpgsql;

            CREATE TRIGGER mid_batch_fault_trigger
                AFTER INSERT ON event
                FOR EACH ROW EXECUTE FUNCTION mid_batch_fault();
            "
        );
        pool.execute(sql.as_str())
            .await
            .expect("a broken test environment: could not arm the mid-batch fault");
    }

    /// Arms the next `read` to fail while the cursor is producing rows.
    ///
    /// # What is injected
    ///
    /// The `event` relation is renamed aside and replaced by a view over it whose
    /// `WHERE` calls a `plpgsql` function that raises above a threshold. The
    /// reader's `DECLARE … CURSOR FOR SELECT … FROM event` then plans and opens
    /// normally, and the raise arrives **while the `FETCH` is producing rows** —
    /// which is the failure this capability is about. A `read` that failed at
    /// name resolution would be a different fault wearing the same name.
    ///
    /// The threshold is 2 and the rule appends 4, so the fault is genuinely
    /// part-way rather than at the first row. `FETCH_CHUNK` is 1024, so all four
    /// rows are asked for in one `FETCH`; the raise is what that `FETCH` answers
    /// with instead of a page.
    ///
    /// # Why not the two injections this fixture's declension used to name
    ///
    /// It said `pg_terminate_backend` against the reader's own backend between
    /// two `FETCH`es, or closing the cursor beneath it. Neither survives contact:
    ///
    /// * **Arming happens before the read starts.** The trait requires it —
    ///   *"a rule connects before it arms"* — so at this point there is no reader
    ///   backend in flight to terminate and no cursor to close. Both injections
    ///   presuppose a read already running, which is a fixture API this port does
    ///   not have and CF-39 says it should not acquire: `arm_read_fault` takes no
    ///   index precisely because a read's granularity belongs to the adapter.
    /// * **A pool absorbs the first one anyway.** Terminating an *idle* pooled
    ///   backend is invisible: `sqlx` tests a connection before handing it out and
    ///   opens a fresh one. That is CF-39's named hazard — *"a connection killed
    ///   mid-statement behind a reconnect-and-retry pool"* — arriving one step
    ///   earlier, and a fixture whose arming is silently absorbed passes this rule
    ///   vacuously, which is the outcome CF-39 forbids.
    /// * **A cursor is session-local.** `pg_cursors` shows only the current
    ///   session's, so no second connection can close another's.
    ///
    /// The rename is reversible and destroys nothing: the rows stay in
    /// `event_read_fault_source`, and the view reads them. It fires for the life
    /// of this fixture instance, which is honest rather than convenient — the
    /// rule reads exactly once after arming, and every rule gets its own fixture,
    /// so "fires once" and "fires from now on" are the same run.
    ///
    /// # Panics
    ///
    /// On any failure to reach or prepare the server. See the module docs.
    async fn arm_read_fault(&self) {
        let pool = self.pool().await;
        pool.execute(READ_FAULT_INJECTION)
            .await
            .unwrap_or_else(|error| {
                panic!("a broken test environment, not a non-conformant adapter: {error}")
            });
    }

    async fn reopen(&self) {
        // Every handle this fixture handed out owns its own `PgPool`, and this
        // fixture keeps none of them — so there is nothing here to close, and
        // the rows are in the schema either way. What `reopen` must *not* do is
        // drop and recreate the schema: that would make
        // `acknowledged_writes_survive_a_reopen` pass by making it vacuous,
        // which is the trap `happenstance-sqlite`'s fixture documents at the
        // equivalent line.
        //
        // Postgres holds no process-level state on the client side that a new
        // pool would not discard by construction, so the honest implementation
        // of "discard handle state" is to do nothing and let the next
        // `connect()` open a genuinely new pool.
    }
}

/// The read-fault injection, as SQL.
///
/// A constant so that the dollar-quoted `plpgsql` bodies sit on their own rather
/// than inside a method: they are the pieces of this file that are not Rust, and
/// burying them in an `unwrap_or_else` chain is how they stop being readable as
/// SQL.
///
/// Wrapped in a `DO` block guarded on `to_regclass`, so that arming twice on one
/// fixture instance is a no-op rather than an error: the rule arms once, and a
/// fixture whose arming is not idempotent fails on the second call for a reason
/// that has nothing to do with the adapter.
const READ_FAULT_INJECTION: &str = concat!(
    "CREATE OR REPLACE FUNCTION read_fault_guard(p bigint) RETURNS boolean AS ",
    "$guard$ BEGIN ",
    "IF p > 2 THEN RAISE EXCEPTION 'the fixture armed a read fault'; END IF; ",
    "RETURN true; ",
    "END; $guard$ LANGUAGE plpgsql;\n",
    "DO $arm$ BEGIN ",
    "IF to_regclass('event_read_fault_source') IS NULL THEN ",
    "ALTER TABLE event RENAME TO event_read_fault_source; ",
    "CREATE VIEW event AS SELECT * FROM event_read_fault_source ",
    "WHERE read_fault_guard(position); ",
    "END IF; ",
    "END $arm$;",
);

// -------------------------------------------------------------------------
// The projection fixture
// -------------------------------------------------------------------------

/// One isolated Postgres schema, and projection-store handles onto it.
///
/// Beside [`PostgresFixture`] rather than in `tests/projection.rs`, because both
/// need the same container, the same schema-per-instance isolation and the same
/// pool sizing — and a second copy of that is how the two would later disagree
/// about `max_connections`. What differs is the migration each applies, which is
/// the one thing [`open_pool`] deliberately does not do.
///
/// One instance is one backing store, which is CLAUDE.md's fixture rule and the
/// premise `commit_rejects_a_foreign_batch` rests on: it opens the fixture
/// **twice** and expects the two to be strangers.
#[cfg(all(feature = "projection-store", feature = "conformance"))]
#[derive(Debug)]
pub(crate) struct PostgresProjectionFixture {
    /// The schema this instance owns. Unique per instance per process.
    schema: String,
    /// This instance's one pool, built on first use. See [`PostgresFixture`]'s
    /// field of the same name for why it is one pool and not one per `connect`.
    pool: OnceCell<PgPool>,
}

#[cfg(all(feature = "projection-store", feature = "conformance"))]
impl PostgresProjectionFixture {
    /// Mints a fresh, empty schema name for this instance.
    ///
    /// Nothing reaches the server here, for the reason [`PostgresFixture::new`]
    /// gives: the macro calls this in expression position.
    ///
    /// The prefix is `hsp_` rather than `hs_` so that the two fixtures cannot
    /// collide on a developer's manually started server. They mint from separate
    /// counters, so `hs_1234_1` would otherwise name two different schemas in one
    /// process — and the symptom would be one fixture's rows appearing in the
    /// other's isolation rule, which is the failure the process id is already
    /// there to prevent between binaries.
    pub(crate) fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        Self {
            schema: format!("hsp_{}_{ordinal}", std::process::id()),
            pool: OnceCell::new(),
        }
    }

    /// This instance's pool, for a test that needs to drive raw SQL against the
    /// same schema its handles use.
    pub(crate) async fn pool_for_test(&self) -> PgPool {
        self.pool().await
    }

    /// The schema this instance owns.
    pub(crate) fn schema(&self) -> &str {
        &self.schema
    }

    async fn pool(&self) -> PgPool {
        self.pool.get_or_init(|| self.build_pool()).await.clone()
    }

    /// Builds this instance's pool and applies the projection schema.
    ///
    /// Migration 2 **and** the probe table, in that order. The probe table is the
    /// suite's own read model and is deliberately not a file in `migrations/`, so
    /// the fixture is the thing that has to create it — which is also the check
    /// that `PROBE_TABLE` stays applicable SQL rather than drifting into prose.
    async fn build_pool(&self) -> PgPool {
        let pool = open_pool(&self.schema).await;

        migration::apply_projection(&pool)
            .await
            .unwrap_or_else(|error| {
                panic!(
                    "a broken test environment: migration 2 failed in schema `{}` ({error}). \
                     A conformance rule that fails because `projection_checkpoint` is \
                     missing is an environment failure wearing an adapter defect's clothes.",
                    self.schema
                )
            });

        pool.execute(happenstance_postgres::projection_store::PROBE_TABLE)
            .await
            .unwrap_or_else(|error| {
                panic!(
                    "a broken test environment: the probe table failed in schema `{}` ({error})",
                    self.schema
                )
            });

        pool
    }
}

#[cfg(all(feature = "projection-store", feature = "conformance"))]
impl happenstance_testkit::ProjectionFixture for PostgresProjectionFixture {
    type Store = PostgresProjectionStore;

    /// A MUST, and this adapter meets it for real. A second `connect` is a second
    /// store over the same pool, and a `PgPool` hands out a different pooled
    /// connection per statement — so two handles execute on **different backend
    /// sessions**, which is exactly the per-session hazard this capability exists
    /// to expose. It is not a refcount clone: the two stores carry distinct
    /// stamps, so they are strangers to each other's batches and neighbours in
    /// the same schema, which is the pair of properties the rules need.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Declined, and the reason is the store's rather than the fixture's
    /// convenience: `PostgresProjectionStore` holds **no protection policy**.
    ///
    /// PS-18 makes refusal a mechanism the port supplies and leaves *what to
    /// protect* to the domain. This adapter owns the checkpoint row and the
    /// transaction that carries it, and knows nothing about which projection a
    /// regulator already holds a hash chain for, so `reset` returns
    /// `ResetError::Refused` on no path at all and a fixture claiming the
    /// capability would fail `refused_reset_changes_nothing` at its first
    /// assertion.
    ///
    /// The same alternative loses here as it does for `happenstance-sqlite`, and
    /// for the same reason: a protected-projections table invented so that one
    /// more rule reports `Ran` would put a domain policy inside an adapter
    /// designed to keep the domain out.
    const RESET_REFUSAL: Capability = Capability::declined(
        "PostgresProjectionStore holds no protection policy: the read model \
         belongs to the caller and this adapter owns only the checkpoint row and \
         the transaction that carries it, so there is no projection it could \
         decline to reset and `reset` returns `Refused` on no path at all. The \
         alternative — a protected-projections table invented here so that one \
         more rule reports `Ran` — would put a domain policy into an adapter \
         designed to keep the domain out",
    );

    /// Supported, and armed for real: a `BEFORE INSERT OR UPDATE` trigger on
    /// `projection_checkpoint` raising an exception aborts the **checkpoint** half
    /// of a commit whose read-model half has already run in the same transaction.
    ///
    /// That is the only way PS-1's second conjunct is observable at all — nothing
    /// a caller holds can make a conformant `commit` fail — and it is the same
    /// mechanism `PostgresFixture::arm_mid_batch_fault` uses one layer down.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    /// One more store over this fixture's pool.
    ///
    /// A distinct instance rather than a clone, so it mints its own stamp: a
    /// clone is the same store and would accept the other's batches, which would
    /// make `commit_rejects_a_foreign_batch` pass for the wrong reason if the
    /// suite ever narrowed to one fixture instance.
    async fn connect(&self) -> Self::Store {
        PostgresProjectionStore::new(self.pool().await)
    }

    /// Arms a trigger that aborts the checkpoint half of the next commit.
    ///
    /// Two objects rather than one, which is the whole difference from SQLite's
    /// inline `RAISE(ABORT, …)`: Postgres has no inline trigger body, so this is a
    /// `plpgsql` function and then a trigger that calls it. Both are created in
    /// this instance's schema, because the pool's `search_path` points there.
    ///
    /// `CREATE OR REPLACE` for the function and `DROP … IF EXISTS` before the
    /// trigger, because Postgres has no `CREATE TRIGGER IF NOT EXISTS` and a
    /// fixture whose arming is not idempotent fails on its second call rather
    /// than arming twice.
    ///
    /// `BEFORE INSERT OR UPDATE` covers both arms because the checkpoint write is
    /// an upsert, so which one Postgres reaches depends on whether the projection
    /// has committed before.
    ///
    /// It stays armed for the life of this fixture instance, which is honest
    /// rather than convenient: the rule that arms it commits exactly once
    /// afterwards, and every rule gets its own fixture, so "fires once" and
    /// "fires from now on" are the same run.
    async fn arm_commit_fault(&self) {
        let pool = self.pool().await;
        pool.execute(COMMIT_FAULT_TRIGGER)
            .await
            .unwrap_or_else(|error| {
                panic!("a broken test environment, not a non-conformant adapter: {error}")
            });
    }
}

/// The commit-fault injection, as SQL.
///
/// A constant so that the dollar-quoted `plpgsql` body sits on its own rather
/// than inside a method: the body is the one piece of this file that is not Rust,
/// and burying it in an `unwrap_or_else` chain is how it stops being readable as
/// SQL.
#[cfg(all(feature = "projection-store", feature = "conformance"))]
const COMMIT_FAULT_TRIGGER: &str = concat!(
    "CREATE OR REPLACE FUNCTION projection_commit_fault() RETURNS trigger AS ",
    "$fault$ BEGIN RAISE EXCEPTION 'the fixture armed a commit fault'; END; $fault$ ",
    "LANGUAGE plpgsql;\n",
    "DROP TRIGGER IF EXISTS projection_commit_fault_trigger ON projection_checkpoint;\n",
    "CREATE TRIGGER projection_commit_fault_trigger ",
    "BEFORE INSERT OR UPDATE ON projection_checkpoint ",
    "FOR EACH ROW EXECUTE FUNCTION projection_commit_fault();",
);
