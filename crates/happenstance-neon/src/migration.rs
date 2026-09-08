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

/// Renders `template` against `config` and splits it into statements.
///
/// The placeholders are `@schema@`, `@event@`, `@sequence@`, `@meta@`,
/// `@checkpoint@` and the three index names — spelled `@…@` rather than `{…}`
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
        .replace("@xact_idx@", &index_name(config, "xact_idx"));
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
    use super::{MIGRATION_1, MIGRATION_2, render, split};
    use crate::config::NeonConfig;

    /// The splitter's own precondition, asserted rather than assumed.
    ///
    /// It is not a SQL parser, and the two things that would make it silently
    /// wrong are a dollar-quoted body and a semicolon inside a string literal.
    /// Neither is in `migrations/`; this is what says so on the commit that adds
    /// one.
    #[test]
    fn no_migration_contains_what_the_splitter_cannot_see() {
        for (name, sql) in [("0001", MIGRATION_1), ("0002", MIGRATION_2)] {
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

    #[test]
    fn splitting_drops_comments_and_empty_tails() {
        let split = split("-- a comment\nSELECT 1;\n\n-- another\nSELECT 2;\n");
        assert_eq!(split, vec!["SELECT 1".to_owned(), "SELECT 2".to_owned()]);
    }
}
