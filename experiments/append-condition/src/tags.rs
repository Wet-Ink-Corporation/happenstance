//! The three tag storages, and the SQL each one produces for a query item.
//!
//! `crates/happenstance-sqlite/src/lib.rs:63-65` names exactly three: a join
//! table, a canonical serialised blob, and SQLite's JSON1. All three are
//! measured. The blob arm in particular is measured rather than dismissed,
//! because `Tags` is canonically sorted *precisely so* that it stays open, and
//! discarding it on taste would contradict the reason that sorting exists.
//!
//! # What each arm has to answer, and where it is asked
//!
//! One question: **which positions match this query item?** A [`QueryItem`] is
//! types OR'd within the item and tags AND'd within the item, and an event
//! matches when it carries *at least* the item's tags — a superset, not an
//! equality. Across items a [`Query`] is a union.
//!
//! It is asked in two places and the second is the one that decides the
//! measurement: on the read path, where the cost is per replayed event, and
//! **inside the append transaction**, where the probe runs while the write lock
//! is held and every other writer is waiting behind it.
//!
//! # The schema correction this file carries
//!
//! `crates/happenstance-sqlite/src/event_store.rs:36-54` sketches
//! `event_tag(tag, position)` with **no type column**. That published sketch is
//! wrong in a way that serialises every writer: a query item constraining both
//! type and tags becomes a join back to `event`, walked under the
//! `BEGIN IMMEDIATE` write lock. [`JoinTable`] carries `event_type` as a
//! **covering column** with the key left `(tag, position)`, so the range stays
//! sorted by position and the type constraint is answered without leaving the
//! index.

use happenstance_core::{Query, QueryItem, Tags};
use rusqlite::Connection;
use rusqlite::types::Value;

/// The byte that separates one encoded tag from the next in [`CanonicalBlob`].
///
/// `0x1F` is safe as a delimiter rather than merely convenient: VT-14's shared
/// validator rejects every character in Unicode general category `Cc` from a
/// `Tag`, and `0x1F` is one of them, so no tag can contain the byte that
/// separates tags. A delimiter a value can contain is how a canonical encoding
/// becomes a matching bug.
const UNIT: char = '\u{1f}';

/// How one arm stores tags and answers "which positions match this item".
///
/// The trait is the *axis*, and the three implementations are its ends. Nothing
/// else in this crate varies with it: one schema for `event`, one read path, one
/// identity story.
pub trait TagStorage {
    /// How the arm names itself in a results table.
    const NAME: &'static str;

    /// Creates whatever this arm needs beyond the `event` table.
    ///
    /// Idempotent — `SqliteEventStore::open` calls `migrate` on every connect
    /// and a fixture connects more than once, so `IF NOT EXISTS` is a
    /// correctness requirement rather than tidiness.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if a statement fails.
    fn migrate(connection: &Connection) -> rusqlite::Result<()>;

    /// Writes the per-event rows this arm needs, if any.
    ///
    /// Called inside the append transaction, immediately after the `event` row
    /// is inserted.
    ///
    /// # Errors
    ///
    /// Returns the driver's error if a statement fails.
    fn write_tags(
        connection: &Connection,
        position: i64,
        event_type: &str,
        tags: &Tags,
    ) -> rusqlite::Result<()>;

    /// SQL selecting the positions matching `item`, pushing its parameters onto
    /// `params`.
    ///
    /// The returned string is a complete `SELECT position …` and is composed
    /// into a `UNION` across the query's items by [`match_sql`].
    fn item_sql(item: &QueryItem, params: &mut Vec<Value>) -> String;
}

/// SQL selecting every position matching `query`, with its parameters.
///
/// `Query::all` short-circuits to the `event` table rather than going through
/// an arm, which is not an optimisation but the definition: `all` matches every
/// event including an untagged one, and an untagged event has no row in a join
/// table at all.
///
/// Items are combined with `UNION` rather than `UNION ALL`, which is what makes
/// `duplicate_items_do_not_duplicate_events` pass by construction rather than by
/// a `DISTINCT` bolted on afterwards.
pub fn match_sql<T: TagStorage>(query: &Query, params: &mut Vec<Value>) -> String {
    match query.items() {
        None => "SELECT position FROM event".to_owned(),
        Some(items) => {
            let arms: Vec<String> = items.iter().map(|item| T::item_sql(item, params)).collect();
            // An empty item list cannot be constructed — `Query::from_items`
            // refuses it, and `Query::Items` is `#[non_exhaustive]` downstream
            // so nobody else can build one — but a `SELECT` with no arms would
            // be a syntax error rather than an empty result, so it is spelled
            // rather than assumed.
            if arms.is_empty() {
                "SELECT position FROM event WHERE 0".to_owned()
            } else {
                arms.join(" UNION ")
            }
        }
    }
}

/// `?,?,?` for `n` bound parameters.
fn placeholders(n: usize) -> String {
    let mut out = String::with_capacity(n * 2);
    for i in 0..n {
        if i > 0 {
            out.push(',');
        }
        out.push('?');
    }
    out
}

/// The item's tags, deduplicated, so that a `HAVING COUNT(DISTINCT tag)` bound
/// is the number of *distinct* tags the item asks for.
fn distinct_tags(tags: &Tags) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(tags.len());
    for tag in tags.iter() {
        let value = tag.as_str().to_owned();
        if !out.contains(&value) {
            out.push(value);
        }
    }
    out
}

/// A join table keyed `(tag, position)`, with `event_type` as a covering
/// column.
///
/// The architecture brief's recommendation, and the arm the wrong sketch was
/// reaching for. The key order is `(tag, position)` and stays that way: it is
/// what keeps a tag's positions contiguous and already sorted, so the probe
/// inside the write transaction is a range scan rather than a sort.
///
/// `event_type` is carried **as a covering column and not as part of the key**.
/// In the key it would break the position ordering; absent altogether — which
/// is what `event_store.rs:36-54` publishes today — a type-and-tag item becomes
/// a join back to `event`, walked while the write lock is held.
#[derive(Debug, Clone, Copy)]
pub struct JoinTable;

impl TagStorage for JoinTable {
    const NAME: &'static str = "join-table";

    fn migrate(connection: &Connection) -> rusqlite::Result<()> {
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS event_tag (
                 tag        TEXT    NOT NULL,
                 position   INTEGER NOT NULL REFERENCES event(position),
                 event_type TEXT    NOT NULL,
                 PRIMARY KEY (tag, position)
             ) WITHOUT ROWID;",
        )
    }

    fn write_tags(
        connection: &Connection,
        position: i64,
        event_type: &str,
        tags: &Tags,
    ) -> rusqlite::Result<()> {
        if tags.is_empty() {
            return Ok(());
        }
        let mut statement = connection
            .prepare("INSERT INTO event_tag (tag, position, event_type) VALUES (?, ?, ?)")?;
        for tag in tags.iter() {
            statement.execute(rusqlite::params![tag.as_str(), position, event_type])?;
        }
        Ok(())
    }

    fn item_sql(item: &QueryItem, params: &mut Vec<Value>) -> String {
        let tags = distinct_tags(item.tags());
        let types = item.types();

        if tags.is_empty() {
            // No tags to intersect, so the join table has nothing to say and an
            // untagged event has no row in it. This arm goes to `event`.
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            return format!(
                "SELECT position FROM event WHERE event_type IN ({})",
                placeholders(types.len())
            );
        }

        for tag in &tags {
            params.push(Value::Text(tag.clone()));
        }
        let type_clause = if types.is_empty() {
            String::new()
        } else {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            format!(" AND event_type IN ({})", placeholders(types.len()))
        };

        // **The single-tag fast path, and it is a measured correction rather
        // than a micro-optimisation.**
        //
        // The general form below groups by position and requires every one of
        // the item's tags. A `GROUP BY` is an optimisation barrier: SQLite
        // cannot push the enclosing `position > ?` boundary — the one an append
        // condition's guard always carries — through an aggregate, so the probe
        // materialises *every* matching position in the log and then discards
        // all of them below the boundary. Measured against its own negative
        // control, `JoinTableGrouped`, in one interleaved run at 50,000 events:
        // 556 us of probe with this path against 1,093 us without it, a factor
        // of 1.97 (`results/tag-storage.md`).
        //
        // With exactly one tag the aggregate says nothing: "carries at least
        // this one tag" *is* membership. Dropping it restores pushdown, and the
        // single-tag item is the overwhelmingly common shape of a consistency
        // boundary. The multi-tag form below is what `tag_cardinality` exists
        // for and is measured at ~65 ms per guard evaluation over a 50,000-event
        // log: probing most-selective-tag-first is a requirement rather than a
        // tuning knob.
        if tags.len() == 1 {
            return format!("SELECT position FROM event_tag WHERE tag = ?{type_clause}");
        }

        // `COUNT(DISTINCT tag) = n` is the superset test: an event matches when
        // it carries *at least* the item's tags, so counting how many of the
        // item's tags it carries and requiring all of them is exactly
        // `Tags::contains_all` pushed into SQL.
        format!(
            "SELECT position FROM event_tag WHERE tag IN ({}){} \
             GROUP BY position HAVING COUNT(DISTINCT tag) = {}",
            placeholders(tags.len()),
            type_clause,
            tags.len()
        )
    }
}

/// [`JoinTable`] with the single-tag fast path removed.
///
/// Not a fourth candidate — the record closes the list of three at
/// `crates/happenstance-sqlite/src/lib.rs:63-65` and does not extend it. This
/// is the **negative control** for the fast path: the same table, the same
/// rows, the same index, always taking the `GROUP BY … HAVING COUNT(DISTINCT
/// tag)` form. Measuring it beside `JoinTable` in one interleaved run is what
/// turns "the aggregate blocks predicate pushdown" from a plausible reading of
/// a query plan into a figure, on a host where two runs an hour apart disagree
/// by 45%.
#[derive(Debug, Clone, Copy)]
pub struct JoinTableGrouped;

impl TagStorage for JoinTableGrouped {
    const NAME: &'static str = "join-table-grouped";

    fn migrate(connection: &Connection) -> rusqlite::Result<()> {
        JoinTable::migrate(connection)
    }

    fn write_tags(
        connection: &Connection,
        position: i64,
        event_type: &str,
        tags: &Tags,
    ) -> rusqlite::Result<()> {
        JoinTable::write_tags(connection, position, event_type, tags)
    }

    fn item_sql(item: &QueryItem, params: &mut Vec<Value>) -> String {
        let tags = distinct_tags(item.tags());
        let types = item.types();

        if tags.is_empty() {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            return format!(
                "SELECT position FROM event WHERE event_type IN ({})",
                placeholders(types.len())
            );
        }

        for tag in &tags {
            params.push(Value::Text(tag.clone()));
        }
        let type_clause = if types.is_empty() {
            String::new()
        } else {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            format!(" AND event_type IN ({})", placeholders(types.len()))
        };

        format!(
            "SELECT position FROM event_tag WHERE tag IN ({}){} \
             GROUP BY position HAVING COUNT(DISTINCT tag) = {}",
            placeholders(tags.len()),
            type_clause,
            tags.len()
        )
    }
}

/// The whole tag set as one canonical, delimited text column.
///
/// `Tags` is canonically sorted, which is what makes a single column a
/// *canonical* encoding rather than an arbitrary one, and is the reason this arm
/// is on the list at all. Matching is `instr(tags_text, ?) > 0` per tag, which
/// is a full scan of the candidate rows and no index anywhere — that is the cost
/// the measurement exists to expose, not a defect in the arm.
///
/// The encoding is `UNIT tag UNIT tag UNIT`, with a leading and a trailing
/// delimiter, so `instr` searching for `UNIT tag UNIT` cannot match a prefix of
/// a longer tag. Without the bracketing delimiters `course:c1` would match
/// `course:c10`, which is a correctness bug an unsuspicious benchmark would
/// never surface.
#[derive(Debug, Clone, Copy)]
pub struct CanonicalBlob;

impl CanonicalBlob {
    /// The canonical column value for a tag set.
    fn encode(tags: &Tags) -> String {
        let mut out = String::new();
        out.push(UNIT);
        for tag in tags.iter() {
            out.push_str(tag.as_str());
            out.push(UNIT);
        }
        out
    }

    /// The needle `instr` looks for when testing one tag.
    fn needle(tag: &str) -> String {
        format!("{UNIT}{tag}{UNIT}")
    }
}

impl TagStorage for CanonicalBlob {
    const NAME: &'static str = "canonical-blob";

    fn migrate(_connection: &Connection) -> rusqlite::Result<()> {
        // Nothing beyond `event`, which already carries `tags_text`. The arm's
        // whole claim is that it needs no second table.
        Ok(())
    }

    fn write_tags(
        _connection: &Connection,
        _position: i64,
        _event_type: &str,
        _tags: &Tags,
    ) -> rusqlite::Result<()> {
        // Written with the event row itself; see `candidate::insert_event`.
        Ok(())
    }

    fn item_sql(item: &QueryItem, params: &mut Vec<Value>) -> String {
        let tags = distinct_tags(item.tags());
        let types = item.types();

        let mut clauses: Vec<String> = Vec::new();
        if !types.is_empty() {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            clauses.push(format!("event_type IN ({})", placeholders(types.len())));
        }
        for tag in &tags {
            params.push(Value::Text(Self::needle(tag)));
            clauses.push("instr(tags_text, ?) > 0".to_owned());
        }

        if clauses.is_empty() {
            "SELECT position FROM event".to_owned()
        } else {
            format!("SELECT position FROM event WHERE {}", clauses.join(" AND "))
        }
    }
}

/// The tag set as a JSON array, matched with SQLite's JSON1 `json_each`.
///
/// The third option `lib.rs:63-65` names. It buys a query language over the
/// column without a second table, and it pays for it twice: `json_each` is a
/// table-valued function called per candidate row, and the column has to be
/// parsed each time. Like the blob arm it has no index, so the honest
/// comparison is against that one rather than against the join table.
#[derive(Debug, Clone, Copy)]
pub struct Json1;

impl Json1 {
    /// The JSON array a tag set encodes to: a flat array of tag strings.
    ///
    /// Flat rather than an array of `[key, value]` pairs, because a `Tag` is
    /// *already* the canonical `key:value` string the contract validates, and
    /// splitting it here would invent a second canonical form for the same
    /// value.
    fn encode(tags: &Tags) -> String {
        let mut out = String::from("[");
        for (index, tag) in tags.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(&json_string(tag.as_str()));
        }
        out.push(']');
        out
    }
}

/// A JSON string literal for `value`.
///
/// Hand-written rather than pulled from `serde_json`: a tag cannot contain a
/// control character (VT-14), so the only escapes reachable are the quote and
/// the backslash, and adding a JSON dependency to a throwaway crate to encode
/// two characters would be the largest thing in its graph.
fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

impl TagStorage for Json1 {
    const NAME: &'static str = "json1";

    fn migrate(_connection: &Connection) -> rusqlite::Result<()> {
        Ok(())
    }

    fn write_tags(
        _connection: &Connection,
        _position: i64,
        _event_type: &str,
        _tags: &Tags,
    ) -> rusqlite::Result<()> {
        Ok(())
    }

    fn item_sql(item: &QueryItem, params: &mut Vec<Value>) -> String {
        let tags = distinct_tags(item.tags());
        let types = item.types();

        let mut clauses: Vec<String> = Vec::new();
        if !types.is_empty() {
            for event_type in types {
                params.push(Value::Text(event_type.as_str().to_owned()));
            }
            clauses.push(format!("event_type IN ({})", placeholders(types.len())));
        }
        for tag in &tags {
            params.push(Value::Text(tag.clone()));
            clauses.push(
                "EXISTS (SELECT 1 FROM json_each(event.tags_json) WHERE json_each.value = ?)"
                    .to_owned(),
            );
        }

        if clauses.is_empty() {
            "SELECT position FROM event".to_owned()
        } else {
            format!("SELECT position FROM event WHERE {}", clauses.join(" AND "))
        }
    }
}

/// The two encoded columns every event row carries, whichever arm is in force.
///
/// Both are written on every append rather than only the one the arm in force
/// reads. That costs the append path the same on all three arms, which is what
/// keeps the *probe* the thing being compared — an arm that also wrote less
/// would be winning on a second axis the record never asked about.
pub(crate) fn encoded_columns(tags: &Tags) -> (String, String) {
    (CanonicalBlob::encode(tags), Json1::encode(tags))
}
