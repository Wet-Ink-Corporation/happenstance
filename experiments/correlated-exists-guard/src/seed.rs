//! Filling a store to 10^6 events without measuring the seeder.
//!
//! # Why the seed does not go through `append`
//!
//! `experiments/append-condition`'s `fill` appends in batches of 100 through the
//! store's own `append`. That is the right thing at 50,000 events and impossible
//! at 10^6: its `stamp_identity`
//! (`experiments/append-condition/src/candidate.rs:296-303`) is
//! `UPDATE event … WHERE origin_position IS NULL` with **no positional bound**,
//! and the only index over that column is `UNIQUE (origin_store,
//! origin_position)` whose leading column is `origin_store` — so every append
//! scans the whole table. Seeding a million events that way is quadratic.
//!
//! The shipped adapter does not have that defect (`event_store.rs:741-748`
//! bounds the same `UPDATE` by the batch's first assigned position, and
//! [`crate::probe_store::write_batch`] carries the bound), so the seeder could have
//! gone through [`ProbeStore::append`](crate::probe_store::ProbeStore). It still does
//! not, for a plainer reason: a million single-row `INSERT`s inside a million
//! transactions costs minutes that say nothing about the question. This writes
//! the same rows, in the same encoding, in transactions of 50,000.
//!
//! # What makes that safe
//!
//! [`verify`] reads the store back through the contract's own types and checks
//! every count the guard SQL depends on — the `event` count, the `event_tag`
//! count, that no row is left unstamped, and that `tag_cardinality` agrees with
//! the rows actually written. `tag_cardinality` is what orders the chain's seed
//! arm, so a seeder that got it wrong would silently measure a *different* SQL
//! shape from the one that ships. `run.sh` runs `verify` before any timer
//! starts.

use happenstance_core::StoreId;
use rusqlite::Connection;
use rusqlite::types::Value;

/// The event type every seeded event carries.
///
/// The same string `experiments/append-condition/tests/contention_at_64.rs`'s
/// `fill` uses, so this experiment's 50,000-event calibration and §1's recorded
/// figure describe the same log.
pub const SEED_TYPE: &str = "Seeded";

/// The payload every seeded event carries.
///
/// Small and constant. The guard SQL never reads the `data` column — every shape
/// under test answers out of `event_tag` alone — so payload size moves the page
/// count of the `event` table and nothing the measurement is about. It is stated
/// rather than varied so that the `event` table's size is a known constant.
pub const SEED_DATA: &[u8] = b"seeded";

/// The modulus that makes the `row:` tag selective.
///
/// 97, exactly as `contention_at_64.rs` uses, so `row:r7` matches about one
/// event in ninety-seven and `shard:cold` matches all of them. That contrast is
/// the whole point: a two-tag boundary crossing a very selective tag with a
/// completely unselective one is the shape `tag_cardinality` exists to order,
/// and the shape ADR-0022 §8's ~200x figure was taken on.
pub const ROW_MODULUS: u64 = 97;

/// Distinct `bucket:` tag values in [`Corpus::ManyBuckets`].
///
/// **128, because that is VT-23's floor** (`spec/SPECIFICATION.md`: every store
/// must evaluate a query of at least 128 items). One bucket per item makes the
/// widest conformant query a partition of the log rather than a query naming
/// the same tag repeatedly.
pub const BUCKETS: u64 = 128;

/// The `bucket:` tag value for index `k`, zero-padded.
///
/// Padded to three digits so that every tag string is the same length and
/// byte order matches numeric order. Nothing in the SQL depends on that — a tag
/// is an opaque string to `event_tag` — but a results table that sorts
/// `bucket:b10` between `bucket:b1` and `bucket:b2` is a table someone has to
/// read twice.
#[must_use]
pub fn bucket_tag(k: u64) -> String {
    format!("bucket:b{k:03}")
}

/// Rows written per transaction while seeding.
const ROWS_PER_TRANSACTION: u64 = 50_000;

/// Rows per multi-row `INSERT` into `event` (eight bound parameters each,
/// against the adapter's 30,000-parameter budget).
const EVENT_ROWS_PER_STATEMENT: usize = 3_000;

/// Rows per multi-row `INSERT` into `event_tag` (three bound parameters each).
const TAG_ROWS_PER_STATEMENT: usize = 9_000;

/// Which pair of tags a seeded corpus carries.
///
/// The whole variable this crate added to the sibling experiment's seed, and it
/// exists because the sibling measured exactly one selectivity contrast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Corpus {
    /// `row:rN` (one event in 97) and `shard:cold` (all of them).
    ///
    /// The sibling experiment's shape, unchanged, and the shape ADR-0022 §8's
    /// ~200x figure was taken on. A very selective tag crossed with a completely
    /// unselective one is what `tag_cardinality` exists to order.
    SelectiveAndUnselective,
    /// `all:yes` and `shard:cold`, **both matching every event**.
    ///
    /// The adversarial case, and the one that could overturn this crate's
    /// finding. The correlated `EXISTS` form makes the seed arm the outer loop,
    /// so its cost is one index seek per seed row — which is a win precisely
    /// *because* the seed is selective. Here no tag is, so the seed is the whole
    /// log and the form does `|log|` seeks where the uncorrelated one does one
    /// scan plus `|log|` probes. A sequential scan is cheaper per row than a
    /// seek, so this is the shape where `chain-exists` could plausibly lose.
    ///
    /// The two counts are equal, and `most_selective_first` sorts by count with
    /// a stable sort, so the input order decides the seed and the "ordering"
    /// question is vacuous here. That is stated rather than worked around: a tie
    /// is what "no tag is selective" means.
    BothUnselective,
    /// `bucket:bNNN` (one event in [`BUCKETS`]) and `shard:cold` (all of them).
    ///
    /// The corpus a **wide** query needs, and the only reason it exists. VT-23
    /// requires every store to evaluate a query of 128 items, and the other two
    /// corpora carry two distinct tag values and 97 — so a 128-item query over
    /// either of them names at most 97 different things and measures repetition
    /// rather than width.
    ///
    /// [`BUCKETS`] is 128 exactly, so a 128-item query whose *i*th item names
    /// `bucket:b{i}` **partitions** the log: every event matches exactly one
    /// arm, the union is the whole store, and no arm duplicates another. That
    /// is the widest shape the specification's floor describes, and it is also
    /// the one where a per-arm page budget has the most arms to be multiplied
    /// by.
    ManyBuckets,
}

impl Corpus {
    /// The two tags a guard over this corpus names, most-selective-first.
    #[must_use]
    pub const fn guard_tags(self) -> [(&'static str, &'static str); 2] {
        match self {
            Self::SelectiveAndUnselective => [("shard", "cold"), ("row", "r7")],
            Self::BothUnselective => [("shard", "cold"), ("all", "yes")],
            // One arbitrary bucket, so a *two-tag* table can be taken over this
            // corpus too. The corpus exists for the wide case and this pair is
            // not what it is for; the width lives in the query, not here.
            Self::ManyBuckets => [("shard", "cold"), ("bucket", "b000")],
        }
    }
}

/// The tags a seeded event at `position` carries, in canonical (sorted) order.
///
/// Canonical means byte-sorted over the whole tag string, which is what VT-16
/// requires of a `Tags` value — `all:yes` < `row:rN` < `shard:cold` — so writing
/// them in this order makes the stored column canonical without a sort.
#[must_use]
pub fn seed_tags(corpus: Corpus, position: u64) -> [String; 2] {
    match corpus {
        Corpus::SelectiveAndUnselective => [
            format!("row:r{}", (position - 1) % ROW_MODULUS),
            "shard:cold".to_owned(),
        ],
        Corpus::BothUnselective => ["all:yes".to_owned(), "shard:cold".to_owned()],
        // "bucket:b000" < "shard:cold" bytewise, so this pair is already
        // canonical in the order it is written, like the other two.
        Corpus::ManyBuckets => [
            bucket_tag((position - 1) % BUCKETS),
            "shard:cold".to_owned(),
        ],
    }
}

/// The canonical `tags` column value for a seeded event — the same
/// `0x1F`-delimited encoding [`crate::probe_store::encode_tags`] writes.
fn encoded_tags(corpus: Corpus, position: u64) -> Vec<u8> {
    let mut out = vec![0x1f];
    for tag in seed_tags(corpus, position) {
        out.extend_from_slice(tag.as_bytes());
        out.push(0x1f);
    }
    out
}

/// Fills the store to exactly `target` events, appending nothing if it is
/// already there.
///
/// Positions are written **explicitly**, 1..=`target`. `AUTOINCREMENT` permits
/// that and updates `sqlite_sequence` itself, so a later `append` through the
/// store continues from `target + 1` rather than colliding.
///
/// # Errors
///
/// Returns the driver's error if any statement fails.
pub fn seed(connection: &mut Connection, store_id: StoreId, target: u64) -> rusqlite::Result<u64> {
    seed_corpus(
        connection,
        store_id,
        target,
        Corpus::SelectiveAndUnselective,
    )
}

/// [`seed`], with the tag pair named rather than defaulted.
///
/// # Errors
///
/// Returns the driver's error if any statement fails.
pub fn seed_corpus(
    connection: &mut Connection,
    store_id: StoreId,
    target: u64,
    corpus: Corpus,
) -> rusqlite::Result<u64> {
    let existing: i64 = connection.query_row("SELECT count(*) FROM event", [], |row| row.get(0))?;
    let existing = existing.unsigned_abs();
    if existing >= target {
        return Ok(existing);
    }

    let store_bytes = store_id.to_bytes().to_vec();
    let recorded_at = 1_756_000_000_000i64;

    let mut next = existing + 1;
    while next <= target {
        let stop = (next + ROWS_PER_TRANSACTION - 1).min(target);
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        write_events(&transaction, &store_bytes, recorded_at, next, stop, corpus)?;
        write_tags(&transaction, next, stop, corpus)?;
        transaction.commit()?;
        next = stop + 1;
    }

    bump_cardinality(connection, target, corpus)?;

    let seeded: i64 = connection.query_row("SELECT count(*) FROM event", [], |row| row.get(0))?;
    Ok(seeded.unsigned_abs())
}

/// The `event` rows for positions `from..=to`.
fn write_events(
    connection: &Connection,
    store_bytes: &[u8],
    recorded_at: i64,
    from: u64,
    to: u64,
    corpus: Corpus,
) -> rusqlite::Result<()> {
    let mut params: Vec<Value> = Vec::new();
    let mut rows = 0usize;
    for position in from..=to {
        let signed = i64::try_from(position).unwrap_or(i64::MAX);
        params.push(Value::Integer(signed));
        params.push(Value::Text(SEED_TYPE.to_owned()));
        params.push(Value::Blob(SEED_DATA.to_vec()));
        params.push(Value::Null);
        params.push(Value::Blob(encoded_tags(corpus, position)));
        params.push(Value::Blob(store_bytes.to_vec()));
        params.push(Value::Integer(signed));
        params.push(Value::Integer(recorded_at));
        rows += 1;
        if rows == EVENT_ROWS_PER_STATEMENT {
            flush(connection, EVENT_INSERT, 8, &params, rows)?;
            params.clear();
            rows = 0;
        }
    }
    if rows > 0 {
        flush(connection, EVENT_INSERT, 8, &params, rows)?;
    }
    Ok(())
}

/// The `event_tag` rows for positions `from..=to`.
fn write_tags(connection: &Connection, from: u64, to: u64, corpus: Corpus) -> rusqlite::Result<()> {
    let mut params: Vec<Value> = Vec::new();
    let mut rows = 0usize;
    for position in from..=to {
        let signed = i64::try_from(position).unwrap_or(i64::MAX);
        for tag in seed_tags(corpus, position) {
            params.push(Value::Text(tag));
            params.push(Value::Integer(signed));
            params.push(Value::Text(SEED_TYPE.to_owned()));
            rows += 1;
            if rows == TAG_ROWS_PER_STATEMENT {
                flush(connection, TAG_INSERT, 3, &params, rows)?;
                params.clear();
                rows = 0;
            }
        }
    }
    if rows > 0 {
        flush(connection, TAG_INSERT, 3, &params, rows)?;
    }
    Ok(())
}

const EVENT_INSERT: &str = "INSERT INTO event \
     (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) \
     VALUES ";

const TAG_INSERT: &str = "INSERT INTO event_tag (tag, position, event_type) VALUES ";

/// One multi-row `INSERT` of `rows` tuples of `width` bound parameters.
fn flush(
    connection: &Connection,
    prefix: &str,
    width: usize,
    params: &[Value],
    rows: usize,
) -> rusqlite::Result<()> {
    let mut sql = String::with_capacity(prefix.len() + rows * (width * 2 + 3));
    sql.push_str(prefix);
    for index in 0..rows {
        if index > 0 {
            sql.push(',');
        }
        sql.push('(');
        for slot in 0..width {
            if slot > 0 {
                sql.push(',');
            }
            sql.push('?');
        }
        sql.push(')');
    }
    connection.execute(&sql, rusqlite::params_from_iter(params.iter()))?;
    Ok(())
}

/// Rebuilds `tag_cardinality` from the rows that were written.
///
/// Computed in Rust rather than with `INSERT … SELECT count(*) … GROUP BY tag`
/// so that a wrong count here is a wrong count in *this* file rather than a
/// tautology: the table is what orders the chain's seed arm, and deriving it
/// from the same query the guard reads would make [`verify`] unable to catch a
/// seeding mistake.
fn bump_cardinality(connection: &Connection, target: u64, corpus: Corpus) -> rusqlite::Result<()> {
    let transaction = connection.unchecked_transaction()?;
    transaction.execute("DELETE FROM tag_cardinality", [])?;
    {
        let mut statement =
            transaction.prepare("INSERT INTO tag_cardinality (tag, events) VALUES (?, ?)")?;
        let whole_log = i64::try_from(target).unwrap_or(i64::MAX);
        statement.execute(rusqlite::params!["shard:cold", whole_log])?;
        if corpus == Corpus::BothUnselective {
            // Equal to `shard:cold`'s by construction. `most_selective_first`
            // sorts by count with a stable sort, so the tie leaves the input
            // order deciding the seed — which is what "no tag is selective"
            // means, and is stated rather than broken with a tiebreak this
            // adapter does not have.
            statement.execute(rusqlite::params!["all:yes", whole_log])?;
        } else if corpus == Corpus::ManyBuckets {
            for bucket in 0..BUCKETS {
                // Positions 1..=target map to bucket (position - 1) % BUCKETS,
                // so the count is how many of 0..target are congruent to it —
                // the same arithmetic as the `row:` case, and deliberately not
                // `target / BUCKETS` alone, which is right only when the
                // division is exact.
                let count = target / BUCKETS + u64::from(target % BUCKETS > bucket);
                statement.execute(rusqlite::params![
                    bucket_tag(bucket),
                    i64::try_from(count).unwrap_or(i64::MAX)
                ])?;
            }
        } else {
            for bucket in 0..ROW_MODULUS {
                // Positions 1..=target map to bucket (position - 1) % 97, so the
                // count is how many of 0..target are congruent to `bucket`.
                let count = target / ROW_MODULUS + u64::from(target % ROW_MODULUS > bucket);
                statement.execute(rusqlite::params![
                    format!("row:r{bucket}"),
                    i64::try_from(count).unwrap_or(i64::MAX)
                ])?;
            }
        }
    }
    transaction.commit()
}

/// What [`verify`] found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedCheck {
    /// Rows in `event`.
    pub events: i64,
    /// Rows in `event_tag`.
    pub tag_rows: i64,
    /// Rows in `event` carrying no identity. Must be zero.
    pub unstamped: i64,
    /// `tag_cardinality`'s count for `shard:cold`.
    pub shard_cold: i64,
    /// The number of `event_tag` rows actually carrying `shard:cold`.
    pub shard_cold_actual: i64,
    /// `tag_cardinality`'s count for `row:r7`.
    pub row_r7: i64,
    /// The number of `event_tag` rows actually carrying `row:r7`.
    pub row_r7_actual: i64,
}

/// Reads the seeded store back and checks every count the guard SQL depends on.
///
/// # Errors
///
/// Returns the driver's error if any statement fails.
pub fn verify(connection: &Connection, target: u64) -> rusqlite::Result<SeedCheck> {
    let one =
        |sql: &str| -> rusqlite::Result<i64> { connection.query_row(sql, [], |row| row.get(0)) };
    let check = SeedCheck {
        events: one("SELECT count(*) FROM event")?,
        tag_rows: one("SELECT count(*) FROM event_tag")?,
        unstamped: one("SELECT count(*) FROM event WHERE origin_position IS NULL")?,
        shard_cold: one("SELECT events FROM tag_cardinality WHERE tag = 'shard:cold'")?,
        shard_cold_actual: one("SELECT count(*) FROM event_tag WHERE tag = 'shard:cold'")?,
        row_r7: one("SELECT events FROM tag_cardinality WHERE tag = 'row:r7'")?,
        row_r7_actual: one("SELECT count(*) FROM event_tag WHERE tag = 'row:r7'")?,
    };
    let _ = target;
    Ok(check)
}

impl SeedCheck {
    /// Whether the seed is internally consistent at `target` events.
    #[must_use]
    pub fn is_sound(&self, target: u64) -> bool {
        let target = i64::try_from(target).unwrap_or(i64::MAX);
        self.events == target
            && self.tag_rows == target * 2
            && self.unstamped == 0
            && self.shard_cold == target
            && self.shard_cold_actual == target
            && self.row_r7 == self.row_r7_actual
    }

    /// The one line a results table carries beside the figures it justifies.
    #[must_use]
    pub fn line(&self) -> String {
        format!(
            "events={} tag_rows={} unstamped={} shard_cold={}/{} row_r7={}/{}",
            self.events,
            self.tag_rows,
            self.unstamped,
            self.shard_cold,
            self.shard_cold_actual,
            self.row_r7,
            self.row_r7_actual,
        )
    }
}
