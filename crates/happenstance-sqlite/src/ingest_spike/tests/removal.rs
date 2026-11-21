//! SY-40 on SQLite (#483, #484, ADR-0110): ingest over a store whose rows an
//! operator deleted with a raw `DELETE`, on a connection the store never saw.
//!
//! The first five tests mirror PR 1's oracle behaviours in
//! `happenstance-sync/src/memory_ingest/removal.rs`. The rest are this
//! adapter's own: the #359 promise that a skip-only call leaves the file
//! byte-identical, the record reaching every handle and surviving reopen
//! (I-4), and a restore from a file copy rolling the record back with the
//! allocator (I-5). The literal positions are **origin** positions, which a
//! peer supplies; every local position is one the store assigned.

use happenstance_core::{EventId, EventStore, SequencePosition};
use happenstance_sync::{FailureClass, IngestError, IngestGroup};

use super::{TempFile, carrying, event, id, ingest, log, peer, shipped};
use crate::event_store::SqliteEventStore;
use happenstance_sync::Classify;

/// Deletes the events at `positions` in one transaction on a second
/// connection, as an operator's retention script would: tag rows first,
/// because `event_tag.position REFERENCES event(position)`, the tag statistic
/// decremented with them, then the event rows. The shape
/// `tests/support/mod.rs`'s `remove_outside_port` uses, and the ingest
/// fixture's `remove_outside_port` calls. The connection waits out a writer
/// for the shipped busy timeout rather than failing at once.
pub(super) fn remove(file: &TempFile, positions: &[SequencePosition]) {
    // `inspect` waits out a writer for the shipped busy timeout.
    let mut operator = file.inspect();
    operator
        .pragma_update(None, "foreign_keys", "ON")
        .expect("the operator's connection enforces foreign keys");
    let transaction = operator
        .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
        .expect("the removal takes the write lock");
    for position in positions {
        let position = i64::try_from(position.get()).expect("a SQLite position fits i64");
        transaction
            .execute(
                "UPDATE tag_cardinality SET events = events - 1 \
                 WHERE tag IN (SELECT tag FROM event_tag WHERE position = ?1)",
                [position],
            )
            .expect("the tag statistic is decremented");
        transaction
            .execute("DELETE FROM event_tag WHERE position = ?1", [position])
            .expect("the tag rows go");
        let deleted = transaction
            .execute("DELETE FROM event WHERE position = ?1", [position])
            .expect("the event row goes");
        assert_eq!(deleted, 1, "position {position} was held");
    }
    transaction.commit().expect("the removal commits");
}

/// The local position of the one event carrying `id`.
async fn position_of(store: &SqliteEventStore, id: EventId) -> SequencePosition {
    carrying(&log(store).await, id).position
}

/// `(appended, skipped, compensated)`, the three counts every test compares.
fn counts(ingested: happenstance_sync::Ingested) -> (usize, usize, usize) {
    (ingested.appended, ingested.skipped, ingested.compensated)
}

/// #483. A removed foreign event offered again is skipped and counted as
/// skipped: no row comes back, the log is the pruned log, and
/// `contains_event_id` keeps answering `false` (ES-38).
///
/// **Red before the record:** `(1, 2, 0)`, the removed event re-admitted at
/// the tail.
#[tokio::test]
async fn a_removed_foreign_event_is_skipped_on_redelivery() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let from = peer(0x61);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
        shipped(id(from, 3), 3, event("Shipped", "c")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("three foreign events land");
    remove(&file, &[position_of(&store, batch[1].id).await]);
    let pruned = log(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&batch, &[])])
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
/// lands.
///
/// **Red before the record:** `UnheldOwnIdentity` naming the removed event,
/// class `Stop`, and nothing landed.
#[tokio::test]
async fn a_removed_own_event_is_skipped_not_refused() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let first = store
        .append(&[event("Local", "a")], None)
        .await
        .expect("an append lands");
    store
        .append(&[event("Local", "b")], None)
        .await
        .expect("an append lands");
    remove(&file, &[first]);
    let echo = EventId::new(store.store_id(), first);
    let arrived = id(peer(0x62), 1);
    let batch = [
        shipped(echo, 0, event("Local", "a")),
        shipped(arrived, 1, event("Shipped", "y")),
    ];

    let landed = ingest(&store, &[IngestGroup::new(&batch, &[])])
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
/// whole as SY-38 says, in a store that has recorded removals.
///
/// Green from the start when the record is probed by identity; it guards
/// against a store that, once anything is removed, skips every unheld own
/// claim (the sync-testkit's `SkipsEveryUnheldOwnIdentity`).
#[tokio::test]
async fn an_own_claim_never_held_is_refused_after_a_removal() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let first = store
        .append(&[event("Local", "a")], None)
        .await
        .expect("an append lands");
    let head = store
        .append(&[event("Local", "b")], None)
        .await
        .expect("an append lands");
    remove(&file, &[first]);
    let never = head
        .next()
        .and_then(SequencePosition::next)
        .and_then(SequencePosition::next)
        .expect("three past a fresh head exist");
    let claim = EventId::new(store.store_id(), never);
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
/// **Red before the record:** `(1, 1, 1)`, the removed event re-admitted and
/// answered a second time.
#[tokio::test]
async fn a_group_of_removed_and_held_events_writes_no_compensation() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let from = peer(0x64);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let answer = [event("ReservationCompensated", "seat")];
    ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("the group lands with its compensation");
    remove(&file, &[position_of(&store, group[0].id).await]);
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
/// the new event and its compensation once, and skips the removed one.
///
/// **Red before the record:** `(2, 0, 1)`, the removed event landing with the
/// new one.
#[tokio::test]
async fn a_group_with_one_new_event_lands_it_and_compensates_once() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let from = peer(0x65);
    let group = [
        shipped(id(from, 1), 1, event("Reserved", "seat-1")),
        shipped(id(from, 2), 2, event("Reserved", "seat-2")),
    ];
    let (removed, fresh) = (&group[0], &group[1]);
    ingest(&store, &[IngestGroup::new(&group[..1], &[])])
        .await
        .expect("the first event lands");
    remove(&file, &[position_of(&store, removed.id).await]);
    let answer = [event("ReservationCompensated", "seat")];

    let landed = ingest(&store, &[IngestGroup::new(&group, &answer)])
        .await
        .expect("a group with one new event lands");

    assert_eq!(counts(landed), (1, 1, 1));
    assert!(store.contains_event_id(fresh.id).await.expect("a read"));
    assert!(!store.contains_event_id(removed.id).await.expect("a read"));
    assert_eq!(log(&store).await.len(), 2, "the new event and its answer");
}

/// The file's bytes and the allocator's value, compared before and after a
/// call that should change neither.
fn snapshot(file: &TempFile) -> (Vec<u8>, i64) {
    let connection = file.inspect();
    // Folds the WAL into the main file, so the bytes read are the store's.
    connection
        .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
        .expect("the WAL checkpoints");
    let sequence: i64 = connection
        .query_row(
            "SELECT seq FROM sqlite_sequence WHERE name = 'event'",
            [],
            |row| row.get(0),
        )
        .expect("the allocator has a row once anything was appended");
    drop(connection);
    (std::fs::read(&file.0).expect("the file reads"), sequence)
}

/// #359 over SY-40: a call that only re-offers removed events lands nothing,
/// burns no `AUTOINCREMENT` value and leaves the file byte-identical, because
/// a recorded event never reaches the insert and the call rolls back. A later
/// append is assigned the position it would have had without the call.
///
/// **Red before the record:** the removed event lands, the log grows and the
/// allocator moves.
#[tokio::test]
async fn a_skip_against_a_removal_burns_no_position_and_leaves_the_file_unchanged() {
    let file = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let from = peer(0x66);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "a")),
        shipped(id(from, 2), 2, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two foreign events land");
    let removed_at = position_of(&store, batch[0].id).await;
    remove(&file, &[removed_at]);
    let before = snapshot(&file);

    let again = ingest(&store, &[IngestGroup::new(&batch[..1], &[])])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(counts(again), (0, 1, 0));
    assert_eq!(
        snapshot(&file),
        before,
        "a skip-only call changed the file or burned a position"
    );
}

/// I-4: the record is in the file, so it reaches a handle connected before
/// the removal, one connected after it, and one opened after every handle was
/// dropped.
///
/// **Named wrong store:** a record kept per handle, as
/// `RecordsRemovalsPerHandle` in the sync-testkit's wrong stores keeps it;
/// the handles opened after the removal re-admit the event.
#[tokio::test]
async fn the_record_reaches_every_handle_and_survives_reopen() {
    let file = TempFile::new("ingest-removal");
    let earlier = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let batch = [shipped(id(peer(0x67), 1), 1, event("Shipped", "a"))];
    ingest(&earlier, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a foreign event lands");
    remove(&file, &[position_of(&earlier, batch[0].id).await]);
    let later = SqliteEventStore::open(&file.0).expect("a second handle opens");

    for (which, store) in [("earlier", &earlier), ("later", &later)] {
        let again = ingest(store, &[IngestGroup::new(&batch, &[])])
            .await
            .expect("a re-delivery is not an error");
        assert_eq!(counts(again), (0, 1, 0), "through the {which} handle");
    }
    drop((earlier, later));

    let reopened = SqliteEventStore::open(&file.0).expect("the file reopens");
    let again = ingest(&reopened, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!(counts(again), (0, 1, 0), "after reopen");
    assert!(
        !reopened
            .contains_event_id(batch[0].id)
            .await
            .expect("a read")
    );
}

/// I-5: a restore from a copy of the file rolls the record back with the
/// allocator and the log, because it is one file. Two own events are
/// appended, the backup taken between them, and both then removed. In the
/// restored store the first is held again, and the second, minted after the
/// backup, is neither held nor recorded, so a claim on it is still SY-38's
/// refusal: a restore under the old identity is the hazard SY-38 names, and a
/// removal is not.
///
/// **Named wrong record:** one kept outside the file, which a file copy cannot
/// roll back; the restored store would skip the post-backup claim as removed.
#[tokio::test]
async fn a_restored_copy_rolls_the_record_back_with_the_allocator() {
    let file = TempFile::new("ingest-removal");
    let backup = TempFile::new("ingest-removal");
    let store = SqliteEventStore::open(&file.0).expect("a fresh file opens");
    let kept = store
        .append(&[event("Local", "kept")], None)
        .await
        .expect("an append lands");
    file.inspect()
        .execute(
            "VACUUM INTO ?1",
            [backup.0.to_str().expect("a UTF-8 temp path")],
        )
        .expect("the backup is taken");
    let after = store
        .append(&[event("Local", "after")], None)
        .await
        .expect("an append lands");
    remove(&file, &[kept, after]);
    drop(store);

    // The restore: the copy put back in place of the file, sidecars and all.
    file.remove();
    std::fs::copy(&backup.0, &file.0).expect("the backup is restored");
    let restored = SqliteEventStore::open(&file.0).expect("the restored file opens");
    let kept_id = EventId::new(restored.store_id(), kept);
    let after_id = EventId::new(restored.store_id(), after);
    assert!(restored.contains_event_id(kept_id).await.expect("a read"));
    let removals: i64 = file
        .inspect()
        .query_row("SELECT count(*) FROM event_removed", [], |row| row.get(0))
        .expect("the record reads");
    assert_eq!(removals, 0, "the record was not rolled back with the file");

    let echo = [shipped(kept_id, 0, event("Local", "kept"))];
    let held = ingest(&restored, &[IngestGroup::new(&echo, &[])])
        .await
        .expect("an echo of a held event is a skip");
    assert_eq!(counts(held), (0, 1, 0));

    let claim = [shipped(after_id, 0, event("Local", "after"))];
    let refused = ingest(&restored, &[IngestGroup::new(&claim, &[])])
        .await
        .expect_err("an own event minted after the backup is SY-38's refusal");
    assert!(
        matches!(refused, IngestError::UnheldOwnIdentity { id, .. } if id == after_id),
        "expected UnheldOwnIdentity naming {after_id}, got {refused:?}"
    );
}
