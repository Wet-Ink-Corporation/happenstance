//! Where the tables live, and how much of a response the adapter will accept.

use crate::transport::{IsolationLevel, MAX_RESPONSE_BYTES};

/// Configuration shared by both Neon-backed stores.
///
/// # Every name here reaches SQL as an identifier, not as a parameter
///
/// The `/sql` endpoint binds *values*. A table name is not a value, so every
/// field below is interpolated into the statement text — which makes a bad one an
/// injection rather than a type error. Two shapes were available and the weaker
/// one is the obvious one.
///
/// **Rejected: validate at construction.** A `with_event_table` returning
/// `Result`, or panicking, on anything outside `[A-Za-z0-9_]`. It reads like a
/// gate and is not one: this struct's fields are `pub`, so a caller can assign
/// past every setter, and `#[non_exhaustive]` stops *construction* from outside
/// the crate rather than assignment. A guard a caller can walk around is a guard
/// that will be walked around, and the walk is silent.
///
/// **Chosen: escape at emission.** [`quote_ident`] wraps every name in double
/// quotes and doubles any embedded double quote, which is Postgres' own quoting
/// rule and is *total* — there is no identifier it fails to neutralise, and it
/// does not have to be reached through a particular door. A table name of
/// `a"; DROP TABLE victim; --` becomes one harmless, entirely unusable
/// identifier, and the statement fails at name resolution rather than executing
/// anything. The unit tests at the bottom of this file are that claim, written
/// out.
///
/// The cost is real and is worth stating: quoting makes every name
/// **case-sensitive**, so `with_event_table("Event")` names a different table
/// from the default `event`, exactly as it would in hand-written SQL. That is
/// Postgres' rule rather than this crate's, and it is the price of not having a
/// name that can escape.
///
/// # Why a `schema` at all
///
/// The endpoint ignores `options=-c search_path=…` in the connection string —
/// measured, `SHOW search_path` still answers `"$user", public` — and there is no
/// session to hold a `SET` in. So an unqualified name is resolved against
/// whatever the proxy's backend happens to have, which is not a property this
/// adapter can own. Every statement it emits is schema-qualified, and the
/// fixture's one-schema-per-instance isolation is bought with this field and
/// nothing else.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct NeonConfig {
    /// The schema every table below lives in.
    ///
    /// Defaults to `public`. Qualifying rather than relying on a `search_path`
    /// is not a preference: the `/sql` proxy discards the connection string's
    /// `options` parameter and holds no session, so there is no `search_path`
    /// this adapter could set and rely on.
    pub schema: Box<str>,
    /// The table events are written to.
    pub event_table: Box<str>,
    /// The sequence positions are drawn from.
    ///
    /// Named separately rather than derived as `{event_table}_position_seq`,
    /// because the derivation would be a second naming rule living in a `format!`
    /// where nobody looks for one — and because `position` deliberately carries
    /// no column default, so the sequence is read *explicitly* at the one
    /// `INSERT` that allocates and its name is therefore a thing this crate says
    /// out loud rather than a thing Postgres remembers.
    pub sequence: Box<str>,
    /// The one-row table holding this store's
    /// [`StoreId`](happenstance_core::StoreId).
    pub meta_table: Box<str>,
    /// The table projection checkpoints are written to.
    pub checkpoint_table: Box<str>,
    /// The isolation level for the batched, non-interactive transaction.
    ///
    /// Defaults to [`IsolationLevel::Serializable`], which is *not* Postgres'
    /// default. At `ReadCommitted` a conditional insert can miss a conflict a
    /// concurrent transaction committed after this one took its snapshot, and
    /// with no interactive transaction there is no second look to catch it. The
    /// price is SQLSTATE `40001` aborts under contention, which the caller sees
    /// as [`NeonSqlError::is_serialization_failure`](crate::NeonSqlError::is_serialization_failure).
    ///
    /// It applies only to a request carrying **two or more** statements, because
    /// that is the only form the endpoint honours the header on — measured, and
    /// the reason a conditional append is a two-statement batch rather than the
    /// single CTE this crate used to document.
    pub isolation: IsolationLevel,
    /// The response-size ceiling, at most [`MAX_RESPONSE_BYTES`].
    pub max_response_bytes: usize,
}

impl Default for NeonConfig {
    fn default() -> Self {
        Self {
            schema: "public".into(),
            event_table: "event".into(),
            sequence: "event_position_seq".into(),
            meta_table: "store_meta".into(),
            checkpoint_table: "projection_checkpoint".into(),
            isolation: IsolationLevel::Serializable,
            max_response_bytes: MAX_RESPONSE_BYTES,
        }
    }
}

impl NeonConfig {
    /// Overrides the schema every table is qualified by.
    #[must_use]
    pub fn with_schema(mut self, schema: impl Into<Box<str>>) -> Self {
        self.schema = schema.into();
        self
    }

    /// Overrides the event table name.
    #[must_use]
    pub fn with_event_table(mut self, table: impl Into<Box<str>>) -> Self {
        self.event_table = table.into();
        self
    }

    /// Overrides the position sequence name.
    #[must_use]
    pub fn with_sequence(mut self, sequence: impl Into<Box<str>>) -> Self {
        self.sequence = sequence.into();
        self
    }

    /// Overrides the store-identity table name.
    #[must_use]
    pub fn with_meta_table(mut self, table: impl Into<Box<str>>) -> Self {
        self.meta_table = table.into();
        self
    }

    /// Overrides the checkpoint table name.
    #[must_use]
    pub fn with_checkpoint_table(mut self, table: impl Into<Box<str>>) -> Self {
        self.checkpoint_table = table.into();
        self
    }

    /// Overrides the isolation level for batched requests.
    #[must_use]
    pub const fn with_isolation(mut self, isolation: IsolationLevel) -> Self {
        self.isolation = isolation;
        self
    }

    /// Lowers the response ceiling below [`MAX_RESPONSE_BYTES`].
    ///
    /// Raising it above has no effect on the endpoint, which enforces its own
    /// cap; the value is clamped so the adapter's error and the endpoint's
    /// behaviour cannot disagree.
    #[must_use]
    pub const fn with_max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = if bytes < MAX_RESPONSE_BYTES {
            bytes
        } else {
            MAX_RESPONSE_BYTES
        };
        self
    }

    /// The event table, quoted and schema-qualified.
    #[must_use]
    pub fn qualified_event(&self) -> String {
        qualify(&self.schema, &self.event_table)
    }

    /// The store-identity table, quoted and schema-qualified.
    #[must_use]
    pub fn qualified_meta(&self) -> String {
        qualify(&self.schema, &self.meta_table)
    }

    /// The checkpoint table, quoted and schema-qualified.
    #[must_use]
    pub fn qualified_checkpoint(&self) -> String {
        qualify(&self.schema, &self.checkpoint_table)
    }

    /// The sequence, quoted and schema-qualified.
    #[must_use]
    pub fn qualified_sequence(&self) -> String {
        qualify(&self.schema, &self.sequence)
    }

    /// The sequence as the SQL **string literal** `nextval` takes.
    ///
    /// `nextval` accepts a `regclass`, and the way to name a quoted, qualified
    /// sequence to it is a string literal holding the quoted, qualified name —
    /// so the identifier's own quotes end up *inside* a literal and the literal
    /// needs its own escaping. Two escaping rules on one value is precisely
    /// where an injection hides, which is why this is one function with one test
    /// rather than a `format!` at the call site.
    ///
    /// A bound `$n` parameter was the alternative and it works — the endpoint
    /// casts a text parameter to `regclass` happily. It lost on placeholder
    /// arithmetic: the append's statement already numbers its guard predicate
    /// from a running index, and threading one more slot through that for a
    /// value that is never caller-supplied buys nothing the escaping below does
    /// not.
    #[must_use]
    pub fn sequence_literal(&self) -> String {
        quote_literal(&self.qualified_sequence())
    }
}

/// `"schema"."name"`, each half escaped.
fn qualify(schema: &str, name: &str) -> String {
    format!("{}.{}", quote_ident(schema), quote_ident(name))
}

/// One Postgres identifier, quoted so that nothing inside it can be syntax.
///
/// The whole of this crate's defence against a hostile table name. Postgres'
/// rule is that a double-quoted identifier ends at the next unpaired `"`, so
/// doubling every `"` in the input makes the result exactly one identifier
/// whatever the input was.
///
/// A NUL is the one byte this cannot neutralise, and it does not have to:
/// Postgres rejects it in an identifier, so the statement fails at parse with
/// the endpoint's own error rather than executing something else.
#[must_use]
pub fn quote_ident(name: &str) -> String {
    let mut quoted = String::with_capacity(name.len() + 2);
    quoted.push('"');
    for character in name.chars() {
        if character == '"' {
            quoted.push('"');
        }
        quoted.push(character);
    }
    quoted.push('"');
    quoted
}

/// One Postgres string literal, quoted so that nothing inside it can be syntax.
///
/// [`quote_ident`]'s sibling, and it exists for exactly one caller —
/// [`NeonConfig::sequence_literal`], where a quoted identifier has to travel
/// *inside* a literal.
#[must_use]
pub fn quote_literal(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('\'');
    for character in value.chars() {
        if character == '\'' {
            quoted.push('\'');
        }
        quoted.push(character);
    }
    quoted.push('\'');
    quoted
}

#[cfg(test)]
mod tests {
    use super::{MAX_RESPONSE_BYTES, NeonConfig, quote_ident, quote_literal};

    #[test]
    fn the_default_is_the_public_schema() {
        let config = NeonConfig::default();
        assert_eq!(config.qualified_event(), r#""public"."event""#);
        assert_eq!(
            config.qualified_checkpoint(),
            r#""public"."projection_checkpoint""#
        );
        assert_eq!(
            config.sequence_literal(),
            r#"'"public"."event_position_seq"'"#
        );
    }

    /// The injection that would work against an unquoted name, and does not.
    ///
    /// Worth writing out rather than asserting the escaping rule in the
    /// abstract: the value below is the one a reader would try, and the assertion
    /// is that it comes back as a single identifier with no `;` outside it.
    #[test]
    fn a_hostile_table_name_is_one_identifier_and_not_two_statements() {
        let config = NeonConfig::default().with_event_table(r#"a"; DROP TABLE victim; --"#);
        let qualified = config.qualified_event();
        assert_eq!(qualified, r#""public"."a""; DROP TABLE victim; --""#);
        // The closing quote is the last character, so everything the attacker
        // supplied is inside one identifier. Counting quotes is what says so: an
        // odd count would mean the identifier ended where the attacker chose.
        assert_eq!(
            qualified.matches('"').count() % 2,
            0,
            "an unpaired quote is an identifier that ended where the attacker chose"
        );
    }

    /// The two escaping rules compose, which is the thing one function exists to
    /// guarantee.
    #[test]
    fn a_quote_survives_both_layers_of_the_sequence_literal() {
        let config = NeonConfig::default().with_sequence("se'q\"n");
        assert_eq!(config.sequence_literal(), "'\"public\".\"se''q\"\"n\"'");
    }

    #[test]
    fn quoting_is_total_over_its_own_metacharacter() {
        assert_eq!(quote_ident("\"\""), "\"\"\"\"\"\"");
        assert_eq!(quote_literal("''"), "''''''");
    }

    #[test]
    fn the_response_ceiling_cannot_be_raised_above_the_endpoints() {
        let config = NeonConfig::default().with_max_response_bytes(usize::MAX);
        assert_eq!(config.max_response_bytes, MAX_RESPONSE_BYTES);
    }
}
