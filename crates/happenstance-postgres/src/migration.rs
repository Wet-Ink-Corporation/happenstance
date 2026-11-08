//! The schema, and the one way it is applied.
//!
//! # Why a file rather than a `sqlx::migrate!`
//!
//! The manifest carries the argument in full. The short form: a migrator's
//! `_sqlx_migrations` bookkeeping table is *per schema*, so one per fixture
//! instance, about a history this crate keeps idempotent instead. Every
//! migration here is `IF NOT EXISTS` throughout, so `apply` brings any schema,
//! fresh or migrated by an older build, to what this build expects by running
//! them all, and needs no record of which already ran. The event log has two
//! since #420: `MIGRATION_1` and `MIGRATION_3`. (Named rather than linked: both
//! are behind `event-store`, and this module is not.)
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

#[cfg(feature = "event-store")]
use crate::error::PostgresEventStoreError;

/// The event log's first migration, as SQL: the table, its sequence and
/// indexes, and the store's identity.
///
/// Exposed rather than private because a consumer running their own migration
/// tooling needs the text, and the alternative is that they copy it out of the
/// repository and it drifts. It is not the whole schema this build expects:
/// [`MIGRATION_3`] follows it. Applying both is [`apply`].
///
/// It is idempotent: every object is created `IF NOT EXISTS`, so applying it to
/// a schema that already has it is a no-op rather than an error. Its text is
/// frozen since this crate's first publish; a change to the log is a new
/// migration.
#[cfg(feature = "event-store")]
pub const MIGRATION_1: &str = include_str!("../migrations/0001_event_log.sql");

/// The event log's second migration, as SQL: the write-group columns and
/// their index (#420).
///
/// `write_group`, on every row, the lowest position the write that put the row
/// in gave out, and `group_guard`, on that first row only, the guard an
/// ingested group arrived under. A runner and a store peer read the log back
/// in whole writes through them (SY-39). Both nullable with no default, so
/// adding them rewrites no row; a row from before this migration reads as a
/// group of its own.
///
/// **Apply it before deploying this build**: every `INSERT` this build sends
/// names `write_group`, and fails against a table without it. Idempotent, as
/// [`MIGRATION_1`] is. The index is built in the migration's transaction; the
/// file says how to build it `CONCURRENTLY` first on a large table.
///
/// Numbered 3 because that is the directory's sequence:
/// `migrations/0002_projection_checkpoint.sql` belongs to the other role.
#[cfg(feature = "event-store")]
pub const MIGRATION_3: &str = include_str!("../migrations/0003_write_group.sql");

/// What [`apply`] sends: [`MIGRATION_1`], then [`MIGRATION_3`], as one batch.
///
/// `concat!` over the two `include_str!`s, so the batch is one constant
/// compiled in and not a string assembled per call.
#[cfg(feature = "event-store")]
const EVENT_LOG: &str = concat!(
    include_str!("../migrations/0001_event_log.sql"),
    "\n",
    include_str!("../migrations/0003_write_group.sql"),
);

/// The **event log's** schema version.
///
/// Two: [`MIGRATION_1`] and [`MIGRATION_3`]. It was one through first
/// publish, while the slice that wired ADR-0024's mechanism could still add
/// its column to [`MIGRATION_1`] in place, because no deployed table existed to
/// alter. That licence ended at this crate's first publish, so write groups,
/// the first change to the log after it, are a migration of their own and a
/// version of their own.
///
/// Nothing stores this number in the database: a schema carries no marker, and
/// [`apply`] is idempotent, so bringing a schema up to this version is running
/// [`apply`] against it.
///
/// `migrations/0002_projection_checkpoint.sql` is not part of it. It adds
/// no object to the event log and belongs to the other role's lineage, which
/// carries its own `PROJECTION_SCHEMA_VERSION` — named in code rather than
/// linked, because that constant is behind the `projection-store` feature and
/// this one is not. The number in that filename is the directory's sequence, so
/// that a DBA applying the files in order gets a working database for whichever
/// roles they use.
#[cfg(feature = "event-store")]
pub const SCHEMA_VERSION: u32 = 2;

/// Applies [`MIGRATION_1`] and then [`MIGRATION_3`] to whatever schema
/// `executor` resolves against.
///
/// Which schema that is depends on the connection's `search_path`, and this
/// function deliberately does not set it: a caller pointing a pool at one schema
/// and a caller using the default `public` are both legitimate, and a library
/// that silently rewrites `search_path` breaks the first one. The test fixture
/// creates a schema per instance and sets the path on the pool; a consumer does
/// whatever their deployment does.
///
/// # Upgrading a schema an older build migrated
///
/// Run this against it, then deploy. Both migrations are idempotent, so on
/// such a schema only [`MIGRATION_3`] does anything. Its `ALTER TABLE`s take
/// a brief exclusive lock on `event`, and its index build blocks writes while
/// it runs; `migrations/0003_write_group.sql` says how to build the index
/// beforehand without blocking them.
///
/// # Errors
///
/// [`PostgresEventStoreError::Driver`] if the server rejects the statements —
/// most plausibly because the role cannot create objects in the target schema.
#[cfg(feature = "event-store")]
pub async fn apply<'e, E>(executor: E) -> Result<(), PostgresEventStoreError>
where
    E: PgExecutor<'e>,
{
    // `execute` rather than a prepared query: this is multiple statements, and
    // the extended protocol admits exactly one. Postgres runs a multi-statement
    // simple-protocol batch in an implicit transaction, so a failure part-way
    // leaves no half-built table behind, and a schema it reaches has both
    // migrations or neither.
    sqlx::raw_sql(EVENT_LOG).execute(executor).await?;
    Ok(())
}

/// The projection store's schema, as SQL.
///
/// Exposed for the same reason `MIGRATION_1` is: a consumer running their own
/// migration tooling needs the text, and the alternative is that they copy it out
/// of the repository and it drifts. Applying it is [`apply_projection`].
///
/// It is idempotent, and it does **not** include the conformance suite's probe
/// read model — that table is the suite's, lives behind the `conformance`
/// feature, and has no business in an application's database.
#[cfg(feature = "projection-store")]
pub const MIGRATION_2: &str = include_str!("../migrations/0002_projection_checkpoint.sql");

/// The projection schema version this crate's code is written against.
///
/// One, and separate from [`SCHEMA_VERSION`] on purpose. The two roles share a
/// crate and a driver; they do not share a schema lineage. A consumer taking
/// `event-store` alone applies migration 1 and never this one, so a single
/// counter spanning both would report a version they are not at.
///
/// The file is named `0002_…` because that is the directory's sequence and a DBA
/// applying the files in order should get a working database for both roles. The
/// number in the filename and the number here answer different questions.
#[cfg(feature = "projection-store")]
pub const PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Applies [`MIGRATION_2`] to whatever schema `executor` resolves against.
///
/// Separate from [`apply`] rather than folded into it, and the separation is the
/// point: an application using only the event store gets no
/// `projection_checkpoint` table, and one using only the projection store gets no
/// event log. A single `apply` would decide for both.
///
/// It does not set `search_path`, for the reason [`apply`] does not.
///
/// # Errors
///
/// [`Driver`](crate::error::PostgresProjectionStoreError::Driver) if the server
/// rejects the statements
/// — most plausibly because the role cannot create objects in the target schema.
#[cfg(feature = "projection-store")]
pub async fn apply_projection<'e, E>(
    executor: E,
) -> Result<(), crate::error::PostgresProjectionStoreError>
where
    E: PgExecutor<'e>,
{
    // `raw_sql` for the reason `apply` uses it: the extended protocol admits one
    // statement, and a simple-protocol batch runs in an implicit transaction, so
    // a failure part-way leaves no half-built table behind.
    sqlx::raw_sql(MIGRATION_2).execute(executor).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "event-store")]
    use super::{MIGRATION_1, SCHEMA_VERSION};

    #[cfg(feature = "event-store")]
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

    #[cfg(feature = "event-store")]
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

    #[cfg(feature = "event-store")]
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

    #[cfg(feature = "event-store")]
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

    #[cfg(feature = "event-store")]
    /// The event log's lineage is at two once migration 3 is on it (#420).
    ///
    /// This read `the_schema_version_is_one_until_first_publish` and asserted
    /// one, under the licence to edit [`MIGRATION_1`] in place that ended at
    /// this crate's first publish. Write groups are the first change to the
    /// log after it, so they are a second migration and a second version,
    /// which is what the expectation now pins.
    #[test]
    fn the_schema_version_counts_the_write_group_migration() {
        assert_eq!(SCHEMA_VERSION, 2);
    }

    #[cfg(feature = "event-store")]
    /// [`MIGRATION_3`](super::MIGRATION_3) with its `--` line comments
    /// removed, as [`statements`] strips migration 1.
    fn migration_3_statements() -> String {
        super::MIGRATION_3
            .lines()
            .map(|line| line.split_once("--").map_or(line, |(code, _)| code))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[cfg(feature = "event-store")]
    /// Migration 3 adds both write-group columns and the partial index the
    /// group read looks rows up through, each `IF NOT EXISTS`, so applying it
    /// twice is a no-op as applying migration 1 twice is.
    #[test]
    fn migration_3_adds_the_write_group_columns_and_their_index() {
        let statements = migration_3_statements();
        assert!(
            !statements.contains("#420"),
            "prose survived the strip, so the assertions below are scanning comments"
        );
        for expected in [
            "ALTER TABLE event ADD COLUMN IF NOT EXISTS write_group bigint",
            "ALTER TABLE event ADD COLUMN IF NOT EXISTS group_guard bytea",
            "CREATE INDEX IF NOT EXISTS event_write_group_idx",
            "WHERE write_group IS NOT NULL",
        ] {
            assert!(
                statements.contains(expected),
                "migration 3 has lost `{expected}`"
            );
        }
        assert!(
            !statements.contains("DEFAULT")
                && !statements.contains("bigint NOT NULL")
                && !statements.contains("bytea NOT NULL"),
            "a default or a NOT NULL on a write-group column rewrites every row of an \
             existing table, and the legacy rule needs NULL"
        );
    }

    #[cfg(feature = "event-store")]
    /// The guards that scan migration 1 for a sequence default hold for
    /// migration 3 too: it never touches `position`.
    #[test]
    fn migration_3_puts_no_sequence_default_on_position() {
        let statements = migration_3_statements();
        for forbidden in ["bigserial", "serial", "AS IDENTITY", "position"] {
            assert!(
                !statements.contains(forbidden),
                "migration 3 mentions `{forbidden}`; it adds two columns and an index, \
                 and touches nothing migration 1 owns"
            );
        }
    }

    #[cfg(feature = "event-store")]
    /// `apply` sends migration 1 and then migration 3, in one batch, so one
    /// implicit transaction: a schema it reaches has both or neither.
    #[test]
    fn apply_sends_migration_1_then_migration_3() {
        let batch = super::EVENT_LOG;
        assert!(batch.starts_with(MIGRATION_1), "migration 1 goes first");
        assert!(batch.ends_with(super::MIGRATION_3), "migration 3 goes last");
        assert_eq!(
            batch.len(),
            MIGRATION_1.len() + 1 + super::MIGRATION_3.len(),
            "the batch is the two migrations and the line between them, nothing else"
        );
    }

    /// Migration 2 stores the authority rather than inferring it, and constrains
    /// it to the two `Authority` variants.
    ///
    /// The cheap half of a guard whose live half — the CHECK actually refusing a
    /// third value — is in the gated conformance target, where a real server can
    /// disagree with the text.
    #[cfg(feature = "projection-store")]
    #[test]
    fn migration_2_stores_a_constrained_authority() {
        let sql = super::MIGRATION_2;
        assert!(
            sql.contains(
                "authority     text   NOT NULL CHECK (authority IN ('live', 'rebuilding'))"
            ),
            "the authority column has lost its CHECK, so a row can record a state              `Checkpoint` cannot represent and `checkpoint` will fail on read              instead of the write failing where it happened"
        );
    }

    /// The suite's probe table is not in the migrations directory.
    ///
    /// A `.sql` file there is, by construction, something a DBA applies. This is
    /// the assertion that keeps the test read model out of an application's
    /// database — the failure `happenstance-sqlite`'s own comment names and that
    /// no test in this repository could otherwise catch, because every test
    /// enables the feature that creates it.
    #[cfg(feature = "projection-store")]
    #[test]
    fn no_migration_file_creates_the_probe_table() {
        for sql in [
            #[cfg(feature = "event-store")]
            MIGRATION_1,
            super::MIGRATION_2,
        ] {
            assert!(
                !sql.contains("projection_probe"),
                "a migration file creates the conformance probe table, so applying                  this crate's schema would put the suite's read model in a                  consumer's database"
            );
        }
    }
}
