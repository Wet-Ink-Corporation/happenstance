//! SY-40 on Postgres (#483, #484, ADR-0110): an ingest drops every foreign
//! event whose identity the store has recorded as removed, before it draws a
//! position for it.
//!
//! The record is `event_removed`, written by migration 4's `BEFORE DELETE`
//! trigger on `event`. The ingest's first round trip takes `LOCK TABLE
//! event_removed IN SHARE MODE` and, in the same batch, reads whether the
//! record holds anything at all ([`super::FIRST_ROUND_TRIP_SQL`]). Only when it
//! does is [`REMOVED_SQL`] sent, once per call, for every foreign identity the
//! call carries. A dropped event never reaches the insert: it draws no sequence
//! value, it is counted as skipped, it is neither landed nor hidden for SY-38's
//! check, and a group made only of dropped and held events lands nothing and
//! so writes no compensation (SY-11).
//!
//! # Why the lock is `SHARE`
//!
//! The trigger's insert takes `ROW EXCLUSIVE` on the record. `SHARE` is the
//! weakest mode that conflicts with it and not with itself, so concurrent
//! ingests do not wait on each other, appends never touch the table, and a
//! removal and an ingest exclude each other: an ingest waits for a removal in
//! flight to commit, and so reads its record, and a removal waits for an ingest
//! in flight to commit, and so deletes rows that ingest wrote rather than
//! racing it (I-6, D-8). A removal is a rare operator action, so the wait costs
//! nothing in practice; `SHARE` also waits behind a manual `VACUUM` of this one
//! small table, which autovacuum yields.
//!
//! The trigger is `BEFORE`, so a deleter takes the record's lock at its first
//! row, before that row is marked deleted. An ingest holding `SHARE` that meets
//! the row under `ON CONFLICT` sees a live row and skips it, and never waits on
//! the deleter, so the two cannot deadlock.
//!
//! # Held events
//!
//! An identity the store still holds is never looked up here: it is skipped by
//! the origin pair's unique index inside the insert, as before (VT-8). The
//! record is consulted only for identities a store once held and no longer
//! does, and only under the lock that excludes every removal, so it is never a
//! race-prone read-then-append.

use std::collections::HashSet;

use happenstance_core::EventId;
use sqlx::{Postgres, Transaction};

use super::OwnedGroup;
use crate::error::PostgresEventStoreError;
use crate::event_store::{position_from_row, stored_store_id};

/// Of the origin pairs bound as two parallel arrays, those the record holds.
///
/// A point lookup per pair on the record's primary key. The arrays travel as
/// `bytea[]` and `bigint[]`, so the statement's text does not depend on the
/// batch's size.
pub(super) const REMOVED_SQL: &str = "SELECT origin_store, origin_position FROM event_removed \
     WHERE (origin_store, origin_position) IN \
     (SELECT * FROM unnest($1::bytea[], $2::bigint[]))";

/// Drops from every group the foreign events whose identity the record holds.
///
/// Called only when the first round trip found the record non-empty, inside
/// the ingest's transaction and under its `SHARE` lock. Each group keeps its
/// `offered` count, so a dropped event is counted as skipped.
///
/// # Errors
///
/// `Driver` if the lookup fails, and the decoding variants if the record holds
/// a store identity that is not sixteen bytes or a position below one.
pub(super) async fn drop_removed(
    transaction: &mut Transaction<'static, Postgres>,
    groups: &mut [OwnedGroup],
) -> Result<(), PostgresEventStoreError> {
    let (stores, positions): (Vec<Vec<u8>>, Vec<i64>) = groups
        .iter()
        .flat_map(|group| &group.events)
        .map(|foreign| {
            (
                foreign.origin.store.to_bytes().to_vec(),
                foreign.origin.position,
            )
        })
        .unzip();
    let rows: Vec<(Vec<u8>, i64)> = sqlx::query_as(REMOVED_SQL)
        .bind(stores)
        .bind(positions)
        .fetch_all(&mut **transaction)
        .await?;
    if rows.is_empty() {
        return Ok(());
    }
    let removed = rows
        .into_iter()
        .map(|(store, position)| {
            Ok(EventId::new(
                stored_store_id(&store)?,
                position_from_row(position)?,
            ))
        })
        .collect::<Result<HashSet<_>, PostgresEventStoreError>>()?;
    for group in groups {
        group
            .events
            .retain(|foreign| !removed.contains(&foreign.id));
    }
    Ok(())
}
