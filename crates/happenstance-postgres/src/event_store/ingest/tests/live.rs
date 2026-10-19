//! The port's behaviour against a live server: VT-5, SY-1, SY-2, SY-5, SY-11
//! and the watermark.
//!
//! Every test reads right after it ingests, as most of the event-store suite
//! does after it appends. Under the frontier that relies on a quiet server and
//! `--test-threads=1`, which is how CI runs them.

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, QueryItem, SequencedEvent,
    Tags,
};
use happenstance_sync::{Classify, FailureClass, IngestError, IngestGroup, Watermark};

use super::super::WATERMARK_SQL;
use super::{
    append, carrying, event, holds, id, ingest, live_store, log, peer, rows, shipped, watermark,
};
use crate::error::PostgresEventStoreError;

/// VT-5 and VT-9: an ingested event keeps its origin's `EventId` and
/// `RecordedAt` exactly, one of them before 1970.
///
/// Rejects re-minting, and restamping with the arrival time.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn ingest_preserves_foreign_event_id_and_recorded_at() {
    let store = live_store().await;
    let from = peer(0xA1);
    let batch = [
        shipped(id(from, 3), -86_400_000, event("Shipped", "a")),
        shipped(id(from, 7), 1_700_000_000_000, event("Shipped", "b")),
    ];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");
    assert_eq!(ingested.appended, 2);

    let log = log(&store).await;
    for sent in &batch {
        let held = carrying(&log, sent.id);
        assert_eq!(held.id, sent.id, "the origin identity was re-minted");
        assert_eq!(
            held.recorded_at, sent.recorded_at,
            "the origin's recorded time was restamped"
        );
        assert_eq!(held.event, sent.event);
    }
}

/// SY-11: the same batch again is a skip, not an error, and writes no row; a
/// batch half held and half new lands the new half.
///
/// Rejects raising on the unique violation, and an `ON CONFLICT` with no
/// target or the wrong one, which the server refuses or which skips nothing.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn redelivery_is_a_skip_not_an_error() {
    let store = live_store().await;
    let from = peer(0xB2);
    let first = [
        shipped(id(from, 1), 10, event("Shipped", "a")),
        shipped(id(from, 2), 20, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&first, &[])])
        .await
        .expect("the first delivery lands");
    let before = rows(&store).await;

    let again = ingest(&store, &[IngestGroup::new(&first, &[])])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!((again.appended, again.skipped), (0, 2));
    assert_eq!(rows(&store).await, before, "a re-delivery wrote a row");

    let mixed = [
        first[1].clone(),
        shipped(id(from, 3), 30, event("Shipped", "c")),
    ];
    let partial = ingest(&store, &[IngestGroup::new(&mixed, &[])])
        .await
        .expect("a half-held batch is not an error");
    assert_eq!((partial.appended, partial.skipped), (1, 1));
    assert_eq!(rows(&store).await, before + 1);
}

/// The same event twice in one call lands once and counts once as a skip, as
/// `MemoryIngestStore` answers.
///
/// Rejects a writer that errors on a duplicate inside one statement.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn the_same_event_twice_in_one_call_lands_once() {
    let store = live_store().await;
    let twice = shipped(id(peer(0xB3), 5), 5, event("Shipped", "a"));
    let batch = [twice.clone(), twice];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a duplicate inside one call is a skip");
    assert_eq!((ingested.appended, ingested.skipped), (1, 1));
    assert_eq!(rows(&store).await, 1);
}

/// SY-5: ingested events land above every position assigned before the call,
/// whatever positions their origin gave them, and the next append lands above
/// them.
///
/// Compared against the position `append` returned, not against `head`, which
/// is a frontier here. Rejects writing at the origin's position.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn ingested_events_land_above_every_position_assigned_before_the_call() {
    let store = live_store().await;
    let assigned = append(
        &store,
        &[
            event("Local", "a"),
            event("Local", "b"),
            event("Local", "c"),
        ],
    )
    .await;

    let from = peer(0xC3);
    let batch = [
        shipped(id(from, 1), 1, event("Shipped", "x")),
        shipped(id(from, 2), 2, event("Shipped", "y")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");

    let log = log(&store).await;
    let landed: Vec<_> = batch
        .iter()
        .map(|sent| carrying(&log, sent.id).position)
        .collect();
    for position in &landed {
        assert!(
            *position > assigned,
            "ingested at {position}, below {assigned}"
        );
    }
    let next = append(&store, &[event("Local", "d")]).await;
    assert!(
        landed.iter().all(|position| next > *position),
        "{next} vs {landed:?}"
    );
}

/// SY-2 and SY-11: compensation lands with the losing event, after it, minted
/// here; a re-delivery writes no second compensation; and a compensation over
/// a ceiling writes neither half.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn compensation_is_atomic_with_the_losing_event_and_skipped_on_redelivery() {
    let store = live_store().await;
    let from = peer(0xD4);
    let losing = [shipped(id(from, 5), 50, event("Reserved", "seat-1"))];
    let compensation = [event("ReservationCompensated", "seat-1")];

    let first = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
        .await
        .expect("a new losing event and its compensation land");
    assert_eq!(
        (first.appended, first.skipped, first.compensated),
        (1, 0, 1)
    );

    let after = log(&store).await;
    assert_eq!(after.len(), 2);
    let lost = carrying(&after, losing[0].id);
    let answer = after
        .iter()
        .find(|held| held.event == compensation[0])
        .expect("the compensation is held");
    assert!(answer.position > lost.position);
    assert_eq!(
        answer.id,
        EventId::new(store.store_id, answer.position),
        "compensation is minted here, like any local append"
    );

    let again = ingest(&store, &[IngestGroup::new(&losing, &compensation)])
        .await
        .expect("a re-delivery is not an error");
    assert_eq!(
        (again.appended, again.skipped, again.compensated),
        (0, 1, 0)
    );
    assert_eq!(rows(&store).await, 2, "compensation written twice");

    let never_fits = [Event::new(
        "ReservationCompensated",
        vec![0_u8; crate::event_store::PostgresEventStore::MAX_EVENT_DATA_LEN + 1],
    )
    .expect("a valid event type")];
    let doomed = [shipped(id(from, 6), 60, event("Reserved", "seat-2"))];
    let refused = ingest(&store, &[IngestGroup::new(&doomed, &never_fits)]).await;
    assert!(
        matches!(
            refused,
            Err(IngestError::ExceedsStoreLimit {
                limit: happenstance_core::StoreLimit::EventDataLen,
                ..
            })
        ),
        "{refused:?}"
    );
    assert!(!holds(&store, doomed[0].id).await);
}

/// SY-2 inside the transaction: the losing event is inserted, the compensation
/// insert then fails, and the losing event goes with it.
///
/// The fault is a `BEFORE INSERT` trigger installed in this store's schema,
/// because nothing a caller can put in an `Event` fails once the ceilings
/// pass. Rejects a writer that commits per group half.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_compensation_that_fails_in_the_transaction_takes_the_losing_event_with_it() {
    let store = live_store().await;
    sqlx::raw_sql(
        "CREATE FUNCTION poison() RETURNS trigger LANGUAGE plpgsql AS \
         $$ BEGIN RAISE EXCEPTION 'poisoned compensation'; END $$; \
         CREATE TRIGGER poison BEFORE INSERT ON event FOR EACH ROW \
         WHEN (NEW.event_type = 'Poisoned') EXECUTE FUNCTION poison();",
    )
    .execute(store.store.pool())
    .await
    .expect("the fault installs");

    let losing = [shipped(id(peer(0xE5), 1), 1, event("Reserved", "seat"))];
    let poisoned = [event("Poisoned", "seat")];
    let failed = ingest(&store, &[IngestGroup::new(&losing, &poisoned)])
        .await
        .expect_err("the poisoned compensation fails");

    match &failed {
        IngestError::Store(PostgresEventStoreError::Driver(sqlx::Error::Database(database))) => {
            assert_eq!(database.code().as_deref(), Some("P0001"));
        }
        other => panic!("expected the trigger's raise, got {other:?}"),
    }
    assert_eq!(failed.class(), FailureClass::Stop);
    assert!(
        !holds(&store, losing[0].id).await,
        "the losing event committed without its compensation"
    );
    assert_eq!(rows(&store).await, 0);
}

/// A local append after an ingest leaves the foreign identities as they came,
/// and stamps its own rows with this store's identity at their own position.
///
/// A pin, not evidence of any marker: Postgres binds each row's origin in the
/// statement that inserts it, so there is no stamp step to get wrong.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn a_local_append_after_ingest_does_not_restamp_foreign_rows() {
    let store = live_store().await;
    let from = peer(0xF6);
    let batch = [
        shipped(id(from, 4), 4, event("Shipped", "a")),
        shipped(id(from, 8), 8, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");
    append(&store, &[event("Local", "a"), event("Local", "b")]).await;

    let log = log(&store).await;
    for sent in &batch {
        assert_eq!(carrying(&log, sent.id).id, sent.id);
    }
    let local: Vec<&SequencedEvent> = log
        .iter()
        .filter(|held| held.id.store() == store.store_id)
        .collect();
    assert_eq!(local.len(), 2);
    for held in local {
        assert_eq!(held.id.position(), held.position);
    }
}

/// The largest origin position a `bigint` holds lands and reads back. With
/// `an_origin_position_past_i64_is_refused_not_saturated` it brackets the
/// conversion; rejects an off-by-one bound.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn the_largest_bigint_origin_position_lands() {
    let store = live_store().await;
    let largest = u64::try_from(i64::MAX).expect("i64::MAX is positive");
    let batch = [shipped(id(peer(0x07), largest), 1, event("Shipped", "a"))];

    let ingested = ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("i64::MAX is a bigint");
    assert_eq!(ingested.appended, 1);
    assert_eq!(carrying(&log(&store).await, batch[0].id).id, batch[0].id);
}

/// `contains_event_id` answers for a foreign identity, and not for another
/// origin at the same position, nor for this store's identity at the row's
/// local position: a row carries one identity.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn contains_event_id_answers_for_a_foreign_id() {
    let store = live_store().await;
    let sent = id(peer(0x17), 42);
    assert!(!holds(&store, sent).await);

    let batch = [shipped(sent, 1, event("Shipped", "a"))];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("a new foreign event lands");

    assert!(holds(&store, sent).await);
    assert!(!holds(&store, id(peer(0x18), 42)).await);
    let landed = carrying(&log(&store).await, sent).position;
    assert!(!holds(&store, EventId::new(store.store_id, landed)).await);
}

/// SY-1: an ingest evaluates no condition, including one every append would
/// violate now, and a conditional append after it sees the ingested event as
/// the conflict.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn ingest_ignores_append_conditions() {
    let store = live_store().await;
    let seat = Query::from_item(
        QueryItem::tagged(Tags::from_pairs([("k", "seat-9")]).expect("a valid tag"))
            .expect("a tagged item is a valid query item"),
    );
    let decided_at = append(&store, &[event("Reserved", "seat-9")]).await;

    let taken = AppendCondition::new(seat.clone());
    let violated = EventStore::append(&store, &[event("Reserved", "seat-9")], Some(&taken))
        .await
        .expect_err("the seat is taken");
    assert!(violated.is_condition_violated(), "{violated:?}");

    let rival = [shipped(id(peer(0x29), 3), 3, event("Reserved", "seat-9"))];
    let ingested = ingest(&store, &[IngestGroup::new(&rival, &[])])
        .await
        .expect("an ingest evaluates no condition");
    assert_eq!(ingested.appended, 1);

    let landed = carrying(&log(&store).await, rival[0].id).position;
    let since_decision = AppendCondition::new(seat).after_every_guard(decided_at);
    match EventStore::append(
        &store,
        &[event("Reserved", "seat-9")],
        Some(&since_decision),
    )
    .await
    {
        Err(AppendError::ConditionViolated(violated)) => {
            assert_eq!(violated.conflicting_position, Some(landed));
        }
        other => panic!("expected the ingested event as the conflict, got {other:?}"),
    }
}

/// The watermark is the highest origin position held per origin store, this
/// store's own included, and empty on a fresh store.
///
/// The oracle is the log read back and folded here, which shares nothing with
/// the `GROUP BY`, and each foreign mark is also held against the maximum
/// sent.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn watermark_is_max_origin_position_per_origin() {
    let store = live_store().await;
    assert!(watermark(&store).await.is_empty());

    let (one, two) = (peer(0x31), peer(0x32));
    let batch = [
        shipped(id(one, 2), 2, event("Shipped", "a")),
        shipped(id(one, 9), 9, event("Shipped", "b")),
        shipped(id(one, 5), 5, event("Shipped", "c")),
        shipped(id(two, 4), 4, event("Shipped", "d")),
    ];
    ingest(
        &store,
        &[
            IngestGroup::new(&batch[..2], &[]),
            IngestGroup::new(&batch[2..], &[]),
        ],
    )
    .await
    .expect("four new foreign events land");
    append(&store, &[event("Local", "a"), event("Local", "b")]).await;

    let mut expected = Watermark::new();
    for held in log(&store).await {
        expected.advance(held.id.store(), held.id.position());
    }
    let marks = watermark(&store).await;
    assert_eq!(marks, expected);
    assert_eq!(marks.len(), 3);
    for from in [one, two] {
        let sent = batch
            .iter()
            .filter(|shipped| shipped.id.store() == from)
            .map(|shipped| shipped.id.position())
            .max();
        assert_eq!(marks.get(from), sent);
    }
}

/// The watermark's query can be served by the origin index with no sort.
///
/// The planner's switches make its choice deterministic on a tiny table, and
/// they ask the property: can the index serve this. Rejects dropping
/// `origin_position IS NOT NULL`, which plans a sequential scan and a sort, and
/// grouping by an expression.
#[tokio::test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container, or set HAPPENSTANCE_POSTGRES_URL)"]
async fn watermark_reads_the_origin_index_without_a_sort() {
    let store = live_store().await;
    let batch = [
        shipped(id(peer(0x33), 1), 1, event("Shipped", "a")),
        shipped(id(peer(0x34), 2), 2, event("Shipped", "b")),
    ];
    ingest(&store, &[IngestGroup::new(&batch, &[])])
        .await
        .expect("two new foreign events land");

    let mut transaction = store
        .store
        .pool()
        .begin()
        .await
        .expect("a transaction opens");
    sqlx::raw_sql(
        "SET LOCAL enable_seqscan = off; SET LOCAL enable_bitmapscan = off; \
         SET LOCAL enable_hashagg = off; SET LOCAL enable_sort = off;",
    )
    .execute(&mut *transaction)
    .await
    .expect("the planner's switches set");
    let plan: Vec<String> = sqlx::query_scalar(&format!("EXPLAIN {WATERMARK_SQL}"))
        .fetch_all(&mut *transaction)
        .await
        .expect("the watermark's query explains");
    transaction.rollback().await.expect("the transaction ends");

    let plan = plan.join("\n");
    assert!(plan.contains("event_origin_idx"), "{plan}");
    assert!(!plan.contains("Sort"), "{plan}");
}
