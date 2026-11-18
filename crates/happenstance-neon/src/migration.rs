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

use crate::config::{NeonConfig, quote_ident, quote_literal};
use crate::transport::{IsolationLevel, SqlRequest, SqlStatement};

/// The event log, before its identifiers are filled in.
pub const MIGRATION_1: &str = include_str!("../migrations/0001_neon_log.sql");

/// The projection checkpoint, before its identifiers are filled in.
pub const MIGRATION_2: &str = include_str!("../migrations/0002_projection_checkpoint.sql");

/// Renders `template` against `config` and splits it into statements.
///
/// The placeholders are `@schema@`, `@event@`, `@sequence@`, `@meta@`,
/// `@checkpoint@`, `@removed@` and the index names — spelled `@…@`, not `{…}`,
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
    split(&rendered.replace("@removed@", &qualified_removed(config)))
}

/// One index name, quoted and **unqualified**.
///
/// `CREATE INDEX` rejects a schema-qualified index name — the index is created in
/// the table's schema and Postgres says so — which is the one place in this
/// module where qualifying everything would be wrong. Derived from the event
/// table's name so that two configurations in one schema do not collide; unique
/// per schema is enough, because the fixture isolates per schema.
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
/// [`MIGRATION_2`] belongs to the projection store. Declared below [`request`]
/// so the lines above keep the numbers the decision records cite.
pub const MIGRATION_3: &str = include_str!("../migrations/0003_write_group.sql");

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

/// The event log's removal record (#483, #484, ADR-0110), before its
/// identifiers are filled in: **the table only**.
///
/// One row per event removed from the event table, keyed by its origin pair,
/// written by the trigger [`removal_trigger_statements`] builds. An ingest
/// treats a recorded identity as one the store holds (SY-40), so a pruned
/// replica that re-pulls, or is re-pushed, its removed events skips them
/// instead of appending them again or refusing them as an unheld own identity.
///
/// The trigger's function has a `plpgsql` body, which [`render`]'s splitter
/// would cut in half, so it is not in this file: this module's doc names the
/// statement-per-constant builder as the honest move, and that is what
/// [`removal_trigger_statements`] is. Apply both in one batch, after
/// [`MIGRATION_1`] and [`MIGRATION_3`]:
///
/// ```
/// use happenstance_neon::NeonConfig;
/// use happenstance_neon::migration::{self, MIGRATION_4};
///
/// let config = NeonConfig::default().with_schema("app");
/// let mut request = migration::request(MIGRATION_4, &config);
/// request
///     .statements
///     .extend(migration::removal_trigger_statements(&config));
/// assert_eq!(request.statements.len(), 3, "the table, the function, the trigger");
/// ```
///
/// **Apply it before deploying this build**: every ingest this build sends
/// locks the record first, and fails against a schema without it. Append and
/// the reads do not touch it. Idempotent, as [`MIGRATION_1`] is. The table is
/// named after the event table, `{event_table}_removed`, so two configurations
/// in one schema do not share one.
///
/// A `TRUNCATE` fires no row trigger and records nothing, and a removal made
/// before this migration ran is not recorded either. The record grows by one
/// row per removal and is kept for ever: deleting a row from it lets that
/// identity be ingested again.
pub const MIGRATION_4: &str = include_str!("../migrations/0004_removal_record.sql");

/// The trigger that writes [`MIGRATION_4`]'s record, against `config`'s
/// tables, as two whole statements: the function, then a `DO` block that
/// creates the trigger only when the event table carries none of that name.
///
/// `BEFORE DELETE ... FOR EACH ROW`, because the deleting transaction must take
/// the record's lock at its first row, before that row is marked deleted: an
/// ingest holding `SHARE` on the record that meets the row under `ON CONFLICT`
/// then sees a live row and skips it, rather than waiting on a deleter that is
/// itself waiting on the ingest's lock. Guarded by the origin index's own
/// predicate, so a row with no origin pair records nothing, and `ON CONFLICT
/// DO NOTHING`, so an identity recorded once already does not fail the
/// `DELETE`. The function names the record schema-qualified, so it is correct
/// whatever `search_path` the deleting session has; the function itself is
/// named `{event_table}_record_removal`, in the configured schema, and so is
/// the trigger.
///
/// Each statement carries a dollar-quoted body and must be sent whole, as one
/// entry of a batch: never through [`render`]. [`MIGRATION_4`] shows the
/// batch.
#[must_use]
pub fn removal_trigger_statements(config: &NeonConfig) -> [SqlStatement; 2] {
    let event = config.qualified_event();
    let name = format!("{}_record_removal", config.event_table);
    let function = format!("{}.{}", quote_ident(&config.schema), quote_ident(&name));
    [
        SqlStatement::new(format!(
            "CREATE OR REPLACE FUNCTION {function}() RETURNS trigger \
             LANGUAGE plpgsql AS $record$ BEGIN \
             INSERT INTO {removed} (origin_store, origin_position) \
             VALUES (OLD.origin_store, OLD.origin_position) ON CONFLICT DO NOTHING; \
             RETURN OLD; END $record$",
            removed = qualified_removed(config),
        )),
        SqlStatement::new(format!(
            "DO $trigger$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_trigger \
             WHERE tgrelid = {event_literal}::regclass \
             AND tgname = {name_literal}) THEN \
             CREATE TRIGGER {trigger} BEFORE DELETE ON {event} \
             FOR EACH ROW \
             WHEN (OLD.origin_store IS NOT NULL AND OLD.origin_position IS NOT NULL) \
             EXECUTE FUNCTION {function}(); END IF; END $trigger$",
            event_literal = quote_literal(&event),
            name_literal = quote_literal(&name),
            trigger = quote_ident(&name),
        )),
    ]
}

/// [`MIGRATION_4`]'s record, quoted and schema-qualified:
/// `{event_table}_removed` in the configured schema, as the index names are
/// derived from the event table.
pub(crate) fn qualified_removed(config: &NeonConfig) -> String {
    format!(
        "{}.{}",
        quote_ident(&config.schema),
        quote_ident(&format!("{}_removed", config.event_table))
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
    use super::{
        MIGRATION_1, MIGRATION_2, MIGRATION_3, MIGRATION_4, qualified_removed,
        removal_trigger_statements, render, split,
    };
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
            ("0004", MIGRATION_4),
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

    /// Migration 4 renders into exactly the record's table, keyed by the
    /// origin pair, in the configured schema and named after the event table,
    /// with no placeholder left.
    #[test]
    fn the_removal_migration_renders_the_record_table() {
        let config = NeonConfig::default().with_schema("hs_4");
        assert_eq!(
            render(MIGRATION_4, &config),
            [
                "CREATE TABLE IF NOT EXISTS \"hs_4\".\"event_removed\" (\n    \
                 origin_store    bytea   NOT NULL,\n    origin_position bigint  NOT NULL,\n    \
                 PRIMARY KEY (origin_store, origin_position)\n)"
            ]
        );
        let renamed = config.with_event_table("ledger");
        assert_eq!(qualified_removed(&renamed), r#""hs_4"."ledger_removed""#);
    }

    /// The trigger that writes the record, as two whole statements: the
    /// function, whose `plpgsql` body inserts into the **qualified** record,
    /// and a `DO` block that creates the `BEFORE DELETE` row trigger only when
    /// the table carries none of that name. Each keeps its dollar-quoted body
    /// whole; the splitter would cut each into pieces, which is why they are
    /// statements and not lines of `migrations/`.
    ///
    /// Rejects an `AFTER` trigger, which deadlocks a removal against an ingest
    /// holding the record's lock; an unqualified record, which the endpoint
    /// has no `search_path` to resolve; and an unguarded or
    /// conflict-intolerant insert, which fails a `DELETE` of an unstamped row
    /// or of an identity recorded once already.
    #[test]
    fn the_trigger_statements_are_whole_and_qualified() {
        let config = NeonConfig::default().with_schema("hs_5");
        let [function, trigger] = removal_trigger_statements(&config);

        assert_eq!(
            function.query,
            "CREATE OR REPLACE FUNCTION \"hs_5\".\"event_record_removal\"() RETURNS trigger \
             LANGUAGE plpgsql AS $record$ BEGIN \
             INSERT INTO \"hs_5\".\"event_removed\" (origin_store, origin_position) \
             VALUES (OLD.origin_store, OLD.origin_position) ON CONFLICT DO NOTHING; \
             RETURN OLD; END $record$"
        );
        assert_eq!(
            trigger.query,
            "DO $trigger$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_trigger \
             WHERE tgrelid = '\"hs_5\".\"event\"'::regclass \
             AND tgname = 'event_record_removal') THEN \
             CREATE TRIGGER \"event_record_removal\" BEFORE DELETE ON \"hs_5\".\"event\" \
             FOR EACH ROW \
             WHEN (OLD.origin_store IS NOT NULL AND OLD.origin_position IS NOT NULL) \
             EXECUTE FUNCTION \"hs_5\".\"event_record_removal\"(); END IF; END $trigger$"
        );
        for statement in [&function, &trigger] {
            assert!(statement.params.is_empty());
            assert!(
                split(&statement.query).len() > 1,
                "the splitter would leave this whole, so it could live in migrations/"
            );
        }
    }

    /// A hostile table name stays one identifier in every place the trigger
    /// names it, including inside the `regclass` literal and the `tgname`
    /// comparison.
    #[test]
    fn the_trigger_statements_quote_a_hostile_table_name() {
        let config = NeonConfig::default()
            .with_schema("s")
            .with_event_table(r#"e"'x"#);
        let [function, trigger] = removal_trigger_statements(&config);

        assert!(
            function
                .query
                .contains(r#"INSERT INTO "s"."e""'x_removed" (origin_store"#),
            "{}",
            function.query
        );
        assert!(
            trigger.query.contains(
                r#"WHERE tgrelid = '"s"."e""''x"'::regclass AND tgname = 'e"''x_record_removal')"#
            ),
            "{}",
            trigger.query
        );
        assert!(
            trigger
                .query
                .contains(r#"CREATE TRIGGER "e""'x_record_removal" BEFORE DELETE ON "s"."e""'x" "#),
            "{}",
            trigger.query
        );
    }

    #[test]
    fn splitting_drops_comments_and_empty_tails() {
        let split = split("-- a comment\nSELECT 1;\n\n-- another\nSELECT 2;\n");
        assert_eq!(split, vec!["SELECT 1".to_owned(), "SELECT 2".to_owned()]);
    }
}
