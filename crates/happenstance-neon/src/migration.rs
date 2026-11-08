//! The schema, rendered for one [`NeonConfig`] and split into statements.
//!
//! # Why this module exists at all, when `happenstance-postgres` has no analogue
//!
//! That adapter compiles its migration in with `include_str!` and hands the
//! whole string to one `execute`. Neither half of that works here.
//!
//! * **The text cannot be constant.** Its identifiers are not known until a
//!   [`NeonConfig`] exists, because there is no `search_path` to place an
//!   unqualified name with: the `/sql` proxy discards `options=-c
//!   search_path=…` from the connection string, and there is no session a `SET`
//!   could live in. So the file carries `@…@` placeholders and this module
//!   renders them from the config's own already-quoted, already-qualified
//!   identifiers.
//! * **The whole file cannot be one statement.** The endpoint's `query` field
//!   takes exactly one; a multi-statement string is refused with
//!   `Database request failed` and no `code`, `detail` or `position` — measured,
//!   and it is the least diagnosable error this endpoint produces. So the file
//!   is split here and sent as one non-interactive **batch**, which is still one
//!   round trip and still one `BEGIN`/`COMMIT`.
//!
//! # The splitter is deliberately small, and the file is written for it
//!
//! It strips whole-line `--` comments and then splits on `;`. That is not a SQL
//! parser and must not become one: it is correct exactly while no statement in
//! `migrations/` contains a dollar-quoted body or a semicolon inside a literal,
//! and this module's own tests assert both. The moment one does, the honest move is a
//! statement-per-constant module rather than a smarter regex — a splitter that
//! is *nearly* a parser is the shape that silently cuts a `plpgsql` body in half.
//!
//! The fixture's fault injections are `plpgsql`, and they are deliberately not
//! in `migrations/` for that reason: they are built as separate
//! [`SqlStatement`]s at the call site, where no splitting
//! happens.

use crate::config::{NeonConfig, quote_ident};
use crate::transport::{IsolationLevel, SqlRequest, SqlStatement};

/// The event log, before its identifiers are filled in.
pub const MIGRATION_1: &str = include_str!("../migrations/0001_neon_log.sql");

/// The projection checkpoint, before its identifiers are filled in.
pub const MIGRATION_2: &str = include_str!("../migrations/0002_projection_checkpoint.sql");

/// The event log's write-group columns and their index (#420), before their
/// identifiers are filled in.
///
/// `write_group`, on every row, the lowest position the write that put the
/// row in gave out, and `group_guard`, on that first row only, the guard an
/// ingested group arrived under. A runner and a store peer read the log back
/// in whole writes through them (SY-39). Both nullable with no default, so
/// adding them rewrites no row, and a row from before this migration reads as
/// a group of its own.
///
/// **Apply it after [`MIGRATION_1`], before deploying this build**, with
/// [`request`]: every `INSERT` this build sends names `write_group`, and fails
/// against a table without it. Idempotent, as [`MIGRATION_1`] is, so applying
/// both to any schema, fresh or migrated by an older build, brings it to what
/// this build expects. Numbered 3 because that is the directory's sequence;
/// [`MIGRATION_2`] belongs to the projection store.
pub const MIGRATION_3: &str = include_str!("../migrations/0003_write_group.sql");

/// Renders `template` against `config` and splits it into statements.
///
/// The placeholders are `@schema@`, `@event@`, `@sequence@`, `@meta@`,
/// `@checkpoint@` and the index names — spelled `@…@` rather than `{…}`
/// so that a `format!`-shaped brace appearing in SQL some day is not a rendering
/// bug, and so that an unsubstituted placeholder is a syntax error at the server
/// rather than a plausible identifier.
#[must_use]
pub fn render(template: &str, config: &NeonConfig) -> Vec<String> {
    let rendered = template
        .replace("@schema@", &quote_ident(&config.schema))
        .replace("@event@", &config.qualified_event())
        .replace("@sequence@", &config.qualified_sequence())
        .replace("@meta@", &config.qualified_meta())
        .replace("@checkpoint@", &config.qualified_checkpoint())
        .replace("@tags_idx@", &index_name(config, "tags_idx"))
        .replace("@type_idx@", &index_name(config, "type_idx"))
        .replace("@origin_idx@", &index_name(config, "origin_idx"))
        .replace("@xact_idx@", &index_name(config, "xact_idx"))
        .replace("@write_group_idx@", &index_name(config, "write_group_idx"));
    split(&rendered)
}

/// One index name, quoted and **unqualified**.
///
/// `CREATE INDEX` rejects a schema-qualified index name — the index is created in
/// the table's schema and Postgres says so — which is the one place in this
/// module where qualifying everything would be wrong. Derived from the event
/// table's name so that two configurations in one schema do not collide, and
/// unique per schema is all that is needed because the fixture's isolation is
/// per schema.
fn index_name(config: &NeonConfig, suffix: &str) -> String {
    quote_ident(&format!("{}_{suffix}", config.event_table))
}

/// The whole of a rendered migration, as one non-interactive transaction.
///
/// `ReadCommitted` rather than the config's level, and deliberately: DDL under
/// `SERIALIZABLE` takes predicate locks it has no use for, and a migration
/// applied twice concurrently by two fixture instances is already made safe by
/// `IF NOT EXISTS` rather than by isolation.
#[must_use]
pub fn request(template: &str, config: &NeonConfig) -> SqlRequest {
    SqlRequest::batch(
        render(template, config)
            .into_iter()
            .map(SqlStatement::new)
            .collect(),
        IsolationLevel::ReadCommitted,
    )
}

/// The re-mint, against `config`'s meta table: the one statement that replaces
/// this store's identity with a freshly minted one, and reads the new value
/// back as base64.
///
/// # Who runs it, and when
///
/// After a Neon branch is created, reset or restored, and after a
/// `pg_restore` or a point-in-time recovery: stop every writer to the copy,
/// run this against it, then resume appends. The crate root's *After a branch
/// or a restore, re-mint before the first append* is the procedure in full.
/// `NeonEventStore::remint_identity` sends exactly this statement. It is published on its own because a branch
/// is made by the platform's control plane, not by this crate, so the tooling
/// that makes branches is where the re-mint has to run, and that tooling needs
/// the text rather than a store. [`SqlStatement::query`] is plain SQL with no
/// placeholder and no parameter, so `psql`, a migration runner or a branch hook
/// can run it as it stands; the `RETURNING` clause only prints the new value.
///
/// It is migration 1's insert turned into an update. An `UPDATE` rather than
/// an upsert, because a schema with no identity row is one migration 1 never
/// ran on, and minting into it would hide that: the statement then matches no
/// row and returns none. A later run of migration 1 cannot undo it, because
/// that insert is `ON CONFLICT (k) DO NOTHING`.
#[must_use]
pub fn remint_statement(config: &NeonConfig) -> SqlStatement {
    SqlStatement::new(format!(
        "UPDATE {meta} SET v = uuid_send(gen_random_uuid()) WHERE k = 'store_id' \
         RETURNING encode(v, 'base64') AS store_id",
        meta = config.qualified_meta(),
    ))
}

/// Strips whole-line comments and splits on `;`.
fn split(sql: &str) -> Vec<String> {
    let stripped: String = sql
        .lines()
        .filter(|line| !line.trim_start().starts_with("--"))
        .collect::<Vec<_>>()
        .join("\n");

    stripped
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{MIGRATION_1, MIGRATION_2, MIGRATION_3, render, split};
    use crate::config::NeonConfig;

    /// The splitter's own precondition, asserted rather than assumed.
    ///
    /// It is not a SQL parser, and the two things that would make it silently
    /// wrong are a dollar-quoted body and a semicolon inside a string literal.
    /// Neither is in `migrations/`; this is what says so on the commit that adds
    /// one.
    #[test]
    fn no_migration_contains_what_the_splitter_cannot_see() {
        for (name, sql) in [
            ("0001", MIGRATION_1),
            ("0002", MIGRATION_2),
            ("0003", MIGRATION_3),
        ] {
            // Comments are stripped before the split, so only executable lines
            // constrain it — which is why migration 1's own header may say
            // `"$user"` and `bigserial` without tripping this.
            for line in sql
                .lines()
                .filter(|line| !line.trim_start().starts_with("--"))
            {
                assert!(
                    !line.contains('$'),
                    "migration {name} has acquired a dollar quote the splitter would cut \
                     in half; move it to a statement constant instead: {line}"
                );
                let quotes = line.matches('\'').count();
                assert_eq!(
                    quotes % 2,
                    0,
                    "migration {name} has a string literal spanning a line break, which \
                     the splitter cannot see the end of: {line}"
                );
            }
        }
    }

    #[test]
    fn every_statement_is_rendered_into_the_configured_schema() {
        let config = NeonConfig::default().with_schema("hs_1");
        let statements = render(MIGRATION_1, &config);
        assert_eq!(
            statements.len(),
            9,
            "the schema, the table, the sequence, four indexes, the meta table \
             and its one row"
        );
        for statement in &statements {
            assert!(
                !statement.contains('@'),
                "an unsubstituted placeholder survived rendering: {statement}"
            );
        }
        assert!(statements.iter().any(|s| s.contains(r#""hs_1"."event""#)));
        assert!(
            statements
                .iter()
                .any(|s| s.contains(r#""hs_1"."event_position_seq""#))
        );
        assert!(
            statements
                .iter()
                .any(|s| s.contains(r#""hs_1"."store_meta""#))
        );
    }

    /// The whole point of migration 1 not being `bigserial`, checked without a
    /// server.
    ///
    /// The half only a server can answer lives in the conformance target; this
    /// is the text scan, and it is cheap enough to run in the default gate.
    #[test]
    fn position_carries_no_column_default() {
        assert!(
            MIGRATION_1.contains("position        bigint  PRIMARY KEY"),
            "`position` has acquired a default or an identity clause. A default is \
             `nextval()`, and `nextval()` allocating outside the transaction is where \
             ES-10 is lost — which is what the frontier predicate exists to answer \
             deliberately"
        );
        assert!(
            !MIGRATION_1
                .lines()
                .any(|line| !line.trim_start().starts_with("--") && line.contains("bigserial")),
            "an executable line says `bigserial`; the header may explain why it does not"
        );
    }

    #[test]
    fn the_projection_migration_names_only_the_checkpoint_table() {
        let config = NeonConfig::default().with_schema("hsp_1");
        let statements = render(MIGRATION_2, &config);
        assert_eq!(statements.len(), 2, "the schema and the checkpoint table");
        assert!(
            statements
                .iter()
                .any(|s| s.contains(r#""hsp_1"."projection_checkpoint""#))
        );
    }

    /// Migration 3 renders into exactly its three statements, both columns and
    /// the partial index, into the configured schema, with no placeholder left:
    /// an unsubstituted `@write_group_idx@` is a syntax error at the server,
    /// which this finds without one.
    #[test]
    fn the_write_group_migration_renders_into_three_statements() {
        let config = NeonConfig::default().with_schema("hs_3");
        let statements = render(MIGRATION_3, &config);

        assert_eq!(
            statements,
            [
                r#"ALTER TABLE "hs_3"."event" ADD COLUMN IF NOT EXISTS write_group bigint"#,
                r#"ALTER TABLE "hs_3"."event" ADD COLUMN IF NOT EXISTS group_guard bytea"#,
                "CREATE INDEX IF NOT EXISTS \"event_write_group_idx\"\n    ON \"hs_3\".\"event\" \
                 (write_group)\n    WHERE write_group IS NOT NULL",
            ]
        );
    }

    #[test]
    fn splitting_drops_comments_and_empty_tails() {
        let split = split("-- a comment\nSELECT 1;\n\n-- another\nSELECT 2;\n");
        assert_eq!(split, vec!["SELECT 1".to_owned(), "SELECT 2".to_owned()]);
    }
}
