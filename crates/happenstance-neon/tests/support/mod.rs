//! The live-Neon fixtures, and the schema-per-instance isolation they rest on.
//!
//! # Why this is `support/mod.rs` and not `support.rs`
//!
//! Cargo compiles every file directly under `tests/` as its own test binary. A
//! `tests/support.rs` would therefore be built, linked and run as a target of its
//! own — one that contains no `#[test]` and so reports success having executed
//! nothing.
//!
//! # Isolation is one schema per instance, and there was no second option
//!
//! `happenstance-postgres` gets one schema per fixture instance and then points a
//! pool's `search_path` at it, so its statements stay unqualified. Neither half
//! is available here. There is no session to hold a `SET` in, and the `/sql`
//! proxy **discards** `options=-c search_path=…` from the connection string —
//! measured, `SHOW search_path` still answers `"$user", public` with the option
//! present. A `SET LOCAL` prepended to each request would work and would make
//! every read a two-statement batch, destroying the one-statement-per-read
//! property ES-11 rests on here.
//!
//! So isolation is bought entirely by [`NeonConfig::with_schema`] and the
//! adapter qualifying every identifier it emits. `no_statement_leaves_a_name_
//! unqualified` in `src/event_store.rs` is the test that keeps it true, because
//! the failure is invisible until two fixture instances run at once.
//!
//! # Schemas are swept, not leaked
//!
//! A Neon branch is not a container that gets thrown away, and one run of the
//! event-store family creates around ninety schemas. Each name carries the second
//! it was minted in, and the first fixture in a process drops every
//! fixture-shaped schema older than [`SWEEP_AFTER_SECONDS`]. An age threshold rather than
//! "everything that is not mine": two test binaries run concurrently under one
//! `cargo test`, with different process ids, and a sweep keyed on the pid would
//! have each of them delete the other's schemas mid-run.

#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use happenstance_neon::event_store::NeonEventStore;
use happenstance_neon::{NeonConfig, SqlRequest, SqlStatement, SqlTransport, migration};
use happenstance_testkit::{Capability, Fixture};
use tokio::sync::OnceCell;

pub(crate) mod transport;

use transport::HyperTransport;

/// How old a fixture schema must be before the sweep drops it, in seconds.
///
/// One hour is far longer than any run of this suite and far shorter than the
/// interval between them. It is a threshold rather than a pid check for the
/// reason the module docs give: two binaries run concurrently.
///
/// Seconds rather than a `Duration` because the comparison is against the
/// second a schema name carries, and a `Duration` would be converted straight
/// back.
const SWEEP_AFTER_SECONDS: u64 = 60 * 60;

/// The sweep, run once per process before the first migration.
static SWEPT: OnceCell<()> = OnceCell::const_new();

/// A live event store over one isolated schema.
///
/// One instance is one backing store, which is CLAUDE.md's fixture rule and
/// `two_fixture_instances_observe_none_of_each_others_appends`'s subject.
#[derive(Debug)]
pub(crate) struct NeonFixture {
    config: NeonConfig,
    migrated: OnceCell<()>,
}

impl NeonFixture {
    /// Mints a fresh, empty schema name for this instance.
    ///
    /// Nothing reaches the endpoint here: `new()` is synchronous because the
    /// macro calls it in expression position, and every fixture instance would
    /// otherwise pay a round trip whether or not its rule ever connects. The
    /// schema is created by migration 1 on the first [`connect`](Fixture::connect).
    pub(crate) fn new() -> Self {
        Self {
            config: NeonConfig::default().with_schema(mint_schema("hs")),
            migrated: OnceCell::const_new(),
        }
    }

    /// The schema this instance owns, for a test that addresses it directly.
    pub(crate) fn schema(&self) -> &str {
        &self.config.schema
    }

    /// This instance's configuration, for a test that needs to build its own
    /// statements against the same tables.
    pub(crate) fn config(&self) -> &NeonConfig {
        &self.config
    }

    /// Applies migration 1 into this instance's schema. Idempotent, and run once.
    async fn migrate(&self) {
        self.migrated
            .get_or_init(|| async {
                sweep_once().await;
                execute(
                    migration::request(migration::MIGRATION_1, &self.config),
                    "migration 1",
                    self.schema(),
                )
                .await;
            })
            .await;
    }
}

impl Fixture for NeonFixture {
    type Store = NeonEventStore<HyperTransport>;

    /// A second handle is a second store over the same schema, and the endpoint
    /// makes that trivially honest: there is no connection to share, so two
    /// handles are two independent request streams onto one set of tables. It is
    /// the only MUST among the capabilities.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Supported, and answered on the merits rather than conveniently.
    ///
    /// This adapter holds **no** process-level state at all — no pool, no
    /// connection, no cache — so "discard every handle's state and open a new
    /// one" is already what happens between any two requests. The schema outlives
    /// every handle opened onto it, so a fresh `connect()` observes exactly what
    /// was committed. What [`reopen`](Fixture::reopen) must not do is drop and
    /// recreate the schema, which would make `acknowledged_writes_survive_a_reopen`
    /// pass by making it vacuous.
    const REOPEN: Capability = Capability::SUPPORTED;

    /// Supported, and armed for real.
    ///
    /// An `AFTER INSERT` trigger counts rows within the statement and raises on
    /// the `after + 1`-th. It works precisely because this adapter inserts a batch
    /// as **one** multi-row statement: the raise aborts that statement, which
    /// aborts the endpoint's implicit transaction, which is the atomic unit
    /// `append` promises.
    ///
    /// The counter is `set_config(…, true)` — **transaction-local**, and that is
    /// the one line where copying `happenstance-postgres`'s trigger verbatim
    /// would have been wrong. Its `false` is session-level, which is safe behind a
    /// pool that hands a session to one test at a time; the `/sql` proxy pools
    /// backends across unrelated requests, so a session GUC set inside one
    /// request can persist onto a backend a later request reuses and arm or
    /// mis-arm an append in a rule running in parallel. Transaction-local is also
    /// strictly more correct: the counter resets per round trip, which is exactly
    /// the "fires once" semantics `arm_mid_batch_fault` states. Verified against
    /// the live endpoint — a four-row batch aborts and leaves zero rows, and a
    /// two-row batch immediately afterwards commits.
    const MID_BATCH_FAULT: Capability = Capability::SUPPORTED;

    /// Supported, and armed for real.
    ///
    /// The `event` relation is renamed aside and replaced by a view over it whose
    /// `WHERE` calls a function that raises above a threshold, so the read plans
    /// and opens normally and the raise arrives while the server is producing
    /// rows. The stream then yields an `Err` **item** on its first poll rather
    /// than ending, which is the one thing
    /// `arming_a_read_fault_makes_the_stream_yield_an_error` requires.
    ///
    /// # Where inside the read it fires, and the divergence worth stating
    ///
    /// This adapter has exactly one round trip per read and no cursor, so "part
    /// way through" is not a place that exists here: the whole result set either
    /// arrives or does not. The capability's own words allow that — "where inside
    /// the read it fires is the adapter's business — first poll, page boundary,
    /// mid-page" — and declining would be the dishonest answer, because the store
    /// demonstrably does **not** absorb the fault.
    ///
    /// The contract says the fault "fires once". The rename fires for the life of
    /// this fixture instance, which is one rule, so the two coincide on every run
    /// the suite makes. Saying so here rather than letting the next reader
    /// discover it is the whole point of writing it down: the divergence is real
    /// and it is unobservable, and those are different facts.
    const READ_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the adapter's own constants, never restated as literals. A
    /// number restated here is a number that drifts from the one `append`
    /// enforces, and the rule checks the fixture's copy — so the drift would show
    /// up as a conformance failure blaming the adapter.
    const MAX_EVENT_DATA_LEN: Option<usize> =
        Some(NeonEventStore::<HyperTransport>::MAX_EVENT_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> =
        Some(NeonEventStore::<HyperTransport>::MAX_TAGS_PER_EVENT);
    const MAX_EVENTS_PER_BATCH: Option<usize> =
        Some(NeonEventStore::<HyperTransport>::MAX_EVENTS_PER_BATCH);

    async fn connect(&self) -> Self::Store {
        self.migrate().await;
        NeonEventStore::new(HyperTransport::shared(), self.config.clone())
    }

    async fn arm_mid_batch_fault(&self, after: usize) {
        self.migrate().await;
        let threshold = i64::try_from(after).unwrap_or(i64::MAX) + 1;
        let schema = happenstance_neon::config::quote_ident(self.schema());
        let statements = vec![
            SqlStatement::new(format!(
                "CREATE OR REPLACE FUNCTION {schema}.mid_batch_fault() RETURNS trigger AS \
                 $fn$ DECLARE seen bigint; BEGIN \
                 seen := coalesce(nullif(current_setting('hs.fault_seen', true), ''), '0')::bigint + 1; \
                 PERFORM set_config('hs.fault_seen', seen::text, true); \
                 IF seen = {threshold} THEN RAISE EXCEPTION 'the fixture armed a mid-batch fault'; END IF; \
                 RETURN NEW; END $fn$ LANGUAGE plpgsql"
            )),
            SqlStatement::new(format!(
                "DROP TRIGGER IF EXISTS mid_batch_fault_trigger ON {}",
                self.config.qualified_event()
            )),
            SqlStatement::new(format!(
                "CREATE TRIGGER mid_batch_fault_trigger AFTER INSERT ON {} \
                 FOR EACH ROW EXECUTE FUNCTION {schema}.mid_batch_fault()",
                self.config.qualified_event()
            )),
        ];
        execute(
            SqlRequest::batch(statements, happenstance_neon::IsolationLevel::ReadCommitted),
            "the mid-batch fault",
            self.schema(),
        )
        .await;
    }

    async fn arm_read_fault(&self) {
        self.migrate().await;
        let schema = happenstance_neon::config::quote_ident(self.schema());
        let event = self.config.qualified_event();
        let statements = vec![
            SqlStatement::new(format!(
                "CREATE OR REPLACE FUNCTION {schema}.read_fault_guard(p bigint) RETURNS boolean AS \
                 $guard$ BEGIN \
                 IF p > 2 THEN RAISE EXCEPTION 'the fixture armed a read fault'; END IF; \
                 RETURN true; END $guard$ LANGUAGE plpgsql"
            )),
            // Reversible and destroys nothing: the rows stay in
            // `event_read_fault_source` and the view reads them. Guarded on
            // `to_regclass` so that arming twice on one instance is a no-op
            // rather than an error.
            SqlStatement::new(format!(
                "DO $arm$ BEGIN \
                 IF to_regclass('{}.event_read_fault_source') IS NULL THEN \
                 ALTER TABLE {event} RENAME TO event_read_fault_source; \
                 CREATE VIEW {event} AS SELECT * FROM {schema}.event_read_fault_source \
                 WHERE {schema}.read_fault_guard(position); \
                 END IF; END $arm$",
                self.schema().replace('\'', "''")
            )),
        ];
        execute(
            SqlRequest::batch(statements, happenstance_neon::IsolationLevel::ReadCommitted),
            "the read fault",
            self.schema(),
        )
        .await;
    }

    async fn reopen(&self) {
        // Nothing to do, and that is the honest implementation rather than a
        // convenient one. This adapter holds no connection, no pool and no cache,
        // so there is no process-level state a new handle would not discard by
        // construction — every request already authenticates and routes on its
        // own. What `reopen` must NOT do is drop and recreate the schema: that
        // would make `acknowledged_writes_survive_a_reopen` pass by making it
        // vacuous, which is the trap `happenstance-sqlite`'s fixture documents at
        // the equivalent line.
    }
}

// -------------------------------------------------------------------------
// The projection fixture
// -------------------------------------------------------------------------

/// A live projection store over one isolated schema.
///
/// Beside [`NeonFixture`] rather than in `tests/neon_projection.rs`, because both
/// need the same sweep, the same schema-per-instance isolation and the same
/// transport — and a second copy of that is how the two would later disagree.
/// What differs is the migration each applies.
///
/// One instance is one backing store, which is the premise
/// `commit_rejects_a_foreign_batch` rests on: it opens the fixture **twice** and
/// expects the two to be strangers.
#[cfg(all(feature = "projection-store", feature = "conformance"))]
#[derive(Debug)]
pub(crate) struct NeonProjectionFixture {
    config: NeonConfig,
    migrated: OnceCell<()>,
}

#[cfg(all(feature = "projection-store", feature = "conformance"))]
impl NeonProjectionFixture {
    /// Mints a fresh, empty schema name for this instance.
    ///
    /// The prefix is `hsp_` rather than `hs_` so that the two fixtures cannot
    /// collide. They mint from separate counters, so one prefix would name two
    /// different schemas in one process — and the symptom would be one fixture's
    /// rows appearing in the other's isolation rule.
    pub(crate) fn new() -> Self {
        Self {
            config: NeonConfig::default().with_schema(mint_schema("hsp")),
            migrated: OnceCell::const_new(),
        }
    }

    /// The schema this instance owns.
    pub(crate) fn schema(&self) -> &str {
        &self.config.schema
    }

    /// This instance's configuration.
    pub(crate) fn config(&self) -> &NeonConfig {
        &self.config
    }

    /// Migration 2 **and** the probe table, in that order.
    ///
    /// The probe table is the suite's own read model and is deliberately not a
    /// file in `migrations/`, so the fixture is the thing that has to create it —
    /// which is also the check that `projection_store::probe_table` stays
    /// applicable SQL rather than drifting into prose.
    async fn migrate(&self) {
        self.migrated
            .get_or_init(|| async {
                sweep_once().await;
                let mut statements: Vec<SqlStatement> =
                    migration::render(migration::MIGRATION_2, &self.config)
                        .into_iter()
                        .map(SqlStatement::new)
                        .collect();
                statements.push(SqlStatement::new(
                    happenstance_neon::projection_store::probe_table(&self.config),
                ));
                execute(
                    SqlRequest::batch(statements, happenstance_neon::IsolationLevel::ReadCommitted),
                    "migration 2 and the probe table",
                    self.schema(),
                )
                .await;
            })
            .await;
    }
}

#[cfg(all(feature = "projection-store", feature = "conformance"))]
impl happenstance_testkit::ProjectionFixture for NeonProjectionFixture {
    type Store = happenstance_neon::NeonProjectionStore<HyperTransport>;

    /// A MUST, and this adapter meets it for real. A second `connect` is a second
    /// store over the same schema with its **own** stamp, so the two are
    /// strangers to each other's batches and neighbours in the same tables —
    /// which is the pair of properties the rules need. It is not a refcount
    /// clone: `NeonProjectionStore::new` mints a fresh identity.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    /// Declined, and the reason is the store's rather than the fixture's
    /// convenience: `NeonProjectionStore` holds **no protection policy**.
    ///
    /// PS-18 makes refusal a mechanism the port supplies and leaves *what to
    /// protect* to the domain. This adapter owns the checkpoint row and the batch
    /// that carries it, and knows nothing about which projection a regulator
    /// already holds a hash chain for, so `reset` returns `ResetError::Refused`
    /// on no path at all and a fixture claiming the capability would fail
    /// `refused_reset_changes_nothing` at its first assertion.
    ///
    /// The reason is written here in this store's own words rather than inherited
    /// from the testkit's default, which is what CF-18 asks for — and it is this
    /// store's, not `happenstance-postgres`'s: the argument happens to land the
    /// same way, and a declension copied because it landed the same way is the
    /// declension-by-inheritance CF-18 forbids.
    const RESET_REFUSAL: Capability = Capability::declined(
        "NeonProjectionStore holds no protection policy: the read model belongs to \
         the caller and this adapter owns only the checkpoint row and the batch that \
         carries it, so there is no projection it could decline to reset and `reset` \
         returns `Refused` on no path at all. The alternative — a protected-projections \
         table invented here so that one more rule reports `Ran` — would put a domain \
         policy into an adapter designed to keep the domain out",
    );

    /// Supported, and armed for real: a `BEFORE INSERT OR UPDATE` trigger on the
    /// checkpoint table raising an exception aborts the **checkpoint** half of a
    /// commit whose read-model half is already in the same batch.
    ///
    /// It is the only way PS-1's second conjunct is observable at all — nothing a
    /// caller holds can make a conformant `commit` fail — and the whole batch is
    /// one server-side transaction, so the abort takes the read-model statements
    /// with it. That is exactly what the rule then reads back and finds unchanged.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        self.migrate().await;
        happenstance_neon::NeonProjectionStore::new(HyperTransport::shared(), self.config.clone())
    }

    async fn arm_commit_fault(&self) {
        self.migrate().await;
        let schema = happenstance_neon::config::quote_ident(self.schema());
        let checkpoint = self.config.qualified_checkpoint();
        let statements = vec![
            SqlStatement::new(format!(
                "CREATE OR REPLACE FUNCTION {schema}.projection_commit_fault() RETURNS trigger AS \
                 $fault$ BEGIN RAISE EXCEPTION 'the fixture armed a commit fault'; END $fault$ \
                 LANGUAGE plpgsql"
            )),
            SqlStatement::new(format!(
                "DROP TRIGGER IF EXISTS projection_commit_fault_trigger ON {checkpoint}"
            )),
            // `BEFORE INSERT OR UPDATE` covers both arms because the checkpoint
            // write is an upsert, so which one Postgres reaches depends on
            // whether the projection has committed before.
            SqlStatement::new(format!(
                "CREATE TRIGGER projection_commit_fault_trigger \
                 BEFORE INSERT OR UPDATE ON {checkpoint} \
                 FOR EACH ROW EXECUTE FUNCTION {schema}.projection_commit_fault()"
            )),
        ];
        execute(
            SqlRequest::batch(statements, happenstance_neon::IsolationLevel::ReadCommitted),
            "the commit fault",
            self.schema(),
        )
        .await;
    }
}

// -------------------------------------------------------------------------
// Shared machinery
// -------------------------------------------------------------------------

/// A schema name carrying the second it was minted in, the process and an
/// ordinal.
///
/// The timestamp is what makes [`sweep_once`] safe; the process id is what stops
/// two binaries colliding; the ordinal is what stops two instances in one process
/// colliding.
fn mint_schema(prefix: &str) -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{prefix}_{seconds}_{}_{ordinal}", std::process::id())
}

/// Drops every fixture-shaped schema older than [`SWEEP_AFTER_SECONDS`], once per process.
///
/// A Neon branch persists, so without this the event-store family's ninety-odd
/// schemas per run accumulate forever. Keyed on age rather than on "not mine"
/// because two test binaries run concurrently under one `cargo test`.
async fn sweep_once() {
    SWEPT
        .get_or_init(|| async {
            let transport = HyperTransport::shared();
            let listing = SqlRequest::single(SqlStatement::new(
                "SELECT nspname FROM pg_namespace \
                 WHERE nspname ~ '^hsp?_[0-9]+_[0-9]+_[0-9]+$'",
            ))
            .read_only();
            let Ok(response) = transport.round_trip(listing).await else {
                // A sweep that cannot run is housekeeping that did not happen,
                // never a reason to fail a rule. The migration below will report
                // a genuinely unreachable endpoint with a message that names it.
                return;
            };
            let Ok(body) = happenstance_neon::wire::ResponseBody::parse(&response.body) else {
                return;
            };
            let cutoff = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .saturating_sub(SWEEP_AFTER_SECONDS);

            let stale: Vec<SqlStatement> = body
                .result_sets()
                .first()
                .map(|result| result.rows.as_slice())
                .unwrap_or_default()
                .iter()
                .filter_map(|row| row.get("nspname")?.as_str())
                .filter(|name| minted_at(name).is_some_and(|seconds| seconds < cutoff))
                .map(|name| {
                    SqlStatement::new(format!(
                        "DROP SCHEMA {} CASCADE",
                        happenstance_neon::config::quote_ident(name)
                    ))
                })
                .collect();

            if stale.is_empty() {
                return;
            }
            let _ = transport
                .round_trip(SqlRequest::batch(
                    stale,
                    happenstance_neon::IsolationLevel::ReadCommitted,
                ))
                .await;
        })
        .await;
}

/// The second a fixture schema was minted in, out of its own name.
fn minted_at(schema: &str) -> Option<u64> {
    schema.split('_').nth(1)?.parse().ok()
}

/// Sends `request` and panics with an environment message if it did not succeed.
///
/// # Panics
///
/// On any failure to reach or prepare the endpoint. That is deliberate and it is
/// the trait's own rule: a fixture that cannot connect is a broken **test
/// environment**, not a non-conformant adapter, and [`Fixture::connect`] has no
/// `Result` so that "the endpoint is down" and "the adapter is wrong" cannot
/// arrive through one channel. The connection string is never part of the message.
async fn execute(request: SqlRequest, what: &str, schema: &str) {
    let transport = HyperTransport::shared();
    let host = transport.host().to_owned();
    let response = transport.round_trip(request).await.unwrap_or_else(|error| {
        panic!(
            "a broken test environment, not a non-conformant adapter: {what} could not \
             reach {host} ({error})"
        )
    });
    assert!(
        response.is_success(),
        "a broken test environment: {what} failed in schema `{schema}` at {host} \
         (HTTP {}) — {}",
        response.status,
        String::from_utf8_lossy(&response.body)
            .chars()
            .take(400)
            .collect::<String>()
    );
}
