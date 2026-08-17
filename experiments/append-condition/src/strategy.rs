//! The three append-condition strategies, and nothing else.
//!
//! `crates/happenstance-sqlite/src/lib.rs:56-62` names exactly three
//! candidates. All three are here, none is invented, and each one differs from
//! the others in **one** thing: the SQL shape that decides whether the batch is
//! allowed to land. Every arm shares the same schema, the same row insert, the
//! same identity stamp and the same read path, so a difference between two
//! figures can only be this.
//!
//! # What all three must be
//!
//! Atomic, and the condition must be evaluated in the same transaction the
//! write happens in. A probe followed by an unrelated insert is the wrong
//! implementation the whole port exists to reject, and
//! `racing_conditional_appends_elect_one_winner` is what would catch it.
//!
//! A rejection also has to leave the store byte-identical, which is what makes
//! the rollback path load-bearing rather than tidy.
//!
//! # The one thing none of them may be
//!
//! Unbounded on `SQLITE_BUSY`. With N connections on one file, `BEGIN
//! IMMEDIATE` on a busy database returns `SQLITE_BUSY` *immediately* unless a
//! busy handler is configured — and an **unbounded** handler converts a
//! livelock into a hung run naming no rule, because there is no watchdog
//! anywhere in the suite (CF-33). The timeout is finite and generous, set on
//! the connection in `candidate::configure`, and its value is one of the three
//! pragma numbers ADR-0022 owes.

use happenstance_core::{AppendCondition, Event, RecordedAt, SequencePosition, StoreId};
use rusqlite::Connection;
use rusqlite::types::Value;

use crate::candidate::{conflicting_position, insert_event, stamp_identity};
use crate::tags::{TagStorage, match_sql};

/// Why an append did not return a position.
///
/// The two are kept apart because the caller must be able to tell "retry the
/// decision" from "something broke" without knowing which adapter it holds —
/// which is the distinction `AppendError` exists to carry, and the one
/// `concurrency::Attempt` collapses to on the other side.
#[derive(Debug)]
pub enum AppendFailure {
    /// The condition matched, at this position if the arm could name it.
    Violated(Option<SequencePosition>),
    /// The driver failed.
    Sqlite(rusqlite::Error),
}

impl From<rusqlite::Error> for AppendFailure {
    fn from(error: rusqlite::Error) -> Self {
        Self::Sqlite(error)
    }
}

/// One candidate answer to "how does a SQLite adapter evaluate an append
/// condition atomically".
pub trait AppendStrategy {
    /// How the arm names itself in a results table.
    const NAME: &'static str;

    /// Appends `events`, rejecting the write if `condition` matches.
    ///
    /// Returns the position assigned to the **last** event of the batch.
    ///
    /// # Errors
    ///
    /// Returns [`AppendFailure::Violated`] when the condition matched, and
    /// [`AppendFailure::Sqlite`] for anything else.
    fn append<T: TagStorage>(
        connection: &mut Connection,
        store_id: StoreId,
        events: &[Event],
        condition: Option<&AppendCondition>,
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, AppendFailure>;
}

/// The position a batch's last insert landed at.
fn last_position(position: i64) -> Result<SequencePosition, AppendFailure> {
    SequencePosition::new(position.unsigned_abs())
        .ok_or_else(|| AppendFailure::Sqlite(rusqlite::Error::IntegralValueOutOfRange(0, position)))
}

/// **Candidate A.** `BEGIN IMMEDIATE`, then an `EXISTS` probe, then the insert.
///
/// The write lock is taken *before* the condition is read, so the snapshot the
/// probe sees is the snapshot the insert writes into and no second writer can
/// fit between them. That is the property the whole strategy is bought for, and
/// it is why this is the architecture brief's recommendation — a recommendation
/// this experiment exists to confirm or overturn with a figure rather than
/// ratify.
///
/// It pays for it in two places: the write lock is held across the probe, so
/// every other writer waits behind whatever the tag storage costs; and naming
/// the conflicting position takes a **second** query, on the rejection path
/// only, because `EXISTS` answers a boolean.
#[derive(Debug, Clone, Copy)]
pub struct BeginImmediateProbe;

impl AppendStrategy for BeginImmediateProbe {
    const NAME: &'static str = "begin-immediate-probe";

    fn append<T: TagStorage>(
        connection: &mut Connection,
        store_id: StoreId,
        events: &[Event],
        condition: Option<&AppendCondition>,
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, AppendFailure> {
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        if let Some(condition) = condition {
            for guard in condition.guards() {
                let mut params: Vec<Value> = Vec::new();
                let matched = match_sql::<T>(&guard.query, &mut params);
                params.push(Value::Integer(
                    guard
                        .after
                        .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX)),
                ));

                let hit: i64 = transaction.query_row(
                    &format!("SELECT EXISTS (SELECT 1 FROM ({matched}) WHERE position > ?)"),
                    rusqlite::params_from_iter(params.iter()),
                    |row| row.get(0),
                )?;

                if hit != 0 {
                    // The boolean said no; a second query says where. It runs
                    // on the rejection path only, which is the trade `EXISTS`
                    // makes against `max(position)`.
                    let at = conflicting_position::<T>(&transaction, condition)?;
                    // Rolling back rather than committing is what "a rejected
                    // append leaves the store byte-identical" means.
                    drop(transaction);
                    return Err(AppendFailure::Violated(
                        at.and_then(|value| SequencePosition::new(value.unsigned_abs())),
                    ));
                }
            }
        }

        let mut last = 0;
        for event in events {
            last = insert_event::<T>(&transaction, event, recorded_at)?;
        }
        stamp_identity(&transaction, store_id)?;
        transaction.commit()?;

        last_position(last)
    }
}

/// **Candidate B.** One `INSERT … SELECT … WHERE NOT EXISTS`, in a deferred
/// transaction.
///
/// The condition and the first row's write are one statement, so there is no
/// interval between them at all — the strongest form of "evaluate and write
/// atomically" available in SQL. The batch's remaining rows follow
/// unconditionally inside the same transaction, which is correct because a
/// batch is never evaluated against its own events.
///
/// What it pays: the transaction is **deferred**, so it starts as a reader and
/// upgrades at the `INSERT`. Two writers that both got a read lock and then both
/// try to upgrade produce `SQLITE_BUSY` on one of them, which the finite busy
/// timeout absorbs — and which is exactly the cost this arm is measured for.
/// Naming the conflicting position also takes a second query, because `changes()`
/// answers a count.
#[derive(Debug, Clone, Copy)]
pub struct ConditionalInsert;

impl AppendStrategy for ConditionalInsert {
    const NAME: &'static str = "conditional-insert";

    fn append<T: TagStorage>(
        connection: &mut Connection,
        store_id: StoreId,
        events: &[Event],
        condition: Option<&AppendCondition>,
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, AppendFailure> {
        let transaction = connection.transaction()?;

        let mut remaining = events;
        if let Some(condition) = condition {
            let first = &events[0];
            let (tags_text, tags_json) = crate::tags::encoded_columns(first.tags());

            let mut params: Vec<Value> = vec![
                Value::Text(first.event_type().as_str().to_owned()),
                Value::Blob(first.data().to_vec()),
                first
                    .metadata()
                    .map_or(Value::Null, |meta| Value::Blob(meta.to_vec())),
                Value::Text(tags_text),
                Value::Text(tags_json),
                Value::Integer(recorded_at.as_millis()),
            ];

            // Each guard is parenthesised into its own `NOT EXISTS`, which is
            // what `append.rs` warns an adapter generating SQL to do: the
            // precedence bug that threatens a single boundary becomes n times
            // more likely with several.
            let mut guards_sql: Vec<String> = Vec::new();
            for guard in condition.guards() {
                let matched = match_sql::<T>(&guard.query, &mut params);
                params.push(Value::Integer(
                    guard
                        .after
                        .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX)),
                ));
                guards_sql.push(format!(
                    "NOT EXISTS (SELECT 1 FROM ({matched}) WHERE position > ?)"
                ));
            }

            let sql = format!(
                "INSERT INTO event \
                 (event_type, data, metadata, tags_text, tags_json, recorded_at) \
                 SELECT ?, ?, ?, ?, ?, ? WHERE {}",
                guards_sql.join(" AND ")
            );

            let inserted = transaction.execute(&sql, rusqlite::params_from_iter(params.iter()))?;

            if inserted == 0 {
                let at = conflicting_position::<T>(&transaction, condition)?;
                drop(transaction);
                return Err(AppendFailure::Violated(
                    at.and_then(|value| SequencePosition::new(value.unsigned_abs())),
                ));
            }

            let position = transaction.last_insert_rowid();
            T::write_tags(
                &transaction,
                position,
                first.event_type().as_str(),
                first.tags(),
            )?;
            crate::candidate::bump_cardinality(&transaction, first.tags())?;
            remaining = &events[1..];
        }

        let mut last = transaction.last_insert_rowid();
        for event in remaining {
            last = insert_event::<T>(&transaction, event, recorded_at)?;
        }
        stamp_identity(&transaction, store_id)?;
        transaction.commit()?;

        last_position(last)
    }
}

/// **Candidate C.** `BEGIN IMMEDIATE`, then one `max(position)` guard.
///
/// The insight this arm trades on: a guard is *"nothing matching after this
/// boundary"*, which is an inequality on the **highest** matching position
/// rather than an existence question. One `SELECT max(position)` answers both
/// halves — whether the condition is violated, and by which event — so the
/// rejection path costs no second query.
///
/// What it was expected to pay, and did not: `max()` cannot stop at the first
/// hit, so where `EXISTS` short-circuits on the earliest matching row this one
/// looked as though it must walk the whole matching range, and the arm was
/// predicted to lose as the matching set grows.
///
/// **The measurement reversed that, and it is why this arm won.** With the join
/// table keyed `(tag, position)`, `max(position)` over a single tag's range is a
/// seek to the end of it rather than a walk — and answering with the position
/// rather than with a boolean means the rejection path needs no second query at
/// all. Measured on the rejection path in one interleaved run, medians:
///
/// | log | 1-tag boundary | 2-tag boundary |
/// | --- | --- | --- |
/// | 5,000 events | **23 us** vs 32 (probe) and 45 (insert) | **972 us** vs 1,513 and 1,532 |
/// | 50,000 events | **213 us** vs 311 and 306 | **42,399 us** vs 65,637 and 65,383 |
///
/// The ordering held in every run at both scales, which the commit-path and
/// contended figures did not (`results/append-condition.md`).
#[derive(Debug, Clone, Copy)]
pub struct MonotonicGuard;

impl AppendStrategy for MonotonicGuard {
    const NAME: &'static str = "monotonic-guard";

    fn append<T: TagStorage>(
        connection: &mut Connection,
        store_id: StoreId,
        events: &[Event],
        condition: Option<&AppendCondition>,
        recorded_at: RecordedAt,
    ) -> Result<SequencePosition, AppendFailure> {
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;

        if let Some(condition) = condition {
            for guard in condition.guards() {
                let mut params: Vec<Value> = Vec::new();
                let matched = match_sql::<T>(&guard.query, &mut params);

                let highest: Option<i64> = transaction.query_row(
                    &format!("SELECT max(position) FROM ({matched})"),
                    rusqlite::params_from_iter(params.iter()),
                    |row| row.get(0),
                )?;

                let boundary = guard
                    .after
                    .map_or(0, |after| i64::try_from(after.get()).unwrap_or(i64::MAX));

                if let Some(highest) = highest
                    && highest > boundary
                {
                    drop(transaction);
                    return Err(AppendFailure::Violated(SequencePosition::new(
                        highest.unsigned_abs(),
                    )));
                }
            }
        }

        let mut last = 0;
        for event in events {
            last = insert_event::<T>(&transaction, event, recorded_at)?;
        }
        stamp_identity(&transaction, store_id)?;
        transaction.commit()?;

        last_position(last)
    }
}
