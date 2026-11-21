//! SY-40 on Neon (#483, #484, ADR-0110): ingest over a store whose rows
//! another client deleted with a raw `DELETE`.
//!
//! The scripted tests pin the request: still one round trip (SY-14), whose
//! first statement takes the record's `SHARE` lock and whose ingest statement
//! drops recorded identities before its insert. `LOCK` is a utility statement
//! and takes no snapshot, so at `REPEATABLE READ` the transaction's snapshot is
//! the ingest statement's, taken once the lock is granted: it sees every
//! removal committed before the call could proceed.
//!
//! The live tests, `#[ignore]`d as every live test in this crate is, mirror PR
//! 1's oracle behaviours in `happenstance-sync/src/memory_ingest/removal.rs`
//! and check the trigger against the endpoint, which has no `search_path` and
//! runs each request on whatever backend the proxy picks. The literal
//! positions are **origin** positions; every local position is one the store
//! assigned.

use base64::Engine as _;
use happenstance_core::{EventId, SequencePosition};
use happenstance_sync::{FailureClass, IngestError, IngestGroup, Ingested};

use super::endpoint::{LiveTransport, event_log_request, live_store};
use super::{
    Classify, IsolationLevel, Scripted, SqlStatement, append, carrying, event, holds, id, ingest,
    log, own, peer, rows, select, send, shipped,
};
use crate::config::NeonConfig;
use crate::event_store::{BASE64, NeonEventStore, decode_body};
use crate::migration::{qualified_removed, removal_trigger_statements};
use crate::transport::NullTransportError;

/// The answer a call that landed two, skipped one and compensated one gets
/// since the record: the lock's empty set, the counts row, the refusal's
/// empty `DO` set and the stamp's `UPDATE`.
pub(super) const LANDED_AFTER_LOCK: &[u8] = br#"{"results":[{"fields":[],"rows":[],"command":"LOCK","rowCount":null,"rowAsArray":false},{"fields":[],"rows":[{"appended":"2","skipped":"1","compensated":"1","unheld":""}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"DO","rowCount":null,"rowAsArray":false},{"fields":[],"rows":[],"command":"UPDATE","rowCount":2,"rowAsArray":false}]}"#;

/// The lock statement every ingest request opens with, against `config`.
fn lock(config: &NeonConfig) -> String {
    format!("LOCK TABLE {} IN SHARE MODE", qualified_removed(config))
}

/// An ingest is still one round trip, at `REPEATABLE READ`, and the record's
/// lock is its first statement, carrying no parameter: before the ingest
/// statement, so the snapshot is taken under the lock. The counts are read
/// from the ingest statement's set, the second, not the lock's.
///
/// Rejects an ingest that reads the record in a round trip of its own, one
/// that locks after the statement that takes the snapshot, and a decoder that
/// reads the lock's empty set for the counts.
#[tokio::test]
async fn the_ingest_request_locks_the_record_first_in_one_round_trip() {
    let transport = Scripted::answering(200, LANDED_AFTER_LOCK);
    let store = transport.store();
    let sent = [shipped(id(peer(0x71), 1), 1, event("Shipped", "a"))];

    let ingested = ingest(&store, &[IngestGroup::new(&sent, &[])])
        .await
        .expect("the answer decodes");

    assert_eq!(
        (ingested.appended, ingested.skipped, ingested.compensated),
        (2, 1, 1)
    );
    assert_eq!(transport.round_trips.get(), 1);
    let request = &transport.sent.borrow()[0];
    assert_eq!(request.isolation, IsolationLevel::RepeatableRead);
    assert_eq!(
        request.statements.len(),
        4,
        "the lock, the write, the refusal, the stamp"
    );
    let first = &request.statements[0];
    assert_eq!(first.query, lock(store.config()));
    assert!(first.params.is_empty());
    assert_eq!(
        request.statements[1].params.len(),
        2,
        "the ingest statement follows the lock"
    );
}

/// The ingest statement drops every foreign row whose origin pair the record
/// holds before its insert, by an anti-join on the record's primary key; the
/// skip count still falls out of `RETURNING`, so a dropped row counts as a
/// skip. Spelled as a `LEFT JOIN`, not `NOT EXISTS`, which is how this
/// adapter spells an append condition's guard and which an ingest's shape
/// test forbids (SY-1).
///
/// Rejects an ingest that dedupes only against what the store holds, the
/// shape every adapter shipped in 0.7.0.
#[test]
fn the_ingest_statement_drops_recorded_identities_before_its_insert() {
    let store = NeonEventStore::new(
        crate::transport::NullTransport::new(),
        NeonConfig::default().with_schema("hs_9"),
    );
    let sent = [shipped(id(peer(0x72), 1), 1, event("Shipped", "a"))];
    let request = store
        .ingest_request::<NullTransportError>(&[IngestGroup::new(&sent, &[])])
        .expect("an origin position that fits a bigint");
    let write = &request.statements[1].query;

    let ins = write.find("ins AS (").expect("the write builds `ins`");
    let won = write.find(", won AS (").expect("the write builds `won`");
    assert!(
        write[ins..won].contains(
            "FROM (SELECT f.* FROM f LEFT JOIN \"hs_9\".\"event_removed\" AS r \
             ON r.origin_store = f.origin_store AND r.origin_position = f.origin_position \
             WHERE r.origin_store IS NULL) b"
        ),
        "the insert reads its rows through the anti-join: {write}"
    );
    assert!(
        write.contains(
            "(jsonb_array_length($1::jsonb) - (SELECT count(*) FROM ins))::text AS skipped"
        ),
        "a dropped row is counted as a skip: {write}"
    );
    assert_eq!(
        write.matches("\"hs_9\".\"event_removed\"").count(),
        1,
        "the record is read once, by the anti-join: {write}"
    );
}

/// An answer of the old shape, with no lock set before the counts, is not
/// read leniently: the counts are looked for in the second set, which is the
/// refusal's and holds none.
#[tokio::test]
async fn an_answer_without_the_locks_set_is_named() {
    const NO_LOCK: &[u8] = br#"{"results":[{"fields":[],"rows":[{"appended":"2","skipped":"1","compensated":"1","unheld":""}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[],"command":"DO","rowCount":null,"rowAsArray":false}]}"#;
    let transport = Scripted::answering(200, NO_LOCK);
    let sent = [shipped(id(peer(0x73), 1), 1, event("Shipped", "a"))];

    let refused = ingest(&transport.store(), &[IngestGroup::new(&sent, &[])])
        .await
        .expect_err("the counts are not where the request put them");
    assert!(
        matches!(
            refused,
            IngestError::Store(crate::error::NeonError::MissingColumn { .. })
        ),
        "{refused:?}"
    );
}

/// The migration request the fixtures send carries migration 4's table after
/// migrations 1 and 3, then the trigger's two statements whole, each a
/// single entry of the one batch: the splitter never sees a `plpgsql` body.
///
/// Rejects a migration that splits the trigger's function at its first `;`,
/// which the endpoint refuses, and one that leaves the trigger out, under
/// which nothing records a removal and every SY-40 test above is red.
#[test]
fn the_migration_request_carries_the_trigger_statements_whole() {
    let config = NeonConfig::default().with_schema("hs_m");
    let request = event_log_request(&config);

    assert_eq!(
        request.statements.len(),
        9 + 3 + 1 + 2,
        "migration 1's nine, migration 3's three, the record's table, the trigger's two"
    );
    let [.., table, function, trigger] = request.statements.as_slice() else {
        panic!("the batch ends with the record: {:?}", request.statements);
    };
    assert!(
        table
            .query
            .starts_with(r#"CREATE TABLE IF NOT EXISTS "hs_m"."event_removed" ("#),
        "{}",
        table.query
    );
    let [want_function, want_trigger] = removal_trigger_statements(&config);
    assert_eq!(function.query, want_function.query);
    assert_eq!(trigger.query, want_trigger.query);
    for statement in [function, trigger] {
        assert_eq!(
            statement.query.matches('$').count() % 2,
            0,
            "a dollar quote was cut: {}",
            statement.query
        );
    }
    assert_eq!(request.isolation, IsolationLevel::ReadCommitted);
}

/// Deletes the events at `positions` in one request, as an operator's
/// retention script would through the endpoint, and checks each was held.
pub(super) async fn remove(store: &NeonEventStore<LiveTransport>, positions: &[SequencePosition]) {
    let array = format!(
        "{{{}}}",
        positions
            .iter()
            .map(|position| position.get().to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let response = send(
        store,
        vec![SqlStatement::with_params(
            format!(
                "DELETE FROM {} WHERE position = ANY($1::bigint[]) \
                 RETURNING position::text AS position",
                store.config().qualified_event()
            ),
            vec![serde_json::Value::String(array)],
        )],
    )
    .await;
    let body = decode_body::<super::endpoint::LiveTransportError>(&response, usize::MAX)
        .expect("the removal commits");
    let deleted = body.result_sets().first().map_or(0, |set| set.rows.len());
    assert_eq!(deleted, positions.len(), "every position named was held");
}

/// The local position of the one event carrying `id`.
async fn position_of(store: &NeonEventStore<LiveTransport>, id: EventId) -> SequencePosition {
    carrying(&log(store).await, id).position
}

/// `(appended, skipped, compensated)`, the three counts every test compares.
fn counts(ingested: Ingested) -> (usize, usize, usize) {
    (ingested.appended, ingested.skipped, ingested.compensated)
}

/// #483. A removed foreign event offered again is skipped and counted as
/// skipped: no row comes back, the log is the pruned log, and
/// `contains_event_id` keeps answering `false` (ES-38). The re-delivery runs
/// through the store held before the removal and a fresh one, which on this
/// adapter is the same thing as a reopen: no handle holds state (I-4).
///
/// **Red before the record:** `(1, 2, 0)`, the removed event re-admitted.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_removed_foreign_event_is_skipped_on_redelivery() {
    let store = live_store().await;
    let from = peer(0x61);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
        shipped(id(from, 3), 3, event("Shipped", "c")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("three foreign events land");
    remove(&store, &[position_of(&store, batch[1].id).await]).await;
    let pruned = log(&store).await;
    // A second store, which keeps both: a cheap handle onto the one client,
    // and a copy of a few names.
    let fresh = NeonEventStore::new(store.transport().clone(), store.config().clone());

    for (which, handle) in [("earlier", &store), ("fresh", &fresh)] {
        let again = ingest(handle, &[IngestGroup::new(&batch, &[])])
            .await
            .expect("a re-delivery over a removal is not an error");
        assert_eq!(counts(again), (0, 3, 0), "through the {which} handle");
    }
    assert_eq!(log(&store).await, pruned, "the log is the pruned log");
    assert!(!holds(&store, batch[1].id).await);
}

/// #484. A removed **own** event offered again is a skip, not
/// `UnheldOwnIdentity`, and the new foreign event beside it lands.
///
/// **Red before the record:** `HS038`, as `UnheldOwnIdentity`.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_removed_own_event_is_skipped_not_refused() {
    let store = live_store().await;
    let first = append(&store, &[event("Local", "a")]).await;
    append(&store, &[event("Local", "b")]).await;
    remove(&store, &[first]).await;
    let echo = EventId::new(own(&store).await, first);
    let arrived = id(peer(0x62), 1);
    let batch = [
        shipped(echo, 0, event("Local", "a")),
        shipped(arrived, 1, event("Shipped", "y")),
    ];

    let landed = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a removed own event is a skip, not SY-38's refusal");

    assert_eq!(counts(landed), (1, 1, 0));
    assert!(holds(&store, arrived).await);
    assert!(
        !holds(&store, echo).await,
        "the removed own event came back"
    );
}

/// I-3: an own claim the store **never** held is still SY-38's refusal in a
/// store that has recorded removals.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn an_own_claim_never_held_is_refused_after_a_removal() {
    let store = live_store().await;
    let first = append(&store, &[event("Local", "a")]).await;
    let head = append(&store, &[event("Local", "b")]).await;
    remove(&store, &[first]).await;
    let never = head
        .next()
        .and_then(SequencePosition::next)
        .and_then(SequencePosition::next)
        .expect("three past a fresh head exist");
    let claim = EventId::new(own(&store).await, never);
    let batch = [
        shipped(id(peer(0x63), 1), 1, event("Shipped", "z")),
        shipped(claim, 2, event("Local", "forged")),
    ];
    let before = log(&store).await;

    let refused = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect_err("a claim never held is refused after a removal too");

    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == claim),
        "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
    assert_eq!(
        log(&store).await,
        before,
        "a refused ingest wrote something"
    );
}

/// SY-11 over SY-40: a group made only of removed and held events lands
/// nothing, so its compensation is not written either.
///
/// **Red before the record:** `(1, 1, 1)`.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_group_of_removed_and_held_events_writes_no_compensation() {
    let store = live_store().await;
    let from = peer(0x64);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let answer = [event("ReservationCompensated", "seat")];
    ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("the group lands with its compensation");
    remove(&store, &[position_of(&store, group[0].id).await]).await;
    let pruned = log(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(
        counts(again),
        (0, 2, 0),
        "compensated a group that landed nothing"
    );
    assert_eq!(log(&store).await, pruned);
}

/// SY-11 over SY-40: a group with one new event beside a removed one lands
/// the new event and its compensation once.
///
/// **Red before the record:** `(2, 0, 1)`.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_group_with_one_new_event_lands_it_and_compensates_once() {
    let store = live_store().await;
    let from = peer(0x65);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let (removed, fresh) = (&group[0], &group[1]);
    ingest(&store, &[IngestGroup::new(&group[..1], &[])])
        .await
        .expect("the first event lands");
    remove(&store, &[position_of(&store, removed.id).await]).await;
    let answer = [event("ReservationCompensated", "seat")];

    let landed = ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("a group with one new event lands");

    assert_eq!(counts(landed), (1, 1, 1));
    assert!(holds(&store, fresh.id).await);
    assert!(!holds(&store, removed.id).await);
    assert_eq!(rows(&store).await, 2, "the new event and its answer");
}

/// The trigger records the origin pair of every row deleted through the
/// endpoint, own and foreign, in the record named after this configuration's
/// event table, and the endpoint accepts the ingest's `LOCK` inside its batch
/// transaction (the plan's first risk, measured here).
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_raw_delete_records_each_removed_identity() {
    let store = live_store().await;
    let local = append(&store, &[event("Local", "a")]).await;
    let foreign = [shipped(id(peer(0xF0), 7), 7, event("Shipped", "b"))];
    ingest(&store, &[IngestGroup::new(&foreign, &[])])
        .await
        .expect("an ingest, lock and all, is accepted by the endpoint");
    remove(&store, &[local, position_of(&store, foreign[0].id).await]).await;

    let record = select(
        &store,
        format!(
            "SELECT encode(origin_store, 'base64') AS s, origin_position::text AS p FROM {}",
            qualified_removed(store.config())
        ),
    )
    .await;
    let mut read: Vec<(String, String)> = (0..record.rows.len())
        .map(|row| {
            (
                record.text(row, "s").unwrap_or_default().to_owned(),
                record.text(row, "p").unwrap_or_default().to_owned(),
            )
        })
        .collect();
    read.sort();
    let mut expected = vec![
        (
            BASE64.encode(own(&store).await.to_bytes()),
            local.get().to_string(),
        ),
        (BASE64.encode(peer(0xF0).to_bytes()), "7".to_owned()),
    ];
    expected.sort();
    assert_eq!(read, expected);
}
