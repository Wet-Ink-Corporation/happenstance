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

use happenstance_core::{Authority, Checkpoint, ProjectionId, ProjectionStore, SequencePosition};
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

    let id = ProjectionId::new("above-bigint");
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

    let id = ProjectionId::new("authority-round-trip");
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
