//! The projection conformance suite, run against a live pinned Postgres.
//!
//! # How this target is gated, and why it is gated *this* way
//!
//! The same argument as `postgres_conformance.rs`, which carries it in full:
//! every test here needs a Docker daemon, and the default gate must not.
//! `required-features` does not work, because the gate runs `--all-features` and
//! would therefore *enable* the required feature and run these tests against a
//! server that is not there. An environment-variable early return does not work
//! either, because "no server, pass quietly" is indistinguishable in CI output
//! from a rule that passed — CF-18's argument about omitted skips, one layer out.
//!
//! `#[ignore]` does work: it composes with `--all-features`, keeps the target
//! compiled and linted by the default gate, and makes the gated tests appear as
//! `ignored` rather than vanishing from the count.
//!
//! The `#![cfg(…)]` above is a different mechanism doing a different job. It is
//! not a gate against the *server*; it is the two features the impl under test
//! lives behind. `projection-store` forwards `happenstance-core`'s own
//! `unstable-projection`, and `conformance` is what compiles
//! `impl ProjectionProbe for PostgresProjectionStore`. With either off there is no
//! store to mount a suite over, and the target compiles to nothing rather than
//! failing to compile.
//!
//! # Running it
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features --test projection -- --ignored --list
//! cargo test -p happenstance-postgres --all-features --test projection -- --ignored --show-output --test-threads=1
//! ```
//!
//! `--list` first, so "no rule is absent from the run" is proven rather than
//! assumed. `--show-output` because a declined capability's reason is printed
//! only under it, and this adapter declines two things on purpose.
//!
//! # What this adapter reports rather than passes
//!
//! Two rules are **reported skips**, and neither is a gap:
//!
//! * `refused_reset_changes_nothing`, because `RESET_REFUSAL` is declined — the
//!   store holds no protection policy, and inventing one so that a rule reports
//!   `Ran` would put a domain decision inside an adapter designed to keep the
//!   domain out.
//! * `batch_reads_reflect_pending_writes`, because `READS_THROUGH_BATCH` is
//!   `false` — a `PostgresProjectionBatch` has been sent to the server exactly
//!   never, and answering from committed state is what PS-12 forbids by name.
//!
//! Both print their reason under `--show-output`. Neither is omitted from the
//! binary, which is the distinction CF-18 exists to hold.

#![cfg(all(
    feature = "projection-store",
    feature = "conformance",
    not(target_arch = "wasm32")
))]
#![allow(clippy::unwrap_used)]

mod support;

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};
use happenstance_postgres::error::PostgresProjectionStoreError;
use happenstance_postgres::live_projection_store::LivePostgresProjectionStore;
use happenstance_postgres::projection_store::{PgParam, PostgresProjectionStore};
use happenstance_postgres::sqlx::postgres::PgPoolOptions;
use happenstance_postgres::sqlx::{self, Row};
use support::PostgresProjectionFixture;

/// The projection family's emitter, plus `#[ignore]`.
///
/// A local macro rather than a parameter on the testkit's, for the reason
/// `postgres_conformance.rs` gives: the emitter is already a parameter of the
/// mount, so adding one attribute needs no change to the testkit at all. The
/// reason string is not decoration — `cargo test -- --ignored --list` prints it,
/// so the one command an adapter author runs to find out what is gated also tells
/// them how to ungate it.
macro_rules! emit_ignored_projection_tokio {
    ($($name:ident),* $(,)?) => {
        $(
            #[tokio::test]
            #[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
            async fn $name() {
                happenstance_testkit::projection::rules::$name(__conformance_fixture)
                    .await
                    .report(::core::stringify!($name));
            }
        )*
    };
}
pub(crate) use emit_ignored_projection_tokio;

happenstance_testkit::projection_store_conformance!(
    mod_name = projection_conformance,
    emit = crate::emit_ignored_projection_tokio,
    fixture = PostgresProjectionFixture::new()
);

// -------------------------------------------------------------------------
// The adapter-private tests the borrowed suite cannot make
// -------------------------------------------------------------------------

/// Migration 2's `CHECK` refuses an authority the contract cannot represent.
///
/// The text half of this guard is a unit test in `migration.rs`; this is the half
/// only a server can answer. It matters because `authority_to_row` writes
/// `'unknown'` for a `#[non_exhaustive]` variant this adapter has never seen —
/// the arm exists so that such a value fails **where it happens** rather than
/// being stored as `live` and reported later as a live projection that is really
/// a half-finished rebuild.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_authority_check_refuses_a_third_value() {
    let fixture = PostgresProjectionFixture::new();
    let pool = fixture.pool_for_test().await;

    let outcome = sqlx::query(
        "INSERT INTO projection_checkpoint (projection_id, position, authority) \
         VALUES ('third-value', 1, 'unknown')",
    )
    .execute(&pool)
    .await;

    let error = outcome.expect_err(
        "migration 2's CHECK accepted an authority `Checkpoint` cannot represent, so a \
         value this adapter writes only for an unrecognised variant would be stored and \
         read back as a failure far from where it happened",
    );
    assert!(
        error.to_string().contains("projection_checkpoint"),
        "the refusal should name the table it came from, and said: {error}"
    );
}

/// A position `bigint` cannot hold is refused rather than wrapped.
///
/// `SequencePosition` is a `NonZeroU64` and `bigint` is signed, so the top half of
/// the domain has no representation. The rejected alternative is a cast, which
/// would write a negative and then read it back as `CheckpointOutOfRange` from a
/// row that looked fine when it was written.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_position_above_bigint_is_refused_at_the_write() {
    let fixture = PostgresProjectionFixture::new();
    let store =
        <PostgresProjectionFixture as happenstance_testkit::ProjectionFixture>::connect(&fixture)
            .await;

    let id = ProjectionId::from_static("above-bigint");
    let too_large = SequencePosition::new(u64::MAX).expect("u64::MAX is non-zero");

    let outcome = store
        .commit(
            store.begin().await.unwrap(),
            &id,
            too_large,
            Authority::Live,
        )
        .await;

    assert!(
        outcome.is_err(),
        "a position `bigint` cannot represent must be refused, and this commit accepted it"
    );
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::NeverRun,
        "the refused commit moved the checkpoint anyway"
    );
}

/// The checkpoint row carries the authority it was committed with, and reading it
/// back is not an inference.
///
/// `Checkpoint` has three states and `Authority` has two; the third state is the
/// absence of the row. A store that inferred `Live` from a present row would pass
/// every rule that only commits live projections and would report a rebuild in
/// progress as finished.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_stored_authority_round_trips() {
    let fixture = PostgresProjectionFixture::new();
    let store =
        <PostgresProjectionFixture as happenstance_testkit::ProjectionFixture>::connect(&fixture)
            .await;
    let pool = fixture.pool_for_test().await;

    let id = ProjectionId::from_static("authority-round-trip");
    store
        .commit(
            store.begin().await.unwrap(),
            &id,
            SequencePosition::FIRST,
            Authority::Rebuilding,
        )
        .await
        .unwrap();

    let stored: String =
        sqlx::query("SELECT authority FROM projection_checkpoint WHERE projection_id = $1")
            .bind(id.as_str())
            .fetch_one(&pool)
            .await
            .unwrap()
            .get(0);
    assert_eq!(
        stored, "rebuilding",
        "the authority was not stored as given"
    );

    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::Rebuilding {
            through: SequencePosition::FIRST
        },
        "the authority did not survive the round trip"
    );
}

/// Two fixture instances are two backing stores.
///
/// CLAUDE.md's fixture rule, checked for this fixture rather than assumed: the
/// isolation is a schema per instance, and a rule that opens the fixture twice —
/// `commit_rejects_a_foreign_batch` does — depends on it.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn two_fixture_instances_own_different_schemas() {
    let first = PostgresProjectionFixture::new();
    let second = PostgresProjectionFixture::new();
    assert_ne!(
        first.schema(),
        second.schema(),
        "two fixture instances share a schema, so one instance's rows are visible \
         to the other and every isolation premise in the suite is false here"
    );
}

// -------------------------------------------------------------------------
// ADR-0084: a statement's values must match the highest `$n` its text uses
// -------------------------------------------------------------------------

/// The probe key the parameter-count tests write through.
const KEY: &str = "arity-kept";

/// The value written under [`KEY`]. Not `0` or `1`, so a default cannot pass.
const VALUE: u64 = 4_211;

/// The surplus leg's text. Issued by no other test, so on every connection it
/// reaches it is a **first** preparation: an adapter without the count check
/// commits the surplus deterministically rather than by the luck of the cache.
const SURPLUS_TEXT: &str = "DELETE FROM projection_probe WHERE k = $1 /* arity-surplus */";

/// A buffered store over a pool of **one** connection onto `fixture`'s schema.
///
/// One connection, so a correct call after a refused surplus reuses the very
/// connection the surplus would have reached (ADR-0084 §6). The fixture's own
/// pool is opened first, because that is what creates the schema, migration 2
/// and the probe table.
async fn one_connection_store(fixture: &PostgresProjectionFixture) -> PostgresProjectionStore {
    let pool = fixture.pool_for_test().await;
    let options = (*pool.connect_options())
        .clone()
        .options([("search_path", fixture.schema())]);
    let one = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("a broken test environment: a one-connection pool onto the fixture's schema");
    PostgresProjectionStore::new(one)
}

/// Commits `KEY = VALUE` at the first position.
async fn commit_kept(store: &PostgresProjectionStore, id: &ProjectionId) {
    let mut batch = store.begin().await.unwrap();
    store.probe_write(&mut batch, KEY, VALUE).await.unwrap();
    store
        .commit(batch, id, SequencePosition::FIRST, Authority::Live)
        .await
        .expect("the setup commit should succeed");
}

/// A mismatched statement fails the commit, in both directions, and moves
/// nothing — and a refused surplus leaves no cached statement behind.
///
/// Each leg holds a valid probe write (statement 0) and then a `DELETE` aimed at
/// the committed row (statement 1), so every wrong implementation changes
/// something observable. The server refuses the too-few leg on its own; the
/// too-many leg is refused only by the adapter's count. The follow-up commits
/// the surplus text correctly on the same single connection, which rejects a
/// check placed *after* the statement reached the server: by then the text is
/// prepared with two parameters, and the correct call is refused by the cache.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_statement_with_the_wrong_parameter_count_fails_the_commit_and_moves_nothing() {
    let fixture = PostgresProjectionFixture::new();
    let store = one_connection_store(&fixture).await;
    let id = ProjectionId::from_static("arity-commit");
    commit_kept(&store, &id).await;
    let second = SequencePosition::new(2).expect("2 is non-zero");

    let legs: [(&str, &'static str, Vec<PgParam>, usize, usize); 2] = [
        (
            "too few",
            "DELETE FROM projection_probe WHERE k = $1 OR k = $2",
            vec![PgParam::text(KEY)],
            2,
            1,
        ),
        (
            "too many",
            SURPLUS_TEXT,
            vec![PgParam::text(KEY), PgParam::text("unused")],
            1,
            2,
        ),
    ];
    for (leg, sql, params, expected_declared, expected_supplied) in legs {
        let mut batch = store.begin().await.unwrap();
        store.probe_write(&mut batch, "fresh", 7).await.unwrap();
        batch.push(sql, params);

        let refused = store
            .commit(batch, &id, second, Authority::Live)
            .await
            .expect_err("a parameter-count mismatch must fail the commit");

        assert!(
            matches!(
                refused,
                CommitError::Store(PostgresProjectionStoreError::ParameterCount {
                    statement: 1,
                    declared,
                    supplied,
                    ..
                }) if declared == expected_declared && supplied == expected_supplied
            ),
            "{leg}: the refusal must name statement 1, {expected_declared} declared and \
             {expected_supplied} supplied: {refused:?}"
        );
        assert_eq!(store.probe_read(KEY).await.unwrap(), Some(VALUE), "{leg}");
        assert_eq!(store.probe_read("fresh").await.unwrap(), None, "{leg}");
        assert_eq!(
            store.checkpoint(&id).await.unwrap(),
            Checkpoint::Live {
                through: SequencePosition::FIRST
            },
            "{leg}"
        );
    }

    let mut batch = store.begin().await.unwrap();
    batch.push(SURPLUS_TEXT, [PgParam::text(KEY)]);
    store
        .commit(batch, &id, second, Authority::Live)
        .await
        .expect("the surplus text with the right count must commit on the same connection");
    assert_eq!(store.probe_read(KEY).await.unwrap(), None);
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::Live { through: second }
    );
}

/// `reset` refuses a surplus value the same way, and moves nothing.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn a_statement_with_the_wrong_parameter_count_fails_the_reset_and_moves_nothing() {
    let fixture = PostgresProjectionFixture::new();
    let store = one_connection_store(&fixture).await;
    let id = ProjectionId::from_static("arity-reset");
    commit_kept(&store, &id).await;

    let mut batch = store.begin().await.unwrap();
    store.probe_delete_all(&mut batch).await.unwrap();
    batch.push(SURPLUS_TEXT, [PgParam::text(KEY), PgParam::text("unused")]);

    let refused = store
        .reset(batch, &id)
        .await
        .expect_err("a parameter-count mismatch must fail the reset");

    assert!(
        matches!(
            refused,
            ResetError::Store(PostgresProjectionStoreError::ParameterCount {
                statement: 1,
                declared: 1,
                supplied: 2,
                ..
            })
        ),
        "{refused:?}"
    );
    assert_eq!(store.probe_read(KEY).await.unwrap(), Some(VALUE));
    assert_eq!(
        store.checkpoint(&id).await.unwrap(),
        Checkpoint::Live {
            through: SequencePosition::FIRST
        }
    );
}

/// How many parameters the **server** infers for `sql`.
///
/// `PREPARE` over the simple-query protocol, read back from
/// `pg_prepared_statements` and deallocated, on one checked-out connection. A
/// named server-side statement rather than `sqlx`'s `describe`, which is
/// `#[doc(hidden)]` and answers from the per-connection cache.
async fn server_parameter_count(pool: &sqlx::PgPool, sql: &str) -> usize {
    let mut connection = pool.acquire().await.unwrap();
    sqlx::raw_sql(&format!("PREPARE arity_probe AS {sql}"))
        .execute(&mut *connection)
        .await
        .unwrap_or_else(|error| panic!("the server could not prepare {sql:?}: {error}"));
    let count: i32 = sqlx::query_scalar(
        "SELECT coalesce(cardinality(parameter_types), 0) \
         FROM pg_prepared_statements WHERE name = 'arity_probe'",
    )
    .fetch_one(&mut *connection)
    .await
    .unwrap();
    sqlx::raw_sql("DEALLOCATE arity_probe")
        .execute(&mut *connection)
        .await
        .unwrap();
    usize::try_from(count).expect("a parameter count is non-negative")
}

/// The scan agrees with the server on every construct it skips.
///
/// The differential behind ADR-0084 §10's last falsifier. For each statement
/// the server's own count is taken, and the live store must then accept
/// exactly that many values and refuse one more — through `execute_raw_sql`,
/// so this also exercises the live batch's refusal on a target CI runs.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
async fn the_placeholder_scan_agrees_with_the_server() {
    const CORPUS: [&str; 18] = [
        "SELECT 1",
        "SELECT $1::text, $2::text, $1::text",
        "SELECT $1::text, '$2'",
        "SELECT $1::text, 'it''s $2'",
        r"SELECT $1::text, '\', $2::text",
        r"SELECT $1::text, E'\'$2'",
        r"SELECT $1::text, e'\\', $2::text",
        "SELECT $1::text, N'$2'",
        "SELECT $1::text, U&'$2'",
        r#"SELECT $1::text AS "c$2""#,
        r#"SELECT $1::text AS "a""$2""#,
        r#"SELECT $1::text AS U&"c$2""#,
        "SELECT $1::text, $t$ $2 $t$",
        "SELECT $1::text, $$ $2 $$",
        "SELECT $1::text /* /* */ $2 */",
        "SELECT $1::text -- $2\n",
        "SELECT $01::text",
        "SELECT $1::text AS a$2",
    ];
    let fixture = PostgresProjectionFixture::new();
    let pool = fixture.pool_for_test().await;
    let store = LivePostgresProjectionStore::new(pool.clone());

    for sql in CORPUS {
        let expected = server_parameter_count(&pool, sql).await;

        let mut batch = store.begin().await.unwrap();
        batch
            .execute_raw_sql(sql, vec![PgParam::text("x"); expected])
            .await
            .unwrap_or_else(|error| {
                panic!("{sql:?}: the server counts {expected}, and the store refused that many: {error:?}")
            });
        store.rollback(batch).await.unwrap();

        let mut batch = store.begin().await.unwrap();
        let surplus = expected + 1;
        let refused = batch
            .execute_raw_sql(sql, vec![PgParam::text("x"); surplus])
            .await
            .expect_err("one value more than the server counts must be refused");
        assert!(
            matches!(
                refused,
                PostgresProjectionStoreError::ParameterCount {
                    statement: 0,
                    declared,
                    supplied,
                    ..
                } if declared == expected && supplied == surplus
            ),
            "{sql:?}: the server counts {expected}: {refused:?}"
        );
        store.rollback(batch).await.unwrap();
    }
}
