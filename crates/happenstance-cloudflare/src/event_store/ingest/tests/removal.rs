//! SY-40 on the Durable Object (#483, #484, ADR-0110): ingest over an object
//! whose rows a retention job deleted through its own SQL, outside every port.
//!
//! The first five tests mirror PR 1's oracle behaviours in
//! `happenstance-sync/src/memory_ingest/removal.rs`, as `happenstance-sqlite`'s
//! `ingest_spike/tests/removal.rs` does. The rest are this adapter's own: a
//! skip-only call writes nothing and spends no position, the record reaches
//! every handle and survives a reopen (I-4), a bookmark restore rolls the
//! record back with the allocator (I-5), and the probe seeks the record's key.
//! The literal positions are **origin** positions, which a peer supplies;
//! every local position is one the store assigned.

use happenstance_core::{EventId, EventStore, SequencePosition, StoreId};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{Opened, connect, local, log, minted, open, raw, remove, scalar};
use crate::event_store::ingest::CloudflareIngestStore;
use crate::event_store::removal::probe_sql;
use crate::event_store::write_group::plan;
use crate::host::{bookmark, restore_bookmark};
use crate::namespace::Tables;
use crate::sql_storage::SqlValue;

/// The peer each test's foreign events come from.
const PEER: StoreId = StoreId::from_bytes([0x6A; 16]);

/// `(appended, skipped, compensated)`, the three counts every test compares.
fn counts(ingested: Ingested) -> (usize, usize, usize) {
    (ingested.appended, ingested.skipped, ingested.compensated)
}

/// The local position of the one event carrying `id`.
async fn position_of(store: &CloudflareIngestStore, id: EventId) -> SequencePosition {
    let held = log(store).await;
    match held
        .iter()
        .filter(|event| event.id == id)
        .collect::<Vec<_>>()[..]
    {
        [event] => event.position,
        ref other => panic!("expected one event carrying {id}, found {other:?}"),
    }
}

/// Appends one local event and returns the position it was given.
async fn append_one(store: &CloudflareIngestStore, data: &[u8]) -> SequencePosition {
    store
        .append(&[local("Local", data, &[])], None)
        .await
        .expect("the append lands")
}

/// #483. A removed foreign event offered again is skipped and counted as
/// skipped: no row comes back, the log is the pruned log, and
/// `contains_event_id` keeps answering `false` (ES-38).
///
/// **Red before the record:** `(1, 2, 0)`, the removed event re-admitted at
/// the tail.
#[wasm_bindgen_test]
async fn a_removed_foreign_event_is_skipped_on_redelivery() {
    let Opened { sql, store, .. } = open();
    let batch = [
        minted(PEER, 1, 1, b"a", &["k:a"]),
        minted(PEER, 2, 2, b"b", &["k:b"]),
        minted(PEER, 3, 3, b"c", &["k:c"]),
    ];
    store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("three foreign events land");
    remove(&sql, &[position_of(&store, batch[1].id).await]);
    let pruned = log(&store).await;

    let again = store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery over a removal is not an error");

    assert_eq!(counts(again), (0, 3, 0), "the removed event came back");
    assert_eq!(log(&store).await, pruned, "the log is the pruned log");
    assert!(
        !store.contains_event_id(batch[1].id).await.expect("a read"),
        "the removed identity reads as held again"
    );
}

/// #484. A removed **own** event offered again is a skip, not
/// `UnheldOwnIdentity`, and the new foreign event beside it in the same call
/// lands. The removed event is the head, so the head read no longer covers
/// it: only the record can tell it from a claim the store never held.
///
/// **Red before the record:** `UnheldOwnIdentity` naming the removed event,
/// class `Stop`, and nothing landed.
#[wasm_bindgen_test]
async fn a_removed_own_event_is_skipped_not_refused() {
    let Opened { sql, store, .. } = open();
    append_one(&store, b"a").await;
    let head = append_one(&store, b"b").await;
    remove(&sql, &[head]);
    let echo = EventId::new(store.store_id().await.expect("the identity reads"), head);
    let batch = [
        happenstance_sync::ReplicatedEvent::new(
            echo,
            happenstance_core::RecordedAt::from_millis(0),
            local("Local", b"b", &[]),
        ),
        minted(PEER, 1, 1, b"y", &[]),
    ];
    let arrived = batch[1].id;

    let landed = store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("a removed own event is a skip, not SY-38's refusal");

    assert_eq!(counts(landed), (1, 1, 0));
    assert!(store.contains_event_id(arrived).await.expect("a read"));
    assert!(
        !store.contains_event_id(echo).await.expect("a read"),
        "the removed own event came back"
    );
}

/// I-3: the record is a set of identities, not "anything unheld is fine". An
/// own claim the store **never** held, past its allocator, is still refused
/// whole as SY-38 says, in an object that has recorded removals.
///
/// Green from the start when the record is probed by identity; it guards
/// against a store that, once anything is removed, skips every unheld own
/// claim (the sync-testkit's `SkipsEveryUnheldOwnIdentity`).
#[wasm_bindgen_test]
async fn an_own_claim_never_held_is_refused_after_a_removal() {
    let Opened { sql, store, .. } = open();
    let first = append_one(&store, b"a").await;
    let head = append_one(&store, b"b").await;
    remove(&sql, &[first]);
    let never = SequencePosition::new(head.get() + 3).expect("three past a fresh head");
    let claim = EventId::new(store.store_id().await.expect("the identity reads"), never);
    let batch = [
        minted(PEER, 1, 1, b"z", &[]),
        happenstance_sync::ReplicatedEvent::new(
            claim,
            happenstance_core::RecordedAt::from_millis(0),
            local("Local", b"forged", &[]),
        ),
    ];
    let (before, raw_before) = (log(&store).await, raw(&sql));

    let refused = store
        .ingest(&[IngestGroup::new(&batch, &[])])
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
    assert_eq!(raw(&sql), raw_before, "and left a row or a position behind");
}

/// SY-11 over SY-40: a group made only of removed and held events lands
/// nothing, so its compensation is not written either.
///
/// **Red before the record:** `(1, 1, 1)`, the removed event re-admitted and
/// answered a second time.
#[wasm_bindgen_test]
async fn a_group_of_removed_and_held_events_writes_no_compensation() {
    let Opened { sql, store, .. } = open();
    let group = [
        minted(PEER, 1, 1, b"seat-1", &[]),
        minted(PEER, 2, 2, b"seat-2", &[]),
    ];
    let answer = [local("ReservationCompensated", b"seat", &[])];
    store
        .ingest(&[IngestGroup::new(&group, &answer)])
        .await
        .expect("the group lands with its compensation");
    remove(&sql, &[position_of(&store, group[0].id).await]);
    let pruned = log(&store).await;

    let again = store
        .ingest(&[IngestGroup::new(&group, &answer)])
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
/// the new event and its compensation once, and skips the removed one.
///
/// **Red before the record:** `(2, 0, 1)`, the removed event landing with the
/// new one.
#[wasm_bindgen_test]
async fn a_group_with_one_new_event_lands_it_and_compensates_once() {
    let Opened { sql, store, .. } = open();
    let group = [
        minted(PEER, 1, 1, b"seat-1", &[]),
        minted(PEER, 2, 2, b"seat-2", &[]),
    ];
    let (removed, fresh) = (&group[0], &group[1]);
    store
        .ingest(&[IngestGroup::new(&group[..1], &[])])
        .await
        .expect("the first event lands");
    remove(&sql, &[position_of(&store, removed.id).await]);
    let answer = [local("ReservationCompensated", b"seat", &[])];

    let landed = store
        .ingest(&[IngestGroup::new(&group, &answer)])
        .await
        .expect("a group with one new event lands");

    assert_eq!(counts(landed), (1, 1, 1));
    assert!(store.contains_event_id(fresh.id).await.expect("a read"));
    assert!(!store.contains_event_id(removed.id).await.expect("a read"));
    assert_eq!(log(&store).await.len(), 2, "the new event and its answer");
}

/// #359's promise over SY-40: a call that only re-offers removed events
/// writes no row and spends no position, because a recorded event is dropped
/// before append's `INSERT` ever stages it. (A held event, by contrast, is
/// staged and discarded, which spends one: `redelivery_is_a_skip_not_an_error`.)
///
/// **Red before the record:** the removed event lands, the log grows and the
/// counter moves.
#[wasm_bindgen_test]
async fn a_skip_against_a_removal_writes_nothing_and_spends_no_position() {
    let Opened { sql, store, .. } = open();
    let batch = [
        minted(PEER, 1, 1, b"a", &["k:a"]),
        minted(PEER, 2, 2, b"b", &["k:b"]),
    ];
    store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("two foreign events land");
    remove(&sql, &[position_of(&store, batch[0].id).await]);
    let before = raw(&sql);

    let again = store
        .ingest(&[IngestGroup::new(&batch[..1], &[])])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(counts(again), (0, 1, 0));
    assert_eq!(
        raw(&sql),
        before,
        "a skip-only call wrote a row or spent a position"
    );
}

/// I-4: the record is in the object's own storage, so it reaches a handle
/// built before the removal, one built after it, and one built on a fresh
/// binding of the object's storage, which is this host's reopen.
///
/// **Named wrong store:** a record kept per handle, as
/// `RecordsRemovalsPerHandle` in the sync-testkit's wrong stores keeps it; the
/// handles built after the removal re-admit the event.
#[wasm_bindgen_test]
async fn the_record_reaches_every_handle_and_survives_reopen() {
    let Opened {
        host,
        sql,
        store: earlier,
    } = open();
    let batch = [minted(PEER, 1, 1, b"a", &[])];
    earlier
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("a foreign event lands");
    remove(&sql, &[position_of(&earlier, batch[0].id).await]);
    let later = connect(&host);

    for (which, store) in [("earlier", &earlier), ("later", &later)] {
        let again = store
            .ingest(&[IngestGroup::new(&batch, &[])])
            .await
            .expect("a re-delivery is not an error");
        assert_eq!(counts(again), (0, 1, 0), "through the {which} handle");
    }
    drop((earlier, later));

    // A fresh binding off the same object's state, as `Fixture::reopen` takes
    // one in `tests/support/mod.rs`.
    let reopened_sql = host.storage();
    let reopened = connect(&host);
    let again = reopened
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!(counts(again), (0, 1, 0), "after reopen");
    assert!(
        !reopened
            .contains_event_id(batch[0].id)
            .await
            .expect("a read")
    );
    assert_eq!(
        scalar(&reopened_sql, "SELECT count(*) FROM event_removed"),
        Some(1),
        "one identity recorded, once"
    );
}

/// I-5: a point-in-time restore rolls back every table in the object, the
/// record with the allocator and the log. Two own events are appended, the
/// bookmark taken between them, and both then removed. In the restored object
/// the first is held again, and the second, minted after the bookmark, is
/// neither held nor recorded, so a claim on it is still SY-38's refusal: a
/// restore under the old identity is the hazard SY-38 names, and a removal is
/// not. The re-mint the crate README prescribes after a restore is skipped
/// here on purpose, because it is what would make the claim foreign.
///
/// **Named wrong record:** one kept outside the object's SQL, which a restore
/// cannot roll back; the restored object would skip the post-bookmark claim as
/// removed.
#[wasm_bindgen_test]
async fn a_restored_bookmark_rolls_the_record_back() {
    let Opened { host, sql, store } = open();
    let own = store.store_id().await.expect("the identity reads");
    let kept = append_one(&store, b"kept").await;
    let mark = bookmark(&sql);
    let after = append_one(&store, b"after").await;
    remove(&sql, &[kept, after]);
    assert_eq!(
        scalar(&sql, "SELECT count(*) FROM event_removed"),
        Some(2),
        "the control: both removals were recorded before the restore"
    );

    restore_bookmark(&sql, &mark);
    let restored_sql = host.storage();
    let restored = connect(&host);
    let (kept_id, after_id) = (EventId::new(own, kept), EventId::new(own, after));
    assert!(restored.contains_event_id(kept_id).await.expect("a read"));
    assert_eq!(
        scalar(&restored_sql, "SELECT count(*) FROM event_removed"),
        Some(0),
        "the record was not rolled back with the object"
    );

    let echo = [happenstance_sync::ReplicatedEvent::new(
        kept_id,
        happenstance_core::RecordedAt::from_millis(0),
        local("Local", b"kept", &[]),
    )];
    let held = restored
        .ingest(&[IngestGroup::new(&echo, &[])])
        .await
        .expect("an echo of a held event is a skip");
    assert_eq!(counts(held), (0, 1, 0));

    let claim = [happenstance_sync::ReplicatedEvent::new(
        after_id,
        happenstance_core::RecordedAt::from_millis(0),
        local("Local", b"after", &[]),
    )];
    let refused = restored
        .ingest(&[IngestGroup::new(&claim, &[])])
        .await
        .expect_err("an own event minted after the bookmark is SY-38's refusal");
    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == after_id),
        "expected UnheldOwnIdentity naming {after_id}, got {refused:?}"
    );
}

/// The probe is a point lookup on the record's primary key, never a scan: a
/// Durable Object bills every row a statement reads, and the record grows by
/// one row per removal for ever (D-5).
///
/// Rejects a probe keyed on one column of the pair, which scans.
#[wasm_bindgen_test]
async fn the_removal_probe_seeks_the_primary_key() {
    let Opened { sql, .. } = open();

    let details = plan(
        &sql,
        &probe_sql(&Tables::UNPREFIXED),
        &[SqlValue::Blob(vec![0x6A; 16]), SqlValue::Integer(1)],
    );

    assert!(
        details
            .iter()
            .any(|detail| detail.starts_with("SEARCH event_removed USING PRIMARY KEY")),
        "the probe does not seek the record's key: {details:?}"
    );
    assert!(
        !details
            .iter()
            .any(|detail| detail.starts_with("SCAN event_removed")),
        "the probe scans the record: {details:?}"
    );
}
