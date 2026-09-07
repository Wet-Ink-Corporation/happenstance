//! The schema, and the one way it is applied.
//!
//! # Why a file rather than a `sqlx::migrate!`
//!
//! The manifest carries the argument in full. The short form: the schema is one
//! migration applied to a schema created seconds earlier, so a migrator's
//! version tracking has no history to track, and its `_sqlx_migrations`
//! bookkeeping table is *per schema* — one row of bookkeeping per fixture
//! instance, about a history that is one row long by construction.
//!
//! # Why the SQL is `include_str!`d rather than written inline
//!
//! `happenstance-sqlite` keeps its `MIGRATION_1` as a `&str` constant in Rust
//! and mirrors it into a doc comment, then has a test compare the mirror against
//! `sqlite_master` object by object — because the two copies drift otherwise. A
//! `.sql` file has no second copy to drift from, gets SQL syntax highlighting in
//! every editor, and is the artefact a DBA asks for when they want to know what
//! this crate will do to their database. `include_str!` means it is still
//! compiled in, so a published crate carries its own schema and a consumer needs
//! nothing from this repository to apply it.

use sqlx::PgExecutor;

use crate::error::PostgresEventStoreError;

/// The schema this crate expects, as SQL.
///
/// Exposed rather than private because a consumer running their own migration
/// tooling needs the text, and the alternative is that they copy it out of the
/// repository and it drifts. Applying it is [`apply`].
///
/// It is idempotent: every object is created `IF NOT EXISTS`, so applying it to
/// a schema that already has it is a no-op rather than an error.
pub const MIGRATION_1: &str = include_str!("../migrations/0001_event_log.sql");

/// The schema version this crate's code is written against.
///
/// One, and it stays one until first publish. The slice that wires ADR-0024's
/// mechanism adds its column to [`MIGRATION_1`] rather than opening a migration
/// 2, because nothing has ever shipped this schema to a consumer — both
/// `publish = false` — so there is no deployed table to alter and "migration"
/// here is a schema-authoring concern rather than a data-movement one. That ends
/// at first publish.
pub const SCHEMA_VERSION: u32 = 1;

/// Applies [`MIGRATION_1`] to whatever schema `executor` resolves against.
///
/// Which schema that is depends on the connection's `search_path`, and this
/// function deliberately does not set it: a caller pointing a pool at one schema
/// and a caller using the default `public` are both legitimate, and a library
/// that silently rewrites `search_path` breaks the first one. The test fixture
/// creates a schema per instance and sets the path on the pool; a consumer does
/// whatever their deployment does.
///
/// # Errors
///
/// [`PostgresEventStoreError::Driver`] if the server rejects the statements —
/// most plausibly because the role cannot create objects in the target schema.
pub async fn apply<'e, E>(executor: E) -> Result<(), PostgresEventStoreError>
where
    E: PgExecutor<'e>,
{
    // `execute` rather than a prepared query: this is multiple statements, and
    // the extended protocol admits exactly one. Postgres runs a multi-statement
    // simple-protocol batch in an implicit transaction, so a failure part-way
    // leaves no half-built table behind.
    sqlx::raw_sql(MIGRATION_1).execute(executor).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{MIGRATION_1, SCHEMA_VERSION};

    /// [`MIGRATION_1`] with its `--` line comments removed.
    ///
    /// The two guards below scan for words that must not appear in the schema —
    /// and the schema's comments are largely *about* those words, explaining why
    /// each is absent. Scanning the raw text therefore fails on a correct file,
    /// which is what the first run of these tests did: `bigserial` and `xid8`
    /// were both found, both in prose saying they were deliberately not used.
    ///
    /// A test that cannot tell a statement from a comment about a statement is
    /// not checking what it claims to. This is not a workaround for the failure;
    /// it is the fix for it.
    fn statements() -> String {
        MIGRATION_1
            .lines()
            .map(|line| line.split_once("--").map_or(line, |(code, _)| code))
            .collect::<Vec<_>>()
            .join(
                "
",
            )
    }

    /// The stripper is itself load-bearing, so it gets a test.
    ///
    /// If it silently stopped stripping, both guards below would go green on a
    /// file that had genuinely acquired a `bigserial` — the failure mode is a
    /// pair of tests that pass because they can no longer see anything.
    #[test]
    fn the_comment_stripper_removes_prose_and_keeps_sql() {
        let stripped = statements();
        assert!(
            !stripped.contains("ADR-0024"),
            "prose survived the strip, so the guards below are scanning comments"
        );
        assert!(
            stripped.contains("CREATE TABLE IF NOT EXISTS event"),
            "the strip ate SQL, so the guards below are scanning nothing"
        );
    }

    /// The mechanism's column is present, and this test changed hands.
    ///
    /// It was written by `postgres-schema-and-live-fixture` asserting the
    /// **absence** of an `xid8` column, so that a schema story could not settle
    /// the position-visibility question in passing. That guard did its job and
    /// is now spent: the story that owns the mechanism has chosen it, on
    /// ADR-0013's phase-2 measurement, and the column is deliberate.
    ///
    /// Inverted rather than deleted. A removed test leaves no trace that the
    /// schema ever had something to prove; an inverted one records that the
    /// column arrived through the story entitled to add it, and fails if some
    /// later edit removes the mechanism while leaving the reads that depend on
    /// it in place.
    #[test]
    fn migration_1_carries_the_visibility_mechanism_column() {
        let statements = statements();
        assert!(
            statements.contains("xact_id"),
            "the frontier mechanism's column is gone, but `head` and every read still              filter on it"
        );
        assert!(
            statements.contains("event_xact_idx"),
            "the frontier predicate has lost its index, which turns every read into a              sequential scan and charges the mechanism for the absence of an index rather              than for what it costs"
        );
    }

    /// `position` must not be a sequence default, and the reason is one sentence
    /// long: a `serial` column is `nextval()`, and `nextval()` allocating outside
    /// the transaction is where ES-10's invariant is lost.
    ///
    /// This is a text assertion because it is cheap and runs with no server. The
    /// live half — `column_default IS NULL` and `is_identity = 'NO'` read back
    /// out of `information_schema` — is in the gated conformance target, where a
    /// real server can disagree with the text.
    #[test]
    fn position_is_not_a_serial_column() {
        let statements = statements();
        for forbidden in ["bigserial", "serial", "AS IDENTITY"] {
            assert!(
                !statements.contains(forbidden),
                "migration 1 contains `{forbidden}`: a sequence default on `position` \
                 is exactly the construction ES-10 forbids"
            );
        }
    }

    #[test]
    fn the_schema_version_is_one_until_first_publish() {
        assert_eq!(SCHEMA_VERSION, 1);
    }
}
