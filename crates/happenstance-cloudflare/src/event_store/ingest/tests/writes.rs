//! The ingest write path, on the Durable Object under the Node shim: what one
//! call writes, what it refuses before writing, and what a throw inside it
//! leaves behind.
//!
//! No test asserts a literal **local** position: those are read from `append`
//! or the log. The literal positions here are origin positions, facts a peer
//! supplies.

use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, EventId, EventStore, Query, QueryItem,
    StoreId, StoreLimit, Tags,
};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, IngestStore, Ingested};
use wasm_bindgen_test::wasm_bindgen_test;

use super::support::{Opened, at, issued, issued_since, local, log, minted, open, raw};
use crate::event_store::ingest::{StoredOrigin, model, stamp_foreign_sql};
use crate::event_store::{Ceilings, CloudflareEventStoreError, insert_statement};
use crate::host::{arm_throw, statements};
use crate::namespace::Tables;
use crate::sql_storage::SqlError;

/// A peer's identity, for the foreign events below.
const PEER: StoreId = StoreId::from_bytes([0x61; 16]);
/// A second peer's.
const OTHER: StoreId = StoreId::from_bytes([0x62; 16]);

/// What an ingest returned, as `(appended, skipped, compensated)`.
fn counts(ingested: Ingested) -> (usize, usize, usize) {
    (ingested.appended, ingested.skipped, ingested.compensated)
}

/// VT-10: a foreign row is written with `append`'s own `INSERT`, byte for
/// byte, and everything else the call issued is the identity read, the
/// ingest-only stamp and the tag rows. Nothing reads the log ahead of the
/// write.
///
/// Rejects a second write path, and a lookup ahead of the stamp.
#[wasm_bindgen_test]
async fn ingest_writes_a_foreign_row_with_appends_insert_text() {
    let Opened { sql, store, .. } = open();
    let event = minted(PEER, 3, 1_700_000_000_000, b"vt10", &["course:c1"]);
    let from = issued(&sql);

    let ingested = store
        .ingest(&[IngestGroup::new(core::slice::from_ref(&event), &[])])
        .await
        .expect("the ingest lands");

    assert_eq!(counts(ingested), (1, 0, 0));
    let ingest = issued_since(&sql, from);
    let Opened {
        sql: twin_sql,
        store: twin,
        ..
    } = open();
    let twin_from = issued(&twin_sql);
    twin.append(core::slice::from_ref(&event.event), None)
        .await
        .expect("the append lands");
    let append_insert = issued_since(&twin_sql, twin_from)
        .into_iter()
        .find(|statement| statement.starts_with("INSERT INTO event ("))
        .expect("an append inserts an event row");
    assert_eq!(append_insert, insert_statement(&Tables::UNPREFIXED));
    assert_eq!(ingest.len(), 4, "{ingest:#?}");
    assert!(ingest[0].starts_with("SELECT v AS store_id FROM store_meta"));
    assert_eq!(ingest[1], append_insert, "append's INSERT, byte for byte");
    assert_eq!(ingest[2], stamp_foreign_sql(&Tables::UNPREFIXED));
    assert!(ingest[3].starts_with("INSERT INTO event_tag "));
}

/// An empty pull writes nothing, and a group of no foreign events writes no
/// compensation (SY-11), so neither issues a statement or opens a transaction.
#[wasm_bindgen_test]
async fn an_empty_ingest_issues_no_statement_and_no_transaction() {
    let Opened { host, sql, store } = open();
    let answer = [local("Compensated", b"never", &[])];
    let from = issued(&sql);

    let none = store.ingest(&[]).await.expect("an empty call succeeds");
    let unanswered = store
        .ingest(&[IngestGroup::new(&[], &answer)])
        .await
        .expect("a group of no events succeeds");

    assert_eq!(counts(none), (0, 0, 0));
    assert_eq!(counts(unanswered), (0, 0, 0));
    assert_eq!(issued(&sql), from, "no statement");
    assert_eq!(model::calls(&host).expect("the model counts"), 0);
}

/// `ingest`'s refusal, which a test expects to be one.
fn refusal(
    outcome: Result<Ingested, IngestError<CloudflareEventStoreError>>,
) -> IngestError<CloudflareEventStoreError> {
    match outcome {
        Ok(ingested) => panic!("the ingest was accepted as {ingested:?}"),
        Err(error) => error,
    }
}

/// The ceilings are checked against the whole call's worst case, foreign
/// events and compensation together, before any statement.
///
/// Rejects counting only the foreign events, checking each group's head
/// only, and checking inside the transaction.
#[wasm_bindgen_test]
async fn ceilings_are_checked_before_any_statement_against_the_worst_case() {
    let Opened { host, sql, store } = open();
    let declared = Ceilings::DECLARED;
    let foreign: Vec<_> = (1..=1_000)
        .map(|n| minted(PEER, n, 0, b"bulk", &[]))
        .collect();
    let answer: Vec<_> = (0..25).map(|_| local("Compensated", b"c", &[])).collect();
    let over_data = [local(
        "Compensated",
        &vec![0_u8; declared.event_data_len + 1],
        &[],
    )];
    let mut heavy = minted(PEER, 2, 0, b"meta", &[]);
    heavy.event = heavy
        .event
        .with_metadata(vec![0_u8; declared.metadata_len + 1]);
    let pair = [minted(PEER, 1, 0, b"light", &[]), heavy];
    let from = issued(&sql);

    let batch = refusal(store.ingest(&[IngestGroup::new(&foreign, &answer)]).await);
    let data = refusal(
        store
            .ingest(&[IngestGroup::new(&foreign[..1], &over_data)])
            .await,
    );
    let metadata = refusal(store.ingest(&[IngestGroup::new(&pair, &[])]).await);

    assert!(
        matches!(
            batch,
            IngestError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: 1_025,
                ..
            }
        ),
        "{batch:?}"
    );
    assert_eq!(batch.class(), FailureClass::Split);
    assert!(
        matches!(data, IngestError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, len, .. } if len == declared.event_data_len + 1),
        "{data:?}"
    );
    assert_eq!(data.class(), FailureClass::Park);
    assert!(
        matches!(metadata, IngestError::ExceedsStoreLimit { limit: StoreLimit::MetadataLen, len, .. } if len == declared.metadata_len + 1),
        "{metadata:?}"
    );
    assert_eq!(issued(&sql), from, "no statement before any refusal");
    assert_eq!(model::calls(&host).expect("the model counts"), 0);
}

/// An origin this runtime cannot bind exactly is refused through the port,
/// `Park`, before any statement.
#[wasm_bindgen_test]
async fn an_unrepresentable_origin_is_refused_before_any_statement() {
    let Opened { sql, store, .. } = open();
    let past = minted(PEER, 9_007_199_254_740_992, 0, b"past", &[]);
    let early = minted(PEER, 1, -9_007_199_254_740_992, b"early", &[]);
    let from = issued(&sql);

    for event in [past, early] {
        let error = refusal(
            store
                .ingest(&[IngestGroup::new(core::slice::from_ref(&event), &[])])
                .await,
        );
        assert!(
            matches!(
                error,
                IngestError::Store(
                    CloudflareEventStoreError::OriginPositionOutOfRange { .. }
                        | CloudflareEventStoreError::RecordedAtOutOfRange { .. }
                )
            ),
            "{error:?}"
        );
        assert_eq!(error.class(), FailureClass::Park);
    }
    assert_eq!(issued(&sql), from, "no statement");
}

/// VT-5, VT-9: an ingested event keeps its origin's identity, its recorded
/// time, a pre-1970 one included, and its bytes.
///
/// Rejects re-minting under this store's identity, and restamping with the
/// arrival time.
#[wasm_bindgen_test]
async fn ingest_preserves_foreign_event_id_and_recorded_at() {
    let Opened { store, .. } = open();
    let sent = [
        minted(PEER, 3, -86_400_000, b"before-1970", &["course:c1"]),
        minted(PEER, 7, 1_700_000_000_000, b"after", &[]),
    ];

    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");

    let held = log(&store).await;
    let read_back: Vec<_> = held
        .iter()
        .map(|row| (row.id, row.recorded_at, row.event.clone()))
        .collect();
    let expected: Vec<_> = sent
        .iter()
        .map(|event| (event.id, event.recorded_at, event.event.clone()))
        .collect();
    assert_eq!(read_back, expected);
}

/// SY-11: a re-delivery is a skip, decided by the write: the same batch twice
/// writes nothing the second time, and a mixed batch appends only the new
/// event. No staged row and no tag row is left behind by a skip.
///
/// Rejects a plain `UPDATE` (a constraint error, not a skip), a missing
/// discard of the staged row, and tags written before the stamp.
#[wasm_bindgen_test]
async fn redelivery_is_a_skip_not_an_error() {
    let Opened { sql, store, .. } = open();
    let batch = [
        minted(PEER, 1, 1, b"one", &["k:1"]),
        minted(PEER, 2, 2, b"two", &["k:2"]),
    ];
    let first = store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("the first delivery lands");
    let after_first = (log(&store).await, store.head().await.expect("a head"));
    let raw_first = raw(&sql);

    let again = store
        .ingest(&[IngestGroup::new(&batch, &[])])
        .await
        .expect("a re-delivery is not an error");
    let mixed = [batch[1].clone(), minted(PEER, 3, 3, b"three", &[])];
    let raw_again = raw(&sql);
    let partly = store
        .ingest(&[IngestGroup::new(&mixed, &[])])
        .await
        .expect("a mixed delivery lands");

    assert_eq!(counts(first), (2, 0, 0));
    assert_eq!(counts(again), (0, 2, 0));
    assert_eq!(
        (log(&store).await.len(), after_first.0.len()),
        (3, 2),
        "only the new event of the mixed batch was added"
    );
    assert_eq!(counts(partly), (1, 1, 0));
    assert_eq!(
        (raw_again.events, raw_again.tags, raw_again.unstamped),
        (raw_first.events, raw_first.tags, Some(0)),
        "a skip leaves no staged row and no tag row"
    );
    assert!(
        raw_again.sequence > raw_first.sequence,
        "a skip spends a position, which is a permitted gap"
    );
}

/// An in-call duplicate stamps once and skips once.
#[wasm_bindgen_test]
async fn the_same_event_twice_in_one_call_lands_once() {
    let Opened { store, .. } = open();
    let event = minted(PEER, 5, 5, b"twice", &[]);
    let twice = [event.clone(), event];

    let ingested = store
        .ingest(&[IngestGroup::new(&twice, &[])])
        .await
        .expect("the ingest lands");

    assert_eq!(counts(ingested), (1, 1, 0));
    assert_eq!(log(&store).await.len(), 1);
}

/// SY-5: ingested events land at the tail, above every position assigned
/// before the call, and the next append lands above them.
///
/// Rejects writing an event at its origin's position.
#[wasm_bindgen_test]
async fn ingested_events_land_above_every_position_assigned_before_the_call() {
    let Opened { store, .. } = open();
    let mut last = None;
    for n in 0..3 {
        last = Some(
            store
                .append(&[local("Local", format!("l{n}").as_bytes(), &[])], None)
                .await
                .expect("the append lands"),
        );
    }
    let last = last.expect("three appends");
    let sent = [
        minted(PEER, 1, 1, b"low-1", &[]),
        minted(PEER, 2, 2, b"low-2", &[]),
    ];

    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");
    let next = store
        .append(&[local("Local", b"after", &[])], None)
        .await
        .expect("the append lands");

    let held = log(&store).await;
    let landed: Vec<_> = held
        .iter()
        .filter(|row| row.id.store() == PEER)
        .map(|row| row.position)
        .collect();
    assert_eq!(landed.len(), 2);
    assert!(
        landed.iter().all(|position| *position > last),
        "{landed:?} above {last}"
    );
    assert!(
        landed.iter().all(|position| *position < next),
        "{landed:?} below {next}"
    );
}

/// SY-2, SY-11: a group's compensation is written with it, above it, under
/// this store's identity at its own position, and not again on redelivery.
#[wasm_bindgen_test]
async fn compensation_is_atomic_with_the_losing_event_and_skipped_on_redelivery() {
    let Opened { store, .. } = open();
    let own = store.store_id().await.expect("the identity reads");
    let losing = [minted(PEER, 4, 4, b"losing", &[])];
    let answer = [local("Compensated", b"answer", &[])];

    let first = store
        .ingest(&[IngestGroup::new(&losing, &answer)])
        .await
        .expect("the group lands");
    let again = store
        .ingest(&[IngestGroup::new(&losing, &answer)])
        .await
        .expect("a re-delivery is not an error");

    assert_eq!(counts(first), (1, 0, 1));
    assert_eq!(counts(again), (0, 1, 0));
    let held = log(&store).await;
    let [event, compensation] = held.as_slice() else {
        panic!("one event and one compensation: {held:?}");
    };
    assert_eq!(event.id, losing[0].id);
    assert!(compensation.position > event.position);
    assert_eq!(compensation.id, EventId::new(own, compensation.position));
    assert_eq!(compensation.event, answer[0]);
}

/// SY-2, SY-30: a throw inside the call takes every row of it with it, the
/// `AUTOINCREMENT` counter included, and the call fails `Stop`.
///
/// Two arrangements: the throw on the compensation's identity stamp, after
/// group 1 has landed, and on a tag row, after an earlier foreign row of the
/// same group was stamped. **The central mutant** is this code without
/// `Transactions::run`, which leaves group 1's stamped rows behind: append's
/// single-stamp atomicity alone does not cover a call of several stamps.
#[wasm_bindgen_test]
async fn a_throw_inside_the_call_takes_every_row_and_the_counter_with_it() {
    for (armed, groups) in [
        (
            "SET origin_store = ?, origin_position = position",
            vec![
                (vec![minted(PEER, 1, 1, b"g1", &[])], vec![]),
                (
                    vec![minted(PEER, 2, 2, b"g2", &[])],
                    vec![local("Compensated", b"answer", &[])],
                ),
            ],
        ),
        (
            "INSERT INTO event_tag ",
            vec![(
                vec![
                    minted(OTHER, 1, 1, b"untagged", &[]),
                    minted(OTHER, 2, 2, b"tagged", &["k:1"]),
                ],
                vec![],
            )],
        ),
    ] {
        let Opened { sql, store, .. } = open();
        store
            .append(&[local("Local", b"before", &[])], None)
            .await
            .expect("the append lands");
        let before = (log(&store).await, raw(&sql));
        let groups: Vec<_> = groups
            .iter()
            .map(|(events, answer)| IngestGroup::new(events, answer))
            .collect();
        arm_throw(&sql, armed, "the storage failed mid-call");

        let error = refusal(store.ingest(&groups).await);

        assert!(
            matches!(
                error,
                IngestError::Store(CloudflareEventStoreError::Sql(SqlError::Thrown(_)))
            ),
            "{armed}: {error:?}"
        );
        assert_eq!(error.class(), FailureClass::Stop);
        assert_eq!(
            (log(&store).await, raw(&sql)),
            before,
            "{armed}: nothing stayed"
        );
        for group in &groups {
            for event in group.events {
                assert!(
                    !store
                        .contains_event_id(event.id)
                        .await
                        .expect("membership reads"),
                    "{armed}: {} stayed held",
                    event.id
                );
            }
        }
    }
}

/// A later append never restamps a foreign row: the `IS NULL` marker keeps
/// append's identity stamp off it, and a local row's identity is its own
/// position.
#[wasm_bindgen_test]
async fn a_later_append_never_restamps_a_foreign_row() {
    let Opened { store, .. } = open();
    let own = store.store_id().await.expect("the identity reads");
    let sent = [minted(PEER, 9, 9, b"foreign", &[])];
    store
        .ingest(&[IngestGroup::new(&sent, &[])])
        .await
        .expect("the ingest lands");

    store
        .append(
            &[local("Local", b"a", &[]), local("Local", b"b", &[])],
            None,
        )
        .await
        .expect("the append lands");

    for row in log(&store).await {
        if row.event == sent[0].event {
            assert_eq!(row.id, sent[0].id);
        } else {
            assert_eq!(row.id, EventId::new(own, row.position));
        }
    }
}

/// 2^53 − 1 is the largest origin position this runtime binds exactly, and
/// it lands and reads back. With the refusal one above it, it brackets the
/// boundary.
#[wasm_bindgen_test]
async fn the_largest_safe_origin_position_lands() {
    let Opened { store, .. } = open();
    let edge = [minted(PEER, 9_007_199_254_740_991, 0, b"edge", &[])];

    store
        .ingest(&[IngestGroup::new(&edge, &[])])
        .await
        .expect("2^53 - 1 lands");

    let held = log(&store).await;
    assert_eq!(held.len(), 1);
    assert_eq!(held[0].id, edge[0].id);
}

/// SY-1: no append condition is evaluated on ingest, and the ingested event
/// is a conflict for a local append afterwards.
#[wasm_bindgen_test]
async fn ingest_ignores_append_conditions() {
    let Opened { store, .. } = open();
    let shipped = QueryItem::new(["Shipped"], Tags::empty()).expect("a valid item");
    let condition = AppendCondition::new(Query::from_item(shipped));
    let taken = [minted(PEER, 1, 1, b"seat", &[])];

    store
        .ingest(&[IngestGroup::new(&taken, &[])])
        .await
        .expect("an ingest evaluates no condition");
    let refused = store
        .append(&[local("Shipped", b"mine", &[])], Some(&condition))
        .await;

    let landed = log(&store).await[0].position;
    assert!(
        matches!(
            refused,
            Err(AppendError::ConditionViolated(ConditionViolated {
                conflicting_position: Some(position),
                ..
            })) if position == landed
        ),
        "{refused:?}"
    );
}

/// One non-empty call is one `transactionSync`, never one per group, and the
/// adapter itself issues no transaction control, which `workerd` refuses
/// through `exec`.
#[wasm_bindgen_test]
async fn one_call_is_one_transaction_and_issues_no_transaction_control() {
    let Opened { host, sql, store } = open();
    let first = [minted(PEER, 1, 1, b"a", &[])];
    let second = [minted(PEER, 2, 2, b"b", &[])];
    let answer = [local("Compensated", b"c", &[])];
    let from = issued(&sql);

    store
        .ingest(&[
            IngestGroup::new(&first, &[]),
            IngestGroup::new(&second, &answer),
        ])
        .await
        .expect("the ingest lands");
    store
        .ingest(&[IngestGroup::new(&first, &[])])
        .await
        .expect("a re-delivery lands");

    assert_eq!(model::calls(&host).expect("the model counts"), 2);
    let all = statements(&sql);
    assert!(
        all[from..]
            .iter()
            .any(|statement| model::STATEMENTS.contains(&statement.as_str())),
        "the control: the model's statements are in the log, so the filter has work"
    );
    for statement in issued_since(&sql, from) {
        let upper = statement.to_ascii_uppercase();
        for control in ["BEGIN", "SAVEPOINT", "RELEASE", "ROLLBACK", "COMMIT"] {
            assert!(
                !upper.starts_with(control),
                "the adapter issued transaction control: {statement}"
            );
        }
    }
}

/// The two ingest-only statements are guarded on the `IS NULL` marker: the
/// stamp cannot restamp an event, and the discard cannot delete one.
#[wasm_bindgen_test]
async fn the_ingest_only_statements_touch_only_rows_that_are_not_events() {
    let Opened { store, .. } = open();
    let own = store.store_id().await.expect("the identity reads");
    let position = store
        .append(&[local("Local", b"held", &[])], None)
        .await
        .expect("the append lands");
    let foreign = StoredOrigin {
        store: PEER,
        position: 1,
    };

    let restamped = store
        .stamp_foreign(position, foreign)
        .expect("the stamp statement runs");
    store
        .discard_staged(position)
        .expect("the discard statement runs");

    assert!(!restamped, "an event is never restamped");
    let held = log(&store).await;
    assert_eq!(held.len(), 1, "an event is never discarded");
    assert_eq!(held[0].id, EventId::new(own, at(position.get())));
}
